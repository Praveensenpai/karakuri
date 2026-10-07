use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::domain::{AuditOptions, InstallTarget, InstallationScope, UpdateOptions};

#[derive(Parser, Debug)]
#[command(
    name = "karakuri",
    author = "Praveen Senpai <pvnt20@gmail.com>",
    version,
    about = "AI Codebase & Skill Repository Orchestrator for Antigravity, OpenCode, and Claude",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Install in current directory (committed with project)
    #[arg(short = 'p', long = "project")]
    pub project: bool,

    /// Install globally across ~/.gemini, ~/.agents, ~/.opencode
    #[arg(short = 'g', long = "global")]
    pub global: bool,

    /// Install rules & guardrails only (default)
    #[arg(short = 'r', long = "rules")]
    pub rules: bool,

    /// Install modular skills only
    #[arg(short = 's', long = "skills")]
    pub skills: bool,

    /// Install rules and all modular skills
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Skip confirmation prompt
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Interactive or automated installer for skills & guardrails
    Install(InstallArgs),
    /// Audit repositories against clean-code rules & documentation standards
    Audit(AuditArgs),
    /// Generate or update an AI-first CODEBASE.md semantic digest
    Digest(DigestArgs),
    /// Synchronize skills across all detected agent runtimes
    Sync(SyncArgs),
    /// Fetch and install the latest release binary
    Update(UpdateArgs),
}

#[derive(Args, Debug, Default, Clone)]
pub struct InstallArgs {
    /// Install in current directory (committed with project)
    #[arg(short = 'p', long = "project")]
    pub project: bool,

    /// Install globally across ~/.gemini, ~/.agents, ~/.opencode
    #[arg(short = 'g', long = "global")]
    pub global: bool,

    /// Install rules & guardrails only
    #[arg(short = 'r', long = "rules")]
    pub rules: bool,

    /// Install modular skills only
    #[arg(short = 's', long = "skills")]
    pub skills: bool,

    /// Install rules and all modular skills
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Skip confirmation prompt
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,
}

#[derive(Args, Debug, Clone)]
pub struct AuditArgs {
    /// Target directory to audit (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum lines allowed per file
    #[arg(long, default_value = "400")]
    pub max_file_lines: usize,

    /// Maximum lines allowed per function
    #[arg(long, default_value = "60")]
    pub max_fn_lines: usize,

    /// Maximum block nesting depth allowed
    #[arg(long, default_value = "3")]
    pub max_depth: usize,

    /// Skip cargo clippy execution
    #[arg(long)]
    pub no_clippy: bool,

    /// Skip checking for CODEBASE.md presence
    #[arg(long)]
    pub no_codebase: bool,

    /// Skip checking for README.md presence
    #[arg(long)]
    pub no_readme: bool,
}

impl AuditArgs {
    pub fn to_domain_options(&self) -> AuditOptions {
        AuditOptions {
            path: self.path.clone(),
            max_file_lines: self.max_file_lines,
            max_fn_lines: self.max_fn_lines,
            max_nesting_depth: self.max_depth,
            check_clippy: !self.no_clippy,
            check_codebase_md: !self.no_codebase,
            check_readme_md: !self.no_readme,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct DigestArgs {
    /// Target project directory (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output markdown to stdout instead of writing to CODEBASE.md
    #[arg(long)]
    pub stdout: bool,
}

#[derive(Args, Debug, Default, Clone)]
pub struct SyncArgs {
    /// Perform a dry-run without writing any files
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args, Debug, Default, Clone)]
pub struct UpdateArgs {
    /// Report whether an update is available without installing it
    #[arg(long)]
    pub check: bool,

    /// Reinstall even when already on the latest release
    #[arg(long)]
    pub force: bool,
}

impl UpdateArgs {
    pub fn to_domain_options(&self) -> UpdateOptions {
        UpdateOptions {
            check_only: self.check,
            force: self.force,
        }
    }
}

impl Cli {
    pub fn resolved_install_args(&self) -> InstallArgs {
        InstallArgs {
            project: self.project,
            global: self.global,
            rules: self.rules,
            skills: self.skills,
            all: self.all,
            yes: self.yes,
        }
    }
}

impl InstallArgs {
    pub fn resolved_scope(&self) -> Option<InstallationScope> {
        if self.project {
            Some(InstallationScope::Project)
        } else if self.global {
            Some(InstallationScope::Global)
        } else {
            None
        }
    }

    pub fn resolved_target(&self) -> Option<InstallTarget> {
        if self.all {
            Some(InstallTarget::All)
        } else if self.skills {
            Some(InstallTarget::Skills)
        } else if self.rules {
            Some(InstallTarget::Rules)
        } else {
            None
        }
    }
}
