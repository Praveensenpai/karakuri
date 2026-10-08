use crate::domain::{SkillFilter, StackSet};
use crate::infra::embedded::{SkillAsset, EMBEDDED_SKILLS};

/// Returns the embedded skills that match the filter and detected stacks.
pub fn select_skills(filter: &SkillFilter, detected: &StackSet) -> Vec<&'static SkillAsset> {
    let active = filter.active_stacks(detected);
    EMBEDDED_SKILLS
        .iter()
        .filter(|asset| is_active(asset, &active))
        .collect()
}

fn is_active(asset: &SkillAsset, active: &StackSet) -> bool {
    asset.stacks.iter().any(|stack| active.contains(*stack))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Stack;

    #[test]
    fn rust_project_excludes_python_and_mobile_skills() {
        let detected = StackSet::from_stacks(&[Stack::Rust]);
        let selected = select_skills(&SkillFilter::Auto, &detected);
        let names: Vec<_> = selected.iter().map(|s| s.name).collect();

        assert!(names.contains(&"rust-clean-code"));
        assert!(!names.contains(&"python-clean-code"));
        assert!(!names.contains(&"compose-clean-code"));
        assert!(!names.contains(&"tayori"));
    }

    #[test]
    fn python_project_gets_both_python_skills() {
        let detected = StackSet::from_stacks(&[Stack::Python]);
        let selected = select_skills(&SkillFilter::Auto, &detected);
        let names: Vec<_> = selected.iter().map(|s| s.name).collect();

        assert!(names.contains(&"python-clean-code"));
        assert!(names.contains(&"rust-style-python"));
    }

    #[test]
    fn auto_never_selects_optional_skills() {
        let detected = StackSet::from_stacks(&[Stack::Rust, Stack::Python]);
        let selected = select_skills(&SkillFilter::Auto, &detected);
        let names: Vec<_> = selected.iter().map(|s| s.name).collect();

        assert!(!names.contains(&"tayori"));
        assert!(!names.contains(&"kuroko"));
    }

    #[test]
    fn all_filter_selects_every_embedded_skill() {
        let selected = select_skills(&SkillFilter::All, &StackSet::new());
        assert_eq!(selected.len(), EMBEDDED_SKILLS.len());
    }

    #[test]
    fn generic_skills_are_always_selected() {
        let selected = select_skills(&SkillFilter::Auto, &StackSet::new());
        let names: Vec<_> = selected.iter().map(|s| s.name).collect();

        assert!(names.contains(&"systematic-code-verification"));
        assert!(names.contains(&"unslop"));
        assert!(names.contains(&"codebase-digest"));
    }
}
