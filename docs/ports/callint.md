# emacs/src/callint.c (933 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 4
- [x] interactive  (Finteractive, 0/UNEVALLED, doc 982cfc4f7114, line 37)
- [x] funcall-interactively  (Ffuncall_interactively, 1/MANY, doc 4a20b9bc85fc, line 239)
- [x] call-interactively  (Fcall_interactively, 1/3, doc c61e4f94d07e, line 256)
- [x] prefix-numeric-value  (Fprefix_numeric_value, 1/1, doc d0884ed23a8f, line 805)

## Functions defined — 6
- [x] quotify_arg  (line 127, static)
- [x] quotify_args  (line 139, static)
- [x] check_mark  (line 156, static)
- [x] fix_command  (line 175, static)
- [x] read_file_name  (line 230, static)
- [x] syms_of_callint  (line 829)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 3
- [x] static Lisp_Object preserved_fns  (line 29, static)
- [x] static Lisp_Object point_marker  (line 32, static)
- [x] static Lisp_Object callint_message  (line 35, static)

## Decisions

- **callint.c has no `#if` at all**, so nothing here is an arm decision.
- **The `switch (*tem)` and its two fall-throughs plus one `goto`.**  C 459-751's switch becomes
  a `match` on the same characters with the same arms in C's order; two entries share a body
  with another arm and cannot in Rust: `case 'N'` jumps to `have_prefix_arg:` (the middle of
  `case 'p'`) when there is a prefix arg and otherwise falls through into `case 'n'`, and
  `case 'R'` falls through into `case 'r'` unless the region is inactive.  The port keeps one
  copy of each shared body and selects the entry (the tparam.c chain, PORTING.md 2.2), and the
  invalid control letter body lives in the `invalid_control_letter` helper so that C 731-732's
  `case '+': default:` keeps a single body.  The `switch`/`case`/`default`/`goto`/`fallthrough`/
  `have_prefix_arg` tokens are in docs/ports/macro-body-notes.tsv.
- **`KVAR (current_kboard, Vlast_command)` is the port's `(*current_kboard).Vlast_command_`**
  (keyboard.h 38's token-pasting macro, docs/ports/excluded-macros.txt).
- **The three `static Lisp_Object`s start as `lisp_h_Qnil ()`** (C's statics are
  zero-initialized and NIL_IS_ZERO holds, PORTING.md 1.3); `callint_argfuns` is `static mut`,
  because a `static` array of `*const c_char` is not `Sync`.
- **The `alloca` ladder is the mirror's four-macro shape**: `USE_SAFE_ALLOCA! (sa_avail,
  sa_count)`, `SAFE_ALLOCA! (sa_avail, …)`, `SAFE_NALLOCA! (sa_avail, args, 3, nargs)`,
  `SAFE_FREE! (sa_count)`; `args` starts as a null pointer because the macro reads the pointee's
  size from the binding.
- **`Finteractive`'s `attributes: const` (C 118) has no `DEFUN!` form** (the macro carries
  `attributes: noreturn` only), so the port's subr record does not set that attribute; recorded
  here as the one deviation of this file.
- **Mirror work this port needed**: `message1_nolog` (lisp.h 4391), the rest of the marker.c
  group's prototypes (lisp.h 4958-4966: `unchain_marker`, `set_marker_restricted`,
  `set_marker_both`, `set_marker_restricted_both`, `syms_of_marker`),
  `temporarily_switch_to_single_kboard` (keyboard.h 483), and `termhooks.rs` added to the
  include list because keyboard.h's `input_event` lives there.
- **The line-count drift (C 933 vs the port's 983, 5.4%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none.  The unit calls the mirror's reading
  primitives, the marker allocator and `funcall` exactly as C does.
