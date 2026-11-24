# CLN294 — Stage C3/A3 Seed & Stage 0 Host Support, Verifiable Package Manifest/Cache & Cache Retirement (2026-11-24)

## Summary

Following Codex App `[FEMACS-C2-A2-R3-REVIEW]` approval on `73a39658` unlocking Stage `C3/A3`, implemented FEmacs-side `C3/A3` support in commit `df887492` (`batch` `command-line-args-left` propagation + re-parsing for `-l`/`-e`, `command-line-1` relative `-l`/`-L` path resolution and left-to-right `-L` ordering, verifiable `dist/lisp/manifest.json` with `source_commit`/`build_config`/`resource_trees`/`package_sha256`, and `tools/femacs-lisp-cache.py` + `tools/test-c3-a3-seed-host.py`), rebuilt `dist/lisp` cleanly, passed all Phase 2 integration/parity gates (`run_id=5b9680b10a8745958fe2f8487ff59721`), linked (`run_id=de0b10aa3cf34f5cbe30f0ed9488fd8c`) and passed all Phase 3 protective unit tests (`run_id=7c945e7c7c6b4c398f488ffdac5d563c`), rebuilt final `dist/lisp` bound to `source_commit=df887492` (`run_id=f554a61f8bfc442e9d14b00edd1bc0d0`), retired `11` temporary scopes (`CLN294`), and passed post-cleanup smoke & Seed/Stage 0/cache parity (`run_id=991f9148fc4142a4bdaf98d4da9f4b5a`). Downstream `eliscript` repository working tree was never modified; all downstream checks ran against isolated clean `git archive HEAD` snapshots (`db9fd8ae3656`).

## C3/A3 FEmacs Implementation & Verification Receipts

- **Reference-Matching Batch CLI & Path Resolution (`crates/femacs-vm/src/host_runtime.rs`, `crates/femacs-host/src/{lib,lisp_cli,bin/femacs_lisp}.rs`)**:
  - Added `HeadlessVm::{eval_with_args, load_library_with_args, funcall_with_args_left}` (exposed safely on `#![forbid(unsafe_code)]` `LispHost`), reading back mutated `command-line-args-left` cleanly inside `UnbindGuard` & `ScopedLispRoot` bounds.
  - Added `BatchAction::ExtraArg(OsString)`, `actions_to_argv`, and `parse_remaining_batch_actions` so `femacs-lisp --batch -Q -l tools/backends/bootstrap.el --stage0 --output <dir>` passes trailing flags in `command-line-args-left`, re-parses remaining arguments, and preserves exit code `2` on unconsumed or pre-consumer invalid flags.
  - Matched `command-line-1` path semantics: `-l` resolves regular files relative to `default-directory` (`expand-file-name` + `file-truename` + `file-regular-p`) before `load-path` fallback; `-L` expands paths relative to `default-directory` and preserves left-to-right order across consecutive `-L` flags.
- **Verifiable `manifest.json` & Host Cache (`xtask/src/standalone.rs`, `tools/femacs-lisp-cache.py`, `tools/test-c3-a3-seed-host.py`, `maintenance/tooling-inventory.json`)**:
  - Upgraded `lisp_package_manifest_value` with `source_commit`, `build_config`, `resource_trees` (`lisp`, `etc`, `runtime-*`, `licenses`), and canonical `package_sha256`, and separated read-only `codesign_verify_strict` in staging validation.
  - Implemented `tools/femacs-lisp-cache.py` (`verify`, `ensure`, `resolve`, `--selftest` 11 checks) and `tools/test-c3-a3-seed-host.py` covering clean `eliscript@db9fd8ae` dual-host byte parity, diagnostics, Stage 0 (`--stage0 --output`), Makefile hooks, and negative tamper/platform/commit rejections.
- **Controlled Verification Runs (`assistant run`)**:
  - Pre-commit rebuild & Phase 2 runtime acceptance (`run_id=59fb1c522cea4104a54f51831fad076f`, `184.113s`, `exit=0`; `run_id=5b9680b10a8745958fe2f8487ff59721`, `142.916s`, `exit=0`): `femacs-lisp-cache.py --selftest` (`PASS`, 11 checks), `test-c3-a3-seed-host.py --selftest` (`PASS`) & full acceptance (Seed `25` files / `1,431,679` B / `0` diffs, Stage 0 `131` files / `15,118,877` B / `0` diffs, `eliscript-clean` `exit 0`, `strict-byte-compile.sh` `exit 0`, cache `rebuilt`/`hit`/rejections), `test-c2-a2-lisp-app.py --selftest` & `--package dist/lisp` (`PASS`), `test-repository-bootstrap.py` (`4/4`), `check-unsafe-boundary.py` (`442` active files, `149` `forbid`, `6,331/6,331` `100.0%` `SAFETY:`, `0` errors), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`).
  - Phase 3 protective unit tests (`run_id=de0b10aa3cf34f5cbe30f0ed9488fd8c`, `exit=0`; `run_id=7c945e7c7c6b4c398f488ffdac5d563c`, `19.999s`, `exit=0`): `femacs_host_unit_tests` (`10/10`), `cargo test -p xtask` (`118/118`), `cargo test -p http-host` (`9/9`).
  - Final `df887492`-bound `dist/lisp` rebuild (`run_id=f554a61f8bfc442e9d14b00edd1bc0d0`, `190.045s`, `exit=0`, `cleanup_complete=true`).
  - Post-cleanup smoke on `df887492` `dist/lisp` (`run_id=991f9148fc4142a4bdaf98d4da9f4b5a`, `127.563s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp 0.1.0 (femacs 32.0.50)`, `femacs-lisp-cache.py verify --expect-commit df887492...` (`verified`, `package_sha256=6b4d8393e9f487982f4028d518d9a94d95e9dc4e382303ba693575edc54dd82e`), Seed `25` files / `1,431,679` B (`0` diffs), diagnostics (`fixture/bad-syntax.eli:3:3: unexpected end of input` + `fixture/bad-analyzer.eli:3:8: duplicate local binding: dup`, `exit 1`), Stage 0 `131` files / `15,118,877` B (`0` diffs, `formalSourceIdentity=e20e1f37768098e99b64496b90646b918679c71fc899ecf2ba6f515993a86a1c`, `productIdentity=d47135b4e50b92235fbf6b68c190efa83dacd29068d5dc5412d676b8f0f28ef6`, `stage0.json=c03a16ce…`), `strict-byte-compile.sh` (`exit 0`), `test-tooling-modernization.py` (`33/33`), `check-own-voice.py` (`0` failures).

## Retired Temporary Scopes

Retired `11` owned temporary build, cache, pin, acceptance, and empty test scratch scopes totaling `17,595` files, `1,620` directories, `16` symlinks, `1,484,804,183` logical bytes, normalized digest `88caebbd1c071f0dc1861bc72c3a9919fd6b198c957c78e20fdb3ab057c0d132`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `457M` | `3,574` | `521` | `0` | `583,230,699` | Root Cargo target cache (`cargo test` / `xtask`) |
| `build/cargo-tools` | `16M` | `55` | `29` | `0` | `15,843,435` | `xtask` host tool target cache |
| `build/lisp` | `335M` | `858` | `468` | `6` | `334,038,271` | Recreated by `xtask build-lisp --out dist/lisp` |
| `build/treesit-0.27.1` | `604K` | `5` | `7` | `2` | `607,238` | Reinstall via `python3 tools/toolchain-pins.py install` |
| `build/acceptance` | `554M` | `13,103` | `595` | `8` | `551,084,540` | Recreated by `tools/test-c3-a3-seed-host.py` |
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
- Unchanged retained sets: `build/terminal` (`45M`), `dist/macos` (`235M`, parity-matched with `/Applications/Fe.app`), `build/generation` (`15M`), and `build/manuals` (`1.4G`) (`build` `1.5G`, `dist` `445M`).
