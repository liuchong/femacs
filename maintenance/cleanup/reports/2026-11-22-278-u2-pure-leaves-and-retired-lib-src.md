# CLN278 — U2 pure leaf migration, Safe Rust companion tools, and retirement of consumed AU2 artifacts

Completed Phase 0 Stage `U2` (`工具与叶子`, `P-481`) and formal acceptance `AU2` (`纯代码可验证`):
- Migrated pure cryptographic digests (`md5`, `sha1`, `sha256`, `sha512`, `sha3`), `filevercmp`, `filemode`, Two-Way `memmem`, `strcase`, and saturating `timespec` arithmetic into `crates/femacs-pure` (`#![forbid(unsafe_code)]`, `overflow-checks = true`), reducing `lib/` counterparts to thin `#![deny(unsafe_op_in_unsafe_fn)]` C-ABI slice wrappers with 100% `// SAFETY:` documentation.
- Rewrote `lib-src/emacsclient.rs` to `#![forbid(unsafe_code)]` Safe Rust atop `crates/femacs-sys/src/client_os.rs`, wired `hexl` into both Terminal (`port/lib-src/hexl`) and Cocoa (`Contents/MacOS/bin/hexl` + `Contents/Resources/libexec/hexl`) deliverables, and retired 8 verified zero-consumer `lib-src`/`lib` files (`19,048` lines; exact SHA-256s and `4ad0d5f3` recovery recorded in `docs/retired-scaffolding.md`).
- Verified `AU2` (`tools/test-u2-pure-and-tools.py` `7/7` passed in `24.856s`, `codesign --verify --deep --strict dist/macos/Femacs.app` `exit 0`): active-product `forbid(unsafe_code)` files rose from `4` to `23`, file-level `allow(unsafe_op_in_unsafe_fn)` / `allow(static_mut_refs)` dropped from `145` to `120` (`-25`), and `unsafe fn` definitions dropped from `5,075` to `4,940` (`-135`).
- Verified protective unit tests: `femacs-pure` (`7/7` passed) and `femacs-tools --bin emacsclient` (`3/3` passed).

Retired `2,908` ordinary files, `743` directories, and `8` symbolic links across `13` consumed build and verification scopes totaling `599,876,910` logical bytes:

| Removed scope | Files | Directories | Symlinks | Logical bytes | Tree SHA-256 | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `build/macos` | 210 | 99 | 6 | 283,799,716 | `9c1883ef78bd38f4d97d869c28672f1a6f1b888f7ea31ae93912b5bc8665e16a` | `dist/macos/Femacs.app`; `cargo run -p xtask -- build-macos` |
| `build/manuals` | 1,188 | 41 | 0 | 94,825,858 | `9d4fbb6623420f814277f0d46806d416ef1ee0632b74141d4d12b51b932ac5dd` | `cargo run -p xtask -- build-manuals` |
| `build/generation` | 662 | 10 | 0 | 398,403 | `71633a28ebf9b30eac2774a258c08fb2df0f6cc2e2b873816bd33d072aba3d2e` | `tools/bootstrap-resources.py` via `xtask build-terminal` / `build-macos` |
| `build/acceptance` | 606 | 466 | 2 | 7,354,155 | `ecf23e180bd98b869a60a6f8479ea2e9fb6f7721a691eab41caad1ea0e5fb558` | `python3 tools/local-acceptance.py --out <dir>` |
| `build/cargo-tools` | 112 | 55 | 0 | 35,678,114 | `99edb24f481bb63a2955aad5323093bd86d91508f59b10f422c274420fab48dd` | `cargo run -p xtask -- link-program <tool>` |
| `build/terminal/cargo` | 80 | 45 | 0 | 166,696,210 | `0e41734211e55140cf1ac69afc4cf08c9bdcf6348b4e18c45d6813352b1f0908` | `cargo run -p xtask -- build-terminal` |
| `build/terminal/config` | 7 | 1 | 0 | 35,082 | `0174b77077739348c15d2700697828835b12b229eb31b58f6e74b395684b6bfb` | `cargo run -p xtask -- build-terminal` |
| `build/terminal/image` | 3 | 3 | 0 | 1,270,503 | `071079e004506a54ccb03ca8be7d8fae4d05ff4211f0dfcfb94d8e8ec84150dd` | `build/terminal/port/runtime-image` |
| `build/terminal/image-staging` | 2 | 1 | 0 | 1,269,940 | `d0988ea4966d9ce8e551b167ca298b63175d2d7befa8cad5dbb855f6a841f227` | `build/terminal/port/runtime-image` |
| `build/terminal/lcms` | 5 | 4 | 0 | 360,608 | `293d84badbb84c8df66cef2f20e9a12507a9747ab65810ac5515cc6a61229cd8` | `build/terminal/port/runtime-lcms` |
| `build/terminal/tls` | 26 | 11 | 0 | 7,878,424 | `5fa9c8ad463ed327b1c4584c113950662c78c0e8661514e7f16a3b5c99b0092a` | `build/terminal/port/runtime-tls` |
| `build/terminal/treesit` | 6 | 7 | 0 | 308,108 | `6a615bd07f8150015f030e708c6e98b7e1e2a44ef5e906c2e3277a114e3bfb9b` | `build/terminal/port/runtime-treesit` |
| `<session-cache>/au2-acceptance.json` | 1 | 0 | 0 | 1,789 | `5d5c32bc2b4c36af58f19d0acec1435c0378292d06a82202e817c55b6a95cfe9` | `python3 tools/test-u2-pure-and-tools.py` |

Retained active working set (and explicit retirement triggers):
- `build/terminal/port` (`44 MB`: `emacs` `427f09b1a52885117d8e3490b2de19e09caa2ad672cfdcbf260e1398e55d15db`, `emacs.pdmp` `eab7aadb6b5371ab0ed8a1833d0e0d64824661f5cef6559d67f8c16383e31dd0`, `emacsclient` `25b0266c2964ee23568ef6a2bb673a7d494ab54ab4b2fe16e5c8c3c47ee5b65b`, `hexl` `85a0bc3657ad3fc47b4e08ebb8504596bbdc8b11615c7d7fea2a7b783c9a41e3`) and `dist/macos/Femacs.app` (`234 MB`: `Femacs` `283c7be6bdcb6e681411ee09f023b8f55750e23137ec6c6372419a2e2925aef1`, `Femacs.pdmp` `c0dd36a846ee4cf522d78b60169dde2e9dabd7f7912395cc9e4893d0302c0121`, `emacsclient` `0c6235d1936e8ebc491278916a898a1faaec02a666f768d0a810d46cb7fd1069`, `hexl` `233436dd6bcc55199fe175995a9fb30abb296624ac45e41fc25ea30d0bae73c5`), verified post-cleanup via strict `codesign` and `-Q --batch` smoke.
- `build/cargo` (`961 MB`), `<session-cache>/u1-baseline`, and `<session-cache>/tree-sitter-0.27.1`: actively consumed by upcoming stages `U3–U5`; scheduled for retirement upon `AU5` completion.
