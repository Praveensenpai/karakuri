use clap::Parser;

use crate::domain::{InstallTarget, InstallationScope};

#[derive(Parser, Debug)]
#[command(
    name = "karakuri",
    author = "Praveen Senpai <pvnt20@gmail.com>",
    version,
    about = "Modular AI Agent Skills & Behavioral Guardrails for Antigravity, Antigravity CLI, and OpenCode",
    long_about = None
)]
pub struct CliArgs {
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

impl CliArgs {
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
