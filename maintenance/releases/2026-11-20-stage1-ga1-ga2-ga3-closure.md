# Stage I (`GA1–GA3`) Closure Receipt (2026-11-20)

This receipt records consolidated Stage I (`GA1`, `GA2`, `GA3`) verification and protective regression results starting from baseline `baa31c12017c63fe7eee565985b17701ba4f971a` on Apple Silicon macOS (`macOS 27.2`, `arm64`).

## 1. Delivered Stage I Standard Products

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| `build/terminal/port/src/emacs` | `2,798,256` | `a0dd02e116b9f37e8eb6fc86be630118bb7d0d82af5e9f3c2c8cdaf5d7960241` |
| `build/terminal/port/src/emacs.pdmp` | `13,101,368` | `cd076b1dc17a9b2c6cac605347442604214bf18d49ef41cdfaace31d62bfac18` |
| `dist/macos/Femacs.app/Contents/MacOS/Femacs` | `4,367,952` | `02f5adafe3c1a5fc86276632e521ed5228ed37d5cfe4443377437d799098104e` |
| `dist/macos/Femacs.app/Contents/Resources/libexec/Femacs.pdmp` | `18,600,752` | `e1515db2344d096fdf23aea1a376ae9628d93993e00fef4b308aea198b639e6b` |
| Pinned `tree-sitter` `0.27.1` (`include/tree_sitter/api.h`) | — | `f8480415ae20cb5e0fd519ba44c84268c5acb4c918106238fc1d328d91431783` |

## 2. `GA2` Clean Tracked Export, Relocation & Gatekeeper Boundary

- **Clean export build**: Exported tracked candidate tree (`7,629` files, tree SHA-1 `ee9725fd3bfbfc2d7ec04dc9133e6e9013e85662`) via `git archive` without `.git`, optional reference checkout, old objects, or caches. Provisioned pinned `tree-sitter 0.27.1` from `build-support/toolchain-pins.json` (`7.955s`) and built both `cargo run --release -p xtask -- build-terminal` and `cargo run --release -p xtask -- build-macos` from empty output under `controlled-run --profile build` (`757.199s`, peak `1,819,213,824 B`, `exit_code=0`, `cleanup_complete=true`).
- **Hidden-export relocation**: Copied `dist/macos/Femacs.app` to an isolated sibling directory, renamed the export tree so its build path was physically absent, and verified (`build/acceptance/s1-ga2-receipt.json`, `14,572 B`, SHA-256 `1311e0ada6d0ddc0aaae109554fd06fcbb4df3971a9d738a66b486a061bb9ef4`):
  - `--batch` startup resolves `data-directory`, `exec-directory`, `DOC`, and `load-path` inside the relocated bundle, with `treesit`, `sqlite`, `png`, `jpeg`, and `webp` enabled and all `17` bundled `@rpath` dylibs loaded from relocated `Contents/Frameworks/**` (`0` build-tree leaks).
  - Real GUI startup via `NSWorkspace.OpenConfiguration` (`tools/fixtures/ime-launch.swift`, PID `19167`) initializes `window-system=ns` against relocated `Contents/Resources/etc/` and exits `0`.
  - Strict signature verification `codesign --verify --deep --strict --verbose=2` exits `0` (`satisfies its Designated Requirement`).
  - Gatekeeper assessment `spctl --assess --type execute -vv` returns `exit_code=3` (`rejected`), recorded explicitly as the unnotarized ad-hoc local-trial boundary.

## 3. `GA1` macOS Terminal & Cocoa GUI Real Workflows

- **Multi-window IME (`tools/test-macos-ime.py --mode multi`)**: `passed=true` (`19.140s`, PID `25856`, `407` native Cocoa trace records; receipt `build/acceptance/s1-ime-multi/results.json`, `26,788 B`, SHA-256 `07a5fcbf8870d924899785e60aabc05ecefb18feb017a5a083acc54cd8280087`). Verified initial cooperative-activation promotion (`activate PID`), HID titlebar-click focus transfer between Frame A and Frame B, Pinyin preedit/commit (`"你好"`), composition cancellation (`Escape`), pending-composition focus transfer (`focus_loss_cleared_marked=true`, `a_text="你好"`, `b_text="你好"`), subsequent Frame B commit (`"你好你好"`), file save (`"你好\n"`), clean `C-x C-c` exit (`exit_code=0`), and restoration of `com.bytedance.inputmethod.doubaoime.pinyin`.
- **External Finder drag-and-drop (`tools/test-macos-drag.py`)**: `passed=true` (`8.969s`, editor PID `26469`, owned Finder icon-view window `106592`; receipt `build/acceptance/s1-finder-drag/results.json`, `6,053 B`, SHA-256 `b1dab2e158e82ef8bed85b32aee66d630635078f76165ebde266456b5434c85c`). Dragged `AXImage` (`64x64`) via HID `CGEventPost`, recording `25` live `ns-drag-motion-function` events with numeric `x`/`y` on `Femacs Drag Target`, `draggingUpdated` + `performDragOperation`, exact opened UTF-8/Chinese payload (`47 B`), unchanged binary/dump digests, and clean window/process teardown.
- **Full local acceptance (`tools/local-acceptance.py --gui --ime`)**: `14/14` checks `passed=true` (`108.856s`; receipt `build/acceptance/s1-local-acceptance-3/results.json`, `16,007 B`, SHA-256 `b781399397f69abe41d50f828e6380509aec8018ad04c8deb01eebb0960ba136`), covering `root-inputs`, `ert-components` (`16/16`), `acceptance-components` (`11/11`), `bundle-signature`, `terminal-batch`, `gui-batch`, `terminal-user-init`, `gui-user-init`, `gui-background-gc` (`8/8`), `terminal-keys` (`26/26`, aligning `<end>` with `lisp/bindings.el:1428` when `term/ns-win` is not preloaded), `terminal-files`, `gui-keys` (`26/26`), `gui-files`, and `ime --mode all` (`steady` + `switch`, `29.74s`). Two prior diagnostic runs (`session-cache/s1-local-acceptance` before moving `--out` under `build/` and fixing terminal `<end>`, and `s1-local-acceptance-2` before preserving user-site `PYTHONPATH` under isolated `HOME`) were retained for diagnosis and recorded before retirement.
- **Abnormal / no-controlling-TTY (`tools/test-goal-stability.py --mode all`)**: `16/16` cases (`10` helper units + `6` `sd_a1_*`/`sd_a2_*` real TTY cases) `passed=true` (`0.546s`; receipt `build/acceptance/s1-tty-stability/results.json`, `14,899 B`, SHA-256 `6d0f381abbac85223e8fd48f45292d301150191eeb5553a697a6e01f0055c965`).

## 4. `GA3` Pinned-Tool Eglot Strict Probes & Scoped Classification Closure (`3 -> 0` Effective Blocking)

- **Pinned-tool strict probes (`tools/functional-strict-probe.py`)** against the bound `32.0.50` installed reference (`fd7afced4036fc1a221567d70705d20a2bd64713`) and Stage I `dist/macos/Femacs.app` (`108.901s`):
  1. `build/acceptance/s1-eglot-probe-insert-replace/adjudication.json` (`833,443 B`, SHA-256 `951577dcddeaabcb150d12a126b128037e2f77360ba34774aca25300361225ac`): pins `stty=/bin/stty` (`217,136 B`, SHA-256 `13f4d12405af334bdc1c6f220b7330f7897181c65a778adf6efeb06929ef37e4`) and `clangd` `23.1.5` (`18,242,600 B`, SHA-256 `728bbde57b30026485e9a40bd85eb4b0556ca139c1551566d8f79eeafb929d1c`). Both `eglot-test-tramp-test` and `eglot-test-tramp-test-2` pass on both sides (`28` total server connections, `2` `mock` remote servers per side reporting `clangd 23.1.5`); `eglot-test-try-completion-inside-symbol-2` fails identically on both sides with `(ert-test-failed ((should (looking-at "123")) :form (looking-at "123") :value nil))` (`verdict=same`).
  2. `build/acceptance/s1-eglot-probe-text-edit/adjudication.json` (`833,783 B`, SHA-256 `a5a40a645c9de6d4896d1903143e3f06f6cd6bac1e88f575263fedd8ec13b51d`): pins `stty=/bin/stty` and Apple `/usr/bin/clangd` `21.0.0` (`200,560 B`, SHA-256 `ed6f5284033aa2aae6a58427c25ba3ab119947b40365627b24e5d51a4b70a1cd`). All `47` `eglot-tests.el` cases pass on both sides (`pair_verdict=all-passed`, `verdict=same`, `28` total / `2` `mock` remote servers per side reporting `Apple clangd version 21.0.0`).
- **Source-rule expansion & scoped classification**:
  - `tools/stable-source-rules.py` expands `578` source rules into `27,744` definitions across `48` profiles (`passed=true`).
  - `build/acceptance/g3-scoped-consume-555-20/results.json` (`5,830 B`, SHA-256 `717c821ee95e3590d492d8a8aa1eb96f31c9ffcbd10f9ffbf06b7366cc9ca52a`): binds `565/574` raw failures on `g3-555-frozen2` (`555` entrypoints, `9,466` cases, raw failures conserved at `574`, `structural_failure_count=0`), reducing `eglot-tests.el` blocking cases from `3 -> 0`. All `9` remaining `old555` blocking cases lie inside the `11` declared subset files.
  - `build/acceptance/g3-scoped-consume-subset-5/results.json` (`10,748 B`, SHA-256 `a0f0a6c2cbcedf19a78f1b88264884563f1f74e53b74ac2785278c8c4b8cd02c`): binds `37/37` raw failures on `g3-subset-native-1` (`11` entrypoints, `603` cases, `classified_corpus_accepted=true`, `blocking_case_failure_count=0`, `capture_failure_count=0`, `structural_failure_count=0`).
  - `build/acceptance/g3-effective-blocking-2.json` (`3,567 B`, SHA-256 `7d504814ac73b22be4a1d0a8a4ea511a2bf706f42b416c3aa205e141d8c0a3fb`): combined effective blocking across all `555` official-scale entrypoints drops from **`3 -> 0`** (`blocking_by_file=[]`). Zero `src/*.rs` VM/evaluator/GC/library files changed in Stage I relative to `baa31c12`; the single opt-in `FEMACS_INPUT_TRACE` field in `platform/macos/nsterm.swift` is directly exercised on Stage I `dist/macos/Femacs.app` by `s1-local-acceptance-3`, `s1-ime-multi`, `s1-finder-drag`, and both Eglot strict probes.

## 5. Protective Unit & Gate Regressions

- `python3 tools/test-functional-strict-probe.py`: `3/3` passed (`1.061s`).
- `python3 tools/test-macos-stability.py`: `11/11` passed (`0.082s`).
- `python3 tools/test-stable-classification.py --mode integration`: `51/51` passed (`16.495s`), including positive and negative counterexample rejections for `remote-unqualified-stty-raw-echo` and `external-lsp-insert-replace-edit`.
- `python3 tools/test-stable-classification.py --mode protection`: `41/41` passed (`6.127s`).
- `python3 tools/check-own-voice.py`: `0` failures; `python3 tools/test-repository-bootstrap.py`: `3/3` passed.
