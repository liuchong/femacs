# CLN272 — Consumed Stage III intermediate outputs, `native-collector-1` cohort, and unit caches

After completing Stage III (`GA5–GA8`) unit/protective suites (`test-runtime-comparison.py` `7/7`, `test-release-baseline.py --component-only` `13/13`, `test-modernization-tools.py` `15/15`, `test-tooling-modernization.py` `33/33`, `test-runtime-measurement-contracts.py` protection, `test-runtime-measurement-report.py` `14/14`, `test-runtime-phases.py` integration & protection, `cargo test -p xtask` `115/115`), actual-source B-vector contracts (`s3-runtime-components-2`, `s3-runtime-contracts` `28/28`), three-product differential gates (`s3-stable-compat-32` `48/48`, `s3-a2-runtime-gates-2` `hotpath` `10/10` × 3, `s3-a2-runtime-gates-3` `call-frames` `7/7` × 3 & `bytecode-runtime` `10/10` × 3), `G6` live `native-2` verification (`s3-a2-runtime-gates-4` `17/17`), `G7` collector/fallback/phase/cadence/native-write-barrier verification (`s3-a2-runtime-gates-4` & `-5`, including `g7-native-write-barrier.el` `3/3` on `native-collector-1`), and end-to-end diagnostic profiling (`s3-g5-comparison-diag-3`, `s3-ga8-delivered-diag-2`, `s3-ga8-bench-cocoa-diag-2`, `s3-ga8-bench-terminal-diag-2`), `build/runtime/native-collector-1`, `build/cargo`, `dist/candidates`, and superseded diagnostic/unit directories completed their consumers.

Removed `41,534` ordinary files, `1,621` directories (including the emptied session-cache root), and `16` symbolic links across `16` owned scopes totaling `2,389,859,343` logical bytes (normalized scope digest `94241a9ec3d4adf9c1b447c9a3b1a9912116f07b7fd2ceb5858b38a2d2014ccc`):

| Scope | Files | Dirs | Symlinks | Logical Bytes | Tree SHA-256 | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/runtime/native-collector-1` | `19,932` | `818` | `14` | `1,426,239,152` | `da29d924f450b8b59ab75f368856134bbc2aa69c044ef16af17f9ba6109d77e7` | `build/acceptance/s3-a2-runtime-gates-5/results.json`; `cargo run --release -p xtask -- runtime-build --product cocoa --variant native-collector --observer` |
| `build/cargo` | `6,570` | `557` | `0` | `797,302,982` | `11b61b025459e53b18d37bf27c1a5b3a4513f818035a7efd2f18faf7d1f6075e` | `CARGO_TARGET_DIR=build/cargo cargo test -p xtask` |
| `dist/candidates` | `4` | `2` | `0` | `131,887,400` | `bac410062d83c5d6b2fb8c91233d91ca4ee44b323dc7783ef9e963641b8036e6` | `docs/support-matrix.md` (`5118ed08`); `cargo run --release -p xtask -- build-android` |
| `build/femacs` | `326` | `7` | `0` | `14,424,823` | `cc83d632cf2d356faed6fa722ec7f209e70905a590dbce07fefbf742d79e6559` | `cargo run --release -p xtask -- build-terminal` |
| `build/generation` | `14,548` | `128` | `0` | `4,230,732` | `a9eec11fa49d2e2b415dc4fe8805658857998168eeefec5a8bfbce39894abb6a` | `python3 tools/bootstrap-resources.py` |
| `build/acceptance/s3-runtime-components` | `94` | `52` | `0` | `10,821,132` | `213892cfb3f59180253ab5842ca518b12fb5a582709e2986e2a91a90afc3c83f` | `build/acceptance/s3-runtime-components-2/receipt.json`; `python3 tools/runtime-components.py` |
| `build/acceptance/s3-a2-runtime-gates` | `0` | `1` | `0` | `0` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `build/acceptance/s3-a2-runtime-gates-2`..`-5` |
| `build/acceptance/s3-diag-plans` | `2` | `1` | `0` | `5,993` | `010e5343f3cf29f271e30e4ff27caf15c09c1d819b1c8bf855d2e2ef6fd1b85c` | `tools/fixtures/runtime-comparison-plan.json` & `tools/fixtures/ga8-delivered-comparison-plan.json` |
| `build/acceptance/s3-g5-comparison-diag-1` | `23` | `17` | `0` | `1,647,284` | `d00af97f3b45303978538444629d8ec66446250043ce8d4844ba79ac1c841657` | `build/acceptance/s3-g5-comparison-diag-3/results.json` |
| `build/acceptance/s3-g5-comparison-diag-2` | `25` | `19` | `0` | `1,673,048` | `f85c1cd24fb98a835481dabaf9ad53f9d6289ce4fb288d33292f8695ad8c7852` | `build/acceptance/s3-g5-comparison-diag-3/results.json` |
| `build/acceptance/s3-unit-comparison` | `1` | `1` | `0` | `550` | `b45bfafdb577a34fa1e1fa8391500af7be88c520405ef049550ede6efdc90232` | `build/acceptance/s3-unit-comparison-2/results.json` |
| `build/acceptance/s3-unit-profile` | `1` | `1` | `0` | `2,843` | `c7c77a7513b634a20a547fbe51267370c1bff4362a5ee5badcd8301cf5e674c9` | `build/acceptance/s3-unit-profile-2/results.json` |
| `build/acceptance/s3-unit-modernization` | `1` | `1` | `0` | `3,254` | `baf0adf7548b6efa5fa2fe94d07c2c7b44ab2ea16f05f13e7036223b0d2fb3ac` | `build/acceptance/s3-unit-modernization-2/results.json` |
| `build/{android,bundle,manuals,package,probes,tooling}-tests` | `0` | `6` | `0` | `0` | `2b21e6f3f22a6c899a773fa11c756ecddaf3f74dc3a2d894c4a55ed33bcdae82` | `CARGO_TARGET_DIR=build/cargo cargo test -p xtask` |
| `session-cache/sdk-treesit-0.27.1` | `5` | `8` | `2` | `607,302` | `c8094db0294a98639a17d92aab684abf361415e70f4db2b6783bc661c8a359bb` | `python3 tools/toolchain-pins.py install --prefix <dir>` |
| `session-cache/s1-helpers` | `2` | `1` | `0` | `1,012,848` | `0c9dccef3862b432463eef13b1232e9c95f2ff2ac01f8845e7a01acaa1a0fdac` | `rustc tools/fixtures/macos-events.rs` & `macos-input-source.rs` & `swiftc tools/fixtures/ime-launch.swift` |

Retained for the pending quiet-host formal `2+7` / 7-round measurement pass and final `GA1–GA8` reverse audit:
- Delivered trial products: `build/terminal/port/src/{emacs,emacs.pdmp}` and `dist/macos/Femacs.app`.
- Active `runtime-comparison-plan.json` candidates: `build/runtime/{native-2,frozen-2}`.
- Closed Stage I and Stage III receipts under `build/acceptance/`.
