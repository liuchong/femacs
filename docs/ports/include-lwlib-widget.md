# emacs/lwlib/lwlib-widget.h (105 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_WIDGET_H  (line 24, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `lwlib/lwlib-widget.h`, complete for this target: the `change_type` and
`button_type` enums, the `widget_value` record and its fields.  `lwlib-widget.h` 26's
`#include "../src/lisp.h"` is the including unit's `include! ("../../src/include/lisp.rs")`,
the `disptab.h` precedent, so this mirror does not pull lisp.rs in itself; it names only
`Lisp_Object` from it.

`widget_value` is the one structure here the toolkit-independent `src/xmenu.c` uses.  The
frozen configuration builds no window system, so even this header is included by no
translation unit of the build, and no probe compiles it; the text gate is the whole acceptance
(docs/status.md 3.31/3.102).

### Deviations

1. C 70-73 (`#ifdef HAVE_NTGUI`) and C 87-101 (`#ifdef USE_X_TOOLKIT`) are both dead in the
   frozen configuration (`HAVE_NTGUI` and `USE_X_TOOLKIT` are `undef` in
   docs/baseline/config/darwin-arm64.macros.tsv) and are carried as `#[cfg (feature = ...)]`
   field text (P-51), which keeps `title`, `change`, `this_one_change`, `toolkit_data` and
   `free_toolkit_data` in the mirror.  The frozen `widget_value` therefore has neither.
2. The ledger's "Macros defined" section prints `tools/cinventory.py`'s `macros` map alone, so
   the seven enum members (`NO_CHANGE` ... `BUTTON_TYPE_RADIO`, C 28-41) are not checklist rows
   even though the `--check` constants rule compares them; all seven are mirrored (PORTING.md
   1.3).
3. `typedef struct _widget_value { ... } widget_value;` is mirrored as the tag struct plus
   `pub type widget_value = _widget_value;`, so the `contents`/`next` fields keep C's
   `struct _widget_value *` spelling.
4. The mirror is not X-dependent, so it names no X11 type at all - the only header of the
   thirteen that does not.

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-widget.h
lwlib/include/lwlib-widget.rs` -> `OK` (1 ledger item(s), the include guard).
