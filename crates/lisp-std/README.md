# lisp-std

[![crates.io](https://img.shields.io/crates/v/lisp-std?logo=rust&label=crates.io)](https://crates.io/crates/lisp-std)
[![docs.rs](https://img.shields.io/docsrs/lisp-std)](https://docs.rs/lisp-std)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](https://github.com/liuchong/femacs/blob/master/LICENSE)
[![Project website](https://img.shields.io/badge/docs-femacs.com-blue)](https://femacs.com/pages/lisp.html)

100% pure-Rust (`#![forbid(unsafe_code)]`, zero `cc` or C `-sys` dependencies) standard library batteries for the standalone [`lisp`](https://crates.io/crates/lisp) runtime and [`elisp`](https://crates.io/crates/elisp) CLI (`0.0.1`).

Enabled by default via the `std` feature in both `lisp` (`default = ["vm", "std"]`) and `elisp` (`default = ["repl-tty", "std"]`), while keeping minimal VM embedders (`default-features = false, features = ["vm"]`) and editor consumers completely isolated from HTTP, TLS, archive, or database dependencies.

## Pure-Rust Guarantees

- `#![forbid(unsafe_code)]` across the entire crate
- Zero C build dependencies (`cc`, `cmake`, `pkg-config`, `bindgen`, OpenSSL, or C SQLite are never pulled)
- Pure-Rust TLS via `ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots`
- Pure-Rust embedded ACID storage and SQL engine via `redb`

## Modules

| Module | Capabilities |
| --- | --- |
| `std/http` | HTTPS/HTTP client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots` + `gzip`) and multi-threaded HTTP/1.1 TCP server (`std-http-server-*`, `std-http-serve`) |
| `std/kv`, `std/db` | Embedded ACID key-value store (`redb`) and transactional SQL engine (`CREATE/DROP TABLE`, `INSERT`, `SELECT ... WHERE ... ORDER BY ... LIMIT/OFFSET`, `COUNT(*)`, `UPDATE`, `DELETE`, `BEGIN`/`COMMIT`/`ROLLBACK`) |
| `std/json`, `std/toml`, `std/csv`, `std/yaml`, `std/url`, `std/base64` | Structured data codecs, URL parser/builder, and standard/URL-safe Base64 codecs |
| `std/archive` | Pure-Rust gzip/zlib (`flate2` `rust_backend`), `tar`/`tar.gz`, and `zip` archives with path-traversal and symlink rejection |
| `std/crypto` | CSPRNG random bytes (`getrandom`), SHA-256/512, HMAC-SHA256, UUID v4/v7, hex, constant-time compare |
| `std/time` | UTC/RFC-3339 timestamps, timezones, formatting, and monotonic timers (`jiff`) |
| `std/fs`, `std/path` | Recursive directory walk (`walkdir`), glob matching (`globset`), file stats, binary/text I/O, and cross-platform path operations |
| `std/regex`, `std/proc`, `std/env`, `std/cli`, `std/log` | Linear-time pure-Rust regex, bounded subprocess runner with timeouts, `.env` parsing, declarative CLI flag parsing, and structured text/JSON logging |

## Rust Overview

```toml
[dependencies]
lisp-std = "0.0.1"
```

Most users consume `lisp-std` indirectly through [`lisp`](https://crates.io/crates/lisp) (`lisp::stdlib`), where a shared `StdRuntime` is automatically wired into the root `Engine` and `NoGil` worker threads. Direct callers can instantiate `StdRuntime` and dispatch operations over `StdValue` directly.

## Sibling Crates

- [`lisp`](https://crates.io/crates/lisp) ([docs.rs](https://docs.rs/lisp)) — High-level `Engine`, `Value`, No-GIL multi-core runtime, and re-exported `lisp::stdlib`
- [`lisp-vm`](https://crates.io/crates/lisp-vm) ([docs.rs](https://docs.rs/lisp-vm)) — Low-level Elisp-compatible VM and bytecode runtime
- [`elisp`](https://crates.io/crates/elisp) — Standalone CLI, script runner, package manager, and interactive TTY REPL

## Platform Support

| Target | Status | Verification Scope |
| --- | --- | --- |
| macOS (`aarch64-apple-darwin`) | Verified | Unit suite (`8/8`), live HTTP/SQL/`NoGil` integration tests, release system-only linkage |
| Linux (`aarch64-unknown-linux-gnu`) | Verified | Ephemeral Debian 12 Lima VM unit & integration suites |
| Linux (`x86_64-unknown-linux-gnu`) | Cross-compile verified | Clean `cargo check` |
| Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) | **Cross-compile ONLY** | Clean `cargo check` across default & test targets; **no physical Windows host runtime verification has been performed** |

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](https://github.com/liuchong/femacs/blob/master/LICENSE).
