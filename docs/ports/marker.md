# emacs/src/marker.c (874 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 8
- [x] marker-buffer  (Fmarker_buffer, 1/1, doc bff10359de2c, line 427)
- [x] marker-position  (Fmarker_position, 1/1, doc 3f494ac72575, line 447)
- [x] marker-last-position  (Fmarker_last_position, 1/1, doc a4c8fa62c515, line 458)
- [x] set-marker  (Fset_marker, 2/3, doc b246b0c09cea, line 607)
- [x] copy-marker  (Fcopy_marker, 0/2, doc 3a66f7316204, line 753)
- [x] marker-insertion-type  (Fmarker_insertion_type, 1/1, doc 1fac03de5903, line 774)
- [x] set-marker-insertion-type  (Fset_marker_insertion_type, 2/2, doc beb59f9f2874, line 784)
- [x] markers-in  (Fmarkers_in, 0/2, doc 164eefa3e001, line 797)

## Functions defined — 18
- [x] byte_char_debug_check  (line 51, static)
- [x] clear_charpos_cache  (line 79)
- [x] CHECK_MARKER  (line 137, static)
- [x] buf_charpos_to_bytepos  (line 167)
- [x] buf_bytepos_to_charpos  (line 320)
- [x] attach_marker  (line 473, static)
- [x] live_buffer  (line 500, static)
- [x] set_marker_internal  (line 510, static)
- [x] set_marker_restricted  (line 620)
- [x] set_marker_both  (line 630)
- [x] set_marker_restricted_both  (line 649)
- [x] detach_marker  (line 675)
- [x] unchain_marker  (line 684)
- [x] marker_position  (line 724)
- [x] marker_byte_position  (line 740)
- [x] count_markers  (line 832)
- [x] verify_bytepos  (line 847)
- [x] syms_of_marker  (line 864)

## Macros defined — 4
- [x] BYTECHAR_DISTANCE_INCREMENT  (line 162, body e1822db470e6)
- [x] BYTECHAR_DISTANCE_INITIAL  (line 161, body e1822db470e6)
- [x] CONSIDER  (line 277, body 79faf2d307ee)
- [x] byte_char_debug_check  (line 74, body cce2980cc6ca)

## Types and their fields — 0

## File-scope objects — 4
- [x] static ptrdiff_t cached_charpos  (line 36, static)
- [x] static ptrdiff_t cached_bytepos  (line 37, static)
- [x] static struct buffer cached_buffer  (line 38, static)
- [x] static modiff_count cached_modiff  (line 39, static)

## Decisions

- **The two `CONSIDER` macros are helper functions** (C 100-136 and C 277-315).  Each ends in an
  early `return` and each is documented not to evaluate its second operand unless the value is
  wanted (C 106-107).  Rust's `macro_rules!` cannot reference the caller's locals - hygiene
  resolves a macro's identifiers at its definition site (measured: `error: cannot find value
  'target' ... not accessible due to macro hygiene`) - and a function would evaluate both
  operands, so each macro's statement sequence lives once in `consider_charpos` /
  `consider_bytepos`, which take the eager operand as a value, the lazy one as an `FnOnce`, the
  four cursor values the macro captures through `&mut`, and return the macro's `return` value as
  `Option`; each of the twelve use sites is `if let Some (value) = … { return value; }`.  The
  macro's name is in docs/ports/excluded-macros.txt and the `consider` token-aliases family
  carries the occurrence check, so dropping a call still fails the gate.
- **`#ifdef MARKER_DEBUG` (C 44-70) / `#else` (C 72-74)** is the P-51 shape: the frozen
  configuration leaves MARKER_DEBUG undefined, so C 74's `do { } while (0)` macro is the live
  text and the port defines `byte_char_debug_check` as a macro that expands to nothing.  The
  arm's function, plus the two debug entry points C 831-859 adds, are kept as
  `#[cfg (feature = "marker_debug")]` items: their bodies need `BUF_BEG_ADDR`/`multibyte_chars_in_text`,
  which this target's mirrors have not got, and the dead-arms gate has no frozen constant to
  flip for them.
- **`#if HAVE_TEXT_CONVERSION` (C 580-602)** is undefined in the frozen configuration, and its
  body calls `bset_redisplay`, which this target's mirror has not got; the arm is recorded in
  docs/ports/marker.md and its tokens in docs/ports/macro-body-notes.tsv (the rawmemchr.c
  CHERI-arm class) rather than written out.
- **`#if EMACS_INT_MAX > PTRDIFF_MAX` (C 544-553)** is false on the frozen target (both are the
  64-bit word), so C's `#else` arm - `charpos = XFIXNUM (position), bytepos = -1;`, which the
  port writes as two statements - is the live text; the wide-int arm's tokens are in the same
  notes row.
- **`struct buffer` is opaque in the lisp.h mirror**, so `Lisp_Marker.buffer` is `*mut c_void`
  and the group's prototypes take `*mut c_void` (lisp.h 4952-4956's comment).  The port's two
  conversion functions therefore take `*mut c_void` and cast to `*mut buffer` once at the top
  (C's parameter is `struct buffer *`; the ABI is identical), `unchain_marker` casts
  `marker->buffer`, and `build_marker` - which the lisp.h mirror declares with the opaque type -
  gets a second, unit-local prototype with the real `*mut buffer` (the tparam.rs precedent).
  `build_marker` itself was added to the lisp.h mirror's `other_units` group in this port (a
  `/// C:` line the lisp-abi-probe type-checks).
- **`insertion_type` writes go through the mirror's mask.**  C writes the 1-bit field
  (`XMARKER (new)->insertion_type = !NILP (type);`); the mirror reads it with
  `MARKER_INSERTION_TYPE_P` and packs it into `bits`, so the port writes `bits = if v { bits |
  MARKER_INSERTION_TYPE } else { bits & !MARKER_INSERTION_TYPE }`, and the `insertion_type`
  token-aliases family (extended with `marker_insertion_type` in this port) keeps the C spelling
  comparable and count-checked.
- **The second macro's `int changed` is a `bool` in the port** (C only ever stores false/true in
  it), the first macro's `bool changed` is already a bool; no other type in either macro differs.
- **The line-count drift (C 874 vs the port's 988, 13.0%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason; the per-function token check is
  unaffected.
- **Call-relationship differences (D7)**: none.  The unit calls `build_marker`,
  `buf_next_char_len`, `buf_prev_char_len`, `decode_buffer`, `error`, `emacs_abort`, its own
  `unchain_marker`, and the subr/`make-fixnum` helpers, as C does.
