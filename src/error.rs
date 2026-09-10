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
}

pub type Result<T> = std::result::Result<T, KarakuriError>;
