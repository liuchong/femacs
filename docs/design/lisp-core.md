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
  ┌──────────────────────────────────┬───────────────────────────────────┐
  │ crates/lisp-std (Pure-Rust std/* HTTP/redb+SQL/formats/crypto/fs)    │
  └──────────────────────────────────┬───────────────────────────────────┘
                                     │ (std-batteries, standalone only)
                                     ▼
  ┌──────────────────────────────────────────────────────────────────────┐
  │ crates/lisp-core (Engine, Value, Error, Brand, StdlibCache, Repl,Pkg)│
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

## 3. Multi-Core No-GIL VM & Concurrency (`Phase 2`)

- **Isolate-local VM Storage (`crates/lisp-vm/src/sys/isolate.rs`)**: Enabled via `lisp-vm/isolate_vm` (default in `lisp-core`, disabled in `femacs-host`/`femacs` editor builds to keep single-VM static C-ABI behavior intact).
- **Modes (`ConcurrencyMode::NoGil | Gil`)**:
  - `NoGil` (default in `lisp-core`): Each `Engine` owns an isolated heap, `VmGlobal` table, GC root/nonlocal state, and pdump mapping; `Engine` is `Send` across OS threads and can run multiple VMs concurrently across CPU cores with zero GIL contention.
  - `Gil`: Preserves single-instance process semantics (`Error::AlreadyInitialized` on 2nd init) and disables `std/task` parallel spawning (`--concurrency=gil` or `LISP_CONCURRENCY=gil`).
- **Stdlib Modules (`stdlib/std/{chan,sync,task}.el`)**: Pure-Rust `#![forbid(unsafe_code)]` MPMC channels (`std-chan-*`), atomics/mutexes/shared maps (`std-atomic-*`, `std-mutex-*`, `std-shared-*`), OS-thread task spawn/await (`std-task-spawn`, `std-task-await`), bounded worker pools (`std-task-pool-*`), and multi-core `std-task-parallel-map`.

## 4. Pure-Rust Standard Library (`crates/lisp-std`) & Project Workflow (`Phase 3`)

- **100% Pure-Rust `crates/lisp-std` (`#![forbid(unsafe_code)]`, zero `cc` or C `-sys` crates)**:
  - Enabled by default inside `lisp-core` via `std-batteries = ["vm", "dep:lisp-std"]`; excluded from `femacs-host`, `femacs-tools`, and `xtask` (`default-features = false`).
  - Shared `StdRuntime` attached to both root `Engine` and `NoGil` `ConcurrencyHub` worker isolates so parallel tasks can use `std/*` capabilities directly.
  - Modules (`stdlib/std/*.el`):
    - `std/http`: HTTPS/HTTP client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots` + `gzip`) and embedded multi-threaded HTTP/1.1 TCP server (`std-http-server-*`, `std-http-serve`).
    - `std/kv` & `std/db`: Embedded ACID key-value store (`redb`) and pure-Rust transactional SQL engine (`CREATE/DROP TABLE`, `INSERT`, `SELECT ... WHERE ... ORDER BY ... LIMIT/OFFSET`, `COUNT(*)`, `UPDATE`, `DELETE`, `BEGIN`/`COMMIT`/`ROLLBACK`, `:memory:` & file persistence).
    - `std/json`, `std/toml`, `std/csv`, `std/yaml`, `std/url`, `std/base64`: Structured codecs, URL parser/builder, and Base64 standard/URL-safe codecs.
    - `std/archive`: Pure-Rust gzip/zlib (`flate2` `rust_backend`), `tar`/`tar.gz` (`tar`), and `zip` (`zip` `deflate`) with path-traversal and symlink rejection.
    - `std/crypto` & `std/time`: `getrandom` CSPRNG, SHA-256/512, HMAC-SHA256, UUID v4/v7, hex, constant-time compare, and `jiff` UTC/RFC-3339 timestamps & monotonic clocks.
    - `std/fs`, `std/path`, `std/regex`, `std/proc`, `std/env`, `std/cli`, `std/log`: Recursive `walkdir`/`globset` traversal, linear-time `regex`, timeout-bounded `std/proc`, `.env` parser/loader, declarative CLI option parser, and structured text/JSON logging.
- **Project Scaffolding & Package Management (`crates/lisp-core/src/pkg.rs`)**:
  - `lisp-core init [DIR] [--name NAME] [--force]` creates `lisp.toml`, `src/main.el`, `tests/main-test.el`, and `.gitignore`.
  - `lisp-core pkg <add|install|list|update|remove>` resolves `--path` and `--url` dependencies into `.lisp-packages/`, records deterministic SHA-256 checksums atomically in `lisp.lock`, and automatically injects project `src/` + `.lisp-packages/*/src` into `load-path` (plus zero-arg `lisp-core run` / `lisp-core test` inside `lisp.toml` projects).

## 5. Multi-Phase Evolution

- **Phase 1 (Complete)**: `HostFnRegistry` + `Value::Bytes`, `brand.rs`, silent cold boot + atomic background `.pdmp` warm cache, pure-Rust TTY REPL, dependency isolation + zero-`cc` static gate (`tools/check-crate-boundaries.py`).
- **Phase 2 (Complete)**: Configurable No-GIL isolate VM (`isolate_vm` feature, thread-local isolate globals, `Engine: Send`, `--concurrency=no-gil|gil`, `std/task` / `std/chan` / `std/sync`), keeping `femacs-host` single-isolate compatible.
- **Phase 3 (Complete)**: Pure-Rust `crates/lisp-std` standard library (`ureq`+`rustls`+`rustls-rustcrypto`, `redb`+SQL, archives, serialization, crypto, FS/CLI/log/proc), `std/*` wrappers, and `init`/`pkg` project workflow.
- **Phase 4 (Complete)**: Linux (`aarch64` real-VM verified on Debian 12 `6.1.190-1`, `x86_64-unknown-linux-gnu` cross-checked) pure-Rust/glibc OS & ABI layer across `lisp-vm`/`lisp-core` (`procfs` memory/process-tree accounting, `util-linux` `script` PTY wrapper, glibc struct layouts & `__sigsetjmp` / `posix_spawn_file_actions_addchdir_np` / `renameat2`, ZST `IsolateContext` ELF `.bss` slot guard, glibc `rpl_realloc`, native `getrandom`/`copy_file_range` resolution, Linux signal/wait/fcntl ABI constants, and multi-isolate `SIGCHLD`/`pthread_kill(0)` safety).
- **Phase 5 (Complete — cross-compile verified only, no physical Windows host)**: Pure-Rust Windows OS backend across `crates/lisp-core` (`windows-sys` Job Objects + `CreateToolhelp32Snapshot` process-tree inspection + `K32GetProcessMemoryInfo`/`GetProcessTimes` + `SetConsoleCtrlHandler`, cross-platform `RpcTransport` with `--rpc-addr` TCP loopback, `LOCALAPPDATA`/`USERPROFILE` cache root) and `crates/lisp-vm` (LLP64 struct layouts, UCRT `_errno` & constants, `mini-gmp`/`bignum` limb widths, `jmp_buf`/`sys_jmp_buf`, `struct face` padding, gnulib shim gating), cross-checked cleanly with `cargo check` on `x86_64-pc-windows-msvc` and `x86_64-pc-windows-gnu` (default, `--no-default-features --features vm`, and `--tests`; **cross-compile architecture verified ONLY — no physical Windows host runtime verification performed**).
- **Phase 6 (Complete)**: 0-warning polish across all feature matrices and rustdoc (`cargo check --no-default-features --all-targets` / `vm` / `repl-tty` / `std-batteries`, optional `flate2`), `crates/lisp-std/LICENSE`, standalone user/embedder `README.md`s, multi-crate `cargo package` + isolated extraction rebuild verification, and pre-publish crate rename checklist.

## 6. Pre-Publish Crate Rename & Release Checklist

### Publish Order (Topological)
1. `lisp-vm` (`crates/lisp-vm`) — zero workspace crate dependencies; can pass full `cargo package` / `cargo publish --dry-run` immediately.
2. `lisp-std` (`crates/lisp-std`) — zero workspace crate dependencies (`#![forbid(unsafe_code)]`, pure crates.io deps); passes full `cargo package` / `cargo publish --dry-run` immediately.
3. `lisp-core` (`crates/lisp-core`) — depends on `lisp-vm` and `lisp-std` (`optional = true`); requires `lisp-vm` and `lisp-std` published first (or local `[patch.crates-io]` verification before publish).

### Pre-Publish One-Step Rename Touchpoints
- Runtime CLI/cache/env branding (single module): `crates/lisp-core/src/brand.rs` (`CLI_NAME`, `CACHE_APP_DIR`, `CACHE_FALLBACK_TEMP_DIR`, `REPL_PROMPT`, `ENV_*`).
- Package manifests & versions: `crates/lisp-vm/Cargo.toml`, `crates/lisp-std/Cargo.toml`, `crates/lisp-core/Cargo.toml` (`[package].name`, `[[bin]].name`, `[dependencies]`).
- Editor workspace consumers (`default-features = false` isolation preserved): `crates/femacs-host/Cargo.toml`, `crates/femacs-tools/Cargo.toml`, `xtask/Cargo.toml`, root `Cargo.toml` (`[profile.*.package.*]`).
- Gates & docs: `tools/check-crate-boundaries.py`, `tools/unsafe-boundary-manifest.json`, `crates/{lisp-vm,lisp-std,lisp-core}/README.md`, and `docs/design/lisp-core.md`.

### Pre-Publish Verification Commands
- `cargo package -p lisp-vm && cargo package -p lisp-std && cargo package -p lisp-vm -p lisp-core -p lisp-std --no-verify` (check archive size `< 10 MiB`, `LICENSE`/`README.md` included, `0` path escapes, isolated extraction rebuild passing `--version`, `eval`, and `std/*`).
- `cargo check -p lisp-core --no-default-features --all-targets` and `python3 tools/check-crate-boundaries.py` + `python3 tools/check-own-voice.py`.
- `cargo publish --dry-run -p <vm-crate>` and `-p <std-crate>`, then live publish in topological order (`lisp-vm` -> `lisp-std` -> `lisp-core`).

### Platform Support Disclosure
- **macOS (`aarch64-apple-darwin`)**: Full real-host verified (`cargo test`, `isolate_vm`, release `otool -L` OS-libs-only, `cargo package` isolated rebuild, and `fe-test`).
- **Linux (`aarch64-unknown-linux-gnu` / `x86_64-unknown-linux-gnu`)**: `aarch64` verified inside ephemeral Debian 12 Lima VM (`ldd` glibc-only, warm `.pdmp`, PTY REPL, `NoGil` 4-worker speedup, `init`/`pkg`); `x86_64` cross-compile verified.
- **Windows (`x86_64-pc-windows-msvc` / `x86_64-pc-windows-gnu`)**: **Cross-compile architecture verified ONLY** (`cargo check` green across default, minimal `vm`, and `--tests`); **no physical Windows host runtime verification has been performed**.
