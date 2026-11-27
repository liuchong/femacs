# lisp-core

A standalone, general-purpose Lisp programming language runtime, Rust embedding engine, and CLI (`0.0.1`).

## Features

- **Zero external C library dependencies by default** (links only standard OS libc).
- **Single-binary portability**: bundles its curated general-purpose Lisp standard library (`subr`, `cl-lib`, `seq`, `map`, `pcase`, `rx`, `bytecomp`, `ert`, `json`, `eieio`, `files`, `env`, `format`) and atomically caches unpacked sources and fingerprinted `.pdmp` fast-boot images on demand.
- **General-purpose CLI (`lisp-core`)**:
  - `lisp-core eval '<expr>'`
  - `lisp-core run <file.el> [args...]` (or direct `<file.el>` / `#!/usr/bin/env lisp-core` shebang execution)
  - `lisp-core repl`
  - `lisp-core compile [--strict] [-L dir]... <file.el>...`
  - `lisp-core test [-L dir]... [-l lib]... <test-file.el>...`
  - `lisp-core cache path | warm | clean`
  - `lisp-core exec supervise | mem-tree | digest ...`
  - Full batch compatibility flags (`--batch`, `-Q`, `-L`, `-l`, `-e`, `-f`, `--script`, `--repl`, `--rpc http`).
- **Safe Rust embedding API (`#![forbid(unsafe_code)]`)**:
  - `Engine`, `EngineConfig`, `Value`, `Error`.

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
