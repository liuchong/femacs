# CLN281 — U5 zero-unsafe module lock-in, pure `lib/` Safe Rust migration, and consumed AU5 staging retirement

Date: 2026-11-22
Scope: Phase 0 Stage `U5` (`零 unsafe 锁门与 lib/ 纯逻辑安全化`, `P-484`) and formal acceptance `AU5` (`lib/ 与工具安全锁门`).

## Context & Verified Deliverables

- Implemented `#![forbid(unsafe_code)]` arbitrary-precision engine `crates/femacs-pure/src/mini_gmp.rs` (`overflow-checks = true`) and pure signal/byte-search helpers `crates/femacs-pure/src/{signal_names,memmem}.rs`; narrowed `lib/{mini-gmp,memrchr,sig2str,sigdescr_np}.rs` to minimal `SAFETY:`-documented C-ABI shims while converting `lib/rawmemchr.rs` to `#![forbid(unsafe_code)]`.
- Converted `lib/include/{stdbit.in,gettext,stddef.in,verify,stat-time}.rs`, `lib/{stat-time,timespec,timespec-add,timespec-sub,dtotimespec,acl-errno-valid,byteswap,c-ctype,endian,u64,stdc_*}.rs`, `29` `xtask/src/**` modules (isolating `xtask/src/os_ffi.rs`), `cli/src/macos.rs`, `crates/femacs-{sys,vm}/build.rs`, `src/include/{gnutls-types,nsgui}.rs`, `lwlib/include/lwlib-widget.rs`, and `30` pure `lib/include/*.rs` headers to `#![forbid(unsafe_code)]` (`32 -> 122` active `#![forbid(unsafe_code)]` files; `6,933 / 6,933` `SAFETY:`-documented active `unsafe` blocks; `293 -> 285` isolated boundary files).
- Verified `AU5` via full `xtask build-terminal`, `check-unsafe-boundary.py` (`0` errors on source + `temacs` `nm`), `check-ledger-ticks.py` (`0` failures), `check-libshadows.py` (`0` failures), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`), ERT suites (`data-tests`, `floatfns-tests`, `fns-tests`, `timefns-tests`, `fileio-tests`), end-to-end bignum/bitwise/digest/version/stat-time checks, and protective unit suites (`femacs-pure` `10/10`, `xtask` `116/116`).

## Retired Superseded & Zero-Consumer Build Scopes

1. Prior to the `U5` build, retired the superseded `U4` `build/runtime/u4-native` (`999M` pre-delete `du -sh`), earlier consumed `build/{acceptance,cargo-tools,manuals,generation}` (`345M` combined pre-delete `du -sh`), and superseded `U4` `build/terminal` (`158` files, `6` symlinks, `99` directories, `243,611,752` logical bytes).
2. Following `AU5` and unit-test completion, swept `19` consumed staging, manual, tool-cache, test-scratch, and `P-190` dump-alias scopes (`3,291` files, `2` symlinks, `302` directories, `356,220,914` logical bytes):

| Removed scope | Measured pre-delete `du -sh` | Replacement / Recovery |
| --- | ---: | --- |
| `build/manuals` | 141M | `cargo run -p xtask -- build-manuals` |
| `build/cargo-tools` | 17M | `cargo run -p xtask -- link-program <tool>` |
| `build/generation` | 1.2M | `tools/bootstrap-resources.py` via `xtask build-terminal` |
| `build/acceptance` + `6` empty `xtask` unit-test scratch dirs | 244K | `cargo test -p xtask` / `test-tooling-modernization.py` |
| `build/terminal/{cargo,config,image,image-staging,lcms,tls,treesit}` + `build/terminal/port/src/emacs-32.0.50.1{,.pdmp}` (`9` scopes) | ~188M (`215M -> 45M`) | `build/terminal/port`; `cargo run -p xtask -- build-terminal` |

## Retained Working Set & Retirement Triggers

- `build/terminal/port` (`45M`: `src/emacs` `4,355,504 B` SHA-256 `4cc93fae8a90e2ed0c807f618880611a4c3d6473274c93c0991bec7468e7623a`, `src/emacs.pdmp` `13,101,352 B` SHA-256 `975ff3597da5a14a9500d90a644982421bb34fbda27b70ac9924d9e018637819`, `lib-src/emacsclient` `564,976 B` SHA-256 `383644deb3002212a39e15d2f8b893db39c5073831eb4e2906f5986d4c31662b`, `lib-src/hexl` `475,072 B` SHA-256 `d44f1c449986013e4a8acef87036b8903ebb27b57442f093ef0165953103b840`), re-verified post-cleanup via `-batch -Q` smoke (`POST_CLN281_SMOKE_OK`, `exit 0`).
- `dist/macos/Femacs.app` (`235M`, retained trial bundle; refreshed at `U8`/`AU8`).
- `build/cargo` (`1.7G` incremental workspace cache) and `<session-cache>/tree-sitter-0.27.1` (`604K` pinned SDK input): actively consumed by `U6–U8` builds and scheduled for retirement at `AU8` completion.
