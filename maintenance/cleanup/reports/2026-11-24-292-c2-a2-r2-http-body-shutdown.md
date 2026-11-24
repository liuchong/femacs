# CLN292 — Stage C2/A2-R2 HTTP Body Deadline, Non-Blocking Stderr & Cache Retirement (2026-11-24)

## Summary

Closed the remaining `[FEMACS-C2-A2-R1-REVIEW]` HTTP lifecycle blocker in `#![forbid(unsafe_code)]` [`crates/http-host/src/lib.rs`](../../../crates/http-host/src/lib.rs), extended [`tools/test-c2-a2-lisp-app.py`](../../../tools/test-c2-a2-lisp-app.py) (`Scenario D1/D2`) and [`maintenance/tooling-inventory.json`](../../tooling-inventory.json), rebuilt `dist/lisp` (`run_id=6303e1942d5a4ccf9c9e6910451fd829`), passed Phase 2 integration & hygiene gates (`run_id=19a7d42286c04c2f9809d858358cfd5f`) and Phase 3 protective unit tests (`run_id=dfed7739aeaf4e0685e8a573c98db8d4`, `8/8`), retired `4` temporary scopes (`CLN292`), and verified post-cleanup smoke (`run_id=89826609e43f4604ab4befba4479c3ed`).

## C2/A2-R2 Implementation & Verification Receipts

- **Body-Read Deadline & Decoupled Non-Blocking Shutdown (`crates/http-host/src/lib.rs`)**:
  - Moved `deadline = Instant::now() + state.timeout` in `handle_http` ahead of request body reading and added async `read_body_bounded` (`tokio::select!` racing `watch::Receiver<bool>` shutdown notification against `tokio::time::timeout_at(deadline, to_bytes(...))`), returning `413` on oversized payload, `408` (`Connection: close`) on stalled body timeout without occupying or killing the healthy backend, and `503`/`502` (`Connection: close`) immediately upon shutdown.
  - Replaced `spawn_stderr_forwarder` pipes and unbounded `out_fwd.join()`/`err_fwd.join()` with direct fd cloning (`io::stderr().as_fd().try_clone_to_owned()` -> child `stdout`) plus `Stdio::inherit()` child `stderr`, eliminating pipe forwarder threads that could stall shutdown on blocked sinks or open inherited pipes.
  - Decoupled child reaping (`tokio::task::spawn_blocking` inside shutdown future) from bounded HTTP drain (`SHUTDOWN_HTTP_DRAIN_GRACE = 750ms` via `tokio::select!` around `axum::serve(...).with_graceful_shutdown(...)`), keeping RPC socketpair isolation and same-PID HTTP `500` recovery intact.
- **Lifecycle E2E & Protective Unit Tests (`tools/test-c2-a2-lisp-app.py`, `crates/http-host/src/lib.rs`, `maintenance/tooling-inventory.json`)**:
  - Added `Scenario D1` (stalled partial `POST` body returns `408` within `--timeout-ms 450`, never reaches handler, preserves same `BACKEND_PID`) and `Scenario D2` (`--timeout-ms 15000` `SIGTERM` while stalled on partial `POST` body with a completely filled un-drained stderr pipe AND a live sibling `/bin/sleep 5` holder process holding stderr open exits `0` in `< 3s`, reaps backend, skips handler, and releases listening port).
  - Added async/TCP `read_body_bounded_enforces_limit_deadline_and_shutdown` unit test verifying `BodyReadError::TooLarge`/`413`, `408` timeout, and `503` shutdown wakeup with zero jobs reaching `mpsc::Receiver<WorkerJob>`.
- **Controlled Verification Runs (`assistant run`)**:
  - `dist/lisp` rebuild (`run_id=6303e1942d5a4ccf9c9e6910451fd829`, `174.563s`, `exit=0`, `cleanup_complete=true`, peak accounted `2,164,310,016` B).
  - Phase 2 runtime acceptance (`run_id=19a7d42286c04c2f9809d858358cfd5f`, `29.874s`, `exit=0`, `cleanup_complete=true`): `test-c2-a2-lisp-app.py --selftest` (`PASS`), `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`, Scenarios A/B/C/D1/D2), `test-repository-bootstrap.py` (`4/4`), `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,327/6,327` `100.0%` `SAFETY:`, `0` errors), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`, `1.487s`).
  - Phase 3 protective unit tests (`run_id=dfed7739aeaf4e0685e8a573c98db8d4`, `19.925s`, `exit=0`, `cleanup_complete=true`): `cargo test -p http-host` (`8/8`), `check-unsafe-boundary.py` (`0` errors), `check-own-voice.py` (`0` failures).
  - Post-cleanup smoke (`run_id=89826609e43f4604ab4befba4479c3ed`, `11.059s`, `exit=0`, `cleanup_complete=true`): `LISP=42`, `HTTP=http-host 0.1.0 (femacs 32.0.50)`, `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`).

## Retired Temporary Scopes

Retired `4` owned temporary build/cache/pin scopes totaling `4,611` files, `1,162` directories, `8` symlinks, `895,652,948` logical bytes, normalized digest `6f5c8f167882402a8cd9f2169bdec6a67321f7fdbb5ed2325db53846e618ffe1`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `427M` | `3,697` | `658` | `0` | `545,428,162` | Root Cargo target cache (`cargo test` / `xtask`) |
| `build/cargo-tools` | `16M` | `51` | `29` | `0` | `15,842,911` | `xtask` host tool target cache |
| `build/lisp` | `327M` | `858` | `468` | `6` | `333,774,590` | Recreated by `xtask build-lisp --out dist/lisp` |
| `build/treesit-0.27.1` | `604K` | `5` | `7` | `2` | `607,285` | Reinstall via `python3 tools/toolchain-pins.py install` |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`):
  - `bin/femacs-lisp`: `5,053,696` B, SHA-256 `c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,325,840` B, SHA-256 `39b97e3ee8f7808719db8e846d1a45f4c45f14003e080171b486687d57054071`
  - `bin/http-host`: `2,147,280` B, SHA-256 `fb6199f2d01b5b0fa203ec2b84ef48aef535d08c5971970b90767a0a75bacc6f` (`codesign --verify --deep --strict` `exit 0`)
  - `dist/lisp/manifest.json` bytes/SHA-256 verified matching disk files.
- Unchanged retained sets: `build/terminal` (`45M`), `dist/macos` (`235M`, parity-matched with `/Applications/Fe.app`), `build/generation` (`13M`), and `build/manuals` (`1.4G`) (`build` `1.4G`, `dist` `445M`).
