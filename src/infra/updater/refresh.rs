use std::path::Path;
use std::process::Command;

use crate::error::{KarakuriError, Result};

/// Re-extracts the updated binary's embedded skills into the global agent
/// directories and fans them out to every detected runtime.
///
/// The running process still holds the *previous* embedded assets, so this
/// shells out to the freshly replaced binary instead of writing in-process.
/// Set `KARAKURI_SKIP_REFRESH=1` to opt out.
pub fn refresh_skills() -> Result<()> {
    if std::env::var_os("KARAKURI_SKIP_REFRESH").is_some() {
        return Ok(());
    }

    let binary =
        std::env::current_exe().map_err(|e| KarakuriError::CurrentExe(e.to_string()))?;
    run(&binary, &["install", "--global", "--all", "-y"])?;
    run(&binary, &["sync"])?;
    Ok(())
}

fn run(binary: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new(binary)
        .args(args)
        .output()
        .map_err(|e| KarakuriError::SkillRefresh(format!("spawn `{}`: {e}", args.join(" "))))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(KarakuriError::SkillRefresh(format!(
        "`{}` exited with {}: {}",
        args.join(" "),
        output.status,
        stderr.trim()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_reports_missing_binary() {
        let err = run(Path::new("/nonexistent/karakuri-xyz"), &["sync"])
            .expect_err("missing binary must fail");
        assert!(matches!(err, KarakuriError::SkillRefresh(_)));
    }
}