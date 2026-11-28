# Unified Multi-Channel Release Process (`xtask release`)

All public workspace crates, prebuilt release archives, the zero-dependency npm launcher, the Homebrew tap, and the shell installer are versioned and released together under a single signed tag (`vX.Y.Z`) orchestrated by `cargo run -p xtask -- release`.

## 1. Distribution & Crate Inventory

| Channel / Crate | Manifest / Location | Type | Role |
| --- | --- | --- | --- |
| `lisp-vm` | [Cargo.toml](../crates/lisp-vm/Cargo.toml) | Rust `[lib]` (Tier 1) | Low-level Elisp-compatible VM, GC, bytecode engine & `isolate_vm` |
| `lisp-std` | [Cargo.toml](../crates/lisp-std/Cargo.toml) | Rust `[lib]` (Tier 1) | 100% pure-Rust (`#![forbid(unsafe_code)]`) standard library batteries |
| `femacs` | [Cargo.toml](../cli/Cargo.toml) | Rust `[[bin]]` (Tier 1) | macOS Fe editor CLI launchers (`femacs`, `fe`) |
| `lisp` | [Cargo.toml](../crates/lisp/Cargo.toml) | Rust `[lib]` (Tier 2) | High-level embeddable No-GIL Lisp runtime (depends on `lisp-vm`, `lisp-std`) |
| `elisp` | [Cargo.toml](../crates/elisp/Cargo.toml) | Rust `[[bin]]` (Tier 3) | Standalone general-purpose CLI, package manager & TTY REPL (depends on `lisp`) |
| Prebuilt archives | GitHub Release (`SHA256SUMS.txt`) | `.tar.gz` (`elisp`, `femacs`, `fe`) | `aarch64-apple-darwin`, `x86_64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu` |
| `femacs` (npm) | [packaging/npm/femacs/](../packaging/npm/femacs/package.json) | Eliscript `.eli` -> ESM wrapper | Zero runtime deps; bundles in `prepack`, downloads SHA-256 verified release archives lazily |
| `liuchong/tap/elisp` | [packaging/homebrew/elisp.rb](../packaging/homebrew/elisp.rb) | Homebrew Formula | Installs standalone `elisp` CLI & REPL via `liuchong/homebrew-tap` |
| `liuchong/tap/femacs` | [packaging/homebrew/femacs.rb](../packaging/homebrew/femacs.rb) | Homebrew Formula | Installs macOS Fe editor launchers (`femacs`, `fe`) via `liuchong/homebrew-tap` |
| `install.sh` | [docs/install.sh](../docs/install.sh) | POSIX shell (`femacs.com/install.sh`) | One-line SHA-256 verified install into `~/.local/bin` (`--only elisp|femacs`) |

### `v0.0.1` Bootstrap Note

- `femacs 0.0.1` was published to crates.io earlier (`cli/`) and unified into `v0.0.1` using `--skip-already-published`.
- Tag `v0.0.1` stays pinned at its original release commit (`874a28d0`); the `x86_64` prebuilt archives attached to `v0.0.1` include the portable non-aarch64 fallback in `NEAR_STACK_TOP!` from `CLN320`.
- Starting with `v0.0.2+`, a single tag drives `xtask release` across all 5 crates, prebuilt archives, npm `femacs`, `liuchong/homebrew-tap`, and `docs/install.sh` together.

## 2. Prerequisites

1. Clean `master` working tree (`git status --short` empty before final publish).
2. Valid GPG key configured for `git commit -S` and `git tag -s`.
3. Valid GitHub CLI authentication (`gh auth status`), crates.io token, and npm login (`npm whoami`).
4. Cross-compile toolchains installed for release archives (`rustup target add` + Zig `0.17+` for `aarch64`/`x86_64` Linux via `tools/zig-linux-linker.sh`).
5. Independent `README.md` and `LICENSE` in every crate and in `packaging/npm/femacs/`, with **zero `../` relative links** and **zero `.js`/`.mjs` files tracked in Git** under `docs/` or `packaging/npm/femacs/`.
6. Website under `docs/` verified in headless Chrome (`1440`, `390`, `320`px viewports, `data-eli-ready="true"`).

## 3. Standardized One-Tag Release Runbook (`v0.0.2+`)

Run all builds, tests, and publish commands under the host resource controller (`controlled-run`).

### Step A — Bump All Manifests & Installers

```sh
cargo run -p xtask -- release plan --version 0.0.2
cargo run -p xtask -- release bump --version 0.0.2
```

`bump` updates all 5 `Cargo.toml` manifests + sibling dependency pins, `packaging/npm/femacs/package.json`, `docs/install.sh`, and `Cargo.lock`.

### Step B — Cross-Compile Release Archives & Sync Distribution Manifests

Build `--release -p elisp -p femacs` for each target (`aarch64-apple-darwin`, `x86_64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu` via `tools/zig-linux-linker.sh`), pack flat `elisp femacs fe LICENSE` into `build/release-assets/femacs-v<X.Y.Z>-<triple>.tar.gz`, write `build/release-assets/SHA256SUMS.txt`, and sync:

```sh
cargo run -p xtask -- release sync-dist --version 0.0.2 --checksums build/release-assets/SHA256SUMS.txt
```

`sync-dist` deterministically regenerates `packaging/npm/femacs/manifest.json`, `packaging/homebrew/elisp.rb`, and `packaging/homebrew/femacs.rb` from `SHA256SUMS.txt`.

### Step C — Refresh Docs, Test Packaging & Run Release Gates

1. Update `README.md`, `docs/i18n/zh-CN/README.md`, crate/npm READMEs, `docs/support-matrix.md`, and `docs/index.html` + `docs/pages/`.
2. Verify the Eliscript npm package (`(cd packaging/npm/femacs && npm install --ignore-scripts && npm test)`).
3. Run release check and workspace tests, then clean temporary artifacts (`target/`, `build/npm-verify/`, `packaging/npm/femacs/{node_modules,dist,.tmp,package-lock.json}`):

```sh
cargo run -p xtask -- release check --version 0.0.2 --allow-dirty
cargo test -p xtask --bin xtask
```

`xtask release check` enforces Cargo metadata, npm `package.json`/`manifest.json`/`README.md`/`LICENSE` (0 runtime deps, 0 tracked `.js`/`.mjs`), Homebrew `ruby -c`, `sh -n docs/install.sh`, `check-own-voice.py`, `check-crate-boundaries.el`, and multi-crate `cargo package` size limits (`< 10 MiB`).

Commit and GPG-sign push to `origin/master` (`[skip ci]`).

### Step D — Single-Tag Multi-Channel Publish

Preview (read-only):

```sh
cargo run -p xtask -- release publish --version 0.0.2 \
  --assets-dir build/release-assets \
  --with-npm \
  --homebrew-tap-dir build/homebrew-tap
```

Execute on clean `master`:

```sh
cargo run -p xtask -- release publish --version 0.0.2 --execute \
  --assets-dir build/release-assets \
  --with-npm \
  --homebrew-tap-dir build/homebrew-tap
```

This single command:
1. Publishes & sparse-index verifies all 5 crates in topological order (`lisp-vm` -> `lisp-std` -> `femacs` -> `lisp` -> `elisp`).
2. Creates and verifies GPG-signed tag `v<X.Y.Z>` on `HEAD` and pushes `origin v<X.Y.Z>`.
3. Creates GitHub Release `v<X.Y.Z>` with standardized multi-channel release notes and uploads all `.tar.gz` archives + `SHA256SUMS.txt`.
4. Tests and publishes `femacs@<X.Y.Z>` (`packaging/npm/femacs`) to npm.
5. Copies `packaging/homebrew/femacs.rb` to `liuchong/homebrew-tap`, GPG-sign commits, and pushes `origin/master`.

## 4. Cleanup & Recovery

- After publishing, remove `target/`, `build/*`, `tools/__pycache__`, and `packaging/npm/femacs/{node_modules,dist,.tmp,package-lock.json}`, re-run the static gates, record the release in `maintenance/cleanup/reports/`, `maintenance/cleanup/LOG.md`, and `docs/status.md`, and GPG-sign push `[skip ci]`.
- If crates.io returns HTTP `429` or npm requires interactive 2FA mid-run, finish authentication/retry with `--skip-already-published` (and `--skip-tag` if `v<X.Y.Z>` is already pushed).
