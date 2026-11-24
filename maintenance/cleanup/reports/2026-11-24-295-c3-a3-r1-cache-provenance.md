# CLN295 — Stage C3/A3-R1 Cache Canonical Layout, Provenance & Output Safety Retirement (2026-11-24)

## Summary

Closed all four review items from `[FEMACS-C3-A3-REVIEW]` on `df887492` / `b1d50708` without modifying the retained `df887492`-bound `dist/lisp` package: enforced exact canonical 1-to-1 `bin/` layout and complete top-level runtime resource tree coverage + full 64-hex `package_sha256` pinning in `tools/femacs-lisp-cache.py`, replaced blind `FEMACS_SOURCE_COMMIT` / `HEAD` trust with clean-Git-tree (`status --porcelain --untracked-files=normal`) or verified `.femacs-source-provenance.json` tree-hash validation and full staged resource-tree enumeration in `xtask/src/standalone.rs`, replaced destructive `shutil.rmtree` in `tools/test-c3-a3-seed-host.py` with non-destructive `claim_fresh_output_dir` after pre-checks, corrected `docs/status.md:13`, `CLN294:18`, and `maintenance/cleanup/LOG.md:3` records for `de0b10aa3cf34f5cbe30f0ed9488fd8c` (`exit=101`) vs `7c945e7c7c6b4c398f488ffdac5d563c` (`exit=0`), passed targeted Phase 2 & Phase 3 verification under controlled-run, retired `8` temporary scopes (`CLN295`), and verified post-cleanup smoke.

## C3/A3-R1 Fixes & Controlled Verification Receipts

- **Canonical cache layout, complete runtime resource coverage & full 64-hex SHA pinning (`tools/femacs-lisp-cache.py`)**:
  - Added `CLEAN_COMMIT_RE` (`40`-hex) and `SHA256_HEX_RE` (`64`-hex) validation on `source_commit`, `package_sha256`, and `expect_*` filters; rejects `dirty-*` and `unverified-export` commits with `unverified-source-commit`.
  - Enforced canonical `CANONICAL_BINARIES` (`bin/femacs-lisp`, `bin/femacs-lisp.pdmp`, `bin/http-host`), verified exact `bin/` directory membership (`unexpected-bin-entry`), and unified returned `lisp_bin`/`http_bin` paths through `_package_binary_paths` across `verify`, `ensure`, and `resolve`.
  - Enforced 1-to-1 equality between all top-level package directories (outside `bin` and `manifest.json`) and `resource_trees`, rejecting unverified extra `runtime-*` directories (`unverified-resource-tree`) and unexpected top-level files (`unexpected-package-entry`).
  - Changed `cache_entry_path` to full 64-hex `package_sha256` and made `--expect-package-sha256` required for `resolve` (`18` built-in `--selftest` checks).
- **Verified clean Git or hash-authenticated archive provenance & full staged resource hashing (`xtask/src/standalone.rs`)**:
  - Removed blind `FEMACS_SOURCE_COMMIT` env override; `resolve_source_commit` now returns 40-hex `HEAD` only when `.git` is clean under `git status --porcelain --untracked-files=normal` (otherwise `dirty-<HEAD>`), or `verify_archive_provenance` via `.femacs-source-provenance.json` `tree_sha256` verification (otherwise `"unverified-export"`).
  - Updated `lisp_package_in` to capture pre-mutation provenance and re-verify Git cleanliness before staging, and refactored `lisp_package_manifest_value_with_commit` to enforce exact 3 canonical binaries in `bin/` and hash every staged top-level resource directory into `resource_trees`.
  - Added `resolve_source_commit_requires_clean_git_tree_or_verified_archive` and expanded `lisp_package_manifest_records_binary_digests_without_host_paths`.
- **Safe `--out` handling & expanded negative checks (`tools/test-c3-a3-seed-host.py`, `maintenance/tooling-inventory.json`)**:
  - Replaced `shutil.rmtree` with `claim_fresh_output_dir`, validating `--reference-host`, `--eliscript-repo`, and `--package` (`verify` + strict `codesign`) before claiming a brand-new output directory.
  - Added full 64-hex `--expect-package-sha256` pinning on all `resolve` calls plus negative checks for wrong `package_sha256`, unverified `runtime-extra/`, spoofed `relative_path`, and existing `--out` preservation (`--selftest` + `run_id=ba4a58a8e46d4528868cc6d75c0956a9`).
- **Corrected historical Phase 3 logs (`docs/status.md`, `CLN294`, `maintenance/cleanup/LOG.md`)**:
  - Separated `run_id=de0b10aa3cf34f5cbe30f0ed9488fd8c` (`exit=101`: linked & passed `femacs_host_unit_tests` `10/10`, then `cargo test -p xtask --lib` failed with no library target) from `run_id=7c945e7c7c6b4c398f488ffdac5d563c` (`19.999s`, `exit=0`: `femacs_host_unit_tests` `10/10`, `cargo test -p xtask` `118/118`, `cargo test -p http-host` `9/9`).
- **Controlled Verification Runs (`assistant run`)**:
  - Phase 2 runtime acceptance & baseline gates (`run_id=ba4a58a8e46d4528868cc6d75c0956a9`, `140.991s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp-cache.py --selftest` (`18` checks PASS), `test-c3-a3-seed-host.py --selftest` (`PASS`), `femacs-lisp-cache.py verify --package dist/lisp` (`verified`, `df887492...`, `6b4d8393...`), `test-c3-a3-seed-host.py` on `eliscript@db9fd8ae3656` (Seed `25` files / `1,431,679` B / `0` diffs; Stage 0 `131` files / `15,118,877` B / `0` diffs; full-SHA cache lifecycle + negative rejections; second invocation on existing `--out` rejected while preserving `report.json`), `test-tooling-modernization.py` (`33/33`), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,331/6,331` `SAFETY:`, `0` errors).
  - Phase 3 protective unit tests (`run_id=b85c86fa103048afa3de221cb5b62902`, `25.028s`, `exit=0`, `cleanup_complete=true`): `FEMACS_SOURCE_COMMIT=0000...0000 cargo test -p xtask` (`119/119` passed) + `test-c2-a2-lisp-app.py --package dist/lisp` (`PASS`).
  - Post-cleanup smoke (`run_id=0ffe170018a349dbad134c59c0ae579d`, `8.458s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp-cache.py --selftest` (`18` checks PASS), `femacs-lisp-cache.py verify --package dist/lisp --expect-commit df887492... --expect-package-sha256 6b4d8393...` (`verified`), strict `codesign` on `femacs-lisp` & `http-host` (`exit 0`), `femacs-lisp --batch -Q --eval "(princ (+ 40 2))"` (`42`), `test-c3-a3-seed-host.py --selftest` (`PASS`), `test-tooling-modernization.py` (`33/33`), `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`).

## Retired Temporary Scopes

Retired `8` owned temporary Cargo target, acceptance, and empty `xtask` test scratch scopes totaling `18,829` files, `987` directories, `8` symlinks, `973,858,470` logical bytes, normalized digest `e3ee96499276196b8f73bf881d0c6548d8ed4720faa7ea784a4f4e40c676e758`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `180M` | `1,310` | `282` | `0` | `213,091,597` | Root Cargo target cache (`cargo test -p xtask`) |
| `build/acceptance` | `764M` | `17,519` | `705` | `8` | `760,766,873` | Recreated by `tools/test-c3-a3-seed-host.py` on a fresh `--out` |
| `build/android-tests` | `0` | `0` | `0` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/bundle-tests` | `0` | `0` | `0` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/manuals-review` | `0` | `0` | `0` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/package-contracts` | `0` | `0` | `0` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/probes-tests` | `0` | `0` | `0` | `0` | `0` | Empty `xtask` unit test scratch dir |
| `build/tooling-tests` | `0` | `0` | `0` | `0` | `0` | Empty `xtask` unit test scratch dir |

## Retained Working Set & Package Verification

- `dist/lisp` (`210M`, bound to `source_commit=df88749200cbee2de1f679f4f02ba8c51239e433`, `package_sha256=6b4d8393e9f487982f4028d518d9a94d95e9dc4e382303ba693575edc54dd82e`):
  - `bin/femacs-lisp`: `5,091,904` B, SHA-256 `c1fedf3734dcb5c02c2840065a4d6a46ea4e6eb7a56607a343bc4c502cd625b1` (`codesign --verify --deep --strict` `exit 0`)
  - `bin/femacs-lisp.pdmp`: `18,325,968` B, SHA-256 `8542d568c8dccf237d9b1a700db6eada758a74b4032bd09bb461d8aa5bf71cfd`
  - `bin/http-host`: `2,145,728` B, SHA-256 `31f438b7ca7b102dff78545a03a128d6305f13732f7714b2e1ad3035013a5c00` (`codesign --verify --deep --strict` `exit 0`)
- Unchanged retained sets: `build/terminal` (`45M`), `dist/macos` (`235M`, parity-matched with `/Applications/Fe.app`), `build/generation` (`15M`), and `build/manuals` (`1.4G`) (`build` `1.4G`, `dist` `445M`).
