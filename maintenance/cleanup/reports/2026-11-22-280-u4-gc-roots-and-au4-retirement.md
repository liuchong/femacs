# CLN280 — U4 GC roots, VM state, safe boundary closure, and consumed AU4 build staging retirement

Date: 2026-11-22
Scope: Phase 0 Stage `U4` (`根与状态收口`, `P-483`) and formal acceptance `AU4` (`GC 根完备`).

## Context & Verified Deliverables

- Added `crates/femacs-vm/src/{roots,state,vm_primitives,safe_domain}.rs`: SpiderMonkey-style `LispVal`, `GcContext`, `NoGcToken`, `UnrootedBorrow`, LIFO `Rooted`/`Handle`/`MutableHandle`, progressive `ScopedRawSpan` (`Flet`, `eval_sub`, `apply_lambda`), generation-checked `PersistentRoot`, single-owner `VmState`/`ThreadState` with 9-category root inventory, `sa_stack_frame_exit` dead-frame scrub (`P-172`-preserving), and `#![forbid(unsafe_code)]` high-level Buffer, Reader, File I/O, and Window operations.
- Removed all remaining file-level `#![allow(static_mut_refs)]` (`120 -> 0`) and `#![allow(unsafe_op_in_unsafe_fn)]` (`120 -> 0`) across active product files, enabled `#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]` across `femacs-sys`, `femacs-vm`, `femacs-host`, and `xtask`, enabled `#![forbid(unsafe_code)]` on `cli`, `packaging`, `safe_domain.rs`, `stdcountof.rs`, and `macuvs.rs` (`32` forbid files total), documented `7,162 / 7,162` (`100.0%`) active `unsafe` blocks with `SAFETY:` contracts, and bound all `293` isolated active boundary files, `9` root categories, and `4` conservative scan entries in `tools/unsafe-boundary-manifest.json`.
- Verified `AU4` (`tools/test-u4-gc-roots.py` `4/4` groups passed in `28.241s`, `codesign --verify --deep --strict` `exit 0`) and `U4` protective unit tests (`tools/test-u4-unit.py` `3/3` groups passed in `1.982s`).

## Retired Zero-Consumer Build & Staging Scopes

Reclaimed `14` consumed staging, manual, generation, acceptance, tool-cache, and `P-190` dump-alias scopes (~`648 MB` measured via `du -sh` immediately prior to removal; exact scope-level allocations recorded below without fabricating unprinted file counts):

| Removed scope | Measured pre-delete `du -sh` | Replacement / Recovery |
| --- | ---: | --- |
| `build/macos` | 252M | `dist/macos/Femacs.app`; `cargo run -p xtask -- build-macos` |
| `build/manuals` | 186M | `cargo run -p xtask -- build-manuals` |
| `build/cargo-tools` | 35M | `cargo run -p xtask -- link-program <tool>` |
| `build/generation` | 1.6M | `tools/bootstrap-resources.py` via `xtask build-terminal` / `build-macos` |
| `build/acceptance` | 76K | `python3 tools/test-u4-gc-roots.py` & `python3 tools/test-u4-unit.py` |
| `build/terminal/{cargo,config,image,image-staging,lcms,tls,treesit}` + `build/terminal/port/src/emacs-32.0.50.1{,.pdmp}` (`9` scopes) | 173M (`217M -> 44M`) | `build/terminal/port`; `cargo run -p xtask -- build-terminal` |

## Retained Working Set & Retirement Triggers

- `build/terminal/port` (`44M`: `emacs` `55b7acf2b54da999820dacd18d670a3d52afc87e4ea68829cfa383585f620eff`, `emacs.pdmp` `07961a36855f0940f0aa690109f67b2b72b13ade9eb04980f0494651ebba175b`, `emacsclient` `cccc5b554c710156b67cdde580708b7c382eb5ca5a3ed49996f5fef1b711f13e`, `hexl` `d44f1c449986013e4a8acef87036b8903ebb27b57442f093ef0165953103b840`), verified post-cleanup via `-Q --batch` smoke (`exit 0`, `+x` preserved).
- `dist/macos/Femacs.app` (`235M`: `Femacs` `e4296a2449bce1e2124b70d7f3818d95bd45d8e1cac56901c54a6fc1badca70e`, `Femacs.pdmp` `b8e2559720aba8957778e32055e706e9a022dc18f637c80dd0e6a55097223a5a`, `emacsclient` `aa8c5f9907c64f121410acf3f16193bc74dab713c158fb65ee404fb3db8e16d2`, `hexl` `3350e1b2d8141c3d6f983d6f787271c2519bf3811156fa5e96443467b5819e07`), verified post-cleanup via `codesign --verify --deep --strict` (`exit 0`).
- `build/runtime/u4-native` (`999M`: `normal` `emacs` `22738c5ba2ef6c0083725250f1c8566a79233ec3bc1ec61ab1635ceed583075a`, `checking` `emacs` `cfbabf31b445010ce16a52463d1248dc3d9f1d9daa6a0ecbb8ecd1419f97d6ba`), `build/cargo` (`1.6G`), `<session-cache>/u1-baseline`, and `<session-cache>/tree-sitter-0.27.1`: actively consumed by Stage `U5` (`AU5` native/GC/performance & subset verification); scheduled for full retirement at `AU5` completion.
