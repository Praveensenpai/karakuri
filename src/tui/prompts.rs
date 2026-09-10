use std::fmt::Display;
use std::io::{stdout, Write};

use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};

use crate::domain::{InstallTarget, InstallationScope};
use crate::error::{KarakuriError, Result};

struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), cursor::Show);
    }
}

fn render_options<T: Display>(options: &[T], selected: usize) {
    let tree_v = "│".bright_black();
    let bullet_active = "●".bright_green();
    let bullet_inactive = "○".bright_black();

    for (i, opt) in options.iter().enumerate() {
        if i == selected {
            println!("  {}  {} {}", tree_v, bullet_active, opt.to_string().bold());
        } else {
            println!(
                "  {}  {} {}",
                tree_v,
                bullet_inactive,
                opt.to_string().bright_black()
            );
        }
    }
    println!(
        "  {}  {}",
        tree_v,
        "↑/↓ to navigate • Enter: confirm".bright_black()
    );
}

fn redraw_options<T: Display>(options: &[T], selected: usize, count: u16) -> Result<()> {
    let mut out = stdout();
    execute!(out, cursor::MoveUp(count))?;

    let tree_v = "│".bright_black();
    let bullet_active = "●".bright_green();
    let bullet_inactive = "○".bright_black();

    for (i, opt) in options.iter().enumerate() {
        execute!(out, terminal::Clear(ClearType::CurrentLine))?;
        if i == selected {
            print!(
                "  {}  {} {}\r\n",
                tree_v,
                bullet_active,
                opt.to_string().bold()
            );
        } else {
            print!(
                "  {}  {} {}\r\n",
                tree_v,
                bullet_inactive,
                opt.to_string().bright_black()
            );
        }
    }
    execute!(out, terminal::Clear(ClearType::CurrentLine))?;
    print!(
        "  {}  {}\r\n",
        tree_v,
        "↑/↓ to navigate • Enter: confirm".bright_black()
    );
    out.flush()?;
    Ok(())
}

pub fn select_menu<T: Display>(
    symbol: &str,
    title: &str,
    options: &[T],
    default_idx: usize,
) -> Result<usize> {
    let total = options.len();
    let mut selected = default_idx.min(total.saturating_sub(1));
    let lines_to_clear = (total + 1) as u16;

    println!("  {}  {}", symbol.bright_cyan(), title.bold());
    render_options(options, selected);

    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;
    let _guard = RawModeGuard;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    selected = if selected > 0 {
                        selected - 1
                    } else {
                        total - 1
                    };
                    redraw_options(options, selected, lines_to_clear)?;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    selected = if selected < total - 1 {
                        selected + 1
                    } else {
                        0
                    };
                    redraw_options(options, selected, lines_to_clear)?;
                }
                KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('\n') | KeyCode::Char(' ') => {
                    break
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Err(KarakuriError::Cancelled);
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    return Err(KarakuriError::Cancelled);
                }
                _ => {}
            }
        }
    }

    drop(_guard);

    let mut out = stdout();
    for _ in 0..lines_to_clear {
        execute!(
            out,
            cursor::MoveUp(1),
            terminal::Clear(ClearType::CurrentLine)
        )?;
    }

    let tree_v = "│".bright_black();
    println!(
        "  {}  {}",
        tree_v,
        options[selected].to_string().bright_cyan()
    );
    println!("  {}", tree_v);
    out.flush()?;

    Ok(selected)
}

pub fn select_scope() -> Result<InstallationScope> {
    let options = [InstallationScope::Project, InstallationScope::Global];
    let idx = select_menu("◇", "Installation scope", &options, 0)?;
    Ok(options[idx])
}

pub fn select_target() -> Result<InstallTarget> {
    let options = [
        InstallTarget::Rules,
        InstallTarget::Skills,
        InstallTarget::All,
    ];
    let idx = select_menu("◇", "What do you want to install?", &options, 0)?;
    Ok(options[idx])
}

pub fn confirm_installation() -> Result<bool> {
    let options = ["Yes", "No"];
    let idx = select_menu("◆", "Proceed with installation?", &options, 0)?;
    Ok(idx == 0)
}
