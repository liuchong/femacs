# CLN299 — Stage C4/A4-R2 System I/O-Wait Host-Exit Wakeup & `dist/lisp` Rebuild (2026-11-24)

## Summary

Closed the remaining `[FEMACS-C4-A4-R1-REVIEW]` blocking gap on `9ef6e40f` (`CODE_SHA`), eliminating lost wakeups when a background Lisp thread calls `kill-emacs` while the main thread is blocked in system I/O wait (`thread_select` -> `pselect`). Initialized the child-signal/wakeup self-pipe under the VM global lock on blocking main-thread headless-host `thread_select` calls (and `wait_reading_process_output`), injected the pipe read descriptor into the temporary `fd_set` read mask, posted a non-blocking wakeup byte in `record_pending_host_exit`, preserved unread wakeup bytes in `child_signal_read` while a host exit is pending, and drained queued wakeup bytes when consuming or resetting `PENDING_HOST_EXIT`. Rebuilt clean `9ef6e40f`-bound `dist/lisp` (`210M`), passed `11/11` `C4-R1` ABI-exit & I/O-wait cases plus `A2 --package dist/lisp` smoke (while reusing previously reviewed `C4/A4` `.eli` local subset and Seed/Stage 0 evidence from `a26152e4`; `.eln` coverage remains state-machine verification via `verify_comp_native_host_exit_state`, not full JIT `.eln` execution), retired `14` temporary scopes (`CLN299`), and passed post-cleanup smoke.

## C4-R2 Changes & Controlled Verification Receipts

- **I/O-Wait Self-Pipe Wakeup & Pending Host-Exit Consumption (`9ef6e40f`, `src/process.rs`, `crates/femacs-vm/src/host_runtime.rs`, `src/thread.rs`)**:
  - `src/process.rs`: ensured `child_signal_init ()` initializes `child_signal_{read,write}_fd` when `is_headless_host_active ()` enters `wait_reading_process_output` with an unopened pipe, guarded `child_signal_read` with `has_pending_host_exit ()` so unread wakeup bytes stay available until main-thread consumption, and added `ensure_host_exit_io_wakeup_pipe`, `notify_host_exit_io_wakeup`, and `drain_host_exit_io_wakeup`.
  - `crates/femacs-vm/src/host_runtime.rs`: updated `record_pending_host_exit` to write a non-blocking wakeup byte via `notify_host_exit_io_wakeup`, and updated `take_pending_host_exit` and `shutdown_headless_host` to drain queued self-pipe wakeup bytes via `drain_host_exit_io_wakeup` before returning control or clearing host state.
  - `src/thread.rs`: updated `thread_select` to run `propagate_pending_host_exit_if_main ()` both before releasing the global lock and immediately after `really_call_select` returns, initialize and inject `wake_fd` into a local `fd_set` copy (raising `sa.max_fds` if needed) when blocking on the main headless-host thread, and strip/decrement the injected `wake_fd` from `sa.result` before copying back `*rfds`.
- **Verification & Acceptance (`crates/femacs-host/tests/a1_host_acceptance.rs`, `tools/test-c4-r1-abi-exit.py`, `maintenance/tooling-inventory.json`)**:
  - Static gates (`exit=0`): `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,340/6,340` `100.0%` `SAFETY:`, `0` `static_mut_items`, `0` errors), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`).
  - Pre-commit Rust & `A1` suite (`run_id=1dc0c5b1016b4c60ab987d4f0421ed66`, `105.116s`, `exit=0`, `cleanup_complete=true`): `xtask` (`119/119`), `http-host` (`9/9`), `femacs_host_unit_tests` (`10/10`), and `a1_host_acceptance` (`host-exit-unwind` normal + `FEMACS_GC_SKIP_STACK_SCAN=1`, verifying subthread `(sleep-for 0.15) (kill-emacs 47)` interrupts `(sleep-for 2.5)` in `< 1200ms` followed by `(+ 20 27) -> 47`, `(length (all-threads)) -> 1`, `(progn (sleep-for 0.05) 'awake) -> awake`, and state-machine `verify_comp_native_host_exit_state(41)` — not full JIT `.eln` execution — plus `cold-minimal-lifecycle`).
  - Clean `9ef6e40f`-bound `dist/lisp` rebuild (`run_id=3c707768504e4e04819ae0ad72a1558a`, `209.656s`, `exit=0`, `cleanup_complete=true`) & `femacs-lisp-cache.py verify --expect-commit 9ef6e40f...` (`target=aarch64-apple-darwin`, `sha256=490340fd8efa6de351c8097fb8ee7afd2a30ad74e0e7f3c0dd90e474b26159af`).
  - `C4-R2` targeted ABI-exit & I/O-wait suite (`tools/test-c4-r1-abi-exit.py`, `run_id=a7ddfdbc500847548166a99a91e80363`, `5.15s`, `exit=0`, `cleanup_complete=true`): `11/11` cases passed on `dist/lisp/bin/femacs-lisp` (Cases 1-9 exit codes `7`, `13`, `17`, `11`, `19`, `29`, `27`, `23`, `31`; Case 10 `lisp_subthread_wakes_main_sleep_for` `rc=47` in `0.1811s`; Case 11 `lisp_subthread_wakes_main_indefinite_io_wait` via `make-pipe-process` + `accept-process-output` `rc=53` in `0.1874s`).
  - `A2` smoke on delivered `dist/lisp` (`run_id=6cb2c548be7b4340bc460295bc82dd2d`, `9.002s`, `exit=0`, `cleanup_complete=true`): `test-c2-a2-lisp-app.py --package dist/lisp --terminal-dump build/terminal/port/src/emacs.pdmp` `PASS`.
  - Post-cleanup smoke (`run_id=b10f53c3a980406b92557e3366a37c9e`, `9.246s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp-cache.py verify`, strict `codesign` (`exit 0`), `SMOKE_KILL_MAIN=7` (`0.0298s`), `SMOKE_KILL_SUBTHREAD=19` (`0.026s`), `SMOKE_IO_SLEEP_FOR=47` (`0.1803s`), `SMOKE_IO_PIPE_WAIT=53` (`0.1801s`), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`).

## Retired Temporary Scopes

Retired `14` owned temporary Cargo target, build-tool, standalone Lisp staging, pinned Tree-sitter SDK, acceptance scratch, empty `xtask` unit test, and terminal build-cache/test-binary scopes totaling `4,565` files, `1,283` directories, `10` symlinks, `1,144,284,379` logical bytes, normalized digest `61c89ce4383e25941066c9046d549054bf288845229ec756d405b68339a08682`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `460M` | `3,358` | `522` | `0` | `563,732,518` | Root Cargo target cache |
| `build/cargo-tools` | `16M` | `51` | `30` | `0` | `15,842,915` | Recreated by `xtask build-lisp` |
| `build/lisp` | `328M` | `858` | `469` | `6` | `334,315,087` | Standalone Lisp build staging |
| `build/treesit-0.27.1` | `604K` | `5` | `8` | `2` | `607,238` | Pinned SDK via `tools/toolchain-pins.py` |
| `build/acceptance` | `92K` | `23` | `103` | `2` | `2,277` | `C4-R2` ABI-exit acceptance scratch |
| `build/android-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/bundle-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/manuals-review` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/package-contracts` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/probes-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/tooling-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/terminal/cargo` | `225M` | `268` | `145` | `0` | `223,791,368` | Terminal host-test Cargo cache |
| `build/terminal/port/src/a1_host_acceptance` | `4.7M` | `1` | `0` | `0` | `4,892,160` | Relinked via `xtask link-host-test` |
| `build/terminal/port/src/femacs_host_unit_tests` | `1.1M` | `1` | `0` | `0` | `1,100,816` | Relinked via `xtask link-host-test` |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`, bound to `source_commit=9ef6e40f2e749980c18f98a0aba74884037a4a97`, `package_sha256=490340fd8efa6de351c8097fb8ee7afd2a30ad74e0e7f3c0dd90e474b26159af`):
  - `manifest.json`: `2,174` B, SHA-256 `1d69d494102ce41c99c4e891def344d6e250219f75fdcc90660d33b5c9b06416`
  - `bin/femacs-lisp`: `5,110,448` B, SHA-256 `82fc21c8879c282fb81f2b679102689fb5dc03505d9aee4538705a7879b10a8d` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,326,520` B, SHA-256 `4ba0581884cadd2b3e2f8e989eaea8546612e36eb1f062c12ffad1a131c7ba0d`
  - `bin/http-host`: `2,145,728` B, SHA-256 `31f438b7ca7b102dff78545a03a128d6305f13732f7714b2e1ad3035013a5c00` (`codesign --verify --deep --strict` `exit 0`)
- Retained supporting sets: `build/terminal` (`47M`), `dist/macos` (`235M`, parity-matched with installed `.app`; interactive `C5/A5` verification pending), `build/generation` (`17M`), and `build/manuals` (`1.4G`).
