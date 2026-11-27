# lisp-std

100% pure-Rust (`#![forbid(unsafe_code)]`, zero `cc` or C `-sys` dependencies) standard library batteries for standalone `lisp-core` (`0.0.1`).

Isolated under the `std-batteries` feature of `lisp-core` so editor builds never pull HTTP, TLS, or embedded database dependencies.

## Modules

- `std/http`: HTTPS/HTTP client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots` + `gzip`) and embedded multi-threaded pure-Rust HTTP/1.1 TCP server (`std-http-server-*`, `std-http-serve`)
- `std/kv` & `std/db`: Embedded ACID key-value store (`redb`) and pure-Rust transactional SQL engine (`CREATE/DROP TABLE`, `INSERT`, `SELECT ... WHERE ... ORDER BY ... LIMIT/OFFSET`, `COUNT(*)`, `UPDATE`, `DELETE`, `BEGIN`/`COMMIT`/`ROLLBACK`)
- `std/json`, `std/toml`, `std/csv`, `std/yaml`, `std/url`, `std/base64`: Structured data codecs, URL parser/builder, and standard/URL-safe Base64 codecs
- `std/archive`: Pure-Rust gzip/zlib (`flate2` `rust_backend`), `tar`/`tar.gz`, and `zip` archives with path-traversal and symlink rejection
- `std/crypto`: CSPRNG random bytes (`getrandom`), SHA-256/512, HMAC-SHA256, UUID v4/v7, hex, constant-time compare
- `std/time`: UTC/RFC-3339 timestamps, timezones, formatting, and monotonic timers (`jiff`)
- `std/fs`, `std/path`: Recursive directory walk (`walkdir`), glob matching (`globset`), file stats, binary/text I/O, and cross-platform path operations
- `std/regex`, `std/proc`, `std/env`, `std/cli`, `std/log`: Linear-time pure-Rust regex, bounded subprocess runner with timeouts, `.env` parsing, declarative CLI flag parsing, and structured text/JSON logging

## Platform Support

- Pure Rust `#![forbid(unsafe_code)]` across macOS, Linux, and Windows targets (`cargo check` verified on macOS, Linux `aarch64`/`x86_64`, and Windows `x86_64-pc-windows-msvc`/`gnu`; Windows is cross-compile verified ONLY without physical Windows host runtime verification).

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
