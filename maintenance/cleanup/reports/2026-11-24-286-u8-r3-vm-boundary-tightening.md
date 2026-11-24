# CLN286 — U8-R3 VM Boundary Tightening, Targeted Re-Acceptance & Cache Retirement (2026-11-24)

## Summary

After closing the sole remaining AU4/AU6 safe/unsafe API boundary blocking finding from the independent `502e87b8` review (`R2`), fixing a deterministic `FEMACS_GC_SKIP_STACK_SCAN=1` rooting bug uncovered in `eval_sub` / `readevalloop` during R3 verification, rebuilding both deliverables (`build/terminal/port`, `dist/macos/Femacs.app`) and `build/runtime/u8-native` (`normal` + `checking`), passing all unit, static-boundary, ABI-export, `U3`/`U4`/`U7`, and `AU8 --group performance` (`bench.py --gate`) re-verification suites (with the `555` corpus and Cocoa UI sweeps explicitly reused from `502e87b8` per `R2` guidance), atomically reinstalling `/Applications/Fe.app`, and confirming zero active child processes, this batch retired `15` consumed temporary build, runtime, acceptance, staging, and hardlink-alias scopes.

## Accepted R3 Scope & Re-Verification Receipts

- **VM-kernel safe/unsafe boundary closure**:
  - `crates/femacs-vm/src/roots.rs`: changed `LispVal::from_raw` to `pub unsafe fn`; restricted `bind_vm_lock_owner` and `unbind_vm_lock_owner` to `pub(crate) unsafe fn` with OS-thread ownership checks (`prev_tid == 0 || prev_tid == tid`), active-lease/borrow checks (`!GC_CONTEXT_ACTIVE`, `!TLS_SLAB.borrowed`, `!nonlocal::tls_state_borrowed()`), and `TLS_SLAB.vm_lock_key == thread_key` validation in `is_vm_lock_owner`; tightened `mark_thread_u4_slab_roots`, `cleanup_thread_u4_slab`, and `mark_persistent_roots` to `pub(crate) unsafe fn` asserting `is_current_os_thread_vm_lock_owner()` and `!borrowed`; tightened `ScopedRawSpan` / `RootedVec` / `rooting_counters()` to `pub(crate)` (adding safe `GcContext::rooting_counters(&self)`); moved `with_narrowed_stack_bottom` onto `GcContext::with_narrowed_stack_bottom(&mut self, ...)`; marked raw-pointer C-ABI `femacs_vm_persistent_root_alloc` as `pub unsafe extern "C" fn` while preserving exact `#[unsafe(no_mangle)]` symbols and C ABI.
  - `crates/femacs-vm/src/nonlocal.rs`, `state.rs`, `vm_primitives.rs`, `safe_domain.rs`, `src/comp.rs`: restricted `ScopedLispRoot` (`unsafe fn new`/`set`), `cleanup_thread_state`, `mark_thread_explicit_roots`, `{raise,resume}_vm_nonlocal`, `{set,consume,check_and_propagate}_comp_native_exit`, `VmState`/`ThreadState` (`!Send + !Sync` + lock-owner guards), and telemetry helpers to `pub(crate)`; marked raw-pointer C-ABI probes `femacs_vm_host_drop_counts` and `femacs_vm_u4_root_stats` as `pub unsafe extern "C" fn`; updated all 12 `LispVal::from_raw` calls in `vm_primitives.rs` and `.eln` exit helpers in `src/comp.rs` with documented `unsafe` blocks; migrated `safe_domain.rs` (`#![forbid(unsafe_code)]`) to `ctx.with_narrowed_stack_bottom`.
  - `src/eval.rs` & `src/lread.rs`: moved `eval_sub`'s `maybe_gc ()` safepoint to immediately follow `SPECPDL_BACKTRACE` registration (matching `Ffuncall` and `bc_call_housekeeping_slow`) so freshly read forms on the `.elc` direct `readevalloop -> eval_sub` path cannot be collected under `FEMACS_GC_SKIP_STACK_SCAN=1`; updated `readevalloop` to preserve `val` via `val = unbind_to (count1, val)` and rooted `subforms` across recursive `readevalloop_eager_expand_eval` via `ScopedRawSpan`.
  - Protective boundary regression tests (`tools/test-u4-unit.py`, `tools/test-u3-unit.py`, `tools/test-u7-safe-subsystems.py`, `tools/test-u4-gc-roots.py`): `test-u4-unit.py` (`4/4` groups, including `12` in-crate tests + genuine separate `.rlib` `#![forbid(unsafe_code)]` external `compile-fail` checks + external `#![forbid(unsafe_code)]` `Mutex`-backed OS-thread handoff & out-of-order `Rooted` test), `test-u3-unit.py` (`3/3` groups, `8/8` Rust tests + structural C-ABI visibility assertions), and `test-u7-safe-subsystems.py --selftest` (`5/5`).
- **Active product static & ABI verification (`436` files, both deliverables)**:
  - `check-unsafe-boundary.py` on both `build/terminal/port/src/emacs` and `dist/macos/Femacs.app/Contents/MacOS/Femacs`: `errors=[]`, `forbid_unsafe_files=143`, `allow_unsafe_op_files=0`, `allow_static_mut_refs_files=0`, `static_mut_items=0`, `stitching_includes=0`, `generated_includes=3`, `unsafe_blocks=6,264`, `safety_documented_blocks=6,264/6,264 (100.0%)`, `unsafe_fn_defs=4,358`, `extern_c_fn_defs=2,325`, `extern_c_blocks=834`, `unsafe_impl_traits=8`.
  - `nm -gU` confirmed all 6 required `_femacs_vm_*` exports on both binaries; `cargo test -p femacs-pure -p femacs-sys -p xtask --lib` (`10/10` + `3/3`), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`), and `test-u8-au8-integration.py --group selftest` (`12/12`) passed (`run_id=c18f5c505c5c46aab00989fa6b28fc82`, `exit=0`, `32.539s`).
- **Targeted runtime & `bench --gate` verification (`assistant run --profile runtime`)**:
  - `tools/test-u4-gc-roots.py --native-dir build/runtime/u8-native`: `4/4` groups passed (`26.5003s`, `run_id=75557913bb1d4dc393d9799ee348d169`, `exit=0`, `cleanup_complete=true`), covering interpreter/bytecode/`FEMACS_GC_SKIP_STACK_SCAN=1`, Cocoa `.elc`, native `.eln` hybrid (`normal` & `checking`), `#![forbid(unsafe_code)]` safe domains, and fresh `.pdmp` dump/restore.
  - `tools/test-u3-nonlocal-exit.py`: `4/4` cases passed (`passed=4, failed=0`, `run_id=a92d6fa8c9c34f41a06e85cbffcbcd7d`, `18.458s`, `exit=0`, `cleanup_complete=true`).
  - `tools/test-u7-safe-subsystems.py`: `4/4` cases passed (`ok=true`, `22` modules, `20` ERT suites, `U7_GC_STRESS_OK gcs=72`, `run_id=324744609e5944f3b0b34c494709d8db`, `31.714s`, `exit=0`, `cleanup_complete=true`).
  - `tools/test-u8-au8-integration.py --group performance` (`--reference /Applications/Emacs.app/Contents/MacOS/Emacs`, `run_id=98c30c92460b462bbca2f97e64efede5`, `91.954s`, `exit=0`, `cleanup_complete=true`): `bench.py --gate` (`docs/baseline/bench-{cocoa,terminal}-ga8.tsv`, `10` baseline items) passed with `regressions=[]` on both Cocoa (`gc 30.727 vs 49.971 ms`, `edit 7.699 vs 9.517 ms`, `regex 2.369 vs 2.691 ms`, `load 1.653 vs 1.668 ms`; standalone `startup 24.497 vs 150.267 ms`) and Terminal (`gc 31.534 vs 50.368 ms`, `edit 7.659 vs 9.330 ms`, `regex 2.196 vs 2.699 ms`, `load 1.593 vs 1.725 ms`; standalone `startup 24.328 vs 148.987 ms`), plus `delivered-comparison` (`delivered-terminal-vs-reference-bytecode`: `bytecode 0.1094`, `edit 1.1435`, `gc-churn 0.6806`, `gc-retained 0.6511`, `load 0.3030`, `regex 1.0888`; `delivered-cocoa-vs-reference-bytecode`: `bytecode 0.1094`, `edit 1.1647`, `gc-churn 0.9172`, `gc-retained 0.7817`, `load 0.6086`, `regex 1.1121`) and `runtime-modes-comparison` (`passed=true`).

## Retired Temporary & Verification Scopes

Retired `15` owned temporary build, runtime, acceptance, staging, and hardlink-alias scopes totaling `12,122` files, `1,253` directories, `20` symlinks, and `1,923,285,065` logical bytes (note: `build/terminal/port/src/emacs-32.0.50.1{,.pdmp}` were hardlinked `P-190` aliases to retained `emacs{,.pdmp}`, contributing `17,833,848` logical bytes in entry totals without consuming additional backing blocks):

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `build/runtime` (`u8-native`) | `1002M` | `9,422` | `449` | `12` | `1,085,667,797` | `cargo run -p xtask -- runtime-build ...` |
| `build/cargo` | `47M` | `686` | `37` | `0` | `54,498,642` | `CARGO_TARGET_DIR` workspace cache (rebuilt on demand) |
| `build/cargo-tools` | `214M` | `1,360` | `337` | `0` | `248,467,188` | `xtask` host tool target dir (rebuilt on demand) |
| `build/acceptance` (`u8-r3-performance`) | `14M` | `308` | `251` | `0` | `13,566,753` | Re-run `tools/test-u8-au8-integration.py --group performance` |
| `build/macos` | `269M` | `210` | `99` | `6` | `306,285,430` | Recreated by `cargo run -p xtask -- build-macos` |
| `build/treesit-0.27.1` | `604K` | `5` | `8` | `2` | `607,285` | Reinstall via `python3 tools/toolchain-pins.py install` |
| `build/terminal/cargo` | `177M` | `80` | `45` | `0` | `185,235,585` | Recreated by `cargo run -p xtask -- build-terminal` |
| `build/terminal/config` | `48K` | `7` | `1` | `0` | `35,082` | Recreated by `xtask build-terminal` |
| `build/terminal/image` | `1.3M` | `3` | `3` | `0` | `1,270,503` | Recreated by `xtask build-terminal` |
| `build/terminal/image-staging` | `1.3M` | `2` | `1` | `0` | `1,269,940` | Recreated by `xtask build-terminal` |
| `build/terminal/lcms` | `364K` | `5` | `4` | `0` | `360,608` | Recreated by `xtask build-terminal` |
| `build/terminal/tls` | `7.6M` | `26` | `11` | `0` | `7,878,424` | Recreated by `xtask build-terminal` |
| `build/terminal/treesit` | `316K` | `6` | `7` | `0` | `307,980` | Recreated by `xtask build-terminal` |
| `build/terminal/port/src/emacs-32.0.50.1` | `4.6M` | `1` | `0` | `0` | `4,732,480` | Hardlink alias to retained `build/terminal/port/src/emacs` |
| `build/terminal/port/src/emacs-32.0.50.1.pdmp` | `13M` | `1` | `0` | `0` | `13,101,368` | Hardlink alias to retained `build/terminal/port/src/emacs.pdmp` |

## Retained Working Set, Post-Cleanup Verification & Application Bundle Parity

- `build/terminal` (`45M`, holding `build/terminal/port/`):
  - `build/terminal/port/src/emacs`: `4,732,480` B, SHA-256 `9464839edbe417be4006fef84a0093a1f9c52b056a39b11649cc3cf77e85c1cc`
  - `build/terminal/port/src/emacs.pdmp`: `13,101,368` B, SHA-256 `66f1d81d06b4864d8a920ae3e9aa9a65d468c3397eb166eba7f4b65c570a33a8`
  - `build/terminal/port/lib-src/emacsclient`: `539,328` B, SHA-256 `1baf54e687c5fd61259f6699a2d2953b7e6567e350a304942b2e06f38ba600c7`
  - `build/terminal/port/lib-src/hexl`: `451,488` B, SHA-256 `15f41ba92128f2549c6e7f8f3e6685f379b9dec79d0f34b4696a4e5dda4878af`
- `dist/macos/Femacs.app` (`235M`) & atomically reinstalled `/Applications/Fe.app` (`diff -rq` empty, exit `0`):
  - `Contents/MacOS/Femacs`: `6,282,528` B, SHA-256 `54d884c1a3c41c8b2889460cde2d500923cfe00accf2f8a25863bab61c839283` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,600,760` B, SHA-256 `92db122b012034a7e301323d239ca43cb778ca6ff62912b09e6767e53ac88530` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/emacsclient`: `538,784` B, SHA-256 `ca959192dfd327a692c9bdc76b1c12ae7889652a0e65ca11193adc53bfafa755` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/hexl`: `451,120` B, SHA-256 `2ee11f3c9964f5aaa3465c07006a78f9d98487c56c81808faf42e3fc20ef7014` (exact match across workspace & `/Applications/Fe.app`)
  - `codesign --verify --deep --strict dist/macos/Femacs.app` -> `exit 0`; `codesign --verify --deep --strict /Applications/Fe.app` -> `exit 0`.
- Retained `build/generation` (`4.4M`) and `build/manuals` (`557M`) unchanged.
- Post-cleanup `-Q --batch` smoke (`run_id=2fdb4d25f6c549069b5e552341dd2568`, `exit 0`): `build/terminal/port/src/emacs`, `dist/macos/Femacs.app/Contents/MacOS/Femacs`, and `/Applications/Fe.app/Contents/MacOS/Femacs` all returned `(3 t nil nil t)` for `(list (+ 1 2) (treesit-available-p) (featurep 'treesit) (null (require 'treesit)) (featurep 'treesit))`, installed CLI wrappers `femacs` and `fe` returned `(3 t)`, and `feclient --version` returned `emacsclient 32.0.50`.
