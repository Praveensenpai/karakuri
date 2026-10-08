use std::path::Path;
use std::process::Command;

use crate::error::{KarakuriError, Result};

/// Re-extracts the updated binary's embedded skills into the global agent
/// directories and fans them out to every detected runtime.
///
/// The running process still holds the *previous* embedded assets, so this
/// shells out to the freshly replaced binary instead of writing in-process.
/// `binary` must be the path captured *before* the replacement: after the
/// old executable is unlinked, `current_exe()` no longer resolves to a
/// spawnable file.
///
/// Set `KARAKURI_SKIP_REFRESH=1` to opt out.
pub fn refresh_skills(binary: &Path) -> Result<()> {
    if std::env::var_os("KARAKURI_SKIP_REFRESH").is_some() {
        return Ok(());
    }

    run(binary, &["install", "--global", "--all", "-y"])?;
    run(binary, &["sync"])?;
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
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn run_reports_missing_binary() {
        let err = run(Path::new("/nonexistent/karakuri-xyz"), &["sync"])
            .expect_err("missing binary must fail");
        assert!(matches!(err, KarakuriError::SkillRefresh(_)));
    }

    #[test]
    fn run_succeeds_for_a_real_binary() {
        let dir = std::env::temp_dir().join(format!("kara-refresh-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        let script = dir.join("fake-karakuri");
        fs::write(&script, b"#!/bin/sh\nexit 0\n").expect("write script");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).expect("chmod script");

        run(&script, &["sync"]).expect("real executable must succeed");

        let _ = fs::remove_dir_all(&dir);
    }
}
