use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct UpdateOptions {
    /// Report whether an update is available without installing it.
    pub check_only: bool,
    /// Reinstall even when the running version already matches the latest tag.
    pub force: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateOutcome {
    /// Running build is already the latest release.
    UpToDate { current: String, latest: String },
    /// A newer release exists but was not installed (`--check`).
    Available { current: String, latest: String },
    /// The running binary was replaced in place.
    Updated {
        from: String,
        to: String,
        path: PathBuf,
    },
    /// No prebuilt asset matched; fell back to `cargo install --git`.
    BuiltFromSource { to: String },
}
