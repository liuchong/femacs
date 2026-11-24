# CLN290 — Stage C2 & A2 Standalone Lisp CLI/HTTP Package & Cache Retirement (2026-11-24)

## Summary

Implemented Phase II Stage `C2` (`femacs-lisp` standalone CLI/REPL/`--rpc http` binary, `#![forbid(unsafe_code)]` `crates/http-host/` Axum/Tokio edge host with inherited `UnixStream` socketpair + `FD_CLOEXEC` adoption, `xtask build-lisp` relocatable packaging pipeline, and shared-core `examples/lisp-web/` static/HTTP app) and Stage `A2` (`tools/test-c2-a2-lisp-app.py`), fixed headless startup (`custom-delayed-init-variables`, `lisp-directory`, `ColdMinimal` guard, and `EMACSLOADPATH` canonical precedence) alongside `test-u4-unit.py` Group 4 proc-macro `.dylib`/`.so` discovery, completed full clean-tree rebuild (`run_id=0d977ae1517f48b09c3556a698f9ef52`), passed all Phase 2 integration/hygiene gates (`run_id=dcdd6dc3e9984f3bb463be747809aed6`, `run_id=1fcaefbae8fc48978a0e20417e57587d`) and Phase 3 protective unit tests (`run_id=ca0f9b79bb3a434e963a44f64d6fb5f4`, `run_id=0313057f76704aafa756cd349bf7575d`), atomically reinstalled `/Applications/Fe.app`, verified post-cleanup smoke (`run_id=dafaf76fbeea4132a2b2c2a9e70bcda1`), and retired `24` temporary build, cache, staging, test-binary, and hardlink-alias scopes.

## C2 & A2 Implementation & Verification Receipts

- **Standalone Headless Runtime & CLI (`src/emacs.rs`, `crates/femacs-vm/src/host_runtime.rs`, `crates/femacs-host/Cargo.toml`, `crates/femacs-host/src/{lib,lisp_cli,bin/femacs_lisp}.rs`)**:
  - Extended `init_headless_vm_body` and `HostInitConfig` (`data_dir`, `replace_load_path`, `defer_delayed_custom_init`, `eval_silent`, `prepend_load_path`, `load_library`, `funcall_with_args`, `funcall_two_strings_to_string`, `dump_portable`), plus headless `(kill-emacs [ARG])` hook execution and unswallowable `VmHostError::ExitRequest(i32)` propagation across `(ignore-errors ...)`.
  - Matched standard startup runtime initialization in `init_headless_host` for non-`ColdMinimal` / non-`--dump-out` sessions (`set-locale-environment`, path coding-system decoding, `lisp-directory` / `load-history`, and `custom-delayed-init-variables` initialization for `temporary-file-directory`), while keeping `--dump-out` dumps relocatable.
  - Added `#![forbid(unsafe_code)]` `femacs_host::lisp_cli` and `femacs-lisp` binary supporting `--eval`, `--batch`, `-Q`, `--no-site-file`, `--no-site-lisp`, `-L`/`-l`/`-f`/`--script`, `--lisp-dir`/`--data-dir`, `--dump-file`/`--no-dump`/`--dump-out`, `--repl` (preserving multiline reader boundary and recovering on errors), and `--rpc http` over `host_runtime::adopt_inherited_rpc_socket` (`F_DUPFD_CLOEXEC`, `/dev/null` stdio isolation, bounded JSON-RPC 2.0 `http.handle`, immediate C `stdout`/`stderr` flushing).
- **Standalone HTTP Edge Host, Packaging Pipeline & Web Example (`Cargo.toml`, `Cargo.lock`, `crates/http-host/`, `xtask/src/{main,cargo_build,runtime,standalone}.rs`, `examples/lisp-web/`)**:
  - Added independent workspace member `crates/http-host` (`#![forbid(unsafe_code)]`, Axum 0.8 + Tokio 1 + Serde/Serde JSON, zero `femacs-*` dependency) spawning sibling `femacs-lisp --rpc http`, forwarding child stdout/stderr onto `http-host` stderr, enforcing `--max-body-bytes` (`413`), bounded queue (`503`), per-request timeout (`504`), keeping healthy Lisp backend alive across `500` errors, emitting `LISTENING_ADDR=`/`BACKEND_PID=` readiness lines, and cleanly releasing backend + listening port on `SIGINT`/`SIGTERM`.
  - Implemented `xtask build-lisp [--out DIR]` assembling a symlink-free relocatable package (`dist/lisp`) via `bundle::publish` with ad-hoc strict `codesign` and SHA-256 `manifest.json`, backed by `examples/lisp-web/{items-core.el,build-site.el,http-app.el}`.
  - Registered `crates/femacs-host/src/{lisp_cli,bin/femacs_lisp}.rs` and `crates/http-host/src/{lib,main}.rs` in `tools/unsafe-boundary-manifest.json` (`149` `forbid_unsafe_files`, `0` errors), updated `tools/test-u4-unit.py:689` to include `*.dylib`/`*.so` proc-macro libs (`serde_derive`), and registered `tools/test-c2-a2-lisp-app.py` in `maintenance/tooling-inventory.json`.
- **Builds, Clean-Export Acceptance, Unit Tests & Hygiene Gates (`assistant run`)**:
  - Clean rebuild (`run_id=0d977ae1517f48b09c3556a698f9ef52`, `684.856s`, `exit=0`, `cleanup_complete=true`, peak accounted `2,208,090,464` B): built `dist/lisp`, `build-terminal`, `a1_host_acceptance`, and `dist/macos/Femacs.app`.
  - Phase 2 runtime acceptance (`run_id=dcdd6dc3e9984f3bb463be747809aed6` & `run_id=1fcaefbae8fc48978a0e20417e57587d`, `exit=0`, `cleanup_complete=true`): `test-c1-a1-host.py` (`ok=true`, `22.97s`, all 3 suites & 13 scenarios + dual-deliverable fidelity `(t 42 "alpha gamma\n")`), `test-c2-a2-lisp-app.py --selftest` (`PASS`) and full `test-c2-a2-lisp-app.py` (`PASS`: clean `--export`, `build-lisp`, hidden export dir, `PATH=/usr/bin:/bin` isolation, `otool -L`/codesign, corrupt/mutated-fingerprint/cross-product dump rejection, `--no-dump`/`.pdmp` `"ok"` + GC, `(kill-emacs 7)` & `kill-emacs-hook` `(ignore-errors (kill-emacs 9))`, `--script`, `--repl` error recovery, offline static HTML/JSON parity vs live HTTP `/`, `/api/items`, `POST /api/echo`, `404`, `413`, `500` multiline log preservation with identical live `BACKEND_PID`, `FD_CLOEXEC` child `/bin/sh` `/dev/fd` clean check, and `SIGTERM` zero-exit port rebind), `test-u3-unit.py` (`3/3`), `test-u4-unit.py` (`4/4`, `19.3424s`), `test-u7-safe-subsystems.py` (`4/4`, `22` modules, `20` ERT suites, `U7_GC_STRESS_OK gcs=72`), `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,327/6,327` `100.0%` `SAFETY:`, `0` errors), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), and `test-tooling-modernization.py` (`33/33`, `1.090s`).
  - Phase 3 protective unit tests (`run_id=ca0f9b79bb3a434e963a44f64d6fb5f4` `exit=0` + `run_id=0313057f76704aafa756cd349bf7575d` `24.42s`, `exit=0`): `http-host` (`4/4`), `xtask` (`118/118`), `femacs_host_unit_tests` (`9/9`), `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`), `check-unsafe-boundary.py` (`0` errors), `check-own-voice.py` (`0` failures).

## Retired Temporary & Verification Scopes

Retired `24` owned temporary build, cache, staging, test-binary, empty `xtask` test, and hardlink-alias scopes totaling `6,174` files, `1,809` directories, `16` symlinks, `1,611,442,831` logical bytes, normalized digest `9f5a8df1bb03593c3a7fba918d5bbce60562ca962ad9d678e9f01d5d9829b38b`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `514M` | `4,222` | `799` | `0` | `639,113,749` | Root Cargo target cache (rebuilt by `cargo test` / `xtask`) |
| `build/acceptance` | `268K` | `43` | `112` | `2` | `153,610` | Recreated by acceptance runners |
| `build/android-tests` | `0` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/bundle-tests` | `0` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/cargo` | `16M` | `217` | `16` | `0` | `18,100,581` | `CARGO_TARGET_DIR` workspace cache (rebuilt on demand) |
| `build/cargo-tools` | `33M` | `130` | `54` | `0` | `34,059,384` | `xtask` host tool target dir (rebuilt on demand) |
| `build/lisp` | `321M` | `858` | `468` | `6` | `333,262,142` | Recreated by `cargo run -p xtask -- build-lisp --out dist/lisp` |
| `build/macos` | `291M` | `377` | `188` | `6` | `328,099,209` | Recreated by `cargo run -p xtask -- build-macos` |
| `build/manuals-review` | `0` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/package-contracts` | `0` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/probes-tests` | `0` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/tooling-tests` | `0` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/treesit-0.27.1` | `604K` | `5` | `7` | `2` | `607,238` | Reinstall via `python3 tools/toolchain-pins.py install` |
| `build/terminal/cargo` | `222M` | `269` | `144` | `0` | `223,208,037` | Recreated by `cargo run -p xtask -- build-terminal` |
| `build/terminal/config` | `48K` | `7` | `0` | `0` | `35,082` | Recreated by `xtask build-terminal` |
| `build/terminal/image` | `1.3M` | `3` | `2` | `0` | `1,270,503` | Recreated by `xtask build-terminal` |
| `build/terminal/image-staging` | `1.3M` | `2` | `0` | `0` | `1,269,940` | Recreated by `xtask build-terminal` |
| `build/terminal/lcms` | `364K` | `5` | `3` | `0` | `360,608` | Recreated by `xtask build-terminal` |
| `build/terminal/tls` | `7.6M` | `26` | `10` | `0` | `7,878,424` | Recreated by `xtask build-terminal` |
| `build/terminal/treesit` | `316K` | `6` | `6` | `0` | `307,980` | Recreated by `xtask build-terminal` |
| `build/terminal/port/src/a1_host_acceptance` | `4.6M` | `1` | `0` | `0` | `4,817,760` | Rebuilt by `xtask link-host-test a1_host_acceptance` |
| `build/terminal/port/src/femacs_host_unit_tests` | `1.1M` | `1` | `0` | `0` | `1,083,040` | Rebuilt by `xtask link-host-test lib` |
| `build/terminal/port/src/emacs-32.0.50.1` | `4.5M` | `1` | `0` | `0` | `4,714,176` | Hardlink alias to retained `build/terminal/port/src/emacs` |
| `build/terminal/port/src/emacs-32.0.50.1.pdmp` | `13M` | `1` | `0` | `0` | `13,101,368` | Hardlink alias to retained `build/terminal/port/src/emacs.pdmp` |

## Retained Working Set, Post-Cleanup Verification & Application Bundle Parity

- `dist/lisp` (`210M`, relocatable standalone Lisp CLI & HTTP edge package):
  - `bin/femacs-lisp`: `5,053,696` B, SHA-256 `c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,326,400` B, SHA-256 `cf7f316dd9a3ad30e19aba1391e607f5818a796c5bfe8e071ee8755c1e8edda2`
  - `bin/http-host`: `2,066,416` B, SHA-256 `be41cb78687323d4c4399943e2396a88512bf991d5248167444ea0501020776a` (`codesign --verify --deep --strict` `exit 0`)
- `build/terminal` (`45M`, holding `build/terminal/port/`):
  - `build/terminal/port/src/emacs`: `4,714,176` B, SHA-256 `3e4349c7fc6dffaac00b125d96c5f75ce8abe0bd231b53f2e4df4eb6539615a3`
  - `build/terminal/port/src/emacs.pdmp`: `13,101,368` B, SHA-256 `37ecaf1d2d28b2a4f902e5ca4e76eec09f1e1393cfe013984f8b87c7846349d6`
  - `build/terminal/port/src/temacs`: `4,714,176` B, SHA-256 `3e4349c7fc6dffaac00b125d96c5f75ce8abe0bd231b53f2e4df4eb6539615a3`
  - `build/terminal/port/lib-src/emacsclient`: `539,328` B, SHA-256 `1baf54e687c5fd61259f6699a2d2953b7e6567e350a304942b2e06f38ba600c7`
  - `build/terminal/port/lib-src/hexl`: `451,488` B, SHA-256 `15f41ba92128f2549c6e7f8f3e6685f379b9dec79d0f34b4696a4e5dda4878af`
- `dist/macos/Femacs.app` (`235M`) & atomically reinstalled `/Applications/Fe.app` (`235M`, `diff -rq` empty, `exit 0`):
  - `Contents/MacOS/Femacs`: `6,247,232` B, SHA-256 `74e6fa0c5a4864ae1169d402c5f05b24925ae89c1ca7f061d4a066c46fdc851e` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,600,760` B, SHA-256 `e0696518a6bf09e272edd64bd13d36b7a98d5f8cc4725f02228783c2b03b6ded` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/emacsclient`: `538,784` B, SHA-256 `ca959192dfd327a692c9bdc76b1c12ae7889652a0e65ca11193adc53bfafa755` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/hexl`: `451,120` B, SHA-256 `2ee11f3c9964f5aaa3465c07006a78f9d98487c56c81808faf42e3fc20ef7014` (exact match across workspace & `/Applications/Fe.app`)
  - `codesign --verify --deep --strict --verbose=2 dist/macos/Femacs.app` -> `exit 0`; `codesign --verify --deep --strict --verbose=2 /Applications/Fe.app` -> `exit 0`.
- Retained `build/generation` (`12M`) and `build/manuals` (`1.4G`) (`build` total `1.4G`, `dist` total `445M`).
- Post-cleanup smoke (`run_id=dafaf76fbeea4132a2b2c2a9e70bcda1`, `exit=0`, `cleanup_complete=true`, `7.31s`): `dist/lisp/bin/femacs-lisp --eval '(+ 40 2)'` -> `42`, `dist/lisp/bin/http-host --version` -> `http-host 0.1.0 (femacs 32.0.50)`, `build/terminal/port/src/emacs -Q --batch` -> `42` + `treesit` + `gnutls`, `dist/macos/Femacs.app` and `/Applications/Fe.app` -> `42` + `treesit` + `gnutls` + `ns`, installed CLI wrappers `femacs` / `fe` -> `42` + `treesit` + `gnutls`, and `feclient --version` -> `emacsclient 32.0.50`.
