pub struct SkillAsset {
    pub name: &'static str,
    pub content: &'static str,
}

pub const AGENTS_MD: &str = include_str!("../../rules/AGENTS.md");
pub const RULES_MD: &str = include_str!("../../rules/RULES.md");

pub const EMBEDDED_SKILLS: &[SkillAsset] = &[
    SkillAsset {
        name: "aesthetic-readme-craft",
        content: include_str!("../../skills/build-tooling/aesthetic-readme-craft/SKILL.md"),
    },
    SkillAsset {
        name: "git-release-craft",
        content: include_str!("../../skills/build-tooling/git-release-craft/SKILL.md"),
    },
    SkillAsset {
        name: "git-repo-craft",
        content: include_str!("../../skills/build-tooling/git-repo-craft/SKILL.md"),
    },
    SkillAsset {
        name: "python-clean-code",
        content: include_str!("../../skills/build-tooling/python-clean-code/SKILL.md"),
    },
    SkillAsset {
        name: "rust-clean-code",
        content: include_str!("../../skills/build-tooling/rust-clean-code/SKILL.md"),
    },
    SkillAsset {
        name: "bash-clean-code",
        content: include_str!("../../skills/system-ops/bash-clean-code/SKILL.md"),
    },
];
