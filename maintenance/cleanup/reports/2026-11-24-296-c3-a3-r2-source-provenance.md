# CLN296 — Stage C3/A3-R2 Source Provenance Closure & Cache Retirement (2026-11-24)

## Summary

Closed the remaining `[FEMACS-C3-A3-R1-REVIEW]` source provenance blocker in `xtask/src/standalone.rs` without disturbing the retained `df887492`-bound `dist/lisp` package or Seed/Stage 0 parity results: removed the unauthenticated `.femacs-source-provenance.json` sidecar branch so all non-Git archives deterministically build with `source_commit="unverified-export"`, added `finalize_source_commit` to reject mid-build `HEAD` switches (`drifted-<initial>-<final>`) and dirtiness so official cache entries accept only single-revision clean Git builds, replaced the unit test with `resolve_source_commit_requires_clean_git_tree_and_rejects_head_drift`, verified under controlled-run (`119/119` `cargo test -p xtask` with `FEMACS_SOURCE_COMMIT` set), retired `8` temporary scopes (`CLN296`), and passed post-cleanup smoke.

## C3/A3-R2 Provenance Fix & Controlled Verification Receipts

- **Source Provenance Removal & Drift Rejection (`xtask/src/standalone.rs`)**:
  - Removed `ARCHIVE_PROVENANCE_FILE`, `ARCHIVE_EXCLUDED_TOP_ENTRIES`, `hash_archive_source_tree`, and `verify_archive_provenance`; simplified `hash_resource_tree` to hash resource directories directly; any non-Git archive now unconditionally resolves to `"unverified-export"` (which `tools/femacs-lisp-cache.py` rejects from official cache entry with `unverified-source-commit`).
  - Added `finalize_source_commit(initial, final_resolved)` and wired it in `lisp_package_in`, preserving initial non-clean tags or flagging mid-build commit changes as `drifted-<initial>-<final>`.
  - Updated unit coverage to `resolve_source_commit_requires_clean_git_tree_and_rejects_head_drift` covering clean Git, modified tracked file, untracked file, clean second-commit drift (`head` -> `head2`), and non-Git archive with forged sidecar.
- **Controlled Verification Runs (`assistant run`)**:
  - Pre-cleanup verification (`run_id=ce60b2b9263146e4a78c80c1a96d6e42`, `15.698s`, `exit=0`, `cleanup_complete=true`): `FEMACS_SOURCE_COMMIT=0000...0000 cargo test -p xtask` (`119/119`), `femacs-lisp-cache.py --selftest` (`18` checks PASS), `femacs-lisp-cache.py verify --package dist/lisp --expect-commit df887492... --expect-package-sha256 6b4d8393...` (`verified`), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`), `test-repository-bootstrap.py` (`4/4`).
  - Post-cleanup smoke (`run_id=d2001783013744bea29fa1847d5a61da`, `8.211s`, `exit=0`, `cleanup_complete=true`): `femacs-lisp-cache.py --selftest` (`18` checks PASS), `verify --package dist/lisp` (`verified`), strict `codesign` on `femacs-lisp` & `http-host` (`exit 0`), `check-own-voice.py` (`0` failures), `test-tooling-modernization.py` (`33/33`), `test-repository-bootstrap.py` (`4/4`).

## Retired Temporary Scopes

Retired `8` owned temporary Cargo target, acceptance scratch, and empty `xtask` test scratch scopes totaling `1,332` files, `383` directories, `2` symlinks, `212,949,468` logical bytes, normalized digest `ec8e5d176aae88bab39709e1b8cf42897d934f46df16001788dfba144892931f`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `181M` | `1,310` | `282` | `0` | `212,949,033` | Root Cargo target cache (`cargo test -p xtask`) |
| `build/acceptance` | `88K` | `22` | `101` | `2` | `435` | Recreated by `xtask` unit test scratch helpers |
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
