use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::domain::digest::{CodebaseDigest, ModuleDigest};
use crate::error::{KarakuriError, Result};
use crate::infra::rust_analyzer::RustAnalyzer;

/// Builds a structured CodebaseDigest from the target directory.
pub struct DigestBuilder;

impl DigestBuilder {
    pub fn build(dir: &Path) -> Result<CodebaseDigest> {
        let project_name = infer_project_name(dir);
        let is_rust = dir.join("Cargo.toml").exists();
        let is_python =
            dir.join("pyproject.toml").exists() || dir.join("requirements.txt").exists();

        let (primary_language, build_cmd, test_cmd, lint_cmd) = if is_rust {
            (
                "Rust 2021 edition".to_string(),
                "cargo build --release --target x86_64-unknown-linux-gnu".to_string(),
                "cargo test --all-targets".to_string(),
                "cargo clippy --all-targets -- -D warnings && cargo fmt --check".to_string(),
            )
        } else if is_python {
            (
                "Python 3.12 (uv)".to_string(),
                "uv sync".to_string(),
                "uv run pytest".to_string(),
                "uv run ruff check && uv run ruff format --check".to_string(),
            )
        } else {
            (
                "Shell / Generic".to_string(),
                "make build".to_string(),
                "make test".to_string(),
                "shellcheck *.sh".to_string(),
            )
        };

        let modules = collect_modules(dir)?;

        Ok(CodebaseDigest {
            project_name,
            primary_language,
            modules,
            build_cmd,
            test_cmd,
            lint_cmd,
        })
    }

    pub fn write_to_file(dir: &Path, digest: &CodebaseDigest) -> Result<()> {
        let content = digest.render_markdown();
        let target = dir.join("CODEBASE.md");
        fs::write(&target, content).map_err(|source| KarakuriError::Io {
            path: target,
            source,
        })
    }
}

fn infer_project_name(dir: &Path) -> String {
    if let Some(name) = read_cargo_pkg_name(&dir.join("Cargo.toml")) {
        return name;
    }

    dir.canonicalize()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "project".to_string())
}

fn read_cargo_pkg_name(cargo_path: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("name =") {
            let val = trimmed.split('=').nth(1)?;
            return Some(val.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn collect_modules(dir: &Path) -> Result<Vec<ModuleDigest>> {
    let mut modules = Vec::new();
    let src_dir = if dir.join("src").is_dir() {
        dir.join("src")
    } else {
        dir.to_path_buf()
    };

    for entry in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            if let Ok(content) = fs::read_to_string(path) {
                let relative = path.strip_prefix(dir).unwrap_or(path);
                let digest = RustAnalyzer::extract_digest(relative, &content);
                modules.push(digest);
            }
        }
    }

    modules.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(modules)
}
