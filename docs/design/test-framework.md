# FEmacs Workspace Boundaries & Native Test Framework (`fe-test`) Design

## 1. Purpose

This document defines two structural improvements in FEmacs:

1. **Workspace Crate Cleanup**: Dissolving the poorly bounded `femacs-pure` crate by homing every module in its real architectural domain (`femacs-sys` for OS/POSIX and low-level `lib/` algorithms, `femacs-vm` for core editor data tables), leaving a clean 5-crate workspace.
2. **Native Test Framework (`fe-test`)**: Replacing ad-hoc sequential Python test drivers with an in-repo native test framework built on **pure Lisp orchestration** (`tools/testing/*.el` executed by `femacs-lisp`) and **thin Rust OS process/hash primitives** (`xtask test-exec`).

---

## 2. Clean 5-Crate Workspace Architecture

Audit of all module consumers showed that `femacs-pure` was an accidental split:
- `femacs-tools` had zero code references to `femacs_pure`.
- The Adobe-Japan1 UVS table (`src/include/macuvs.rs`) belongs strictly to the `src/` VM layer.
- All 14 remaining `#![forbid(unsafe_code)]` modules (`bitops`, `c_ctype`, `filemode`, `filevercmp`, `md5`, `memmem`, `mini_gmp`, `sha1`, `sha256`, `sha3`, `sha512`, `signal_names`, `strcase`, `timespec`) exclusively back `lib/*.rs` and `lib/include/*.rs` inside `crates/femacs-sys`.

Therefore, the workspace converges to five domain-authentic crates:

| Crate | Role | Unsafe Policy |
| --- | --- | --- |
| `femacs-sys` | OS/POSIX FFI wrappers (`lib/`), client OS primitives (`client_os`), `VmGlobal`, and the 14 `#![forbid(unsafe_code)]` safe core algorithm/math/digest modules | Mixed (`#![deny(unsafe_op_in_unsafe_fn, static_mut_refs)]` at crate root + `#![forbid(unsafe_code)]` on each pure algorithm module) |
| `femacs-vm` | Lisp VM, GC, evaluator, bytecode, compiler, and core editor modules (`src/`, `safe_*.rs`, `buildobj`, `macuvs`) | Mixed (`#![deny(...)]` at crate root + `#![forbid(unsafe_code)]` on `safe_*.rs` domain modules) |
| `femacs-host` | Headless `LispHost` embedding API and standalone `femacs-lisp` CLI binary | `#![forbid(unsafe_code)]` |
| `femacs-tools` | Companion CLI binaries (`make-fingerprint`, `hexl`, `asset-directory-tool`, `femacsclient`) | `#![forbid(unsafe_code)]` |
| `http-host` | Standalone Axum/Tokio HTTP edge host supervising `femacs-lisp --rpc http` | `#![forbid(unsafe_code)]` |

Note on path sharing: `crates/femacs-sys/src/lib.rs` declares `extern crate self as femacs_sys;` so shared `lib/include/*.rs` headers compiled inside both `femacs-sys` and `femacs-vm` resolve `::femacs_sys::<module>` uniformly without duplicate wrapper code.

---

## 3. `fe-test` Architecture Overview

```text
  Verified Control Host (dist/lisp/bin/femacs-lisp)
        │
        ▼
  tools/testing/fe-test.el                  (CLI, suite selection, shard, --changed, --rerun-failed)
    ├── fe-test-dsl.el                      (case struct, target discovery, sandbox temp dirs)
    ├── fe-test-profile.el                  (EWMA elapsed/memory history + last-failures ledger)
    ├── fe-test-cache.el                    (content-addressed pass cache keyed by SHA-256 digests)
    ├── fe-test-scheduler.el                (async make-process LPT + memory admission control)
    ├── fe-test-reporter.el                 (latest-report.json + concise console summary)
    └── suites/suite-{static,smoke,host,ert-core}.el
        │
        ├── async workers (`xtask test-exec supervise ...`)
        ├── live tree memory (`xtask test-exec mem-tree <pids...>`)
        └── streaming SHA-256 (`xtask test-exec digest <paths...>`)
```

### 3.1 Control Plane vs. Target-Under-Test Isolation

- **Control host**: Orchestration runs on the verified `dist/lisp/bin/femacs-lisp` package (optionally pre-checked with `--verify-host` against `dist/lisp/manifest.json`).
- **Candidate targets**: Every test runs in a **fresh child process** supervised by `xtask test-exec supervise` against explicit target binaries (Terminal `build/terminal/port/src/emacs`, Cocoa `/Applications/Fe.app/Contents/MacOS/Femacs`, candidate `femacs-lisp`, or `http-host`). Test assertions and VM crashes inside candidate targets remain strictly isolated from the orchestrator.

---

## 4. Thin Rust OS Supervision Layer (`xtask test-exec`)

`xtask/src/test_exec.rs` (`#![forbid(unsafe_code)]`) delegates minimal OS syscalls (`kill(-pgid, sig)` and macOS `proc_pid_rusage`) to `xtask/src/os_ffi.rs` and exposes three JSON subcommands:

1. **`xtask test-exec supervise [OPTIONS] -- <PROGRAM> [ARGS...]`**
   - Options: `--timeout-ms N`, `--memory-limit-bytes N`, `--poll-ms N`, `--cwd DIR`, `--env K=V`, `--unset-env K`, `--pty`, `--max-output-bytes N`.
   - Places the child in its own process group (`process_group(0)`), optionally allocates a PTY, drains bounded `stdout`/`stderr` with a hard drain deadline after exit, tracks live process identities (`ri_proc_start_abstime` + `ps` `lstart` + discovered PGIDs including PTY sub-groups) across ticks, samples process-tree RSS and macOS `phys_footprint`, enforces timeouts and memory ceilings across all tracked PGIDs and descendant PIDs (never treating `ESRCH` on a root PGID as proof descendants exited), and marks `children_reaped: true` only when all tracked descendants exited and both output pipes drained within deadline.
   - Emits `femacs-test-exec-v1`:
     `{ schema, exit_code, signal, timed_out, memory_exceeded, elapsed_ms, peak_rss_bytes, peak_footprint_bytes, peak_accounted_bytes, max_tree_pids, output_drained, children_reaped, stdout, stderr, stdout_truncated, stderr_truncated }`.

2. **`xtask test-exec mem-tree <PID>...`**
   - Samples the transitive process-tree + process-group closure of each active worker PID in one snapshot (deduplicating overlapping PIDs).
   - Emits `femacs-test-mem-v1`: `{ schema, total_rss_bytes, total_footprint_bytes, total_accounted_bytes, per_pid }`.

3. **`xtask test-exec digest <PATH>...`**
   - Streams SHA-256 across files and sorted directory trees (skipping `.git` and `__pycache__`).
   - Emits `femacs-test-digest-v1`: `{ schema, digest, file_count, total_bytes, entries }`.

---

## 5. Pure Lisp Framework Components (`tools/testing/`)

### 5.1 Memory-Aware LPT Dynamic Admission (`fe-test-scheduler.el`)

- **LPT Ordering**: Pending cases are sorted in descending order of predicted `elapsedMs` from `profile.json` (defaulting `heavy` cases to `5000 ms`, standard cases to `1000 ms`, deterministic tie-break by case `id`) so long tails start immediately.
- **Continuous Sampling & Admission Gating**:
  - Constants: `admission-ratio = 0.78`, `cold-start-grace-ms = 4000`, `poll-interval = 0.05 s`, `mem-sample-interval = 150 ms`.
  - Every active run polls `mem-tree` at `mem-sample-interval` continuously regardless of whether launch slots remain open or `pending` is empty.
  - Between samples, each worker preserves its latest valid `observed-mem`, its full predicted reservation while unsampled, and `(max observed-mem est-mem)` during `cold-start-grace-ms` so sampling gaps cannot bypass the `0.78 * budget_bytes` ceiling.
  - Always admits at least `1` worker when idle so a large single test never deadlocks; records `admissionPauses` and `peakAggregateRssBytes`.
  - Result validation enforces supervisor exit 0, valid `femacs-test-exec-v1` schema, and `children_reaped = true` before passing or writing cache entries.
- **Fail-Fast & `--keep-going`**: Stops admitting and reaps active workers on first failure by default; runs full suite when `--keep-going` is passed.

### 5.2 Persistent State & Schemas (`build/test-framework/`)

- **Profile Ledger (`profile.json`, `femacs-test-profile-v1`)**: Per-case `{ runs, status, updatedAt, elapsedMs, peakRssBytes, peakFootprintBytes }` updated via EWMA (`round(0.7 * latest + 0.3 * prev)`).
- **Failure Ledger (`last-failures.json`, `femacs-test-failures-v1`)**: `{ format, suite, updatedAt, failedTests }` consumed by `--rerun-failed`.
- **Result Cache (`result-cache/<sha256>.json`, `femacs-test-cache-v1`)**: Stores passed cacheable case verdicts keyed by SHA-256 over case identity, exact command/env, complete product runtime closure (`binary`, `.pdmp`, `lisp/`, `etc/`, bundled shared runtimes — disabling cache automatically when the runtime closure cannot be resolved), declared test inputs, and framework digest.
- **Run Report (`latest-report.json`, `femacs-test-report-v1`)**: `{ format, suite, total, passed, failed, cached, unstarted, cancelled, wallMs, cumulativeCaseMs, observedConcurrency, peakAggregateRssBytes, peakAggregateRssMiB, admissionPauses, slowest, failedTests, cases }`.

### 5.3 High-Frequency Core Suites (`tools/testing/suites/`)

- `static`: Parallel cached static/boundary/voice/bootstrap/inventory gates plus strict byte-compilation of all `tools/testing/**/*.el` files (zero warnings).
- `smoke`: Native Lisp checks replacing ad-hoc shell/Python smoke snippets (bundle parity, strict codesign, Terminal & Cocoa batch Lisp + `treesit` + SHA-256 checks, `femacs-lisp` eval, manifest binary verification, `emacsclient --version`, zero Python bytecode cache).
- `host`: Headless `femacs-lisp` CLI flags, `--script` argument passing, exit codes and `unwind-protect` on `(kill-emacs)`, multiline `--repl`, and `http-host` live HTTP roundtrip.
- `ert`: Core `test/src/*-tests.el` ERT files dispatched as independent parallel LPT-scheduled workers with per-file profiling, caching, and `--rerun-failed`.
- `self`: Framework unit and integration self-tests (`tools/testing/tests/fe-test-self-tests.el`).
