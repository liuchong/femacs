# emacs/src/font.h (1034 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 20
- [x] FONTP  (line 435)
- [x] FONT_SPEC_P  (line 442)
- [ ] GC_FONT_SPEC_P  (line 449)
- [x] FONT_ENTITY_P  (line 456)
- [ ] GC_FONT_ENTITY_P  (line 463)
- [x] FONT_OBJECT_P  (line 470)
- [ ] GC_FONT_OBJECT_P  (line 477)
- [ ] CHECK_FONT  (line 485)
- [ ] CHECK_FONT_SPEC  (line 491)
- [ ] CHECK_FONT_ENTITY  (line 497)
- [ ] CHECK_FONT_OBJECT  (line 503)
- [ ] XFONT_SPEC  (line 511)
- [ ] GC_XFONT_SPEC  (line 518)
- [ ] XFONT_ENTITY  (line 525)
- [ ] GC_XFONT_ENTITY  (line 532)
- [ ] XFONT_OBJECT  (line 539)
- [ ] GC_XFONT_OBJECT  (line 546)
- [ ] CHECK_FONT_GET_OBJECT  (line 555)
- [ ] valid_font_driver  (line 902)
- [ ] font_data_structures_may_be_ill_formed  (line 1021)

## Macros defined — 28
- [x] EMACS_FONT_H  (line 23, body da39a3ee5e6b)
- [ ] FONT_ADD_LOG  (line 1004, body 9c0a062da66b)
- [ ] FONT_BASE  (line 245, body fa7425ff23ab)
- [ ] FONT_DEBUG  (line 998, body da39a3ee5e6b)
- [ ] FONT_DEFERRED_LOG  (line 1010, body 2be01ccfaba8)
- [ ] FONT_DESCENT  (line 246, body 9ac285903217)
- [ ] FONT_ENCODING_NOT_DECIDED  (line 273, body 3028f51407d8)
- [ ] FONT_HEIGHT  (line 244, body 17ec4152d2d2)
- [ ] FONT_INVALID_CODE  (line 585, body bd700042aab3)
- [ ] FONT_PIXEL_SIZE_QUANTUM  (line 583, body 356a192b7913)
- [ ] FONT_SET_STYLE  (line 236, body c194bafab603)
- [ ] FONT_SLANT_FOR_FACE  (line 217, body 1a82bbad5872)
- [ ] FONT_SLANT_NAME_NUMERIC  (line 227, body 3a529e8dc132)
- [ ] FONT_SLANT_NUMERIC  (line 197, body 518e029c9a6e)
- [ ] FONT_SLANT_SYMBOLIC  (line 208, body b630f2b4c11e)
- [ ] FONT_WEIGHT_FOR_FACE  (line 214, body 1215e8b22009)
- [ ] FONT_WEIGHT_NAME_NUMERIC  (line 224, body 84c695f2ce3e)
- [ ] FONT_WEIGHT_NUMERIC  (line 193, body 77d5c64e797a)
- [ ] FONT_WEIGHT_SYMBOLIC  (line 205, body 61d819e5e43a)
- [ ] FONT_WIDTH  (line 240, body f9796d0cbc37)
- [ ] FONT_WIDTH_FOR_FACE  (line 220, body 4159c49613a0)
- [ ] FONT_WIDTH_NAME_NUMERIC  (line 230, body dadeeb47050a)
- [ ] FONT_WIDTH_NUMERIC  (line 201, body e80f3e7f5190)
- [ ] FONT_WIDTH_SYMBOLIC  (line 211, body d1d27c38efcc)
- [ ] PIXEL_TO_POINT  (line 579, body 2b796ab22aad)
- [ ] POINT_TO_PIXEL  (line 572, body 25504e1cec3d)
- [ ] PT_PER_INCH  (line 563, body cd14d6bdf7d0)
- [ ] XSETFONT  (line 552, body a7abab8225c7)

## Types and their fields — 7
- [x] font: union vectorlike_header header, Lisp_Object props, int min_width, int max_width, int pixel_size, int height, int space_width, int average_width, int ascent, int descent, struct font_driver const* driver
- [ ] font_bitmap: int bits_per_pixel, int rows, int width, int pitch, unsigned char* buffer, int left, int top, int advance
- [ ] font_driver: Lisp_Object type, bool case_sensitive, Lisp_Object (*) get_cache, Lisp_Object (*) list, Lisp_Object (*) match, Lisp_Object (*) list_family, void (*) free_entity, Lisp_Object (*) open_font, void (*) close_font, void (*) prepare_face, void (*) done_face, int (*) has_char, unsigned (*) encode_char, void (*) text_extents, int (*) anchor_point, Lisp_Object (*) otf_capability, int (*) otf_drive, int (*) start_for_frame, int (*) end_for_frame, Lisp_Object (*) shape, int (*) check, int (*) get_variation_glyphs, void (*) filter_properties, bool (*) cached_font_ok, Lisp_Object (*) combining_capability, void (*) drop_xrender_surfaces
- [ ] font_driver_list: bool on, struct font_driver const* driver, struct font_driver_list* next
- [ ] font_entity: union vectorlike_header header, Lisp_Object props
- [ ] font_metrics: short lbearing, short rbearing, short width, short ascent, short descent
- [ ] font_spec: union vectorlike_header header, Lisp_Object props

## File-scope objects — 0

## Decisions

- **This round carries `struct font` (font.h 275-418)** and nothing else new: `EMACS_FONT_H`,
  `enum font_property_index` (as constants) and the four predicates were already here.  The
  record sits in C's order (after the property constants, before the predicates), and its
  `HAVE_WINDOW_SYSTEM` members are `#[cfg (feature = "window_system")]` (P-51), the shape
  `font.rs`'s use-site copy had.  The trigger is `haikuterm.h`'s `struct haikufont_info`,
  whose first member is this record **by value** (haikuterm.h 42-44, status 3.154).
- **Still open in this ledger** (the unchecked boxes above): `struct font_spec`,
  `struct font_entity`, `enum font_spacing`, `struct font_metrics`, `struct font_bitmap`,
  `struct font_driver`, `struct font_driver_list`, the accessors (`XFONT_*`, `GC_XFONT_*`,
  `CHECK_FONT*`, `CHECK_FONT_GET_OBJECT`, `XSETFONT`), the `FONT_*` style/point/pixel macros
  and `valid_font_driver`/`font_data_structures_may_be_ill_formed`.  They are the next rounds'
  work, not this one's.
- **`driver` is `*const c_void`, not `*const font_driver`.**  `struct font_driver` is this
  header's own record and has not landed here, and Rust has no incomplete tag: naming it would
  break the ~24 units that include `font.rs` without defining it, while a stand-in declared
  here would be `E0428` in the three units that *do* define their own
  (`src/font.rs`, `src/alloc.rs`, `src/composite.rs`).  The field therefore keeps the untyped
  pointer with a `/// C:` line (PORTING.md P-55/P-72 rule 3; C's `const` is kept, as the
  `xfaces.rs` copy had it), and the reads of it cast at the use site (30 occurrences:
  `src/font.rs` 28, `src/alloc.rs` 1, `src/composite.rs` 1) — the `frame.rs`
  `face_cache`/`font_driver_list` precedent.  Flipping it to `*const font_driver` and deleting
  those casts is the commit that lands `struct font_driver` here.
- **Five use-site copies retired, not three.**  Besides `src/font.rs` 256, `src/window.rs`
  893 and `src/xfaces.rs` 337, `src/alloc.rs` 3429 and `src/composite.rs` 126 also declared a
  `struct font` (byte-padded prefixes) and would have been `E0428` through their own
  `include!("include/font.rs")` (alloc.rs 76, composite.rs 44).  All five are gone.  `font.rs`
  keeps its `struct font_driver`/`font_driver_list`, and `alloc.rs`/`composite.rs` keep their
  `font_driver` prefixes, so the only other edits the retirement needed were the 30
  `(…).driver as *const font_driver` casts.
- Two comments made stale by this change are fixed with it: `src/xfaces.rs`'s use-site note
  and `include/dispextern.rs`'s two "they need `struct font`" notes.  Still stale and *not*
  this lane's files (reported to the integrator instead): `include/haikuterm.rs` 185/294,
  `include/androidterm.rs` 250, and the in-flight `src/xdisp.rs` 6548-6549.

## Evidence

- Frozen-clang measurement of the record (the numbers the mirror's `const _` assertions use),
  run from the repository root:

      printf '%s\n' '#include <config.h>' '#include <stddef.h>' '#include "lisp.h"' \
        '#include "frame.h"' '#include "dispextern.h"' '#include "font.h"' \
        'int main (void) { printf ("%zu %zu %zu %zu\n", sizeof (struct font),' \
        '  _Alignof (struct font), offsetof (struct font, props),' \
        '  offsetof (struct font, driver)); return 0; }' > /tmp/font_layout_probe.c
      clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim \
        -I build/emacs-ref/src -I emacs/src -I emacs/lib \
        /tmp/font_layout_probe.c -o /tmp/font_layout_probe && /tmp/font_layout_probe

  `Apple clang version 21.0.0 (clang-2100.3.34.2)`; output `sizeof` 184, `alignof` 8,
  `header`@0, `props`@8, `min_width`@144, `max_width`@148, `pixel_size`@152, `height`@156,
  `space_width`@160, `average_width`@164, `ascent`@168, `descent`@172, `driver`@176,
  `FONT_OBJECT_MAX` 17.  The mirror pins `sizeof`/`alignof` and all twelve offsets.
- The five retirements rebuild (from `femacs/`, `CARGO_TARGET_DIR=/tmp/femacs-target-font`):
  `cargo run -q -p xtask -- build-tu src/font.c` -> `built src/font.rs -> ../build/femacs/font.o`;
  `src/window.c` -> `built src/window.rs -> ../build/femacs/window.o`; `src/xfaces.c` ->
  `built src/xfaces.rs -> ../build/femacs/xfaces.o`; the two units whose padded prefixes went
  too, `src/alloc.c` -> `built src/alloc.rs -> ../build/femacs/alloc.o` and `src/composite.c`
  -> `built src/composite.rs -> ../build/femacs/composite.o`.  A sample of further `font.rs`
  consumers (`src/dispnew.c`, `src/data.c`, `src/frame.c`, `src/term.c`, `src/xdisp.c`,
  `src/keyboard.c`) all report `built`; `cargo run -q -p xtask -- layout-check` ->
  `layout-check passed`; and the two platform mirrors that include this one still compile under
  their own probes (`probe ../spikes/haiku_inc_probe.rs …` and the android one, exit 0).
- `python3 tools/extract-abi.py --check emacs/src/font.h femacs/src/include/font.rs`: the
  `missing type deferred: src/font.h font` line is **gone** (the record is found now) and no
  `- missing field font.…` line appears.  The 49 remaining `- ` problems are exactly the
  ledger's other unchecked boxes (16 functions, 26 macros, 7 constants) — the same list this
  file's boxes already named before the round, so this check cannot report `OK` until the rest
  of the header lands (the ledger's own rule; `--check` has no exemption for a WIP mirror
  except the missing-type one).
- The unit pairs stay `OK`: `python3 tools/extract-abi.py --check emacs/src/font.c
  femacs/src/font.rs` (also `window.c`, `xfaces.c`, `alloc.c`, `composite.c`), and
  `python3 tools/check-mirror-dupes.py` reports `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)`.
- `python3 tools/extract-abi.py --check emacs/src/haikuterm.h
  femacs/src/include/haikuterm.rs` -> `OK`; its `missing type deferred: src/haikuterm.h
  haikufont_info` line is unchanged, and the type that blocked that record — this one — now
  exists, so the remaining reason is only that `haikufont_info` itself is not written in that
  mirror (the Haiku lane's file, whose ledger box is still unchecked).
