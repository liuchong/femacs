# CLN315 — Standalone `lisp-core` Phase 4: Linux (`aarch64` / `x86_64`) Support & Ephemeral Lima Verification (2026-11-27)

## Summary

Completed Phase 4 of the standalone general-purpose Lisp runtime roadmap across `crates/lisp-vm` and `crates/lisp-core`, bringing full Linux (`aarch64` real-VM verified, `x86_64` cross-checked) portability without C compilers or third-party native dependencies. Spun up a dedicated ephemeral Lima VM (`femacs-linux-verify`, Debian 12 `6.1.0-54-cloud-arm64`), fixed five Linux runtime/ABI bugs uncovered by real-machine execution, passed every unit, integration, release-binary, PTY REPL, `.pdmp` warm-cache, `NoGil` speedup, and `init`/`pkg` workflow check on Linux, tore down the temporary VM (`intent-backend` left `Stopped` and untouched), re-verified the macOS host (`cargo test`, `--workspace`, `femacs-host`, `--release`, `fe-test`), and cleaned local caches (`CLN315`).

## Changes & Fixes

- **Linux OS & glibc ABI layer (`crates/lisp-vm`, `crates/lisp-core`)**:
  - Updated Linux `aarch64`/`x86_64` struct layouts, constants, and symbol bindings across `crates/lisp-vm/src/ported/lib/include/*.rs`, `lib/{boot-time,nproc}.rs`, `src/{callproc,dired,keyboard,pdumper,process,sysdep,timefns}.rs`, `src/include/{lisp,sfnt,systty,thread}.rs`, and `src/sys/client_os.rs` (`__sigsetjmp`, `posix_spawn_file_actions_addchdir_np`, `renameat2`, `jmp_buf`/`sigjmp_buf`, `stat`, `dirent`, `termios`, `utmpx`, `sigaction`, `sockaddr_un`, `addrinfo`).
  - Added Linux `/proc/{pid}/status` + `/proc/{pid}/stat` memory/start-time accounting (`crates/lisp-core/src/os_ffi.rs`), `script -qefc <cmd> /dev/null` PTY supervision (`crates/lisp-core/src/exec.rs`), and `#[cfg(feature = "repl-tty")]` gating (`crates/lisp-core/src/repl.rs`) for `0` warnings under `--no-default-features --features vm`.
- **Five real-Linux defects diagnosed and resolved**:
  1. *ELF `.bss` ZST `IsolateContext` collision* (`crates/lisp-vm/src/sys/isolate.rs`): Zero-sized `static globals: VmGlobal<emacs_globals>` shared its `.bss` address with adjacent `intervals_consed`, poisoning `self.slots` (`existing=0 < requested=8`). Guarded `size == 0` in `get_or_alloc`, `alloc_and_insert`, and `resolve_fwd_ptr` to return aligned non-null dangling pointers without registering zero-byte slots, and added `zst_global_does_not_poison_adjacent_nonzero_slot`.
  2. *glibc `realloc(p, 0)` returning `NULL`* (`crates/lisp-vm/src/ported/lib/include/stdlib.rs`, `scratch_buffer_grow_preserve.rs`): Enabled `rpl_realloc` (`size == 0 -> 1`) for `target_os = "linux"` so `shrink_regexp_cache` does not trigger `memory_full(0)`.
  3. *`getrandom` / `copy_file_range` symbol shadowing* (`crates/lisp-vm/src/ported/lib/{getrandom,copy-file-range}.rs`): Gated gnulib shims off on `target_os = "linux"` so Rust `std::sys::random::linux` and `lisp-vm` bind directly to glibc's native syscalls instead of closing a shared `/dev/urandom` descriptor across isolates (`EBADF`, `errno=9`).
  4. *Darwin vs Linux signal/wait/fcntl constants* (`crates/lisp-vm/src/ported/src/{sysdep,process}.rs`): Replaced Darwin `SA_RESTART` (`0x0002`, which on Linux is `SA_NOCLDWAIT` causing `wait()` `ECHILD`), `SA_ONSTACK`, `SA_SIGINFO`, `siginfo_t`, `stack_t`, `FASYNC`, and `WCONTINUED` with Linux definitions.
  5. *Multi-isolate `SIGCHLD` & `pthread_kill(0)` safety* (`crates/lisp-vm/src/ported/src/{process,sysdep}.rs`): Skipped process-wide `catch_child_signal()` when `isolate_vm` is active and guarded `deliver_process_signal` / `deliver_thread_signal` against inactive isolates or `pthread_t == 0`.

## Verification Evidence

- **Ephemeral Linux VM (`Linux lima-femacs-linux-verify 6.1.0-54-cloud-arm64 #1 SMP Debian 6.1.190-1 aarch64 GNU/Linux`, later destroyed)**:
  - `cargo check -p lisp-vm -p lisp-core -p lisp-std --target x86_64-unknown-linux-gnu`: passed.
  - `cargo check -p lisp-core --no-default-features --features vm`: passed (`0` warnings in `crates/lisp-core/`).
  - `cargo test -p lisp-std`: `8 passed; 0 failed`.
  - `cargo test -p lisp-vm --lib`: `12 passed; 0 failed`; `--features isolate_vm --lib`: `17 passed; 0 failed`.
  - `cargo test -p lisp-core`: `27 passed; 0 failed` unit tests, `engine_integration` `13/13` cases passed, `std_batteries_integration` `5/5` suites passed.
  - `cargo build --release -p lisp-core` + `ldd target/release/lisp-core`: dynamically linked only to `linux-vdso.so.1`, `libgcc_s.so.1`, `libm.so.6`, `libc.so.6`, `/lib/ld-linux-aarch64.so.1`.
  - Stage 7 standalone release acceptance: `--version` (`lisp-core 0.0.1`), silent cold boot + background auto-warm `.pdmp` (`9,320,664` B, `13638.8ms -> 33.7ms`; prior run `5035.9ms -> 22.8ms`), real PTY REPL (`history.txt`, multiline, `Ctrl-C`, `Ctrl-D`), 4-worker `NoGil` parallel speedup (`0.4705s -> 0.1815s`, `2.59x`), and `init`/`pkg add --path`/`run`/`test` CLI flow.
- **macOS Host Re-verification (`assistant run`)**:
  - Build/test pass (`run_id=04ef0dd6cbfa402687df625a1a2cab1c`, `exit=0`): `lisp-vm --lib` (`12/12`), `lisp-vm --features isolate_vm --lib` (`17/17`), `lisp-std` (`8/8`), `lisp-core` (`27/27` unit + `13/13` `engine_integration` + `5/5` `std_batteries_integration`), `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`.
  - Release binary (`run_id=b011f16167314dda9d50af293b6a4870`, `exit=0`): `otool -L target/release/lisp-core` links only `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`.
  - Runtime test suite (`run_id=c684b46830e54ff3aa854ab2acbd6933`, `exit=0`): `fe-test --suite self` (`9/9`, `7.37s`) and `fe-test --suite all --no-cache` (`39/39`, `wall=15.354s`, `peakMem=198.92 MiB`).

## Controlled Post-Cleanup Verification & Teardown

- Torn down ephemeral Lima VM (`limactl stop femacs-linux-verify && limactl delete -f femacs-linux-verify`); `limactl list` confirms only untouched `intent-backend` (`Stopped`) remains.
- Cleaned local `target/`, `build/*`, and `tools/__pycache__`; `build/` is empty, `target/` and `tools/__pycache__` are absent; `dist/macos/Femacs.app` and `dist/lisp` remain intact with empty `diff -rq dist/macos/Femacs.app /Applications/Fe.app`.
- All `6` static gates + `git diff --check` passed post-cleanup: `tools/check-own-voice.py` (`0` failures), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-unsafe-boundary.py` (`471` active files, `175` `forbid_unsafe_files`, `6,437/6,437` `SAFETY:` blocks, `0` `static_mut`, `0` errors), `tools/check-crate-boundaries.py` (`ok`), and `tools/test-repository-bootstrap.py` (`4/4`).
