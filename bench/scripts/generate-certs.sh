#!/usr/bin/env bash
# Generates the certificate material the TLS rows of the matrix need.
#
# The material is generated rather than committed: a private key in a
# repository is a private key in every clone of that repository, and this one
# would be a throwaway that anyone could use to impersonate the benchmark
# server. The generator is the durable part; the material is disposable.
#
# The certificate authority signs one leaf certificate. The leaf carries the
# names the harness addresses the server by, and the clients verify the chain
# against the authority explicitly: the authority is the trust anchor, and the
# leaf is the end entity.
#
# Arguments:
#   $1 - The directory the material is written to.
#
# Returns:
#   0 when the material is ready, 1 when it could not be produced.
set -euo pipefail

# Every step reports its own failure: a generator that dies without a reason
# cannot be told apart from a generator that was never run.
step() {
    local what="$1"
    shift
    if ! "$@" >/dev/null 2>"${WORK}/error"; then
        echo "the certificate generator failed at ${what}:" >&2
        cat "${WORK}/error" >&2
        exit 1
    fi
}

TARGET="${1:-}"
[[ -n "${TARGET}" ]] || { echo "usage: generate-certs.sh <directory>" >&2; exit 1; }
mkdir -p "${TARGET}"

WORK="$(mktemp -d)"
trap 'rm -rf "${WORK}"' EXIT

step "the certificate authority" openssl req -x509 -newkey rsa:2048 -sha256 -days 3650 -nodes \
    -keyout "${WORK}/ca.key" -out "${WORK}/ca.pem" \
    -subj "/CN=hyperlane-bench-ca" \
    -addext "basicConstraints=critical,CA:TRUE"

step "the leaf key" openssl req -newkey rsa:2048 -nodes -keyout "${WORK}/leaf.key" \
    -out "${WORK}/leaf.csr" -subj "/CN=perf-server"

cat > "${WORK}/leaf.ext" <<'EOF'
basicConstraints=CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=serverAuth
subjectAltName=DNS:perf-server-a,DNS:perf-server-b,DNS:perf-server-control,DNS:localhost,IP:127.0.0.1
EOF

step "the leaf certificate" openssl x509 -req -in "${WORK}/leaf.csr" -CA "${WORK}/ca.pem" -CAkey "${WORK}/ca.key" \
    -CAcreateserial -out "${WORK}/leaf.pem" -days 3650 -sha256 \
    -extfile "${WORK}/leaf.ext"

# The signing key does not leave the temporary directory: only the trust
# anchor and the leaf are needed, and the leaf's key is not a signing key.
cp "${WORK}/ca.pem" "${TARGET}/ca.pem"
cp "${WORK}/leaf.pem" "${TARGET}/leaf.pem"
cp "${WORK}/leaf.key" "${TARGET}/leaf.key"
chmod 600 "${TARGET}/leaf.key"

step "the chain verification" openssl verify -CAfile "${TARGET}/ca.pem" "${TARGET}/leaf.pem"
echo "the benchmark certificate material is ready in ${TARGET}"
