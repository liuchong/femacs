# CLN283 — U7 `#![forbid(unsafe_code)]` high-level subsystems, `RootedVec` & narrowed stack scan, and consumed AU7 staging retirement

Date: 2026-11-22
Scope: Phase 0 Stage `U7` (`编辑器高层与核心原语批量去 unsafe`, `P-486`) and formal acceptance `AU7` (`高层子系统去 unsafe 验证`).

## Context & Verified Deliverables

- Added `20` new `#![forbid(unsafe_code)]` active modules (`crates/femacs-vm/src/safe_{casetab,category,floatfns,undo,macros,doc,casefiddle,callint,sort,json,marker,character,syntax,keymap,textprop,search,data,buffer,window}.rs` and `src/include/buildobj.rs`), expanding active-product `#![forbid(unsafe_code)]` files from `123 -> 143` (`21` inside `crates/femacs-vm/src/`).
- Upgraded `xtask gen-globals` (`pub safe static {name}: VmGlobal<{ty}>;` across all `836` `DEFVAR_*` globals), added `DEFUN!(..., safe { ... })` arms rejecting any `unsafe` inside fixed-arity (`0..=8`) and `UNEVALLED` subr bodies, implemented `RootedVec` + `NarrowedStackScanGuard` / `with_narrowed_stack_bottom` in `crates/femacs-vm/src/roots.rs`, and stripped compiler-verified redundant mechanical `unsafe` blocks and pure-safe internal `unsafe fn`s across `src/*.rs`:
  - `active_product.unsafe_blocks`: **`6,242`** (`-2,408` / `-27.8%` vs `U6` `8,650`; `-920` / `-12.8%` vs `U4` `7,162`)
  - `active_product.unsafe_fn_defs`: **`4,333`** (`-579` vs `U6` `4,912`; `-592` vs `U4` `4,925`)
  - `active_product.safety_documented_blocks`: **`6,242 / 6,242` (`100.0%`)**
  - `active_product.static_mut_items`: **`0`**
- Verified `AU7` (`tools/test-u7-safe-subsystems.py` `4/4` groups passed): `static-and-nm-gates` (`28.45s`), `safe-subsystems-functional` (`0.09s` across all 19 new subsystems), `core-ert-suites` (`5.70s` across `20` official ERT suites with `0` failures), and `gc-stress-and-stack-narrowing` (`0.19s`, `U7_GC_STRESS_OK gcs=72` under `FEMACS_GC_SKIP_STACK_SCAN=1` + `gc-cons-threshold=1`), plus protective unit tests (`u7 selftest` `5/5` including `2/2` isolated `roots.rs` Rust unit tests, `test-u4-unit.py` `3/3`, `test-u3-unit.py` `3/3`, `femacs-pure` `10/10`, `femacs-sys` `3/3`, `check-ledger-ticks --selftest` `6/6`, `check-defvar-carriers --selftest` `7/7`, `check-unsafe-boundary --selftest` `6/6`, `test-repository-bootstrap` `4/4`, `test-tooling-modernization` `33/33`, `check-own-voice` `0` failures).

## Retired Superseded & Zero-Consumer Build Scopes

1. Prior to the final `U7` build, retired the superseded `U6` `build/terminal` (`45M` pre-delete `du -sh`) and intermediate rebuild attempts.
2. Following `AU7` and protective unit-test completion, swept `13` consumed terminal staging, manual, tool-cache, generation, acceptance, and `P-190` dump-alias scopes (`2,484` files, `0` symlinks, `170` directories, `322,811,672` logical bytes):

| Removed scope | Measured pre-delete `du -sh` | Replacement / Recovery |
| --- | ---: | --- |
| `build/terminal/{cargo,config,image,image-staging,lcms,tls,treesit}` + `build/terminal/port/src/emacs-32.0.50.1{,.pdmp}` (`9` scopes) | ~187M (`232M -> 45M`) | `build/terminal/port`; `cargo run -p xtask -- build-terminal` |
| `build/manuals` | 96M | `cargo run -p xtask -- build-manuals` |
| `build/cargo-tools` | 18M | `cargo run -p xtask -- link-program <tool>` |
| `build/generation` + `build/acceptance` | 944K | `tools/bootstrap-resources.py` / `test-tooling-modernization.py` |

## Retained Working Set & Retirement Triggers

- `build/terminal/port` (`45M`: `src/emacs` `4,487,344 B` SHA-256 `74a5737b7f8ffa328c9bd9f98ba87ca233d5a906dc39dc920e07f86192dfdf55`, `src/emacs.pdmp` `13,101,368 B` SHA-256 `cbb55fc44223bb78fef90c81d696d1346fcfc69634719d2b28d8bbe0e34048fd`, `lib-src/emacsclient` `564,976 B` SHA-256 `383644deb3002212a39e15d2f8b893db39c5073831eb4e2906f5986d4c31662b`, `lib-src/hexl` `475,072 B` SHA-256 `d44f1c449986013e4a8acef87036b8903ebb27b57442f093ef0165953103b840`), re-verified post-cleanup via `-Q --batch` smoke (`POST_CLN283_SMOKE_OK`, `exit 0`).
- `dist/macos/Femacs.app` (`235M`, retained trial bundle; refreshed in `U8`/`AU8`).
- `build/cargo` (`2.4G` incremental workspace cache), `<session-cache>/tree-sitter-0.27.1` (`604K` pinned SDK input), and `<session-cache>/u1-baseline` (`616K` baseline metrics JSON): actively consumed by `U8` (`集中整合与终验`) and scheduled for retirement at `AU8` completion.
