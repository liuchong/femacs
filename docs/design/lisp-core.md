# `lisp` & `elisp` — Standalone General-Purpose Lisp Runtime Architecture

## 1. 4-Crate Positioning & Isolation Boundaries

The repository cleanly separates the standalone general-purpose Lisp language crates (`lisp-vm`, `lisp-std`, `lisp`, `elisp`) from the editor workspace (`femacs-host`, `femacs-tools`, `xtask`):

```
  CLI Users (`cargo install elisp`)        Rust Embedders (`cargo add lisp`)
  ┌──────────────────────────────────┐     ┌──────────────────────────────────┐
  │ crates/elisp (pure [[bin]])      │     │ crates/lisp (pure [lib])         │
  │ CLI, --help, REPL (rustyline)    │────►│ default = ["vm", "std"]          │
  │ default = ["repl-tty", "std"]    │     │ Engine/Value/Error/prelude/pkg/  │
  └──────────────────────────────────┘     │ exec/rpc/stdlib_cache + re-exports│
                                           └───────┬─────────────────┬────────┘
                                                   │ optional        │ optional
                                                   ▼                 ▼
  ┌───────────────────────────────────┐    ┌───────────────┐  ┌───────────────┐
  │ Editor Host & Tooling             │    │crates/lisp-std│  │crates/lisp-vm │
  │ femacs-host / femacs-tools / xtask│───►│ 100% pure Rust│  │ VM/GC/bytecode│
  │ xtask -> lisp (default=false)     │    │ std/* modules │  │ isolate_vm    │
  └───────────────────────────────────┘    └───────────────┘  └───────────────┘
```

### Feature Matrix

- **`crates/lisp-vm`**: `default = []`; opt-in `isolate_vm` enables per-isolate No-GIL storage for standalone `lisp` while preserving single-VM static C-ABI behavior in editor builds.
- **`crates/lisp-std`**: `default = []`; 100% `#![forbid(unsafe_code)]`, zero `cc` or C `-sys` dependencies.
- **`crates/lisp`**: Pure `[lib]`:
  - `default = ["vm", "std"]` (full embeddable runtime + `22` `std/*` pure-Rust modules + `lisp::prelude`, `lisp::vm`, `lisp::stdlib`)
  - `vm = ["dep:lisp-vm", "lisp-vm/isolate_vm", "dep:flate2"]` (lightweight zero-HTTP/zero-DB embeddable kernel)
  - `std = ["vm", "dep:lisp-std"]` (pure-Rust batteries)
  - `pkg = ["vm", "dep:toml"]` (`lisp.toml` / atomic `lisp.lock` / `.lisp-packages/` resolver)
  - `default-features = false`: zero-VM `Value`, `Error`, `exec`, `rpc`, `stdlib_cache`, `brand` shared with `xtask`
- **`crates/elisp`**: Pure `[[bin]]` (`elisp`):
  - `default = ["repl-tty", "std"]` (`lisp` with `features = ["vm", "pkg"]`, plus optional `rustyline` TTY editor and `lisp/std`)

### Non-Negotiable Isolation Rules

1. **Zero C compiler / C `-sys` dependencies in standalone crates**: `lisp-vm`, `lisp-std`, `lisp`, and `elisp` must never transitively pull `cc`, `cmake`, `pkg-config`, `bindgen`, `ring`, `aws-lc-*`, `rusqlite`, or non-whitelisted `*-sys` crates (`windows-sys` only on Windows).
2. **Editor build isolation**: `femacs-host`, `femacs-tools`, and `xtask` never pull `elisp`, `lisp-std`, `rustyline`, `ureq`, `rustls`, or `redb` (`xtask` uses `lisp` with `default-features = false`).
3. **Centralized runtime environment & cache identifiers**: Managed in `crates/lisp/src/brand.rs` (`LISP_CACHE_DIR`, `LISP_VERBOSE`, `LISP_NO_AUTO_WARM`, `LISP_CONCURRENCY`, `~/.cache/lisp/`); CLI presentation (`elisp`, `elisp> `, help/errors) lives strictly in `crates/elisp/src/`.

## 2. Core Capabilities

### 2.1 Host Extension Bridge (`HostFnRegistry` & `Value::Bytes`)

- `Value::Bytes(Vec<u8>)` / `VmValue::Bytes(Vec<u8>)` represent raw byte buffers backed by unibyte Lisp strings (`make_unibyte_string` in `lisp-vm`). On return from Lisp into Rust, unibyte non-ASCII strings stay distinct from multibyte UTF-8 text.
- `Engine::register_fn(name, Fn)` registers host Rust callbacks into a thread-safe `HostFnRegistry` exposed inside the Lisp runtime via `internal-host-call` (registered in `init_headless_host` after VM initialization so both cold boot and `.pdmp` warm boot work without custom dump relocation).

### 2.2 Silent Cold Boot & Atomic Auto-Warm `.pdmp` Cache

- `stdlib/core-boot.el` honors `LISP_VERBOSE`: when unset, cold bootstrap passes non-nil `nomessage` to `(load ...)` calls so CLI startup emits zero diagnostic noise on stderr (`--verbose` or `LISP_VERBOSE=1` restores load traces).
- `stdlib_cache::ensure_warm_dump` and `elisp cache warm` write portable dump images via `.tmp.<pid>` + atomic `rename` under an advisory `.warm.lock` guard.
- CLI invocations (`eval`, `run`, `repl`, `compile`, `test`) automatically trigger detached background warming unless `--no-auto-warm` or `LISP_NO_AUTO_WARM=1` is set.

### 2.3 Pure-Rust Interactive TTY REPL (`crates/elisp/src/repl.rs`)

- Controlled by `repl-tty` inside `crates/elisp` (excluded from all 4 library crates).
- Non-TTY stdin runs a clean line-oriented read-eval-print loop with buffered multi-line form accumulation.
- Interactive TTY stdin uses pure-Rust `rustyline` (`default-features = false`) for multi-line incomplete s-expression detection, Lisp symbol completion (`Engine::complete_symbols`), persistent `history.txt`, clean `Ctrl-C` line abort recovery, and `Ctrl-D` exit (`0`).

## 3. Multi-Core No-GIL VM & Concurrency

- **Isolate-local VM Storage (`crates/lisp-vm/src/sys/isolate.rs`)**: Enabled via `lisp-vm/isolate_vm` (used by `lisp`/`elisp`, disabled in `femacs-host` editor builds to keep single-VM static C-ABI behavior intact).
- **Modes (`ConcurrencyMode::NoGil | Gil`)**:
  - `NoGil` (default): Each `Engine` owns an isolated heap, `VmGlobal` table, GC root/nonlocal state, and pdump mapping; `Engine` is `Send` across OS threads and can run multiple VMs concurrently across CPU cores with zero GIL contention.
  - `Gil`: Preserves single-instance process semantics (`Error::AlreadyInitialized` on 2nd init) and disables `std/task` parallel spawning (`--concurrency=gil` or `LISP_CONCURRENCY=gil`).
- **Stdlib Modules (`stdlib/std/{chan,sync,task}.el`)**: Pure-Rust `#![forbid(unsafe_code)]` MPMC channels (`std-chan-*`), atomics/mutexes/shared maps (`std-atomic-*`, `std-mutex-*`, `std-shared-*`), OS-thread task spawn/await (`std-task-spawn`, `std-task-await`), bounded worker pools (`std-task-pool-*`), and multi-core `std-task-parallel-map`.

## 4. Pure-Rust Standard Library (`crates/lisp-std`) & Project Workflow

- **100% Pure-Rust `crates/lisp-std` (`#![forbid(unsafe_code)]`, zero `cc` or C `-sys` crates)**:
  - Enabled by `lisp`'s `std` feature (`default = ["vm", "std"]`) and `elisp`'s `std` feature (`default = ["repl-tty", "std"]`); excluded from editor targets.
  - Shared `StdRuntime` attached to both root `Engine` and `NoGil` `ConcurrencyHub` worker isolates so parallel tasks use `std/*` capabilities directly (`std/http`, `std/kv`, `std/db`, `std/{json,toml,csv,yaml,url,base64}`, `std/archive`, `std/{crypto,time}`, `std/{fs,path}`, `std/{regex,proc,env,cli,log}`).
- **Project Scaffolding & Package Management (`crates/lisp/src/pkg.rs`, `pkg` feature)**:
  - `elisp init [DIR] [--name NAME] [--force]` creates `lisp.toml`, `src/main.el`, `tests/main-test.el`, and `.gitignore`.
  - `elisp pkg <add|install|list|update|remove>` resolves `--path` and `--url` dependencies into `.lisp-packages/`, records deterministic SHA-256 checksums atomically in `lisp.lock`, and automatically injects project `src/` + `.lisp-packages/*/src` into `load-path`.

## 5. Multi-Phase Roadmap

- **Phase 1–6 (Complete)**: `HostFnRegistry` + `Value::Bytes`, `NoGil` `isolate_vm`, 100% pure-Rust `crates/lisp-std` + `init`/`pkg`, Linux real Debian 12 VM verification, Windows `windows-sys` cross-compile backend, and zero-warning packaging polish.
- **4-Crate Consolidation (`CLN318`)**: Converted `crates/lisp-core` into pure embeddable library `crates/lisp` (with `vm`, `std`, `pkg` features and unified `LISP_*` env vars), extracted standalone CLI binary crate `crates/elisp`, decoupled `femacs-host`/`xtask`, updated crate-boundary and unsafe-boundary gates across all 4 packages, and verified isolated 4-crate packaging.

## 6. Release Status & Verification Checklist (`v0.0.1`)

See [docs/release-process.md](../release-process.md) for the full programmatic 5-crate release workflow (`cargo run -p xtask -- release`, counting already-published `femacs 0.0.1` into `v0.0.1` and synchronizing future 5-crate releases).

### Topological Publish Order (`0.0.1`)
1. `lisp-vm` (`crates/lisp-vm`), `lisp-std` (`crates/lisp-std`), & `femacs` (`cli/`, skipped via `--skip-already-published` when `0.0.1` is already live) — zero workspace crate dependencies (`homepage = "https://femacs.com/"`).
2. `lisp` (`crates/lisp`) — depends on `lisp-vm` and `lisp-std` (`categories = ["compilers", "concurrency", "development-tools"]`).
3. `elisp` (`crates/elisp`) — depends on `lisp` (aggregating all three lower crates into standalone binary `elisp`).

### Release Verification Gates
- `cargo package -p lisp-vm && cargo package -p lisp-std && cargo package -p lisp-vm -p lisp-std -p lisp -p elisp --no-verify` (archive size `< 10 MiB`, `LICENSE` + independent `README.md` without `../` links, `0` path escapes, and isolated extraction `release` rebuild of `elisp` passing `--version`, `--help`, `eval`, and `std/*`).
- `python3 tools/check-crate-boundaries.py`, `python3 tools/check-own-voice.py`, and `fe-test --suite all --no-cache`.
- GPG-signed tag `v0.0.1` (`git tag -s v0.0.1`) and GitHub Release publishing the `0.0.1` crates.io & platform verification notes.

### Platform Support Disclosure
- **macOS (`aarch64-apple-darwin`)**: Full real-host verified (`cargo test`, `isolate_vm`, release `otool -L` OS-libs-only, `cargo package` isolated rebuild, and `fe-test`).
- **Linux (`aarch64-unknown-linux-gnu` / `x86_64-unknown-linux-gnu`)**: `aarch64` verified inside ephemeral Debian 12 Lima VM (`ldd` glibc-only, warm `.pdmp`, PTY REPL, `NoGil` 4-worker speedup, `init`/`pkg`); `x86_64` cross-compile verified.
- **Windows (`x86_64-pc-windows-msvc` / `x86_64-pc-windows-gnu`)**: **Cross-compile architecture verified ONLY** (`cargo check` green across all 4 crates); **no physical Windows host runtime verification has been performed**.
