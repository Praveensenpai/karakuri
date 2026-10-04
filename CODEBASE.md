# CODEBASE.md: karakuri Semantic Digest

> **Notice**: AI-optimized semantic index. Do not write narrative prose. Keep token density high.

## 1. System Topology & Data Flow
```text
Entrypoint ──> CLI/Parser ──> Domain Logic ──> Infra/IO
```

## 2. Global Constraints & Architecture Patterns
- **Primary Language**: Rust 2021 edition
- **Architectural Paradigm**: Role-based (domain/, infra/, api/cli/, tui/)
- **Hard Constraints**: <400 lines/file, <60 lines/fn, zero production unwrap(), 0 warnings.
- **Target Distribution**: Linux x86_64 standalone binary

## 3. Module & Interface Skeleton

### `src/cli/args.rs` (Role: cli, Lines: 180)
- **Responsibility**: Core cli logic in src/cli/args.rs
- **Imports**: use clap :: { Args , Parser , Subcommand } , use std :: path :: PathBuf , use crate :: domain :: { AuditOptions , InstallTarget , InstallationScope } 
- **Types & Enums**:
  ```rust
  pub struct Cli
  pub enum Command
  pub struct InstallArgs
  pub struct AuditArgs
  pub struct DigestArgs
  pub struct SyncArgs
  ```
- **Public Functions & Signatures**:
  ```rust
  fn to_domain_options (& self) -> AuditOptions
  fn resolved_install_args (& self) -> InstallArgs
  fn resolved_scope (& self) -> Option < InstallationScope >
  fn resolved_target (& self) -> Option < InstallTarget >
  ```

### `src/cli.rs` (Role: cli, Lines: 3)
- **Responsibility**: Core cli logic in src/cli.rs
- **Imports**: pub use args :: { AuditArgs , Cli , Command , DigestArgs , InstallArgs , SyncArgs } 

### `src/domain/agent.rs` (Role: domain, Lines: 18)
- **Responsibility**: Core domain logic in src/domain/agent.rs
- **Types & Enums**:
  ```rust
  pub enum SupportedAgent
  ```
- **Public Functions & Signatures**:
  ```rust
  fn display_name (& self) -> & 'static str
  ```

### `src/domain/audit.rs` (Role: domain, Lines: 100)
- **Responsibility**: Core domain logic in src/domain/audit.rs
- **Imports**: use std :: path :: PathBuf 
- **Types & Enums**:
  ```rust
  pub struct AuditOptions
  pub enum ViolationKind
  pub struct Violation
  pub struct AuditReport
  ```
- **Public Functions & Signatures**:
  ```rust
  fn is_clean (& self) -> bool
  ```

### `src/domain/component.rs` (Role: domain, Lines: 32)
- **Responsibility**: Core domain logic in src/domain/component.rs
- **Imports**: use std :: fmt 
- **Types & Enums**:
  ```rust
  pub enum InstallTarget
  ```
- **Public Functions & Signatures**:
  ```rust
  fn description (& self) -> & 'static str
  fn includes_rules (& self) -> bool
  fn includes_skills (& self) -> bool
  ```

### `src/domain/digest.rs` (Role: domain, Lines: 152)
- **Responsibility**: Core domain logic in src/domain/digest.rs
- **Imports**: use std :: path :: PathBuf 
- **Types & Enums**:
  ```rust
  pub enum ModuleRole
  pub struct ModuleDigest
  pub struct CodebaseDigest
  ```
- **Public Functions & Signatures**:
  ```rust
  fn as_str (& self) -> & 'static str
  fn infer_from_path (path : & str) -> Self
  fn render_markdown (& self) -> String
  ```

### `src/domain/scope.rs` (Role: domain, Lines: 24)
- **Responsibility**: Core domain logic in src/domain/scope.rs
- **Imports**: use std :: fmt 
- **Types & Enums**:
  ```rust
  pub enum InstallationScope
  ```
- **Public Functions & Signatures**:
  ```rust
  fn description (& self) -> & 'static str
  ```

### `src/domain/sync.rs` (Role: domain, Lines: 38)
- **Responsibility**: Core domain logic in src/domain/sync.rs
- **Imports**: use std :: path :: PathBuf 
- **Types & Enums**:
  ```rust
  pub struct AgentRuntime
  pub enum SyncStatus
  pub struct SkillSyncRecord
  pub struct SyncReport
  ```
- **Public Functions & Signatures**:
  ```rust
  fn new (name : & 'static str , skills_dir : PathBuf) -> Self
  ```

### `src/domain.rs` (Role: domain, Lines: 12)
- **Responsibility**: Core domain logic in src/domain.rs
- **Imports**: pub use agent :: SupportedAgent , pub use audit :: AuditOptions , pub use component :: InstallTarget , pub use digest :: ModuleRole , pub use scope :: InstallationScope 

### `src/error.rs` (Role: general, Lines: 23)
- **Responsibility**: Core general logic in src/error.rs
- **Imports**: use std :: path :: PathBuf , use thiserror :: Error 
- **Types & Enums**:
  ```rust
  pub enum KarakuriError
  ```

### `src/infra/auditor.rs` (Role: infra, Lines: 124)
- **Responsibility**: Core infra logic in src/infra/auditor.rs
- **Imports**: use std :: fs , use std :: path :: Path , use std :: process :: Command , use walkdir :: { DirEntry , WalkDir } , use crate :: domain :: audit :: { AuditOptions , AuditReport , Violation , ViolationKind } , use crate :: error :: Result , use crate :: infra :: rust_analyzer :: RustAnalyzer 
- **Public Functions & Signatures**:
  ```rust
  fn run_audit (opts : & AuditOptions) -> Result < AuditReport >
  ```

### `src/infra/digest_builder.rs` (Role: infra, Lines: 112)
- **Responsibility**: Core infra logic in src/infra/digest_builder.rs
- **Imports**: use std :: fs , use std :: path :: Path , use walkdir :: WalkDir , use crate :: domain :: digest :: { CodebaseDigest , ModuleDigest } , use crate :: error :: { KarakuriError , Result } , use crate :: infra :: rust_analyzer :: RustAnalyzer 
- **Types & Enums**:
  ```rust
  pub struct DigestBuilder
  ```
- **Public Functions & Signatures**:
  ```rust
  fn build (dir : & Path) -> Result < CodebaseDigest >
  fn write_to_file (dir : & Path , digest : & CodebaseDigest) -> Result < () >
  ```

### `src/infra/embedded.rs` (Role: infra, Lines: 62)
- **Responsibility**: Core infra logic in src/infra/embedded.rs
- **Types & Enums**:
  ```rust
  pub struct SkillAsset
  ```

### `src/infra/installer.rs` (Role: infra, Lines: 137)
- **Responsibility**: Core infra logic in src/infra/installer.rs
- **Imports**: use std :: fs , use std :: path :: { Path , PathBuf } , use crate :: domain :: { InstallTarget , InstallationScope } , use crate :: error :: { KarakuriError , Result } , use crate :: infra :: embedded :: { AGENTS_MD , EMBEDDED_SKILLS , RULES_MD } 
- **Types & Enums**:
  ```rust
  pub struct InstalledRecord
  ```
- **Public Functions & Signatures**:
  ```rust
  fn install (scope : InstallationScope , target : InstallTarget) -> Result < Vec < InstalledRecord > >
  ```

### `src/infra/rust_analyzer.rs` (Role: infra, Lines: 302)
- **Responsibility**: Core infra logic in src/infra/rust_analyzer.rs
- **Imports**: use std :: path :: Path , use syn :: spanned :: Spanned , use syn :: { Expr , Item , ItemFn , ItemImpl , Stmt } , use crate :: domain :: audit :: { Violation , ViolationKind } , use crate :: domain :: digest :: ModuleDigest , use crate :: domain :: ModuleRole 
- **Types & Enums**:
  ```rust
  pub struct RustAnalyzer
  ```
- **Public Functions & Signatures**:
  ```rust
  fn audit_source (path : & Path , content : & str , max_fn_lines : usize , max_nesting_depth : usize ,) -> Vec < Violation >
  fn extract_digest (path : & Path , content : & str) -> ModuleDigest
  ```

### `src/infra/sync_engine.rs` (Role: infra, Lines: 132)
- **Responsibility**: Core infra logic in src/infra/sync_engine.rs
- **Imports**: use std :: fs , use std :: path :: Path , use crate :: domain :: sync :: { AgentRuntime , SkillSyncRecord , SyncReport , SyncStatus } , use crate :: error :: { KarakuriError , Result } 
- **Types & Enums**:
  ```rust
  pub struct SyncEngine
  ```
- **Public Functions & Signatures**:
  ```rust
  fn run () -> Result < SyncReport >
  ```

### `src/infra.rs` (Role: infra, Lines: 11)
- **Responsibility**: Core infra logic in src/infra.rs
- **Imports**: pub use auditor :: run_audit , pub use digest_builder :: DigestBuilder , pub use installer :: install , pub use sync_engine :: SyncEngine 

### `src/main.rs` (Role: general, Lines: 116)
- **Responsibility**: Core general logic in src/main.rs
- **Imports**: use clap :: Parser , use colored :: Colorize , use std :: process :: exit , use cli :: { AuditArgs , Cli , Command , DigestArgs , InstallArgs , SyncArgs } , use error :: Result 

### `src/tui/banner.rs` (Role: tui, Lines: 67)
- **Responsibility**: Core tui logic in src/tui/banner.rs
- **Imports**: use colored :: Colorize , use crate :: domain :: SupportedAgent 
- **Public Functions & Signatures**:
  ```rust
  fn print_banner ()
  fn print_header_info ()
  ```

### `src/tui/cards.rs` (Role: tui, Lines: 233)
- **Responsibility**: Core tui logic in src/tui/cards.rs
- **Imports**: use colored :: Colorize , use crate :: domain :: { InstallTarget , InstallationScope } , use crate :: infra :: installer :: InstalledRecord 
- **Public Functions & Signatures**:
  ```rust
  fn print_summary_card (scope : InstallationScope , target : InstallTarget)
  fn print_security_card ()
  fn print_success_box (records : & [InstalledRecord])
  ```

### `src/tui/prompts.rs` (Role: tui, Lines: 193)
- **Responsibility**: Core tui logic in src/tui/prompts.rs
- **Imports**: use std :: fmt :: Display , use std :: io :: { stdout , Write } , use colored :: Colorize , use crossterm :: { cursor , event :: { self , Event , KeyCode , KeyEventKind , KeyModifiers } , execute , terminal :: { self , ClearType } , } , use crate :: domain :: { InstallTarget , InstallationScope } , use crate :: error :: { KarakuriError , Result } 
- **Public Functions & Signatures**:
  ```rust
  fn select_menu < T : Display > (symbol : & str , title : & str , options : & [T] , default_idx : usize ,) -> Result < usize >
  fn select_scope () -> Result < InstallationScope >
  fn select_target () -> Result < InstallTarget >
  fn confirm_installation () -> Result < bool >
  ```

### `src/tui/reports.rs` (Role: tui, Lines: 197)
- **Responsibility**: Core tui logic in src/tui/reports.rs
- **Imports**: use colored :: Colorize , use std :: path :: Path , use crate :: domain :: audit :: { AuditReport , ViolationKind } , use crate :: domain :: digest :: CodebaseDigest , use crate :: domain :: sync :: { SyncReport , SyncStatus } 
- **Public Functions & Signatures**:
  ```rust
  fn print_audit_report (report : & AuditReport)
  fn print_digest_success (digest : & CodebaseDigest , path : & Path)
  fn print_sync_report (report : & SyncReport)
  ```

### `src/tui.rs` (Role: tui, Lines: 9)
- **Responsibility**: Core tui logic in src/tui.rs
- **Imports**: pub use banner :: { print_banner , print_header_info } , pub use cards :: { print_security_card , print_success_box , print_summary_card } , pub use prompts :: { confirm_installation , select_scope , select_target } , pub use reports :: { print_audit_report , print_digest_success , print_sync_report } 

## 4. Execution Lifecycle Trace
1. **Startup**: Entrypoint parses CLI flags & dispatches command.
2. **Execution**: Core domain logic processes inputs and evaluates rules.
3. **Persistence / I/O**: Domain logic calls infra for disk/terminal I/O.
4. **Exit**: Graceful termination with standard exit codes.

## 5. Verification Commands
```bash
cargo build --release --target x86_64-unknown-linux-gnu
cargo test --all-targets
cargo clippy --all-targets -- -D warnings && cargo fmt --check
```
