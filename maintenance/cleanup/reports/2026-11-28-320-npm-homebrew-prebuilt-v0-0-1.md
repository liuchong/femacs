# CLN320 — Multi-Platform `v0.0.1` Prebuilt Archives, `femacs@0.0.1` npm (Eliscript), `liuchong/homebrew-tap`, `install.sh` & Unified `xtask release` (2026-11-28)

## Summary

Completed multi-channel binary distribution for `v0.0.1` while keeping the signed `v0.0.1` GitHub tag pinned at `874a28d0`: fixed `NEAR_STACK_TOP!` in `crates/lisp-vm` for non-`aarch64` architectures, cross-compiled and uploaded 4 Unix release archives (`aarch64-apple-darwin`, `x86_64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`) + `SHA256SUMS.txt` onto the existing `v0.0.1` GitHub Release, published zero-dependency `femacs@0.0.1` to npm compiled from 100% Eliscript (`.eli`) source (`Method 2` `prepack` `bun build`), created and pushed GPG-signed `liuchong/homebrew-tap` (`Formula/femacs.rb`), shipped `docs/install.sh` (`https://femacs.com/install.sh`), integrated all channels into `xtask release` (`v0.0.2+` single-tag flow), refreshed docs & GitHub Pages (`eliscript@0.1.4`), retired all transient build/npm artifacts, and verified all 6 static gates cleanly.

## Changes

- **Portable `NEAR_STACK_TOP!` & 4-Target Release Archives (`crates/lisp-vm/src/ported/src/include/thread.rs`)**:
  - Preserved `#[cfg(target_arch = "aarch64")]` inline asm (`mov {}, x29`) and added `#[cfg(not(target_arch = "aarch64"))]` fallback `($addr) as *const c_void` (matching C `thread.h`).
  - Built & packed `--release -p elisp -p femacs` archives (`elisp`, `femacs`, `fe`, `LICENSE`) + `SHA256SUMS.txt` and uploaded to existing `v0.0.1` Release (tag `v0.0.1` kept at `874a28d0d9a2010f1b4b67e8c907177486f2e845`):
    - `femacs-v0.0.1-aarch64-apple-darwin.tar.gz` (`9,318,982` B, sha256 `f42a8fddfe5fbd81b90c5146c7824cdd27c7b4af41f8f389ce4103d214d37700`)
    - `femacs-v0.0.1-x86_64-apple-darwin.tar.gz` (`10,219,668` B, sha256 `d341fdca2c411fc1583243ba965d3f360fb74ebdbf25860a5fd7fa843da0c6b0`)
    - `femacs-v0.0.1-aarch64-unknown-linux-gnu.tar.gz` (`8,973,511` B, sha256 `b2d6965e261d13bc0030d5c8c923b8361888caf231b154c4cb897d141aa579f6`)
    - `femacs-v0.0.1-x86_64-unknown-linux-gnu.tar.gz` (`9,812,571` B, sha256 `2678a3a76bb4dfe96e4be9a86094d1223e6f09003b7dc54e95ba8bfe97311bca`)
    - Windows (`x86_64-pc-windows-gnu`) failed on missing `x86_64-w64-mingw32-dlltool` and remains unprebuilt in `v0.0.1` (`cargo install elisp femacs` fallback documented).
- **Zero-Dependency Eliscript npm Package (`packaging/npm/femacs/`)**:
  - Authored `src/index.eli`, `src/postinstall.eli`, `src/bin/{elisp,femacs,fe}.eli`, `test/installer.test.eli`, `scripts/{build,test}.sh`, `package.json` (`devDependencies: { "eliscript": "^0.1.4" }`, zero runtime `"dependencies"`), `.gitignore` (ignoring `dist/`, `.tmp/`, `node_modules/`, `package-lock.json`), `manifest.json`, `types/index.d.ts`, dedicated `README.md` (`5.7 kB` covering contents, cached binaries, excluded items, platform matrix, API & env vars), and `LICENSE`.
  - Normalized `bin` values (`dist/bin/{elisp,femacs,fe}.mjs`) and `repository.url` (`git+https://github.com/liuchong/femacs.git`) in `package.json` for zero publish warnings on `v0.0.2+`.
- **Homebrew Tap & Shell Installer**:
  - Added canonical `packaging/homebrew/femacs.rb` (`Femacs` formula for macOS ARM/Intel + Linux ARM/Intel, `ruby -c` Syntax OK) and created GPG-signed public repository https://github.com/liuchong/homebrew-tap (`master` with GPG-signed `init: empty` + `Formula/femacs.rb` & `README.md`).
  - Added `docs/install.sh` (`chmod +x`, `sh -n` OK, SHA-256 verified against `SHA256SUMS.txt` into `~/.local/bin`).
- **Unified Multi-Channel `xtask release` & Docs/Pages (`xtask/src/release.rs`, `xtask/src/release_tests.rs`, `docs/release-process.md`)**:
  - Extended `bump` (Cargo manifests + `packaging/npm/femacs/package.json` + `docs/install.sh`), added `sync-dist --version --checksums` (deterministic `manifest.json` + `femacs.rb`), extended `check` (npm/Homebrew/`install.sh` validation + zero tracked `.js`/`.mjs`), extended `publish` (`--assets-dir`, `--with-npm`, `--homebrew-tap-dir`), and updated `render_release_notes`, `README.md`, `docs/i18n/zh-CN/README.md`, `docs/support-matrix.md`, `docs/index.html`, and `docs/pages/{lisp,start}.html`.

## Verification Evidence

- **Cross-Compile & Host Inspect (`run_id=b604239005fd49e2b763766be13cd3bf` & `6518a70043684b3cbe082d53739589b3`, `exit=0`)**:
  - `aarch64-apple-darwin`: real host smoke passed (`elisp 0.0.1`, `42`, `"[[\"ok\",true]]"`, `fe --help`, `otool -L` `/usr/lib/libiconv.2.dylib` & `/usr/lib/libSystem.B.dylib` only).
  - `x86_64-apple-darwin`: clean `--release` Mach-O + `otool -L` system-dylib-only inspection (`ROSETTA_AVAILABLE=0` on host; no physical Intel Mac or Rosetta runtime execution).
  - `aarch64-unknown-linux-gnu` & `x86_64-unknown-linux-gnu`: Zig-linked (`glibc >= 2.31`) ELF binaries built & archived; native `cargo check -p elisp -p femacs` clean.
- **npm & `install.sh` End-to-End Verification (`run_id=2b1c5556f4c64845ae1be49be3e0d41f`, `f717875435e14dc28847f9abd229b801`, `86ff709564c84e49b9b56b0b4d862328`, `exit=0`)**:
  - `npm test`: `FEMACS_NPM_ELISCRIPT_TESTS_OK (9 cases)` (mock HTTP server, SHA-256 verification/rejection, atomic cache reuse, exit-code forwarding, `FEMACS_SKIP_DOWNLOAD=1`, `dist/bin/elisp.mjs` `manifest.json` resolution).
  - `npm pack`: `femacs-0.0.1.tgz` (`85.8 kB`, `10` files, `0` external `eliscript/` imports, `0` runtime deps).
  - Isolated real-network `npm install` + `docs/install.sh` smoke: `elisp 0.0.1`, `42`, `"[[\"ok\",true]]"`, `fe --help` exited `0`.
  - Live on npm: `npm view femacs` confirms `latest: 0.0.1`, `bin: { elisp, femacs, fe }`, `license: GPL-3.0-or-later`, `0` runtime dependencies (`run_id=cd7922157bed412eb228af71e3a33a8e`).
- **Homebrew Tap, `xtask` & Headless Chrome Pages (`run_id=86ff709564c84e49b9b56b0b4d862328`, `57b6bd9b619f49ef965d853424812ed3`, `b4a10b7b531f467da1129edbe0427eaa`, `d4946c4800574c04b548ea08f8150a4c`, `exit=0`)**:
  - `liuchong/homebrew-tap` live on `master` with valid `Formula/femacs.rb`.
  - `cargo test -p xtask --bin xtask`, `xtask release sync-dist`, and `xtask release check --version 0.0.1 --allow-dirty --skip-package` passed; `gh release edit v0.0.1` updated release notes while tag `v0.0.1` stayed pinned at `874a28d0`.
  - `PAGES_CHROME_VERIFY_OK`: `Eliscript 0.1.4 · live` across all 5 routes × 3 viewports (`1440`, `390`, `320`) with `0` errors and `0` overflow.
- **Post-Cleanup Static Gates (`run_id=d952e3cec4254c77be3b743991d162d0`, `exit=0`, `16.997s`)**:
  - Removed `target/`, `build/*`, `tools/__pycache__`, `packaging/npm/femacs/{node_modules,dist,.tmp,package-lock.json}` (empty `build/`).
  - Passed `check-own-voice.py` (`0` failures), `check-ledger-ticks.py` (`58` bound / `0` failures), `test-tooling-modernization.py` (`33/33`), `check-unsafe-boundary.py` (`474` active files, `178` `forbid_unsafe_files`, `6,446/6,446` `SAFETY:`, `0` errors), `check-crate-boundaries.py` (`ok standalone=8`), `test-repository-bootstrap.py` (`4/4`), `git diff --check`, zero host home paths in diff, and zero tracked `.js`/`.mjs` under `docs/` or `packaging/npm/femacs/`.
