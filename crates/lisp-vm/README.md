# lisp-vm

Standalone headless Lisp virtual machine, garbage collector, bytecode engine, and low-level system runtime (`0.0.1`).

By default, `lisp-vm` builds with **zero external C dynamic library dependencies** (linking only the standard OS C runtime) and exposes:

- `sys`: safe core algorithms, `VmGlobal`, POSIX wrappers, and client utilities.
- `host_runtime`: single-process `HeadlessVm` runtime with anchored stack GC roots, `condition-case` isolation, cold-boot, script execution, and portable `.pdmp` dump/restore.
- `roots`, `nonlocal`, `state`, `safe_domain`, `vm_primitives`: memory-safe Rust boundary types over the VM.

For high-level Rust embedding (`Engine`, `Value`, `Error`), embedded standard library caching, and standalone CLI tools, see [`lisp-core`](../lisp-core/README.md).

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
