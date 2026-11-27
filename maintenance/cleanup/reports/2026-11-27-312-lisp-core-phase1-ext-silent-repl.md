# CLN312 — Standalone `lisp-core` Phase 1: Host Extensions, Silent Auto-Warm Cache, TTY REPL & Isolation Gates (2026-11-27)

## Summary

Completed Phase 1 of the standalone general-purpose Lisp runtime roadmap across `crates/lisp-vm` and `crates/lisp-core`: added `Value::Bytes` and safe host extension callbacks (`HostFnRegistry` / `Engine::register_fn`), centralized branding (`crates/lisp-core/src/brand.rs`), suppressed cold-bootstrap stderr diagnostics (`lisp-core--quiet-boot`), implemented lock-guarded atomic background `.pdmp` warm caching, built a pure-Rust interactive TTY REPL via `rustyline` (`repl-tty` feature, default on standalone, isolated from editor targets), added `tools/check-crate-boundaries.py` (`static/crate-boundaries` in `fe-test`), verified all acceptance gates and workspace tests cleanly, and retired `3` temporary build/packaging/test scopes (`652` files, `328` dirs, `0` symlinks, `271,163,633` logical bytes).

## Delivered & Verified Evidence

- **Host Extension Bridge & `Value::Bytes`**: `crates/lisp-vm/src/host_runtime.rs` adds `VmValue::Bytes(Vec<u8>)` (unibyte string round-trip via `make_unibyte_string_bytes` & `!STRING_MULTIBYTE`), `HostExtFn` / `HOST_EXT_REGISTRY`, `ensure_host_call_subr_registered()` (`internal-host-call` bound cleanly on both cold boots and `.pdmp` restores), `HeadlessVm::register_host_fn`, and `HeadlessVm::list_interned_symbols`; `crates/lisp-core/src/{value,engine}.rs` expose `Value::Bytes`, `Engine::register_fn`, `Engine::complete_symbols`, `Engine::booted_from_dump`, and `EngineConfig::with_verbose`.
- **Release Build, Isolation & Pure-Rust Closure**: `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`, `cargo check -p lisp-core --no-default-features --features vm` (`0` warnings in `lisp-core`), and `cargo build --release -p lisp-core --target-dir build/cargo` (`run_id=e4a21148eff841e484cc9dec190575bd`) succeeded; `tools/check-crate-boundaries.py` passed (`standalone=3` zero `cc`/`cmake`/`pkg-config`/`bindgen`/`*-sys`; editor targets isolated from `lisp-std`/`rustyline`/HTTP/DB stacks); `otool -L` on release `lisp-core` links exclusively `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`.
- **Silent Cold Boot, Auto-Warm `.pdmp` & PTY TTY REPL**: verified from `/tmp` with scrubbed `EMACS*` env vars:
  1. Silent cold boot `--no-auto-warm eval "(+ 20 22)"` -> `rc=0`, `stdout="42\n"`, `stderr=""` (`0` bytes).
  2. `--verbose eval "(+ 20 22)"` -> `rc=0`, `stdout="42\n"`, `"Loading "` on `stderr`.
  3. Detached auto-warm produced valid `<version_root>/lisp-core.pdmp` (`DUMPEDGNUEMACS\x00\x00`, zero `.tmp.*`/`.warm.lock` leftovers), subsequent warm `eval "(+ 20 22)"` returned `42` in `18.7 ms`.
  4. Real `pty.openpty()` TTY REPL session handled multi-line `(defun sq (x)\n  (* x x))` -> `sq`, `(sq 9)` -> `81`, `Ctrl-C` recovery + `(+ 20 22)` -> `42`, `Ctrl-D` -> exit `0`, and persisted `<version_root>/history.txt` cleanly.
- **Test Suites & Crate Packaging**: `cargo test` passed across `lisp-vm --lib` (`12/12`), `lisp-core` (`26/26` unit + `8/8` `engine_integration` incl. `engine_register_fn_and_bytes_roundtrip` & silent/verbose CLI checks), `xtask --bin xtask` (`119/119`), `http-host` (`9/9`), and `femacs --lib` (`0/0`); `fe-test` passed `--suite self` (`9/9`, `9.680s`) and `--suite all --no-cache` (`39/39`, `wall=12.170s`, `peakMem=210.72 MiB`); `cargo package -p lisp-vm -p lisp-core --allow-dirty --no-verify` produced `lisp-vm-0.0.1.crate` (`3.8MiB compressed`, `397` files, `0` escapes) and `lisp-core-0.0.1.crate` (`1.1MiB compressed`, `106` files, `0` escapes).

## Controlled Post-Cleanup Verification (`run_id=3cdec2fa5c9e402fb49b9c5fd3702aa3`, `15.594s`, `exit=0`, `cleanup_complete=true`)

- `build/` directory is empty and `tools/__pycache__` is absent.
- Retained `dist/macos/Femacs.app` and `dist/lisp` remain intact, and `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty.
- All `6` static gates passed: `tools/check-own-voice.py` (`0` failures across `334` ported + `375` own files), `tools/test-repository-bootstrap.py` (`4/4`), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/check-unsafe-boundary.py` (`active_product`: `457` files, `162` `forbid_unsafe_files`, `6,357/6,357` `SAFETY:` blocks, `0` `static_mut`, `0` errors), `tools/check-crate-boundaries.py` (`ok`), and `git diff --check` is clean.

## Retired Temporary Scopes

Retired `3` owned temporary build, packaging, and test-framework scopes totaling `652` files, `328` dirs (`321` nested + `7` top-level `build/*` directories), `0` symlinks, and `271,163,633` logical bytes:

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `640` | `312` | `0` | `255,807,546` | `94d8221fd8e1deef783d3d155321db9a4ce21874065ab12fb226cc4b08150694` | Rebuild via `cargo build --release -p lisp-core --target-dir build/cargo` |
| `build/pkg-check` | `9` | `7` | `0` | `15,338,076` | `da60de5a7d85ff74db3ca5cef135f0a387dbd0ba59692785fd8b84113f506294` | Recreated via `cargo package -p lisp-vm -p lisp-core --allow-dirty --no-verify` |
| `build/test-framework` + `build/*` scratch dirs | `3` | `9` | `0` | `18,011` | `2d814dc733c36e93ccc4f5137a45e51e1700455d8a3383487f6e05ff95dc9c5f` | Recreated on demand by `fe-test` |
