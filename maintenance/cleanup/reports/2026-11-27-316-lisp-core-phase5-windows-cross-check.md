# CLN316 — Standalone `lisp-core` Phase 5: Windows Pure-Rust OS Layer & Cross-Compile Check (2026-11-27)

## Summary

Completed Phase 5 of the standalone general-purpose Lisp runtime roadmap across `crates/lisp-core` and `crates/lisp-vm`, implementing a pure-Rust Windows OS backend (`std` + `windows-sys`) and LLP64 VM ABI support without C compilers or external native build steps. Verified full cross-compilation (`x86_64-pc-windows-msvc` and `x86_64-pc-windows-gnu`, default features, `--no-default-features --features vm`, and `--tests`) with zero errors and zero warnings in `crates/lisp-core/` and `crates/lisp-std/`, re-verified all macOS host suites, and retired temporary build/cache artifacts (`CLN316`).

> **Platform Caveat**: Windows support is **cross-compile architecture verified ONLY** (`x86_64-pc-windows-msvc` and `x86_64-pc-windows-gnu` `cargo check` + dependency tree audit). No physical Windows host or VM runtime execution has been performed.

## Changes

- **Pure-Rust Windows OS & Transport Layer (`crates/lisp-core`, `tools/unsafe-boundary-manifest.json`, `Cargo.lock`)**:
  - `crates/lisp-core/Cargo.toml`: Added target-gated `[target.'cfg(windows)'.dependencies] windows-sys = "0.61"` (depends solely on `windows-link`, zero C compilation).
  - `crates/lisp-core/src/os_ffi.rs`: Added `#[cfg(windows)]` pure-Rust `windows-sys` backend implementing `install_termination_handlers` (`SetConsoleCtrlHandler`), `current_pgid`, `signal_process`/`signal_process_group` (`OpenProcess`/`TerminateProcess`), `snapshot_windows_processes` (`CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS)` / `Process32FirstW` / `Process32NextW`), RAII `ProcessJob` (`CreateJobObjectW` + `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` + `AssignProcessToJobObject` + `TerminateJobObject` + `CloseHandle`), and `pid_rusage_info` (`K32GetProcessMemoryInfo` + `GetProcessTimes`), with `// SAFETY:` contracts on every `unsafe` block reflected in `tools/unsafe-boundary-manifest.json` (and `maintenance/tooling-inventory.json`).
  - `crates/lisp-core/src/exec.rs` & `exec_tests.rs`: Remains `#![forbid(unsafe_code)]`; added Windows Toolhelp32 process-table snapshot and Job-Object lifecycle supervision in `run_supervise`, clean explicit non-Unix error for `--pty` on Windows, and `#[cfg(unix)]` gating for `/bin/sh`/`ps`/`perl` subprocess tests while keeping pure logic and `mem_tree_reports_nonzero_memory_for_current_pid` cross-platform.
  - `crates/lisp-core/src/rpc.rs`, `cli.rs`, `stdlib_cache.rs`, and `tests/engine_integration.rs`: Remains `#![forbid(unsafe_code)]`; introduced cross-platform `RpcTransport` (`#[cfg(unix)] Unix(UnixStream)`, `Tcp(TcpStream)`) with `--rpc-addr <HOST:PORT>` TCP loopback support (`case_engine_rpc_tcp_loopback` integration test), Windows `LOCALAPPDATA` / `USERPROFILE/.cache` root resolution, and `\` path handling in script auto-dispatch.
- **Windows LLP64 VM Layouts & Gating (`crates/lisp-vm`)**:
  - `crates/lisp-vm/src/{sys/mod.rs,host_runtime.rs}`: Gated Unix-only `client_os`, `UnixStream`, `RawSockaddrStorageHeader`, and `adopt_inherited_rpc_socket` cleanly under `#[cfg(unix)]`.
  - `crates/lisp-vm/src/ported/lib/{getrandom,copy-file-range,mini-gmp,mktime}.rs` & `lib/include/{errno,limits.in,mini-gmp,stat-time,stdint.in,termios,time.in,timespec}.rs`: Gated gnulib `getrandom`/`copy_file_range` shims off on Windows, added UCRT `_errno()` and Windows errno constants, LLP64 `LONG_BIT = 32` / 64-bit `intmax` macros / `long_int = c_longlong`, `tm` layout asserts, and Windows `mp_limb_t`/`mp_size_t`/`mp_bitcnt_t` FFI widths.
  - `crates/lisp-vm/src/ported/src/{bignum,font,process,regex-emacs,sysdep,term,xdisp,xfaces}.rs` & `src/include/{bignum,lisp,sfnt,systty,thread}.rs`: Updated LLP64 `EMACS_INT_WIDTH`, `#[repr(C, align(8))]` `sys_jmp_buf`, `thread_state`, `sfnt_interpreter`, `compile_stack_elt_t`, `speed_struct`, `struct face` padding (`xdisp.rs`, `term.rs`), and Unix-only POSIX `termios`/`rusage` gating.

## Verification Evidence

- **Windows Cross-Compile Architecture Verification (No Physical Windows Host Runtime)**:
  - `cargo check -p lisp-std -p lisp-vm -p lisp-core --target x86_64-pc-windows-msvc` (`exit=0`, `0` warnings in `lisp-core` / `lisp-std`).
  - `cargo check -p lisp-std -p lisp-vm -p lisp-core --target x86_64-pc-windows-gnu` (`exit=0`, `0` warnings in `lisp-core` / `lisp-std`).
  - `cargo check -p lisp-core --no-default-features --features vm --target x86_64-pc-windows-msvc` & `--target x86_64-pc-windows-gnu` (`exit=0`, `0` warnings).
  - `cargo check -p lisp-core --tests --target x86_64-pc-windows-msvc` (`exit=0`, `0` warnings).
  - `cargo tree -p lisp-core --target x86_64-pc-windows-msvc -e normal,build --prefix none` & `lisp-std`: confirmed zero forbidden C-build/`-sys` crates (only `windows-sys` and pure-Rust `libc`).
- **macOS Host Re-Verification (`assistant run`)**:
  - Workspace & feature checks (`run_id=18b33918d71c4a898cc33d4367761fa2`, `exit=0`): `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`, `cargo check -p lisp-core --no-default-features --features vm` (`0` warnings in `lisp-core`/`lisp-std`), and `cargo build --release -p lisp-core`.
  - Unit, integration & release smoke: `cargo test -p lisp-vm --lib` (`12/12`), `cargo test -p lisp-vm --features isolate_vm --lib` (`17/17`), `cargo test -p lisp-std` (`8/8`), `cargo test -p lisp-core` (`27/27` unit + `14/14` `engine_integration` incl. `engine_rpc_tcp_loopback` + `5/5` `std_batteries_integration`), `otool -L target/release/lisp-core` (`/usr/lib/libiconv.2.dylib` & `/usr/lib/libSystem.B.dylib` only), `--version` -> `lisp-core 0.0.1`, `--no-auto-warm eval '(+ 20 22)'` -> `42`.
  - `fe-test` suites: `--suite self` (`9/9`, `8.31s`, `run_id=c1c1a159f23d456eb0eb5b8337265e64` self portion) and `--suite all --no-cache` (`39/39`, `wall=17.248s`, `caseSum=37.701s`, `concurrency=2.19x`, `peakMem=206.23 MiB`, `run_id=f0d5a1c44a12413097c757933fc15467`, `exit=0`).

## Post-Cleanup Verification

- Removed temporary `target/`, `build/*`, and `tools/__pycache__`; confirmed `build/` is empty, `target/` and `tools/__pycache__` are absent, `dist/lisp` is intact, and `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty (`run_id=75a4c1fa1e3545f3aa90a2b8e684a6d9`, `exit=0`).
- All `6` static gates + `git diff --check` passed post-cleanup: `tools/check-own-voice.py` (`0` failures), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-unsafe-boundary.py` (`471` active files, `175` `forbid_unsafe_files`, `6,446/6,446` `SAFETY:` blocks, `0` `static_mut_items`, `0` errors), `tools/check-crate-boundaries.py` (`ok`), and `tools/test-repository-bootstrap.py` (`4/4`).
