<p align="center"><img src="packaging/assets/fe-logo.png" width="180" alt="Fe — iron F and vertical GNU-inspired horns"></p>

# Fe

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

### macOS command-line launchers

After building or installing the Cocoa app, run `cargo install --locked --path cli`
from the repository root. The `femacs` package installs two commands:

- `femacs notes.org` forwards arguments directly to the editor. Use `femacs -nw notes.org` for native terminal startup.
- `fe notes.org` uses the clean terminal preset. Use `fe -1 a.txt b.txt` to open files one at a time or `fe -R -nw notes.org` to discard the preset and use native terminal startup.

Both launchers require the Cocoa app. Set `FEMACS_APP` to its bundle path if it
is outside the standard Applications folders. See the [launcher guide](cli/README.md).

## Local acceptance

Current construction and ordered modernization scope: [docs/modernization.md](docs/modernization.md).
Build terminal and Cocoa products first, then run the runtime acceptance under your resource
controller. This command does not build or contact a remote CI service:

```sh
python3 tools/local-acceptance.py --out build/acceptance/local-run
```

The evidence directory must be new. The report identifies the exact existing binaries,
dumps and configuration, includes per-check results, and explicitly distinguishes artifact
identity from build provenance. Real native key/file and IME checks are opt-in with
`--gui --ime --sender <event-helper> --sources <input-source-helper>`; missing requested
dependencies fail rather than silently skip. IME scenarios split steady-source and repeated
source-switch composition, require exact expected text and preserve failed attempts. Native
callback tracing is normally off; `FEMACS_INPUT_TRACE` opts one process into a bounded private
trace file (up to 1 MiB), which may contain typed text and belongs in ignored local evidence.
App publication builds and verifies a separate staged bundle before replacing an existing app.
Obsolete bring-up aggregators have been retired; see [the retirement ledger](docs/retired-scaffolding.md).
Differential ERT remains an optional audit: `check-ert.py` is strict by default and
`--collect-only` explicitly records known debt without claiming a passing gate.

## Source layout

`src/`, `lib/`, `lib-src/`, `platform/` and `xtask/` are at the repository root.
`lisp/`, `leim/`, `admin/` and `etc/` contain the committed raw inputs and generators;
`doc/` is the copied manual source and `docs/` is project documentation.
`build-support/` contains maintained target/recipe descriptions. `build/`, `target/`,
`.femacs-cache/` and `dist/macos/` are generated and ignored.

## Documentation

- [Project website](docs/index.html) — overview, macOS build guide, runtime architecture and documentation index.

- [docs/i18n](docs/i18n/README.md) — language index. Translated documents live under
  `docs/i18n/<locale>/`, mirroring the paths of their English originals.

## License

femacs is distributed under the **GNU General Public License, version 3 or later**
(`GPL-3.0-or-later`). The full text is in [LICENSE](LICENSE).

## Runtime measurements and optional reference audits

After building, run these commands under the resource controller. No reference program
is needed to measure the current editor, and no command invokes a remote CI service.

```sh
python3 tools/bench.py --rounds 5 --out build/bench/current
# Optional comparison against an existing reference executable:
python3 tools/bench.py --ref build/emacs-ref/src/emacs --out build/bench/paired
python3 tools/differential.py --out build/acceptance/differential-run
```

The measurement records all round samples, median/MAD/IQR and program/dump/corpus hashes.
Ten in-process cases retain the explicitly named minimum-of-five metric; startup is a
separate process-wall measurement. `--save FILE` creates a new baseline when the host is
quiet (`--force` explicitly overrides that guard); `--gate FILE` rejects incompatible old
minimum-only tables. It is a regression check against the candidate's own recorded
median, not proof that it is faster than another implementation. `--ref` alternates
program order across rounds. Existing `bench.sh`, `smoke.sh` and `differential.sh` remain
small argument-forwarding launchers. Differential comparison requires both processes to
exit normally; equal crashes or an empty corpus fail. Every output directory must be new.

Explicit translation-unit lists use `cargo run -p xtask -- check-units [list]`,
with the same configured normal and checking compiler as full builds. The legacy
`check-all-units.sh` only forwards arguments; it no longer deletes shared unit trees.

Default resource generation also compiles the core Lisp compiler libraries, so a
clean checkout does not rely on old ignored bytecode for compiler loading. Explicit
bytecode scopes remain scoped. Runtime research, the rejected shared-tail dispatch
experiment and bounded measurements are recorded in [the runtime report](docs/runtime-modernization.md).
No new Lisp native compiler or collector is enabled.

## Release packages

The independent [`packaging`](packaging/README.md) workspace module owns artwork and
platform packaging. On macOS, package an already built native app with:

```sh
cargo run -p fe-packaging -- macos --app dist/macos/Femacs.app --out dist/releases/local
```

This produces a branded `Fe.app`, a verified disk image and SHA-256 manifest. The
local package is ad-hoc signed, not notarized. Windows and Linux packaging are planned.
