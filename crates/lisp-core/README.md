# lisp-core

A standalone, general-purpose Lisp programming language runtime, multi-core Rust embedding engine, and CLI (`0.0.1`).

> Note: Crate and CLI identifiers (`lisp-core`, `lisp-vm`, `lisp-std`) are provisional pre-publish names centralized in [`src/brand.rs`](src/brand.rs).

## Highlights

- **100% Pure-Rust Build Graph (Zero C Build Dependencies)**: Compiles without `cc`, `cmake`, `pkg-config`, `bindgen`, OpenSSL, or C SQLite. Release binaries link solely against standard OS runtime libraries (`libSystem.B.dylib` + `libiconv.2.dylib` on macOS, glibc on Linux, Windows Win32/UCRT on Windows).
- **Multi-Core No-GIL Concurrency (`NoGil` by Default)**: Each `Engine` owns an isolated VM heap/globals context (`lisp-vm/isolate_vm`), making `Engine: Send` across OS threads. Lisp programs get `std/task`, `std/chan`, and `std/sync` for true multi-core execution, or can opt into `--concurrency=gil` / `LISP_CONCURRENCY=gil` for legacy single-isolate semantics.
- **Full Pure-Rust Standard Library (`std/*`)**: Ships a bundled Lisp standard library plus `crates/lisp-std` (`std-batteries`, 22 `std/*` modules covering HTTPS/HTTP server & client, ACID KV/SQL storage, archives, serialization, crypto, regex, time, FS traversal, CLI parsing, and structured logging).
- **Silent Cold Boot & Automatic `.pdmp` Warm Cache**: Silent stderr by default (use `--verbose` or `LISP_CORE_VERBOSE=1` for load traces) with atomic background portable dump generation for millisecond warm startups.
- **Pure-Rust Interactive TTY REPL**: Line editing, multi-line incomplete form detection, symbol completion, `Ctrl-C` abort recovery, and persistent history (`repl-tty` feature via `rustyline`, with clean non-TTY pipe fallback).
- **Built-in Project & Package Workflow**: `init`, `pkg`, `lisp.toml`, `lisp.lock`, and `.lisp-packages/` for local path and URL dependencies with SHA-256 verification.

## Installation & Running

After crates.io publication:

```sh
cargo install lisp-core
```

From source in this repository:

```sh
cargo build --release -p lisp-core
./target/release/lisp-core --version
```

## CLI Usage

```sh
# Evaluate expressions
lisp-core eval '(+ 20 22)'
lisp-core --no-auto-warm eval '(progn (require (quote std/json)) (std-json-stringify (list (cons "ok" t))))'

# Run scripts directly or via #!/usr/bin/env lisp-core shebang
lisp-core run path/to/script.el arg1 arg2
lisp-core path/to/script.el arg1 arg2

# Interactive TTY REPL (or piped stdin)
lisp-core repl

# Byte-compile and run ERT test suites
lisp-core compile --strict -L src src/main.el
lisp-core test -L src tests/main-test.el

# Project scaffolding & dependency management (lisp.toml + lisp.lock + .lisp-packages/)
lisp-core init my-app
cd my-app
lisp-core pkg add helper --path ../helper
lisp-core pkg add remote-lib --url https://example.com/lib.tar.gz --sha256 <HEX>
lisp-core pkg install
lisp-core pkg list
lisp-core pkg update
lisp-core pkg remove helper
lisp-core run    # auto-injects src/ and .lisp-packages/*/src, runs src/main.el
lisp-core test   # runs tests/main-test.el

# Portable dump cache management
lisp-core cache path
lisp-core cache warm [--quiet]
lisp-core cache clean

# Supervised subprocess, process-tree memory, and content digest tools
lisp-core exec supervise ...
lisp-core exec mem-tree --pid <PID>
lisp-core exec digest ...

# JSON-RPC over HTTP transport (inherited Unix fd on Unix, or TCP loopback)
lisp-core --rpc http --rpc-fd <FD>
lisp-core --rpc http --rpc-addr 127.0.0.1:8080
```

Common flags:
- `--concurrency=no-gil|gil` (or `LISP_CONCURRENCY=no-gil|gil`)
- `--cache-dir <DIR>` (`LISP_CORE_CACHE_DIR`), `--no-dump`, `--no-auto-warm` (`LISP_CORE_NO_AUTO_WARM=1`), `--verbose` (`LISP_CORE_VERBOSE=1`)
- Batch compatibility flags: `--batch`, `-Q`, `--quick`, `-L`/`--directory`, `-l`/`--load`, `--eval`, `-f`/`--funcall`, `--script`, `--repl`, `--version`, `--help`

## Pure-Rust Standard Library (`stdlib/std/*`)

Enabled by the default `std-batteries` feature (`crates/lisp-std`), usable from both the root `Engine` and `std/task` `NoGil` workers:

| Module | Capabilities |
| --- | --- |
| `std/task`, `std/chan`, `std/sync` | Multi-core OS threads, worker pools, `parallel-map`, bounded/unbounded MPMC channels, atomics, mutexes, shared maps |
| `std/http` | Pure-Rust HTTPS/HTTP client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots` + `gzip`) and multi-threaded HTTP/1.1 TCP server |
| `std/kv`, `std/db` | Embedded ACID key-value store (`redb`) and transactional SQL engine (`CREATE/DROP TABLE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE`, `BEGIN/COMMIT/ROLLBACK`) |
| `std/json`, `std/toml`, `std/csv`, `std/yaml`, `std/url`, `std/base64` | Pure-Rust structured codecs, URL parsing/building, and standard/URL-safe Base64 |
| `std/archive` | Pure-Rust gzip/zlib (`flate2` `rust_backend`), `tar`/`tar.gz`, and `zip` with path-traversal and symlink guards |
| `std/crypto`, `std/time` | `getrandom` CSPRNG, SHA-256/512, HMAC-SHA256, UUID v4/v7, hex, constant-time equality, RFC-3339/UTC/timezone arithmetic (`jiff`), monotonic timers |
| `std/fs`, `std/path` | Recursive directory traversal (`walkdir`), glob filtering (`globset`), file stats, binary/text I/O |
| `std/regex`, `std/proc`, `std/env`, `std/cli`, `std/log` | Linear-time regex, bounded subprocess execution, `.env` loader, declarative CLI flag parser, structured text/JSON logging |

## Rust Embedding API (`#![forbid(unsafe_code)]`)

```rust
use lisp_core::{ConcurrencyMode, Engine, EngineConfig, Value};

fn main() -> Result<(), lisp_core::Error> {
    let mut engine = Engine::init(
        EngineConfig::cold_stdlib()
            .with_concurrency(ConcurrencyMode::NoGil)
            .with_verbose(false),
    )?;

    engine.register_fn("rust-sha-len", |args| {
        let payload = args
            .first()
            .and_then(|v| v.as_bytes())
            .ok_or_else(|| lisp_core::Error::Host("expected bytes".into()))?;
        Ok(Value::Int(payload.len() as i64))
    })?;

    let n = engine.eval(r#"(internal-host-call "rust-sha-len" "hello")"#)?;
    assert_eq!(n.as_int(), Some(5));
    Ok(())
}
```

Key types (`lisp_core::*`):
- `Engine` (`new`, `minimal`, `init`, `eval`, `eval_printed`, `eval_silent`, `call`, `register_fn`, `complete_symbols`, `run_script`, `compile_files`, `run_ert_files`, `dump_image`, `gc`, `booted_from_dump`)
- `EngineConfig` (`minimal`, `cold_stdlib`, `with_cache_dir`, `with_load_path`, `with_bootstrap`, `with_quiet_bootstrap`, `with_verbose`, `with_concurrency`)
- `Value` (`Nil`, `Bool`, `Int`, `Float`, `Symbol`, `String`, `Bytes`, `List`, `Map`) and `Error`

`Engine` is `Send` when using `ConcurrencyMode::NoGil` (default), enabling multiple independent engines to run concurrently across threads.

## Feature Flags & Consumer Isolation

- `default = ["vm", "repl-tty", "std-batteries"]`: Full standalone language CLI + embeddable multi-core runtime with pure-Rust batteries and TTY REPL.
- `vm`: Embeddable `Engine`, `NoGil`/`Gil` VM (`lisp-vm/isolate_vm`), and bundle cache without TTY REPL or `lisp-std`.
- `repl-tty`: Enables pure-Rust `rustyline` interactive line editing (`std` pipe REPL still works without it).
- `std-batteries`: Enables `crates/lisp-std` (`std/*` pure-Rust OS/network/DB modules).
- `default-features = false`: Minimal zero-VM `value`, `error`, `exec`, `rpc`, `stdlib_cache`, and `brand` utilities (used by editor workspace tooling to guarantee complete isolation from standalone CLI/battery dependencies).

## Platform Support & Verification Matrix

| Target | Architecture | Status | Verification Scope |
| --- | --- | --- | --- |
| macOS (`aarch64-apple-darwin`) | Real host | Verified | Full unit, `engine_integration`, `std_batteries_integration`, `isolate_vm`, release `otool -L` (OS libs only), `cargo package`, and `fe-test` suites |
| Linux (`aarch64-unknown-linux-gnu`) | Ephemeral Debian 12 VM | Verified | Unit, integration, release `ldd` (glibc + OS libs only), `.pdmp` warm cache, PTY REPL, 4-worker `NoGil` speedup, `init`/`pkg` CLI workflow |
| Linux (`x86_64-unknown-linux-gnu`) | Cross-compile | Verified | Clean `cargo check` across workspace standalone crates |
| Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) | Cross-compile ONLY | Architecture Verified (No Windows Host Runtime) | Pure-Rust `windows-sys` Job Objects, Toolhelp32 process snapshots, memory accounting, `RpcTransport` TCP loopback, and `LOCALAPPDATA` cache root pass `cargo check` (`default`, `--no-default-features --features vm`, and `--tests`) with 0 warnings in `lisp-core`/`lisp-std`; **no physical Windows host runtime verification has been performed** |

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
