use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KarakuriError {
    #[error("I/O failure at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Terminal I/O error: {0}")]
    Terminal(#[from] std::io::Error),

    #[error("Home directory could not be resolved")]
    HomeNotFound,

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Network request to {url} failed: {message}")]
    Network { url: String, message: String },

    #[error("Checksum mismatch for {artifact}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        artifact: String,
        expected: String,
        actual: String,
    },

    #[error("Could not determine the running executable path: {0}")]
    CurrentExe(String),

    #[error("Release asset {0} has an unexpected layout: missing `karakuri` binary")]
    MalformedArchive(String),

    #[error("Architecture {arch} on {os} has no prebuilt asset and cargo is unavailable")]
    UnsupportedPlatform { arch: String, os: String },
}

pub type Result<T> = std::result::Result<T, KarakuriError>;
