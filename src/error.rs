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

    #[error("Interactive prompt error: {0}")]
    Prompt(#[from] inquire::InquireError),

    #[error("Home directory could not be resolved")]
    HomeNotFound,
}

pub type Result<T> = std::result::Result<T, KarakuriError>;
