# `lisp-vm` & `lisp-core` — Standalone General-Purpose Lisp Runtime Design

## 1. Positioning & Architecture Split

The repository extracts the pure-Rust Lisp VM (`crates/lisp-vm`) and public general-purpose runtime/CLI (`crates/lisp-core`) cleanly from the editor host (`crates/femacs-host`, `crates/femacs-tools`, `xtask`).

```
  Editor Consumers                   Standalone Language Consumers
  ┌──────────────────────────┐       ┌──────────────────────────────────┐
  │ femacs-host / femacs     │       │ lisp-core binary (`lisp`)        │
  │ femacs-tools / xtask     │       │ embedders (`lisp_core::Engine`)  │
  └────────────┬─────────────┘       └────────────────┬─────────────────┘
               │ default-features = false             │ default features (vm + repl-tty [+ std-batteries])
               ▼                                      ▼
  ┌──────────────────────────────────────────────────────────────────────┐
  │ crates/lisp-core (Engine, Value, Error, Brand, StdlibCache, Repl)    │
  └──────────────────────────────────┬───────────────────────────────────┘
                                     │
                                     ▼
  ┌──────────────────────────────────────────────────────────────────────┐
  │ crates/lisp-vm (Pure-Rust VM, Bytecode, GC, HeadlessVm, HostExt)     │
  └──────────────────────────────────────────────────────────────────────┘
```

### Non-Negotiable Isolation Rules

1. **Zero C compiler / C-sys dependencies in standalone builds**: `lisp-vm`, `lisp-core`, and upcoming `lisp-std` must never transitively pull `cc`, `cmake`, `pkg-config`, `bindgen`, `ring`, `aws-lc-*`, `rusqlite`, or `*-sys` crates.
2. **Editor build isolation**: `femacs-host`, `femacs-tools`, and `xtask` depend on `lisp-core` with `default-features = false`, so they never pull TTY REPL dependencies (`rustyline`), standalone network/DB batteries, or non-editor features.
3. **Centralized branding (`crates/lisp-core/src/brand.rs`)**: Crate/CLI/env/cache identifiers live in one `#![forbid(unsafe_code)]` module so pre-publish naming stays trivial.

## 2. Core Capabilities

### 2.1 Host Extension Bridge (`HostFnRegistry` & `Value::Bytes`)

- `Value::Bytes(Vec<u8>)` / `VmValue::Bytes(Vec<u8>)` represent raw byte buffers backed by unibyte Lisp strings (`make_unibyte_string` in `lisp-vm`). On return from Lisp into Rust, unibyte non-ASCII strings stay distinct from multibyte UTF-8 text.
- `Engine::register_fn(name, Fn)` registers host Rust callbacks into a thread-safe `HostFnRegistry` exposed inside the Lisp runtime via `internal-host-call` (registered in `init_headless_host` after VM initialization so both cold boot and `.pdmp` warm boot work without custom dump relocation).

### 2.2 Silent Cold Boot & Atomic Auto-Warm `.pdmp` Cache

- `stdlib/core-boot.el` honors `LISP_CORE_VERBOSE`: when unset, cold bootstrap passes non-nil `nomessage` to `(load ...)` calls so CLI startup emits zero diagnostic noise on stderr (`--verbose` or `LISP_CORE_VERBOSE=1` restores load traces).
- `stdlib_cache::ensure_warm_dump` and CLI `cache warm` write portable dump images via `.tmp.<pid>` + atomic `rename` under an advisory `.warm.lock` guard.
- Interactive/CLI invocations (`eval`, `run`, `repl`, `compile`, `test`) automatically trigger detached background warming unless `--no-auto-warm` or `LISP_CORE_NO_AUTO_WARM=1` is set, making subsequent launches start from `.pdmp` in milliseconds.

### 2.3 Pure-Rust Interactive TTY REPL (`crates/lisp-core/src/repl.rs`)

- Controlled by the `repl-tty` feature (enabled by default in `lisp-core`; omitted when `default-features = false`).
- Non-TTY stdin runs a clean line-oriented read-eval-print loop with buffered multi-line form accumulation.
- Interactive TTY stdin uses pure-Rust `rustyline` (`default-features = false`) for:
  - multi-line incomplete s-expression detection (`Validator`),
  - Lisp symbol completion (`Completer` backed by `Engine::complete_symbols`),
  - persistent history (`history.txt` under cache dir),
  - clean `Ctrl-C` line abort recovery and `Ctrl-D` graceful exit (`0`).

## 3. Multi-Phase Evolution

- **Phase 1 (Current)**: `HostFnRegistry` + `Value::Bytes`, `brand.rs`, silent cold boot + atomic background `.pdmp` warm cache, pure-Rust TTY REPL, dependency isolation + zero-`cc` static gate (`tools/check-crate-boundaries.py`).
- **Phase 2**: Configurable No-GIL isolate VM (`isolate_vm` feature, thread-local isolate globals, `Engine: Send`, `--concurrency=no-gil|gil`, `std/task` / `std/chan` / `std/sync`), keeping `femacs-host` single-isolate compatible.
- **Phase 3**: Pure-Rust `crates/lisp-std` standard library (`ureq`+`rustls`+`rustls-rustcrypto`, `redb`+SQL, archives, serialization, crypto, FS/CLI/log/proc) and `init`/`pkg` project workflow.
- **Phase 4**: Linux (`aarch64`/`x86_64`) OS layer & real-machine VM verification.
- **Phase 5**: Pure-Rust Windows OS layer & cross-compile check verification.
- **Phase 6**: 0-warning polish, multi-crate `cargo package` verification, documentation, and pre-publish rename checklist.
