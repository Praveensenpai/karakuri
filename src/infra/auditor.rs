use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

use crate::domain::audit::{AuditOptions, AuditReport, Violation, ViolationKind};
use crate::error::Result;
use crate::infra::rust_analyzer::RustAnalyzer;

/// Executes the full audit suite against the target directory.
pub fn run_audit(opts: &AuditOptions) -> Result<AuditReport> {
    let mut report = AuditReport::default();

    if opts.check_codebase_md && !opts.path.join("CODEBASE.md").exists() {
        report.violations.push(Violation {
            path: opts.path.join("CODEBASE.md"),
            kind: ViolationKind::MissingCodebaseMd,
        });
    }

    if opts.check_readme_md && !opts.path.join("README.md").exists() {
        report.violations.push(Violation {
            path: opts.path.join("README.md"),
            kind: ViolationKind::MissingReadmeMd,
        });
    }

    scan_directory(&opts.path, opts, &mut report)?;

    if opts.check_clippy && opts.path.join("Cargo.toml").exists() {
        run_clippy_check(&opts.path, &mut report);
    }

    Ok(report)
}

fn should_skip_entry(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    if entry.file_type().is_dir() {
        matches!(
            name.as_ref(),
            ".git" | "target" | "node_modules" | "dist" | "build" | ".venv" | "__pycache__"
        )
    } else {
        name.starts_with('.')
    }
}

fn is_supported_code_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rs" | "py" | "sh" | "kt" | "java" | "js" | "ts" | "qml")
    )
}

fn scan_directory(dir: &Path, opts: &AuditOptions, report: &mut AuditReport) -> Result<()> {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !should_skip_entry(e))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && is_supported_code_file(path) {
            audit_single_file(path, opts, report)?;
        }
    }
    Ok(())
}

fn audit_single_file(path: &Path, opts: &AuditOptions, report: &mut AuditReport) -> Result<()> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(()), // Skip non-UTF8 binary files safely
    };

    let line_count = content.lines().count();
    report.files_scanned += 1;
    report.lines_scanned += line_count;

    if line_count > opts.max_file_lines {
        report.violations.push(Violation {
            path: path.to_path_buf(),
            kind: ViolationKind::FileTooLong {
                lines: line_count,
                limit: opts.max_file_lines,
            },
        });
    }

    if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
        let rust_violations =
            RustAnalyzer::audit_source(path, &content, opts.max_fn_lines, opts.max_nesting_depth);
        report.violations.extend(rust_violations);
    }

    Ok(())
}

fn run_clippy_check(project_dir: &Path, report: &mut AuditReport) {
    let output = match Command::new("cargo")
        .args(["clippy", "--all-targets", "--", "-D", "warnings"])
        .current_dir(project_dir)
        .output()
    {
        Ok(out) => out,
        Err(_) => return,
    };

    if output.status.success() {
        return;
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}\n{}", stderr, stdout);
    for line in combined.lines() {
        if line.contains("error:") || line.contains("warning:") {
            report.violations.push(Violation {
                path: project_dir.to_path_buf(),
                kind: ViolationKind::ClippyWarning(line.trim().to_string()),
            });
        }
    }
}
