# CLN321 — Split Homebrew Tap (`elisp` / `femacs`), Local `elisp 0.0.1` Dogfooding, Eliscript Dispatch & Script Migration Batch 1 (2026-11-28)

## Summary

Split the Homebrew tap into standalone `elisp` and editor-launcher `femacs`/`fe` formulae (`liuchong/homebrew-tap` commit `58239c1`), installed published `elisp 0.0.1` locally via Homebrew for real dogfooding (with zero Homebrew paths or commands committed inside project repositories), dispatched Stage-0 bootstrap compiler migration to the `Eliscript` workspace conversation, completed Batch 1 of migrating repository tooling onto published `elisp 0.0.1` (`tools/check-crate-boundaries.py` retired and replaced by `tools/check-crate-boundaries.el`; `tools/testing/fe-test*.el` switched to `elisp`), refreshed `v0.0.1` docs/Pages/`xtask release` + GitHub Release notes, and recorded concrete `elisp 0.0.1` dogfooding findings (`make-process` isolate child reaping bug for `v0.0.2`).

## Changes & Retired Tooling

- **Split Homebrew Formulae & Installer (`packaging/homebrew/`, `docs/install.sh`, `xtask/src/release*.rs`)**:
  - Added canonical [`packaging/homebrew/elisp.rb`](../../../packaging/homebrew/elisp.rb) (`class Elisp < Formula`, installs standalone `elisp`, tests `elisp --version` & `eval '(+ 20 22)'` = `"42"`).
  - Updated [`packaging/homebrew/femacs.rb`](../../../packaging/homebrew/femacs.rb) (`class Femacs < Formula`, installs `femacs` and `fe` without binary conflict with `elisp`).
  - Pushed GPG-signed commit `58239c1` to `https://github.com/liuchong/homebrew-tap` (`Formula/elisp.rb`, `Formula/femacs.rb`, `README.md`; verified raw HTTPS 200 and local `brew install liuchong/tap/elisp` + `brew test liuchong/tap/elisp`).
  - Added `--only elisp|femacs|all` (`FEMACS_ONLY`) filtering in [`docs/install.sh`](../../../docs/install.sh) (`sh -n` OK).
  - Extended [`xtask/src/release.rs`](../../../xtask/src/release.rs) and [`xtask/src/release_tests.rs`](../../../xtask/src/release_tests.rs) (`HOMEBREW_ELISP_FORMULA`, `render_homebrew_elisp_formula`, split `render_homebrew_formula`, `sync-dist`, `check`, `publish`, `notes`) and executed `gh release edit v0.0.1` while keeping signed tag `v0.0.1` pinned at `874a28d0d9a2010f1b4b67e8c907177486f2e845`.
  - Refreshed [`README.md`](../../../README.md), [`docs/i18n/zh-CN/README.md`](../../../docs/i18n/zh-CN/README.md), [`packaging/npm/femacs/README.md`](../../../packaging/npm/femacs/README.md), [`docs/release-process.md`](../../../docs/release-process.md), [`docs/design/lisp-core.md`](../../../docs/design/lisp-core.md), [`docs/index.html`](../../../docs/index.html), [`docs/pages/lisp.html`](../../../docs/pages/lisp.html), and [`docs/pages/start.html`](../../../docs/pages/start.html).
- **Script Migration Batch 1 & Python Retirement**:
  - Authored pure Lisp [`tools/check-crate-boundaries.el`](../../../tools/check-crate-boundaries.el) (`8,704` B, SHA-256 `499de95363418680b7e263cd11c356b1692bcc3425cc1370fefb9e0d6851c65f`), including `--self-test` (`3/3`), and switched both [`xtask/src/release.rs`](../../../xtask/src/release.rs) (`FEMACS_ELISP` / `elisp`) and [`tools/testing/suites/suite-static.el`](../../../tools/testing/suites/suite-static.el) (`static/crate-boundaries`, `static/fe-test-byte-compile`) to run on published `elisp`.
  - Updated [`tools/testing/fe-test.el`](../../../tools/testing/fe-test.el) and [`tools/testing/fe-test-dsl.el`](../../../tools/testing/fe-test-dsl.el) (`fe-test-find-elisp`, `fe-test-find-exec-runner`) to resolve `FEMACS_ELISP` / `elisp` on `PATH` (falling back to workspace builds if present) and added `(require 'bytecomp)` before `let`-binding `byte-compile-dest-file-function` in `suite-static.el`.
  - **Retired `tools/check-crate-boundaries.py`**: removed legacy Python script (`4,193` B, SHA-256 `b6f812e1802689fa187b717bf8e20b8e15b8c1f4b85007993c20d7b2f8e49694`), registered retirement in [`docs/retired-scaffolding.md`](../../../docs/retired-scaffolding.md) Section 九 (recoverable via `git show 07f77fb0:tools/check-crate-boundaries.py`), and marked `retire` with replacement `tools/check-crate-boundaries.el` in [`maintenance/tooling-inventory.json`](../../../maintenance/tooling-inventory.json).

## Dogfooding Findings (`elisp 0.0.1` -> `v0.0.2`)

- **P0 (`v0.0.2`): Async `make-process`/`start-process` + `accept-process-output` hangs with `<defunct>` zombies**:
  - `elisp 0.0.1` compiles with `isolate_vm`, and `init_headless_host_with_isolate` enters `IsolateScopeGuard` for both `NoGil` and `Gil`, making `is_isolate_active()` always `true`; `init_process_emacs` then skips `catch_child_signal()` (`handler=0x0`), while `wait_reading_process_output` lacks a per-isolate `waitpid(pid, &status, WNOHANG)` reap loop.
  - Synchronous `call-process` completes normally, whereas `fe-test --suite self` (which uses async `make-process` + `accept-process-output`) leaves exited children `<defunct>` and blocks in `pselect`. Recorded in the external migration notebook (`~/AI/docs/femacs/elisp-dogfooding-and-script-migration.md`) as P0 fix #1 for `v0.0.2`.
- **Lexical `--eval` scope**: `elisp --eval` defaults to `lexical-binding = t`, so `--eval` forms dynamically binding external special variables (`byte-compile-dest-file-function`) must `(require 'bytecomp)` first.

## Verification Evidence & Cleanup

- `ruby -c` (`elisp.rb`, `femacs.rb`), `sh -n docs/install.sh`, `liuchong/homebrew-tap` HTTP 200 (`run_id=8e8fc4d30fa345c7907aed2ec8c286a5`), and local `brew install liuchong/tap/elisp` + `brew test liuchong/tap/elisp` (`run_id=4de8ce51ca5940ae9dc3dc4a4c2590f7`).
- `cargo test -p xtask --bin xtask` (`125 passed; 0 failed`), `xtask release check --version 0.0.1 --allow-dirty --skip-package` (`OK`), and `xtask release notes` (`run_id=f334a9168b17435ba52567bf6b641af4`), followed by `gh release edit v0.0.1`.
- Published `elisp 0.0.1` checks: `elisp --batch -Q --script tools/check-crate-boundaries.el -- --self-test` (`ok (3/3)`), full crate boundary scan (`ok (checked standalone=8, editor isolation verified)`, `run_id=9e2403e82b504b6497170a0bb2a5869b`), and strict zero-warning byte-compile of all `tools/testing/**/*.el` (`BYTE_COMPILE_ALL_OK`).
- Headless Chrome Pages check: `PAGES_CHROME_VERIFY_OK` across all 5 routes × 3 viewports (`1440`, `390`, `320`, `Eliscript 0.1.4 · live`, `0` errors, `0` overflow, `run_id=f78c767a948043898e555653bfee18d4`).
- Cleaned transient `target/`, `build/*`, and `tools/__pycache__` (empty `build/`, absent `target/` and `tools/__pycache__`), keeping `dist/lisp` and `dist/macos/Femacs.app`.
