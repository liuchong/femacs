# CLN282 — U6 `VmGlobal` zero-`static mut` migration, safe `DEFVAR_*`/`DEFUN!` & kernel primitives, and consumed AU6 staging retirement

Date: 2026-11-22
Scope: Phase 0 Stage `U6` (`全局状态与安全内核原语`, `P-485`) and formal acceptance `AU6` (`全局状态与原语安全等价`).

## Context & Verified Deliverables

- Added `crates/femacs-sys/src/globals_cell.rs` (`#[repr(transparent)] pub struct VmGlobal<T>(UnsafeCell<T>)` with `const fn new`/`as_mut_ptr`/`as_ptr` and hardware-atomic `Relaxed` `.get()`/`.set()`/`.wrapping_add()` for `bool`, `i8..=u64`, `isize`, `usize`, `f64`, `*mut P`, and `*const P`), eliminating **`100.0%` (`2,507 -> 0`)** of active-product `static mut` items across all `417` active workspace files (`lib/` and `src/`).
- Upgraded `xtask gen-globals` (`src/include/globals.rs`), `tools/gen-macos-runtime.py`, `DEFVAR_LISP!`/`DEFVAR_LISP_NOPRO!`/`DEFVAR_INT!`/`DEFVAR_BOOL!`/`DEFVAR_KBOARD!`/`DEFVAR_PER_BUFFER!` (`SyncLispFwd`), `DEFUN!` (safe fixed-arity `0..=8` and `UNEVALLED` subr signatures + `globals::subrs` `pub safe fn`), `CALLN!`/`safe_calln!` (`call_many_slice`), and tag/bounds-checked kernel primitives (`XCAR`/`XCDR`/`XSETCAR`/`XSETCDR`/`AREF`/`ASET`/`ASIZE`/`SCHARS`/`SBYTES`/`SREF`/`SSET`/`STRING_MULTIBYTE`/`XFLOAT_DATA` + safe buffer/signal/specpdl helpers).
- Migrated `src/cmds.rs` implementation into `#![forbid(unsafe_code)]` `crates/femacs-vm/src/safe_cmds.rs` (`122 -> 123` active `#![forbid(unsafe_code)]` files; `src/cmds.rs` `0` `unsafe` blocks, `0` `unsafe fn`, `0` `static mut`).
- Verified `AU6` (`tools/test-u6-globals-and-primitives.py` `4/4` groups passed in `27.1s`): `static-gates`, `pdmp-and-defvar` (`13,217,192 B` roundtrip + dynamic unwind/per-buffer/kboard/watcher checks), `safe-primitives-and-ert` (`cmds-tests`, `data-tests`, `eval-tests`, `fns-tests`, `buffer-tests`, `editfns-tests`), and `native-eln` (`libgccjit` `.eln` compilation and execution against `VmGlobal` globals and safe `DEFUN!` subrs), plus protective unit tests (`femacs-sys` `globals_cell` `3/3`, `femacs-pure` `10/10`, `u6 selftest` `2/2`, `check-defvar-carriers --selftest` `7/7`, `check-unsafe-boundary --selftest` `6/6`, `test-tooling-modernization` `33/33`, `check-own-voice` `0` failures).

## Retired Superseded & Zero-Consumer Build Scopes

1. Prior to the `U6` build, retired the superseded `U5` `build/terminal` (`38` files, `6` symlinks, `27` directories, `46,188,640` logical bytes).
2. Following `AU6` and protective unit-test completion, swept `16` consumed `u6-native`, manual, tool-cache, generation, acceptance, terminal staging, `P-190` dump-alias, and temporary session-cache scopes (`12,703` files, `12` symlinks, `640` directories, `1,332,722,980` logical bytes):

| Removed scope | Measured pre-delete `du -sh` | Replacement / Recovery |
| --- | ---: | --- |
| `build/runtime/u6-native` (+ empty `build/runtime`) | 899M | `cargo run -p xtask -- runtime-build --product terminal --variant native --out build/runtime/u6-native` |
| `build/terminal/{cargo,config,image,image-staging,lcms,tls,treesit}` + `build/terminal/port/src/emacs-32.0.50.1{,.pdmp}` (`9` scopes) | ~207M (`252M -> 45M`) | `build/terminal/port`; `cargo run -p xtask -- build-terminal` |
| `build/manuals` | 139M | `cargo run -p xtask -- build-manuals` |
| `build/cargo-tools` | 18M | `cargo run -p xtask -- link-program <tool>` |
| `build/generation` + `build/acceptance` | 1.3M | `tools/bootstrap-resources.py` / `test-tooling-modernization.py` |
| `<session-cache>/{u6_rewrite_statics.py,cln282_pre.json}` | 56K | Committed `VmGlobal` sources + this `CLN282` report |

## Retained Working Set & Retirement Triggers

- `build/terminal/port` (`45M`: `src/emacs` `4,407,120 B` SHA-256 `8c4b38aedd3bf801051b6eca0457d5ceec68824fcc2998918807ee04628b85bc`, `src/emacs.pdmp` `13,101,368 B` SHA-256 `5c1174e817ff6fa1ffe9655267d8e8f2d8d9d165f4e1e62d4df701ddd5746fae`, `lib-src/emacsclient` `564,976 B` SHA-256 `383644deb3002212a39e15d2f8b893db39c5073831eb4e2906f5986d4c31662b`, `lib-src/hexl` `475,072 B` SHA-256 `d44f1c449986013e4a8acef87036b8903ebb27b57442f093ef0165953103b840`), re-verified post-cleanup via `-batch -Q` smoke (`POST_CLN282_SMOKE_OK`, `exit 0`).
- `dist/macos/Femacs.app` (`235M`, retained trial bundle; refreshed at `U8`/`AU8`).
- `build/cargo` (`1.8G` incremental workspace cache), `<session-cache>/tree-sitter-0.27.1` (`604K` pinned SDK input), and `<session-cache>/u1-baseline` (`616K` baseline metrics JSON): actively consumed by `U7–U8` and scheduled for retirement at `AU8` completion.
