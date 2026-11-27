# CLN309 — `fe-test` R2 Narrow Fixes Closure & Cleanup (2026-11-27)

## Summary

Resolved the three remaining edge-case items from independent review of `8883b678`, added three simulated-process-table Rust unit tests and deterministic missing-row + hermetic env self-tests, verified `xtask` (`132/132`), `--suite self` (`9/9`), and targeted cold/warm hermetic `smoke`/`host`/`ert` checks, retired `3` owned temporary build/test scopes (`1,901` files, `671` dirs, `2` symlinks, `296,224,827` logical bytes), and verified all retained products post-cleanup.

## Hardening Scope

- **Process-table error propagation & active-member PGID derivation** (`xtask/src/test_exec.rs`): `snapshot_process_table()` now returns `Result<Vec<ProcRow>, String>`; `compute_mem_tree` exits non-zero on snapshot failure; `reap_tracked_tree_with` returns `false` if process-table snapshotting cannot confirm zero survivors; `TrackedTree` permanently locks `root_retired` on `drop_root_after_wait()` so recycled root PIDs cannot re-anchor; `refresh_with_ident` & `verified_signal_targets` derive active PGIDs and signals strictly from currently alive identity-verified members.
- **Memory schema/row validation** (`tools/testing/fe-test-scheduler.el`): `fe-test--refresh-active-memory!` validates `femacs-test-mem-v1` and numeric `accounted_bytes` per live worker PID row, leaving prior `sampled`/`observed-mem` or unsampled reservations untouched when rows are missing or `mem-tree` fails.
- **Hermetic env scrubbing & unset-vs-empty cache keys** (`tools/testing/{fe-test-dsl,fe-test-cache}.el`, `suites/suite-{smoke,host,ert-core}.el`): cache keys encode `"unset"` (`nil`) vs `"set"` (`""`); `fe-test-hermetic-editor-env` unsets `EMACSLOADPATH`/`EMACSDATA`/`EMACSDOC`/`EMACSPATH` across product suites, and `fe-test-cache-key` returns `nil` (disabling cache) whenever an un-scrubbed external resource override variable is active.

## Controlled Post-Cleanup Verification (`run_id=b3559cdb8fdf407aa9952642ad81faef`, `18.869s`, `exit=0`, `cleanup_complete=true`)

- `build/` directory is completely empty and `tools/__pycache__` is absent.
- `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty and strict `codesign --verify --deep --strict /Applications/Fe.app` passed.
- Standalone `dist/lisp/bin/femacs-lisp` evaluated `42`; `/Applications/Fe.app/Contents/MacOS/Femacs --batch -Q` evaluated `(42 t t nil t)`; `/Applications/Fe.app/Contents/MacOS/bin/emacsclient --version` returned `emacsclient 32.0.50`.
- Static and repository gates passed: `tools/check-unsafe-boundary.py --manifest tools/unsafe-boundary-manifest.json --require-zero-static-mut --require-zero-active-stitching --require-100pct-safety-docs --require-no-file-allows --min-forbid-files 151` (`error_count=0`, `445` active files, `152` `forbid_unsafe_files`, `6,345/6,345` `SAFETY:` blocks), `tools/check-ledger-ticks.py` (`0` failures), `tools/check-own-voice.py` (`0` failures), `tools/test-repository-bootstrap.py` (`4/4`), and `tools/test-tooling-modernization.py` (`33/33`).

## Retired Temporary Scopes

Retired `3` owned temporary Cargo target, framework state, and unit-test scratch scopes totaling `1,901` files, `671` dirs, `2` symlinks, and `296,224,827` logical bytes:

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `1,870` | `560` | `0` | `296,219,780` | `42dd4f27d0197c4bef23489c3f61081364e1d850c3f21e301a24bd01aadf1500` | Rebuild on demand via `cargo build -p xtask --target-dir build/cargo` |
| `build/test-framework` | `9` | `2` | `0` | `4,612` | `93e85ac051e7c5c4222f92995a670e20e654fd06e3362b00f733701cdf5234e` | Re-created on demand by `dist/lisp/bin/femacs-lisp --batch -Q -L tools/testing -l fe-test.el` |
| `build/{acceptance,android-tests,bundle-tests,manuals-review,package-contracts,probes-tests,test-exec-unit-tmp,tooling-tests}` | `22` | `109` | `2` | `435` | `42cea037a873dfdca6f1a53b2617c3f852bd44fabc11019fa5451374e71bcbdb` | Ephemeral unit-test scratch recreated on demand by `cargo test -p xtask` |

## Retained Working Set

- `dist/macos/Femacs.app` (`235M`), `/Applications/Fe.app` (`235M`), and `dist/lisp` (`210M`, `9ef6e40f`).
