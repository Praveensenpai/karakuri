#!/usr/bin/env bash
# ==============================================================================
#  🌸 からくり (Karakuri) — Bootstrap Launcher
#  Builds and launches the native aesthetic Karakuri CLI
# ==============================================================================

set -euo pipefail
IFS=$'\n\t'

REPO_URL="https://github.com/Praveensenpai/karakuri.git"

# Reconnect stdin to terminal if script was piped via curl | bash
if [[ ! -t 0 && -e /dev/tty ]]; then
    exec < /dev/tty
fi

SCRIPT_DIR=""
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
LATEST_TAG=""
if command -v curl >/dev/null 2>&1; then
    LATEST_TAG="$(curl -fsSL -o /dev/null -w "%{url_effective}" "https://github.com/Praveensenpai/karakuri/releases/latest" 2>/dev/null | sed 's|.*/tag/||' || true)"
fi

# 3. If installed binary already matches latest release, execute directly
if command -v karakuri >/dev/null 2>&1; then
    CURRENT_VER="v$(karakuri --version 2>/dev/null | awk '{print $2}' || true)"
    if [[ -n "${LATEST_TAG}" && -n "${CURRENT_VER}" && "${CURRENT_VER}" == "${LATEST_TAG}" ]]; then
        exec karakuri "$@"
    fi
fi

# 4. Resolve preferred target bin directory in PATH
INSTALL_DIR="${HOME}/.cargo/bin"
if [[ ! -d "${INSTALL_DIR}" ]] || [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
    INSTALL_DIR="${HOME}/.local/bin"
fi
mkdir -p "${INSTALL_DIR}"

# 5. Download and install precompiled release binary
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
        exec "${INSTALL_DIR}/karakuri" "$@"
    fi
fi

# 6. Fallback: Build latest from source via Cargo
if command -v cargo >/dev/null 2>&1; then
    echo "Notice: Compiling latest karakuri release via Cargo..." >&2
    cargo install --git "${REPO_URL}" --force --quiet
    exec karakuri "$@"
fi

echo "Error: Neither precompiled binary for ${ARCH}-${OS} nor cargo is available." >&2
exit 1
