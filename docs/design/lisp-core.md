# Standalone Lisp Core (`lisp-vm` + `lisp-core` `0.0.1`)

## 1. Positioning & Scope

`lisp-core` (backed by `lisp-vm`) extracts the headless Lisp runtime from the editor repository into a **self-contained, general-purpose Lisp programming language** engine and CLI (`version = "0.0.1"`), publishable to crates.io with zero external repository path escapes and zero non-system C dynamic library dependencies by default.

Design goals:
- **General-purpose language surface**: clean CLI (`eval`, `run`, direct `<file>` / shebang execution, `repl`, `compile`, `test`, `exec`, `cache`) and a `#![forbid(unsafe_code)]` Rust embedding API (`Engine`, `EngineConfig`, `Value`, `Error`) independent of editor concepts (`temacs`, windows, frames, faces, minibuffers).
- **Drop-in compatibility**: 100% compatible with existing Lisp codebases, bytecode (`.elc`), portable dumps (`.pdmp`), ERT test suites, JSON-RPC `--rpc http`, and the native `fe-test` framework.
- **Zero external C dependencies by default**: default builds link only against the OS standard C runtime (no `gnutls`, `lcms2`, `tree-sitter`, `ncurses`, `libxml2`, `sqlite3`, or `zlib`). Editor targets opt into `tty_terminfo` when needed.
- **Single-binary portability**: embeds the compressed general-purpose Lisp standard library (`core-boot.el`, `core-prelude.el`, and core libraries) and atomically caches unpacked stdlib sources + fingerprinted `.pdmp` fast-boot images in a user cache directory.

---

## 2. Two-Crate Architecture

```text
  External Rust Embedders / CLI Users / `fe-test`
                        │
                        ▼
  crates/lisp-core (0.0.1, publishable, #![forbid(unsafe_code)] except os_ffi.rs)
    ├── Engine / EngineConfig / Value / Error (when feature `vm` enabled)
    ├── Embedded stdlib bundle (flate2 + sha256) & atomic .pdmp cache manager
    ├── CLI (`eval`, `run`, `repl`, `compile`, `test`, `exec`, `cache`, compat flags)
    └── OS supervision & hashing (`exec`: `supervise`, `mem-tree`, `digest`)
                        │ (optional feature `vm`, enabled by default)
                        ▼
  crates/lisp-vm (0.0.1, publishable, optional C FFI)
    ├── `sys` module (safe core algorithms, `VmGlobal`, POSIX wrappers, `client_os`)
    ├── Safe domain wrappers, GC roots/handles, `HeadlessVm` runtime
    ├── `ported/` (self-contained C-ported VM, bytecode, evaluator, regex, reader, bignum, buffer, process)
    └── Pure-Rust `build.rs` (synthesizes neutral `config.rs`/`epaths.rs` when editor overrides absent)

  Repository-internal consumers (`publish = false`):
    ├── crates/femacs-host (`tty_terminfo` enabled) -> `temacs` & compat `femacs-lisp`
    ├── crates/femacs-tools -> companion utilities (`emacsclient`, `hexl`, ...)
    └── xtask (depends on `lisp-core` with `default-features = false`) -> shares `lisp_core::exec`
```

### Why `femacs-sys` + `femacs-vm` Consolidated into `lisp-vm`

Previously, `femacs-sys` and `femacs-vm` shared `lib/include/*.rs` macro files across crate boundaries via `#[path = "../../../..."]` relative escapes and an `extern crate self as ...` workaround. Consolidating them into a single self-contained crate `crates/lisp-vm` (`lisp_vm::sys` + `lisp_vm::host_runtime`) eliminates path escapes outside the crate root, removes cross-crate macro duplication, and makes `cargo package` work cleanly under `< 10 MiB`.

### Zero Mandatory `ncurses` (`tty_terminfo` Feature)

Interactive terminal builds require `ncurses`/`terminfo` (`tgetent`, `tputs`, `tparm`, `UP`, `BC`, `PC`). Headless `lisp-vm` disables `tty_terminfo` by default and provides crate-private stub implementations (with no `#[no_mangle]` C symbol exports so downstream Rust binaries linking C `ncurses` never collide). `femacs-host` and `xtask` editor links explicitly enable `tty_terminfo`.

---

## 3. Standard Library & Cache Model (`crates/lisp-core`)

- `crates/lisp-core/stdlib/` holds a curated general-purpose language standard library (`subr`, `byte-run`, `backquote`, `macroexp`, `pcase`, `gv`, `cl-lib`, `cl-generic`, `seq`, `map`, `subr-x`, `nadvice`, `timer`, `rx`, `regexp-opt`, `bytecomp`, `cconv`, `ert`, `pp`, `json`, `files`, `env`, `format`, `obarray`, `case-table`, `mule`/`characters`, `eieio`, plus `core-boot.el` and `core-prelude.el`), omitting editor-only subsystems (`org`, `gnus`, `calc`, `vc`, `leim`, `erc`, UI toolkits).
- `build.rs` deterministically packs and gzip-compresses `stdlib/` into `OUT_DIR/stdlib.bundle.gz` along with `LISP_CORE_STDLIB_SHA256` (skipped when `--no-default-features` / `vm` is off, keeping `xtask` builds fast).
- On startup, `stdlib_cache`:
  1. Resolves cache root from `LISP_CORE_CACHE_DIR`, `XDG_CACHE_HOME/lisp-core`, or platform user cache directory (`~/Library/Caches/lisp-core` on macOS, `~/.cache/lisp-core` on Unix).
  2. Atomically materializes `v0.0.1-<stdlib_sha256_16>/stdlib/` via staging directory + rename with a `.ready` marker.
  3. Checks for `core-<exe_sha256_16>.pdmp`; if valid, uses fast portable-dump boot, otherwise transparently falls back to cold-booting `core-boot.el`. Corrupted or stale dumps never abort startup.

---

## 4. Public Rust API & CLI Surface

### 4.1 Embedding API (`lisp_core`, `#![forbid(unsafe_code)]`)
- `EngineConfig`: `embedded_stdlib()`, `minimal()`, `with_load_path()`, `with_stdlib_dir()`, `with_data_dir()`, `with_dump_file()`, `with_cache_dir()`, `with_no_dump()`.
- `Engine` (`!Send + !Sync`, matching underlying single-process VM invariant): `new()`, `minimal()`, `with_config()`, `eval()`, `eval_printed()`, `eval_silent()`, `call()`, `load_file()`, `load_library()`, `add_load_path()`, `run_script()`, `compile_file()`, `dump_image()`, `gc()`.
- `Value`: `Nil`, `Bool(bool)`, `Int(i64)`, `Float(f64)`, `String(String)`, `Symbol(String)`, `List(Vec<Value>)`, `Vector(Vec<Value>)`, `Map(BTreeMap<String, Value>)`.
- `Error`: `AlreadyInitialized`, `InitFailed(String)`, `Exit(i32)`, `Syntax { .. }`, `Runtime { .. }`, `Interrupted { .. }`, `Io { .. }`.

### 4.2 Standalone CLI (`lisp-core`)
- Fast Rust paths (zero VM startup):
  - `lisp-core exec supervise|mem-tree|digest ...`
  - `lisp-core cache path|clean`
  - `-h` / `--help`, `-V` / `--version`
- General-purpose commands:
  - `lisp-core eval <EXPR>`
  - `lisp-core run <FILE> [--] [ARGS...]` (and direct `lisp-core <FILE> [ARGS...]` for shebang scripts)
  - `lisp-core repl` (also default when invoked with no arguments on a terminal/stdin)
  - `lisp-core compile [--strict] [-L DIR]... <FILE...>`
  - `lisp-core test [-L DIR]... [-l LIB]... <FILE...>`
  - `lisp-core cache warm`
- Compatibility mode flags: `--batch`, `-Q`/`--quick`, `--no-site-file`, `--no-site-lisp`, `-L`, `-l`, `-e`, `-f`, `--script`, `--repl`, `--lisp-dir`, `--data-dir`, `--dump-file`, `--no-dump`, `--dump-out`, `--rpc http` (`--rpc-handler`, `--rpc-entry`, `--rpc-max-frame-bytes`, `--rpc-fd`).

---

## 5. Headless Invariants

- When running under headless `Engine` / `HeadlessVm`, missing `etc/charsets` directories do not terminate the host OS process (`exit(1)` is suppressed and `charset-map-path` remains `nil`); interactive editor launches preserve original diagnostics and exit behavior.
- Built-in OS primitives (`exec supervise`, `exec mem-tree`, `exec digest`) enable `fe-test` to execute directly under `lisp-core` with zero runtime dependency on `xtask`.
