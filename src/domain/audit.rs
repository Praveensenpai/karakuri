use std::path::PathBuf;

/// Options configuring the codebase audit rules.
#[derive(Debug, Clone)]
pub struct AuditOptions {
    pub path: PathBuf,
    pub max_file_lines: usize,
    pub max_fn_lines: usize,
    pub max_nesting_depth: usize,
    pub check_clippy: bool,
    pub check_codebase_md: bool,
    pub check_readme_md: bool,
}

impl Default for AuditOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            max_file_lines: 400,
            max_fn_lines: 60,
            max_nesting_depth: 3,
            check_clippy: true,
            check_codebase_md: true,
            check_readme_md: true,
        }
    }
}

/// Category and detail of an identified rule violation.
#[derive(Debug, Clone)]
pub enum ViolationKind {
    FileTooLong {
        lines: usize,
        limit: usize,
    },
    FunctionTooLong {
        name: String,
        lines: usize,
        limit: usize,
        line_no: usize,
    },
    NestingTooDeep {
        name: String,
        depth: usize,
        limit: usize,
        line_no: usize,
    },
    ForbiddenPattern {
        pattern: String,
        line_no: usize,
    },
    MissingCodebaseMd,
    MissingReadmeMd,
    ClippyWarning(String),
}

/// A violation anchored to a specific file path.
#[derive(Debug, Clone)]
pub struct Violation {
    pub path: PathBuf,
    pub kind: ViolationKind,
}

/// Aggregated report of an audit run.
#[derive(Debug, Default)]
pub struct AuditReport {
    pub files_scanned: usize,
    pub lines_scanned: usize,
    pub violations: Vec<Violation>,
}

impl AuditReport {
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_report_clean() {
        let report = AuditReport::default();
        assert!(report.is_clean());
    }

    #[test]
    fn test_audit_report_with_violations() {
        let mut report = AuditReport::default();
        report.violations.push(Violation {
            path: PathBuf::from("src/test.rs"),
            kind: ViolationKind::FileTooLong {
                lines: 500,
                limit: 400,
            },
        });
        assert!(!report.is_clean());
    }
}
