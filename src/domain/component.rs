use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallTarget {
    Rules,
    Skills,
    All,
}

impl InstallTarget {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Rules => "Rules & Guardrails (AGENTS.md, GEMINI.md, RULES.md)",
            Self::Skills => "Modular Skills (7 curated Karakuri skills)",
            Self::All => "Everything (Rules + Skills)",
        }
    }

    pub fn includes_rules(&self) -> bool {
        matches!(self, Self::Rules | Self::All)
    }

    pub fn includes_skills(&self) -> bool {
        matches!(self, Self::Skills | Self::All)
    }
}

impl fmt::Display for InstallTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
