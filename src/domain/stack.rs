use std::fmt;

/// Technology stack a skill applies to.
///
/// `Generic` skills are stack-agnostic guardrails that every project gets.
/// `Optional` skills are never auto-selected; they require an explicit request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stack {
    Generic,
    Rust,
    Python,
    Mobile,
    Shell,
    Optional,
}

impl Stack {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Mobile => "mobile",
            Self::Shell => "shell",
            Self::Optional => "optional",
        }
    }

    /// Parses a user-facing stack token. Case-insensitive, accepts aliases.
    pub fn parse(token: &str) -> Option<Self> {
        match token.trim().to_ascii_lowercase().as_str() {
            "generic" | "core" | "common" => Some(Self::Generic),
            "rust" | "rs" => Some(Self::Rust),
            "python" | "py" => Some(Self::Python),
            "mobile" | "kotlin" | "android" | "compose" => Some(Self::Mobile),
            "shell" | "bash" | "sh" => Some(Self::Shell),
            "optional" | "extra" => Some(Self::Optional),
            _ => None,
        }
    }

    /// Parses a comma or space separated list of stack tokens.
    pub fn parse_list(raw: &str) -> Vec<Self> {
        raw.split([',', ' ', '\t'])
            .filter_map(Self::parse)
            .collect()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// An ordered, de-duplicated set of stacks.
#[derive(Debug, Default, Clone)]
pub struct StackSet {
    stacks: Vec<Stack>,
}

impl StackSet {
    pub fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    pub fn from_stacks(stacks: &[Stack]) -> Self {
        let mut set = Self::new();
        for stack in stacks {
            set.insert(*stack);
        }
        set
    }

    pub fn insert(&mut self, stack: Stack) {
        if !self.stacks.contains(&stack) {
            self.stacks.push(stack);
        }
    }

    pub fn contains(&self, stack: Stack) -> bool {
        self.stacks.contains(&stack)
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.is_empty()
    }

    /// Comma-joined labels, e.g. `generic, rust`.
    pub fn label(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Determines which skills an install pass should consider.
#[derive(Debug, Clone)]
pub enum SkillFilter {
    /// Every embedded skill, regardless of stack.
    All,
    /// Generic skills plus everything matching the detected stacks.
    Auto,
    /// Generic skills plus the explicitly requested stacks.
    Explicit(Vec<Stack>),
}

impl SkillFilter {
    /// Resolves the set of stacks that activate a skill for this filter.
    ///
    /// `Generic` is always active. Detected stacks never activate `Optional`.
    pub fn active_stacks(&self, detected: &StackSet) -> StackSet {
        match self {
            Self::All => {
                let mut set = StackSet::new();
                for stack in [
                    Stack::Generic,
                    Stack::Rust,
                    Stack::Python,
                    Stack::Mobile,
                    Stack::Shell,
                    Stack::Optional,
                ] {
                    set.insert(stack);
                }
                set
            }
            Self::Auto => {
                let mut set = detected.clone();
                set.insert(Stack::Generic);
                set
            }
            Self::Explicit(stacks) => {
                let mut set = StackSet::from_stacks(stacks);
                set.insert(Stack::Generic);
                set
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_aliases_case_insensitively() {
        assert_eq!(Stack::parse("RS"), Some(Stack::Rust));
        assert_eq!(Stack::parse(" py "), Some(Stack::Python));
        assert_eq!(Stack::parse("Kotlin"), Some(Stack::Mobile));
        assert_eq!(Stack::parse("bash"), Some(Stack::Shell));
        assert_eq!(Stack::parse("nonsense"), None);
    }

    #[test]
    fn parse_list_splits_on_commas_and_spaces() {
        let stacks = Stack::parse_list("rust, python mobile");
        assert_eq!(stacks, vec![Stack::Rust, Stack::Python, Stack::Mobile]);
    }

    #[test]
    fn stack_set_insert_deduplicates_and_preserves_order() {
        let mut set = StackSet::new();
        set.insert(Stack::Rust);
        set.insert(Stack::Generic);
        set.insert(Stack::Rust);
        assert_eq!(set.label(), "rust, generic");
    }

    #[test]
    fn explicit_filter_always_includes_generic() {
        let detected = StackSet::new();
        let filter = SkillFilter::Explicit(vec![Stack::Python]);
        let active = filter.active_stacks(&detected);
        assert!(active.contains(Stack::Generic));
        assert!(active.contains(Stack::Python));
        assert!(!active.contains(Stack::Rust));
    }

    #[test]
    fn all_filter_activates_every_stack_including_optional() {
        let active = SkillFilter::All.active_stacks(&StackSet::new());
        assert!(active.contains(Stack::Optional));
        assert!(active.contains(Stack::Mobile));
    }

    #[test]
    fn auto_filter_uses_detected_stacks_and_never_adds_optional() {
        let detected = StackSet::from_stacks(&[Stack::Rust]);
        let active = SkillFilter::Auto.active_stacks(&detected);
        assert!(active.contains(Stack::Rust));
        assert!(active.contains(Stack::Generic));
        assert!(!active.contains(Stack::Optional));
    }
}
