# emacs/src/intervals.h (314 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] set_interval_object  (line 147)
- [x] set_interval_parent  (line 155)
- [x] set_interval_plist  (line 162)

## Macros defined — 29
- [x] AM_LEFT_CHILD  (line 80, body 75d6df7dd56d)
- [x] AM_RIGHT_CHILD  (line 84, body 56bab2704d44)
- [x] BOTH_KIDS_P  (line 97, body 6b221477fe70)
- [x] COPY_INTERVAL_CACHE  (line 186, body 18798c8aa850)
- [x] DEFAULT_INTERVAL_P  (line 125, body e8b91f269760)
- [x] EMACS_INTERVALS_H  (line 20, body da39a3ee5e6b)
- [x] GET_INTERVAL_OBJECT  (line 141, body 69772cfb6a7f)
- [x] INTERVAL_EXPRESSLY_WRITABLE_P  (line 217, body 4540c3bb9a63)
- [x] INTERVAL_GENERALLY_WRITABLE_P  (line 210, body 92572abd891d)
- [x] INTERVAL_HAS_OBJECT  (line 130, body 981f6cceda30)
- [x] INTERVAL_HAS_PARENT  (line 129, body cabc692f9ddb)
- [x] INTERVAL_LAST_POS  (line 113, body 868ca50fcc0a)
- [x] INTERVAL_PARENT  (line 138, body 39d13c7b15c1)
- [x] INTERVAL_PARENT_OR_NULL  (line 170, body 181041307952)
- [x] INTERVAL_VISIBLE_P  (line 204, body 562734ea6040)
- [x] LEAF_INTERVAL_P  (line 88, body cabb59d0af0f)
- [x] LEFT_TOTAL_LENGTH  (line 116, body 09f829681350)
- [x] LENGTH  (line 107, body 969f813c8e72)
- [x] MERGE_INTERVAL_CACHE  (line 195, body 1db639537fae)
- [x] NULL_LEFT_CHILD  (line 74, body 848c105e73a1)
- [x] NULL_PARENT  (line 77, body d16419e8df35)
- [x] NULL_RIGHT_CHILD  (line 71, body 9e64d25dae14)
- [x] ONLY_INTERVAL_P  (line 94, body 1e30108e7530)
- [x] RESET_INTERVAL  (line 174, body a4fecf6e2a59)
- [x] RIGHT_TOTAL_LENGTH  (line 119, body 1c9ec5f5ec5f)
- [x] ROOT_INTERVAL_P  (line 91, body b6b61c702fc1)
- [x] TEXT_PROP_MEANS_INVISIBLE  (line 240, body 56e3b8b48cb7)
- [x] TOTAL_LENGTH  (line 101, body de2d8da351d2)
- [x] TOTAL_LENGTH0  (line 104, body 220a8a398f9d)

## Types and their fields — 1
- [x] interval: ptrdiff_t total_length, ptrdiff_t position, struct interval* left, struct interval* right, struct interval* interval, Lisp_Object obj, <aggregate> up, bool_bf up_obj : 1, bool_bf gcmarkbit : 1, bool_bf write_protect : 1, bool_bf visible : 1, bool_bf front_sticky : 1, bool_bf rear_sticky : 1, Lisp_Object plist

## File-scope objects — 0

## Decisions

- **What the mirror carries**: the include guard (`EMACS_INTERVALS_H`), `struct interval` with
  its `bits: u8` byte and the `interval_up` union, the six `INTERVAL_*_BIT` masks and the
  `interval_bits!` macro (PORTING.md 1.3/P-80, the cm.rs `bits_*` shape), C's 29 macros in
  C's order, the three INLINE setters, and every `extern` prototype -- the intervals.c block
  in an `intervals_decls` module + glob import (PORTING.md 0.1-10) because that unit defines
  24 of them, and the alloc.c/xdisp.c/textprop.c declarations at the root.
- **The measured layout**: clang, with the spikes/run.sh flags, reports `sizeof (struct
  interval)` 56 / align 8, offsets `total_length` 0, `position` 8, `left` 16, `right` 24,
  `up` 32, the six bits at 40:0-40:5 (one byte, `bool_bf` is `bool` here per conf_post.h 61),
  `plist` 48.  `struct interval` is not in `docs/baseline/layout/c-layout-*.tsv` (the frozen
  baseline's key set is a ratchet that does not name it), so `xtask layout-asserts` generates
  nothing for it and the measurement is recorded here instead (the include-ccl.md shape).
- **`TEXT_PROP_MEANS_INVISIBLE` was retyped to `c_int`** in this commit: the WIP mirror's
  two arms were `bool` and `c_int`, which cannot type-check, and C's value *is* an int
  (xdisp.c 3968/3972 compare it with `0` and `2`).  The unit's one use writes `!= 0`.
- **`LEAF_INTERVAL_P`/`BOTH_KIDS_P` are spelled out** with C's own two field comparisons:
  routing them through `NULL_LEFT_CHILD!`/`NULL_RIGHT_CHILD!` hid the `left`/`right`/`null`
  tokens the macro-body check needs.
- **`interval_up` is `#[derive(Clone, Copy)]`** (the 13 lisp.rs unions' shape) because C's
  `copy_interval_parent` assigns the whole union (`d->up = s->up`; PORTING.md 0.1-9).
- **Two macros cast their own operand**: `INTERVAL_PARENT_OR_NULL`'s value is C's `struct
  interval *` (= the port's `INTERVAL`) while its `INTERVAL_PARENT!` arm is the raw
  `*mut interval` field, and `RESET_INTERVAL` passes `$i` to the two `INTERVAL`-taking
  setters.  Both take the `as INTERVAL` cast the mirror's uniform `((i) as *mut interval)`
  shape already implies (P-72 rule 4).
- **The six token rows the gate cannot express** are in `docs/ports/macro-body-notes.tsv`:
  the `do { … } while (false)` `false` of `COPY_INTERVAL_CACHE`/`MERGE_INTERVAL_CACHE` (the
  `CHECK_CCL_PROGRAM` class), `set_interval_object`'s `1` (C's `i->up_obj = 1` is the port's
  `interval_bits! (up_obj, i, true)`), and the `0` of `INTERVAL_HAS_PARENT`/
  `INTERVAL_PARENT`/`INTERVAL_PARENT_OR_NULL` (C's literal 0 is the port's
  `is_null ()`/`null_mut ()`, the `FRAME_LIVE_P` class).
- **The `up_obj`/`write_protect`/`visible`/`front_sticky`/`rear_sticky` families** in
  `docs/ports/token-aliases.tsv` carry the occurrence check for every read and write of the
  packed bits (both inside the mirror's macros and in `femacs/src/intervals.rs`); the
  `gcmarkbit` bit is alloc.c's alone and has no family yet.
- **`femacs/src/include/buffer.rs` needed one integrator edit** and this unit reports it:
  buffer.h's six prototypes for functions other units define (intervals.c's five
  point-setters and buffer.c's `enlarge_buffer_text`) moved into a `buffer_point_decls`
  module + glob import, because a root-scope declaration plus intervals.c's definition is one
  `E0428` (PORTING.md 0.1-10, the P-70 same-commit rule; the composite.rs/`window_redisplay_decls`
  shape).  No other mirror declares those six names.
- **`#if false` at intervals.h 227-234** (`FRONT_STICKY_P`/`END_NONSTICKY_P`/
  `FRONT_NONSTICKY_P`) is literal-false text, not a configuration arm, so P-51's feature gate
  does not apply and the three macros are not mirrored (the cm.rs/font.rs precedent).
