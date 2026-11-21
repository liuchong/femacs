# CLN277 — U1 unified Cargo workspace graph & retirement of consumed AU1 verification trees

Completed Phase 0 Stage `U1` (`Cargo 图与审计`, `P-480`) and formal acceptance `AU1` (`同产品可重建`):
- Unified the standalone Terminal, Cocoa, `runtime-build`, and companion `lib-src` builds onto the single Cargo workspace (`crates/femacs-pure`, `crates/femacs-sys`, `crates/femacs-vm`, `crates/femacs-host`, `crates/femacs-tools`), retiring per-unit `rustc --emit=obj` (`build-tu` / `build-units`), `LINK_DRIVER`, and `libgnu-port.a` / `libgnu-program.a`.
- Verified from a fresh pure-tracked export (`tools/test-repository-bootstrap.py --export`) that `build-terminal` (`emacs` SHA-256 `c9d1892e90d9166625ca244956847cda0dc8ba8b56e89dd86d15cd45b23698ee`, `emacs.pdmp` `efd9c8ee54e05912daa04c795df9d3f532f35d20c94aaccaedc4343223940f86`), `build-macos` (`Femacs` SHA-256 `90a1f78b87df3aa2375c2cd225110d5f115b55ff5d621b9230af6d600993d47e`, `Femacs.pdmp` `dc8c283bde21b7dadc736bfcec5a68bbce54b9e7a67fe078db97bd0ec30fc8d5`), and `runtime-build --product terminal --variant native` (`emacs` SHA-256 `473616f9f96e0bd9b640960db3029b761ddfee107571a9f0c9fbc850b3eb4d16`) build, pass `tools/check-unsafe-boundary.py` (`active_product.stitching_includes = 0`, `forbid_unsafe_files = 4`, single `lispsym`/`dump_hooks`/`nr_dump_hooks`/`main_thread`), load and execute the real C dynamic module (`mod-test.dylib`), pass `tools/local-acceptance.py` (`9/9`), and pass interactive Cocoa GUI startup (`window-system = ns`).
- Protective unit tests passed: `tools/check-unsafe-boundary.py --selftest` (`4/4`), `femacs-pure` (`3/3`), and `xtask` (`116/116`).

Retired `29,739` ordinary files, `1,921` directories, and `26` symbolic links across `20` consumed build and verification scopes totaling `3,249,751,743` logical bytes:

| Removed scope | Files | Directories | Symlinks | Logical bytes | Tree SHA-256 | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `<session-cache>/au1-export` | 24,000 | 1,468 | 20 | 2,536,859,014 | `6f743bc339df41fc2d7956a43ec90cefef8c113e00f1d9a928114ed90071e37b` | `python3 tools/test-repository-bootstrap.py --export <dir>` + `cargo run -p xtask -- build-terminal` / `build-macos` |
| `<session-cache>/au1-local-acceptance` | 236 | 99 | 0 | 2,653,913 | `2e5a950b09ea29ab57a1847ece8b971db2d1713df4a95200383cbecf4a1f4edc` | `python3 tools/local-acceptance.py --out <dir>` |
| `<session-cache>/au1-cocoa-gui-init` | 7 | 3 | 0 | 1,892 | `911337f5aed68546786dca261167ec4b1d02e3aad93fb751b50ee481d195b40e` | `tools/local-acceptance.py::user_init_workflow(..., gui=True)` |
| `<session-cache>/run_au1_acceptance.py` | 1 | 0 | 0 | 13,631 | `947e24f3866548fe7d7728475739370f5cb346e5587d9aadefe44dff57c5bb38` | `tools/local-acceptance.py`, `tools/check-unsafe-boundary.py`, `xtask` |
| `<session-cache>/au1-boundary-terminal.json` | 1 | 0 | 0 | 326,615 | `8b15cea4cde6602e102a2b12175fa4eaa9a382818cc9ce6f0d8089113c0100c8` | `python3 tools/check-unsafe-boundary.py --binary build/terminal/port/src/emacs --kind terminal --require-modules` |
| `<session-cache>/au1-boundary-cocoa.json` | 1 | 0 | 0 | 326,626 | `525d809988a4cd5aa2d414504f1f25df8940b07620943a40ae5cf70512045520` | `python3 tools/check-unsafe-boundary.py --binary dist/macos/Femacs.app/Contents/MacOS/Femacs --kind cocoa --require-modules` |
| `<session-cache>/au1-acceptance-receipt.json` | 1 | 0 | 0 | 5,720 | `1a8cfb034a9057c9ae371fb2465e4c1c09caec2d5ffac5ee6705a797d89b92e5` | Digests and step timings recorded in this report and `docs/status.md` |
| `build/macos` | 207 | 99 | 6 | 259,447,343 | `6c3bbf6633d5ecb4291ebbac248f89793acb2630c7d0f14f2aac8e8bededd614` | `dist/macos/Femacs.app`; `cargo run -p xtask -- build-macos` |
| `build/generation` | 2,072 | 22 | 0 | 840,416 | `b7b536135a4eb9c250803108a6c77e0b31834940606ba0f71389b1611a0b145a` | `tools/bootstrap-resources.py` via `xtask build-terminal` / `build-macos` |
| `build/manuals` | 2,970 | 101 | 0 | 237,069,294 | `03cd1df22158e49bfd17e46e84b3df9a4447e447a131d501a14e69f05a186f61` | `cargo run -p xtask -- build-manuals` |
| `build/cargo-tools` | 96 | 49 | 0 | 34,193,484 | `8d4374fa3a0ad82ef63a3053d2a6754bd973e77fce16b6a3b5b3c71275a92ec2` | `cargo run -p xtask -- link-program <tool>` |
| `build/acceptance/info-publication` | 1 | 2 | 0 | 44,925 | `7b2411c589fc24d07510ec2e1a6a490843b59900ee7b476539f94df1db13f735` | `tools/bootstrap-resources.py` |
| `build/acceptance/manual-command` | 15 | 6 | 0 | 15,330 | `d9594905e434aef4c9300bae9d7c198d90376ffd78f2bf8c7d0485e45a6d9c58` | `tools/bootstrap-resources.py` |
| `build/terminal/cargo` | 82 | 45 | 0 | 166,830,875 | `6edb080e09fd73838f62b26fcd992030cd318e7fdf530f821862d12330842c2a` | `cargo run -p xtask -- build-terminal` |
| `build/terminal/config` | 7 | 1 | 0 | 35,082 | `74a279662b3caba97b4e9ee70c9bc73bf0349da50c772477e31d4c24665c229d` | `cargo run -p xtask -- build-terminal` |
| `build/terminal/image` | 3 | 3 | 0 | 1,270,503 | `1ffef20bb1463c6dbd090544850176c4cdf431fe2654563c78c78c939f2c0ebf` | `build/terminal/port/runtime-image` |
| `build/terminal/image-staging` | 2 | 1 | 0 | 1,269,940 | `1983e32038f239a3ffb105c0571203f663fcd86f5454264ceb4392e84628ff73` | `build/terminal/port/runtime-image` |
| `build/terminal/lcms` | 5 | 4 | 0 | 360,608 | `a8ab18a88ab527f1f08f0b11184b537c8fb52bad32567a0b3048e25585a3f6f8` | `build/terminal/port/runtime-lcms` |
| `build/terminal/tls` | 26 | 11 | 0 | 7,878,424 | `d3af2adeb81366ed2da64467d9031f0f6853fa212b3b713b9cf38df8b168ffe6` | `build/terminal/port/runtime-tls` |
| `build/terminal/treesit` | 6 | 7 | 0 | 308,108 | `0fcc6476f7563c0a0a920f54f9c080793fb67d821217a4a0593a2a02c9e42aec` | `build/terminal/port/runtime-treesit` |

Retained active working set (and explicit retirement triggers):
- `build/terminal/port` (`43 MB`) and `dist/macos/Femacs.app` (`234 MB`): verified post-cleanup via `codesign --verify --deep --strict` and `-Q --batch` smoke (`(32.0.50 42)`, `(32.0.50 t)`).
- `build/cargo` (`888 MB`), `<session-cache>/u1-baseline`, and `<session-cache>/tree-sitter-0.27.1`: actively consumed by upcoming stages `U2–U5`; scheduled for retirement upon `AU5` completion.
