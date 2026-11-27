# CLN310 — `fe-test` R3 Narrow Fixes Closure & Cleanup (2026-11-27)

## Summary

Resolved the two remaining `xtask/src/test_exec.rs` edge-case items from independent review of `e1685687`, added two targeted Rust unit tests (`reap_skips_sigkill_and_returns_false_when_snapshots_fail_after_term` and `sample_pid_memories_omits_unconfirmed_root_not_in_snapshot`), verified `xtask` (`134/134`), retired `2` owned temporary build/scratch scopes (`1,346` files, `391` dirs, `2` symlinks, `231,711,905` logical bytes), and passed post-cleanup static and smoke gates.

## Fixes Closed

- **No stale `SIGKILL` after transient snapshot outage & immediate `root_pid` retirement**: removed `last_survivors` carry-over across `SIGTERM`→`SIGKILL` phases in `reap_tracked_tree_with_ident` so `SIGKILL` is issued only after a fresh snapshot and `refresh_with_ident`; retired `tree.drop_root_after_wait()` immediately when `child.try_wait()` observes `Ok(Some(_))` inside `terminate_tracked_tree`.
- **Omit unconfirmed `mem-tree` roots**: skipped any requested `root` absent from `ps_rss_by_pid` in `sample_pid_memories`, preventing `0`-byte synthetic entries from clearing scheduler memory reservations.

## Controlled Post-Cleanup Verification (`run_id=26243c746b2b448181f99a9463aa3159`, `19.867s`, `exit=0`, `cleanup_complete=true`)

- `build/` directory is completely empty and `tools/__pycache__` is absent.
- `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty and strict `codesign --verify --deep --strict /Applications/Fe.app` passed.
- Standalone `dist/lisp/bin/femacs-lisp` evaluated `42`; `/Applications/Fe.app/Contents/MacOS/Femacs --batch -Q` evaluated `(42 t t nil t)`; `/Applications/Fe.app/Contents/MacOS/bin/emacsclient --version` returned `emacsclient 32.0.50`.
- Static and repository gates passed: `tools/check-unsafe-boundary.py --manifest tools/unsafe-boundary-manifest.json --require-zero-static-mut --require-zero-active-stitching --require-100pct-safety-docs --require-no-file-allows --min-forbid-files 151` (`error_count=0`, `445` active files, `152` `forbid_unsafe_files`, `6,345/6,345` `SAFETY:` blocks), `tools/check-ledger-ticks.py` (`0` failures), `tools/check-own-voice.py` (`0` failures), `tools/test-repository-bootstrap.py` (`4/4`), and `tools/test-tooling-modernization.py` (`33/33`).

## Retired Temporary Scopes

Retired `2` owned temporary Cargo target and unit-test scratch scopes totaling `1,346` files, `391` dirs, `2` symlinks, and `231,711,905` logical bytes:

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `1,324` | `282` | `0` | `231,711,470` | `c25481457e43ba0bc4fb1ba5ea3c601e8fded90b89e8d49e15c68214c5507a43` | Rebuild on demand via `cargo build -p xtask --target-dir build/cargo` |
| `build/{acceptance,android-tests,bundle-tests,manuals-review,package-contracts,probes-tests,test-exec-unit-tmp,tooling-tests}` | `22` | `109` | `2` | `435` | `5f90c4394ec246f0d4388dc2857a28664305031afde682f0f48dff331e21446b` | Ephemeral unit-test scratch recreated on demand by `cargo test -p xtask` |

## Retained Working Set

- `dist/macos/Femacs.app` (`235M`), `/Applications/Fe.app` (`235M`), and `dist/lisp` (`210M`, `9ef6e40f`).
