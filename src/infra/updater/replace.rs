use std::fs;
use std::io::Cursor;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::error::{KarakuriError, Result};

/// Extract the `karakuri` binary from a gzipped tar archive.
pub fn extract_binary(asset: &str, archive: &[u8]) -> Result<Vec<u8>> {
    let gz = flate2::read::GzDecoder::new(Cursor::new(archive));
    let mut tar = tar::Archive::new(gz);

    let entries = tar.entries().map_err(|e| KarakuriError::Network {
        url: asset.to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let mut entry = entry.map_err(|e| KarakuriError::Network {
            url: asset.to_string(),
            message: e.to_string(),
        })?;
        let path = entry.path().map_err(|e| KarakuriError::Network {
            url: asset.to_string(),
            message: e.to_string(),
        })?;
        if path.file_name().and_then(|n| n.to_str()) == Some("karakuri") {
            let mut buf = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut buf).map_err(|e| {
                KarakuriError::Network {
                    url: asset.to_string(),
                    message: e.to_string(),
                }
            })?;
            return Ok(buf);
        }
    }

    Err(KarakuriError::MalformedArchive(asset.to_string()))
}

/// Replace `target` with `bytes`, atomically when the filesystem allows.
pub fn replace_binary(target: &Path, bytes: &[u8]) -> Result<()> {
    let parent = target.parent().ok_or_else(|| KarakuriError::Io {
        path: target.to_path_buf(),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "no parent directory"),
    })?;

    let staged = parent.join(format!(".karakuri.new.{}", std::process::id()));
    write_executable(&staged, bytes)?;

    // On Linux, renaming over a busy (running) executable is fine, but some
    // filesystems reject it; move the old binary aside as a fallback.
    if fs::rename(&staged, target).is_err() {
        let backup = parent.join(format!(".karakuri.old.{}", std::process::id()));
        let _ = fs::rename(target, &backup);
        fs::rename(&staged, target).map_err(|e| KarakuriError::Io {
            path: target.to_path_buf(),
            source: e,
        })?;
        let _ = fs::remove_file(&backup);
    }

    Ok(())
}

fn write_executable(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).map_err(|e| KarakuriError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).map_err(|e| {
        KarakuriError::Io {
            path: path.to_path_buf(),
            source: e,
        }
    })?;
    Ok(())
}

/// Mirror `install.sh`: copy the fresh binary into other writable bin dirs.
pub fn sync_secondaries(primary: &Path) {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".local/bin/karakuri"));
        candidates.push(home.join(".cargo/bin/karakuri"));
    }

    for candidate in candidates {
        if candidate == primary || !candidate.is_file() {
            continue;
        }
        let writable = candidate
            .metadata()
            .map(|m| m.permissions().mode() & 0o200 != 0)
            .unwrap_or(false);
        if writable {
            let _ = fs::copy(primary, &candidate);
            let _ = fs::set_permissions(&candidate, fs::Permissions::from_mode(0o755));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gzipped_tar(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut tar_bytes = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar_bytes);
            for (name, body) in entries {
                let mut header = tar::Header::new_gnu();
                header.set_size(body.len() as u64);
                header.set_mode(0o755);
                header.set_cksum();
                builder
                    .append_data(&mut header, name, *body)
                    .expect("append test entry");
            }
            builder.finish().expect("finish test tar");
        }

        let mut gz = Vec::new();
        {
            let mut encoder = flate2::write::GzEncoder::new(&mut gz, flate2::Compression::fast());
            std::io::Write::write_all(&mut encoder, &tar_bytes).expect("gzip test tar");
            encoder.finish().expect("finish gzip");
        }
        gz
    }

    #[test]
    fn test_extract_binary_finds_karakuri() {
        let archive = gzipped_tar(&[("karakuri", b"#!/bin/sh\necho hi\n")]);
        let binary = extract_binary("test.tar.gz", &archive).expect("extract karakuri");
        assert_eq!(binary, b"#!/bin/sh\necho hi\n");
    }

    #[test]
    fn test_extract_binary_rejects_missing_member() {
        let archive = gzipped_tar(&[("README.md", b"no binary here")]);
        let err = extract_binary("test.tar.gz", &archive)
            .expect_err("archive without karakuri must fail");
        assert!(matches!(err, KarakuriError::MalformedArchive(_)));
    }

    #[test]
    fn test_replace_binary_round_trip() {
        let dir = std::env::temp_dir().join(format!("kara-replace-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temp dir");
        let target = dir.join("karakuri");
        fs::write(&target, b"old").expect("seed target");

        replace_binary(&target, b"new-binary").expect("replace binary");

        let after = fs::read(&target).expect("read replaced binary");
        assert_eq!(after, b"new-binary");

        let mode = fs::metadata(&target)
            .expect("stat replaced binary")
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o755);

        let _ = fs::remove_dir_all(&dir);
    }
}
