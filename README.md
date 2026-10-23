# femacs

**English** · [简体中文](docs/i18n/zh-CN/README.md)

An editor project that rewrites and accelerates Emacs in Rust.

## The name

`Fe` is the chemical symbol of iron, and `Rust` is what iron turns into — rust comes from iron,
and the two are two faces of the same thing. Taking an editor we already trust and forging it
again in Rust is exactly what this project is about.

## Goals

- Rewrite the core of the editor in Rust, retiring the parts of Emacs that carry the most
  historical weight.
- Keep what makes Emacs worth keeping: Lisp extensibility, the package ecosystem, and the
  configuration habits its users already have.
- Make startup time, editing latency and bulk operations an order of magnitude better, using
  real threads instead of workarounds.
- Treat text editing as a systems problem: incremental parsing, asynchronous I/O, native
  rendering.

## Status

The translated Rust core and native Swift Cocoa frontend run on Apple Silicon macOS.
Functional acceptance is separate from the original performance target; no speedup is claimed.

## Build and try

Install Xcode command-line tools, Python 3.9+, Rust via rustup, and GNU awk (`brew install gawk`).
The committed `rust-toolchain.toml` selects the compiler. From the repository root:

```sh
cargo run -p xtask -- build-terminal
build/terminal/port/src/emacs -Q -nw
cargo run -p xtask -- build-macos
dist/macos/Femacs.app/Contents/MacOS/Femacs -Q
```

Both build entries bootstrap resources from this repository. They do not require an installed
editor, an `emacs/` checkout, configure, reference objects, or previous build output. Compiler
and runtime workloads should run under your resource controller. The Cocoa build is native
Rust/Swift; terminal and GUI outputs are separate.

To regenerate resources after a source/data change:

```sh
python3 tools/bootstrap-resources.py --program build/terminal/port/src/emacs --family all
```

`--family bytecode --all-bytecode` also compiles optional Lisp libraries. See
[bootstrap design and acceptance](docs/repository-bootstrap.md) for the generation stages,
requirements, clean-export evidence and optional reference-audit boundary.

## Source layout

`src/`, `lib/`, `lib-src/`, `platform/` and `xtask/` are at the repository root.
`lisp/`, `leim/`, `admin/` and `etc/` contain the committed raw inputs and generators;
`doc/` is the copied manual source and `docs/` is project documentation.
`build-support/` contains maintained target/recipe descriptions. `build/`, `target/`,
`.femacs-cache/` and `dist/macos/` are generated and ignored.

## Documentation

- [docs/i18n](docs/i18n/README.md) — language index. Translated documents live under
  `docs/i18n/<locale>/`, mirroring the paths of their English originals.

## License

femacs is distributed under the **GNU General Public License, version 3 or later**
(`GPL-3.0-or-later`). The full text is in [LICENSE](LICENSE).
