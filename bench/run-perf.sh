#!/usr/bin/env bash
# The orchestrator of the hyperlane performance harness.
#
# Three sub-commands:
#
#   build <variant> <context>   build the images of one variant
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

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "${ROOT}/.." && pwd)"
DOCKERFILE="${ROOT}/docker/Dockerfile.perf"
RESULTS="${PERF_RESULTS:-${ROOT}/results}"
NETWORK="${PERF_NETWORK:-hyperlane-perf}"
IMAGE_PREFIX="${PERF_IMAGE_PREFIX:-hyperlane-perf}"
SERVER_CPUS="${PERF_SERVER_CPUS:-4}"
LOADGEN_CPUS="${PERF_LOADGEN_CPUS:-4}"
MEMORY="${PERF_MEMORY:-4g}"
CONNECTIONS="${PERF_CONNECTIONS:-64 256}"
DURATION="${PERF_DURATION:-12}"
WARMUP="${PERF_WARMUP:-4}"
REPEATS="${PERF_REPEATS:-3}"

log() { printf '\n\033[1m==> %s\033[0m\n' "$*"; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }

cleanup() {
    docker rm -f $(docker ps -aq --filter "name=perf-") >/dev/null 2>&1 || true
}
trap cleanup EXIT

# Builds the images of one variant and the shared harness images.
#
# Arguments:
#   $1 - The name of the variant.
#   $2 - The build context, holding `hyperlane/` and `bench/`.
build_variant() {
    local variant="$1"
    local context="$2"
    [[ -f "${context}/Cargo.toml" ]] || die "the context ${context} has no library tree"
    [[ -d "${context}/bench" ]] || die "the context ${context} has no bench tree"
    log "building the ${variant} server image"
    docker build -q -f "${DOCKERFILE}" --target server-runtime \
        -t "${IMAGE_PREFIX}-server:${variant}" "${context}" >/dev/null
    log "building the load generator image"
    docker build -q -f "${DOCKERFILE}" --target loadgen-runtime \
        -t "${IMAGE_PREFIX}-loadgen:1.0" "${context}" >/dev/null
    log "building the control server image"
    docker build -q -f "${DOCKERFILE}" --target control-runtime \
        -t "${IMAGE_PREFIX}-control:1.0" "${context}" >/dev/null
}

# Starts one server container and waits until it serves.
#
# Arguments:
#   $1 - The name of the container.
#   $2 - The image to run.
#   $3 - The name of the environment variable that carries the bind address.
start_server() {
    local container="$1"
    local image="$2"
    docker rm -f "${container}" >/dev/null 2>&1 || true
    docker run -d --name "${container}" \
        --network "${NETWORK}" \
        --cpus "${SERVER_CPUS}" \
        --memory "${MEMORY}" \
        -e BENCH_ADDR="0.0.0.0:8080" \
        "${image}" >/dev/null
    local ready=0
    local attempt=0
    while [[ "${attempt}" -lt 300 ]]; do
        if docker exec "${container}" wget -q -O /dev/null "http://127.0.0.1:8080/" >/dev/null 2>&1; then
            ready=1
            break
        fi
        attempt=$((attempt + 1))
        sleep 0.1
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
drive() {
    local container="$1"
    local address="$2"
    local connections="$3"
    docker run --rm --name "${container}" \
        --network "${NETWORK}" \
        --cpus "${LOADGEN_CPUS}" \
        --memory "${MEMORY}" \
        "${IMAGE_PREFIX}-loadgen:1.0" \
        --addr "${address}:8080" \
        --connections "${connections}" \
        --duration "${DURATION}" \
        --warmup "${WARMUP}"
}

# Runs one paired measurement and appends its rows to the result file.
#
# Arguments:
#   $1 - The name of the first variant.
#   $2 - The name of the second variant.
#   $3 - The number of connections.
#   $4 - The number of the repeat.
run_pair() {
    local first="$1"
    local second="$2"
    local connections="$3"
    local repeat="$4"
    local first_container="perf-server-${first}-${connections}"
    local second_container="perf-server-${second}-${connections}"
    start_server "${first_container}" "${IMAGE_PREFIX}-server:${first}"
    start_server "${second_container}" "${IMAGE_PREFIX}-server:${second}"
    local first_file="${RESULTS}/first-${connections}-${repeat}.json"
    local second_file="${RESULTS}/second-${connections}-${repeat}.json"
    drive "perf-loadgen-${first}-${connections}-${repeat}" "${first_container}" "${connections}" \
        > "${first_file}" 2>/dev/null &
    local first_pid=$!
    drive "perf-loadgen-${second}-${connections}-${repeat}" "${second_container}" "${connections}" \
        > "${second_file}" 2>/dev/null &
    local second_pid=$!
    wait "${first_pid}" || true
    wait "${second_pid}" || true
    local first_output=""
    local second_output=""
    first_output="$(cat "${first_file}")"
    second_output="$(cat "${second_file}")"
    [[ -n "${first_output}" ]] || die "the load generator produced no output for ${first}"
    [[ -n "${second_output}" ]] || die "the load generator produced no output for ${second}"
    printf '{"variant":"%s","connections":%s,"repeat":%s,"result":%s}\n' \
        "${first}" "${connections}" "${repeat}" "${first_output}" >> "${RESULTS}/raw.jsonl"
    printf '{"variant":"%s","connections":%s,"repeat":%s,"result":%s}\n' \
        "${second}" "${connections}" "${repeat}" "${second_output}" >> "${RESULTS}/raw.jsonl"
    echo "  ${first}: ${first_output}"
    echo "  ${second}: ${second_output}"
}

# Runs the control probe, which measures what the machine itself allows.
#
# Arguments:
#   $1 - The number of connections.
#   $2 - The number of the repeat.
run_control() {
    local connections="$1"
    local repeat="$2"
    local container="perf-server-control-${connections}"
    start_server "${container}" "${IMAGE_PREFIX}-control:1.0"
    local output=""
    output=$(drive "perf-loadgen-control-${connections}-${repeat}" "${container}" "${connections}")
    [[ -n "${output}" ]] || die "the control load generator produced no output"
    printf '{"variant":"control","connections":%s,"repeat":%s,"result":%s}\n' \
        "${connections}" "${repeat}" "${output}" >> "${RESULTS}/raw.jsonl"
    echo "  control: ${output}"
}

# The sub-commands.
case "${1:-}" in
    build)
        [[ -n "${2:-}" && -n "${3:-}" ]] || die "usage: run-perf.sh build <variant> <context>"
        docker network inspect "${NETWORK}" >/dev/null 2>&1 || docker network create "${NETWORK}" >/dev/null
        build_variant "$2" "$3"
        ;;
    bench)
        [[ -n "${2:-}" && -n "${3:-}" ]] || die "usage: run-perf.sh bench <variant-a> <variant-b>"
        mkdir -p "${RESULTS}"
        : > "${RESULTS}/raw.jsonl"
        for connections in ${CONNECTIONS}; do
            for repeat in $(seq 1 "${REPEATS}"); do
                log "conn=${connections} repeat=${repeat}/${REPEATS}"
                run_pair "$2" "$3" "${connections}" "${repeat}"
                run_control "${connections}" "${repeat}"
            done
        done
        ;;
    report)
        python3 "${ROOT}/summarize.py" "${RESULTS}/raw.jsonl"
        ;;
    *)
        die "usage: run-perf.sh {build <variant> <context>|bench <variant-a> <variant-b>|report}"
        ;;
esac
