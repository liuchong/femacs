# CLN317 — Standalone `lisp-core` Phase 6: Release Polish, `cargo package`, Docs & Pre-Publish Checklist (2026-11-27)

## Summary

Completed Phase 6 (final phase) of the standalone general-purpose Lisp runtime roadmap across `crates/lisp-vm`, `crates/lisp-std`, `crates/lisp-core`, and `docs/design/lisp-core.md`. Eliminated non-default feature manifest/test warnings, bundled `LICENSE` across all three publishable crates, verified 0-warning compilation across all feature combinations and rustdoc, completed user/embedder `README.md`s and the §6 pre-publish rename checklist (explicitly noting Windows is cross-compile verified ONLY without physical Windows host runtime verification), passed multi-crate `cargo package` + isolated extraction `release` rebuild verification, re-verified macOS tests and `fe-test`, and retired temporary build/packaging/cache artifacts (`CLN317`).

## Changes

- **Zero-Warning Feature Matrix & License Packaging (`crates/lisp-core`, `crates/lisp-std`)**:
  - `crates/lisp-std/LICENSE`: Added verbatim `GPL-3.0-or-later` license file matching `crates/lisp-core/LICENSE` and `crates/lisp-vm/LICENSE` so every publishable `.crate` tarball ships `LICENSE` + `README.md`.
  - `crates/lisp-core/Cargo.toml`: Marked `flate2` in `[dependencies]` as `optional = true` and enabled `"dep:flate2"` via the `vm` feature (`stdlib_cache::GzDecoder` is gated on `feature = "vm"` while `build.rs` already carries `[build-dependencies] flate2`), eliminating the `cargo::unused_dependencies` warning on minimal editor builds (`--no-default-features --all-targets`).
  - `crates/lisp-core/src/repl.rs`: Updated `is_symbol_char` and `symbol_prefix_at` to `#[cfg(any(feature = "repl-tty", test))]` so `--no-default-features --features vm --all-targets` compiles and runs `repl::tests::symbol_prefix_extracts_lisp_identifiers` cleanly with `0` warnings.
- **Standalone User, Embedder & Pre-Publish Documentation (`crates/lisp-{core,vm,std}/README.md`, `docs/design/lisp-core.md`)**:
  - `crates/lisp-core/README.md`: Documented CLI installation/invocation (`eval`, `run`, direct/shebang script, `repl`, `compile`, `test`, `init`, `pkg`, `cache`, `exec`, `--rpc http` with `--rpc-fd`/`--rpc-addr`), `--concurrency=no-gil|gil` + `LISP_CONCURRENCY`, all `22` pure-Rust `stdlib/std/*.el` modules, `Engine` / `EngineConfig` / `Value` Rust embedding API, consumer feature flags (`default-features = false` editor isolation), and explicit platform matrix marking Windows as cross-compile architecture verified ONLY (no physical Windows host runtime execution).
  - `crates/lisp-vm/README.md` & `crates/lisp-std/README.md`: Documented `isolate_vm` opt-in multi-isolate mode vs single-VM editor profile, `std/*` pure-Rust modules, `GPL-3.0-or-later` license links, and platform verification boundaries.
  - `docs/design/lisp-core.md`: Marked **Phase 6 (Complete)** in §5 and added **§6 Pre-Publish Crate Rename & Release Checklist** covering topological crates.io publish order (`lisp-vm` -> `lisp-std` -> `lisp-core`), file-by-file rename touchpoints (`brand.rs`, crate manifests, editor consumers `femacs-host`/`femacs-tools`/`xtask`/root `Cargo.toml`, static gates, READMEs), pre-publish verification commands, and platform verification disclosures.

## Verification Evidence

- **0-Warning Matrix & Rustdoc (`run_id=5210ab5326cf488b8310b3ea78983c5e`, `exit=0`)**:
  - Passed with `0` errors and `0` warnings across `crates/lisp-core/` and `crates/lisp-std/` on: `lisp-std --all-targets`, `lisp-core --all-targets`, `lisp-core --no-default-features --all-targets`, `lisp-core --no-default-features --features vm --all-targets`, `lisp-core --no-default-features --features vm,repl-tty --all-targets`, `lisp-core --no-default-features --features vm,std-batteries --all-targets`, `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`, and `cargo doc --no-deps -p lisp-std -p lisp-core`.
- **Packaging & Isolated Tarball Rebuild Verification (`cargo publish` and `~/.cargo/bin` install NOT run)**:
  - Full crates.io verification: `cargo package -p lisp-vm --allow-dirty` passed; `cargo package -p lisp-std --allow-dirty` passed (`0` warnings).
  - Final multi-crate tarballs (`run_id=b3bc0990cecf44f4922c4f7826243cf2`, `exit=0`):
    - `lisp-vm-0.0.1.crate`: `4,030,001` B (`3.84 MiB`), `398` entries, `LICENSE` + `README.md`, `0` escapes.
    - `lisp-std-0.0.1.crate`: `57,607` B (`0.05 MiB`), `16` entries, `LICENSE` + `README.md`, `0` escapes.
    - `lisp-core-0.0.1.crate`: `1,154,800` B (`1.10 MiB`), `131` entries, `LICENSE` + `README.md`, `0` escapes.
  - Isolated extraction release rebuild (`build/pkg-verify/`, `run_id=669b93f2923446868a805d0fc72e0c24` build + `run_id=d6a687899c7148b89aa739ebfd3f2b15` smoke, `exit=0`): compiled standalone `release` binary from extracted `.crate` tarballs with `[patch.crates-io]`, verified `--version` (`lisp-core 0.0.1`), `--no-auto-warm eval '(+ 20 22)'` (`42`), `--no-auto-warm eval '(progn (require (quote std/json)) (std-json-stringify (list (cons "ok" t) (cons "n" 42))))'` (`"[[\"ok\",true],[\"n\",42]]"`), and `otool -L` (`/usr/lib/libiconv.2.dylib` & `/usr/lib/libSystem.B.dylib` only).
- **macOS Host Suite Re-Verification After Step 5 Code/Cargo Edits (`run_id=9f67a17c86184075b749a3258cd2db37` & `b3bc0990cecf44f4922c4f7826243cf2`, `exit=0`)**:
  - `cargo test -p lisp-vm --lib` (`12/12`), `cargo test -p lisp-vm --features isolate_vm --lib` (`17/17`), `cargo test -p lisp-std` (`8/8`), `cargo test -p lisp-core` (`27/27` unit + `engine_integration` all passed + `std_batteries_integration` `5/5`), `cargo test -p lisp-core --no-default-features --features vm --lib` (`27/27`), `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`, `cargo build --release -p lisp-core` (`otool -L`: OS libs only).
  - `fe-test`: `--suite self` (`9/9`, `10.61s`) and `--suite all --no-cache` (`39/39`, `wall=34.541s`, `caseSum=59.331s`, `concurrency=1.72x`, `peakMem=180.53 MiB`).

## Post-Cleanup Verification

- Removed temporary `target/`, `build/*`, and `tools/__pycache__`; confirmed `build/` is empty, `target/` and `tools/__pycache__` are absent, `dist/lisp` is intact, and `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty (`run_id=4d541eb568e44b3cb4ab9619535ddc5f`, `exit=0`).
- All `6` static gates + `git diff --check` passed post-cleanup: `tools/check-own-voice.py` (`0` failures), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-unsafe-boundary.py` (`471` active files, `175` `forbid_unsafe_files`, `6,446/6,446` `SAFETY:` blocks, `0` `static_mut_items`, `0` errors), `tools/check-crate-boundaries.py` (`ok`, `standalone=4`, editor isolation verified), and `tools/test-repository-bootstrap.py` (`4/4`).
