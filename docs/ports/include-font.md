# emacs/src/font.h (1034 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 20
- [x] FONTP  (line 435)
- [x] FONT_SPEC_P  (line 442)
- [x] GC_FONT_SPEC_P  (line 449)
- [x] FONT_ENTITY_P  (line 456)
- [x] GC_FONT_ENTITY_P  (line 463)
- [x] FONT_OBJECT_P  (line 470)
- [x] GC_FONT_OBJECT_P  (line 477)
- [x] CHECK_FONT  (line 485)
- [x] CHECK_FONT_SPEC  (line 491)
- [x] CHECK_FONT_ENTITY  (line 497)
- [x] CHECK_FONT_OBJECT  (line 503)
- [x] XFONT_SPEC  (line 511)
- [x] GC_XFONT_SPEC  (line 518)
- [x] XFONT_ENTITY  (line 525)
- [x] GC_XFONT_ENTITY  (line 532)
- [x] XFONT_OBJECT  (line 539)
- [x] GC_XFONT_OBJECT  (line 546)
- [x] CHECK_FONT_GET_OBJECT  (line 555)
- [x] valid_font_driver  (line 902)
- [x] font_data_structures_may_be_ill_formed  (line 1021)

## Macros defined — 28
- [x] EMACS_FONT_H  (line 23, body da39a3ee5e6b)
- [x] FONT_ADD_LOG  (line 1004, body 9c0a062da66b)
- [x] FONT_BASE  (line 245, body fa7425ff23ab)
- [x] FONT_DEBUG  (line 998, body da39a3ee5e6b)
- [x] FONT_DEFERRED_LOG  (line 1010, body 2be01ccfaba8)
- [x] FONT_DESCENT  (line 246, body 9ac285903217)
- [x] FONT_ENCODING_NOT_DECIDED  (line 273, body 3028f51407d8)
- [x] FONT_HEIGHT  (line 244, body 17ec4152d2d2)
- [x] FONT_INVALID_CODE  (line 585, body bd700042aab3)
- [x] FONT_PIXEL_SIZE_QUANTUM  (line 583, body 356a192b7913)
- [x] FONT_SET_STYLE  (line 236, body c194bafab603)
- [x] FONT_SLANT_FOR_FACE  (line 217, body 1a82bbad5872)
- [x] FONT_SLANT_NAME_NUMERIC  (line 227, body 3a529e8dc132)
- [x] FONT_SLANT_NUMERIC  (line 197, body 518e029c9a6e)
- [x] FONT_SLANT_SYMBOLIC  (line 208, body b630f2b4c11e)
- [x] FONT_WEIGHT_FOR_FACE  (line 214, body 1215e8b22009)
- [x] FONT_WEIGHT_NAME_NUMERIC  (line 224, body 84c695f2ce3e)
- [x] FONT_WEIGHT_NUMERIC  (line 193, body 77d5c64e797a)
- [x] FONT_WEIGHT_SYMBOLIC  (line 205, body 61d819e5e43a)
- [x] FONT_WIDTH  (line 240, body f9796d0cbc37)
- [x] FONT_WIDTH_FOR_FACE  (line 220, body 4159c49613a0)
- [x] FONT_WIDTH_NAME_NUMERIC  (line 230, body dadeeb47050a)
- [x] FONT_WIDTH_NUMERIC  (line 201, body e80f3e7f5190)
- [x] FONT_WIDTH_SYMBOLIC  (line 211, body d1d27c38efcc)
- [x] PIXEL_TO_POINT  (line 579, body 2b796ab22aad)
- [x] POINT_TO_PIXEL  (line 572, body 25504e1cec3d)
- [x] PT_PER_INCH  (line 563, body cd14d6bdf7d0)
- [x] XSETFONT  (line 552, body a7abab8225c7)

## Types and their fields — 7
- [x] font: union vectorlike_header header, Lisp_Object props, int min_width, int max_width, int pixel_size, int height, int space_width, int average_width, int ascent, int descent, struct font_driver const* driver
- [x] font_bitmap: int bits_per_pixel, int rows, int width, int pitch, unsigned char* buffer, int left, int top, int advance
- [x] font_driver: Lisp_Object type, bool case_sensitive, Lisp_Object (*) get_cache, Lisp_Object (*) list, Lisp_Object (*) match, Lisp_Object (*) list_family, void (*) free_entity, Lisp_Object (*) open_font, void (*) close_font, void (*) prepare_face, void (*) done_face, int (*) has_char, unsigned (*) encode_char, void (*) text_extents, int (*) anchor_point, Lisp_Object (*) otf_capability, int (*) otf_drive, int (*) start_for_frame, int (*) end_for_frame, Lisp_Object (*) shape, int (*) check, int (*) get_variation_glyphs, void (*) filter_properties, bool (*) cached_font_ok, Lisp_Object (*) combining_capability, void (*) drop_xrender_surfaces
- [x] font_driver_list: bool on, struct font_driver const* driver, struct font_driver_list* next
- [x] font_entity: union vectorlike_header header, Lisp_Object props
- [x] font_metrics: short lbearing, short rbearing, short width, short ascent, short descent
- [x] font_spec: union vectorlike_header header, Lisp_Object props

## File-scope objects — 0

## Decisions

- **This round carries `struct font` (font.h 275-418)** and nothing else new: `EMACS_FONT_H`,
  `enum font_property_index` (as constants) and the four predicates were already here.  The
  record sits in C's order (after the property constants, before the predicates), and its
  `HAVE_WINDOW_SYSTEM` members are `#[cfg (feature = "have_window_system")]` (P-51), the shape
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

## Decisions (font_driver round)

- **`struct font_driver` (C 589-821) and `struct font_driver_list` (C 824-837) moved into this
  mirror from `femacs/src/font.rs`**, which carried them at the use site; the unit's copies are
  retired (PORTING.md P-26, the 3.181/3.211/3.220 ownership rule), and `femacs/src/alloc.rs`'s
  own `font_driver` stand-in was retired for the same reason (`build-tu src/alloc.c` was the
  check that found it).  Layout, measured with the probe this ledger records: `struct
  font_driver` sizeof 208, align 8, `type`@0 `get_cache`@16 `list`@24 `match`@32
  `close_font`@64 `text_extents`@104 `otf_capability`@120 `shape`@152 `check`@160
  `cached_font_ok`@184 `combining_capability`@192; `struct font_driver_list` sizeof 24, align 8,
  on@0 driver@8 next@16.  The `HAVE_WINDOW_SYSTEM` members (`draw`, `get_bitmap`, `free_bitmap`,
  C 682-698) are absent from the frozen layout and carried under
  `#[cfg (feature = "have_window_system")]` (P-51, the status 3.220 shape).

- **Why `font_driver_list` could not move before `font_driver`** - the "unit copy vs mirror copy"
  ruling again (status 3.220's `face`/`face_cache` case).  With the record in the mirror and its
  `driver` member spelled `*const c_void` (P-72 (3)), the *unit*'s own code stopped compiling:

      error[E0609]: no field `filter_properties` on type `std::ffi::c_void`
          if (*((*driver_list).driver)).filter_properties.is_some ()
      error[E0609]: no field `r#type` on type `std::ffi::c_void`
          && (NILP (ftype) || EQ ((*((*driver_list).driver)).r#type, ftype))

  So the mirror needed the **typed** `font_driver` first, and this round provides it: the
  `driver` member is `*const font_driver` now, exactly as C writes it.

- **Two members keep P-72 (3)'s untyped pointer, and for a measured reason.**  `prepare_face`
  and `done_face` take `struct face *`, and `struct face` lives in `include/dispextern.rs`
  under `#[cfg (feature = "dispextern_face_records")]` (status 3.220), so the *layout probe*
  (which compiles font.rs without that feature) has no `face` type and every unit that includes
  font.rs without the dispextern arm failed:

      error[E0425]: cannot find type `face` in this scope
          pub prepare_face: Option<unsafe extern "C" fn (*mut frame, *mut face)>,
      note: found an item that was configured out

  They are therefore `*mut c_void` with their `/// C:` prototype lines (the `buffer`/
  `face_cache.f` precedent in this file), and the unit's two call sites pass the pointer with a
  cast (`p (f, face as *mut c_void)`), which is C's own pointer with the port's spelling.

- **State after this round**: the ledger's 55 boxes are all ticked, `--check` is `OK` with **no
  deferred line** (the six records the deferral was hiding - `font_spec`, `font_entity`,
  `font_metrics`, `font_bitmap`, `font_driver`, `font_driver_list` - are all in the mirror now),
  and the five units that include this header build (`src/font.c`, `src/keyboard.c`,
  `src/alloc.c`, `src/xfaces.c`, `src/dispnew.c`).

# ----------------------------------------------------------------------
## Decision and Evidence (`struct font_entity`'s size assertion, 2026-10-05)

- **The size assertion was frozen-only while the member it guards is not.**  font.h 264-268 adds
  `bool is_android` under `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`, which this
  mirror has carried as `#[cfg (feature = "have_android")] pub is_android: bool` since the type
  landed; the `const _` below it kept asserting the frozen measurement (120) in *every*
  configuration.  The arm-flip compile is the first compile that turns the feature on, and it
  stopped there: `error[E0080]: evaluation panicked: assertion failed:
  std::mem::size_of::<font_entity>() == 120`.
- The other three assertions (align 8, `header` 0, `props` 8) hold in both arms - the member is
  appended - so only the size assertion carries `#[cfg (not (feature = "have_android"))]`.  The
  Android arm is **not** asserted: there is no Android configuration in this tree to measure it
  with, and an unmeasured number is not written (the `glyph_string` / `MonitorInfo` rule,
  P-51/P-82).  Recorded here rather than silently: the arm-flip compile is the only measurement
  this arm has.

## Evidence (`font_entity`)

- `rustc --cfg 'feature="have_android"'` on
  `build/femacs/units/androidmenu/_root.rs` -> before the gate: `E0080` (above), the only
  remaining error after the name-resolution blockers were fixed; after: **exit 0, 0 `^error`
  lines**.
- `python3 tools/extract-abi.py --check emacs/src/font.h femacs/src/include/font.rs` -> `OK
  emacs/src/font.h -> femacs/src/include/font.rs`.
- `python3 tools/check-citations.py` -> `0 drift`, `0 stale` (the allowlist row at
  `femacs/src/include/font.rs:5` is above the insertion and did not move).
- Frozen: this `const _` is still active in the frozen configuration (the gate is
  `not have_android`); 30 units in `femacs/src/` include this mirror, and the frozen units
  re-read for this change (`src/keyboard.c` among them) are in that set - `build-tu` ->
  `built …`.

### status 3.496 (the mirror round this file took part in)

- The edge/declaration changes are recorded in `docs/status.md` 3.496: `include/dispextern.rs`'s
  `include!("xlib.rs")` edge lost its `#[cfg]` (the `xtask` dedupe gives the first edge the slot),
  `include/xlib.rs` widened `XRectangle`/`Window`/`XrmDatabase`/`XID` to
  `any (have_x_windows, have_pgtk)`, `include/pgtkterm.rs` fixed a dangling `///` (a syntax error
  that made every `have_pgtk` TU unparsable), its `pgtk_im_set_cursor_location` prototype and its
  two bare `atimer` tags, `include/font.rs` gained the `harfbuzz` arm of the `font_driver` size
  assertion (clang-measured 208/224), `include/xterm.rs`'s three INLINE bodies became `unsafe fn`,
  `include/gtk.rs` gained `GtkWidgetClass.get_preferred_height`, and `include/emacsgtkfixed.rs`
  moved `emacs_fixed_new` into a `_decls` module.  The two arm flips that were blocked now read
  **exit 0**: `src/pgtkim.rs` (`have_pgtk` + `have_gtk3`, 14,083,208-byte rlib) and
  `src/hbfont.rs` (`harfbuzz`, 14,580,952-byte rlib); `layout-check` passes and
  `tools/check-all-units.sh` is **83/0 + 83/0**.

## 2026-10-05: the two records' window-system arms are read per arm (3.506)

`struct font` and `struct font_driver` already carried C 323-395's and C 676-698's
`HAVE_WINDOW_SYSTEM` members behind `have_window_system`; what was frozen-only was the
*assertions*: the arm read `8 x E0080` on them (`size_of::<font>() == 184`,
`offset_of!(font, driver) == 176`, `size_of::<font_driver>() == 208` and five member offsets),
which is every window-system arm's reading (`src/fringe.rs`'s flip, status 3.501/3.505).

The numbers were not in the tree, so they were measured with the tree's own producer:
`spikes/gated-layout-probe.py` gained `--prelude HEADER` (C's own include order - `font.h` uses
`Lisp_Object` and `union vectorlike_header` from `lisp.h`, so `struct font` is unreadable without
it: the header alone reports 20 errors in every arm), and the two readings are pinned as
`docs/baseline/font-layout.tsv` and `docs/baseline/font_driver-layout.tsv`:

* `struct font`: `frozen` 184/8 with `driver`@176; `x11`/`ns_cocoa`/`x11_ext` 216/8 with
  `driver`@208, the eight members at 176/180/184/188/192/196/200/204.
* `struct font_driver`: `frozen` 208/8 with `anchor_point`@112; the three arms 232/8 with
  `anchor_point`@136, the three members at 112/120/128.  The `harfbuzz` members stay last
  (208 -> 224), so the two shifts compose.

The assertions now add `FONT_WS_SHIFT` (0/32) and `FONT_DRIVER_WS_SHIFT` (0/24), with the
arm-only members asserted at their measured offsets - the `include/dispextern.rs`
`FACE_WS_SHIFT` shape of status 3.505.  Same flip, in order: `38` errors when `src/fringe.rs`
landed, `24` after 3.505's two `dispextern.rs` fixes, `13` after its `estimate_mode_line_height`
decl, and `5` after this one - the five are the four deferred X11 typedefs of
`include/dispextern.rs` (next item, `docs/ports/include-dispextern.md`) and one `E0080` in
`include/systime.rs` whose faithful form is `xtask dead-arms` (it asserts that the feature and
the `config.rs` constant move together), not a mirror defect.
