# CLN293 — Stage C2/A2-R3 Non-Blocking RPC Error Reply, Genuine Blocking Pipe Coverage & Cache Retirement (2026-11-24)

## Summary

Closed the sole remaining `[FEMACS-C2-A2-R2-REVIEW]` finding in `#![forbid(unsafe_code)]` [`crates/http-host/src/lib.rs`](../../../crates/http-host/src/lib.rs), updated [`tools/test-c2-a2-lisp-app.py`](../../../tools/test-c2-a2-lisp-app.py) (`Scenario D2/D3`) and [`maintenance/tooling-inventory.json`](../../tooling-inventory.json), rebuilt `dist/lisp` (`run_id=032f31e33c684afda24a1372698f91c0`), passed Phase 2 integration & hygiene gates (`run_id=02d73762546a4692a7b3184227a54991`) and Phase 3 protective `http-host` unit tests (`run_id=1bf23cb6c89640e085d976b8de59103e`, `9/9`), retired `4` temporary scopes (`CLN293`), and verified post-cleanup smoke (`run_id=33e9fc2ff1b14fa0baca25df1aa72661`).

## C2/A2-R3 Implementation & Verification Receipts

- **Non-Blocking RPC Error Branch (`crates/http-host/src/lib.rs`)**:
  - Removed the synchronous `io::stderr().lock()` write/flush from the `RpcResponseFrame::Error` arm in `run_rpc_worker`; Lisp output already flows directly to `stderr` via cloned/inherited fds and error messages are returned verbatim in HTTP `500` responses, preventing blocked log sinks from stalling HTTP `500` delivery or shutdown worker join.
- **Genuine Blocking Stderr Coverage & Protective Unit Test (`tools/test-c2-a2-lisp-app.py`, `crates/http-host/src/lib.rs`, `maintenance/tooling-inventory.json`)**:
  - Updated `Scenario D2` via `fill_pipe_and_restore_blocking(fd)` to restore `O_NONBLOCK` -> blocking mode (`os.set_blocking(fd, True)`) after filling the shared stderr pipe.
  - Added `Scenario D3` exercising a completely full blocking stderr pipe + sibling `/bin/sleep 5` holder process: `/api/boom-quiet` returns HTTP `500` without blocking, subsequent `/api/ping` returns `200` on the same live backend PID, and `SIGTERM` exits `0` in `< 3s` while reaping the backend and freeing the listening port.
  - Added unit test `rpc_worker_returns_handler_error_and_continues_on_nonfatal_rpc_error` confirming non-fatal RPC errors return `WorkerReply::HandlerError` and leave the worker healthy for subsequent requests (`9/9` passed).
- **Controlled Verification Runs (`assistant run`)**:
  - `dist/lisp` rebuild (`run_id=032f31e33c684afda24a1372698f91c0`, `184.37s`, `exit=0`, `cleanup_complete=true`, peak accounted `2,127,888,792` B).
  - Phase 2 runtime acceptance (`run_id=02d73762546a4692a7b3184227a54991`, `30.067s`, `exit=0`, `cleanup_complete=true`): `test-c2-a2-lisp-app.py --selftest` (`PASS`), `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`, Scenarios A/B/C/D1/D2/D3), `test-repository-bootstrap.py` (`4/4`), `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,327/6,327` `100.0%` `SAFETY:`, `0` errors), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`, `1.148s`).
  - Phase 3 protective unit tests (`run_id=1bf23cb6c89640e085d976b8de59103e`, `24.352s`, `exit=0`, `cleanup_complete=true`): `cargo test -p http-host` (`9/9`), `check-unsafe-boundary.py` (`0` errors), `check-own-voice.py` (`0` failures).
  - Post-cleanup smoke (`run_id=33e9fc2ff1b14fa0baca25df1aa72661`, `13.07s`, `exit=0`, `cleanup_complete=true`): `LISP=42`, `HTTP=http-host 0.1.0 (femacs 32.0.50)`, `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`).

## Retired Temporary Scopes

Retired `4` owned temporary build/cache/pin scopes totaling `3,761` files, `1,020` directories, `8` symlinks, `822,615,794` logical bytes, normalized digest `f5d8ee2b6c170143d50e359d3d308b35b487f760b6dbb06af781d6a72d7b313e`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `392M` | `2,849` | `516` | `0` | `472,407,287` | Root Cargo target cache (`cargo test` / `xtask`) |
| `build/cargo-tools` | `16M` | `49` | `29` | `0` | `15,842,651` | `xtask` host tool target cache |
| `build/lisp` | `335M` | `858` | `468` | `6` | `333,758,571` | Recreated by `xtask build-lisp --out dist/lisp` |
| `build/treesit-0.27.1` | `604K` | `5` | `7` | `2` | `607,285` | Reinstall via `python3 tools/toolchain-pins.py install` |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`):
  - `bin/femacs-lisp`: `5,053,696` B, SHA-256 `c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,325,840` B, SHA-256 `8b7033d0f887714299cc7a9ed134cff88e62bbf348e26c864be2936e1bdee14f`
  - `bin/http-host`: `2,145,728` B, SHA-256 `31f438b7ca7b102dff78545a03a128d6305f13732f7714b2e1ad3035013a5c00` (`codesign --verify --deep --strict` `exit 0`)
  - `dist/lisp/manifest.json` bytes/SHA-256 verified matching disk files.
- Unchanged retained sets: `build/terminal` (`45M`), `dist/macos` (`235M`, parity-matched with `/Applications/Fe.app`), `build/generation` (`13M`), and `build/manuals` (`1.4G`) (`build` `1.4G`, `dist` `445M`).
