# CLN302 — Stage C5-R2 Read-Only Guard & Stage C6/A6 Node ESM Consumer Verification (2026-11-25)

## Summary

Closed the final **C5-R2** read-only `--frozen-package` guard from `[FEMACS-C5-A5-R1-REVIEW]` in `724b118c`, and delivered FEmacs-side **Stage C6 / A6** (`f467da6f`) verifying external `node_modules/eliscript` consumption under Bun-free `node v26.13.1` (`08bfa72a…`) against the exact same verified `209`-file `db9fd8ae3656c0671a3c6f89acfc554b736334e2` closure (`closure_sha256=4456a330002e28f16fb40fe3569e35bedd29e8b7f9830d4559957fa649e4c33b`). Zero edits were made to the core `lisp/` tree or the sibling Eliscript repository, and neither `Fe.app`, `build/terminal`, nor `dist/lisp` was rebuilt.

## Changes & Controlled Verification Receipts

- **C5-R2 Read-Only Output Containment (`724b118c`, `tools/test-c5-a5-worker.py`, `maintenance/tooling-inventory.json`)**:
  - Rejected `out_dir == pkg_dir` or `out_dir.is_relative_to(pkg_dir)` immediately after verifying `--frozen-package` before any directory creation or editor process launch, added `refuse_out_inside_frozen_package` (`9/9` guards pass cleanly in `run_id=b866349ae69142e380f0efda4ddc519c`, `1.055s`, `exit=0`), and added `extra_env` support to `run_cocoa_gui_cases` / `run_terminal_pty_cases`. Prior `11/11` Bun Cocoa/Terminal receipts remain valid.
- **Stage C6/A6 Client Probe & Node External ESM Consumer Harness (`f467da6f`, `examples/eliscript-worker/{femacs-eliscript-worker.el,README.md}`, `tools/fixtures/c5-a5-worker-cases.el`, `tools/test-c6-a6-node.py`, `maintenance/tooling-inventory.json`)**:
  - `examples/eliscript-worker/femacs-eliscript-worker.el`: added `femacs-eliscript-probe-runtime-resolution` and `femacs-eliscript-runtime-resolution-probe` to compare advertised `runtime-resolution` capability against live probe execution on modules relocated outside any `node_modules` ancestor without terminating the running worker session.
  - `tools/test-c6-a6-node.py` & `tools/fixtures/c5-a5-worker-cases.el`: constructed a Bun-free `PATH` (`node v26.13.1` only), staged an external ESM consumer project (`node_modules/eliscript` verbatim copy verified at `closure_sha256=4456a330…`) plus a separate ancestor-less relocated probe tree, ran `node` direct ESM import of `eliscript/runtime/lisp` (`evalProgram("(+ 20 22)")` -> `42`, `createSession`/`evalSession` -> `"42"`), and drove all `12/12` shared acceptance cases in Cocoa `/Applications/Fe.app` and Terminal PTY.
- **Controlled Execution Receipts**:
  - Guard self-tests (`run_id=a3a33081ffd34d7d9238e1fa339b74be`, `1.565s`, `exit=0`, `cleanup_complete=true`): `test-c5-a5-worker.py --self-test-guards` (`9/9`) and `test-c6-a6-node.py --self-test-guards` (`4/4`: `refuse_out_inside_frozen_package`, `refuse_existing_out_dir`, `reject_tampered_frozen_closure`, `verify_bun_free_path`).
  - Full `C6/A6` Bun-free `node v26.13.1` acceptance (`run_id=26256368f7d94e2f8922b01b382f1e66`, `8.72s`, `exit=0`, `cleanup_complete=true`):
    - `standard_esm_lisp_import` (`eliscript/runtime/lisp`): `passed=true`, `v1=42`, `v2="42"`, `printed="42"`.
    - Cocoa `/Applications/Fe.app` (`framep=ns`, `3.0926s`, `pid=43831`, `binary_sha256=8bfe40f328df575f99b220dad91687a32f736c43ab154f93bfe050cfb8cad03a`): `12/12` cases passed (`mapped_source_error` mapped `c5-probes.eli:31:12`, `session_cwd_isolation` `gen_after_auto_relaunch=3`, `relocated_runtime_resolution_probe` confirmed structured `ERR_MODULE_NOT_FOUND` with live worker follow-up).
    - Terminal PTY `build/terminal/port/src/emacs` (`framep=t`, `5.0008s`, `pid=53145`, `binary_sha256=67c6d379fc73a611cb829b90e1d231b6d41b091f131add5e481b3be2550f7d77`): `12/12` cases passed cleanly.
    - Observed upstream `db9fd8ae` `runtime/worker.mjs` capability-honesty gap: `ready` advertises `"runtime-resolution"` unconditionally under `node v26.13.1` (`advertised=true`) even though `installPackageResolver` only registers `Bun.plugin` and relocated modules outside `node_modules` fail with `ERR_MODULE_NOT_FOUND` (`supported=false`, `honest=false`), tripping `femacs-eliscript-require-capabilities` (`unsupported-capability`).
  - Strict `--require-capability-honesty` negative gate (`run_id=0f56e80c79044846a6ef899dd95efcbe`, `8.725s`, `exit=0`, `cleanup_complete=true`): `HONESTY_GATE_RC=1` confirmed non-zero rejection while `honest=false` on `db9fd8ae`.
  - Post-cleanup verification (`run_id=d729a925ba7b4537ba39804a034b1ca3`, `12.302s`, `exit=0`, `cleanup_complete=true`): `test-c5-a5-worker.py --self-test-guards` (`9/9`), `test-c6-a6-node.py --self-test-guards` (`4/4`), empty `diff -rq dist/macos/Femacs.app /Applications/Fe.app`, strict `codesign` valid on both bundles and `dist/lisp/bin/{femacs-lisp,http-host}`, `femacs-lisp-cache.py verify --expect-commit 9ef6e40f2e749980c18f98a0aba74884037a4a97` (`490340fd…`), `(42 t t nil nil 0)` on both editors, `emacsclient 32.0.50`, `check-own-voice.py` `0` failures, `test-repository-bootstrap.py` `4/4`, `test-tooling-modernization.py` `33/33`, and zero `tools/__pycache__`.

## Retired Temporary Scopes

Retired `2` owned `C6/A6` acceptance report scratch directories totaling `2` files, `0` dirs, `0` symlinks, `16,342` logical bytes (`tools/__pycache__` already absent):

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Scope SHA-256 | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `build/acceptance/c6-a6-node` | `8.0K` | `1` | `0` | `0` | `8,171` | `e9f14d4e2e039ff6fbcc2eb059b4b67d12ae5b9c1da6bff705c8c98100203506` | Regenerated by `tools/test-c6-a6-node.py --frozen-package build/acceptance/frozen-eliscript-db9fd8ae --out build/acceptance/c6-a6-node` |
| `build/acceptance/c6-a6-honesty-gate` | `8.0K` | `1` | `0` | `0` | `8,171` | `a30f0070babbd15fc3d2f257d2ad0ede1ffccdb803f9d5b596cee89251698b1a` | Regenerated by `tools/test-c6-a6-node.py --frozen-package build/acceptance/frozen-eliscript-db9fd8ae --require-capability-honesty --out build/acceptance/c6-a6-honesty-gate` |

## Retained Working Set

- `build/acceptance/frozen-eliscript-db9fd8ae` (`6.5M`, `209` files, `closure_sha256=4456a330002e28f16fb40fe3569e35bedd29e8b7f9830d4559957fa649e4c33b`): explicitly retained temporary verified package while peer review on `C5-R2 + C6` completes and upstream `eliscript` C3–C6 closure lands; scheduled for retirement on post-review / `A7` cleanup.
- `build/terminal` (`48M`), `dist/macos/Femacs.app` (`235M`), `/Applications/Fe.app` (`235M`), `dist/lisp` (`210M`), `build/generation` (`17M`), and `build/manuals` (`1.4G`).
