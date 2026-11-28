# Unified 5-Crate Release Process (`xtask release`)

All public crates in this workspace are versioned and released together under a single tagged release (`vX.Y.Z`) orchestrated by `cargo run -p xtask -- release`.

## 1. Crate Inventory & Publish Order (3-Tier DAG)

| Tier | Crate | Manifest | Type | Role |
| --- | --- | --- | --- | --- |
| 1 | `lisp-vm` | [Cargo.toml](../crates/lisp-vm/Cargo.toml) | `[lib]` | Low-level Elisp-compatible VM, GC, bytecode engine & `isolate_vm` |
| 1 | `lisp-std` | [Cargo.toml](../crates/lisp-std/Cargo.toml) | `[lib]` | 100% pure-Rust (`#![forbid(unsafe_code)]`) standard library batteries |
| 1 | `femacs` | [Cargo.toml](../cli/Cargo.toml) | `[[bin]]` (`femacs`, `fe`) | macOS Fe editor CLI launchers |
| 2 | `lisp` | [Cargo.toml](../crates/lisp/Cargo.toml) | `[lib]` | High-level embeddable No-GIL Lisp runtime (depends on `lisp-vm`, `lisp-std`) |
| 3 | `elisp` | [Cargo.toml](../crates/elisp/Cargo.toml) | `[[bin]]` (`elisp`) | Standalone general-purpose CLI, package manager & TTY REPL (depends on `lisp`) |

### `v0.0.1` Baseline Note

- `femacs 0.0.1` was published to crates.io earlier (`cli/`) and is unified into `v0.0.1` (GitHub Release + tag `v0.0.1`) using `--skip-already-published`, without re-uploading `femacs 0.0.1`.
- Starting with the next release (`v0.0.2+`), `xtask release bump` and `xtask release publish` update and publish **all 5 crates simultaneously** in a single unified flow.

## 2. Prerequisites

1. Clean `master` working tree (`git status --short` empty before final publish).
2. Valid GPG key configured for `git commit -S` and `git tag -s`.
3. Valid GitHub CLI authentication (`gh auth status`) and crates.io token (`~/.cargo/credentials.toml`, never inspected or printed in logs).
4. Independent `README.md` and `LICENSE` in every crate with **zero `../` relative links** (all cross-crate links use `https://crates.io/crates/...` or `https://docs.rs/...`).
5. Website under `docs/` (`docs/index.html` + `docs/pages/` with browser-compiled Eliscript `site.eli` and zero committed `.js`/`.mjs` files) verified in headless Chrome.

## 3. Standardized Release Runbook

Run all builds, tests, and publish commands under the host resource controller (`controlled-run`).

### Step A — Inspect Current State & Bump Version (`0.0.2+`)

```sh
cargo run -p xtask -- release plan --version 0.0.2
cargo run -p xtask -- release bump --version 0.0.2
```

For `v0.0.1` (where all five manifests are already `0.0.1`), `plan` reports `femacs` as already `LIVE on crates.io` and the remaining four crates as pending.

### Step B — Refresh Documentation & GitHub Pages

1. Update root `README.md`, `docs/i18n/zh-CN/README.md`, each crate's independent `README.md`, `docs/support-matrix.md`, and `docs/design/lisp-core.md`.
2. Update `docs/index.html` and `docs/pages/{lisp,start,architecture,docs-hub}.html` (plus `docs/pages/README.md` / `site.eli` as needed). Keep Eliscript live-compiled in browser with zero `.js` files committed under `docs/`.
3. Verify Pages locally in headless Chrome across `1440`, `390`, and `320`px viewports (`0` console/HTTP errors, `0` horizontal overflow, `data-eli-ready="true"`).

### Step C — Run Release Gate & Package Verification

```sh
cargo run -p xtask -- release check --version 0.0.1 --allow-dirty
```

`xtask release check` enforces:
- All 5 crate versions and sibling dependency pins (`lisp -> lisp-vm, lisp-std`; `elisp -> lisp`) equal `<X.Y.Z>`.
- Required metadata (`homepage = "https://femacs.com/"`, `repository`, `readme = "README.md"`, `LICENSE`, zero `../` links inside crate READMEs).
- Zero `.js`/`.mjs` files under `docs/`.
- Static gates (`tools/check-own-voice.py` and `tools/check-crate-boundaries.py`).
- Full `cargo package` verification on Tier 1 crates (`lisp-vm`, `lisp-std`, `femacs`) and multi-crate `cargo package --no-verify` across all 5 crates (`< 10 MiB` each).

Also run `cargo test -p xtask --bin xtask`, the workspace test suites, and `fe-test --suite all --no-cache`, then clean `target/`, `build/*`, and `tools/__pycache__`, run all 6 static gates, and GPG-sign commit & push to `origin/master`.

### Step D — Execute Publication, Signed Tag & GitHub Release

Preview first (read-only):

```sh
cargo run -p xtask -- release publish --version 0.0.1 --skip-already-published
```

Execute on clean `master` (pass `--skip-already-published` only for `0.0.1` where `femacs 0.0.1` is already live; omit it for `v0.0.2+`):

```sh
cargo run -p xtask -- release publish --version 0.0.1 --execute --skip-already-published
```

This single command:
1. Confirms clean git status and manifest consistency.
2. Publishes each pending crate in topological order (`lisp-vm` -> `lisp-std` -> `femacs` -> `lisp` -> `elisp`) and polls the crates.io sparse index (`https://index.crates.io/...`) until each tier is queryable before publishing dependents.
3. Creates and verifies GPG-signed tag `git tag -s v<X.Y.Z>` on `HEAD` and pushes `origin v<X.Y.Z>`.
4. Generates standardized release notes (`xtask release notes`) and runs `gh release create v<X.Y.Z> --verify-tag`.

## 4. Recovery Notes

- If crates.io returns HTTP `429` (rate limit) mid-tier after pushing earlier crates, wait for the retry window and re-run `xtask release publish --version <X.Y.Z> --execute --skip-already-published` — already-published crates are skipped safely while pending tiers, tagging, and the GitHub Release complete idempotently.
- Remove any temporary `target/` artifacts produced during `cargo package` / `cargo publish`, verify `build/` is empty, record the release in `maintenance/cleanup/reports/`, `maintenance/cleanup/LOG.md`, and `docs/status.md`, and GPG-sign push the final ledger commit (`[skip ci]`).
