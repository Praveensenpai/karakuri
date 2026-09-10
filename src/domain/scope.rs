use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallationScope {
    Project,
    Global,
}

impl InstallationScope {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Project => "Project (Install in current directory / committed with your project)",
            Self::Global => {
                "Global (Install machine-wide across ~/.gemini, ~/.agents, ~/.opencode)"
            }
        }
    }
}

impl fmt::Display for InstallationScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
