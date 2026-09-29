# CODEBASE.md: Karakuri Semantic Digest

> **Notice**: AI-optimized semantic index. Do not write narrative prose. Keep token density high.

## 1. System Topology & Data Flow
```text
CLI (args.rs) ──> Dispatcher (main.rs) ──> Domain Logic (agent, scope, component) 
                                       └──> Infra (installer.rs ──> embedded.rs assets)
                                       └──> TUI (banner, cards, prompts)
```

## 2. Global Constraints & Architecture Patterns
- **Language**: Rust 2021 edition.
- **Architecture**: Role-based (`domain/`, `infra/`, `cli/`, `tui/`, `error.rs`). Modern module style (`foo.rs` + `foo/`).
- **Hard Limits**: <400 lines/file, <60 lines/fn, zero production `unwrap()`/`expect()`.
- **Target Distribution**: Standalone Linux x86_64 binary (`x86_64-unknown-linux-gnu`) via GitHub Releases.

## 3. Module & Interface Skeleton

### `src/main.rs` (Role: Entrypoint, Lines: 86)
- **Responsibility**: Parses CLI arguments and routes execution to interactive TUI or headless installation.
- **Imports**: `clap::Parser`, `crate::cli::args::*`, `crate::infra::installer::*`, `crate::tui::*`, `crate::error::Result`.
- **Public Functions**: `fn main() -> Result<()>`
- **Consumers**: OS process execution.
- **Side Effects**: Reads CLI arguments, invokes installer, prints to stdout/stderr.

### `src/error.rs` (Role: Error Handling, Lines: 36)
- **Responsibility**: Centralized domain error definitions and Result alias.
- **Imports**: `thiserror::Error`, `std::io`.
- **Types & Enums**:
  ```rust
  pub enum KarakuriError { Io(std::io::Error), Config(String), UserCancelled, Interrupted }
  pub type Result<T> = std::result::Result<T, KarakuriError>;
  ```
- **Consumers**: All modules (`main`, `infra`, `tui`, `cli`).

### `src/cli/args.rs` (Role: CLI, Lines: 44)
- **Responsibility**: Clap-derived command line argument parser schema.
- **Types & Enums**:
  ```rust
  pub struct CliArgs {
      pub global: bool, pub project: bool, pub rules: bool,
      pub skills: bool, pub all: bool, pub yes: bool,
  }
  ```
- **Consumers**: `src/main.rs`.

### `src/domain/agent.rs` (Role: Domain, Lines: 36)
- **Responsibility**: Supported AI agent target platform metadata.
- **Types & Enums**:
  ```rust
  pub enum Agent { Antigravity, AntigravityCli, OpenCode }
  impl Agent { pub fn all() -> &'static [Agent]; pub fn name(&self) -> &'static str; pub fn config_dir(&self) -> &'static str; }
  ```
- **Consumers**: `src/infra/installer.rs`.

### `src/domain/scope.rs` (Role: Domain, Lines: 24)
- **Responsibility**: Target configuration scope (Global vs Project-level).
- **Types & Enums**:
  ```rust
  pub enum TargetScope { Global, Project }
  ```
- **Consumers**: `src/main.rs`, `src/infra/installer.rs`.

### `src/domain/component.rs` (Role: Domain, Lines: 24)
- **Responsibility**: Selectable asset components (Rules, Skills, All).
- **Types & Enums**:
  ```rust
  pub enum InstallComponent { Rules, Skills, All }
  ```
- **Consumers**: `src/main.rs`, `src/tui/prompts.rs`.

### `src/infra/embedded.rs` (Role: Infra / Assets, Lines: 50)
- **Responsibility**: Compile-time embedded skills and rule files via `include_str!`.
- **Types & Enums**:
  ```rust
  pub struct SkillAsset { pub name: &'static str, pub content: &'static str }
  pub const AGENTS_MD: &str;
  pub const RULES_MD: &str;
  pub const EMBEDDED_SKILLS: &[SkillAsset]; // 10 modular skills
  ```
- **Consumers**: `src/infra/installer.rs`.

### `src/infra/installer.rs` (Role: Infra / File Operations, Lines: 145)
- **Responsibility**: Writes embedded rules and skills to target filesystem directories (`~/.gemini/config/`, `~/.agents/`, `.agents/`).
- **Public Functions**:
  ```rust
  pub fn install_global(component: InstallComponent) -> Result<()>;
  pub fn install_project(component: InstallComponent) -> Result<()>;
  ```
- **Consumers**: `src/main.rs`.
- **Side Effects**: Filesystem directory creation and file writes.

### `src/tui/banner.rs` (Role: TUI, Lines: 28)
- **Responsibility**: ASCII art and stylized terminal banner printing.
- **Public Functions**: `pub fn print_banner()`, `pub fn print_success(msg: &str)`
- **Consumers**: `src/main.rs`.

### `src/tui/cards.rs` (Role: TUI, Lines: 48)
- **Responsibility**: Stylized card layout printing for installed items.
- **Public Functions**: `pub fn print_summary_card(title: &str, items: &[&str])`
- **Consumers**: `src/main.rs`.

### `src/tui/prompts.rs` (Role: TUI, Lines: 56)
- **Responsibility**: Interactive terminal prompt for interactive configuration.
- **Public Functions**: `pub fn prompt_interactive() -> Result<(TargetScope, InstallComponent)>`
- **Consumers**: `src/main.rs`.

## 4. Execution Lifecycle Trace
1. `src/main.rs::main()` invoked.
2. `CliArgs::parse()` evaluates command-line flags.
3. If flags are provided: Headless installation via `installer::install_global()` or `install_project()`.
4. If no flags provided: `banner::print_banner()` displays TUI, `prompts::prompt_interactive()` collects scope and components.
5. `installer::install_*` writes compile-time embedded assets from `embedded.rs` to disk.
6. `cards::print_summary_card()` renders confirmation card; process exits with code 0.

## 5. Verification Commands
```bash
cargo build --release --target x86_64-unknown-linux-gnu
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

- **2026-09-29**: Fixed `install.sh` target resolution bug that caused version shadowing when upgrading. `install.sh` now resolves `INSTALL_DIR` to the active `CURRENT_BIN` location or prioritized `$PATH` entry, and automatically synchronizes secondary binaries across `~/.local/bin` and `~/.cargo/bin`.
- **2026-09-29**: Enhanced `install.sh` bootstrap installer with aesthetic stderr version and progress logging (`log_info`, `log_ok`). Added version indicators for fresh installation, upgrades, and up-to-date status before launching Karakuri.
- **2026-09-28**: Added `compose-clean-code` skill under `skills/mobile-dev/compose-clean-code/SKILL.md` (Material 3, API 33+, zero-emoji policy, Material theme icons, type-safe navigation), embedded in `src/infra/embedded.rs`, updated Jetpack Compose engineering standards across `rules/RULES.md`, `rules/AGENTS.md`, `AGENTS.md`, and `GEMINI.md`, updated `README.md` catalog (10 modular skills), and bumped version to `0.2.11`.
- **2026-09-23**: Added `systematic-code-verification` skill, embedded in `embedded.rs`, updated behavioral rules and README, bumped version to `0.2.10`.
- **2026-09-18**: Added `tayori` AI-agent notification and alert skill to `skills/ai-agents/tayori/SKILL.md` and registered it in `src/infra/embedded.rs`. Bumped version to `v0.2.9`.
- **2026-09-16**: Fixed `install.sh` bootstrap script where piping via `curl -fsSL ... | bash` was interrupted by early `/dev/tty` stdin redirection. Encapsulated installer logic in `main()` and deferred `/dev/tty` reconnection until binary execution.
- **2026-09-13**: Added `codebase-digest` skill, registered in `embedded.rs`, updated rules and README, streamlined release to Linux x86_64.
