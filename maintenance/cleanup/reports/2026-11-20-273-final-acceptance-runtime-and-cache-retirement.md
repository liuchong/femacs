# CLN273 — Final retirement of consumed `build/acceptance`, `build/runtime`, `.femacs-cache`, and staging backups

After completing the formal `2+7` runtime comparison suites (`s3-g5-comparison-formal`, `s3-ga8-delivered-formal`), the paired 7-round `bench.py` suites (`s3-ga8-bench-cocoa`, `s3-ga8-bench-terminal`), and archiving all round-by-round series, statistics, pause/phase/survival observations, SHA-256 digests, and the item-by-item `GA1–GA8` reverse audit in [Stage III closure](../releases/2026-11-20-stage3-ga5-ga6-ga7-ga8-closure.md), all remaining temporary acceptance trees, experimental runtime cohorts, `.femacs-cache` directories, superseded `.app` backups, terminal image-staging directories, and hardlinked `P-190` numbered dump aliases completed their consumers.

Removed `159,109` ordinary files, `21,542` directories, and `1,843` symbolic links across `8` owned scopes totaling `10,763,439,608` logical bytes (normalized scope digest `a3a6af215b15d77a519e8e686e38ae429f879f1b38f957e49ee057b20c56bfe7`):

| Scope | Files | Dirs | Symlinks | Logical Bytes | Tree SHA-256 | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/acceptance` | `111,451` | `19,313` | `1,814` | `7,934,226,116` | `e7892e4aae5dde7333e1db3b5f1b189c85652147875d122b4c65d6f2c175a81b` | `maintenance/releases/2026-11-20-stage{1-ga1-ga2-ga3,3-ga5-ga6-ga7-ga8}-closure.md`; re-run documented `tools/*.py` entrypoints |
| `build/runtime` | `39,774` | `1,593` | `28` | `2,504,897,262` | `5e73c9ad2dc22bf1e0e4f135f270844309c5d04d38d5d717147a349115f423c9` | `maintenance/releases/2026-11-20-stage3-ga5-ga6-ga7-ga8-closure.md`; `cargo run --release -p xtask -- runtime-build --product terminal --variant {native,collector} --observer` |
| `.femacs-cache` | `3,391` | `519` | `0` | `76,747,743` | `019fda4f903fc6e3bbbfd0003dd95bd4318430bb7b6e50790727ee77f28266af` | `python3 tools/test-keyboard-platform-offset.py` & `python3 tools/functional-corpus.py` |
| `dist/macos/.Femacs-before-p466.app` | `4,486` | `113` | `1` | `229,128,420` | `e4c1016049e64e80803d7cf4b82bcf3dd7fa250d6874b5ea9060b9ad29289f32` | `dist/macos/Femacs.app`; `cargo run --release -p xtask -- build-macos` |
| `build/terminal/image` | `3` | `3` | `0` | `1,270,503` | `414dcd5f8ea7ea005ce992385e02e000bc39e0cfad77831a349e0a2fc0bdc9a7` | `build/terminal/port/runtime-image`; `cargo run --release -p xtask -- build-terminal` |
| `build/terminal/image-staging` | `2` | `1` | `0` | `1,269,940` | `0936430bee818bcbec93a9b5086a50453c5a22e6728c02c21f72a49c9a723fbf` | `build/terminal/port/runtime-image`; `cargo run --release -p xtask -- build-terminal` |
| `build/terminal/port/src/emacs-32.0.50.1` | `1` | `0` | `0` | `2,798,256` | `df40439f422a9ef9f3fbdb77060e628c5629489fcfb4c1504dbae637ae24d02e` | `build/terminal/port/src/emacs` (`a0dd02e1…`) |
| `build/terminal/port/src/emacs-32.0.50.1.pdmp` | `1` | `0` | `0` | `13,101,368` | `0536cd647a00196d6a661c57cf6bc213beb2547c4795f4d5de9675f450436c51` | `build/terminal/port/src/emacs.pdmp` (`cd076b1d…`) |

Note on hardlinked executable mode: unlinking `emacs-32.0.50.1` after `chmod 0644` cleared the execute bit on the shared inode with `build/terminal/port/src/emacs`; `chmod 0755 build/terminal/port/src/emacs` immediately restored `+x` without changing file bytes (`shasum -a 256 -c pair.sha256` passed `4/4`).

Post-cleanup retained local workspace footprint and verification:
- `build/terminal` (`40 MB`, solely `build/terminal/port`): `emacs` (`a0dd02e1…`) + `emacs.pdmp` (`cd076b1d…`) verified via `pair.sha256` and `-Q --batch` (`(32.0.50 42)`).
- `dist/macos` (`231 MB`, solely `dist/macos/Femacs.app`): `Contents/MacOS/Femacs` (`02f5adaf…`) + `Femacs.pdmp` (`e1515db2…`), `codesign --verify --deep --strict` `exit 0`, and `-Q --batch` (`(32.0.50 t t)`).
