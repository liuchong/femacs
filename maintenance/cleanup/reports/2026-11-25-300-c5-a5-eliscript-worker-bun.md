# CLN300 — Stage C5/A5 Opt-In Eliscript Worker Integration & Bun Verification (2026-11-25)

## Summary

Implemented FEmacs-side **Stage C5/A5** across `140afcc6` (feature & detached harness) and `62ab8a94` (frozen runtime `stdlib/bit.mjs` closure & fixture balance), rebuilt Terminal (`build/terminal/port`) and Cocoa (`dist/macos/Femacs.app`), atomically reinstalled `/Applications/Fe.app` (`diff -rq` empty, strict `codesign` `exit 0`), validated clean-archive `eliscript` (`db9fd8ae3656c0671a3c6f89acfc554b736334e2`) frozen package execution with `bun 1.4.0` across detached Cocoa GUI (`/Applications/Fe.app`, `framep=ns`) and Terminal PTY (`framep=t`) sessions (passing all `9/9` case groups in both editors), verified TTY/Cocoa editor workflows & byte-compile/undo fidelity, retired `14` temporary scopes (`CLN300`), and passed post-cleanup verification. Zero edits were made to the core `lisp/` tree or the sibling Eliscript repository.

## C5/A5 Changes & Controlled Verification Receipts

- **Opt-In Worker Client & Acceptance Suite (`140afcc6`, `62ab8a94`, `examples/eliscript-worker/{femacs-eliscript-worker.el,README.md}`, `tools/test-c5-a5-worker.py`, `tools/fixtures/c5-a5-worker-cases.el`, `maintenance/tooling-inventory.json`)**:
  - `examples/eliscript-worker/femacs-eliscript-worker.el`: strictly opt-in asynchronous worker integration resolving installed `bun`/`node` runners and prebuilt frozen packages without bootstrap fallback (`missing-js-runtime`, `missing-worker-resource`), exposing `femacs-eliscript-status` diagnostics, capability checks (`unsupported-capability`), `*Eliscript Analysis*` buffer output + `atomic-change-group` buffer annotations (`stale-buffer` tick rejection, single-step `undo`), cooperative cancellation, and single-run `side-effect-completion-unknown` semantics (disallowing `:verify 'always` and never replaying failed side-effect calls).
  - `tools/test-c5-a5-worker.py` & `tools/fixtures/c5-a5-worker-cases.el`: archives `eliscript` (`db9fd8ae`) cleanly, builds frozen package artifacts via `dist/lisp/bin/femacs-lisp` + `make bootstrap-release && make prepare-release-package`, strips `compiler/` and `tools/bootstrap-runtime.el` (plus per-example `node_modules` so `worker.mjs` resolves `runtime/`/`platform/`/`stdlib/` directly), copies client + fixture to a detached directory outside both repositories, and drives full Cocoa GUI and Terminal PTY acceptance.
- **Rebuilt Product & Controlled Verification**:
  - Static gates (`exit=0`): `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`), and zero personal paths in tracked changes.
  - Full Terminal & Cocoa rebuild (`run_id=572fa3a0ae2a442291f50f4ab4e46d44`, `644.389s`, `exit=0`, `cleanup_complete=true`) + atomic `/Applications/Fe.app` install (`diff -rq` empty, strict `codesign` `exit 0`).
  - Frozen `db9fd8ae` Eliscript runtime package preparation (`run_id=3f81b5b5be3240ef8256ecb4b04a8235`, `152.609s`, `exit=0`, `cleanup_complete=true`), verified by SHA-256 across `runtime/worker.mjs` (`61,868` B, `4ca1820e...`), `platform/worker.mjs` (`8,307` B, `8a5e17b8...`), `stdlib/bit.mjs` (`7,461` B, `3776caaa...`), `dist/runtime/lisp.mjs` (`5,056` B, `0255dddc...`), `analysis/analysis.mjs` (`5,797` B, `e6a543cc...`), `probes/c5-probes.mjs` (`3,451` B, `55a2f33d...`) + `.mjs.map` (`3,433` B, `a44f1c8a...`), and the 4 Lisp worker client files.
  - `A5` detached Bun `1.4.0` acceptance (`run_id=f545b187f964451d8f8bc4c334392390`, `7.288s`, `exit=0`, `cleanup_complete=true`): `opt_in_clean` `(nil nil 0)` on both binaries; Cocoa `/Applications/Fe.app` (`ns` GUI frame, `2.2074s`, `pid=81068`, `module_digest=683ca703...`, `13` capabilities including `runtime-resolution`, `value-codec`, `value-chunks`) and Terminal PTY (`t` frame, `4.2021s`, `pid=81145`) both passed `9/9` case groups covering opt-in cleanliness, negative missing runtime/resource/capability errors, verified buffer analysis + `*Eliscript Analysis*` + atomic annotation + single `undo` + search (`2` hits) + `stale-buffer`, 18k persistent chunked map/keyword values + stream limit enforcement, mid-flight cancellation, `.eli` source-mapped errors (`c5-probes.eli:31:17`), stderr isolation, `SIGKILL` crash + generation `1 -> 2` recovery, and timeout/`SIGKILL` side-effect `side-effect-completion-unknown` with strictly single execution.
  - Editor regressions (`run_id=1576c20d9f54480dbda4e004050ab395`, `8.127s`, `exit=0`, `cleanup_complete=true`): `test-macos-delivery.py --file-workflow` (`PASS`, `4.355771s`), `C-a`/`C-k-C-y`/`C-g-recover`/`undo` (`PASS`), and Terminal/Cocoa byte-compile + edit + undo fidelity (`(t 42 "alpha gamma\n")`).
  - Post-cleanup smoke (`run_id=60dc72c2207d4b1e927a4acebebb5cad`, `14.183s`, `exit=0`, `cleanup_complete=true`): `diff -rq dist/macos/Femacs.app /Applications/Fe.app` empty, strict `codesign` valid on both bundles + `dist/lisp/bin/{femacs-lisp,http-host}`, `femacs-lisp-cache.py verify --expect-commit 9ef6e40f...` verified (`sha256=490340fd...`), `terminal` & `Fe.app` `-Q --batch` `(42 t t nil nil 0)`, `emacsclient 32.0.50`, `check-own-voice.py` (`0` failures), `test-repository-bootstrap.py` (`4/4`), `test-tooling-modernization.py` (`33/33`), and zero `tools/__pycache__`.

## Retired Temporary Scopes

Retired `14` owned temporary Cargo target, build-tool, pinned Tree-sitter SDK, acceptance scratch, Cocoa build staging, terminal build-cache, and duplicate versioned dump scopes totaling `2,289` files, `738` directories, `8` symlinks, `824,241,077` logical bytes, normalized digest `b8b9d16cffcad57c3bbc96cc671d51cfa0da18f253cad97e37111444efebc024`:

| Removed scope | Pre-delete `du` | Files | Dirs | Symlinks | Logical bytes | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `target` | `184M` | `1,246` | `283` | `0` | `217,989,524` | Root Cargo target cache |
| `build/cargo-tools` | `33M` | `112` | `55` | `0` | `34,056,663` | Recreated by `xtask build-terminal` / `build-macos` |
| `build/treesit-0.27.1` | `604K` | `5` | `8` | `2` | `607,238` | Pinned SDK via `tools/toolchain-pins.py` |
| `build/acceptance` | `6.9M` | `251` | `41` | `0` | `6,581,873` | `C5/A5` frozen package, GUI/PTY, and TTY workflow scratch |
| `build/macos` | `292M` | `377` | `189` | `6` | `328,548,709` | Cocoa build staging (`dist/macos/Femacs.app` and `/Applications/Fe.app` retained) |
| `build/terminal/cargo` | `199M` | `247` | `135` | `0` | `207,501,053` | Terminal Cargo build cache |
| `build/terminal/config` | `48K` | `7` | `1` | `0` | `35,082` | Recreated by `xtask` terminal prepare |
| `build/terminal/image` | `1.3M` | `3` | `3` | `0` | `1,270,503` | Recreated by `xtask` terminal prepare |
| `build/terminal/image-staging` | `1.3M` | `2` | `1` | `0` | `1,269,940` | Recreated by `xtask` terminal prepare |
| `build/terminal/lcms` | `364K` | `5` | `4` | `0` | `360,608` | Recreated by `xtask` terminal prepare |
| `build/terminal/tls` | `7.6M` | `26` | `11` | `0` | `7,878,424` | Recreated by `xtask` terminal prepare |
| `build/terminal/treesit` | `316K` | `6` | `7` | `0` | `307,980` | Recreated by `xtask` terminal prepare |
| `build/terminal/port/src/emacs-32.0.50.1` | `4.6M` | `1` | `0` | `0` | `4,732,112` | Duplicate versioned binary alias |
| `build/terminal/port/src/emacs-32.0.50.1.pdmp` | `13M` | `1` | `0` | `0` | `13,101,368` | Duplicate versioned dump alias |

## Retained Working Set & Artifact Verification

- `build/terminal` (`48M`, `build/terminal/port`):
  - `src/emacs`: `4,732,112` B, SHA-256 `67c6d379fc73a611cb829b90e1d231b6d41b091f131add5e481b3be2550f7d77`
  - `src/emacs.pdmp`: `13,101,368` B, SHA-256 `958ce1eac86fd3c6f535b6d08fab33fe27107a8544ef203e8f0aab7433bcda72`
  - `lib-src/emacsclient`: `539,328` B, SHA-256 `1baf54e687c5fd61259f6699a2d2953b7e6567e350a304942b2e06f38ba600c7`
  - `lib-src/hexl`: `451,488` B, SHA-256 `15f41ba92128f2549c6e7f8f3e6685f379b9dec79d0f34b4696a4e5dda4878af`
- `dist/macos/Femacs.app` (`235M`) & atomically reinstalled `/Applications/Fe.app` (`235M`, `diff -rq` empty, strict `codesign` `exit 0` on both):
  - `Contents/MacOS/Femacs`: `6,264,592` B, SHA-256 `8bfe40f328df575f99b220dad91687a32f736c43ab154f93bfe050cfb8cad03a`
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,600,760` B, SHA-256 `b38b8db8a4091cf8f3207e748071cf1dc2471433e08b5ade72523c44cde30d4a`
  - `Contents/MacOS/bin/emacsclient`: `538,784` B, SHA-256 `ca959192dfd327a692c9bdc76b1c12ae7889652a0e65ca11193adc53bfafa755`
  - `Contents/MacOS/bin/hexl`: `451,120` B, SHA-256 `2ee11f3c9964f5aaa3465c07006a78f9d98487c56c81808faf42e3fc20ef7014`
- Retained supporting sets: `dist/lisp` (`210M`, bound to `9ef6e40f2e749980c18f98a0aba74884037a4a97`, `package_sha256=490340fd8efa6de351c8097fb8ee7afd2a30ad74e0e7f3c0dd90e474b26159af`), `build/generation` (`17M`), and `build/manuals` (`1.5G`).
