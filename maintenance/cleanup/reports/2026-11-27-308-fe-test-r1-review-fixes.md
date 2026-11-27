# CLN308 — `fe-test` R1 Review Hardening & Cleanup (2026-11-27)

## Summary

Resolved the three technical issues and metric naming correction identified during independent review of `0086066a` / `b12d7e4e`, added four Rust and three Elisp protective unit tests, verified `xtask` (`129/129`), `femacs-sys` (`12/12`), `http-host` (`9/9`), `--suite self` (`9/9`), cold & warm `fe-test --suite all` (`38/38`), and all five static Python gates, retired `3` owned temporary build/test scopes (`4,266` files, `1,023` dirs, `4` symlinks, `620,702,598` logical bytes), and verified all retained products post-cleanup.

## Hardening & Evidence Boundary

- **OS process identity reaping & bounded pipe drain** (`xtask/src/os_ffi.rs`, `xtask/src/test_exec.rs`, `tools/testing/fe-test-scheduler.el`): tracked live descendant PIDs/PGIDs across ticks using `ri_proc_start_abstime` + `ps lstart` so PTY sub-groups and reparented stdout holders are killed and awaited cleanly, removed the `ESRCH` short-circuit in reaping, replaced unbounded thread joins with `500ms`/`300ms` bounded `PipeDrain`, added `output_drained` to `femacs-test-exec-v1`, and rejected non-zero supervisor exits, bad schemas, or `children_reaped=false` before pass verdicts or cache writes.
- **Cross-tick memory reservation & continuous sampling** (`tools/testing/fe-test-scheduler.el`): preserved prior `observed-mem` and unsampled `est-mem` across ticks instead of resetting `projected-bytes` to `0`, and polled `mem-tree` continuously at `fe-test-mem-sample-interval-ms` (`150ms`) even at full concurrency or empty `pending`.
- **Full product runtime closure keys** (`tools/testing/fe-test-dsl.el`, `tools/testing/fe-test-cache.el`, `suites/suite-{smoke,host,ert-core}.el`): keyed cached editor/host targets on `(executable, .pdmp, lisp/, etc/, runtime-* / Frameworks/)` and automatically disabled caching when a binary's runtime closure is unknown.
- **Accurate reporter metrics** (`tools/testing/fe-test-reporter.el`): renamed cumulative elapsed sums and concurrency ratios to `cumulativeCaseMs` / `observedConcurrency` (`caseSum` / `concurrency` in console output).
- **Retained product evidence boundary**: no changes touched `src/`, `lib/`, `crates/femacs-vm`, or `crates/femacs-host`; a redundant full editor re-link was declined, new Rust sources are verified via `cargo check --workspace` and unit suites (`129/129`, `12/12`, `9/9`), and `fe-test` framework/cleanup checks ran against retained `/Applications/Fe.app` and `dist/lisp` (`9ef6e40f`).

## Controlled Post-Cleanup Verification (`run_id=46e5d45636c74178b1a4b740a8aab236`, `18.887s`, `exit=0`, `cleanup_complete=true`)

- `build/` directory is completely empty and `tools/__pycache__` is absent.
- `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty and strict `codesign --verify --deep --strict /Applications/Fe.app` passed.
- Standalone `dist/lisp/bin/femacs-lisp` evaluated `42`; `/Applications/Fe.app/Contents/MacOS/Femacs --batch -Q` evaluated `(42 t t nil t)`; `/Applications/Fe.app/Contents/MacOS/bin/emacsclient --version` returned `emacsclient 32.0.50`.
- Static and repository gates passed: `tools/check-unsafe-boundary.py --manifest tools/unsafe-boundary-manifest.json --require-zero-static-mut --require-zero-active-stitching --require-100pct-safety-docs --require-no-file-allows --min-forbid-files 151` (`error_count=0`, `445` active files, `152` `forbid_unsafe_files`, `6,345/6,345` `SAFETY:` blocks), `tools/check-ledger-ticks.py` (`0` failures), `tools/check-own-voice.py` (`0` failures), `tools/test-repository-bootstrap.py` (`4/4`), and `tools/test-tooling-modernization.py` (`33/33`).

## Retired Temporary Scopes

Retired `3` owned temporary Cargo target, framework state, and unit-test scratch scopes totaling `4,266` files, `1,023` dirs, `4` symlinks, and `620,702,598` logical bytes:

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `4,185` | `814` | `0` | `620,673,856` | `4190d4c4ead393d6f164ab09f1d20e5b9debb6c8db49150e95f80b361b394068` | Rebuild on demand via `cargo build -p xtask --target-dir build/cargo` |
| `build/test-framework` | `37` | `2` | `0` | `27,872` | `4cdb22fbd1461bd76355b26808176ac4a35b72ee438e772ca977707d180d7641` | Re-created on demand by `dist/lisp/bin/femacs-lisp --batch -Q -L tools/testing -l fe-test.el` |
| `build/{acceptance,android-tests,bundle-tests,manuals-review,package-contracts,probes-tests,test-exec-unit-tmp,tooling-tests}` | `44` | `207` | `4` | `870` | `65399e96cc10d7ce653b26d6c39d7664aedc4bf17db8122b62dfeef2e47c5801` | Ephemeral unit-test scratch recreated on demand by `cargo test -p xtask` |

## Retained Working Set

- `dist/macos/Femacs.app` (`235M`), `/Applications/Fe.app` (`235M`), and `dist/lisp` (`210M`).
