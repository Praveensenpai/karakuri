#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedAgent {
    AntigravityCli,
    Antigravity,
    OpenCode,
}

impl SupportedAgent {
    pub const ALL: [Self; 3] = [Self::AntigravityCli, Self::Antigravity, Self::OpenCode];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::AntigravityCli => "Antigravity CLI",
            Self::Antigravity => "Antigravity",
            Self::OpenCode => "OpenCode",
        }
    }
}
