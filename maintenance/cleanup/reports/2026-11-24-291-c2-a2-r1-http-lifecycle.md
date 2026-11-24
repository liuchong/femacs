# CLN291 — Stage C2/A2-R1 HTTP Host Lifecycle Closure & Cache Retirement (2026-11-24)

## Summary

Closed the HTTP edge lifecycle blocker from `[FEMACS-C2-A2-REVIEW]` (`17ef5430`) in `#![forbid(unsafe_code)]` [`crates/http-host/src/lib.rs`](../../../crates/http-host/src/lib.rs), registered `/dist/lisp` in [`.gitignore`](../../../.gitignore) and [`tools/test-repository-bootstrap.py`](../../../tools/test-repository-bootstrap.py), extended [`tools/test-c2-a2-lisp-app.py`](../../../tools/test-c2-a2-lisp-app.py) and [`maintenance/tooling-inventory.json`](../../tooling-inventory.json) with disconnect eviction, hung-handler `504` / queued eviction, and busy-`SIGTERM` lifecycle checks, rebuilt `dist/lisp` (`run_id=c127f7bd43614fe3914e98193c74d2ae`), passed Phase 2 integration & hygiene gates (`run_id=64d4b408ca8542bcbdd3f776e37dcb01`) and Phase 3 protective `http-host` unit tests (`run_id=31594183b0ac4307bdb437dffae12af8`, `7/7`), retired `4` temporary scopes (`CLN291`), and passed post-cleanup package smoke (`run_id=0be5b28c5be4447bae75523a0273a825`). Also corrected the prior `Stage C2 & A2` note on `dcdd6dc3e9984f3bb463be747809aed6` so it accurately reflects A2 subcommand passage with overall `exit=1` on the pre-fix `test-u4-unit.py` `serde_derive` probe, superseded by `1fcaefbae8fc48978a0e20417e57587d` and `0313057f76704aafa756cd349bf7575d` (`exit=0`).

## C2/A2-R1 Implementation & Verification Receipts

- **Bounded HTTP Host Lifecycle (`crates/http-host/src/lib.rs`)**:
  - Replaced unbounded blocking RPC socket reads/writes in `run_rpc_worker` with `RPC_IO_POLL_SLICE` (`50ms`) deadline- and shutdown-aware helpers (`write_all_with_deadline`, `read_bounded_rpc_line_with_deadline`), propagating per-request `Instant` deadlines from `handle_http` via `WorkerJob::deadline`.
  - Evicts disconnected callers (`job.respond.is_closed()`) and expired queued items (`Instant::now() >= job.deadline`) before writing to the RPC socketpair; never replays timed-out requests.
  - On `TimedOut`, closes the RPC socket, reaps the wedged backend process immediately (`kill_child_slot_now`), and drains remaining queued jobs via `drain_queued_jobs`.
  - On `SIGTERM`/`SIGINT` graceful shutdown (`serve()`), flips `shutdown_flag` and shuts down cloned `control_sock_sig`/`control_sock` (`Shutdown::Both`), then runs `reap_child_slot(&child_slot, 500ms)` strictly before `worker_handle.join()` and stdout/stderr forwarder joins, while preserving same-PID backend persistence across normal Lisp condition `500` errors.
- **Ignore Rules & Lifecycle Acceptance (`/.gitignore`, `tools/test-repository-bootstrap.py`, `tools/test-c2-a2-lisp-app.py`, `maintenance/tooling-inventory.json`)**:
  - Added `/dist/lisp` to `.gitignore` and ignore coverage (`dist/lisp/manifest.json`, `dist/lisp/bin/http-host`) in `tools/test-repository-bootstrap.py`.
  - Added `exercise_http_lifecycle_edge_cases` (`Scenario B` disconnect eviction + `/api/hang` `504` + wedged child reap + queued non-execution; `Scenario C` busy `SIGTERM` with `--timeout-ms 15000` exiting `0` in `< 3s` + child reap + queued non-execution + `assert_port_reusable`) while retaining `Scenario A` (`500` logs, live same-PID `200` recovery, `/api/check-cloexec`, idle `SIGTERM`).
- **Controlled Verification Runs (`assistant run`)**:
  - Clean `dist/lisp` rebuild (`run_id=c127f7bd43614fe3914e98193c74d2ae`, `181.142s`, `exit=0`, `cleanup_complete=true`, peak accounted `2,063,878,400` B).
  - Phase 2 runtime acceptance (`run_id=64d4b408ca8542bcbdd3f776e37dcb01`, `28.208s`, `exit=0`, `cleanup_complete=true`): `test-c2-a2-lisp-app.py --selftest` (`PASS`), `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`), `test-repository-bootstrap.py` (`4/4`), `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,327/6,327` `100.0%` `SAFETY:`, `0` errors), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`, `1.074s`).
  - Phase 3 protective unit tests (`run_id=31594183b0ac4307bdb437dffae12af8`, `25.89s`, `exit=0`, `cleanup_complete=true`): `cargo test -p http-host` (`7/7`, including `rpc_worker_evicts_expired_and_disconnected_jobs_without_writing_to_socket`, `rpc_worker_enforces_absolute_deadline_on_hung_peer`, and `rpc_worker_wakes_promptly_on_shutdown_flag_and_drains_queue`), `check-unsafe-boundary.py` (`0` errors), `check-own-voice.py` (`0` failures).
  - Post-cleanup smoke (`run_id=0be5b28c5be4447bae75523a0273a825`, `13.132s`, `exit=0`, `cleanup_complete=true`): `LISP=42`, `HTTP=http-host 0.1.0 (femacs 32.0.50)`, `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`).

## Retired Temporary Scopes

Retired `4` owned temporary build/cache/pin scopes totaling `3,761` files, `1,020` directories, `8` symlinks, `803,855,644` logical bytes, normalized digest `72075c6ea00064d22d512134f5d960ce5d24ef17aa3d04956eb4036d5f0f16b3`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `379M` | `2,849` | `516` | `0` | `453,982,309` | Root Cargo target cache (`cargo test` / `xtask`) |
| `build/cargo-tools` | `16M` | `49` | `29` | `0` | `15,842,653` | `xtask` host tool target cache |
| `build/lisp` | `321M` | `858` | `468` | `6` | `333,423,397` | Recreated by `xtask build-lisp --out dist/lisp` |
| `build/treesit-0.27.1` | `604K` | `5` | `7` | `2` | `607,285` | Reinstall via `python3 tools/toolchain-pins.py install` |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`):
  - `bin/femacs-lisp`: `5,053,696` B, SHA-256 `c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,325,840` B, SHA-256 `42ada548655372605714b99411969e4cba4bb2427b550bab86cfa77aeddd66d0`
  - `bin/http-host`: `2,089,888` B, SHA-256 `d91553c1e067e2b563c463bcd02fc8d9de29572bdd1e4e93abf9fd48bc97f39f` (`codesign --verify --deep --strict` `exit 0`)
  - `dist/lisp/manifest.json` bytes/SHA-256 verified matching disk files.
- Unchanged retained sets: `build/terminal` (`45M`), `dist/macos` (`235M`, parity-matched with `/Applications/Fe.app`), `build/generation` (`12M`), and `build/manuals` (`1.4G`) (`build` `1.4G`, `dist` `445M`).
