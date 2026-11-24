# CLN285 — U8-R2 Review Closure, AU8 Strict Re-Acceptance & Consumed Cache Retirement (2026-11-24)

## Summary

After closing all three blocking findings and the performance `--gate` gap reported by independent review of `8b459250` (`P-488`), completing clean-export terminal and Cocoa rebuilds, passing all four `AU8` acceptance groups (`clean-export`, `macos-ui`, `corpus-555`, `performance`) under strict `stable-classification` adjudication and `bench.py --gate`, atomically reinstalling `/Applications/Fe.app`, and keeping all retained deliverables intact, this batch retired `14` consumed temporary build, runtime, acceptance, session-cache, and `<noindex>` container scopes.

## Accepted Closure & Re-Acceptance Scope

- **Review closure (`P-488`)**:
  - Blocking #1 (`crates/femacs-vm/src/{roots,nonlocal,vm_primitives}.rs`, `src/{thread,emacs}.rs`): bound VM global lock ownership to `(OS thread ID, current_thread)` across acquire/release and `cond_wait_releasing_vm_lock`; tightened `GcContext::current()` to a single process-wide lease (`GC_CONTEXT_ACTIVE` CAS + `Drop`) with `!Send + !Sync` markers and `with_reentrant_safepoint` at VM boundaries; replaced LIFO `truncate` in `Rooted`/`ScopedRawSpan`/`ScopedLispRoot` with 64-bit generation-checked free-list slot release and strict panic on stale `Handle::get()`; moved hot root counters to per-thread plain counters and removed redundant incremental `set_len` strides in `src/eval.rs` and hot `maybe_grow` checks in `src/bytecode.rs`.
  - Blocking #2 (`crates/femacs-vm/src/nonlocal.rs`, `PORTING.md` `P-488`, `tools/test-u3-unit.py`): codified user-approved Option A (`[profile.release] panic = "unwind"`, `VmNonLocalUnwind` + `extern "C-unwind"` internal to `femacs-vm` `src/*.rs`, explicit `VmResult<T>` / C status propagation across `vm_primitives.rs`, `.eln` `src/comp.rs`, `src/emacs-module.rs`, and `src/thread.rs::run_thread`).
  - Blocking #3 & performance `--gate` (`tools/{test-u8-au8-integration,g3-reference-binding,stable-classification,test-stable-classification}.py`, `docs/baseline/bench-{cocoa,terminal}-ga8.tsv`): removed `(file, id)` and `"Femacs"` pre-screen waivers, bound all `76` strict followup files + `12` external control envelopes through `stable-classification.py` `_accept_rule`/`assess`, added `non-collector` `FEMACS_GC_COLLECTOR="major"` support in `scoped_candidate_profile`, isolated the reference PTY `"Cannot determine test monitor"` flake in `test/lisp/filenotify-tests.el` into a bounded retry slice without relaxing rules, and enforced `bench.py --gate` (`regressions: []`) on both Cocoa and terminal builds.
- **`AU8` four-group R2 acceptance (`4/4` passed on final rebuilt deliverables)**:
  1. `clean-export` (`930.176s`, `7,722` tracked exports, `23/23` steps passed): active-product closure `436` files (`143` `#![forbid(unsafe_code)]`, `0` file-level allows, `0` `static mut`, `0` stitching `include!`, `6,245/6,245` `SAFETY:` blocks), `rpath_dylib_count=9`, `bundled_framework_dylib_count=17`, strict `codesign` `exit 0` (`spctl exit 3`), relocated GUI (`window_system=ns`) and batch execution passed.
  2. `macos-ui` (`137.027s`): `local-acceptance --gui --ime` (`14` checks, `110.791s`), `macos-ime-multi-window` (`447` callbacks, `a_text="你好"`, `b_text="你好"`, `focus_loss_cleared_marked=true`, `16.738s`), `macos-finder-drag` (`26` motion / `31` callbacks, `9.147s`), `abnormal-tty-stability` (`16` cases, `0.297s`).
  3. `corpus-555` (`5625.912s` strict closure): `555` files / `8,685` compared cases + `12` external control envelopes; `u8-native-normal` (`11/11` / `603` cases: `37` bound rules, `0` blocking, `0` structural) and `u8-native-checking` (`11/11` / `603` cases: `37` bound rules, `0` blocking, `0` structural); `strict_incomplete_followup.primary` (`cocoa-baseline-normal`, `76` files / `1,031` raw cases: `801` passed, `221` classified, `9` verified in both `u8-native` channels, `0` unexplained blocking, `0` structural); `10` ERT fixtures + `runtime-execution.py` contracts (`prepare_negative=1`, `ert_native_collector_young=120/120`, `ert_collector_major=44/44`) passed.
  4. `performance` (`103.607s`): `bench.py --gate` `0` regressions on Cocoa (`docs/baseline/bench-cocoa-ga8.tsv`, `startup` `20.8524 vs 181.2942 ms`, `gc` `32.7082 vs 51.8510 ms`, `edit` `7.8261 vs 9.6340 ms`, `regex` `2.2972 vs 2.7621 ms`, `load` `1.7502 vs 1.8458 ms`) and terminal (`docs/baseline/bench-terminal-ga8.tsv`, `startup` `19.2763 vs 184.2832 ms`, `gc` `31.0209 vs 50.4651 ms`, `edit` `7.6158 vs 9.5429 ms`, `regex` `2.2271 vs 2.7461 ms`, `load` `1.6029 vs 1.6978 ms`); `delivered-terminal-vs-reference-bytecode` (`bytecode 0.1086`, `edit 1.1607`, `gc-churn 0.7059`, `gc-retained 0.6816`, `load 0.2796`, `regex 1.0460`) and `delivered-cocoa-vs-reference-bytecode` (`bytecode 0.1086`, `edit 1.1511`, `gc-churn 0.9130`, `gc-retained 0.8258`, `load 0.5885`, `regex 1.0819`).

## Retired Temporary & Verification Scopes

Retired `14` owned temporary build, runtime, acceptance, session-cache, and container scopes totaling `48,469` files, `5,718` directories, `140` symlinks, and `2,741,223,009` logical bytes (plus removed empty `<session-cache>` parent directory):

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `build/runtime` (`u8-native`, `u8-baseline`) | `1.7G` | `18,990` | `893` | `32` | `1,821,976,441` | `cargo run -p xtask -- runtime-build ...` |
| `build/cargo` | `292M` | `2,473` | `593` | `0` | `333,805,215` | `CARGO_TARGET_DIR` workspace incremental cache (rebuilt on demand) |
| `build/acceptance` (`u8-r2-controls`, `u8-runtime-contracts`, `ert`, `info-publication`, `manual-command`) | `270M` | `13,430` | `2,742` | `56` | `248,392,255` | Re-run `tools/stable-controls.py` / `tools/runtime-execution.py` / `tools/test-u8-au8-integration.py` |
| `build/goal-identity-unit` + `build/goal-output-guard-unit` | `0` | `0` | `2` | `0` | `0` | Recreated by `tools/test-goal-stability.py` |
| `<session-cache>/au8-r2` | `344M` | `12,968` | `1,374` | `48` | `326,859,912` | Re-run `tools/test-u8-au8-integration.py` groups |
| `<session-cache>/treesit-0.27.1` | `604K` | `5` | `8` | `2` | `607,298` | Rebuild from pinned source in `build-support/toolchain-pins.json` |
| `<noindex>/f*` (`7` owned test containers) | `11M` | `603` | `106` | `2` | `9,581,888` | Owned `stable-suite.py` temporary containers; recreated on demand |

## Retained Working Set, Post-Cleanup Verification & Application Bundle Parity

- `build/terminal` (`45M`):
  - `build/terminal/port/src/emacs`: `4,733,952` B, SHA-256 `f1450fadfce4674d17bde284c51b1f0bcd313f48eb93dbd6067d973a67ec40c6`
  - `build/terminal/port/src/emacs.pdmp`: `13,101,368` B, SHA-256 `087e5609b9e8559321b6df296135a63c5866b57a720113fd957aa95831b5eac2`
  - `build/terminal/port/lib-src/emacsclient`: `539,328` B, SHA-256 `1baf54e687c5fd61259f6699a2d2953b7e6567e350a304942b2e06f38ba600c7`
  - `build/terminal/port/lib-src/hexl`: `451,488` B, SHA-256 `15f41ba92128f2549c6e7f8f3e6685f379b9dec79d0f34b4696a4e5dda4878af`
- `dist/macos/Femacs.app` (`235M`) & atomically installed `/Applications/Fe.app`:
  - `Contents/MacOS/Femacs`: `6,267,632` B, SHA-256 `c1481fe564794624663c17b8cc9b9ff5dfbbd2e3509af8feb08d8fedcc89992a` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,601,672` B, SHA-256 `65d5e30c9b991dc98f128846da3c5ee2d4a848bf0d08e63264e8f6458557e3ac` (exact match across workspace & `/Applications/Fe.app`)
  - `Contents/MacOS/bin/emacsclient`: SHA-256 `7d4f5304e19d0cb5a73e26aee5e18e825f2a3c5259e28ba85de03b61af79e0b1`
  - `Contents/MacOS/bin/hexl`: SHA-256 `bf86dfb83eeaf61783147a636f90db50bb085054c884b57bb7bfd6d1e3d04604`
  - `codesign --verify --deep --strict dist/macos/Femacs.app` -> `exit 0`; `codesign --verify --deep --strict /Applications/Fe.app` -> `exit 0`.
- Retained `build/generation` (`1.5M`) and `build/manuals` (`185M`) unchanged.
- Post-cleanup SHA-256 checks matched `clean-export` `published_deliverables` `4/4`, `-Q --batch` `(+ 1 2)` / `(treesit-available-p)` / `(require 'treesit)` returned `(3 t t nil t)` (`exit 0`) on both `build/terminal/port/src/emacs` and `dist/macos/Femacs.app/Contents/MacOS/Femacs`, and installed CLI wrappers `femacs`/`fe` (`-Q --batch`) returned `(3 t)` while `feclient --version` returned `emacsclient 32.0.50`.
