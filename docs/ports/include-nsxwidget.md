# emacs/src/nsxwidget.h (82 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] NSXWIDGET_H_INCLUDED  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header's text is unconditional, and nothing on this target compiles it.**  Its C
  includers are `src/xwidget.c` 51 (behind `NS_IMPL_COCOA`) and `src/xwidget.h` 41-44 (behind
  `NS_IMPL_COCOA && __OBJC__`).  The frozen configuration has no `HAVE_NS`, no `HAVE_XWIDGETS`
  and no `NS_IMPL_COCOA` (`docs/baseline/config/darwin-arm64.macros.tsv`), and
  `docs/baseline/frozen-build-sources.txt` lists no `.m`, so no translation unit of this target
  includes `nsxwidget.h`; the header itself has
  no platform guard, so the mirror carries its live text anyway (the `src/w32select.h` reading,
  status 3.158).  There is no `src/nsxwidget.rs` empty unit: the unit that implements this
  header is `src/nsxwidget.m`, whose port target is `platform/macos/nsxwidget.swift` (the
  Swift lane's, still `todo` in `docs/port-inventory.md`).
- **The three dead arms are recorded, not transcribed** (PORTING.md P-08;
  `tools/check-cond-eval.py src/nsxwidget.h` -> `4 conditions, 0 unknown to the evaluator, 0
  disagreements`; `__OBJC__` is undefined because this configuration compiles C): C 24-26 (the
  `#import <AppKit/NSView.h>` and its guard), C 47-51 (`@interface XwWindow`) and C 60-65
  (`@interface XvWindow`) - Objective-C class declarations with no Rust counterpart.  C 20-21 is the
  include guard; C 28-43, C 53-56 and C 67-81 (21 prototypes) are live text.  The file's own
  trailing local-variables block is absent from the reference (C 82 is the guard's `#endif`), so
  there is none to carry.
- **Degraded names (PORTING.md P-55/P-72 rule 3)**: `struct xwidget`, `struct xwidget_view` and
  `struct glyph_string` have no mirror in the port (`docs/port-inventory.md` still has
  `src/xwidget.h` as `todo`, and `include/dispextern.rs` carries the glyph datatype but not
  `struct glyph_string` - the same finding `include-haikuterm.md` records for its two prototypes),
  so every pointer to one is `*mut c_void` with the C text in a `/// C:` line above the
  declaration, and C's pointer depth is kept.
- **`Lisp_Object` is named but not declared** - the `w32inevt.rs` reading of PORTING.md 0.1-9:
  C 29 includes `lisp.h`, and the port's lisp.h mirror is the one header every translation unit
  reaches through its own includes, so the mirror does not `include!` it (`spikes/ns_inc_probe.rs`
  supplies it the way a unit does).  `bool`, `f64` and `c_int` are C's `bool`/`double`/`int`
  (PORTING.md 1.1), and `char *`/`const char *` are `*mut c_char`/`*const c_char`.
- **The 21 declarations stay at the mirror's top level** (the `w32select.rs` shape rather than
  `haiku_support.rs`'s `pub mod … + use …::*;`): the unit that defines them is the Swift port of
  `src/nsxwidget.m`, which never `include!`s this mirror, so there is no same-crate definition for a
  declaration to collide with.
- **`--check` asks for less than fidelity does.**  The extractor's item list for this header is the
  guard macro alone (`## Functions defined — 0` counts definitions, not declarations), so a mirror
  holding only `NSXWIDGET_H_INCLUDED` would already check OK; the 21 declarations are here because
  C 32-81 is live text, and this paragraph keeps that from looking like a required item later.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/nsxwidget.h src/include/nsxwidget.rs`
  -> `OK emacs/src/nsxwidget.h -> src/include/nsxwidget.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/nsxwidget.h` -> `4 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/nsxwidget.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 0 types, 0 file-scope objects.
- Compile proof: this mirror had no compiler at all (no unit includes it, `layout-check` was not
  written for the platform-exclusive mirrors), so this round added `spikes/ns_inc_probe.rs` - the
  minimal `spikes/f_inc_probe.rs` shape: it declares the `c_*` aliases and stubs `Lisp_Object` as
  the frozen configuration's one machine word, then `include!`s `nsgui.rs` and this mirror.  The
  probe is **not** in `xtask`'s `PROBES` list (this lane may not edit `xtask`); the integrator
  decides that, and whether this mirror joins `docs/baseline/layout-excluded-mirrors.txt` (`xtask
  layout-check` compiles it without error today - measured: this round's only layout-probe failure
  was `nsgui.rs`'s `Emacs_Cursor`).
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-ns cargo run -q -p xtask -- probe
  ../spikes/ns_inc_probe.rs /tmp/probes-ns/ns.rmeta` -> exit 0.
- **The 21 declarations were checked mechanically against the reference text** (no gate
  inventories prototypes, which is why `## Functions defined — 0`): every C prototype of C 34-81
  matches the `/// C:` line above its Rust declaration character for character after whitespace
  normalisation, 21 of 21, and there is no extra `/// C:` line (the `haiku_support.md` convention
  for the same blind spot).  The `*mut c_void` degradations are C 34-43's `struct xwidget *` (8
  prototypes), C 53-56's `struct xwidget *` (4) and C 67-81's `struct xwidget_view *` and one
  `struct glyph_string *`.
- `rg -n "nsxwidget_is_web_view|nsxwidget_webkit_uri|nsxwidget_init_view|nsxwidget_set_needsdisplay"
  femacs` -> only this mirror: the port has no definer yet (the Swift lane has not written
  `platform/macos/nsxwidget.swift`), so there is no use-site copy to retire.
