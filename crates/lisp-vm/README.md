# lisp-vm

Standalone headless Lisp virtual machine, garbage collector, bytecode engine, and low-level system runtime (`0.0.1`).

By default, `lisp-vm` builds with **zero external C build or dynamic library dependencies** (linking only standard OS runtime libraries) and exposes:

- `sys`: safe core algorithms, `VmGlobal`, POSIX/Windows compatibility wrappers, and `isolate_vm` per-isolate state (`IsolateContext`, `IsolateScopeGuard`).
- `host_runtime`: headless VM runtime (`HeadlessVm`) with anchored stack GC roots, `condition-case` isolation, cold-boot, script execution, `HostExtFn` host-callback registration (`VmValue::Bytes` unibyte buffer support), and portable `.pdmp` dump/restore.
- `roots`, `nonlocal`, `state`, `safe_domain`, `vm_primitives`: memory-safe Rust boundary types over the VM.

## Feature Isolation

- `default = []`: Single-process static-global mode preserved for editor consumers.
- `isolate_vm`: Opt-in per-isolate VM storage enabling multi-core `NoGil` execution (`Engine: Send` in `lisp-core`) without impacting editor single-VM targets.

For high-level Rust embedding (`Engine`, `Value`, `Error`), embedded standard library caching, pure-Rust `std/*` batteries, and the standalone CLI, see [`lisp-core`](../lisp-core/README.md).

## Platform Support

- **macOS (`aarch64-apple-darwin`)**: Real-host verified (`--lib`, `--features isolate_vm --lib`, release linkage).
- **Linux (`aarch64` / `x86_64`)**: Real Debian 12 `aarch64` VM verified + `x86_64-unknown-linux-gnu` cross-checked.
- **Windows (`x86_64-pc-windows-msvc` / `x86_64-pc-windows-gnu`)**: **Cross-compile architecture verified ONLY** (LLP64 layouts, UCRT errno, `mini-gmp`/`bignum` limb widths, `sys_jmp_buf`); no physical Windows host runtime execution has been performed.

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
