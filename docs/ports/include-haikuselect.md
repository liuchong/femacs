# emacs/src/haikuselect.h (88 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _HAIKU_SELECT_H_  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The live text is carried whole; the three dead arms are recorded, not transcribed** (PORTING.md
  P-08; `tools/check-cond-eval.py src/haikuselect.h` -> `4 conditions, 0 unknown to the evaluator,
  0 disagreements`).  `__cplusplus` is undefined for the frozen configuration, so: C 22-24 is dead
  (the C++ half of the include pair - `<cstdio>`/`<cstdint>`; the live half is C 26-27's
  `<stdio.h>`/`<stdint.h>`), C 39-42 and C 82-84 are dead (the `extern "C" { … }` braces, C++
  linkage decoration with no Rust counterpart).  C 19-20 is the include guard and C 86-88 is the
  file's own local-variables block, carried verbatim (PORTING.md 0.3).
- **The unit and the header have two different answers, and both are right** (the
  `src/w32select.h` reading, status 3.158).  The frozen configuration compiles no Haiku unit
  (`docs/baseline/frozen-build-sources.txt` lists no `src/haiku*.c`), so `femacs/src/haikuselect.rs`
  is the recorded empty shape; the header's own text is unconditional, so the mirror carries its
  declarations and their definitions are deferred to the Haiku target.
- **`enum haiku_clipboard` (C 32-37) is a `pub type` plus one constant per member** - the shape
  PORTING.md 1.3 fixes for an enum whose members C compares as integers (`haiku_support.rs`'s
  `haiku_cursor`, `termhooks.rs`, `dispextern.rs`).  The members have no initialisers, so C's rule
  gives 0/1/2; `extract-abi.py` expands them the same way and compares them as constants.
- **The names from the system are named but not declared** (PORTING.md 0.1-9): `int32`/`int64`/
  `uint32` are Haiku's `SupportDefs.h` spellings (C 30) and are written as the `<stdint.h>` names of
  the live include at C 27 (`int32_t`/`int64_t`/`uint32_t`, carried by `lib/include/stdint.in.rs`);
  declaring a second `int32` here would be an `E0428` against `haiku_support.rs`, which already owns
  that alias for the same toolchain header and lands in the same crate
  (`spikes/haiku_inc_probe.rs`).  `ssize_t` comes from `lib/include/unistd.in.rs` (the
  `SupportDefs.h` chain reaches `<sys/types.h>` - the reading `include-haiku_support.md` records for
  the same header).  `intmax_t` (C 78-79) is spelled `i64`, PORTING.md 1.1's mapping, because the
  port's alias lives in `lisp.rs`, which this header does not include.
- **Nothing here is degraded**: the brief's P-55/P-72 case does not arise - `void *` is C's own
  untyped pointer and stays `*mut c_void`, and every other pointer is to a scalar or to `char`
  (`c_char`, `f32`, `int64_t`).  C's pointer depths are kept (`const void **` -> `*mut *const
  c_void`, `void **` -> `*mut *mut c_void`).
- **The 25 declarations sit in `pub mod haikuselect_decls` with a glob import** (PORTING.md
  0.1-10/P-17/P-21, the `haiku_support.rs` shape): `src/haikuselect.c` defines one of them and
  `src/haiku_select.cc` the rest, so a defining unit's own definitions shadow these declarations
  instead of colliding with them.  Unnamed C parameters become `arg0`, `arg1`, … (the
  `pdumper.rs`/`haiku_support.rs` convention).
- **`--check` asks for less than fidelity does.**  The extractor's item list for this header is the
  guard macro and the three enum members (`## Functions defined — 0` counts definitions, not
  declarations), so a mirror holding only those would already check OK; the 25 declarations are
  here because C 43-79 is live text, and this paragraph keeps that from looking like a required item
  later.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/haikuselect.h femacs/src/include/haikuselect.rs`
  -> `OK emacs/src/haikuselect.h -> femacs/src/include/haikuselect.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/haikuselect.h` -> `4 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/haikuselect.h` -> this ledger: 0 subrs, 0 functions
  defined, 1 macro (the guard), 0 types, 0 file-scope objects.
- Compile proof: the Haiku platform probe, which compiles this mirror with `haikugui.rs`,
  `haiku_support.rs` and `haikuterm.rs` (docs/status.md 3.149's probe, extended this round):
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-ns cargo run -q -p xtask -- probe
  ../spikes/haiku_inc_probe.rs /tmp/probes-ns/haiku.rmeta` -> exit 0.  (The output directory has
  to exist first: `rustc`'s temp dir is created inside it - the first run of this round failed with
  `couldn't create a temp dir: No such file or directory` until `mkdir -p /tmp/probes-ns`.
  `xtask layout-check` also compiles this mirror without error - measured: this round's only
  layout-probe failure was `nsgui.rs`'s `Emacs_Cursor`.)
- **The 25 declarations were checked mechanically against the reference text** (no gate
  inventories prototypes, which is why `## Functions defined — 0`): every C prototype of C 43-79
  matches the `/// C:` line above its Rust declaration character for character after whitespace
  normalisation, 25 of 25, and there is no extra `/// C:` line (the `haiku_support.md` convention
  for the same blind spot).
- `rg -n "be_clipboard_init|be_find_clipboard_data|be_display_notification|haiku_selection_disowned"
  femacs` -> only this mirror: the port has no definer yet (`femacs/src/haikuselect.rs` is the empty
  note), so there is no use-site copy to retire (unlike 3.143/3.153).
