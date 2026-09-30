# Phase 1: one-to-one port plan

[femacs](../README.md) replicates GNU Emacs in two workstreams: the editor's own
portable C is translated into Rust statement by statement, and everything that
is platform-specific, third-party or build-related is rebuilt on current
technology instead of inheriting the legacy stack. The Emacs Lisp sources are
copied, never rewritten.

- **Baseline**: the `emacs/` checkout at `fd7afced403` (GNU Emacs 32.0.50). It is
  git-ignored and is the only reference for every port.
- **Companion**: [port-map.md](port-map.md) is the generated file-by-file
  correspondence — mode, names, line counts and status. It is data, not prose.
- **Phase 1 goal**: the shipped Emacs Lisp tree runs on the result, and every
  source file of the reference tree has a decided fate: a statement-by-statement
  Rust twin, a modern rewrite, or a recorded drop.

## 0. Technology policy

**T1 — Emacs Lisp is never rewritten.** `lisp/`, `leim/` and the Lisp generators
in `admin/` are copied byte for byte. No translation, no reformatting, no
modernising, and no tokens spent rewriting them.

**T2 — Emacs' own portable C becomes Rust, one to one.** Same file name, same
identifiers, same statement order, same semantics, `unsafe` where C needs it
(§2). This is the only workstream that is a translation rather than a rewrite.

**T3 — The platform layer uses the platform's current technology, and Rust
whenever Rust can do the job.**

| Target | Reference implementation | femacs |
| --- | --- | --- |
| macOS / iOS | Objective-C (`ns*.m`, `macfont.m`) | **Swift**: AppKit, CoreText, WKWebView, Swift concurrency. No new Objective-C by anyone. |
| Windows | Win32/GDI/Uniscribe C (`w32*`) | **Rust + windows-rs**: DirectWrite, Direct2D, current Win32 |
| Linux / BSD | Xlib/Xt with Lucid, or GTK 3 | **Rust + GTK 4 / libadwaita on Wayland** |
| Android | Java + JNI + execve trampolines | **Rust (NDK/JNI) + Kotlin + Gradle** |
| Haiku | C++ with BeOS-era calls | **Modern C++** (the platform API itself is C++) |
| tty | terminfo | unchanged — terminfo is still the current mechanism |

Backend headers stay Rust even when the implementation behind them moves to
Swift, C++ or Kotlin: they declare the interface and the layouts shared with the
core, and the core must be able to link against them (a C header is generated
from them for Swift to import).

**T4 — Third-party C libraries are replaced by current Rust crates.**

| Was | Becomes |
| --- | --- |
| GnuTLS | `rustls` |
| libgmp | pure-Rust big integers |
| SQLite binding | `rusqlite` |
| jansson (`json.c`) | `serde_json` |
| libxml2 | `quick-xml` |
| libdbus | `zbus` |
| zlib | `flate2` |
| `dlopen` | `libloading` |
| ImageMagick, lcms2 | the `image` crate and current codecs |
| Emacs' own TrueType rasterizer (`sfnt.c`) plus FreeType/HarfBuzz drivers | HarfBuzz shaping, FreeType and the platform text stacks |
| libgccjit (`comp.c`) | native compilation on LLVM or Cranelift |
| WebKit widget (`xwidget.c`) | WebKitGTK / WKWebView |

**T5 — Build and scaffolding are current too.** A Cargo workspace with a Rust
`xtask` drives loadup, `DOC`, dumping and the generated tables; SwiftPM builds
the macOS side; Gradle builds the Android side. autoconf, `m4/`, `Makefile.in`,
NMake project files and gnulib are dropped: Rust std, `libc`, `nix` and
`windows` cover what gnulib covered.

**T6 — The semantic boundary.** Wherever behaviour is visible to Lisp, fidelity
beats modernisation, even when the reference implementation looks old: regexp
syntax, bytecode format, dump format, module ABI, charsets and coding systems,
text properties, keymaps, undo, GC reporting, and the `lisp/` tree itself. See
§10.

## 1. Scale

| Mode | Files | Lines |
| --- | ---: | ---: |
| `1:1 Rust` — statement-by-statement translation | 183 | 334,348 |
| `Modern Rust` — same name, current APIs | 55 | 170,318 |
| `Swift` — macOS backend | 8 | 26,338 |
| `C++` — Haiku backend | 8 | 12,616 |
| `Kotlin` — Android app layer | 40 | 15,157 |
| **written by hand, total** | **294** | **558,777** |
| `Drop` — superseded, replaced | 580 | 212,023 |
| `Verbatim` — copied unchanged (Emacs Lisp, tests, data, manuals) | 4,616 | 4,400,294 |
| `Generated` — produced by the build | — | — |

`Modern Rust` splits into Windows (`windows-rs`, 17 files, 50,959 lines),
Android (`NDK/JNI`, 8 files, 29,754), GTK 4/Wayland (5 files, 15,095) and
crate-backed replacements (25 files, 74,510).

`Drop` splits into gnulib (237 files, 69,423), the Xlib/Xt backend (9 files,
54,237), the Lucid widget set (48 files, 13,529), legacy Windows/DOS code
(66 files, 18,890), the Android execve trampolines (8 files, 4,274), autoconf
and friends (209 files, 41,920) and legacy assets (3 files, 9,750).

## 2. Fidelity rules for the 1:1 set

**R1 — Names.** `src/NAME.c` becomes `femacs/src/NAME.rs`; `src/NAME.h` becomes
`femacs/src/include/NAME.rs`; other C directories keep their directory name under
`femacs/`. No renaming, merging or splitting.

**R2 — Identifiers.** Functions, macros, enums, structs, union members and
globals keep their exact spelling: `Ffuncall`, `Lisp_Object`, `CHECK_STRING`,
`Vload_path`, `Qnil`, `struct Lisp_String`, `GC_MARK_STACK`. A C `#define` that
carries code becomes a `macro_rules!` of the same name; one that carries a
constant becomes a Rust constant of the same name.

**R3 — Translation units and ABI.** One `.c` file becomes one independently
compiled Rust file. Each file declares what it uses; `src/NAME.h` is textually
included the way C includes it — `include!("include/NAME.rs")` mirrors
`#include "NAME.h"`. Exported symbols use `#[no_mangle] pub unsafe extern "C"
fn` with the C name, so C and Rust objects link into one binary and the tree can
be switched over one file at a time.

**R4 — Layout.** `#[repr(C)]` structs with the same member order; tag bits, GC
bits, `USE_LSB_TAG` and every size and offset identical to C, asserted at
compile time from both sides (C `offsetof`/`verify` against Rust `offset_of!`
and `const` assertions). A layout that cannot be expressed this way is a
recorded deviation, not a redesign.

**R5 — Semantics.** Control flow, error model and memory behaviour are
preserved: `sys_setjmp`/`sys_longjmp` plus `record_unwind_protect` for errors (no
Rust `Result`, no `Option`-driven control flow, no panic across FFI), the
mark-sweep collector with conservative stack scanning, the same allocator. Rust's
global allocator is never installed.

**R6 — Statement parity.** Definition order follows the C file and one C
statement becomes one Rust statement. Where Rust's construct differs (a `match`
for a `switch`, a closure for a function pointer plus context) the deviation is
recorded per file instead of forcing fake parity.

**R7 — Generated sources stay generated.** `src/config.h`, `src/lisp.mk`,
`src/emacs-module.h`, `etc/DOC`, `lisp/loaddefs.el`, the `admin/unidata` and
`admin/charsets` tables are produced by the build. Byte-identical output is a
verification target, not a port target.

**R8 — Verbatim trees are copied, never edited.** `lisp/`, `test/`, `etc/`,
`doc/`, `leim/`, `admin/`, the root documents and the ChangeLogs keep their paths
and bytes. Emacs Lisp is not reformatted or translated, not even when a modern
style would be shorter.

## 3. Work batches

| Batch | Content | Exit gate |
| --- | --- | --- |
| 0 | Workspace and build: Cargo workspace, `xtask`, the SwiftPM package, the Gradle project, the per-file switch that links Rust and remaining C into one binary | both variants of every ported file build and link; the switch is generated from the port map |
| 1A | Interpreter core (`1:1`): `lisp.h`, `alloc.c`, `gmalloc.c`, `ralloc.c`, `vm-limit.c`, `eval.c`, `lread.c`, `print.c`, `bytecode.c`, `fns.c`, `data.c`, `doc.c`, `doprnt.c`, `floatfns.c`, `sort.c`, `cmds.c`, `macros.c`, `callint.c`, `atimer.c` | `femacs --batch -Q --eval` evaluates, prints, signals and unwinds byte-identically with the reference on the 1A corpus |
| 1B | Text and buffers (`1:1`): `buffer.c`, `insdel.c`, `marker.c`, `intervals.c`, `textprop.c`, `casefiddle.c`, `casetab.c`, `indent.c`, `syntax.c`, `search.c`, `regex-emacs.c`, `region-cache.c`, `itree.c`, `undo.c`, `editfns.c` | buffer, marker, text-property, syntax and regexp ERT groups pass |
| 1C | Characters and coding (`1:1`): `character.c`, `charset.c`, `chartab.c`, `category.c`, `ccl.c`, `composite.c`, `coding.c`, `bidi.c` | `coding-tests`, `charset-tests`, `character-tests` pass; file round-trips are identical |
| 1D | System surface (`1:1`, with `systhread.c`/`sysdep.c`/notifications on modern Rust): `emacs.c`, `fileio.c`, `filelock.c`, `dired.c`, `process.c`, `callproc.c`, `thread.c`, `timefns.c`, `profiler.c` | `process-tests`, `fileio-tests`, `thread-tests` pass with real subprocesses |
| 1E | Display and terminal (`1:1` core, modern font stack and native compilation): `xdisp.c`, `dispnew.c`, `window.c`, `frame.c`, `term.c`, `terminal.c`, `font.c`, `fontset.c`, `xfaces.c`, `fringe.c`, `cm.c`, `scroll.c`, `keyboard.c`, `keymap.c`, `minibuf.c`, `menu.c`, `widget.c`, `textconv.c` | `xdisp-tests`, `window-tests` pass; terminal sessions match recorded frames; text renders through HarfBuzz |
| 1F | Platform layer, modernised: Swift (macOS), `windows-rs` (Windows), GTK 4/Wayland (Linux/BSD), Rust + Kotlin (Android), modern C++ (Haiku) | a GUI session starts on each target, Lisp-visible behaviour matches the reference, platform smoke tests pass |
| 1G | Dump, modules and tools (`1:1`): `pdumper.c`, `emacs-module.c`, `lib-src/` (`emacsclient`, `etags`, `make-docfile`, `movemail`, `hexl`, …) | dump files, `.elc` files and dynamic modules are interchangeable with the reference |

Batches 1A–1E are ordered by dependency; 1F and 1G parallelise once 1E stands.
Batch 1F is by design a rewrite against current APIs, so its gate is behavioural
rather than line-by-line.

## 4. The loading ladder

| Rung | Loads | Target |
| --- | --- | --- |
| L0 | hand-written expressions | 1A |
| L1 | all of `lisp/emacs-lisp/*.elc` | 1B |
| L2 | `subr.elc`, `simple.elc`, `loaddefs.elc` | 1C |
| L3 | the complete `loadup.el` preloaded set | 1D |
| L4 | `make -C test check` default selector | 1E |
| L5 | the whole suite, `:expensive-test` included | phase 1 exit |

## 5. Verification

1. **ERT against either binary.** `test/Makefile.in` takes `EMACS` from the
   command line, so `make -C test EMACS=…/femacs check` runs the upstream suite
   against the port. The suite is the acceptance criterion, not a proxy.
2. **Differential runs.** The same Lisp expressions evaluated by the reference
   and by the port in `--batch` must match byte for byte on stdout, stderr and
   exit status.
3. **Layout assertions.** Generated C/Rust assertions cover every shared struct;
   a mismatch fails the build.
4. **Binary interchange.** `.elc` files, dump files and dynamic modules must load
   in both directions.
5. **Generated artifacts.** `admin/unidata`, `admin/charsets`, `src/lisp.mk` and
   `ldefs-boot.el` produced by the port must be byte-identical to the reference.
6. **Modern layers are accepted behaviourally**: the Lisp-visible behaviour of
   each modernised backend is compared against the reference, and the backend
   itself is smoked on its platform.
7. **Performance**, measured after functional parity.

## 6. Build integration

The workspace builds with Cargo; `xtask` orchestrates the steps the reference
Makefile used to own (loadup, `DOC`, dumping, the unidata and charset tables).
A generated per-file switch decides whether `src/NAME.c` or `femacs/src/NAME.rs`
is compiled into the binary, so the tree migrates one file at a time and either
variant can be built at will. Rust files compile as independent units
(`--emit=obj`, `-C panic=abort`, one codegen unit) and link with the remaining C
into a single binary with a single symbol space. The macOS backend is a SwiftPM
package that links the Rust core as a static library and imports a C header
generated from the Rust interface definitions; the Android app is a Gradle
project around the Rust core.

## 7. Unsafe policy

`unsafe` is expected and allowed, because the point is to reproduce C semantics.
It is scoped, not casual:

- Points that are inherently unsafe — tagged pointers, object layout, the
  collector, `setjmp`/`longjmp`, raw buffers, FFI — may use `unsafe` freely.
- Every `unsafe` block carries a `SAFETY:` comment naming the C invariant it
  reproduces.
- No panic may cross an FFI boundary (`panic = abort`, no `Result` plumbing).
- Unsafe is never used to invent behaviour the C source does not have.

## 8. Per-file workflow

1. Decide the file's mode (`1:1 Rust`, `Modern Rust`, `Swift`, `C++`, `Kotlin`,
   `Drop`) and record it in the port map.
2. Port or rewrite it at `femacs/…` in the target language.
3. Assert: extend the layout assertions for the structs it declares.
4. Build both ways when the file is `1:1`: with the switch on and off, both must
   link.
5. Test: differential run plus the affected ERT groups.
6. Record: `python3 tools/gen-port-map.py` refreshes modes, targets and line
   counts in `docs/port-map.md`.
7. Commit: signed, messages `port(NAME): …`, `modern(NAME): …` or `drop(NAME): …`,
   then push.

## 9. Modernization and drop ledger

| Dropped | Files | Lines | Replaced by |
| --- | ---: | ---: | --- |
| gnulib (`lib/`) | 237 | 69,423 | Rust std, `libc`, `nix`, `windows` |
| Xlib/Xt backend (`xterm.c`, `xfns.c`, `xselect.c`, …) | 9 | 54,237 | Rust + GTK 4 on Wayland |
| Lucid widget set (`lwlib/`, `oldXMenu/`) | 48 | 13,529 | GTK 4 |
| NT/MS-DOS legacy (`nt/`, `msdos/`, `dosfns.c`, `cygw32.c`, `w16select.c`, `w32heap.c`) | 66 | 18,890 | cargo targets, current Win32 |
| Android execve trampolines (`exec/`) | 8 | 4,274 | plain NDK/JNI |
| autoconf scaffolding (`configure.ac`, `m4/`, `build-aux/`, `Makefile.in`, `cross/`, `nextstep/`) | 209 | 41,920 | Cargo, `xtask`, SwiftPM |
| Legacy assets (`macuvs.h`, `emacs-icon.h`, `be_resources.cc`) | 3 | 9,750 | CoreText, asset catalogs, SwiftPM resources |

## 10. Fidelity beats modernisation

These are old-looking on purpose and are replicated as they are, because Lisp
can see them:

| Kept as in the reference | Why |
| --- | --- |
| `regex-emacs.c` | Emacs regexp syntax and backtracking semantics are part of the language |
| `bytecode.c` | `.elc` files and `byte-code` objects must stay compatible (the computed `goto` becomes a `match` or inline assembly) |
| `charset.c`, `coding.c`, character and category tables | coding systems and charsets define how files are read and written |
| `intervals.c`, `textprop.c` | text property behaviour is Lisp-visible |
| keymaps, undo, markers, overlays | user-visible semantics |
| `alloc.c` and the collector, `pdumper.c` | dump format and `garbage-collect` reporting |
| `emacs-module.c` and `module-env-*.h` | the dynamic module ABI is a compatibility contract |
| `term.c`, `termcap.c`, `terminfo.c` | terminfo remains the current mechanism for ttys |

## 11. Open decisions

1. Swift ↔ Rust bridge: a hand-written C header for the Swift side, or a
   generated one from the Rust layout definitions (current plan: generated).
2. Whether the GTK 4 backend also serves X11 sessions through XWayland, or the
   port targets Wayland only.
3. Whether gnulib is truly droppable for every platform we ship (current plan:
   dropped; Rust std and `nix`/`windows` cover it).
4. Whether `comp.c` is rebuilt on LLVM or Cranelift.
5. Whether rendering differences (font shaping and rasterisation now come from
   the platform text stack) are acceptable, i.e. acceptance is behavioural
   rather than pixel-identical.
6. Which platform backends phase 1 actually ships, given that 1F now covers
   macOS, Windows, Linux/BSD, Android and Haiku.
