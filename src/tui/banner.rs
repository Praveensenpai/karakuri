use colored::Colorize;

use crate::domain::SupportedAgent;

pub fn print_banner() {
    println!();
    println!(
        "{}",
        "  ██╗  ██╗ █████╗ ██████╗  █████╗ ██╗  ██╗██╗   ██╗██████╗ ██╗".white()
    );
    println!(
        "{}",
        "  ██║ ██╔╝██╔══██╗██╔══██╗██╔══██╗██║ ██╔╝██║   ██║██╔══██╗██║".bright_black()
    );
    println!(
        "{}",
        "  █████╔╝ ███████║██████╔╝███████║█████╔╝ ██║   ██║██████╔╝██║".bright_black()
    );
    println!(
        "{}",
        "  ██╔═██╗ ██╔══██║██╔══██╗██╔══██║██╔═██╗ ██║   ██║██╔══██╗██║".bright_black()
    );
    println!(
        "{}",
        "  ██║  ██╗██║  ██║██║  ██║██║  ██║██║  ██╗╚██████╔╝██║  ██║██║".bright_black()
    );
    println!(
        "{}",
        "  ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝".bright_black()
    );
    println!();
    println!(
        "  {}  {}",
        " karakuri ".on_bright_black().bright_cyan().bold(),
        "Modular AI Agent Skills & Behavioral Guardrails".bright_black()
    );
    println!();
}

pub fn print_header_info() {
    let tree_v = "│".bright_black();
    let diamond = "◇".bright_cyan();

    println!("  {}", tree_v);
    println!(
        "  {}  {}: {}",
        diamond,
        "Source".bold(),
        "https://github.com/Praveensenpai/karakuri.git".bright_cyan()
    );
    println!("  {}", tree_v);
    println!(
        "  {}  {} {}",
        diamond,
        "Target Agents".bold(),
        "(Exclusive)".bright_black()
    );
    for agent in SupportedAgent::ALL {
        println!(
            "  {}  {} {}",
            tree_v,
            "•".bright_green(),
            agent.display_name().bold()
        );
    }
    println!("  {}", tree_v);
}
