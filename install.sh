#!/usr/bin/env bash
# ==============================================================================
#  🌸 からくり (Karakuri) — Bootstrap Launcher
#  Builds and launches the native aesthetic Karakuri CLI
# ==============================================================================

set -euo pipefail
IFS=$'\n\t'

REPO_URL="https://github.com/Praveensenpai/karakuri.git"

SCRIPT_DIR=""
if [[ -n "${BASH_SOURCE[0]:-}" && -f "${BASH_SOURCE[0]}" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fi

# 1. If karakuri binary is already in PATH, run it
if command -v karakuri >/dev/null 2>&1; then
    exec karakuri "$@"
fi

# 2. If prebuilt binary exists in repo target/release, run it
if [[ -n "${SCRIPT_DIR}" && -x "${SCRIPT_DIR}/target/release/karakuri" ]]; then
    exec "${SCRIPT_DIR}/target/release/karakuri" "$@"
fi

# 3. If cargo is available, build and run
if command -v cargo >/dev/null 2>&1; then
    if [[ -n "${SCRIPT_DIR}" && -f "${SCRIPT_DIR}/Cargo.toml" ]]; then
        cargo run --quiet --release --manifest-path "${SCRIPT_DIR}/Cargo.toml" -- "$@"
        exit 0
    else
        TEMP_DIR="$(mktemp -d -t karakuri_run.XXXXXXXXXX)"
        cleanup() { rm -rf "${TEMP_DIR}"; }
        trap cleanup EXIT INT TERM HUP

        git clone --depth 1 "${REPO_URL}" "${TEMP_DIR}" >/dev/null 2>&1
        cargo run --quiet --release --manifest-path "${TEMP_DIR}/Cargo.toml" -- "$@"
        exit 0
    fi
fi

# 4. Fallback: Download precompiled release binary
ARCH="$(uname -m)"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
BIN_NAME="karakuri-${ARCH}-${OS}"
RELEASE_URL="https://github.com/Praveensenpai/karakuri/releases/latest/download/${BIN_NAME}"

TEMP_BIN="$(mktemp -t karakuri_bin.XXXXXXXXXX)"
cleanup_bin() { rm -f "${TEMP_BIN}"; }
trap cleanup_bin EXIT INT TERM HUP

if curl -fsSL "${RELEASE_URL}" -o "${TEMP_BIN}" 2>/dev/null; then
    chmod +x "${TEMP_BIN}"
    exec "${TEMP_BIN}" "$@"
fi

echo "Error: Neither cargo nor a precompiled binary could be found for ${ARCH}-${OS}." >&2
echo "Please install Rust (https://rustup.rs) to build karakuri." >&2
exit 1
