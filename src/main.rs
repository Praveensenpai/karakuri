use std::process::exit;

use clap::Parser;
use colored::Colorize;

mod cli;
mod domain;
mod error;
mod infra;
mod tui;

use cli::CliArgs;
use error::Result;

fn run() -> Result<()> {
    let args = CliArgs::parse();

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

fn main() {
    if let Err(err) = run() {
        eprintln!("  {}  {}", "Error:".bright_red().bold(), err);
        exit(1);
    }
}
