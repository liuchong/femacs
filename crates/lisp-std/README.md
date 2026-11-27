# lisp-std

100% pure-Rust (`#![forbid(unsafe_code)]`, zero `cc` or C `-sys` dependencies) standard library batteries for standalone `lisp-core`.

## Modules

- `std/http`: HTTPS client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots`) and embedded pure-Rust HTTP/1.1 server
- `std/kv` & `std/db`: Embedded ACID key-value store (`redb`) and pure-Rust transactional SQL engine (`CREATE TABLE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE`, `BEGIN`/`COMMIT`/`ROLLBACK`)
- `std/json`, `std/toml`, `std/csv`, `std/yaml`, `std/url`, `std/base64`: Structured data formats, URL parsing, and Base64 codecs
- `std/archive`: Pure-Rust gzip/zlib, tar/tar.gz, and zip archives with path-traversal protection
- `std/crypto`: CSPRNG random bytes, SHA-256/512, HMAC-SHA256, UUID v4/v7, hex, constant-time compare
- `std/time`: UTC/RFC-3339 timestamps, timezones, formatting, and monotonic timers (`jiff`)
- `std/fs`, `std/path`: Recursive directory walk, glob matching, file stats, binary/text I/O, and cross-platform path operations
- `std/regex`, `std/proc`, `std/env`, `std/cli`, `std/log`: Pure-Rust regex, bounded subprocess runner with timeouts, `.env` parsing, CLI flags, and structured logging
