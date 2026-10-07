use std::io::Read;

use sha2::{Digest, Sha256};

use crate::error::{KarakuriError, Result};

const REPO: &str = "https://github.com/Praveensenpai/karakuri";

pub fn latest_tag() -> Result<String> {
    let url = format!("{REPO}/releases/latest");
    let resp = ureq::get(&url).call().map_err(|e| KarakuriError::Network {
        url: url.clone(),
        message: e.to_string(),
    })?;

    // ureq follows redirects; the effective URL ends with /tag/<tag>.
    let effective = resp.get_url().to_string();
    effective
        .rsplit('/')
        .next()
        .filter(|t| !t.is_empty())
        .map(str::to_string)
        .ok_or_else(|| KarakuriError::Network {
            url,
            message: "could not parse release tag from redirect".to_string(),
        })
}

pub fn asset_name() -> Result<String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    Ok(format!("karakuri-{arch}-{os}.tar.gz"))
}

pub fn download(tag: &str, asset: &str) -> Result<Vec<u8>> {
    let url = format!("{REPO}/releases/download/{tag}/{asset}");
    let mut bytes = Vec::new();
    ureq::get(&url)
        .call()
        .map_err(|e| KarakuriError::Network {
            url: url.clone(),
            message: e.to_string(),
        })?
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| KarakuriError::Network {
            url,
            message: e.to_string(),
        })?;
    Ok(bytes)
}

/// Fetch `<asset>.sha256` and verify `archive` against it.
pub fn verify(tag: &str, asset: &str, archive: &[u8]) -> Result<()> {
    let url = format!("{REPO}/releases/download/{tag}/{asset}.sha256");
    let manifest = ureq::get(&url)
        .call()
        .map_err(|e| KarakuriError::Network {
            url: url.clone(),
            message: e.to_string(),
        })?
        .into_string()
        .map_err(|e| KarakuriError::Network {
            url,
            message: e.to_string(),
        })?;

    let expected = manifest
        .split_whitespace()
        .next()
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| KarakuriError::ChecksumMismatch {
            artifact: asset.to_string(),
            expected: "<empty>".to_string(),
            actual: String::new(),
        })?;

    check_checksum(asset, &expected, archive)
}

/// Pure checksum comparison, split out so it is testable without network.
pub fn check_checksum(artifact: &str, expected: &str, archive: &[u8]) -> Result<()> {
    let actual = hex(&Sha256::digest(archive));
    if actual != expected {
        return Err(KarakuriError::ChecksumMismatch {
            artifact: artifact.to_string(),
            expected: expected.to_string(),
            actual,
        });
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_name_has_tarball_suffix() {
        let name = asset_name().expect("asset name must resolve on supported hosts");
        assert!(name.starts_with("karakuri-"));
        assert!(name.ends_with(".tar.gz"));
    }

    #[test]
    fn test_check_checksum_accepts_matching_digest() {
        // sha256 of the empty input.
        let empty_digest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert!(check_checksum("empty.tar.gz", empty_digest, &[]).is_ok());
    }

    #[test]
    fn test_check_checksum_rejects_mismatch() {
        let err = check_checksum("bad.tar.gz", "deadbeef", b"payload")
            .expect_err("mismatched digest must fail");
        assert!(matches!(err, KarakuriError::ChecksumMismatch { .. }));
    }
}
