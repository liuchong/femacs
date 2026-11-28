# CLN319 — Unified `v0.0.1` Crates.io Publication, Signed Tag, GitHub Release & `eliscript@0.1.4` Pages Upgrade (2026-11-28)

## Summary

Completed the unified `v0.0.1` public release across all five workspace crates (`lisp-vm`, `lisp-std`, `femacs`, `lisp`, and `elisp`), upgraded the `femacs.com` GitHub Pages site (`docs/index.html` + `docs/pages/`) to `eliscript@0.1.4` with real headless Chrome verification across desktop and mobile viewports, implemented the programmatic 5-crate release driver (`xtask release`), created GPG-signed tag `v0.0.1` and GitHub Release `v0.0.1`, retired temporary build/packaging artifacts, and verified post-publish hygiene.

## Changes

- **Programmatic 5-Crate Release Workflow (`xtask/src/release.rs`, `xtask/src/release_tests.rs`, `docs/release-process.md`)**:
  - Implemented `#![forbid(unsafe_code)]` `cargo run -p xtask -- release <plan|bump|check|notes|publish> --version <X.Y.Z>` covering `PUBLISH_CRATES` in 3-tier topological order (`lisp-vm`, `lisp-std`, `femacs`, `lisp`, `elisp`), checking crates.io sparse index state (`--skip-already-published` for counting previously published `femacs 0.0.1` into `v0.0.1` while synchronizing future `0.0.2+` five-crate releases), enforcing zero broken `](../` links in crate READMEs, zero `.js`/`.mjs` under `docs/`, `check-own-voice.py`, `check-crate-boundaries.py`, 5-crate `cargo package` size/metadata checks, GPG-signed tag creation/verification, and `gh release create`.
  - Registered `xtask/src/release.rs` and `xtask/src/release_tests.rs` in `tools/unsafe-boundary-manifest.json` (`178` `forbid_unsafe_files`) and updated `maintenance/tooling-inventory.json`.
- **Crate Metadata, Independent READMEs & Documentation**:
  - Updated `Cargo.toml` (`homepage = "https://femacs.com/"`, cleaned `lisp-std` description and `lisp` categories `["compilers", "concurrency", "development-tools"]`) and refreshed all 4 standalone `README.md`s (`crates/{lisp-vm,lisp-std,lisp,elisp}/README.md`) with zero `../` relative links and crates.io/docs.rs badges.
  - Updated `README.md`, `docs/i18n/zh-CN/README.md`, `docs/support-matrix.md`, `docs/design/lisp-core.md`, and created `docs/release-process.md`.
- **GitHub Pages Upgrade (`docs/index.html`, `docs/pages/`, `eliscript@0.1.4`)**:
  - Added `docs/pages/lisp.html` (`Standalone Lisp · FEmacs`), responsive `.crate-grid`/`.spec-table`/`.table-wrap` styles in `docs/pages/assets/site.css`, unified 5-item nav & release/crate footer links across all 5 HTML pages, `15`-card `docs/pages/docs-hub.html`, updated `docs/pages/README.md` and `docs/pages/site.eli`, and upgraded live browser runtime compiler to published `eliscript@0.1.4` (`unpkg.com` HTTP `200`).

## Verification & Release Evidence

- **`xtask release` Tests & 5-Crate Check (`run_id=bfa79959f00947b08108141238dbde3c` & `3660a665087143abafbabf03994d768b`, `exit=0`)**:
  - `cargo test -p xtask --bin xtask`: `123` passed (`119` existing + `4` `release::tests`).
  - `cargo run -p xtask -- release check --version 0.0.1 --allow-dirty`: full leaf `cargo package` on `lisp-vm`, `lisp-std`, and `femacs` plus 5-crate `cargo package --no-verify` passed (`lisp-vm` `4,030,735` B / `3.84 MiB`, `lisp-std` `58,493` B / `0.06 MiB`, `femacs` `17,222` B / `0.02 MiB`, `lisp` `1,138,205` B / `1.09 MiB`, `elisp` `48,610` B / `0.05 MiB`), `check-own-voice.py` `0` failures, `check-crate-boundaries.py` `ok (standalone=8)`.
- **Headless Chrome Pages Verification (`eliscript@0.1.4`, `run_id=8f825a6666dc412fadb47c7ccc0ba1b2`, `exit=0`, `16.733s`)**:
  - Verified all `5` routes (`/index.html`, `/pages/lisp.html`, `/pages/start.html`, `/pages/architecture.html`, `/pages/docs-hub.html`) across `1440x900`, `390x844`, and `320x800` viewports: `Eliscript 0.1.4 · live` mounted cleanly on all `15` checks with `0` console/JS errors, `0` HTTP `>=400` responses, `0` broken local links/images, `0` horizontal overflow, working EN/ZH switch, workflow tabs, and `15`-card (`5 / 15` Build) `docs-hub` filter/search.
- **Pre-Release 6 Static Gates & Clean Commit (`run_id=eadd1dd4c7a64d678345546388e54bec`, `exit=0`, `14.979s`)**:
  - `check-own-voice.py` (`0` failures), `check-ledger-ticks.py` (`58` bound / `0` failures), `test-tooling-modernization.py` (`33/33`), `check-unsafe-boundary.py` (`474` active files, `178` `forbid_unsafe_files`, `6,446/6,446` `SAFETY:`, `0` `static_mut_items`, `0` errors), `check-crate-boundaries.py` (`ok standalone=8`), `test-repository-bootstrap.py` (`4/4`), `git diff --check`, and `0` `.js`/`.mjs` under `docs/`.
  - Pre-release commit `874a28d0d9a2010f1b4b67e8c907177486f2e845` GPG-signed and pushed to `origin/master`.
- **Live Publication (`run_id=2875d24452f54ecd94c99144482ff1ea`, `exit=0`, `142.34s`)**:
  - Crates.io `0.0.1` live across all 5 packages:
    - `lisp-vm 0.0.1`: https://crates.io/crates/lisp-vm/0.0.1
    - `lisp-std 0.0.1`: https://crates.io/crates/lisp-std/0.0.1
    - `femacs 0.0.1`: https://crates.io/crates/femacs/0.0.1 (previously published; skipped upload and unified into `v0.0.1`)
    - `lisp 0.0.1`: https://crates.io/crates/lisp/0.0.1
    - `elisp 0.0.1`: https://crates.io/crates/elisp/0.0.1
  - Signed tag `v0.0.1`: tag object `5979d6d94f60602cb861d94493785f1b4742ba5e` -> commit `874a28d0d9a2010f1b4b67e8c907177486f2e845` (`git tag -v v0.0.1` valid EDDSA signature, pushed to `origin`).
  - GitHub Release: https://github.com/liuchong/femacs/releases/tag/v0.0.1 (`isDraft=false`, `isPrerelease=false`).
  - GitHub Pages (`repos/liuchong/femacs/pages`): `https://femacs.com/`, `status: "built"`, `source: master:/docs`.

## Platform Boundary Note

- Windows targets (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) remain **cross-compile architecture verified ONLY** (no physical Windows host runtime verification was performed), as explicitly documented in all published READMEs, Pages, and GitHub Release notes.

## Post-Publish Cleanup

- Retired transient `target/` created during `xtask release publish` and `cargo test`; verified `build/` is empty, `target/` and `tools/__pycache__` are absent, `dist/lisp` is intact, `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty, and `check-own-voice.py` / `check-ledger-ticks.py` / `git diff --check` are clean.
