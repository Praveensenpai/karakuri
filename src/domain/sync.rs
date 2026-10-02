use std::path::PathBuf;

/// Identified agent execution environment and its skill directory.
#[derive(Debug, Clone)]
pub struct AgentRuntime {
    pub name: &'static str,
    pub skills_dir: PathBuf,
}

impl AgentRuntime {
    pub fn new(name: &'static str, skills_dir: PathBuf) -> Self {
        Self { name, skills_dir }
    }
}

/// The synchronization outcome for a specific skill asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatus {
    Created,
    Updated,
    UpToDate,
}

/// A recorded synchronization action for a skill.
#[derive(Debug, Clone)]
pub struct SkillSyncRecord {
    pub runtime_name: &'static str,
    pub skill_name: String,
    pub status: SyncStatus,
}

/// Summary report of the skill synchronization cycle.
#[derive(Debug, Default)]
pub struct SyncReport {
    pub runtimes_detected: usize,
    pub skills_synced: usize,
    pub records: Vec<SkillSyncRecord>,
}
