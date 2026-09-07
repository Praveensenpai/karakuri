# からくり (Karakuri)

> **Karakuri (絡繰り)** — *Clever mechanical mechanisms and automations.*  
> A curated, categorized arsenal of modular AI agent skills designed for fast lookup and automated workflows.

---

## 🧭 How to Navigate (For AI & Developers)

Skills are categorized logically so an AI agent can quickly identify where to look without scanning irrelevant procedures:

```
skills/
├── build-tooling/      # Compilers, build offloading, CI/CD, language tools
├── web-browser/        # Frontend development, browser automation, DevTools
├── cloud-data/         # Cloud platforms, data pipelines, SQL, databases
├── mobile-dev/         # Mobile development, Flutter, Android, iOS
├── ai-agents/          # LLM integrations, prompt engineering, agent frameworks
└── system-ops/         # OS administration, shell scripts, system performance
```

When an agent needs to perform a specialized task:
1. Locate the relevant category in the **[Skill Catalog](#-skill-catalog)** below.
2. Read the 2–3 line summary to verify it matches the user's intent.
3. Open and follow the linked `SKILL.md` for full operational instructions.

---

## 📚 Skill Catalog

| Skill | Category | Description | Path |
| :--- | :--- | :--- | :--- |
| **`kbuild`** | `build-tooling` | Offloads Rust builds, checks, and tests to Kaggle Cloud (30 GB RAM) with zero local CPU load and zero RAM thrashing. Prompts the user to install or use the latest `kbuild` binary, prioritizes cloud compilation over cargo, and provides seamless fallback to local `cargo`. | [`skills/build-tooling/kbuild/SKILL.md`](skills/build-tooling/kbuild/SKILL.md) |
| **`rust-clean-code`** | `build-tooling` | Enforces strict architecture, readability, and scalability standards for Rust codebases: zero warning suppressions (`#[allow(...)]`), zero dead code, strict file/function size limits, role-based folder hierarchy, DRY abstractions, and resilient error handling. | [`skills/build-tooling/rust-clean-code/SKILL.md`](skills/build-tooling/rust-clean-code/SKILL.md) |
| **`python-clean-code`** | `build-tooling` | Enforces modern type hints, clean architecture, and rigorous quality tooling for Python: exclusive use of `uv` package manager, 100% type annotations, automated `ruff` linting and import sorting, zero dead code, and role-based folder hierarchy. | [`skills/build-tooling/python-clean-code/SKILL.md`](skills/build-tooling/python-clean-code/SKILL.md) |
| **`git-release-craft`** | `build-tooling` | Automates the complete Git release lifecycle: conventional commits, semantic version bumps, rich aesthetic release descriptions with highlights, tag creation, and mandatory post-release verification of GitHub Actions workflows and assets. | [`skills/build-tooling/git-release-craft/SKILL.md`](skills/build-tooling/git-release-craft/SKILL.md) |
| **`git-repo-craft`** | `build-tooling` | Automates GitHub repository creation, editing, and metadata auditing: mandates minimal yet meaningful descriptions (<90 chars), curated discoverable topics, clean initialization, and active verification via GitHub CLI. | [`skills/build-tooling/git-repo-craft/SKILL.md`](skills/build-tooling/git-repo-craft/SKILL.md) |
| **`aesthetic-readme-craft`** | `build-tooling` | Crafts stunning, aesthetic READMEs following Praveensenpai's design standard: badge ribbons, ASCII/Mermaid architecture diagrams, alert callouts, emoji feature matrices, and 1-liner installers. | [`skills/build-tooling/aesthetic-readme-craft/SKILL.md`](skills/build-tooling/aesthetic-readme-craft/SKILL.md) |
| **`bash-clean-code`** | `system-ops` | Enforces production standards for Bash and POSIX shell scripting: strict error handling (`set -euo pipefail`), zero ShellCheck warnings, temporary file cleanup traps, XDG directory compliance, and portable OS/architecture detection. | [`skills/system-ops/bash-clean-code/SKILL.md`](skills/system-ops/bash-clean-code/SKILL.md) |

*(New skills are added here on demand as specialized workflows are created.)*

---

## 📁 Category Breakdown

### 1. `build-tooling`
Tools and procedures that offload, accelerate, or automate code compilation, package management, and build pipelines.
- **Examples**: Cloud build offloading (`kbuild`), Rust cross-compilation, build cache management, dependency resolvers.

### 2. `web-browser`
Modern frontend best practices, browser DevTools debugging, web performance, and browser extensions.
- **Examples**: Chrome DevTools automation, Core Web Vitals optimization, accessibility (a11y) auditing, Manifest V3 extensions.

### 3. `cloud-data`
Cloud infrastructure, serverless runtimes, data warehouses, and ETL pipelines.
- **Examples**: BigQuery optimization, dbt / Dataform pipelines, Apache Beam / Dataflow, cloud storage lifecycle.

### 4. `mobile-dev`
Mobile application development, multi-platform toolchains, and device automation.
- **Examples**: Android CLI SDK management, Flutter architectures, widget/integration testing, declarative routing.

### 5. `ai-agents`
Building, orchestrating, and extending AI applications, function calling, and multi-agent systems.
- **Examples**: Gemini API multimodal workflows, real-time Live API streaming, agent orchestration frameworks.

### 6. `system-ops`
System administration, operating system utilities, shell scripting, and hardware diagnostics.
- **Examples**: Linux package cleaning, system metrics monitoring, service orchestration.

---

## ➕ Adding a New Skill

When adding a new skill to **Karakuri**:

1. **Choose the Category**: Pick the appropriate folder under `skills/<category>/`.
2. **Create the Skill Directory**:
   ```bash
   mkdir -p skills/<category>/<skill-name>
   touch skills/<category>/<skill-name>/SKILL.md
   ```
3. **Format `SKILL.md` with YAML Frontmatter**:
   ```markdown
   ---
   name: <skill-name>
   description: >-
     A concise 2-3 sentence description explaining WHAT this skill does
     and WHEN the agent should activate it.
   ---

   # Skill Title

   Detailed procedures, command mappings, and execution steps.
   ```
4. **Update Catalog**: Add an entry for the new skill in the [Skill Catalog](#-skill-catalog) table above with a concise 2–3 line summary.

---

## 🚀 Installation & Usage in Projects

### Global Installation (Machine-wide)
To make a skill available across all projects on your machine:
```bash
mkdir -p ~/.gemini/config/skills/<skill-name>
cp skills/<category>/<skill-name>/SKILL.md ~/.gemini/config/skills/<skill-name>/
```

### Workspace Installation (Single Project)
To add a skill to a specific repository:
```bash
mkdir -p .agents/skills/<skill-name>
cp /path/to/karakuri/skills/<category>/<skill-name>/SKILL.md .agents/skills/<skill-name>/
```

---

## 🛡️ Project Rules & Behavioral Guardrails (`rules/`)

Karakuri provides a standard behavioral contract in [`rules/RULES.md`](rules/RULES.md) (and [`rules/AGENTS.md`](rules/AGENTS.md)):

- **Explicit Approval Protocol**: Mandatory requirement that AI agents explain **WHY** and **WHAT EFFECT** a change has, and obtain explicit user consent before editing code.
- **Unified Standards Enforcement**: Directly references and enforces domain skills (`rust-clean-code`, `python-clean-code`, `git-release-craft`) without redundant rule sprawl.

To enforce this in any repository, copy `rules/RULES.md` or `rules/AGENTS.md` to the root of your project:
```bash
cp /path/to/karakuri/rules/AGENTS.md ./AGENTS.md
```

---

## 📜 License
MIT OR Apache-2.0 © Praveensenpai
