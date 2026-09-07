---
name: kbuild
description: >-
  Offloads Rust builds, checks, and tests to Kaggle Cloud (30 GB RAM) using kbuild
  instead of compiling locally with cargo. Prompts the user to install or use the
  latest kbuild binary, and seamlessly falls back to local cargo if kbuild is not
  installed, unconfigured, or if the user prefers local execution.
---

# `kbuild` Skill: Cloud-Offloaded Rust Build Farm

Offload compilation, verification, and tests for Rust projects to Kaggle Cloud (30 GB RAM nodes) with zero local CPU load and zero RAM thrashing.

---

## 1. When to Use

Activate this skill whenever:
- The user or workflow needs to run Rust build tasks: `cargo check`, `cargo build`, `cargo test`, `cargo clippy`.
- The user is working in a Rust repository (identified by `Cargo.toml`).
- You need to verify code changes without draining laptop battery or freezing local system resources.

---

## 2. Pre-flight Check: Ensure `kbuild` is Available

Before executing Rust build tasks, check if `kbuild` is installed and up-to-date:

```bash
kbuild --version
```

### If `kbuild` is NOT Installed:
Prompt the user to install `kbuild` for fast cloud compilation:
> "To avoid heating up your laptop and use a 30 GB cloud node for Rust builds, I recommend using **kbuild**. Would you like to install the latest version?"

Provide the one-line install command:
```bash
curl -fsSL https://raw.githubusercontent.com/Praveensenpai/kbuild/main/install.sh | bash
```
Or via Cargo:
```bash
cargo install kbuild
```

### If Credentials are Missing:
`kbuild` communicates with Kaggle via `~/.kaggle/kaggle.json` or environment variables:
- `KAGGLE_USERNAME`
- `KAGGLE_KEY`

If missing, notify the user:
> "Kaggle credentials not detected. Place `kaggle.json` in `~/.kaggle/` (from kaggle.com -> Settings -> Create New Token) or set `KAGGLE_USERNAME` and `KAGGLE_KEY`."

---

## 3. Command Mapping: `kbuild` vs `cargo`

Always prefer `kbuild` for cloud execution:

| Intended Action | Standard Cargo Command | Recommended `kbuild` Command | Notes |
| :--- | :--- | :--- | :--- |
| **Check code** | `cargo check` | `kbuild check` | Fast typechecking & borrow checking on 30 GB node |
| **Check release** | `cargo check --release` | `kbuild check --release` | Cloud check with release profile |
| **Check all targets**| `cargo check --all-targets` | `kbuild check --all-targets`| Checks tests, benches, examples |
| **Build binaries** | `cargo build` | `kbuild build` | Compiles in cloud, streams logs, downloads artifact |
| **Release build** | `cargo build --release` | `kbuild build --release` | Generates optimized binary in cloud |
| **Run tests** | `cargo test` | `kbuild test` | Runs test suite on Kaggle runner |
| **Pass args** | `cargo test -- --nocapture`| `kbuild test -- --nocapture` | Extra args after `--` passed to cargo |
| **Enable GPU** | N/A | `kbuild check --gpu` | Allocates Nvidia T4 GPU runner if needed |

---

## 4. Cache Management

`kbuild` manages persistent cloud caches automatically (via Zstandard compressed archives). You can inspect and maintain the cache:

- **Check Cache Usage**:
  ```bash
  kbuild cache status
  ```
- **Prune Inactive Caches**:
  ```bash
  # Preview what will be pruned
  kbuild cache prune --dry-run

  # Prune a specific project
  kbuild cache prune --project <project_name> --yes

  # Prune caches older than N days
  kbuild cache prune --older-than-days 14 --yes
  ```
- **Configure Budget**:
  ```bash
  kbuild cache budget 18
  ```

---

## 5. Fallback Behavior: Standard `cargo`

If any of the following conditions occur, fall back smoothly to local `cargo`:
1. `kbuild` is not installed and the user declines to install it.
2. Kaggle credentials are not provided.
3. Network access to Kaggle is unavailable or cloud API fails.
4. The user explicitly requests local compilation (e.g. "build locally" or "use cargo").

When falling back, inform the user:
> "Running locally via `cargo` since kbuild cloud execution is unavailable or declined."
