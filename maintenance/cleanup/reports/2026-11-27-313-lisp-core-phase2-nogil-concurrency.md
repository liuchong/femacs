# CLN313 — Standalone `lisp-core` Phase 2: No-GIL Per-Isolate Multi-Core VM & Concurrency Stdlib (2026-11-27)

## Summary

Completed Phase 2 of the standalone general-purpose Lisp runtime roadmap across `crates/lisp-vm` and `crates/lisp-core`: added opt-in `isolate_vm` per-isolate VM global isolation (`default = []` preserved for editor builds; enabled by `lisp-core` `vm`), implemented `IsolateContext` / `IsolateScopeGuard` and idempotent `resolve_fwd_ptr` / `.pdmp` isolate relocations, guarded process-wide OS signal/locale/FD initialization, removed the process-wide GIL in `NoGil` mode (`Engine: Send` with runtime `--concurrency=no-gil|gil` and `LISP_CONCURRENCY` switch), shipped pure-Rust `std/task`, `std/chan`, and `std/sync` concurrency modules, verified `3.39x` parallel CPU speedup across `4` worker isolates, passed all unit, integration, `fe-test`, `cargo package`, and static boundary gates cleanly, and retired `3` temporary build/packaging/test scopes (`15,538` files, `1,700` dirs, `2` symlinks, `7,256,828,261` logical bytes).

## Delivered & Verified Evidence

- **`isolate_vm` Per-Isolate VM Storage (`crates/lisp-vm`)**:
  - `crates/lisp-vm/Cargo.toml` defines `isolate_vm = []` (`default = []`) so `femacs-host` / `femacs-tools` / `xtask` / `femacs` keep single-process static globals unchanged, while `crates/lisp-core` enables `lisp-vm/isolate_vm`.
  - `crates/lisp-vm/src/sys/isolate.rs` + `globals_cell.rs` implement `IsolateContext` (`Send`), `IsolateScopeGuard`, open-addressed slot cache + sorted static/reverse interval tables, size/overlap panic invariants, idempotent `resolve_fwd_ptr`, and clean `Drop` teardown.
  - Updated symbol/global forwarding (`XBARE_SYMBOL`, `make_lisp_symbol_internal`, `DEFVAR_*`, `XBOOLVAR`/`XINTVAR`/`XOBJVAR`, `defsubr`, `defvar_lisp`), `pdumper` isolate relocations + contiguous mark-bitset teardown, pre-registered composite globals (`alloc`, `buffer`, `charset`, `coding`, `eval`, `font`, `lread`, `pdumper`, `search`, `syntax`, `thread`, `xdisp`), per-isolate `/dev/urandom` FD cleanup (`close_isolate_random_fds` in `getrandom.rs`), and process-once OS guards (`emacs`, `sysdep`, `atimer`, `keyboard`, `process`, `callproc`).
- **No-GIL `Engine: Send`, CLI Switch & Concurrency Stdlib (`crates/lisp-core`)**:
  - Added `ConcurrencyMode` (`NoGil` default / `Gil`), `Engine: Send`, `EngineConfig::with_concurrency`, `--concurrency=no-gil|gil`, and `ENV_CONCURRENCY` (`LISP_CONCURRENCY` in `brand.rs`).
  - Added `crates/lisp-core/src/concurrency.rs` (`#![forbid(unsafe_code)]`) and `crates/lisp-core/stdlib/std/{task,chan,sync}.el` exposing `std-task-*` (threads, bounded worker pools, `std-task-parallel-map`), `std-chan-*` (bounded/unbounded typed channels returning uniform `(ok . VAL)` / `(closed . nil)`), and `std-mutex-*` / `std-atomic-*` synchronization primitives.
- **Test Suites, Release Binary & Multi-Core Benchmark**:
  - `cargo test -p lisp-vm --lib`: `12/12` passed (`run_id=b708af9a9d524d6ca2dd30d76a0bff6e`).
  - `cargo test -p lisp-vm --features isolate_vm --lib`: `16/16` passed (`run_id=6a8916ddb70842c89215e7cd25098500`).
  - `cargo test -p lisp-core`: `26/26` unit tests (`0` warnings in `lisp-core`) + `13/13` `engine_integration` subprocess cases (`engine_nogil_multi_thread_parallel_and_gc_stress`, `engine_nogil_send_across_threads`, `engine_nogil_std_task_chan_sync`, `engine_nogil_lifecycle_no_fd_leak`, `engine_gil_mode_preserves_single_init`, and `run_cli_subcommands` including warm `.pdmp` `warm-gc-ok`).
  - Workspace checks (`run_id=5c1e36eed2f64145b3d2c3f5c099aba0`): `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`, `cargo check -p lisp-core --no-default-features --features vm`, `xtask --bin xtask` (`119/119`), `http-host` (`9/9`), `femacs --lib` (`0/0`), `fe-test --suite self` (`9/9`, `8.192s`), and `fe-test --suite all --no-cache` (`39/39`, `wall=15.948s`, `caseSum=39.909s`, `peakMem=212.27 MiB`, `run_id=6a8916ddb70842c89215e7cd25098500`).
  - Release build (`run_id=13f4f3f5e9a2447396ac60c525b0fcef`) & warm-`.pdmp` runtime verification (`run_id=4412c3a6671a406a827fc385745fa347`): `otool -L` links exclusively `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`; `--concurrency=no-gil` (`no-gil`), `--concurrency=gil` (`gil`), warm `(require 'bytecomp)`, warm `(require 'ert)`, and multi-round `(garbage-collect)` all exited `0`; `4`-worker `std-task-parallel-map` over `8` byte-compiled `(bm-fib 32)` tasks vs sequential `mapcar` produced identical results (`equal=t`, `val=2178309`) in `344.78 ms` vs `1168.30 ms` (**`3.39x` speedup**).
  - `cargo package -p lisp-vm -p lisp-core --allow-dirty --no-verify`: `lisp-vm-0.0.1.crate` (`4,024,140` B, `398` entries, `0` `../` escapes) and `lisp-core-0.0.1.crate` (`1,115,654` B, `110` entries, `0` `../` escapes).

## Controlled Post-Cleanup Verification

- Cleaned `build/` (now empty) and verified `tools/__pycache__` is absent; `dist/macos/Femacs.app` and `dist/lisp` remain intact, and `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty.
- All `6` static gates passed post-cleanup (`run_id=50aa4f9657c443e794fcebef2c10b68b`, `18.162s`): `tools/check-own-voice.py` (`0` failures across `334` ported + `375` own files), `tools/test-repository-bootstrap.py` (`4/4`), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/check-unsafe-boundary.py` (`active_product`: `459` files, `163` `forbid_unsafe_files`, `6,423/6,423` `SAFETY:` blocks, `0` `static_mut`, `0` errors), and `tools/check-crate-boundaries.py` (`ok`, `standalone=3`).

## Retired Temporary Scopes

Retired `3` owned temporary build, packaging, and test-framework scopes totaling `15,538` files, `1,700` dirs, `2` symlinks, and `7,256,828,261` logical bytes (`run_id=0f20b06509754bac8484b588777b4c08f6`, `4.231s`):

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `15,324` | `1,567` | `0` | `7,214,629,368` | `93f3643d829ef3d395a60134cc6be82af869e39d249663960f4340df56fd7583` | Rebuild via `cargo build --release -p lisp-core --target-dir build/cargo` |
| `build/pkg-check` | `9` | `7` | `0` | `15,422,248` | `65ea592e7abd06b27bb53c79661fe41218f56360432b69c8f6e2fe69d9bbcd5b` | Recreated via `cargo package -p lisp-vm -p lisp-core --allow-dirty --no-verify` |
| `build/test-framework` + `build/*` scratch dirs | `205` | `126` | `2` | `26,776,645` | `fa0db1b1c33a9ec84dea4966947c7669e85aa179b36c8e331b9c6085ba9858f0` | Recreated on demand by `fe-test` |
