#!/usr/bin/env bash
# ==============================================================================
#  🌸 からくり (Karakuri) — Bootstrap Launcher
#  Builds and launches the native aesthetic Karakuri CLI
# ==============================================================================

set -euo pipefail
IFS=$'\n\t'

REPO_URL="https://github.com/Praveensenpai/karakuri.git"

reconnect_tty_and_exec() {
    if [[ ! -t 0 && -t 1 && -e /dev/tty ]]; then
        exec "$@" < /dev/tty
    else
        exec "$@"
    fi
}

main() {
    local SCRIPT_DIR=""
    if [[ -n "${BASH_SOURCE[0]:-}" && -f "${BASH_SOURCE[0]}" ]]; then
        SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    fi

    # 1. If running from within a local repository checkout, run local build
    if [[ -n "${SCRIPT_DIR}" && -f "${SCRIPT_DIR}/Cargo.toml" ]]; then
        if command -v cargo >/dev/null 2>&1; then
            cargo run --quiet --release --manifest-path "${SCRIPT_DIR}/Cargo.toml" -- "$@"
            exit 0
        fi
    fi

    # 2. Query latest release tag from GitHub
    local LATEST_TAG=""
    if command -v curl >/dev/null 2>&1; then
        LATEST_TAG="$(curl -fsSL -o /dev/null -w "%{url_effective}" "https://github.com/Praveensenpai/karakuri/releases/latest" 2>/dev/null | sed 's|.*/tag/||' || true)"
    fi

    # 3. If installed binary already matches latest release, execute directly
    if command -v karakuri >/dev/null 2>&1; then
        local CURRENT_VER=""
        CURRENT_VER="v$(karakuri --version 2>/dev/null | awk '{print $2}' || true)"
        if [[ -n "${LATEST_TAG}" && -n "${CURRENT_VER}" && "${CURRENT_VER}" == "${LATEST_TAG}" ]]; then
            reconnect_tty_and_exec karakuri "$@"
        fi
    fi

    # 4. Resolve preferred target bin directory in PATH
    local INSTALL_DIR="${HOME}/.cargo/bin"
    if [[ ! -d "${INSTALL_DIR}" ]] || [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
        INSTALL_DIR="${HOME}/.local/bin"
    fi
    mkdir -p "${INSTALL_DIR}"

    # 5. Download and install precompiled release binary
    local ARCH OS ARCHIVE_NAME RELEASE_URL TEMP_RUN_DIR
    ARCH="$(uname -m)"
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCHIVE_NAME="karakuri-${ARCH}-${OS}.tar.gz"
    RELEASE_URL="https://github.com/Praveensenpai/karakuri/releases/latest/download/${ARCHIVE_NAME}"

    TEMP_RUN_DIR="$(mktemp -d -t karakuri_bin.XXXXXXXXXX)"
    cleanup_bin() { rm -rf "${TEMP_RUN_DIR}"; }
    trap cleanup_bin EXIT INT TERM HUP

    if curl -fsSL "${RELEASE_URL}" 2>/dev/null | tar -xzf - -C "${TEMP_RUN_DIR}" 2>/dev/null; then
        if [[ -x "${TEMP_RUN_DIR}/karakuri" ]]; then
            install -m 755 "${TEMP_RUN_DIR}/karakuri" "${INSTALL_DIR}/karakuri"
            reconnect_tty_and_exec "${INSTALL_DIR}/karakuri" "$@"
        fi
    fi

    # 6. Fallback: Build latest from source via Cargo
    if command -v cargo >/dev/null 2>&1; then
        echo "Notice: Compiling latest karakuri release via Cargo..." >&2
        cargo install --git "${REPO_URL}" --force --quiet
        reconnect_tty_and_exec karakuri "$@"
    fi

    echo "Error: Neither precompiled binary for ${ARCH}-${OS} nor cargo is available." >&2
    exit 1
}

main "$@"
