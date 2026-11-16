use clap::Parser;
use colored::Colorize;
use std::path::Path;
use std::process::exit;

mod cli;
mod domain;
mod error;
mod infra;
mod tui;

use cli::{AuditArgs, Cli, Command, DigestArgs, EnsureArgs, InstallArgs, SyncArgs, UpdateArgs};
use domain::InstallationScope;
use error::Result;

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Install(args)) => handle_install(args),
        Some(Command::Audit(args)) => handle_audit(args),
        Some(Command::Digest(args)) => handle_digest(args),
        Some(Command::Sync(args)) => handle_sync(args),
        Some(Command::Update(args)) => handle_update(args),
        Some(Command::Ensure(args)) => handle_ensure(args),
        None => {
            let args = cli.resolved_install_args();
            handle_install(args)
        }
    }
}

fn handle_install(args: InstallArgs) -> Result<()> {
    tui::print_banner();
    tui::print_header_info();

    let scope = match args.resolved_scope() {
        Some(s) => {
            println!("  \u{25c7}  {}", "Installation scope".bold());
            println!("  \u{2502}  {}", s.description().bright_cyan());
            println!("  \u{2502}");
            s
        }
        None => tui::select_scope()?,
    };

    let target = match args.resolved_target() {
        Some(t) => {
            println!("  \u{25c7}  {}", "Components".bold());
            println!("  \u{2502}  {}", t.description().bright_cyan());
            println!("  \u{2502}");
            t
        }
        None => tui::select_target()?,
    };

    let detected = match scope {
        InstallationScope::Project => infra::detector::detect(Path::new(".")),
        InstallationScope::Global => domain::StackSet::new(),
    };
    let filter = args.resolved_filter();
    let skills = infra::select_skills(&filter, &detected);
    let active = filter.active_stacks(&detected);

    tui::print_summary_card(scope, target, skills.len());
    tui::print_security_card();

    if !args.yes {
        let confirmed = tui::confirm_installation()?;
        if !confirmed {
            println!(
                "  \u{25c7}  {}\n",
                "Installation cancelled by user.".bright_yellow()
            );
            return Ok(());
        }
    }

    println!(
        "  \u{25c7}  {} {}",
        "Selected skills:".bold(),
        skills.len().to_string().bright_cyan().bold()
    );
    println!("  \u{2502}  {}", format_stacks(&active).bright_black());
    println!("  \u{2502}");

    let records = infra::install(scope, target, &skills)?;
    tui::print_success_box(&records);
    Ok(())
}

fn handle_ensure(args: EnsureArgs) -> Result<()> {
    println!(
        "  \u{25c7}  Ensuring skills for: {}",
        args.path.display().to_string().bright_cyan()
    );
    let report = infra::ensure(&args.path)?;
    tui::print_ensure_report(&report);
    Ok(())
}

fn format_stacks(set: &domain::StackSet) -> String {
    if set.is_empty() {
        "no stacks detected".to_string()
    } else {
        format!("stacks: {}", set.label())
    }
}

fn handle_audit(args: AuditArgs) -> Result<()> {
    let opts = args.to_domain_options();
    println!(
        "  \u{25c7}  Auditing codebase at: {}",
        opts.path.display().to_string().bright_cyan()
    );
    let report = infra::run_audit(&opts)?;
    tui::print_audit_report(&report);

    if !report.is_clean() {
        exit(1);
    }
    Ok(())
}

fn handle_digest(args: DigestArgs) -> Result<()> {
    println!(
        "  \u{25c7}  Analyzing codebase at: {}",
        args.path.display().to_string().bright_cyan()
    );
    let digest = infra::DigestBuilder::build(&args.path)?;

    if args.stdout {
        println!("{}", digest.render_markdown());
    } else {
        infra::DigestBuilder::write_to_file(&args.path, &digest)?;
        let target_file = args.path.join("CODEBASE.md");
        tui::print_digest_success(&digest, &target_file);
    }
    Ok(())
}

fn handle_sync(_args: SyncArgs) -> Result<()> {
    println!("  \u{25c7}  Synchronizing skills across agent runtimes...");
    let report = infra::SyncEngine::run()?;
    tui::print_sync_report(&report);
    Ok(())
}

fn handle_update(args: UpdateArgs) -> Result<()> {
    println!("  \u{25c7}  Checking for the latest Karakuri release...");
    let outcome = infra::Updater::run(&args.to_domain_options())?;
    tui::print_update_report(&outcome);
    Ok(())
}

fn maybe_daily_update_check() {
    // Only run once per calendar day.
    let today = today_stamp();
    if today.is_empty() {
        return;
    }
    let marker = match dirs::home_dir() {
        Some(h) => h.join(".karakuri").join("last_update_check"),
        None => return,
    };
    if let Ok(last) = std::fs::read_to_string(&marker) {
        if last.trim() == today {
            return;
        }
    }

    let current = env!("CARGO_PKG_VERSION").to_string();
    let tag = match infra::Updater::latest_tag() {
        Ok(t) => t,
        Err(_) => return, // network failure: retry next run
    };
    let latest = tag.trim_start_matches('v').to_string();
    if latest == current {
        let _ = write_marker(&marker, &today);
        return;
    }

    println!(
        "  \u{25c7}  {} {} \u{2192} {}",
        "New Karakuri version available:".bold(),
        current.bright_cyan(),
        latest.bright_cyan().bold()
    );
    println!("  \u{2502}  Updating automatically...");

    let opts = domain::UpdateOptions {
        check_only: false,
        force: false,
    };
    match infra::Updater::run(&opts) {
        Ok(outcome) => {
            tui::print_update_report(&outcome);
        }
        Err(err) => {
            println!("  {}  {}", "Update failed:".bright_red().bold(), err);
        }
    }
    let _ = write_marker(&marker, &today);
}

fn write_marker(path: &std::path::Path, today: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, today)
}

fn today_stamp() -> String {
    std::process::Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn main() {
    maybe_daily_update_check();
    if let Err(err) = run() {
        eprintln!("  {}  {}", "Error:".bright_red().bold(), err);
        exit(1);
    }
}
