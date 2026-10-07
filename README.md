# 🌸 からくり (karakuri) — AI Codebase & Skill Repository Orchestrator

> **High-performance AI codebase auditor, automated `CODEBASE.md` AST generator, and multi-agent skill synchronizer.**

[![Latest Release](https://img.shields.io/github/v/release/Praveensenpai/karakuri?style=for-the-badge&color=89b4fa)](https://github.com/Praveensenpai/karakuri/releases)
[![Rust Edition](https://img.shields.io/badge/Rust-2021%20Edition-DEA584?style=for-the-badge&logo=rust)](Cargo.toml)
[![Platform](https://img.shields.io/badge/Platform-Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://github.com/Praveensenpai/karakuri)
[![Skills Count](https://img.shields.io/badge/Skills-14%20Modular-cba6f7?style=for-the-badge)](skills/)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-a6e3a1?style=for-the-badge)](LICENSE)

<br>

<p align="center">
  <a href="#-quick-start">⚡ Quick Install</a> • 
  <a href="#-key-features">✨ Key Features</a> • 
  <a href="#️-architecture--orchestration-flow">🔄 Architecture</a> • 
  <a href="#-cli-usage--subcommands">💻 CLI Commands</a> • 
  <a href="#-skill-catalog">📚 Skill Catalog</a> • 
  <a href="#️-behavioral-guardrails--approval-protocol">🛡️ Guardrails</a>
</p>

> [!TIP]
> **Zero Assumptions · Evidence-Based Proof · Multi-Agent Synchronized**  
> Karakuri eliminates code sprawl and context bloat. It audits repositories against strict clean-code limits, generates living AI-first `CODEBASE.md` indexes in sub-seconds, and keeps custom skills unified across all local AI agent environments.

<br>

<p align="center">
  <img src="assets/demo.gif" alt="Karakuri CLI Showcase" width="850">
</p>

---

## 🏗️ Architecture & Orchestration Flow

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        🌸 からくり (karakuri) CLI                      │
└───────┬───────────────────┬───────────────────┬────────────────┬───────┘
        │                   │                   │                │
        │ 1. audit          │ 2. digest         │ 3. sync        │ 4. install
        ▼                   ▼                   ▼                ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│ Codebase      │   │ AST Parser    │   │ Cross-Agent   │   │ Interactive   │
│ Auditor       │   │ & Indexer     │   │ Synchronizer  │   │ TUI Wizard    │
│ · <400 LOC    │   │ · Rust `syn`  │   │ · ~/.gemini   │   │ · Scopes      │
│ · <60 fn LOC  │   │ · Type tokens │   │ · ~/.agents   │   │ · Components  │
│ · Nesting <=3 │   │ · Signatures  │   │ · ~/.claude   │   │ · Guardrails  │
│ · Zero clippy │   │ · Module role │   │ · ~/.codex    │   │ · Project CWD │
└───────┬───────┘   └───────┬───────┘   └───────┬───────┘   └───────┬───────┘
        ▼                   ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│ Formatted Pass│   │ Generated     │   │ Idempotent    │   │ Behavioral    │
│ / Fail Report │   │ `CODEBASE.md` │   │ Multi-Runtime │   │ Rules Active  │
│ with line refs│   │ Living Index  │   │ Skill Parity  │   │ & Discovered  │
└───────────────┘   └───────────────┘   └───────────────┘   └───────────────┘
```

---

## ✨ Key Features

| Feature | Description |
| :--- | :--- |
| **🔍 Strict Codebase Auditor** | Recursively audits codebases against hard limits: `<400` LOC/file, `<60` LOC/function, max nesting depth `<=3`, and flags forbidden suppressions (`#[allow(dead_code)]`). |
| **📑 Living Semantic Digest** | AST-parses codebases (via `syn`) to extract module roles, public structs, enums, and exact function signatures into a high-density, AI-first `CODEBASE.md`. |
| **🔄 Cross-Agent Skill Sync** | Automatically detects and synchronizes custom agent skills across multiple local runtimes (`~/.gemini`, `~/.agents`, `~/.claude`, `~/.codex`). |
| **🛡️ Behavioral Guardrails** | Embeds and installs standard behavioral contracts (Mandatory Explicit Approval, 3-phase systematic code verification, unslop prose hygiene). |
| **🎮 Interactive TUI Wizard** | Standalone keyboard-driven terminal menu (`↑`/`↓`, `Enter`) for selecting install scopes (global vs project) and components. |
| **⚡ Zero-Dependency Binary** | Compiles to a single standalone, stripped native binary for Linux x86_64 with zero runtime dependencies. |
| **🔄 Self-Update Command** | `karakuri update` fetches the latest release, verifies its SHA-256 checksum, and atomically replaces the running binary, with a `cargo install` fallback. |

---

## 🚀 Quick Start

### 🪄 One-Liner Magic (Recommended)

Install the compiled standalone binary directly with automatic shell detection:

```bash
curl -fsSL https://raw.githubusercontent.com/Praveensenpai/karakuri/main/install.sh | bash
```

<br>

### 🛠️ Building From Source

```bash
git clone https://github.com/Praveensenpai/karakuri.git
cd karakuri
cargo build --release
install -Dm 755 target/release/karakuri ~/.local/bin/karakuri
```

---

## 💻 CLI Usage & Subcommands

### 1. `karakuri audit [PATH]`
Scans the target repository (defaults to `.`) against clean-code rules and documentation standards:

```bash
# Audit the current repository
karakuri audit .

# Audit an external repository with custom limits
karakuri audit /path/to/project --max-file-lines 300 --max-fn-lines 40 --max-depth 2

# Audit without running cargo clippy
karakuri audit . --no-clippy
```

### 2. `karakuri digest [PATH]`
Parses the target repository AST to generate or synchronize a living `CODEBASE.md` semantic index:

```bash
# Generate and write CODEBASE.md in project root
karakuri digest .

# Output formatted markdown to stdout without writing to disk
karakuri digest . --stdout
```

### 3. `karakuri sync`
Synchronizes custom skills across all detected agent directories (`~/.gemini/config/skills/`, `~/.agents/skills/`, `~/.claude/skills/`, `~/.codex/skills/`):

```bash
karakuri sync
```

### 4. `karakuri install [FLAGS]`
Launches the interactive TUI wizard or installs rules and skills headlessly:

```bash
# Interactive TUI Wizard
karakuri

# Headless installation flags
karakuri install --project --rules -y   # Current project only (committed with repo)
karakuri install --project --all -y     # Current project with all skills
karakuri install --global --rules -y    # Machine-wide across ~/.gemini, ~/.agents
karakuri install --global --all -y      # Machine-wide rules and all skills
```

### 5. `karakuri update [FLAGS]`
Self-updates the running binary by fetching the latest GitHub release, verifying its SHA-256 checksum, and replacing the executable in place:

```bash
# Check whether a newer release exists (no install)
karakuri update --check

# Fetch and install the latest release
karakuri update

# Reinstall even when already current
karakuri update --force
```

Falls back to `cargo install --git` when no prebuilt asset matches the host. Also mirrors the refreshed binary into writable `~/.local/bin` and `~/.cargo/bin` copies, matching `install.sh`.

---

## 📚 Skill Catalog

Karakuri embeds 14 modular, battle-tested skills for AI coding agents:

| Skill | Category | Description | Path |
| :--- | :--- | :--- | :--- |
| **`karakuri`** | `build-tooling` | Instructs AI agents to audit repositories (<400 LOC/file, <60 LOC/fn, 0 clippy warnings), auto-generate/sync living CODEBASE.md semantic indexes, and synchronize skills across agent runtimes. | [`skills/build-tooling/karakuri/SKILL.md`](skills/build-tooling/karakuri/SKILL.md) |
| **`rust-clean-code`** | `build-tooling` | Enforces strict architecture, readability, and scalability for Rust: zero warning suppressions (`#[allow(...)]`), zero dead code, strict limits, role-based hierarchy, and resilient error handling. | [`skills/build-tooling/rust-clean-code/SKILL.md`](skills/build-tooling/rust-clean-code/SKILL.md) |
| **`python-clean-code`** | `build-tooling` | Enforces modern type hints and quality tooling for Python: exclusive use of `uv`, 100% type annotations, automated `ruff` linting and formatting, zero dead code, and role-based hierarchy. | [`skills/build-tooling/python-clean-code/SKILL.md`](skills/build-tooling/python-clean-code/SKILL.md) |
| **`rust-style-python`** | `build-tooling` | Rust/Go-style explicit error handling for Python: typed `Result[T, E]` values instead of surprise exceptions, boundary-only catches, exhaustive matching, error context, and `main` exit-code mapping. | [`skills/build-tooling/rust-style-python/SKILL.md`](skills/build-tooling/rust-style-python/SKILL.md) |
| **`git-release-craft`** | `build-tooling` | Automates the complete Git release lifecycle: conventional commits, semantic version bumps, aesthetic release descriptions, tags, and mandatory CI/CD verification. | [`skills/build-tooling/git-release-craft/SKILL.md`](skills/build-tooling/git-release-craft/SKILL.md) |
| **`git-repo-craft`** | `build-tooling` | Automates GitHub repository metadata auditing: minimal descriptions (<90 chars), curated discoverable topics, clean initialization, and active verification via `gh`. | [`skills/build-tooling/git-repo-craft/SKILL.md`](skills/build-tooling/git-repo-craft/SKILL.md) |
| **`aesthetic-readme-craft`** | `build-tooling` | Crafts stunning, aesthetic READMEs following Praveensenpai's design standard: badge ribbons, ASCII architecture diagrams, alert callouts, emoji feature matrices, and 1-liner installers. | [`skills/build-tooling/aesthetic-readme-craft/SKILL.md`](skills/build-tooling/aesthetic-readme-craft/SKILL.md) |
| **`bash-clean-code`** | `system-ops` | Enforces production standards for Bash: strict error handling (`set -euo pipefail`), zero ShellCheck warnings, temporary file cleanup traps, and XDG compliance. | [`skills/system-ops/bash-clean-code/SKILL.md`](skills/system-ops/bash-clean-code/SKILL.md) |
| **`codebase-digest`** | `build-tooling` | Generates and maintains a high-density, AI-first `CODEBASE.md` index. Gives LLMs full architecture and symbol comprehension in a single file without burning tool calls. | [`skills/build-tooling/codebase-digest/SKILL.md`](skills/build-tooling/codebase-digest/SKILL.md) |
| **`systematic-code-verification`** | `build-tooling` | Eliminates the "fire-and-forget" assumption antipattern. Mandates a zero-assumption policy, a 3-phase verification cycle, and mandatory terminal proof presentation. | [`skills/build-tooling/systematic-code-verification/SKILL.md`](skills/build-tooling/systematic-code-verification/SKILL.md) |
| **`tayori`** | `ai-agents` | Integrates AI coding assistants with Telegram notifications via the `tayori` CLI: interactive approval requests (`tayori ask -i`), completion alerts (`tayori done`), and critical errors (`tayori alert`). | [`skills/ai-agents/tayori/SKILL.md`](skills/ai-agents/tayori/SKILL.md) |
| **`compose-clean-code`** | `mobile-dev` | Enforces architecture, performance, and Material 3 theming for Jetpack Compose: strict limits, two-layer `XRoute`/`XScreen` separation, immutable collections, and zero emojis. | [`skills/mobile-dev/compose-clean-code/SKILL.md`](skills/mobile-dev/compose-clean-code/SKILL.md) |
| **`unslop`** | `writing` | Strips AI tells from written output: removes AI buzzwords (delve, pivotal, tapestry, etc.), em-dash overuse, sycophantic phrasing, and filler. Keeps communication direct and human. | [`skills/writing/unslop/SKILL.md`](skills/writing/unslop/SKILL.md) |

---

## 🛡️ Behavioral Guardrails & Approval Protocol

Karakuri provides a standard behavioral contract in [`rules/RULES.md`](rules/RULES.md) (and [`rules/AGENTS.md`](rules/AGENTS.md)):

- **Explicit Approval Protocol**: Mandatory requirement that AI agents explain **WHY** and **WHAT EFFECT** a change has, and obtain explicit user consent before editing code.
- **Autonomous Error Self-Healing**: Once a task or release is approved, the agent owns compiler errors, clippy warnings, and CI failures in a closed loop until verified green without permission-seeking.
- **Evidence-Based Proof**: Mandates terminal command execution and exit code verification before declaring any task complete.

---

## 📜 License

Licensed under either of [Apache License, Version 2.0](LICENSE) or [MIT License](LICENSE) at your option.  
© Praveen Senpai ([@Praveensenpai](https://github.com/Praveensenpai))
