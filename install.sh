#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# Aesthetic terminal colors
C_RESET='\033[0m'
C_BOLD='\033[1m'
C_PINK='\033[38;5;218m'
C_GREEN='\033[38;5;120m'
C_CYAN='\033[38;5;117m'
C_GRAY='\033[38;5;245m'

printf "\n%b" "${C_PINK}${C_BOLD}"
cat << 'EOF'
  🌸 からくり (Karakuri) — Global Installer
  Modular AI Agent Skills & Behavioral Guardrails
EOF
printf "%b\n" "${C_RESET}"

REPO_URL="https://github.com/Praveensenpai/karakuri.git"
GLOBAL_GEMINI_SKILLS="${HOME}/.gemini/config/skills"
GLOBAL_AGENTS_SKILLS="${HOME}/.agents/skills"
GLOBAL_GEMINI_RULES="${HOME}/.gemini/config/rules"
GLOBAL_AGENTS_RULES="${HOME}/.agents/rules"

TEMP_DIR=""
cleanup() {
    if [[ -n "${TEMP_DIR}" && -d "${TEMP_DIR}" ]]; then
        rm -rf "${TEMP_DIR}"
    fi
}
trap cleanup EXIT INT TERM

# Determine source directory
SCRIPT_DIR=""
if [[ -n "${BASH_SOURCE[0]:-}" && -f "${BASH_SOURCE[0]}" ]]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fi

SOURCE_DIR=""
if [[ -n "${SCRIPT_DIR}" && -d "${SCRIPT_DIR}/skills" ]]; then
    SOURCE_DIR="${SCRIPT_DIR}"
else
    printf "  %b⏳ Downloading latest karakuri repository...%b\n" "${C_CYAN}" "${C_RESET}"
    TEMP_DIR="$(mktemp -d -t karakuri_install.XXXXXXXXXX)"
    git clone --depth 1 "${REPO_URL}" "${TEMP_DIR}" >/dev/null 2>&1
    SOURCE_DIR="${TEMP_DIR}"
fi

# Ensure destination directories exist
mkdir -p "${GLOBAL_GEMINI_SKILLS}" "${GLOBAL_AGENTS_SKILLS}" "${GLOBAL_GEMINI_RULES}" "${GLOBAL_AGENTS_RULES}"

printf "  %b📦 Installing skills globally...%b\n\n" "${C_CYAN}" "${C_RESET}"

# Find all SKILL.md files and install them
installed_count=0
while IFS= read -r skill_file; do
    skill_dir="$(dirname "${skill_file}")"
    skill_name="$(basename "${skill_dir}")"
    
    target_gemini="${GLOBAL_GEMINI_SKILLS}/${skill_name}"
    target_agents="${GLOBAL_AGENTS_SKILLS}/${skill_name}"
    
    mkdir -p "${target_gemini}" "${target_agents}"
    cp -r "${skill_dir}/." "${target_gemini}/"
    cp -r "${skill_dir}/." "${target_agents}/"
    
    printf "    %b[✓]%b %b%-26s%b %binstalled%b\n" \
        "${C_GREEN}" "${C_RESET}" \
        "${C_BOLD}" "${skill_name}" "${C_RESET}" \
        "${C_GRAY}" "${C_RESET}"
    
    installed_count=$((installed_count + 1))
done < <(find "${SOURCE_DIR}/skills" -type f -name "SKILL.md" | sort)

# Install behavioral rules globally
if [[ -d "${SOURCE_DIR}/rules" ]]; then
    printf "\n  %b🛡️  Installing global agent guardrails & rules...%b\n\n" "${C_CYAN}" "${C_RESET}"
    
    if [[ -f "${SOURCE_DIR}/rules/AGENTS.md" ]]; then
        cp "${SOURCE_DIR}/rules/AGENTS.md" "${HOME}/.gemini/config/AGENTS.md"
        cp "${SOURCE_DIR}/rules/AGENTS.md" "${HOME}/.agents/AGENTS.md"
        cp "${SOURCE_DIR}/rules/AGENTS.md" "${GLOBAL_GEMINI_RULES}/AGENTS.md"
        cp "${SOURCE_DIR}/rules/AGENTS.md" "${GLOBAL_AGENTS_RULES}/AGENTS.md"
        printf "    %b[✓]%b AGENTS.md globally installed\n" "${C_GREEN}" "${C_RESET}"
    fi
    
    if [[ -f "${SOURCE_DIR}/rules/RULES.md" ]]; then
        cp "${SOURCE_DIR}/rules/RULES.md" "${GLOBAL_GEMINI_RULES}/RULES.md"
        cp "${SOURCE_DIR}/rules/RULES.md" "${GLOBAL_AGENTS_RULES}/RULES.md"
        printf "    %b[✓]%b RULES.md globally installed\n" "${C_GREEN}" "${C_RESET}"
    fi
fi

printf "\n  %b✨ Successfully installed %d skills and global rules!%b\n" "${C_GREEN}${C_BOLD}" "${installed_count}" "${C_RESET}"
printf "  %bCustomization Locations:%b\n" "${C_GRAY}" "${C_RESET}"
printf "    • %b\n" "${GLOBAL_GEMINI_SKILLS}"
printf "    • %b\n" "${GLOBAL_AGENTS_SKILLS}"
printf "    • %b\n\n" "${HOME}/.gemini/config/AGENTS.md"
