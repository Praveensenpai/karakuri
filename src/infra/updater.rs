use crate::domain::{UpdateOptions, UpdateOutcome};
use crate::error::{KarakuriError, Result};

mod asset;
mod refresh;
mod replace;

use replace::replace_binary;

pub struct Updater;

impl Updater {
    pub fn run(opts: &UpdateOptions) -> Result<UpdateOutcome> {
        let current = env!("CARGO_PKG_VERSION").to_string();
        let tag = asset::latest_tag()?;
        let latest = tag.trim_start_matches('v').to_string();

        if !opts.force && latest == current {
            return Ok(UpdateOutcome::UpToDate { current, latest });
        }
        if opts.check_only {
            return Ok(UpdateOutcome::Available { current, latest });
        }

        let binary = download_binary(&tag)?;
        let target =
            std::env::current_exe().map_err(|e| KarakuriError::CurrentExe(e.to_string()))?;

        match replace_binary(&target, &binary) {
            Ok(()) => {
                replace::sync_secondaries(&target);
                let refresh = refresh::refresh_skills(&target);
                Ok(UpdateOutcome::Updated {
                    from: current,
                    to: latest,
                    path: target,
                    refresh,
                })
            }
            Err(err) if is_permission_denied(&err) => build_from_source(latest, &target),
            Err(err) => Err(err),
        }
    }
}

fn download_binary(tag: &str) -> Result<Vec<u8>> {
    let name = asset::asset_name()?;
    let archive = asset::download(tag, &name)?;
    asset::verify(tag, &name, &archive)?;
    replace::extract_binary(&name, &archive)
}

fn build_from_source(latest: String, target: &std::path::Path) -> Result<UpdateOutcome> {
    let status = std::process::Command::new("cargo")
        .args([
            "install",
            "--git",
            "https://github.com/Praveensenpai/karakuri",
            "--force",
            "--quiet",
        ])
        .status()
        .map_err(|e| KarakuriError::CurrentExe(e.to_string()))?;

    if status.success() {
        sync_cargo_install(target);
        let refresh = refresh::refresh_skills(target);
        Ok(UpdateOutcome::BuiltFromSource {
            to: latest,
            refresh,
        })
    } else {
        Err(KarakuriError::UnsupportedPlatform {
            arch: std::env::consts::ARCH.to_string(),
            os: std::env::consts::OS.to_string(),
        })
    }
}

fn sync_cargo_install(target: &std::path::Path) {
    if let Some(home) = dirs::home_dir() {
        let installed = home.join(".cargo/bin/karakuri");
        if installed.is_file() && installed != target {
            let _ = std::fs::copy(&installed, target);
        }
    }
}

fn is_permission_denied(err: &KarakuriError) -> bool {
    matches!(
        err,
        KarakuriError::Io { source, .. }
            if source.kind() == std::io::ErrorKind::PermissionDenied
    )
}
