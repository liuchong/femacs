# CLN318 — Four-Crate Consolidation (`lisp-vm`, `lisp-std`, `lisp`, `elisp`) & Clean Feature Design (2026-11-27)

## Summary

Executed the final pre-publish consolidation across four publishable crates (`crates/lisp-vm`, `crates/lisp-std`, `crates/lisp`, and `crates/elisp`), eliminating `crates/lisp-core` overlap by turning the renamed `crates/lisp` into a pure Rust embedder library (`[lib]`) with orthogonal feature flags and splitting the standalone CLI/REPL application into pure binary crate `crates/elisp` (`[[bin]] name = "elisp"`). Verified zero-warning compilation across all 14 workspace/cross/doc checks, unit and integration tests across every feature combination, `fe-test` (`--suite self` `9/9`, `--suite all --no-cache` `39/39`), 4-crate `cargo package` tarballs plus isolated extracted `release` rebuild in `build/pkg-verify/`, complete build/cache cleanup, and all 6 post-cleanup static gates. Neither `cargo publish` nor `~/.cargo/bin` installation was executed.

## Crate Architecture & Orthogonal Feature Matrix

- **`crates/lisp-vm` (`[lib]`, v0.0.1)**: Low-level Elisp-compatible VM and bytecode runtime; `default = []` (preserving single-VM editor layout), `isolate_vm = []` (opt-in thread-local `IsolateContext` multi-core isolation).
- **`crates/lisp-std` (`[lib]`, v0.0.1)**: 100% pure-Rust (`#![forbid(unsafe_code)]`, zero `cc` / C `-sys` crates) batteries-included runtime modules (`std/{http,kv,db,json,toml,csv,yaml,url,base64,archive,crypto,time,fs,path,regex,proc,env,cli,log}`). `default = []`.
- **`crates/lisp` (`[lib]`, v0.0.1, renamed from `crates/lisp-core`)**: Pure embedder library (`Engine`, `EngineConfig`, `Value`, `HostFnRegistry`, `NoGil` pool, `.pdmp` warm cache, `pkg`, `exec`, `rpc`, `stdlib_cache`); removes `[[bin]]`, `main.rs`, `cli.rs`, `repl.rs`, and `rustyline`:
  - `default = ["vm", "std"]` (batteries-included one-stop Rust embedder experience)
  - `vm = ["dep:lisp-vm", "lisp-vm/isolate_vm", "dep:flate2"]`
  - `std = ["vm", "dep:lisp-std"]`
  - `pkg = ["vm", "dep:toml"]`
  - `default-features = false` (`0`-VM / minimal editor host reuse for `exec`, `rpc`, `stdlib_cache`)
  - Public re-exports `lisp::prelude::*`, `lisp::vm`, `#[cfg(feature = "std")] lisp::stdlib`, unified env vars `LISP_CACHE_DIR`, `LISP_VERBOSE`, `LISP_NO_AUTO_WARM`, `LISP_CONCURRENCY`, and cache subdir `lisp`.
- **`crates/elisp` (`[[bin]] name = "elisp"`, v0.0.1)**: Standalone general-purpose Lisp CLI & REPL binary (`main.rs`, `cli.rs`, `repl.rs`, `tests/cli_integration.rs`, independent `README.md`, `LICENSE`):
  - Depends on `lisp` with `default-features = false, features = ["vm", "pkg"]` plus optional `rustyline`
  - `default = ["repl-tty", "std"]`
  - `repl-tty = ["dep:rustyline"]` (`elisp> ` prompt, completion, history, `Ctrl-C` recovery)
  - `std = ["lisp/std"]`
- **Editor Isolation & Tooling**: Root `Cargo.toml`, `crates/femacs-host/Cargo.toml` (dropped unused `lisp-core` dep), `xtask/{Cargo.toml,src/main.rs,src/cargo_build.rs}` (switched to `lisp` `default-features = false`), `tools/testing/{fe-test.el,fe-test-dsl.el,suites/suite-static.el}`, `tools/check-crate-boundaries.py` (`standalone=8`), `tools/unsafe-boundary-manifest.json`, `maintenance/tooling-inventory.json`, all 4 crate `README.md`s, and `docs/design/lisp-core.md`.

## Verification Evidence

- **14-Target 0-Warning Matrix & Rustdoc (`run_id=2909e11fc75f4b0b847fd2895e8e47f6`, `exit=0`, `8.175s`)**:
  - `0` errors and `0` warnings across `crates/lisp-std/`, `crates/lisp/`, and `crates/elisp/` for: `lisp-std --all-targets`, `lisp` (`--all-targets`, `--all-features --all-targets`, `--no-default-features --all-targets`, `--no-default-features --features vm --all-targets`, `--no-default-features --features vm,pkg --all-targets`), `elisp` (`--all-targets`, `--no-default-features --all-targets`, `--no-default-features --features std --all-targets`), `--workspace`, `femacs-host --features tty_terminfo`, `x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`, and `cargo doc --no-deps -p lisp-std -p lisp`.
- **Unit, Integration & Release Binary Suites (`run_id=0116da1196d24de38eb8f64324ada9d5` & `ee365805201b44abbfe9bc2d63f84ddb`, `exit=0`)**:
  - `lisp-vm --lib`: `12/12`; `lisp-vm --features isolate_vm --lib`: `17/17`; `lisp-std`: `8/8`.
  - `lisp --all-features`: `23/23` unit + `engine_integration` + `std_batteries_integration` (`4/4` library suites); `lisp --no-default-features --features vm --lib`: `22/22`; `lisp --no-default-features --lib`: `20/20`.
  - `elisp` (default): `4/4` unit + `cli_integration` (subcommands, shebang `#!elisp run`, pipe REPL, `--rpc-addr` TCP loopback, `init`/`pkg` workflow); `elisp --no-default-features`: `4/4` unit + `cli_integration`.
  - `xtask --bin xtask`: `119/119`; `cargo build --release -p elisp` (`elisp 0.0.1`, `otool -L` links ONLY `/usr/lib/libiconv.2.dylib` & `/usr/lib/libSystem.B.dylib`).
- **`fe-test` Verification**:
  - `--suite self` (`run_id=3087b8fbca3b4eb088616a1bd512d7a3`): `9/9` passed (`10.96s`).
  - `tools/test-tooling-modernization.py` (`33/33` in `2.133s`) + `fe-test --exec-bin target/release/elisp --suite all --no-cache` (`run_id=f4b1ce4c22044f0886783fc769918942`, `exit=0`, `18.23s`): `39/39` passed (`0` failed, `0` cached, `0` unstarted, `0` cancelled; `wall=15.697s`, `caseSum=35.124s`, `concurrency=2.24x`, `peakMem=208.09 MiB`).
- **4-Crate Packaging & Isolated Extraction Rebuild (`run_id=279c1523521742459b9c7b02349282a8` & `38a8048dfa8d4154ac5e00270008b704`, `cargo publish` and `~/.cargo/bin` install NOT run)**:
  - Full crates.io verify passed for leaf crates `lisp-vm` and `lisp-std`; 4-crate `cargo package -p lisp-vm -p lisp-std -p lisp -p elisp --allow-dirty --no-verify` produced:
    - `lisp-vm-0.0.1.crate`: `4,030,009` B (`3.84 MiB`), `398` entries, `LICENSE` + `README.md`, `0` escapes.
    - `lisp-std-0.0.1.crate`: `57,687` B (`0.06 MiB`), `16` entries, `LICENSE` + `README.md`, `0` escapes.
    - `lisp-0.0.1.crate`: `1,137,877` B (`1.09 MiB`), `128` entries, `LICENSE` + `README.md`, `0` escapes.
    - `elisp-0.0.1.crate`: `48,317` B (`0.05 MiB`), `10` entries, `LICENSE` + `README.md`, `0` escapes.
  - Isolated extracted workspace (`build/pkg-verify/` with `[patch.crates-io]`) passed `cargo check -p lisp --no-default-features --features vm`, `cargo check -p elisp --no-default-features`, `cargo build --release -p elisp`, `elisp --version` (`elisp 0.0.1`), `--no-auto-warm eval '(+ 20 22)'` (`42`), `std/json` `std-json-stringify` (`"[[\"ok\",true]]"`), and `otool -L` (`/usr/lib/libiconv.2.dylib` & `/usr/lib/libSystem.B.dylib` only).

## Platform Coverage Note

- **macOS (`aarch64`)**: Full runtime, CLI/REPL, `fe-test`, packaging, and release verification completed locally.
- **Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`)**: **Cross-compile architecture verified ONLY** (no physical Windows host runtime verification was performed).

## Post-Cleanup Verification

- Removed transient `target/` (`6.7G`), `build/pkg-verify/` (`880M`), `build/cln318_matrix.log` (`4.9M`), `build/acceptance` (`88K`), `build/test-framework` (`24K`), remaining empty `build/*` directories, and `tools/__pycache__`; confirmed `build/` is empty, `target/` and `tools/__pycache__` are absent, `dist/lisp/bin/femacs-lisp` is intact, and `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty.
- All `6` static gates + `git diff --check` passed post-cleanup (`run_id=b73d9d2be3bd456589bb7ebb55ba4a11`, `exit=0`, `21.84s`): `check-own-voice.py` (`0` failures), `check-ledger-ticks.py` (`58` bound / `0` failures), `test-tooling-modernization.py` (`33/33`), `check-unsafe-boundary.py` (`472` active files, `176` `forbid_unsafe_files`, `6,446/6,446` `SAFETY:`, `0` `static_mut_items`, `0` errors), `check-crate-boundaries.py` (`ok (checked standalone=8, editor isolation verified)`), and `test-repository-bootstrap.py` (`4/4`).
