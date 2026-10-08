use std::fs;
use std::path::Path;

use crate::domain::{SkillFilter, StackSet};
use crate::error::{KarakuriError, Result};
use crate::infra::detector;
use crate::infra::embedded::SkillAsset;
use crate::infra::selector::select_skills;

/// Outcome of a project reconciliation pass.
#[derive(Debug, Default)]
pub struct EnsureReport {
    pub stacks: StackSet,
    pub selected: Vec<String>,
    pub written: Vec<String>,
    pub up_to_date: Vec<String>,
}

impl EnsureReport {
    pub fn changed(&self) -> bool {
        !self.written.is_empty()
    }
}

/// Detects the project stack and installs the matching skills that are
/// missing or stale under `<project>/.agents/skills`.
///
/// Idempotent: running it twice writes nothing the second time.
pub fn ensure(path: &Path) -> Result<EnsureReport> {
    let stacks = detector::detect(path);
    let selected = select_skills(&SkillFilter::Auto, &stacks);
    let base = path.join(".agents/skills");

    let mut report = EnsureReport {
        stacks,
        selected: selected.iter().map(|s| s.name.to_string()).collect(),
        ..Default::default()
    };

    for skill in selected {
        reconcile(&base, skill, &mut report)?;
    }

    Ok(report)
}

/// Writes a skill only when its `SKILL.md` differs from the embedded copy.
fn reconcile(base: &Path, skill: &SkillAsset, report: &mut EnsureReport) -> Result<()> {
    let target = base.join(skill.name).join("SKILL.md");
    let current = fs::read_to_string(&target).ok();

    if current.as_deref() == Some(skill.content) {
        report.up_to_date.push(skill.name.to_string());
        return Ok(());
    }

    write_skill(&target, skill.content)?;
    report.written.push(skill.name.to_string());
    Ok(())
}

fn write_skill(path: &Path, content: &str) -> Result<()> {
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
