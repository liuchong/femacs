# CLN311 — Standalone `lisp-core` v0.0.1 Extraction & Cleanup (2026-11-27)

## Summary

Extracted and consolidated the standalone general-purpose Lisp language runtime (`crates/lisp-vm` `0.0.1` + `crates/lisp-core` `0.0.1`) cleanly inside the workspace, preserving full editor/host/tool compatibility (`femacs-host`, `femacs-tools`, `xtask`, `http-host`, `femacs`), verifying all A1–A6 acceptance criteria, retiring `3` temporary build/acceptance/packaging scopes (`12,761` files, `1,950` dirs, `4` symlinks, `4,989,401,412` logical bytes), and passing all post-cleanup static gates.

## Delivered & Verified Evidence

- **A1 Standalone Binary & Dynamic Link Closure**: `cargo build --release -p lisp-core --target-dir build/cargo` (`run_id=1484d33a545d48f3a62c7c1295367591`, `run_id=d8582c540a334cc99d47df8712e8d9df`) succeeded with zero external build tools or Homebrew link flags; `otool -L build/cargo/release/lisp-core` links exclusively `/usr/lib/libSystem.B.dylib`.
- **A2 Crate Packaging & Isolated Verification**: `cargo package -p lisp-vm -p lisp-core --allow-dirty --no-verify --target-dir build/pkg-check` produced `lisp-vm-0.0.1.crate` (`3.82 MiB`, `397` entries, `0` escapes) and `lisp-core-0.0.1.crate` (`1.04 MiB`, `103` entries, `0` escapes); unpacked into `build/pkg-verify/`, rebuilt release cleanly in `22.72s`, and evaluated `(+ 20 22)` to `42` both cold and warm from `/tmp` with scrubbed `EMACS*` environment variables (`run_id=d8582c540a334cc99d47df8712e8d9df`).
- **A3 Standalone CLI, Cache Fallback & JSON-RPC Matrix**: verified `--version` (`lisp-core 0.0.1`), `--help`, exit `2` on unknown flag, `cache path|clean|warm`, `eval` (`42` and `cl-lib`/`seq`/`map`/`pcase`/`rx`/`json`), `run` + direct shebang script (`lexical-binding` and `command-line-args-left`), multi-line `repl` with error recovery, `compile --strict` pass/fail, ERT `test` pass/fail, warm `.pdmp` startup + corrupt `.pdmp` cold fallback (bad magic and valid magic with corrupt body), `exec supervise|mem-tree|digest`, and `--rpc http` round-trip over an inherited Unix socket pair (`run_id=64af9b93f6bd45ef8d35f37eb0964642`, `run_id=21484b4b12ac4187afb28ddf37678f61`).
- **A4 `fe-test` Independence (Zero `xtask`)**: with neither release nor debug `xtask` binary present, `lisp-core --batch -Q -L tools/testing -l fe-test.el -- --suite self` passed `9/9` (`7.659s`), and `--suite all --no-cache` passed `38/38` (`wall=13.397s`, `caseSum=23.329s`, `concurrency=1.74x`, `peakMem=191.14 MiB`, `run_id=1484d33a545d48f3a62c7c1295367591`).
- **A5 Workspace, Host Features & Isolated Install**: `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`, `cargo check -p femacs-tools`, and `cargo install --path crates/lisp-core --root build/acceptance/install-root` passed with installed smoke `eval '(* 6 7)'` returning `42` from `/tmp` (`run_id=d8582c540a334cc99d47df8712e8d9df`).
- **A6 Protective Unit & Integration Tests**: `cargo test -p xtask --bin xtask` (`119/119`), `cargo test -p lisp-vm --lib` (`12/12`), `cargo test -p lisp-core` (`23/23` unit + `7/7` real-process `engine_integration` cases covering `Engine::minimal`, `Engine::new`, `Value` round-trips, `Error` variants, corrupt `.pdmp` fallbacks, `serve_http_rpc`, and CLI subcommands), `cargo test -p femacs` (`5/5` launcher), and `cargo test -p http-host` (`9/9`) all passed (`run_id=b0448bc458e24622b13fa88c604557aa`, `70.145s`).

## Controlled Post-Cleanup Verification (`run_id=270e31929eeb4d419dc769bf25407e30`, `18.672s`, `exit=0`, `cleanup_complete=true`)

- `build/` directory is empty and `tools/__pycache__` is absent.
- Retained `dist/macos/Femacs.app` and `dist/lisp` remain intact, and `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty.
- All 5 static gates passed: `tools/check-own-voice.py` (`0` failures across `334` ported + `374` own files), `tools/test-repository-bootstrap.py` (`4/4`), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/check-unsafe-boundary.py` (`active_product`: `455` files, `160` `forbid_unsafe_files`, `6,351/6,351` `SAFETY:` blocks, `0` `static_mut`, `0` errors), and `git diff --check` is clean.

## Retired Temporary Scopes

Retired `3` owned temporary build, acceptance, packaging, and test-framework scopes totaling `12,761` files, `1,950` dirs, `4` symlinks, and `4,989,401,412` logical bytes:

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `11,321` | `1,441` | `0` | `4,669,223,556` | `1c1423e8a8e6bd44adb3efb801d4760c350baeb454fa3eba5b88d8ae49d64e2f` | Rebuild on demand via `cargo build --release -p lisp-core --target-dir build/cargo` |
| `build/{acceptance,pkg-check,pkg-verify}` | `1,440` | `508` | `4` | `320,177,856` | `1acc18f5dc35f26ca1fe3ceb5dcce352ec8ad89e7b97fa711b4fb301adb5b2c0` | Recreated on demand by `lisp-core`, `cargo package`, or `cargo install` |
| `build/test-framework` | `0` | `1` | `0` | `0` | `a4d0eb6872c6f360efc63858420e0410bafc5a4db72513f97044efe607476a7a` | Ephemeral `fe-test` scratch directory |
