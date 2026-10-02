use std::path::Path;
use syn::spanned::Spanned;
use syn::{Expr, Item, ItemFn, ItemImpl, Stmt};

use crate::domain::audit::{Violation, ViolationKind};
use crate::domain::digest::ModuleDigest;
use crate::domain::ModuleRole;

/// Analyzes a Rust source file for clean-code violations and extracts AST symbols.
pub struct RustAnalyzer;

impl RustAnalyzer {
    /// Audits a Rust source file for function length, nesting depth, and forbidden attributes.
    pub fn audit_source(
        path: &Path,
        content: &str,
        max_fn_lines: usize,
        max_nesting_depth: usize,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();
        let parsed = match syn::parse_file(content) {
            Ok(file) => file,
            Err(_) => return violations, // Skip unparseable files without crashing
        };

        check_forbidden_attributes(&parsed.items, path, &mut violations);

        for item in &parsed.items {
            match item {
                Item::Fn(f) => {
                    check_function(f, path, max_fn_lines, max_nesting_depth, &mut violations);
                }
                Item::Impl(i) => {
                    check_impl_block(i, path, max_fn_lines, max_nesting_depth, &mut violations);
                }
                _ => {}
            }
        }

        violations
    }

    /// Extracts module symbols, public items, and signatures for CODEBASE.md.
    pub fn extract_digest(path: &Path, content: &str) -> ModuleDigest {
        let relative_path = path.to_path_buf();
        let role = ModuleRole::infer_from_path(&path.to_string_lossy());
        let lines = content.lines().count();

        let mut types = Vec::new();
        let mut functions = Vec::new();
        let mut imports = Vec::new();

        if let Ok(file) = syn::parse_file(content) {
            for item in file.items {
                extract_item_symbols(item, &mut types, &mut functions, &mut imports);
            }
        }

        let responsibility = format!(
            "Core {} logic in {}",
            role.as_str(),
            relative_path.display()
        );

        ModuleDigest {
            relative_path,
            role,
            lines,
            responsibility,
            types,
            functions,
            imports,
        }
    }
}

fn extract_item_symbols(
    item: Item,
    types: &mut Vec<String>,
    functions: &mut Vec<String>,
    imports: &mut Vec<String>,
) {
    match item {
        Item::Use(u) => {
            let text = quote::quote!(#u).to_string();
            imports.push(text.replace(';', ""));
        }
        Item::Struct(s) if matches!(s.vis, syn::Visibility::Public(_)) => {
            types.push(format!("pub struct {}", s.ident));
        }
        Item::Enum(e) if matches!(e.vis, syn::Visibility::Public(_)) => {
            types.push(format!("pub enum {}", e.ident));
        }
        Item::Fn(f) if matches!(f.vis, syn::Visibility::Public(_)) => {
            let sig_ref = &f.sig;
            let sig = quote::quote!(#sig_ref).to_string();
            functions.push(sig);
        }
        Item::Impl(i) => extract_impl_methods(i, functions),
        _ => {}
    }
}

fn extract_impl_methods(i: ItemImpl, functions: &mut Vec<String>) {
    for impl_item in i.items {
        if let syn::ImplItem::Fn(method) = impl_item {
            if matches!(method.vis, syn::Visibility::Public(_)) {
                let sig_ref = &method.sig;
                let sig = quote::quote!(#sig_ref).to_string();
                functions.push(sig);
            }
        }
    }
}

fn check_function(
    f: &ItemFn,
    path: &Path,
    max_fn_lines: usize,
    max_nesting_depth: usize,
    violations: &mut Vec<Violation>,
) {
    let fn_name = f.sig.ident.to_string();
    let start_line = f.sig.span().start().line;
    let end_line = f.block.span().end().line;
    let fn_lines = end_line.saturating_sub(start_line) + 1;

    if fn_lines > max_fn_lines {
        violations.push(Violation {
            path: path.to_path_buf(),
            kind: ViolationKind::FunctionTooLong {
                name: fn_name.clone(),
                lines: fn_lines,
                limit: max_fn_lines,
                line_no: start_line,
            },
        });
    }

    let depth = calculate_block_depth(&f.block.stmts, 0);
    if depth > max_nesting_depth {
        violations.push(Violation {
            path: path.to_path_buf(),
            kind: ViolationKind::NestingTooDeep {
                name: fn_name,
                depth,
                limit: max_nesting_depth,
                line_no: start_line,
            },
        });
    }
}

fn check_impl_block(
    i: &ItemImpl,
    path: &Path,
    max_fn_lines: usize,
    max_nesting_depth: usize,
    violations: &mut Vec<Violation>,
) {
    for item in &i.items {
        if let syn::ImplItem::Fn(method) = item {
            let fn_name = method.sig.ident.to_string();
            let start_line = method.sig.span().start().line;
            let end_line = method.block.span().end().line;
            let fn_lines = end_line.saturating_sub(start_line) + 1;

            if fn_lines > max_fn_lines {
                violations.push(Violation {
                    path: path.to_path_buf(),
                    kind: ViolationKind::FunctionTooLong {
                        name: fn_name.clone(),
                        lines: fn_lines,
                        limit: max_fn_lines,
                        line_no: start_line,
                    },
                });
            }

            let depth = calculate_block_depth(&method.block.stmts, 0);
            if depth > max_nesting_depth {
                violations.push(Violation {
                    path: path.to_path_buf(),
                    kind: ViolationKind::NestingTooDeep {
                        name: fn_name,
                        depth,
                        limit: max_nesting_depth,
                        line_no: start_line,
                    },
                });
            }
        }
    }
}

fn calculate_stmt_depth(stmt: &Stmt, current_depth: usize) -> usize {
    match stmt {
        Stmt::Expr(expr, _) => calculate_expr_depth(expr, current_depth),
        Stmt::Local(local) => local
            .init
            .as_ref()
            .map(|init| calculate_expr_depth(&init.expr, current_depth))
            .unwrap_or(current_depth),
        _ => current_depth,
    }
}

fn calculate_block_depth(stmts: &[Stmt], current_depth: usize) -> usize {
    stmts
        .iter()
        .map(|s| calculate_stmt_depth(s, current_depth))
        .max()
        .unwrap_or(current_depth)
}

fn calculate_expr_depth(expr: &Expr, current_depth: usize) -> usize {
    match expr {
        Expr::If(e) => {
            let then_d = calculate_block_depth(&e.then_branch.stmts, current_depth + 1);
            let else_d = if let Some((_, else_expr)) = &e.else_branch {
                calculate_expr_depth(else_expr, current_depth)
            } else {
                current_depth
            };
            then_d.max(else_d)
        }
        Expr::Match(m) => {
            let mut arm_max = current_depth + 1;
            for arm in &m.arms {
                let d = calculate_expr_depth(&arm.body, current_depth + 1);
                arm_max = arm_max.max(d);
            }
            arm_max
        }
        Expr::Loop(l) => calculate_block_depth(&l.body.stmts, current_depth + 1),
        Expr::While(w) => calculate_block_depth(&w.body.stmts, current_depth + 1),
        Expr::ForLoop(f) => calculate_block_depth(&f.body.stmts, current_depth + 1),
        Expr::Block(b) => calculate_block_depth(&b.block.stmts, current_depth + 1),
        _ => current_depth,
    }
}

fn check_forbidden_attributes(items: &[Item], path: &Path, violations: &mut Vec<Violation>) {
    for item in items {
        let attrs = match item {
            Item::Fn(f) => &f.attrs,
            Item::Struct(s) => &s.attrs,
            Item::Enum(e) => &e.attrs,
            Item::Impl(i) => &i.attrs,
            Item::Mod(m) => &m.attrs,
            _ => continue,
        };

        for attr in attrs {
            let formatted = quote::quote!(#attr).to_string();
            if formatted.contains("allow (dead_code)") || formatted.contains("allow (unused)") {
                let line_no = attr.span().start().line;
                violations.push(Violation {
                    path: path.to_path_buf(),
                    kind: ViolationKind::ForbiddenPattern {
                        pattern: formatted,
                        line_no,
                    },
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extract_digest_symbols() {
        let code = r#"
            use std::path::Path;
            pub struct Foo { pub bar: u32 }
            pub enum State { Active, Idle }
            pub fn run_task() -> bool { true }
        "#;
        let digest = RustAnalyzer::extract_digest(&PathBuf::from("src/domain/test.rs"), code);
        assert_eq!(digest.types.len(), 2);
        assert_eq!(digest.functions.len(), 1);
        assert_eq!(digest.imports.len(), 1);
    }

    #[test]
    fn test_audit_detects_forbidden_attribute() {
        let code = r#"
            #[allow(dead_code)]
            fn unused_function() {}
        "#;
        let violations = RustAnalyzer::audit_source(&PathBuf::from("src/test.rs"), code, 60, 3);
        assert!(!violations.is_empty());
        assert!(matches!(
            violations[0].kind,
            ViolationKind::ForbiddenPattern { .. }
        ));
    }
}
