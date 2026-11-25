# CLN306 — Phase I Joint A7 Acceptance & Final Temporary Artifact Retirement (2026-11-25)

## Summary

Following dual independent approval of Joint A7 from FEmacs Codex (`01a144ea`, `[FEMACS-JOINT-A7-R1-REVIEW]`: verified `3cc7d060` / `ceb643e5` GPG signatures and remotes, `attempt-MwaM6L` install receipt `passed`, three-way SHA-256 `5ef7383a29ccc2c159d6ec94b25d4fe70632abc7f1e4e0a53745daf313296816`, `206`-file closure `339aa139…`, and pristine `exit=0` receipts) and Eliscript Codex (`01a1030d`, turn `01a2110b`: approved `ceb643e5` fixes and joint A7 tarball evidence, explicitly scoped to the signed isolated candidate and local tarball with `0.1.4` merge and npm publish handled separately), retired the two temporarily retained A7 verification directories, re-verified all kept products, and closed Phase I (`U1–U8`/`AU1–AU8`, `C1–C6`, `A1–A7`; `C7` Phase II remains deferred).

## Controlled Post-Cleanup Verification (`run_id=2d8522e13e1a4d9f9500406d1a92d9c3`, `10.707s`, `exit=0`, `cleanup_complete=true`)

- `build/acceptance` and `tools/__pycache__` verified absent.
- `diff -rq dist/macos/Femacs.app /Applications/Fe.app` empty; strict `codesign --verify --deep --strict` passed on `/Applications/Fe.app` & `dist/lisp/bin/femacs-lisp` and `--strict` passed on `build/terminal/port/src/emacs`.
- `tools/femacs-lisp-cache.py verify --package dist/lisp --expect-commit 9ef6e40f2e749980c18f98a0aba74884037a4a97` passed (`490340fd8efa6de351c8097fb8ee7afd2a30ad74e0e7f3c0dd90e474b26159af`).
- Batch execution smoke passed on all maintained targets: `/Applications/Fe.app/Contents/MacOS/Femacs` `(42 t t nil nil 0)`, `build/terminal/port/src/emacs` `(42 t t nil nil 0)`, `dist/lisp/bin/femacs-lisp` `42`, and `/Applications/Fe.app/Contents/MacOS/bin/emacsclient --version` `32.0.50`.
- Static and repository gates passed: `tools/check-own-voice.py` (`0` failures), `tools/test-repository-bootstrap.py` (`4/4`), `tools/test-tooling-modernization.py` (`33/33`), and `tools/__pycache__` absent.

## Retired Temporary Scopes

Retired `2` owned temporary joint-A7 review directories totaling `210` files, `18` dirs, `0` symlinks, `11,899,722` logical bytes (`build/acceptance` and `tools/__pycache__` absent):

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| `build/acceptance/eliscript-formal-tarball` | `5.4M` | `3` | `0` | `0` | `5,634,307` | `770c92ef8e224364f5db09e811b12be373ecde2d5fc8c7bbfc88a42d853fffc9` | Regenerated from `origin/femacs-phase1 @ ceb643e5` via cached `dist/lisp/bin/femacs-lisp` & `make check-release-package` |
| `build/acceptance/frozen-eliscript-ceb643e5` | `6.5M` | `207` | `18` | `0` | `6,265,415` | `441b9c3288224364f5db09e811b12be373ecde2d5fc8c7bbfc88a42d853cbe33` | Regenerated via `tools/test-c5-a5-worker.py --eliscript-tarball … --eliscript-tarball-commit ceb643e5ae42e939310aa55e980717ae78ec8687` |

## Retained Working Set

- `build/terminal` (`45M`), `dist/macos/Femacs.app` (`235M`), `/Applications/Fe.app` (`235M`), `dist/lisp` (`210M`), `build/generation` (`17M`), and `build/manuals` (`1.4G`).
