# CLN297 — Stage C4/A4 Headless Exit Unwind, Modern Local `.eli` Acceptance & Cache Retirement (2026-11-24)

## Summary

Completed Phase II Stage `C4/A4` (in-process modern local `.eli` extension subset and ERT parity on `femacs-lisp`) across commits `93ecb24e` (VM/host uncatchable exit unwind + `A1` scenario + `C4/A4` harness) and `a26152e4` (`CODE_SHA`: ignore `/dist/.{macos,lisp}.{stage,previous}-*` staging dirs during `bundle::publish`), rebuilt clean `a26152e4`-bound `dist/lisp`, verified dual-host `C4/A4`, `C3/A3`, and `C2/A2` acceptance cleanly against isolated `git archive HEAD` snapshots of the downstream compiler repository (`db9fd8ae3656`), retired `11` temporary scopes (`CLN297`), and passed post-cleanup smoke. Scope boundary: `C4/A4` verified `dist/lisp` (`femacs-lisp`, `femacs-lisp.pdmp`, `http-host`), `femacs_host_unit_tests`, `a1_host_acceptance`, `xtask`, and `http-host`; TTY (`build/terminal`) and Cocoa (`dist/macos`) interactive verification belong to `C5/A5`.

## C4/A4 Changes & Controlled Verification Receipts

- **Uncatchable Headless `(kill-emacs)` Unwind (`93ecb24e`, `crates/femacs-vm/src/{nonlocal,host_runtime}.rs`, `src/{eval,bytecode,comp}.rs`, `crates/femacs-host/tests/a1_host_acceptance.rs`, `tools/test-c1-a1-host.py`)**:
  - Replaced error-condition signaling in `intercept_headless_kill_emacs` with dedicated `VmHostExitUnwind(i32)` (`raise_host_exit`, caught strictly at `with_host_condition_catch_ctx` via `catch_vm_nonlocal_or_host_exit` while passing untouched through `catch_vm_nonlocal`), preventing `ert-run-tests-batch-and-exit`'s outer `unwind-protect` / `condition-case` cleanup from swallowing exit codes and overwriting them with exit `2`.
  - Added `HOST_EXIT_UNWINDING`, `HOST_KILL_HOOK_RUNNING` (`HostKillHookGuard`) to guard recursive `(kill-emacs)` inside `kill-emacs-hook`, short-circuit guards in `prog_ignore` (`src/eval.rs`) and `bcall0` (`src/{bytecode,comp}.rs`), `restore_vm_state_to_host_handler` (`src/eval.rs`), and `!is_host_exit_unwinding()` skip in `UnbindGuard::drop` so `specpdl` unwinds top-to-bottom per handler without underflow.
  - Added `host-exit-unwind` scenario in `crates/femacs-host/tests/a1_host_acceptance.rs` covering `unwind-protect` bypass, `condition-case t` bypass, re-entrant `kill-emacs-hook`, `eval_with_args` + nested `condition-case`, and post-unwind reuse, wired into `tools/test-c1-a1-host.py`.
- **Clean `bundle::publish` Staging Ignore & Dual-Host `C4/A4` Harness (`a26152e4`, `.gitignore`, `tools/test-c4-a4-modern-local.py`, `maintenance/tooling-inventory.json`)**:
  - Added `/dist/.{macos,lisp}.{stage,previous}-*` to `.gitignore` so transient `bundle::publish` directories do not trigger `dirty-<head>` during `resolve_source_commit`.
  - Built `tools/test-c4-a4-modern-local.py` (`31,963` B, SHA-256 `6419cd611d3fe2516f2be9130588ff534140d76954187e7ee80f0077704b022d`) testing ERT/`kill-emacs` exit contract, `84` pure-Lisp ERT tests across `10` files (`+ 2` excluded out-of-scope `eliscript-0-1-0-t16-*` parity failures), same-source modern `.eli` local evaluation parity, and `14` explicit out-of-scope / static rejections with `PATH=/usr/bin:/bin` and `ELISCRIPT_JS_RUNTIME=/nonexistent/no-js`.
- **Controlled Build & Verification Runs (`assistant run`)**:
  - Pre-commit incremental build & test (`run_id=3de4c4c04456443099bda423aebe7ca5`, `69.912s`, `exit=0`; `run_id=f0c7b56608164152a8af8a3c0b689faf`, `16.64s`, `exit=0`): `femacs_host_unit_tests` (`10/10`), `a1_host_acceptance` (`host-exit-unwind`, `safe-domain-protected`, `cold-minimal-lifecycle`, `headless-diagnostics`), `femacs-lisp` smoke (`7`/`9`/`11`/ERT `0`), `xtask` (`119/119`), `http-host` (`9/9`); static gates `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,332/6,332` `SAFETY:`, `0` `static_mut_items`, `0` errors), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`), and `--selftest` (`PASS`).
  - Clean `a26152e4`-bound `dist/lisp` rebuild (`run_id=f46d75a8d9a64cc19b684f21aca91156`, `193.112s`, `exit=0`, `cleanup_complete=true`, peak accounted `2,061,977,904` B).
  - Full dual-host acceptance (`run_id=25d1f1c9d91847b0a0d450d4fc80bd4e`, `206.941s`, `exit=0`, `cleanup_complete=true`, peak accounted `532,939,160` B):
    - `femacs-lisp-cache.py verify --expect-commit a26152e4...` -> `verified`, `package_sha256=3c86eb8f555cde1b3fd82c20f62c86a9889e5179d2fa3165c0389694b174822a`.
    - `test-c4-a4-modern-local.py` -> `PASS` (`84` ERT tests across `10` files + `2` `t16-*` parity failures, modern `.eli` factorial `720` & all `11` suite assertions matching the reference host, `14/14` rejections matched).
    - `test-c3-a3-seed-host.py` -> `PASS` (Seed `25` files / `1,431,679` B / `0` diffs, Stage 0 `131` files / `15,118,877` B / `0` diffs, cache `rebuilt`/`hit` `3c86eb8f…`).
    - `test-c2-a2-lisp-app.py --package dist/lisp` -> `PASS`.
  - Post-cleanup smoke (`run_id=dffed7501ebb47d3b84c177c27b039d6`, `17.279s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp-cache.py verify` (`verified`), strict `codesign` (`exit 0`), `kill_rc1=7` / `kill_rc2=9` / `ert_rc3=0`, `A2 --package dist/lisp` `PASS`, `--selftest` `PASS`, `33/33` modernization, `4/4` bootstrap, `0` own-voice failures.

## Retired Temporary Scopes

Retired `11` owned temporary Cargo target, build-tool, standalone Lisp staging, pinned Tree-sitter SDK, acceptance scratch, and empty `xtask` unit test scratch scopes totaling `25,656` files, `1,978` directories, `20` symlinks, `1,768,772,345` logical bytes, normalized digest `132c4621b74ea3486fb281107d24cd3abdbacf3b20b3035d23ad429fce1d7624`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `459M` | `3,358` | `522` | `0` | `563,732,537` | Root Cargo target cache (`xtask`, `http-host`) |
| `build/cargo-tools` | `16M` | `53` | `30` | `0` | `15,843,171` | Recreated by `xtask build-lisp` |
| `build/lisp` | `328M` | `858` | `469` | `6` | `334,086,241` | Standalone Lisp package build staging & test links |
| `build/treesit-0.27.1` | `604K` | `5` | `8` | `2` | `607,285` | Pinned `tree-sitter 0.27.1` prefix via `tools/toolchain-pins.py` |
| `build/acceptance` | `861M` | `21,382` | `943` | `12` | `854,503,111` | Clean-snapshot `C4/A4` and `C3/A3` acceptance outputs |
| `build/android-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/bundle-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/manuals-review` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/package-contracts` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/probes-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/tooling-tests` | `0` | `0` | `1` | `0` | `0` | Empty `xtask` unit test scratch dir |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`, bound to `source_commit=a26152e41ad72ffee9e7a99b32b77984d32e7dc7`, `package_sha256=3c86eb8f555cde1b3fd82c20f62c86a9889e5179d2fa3165c0389694b174822a`):
  - `bin/femacs-lisp`: `5,093,248` B, SHA-256 `923aa0e0ed579935d49242be0c9145141ff7b04302bf9e1a4ce8ffa2c5781126` (`codesign --verify --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,325,904` B, SHA-256 `51656e2355da718e1338f9b33ea7614a6f04d3a0488ba0e46416b2c0bea7a442`
  - `bin/http-host`: `2,145,728` B, SHA-256 `31f438b7ca7b102dff78545a03a128d6305f13732f7714b2e1ad3035013a5c00` (`codesign --verify --strict` `exit 0`)
- Unchanged retained sets: `build/terminal` (`45M`), `dist/macos` (`235M`, parity-matched with `/Applications/Fe.app`; interactive `C5/A5` verification pending), `build/generation` (`16M`), and `build/manuals` (`1.4G`) (`build` `1.4G`, `dist` `445M`).
