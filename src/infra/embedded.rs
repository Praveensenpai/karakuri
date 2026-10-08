use crate::domain::Stack;

pub struct SkillAsset {
    pub name: &'static str,
    pub stacks: &'static [Stack],
    pub content: &'static str,
}

pub const AGENTS_MD: &str = include_str!("../../rules/AGENTS.md");
pub const RULES_MD: &str = include_str!("../../rules/RULES.md");

/// Generic guardrails every project receives.
const GENERIC: &[Stack] = &[Stack::Generic];

pub const EMBEDDED_SKILLS: &[SkillAsset] = &[
    SkillAsset {
        name: "aesthetic-readme-craft",
        stacks: GENERIC,
        content: include_str!("../../skills/build-tooling/aesthetic-readme-craft/SKILL.md"),
    },
    SkillAsset {
        name: "git-release-craft",
        stacks: GENERIC,
        content: include_str!("../../skills/build-tooling/git-release-craft/SKILL.md"),
    },
    SkillAsset {
        name: "git-repo-craft",
        stacks: GENERIC,
        content: include_str!("../../skills/build-tooling/git-repo-craft/SKILL.md"),
    },
    SkillAsset {
        name: "codebase-digest",
        stacks: GENERIC,
        content: include_str!("../../skills/build-tooling/codebase-digest/SKILL.md"),
    },
    SkillAsset {
        name: "systematic-code-verification",
        stacks: GENERIC,
        content: include_str!("../../skills/build-tooling/systematic-code-verification/SKILL.md"),
    },
    SkillAsset {
        name: "karakuri",
        stacks: GENERIC,
        content: include_str!("../../skills/build-tooling/karakuri/SKILL.md"),
    },
    SkillAsset {
        name: "unslop",
        stacks: GENERIC,
        content: include_str!("../../skills/writing/unslop/SKILL.md"),
    },
    SkillAsset {
        name: "rust-clean-code",
        stacks: &[Stack::Rust],
        content: include_str!("../../skills/build-tooling/rust-clean-code/SKILL.md"),
    },
    SkillAsset {
        name: "python-clean-code",
        stacks: &[Stack::Python],
        content: include_str!("../../skills/build-tooling/python-clean-code/SKILL.md"),
    },
    SkillAsset {
        name: "rust-style-python",
        stacks: &[Stack::Python],
        content: include_str!("../../skills/build-tooling/rust-style-python/SKILL.md"),
    },
    SkillAsset {
        name: "compose-clean-code",
        stacks: &[Stack::Mobile],
        content: include_str!("../../skills/mobile-dev/compose-clean-code/SKILL.md"),
    },
    SkillAsset {
        name: "bash-clean-code",
        stacks: &[Stack::Shell],
        content: include_str!("../../skills/system-ops/bash-clean-code/SKILL.md"),
    },
    SkillAsset {
        name: "tayori",
        stacks: &[Stack::Optional],
        content: include_str!("../../skills/ai-agents/tayori/SKILL.md"),
    },
    SkillAsset {
        name: "kuroko",
        stacks: &[Stack::Optional],
        content: include_str!("../../skills/ai-agents/kuroko/SKILL.md"),
    },
];
