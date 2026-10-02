use std::fs;
use std::path::Path;

use crate::domain::sync::{AgentRuntime, SkillSyncRecord, SyncReport, SyncStatus};
use crate::error::{KarakuriError, Result};

/// Synchronizes skill assets across multiple local AI agent runtimes.
pub struct SyncEngine;

impl SyncEngine {
    pub fn run() -> Result<SyncReport> {
        let home = dirs::home_dir().ok_or(KarakuriError::HomeNotFound)?;
        let runtimes = detect_agent_runtimes(&home);
        let master_skills_dir = home.join(".gemini/config/skills");

        let mut report = SyncReport {
            runtimes_detected: runtimes.len(),
            ..Default::default()
        };

        if !master_skills_dir.exists() {
            return Ok(report);
        }

        let available_skills = list_skills(&master_skills_dir)?;
        report.skills_synced = available_skills.len();

        for runtime in &runtimes {
            for skill in &available_skills {
                let status = sync_single_skill(&master_skills_dir, &runtime.skills_dir, skill)?;
                report.records.push(SkillSyncRecord {
                    runtime_name: runtime.name,
                    skill_name: skill.clone(),
                    status,
                });
            }
        }

        Ok(report)
    }
}

fn detect_agent_runtimes(home: &Path) -> Vec<AgentRuntime> {
    let mut list = Vec::new();
    let candidates = [
        (
            "Antigravity / Gemini",
            ".gemini/config",
            ".gemini/config/skills",
        ),
        ("Standard Agents / OpenCode", ".agents", ".agents/skills"),
        ("Claude Agent", ".claude", ".claude/skills"),
        ("Codex Agent", ".codex", ".codex/skills"),
    ];

    for (name, base, skills) in candidates {
        let base_dir = home.join(base);
        if base_dir.exists() {
            list.push(AgentRuntime::new(name, home.join(skills)));
        }
    }
    list
}

fn list_skills(dir: &Path) -> Result<Vec<String>> {
    let mut skills = Vec::new();
    let entries = fs::read_dir(dir).map_err(|source| KarakuriError::Io {
        path: dir.to_path_buf(),
        source,
    })?;

    for entry in entries.flatten() {
        if entry.path().is_dir() && entry.path().join("SKILL.md").exists() {
            skills.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    skills.sort();
    Ok(skills)
}

fn sync_single_skill(
    master_dir: &Path,
    target_skills_dir: &Path,
    skill_name: &str,
) -> Result<SyncStatus> {
    let src = master_dir.join(skill_name);
    let dst = target_skills_dir.join(skill_name);

    if !dst.exists() {
        copy_dir_recursive(&src, &dst)?;
        return Ok(SyncStatus::Created);
    }

    let src_file = src.join("SKILL.md");
    let dst_file = dst.join("SKILL.md");
    if src_file.exists() && dst_file.exists() {
        let src_bytes = fs::read(&src_file).unwrap_or_default();
        let dst_bytes = fs::read(&dst_file).unwrap_or_default();
        if src_bytes != dst_bytes {
            copy_dir_recursive(&src, &dst)?;
            return Ok(SyncStatus::Updated);
        }
    }

    Ok(SyncStatus::UpToDate)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(|source| KarakuriError::Io {
        path: dst.to_path_buf(),
        source,
    })?;

    let entries = fs::read_dir(src).map_err(|source| KarakuriError::Io {
        path: src.to_path_buf(),
        source,
    })?;

    for entry in entries.flatten() {
        let path = entry.path();
        let dest_child = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &dest_child)?;
        } else {
            fs::copy(&path, &dest_child).map_err(|source| KarakuriError::Io {
                path: dest_child,
                source,
            })?;
        }
    }
    Ok(())
}
