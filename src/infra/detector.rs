use std::path::Path;

use crate::domain::{Stack, StackSet};

/// Root-level markers that identify a technology stack.
const RUST_MARKERS: &[&str] = &["Cargo.toml"];
const PYTHON_MARKERS: &[&str] = &[
    "pyproject.toml",
    "requirements.txt",
    "uv.lock",
    "setup.py",
    "Pipfile",
];
const MOBILE_MARKERS: &[&str] = &[
    "build.gradle.kts",
    "build.gradle",
    "settings.gradle.kts",
    "AndroidManifest.xml",
];
const SHELL_MARKERS: &[&str] = &["Makefile", "makefile"];

/// Inspects a project directory and reports the stacks it uses.
///
/// Always includes `Generic`. Detection is marker-based and shallow so it
/// stays fast; it never returns `Optional`.
pub fn detect(dir: &Path) -> StackSet {
    let mut set = StackSet::new();
    set.insert(Stack::Generic);

    if has_any(dir, RUST_MARKERS) || has_extension(dir, "rs") {
        set.insert(Stack::Rust);
    }
    if has_any(dir, PYTHON_MARKERS) || has_extension(dir, "py") {
        set.insert(Stack::Python);
    }
    if has_any(dir, MOBILE_MARKERS) || has_extension(dir, "kt") {
        set.insert(Stack::Mobile);
    }
    if has_any(dir, SHELL_MARKERS) || has_extension(dir, "sh") {
        set.insert(Stack::Shell);
    }

    set
}

fn has_any(dir: &Path, names: &[&str]) -> bool {
    names.iter().any(|name| dir.join(name).exists())
}

/// True when a top-level file uses the given extension (without the dot).
fn has_extension(dir: &Path, ext: &str) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    entries.flatten().any(|entry| {
        entry
            .path()
            .extension()
            .is_some_and(|found| found.eq_ignore_ascii_case(ext))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempDir {
        path: std::path::PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "karakuri-detector-{}-{}",
                std::process::id(),
                n
            ));
            fs::create_dir_all(&path).expect("create temp dir");
            Self { path }
        }

        fn touch(&self, name: &str) {
            fs::write(self.path.join(name), "").expect("write marker");
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn detects_rust_from_cargo_toml() {
        let dir = TempDir::new();
        dir.touch("Cargo.toml");
        let stacks = detect(&dir.path);
        assert!(stacks.contains(Stack::Rust));
        assert!(stacks.contains(Stack::Generic));
    }

    #[test]
    fn detects_python_from_top_level_source_file() {
        let dir = TempDir::new();
        dir.touch("main.py");
        let stacks = detect(&dir.path);
        assert!(stacks.contains(Stack::Python));
    }

    #[test]
    fn detects_multiple_stacks() {
        let dir = TempDir::new();
        dir.touch("build.gradle.kts");
        dir.touch("Makefile");
        let stacks = detect(&dir.path);
        assert!(stacks.contains(Stack::Mobile));
        assert!(stacks.contains(Stack::Shell));
    }

    #[test]
    fn empty_directory_is_generic_only() {
        let dir = TempDir::new();
        let stacks = detect(&dir.path);
        assert!(stacks.contains(Stack::Generic));
        assert!(!stacks.contains(Stack::Rust));
        assert!(!stacks.contains(Stack::Python));
    }
}
