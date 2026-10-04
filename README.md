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

Early days. The repository currently holds documentation and a license, and no runnable
implementation yet.

## Documentation

- [docs/i18n](docs/i18n/README.md) — language index. Translated documents live under
  `docs/i18n/<locale>/`, mirroring the paths of their English originals.

## License

femacs is distributed under the **GNU General Public License, version 3 or later**
(`GPL-3.0-or-later`). The full text is in [LICENSE](LICENSE).
