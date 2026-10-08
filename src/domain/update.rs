use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct UpdateOptions {
    /// Report whether an update is available without installing it.
    pub check_only: bool,
    /// Reinstall even when the running version already matches the latest tag.
    pub force: bool,
}

/// Result of the post-update skill re-extraction step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillRefresh {
    /// The new binary re-extracted embedded skills and synced every agent.
    Refreshed,
    /// `KARAKURI_SKIP_REFRESH` was set, so the step was intentionally skipped.
    Skipped,
    /// Re-extraction ran but failed; the binary itself is still updated.
    Failed,
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
        /// Outcome of the post-update embedded-skill re-extraction.
        refresh: SkillRefresh,
    },
    /// No prebuilt asset matched; fell back to `cargo install --git`.
    BuiltFromSource {
        to: String,
        /// Outcome of the post-update embedded-skill re-extraction.
        refresh: SkillRefresh,
    },
}
