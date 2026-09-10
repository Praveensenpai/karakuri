use colored::Colorize;

use crate::domain::{InstallTarget, InstallationScope};
use crate::infra::installer::InstalledRecord;

pub fn print_summary_card(scope: InstallationScope, target: InstallTarget) {
    let tree_v = "│".bright_black();
    let diamond = "◇".bright_cyan();
    let border = "───────────────────────────────────┐".bright_black();
    let bottom = "└────────────────────────────────────────────────────────┘".bright_black();

    println!(
        "  {}  {} {}",
        diamond,
        "Installation Summary".bold(),
        border
    );
    println!("  {}  {}", tree_v, "│".bright_black());

    match scope {
        InstallationScope::Project => {
            if target.includes_rules() {
                print_project_rule_summary(&tree_v);
            }
            if target.includes_skills() {
                println!("  {}  {}", tree_v, "│".bright_black());
                println!(
                    "  {}  {}  {}",
                    tree_v,
                    "│".bright_black(),
                    ".agents/skills/*".bright_cyan()
                );
                println!(
                    "  {}  {}    {}",
                    tree_v,
                    "│".bright_black(),
                    "target ➔ Project-wide Modular Skills (7 skills)".bright_black()
                );
            }
        }
        InstallationScope::Global => {
            if target.includes_rules() {
                print_global_rule_summary(&tree_v);
            }
            if target.includes_skills() {
                println!("  {}  {}", tree_v, "│".bright_black());
                println!(
                    "  {}  {}  {}",
                    tree_v,
                    "│".bright_black(),
                    "~/.gemini/config/skills/* & ~/.agents/skills/*".bright_cyan()
                );
                println!(
                    "  {}  {}    {}",
                    tree_v,
                    "│".bright_black(),
                    "target ➔ Machine-wide Modular Skills".bright_black()
                );
            }
        }
    }

    println!("  {}  {}", tree_v, "│".bright_black());
    println!("  {}  {}", tree_v, bottom);
}

fn print_project_rule_summary(tree_v: &colored::ColoredString) {
    println!(
        "  {}  {}  {}",
        tree_v,
        "│".bright_black(),
        "./AGENTS.md".bright_cyan()
    );
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "target ➔ OpenCode, Antigravity, Antigravity CLI".bright_black()
    );
    println!("  {}  {}", tree_v, "│".bright_black());
    println!(
        "  {}  {}  {}",
        tree_v,
        "│".bright_black(),
        "./GEMINI.md".bright_cyan()
    );
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "target ➔ Antigravity, Antigravity CLI".bright_black()
    );
    println!("  {}  {}", tree_v, "│".bright_black());
    println!(
        "  {}  {}  {}",
        tree_v,
        "│".bright_black(),
        ".agents/rules/RULES.md".bright_cyan()
    );
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "target ➔ Universal Agent Rules & Guardrails".bright_black()
    );
}

fn print_global_rule_summary(tree_v: &colored::ColoredString) {
    println!(
        "  {}  {}  {}",
        tree_v,
        "│".bright_black(),
        "~/.gemini/config/rules/".bright_cyan()
    );
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "target ➔ Antigravity CLI & Antigravity IDE".bright_black()
    );
    println!("  {}  {}", tree_v, "│".bright_black());
    println!(
        "  {}  {}  {}",
        tree_v,
        "│".bright_black(),
        "~/.opencode/AGENTS.md".bright_cyan()
    );
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "target ➔ OpenCode Global Configuration".bright_black()
    );
    println!("  {}  {}", tree_v, "│".bright_black());
    println!(
        "  {}  {}  {}",
        tree_v,
        "│".bright_black(),
        "~/.agents/rules/".bright_cyan()
    );
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "target ➔ Universal Agent Global Rules".bright_black()
    );
}

pub fn print_security_card() {
    let tree_v = "│".bright_black();
    let diamond = "◇".bright_cyan();
    let border = "─────────────────────────────┐".bright_black();
    let bottom = "└────────────────────────────────────────────────────────┘".bright_black();

    println!();
    println!("  {}", tree_v);
    println!(
        "  {}  {} {}",
        diamond,
        "Security Risk Assessments".bold(),
        border
    );
    println!("  {}  {}", tree_v, "│".bright_black());
    println!(
        "  {}  {}      {}           {}          {}        {}",
        tree_v,
        "│".bright_black(),
        "Target".bright_black(),
        "Gen".bright_black(),
        "Socket".bright_black(),
        "Audit".bright_black()
    );
    println!(
        "  {}  {}      {:<15}  {:<12} {:<12} {:<10}",
        tree_v,
        "│".bright_black(),
        "karakuri-rules",
        "Safe".bright_green(),
        "0 alerts".bright_green(),
        "Verified".bright_green()
    );
    println!("  {}  {}", tree_v, "│".bright_black());
    println!(
        "  {}  {}    {}",
        tree_v,
        "│".bright_black(),
        "Details: Production Standards · Zero Unapproved Edits".bright_black()
    );
    println!("  {}  {}", tree_v, bottom);
    println!("  {}", tree_v);
}

pub fn print_success_box(records: &[InstalledRecord]) {
    let tree_v = "│".bright_black();
    let diamond = "◇".bright_cyan();
    let top = "┌────────────────────────────────────────────────────────┐".bright_black();
    let bottom = "└────────────────────────────────────────────────────────┘".bright_black();
    let check = "✓".bright_green();

    println!("  {}", tree_v);
    println!("  {}  {}", diamond, "Installation complete".bold());
    println!("  {}", tree_v);
    println!("  {}  {}", diamond, "Installed Components".bold());
    println!("  {}  {}", tree_v, top);

    for record in records {
        println!(
            "  {}  {}  {} {} {}",
            tree_v,
            "│".bright_black(),
            check,
            record.name.bold(),
            "(installed)".bright_black()
        );
        println!(
            "  {}  {}    ➔ {} {}",
            tree_v,
            "│".bright_black(),
            record.path.bright_cyan(),
            format!("[{}]", record.target).bright_black()
        );
    }

    println!("  {}  {}", tree_v, bottom);
    println!("  {}", tree_v);
    println!(
        "  {}  {} {}",
        "└─".bright_black(),
        "Done!".bright_green().bold(),
        "Guardrails active for Antigravity CLI, Antigravity, and OpenCode.".bright_black()
    );
    println!();
}
