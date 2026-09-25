#!/usr/bin/env bash
#
# sync_versions.sh
#
# Read [workspace.package].version from root Cargo.toml and rewrite every
# `version = "..."` field on the local path-only entries inside
# [workspace.dependencies] so they always equal the workspace version.
# Also aligns the left-hand dependency name on each line with the actual
# [package].name declared inside the member crate, so renames like
# `hyperlane-type -> http-type` flow out of the member dir and into the
# workspace's dependency table automatically.
#
# This script is invoked by .github/workflows/rust.yml (publish job) before
# `cargo publish`. The publish job commits and pushes the resulting diff
# back to master so the next workflow run sees an already-synchronized
# tree and exits early without committing.
#
# Usage:
#   scripts/sync_versions.sh [path/to/Cargo.toml]
#
# Behavior:
#   * No args defaults to ./Cargo.toml.
#   * Reads WORKSPACE_VERSION via the first `version = "X.Y.Z"` line in
#     [workspace.package].
#   * Walks [workspace.members]. For each member path P:
#       1. Reads CRATE_NAME = the first `name = "X"` line in P/Cargo.toml's
#          [package] block.
#       2. Locates the line in [workspace.dependencies] whose `path = "P"`
#          entry has the current member path. If the left-hand dep name
#          differs from CRATE_NAME, the LHS is rewritten in-place.
#       3. If the `version = "..."` literal on that line differs from
#          WORKSPACE_VERSION, it is rewritten to WORKSPACE_VERSION.
#   * Idempotent: if no LHS or version rewrite is needed for any member,
#     exits 0 and prints "already in sync: vX.Y.Z".
#   * Exits non-zero on any I/O or parse error.

set -euo pipefail

CARGO_TOML="${1:-Cargo.toml}"

if [ ! -f "${CARGO_TOML}" ]; then
    echo "sync_versions: ${CARGO_TOML} not found" >&2
    exit 1
fi

# Locate [workspace.package] and pull the first `version = "..."` line.
WORKSPACE_VERSION=$(
    awk '
        /^\[workspace\.package\]/ { in_pkg = 1; next }
        /^\[/                       { in_pkg = 0 }
        in_pkg && /^version[[:space:]]*=/ {
            sub(/^version[[:space:]]*=[[:space:]]*"/, "")
            sub(/".*/, "")
            print
            exit
        }
    ' "${CARGO_TOML}"
)

if [ -z "${WORKSPACE_VERSION}" ]; then
    echo "sync_versions: could not read workspace.package.version" >&2
    exit 1
fi

echo "sync_versions: workspace version = ${WORKSPACE_VERSION}"

# Collect the list of local path-only member crate paths from
# [workspace] members = [...] so we know which entries to rewrite.
MEMBER_PATHS=$(
    awk '
        /^\[workspace\]/ { in_ws = 1; next }
        /^\[/            { in_ws = 0 }
        in_ws && /^members[[:space:]]*=/ {
            line = $0
            sub(/^members[[:space:]]*=[[:space:]]*/, "", line)
            gsub(/[\[\]" ]/, "", line)
            n = split(line, parts, ",")
            for (i = 1; i <= n; i++) {
                if (parts[i] != "") print parts[i]
            }
            exit
        }
    ' "${CARGO_TOML}"
)

if [ -z "${MEMBER_PATHS}" ]; then
    echo "sync_versions: no workspace members found, nothing to do"
    exit 0
fi

# For each member path P, look up the actual crate name from P/Cargo.toml's
# [package] block, then align the LHS dep name + version literal in the
# root [workspace.dependencies] table.
REWRITE_COUNT=0
for member_path in ${MEMBER_PATHS}; do
    MEMBER_CARGO="${member_path}/Cargo.toml"
    if [ ! -f "${MEMBER_CARGO}" ]; then
        echo "sync_versions: warning: ${MEMBER_CARGO} missing, skipping ${member_path}" >&2
        continue
    fi

    CRATE_NAME=$(
        awk '
            /^\[package\]/ { in_pkg = 1; next }
            /^\[/          { in_pkg = 0 }
            in_pkg && /^name[[:space:]]*=/ {
                sub(/^name[[:space:]]*=[[:space:]]*"/, "")
                sub(/".*/, "")
                print
                exit
            }
        ' "${MEMBER_CARGO}"
    )

    if [ -z "${CRATE_NAME}" ]; then
        echo "sync_versions: warning: could not read package.name from ${MEMBER_CARGO}, skipping" >&2
        continue
    fi

    # Find the line that references this member path. Extract the current
    # LHS dep name from it for diagnostics and for the rename rewrite.
    LINE_PATTERN="^[^=]+=[[:space:]]*\\{[^}]*path[[:space:]]*=[[:space:]]*\"${member_path}\"[^}]*\\}"
    MATCHING_LINE=$(grep -E "${LINE_PATTERN}" "${CARGO_TOML}" | head -1 || true)

    if [ -z "${MATCHING_LINE}" ]; then
        # No workspace dependency entry references this member yet. Skip.
        continue
    fi

    CURRENT_LHS=$(echo "${MATCHING_LINE}" | sed -E 's|[[:space:]]*=.*||')

    NEEDS_NAME_REWRITE=false
    if [ "${CURRENT_LHS}" != "${CRATE_NAME}" ]; then
        NEEDS_NAME_REWRITE=true
    fi

    # Check if the version literal already equals WORKSPACE_VERSION.
    IDEMPOTENT_NEEDLE="${CURRENT_LHS} = { path = \"${member_path}\", version = \"${WORKSPACE_VERSION}\""
    if [ "${NEEDS_NAME_REWRITE}" = "false" ] && grep -F -q "${IDEMPOTENT_NEEDLE}" "${CARGO_TOML}"; then
        continue
    fi

    # Rewrite the line. Preserve the path = "P" literal and replace the
    # entire left-hand dep + version literal block with the canonical form:
    #     CRATE_NAME = { path = "P", version = "WORKSPACE_VERSION" }
    sed -i.bak -E "s|^([^=]+)=[[:space:]]*\\{[^}]*path[[:space:]]*=[[:space:]]*\"${member_path}\"[^}]*\\}|${CRATE_NAME} = { path = \"${member_path}\", version = \"${WORKSPACE_VERSION}\" }|" "${CARGO_TOML}"
    rm -f "${CARGO_TOML}.bak"
    REWRITE_COUNT=$((REWRITE_COUNT + 1))

    if [ "${NEEDS_NAME_REWRITE}" = "true" ]; then
        echo "sync_versions: ${member_path} -> ${CRATE_NAME} v${WORKSPACE_VERSION} (renamed from ${CURRENT_LHS})"
    else
        echo "sync_versions: ${member_path} -> ${CRATE_NAME} v${WORKSPACE_VERSION}"
    fi
done

if [ "${REWRITE_COUNT}" -gt 0 ]; then
    echo "sync_versions: rewrote ${REWRITE_COUNT} entries to v${WORKSPACE_VERSION}"
else
    echo "sync_versions: already in sync: v${WORKSPACE_VERSION}"
fi

exit 0