# CLN284 — U8 final integration, `rtn15` stack hygiene fix, AU8 four-group verification, and cache retirement

Date: 2026-11-23
Scope: Phase 0 Stage `U8` (`集中整合与终验`, `P-487`) and formal acceptance `AU8` (`现有编辑器可交付`).

## Context & Verified Deliverables

- Completed `U8` integration and `rtn15-unreachable-native-unit-is-not-strong-cache-rooted` stack-hygiene fix:
  - Added unified `tools/test-u8-au8-integration.py` (retiring superseded `tools/test-u5-au5-integration.py` in `docs/retired-scaffolding.md`), updated Cocoa runtime forwarders to writable `VmGlobal`, and enforced `check-unsafe-boundary.py` active ratchets (`436` active files: `143` `#![forbid(unsafe_code)]`, `0` `allow(unsafe_op_in_unsafe_fn)`, `0` `allow(static_mut_refs)`, `0` `static mut`, `0` stitching `include!`, `6,243 / 6,243` `100.0%` `SAFETY:` blocks, `4,337` `unsafe fn`).
  - Fixed `rtn15` in `src/alloc.rs`, `src/include/lisp.rs`, and `src/eval.rs`: added `frame_depth: usize` + `SA_FRAME_DEPTH` + `sa_stack_frame_enter` / `sa_stack_frame_exit(mark, depth)` + `SaStackUnwindGuard::enter(mark)` to scrub recycled `sa_stack_free` blocks after nested frame exit, replaced 4 `MaybeUninit<[Lisp_Object; 8]>` arrays with `[Qnil (); 8]`, added `_cc_sa_guard` to `internal_lisp_condition_case`, and isolated `#[inline(never)]` slow spread paths (`apply_spread_slow`, `apply1_spread_slow`, `funcall_subr_4_to_8`).
- Verified all four `AU8` groups (`4/4` passed on final rebuilt deliverables):
  1. `clean-export` (`834.215s`, `7,719` tracked exports): clean-export terminal & Cocoa builds, `nm` singleton ABI & `check-libshadows` & boundary ratchets, relocated Cocoa/TTY batch checks (`spctl exit 3` unsigned/unnotarized local boundary), and repo deliverable rebuilds.
  2. `macos-ui` (`142.073s`): `local-acceptance` (`14` checks, `113.795s`), `macos-ime-multi-window` (`456` callbacks, `a_text="你好"`, `b_text="你好"`, `focus_loss_cleared_marked=true`, `17.95s`), `macos-finder-drag` (`23` motion / `28` callbacks, `9.78s`), and `abnormal-tty-stability` (`16` cases, `0.496s`).
  3. `corpus-555` (`2887.939s` completion leg): `adjudication_555` (`555` files / `8,685` compared cases, `0` blocking), `strict_incomplete_followup` (`72/72` files / `781` cases, `0` blocking), `u8-native-normal` (`11/11` / `603` cases, `0` blocking) & `u8-native-checking` (`11/11` / `603` cases, `0` blocking, `0` incomplete), `g7-native-write-barrier.el` (`0.863s`) + 6 runtime/bytecode ERTs, and `runtime-execution.py` contracts (`prepare_negative=1`, `ert_native_collector_young=120/120`, `ert_collector_major=44/44`).
  4. `performance` (`98.264s`): 7-round `bench.py` + 4 delivered + 4 mode comparisons against installed `32.0.50` the reference with zero regressions (`startup` `23.918–24.259 vs 146.635–150.667 ms`, `delivered-terminal-vs-reference-bytecode` `bytecode 0.116`, `gc-churn 0.679`, `gc-retained 0.647`, `load 0.284`, `regex 0.955`).
- Passed all `12` Phase 3 protective gates (`test-u8-au8-integration.py --group selftest` `9/9`, `check-unsafe-boundary.py --selftest` `7/7`, active boundary ratchet `0` errors, `test-u7-safe-subsystems.py --selftest` `5/5`, `test-u4-unit.py` `3/3`, `test-u3-unit.py` `3/3`, `check-defvar-carriers.py --selftest` `7/7`, `check-ledger-ticks.py --selftest` `6/6`, `check-libshadows.py --selftest`, `test-repository-bootstrap.py`, `test-tooling-modernization.py` `33/33`, `check-own-voice.py` `0` failures) plus `cargo test -p femacs-pure -p femacs-sys -p xtask --lib` (`femacs-pure 10/10`, `femacs-sys 3/3`).

## Retired Temporary & Verification Scopes

Retired `45` owned temporary build, verification, and container scopes totaling `23,532` files, `2,703` directories, `54` symlinks, and `1,486,480,381` logical bytes (plus removed empty parent directories `build/runtime` and `<session-cache>`):

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `build/runtime/u8-native` | `1012M` | `9,540` | `468` | `16` | `1,110,803,194` | `cargo run -p xtask -- runtime-build --variant native-collector --observer ...` |
| `build/cargo` | `75M` | `763` | `295` | `0` | `77,926,721` | `CARGO_TARGET_DIR` workspace incremental cache (rebuilt on demand) |
| `build/acceptance` | `26M` | `2,152` | `668` | `0` | `21,704,250` | Re-run `tools/test-u8-au8-integration.py` / `tools/runtime-execution.py` |
| `build/goal-identity-unit` + `build/goal-output-guard-unit` | `0` | `0` | `2` | `0` | `0` | Re-created by `tools/test-goal-stability.py` |
| `<session-cache>/u8-run` | `288M` | `11,033` | `1,098` | `36` | `273,665,017` | Re-run `tools/test-u8-au8-integration.py` groups |
| `<session-cache>/u1-baseline` | `616K` | `4` | `1` | `0` | `619,326` | Regenerate via `tools/check-unsafe-boundary.py` on baseline commit |
| `<session-cache>/tree-sitter-0.27.1` | `604K` | `5` | `8` | `2` | `607,255` | Re-build from pinned source in `build-support/toolchain-pins.json` |
| `<noindex>/f*` (`37` owned test containers) | `1.2M` | `35` | `149` | `0` | `1,154,618` | Owned `stable-suite.py` temporary containers; recreated on demand |

## Retained Working Set & Post-Cleanup Verification

- `build/terminal` (`45M`):
  - `build/terminal/port/src/emacs`: `4,475,248` B, SHA-256 `bf497bb5e380caf633325890127a757e3e6037a88d049a02ab19a922160a60cd`
  - `build/terminal/port/src/emacs.pdmp`: `13,101,352` B, SHA-256 `3e56fbf0bf454da5d83ebb348d9f2c6435fce62d48d7245a984c23c85985feab`
  - `build/terminal/port/lib-src/emacsclient`: `564,976` B, SHA-256 `383644deb3002212a39e15d2f8b893db39c5073831eb4e2906f5986d4c31662b`
  - `build/terminal/port/lib-src/hexl`: `475,072` B, SHA-256 `d44f1c449986013e4a8acef87036b8903ebb27b57442f093ef0165953103b840`
- `dist/macos/Femacs.app` (`235M`):
  - `Contents/MacOS/Femacs`: `5,998,256` B, SHA-256 `e5e702bf00b3ecd08f0abe46ec2d2b70dde78340b26ca040eda54685d13863c2`
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,601,728` B, SHA-256 `67a2ebadcc6709e799062833bff5c86be132f60b3fbb3f8394ddf21a2c4008d0`
  - `codesign --verify --deep --strict dist/macos/Femacs.app` -> `exit 0`
- Post-cleanup SHA-256 checks matched `clean-export` `published_deliverables` `4/4`, and `-Q --batch --eval '(princ (list (+ 1 2) (fboundp (quote treesit-available-p)) (and (fboundp (quote treesit-available-p)) (treesit-available-p)) (featurep (quote treesit)) (progn (require (quote treesit)) (featurep (quote treesit)))))'` returned `(3 t t nil t)` (`exit 0`) on both delivered executables.
