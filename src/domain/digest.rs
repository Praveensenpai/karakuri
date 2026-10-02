use std::path::PathBuf;

/// Architectural role of a source code module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleRole {
    Domain,
    Infra,
    Api,
    Cli,
    Tui,
    Utils,
    General,
}

impl ModuleRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Infra => "infra",
            Self::Api => "api",
            Self::Cli => "cli",
            Self::Tui => "tui",
            Self::Utils => "utils",
            Self::General => "general",
        }
    }

    pub fn infer_from_path(path: &str) -> Self {
        if path.contains("domain") {
            Self::Domain
        } else if path.contains("infra") {
            Self::Infra
        } else if path.contains("api") {
            Self::Api
        } else if path.contains("cli") || path.contains("args") {
            Self::Cli
        } else if path.contains("tui") || path.contains("ui") {
            Self::Tui
        } else if path.contains("util") || path.contains("helper") {
            Self::Utils
        } else {
            Self::General
        }
    }
}

/// Metadata and extracted symbols for a single module.
#[derive(Debug, Clone)]
pub struct ModuleDigest {
    pub relative_path: PathBuf,
    pub role: ModuleRole,
    pub lines: usize,
    pub responsibility: String,
    pub types: Vec<String>,
    pub functions: Vec<String>,
    pub imports: Vec<String>,
}

/// High-density semantic digest representing the entire codebase.
#[derive(Debug, Clone)]
pub struct CodebaseDigest {
    pub project_name: String,
    pub primary_language: String,
    pub modules: Vec<ModuleDigest>,
    pub build_cmd: String,
    pub test_cmd: String,
    pub lint_cmd: String,
}

impl CodebaseDigest {
    /// Renders the complete, schema-compliant CODEBASE.md markdown document.
    pub fn render_markdown(&self) -> String {
        let mut out = String::with_capacity(4096);
        self.render_header(&mut out);
        self.render_modules(&mut out);
        self.render_footer(&mut out);
        out
    }

    fn render_header(&self, out: &mut String) {
        out.push_str(&format!(
            "# CODEBASE.md: {} Semantic Digest\n\n",
            self.project_name
        ));
        out.push_str("> **Notice**: AI-optimized semantic index. Do not write narrative prose. Keep token density high.\n\n");
        out.push_str("## 1. System Topology & Data Flow\n```text\n");
        out.push_str("Entrypoint ──> CLI/Parser ──> Domain Logic ──> Infra/IO\n");
        out.push_str("```\n\n");
        out.push_str("## 2. Global Constraints & Architecture Patterns\n");
        out.push_str(&format!(
            "- **Primary Language**: {}\n",
            self.primary_language
        ));
        out.push_str(
            "- **Architectural Paradigm**: Role-based (domain/, infra/, api/cli/, tui/)\n",
        );
        out.push_str("- **Hard Constraints**: <400 lines/file, <60 lines/fn, zero production unwrap(), 0 warnings.\n");
        out.push_str("- **Target Distribution**: Linux x86_64 standalone binary\n\n");
        out.push_str("## 3. Module & Interface Skeleton\n\n");
    }

    fn render_modules(&self, out: &mut String) {
        for m in &self.modules {
            render_single_module(m, out);
        }
    }

    fn render_footer(&self, out: &mut String) {
        out.push_str("## 4. Execution Lifecycle Trace\n");
        out.push_str("1. **Startup**: Entrypoint parses CLI flags & dispatches command.\n");
        out.push_str("2. **Execution**: Core domain logic processes inputs and evaluates rules.\n");
        out.push_str("3. **Persistence / I/O**: Domain logic calls infra for disk/terminal I/O.\n");
        out.push_str("4. **Exit**: Graceful termination with standard exit codes.\n\n");

        out.push_str("## 5. Verification Commands\n```bash\n");
        out.push_str(&format!("{}\n", self.build_cmd));
        out.push_str(&format!("{}\n", self.test_cmd));
        out.push_str(&format!("{}\n", self.lint_cmd));
        out.push_str("```\n");
    }
}

fn render_single_module(m: &ModuleDigest, out: &mut String) {
    out.push_str(&format!(
        "### `{}` (Role: {}, Lines: {})\n",
        m.relative_path.display(),
        m.role.as_str(),
        m.lines
    ));
    out.push_str(&format!("- **Responsibility**: {}\n", m.responsibility));

    if !m.imports.is_empty() {
        out.push_str(&format!("- **Imports**: {}\n", m.imports.join(", ")));
    }

    if !m.types.is_empty() {
        out.push_str("- **Types & Enums**:\n  ```rust\n");
        for t in &m.types {
            out.push_str(&format!("  {}\n", t));
        }
        out.push_str("  ```\n");
    }

    if !m.functions.is_empty() {
        out.push_str("- **Public Functions & Signatures**:\n  ```rust\n");
        for f in &m.functions {
            out.push_str(&format!("  {}\n", f));
        }
        out.push_str("  ```\n");
    }
    out.push('\n');
}
