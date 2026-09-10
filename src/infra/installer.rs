use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::{InstallTarget, InstallationScope};
use crate::error::{KarakuriError, Result};
use crate::infra::embedded::{AGENTS_MD, EMBEDDED_SKILLS, RULES_MD};

#[derive(Debug, Clone)]
pub struct InstalledRecord {
    pub name: String,
    pub path: String,
    pub target: String,
}

pub fn install(scope: InstallationScope, target: InstallTarget) -> Result<Vec<InstalledRecord>> {
    match scope {
        InstallationScope::Project => install_project(target),
        InstallationScope::Global => install_global(target),
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

fn install_project(target: InstallTarget) -> Result<Vec<InstalledRecord>> {
    let mut records = Vec::new();
    let current_dir = PathBuf::from(".");

    if target.includes_rules() {
        let agents_path = current_dir.join("AGENTS.md");
        write_file(&agents_path, AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "AGENTS.md".to_string(),
            path: "./AGENTS.md".to_string(),
            target: "OpenCode, Antigravity, Antigravity CLI".to_string(),
        });

        let gemini_path = current_dir.join("GEMINI.md");
        write_file(&gemini_path, AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "GEMINI.md".to_string(),
            path: "./GEMINI.md".to_string(),
            target: "Antigravity, Antigravity CLI".to_string(),
        });

        let universal_rules = current_dir.join(".agents/rules/RULES.md");
        write_file(&universal_rules, RULES_MD)?;
        let universal_agents = current_dir.join(".agents/rules/AGENTS.md");
        write_file(&universal_agents, AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "RULES.md".to_string(),
            path: ".agents/rules/RULES.md".to_string(),
            target: "Universal Agent Rules".to_string(),
        });
    }

    if target.includes_skills() {
        let skills_base = current_dir.join(".agents/skills");
        for skill in EMBEDDED_SKILLS {
            let skill_dir = skills_base.join(skill.name);
            let skill_path = skill_dir.join("SKILL.md");
            write_file(&skill_path, skill.content)?;
            records.push(InstalledRecord {
                name: skill.name.to_string(),
                path: format!(".agents/skills/{}", skill.name),
                target: "Antigravity & OpenCode Skill".to_string(),
            });
        }
    }

    Ok(records)
}

fn install_global(target: InstallTarget) -> Result<Vec<InstalledRecord>> {
    let home = dirs::home_dir().ok_or(KarakuriError::HomeNotFound)?;
    let mut records = Vec::new();

    if target.includes_rules() {
        let gemini_rule = home.join(".gemini/config/rules/AGENTS.md");
        write_file(&gemini_rule, AGENTS_MD)?;
        let gemini_rules_md = home.join(".gemini/config/rules/RULES.md");
        write_file(&gemini_rules_md, RULES_MD)?;
        records.push(InstalledRecord {
            name: "AGENTS.md".to_string(),
            path: "~/.gemini/config/rules/AGENTS.md".to_string(),
            target: "Antigravity & Antigravity CLI".to_string(),
        });

        let opencode_rule = home.join(".opencode/AGENTS.md");
        write_file(&opencode_rule, AGENTS_MD)?;
        records.push(InstalledRecord {
            name: "AGENTS.md".to_string(),
            path: "~/.opencode/AGENTS.md".to_string(),
            target: "OpenCode Global Rules".to_string(),
        });

        let universal_rule = home.join(".agents/rules/AGENTS.md");
        write_file(&universal_rule, AGENTS_MD)?;
        let universal_rules_md = home.join(".agents/rules/RULES.md");
        write_file(&universal_rules_md, RULES_MD)?;
        records.push(InstalledRecord {
            name: "RULES.md".to_string(),
            path: "~/.agents/rules/RULES.md".to_string(),
            target: "Universal Agent Standards".to_string(),
        });
    }

    if target.includes_skills() {
        let gemini_skills = home.join(".gemini/config/skills");
        let agents_skills = home.join(".agents/skills");
        for skill in EMBEDDED_SKILLS {
            let gemini_path = gemini_skills.join(skill.name).join("SKILL.md");
            write_file(&gemini_path, skill.content)?;
            let agents_path = agents_skills.join(skill.name).join("SKILL.md");
            write_file(&agents_path, skill.content)?;

            records.push(InstalledRecord {
                name: skill.name.to_string(),
                path: format!("~/.gemini/config/skills/{}", skill.name),
                target: "Global Modular Skill".to_string(),
            });
        }
    }

    Ok(records)
}
