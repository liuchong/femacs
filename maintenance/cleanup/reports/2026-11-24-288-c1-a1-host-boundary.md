# CLN288 — Stage C1 & A1 Headless Host Boundary & Cache Retirement (2026-11-24)

## Summary

Following Codex App independent final review approval (`[FEMACS-AU8-FINAL-REVIEW-R4]`, `c187c486`) unlocking Phase II `C1–C6`, implemented Stage `C1` (safe single-process headless `LispHost` boundary in `crates/femacs-host` and `crates/femacs-vm/src/host_runtime.rs` + `src/emacs.rs::init_headless_vm_body`) and Stage `A1` (`a1_host_acceptance` and `tools/test-c1-a1-host.py`), rebuilt Terminal, integration/unit test binaries, and Cocoa from the final source tree (`task-38888`), passed all `A1`, `U3`/`U4`/`U7`, workspace unit, and hygiene checks (`task-38893`), atomically reinstalled `/Applications/Fe.app`, verified post-cleanup batch/CLI smoke (`run_id=98e08a361bd341fb87966635eba108fb`), and retired `22` consumed temporary build, cache, staging, test-binary, and hardlink-alias scopes.

## C1 / A1 Scope & Verification Receipts

- **Headless VM & `#![forbid(unsafe_code)]` Host Interface (`src/emacs.rs`, `crates/femacs-vm/src/{lib,host_runtime}.rs`, `crates/femacs-host/src/{lib,bin/temacs}.rs`, `crates/femacs-host/Cargo.toml`)**:
  - Factored shared VM initialization helpers in `src/emacs.rs` (`grow_regex_stack_limit`, `init_vm_once_tables`, `init_vm_core_subsystems`, `syms_of_vm_all`, `init_vm_display_subsystems`) without altering `emacs_init_body` behavior, and implemented `init_headless_vm_body` to initialize single-process batch/headless VM subsystems (optional `.pdmp` load, zero GUI window-system/TTY activation).
  - Implemented `crates/femacs-vm/src/host_runtime.rs` (`#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]`) enforcing single-init `HOST_INIT_STATE` CAS (`AlreadyInitialized`), live stack-bottom anchoring (`with_host_stack_anchor` without shrinking nested frames), condition-case exit capture & classification (`VmHostError::{AlreadyInitialized, InitFailed, Read, LispCondition, Quit, Io}`), `read_eval_to_string`, `load_script_file` (with `command-line-args-left`/`argv` dynamic binding & `specpdl` cleanup), `garbage_collect`, `inspect_headless_environment` (`HeadlessDiagnostics`), `with_anchored_context`, and VM lock unbinding on shutdown/init error (`unbind_vm_lock_owner`).
  - Converted `crates/femacs-host/src/lib.rs` to `#![forbid(unsafe_code)]` exporting `!Send + !Sync` `LispHost`, rewrote `crates/femacs-host/src/bin/temacs.rs` via `femacs_host::femacs_vm::editor_main`, and added `#![forbid(unsafe_code)]` `#[cfg(test)]` protective unit tests (`3/3`, including compile-time `Send`/`Sync` negative probes for `LispHost` and `GcContext`).
- **Build & Acceptance Tooling (`xtask/src/{standalone,cargo_build,main}.rs`, `crates/femacs-host/tests/a1_host_acceptance.rs`, `tools/test-c1-a1-host.py`, `tools/unsafe-boundary-manifest.json`, `tools/test-u4-unit.py`, `maintenance/tooling-inventory.json`)**:
  - Updated `xtask/src/cargo_build.rs` to pass host link flags via `CARGO_ENCODED_RUSTFLAGS` (covering prerequisite `bin temacs`, integration `--test` harnesses, and `--lib` unit test binaries), generalized `link-host-test` / `link-host-test-inner` (including `--lib` -> `femacs_host_unit_tests`), and unified recursive artifact lookup across `deps/` and `release/build/*/out/` (covered by `xtask` unit tests, `116/116` passed).
  - Added `crates/femacs-host/tests/a1_host_acceptance.rs` (`#![forbid(unsafe_code)]`, `harness = false`) and `tools/test-c1-a1-host.py`, moved `crates/femacs-host/src/lib.rs` into `forbid_unsafe_modules` and registered `crates/femacs-vm/src/host_runtime.rs` in `isolated_unsafe_boundaries`.
- **Clean Rebuild & Final Verification Suite (`assistant run`)**:
  - Clean rebuild (`run_id=75e175dbb51846a48ef453182e51fbe1`, `571.551s`, `exit=0`, `cleanup_complete=true`, peak accounted `2,203,306,744` B): `cargo test -p xtask` (`116/116`), `build-terminal`, `link-host-test a1_host_acceptance`, `link-host-test lib`, and `build-macos` built from the exact final source tree.
  - Full verification suite on rebuilt binaries (`run_id=cb1531690af3400fbaff363eff92182e`, `116.46s`, `exit=0`, `cleanup_complete=true`):
    - `femacs_host_unit_tests` (`3/3` ok)
    - `tools/test-c1-a1-host.py` (`ok=true`, `37.7158s`):
      - `static-and-abi-gates` (`34.1777s`, Terminal & Cocoa): `error_count=0`, `total_files=438`, `forbid_unsafe_files=145`, `allow_unsafe_op_files=0`, `allow_static_mut_refs_files=0`, `static_mut_items=0`, `stitching_includes=0`, `generated_includes=3`, `unsafe_blocks=6,308`, `safety_documented_blocks=6,308/6,308 (100.0%)`, `unsafe_fn_defs=4,374`, `extern_c_fn_defs=2,325`, `extern_c_blocks=834`, `unsafe_impl_traits=10`; `nm -gU` ABI exports & `check-libshadows.py` ok on both.
      - `a1-host-lifecycle-and-scripts` (`2.1780s`, `13` sub-runs): `cold_minimal_lifecycle` (normal + `FEMACS_GC_SKIP_STACK_SCAN=1`), `headless_diagnostics` (normal + `FEMACS_GC_SKIP_STACK_SCAN=1`, verifying `noninteractive=t`, `inhibit_window_system=true`, `display_graphic_p=false`, `frame_count=1` `("t")`, `terminal_count=1` `("t")`), `cold_loadup_and_script_normal`, `safe_domain_with_context` (normal + `.pdmp` `FEMACS_GC_SKIP_STACK_SCAN=1`), `self_dump_create`, `dump_file_roundtrip` (normal + `FEMACS_GC_SKIP_STACK_SCAN=1`), `bad_dump_mismatch_rejected`, `bad_dump_corrupt_rejected`, and `second_init_rejected` (including post-`Drop` `GcContext::current().is_none()`).
      - `same-source-editor-fidelity` (`1.3599s`): both Terminal and Cocoa returned `(t 42 "alpha gamma\n")` for byte-compiled helper, regex replace, and `undo` cycle.
    - `tools/test-c1-a1-host.py --selftest` (`4/4`), `tools/test-u4-unit.py` (`4/4` groups, `25.8485s`), `tools/test-u3-unit.py` (`3/3`, `8/8` Rust tests), `tools/test-u7-safe-subsystems.py` (`4/4`, `22` modules, `20` ERT suites, `U7_GC_STRESS_OK gcs=72`), `cargo test -p femacs-pure -p femacs-sys --lib` (`10/10` + `3/3`), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), and `test-tooling-modernization.py` (`33/33`, `1.868s`).

## Retired Temporary & Verification Scopes

Retired `22` owned temporary build, cache, staging, test-binary, and hardlink-alias scopes totaling `3,894` files, `861` directories, `14` symlinks, `1,001,098,146` logical bytes, normalized digest `2a390cbaf3c49cf6c0ac0b6304ce3c9cffbf6cddcfc97052059c4f19104fe129` (including hardlinked `P-190` aliases `emacs-32.0.50.1{,.pdmp}` pointing to retained `emacs{,.pdmp}`):

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `build/acceptance` | `436K` | `84` | `307` | `6` | `151,401` | Recreated by `xtask test` / acceptance runners |
| `build/android-tests` | `0B` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/bundle-tests` | `0B` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/cargo` | `47M` | `686` | `36` | `0` | `54,498,637` | `CARGO_TARGET_DIR` workspace cache (rebuilt on demand) |
| `build/cargo-tools` | `284M` | `2,762` | `341` | `0` | `420,558,702` | `xtask` host tool target dir (rebuilt on demand) |
| `build/macos` | `267M` | `210` | `98` | `6` | `302,754,298` | Recreated by `cargo run -p xtask -- build-macos` |
| `build/manuals-review` | `0B` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/package-contracts` | `0B` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/probes-tests` | `0B` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/tooling-tests` | `0B` | `0` | `0` | `0` | `0` | Empty test scratch dir from `cargo test -p xtask` |
| `build/treesit-0.27.1` | `604K` | `5` | `7` | `2` | `607,238` | Reinstall via `python3 tools/toolchain-pins.py install` |
| `build/terminal/cargo` | `194M` | `94` | `51` | `0` | `187,826,365` | Recreated by `cargo run -p xtask -- build-terminal` |
| `build/terminal/config` | `48K` | `7` | `0` | `0` | `35,082` | Recreated by `xtask build-terminal` |
| `build/terminal/image` | `1.3M` | `3` | `2` | `0` | `1,270,503` | Recreated by `xtask build-terminal` |
| `build/terminal/image-staging` | `1.3M` | `2` | `0` | `0` | `1,269,940` | Recreated by `xtask build-terminal` |
| `build/terminal/lcms` | `364K` | `5` | `3` | `0` | `360,608` | Recreated by `xtask build-terminal` |
| `build/terminal/tls` | `7.6M` | `26` | `10` | `0` | `7,878,424` | Recreated by `xtask build-terminal` |
| `build/terminal/treesit` | `316K` | `6` | `6` | `0` | `307,980` | Recreated by `xtask build-terminal` |
| `build/terminal/port/src/a1_host_acceptance` | `4.6M` | `1` | `0` | `0` | `4,816,960` | Rebuilt by `xtask link-host-test a1_host_acceptance` |
| `build/terminal/port/src/femacs_host_unit_tests` | `924K` | `1` | `0` | `0` | `946,464` | Rebuilt by `xtask link-host-test lib` |
| `build/terminal/port/src/emacs-32.0.50.1` | `4.5M` | `1` | `0` | `0` | `4,714,176` | Hardlink alias to retained `build/terminal/port/src/emacs` |
| `build/terminal/port/src/emacs-32.0.50.1.pdmp` | `13M` | `1` | `0` | `0` | `13,101,368` | Hardlink alias to retained `build/terminal/port/src/emacs.pdmp` |

## Retained Working Set, Post-Cleanup Verification & Application Bundle Parity

- `build/terminal` (`45M`, holding `build/terminal/port/`):
  - `build/terminal/port/src/emacs`: `4,714,176` B, SHA-256 `9952ddd4710feebaaf572479dc1ebd9f6ab271bde9e81fac794c68a42fb88fa0`
  - `build/terminal/port/src/emacs.pdmp`: `13,101,368` B, SHA-256 `96828ce7a7c1db398639af0d40a1c5d803b4facd07a3834c1d30fcc59d084fa9`
  - `build/terminal/port/lib-src/emacsclient`: `539,328` B, SHA-256 `1baf54e687c5fd61259f6699a2d2953b7e6567e350a304942b2e06f38ba600c7`
  - `build/terminal/port/lib-src/hexl`: `451,488` B, SHA-256 `15f41ba92128f2549c6e7f8f3e6685f379b9dec79d0f34b4696a4e5dda4878af`
- `dist/macos/Femacs.app` (`235M`) & atomically reinstalled `/Applications/Fe.app` (`235M`, `diff -rq` empty, `DIFF_EMPTY=0`):
  - `Contents/MacOS/Femacs`: `6,247,232` B, SHA-256 `e007fa58055a1d457afa7574d127e1cecce1f1b3d56eccd72ed75fbc0790eefe` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,600,760` B, SHA-256 `816242cf56bc4a4011cfe7dae55b4abdbaed34215baa6dbdb00b027bad6310f5` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/emacsclient`: `538,784` B, SHA-256 `ca959192dfd327a692c9bdc76b1c12ae7889652a0e65ca11193adc53bfafa755` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/hexl`: `451,120` B, SHA-256 `2ee11f3c9964f5aaa3465c07006a78f9d98487c56c81808faf42e3fc20ef7014` (exact match across workspace & `/Applications/Fe.app`)
  - `codesign --verify --deep --strict --verbose=2 dist/macos/Femacs.app` -> `exit 0`; `codesign --verify --deep --strict --verbose=2 /Applications/Fe.app` -> `exit 0`.
- Retained `build/generation` (`7.6M`) and `build/manuals` (`971M`) (`build` total `1.0G`, `dist` `235M`).
- Post-cleanup `-Q --batch` smoke (`run_id=98e08a361bd341fb87966635eba108fb`, `exit=0`, `cleanup_complete=true`, `1.865s`): `build/terminal/port/src/emacs`, `dist/macos/Femacs.app/Contents/MacOS/Femacs`, and `/Applications/Fe.app/Contents/MacOS/Femacs` all returned `(3 t nil nil t)` for `(list (+ 1 2) (treesit-available-p) (featurep 'treesit) (null (require 'treesit)) (featurep 'treesit))`, installed CLI wrappers `femacs` and `fe` returned `(3 t)`, and `feclient --version` returned `emacsclient 32.0.50`.
