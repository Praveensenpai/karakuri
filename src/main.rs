use clap::Parser;
use colored::Colorize;
use std::process::exit;

mod cli;
mod domain;
mod error;
mod infra;
mod tui;

use cli::{AuditArgs, Cli, Command, DigestArgs, InstallArgs, SyncArgs, UpdateArgs};
use error::Result;

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Install(args)) => handle_install(args),
        Some(Command::Audit(args)) => handle_audit(args),
        Some(Command::Digest(args)) => handle_digest(args),
        Some(Command::Sync(args)) => handle_sync(args),
        Some(Command::Update(args)) => handle_update(args),
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
            println!("  ◇  {}", "Installation scope".bold());
            println!("  │  {}", s.description().bright_cyan());
            println!("  │");
            s
        }
        None => tui::select_scope()?,
    };

    let target = match args.resolved_target() {
        Some(t) => {
            println!("  ◇  {}", "Components".bold());
            println!("  │  {}", t.description().bright_cyan());
            println!("  │");
            t
        }
        None => tui::select_target()?,
    };

    tui::print_summary_card(scope, target);
    tui::print_security_card();

    if !args.yes {
        let confirmed = tui::confirm_installation()?;
        if !confirmed {
            println!(
                "  ◇  {}\n",
                "Installation cancelled by user.".bright_yellow()
            );
            return Ok(());
        }
    }

    let records = infra::install(scope, target)?;
    tui::print_success_box(&records);
    Ok(())
}

fn handle_audit(args: AuditArgs) -> Result<()> {
    let opts = args.to_domain_options();
    println!(
        "  ◇  Auditing codebase at: {}",
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
        "  ◇  Analyzing codebase at: {}",
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
    println!("  ◇  Synchronizing skills across agent runtimes...");
    let report = infra::SyncEngine::run()?;
    tui::print_sync_report(&report);
    Ok(())
}

fn handle_update(args: UpdateArgs) -> Result<()> {
    println!("  ◇  Checking for the latest Karakuri release...");
    let outcome = infra::Updater::run(&args.to_domain_options())?;
    tui::print_update_report(&outcome);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("  {}  {}", "Error:".bright_red().bold(), err);
        exit(1);
    }
}
