#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Pulling latest changes from remote..."
git pull --rebase

echo "Updating installed skills in ~/.gemini/config/skills/..."
mkdir -p "$HOME/.gemini/config/skills"

for skill_dir in "$SCRIPT_DIR"/skills/*/*; do
  if [ -d "$skill_dir" ] && [ -f "$skill_dir/SKILL.md" ]; then
    skill_name=$(basename "$skill_dir")
    echo "  -> Updating: $skill_name"
    mkdir -p "$HOME/.gemini/config/skills/$skill_name"
    cp -rf "$skill_dir"/* "$HOME/.gemini/config/skills/$skill_name/"
  fi
done

echo "Updating installed rules in ~/.gemini/config/rules/..."
mkdir -p "$HOME/.gemini/config/rules"
cp -rf "$SCRIPT_DIR"/rules/* "$HOME/.gemini/config/rules/"

echo "Karakuri skills and rules update complete!"
