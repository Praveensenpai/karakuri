---
name: rust-clean-code
description: >-
  Enforces strict architecture, readability, and scalability standards for Rust codebases.
  Mandates zero warning suppressions (no #[allow(...)]), zero dead code, strict line and
  function size limits, role-based folder hierarchy, DRY principles, and resilient error handling.
---

# `rust-clean-code` Skill: Architecture, Scalability & Quality Rules

Enforces strict production standards for Rust projects. Code must be idiomatic, maintainable, modular, and free of compiler or linter warnings.

---

## 1. Zero Warnings & Zero Dead Code Policy (Non-Negotiable)

1. **No Warning Suppressions**:
   - **Never** add `#[allow(dead_code)]`, `#[allow(unused)]`, or `#[allow(clippy::...)]` to bypass compiler feedback.
   - If code triggers a warning, fix the root cause immediately instead of silencing it.
2. **Remove Dead Code Aggressively**:
   - Delete unused functions, imports, variables, and struct fields.
   - Do not leave commented-out code blocks or orphan helper functions.
3. **Mandatory Verification**:
   - Every change must pass without warnings:
     ```bash
     cargo clippy --all-targets -- -D warnings
     cargo fmt --check
     ```

---

## 2. Quantitative Size Limits & Modularity

Keep files and functions small, focused, and single-purpose:

| Metric | Target | Hard Limit | Action if Exceeded |
| :--- | :--- | :--- | :--- |
| **File Length** | 150–250 lines | **350 lines** | Split into submodules by domain/role |
| **Function Length** | 15–30 lines | **50 lines** | Extract helper functions or pipeline stages |
| **Functions per File** | 4–6 functions | **8 functions** | Decompose into separate responsibility files |
| **Files per Folder** | 3–5 files | **7 files** | Create a subfolder with cohesive domain grouping |
| **Parameters per Function** | 1–3 parameters | **4 parameters** | Group related inputs into a dedicated config/options struct |

---

## 3. Role-Based Folder Hierarchy

Organize `src/` by architectural role rather than dumping all logic into flat files:

```
src/
├── main.rs / lib.rs         # Entrypoint & module tree declarations only
├── error.rs                 # Centralized error types & conversions
├── cli/                     # CLI parsing, flag definitions & argument validation
│   ├── mod.rs
│   └── args.rs
├── domain/                  # Pure data structures, enums & domain models
│   ├── mod.rs
│   └── models.rs
├── services/                # Core business logic, pipelines & algorithms
│   ├── mod.rs
│   └── runner.rs
├── client/ (or api/)        # External network/HTTP calls, REST clients
│   ├── mod.rs
│   └── kaggle.rs
├── storage/ (or cache/)     # Local state persistence, disk cache, serialization
│   ├── mod.rs
│   └── ledger.rs
└── ui/                      # Terminal rendering, spinners, progress bars, formatting
    ├── mod.rs
    └── display.rs
```

### Module Boundary Rules:
- **`domain`** must remain pure and free of I/O or network dependencies.
- **`client`** handles raw HTTP requests and returns domain models or typed errors.
- **`ui`** only handles output formatting; business logic must never reside in UI formatters.
- **`storage`** abstracts file persistence behind clear read/write APIs.

---

## 4. DRY (Don't Repeat Yourself) & Scalability

1. **Centralize Shared Logic**:
   - Repeated string formatting, duration calculations, or byte-size conversions must live in a dedicated utility module (e.g., `ui::format` or `util.rs`).
   - URLs, default timeouts, and buffer sizes must be declared as `const` values at module heads, never hardcoded inline.
2. **Trait-Driven Abstractions**:
   - Use traits to define shared behavior across different storage or client implementations.
   - Avoid duplicate `match` arms across multiple files; encapsulate the dispatch logic in a method on the enum itself.
3. **Prefer Functional Combinators**:
   - Use `map`, `and_then`, `filter_map`, `find`, and iterator pipelines instead of verbose mutable loop accumulators when clarity is preserved.

---

## 5. Robust Error Handling (No Panics)

1. **No Production Panics**:
   - Never use `.unwrap()` or `.expect()` in library, service, or API code.
   - Reserve `.expect()` strictly for unit test assertions or guaranteed invariants (with a detailed rationale in the message).
2. **Propagate Errors via `?`**:
   - Return `Result<T, AppError>` and propagate errors using the `?` operator.
   - Provide contextual information when errors occur (e.g. using `thiserror` or descriptive error variants).

---

## 6. Pre-Commit Checklist

Before completing any task or committing changes:

```bash
# 1. Format code to standard style
cargo fmt

# 2. Verify zero compiler or Clippy warnings
cargo clippy --all-targets -- -D warnings

# 3. Ensure all unit and integration tests pass
cargo test --all-targets
```
