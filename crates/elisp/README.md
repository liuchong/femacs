# elisp

A standalone, zero-C-dependency, general-purpose Elisp-compatible Lisp CLI, script runner, package manager, and interactive TTY REPL (`0.0.1`).

> Want to embed this Lisp runtime inside a Rust application (`cargo add lisp`)? See the library crate [`lisp`](../lisp/README.md).
> Under the hood, `elisp` combines [`lisp`](../lisp/README.md), [`lisp-std`](../lisp-std/README.md), and [`lisp-vm`](../lisp-vm/README.md) with a pure-Rust `rustyline` TTY frontend into one self-contained executable.

## Highlights

- **Single Binary, Zero C Runtime Dependencies**: Installs cleanly and links solely against standard OS libraries (`libSystem.B.dylib` + `libiconv.2.dylib` on macOS, glibc on Linux, Win32/UCRT on Windows).
- **No-GIL Multi-Core Parallelism**: Runs OS threads concurrently across CPU cores (`--concurrency=no-gil` default, `std/task`, `std/chan`, `std/sync`), with `--concurrency=gil` / `LISP_CONCURRENCY=gil` available for legacy single-VM behavior.
- **Pure-Rust Batteries Included (`std/*`)**: HTTP/HTTPS client & embedded HTTP/1.1 server, `redb` ACID KV + transactional SQL engine, JSON/TOML/CSV/YAML/Base64/URL, tar/zip/gzip/zlib, crypto/UUID, timezones, recursive FS/glob, regex, bounded subprocesses, CLI flags, and structured logging.
- **Fast Startup**: Silent cold boot by default with automatic atomic `.pdmp` warm caching in the background for millisecond subsequent launches.
- **Interactive TTY REPL**: Multi-line form validation, Lisp symbol completion, `Ctrl-C` abort recovery, persistent history, and clean piped stdin support.
- **Project Workflow (`init` / `pkg`)**: Scaffold projects (`lisp.toml`, `lisp.lock`, `.lisp-packages/`), manage local `--path` or HTTP `--url` dependencies, and run `elisp run` / `elisp test` cleanly.

## Installation

After crates.io publication:

```sh
cargo install elisp
```

Minimal install without TTY line editing (`cargo install elisp --no-default-features --features std`) or without `std/*` network/DB batteries (`--no-default-features --features repl-tty`) is also supported via feature flags (`default = ["repl-tty", "std"]`).

From source in this repository:

```sh
cargo build --release -p elisp
./target/release/elisp --version
```

## CLI Quickstart

```sh
# Evaluate expressions
elisp eval '(+ 20 22)'
elisp --no-auto-warm eval '(progn (require (quote std/json)) (std-json-stringify (list (cons "ok" t))))'

# Run scripts directly or via #!/usr/bin/env elisp shebang
elisp run path/to/script.el arg1 arg2
elisp path/to/script.el arg1 arg2

# Interactive TTY REPL (prompt: `elisp> `) or piped stdin
elisp repl

# Multi-core No-GIL script evaluation
elisp --concurrency=no-gil eval '(progn (require (quote std/task)) (std-task-parallel-map (lambda (x) (* x x)) (quote (1 2 3 4)) 4))'

# Byte-compile and run ERT tests
elisp compile --strict -L src src/main.el
elisp test -L src tests/main-test.el

# Scaffold a project & manage packages (`lisp.toml`, `lisp.lock`, `.lisp-packages/`)
elisp init my-app
cd my-app
elisp pkg add helper --path ../helper
elisp pkg add remote-lib --url https://example.com/lib.tar.gz --sha256 <HEX>
elisp pkg install
elisp pkg list
elisp pkg update
elisp pkg remove helper
elisp run    # auto-injects src/ and .lisp-packages/*/src, runs src/main.el
elisp test   # runs tests/*.el

# Cache & OS supervisor tools
elisp cache path | warm [--quiet] | clean
elisp exec supervise ... | mem-tree <PID> | digest <PATH>

# JSON-RPC 2.0 HTTP worker loop (Unix inherited fd or TCP loopback)
elisp --rpc http --rpc-fd <FD>
elisp --rpc http --rpc-addr 127.0.0.1:8080
```

## Standard Library Modules (`stdlib/std/*`)

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

## Common Flags & Environment Variables

- `--cache-dir <DIR>` (`LISP_CACHE_DIR`)
- `--concurrency=no-gil|gil` (`LISP_CONCURRENCY`)
- `--no-dump`, `--no-auto-warm` (`LISP_NO_AUTO_WARM=1`), `--verbose` (`LISP_VERBOSE=1`)
- Batch compatibility flags: `--batch`, `-Q`, `--quick`, `-L`/`--directory`, `-l`/`--load`, `-e`/`--eval`, `-f`/`--funcall`, `--script`, `--repl`, `--version`, `--help`

## Platform Support Matrix

| Target | Architecture | Status | Verification Scope |
| --- | --- | --- | --- |
| macOS (`aarch64-apple-darwin`) | Real host | Verified | CLI integration (`eval`, `run`/shebang, `compile --strict`, `test`, `cache`, `exec`, `--rpc-addr`, `init`/`pkg`), PTY REPL, release `otool -L` (OS libs only), `cargo package` isolated rebuild, `fe-test` |
| Linux (`aarch64-unknown-linux-gnu`) | Ephemeral Debian 12 VM | Verified | CLI/integration, release `ldd` (glibc + OS libs only), `.pdmp` warm cache, PTY REPL, 4-worker `NoGil` speedup, `init`/`pkg` workflow |
| Linux (`x86_64-unknown-linux-gnu`) | Cross-compile | Verified | Clean `cargo check` |
| Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) | Cross-compile ONLY | Architecture Verified (No Windows Host Runtime) | Pure-Rust `windows-sys` Job Objects, Toolhelp32 process snapshots, `--rpc-addr` TCP loopback, and `LOCALAPPDATA` cache root pass `cargo check` (`default` & `--no-default-features`) with 0 warnings; **no physical Windows host runtime verification has been performed** |

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
