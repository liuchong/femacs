# CLN298 — Stage C4/A4-R1 ABI-Safe Host-Exit Closure & `dist/lisp` Rebuild (2026-11-24)

## Summary

Closed the single `[FEMACS-C4-A4-REVIEW]` blocking mechanism on `e95227f7` (`CODE_SHA`), eliminating every `extern "C"` / OS thread / `.eln` helper unwind crossing for `VmHostExitUnwind(code)`, extended `A1` `host-exit-unwind` and added the 9-case pure public-ABI C dynamic module + Lisp subthread verification harness (`tools/test-c4-r1-abi-exit.py`), rebuilt clean `e95227f7`-bound `dist/lisp` (`210M`), passed `9/9` `C4-R1` ABI-exit cases plus `A2 --package dist/lisp` smoke (while preserving previously reviewed `C4/A4` `.eli` local subset and Seed/Stage 0 evidence from `a26152e4`), retired `21` temporary scopes (`CLN298`), and passed post-cleanup smoke.

## C4-R1 Changes & Controlled Verification Receipts

- **ABI-Safe Host-Exit Closure (`e95227f7`, `crates/femacs-vm/src/{nonlocal,host_runtime}.rs`, `src/{emacs-module,thread,eval,keyboard,comp}.rs`)**:
  - `src/emacs-module.rs`: added `host_exit_pending` and `host_exit_code` to opaque `emacs_env_private`, converted `module_with_nonlocal_catch` to `catch_vm_nonlocal_or_host_exit`, restored VM state to `internal_handler` and cleared `HOST_EXIT_UNWINDING` before returning `emacs_funcall_exit_signal` cleanly to C, guarded `module_non_local_exit_clear` so C modules cannot swallow host exit, updated `module_should_quit`, and cleanly unbound/finalized `env` before resuming host unwind in `Fmodule_load` and `funcall_module`.
  - `src/thread.rs`, `src/eval.rs`, `src/keyboard.rs`: updated `run_thread` to intercept `HostExit(code)` via `catch_vm_nonlocal_or_host_exit`, call `record_pending_host_exit(code)` (preserving first-writer exit code via `PENDING_HOST_EXIT` and arming `pending_signals`), restore handlers to `handlerlist_sentinel!()`, clear `HOST_EXIT_UNWINDING`, wake blocked main `wait_condvar`, and finish normal thread teardown and lock release; added deadlock-safe wait breaks in `lisp_mutex_lock_for_thread`, `condition_wait_callback`, and `thread_join_callback`; propagated pending main-thread host exit in `Fthread_join`, `Fthread_yield`, `Fmutex_lock`, `Fcondition_wait`, `thread_select`, `probably_quit`, and `process_pending_signals`.
  - `crates/femacs-vm/src/nonlocal.rs` & `src/comp.rs`: extended `NativePendingExit` with `NativePendingKind::HostExit(i32)` across all four `.eln` helpers (`comp_run_void_helper`, `comp_run_lisp_helper`, `comp_push_handler`, `comp_helper_GET_SYMBOL_WITH_POSITION`), prevented secondary `set_comp_native_exit` or `.eln` `consume_comp_native_exit_for_handler` from overwriting or swallowing `HostExit(code)`, and resumed host unwind at `propagate_comp_native_exit_slow`.
  - `crates/femacs-vm/src/host_runtime.rs`: added `format_error_payload_message` `HostExit` guard, `with_host_condition_catch_ctx` pending-exit drain, `shutdown_headless_host` reset, and `verify_comp_native_host_exit_state`.
- **Verification & Acceptance (`crates/femacs-host/{src/lib.rs,tests/a1_host_acceptance.rs}`, `tools/test-c4-r1-abi-exit.py`, `maintenance/tooling-inventory.json`)**:
  - Static gates (`exit=0`): `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,336/6,336` `100.0%` `SAFETY:`, `0` `static_mut_items`, `0` errors), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`).
  - Pre-commit Rust & `A1` suite (`run_id=a7bbf7fad4354628bd671d63a5d69204`, `114.931s`, `exit=0`, `cleanup_complete=true`): `xtask` (`119/119`), `http-host` (`9/9`), `femacs_host_unit_tests` (`10/10`), and `a1_host_acceptance` (`host-exit-unwind` normal + `FEMACS_GC_SKIP_STACK_SCAN=1` covering subthread exit `17` + mutex/condvar subthread exit `29` + `verify_comp_native_host_exit_state(41)`, plus `cold-minimal-lifecycle`).
  - Clean `e95227f7`-bound `dist/lisp` rebuild (`run_id=029020d79f16433489a23f5ebd358f76`, `181.95s`, `exit=0`, `cleanup_complete=true`) & `femacs-lisp-cache.py verify --expect-commit e95227f7...` (`verified`, `package_sha256=2ce0ec0bc71a01aae51aca6c4aaddca98dfaf80c83426dff3bc35624d7a56c39`).
  - `C4-R1` targeted ABI-exit suite (`run_id=bec47de8a08c437895ae100d013f53ac`, `5.418s`, `exit=0`, `cleanup_complete=true`): `9/9` real C dynamic module & Lisp subthread cases passed on `dist/lisp/bin/femacs-lisp` (exit codes `7`, `13`, `17`, `11`, `19`, `29`, `27`, `23`, `31`, confirming C cleanup markers and skipped Lisp handlers).
  - `A2` smoke on delivered `dist/lisp` (`run_id=14241a00d8b143f9affbdaa8695f014b`, `13.765s`, `exit=0`, `cleanup_complete=true`): `test-c2-a2-lisp-app.py --package dist/lisp --terminal-dump build/terminal/port/src/emacs.pdmp` `PASS`.
  - Post-cleanup smoke (`run_id=6b944eb97cd548c08282368027fb7db2`, `10.246s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp-cache.py verify`, strict `codesign` (`exit 0`), `SMOKE_KILL_MAIN=7`, `SMOKE_KILL_SUBTHREAD=19`, `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`).

## Retired Temporary Scopes

Retired `21` owned temporary Cargo target, build-tool, standalone Lisp staging, pinned Tree-sitter SDK, acceptance scratch, empty `xtask` unit test, terminal build-cache/test-binary, and Python bytecode cache scopes totaling `4,683` files, `1,605` directories, `16` symlinks, `1,155,097,871` logical bytes, normalized digest `8e7742b3b63c047cc2807258423b4265633ba6c36bda1146db4d7d68076c140a`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `460M` | `3,358` | `522` | `0` | `563,732,463` | Root Cargo target cache |
| `build/cargo-tools` | `16M` | `51` | `30` | `0` | `15,842,913` | Recreated by `xtask build-lisp` |
| `build/lisp` | `336M` | `858` | `469` | `6` | `334,178,998` | Standalone Lisp build staging |
| `build/treesit-0.27.1` | `604K` | `5` | `8` | `2` | `607,238` | Pinned SDK via `tools/toolchain-pins.py` |
| `build/acceptance` | `356K` | `89` | `397` | `8` | `3,286` | `C4-R1` & contract acceptance scratch |
| `build/android-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/bundle-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/manuals-review` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/package-contracts` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/probes-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/tooling-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/terminal/cargo` | `221M` | `268` | `145` | `0` | `223,605,607` | Terminal host-test Cargo cache |
| `build/terminal/config` | `44K` | `6` | `1` | `0` | `34,840` | Recreated by `xtask` terminal prepare |
| `build/terminal/image` | `1.3M` | `3` | `3` | `0` | `1,270,503` | Recreated by `xtask` terminal prepare |
| `build/terminal/image-staging` | `1.3M` | `2` | `1` | `0` | `1,269,940` | Recreated by `xtask` terminal prepare |
| `build/terminal/lcms` | `364K` | `5` | `4` | `0` | `360,608` | Recreated by `xtask` terminal prepare |
| `build/terminal/tls` | `7.6M` | `26` | `11` | `0` | `7,878,424` | Recreated by `xtask` terminal prepare |
| `build/terminal/treesit` | `316K` | `6` | `7` | `0` | `307,980` | Recreated by `xtask` terminal prepare |
| `build/terminal/port/src/a1_host_acceptance` | `4.7M` | `1` | `0` | `0` | `4,858,112` | Relinked via `xtask link-host-test` |
| `build/terminal/port/src/femacs_host_unit_tests` | `1.1M` | `1` | `0` | `0` | `1,100,816` | Relinked via `xtask link-host-test` |
| `tools/__pycache__` | `52K` | `4` | `1` | `0` | `46,143` | Python bytecode cache |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`, bound to `source_commit=e95227f79d540ce84f1afd371c9d447a4b01af1c`, `package_sha256=2ce0ec0bc71a01aae51aca6c4aaddca98dfaf80c83426dff3bc35624d7a56c39`):
  - `manifest.json`: `2,174` B, SHA-256 `a256af1b2328541fb4b2fcdc38175ea5b425e464c4e488fa48eac819e2b9b6b1`
  - `bin/femacs-lisp`: `5,094,000` B, SHA-256 `8625038ba6d2b1accaaa8d150796576a45b8be76f8d4e1eb820d4fb682d1285a` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,325,912` B, SHA-256 `44fc4c04549b371082dedc829664142a127813200cda7012673a604350b909d6`
  - `bin/http-host`: `2,145,728` B, SHA-256 `31f438b7ca7b102dff78545a03a128d6305f13732f7714b2e1ad3035013a5c00` (`codesign --verify --deep --strict` `exit 0`)
- Retained supporting sets: `build/terminal` (`48M`), `dist/macos` (`235M`, parity-matched with installed `.app`; interactive `C5/A5` verification pending), `build/generation` (`16M`), and `build/manuals` (`1.4G`).
