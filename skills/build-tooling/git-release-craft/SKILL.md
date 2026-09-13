---
name: git-release-craft
description: >-
  Automates the complete Git release lifecycle: atomic conventional commits, semantic version
  bumps, mandatory multi-arch GitHub Actions workflows for binary apps, rich aesthetic release
  descriptions, tag creation, and autonomous closed-loop workflow verification and self-healing.
---

# `git-release-craft` Skill: Release Lifecycle & Workflow Verification

Automates production releases with aesthetic, well-formatted release notes, mandatory multi-arch GitHub Actions workflows for binary applications, and autonomous verification and self-healing of CI/CD pipelines.

---

## 1. The Golden Rule: Never Fire-and-Forget & Own Failures

Creating a release is not finished when `git push` or `gh release create` exits. You must:
1. **Ensure Release Workflows Exist**: Any project producing standalone compiled binaries must have a verified `.github/workflows/release.yml`.
2. **Craft a Proper Description**: Never use blank releases or bare `--generate-notes`. Every release must follow the aesthetic highlight format.
3. **Actively Monitor Workflows**: Watch the triggered GitHub Actions CI/Release pipelines until completion.
4. **Autonomously Heal Failures**: If GitHub Actions or release builds fail, diagnose and fix them immediately without asking for permission to resolve release errors.
5. **Verify Release Assets**: Confirm that compiled binaries, packages, or checksums are physically generated and attached to the release.

---

## 2. Mandatory Workflow for Binary Projects

Before publishing a release for any repository that produces a standalone binary (e.g. Rust CLI or application, Go, C/C++):

1. **Verify Workflow Existence**:
   Check if `.github/workflows/release.yml` exists. If missing, create it before tagging.

2. **Standard Multi-Arch Release Workflow Template (Rust Example)**:
   Place at `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    branches:
      - main
    tags:
      - 'v*'
  workflow_dispatch:

permissions:
  contents: write

jobs:
  build-release:
    name: Build (${{ matrix.target }})
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            archive_name: <app>-x86_64-unknown-linux-gnu.tar.gz
          - target: aarch64-unknown-linux-gnu
            archive_name: <app>-aarch64-unknown-linux-gnu.tar.gz

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2
        with:
          shared-key: "release-${{ matrix.target }}"
          save-if: true

      - name: Install cross-compilation toolchain for ARM64
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: |
          sudo apt-get update
          sudo apt-get install -y gcc-aarch64-linux-gnu

      - name: Build release binary
        run: |
          if [ "${{ matrix.target }}" = "aarch64-unknown-linux-gnu" ]; then
            export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
          fi
          cargo build --release --target ${{ matrix.target }}

      - name: Package release archive
        if: startsWith(github.ref, 'refs/tags/v')
        run: |
          mkdir -p staging
          cp target/${{ matrix.target }}/release/<binary_name> staging/
          cd staging
          tar -czf ../${{ matrix.archive_name }} <binary_name>
          cd ..

      - name: Upload artifact
        if: startsWith(github.ref, 'refs/tags/v')
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.target }}
          path: ${{ matrix.archive_name }}

  publish-release:
    name: Publish GitHub Release
    if: startsWith(github.ref, 'refs/tags/v')
    needs: build-release
    runs-on: ubuntu-latest
    steps:
      - name: Download all built artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts
          merge-multiple: true

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          name: <Project Name> ${{ github.ref_name }}
          files: artifacts/*.tar.gz
          generate_release_notes: true
          append_body: true
```

---

## 3. Pre-Release Quality Checklist

Before tagging or creating a release:

1. **Verify Local Quality**:
   - Rust: `cargo test --all-targets && cargo clippy --all-targets -- -D warnings && cargo fmt --check`
   - Python: `uv run pytest && uv run ruff check && uv run ruff format --check && uv run mypy .`
2. **Bump Semantic Version**:
   - Update `Cargo.toml`, `pyproject.toml`, or `package.json` to target version `vX.Y.Z`.
3. **Commit Manifest Changes**:
   ```bash
   git add Cargo.toml Cargo.lock  # or pyproject.toml / uv.lock
   git commit -m "chore(release): bump to vX.Y.Z"
   ```

---

## 4. Aesthetic Release Note Template (Mandatory)

Every release must have a rich, beautifully structured description matching this format:

```markdown
# 🌸 <Project Name> vX.Y.Z ✨

<One-sentence punchy summary highlighting the theme or milestone of this release>

### 🌟 Key Highlights (or 🌸 What's New in vX.Y.Z)

- **• <Icon> <Feature / Fix Name>**: <Clear, concrete explanation of what changed and its user impact>.
- **• <Icon> <Feature / Fix Name>**: <Clear, concrete explanation of what changed and its user impact>.
- **• <Icon> <Feature / Fix Name>**: <Clear, concrete explanation of what changed and its user impact>.
- **• ✒️ Strict Codebase Quality**: <Mention code quality achievements: 100% tests passing, zero warnings, clean architecture>.

### 🚀 Direct Download & Run

```bash
<One-line curl installer, cargo install, or package manager command>
```
```

### Example Icons to Use:
- `✨` / `🌟` : Major new features
- `⚡` / `🚀` : Performance boosts, cloud offloading
- `📦` : Standalone binaries, asset bundling
- `🤫` / `🛡️` : Silent clutter-free logging, security, safety
- `🌸` / `🎨` : UI/TUI polish, aesthetics, styling
- `🐛` / `🔧` : Bug fixes, stability patches

---

## 5. Tagging & Publishing the Release

```bash
# 1. Create annotated tag locally
git tag -a vX.Y.Z -m "<Project Name> vX.Y.Z: <Short Summary>"

# 2. Push commit and tag to remote
git push origin <branch>
git push origin vX.Y.Z

# 3. Write release notes to a temporary file
cat << 'NOTES_EOF' > /tmp/release_notes.md
<Aesthetic Release Notes from Section 4>
NOTES_EOF

# 4. Create the official GitHub release
gh release create vX.Y.Z \
  --title "🌸 <Project Name> vX.Y.Z ✨" \
  --notes-file /tmp/release_notes.md

rm -f /tmp/release_notes.md
```

---

## 6. Post-Release Verification & Autonomous Self-Healing

Immediately after publishing the release:

### Step 6.1: Retrieve Triggered Actions
```bash
gh run list --limit 4 --json databaseId,name,status,conclusion,headBranch,url
```

### Step 6.2: Watch and Verify Workflows
Wait for all active workflows (`Release`, `CI`, `Build`) to finish:
```bash
gh run watch <run_id>
```

### Step 6.3: Autonomous Failure Recovery Loop (Zero Permission-Seeking)
If any workflow fails (`conclusion: failure`):

> [!IMPORTANT]
> **Do NOT ask the user for permission to fix a failed release workflow!**
> Never ask: *"I made a mistake, would you like me to fix it?"* or *"Do you allow me to fix this?"*.
> The error happened during the release execution initiated by the agent. You are 100% responsible for delivering a working, green release. Fix it autonomously.

**Autonomous Healing Protocol**:
1. **Inspect Failed Logs**:
   ```bash
   gh run view <run_id> --log-failed
   ```
2. **Diagnose Root Cause**:
   - Compiler or clippy errors on matrix targets.
   - Missing cross-compilation linker (e.g. `aarch64-linux-gnu-gcc`).
   - Packaging path or binary naming mismatch.
   - GitHub Actions permission issues (`contents: write`).
3. **Fix Immediately**:
   Apply code or workflow fixes directly in the workspace.
4. **Verify Locally**:
   Run tests and lints locally (`cargo test`, `cargo clippy`, `cargo fmt`).
5. **Update Release Tag & Re-Push**:
   ```bash
   git add -A
   git commit -m "fix(ci): resolve release build failure for <target>"
   git push origin <branch>
   git tag -fa vX.Y.Z -m "<Project Name> vX.Y.Z: <Summary>"
   git push origin vX.Y.Z --force
   ```
6. **Re-watch & Iterate**:
   Monitor the newly triggered workflow run until all jobs are **GREEN**. Repeat until successful.

### Step 6.4: Inspect Uploaded Assets
Confirm that release artifacts are physically attached to the release:
```bash
gh release view vX.Y.Z
```
Verify:
- Pre-compiled binaries/archives exist (e.g. `.tar.gz`, `.zip`, `.whl`).
- Cross-platform targets are present (`x86_64` and `aarch64`).

---

## 7. Stale Tag & Release Cleanup (Hygiene)

When cleaning up older development or superseded tags:
```bash
# Delete GitHub release
gh release delete <tag> -y

# Delete remote git tag
git push origin --delete <tag>

# Delete local tag
git tag -d <tag>
```

---

## 8. Report Format to User

Always provide a verified release report:
1. Release tag & title link.
2. Summary of published highlights.
3. CI/CD workflow status (Jobs passed, run time).
4. List of uploaded artifacts with file sizes.
