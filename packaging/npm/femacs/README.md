# femacs

Prebuilt **FEmacs** and standalone **Elisp** CLI binaries (`elisp`, `femacs`, `fe`) packaged for npm with **zero runtime npm dependencies**.

The published npm package itself is a compact (~85 KiB) launcher compiled from pure [Eliscript](https://www.npmjs.com/package/eliscript) (`.eli`) source. On installation (or lazily on first invocation), it downloads the official release archive for your OS/architecture from [GitHub Releases](https://github.com/liuchong/femacs/releases/tag/v0.0.1), verifies its **SHA-256** checksum against `manifest.json`, and caches the native binaries locally.

## Quickstart

```sh
# Install globally
npm install -g femacs

# Or run directly without global installation
npx --package femacs elisp --version
```

Try the standalone No-GIL Elisp runtime right away:

```sh
elisp eval '(+ 20 22)'
elisp --concurrency=no-gil eval '(progn (require (quote std/json)) (std-json-stringify (list (cons "ok" t))))'
elisp repl
elisp init my-app && cd my-app && elisp run
```

Or launch the macOS Fe editor (requires `Fe.app` / `Femacs.app` built from source or installed in `/Applications`):

```sh
femacs notes.org
fe -1 a.txt b.txt
fe -R -nw notes.org
```

## What Is Inside This Package

Every published tarball (`femacs@0.0.1`) contains only 10 small files and **0 runtime npm dependencies**:

| Path in tarball | Purpose |
| --- | --- |
| `dist/bin/{elisp,femacs,fe}.mjs` | ESM CLI shim entrypoints exposed as npm `bin` commands |
| `dist/index.mjs` | Zero-dep launcher API (platform lookup, SHA-256 verification, atomic local cache) |
| `dist/postinstall.mjs` | Best-effort `postinstall` warm-up (warns instead of failing when offline) |
| `manifest.json` | Release version (`0.0.1`), target triples, archive filenames, and expected SHA-256 digests |
| `types/index.d.ts` | TypeScript declarations for programmatic Node.js/ESM usage |
| `README.md` & `LICENSE` | Package guide and `GPL-3.0-or-later` license |

### Downloaded Native Binaries

Once cached (in `~/Library/Caches/femacs-npm` on macOS or `${XDG_CACHE_HOME:-~/.cache}/femacs-npm` on Linux), three standalone executables are available:

| Command | Role | Requirements |
| --- | --- | --- |
| `elisp` | Standalone, zero-C-dependency, No-GIL multi-core Elisp-compatible CLI, TTY REPL (`elisp> `), bytecode compiler, ERT runner, `init`/`pkg` manager, and 22 pure-Rust `std/*` modules (`std/http`, `std/kv`, `std/db`, `std/json`, `std/task`, …) | Self-contained (uses OS system libraries only) |
| `femacs` | Forward-all CLI launcher for the native Fe Cocoa/terminal editor | Requires `Fe.app` or `Femacs.app` (via `/Applications`, `~/Applications`, or `FEMACS_APP`) |
| `fe` | Shortcut terminal-preset launcher (`fe -Q -nw`, `-1` sequential files, `-R` reset args) | Requires `Fe.app` or `Femacs.app` |

### What Is NOT Included

- **Cocoa `.app` bundle (`Femacs.app` / `Fe.app`)**: Not distributed inside this npm package; build from source on Apple Silicon macOS via `cargo run -p xtask -- build-macos` ([Build guide](https://femacs.com/pages/start.html)).
- **Prebuilt Windows binaries (`v0.0.1`)**: Windows does not have prebuilt archives in `v0.0.1`; install from Rust source via `cargo install elisp femacs` instead.

## Supported Prebuilt Platforms (`v0.0.1`)

| npm `platform-arch` | Target Triple | Archive SHA-256 Prefix | Verification Scope |
| --- | --- | --- | --- |
| `darwin-arm64` | `aarch64-apple-darwin` | `f42a8fdd...` | Real Apple Silicon macOS host verified (`elisp`, `femacs`, `fe`, `otool -L` system dylibs only) |
| `darwin-x64` | `x86_64-apple-darwin` | `d341fdca...` | Cross-compiled Mach-O + `otool -L` system-dylib inspection (no physical Intel Mac / Rosetta runtime test) |
| `linux-arm64` | `aarch64-unknown-linux-gnu` | `b2d6965e...` | Cross-compiled ELF via Zig (`glibc >= 2.31`); matches Debian 12 `aarch64` VM runtime |
| `linux-x64` | `x86_64-unknown-linux-gnu` | `2678a3a7...` | Cross-compiled ELF via Zig (`glibc >= 2.31`) |

Full SHA-256 digests are listed in `manifest.json` and [`SHA256SUMS.txt`](https://github.com/liuchong/femacs/releases/download/v0.0.1/SHA256SUMS.txt).

## Programmatic Node.js / ESM API

You can also invoke the verified cache & launcher programmatically from Node >= 18 (`types/index.d.ts`):

```js
import { binaryPath, ensureInstalled, loadManifest, runBinary } from "femacs";

const manifest = loadManifest();
const info = await ensureInstalled({ manifest });
const elispExe = await binaryPath("elisp");
const code = await runBinary("elisp", ["--no-auto-warm", "eval", "(+ 20 22)"]);
```

## Configuration Environment Variables

- `FEMACS_NPM_CACHE_DIR` — Override where verified binary archives are cached on disk.
- `FEMACS_DOWNLOAD_BASE_URL` — Mirror base URL used to download `femacs-v<version>-<triple>.tar.gz`.
- `FEMACS_SKIP_DOWNLOAD=1` — Skip `postinstall` warm-up download; the binary is downloaded on first command execution instead (useful in CI image builds or when package managers disable install scripts).

## Other Install Channels

- **Homebrew (macOS & Linux — separate packages)**:
  - Standalone Lisp runtime & REPL: `brew install liuchong/tap/elisp`
  - macOS Fe editor launchers (`femacs`, `fe`): `brew install liuchong/tap/femacs`
- **Shell installer (macOS & Linux)**: `curl -fsSL https://femacs.com/install.sh | sh` (pass `--only elisp` or `--only femacs` to install a subset)
- **Cargo (from source, all targets)**: `cargo install elisp` and `cargo install femacs --locked`
- **Embed in Rust (`Cargo.toml`)**: `lisp = "0.0.1"` (`lisp-vm`, `lisp-std`, `lisp`, `elisp` on [crates.io](https://crates.io/crates/lisp))

## Links

- Project homepage & documentation: https://femacs.com/
- Standalone Lisp & CLI guide: https://femacs.com/pages/lisp.html
- GitHub Repository: https://github.com/liuchong/femacs
- GitHub Release (`v0.0.1`): https://github.com/liuchong/femacs/releases/tag/v0.0.1
- License: `GPL-3.0-or-later`
