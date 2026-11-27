# CLN307 — `femacs-pure` Dissolution & `fe-test` Native Framework Closure (2026-11-27)

## Summary

Completed architectural dissolution of `crates/femacs-pure` (`0086066a`, `PORTING.md` `P-489` / `AGENTS.md` §2 `P-102`) and delivered the first closed loop of `fe-test` (pure `femacs-lisp` LPT/memory-gated/cached test runner under `tools/testing/` + `#![forbid(unsafe_code)]` `xtask test-exec` OS supervision primitives), passed full cold/warm integration runs (`38/38`), Rust (`125` `xtask` + `12` `femacs-sys` + `9` `http-host`) & Elisp (`6/6` self-test) protective suites, retired `3` temporary build/test scopes (`5,407` files, `992` dirs, `2` symlinks, `1,267,413,582` logical bytes), and verified all retained products post-cleanup.

## Controlled Post-Cleanup Verification (`run_id=440f00aa40634421ba92857154339a17`, `18.849s`, `exit=0`, `cleanup_complete=true`)

- `build/` directory is completely empty and `tools/__pycache__` is absent.
- `diff -rq dist/macos/Femacs.app /Applications/Fe.app` is empty and strict `codesign --verify --deep --strict /Applications/Fe.app` passed.
- Standalone `dist/lisp/bin/femacs-lisp` evaluated `42`; `/Applications/Fe.app/Contents/MacOS/Femacs --batch -Q` evaluated `(42 t t nil t)` (`(+ 20 22)`, `treesit-available-p`, `(require 'treesit)`, non-X, `secure-hash 'sha256`); `/Applications/Fe.app/Contents/MacOS/bin/emacsclient --version` returned `emacsclient 32.0.50`.
- Static and repository gates passed: `tools/check-unsafe-boundary.py --manifest tools/unsafe-boundary-manifest.json --require-zero-static-mut --require-zero-active-stitching --require-100pct-safety-docs --require-no-file-allows --min-forbid-files 151` (`error_count=0`), `tools/check-ledger-ticks.py` (`0` failures), `tools/check-own-voice.py` (`0` failures), `tools/test-repository-bootstrap.py` (`4/4`), and `tools/test-tooling-modernization.py` (`33/33`).

## Retired Temporary Scopes

Retired `3` owned temporary Cargo target, framework state, and unit-test scratch scopes totaling `5,407` files, `992` dirs, `2` symlinks, and `1,267,413,582` logical bytes:

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/cargo` | `5,348` | `880` | `0` | `1,267,394,788` | `464f26fe3b40c42841a78c5f091a798eb400d9a83836991f0bbcd98b95be8145` | Rebuild on demand via `cargo build -p xtask --target-dir build/cargo` |
| `build/test-framework` | `37` | `3` | `0` | `18,359` | `045f3d364f84073268a5de44f8712d4e6c7f58ffc7cf1d8be8a859de28696559` | Re-created on demand by `dist/lisp/bin/femacs-lisp --batch -Q -L tools/testing -l fe-test.el` |
| `build/{acceptance,android-tests,bundle-tests,manuals-review,package-contracts,probes-tests,test-exec-unit-tmp,tooling-tests}` | `22` | `109` | `2` | `435` | `01a8ae73688c2e82613debb5d3a64a348907792155d6213bceb20d3437073c69` | Ephemeral unit-test scratch recreated on demand by `cargo test -p xtask` |

## Retained Working Set

- `dist/macos/Femacs.app` (`235M`), `/Applications/Fe.app` (`235M`), and `dist/lisp` (`210M`). (`build/terminal` was absent prior to this round and left unbuilt.)
