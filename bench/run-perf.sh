#!/usr/bin/env bash
# The orchestrator of the hyperlane performance harness.
#
# Four sub-commands:
#
#   instrument <context>       build the load generator and the control
#   build <variant> <context>   build the server image of one variant
#   bench <variant-a> <variant-b>  drive both variants at the same time
#   report                      turn the measurements into a conclusion
#
# The build context of a variant holds two trees: `hyperlane/`, the library
# under test, and `bench/`, the harness. A candidate uses the working tree; a
# baseline uses the merge-base of the base branch, with the same harness
# copied into it, so the two builds differ only in the library.
#
# The measurement drives both variants at the same time, with one load
# generator each, over the same wall clock window. A change in the state of
# the machine lands on both measurements and cancels out, which a sequential
# comparison cannot do: two runs of the same binary were measured to differ by
# up to 65 percent.
#
# The measurement is a matrix, not a single row. The protocol axis is the
# wire protocol the server speaks, and the connection axis is whether a
# connection carries many requests or exactly one. Every cell is measured, and
# a cell that was not measured is reported as such rather than left unstated.
#
# The frozen parameters and the verdict rules are in CONTRACT.md.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "${ROOT}/.." && pwd)"
DOCKERFILE_NAME="docker/Dockerfile.perf"
RESULTS="${PERF_RESULTS:-${ROOT}/results}"
NETWORK="${PERF_NETWORK:-hyperlane-perf}"
IMAGE_PREFIX="${PERF_IMAGE_PREFIX:-hyperlane-perf}"
SERVER_CPUS="${PERF_SERVER_CPUS:-4}"
LOADGEN_CPUS="${PERF_LOADGEN_CPUS:-4}"
MEMORY="${PERF_MEMORY:-4g}"
CONNECTIONS="${PERF_CONNECTIONS:-64 256}"
CONNECTIONS_SHORT="${PERF_CONNECTIONS_SHORT:-16 64}"
DURATION="${PERF_DURATION:-12}"
WARMUP="${PERF_WARMUP:-4}"
REPEATS="${PERF_REPEATS:-3}"
PROTOCOLS="${PERF_PROTOCOLS:-h1 h1-tls h2c h2-tls h3}"
MODES="${PERF_MODES:-keepalive short}"
COOLDOWN_SHORT="${PERF_COOLDOWN_SHORT:-20}"
RATE_SHORT="${PERF_RATE_SHORT:-200}"
CERTS="${PERF_CERTS:-${ROOT}/certs}"
CA_CERT="${PERF_CA_CERT:-${CERTS}/ca.pem}"
TLS_CERTIFICATE="${PERF_TLS_CERT:-${CERTS}/leaf.pem}"
TLS_KEY="${PERF_TLS_KEY:-${CERTS}/leaf.key}"
READY_MARKER="ready on"

log() { printf '\n\033[1m==> %s\033[0m\n' "$*"; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }

cleanup() {
    docker rm -f $(docker ps -aq --filter "name=perf-") >/dev/null 2>&1 || true
}
trap cleanup EXIT

# Reports the server-side name of a protocol variant.
#
# The client names the variants the way a caller would, and the server names
# them the way the library names them, and the two are not the same strings.
#
# Arguments:
#   $1 - The name of the protocol variant.
#
# Returns:
#   The name the server takes, on stdout.
server_protocol_of() {
    case "$1" in
        h1) echo "http1" ;;
        h1-tls) echo "http1-tls" ;;
        h2c) echo "http2" ;;
        h2-tls) echo "http2-tls" ;;
        h3) echo "http3" ;;
        *) echo "$1" ;;
    esac
}

# Reports whether a protocol variant terminates TLS.
#
# Arguments:
#   $1 - The name of the protocol variant.
#
# Returns:
#   0 when the variant terminates TLS, 1 when it does not.
variant_uses_tls() {
    case "$1" in
        h1-tls|h2-tls|h3) return 0 ;;
        *) return 1 ;;
    esac
}

# Copies the current harness into a build context.
#
# The load generator and the control are the measuring instrument, and an
# instrument that differs between the two arms measures the instrument. The
# baseline context therefore receives the harness of the candidate, so the
# only difference between the two images is the library under test.
#
# Arguments:
#   $1 - The context to copy into.
sync_harness() {
    local context="$1"
    local context_real
    context_real="$(cd "${context}" && pwd -P)"
    local root_real
    root_real="$(cd "${ROOT}" && pwd -P)"
    # The harness lives inside the repository, so a context that IS the
    # repository must not be synced into itself: removing its harness would
    # remove the source this function copies from.
    if [[ "${context_real}" == "${root_real}" ]]; then
        return 0
    fi
    # The harness is removed only when it is a copy, never when it is the
    # source. The check is on the harness path itself, not on the context:
    # the context of a candidate IS the repository, and its harness is the
    # one this function copies from.
    local target="${context_real}/bench"
    if [[ "${target}" == "${root_real}" ]]; then
        return 0
    fi
    rm -rf "${target}"
    cp -R "${ROOT}" "${target}"
    # The lock has to name this context's own workspace members. The
    # baseline's source carries a different version than the candidate's, and
    # a lock that names the other one cannot be used with `--locked`, which
    # would either fail the build or, worse, resolve the dependencies again
    # and let the two arms drift apart. Only the workspace members move: every
    # third-party pin stays exactly as it was.
    cargo update --workspace --offline --manifest-path "${target}/Cargo.toml" \
        >/dev/null 2>&1 || die "the lock of ${context} does not match its source"
}

# Builds the images of one variant.
#
# Arguments:
#   $1 - The name of the variant.
#   $2 - The build context, holding `hyperlane/` and `bench/`.
build_variant() {
    local variant="$1"
    local context="$2"
    [[ -f "${context}/Cargo.toml" ]] || die "the context ${context} has no library tree"
    [[ -d "${context}/bench" ]] || die "the context ${context} has no bench tree"
    sync_harness "${context}"
    ensure_certificates
    local dockerfile="${context}/bench/${DOCKERFILE_NAME}"
    [[ -f "${dockerfile}" ]] || die "the context has no ${DOCKERFILE_NAME}"
    log "building the ${variant} server image"
    docker build -q -f "${dockerfile}" --target server-runtime \
        -t "${IMAGE_PREFIX}-server:${variant}" "${context}" >/dev/null
}

# Builds the images of the measuring instrument.
#
# The load generator and the control are the instrument. They are built once,
# from the same harness for both arms, so the two measurements differ only in
# the server.
#
# Arguments:
#   $1 - The build context.
build_instrument() {
    local context="$1"
    ensure_certificates
    local dockerfile="${context}/bench/${DOCKERFILE_NAME}"
    [[ -f "${dockerfile}" ]] || die "the context has no ${DOCKERFILE_NAME}"
    log "building the load generator image"
    docker build -q -f "${dockerfile}" --target loadgen-runtime \
        -t "${IMAGE_PREFIX}-loadgen:1.0" "${context}" >/dev/null
    log "building the control server image"
    docker build -q -f "${dockerfile}" --target control-runtime \
        -t "${IMAGE_PREFIX}-control:1.0" "${context}" >/dev/null
}

# Makes sure the certificate material the TLS rows need exists.
#
# The material is generated rather than committed: a private key in a
# repository is a private key in every clone of that repository.
ensure_certificates() {
    if [[ -f "${CERTS}/ca.pem" && -f "${CERTS}/leaf.pem" && -f "${CERTS}/leaf.key" ]]; then
        return 0
    fi
    log "generating the benchmark certificate material"
    "${ROOT}/scripts/generate-certs.sh" "${CERTS}" >/dev/null
}

# Proves the two arms are different images before the first repeat.
#
# Two arms that share a build directory can collapse into one image: the
# second build overwrites the first, both builds report success, and the
# comparison then measures one build against itself and reports a flattering
# zero difference. The image ids are different files or the measurement is
# void.
#
# Arguments:
#   $1 - The name of the first variant.
#   $2 - The name of the second variant.
#
# Returns:
#   0 when the two images differ, 1 when they are the same.
verify_arms_differ() {
    local first="$1"
    local second="$2"
    local first_id
    local second_id
    first_id="$(docker image inspect --format '{{.Id}}' "${IMAGE_PREFIX}-server:${first}" 2>/dev/null)"
    second_id="$(docker image inspect --format '{{.Id}}' "${IMAGE_PREFIX}-server:${second}" 2>/dev/null)"
    if [[ -z "${first_id}" || -z "${second_id}" ]]; then
        echo "error: an arm image is missing" >&2
        return 1
    fi
    if [[ "${first_id}" == "${second_id}" ]]; then
        echo "error: the two arms are the same image (${first_id})" >&2
        return 1
    fi
    echo "  arm images differ:"
    echo "    ${first}: ${first_id}"
    echo "    ${second}: ${second_id}"
    return 0
}

# Starts one server container and waits until it serves.
#
# The readiness marker is the server's own announcement, so the wait is not a
# guess. A TCP probe cannot tell a bound HTTP/3 listener from a dead one,
# because HTTP/3 is carried over UDP.
#
# Arguments:
#   $1 - The name of the container.
#   $2 - The image to run.
#   $3 - The name of the protocol the server speaks.
start_server() {
    local container="$1"
    local image="$2"
    local protocol="$3"
    docker rm -f "${container}" >/dev/null 2>&1 || true
    local -a server_environment=(
        "BENCH_ADDR=0.0.0.0:8080"
        "BENCH_PROTOCOL=$(server_protocol_of "${protocol}")"
    )
    if variant_uses_tls "${protocol}"; then
        server_environment+=("BENCH_TLS_CERT=/certs/leaf.pem" "BENCH_TLS_KEY=/certs/leaf.key")
    fi
    # Every environment variable gets its own flag: the `-e` flag takes one
    # value, so an array after it would be read as the image name.
    local -a flags=()
    local variable
    for variable in "${server_environment[@]}"; do
        flags+=(-e "${variable}")
    done
    docker run -d --name "${container}" \
        --network "${NETWORK}" \
        --cpus "${SERVER_CPUS}" \
        --memory "${MEMORY}" \
        -v "${CERTS}:/certs:ro" \
        "${flags[@]}" \
        "${image}" >/dev/null
    local ready=0
    local attempt=0
    while [[ "${attempt}" -lt 300 ]]; do
        if docker logs "${container}" 2>&1 | grep -q "${READY_MARKER}"; then
            ready=1
            break
        fi
        if ! docker inspect "${container}" >/dev/null 2>&1; then
            break
        fi
        attempt=$((attempt + 1))
        sleep 0.2
    done
    [[ "${ready}" -eq 1 ]] || {
        docker logs "${container}" >&2 || true
        die "the server ${container} did not become ready"
    }
}

# Drives one server with the load generator.
#
# Arguments:
#   $1 - The name of the load generator container.
#   $2 - The address of the server to drive.
#   $3 - The number of connections.
#   $4 - The protocol variant the client speaks.
#   $5 - The connection mode.
drive() {
    local container="$1"
    local address="$2"
    local connections="$3"
    local variant="$4"
    local mode="$5"
    local -a arguments=(
        --addr "${address}:8080"
        --connections "${connections}"
        --duration "${DURATION}"
        --warmup "${WARMUP}"
        --protocol "${variant}"
        --mode "${mode}"
    )
    if variant_uses_tls "${variant}"; then
        arguments+=(--ca "/certs/ca.pem")
    fi
    if [[ "${mode}" == "short" ]]; then
        # A short-connection row opens one connection per request, and the
        # kernel holds each of them in TIME_WAIT. Without a rate cap the
        # generator spends the ephemeral port range in seconds and then fails
        # with EADDRNOTAVAIL, which reads as a server defect and is not one.
        arguments+=(--rate "${RATE_SHORT}")
    fi
    docker run --rm --name "${container}" \
        --network "${NETWORK}" \
        --cpus "${LOADGEN_CPUS}" \
        --memory "${MEMORY}" \
        -v "${CERTS}:/certs:ro" \
        "${IMAGE_PREFIX}-loadgen:1.0" \
        "${arguments[@]}"
}

# Runs one paired measurement and appends its rows to the result file.
#
# Arguments:
#   $1 - The name of the first variant.
#   $2 - The name of the second variant.
#   $3 - The number of connections.
#   $4 - The number of the repeat.
#   $5 - The protocol variant.
#   $6 - The connection mode.
run_pair() {
    local first="$1"
    local second="$2"
    local connections="$3"
    local repeat="$4"
    local variant="$5"
    local mode="$6"
    # The order in which the two arms are measured can bias the result: the
    # first arm starts its server and its load generator first, so it measures
    # a slightly different machine state than the second. The order is
    # alternated across the repeats, so the bias cancels instead of landing on
    # the same arm in every pair.
    if ((repeat % 2 == 0)); then
        local swap="${first}"
        first="${second}"
        second="${swap}"
    fi
    local first_container="perf-server-${first}-${variant}-${mode}-${connections}"
    local second_container="perf-server-${second}-${variant}-${mode}-${connections}"
    start_server "${first_container}" "${IMAGE_PREFIX}-server:${first}" "${variant}"
    start_server "${second_container}" "${IMAGE_PREFIX}-server:${second}" "${variant}"
    local first_file="${RESULTS}/first-${variant}-${mode}-${connections}-${repeat}.json"
    local second_file="${RESULTS}/second-${variant}-${mode}-${connections}-${repeat}.json"
    drive "perf-loadgen-${first}-${variant}-${mode}-${connections}-${repeat}" \
        "${first_container}" "${connections}" "${variant}" "${mode}" \
        > "${first_file}" 2>/dev/null &
    local first_pid=$!
    drive "perf-loadgen-${second}-${variant}-${mode}-${connections}-${repeat}" \
        "${second_container}" "${connections}" "${variant}" "${mode}" \
        > "${second_file}" 2>/dev/null &
    local second_pid=$!
    wait "${first_pid}" || true
    wait "${second_pid}" || true
    local first_output
    local second_output
    first_output="$(cat "${first_file}")"
    second_output="$(cat "${second_file}")"
    [[ -n "${first_output}" ]] || die "the load generator produced no output for ${first}"
    [[ -n "${second_output}" ]] || die "the load generator produced no output for ${second}"
    printf '{"variant":"%s","protocol":"%s","mode":"%s","connections":%s,"repeat":%s,"result":%s}\n' \
        "${first}" "${variant}" "${mode}" "${connections}" "${repeat}" "${first_output}" \
        >> "${RESULTS}/raw.jsonl"
    printf '{"variant":"%s","protocol":"%s","mode":"%s","connections":%s,"repeat":%s,"result":%s}\n' \
        "${second}" "${variant}" "${mode}" "${connections}" "${repeat}" "${second_output}" \
        >> "${RESULTS}/raw.jsonl"
    docker rm -f "${first_container}" "${second_container}" \
        >/dev/null 2>&1 || true
}

# The control probe, which measures what the machine itself allows.
#
# The control speaks the same protocol as the row it references, so its number
# is a like-for-like machine ceiling for that cell rather than for HTTP/1.1.
#
# The control is measured after the two arms, with the arms stopped.
#
# Measuring it at the same time as the arms asks for six containers at four
# CPUs each, which is twenty-four of the machine's eighteen, and a control
# that is competing for what is left cannot hold its own number still: it
# swung by a factor of two while the arms in the same cell held steady. A
# drift detector that drifts is not a detector. In its own window the control
# is the machine's ceiling for that protocol, which is what it is there to
# say.
#
# Arguments:
#   $1 - The number of connections.
#   $2 - The number of the repeat.
#   $3 - The protocol variant.
#   $4 - The connection mode.
run_control() {
    local connections="$1"
    local repeat="$2"
    local variant="$3"
    local mode="$4"
    local container="perf-server-control-${variant}-${mode}-${connections}"
    start_server "${container}" "${IMAGE_PREFIX}-control:1.0" "${variant}"
    local output
    output="$(drive "perf-loadgen-control-${variant}-${mode}-${connections}-${repeat}" \
        "${container}" "${connections}" "${variant}" "${mode}" 2>/dev/null)"
    [[ -n "${output}" ]] || die "the control produced no output"
    printf '{"variant":"control","protocol":"%s","mode":"%s","connections":%s,"repeat":%s,"result":%s}\n' \
        "${variant}" "${mode}" "${connections}" "${repeat}" "${output}" \
        >> "${RESULTS}/raw.jsonl"
    docker rm -f "${container}" >/dev/null 2>&1 || true
}

case "${1:-}" in
    instrument)
        [[ -n "${2:-}" ]] || die "usage: run-perf.sh instrument <context>"
        docker network inspect "${NETWORK}" >/dev/null 2>&1 || docker network create "${NETWORK}" >/dev/null
        build_instrument "$2"
        ;;
    build)
        [[ -n "${2:-}" && -n "${3:-}" ]] || die "usage: run-perf.sh build <variant> <context>"
        docker network inspect "${NETWORK}" >/dev/null 2>&1 || docker network create "${NETWORK}" >/dev/null
        build_variant "$2" "$3"
        ;;
    bench)
        [[ -n "${2:-}" && -n "${3:-}" ]] || die "usage: run-perf.sh bench <variant-a> <variant-b>"
        mkdir -p "${RESULTS}"
        : > "${RESULTS}/raw.jsonl"
        log "proving the two arms are different images"
        verify_arms_differ "$2" "$3" || die "the two arms are the same image"
        for variant in ${PROTOCOLS}; do
            for mode in ${MODES}; do
                levels="${CONNECTIONS}"
                if [[ "${mode}" == "short" ]]; then
                    levels="${CONNECTIONS_SHORT}"
                fi
                for connections in ${levels}; do
                    for repeat in $(seq 1 "${REPEATS}"); do
                        log "protocol=${variant} mode=${mode} conn=${connections} repeat=${repeat}/${REPEATS}"
                        run_pair "$2" "$3" "${connections}" "${repeat}" "${variant}" "${mode}"
                        run_control "${connections}" "${repeat}" "${variant}" "${mode}"
                        if [[ "${mode}" == "short" ]]; then
                            # A short-connection row spends one ephemeral port
                            # per request, and the kernel holds each of them in
                            # TIME_WAIT for twice the maximum segment lifetime.
                            # Without a cooldown the ports of one repeat are
                            # still held when the next one starts, and the
                            # client runs out of addresses before it runs out
                            # of server.
                            sleep "${COOLDOWN_SHORT}"
                        fi
                    done
                done
            done
        done
        ;;
    report)
        python3 "${ROOT}/summarize.py" "${RESULTS}/raw.jsonl"
        ;;
    *)
        die "usage: run-perf.sh <instrument|build|bench|report> ..."
        ;;
esac
