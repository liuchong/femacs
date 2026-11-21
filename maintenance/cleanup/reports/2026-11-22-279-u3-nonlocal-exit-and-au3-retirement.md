# CLN279 — U3 explicit non-local exits, `.eln` ABI status protocol, and retirement of consumed AU3 artifacts

Completed Phase 0 Stage `U3` (`显式退出`, `P-482`) and formal acceptance `AU3` (`非局部退出等价`):
- Replaced Lisp-level `sys_setjmp` / `sys_longjmp` across `src/eval.rs`, `src/bytecode.rs`, `src/comp.rs`, `src/emacs-module.rs`, `src/editfns.rs`, `src/treesit.rs`, `src/keyboard.rs`, and `src/thread.rs` with typed `VmResult<T>` (`Result<T, NonLocalExit>`) and deterministic `extern "C-unwind"` stack unwinding (`Cargo.toml` `[profile.release] panic = "unwind"`), backed by `crates/femacs-vm/src/nonlocal.rs` (`ScopedLispRoot`, `HostDropGuard`, `catch_vm_nonlocal`, and `.eln` pending-exit status propagation).
- Upgraded `src/comp.rs` to `ABI_VERSION` `"14"`, removing `emit_setjmp` and `_setjmp` imports from generated `.eln` units in favor of 28 non-unwinding `comp_*` status helpers and explicit JIT status/handler dispatch blocks; isolated external C dynamic modules (`module_with_nonlocal_catch`, `module_signal_or_throw` `extern "C-unwind"`) and `pthread_create` (`run_thread` `catch_vm_nonlocal`) so zero unwinds cross C FFI boundaries; hardened `flush_stack_call_func` register-spill frame lifetime and `src/print.rs` `prstack` reallocation / RAII unwind guards.
- Verified `AU3` (`tools/test-u3-nonlocal-exit.py` `4/4` passed in `15.891s`): legacy/wrong-ABI `.eln` rejected with `native-lisp-file-inconsistent` on both `normal` and `ENABLE_CHECKING` native builds, newly compiled `.eln` (`comp-abi-hash = 1f71b1b1`) verified with zero `setjmp`/`longjmp` imports, `882`-byte canonical event trace matched identically across Interpreter, Bytecode, Native `.eln` (`normal` + `checking`), Cocoa, Terminal, and nested C dynamic module frames, and `HostDropGuard`/`ScopedLispRoot`/`c_frames` balanced exact-once (`9/9` drops, `12/12` C frames) across all modes.
- Verified protective unit tests (`tools/test-u3-unit.py` `3/3` groups / `5/5` isolated `nonlocal.rs` Rust unit tests, `femacs-pure` `7/7`, `check-mirror-dupes.py` `0` failures, `check-unsafe-boundary.py` `0` errors with `safety_documented_blocks` `92 -> 104`).

Retired `18,666` ordinary files, `921` directories, and `18` symbolic links across `15` consumed build and verification scopes totaling `1,948,903,162` logical bytes:

| Removed scope | Files | Directories | Symlinks | Logical bytes | Tree SHA-256 | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `build/runtime` | 9,422 | 449 | 12 | 954,076,260 | `b83e1dd520bb5998a842e1b48f25a8188ead4803851f1c4ff22f1618426cc302` | `cargo run -p xtask -- runtime-build --product terminal --variant native --backend-prefix <libgccjit> --out build/runtime/u3-native` |
| `build/macos` | 210 | 99 | 6 | 286,495,438 | `47cb5347248831a403378d359b8b6f90b7db959e0bfc9cca1b94c1a45a2aa5aa` | `dist/macos/Femacs.app`; `cargo run -p xtask -- build-macos` |
| `build/manuals` | 5,940 | 201 | 0 | 474,130,627 | `f6737318d11f56226888d51f86e593b7d3920b844d6cb61ff347535db07b6a09` | `cargo run -p xtask -- build-manuals` |
| `build/generation` | 2,809 | 29 | 0 | 1,286,119 | `c277720e585ad5bd726bee90fd604a690e0f54e0f8fe266dfaf5c7b39cceb03e` | `tools/bootstrap-resources.py` via `xtask build-terminal` / `build-macos` |
| `build/acceptance` | 36 | 16 | 0 | 497,096 | `a283e250264ffa20dcefe73c46b98041cf2e9c7a4e28ebe3354990924bd1e38c` | `python3 tools/test-u3-nonlocal-exit.py` & `python3 tools/test-u3-unit.py` |
| `build/cargo-tools` | 118 | 55 | 0 | 35,879,990 | `21381f138d6fd640cc5dcbf93ca40a9de019eee46ad93efbe51a271120b28b49` | `cargo run -p xtask -- link-program <tool>` |
| `build/terminal/cargo` | 80 | 45 | 0 | 168,123,839 | `6ef938eee324c8144a7009d050e329bb83cf59b3c795a27666175fc4301dcd9b` | `cargo run -p xtask -- build-terminal` |
| `build/terminal/config` | 7 | 1 | 0 | 35,082 | `5aa28ac689669bf93a8192e26a3f9e7397cea00bb4df5260a21d8ed438ff9544` | `cargo run -p xtask -- build-terminal` |
| `build/terminal/image` | 3 | 3 | 0 | 1,270,503 | `5c13a747ca4a5013496a40e98ebd9a44f96174ec90bd93e6ad5db92cec545add` | `build/terminal/port/runtime-image` |
| `build/terminal/image-staging` | 2 | 1 | 0 | 1,269,940 | `d6f969bc2794a9ecf78bc80b13e552c0d721ce7eea2459db7736f2684d03b7da` | `build/terminal/port/runtime-image` |
| `build/terminal/lcms` | 5 | 4 | 0 | 360,608 | `46c5cc3547cb632882f19efef14f1fced29dfd1490d7f5dc8eafc4b2e95b1fa8` | `build/terminal/port/runtime-lcms` |
| `build/terminal/tls` | 26 | 11 | 0 | 7,878,424 | `0db12ef8f33402fc52b59e60faeeb457f08c33b19d43e316d2e5834e6172b317` | `build/terminal/port/runtime-tls` |
| `build/terminal/treesit` | 6 | 7 | 0 | 308,108 | `b72ecedc8d72ace68b9993c556780804a100c77b2a255c19ff3bf85f916addb3` | `build/terminal/port/runtime-treesit` |
| `build/terminal/port/src/emacs-32.0.50.1` | 1 | 0 | 0 | 4,189,760 | `3a77f2ec3b42446d129160df1c58ed7b0e3029a3813352e1066885abd9b0c622` | `build/terminal/port/src/emacs` (`P-190` dump hardlink alias) |
| `build/terminal/port/src/emacs-32.0.50.1.pdmp` | 1 | 0 | 0 | 13,101,368 | `eca05ed9b1ecaad4914aab2754ab154aaa3d97ead9cef6598a34b769417094a3` | `build/terminal/port/src/emacs.pdmp` (`P-190` dump alias) |

Retained active working set (and explicit retirement triggers):
- `build/terminal/port` (`44 MB`: `emacs` `a2e67f482afa14a1f2506ee616f41c3dd91e70bb33716298a1797704e8636f86`, `emacs.pdmp` `9b37fd1e3edbb68b1113d0636b9c2b0256f8cc41f975673c80b245b3383d5833`, `emacsclient` `6d676352ba2649fa90143258ccbc11472a2c6816dd8f7b0b716f16eabc052160`, `hexl` `d44f1c449986013e4a8acef87036b8903ebb27b57442f093ef0165953103b840`) and `dist/macos/Femacs.app` (`235 MB`: `Femacs` `e6b9b5fe7016883ef372e053145632414e07e6e1fd0d00f7b68381fce426f8d1`, `Femacs.pdmp` `4df4f2f77518eb6516cf45d916465e4946dee39321818bfeadb18d85e0d1cb0a`, `emacsclient` `95b4ad24ac0b9671da60764a46b6ac6b8ea12596485dbbcb4f5f7e885d0a9e87`, `hexl` `3350e1b2d8141c3d6f983d6f787271c2519bf3811156fa5e96443467b5819e07`), verified post-cleanup via strict `codesign` and `-Q --batch` smoke.
- `build/cargo` (`962 MB`), `<session-cache>/u1-baseline`, and `<session-cache>/tree-sitter-0.27.1`: actively consumed by upcoming stages `U4–U5`; scheduled for retirement upon `AU5` completion.
