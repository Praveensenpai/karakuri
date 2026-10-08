use colored::Colorize;
use std::path::Path;

use crate::domain::audit::{AuditReport, ViolationKind};
use crate::domain::digest::CodebaseDigest;
use crate::domain::sync::{SyncReport, SyncStatus};
use crate::domain::UpdateOutcome;

/// Prints a colorized, aesthetic audit report.
pub fn print_audit_report(report: &AuditReport) {
    println!(
        "\n  {}",
        "─── Codebase Audit Report ───".bright_cyan().bold()
    );
    println!(
        "  Files Scanned: {}   Lines Analyzed: {}",
        report.files_scanned.to_string().bright_yellow(),
        report.lines_scanned.to_string().bright_yellow()
    );
    println!();

    if report.is_clean() {
        println!(
            "  {} {}",
            "✔".bright_green().bold(),
            "All clean-code rules and documentation standards satisfied!"
                .bright_green()
                .bold()
        );
        println!();
        return;
    }

    println!(
        "  {} Found {} violation(s):\n",
        "✘".bright_red().bold(),
        report.violations.len().to_string().bright_red().bold()
    );

    for (idx, v) in report.violations.iter().enumerate() {
        print_violation(idx, v);
    }
    println!();
}

fn print_violation(idx: usize, v: &crate::domain::audit::Violation) {
    let num = format!("[{}]", idx + 1).bright_black();
    if !print_structural_violation(&v.kind, &v.path, &num) {
        print_meta_violation(&v.kind, &v.path, &num);
    }
}

fn print_structural_violation(
    kind: &ViolationKind,
    path: &Path,
    num: &colored::ColoredString,
) -> bool {
    match kind {
        ViolationKind::FileTooLong { lines, limit } => {
            println!(
                "  {} {} File exceeds limit ({} > {} lines)",
                num,
                path.display().to_string().bold(),
                lines.to_string().bright_red(),
                limit
            );
            true
        }
        ViolationKind::FunctionTooLong {
            name,
            lines,
            limit,
            line_no,
        } => {
            println!(
                "  {} {}:{} Function `{}` exceeds limit ({} > {} lines)",
                num,
                path.display(),
                line_no,
                name.bright_yellow(),
                lines.to_string().bright_red(),
                limit
            );
            true
        }
        ViolationKind::NestingTooDeep {
            name,
            depth,
            limit,
            line_no,
        } => {
            println!(
                "  {} {}:{} Function `{}` nesting too deep (depth {} > limit {})",
                num,
                path.display(),
                line_no,
                name.bright_yellow(),
                depth.to_string().bright_red(),
                limit
            );
            true
        }
        _ => false,
    }
}

fn print_meta_violation(kind: &ViolationKind, path: &Path, num: &colored::ColoredString) {
    match kind {
        ViolationKind::ForbiddenPattern { pattern, line_no } => {
            println!(
                "  {} {}:{} Forbidden pattern detected: `{}`",
                num,
                path.display(),
                line_no,
                pattern.bright_red()
            );
        }
        ViolationKind::MissingCodebaseMd => {
            println!(
                "  {} {} Missing required AI-first living index (CODEBASE.md)",
                num,
                path.display().to_string().bright_yellow().bold()
            );
        }
        ViolationKind::MissingReadmeMd => {
            println!(
                "  {} {} Missing required README.md documentation",
                num,
                path.display().to_string().bright_red().bold()
            );
        }
        ViolationKind::ClippyWarning(msg) => {
            println!("  {} Clippy warning: {}", num, msg.bright_yellow());
        }
        _ => {}
    }
}

/// Prints completion summary for CODEBASE.md digest generation.
pub fn print_digest_success(digest: &CodebaseDigest, path: &Path) {
    println!(
        "\n  {}",
        "─── Codebase Digest Generated ───".bright_green().bold()
    );
    println!(
        "  Target File: {}",
        path.display().to_string().bright_cyan()
    );
    println!(
        "  Project Name: {}",
        digest.project_name.bright_white().bold()
    );
    println!(
        "  Primary Stack: {}",
        digest.primary_language.bright_yellow()
    );
    println!(
        "  Indexed Modules: {}",
        digest.modules.len().to_string().bright_cyan().bold()
    );
    println!(
        "\n  {} {}\n",
        "✔".bright_green().bold(),
        "CODEBASE.md successfully synchronized!".bright_green()
    );
}

/// Prints the outcome of a self-update run.
pub fn print_update_report(outcome: &UpdateOutcome) {
    println!("\n  {}", "─── Self-Update Report ───".bright_cyan().bold());

    match outcome {
        UpdateOutcome::UpToDate { current, latest } => {
            println!(
                "  Current: {}   Latest: {}",
                current.bright_white(),
                latest.bright_white()
            );
            println!(
                "\n  {} {}\n",
                "✔".bright_green().bold(),
                "Already running the latest release.".bright_green()
            );
        }
        UpdateOutcome::Available { current, latest } => {
            println!(
                "  Current: {}   Latest: {}",
                current.bright_white(),
                latest.bright_yellow().bold()
            );
            println!(
                "\n  {} {}\n",
                "↑".bright_yellow().bold(),
                "A newer release is available. Run `karakuri update` to install it."
                    .bright_yellow()
            );
        }
        UpdateOutcome::Updated {
            from,
            to,
            path,
            skills_refreshed,
        } => {
            println!("  {} → {}", from.bright_white(), to.bright_green().bold());
            println!("  Path: {}", path.display().to_string().bright_cyan());
            print_refresh_note(*skills_refreshed);
            println!(
                "\n  {} {}\n",
                "✔".bright_green().bold(),
                "Karakuri updated successfully.".bright_green().bold()
            );
        }
        UpdateOutcome::BuiltFromSource {
            to,
            skills_refreshed,
        } => {
            println!("  Installed: {}", to.bright_green().bold());
            print_refresh_note(*skills_refreshed);
            println!(
                "\n  {} {}\n",
                "✔".bright_green().bold(),
                "Compiled and installed from source via cargo.".bright_green()
            );
        }
    }
}

/// Prints whether the post-update skill re-extraction succeeded.
fn print_refresh_note(refreshed: bool) {
    if refreshed {
        println!(
            "  {} {}",
            "↻".bright_cyan().bold(),
            "Embedded skills re-extracted and synced across agents.".bright_cyan()
        );
    } else {
        println!(
            "  {} {}",
            "!".bright_yellow().bold(),
            "Binary updated, but skill refresh failed. Run `karakuri install --global --all -y` then `karakuri sync`."
                .bright_yellow()
        );
    }
}

/// Prints the outcome of a stack-aware `ensure` reconciliation.
pub fn print_ensure_report(report: &crate::infra::EnsureReport) {
    println!(
        "\n  {}",
        "─── Stack-Aware Skill Ensure ───".bright_cyan().bold()
    );
    println!(
        "  Detected Stacks: {}   Matched Skills: {}\n",
        report.stacks.label().bright_yellow(),
        report.selected.len().to_string().bright_yellow()
    );

    for name in &report.written {
        println!(
            "  {} {} {} ──> {}",
            "[INSTALLED]".bright_green(),
            "✔".bright_green().bold(),
            name.bright_cyan(),
            ".agents/skills/".bright_black()
        );
    }
    for name in &report.up_to_date {
        println!("  {} {}", "[OK]".bright_black(), name.bright_white());
    }

    if report.changed() {
        println!(
            "\n  {} {}\n",
            "✔".bright_green().bold(),
            "Project skills reconciled to the detected stack.".bright_green()
        );
    } else {
        println!(
            "\n  {} {}\n",
            "✔".bright_green().bold(),
            "All matching skills already up to date.".bright_green()
        );
    }
}

pub fn print_sync_report(report: &SyncReport) {
    println!(
        "\n  {}",
        "─── Skill Synchronization Report ───".bright_cyan().bold()
    );
    println!(
        "  Detected Runtimes: {}   Available Skills: {}\n",
        report.runtimes_detected.to_string().bright_yellow(),
        report.skills_synced.to_string().bright_yellow()
    );

    for rec in &report.records {
        let status_badge = match rec.status {
            SyncStatus::Created => "[CREATED]".bright_green(),
            SyncStatus::Updated => "[UPDATED]".bright_yellow(),
            SyncStatus::UpToDate => "[OK]".bright_black(),
        };
        println!(
            "  {} {:<28} ──> {}",
            status_badge,
            rec.runtime_name.bright_white(),
            rec.skill_name.bright_cyan()
        );
    }
    println!(
        "\n  {} {}\n",
        "✔".bright_green().bold(),
        "Skills synchronized across all agent runtimes.".bright_green()
    );
}
