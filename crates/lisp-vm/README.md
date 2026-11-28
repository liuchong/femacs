# lisp-vm

[![crates.io](https://img.shields.io/crates/v/lisp-vm?logo=rust&label=crates.io)](https://crates.io/crates/lisp-vm)
[![docs.rs](https://img.shields.io/docsrs/lisp-vm)](https://docs.rs/lisp-vm)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](https://github.com/liuchong/femacs/blob/master/LICENSE)
[![Project website](https://img.shields.io/badge/docs-femacs.com-blue)](https://femacs.com/pages/lisp.html)

Standalone, zero-C-build-dependency Elisp-compatible Lisp virtual machine, garbage collector, bytecode engine, and low-level system runtime (`0.0.1`).

> Looking for the ergonomic high-level Rust embedder API (`Engine`, `Value`, `prelude`, and `std/*`)? See [`lisp`](https://crates.io/crates/lisp).
> Looking for the standalone CLI and interactive TTY REPL (`cargo install elisp`)? See [`elisp`](https://crates.io/crates/elisp).

## Capabilities

- **Zero External C Compiler Dependencies**: Builds cleanly without `cc`, `cmake`, `pkg-config`, `bindgen`, or external C `-sys` libraries, linking only against standard OS runtime libraries.
- **`host_runtime` (`HeadlessVm`)**: Anchored stack GC roots, `condition-case` isolation, cold-boot and script execution, `HostExtFn` host-callback registration (`VmValue::Bytes` unibyte buffers), and portable `.pdmp` dump/restore.
- **`sys` & Isolate Support**: Safe core algorithms, `VmGlobal`, POSIX/Windows compatibility wrappers, and opt-in `isolate_vm` thread-local `IsolateContext` storage (`IsolateScopeGuard`).
- **Safe Boundary Types**: `roots`, `nonlocal`, `state`, `safe_domain`, and `vm_primitives`.

## Feature Flags

| Feature | Default | Purpose |
| --- | --- | --- |
| `default` | `[]` | Single-process static-global VM layout preserved for editor hosts |
| `isolate_vm` | off | Opt-in per-isolate `IsolateContext` storage enabling multi-core `NoGil` execution (`Engine: Send` in [`lisp`](https://crates.io/crates/lisp)) |

## Low-Level Usage Sketch

```toml
[dependencies]
lisp-vm = { version = "0.0.1", features = ["isolate_vm"] }
```

```rust,no_run
use lisp_vm::host_runtime::{HeadlessVm, HostRuntimeConfig, VmValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = HeadlessVm::init(HostRuntimeConfig::default())?;
    let val = vm.eval_string("(+ 20 22)")?;
    assert!(matches!(val, VmValue::Int(42)));
    Ok(())
}
```

For almost all Rust application integrations, prefer [`lisp`](https://crates.io/crates/lisp) instead of calling `lisp-vm` directly.

## Sibling Crates

- [`lisp`](https://crates.io/crates/lisp) ([docs.rs](https://docs.rs/lisp)) — High-level `Engine`, `Value`, No-GIL worker pool, `.pdmp` warm cache, `pkg`, and `lisp::stdlib`
- [`lisp-std`](https://crates.io/crates/lisp-std) ([docs.rs](https://docs.rs/lisp-std)) — 100% pure-Rust `#![forbid(unsafe_code)]` standard library modules
- [`elisp`](https://crates.io/crates/elisp) — Standalone CLI, script runner, package manager, and interactive TTY REPL

## Platform Support

| Target | Status | Verification Scope |
| --- | --- | --- |
| macOS (`aarch64-apple-darwin`) | Verified | Real host `--lib`, `--features isolate_vm --lib`, and release system-only linkage |
| Linux (`aarch64-unknown-linux-gnu`) | Verified | Ephemeral Debian 12 Lima VM unit, `isolate_vm`, release `ldd`, `.pdmp`, `NoGil` execution |
| Linux (`x86_64-unknown-linux-gnu`) | Cross-compile verified | Clean `cargo check` |
| Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) | **Cross-compile ONLY** | Architecture verified (LLP64 layouts, UCRT errno, `mini-gmp`/`bignum` limb widths, `sys_jmp_buf`); **no physical Windows host runtime verification has been performed** |

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](https://github.com/liuchong/femacs/blob/master/LICENSE).
