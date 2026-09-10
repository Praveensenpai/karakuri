use inquire::ui::{Color, RenderConfig, Styled};
use inquire::Select;

use crate::domain::{InstallTarget, InstallationScope};
use crate::error::{KarakuriError, Result};

fn custom_render_config() -> RenderConfig<'static> {
    let mut config = RenderConfig::default_colored();
    config.prompt_prefix = Styled::new("  ◇").with_fg(Color::LightCyan);
    config.highlighted_option_prefix = Styled::new("  │  ●").with_fg(Color::LightGreen);
    config.help_message = config.help_message.with_fg(Color::DarkGrey);
    config
}

pub fn select_scope() -> Result<InstallationScope> {
    let options = vec![InstallationScope::Project, InstallationScope::Global];
    let config = custom_render_config();

    let ans = Select::new("Installation scope", options)
        .with_render_config(config)
        .with_help_message("↑/↓ to navigate • Enter: confirm")
        .prompt()
        .map_err(KarakuriError::Prompt)?;

    println!("  │");
    Ok(ans)
}

pub fn select_target() -> Result<InstallTarget> {
    let options = vec![
        InstallTarget::Rules,
        InstallTarget::Skills,
        InstallTarget::All,
    ];
    let config = custom_render_config();

    let ans = Select::new("What do you want to install?", options)
        .with_render_config(config)
        .with_help_message("↑/↓ to navigate • Enter: confirm")
        .prompt()
        .map_err(KarakuriError::Prompt)?;

    println!("  │");
    Ok(ans)
}

pub fn confirm_installation() -> Result<bool> {
    let options = vec!["Yes", "No"];
    let mut config = custom_render_config();
    config.prompt_prefix = Styled::new("  ◆").with_fg(Color::LightCyan);

    let ans = Select::new("Proceed with installation?", options)
        .with_render_config(config)
        .with_help_message("↑/↓ to navigate • Enter: confirm")
        .prompt()
        .map_err(KarakuriError::Prompt)?;

    println!("  │");
    Ok(ans == "Yes")
}
