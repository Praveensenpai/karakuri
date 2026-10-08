use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::{InstallTarget, InstallationScope};
use crate::error::{KarakuriError, Result};
use crate::infra::embedded::{SkillAsset, AGENTS_MD, RULES_MD};

#[derive(Debug, Clone)]
pub struct InstalledRecord {
    pub name: String,
    pub path: String,
    pub target: String,
}

/// Installs rules and/or skills.
///
/// `skills` is the caller-resolved selection (see `infra::selector`), so the
/// installer itself stays stack-agnostic.
pub fn install(
    scope: InstallationScope,
    target: InstallTarget,
    skills: &[&SkillAsset],
) -> Result<Vec<InstalledRecord>> {
    match scope {
        InstallationScope::Project => install_project(target, skills),
        InstallationScope::Global => install_global(target, skills),
    }
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| KarakuriError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }
    fs::write(path, content).map_err(|e| KarakuriError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    Ok(())
}

fn write_skill_set(
    base: &Path,
    skills: &[&SkillAsset],
    label: &str,
    records: &mut Vec<InstalledRecord>,
) -> Result<()> {
    for skill in skills {
        let skill_path = base.join(skill.name).join("SKILL.md");
        write_file(&skill_path, skill.content)?;
        records.push(InstalledRecord {
            name: skill.name.to_string(),
            path: skill_path.display().to_string(),
            target: label.to_string(),
        });
    }
    Ok(())
}

fn install_project(target: InstallTarget, skills: &[&SkillAsset]) -> Result<Vec<InstalledRecord>> {
    let mut records = Vec::new();
    let current_dir = PathBuf::from(".");

    if target.includes_rules() {
        write_file(&current_dir.join("AGENTS.md"), AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "AGENTS.md".to_string(),
            path: "./AGENTS.md".to_string(),
            target: "OpenCode, Antigravity, Antigravity CLI".to_string(),
        });

        write_file(&current_dir.join("GEMINI.md"), AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "GEMINI.md".to_string(),
            path: "./GEMINI.md".to_string(),
            target: "Antigravity, Antigravity CLI".to_string(),
        });

        write_file(&current_dir.join(".agents/rules/RULES.md"), RULES_MD)?;
        write_file(&current_dir.join(".agents/rules/AGENTS.md"), AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "RULES.md".to_string(),
            path: ".agents/rules/RULES.md".to_string(),
            target: "Universal Agent Rules".to_string(),
        });
    }

    if target.includes_skills() {
        let base = current_dir.join(".agents/skills");
        write_skill_set(&base, skills, "Antigravity & OpenCode Skill", &mut records)?;
    }

    Ok(records)
}

fn install_global(target: InstallTarget, skills: &[&SkillAsset]) -> Result<Vec<InstalledRecord>> {
    let home = dirs::home_dir().ok_or(KarakuriError::HomeNotFound)?;
    let mut records = Vec::new();

    if target.includes_rules() {
        write_file(&home.join(".gemini/config/rules/AGENTS.md"), AGENTS_MD)?;
        write_file(&home.join(".gemini/config/rules/RULES.md"), RULES_MD)?;
        records.push(InstalledRecord {
            name: "AGENTS.md".to_string(),
            path: "~/.gemini/config/rules/AGENTS.md".to_string(),
            target: "Antigravity & Antigravity CLI".to_string(),
        });

        write_file(&home.join(".opencode/AGENTS.md"), AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "AGENTS.md".to_string(),
            path: "~/.opencode/AGENTS.md".to_string(),
            target: "OpenCode Global Rules".to_string(),
        });

        write_file(&home.join(".agents/rules/AGENTS.md"), AGENTS_MD)?;
        write_file(&home.join(".agents/rules/RULES.md"), RULES_MD)?;
        records.push(InstalledRecord {
            name: "RULES.md".to_string(),
            path: "~/.agents/rules/RULES.md".to_string(),
            target: "Universal Agent Standards".to_string(),
        });
    }

    if target.includes_skills() {
        write_skill_set(
            &home.join(".gemini/config/skills"),
            skills,
            "Global Modular Skill",
            &mut records,
        )?;
        write_skill_set(
            &home.join(".agents/skills"),
            skills,
            "Global Modular Skill",
            &mut records,
        )?;
    }

    Ok(records)
}
