# emacs/src/font.c (6091 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 22
- [x] fontp  (Ffontp, 1/2, doc 3ca7d704d1c7, line 3999)
- [x] font-spec  (Ffont_spec, 0/MANY, doc ec9414fe6ee1, line 4018)
- [x] font-get  (Ffont_get, 2/2, doc 2f2420308ef5, line 4210)
- [x] font-face-attributes  (Ffont_face_attributes, 1/2, doc 781f52276506, line 4278)
- [x] font-put  (Ffont_put, 3/3, doc f5bb85470fd8, line 4358)
- [x] list-fonts  (Flist_fonts, 1/4, doc c7b7181d9a4d, line 4394)
- [x] font-family-list  (Ffont_family_list, 0/1, doc fc76e80a3890, line 4442)
- [x] find-font  (Ffind_font, 1/2, doc 08473dc49f70, line 4466)
- [x] font-xlfd-name  (Ffont_xlfd_name, 1/3, doc 22f4c3c2da56, line 4478)
- [x] clear-font-cache  (Fclear_font_cache, 0/0, doc 6929952d57a5, line 4576)
- [x] font-shape-gstring  (Ffont_shape_gstring, 2/2, doc ce276052c985, line 4604)
- [x] font-variation-glyphs  (Ffont_variation_glyphs, 2/2, doc 799c79f62d6a, line 4692)
- [x] internal-char-font  (Finternal_char_font, 1/2, doc 50db753621ec, line 4762)
- [x] open-font  (Fopen_font, 1/3, doc e8058088c839, line 5242)
- [x] close-font  (Fclose_font, 1/2, doc eacdc2ccd552, line 5268)
- [x] query-font  (Fquery_font, 1/1, doc da7e485f08b5, line 5277)
- [x] font-has-char-p  (Ffont_has_char_p, 2/3, doc 273305e008e9, line 5335)
- [x] font-get-glyphs  (Ffont_get_glyphs, 3/4, doc 017215eb2f31, line 5362)
- [x] font-match-p  (Ffont_match_p, 2/2, doc 827f3d95d34e, line 5492)
- [x] font-at  (Ffont_at, 1/3, doc 7a94a72d9a67, line 5503)
- [x] frame-font-cache  (Fframe_font_cache, 0/1, doc b5adaf653cce, line 5575)
- [x] font-info  (Ffont_info, 1/2, doc bb3a24ac8a83, line 5594)

## Functions defined — 82
- [x] valid_font_driver  (line 140)
- [x] font_make_spec  (line 160, static)
- [x] font_make_entity  (line 172)
- [x] font_make_entity_android  (line 191)
- [x] font_make_object  (line 213)
- [x] font_build_object  (line 250)
- [x] font_intern_prop  (line 296)
- [x] font_pixel_size  (line 339, static)
- [x] font_style_to_value  (line 376)
- [x] font_style_symbolic  (line 471)
- [x] find_font_encoding  (line 497)
- [x] font_registry_charsets  (line 520)
- [x] font_prop_validate_symbol  (line 586, static)
- [x] font_prop_validate_style  (line 599, static)
- [x] font_prop_validate_non_neg  (line 638, static)
- [x] font_prop_validate_spacing  (line 645, static)
- [x] font_prop_validate_otf  (line 666, static)
- [x] get_font_prop_index  (line 733, static)
- [x] font_prop_validate  (line 748, static)
- [x] font_put_extra  (line 775)
- [x] parse_matrix  (line 865, static)
- [x] font_expand_wildcards  (line 887, static)
- [x] font_parse_xlfd_1  (line 1087, static)
- [x] font_parse_xlfd  (line 1283)
- [x] font_dynamic_unparse_xlfd  (line 1309)
- [x] font_unparse_xlfd  (line 1470)
- [x] font_parse_fcname  (line 1624, static)
- [x] font_unparse_fcname  (line 1882, static)
- [x] font_parse_name  (line 2009, static)
- [x] font_parse_family_registry  (line 2022)
- [x] font_rescale_ratio  (line 2077, static)
- [x] font_score  (line 2128, static)
- [x] font_vconcat_entity_vectors  (line 2177, static)
- [x] font_compare  (line 2204, static)
- [x] font_sort_entities  (line 2232, static)
- [x] font_update_sort_order  (line 2338)
- [x] font_check_otf_features  (line 2358, static)
- [x] font_check_otf  (line 2399, static)
- [x] font_match_p  (line 2433)
- [x] font_prepare_cache  (line 2557, static)
- [x] font_finish_cache  (line 2579, static)
- [x] font_get_cache  (line 2600, static)
- [x] font_clear_cache  (line 2615, static)
- [x] font_is_ignored  (line 2671)
- [x] font_delete_unmatched  (line 2695, static)
- [x] font_list_entities  (line 2801)
- [x] font_matching_entity  (line 2885, static)
- [x] font_open_entity  (line 2935, static)
- [x] font_close_object  (line 3023, static)
- [x] font_has_char  (line 3043)
- [x] font_encode_char  (line 3078, static)
- [x] font_get_name  (line 3091)
- [x] font_spec_from_name  (line 3102)
- [x] font_clear_prop  (line 3116)
- [x] font_select_entity  (line 3171, static)
- [x] font_find_for_lface  (line 3219)
- [x] font_open_for_lface  (line 3384)
- [x] font_load_for_lface  (line 3436)
- [x] font_prepare_for_face  (line 3497)
- [x] font_done_for_face  (line 3507)
- [x] font_open_by_spec  (line 3518)
- [x] font_open_by_name  (line 3542)
- [x] register_font_driver  (line 3567)
- [x] free_font_driver_list  (line 3597)
- [x] font_update_drivers  (line 3622)
- [x] fset_font_data  (line 3733, static)
- [x] font_put_frame_data  (line 3739)
- [x] font_get_frame_data  (line 3756)
- [x] font_filter_properties  (line 3771)
- [x] font_at  (line 3823, static)
- [x] codepoint_is_emoji_eligible  (line 3899, static)
- [x] font_range  (line 3926)
- [x] copy_font_spec  (line 4149)
- [x] merge_font_spec  (line 4185)
- [x] clear_font_cache  (line 4554)
- [x] font_fill_lglyph_metrics  (line 4590)
- [x] build_style_table  (line 5706, static)
- [x] font_add_log  (line 5733)
- [x] font_deferred_log  (line 5822)
- [x] font_drop_xrender_surfaces  (line 5832)
- [x] syms_of_font  (line 5842)
- [x] init_font  (line 6088)

## Macros defined — 10
- [x] BUILD_STYLE_TABLE  (line 5703, body ac648d828b95)
- [x] DEFAULT_ENCODING  (line 47, body 61e210340254)
- [x] INTERN_FIELD  (line 1121, body fc08d843ad77)
- [x] INTERN_FIELD_SYM  (line 1122, body 54f2c4ff1add)
- [x] PROP_MATCH  (line 1822, body 9b3c080b5053)
- [x] PROP_SAVE  (line 1825, body 2a3d51e02623)
- [x] XLFD_LARGENUM_MASK  (line 908, body 2050036f716c)
- [x] XLFD_NULL_MASK  (line 907, body 8f814a983c97)
- [x] XLFD_REGENC_MASK  (line 910, body 864ea6d01043)
- [x] XLFD_SYMBOL_MASK  (line 905, body be983e9959d7)

## Types and their fields — 3
- [x] <anon@893>: int from, int to, int mask
- [x] font_sort_data: unsigned score, int font_driver_preference, Lisp_Object entity
- [x] table_entry: int numeric, const char* names

## File-scope objects — 8
- [x] static Lisp_Object font_style_table  (line 50, static)
- [x] static Lisp_Object font_charset_alist  (line 128, static)
- [x] static struct font_driver_list font_driver_list  (line 133, static)
- [x] static int num_font_drivers  (line 287, static)
- [x] static int sort_shift_bits  (line 2121, static)
- [x] static Lisp_Object scratch_font_spec  (line 2686, static)
- [x] static Lisp_Object scratch_font_prefer  (line 2686, static)
- [x] static Lisp_Object Vfont_log_deferred  (line 5726, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/font.c` @ `fd7afced403`; port `src/font.rs` (6,934 lines vs C's 6,091 -
+13.8% on the **total** basis, declared in `docs/ports/line-drift-tolerance.tsv` with the
reason: the use-site declaration block that stands in for the un-mirrored `font.h`, the
`#![allow]`/include block, the carried `#[cfg (feature = …)]` arms, and the labelled blocks C
writes as `goto`; the 22 `DEFUN!` docstrings are the one place the port saves text).

- **Subrs 22/22, functions 82/82, macros 10/10, types 3/3, file-scope objects 8/8** — all
  boxes above are ticked, and `python3 tools/extract-abi.py --check emacs/src/font.c
  src/font.rs` prints `OK`.

- **`FONT_DEBUG`: the arm the macro table calls dead is the *live* text, and its eight
  registrations cannot be transcribed.**  `src/font.c` is **not** in
  `docs/baseline/config/cond-eval-verified.txt` (`tools/check-cond-eval.py src/font.c` =
  `45 conditions, 0 unknown, 2 disagreements [NOT VERIFIED]`), so P-51/status 3.49 applies:
  every `#if`'s other arm is carried as text.  The two disagreements are `#ifdef FONT_DEBUG`
  (C 5240, C 5938), and they are a §3.35/§3.51-class table gap, not a dead arm: `src/font.h`
  997-998 is
  ```c
  #ifndef FONT_DEBUG
  #define FONT_DEBUG
  #endif
  ```
  so clang compiles the block.  Measured with a probe (`#include <config.h>`, `lisp.h`,
  `font.h`, then `#ifdef FONT_DEBUG` picks the live branch; the command is in the port's
  file header).  **The port therefore keeps C 5240-5589 as ordinary text** — the seven subrs
  below `Fopen_font` and `Fframe_font_cache` are compiled by the reference build and the
  reference `font.o` will export `_Fopen_font`, `_Fclose_font`, `_Fquery_font`,
  `_Ffont_has_char_p`, `_Ffont_get_glyphs`, `_Ffont_match_p`, `_Ffont_at` and
  `_Fframe_font_cache`, which is what the port's object does too.  The one deviation is C
  5938-5949's `defsubr` block: `tools/extract-abi.py` decides `defsubr` liveness with the
  *frozen macro table* (`c_defsubr_live`, deliberately not under `blind_conditions`), so it
  reads `FONT_DEBUG` as undefined and would report each of the eight as `unexpected defsubr
  (&Sopen_font)`; they are recorded here and reported to the integrator (see the list at the
  end).  `Fdraw_string` itself (C 5534-5589) is inside that block's own `#if 0` and is only
  recorded.  The port's `syms_of_font` registers the twelve defsubrs the table calls live
  (C 5918-5932): `Sfontp`, `Sfont_spec`, `Sfont_get`, `Sfont_put`, `Slist_fonts`,
  `Sfont_family_list`, `Sfind_font`, `Sfont_xlfd_name`, `Sclear_font_cache`,
  `Sfont_shape_gstring`, `Sfont_variation_glyphs`, `Sinternal_char_font`.

- **The carried arms** (P-51: the clause the frozen configuration does not compile, each
  under a `#[cfg (feature = "…")]` naming the condition's non-frozen side; none of these
  features is ever set, so the compiled build is what clang compiles):
  C 43-44 `#include TERM_HEADER` (HAVE_WINDOW_SYSTEM) and every `#ifdef HAVE_WINDOW_SYSTEM`
  block — C 341-362 `font_pixel_size`, C 2987-3013 and 3032-3034 `font_open_entity`/
  `font_close_object`, C 3572-3575 `register_font_driver`, C 3892-3993
  `codepoint_is_emoji_eligible`/`font_range`, C 4276-4355 `Ffont_face_attributes`, C
  5580-5588 `Fframe_font_cache`, C 5592-5699 `Ffont_info`, C 6053-6083 the `syms_of_*font`
  chain — all under `feature = "have_window_system"` (and its sub-features `freetype`,
  `have_x_windows`, `use_cairo`, `xft`, `bdffont`, `have_ntgui`, `be_cairo` for the chain); C 135-154
  `valid_font_driver` under the bare `#[cfg (enable_checking)]` pass (the bidi.rs/
  regex-emacs.rs key, status 3.50); C 181-182, 188-206 (`HAVE_ANDROID`, `ANDROID_STUBIFY`)
  and `font_entity.is_android` under `feature = "have_android"`; C 242-277 `font_build_object` and
  `font_unparse_fcname` (C 1882-2001) under `feature = "xft_or_freetype_or_ns"`; C 1647-1658
  and 2034-2041 under `feature = "w32"`; C 3414-3421 under `feature = "have_ns"`; C 3730-3762
  `fset_font_data`/`font_put_frame_data`/`font_get_frame_data` under
  `feature = "xft_or_freetype"`; C 6031-6034 (`inhibit_compacting_font_caches = 1`) under
  `feature = "windowsnt"`; the `font_driver` members C guards with `HAVE_WINDOW_SYSTEM`
  (`draw`/`get_bitmap`/`free_bitmap`) and `HAVE_HARFBUZZ` (`begin_hb_font`/`end_hb_font`),
  and `font_entity.is_android`/`font`'s eight `HAVE_WINDOW_SYSTEM` metrics, under the same
  two features.  `#if 0` regions are recorded here and **not** transcribed (status 3.55's
  `ccl.rs` rule): C 4838-5236 (the experimental LGSTRING/OTF helpers: `check_gstring`,
  `generate_otf_features`, `check_otf_features`, `font_otf_*`, `otf_open`/`otf_tag_symbol`/
  `otf_drive`, `Ffont_drive_otf`/`Ffont_otf_alternates` and their registrations), C 5534-5589
  (`Fdraw_string`/`Sdraw_string`), C 5692, C 5911-5915 (the `otf_list` staticpro), 5933-5936
  and 5946-5948 (the `#if 0` defsubrs), plus C 4926's `HAVE_LIBOTF` block.

- **Use-site declarations (the mirror gap this unit reports and does not touch).**
  `emacs/src/font.h` has only a WIP mirror (`src/include/font.rs`: `enum
  font_property_index` as constants and the four predicates), so this unit declares the rest
  of the header itself, each with C's own text and a `/// C: font.h <lines>` line
  (PORTING.md 0.1-2/P-72 rule 3); every record's layout is measured with the frozen clang and
  pinned with `const _` assertions (PORTING.md 1.2/P-73).  What is declared here and should
  move into `include/font.rs` when the mirror lands: `enum font_spacing`, `struct font_spec`,
  `struct font_entity`, `struct font`, `struct font_driver`, `struct font_driver_list`,
  `struct font_metrics`, `struct font_bitmap`, the macros `FONT_ENCODING_NOT_DECIDED`,
  `FONT_INVALID_CODE`, `FONT_PIXEL_SIZE_QUANTUM`, `PT_PER_INCH`, `POINT_TO_PIXEL`,
  `PIXEL_TO_POINT`, the six `FONT_*_NUMERIC`/`_SYMBOLIC`/`_FOR_FACE` style macros,
  `FONT_WEIGHT/SLANT/WIDTH_NAME_NUMERIC`, `FONT_SET_STYLE`, `FONT_WIDTH`/`HEIGHT`/`BASE`/
  `DESCENT`, `FONT_ADD_LOG`/`FONT_DEFERRED_LOG`, and the accessors `XSETFONT`, `XFONT_SPEC`/
  `XFONT_ENTITY`/`XFONT_OBJECT` and their `GC_` twins, `CHECK_FONT*`,
  `GC_FONT_{SPEC,ENTITY,OBJECT}_P`, `CHECK_FONT_GET_OBJECT`,
  `font_data_structures_may_be_ill_formed` and `valid_font_driver`.  The same rule owes
  `dispextern.h` (`struct face`, `struct face_cache`, the `FACE_*` bit masks, and the five
  declarations `FRAME_FACE_CACHE`/`FACE_FROM_ID`/`FACE_FOR_CHAR`/`face_at_buffer_position`/
  `face_at_string_position` together with `lookup_basic_face`/`recompute_basic_faces` and
  `xstrcasecmp`), `frame.h` (`CHECK_FRAME`, `decode_live_frame`,
  `decode_window_system_frame`), `window.h` (`decode_live_window`), `termhooks.h`
  (`terminal_glyph_code`), `composite.h` (`composition_gstring_p`,
  `composition_gstring_put_cache`, `composition_gstring_adjust_zero_width`),
  `lisp.h`/`character.h` (`larger_vector` — the P-26/3.57 gap — `nconc2`,
  `make_specified_string`, `parse_str_as_multibyte`, `intern_driver`, `assoc_no_quit`,
  `get_font_ascent_descent`), `fontset.h` (no mirror at all: `fs_query_fontset`,
  `fontset_ascii`) and `<stdlib.h>`/`<string.h>`/`<stdio.h>`/`<inttypes.h>`
  (`atoi`, `qsort`, `strstr`, `snprintf`, `PRIdMAX`).  `struct glyph_string` is a pointer-only
  type (P-70's placeholder).  Two consequences are recorded here: `frame.face_cache` and
  `frame.font_driver_list` are `*mut c_void` in the `frame.rs` mirror, so
  `FRAME_FACE_CACHE`/the driver-list walks cast at the use site (P-72 rule 3's explicit
  conversion); and `struct table_entry` needs `unsafe impl Sync` for the three `static`
  tables (Rust requires `Sync` of an immutable static's type; C's tables are immutable data
  pointing at string literals).

- **Function-pointer members of `struct font_driver`.**  C's struct marks some members
  "Optional" and its code checks those for NULL (`if (…->prepare_face)`, `if
  (…->otf_capability)`, `if (…->combining_capability)`, `if (…->free_entity)`, `if
  (…->drop_xrender_surfaces)`, `if (…->end_for_frame)`, `if (…->cached_font_ok == NULL || …)`,
  `if (…->get_variation_glyphs)`, `if (…->shape)`, `if (…->list_family)`,
  `if (…->has_char)`), and the port's field type is `Option<unsafe extern "C" fn …>` so the
  null checks are `is_none ()`/`is_some ()` and the calls go through `if let Some (f) = …`.
  Where C calls a member *unconditionally* (`get_cache`, `list`, `match`, `open_font`,
  `close_font`, `encode_char`, `text_extents`, `otf_capability` inside `Fquery_font`/`Ffont_info`),
  the port unwraps the same node C dereferences; see "What no gate can see" (c).

- **The gates' tail.**  `docs/ports/macro-body-notes.tsv` gained seventeen rows for this
  file (`INTERN_FIELD`, `PROP_SAVE`, `font_style_to_value`, `font_registry_charsets`,
  `font_prop_validate_spacing`, `font_dynamic_unparse_xlfd`, `font_unparse_xlfd`,
  `font_parse_fcname`, `font_parse_name`, `font_check_otf_features`, `font_check_otf`,
  `register_font_driver`, `build_style_table`, `syms_of_font`, `Ffont_xlfd_name`,
  `Ffont_shape_gstring`), each with its reason: C's `switch`/`case`/`break`/`goto`/label
  tokens become a `match`/labelled block, C's `0`/`1` are the port's `false`/`true` (the gate
  compares tokens, not values), `[0]` indexing becomes a dereference, `alloca` is
  `sa_stack_alloc` (P-05), `PRIdMAX`/`strlen` are concatenated or replaced by `&CStr`
  helpers, and the `#ifdef FONT_DEBUG` registrations the table does not see.  A side effect
  is recorded rather than hidden: a note exempts the token everywhere in that body, so a
  *dropped* `0`/`break`/`goto` inside those bodies is invisible to the token check — the
  bodies themselves are still compared token by token.  `docs/baseline/undef-pending.txt`
  gained nine entries this unit owes to units that are not ported yet (`_assoc_no_quit`
  (fns.c), `_face_at_buffer_position`/`_face_at_string_position`/`_lookup_basic_face` and
  `_Vface_alternative_font_family_alist`/`_Vface_font_rescale_alist`/`_Vface_ignored_fonts`
  (xfaces.c), `_recompute_basic_faces` (dispnew.c), `_Vdisplay_pixels_per_inch`
  (frame.c/xfaces.c)) and `_update_face_from_frame_parameter`; `tools/check-undef.py` then
  reports `182 pending, 0 new`.

- **The line count relation.**  C 6,091 total / 4,662 live (`python3 tools/extract-abi.py`
  prints the total; the live figure is the four-`#if`-free prefix plus the non-platform
  text).  The port is 6,934 lines: +13.8%, declared in
  `docs/ports/line-drift-tolerance.tsv` (row `font`, tolerance 0.15, `total` basis).  The
  per-function token comparison is the real guard and it is `OK`.

- **C's forward declarations are dropped** (PORTING.md 0.1-12): the `factories`'
  prototypes, `font_unparse_fcname`'s (C 242), `font_clear_cache`'s (C 2546-2547), and the
  `parse_matrix`/`font_expand_wildcards`/`font_parse_name` trio (C 787-789); their
  definitions follow.

### Call-relationship differences (D7)

The two sets of "identifier followed by `(`" (C's `ident (`, the port's `ident (` or
`ident! (`) differ by 59 on the port's side and 50 on C's; every one is a spelling of the
same callee or text this file records rather than transcribes:

* **C's `Q*` are compile-time constants, the port's are inline functions** (P-17): `Qnil`,
  `Qt`, `Qframe`, `Qfont`, `Qfont_spec`, `Qfont_entity`, `Qfont_object`, `Qfontsize`,
  `Qerror`, `Qoverflow_error`, `Qunbound`, `Qnormal`, `Qarrayp`, `Qopentype`, `Qemoji`,
  `Qc`/`Qm`/`Qp`/`Qd`, `Qiso8859_1`, `Qascii_0`, `Qfont_extra_type`,
  `Qfont_driver_superseded_by` and the thirteen `QC*` keys — C writes them bare, the port
  calls them (PORTING.md P-17).
* **C's object-like macros used as values** are the port's macro calls: `BEGV`/`ZV`
  (`BEGV! ()`/`ZV! ()`), `DEFAULT_ENCODING`, `USE_SAFE_ALLOCA` (C writes it without
  parentheses, so the C set never had it), and — because this unit transcribes parts of
  `font.h` — `XSETFONT`'s expansion `XSETPSEUDOVECTOR` and the `XUNTAG` inside
  `XFONT_SPEC`/`XFONT_ENTITY`/`XFONT_OBJECT`.
* **The port's own machinery**: `include!` (C's `#include`), `offset_of!`/`assert!` (the
  `const _` layout pins), `CHECK_TYPE` + `FRAMEP` (the body of the header's `CHECK_FRAME`),
  `GC_FONT_*_P`/`GC_XFONT_*`/`XFONT_ENTITY`/`font_data_structures_may_be_ill_formed`
  (transcribed from `font.h`, not all used by C's text), `sa_stack_alloc` (C's `alloca`,
  P-05), and `face_for_char` (inside the carried `FACE_FOR_CHAR` arm).
* **C-side-only names that are the same callee spelled differently**: every
  `struct font_driver` method (`get_cache`, `list`, `list_family`, `match`, `open_font`,
  `close_font`, `encode_char`, `text_extents`, `has_char`, `shape`, `otf_capability`,
  `otf_drive`, `free_entity`, `prepare_face`, `done_face`, `start_for_frame`,
  `end_for_frame`, `cached_font_ok`, `combining_capability`, `drop_xrender_surfaces`,
  `get_variation_glyphs`) is a `driver->method (…)` field call in C and
  `(*driver).method.unwrap () (…)` in the port (`unwrap` is the callee the regex sees);
  `memcpy` → `copy_nonoverlapping` and `ckd_mul` → `overflowing_mul` (the
  token-aliases.tsv families, P-15); `alloca` → `sa_stack_alloc` (P-05); `max` (C's array
  size) → the measured `313`.
* **Text this file records and does not transcribe**: `LGLYPH_CHAR`, `LGLYPH_CODE`,
  `LGSTRING_SLOT`, `check_gstring`, `check_otf_features`, `generate_otf_features`,
  `font_otf_Anchor`/`DeviceTable`/`ValueRecord`/`capability`, `otf_open`, `otf_tag_symbol`,
  `otf_drive`, `OTF_get_features`/`OTF_open`/`OTF_tag_name`, `esprintf`, `CHECK_CONS`,
  `Faref`, `Fcar`, `Fassoc`, `Ffont_make_gstring`, `make_unibyte_string`, `SYMBOL_INDEX`,
  `INT_BUFSIZE_BOUND`, `draw_text`, `Lisp_Object` (a C cast) — all inside the `#if 0`
  regions, the `HAVE_LIBOTF` arm, or `Fdraw_string`.

### What no gate can see

(a) **The eight `defsubr` registrations of C 5938-5949 cannot be transcribed** (see above):
the port's `Sopen_font`…`Sframe_font_cache` records exist, but Lisp cannot reach them until
`src/font.h`'s `FONT_DEBUG` row joins
`docs/baseline/config/source-header-macros.tsv` and the macro table is regenerated; the
ledger records the exact C lines and the integrator's to-do list below has the fix.  (b) The
`FONT_DEBUG` arm is live text, so the *compiled* behaviour of the port matches the reference
here, but no gate checks a `#ifdef`'s liveness against clang: `--check` only demands that
both arms' tokens exist, which is why a wrong verdict can pass.  (c) Every unconditional
driver-method call goes through `Option::unwrap`; if a driver left one of those members NULL,
C would call a NULL pointer and the port would abort — the "node C dereferences" reading is
a human argument, not a gate's.  (d) `FRAME_FACE_CACHE`/`font_driver_list` walks cast
`*mut c_void` to the record type at the use site; the cast is what makes the field readable
and only the `frame.rs` mirror's own layout assertions cover the offsets.  (e) The
`unsafe impl Sync for table_entry` states an immutability C gets from `const` — no gate
compares it.  (f) `nm -u build/femacs/font.o` was read: every name resolves against the
reference objects, libSystem, or the ratchet; the nine names this unit owes to units that
are not ported yet are in `docs/baseline/undef-pending.txt` (written by
`python3 tools/check-undef.py --write-pending`), and no `#[link_name]` was needed for any
name this unit declares.  (g) The `default.value` of C's `font_add_log` string comparison
(`"false"`/`"False"`/…/`"off"`) is transcribed literally; a byte difference in those
literals is invisible to the token gate (literals are dropped) and only a behavioural test
would see it.

### Integrator to-do list (this unit's mirror gaps and the one blocked block)

1. **`emacs/src/font.h`'s mirror is a WIP stub**: the records/accessors/macros listed above
   belong in `src/include/font.rs`; this unit declares them at its use site until
   then (P-72 rule 3).  `docs/ports/font.md` is the record; `docs/ports/include-font.md`'s
   boxes stay open.
2. **`FONT_DEBUG` is a missing row in `docs/baseline/config/source-header-macros.tsv`**
   (`src/font.h<TAB>FONT_DEBUG`, measured defined with clang).  Adding it regenerates the
   table, makes `check-cond-eval.py src/font.c` report `0 disagreements` and font.c becomes a
   *verified* file — at which point C 5938-5949's eight `defsubr` calls must be transcribed
   in `syms_of_font` (the ledger above says which eight), and the `syms_of_font` notes row in
   `docs/ports/macro-body-notes.tsv` shrinks to `1,doc` alone.
3. `struct face`/`struct face_cache` and the five `dispextern.h`/`frame.h` macros this unit
   declares are `include/dispextern.rs`'s gap (it currently stops at the `LFACE_*` enum, and
   `frame.rs` carries `face_cache`/`font_driver_list` as `*mut c_void`).
4. `fontset.h` has no mirror at all (`fs_query_fontset`, `fontset_ascii`), and
   `lisp.rs` still lacks `larger_vector`/`make_specified_string` (status 3.57's list).
5. `docs/baseline/undef-pending.txt`: the nine entries this unit adds (see the gates' tail);
   they shrink when xfaces.c/frame.c/dispnew.c/fns.c land.

- **Integrator note (3.263): `snprintf` is declared by the mirror, not here.**  `lib/include/stdio.rs`
  gained the `<stdio.h>` entry so the `GLYPH_DEBUG` arms (which call it) compile - `xtask dead-arms`
  with the `glyph_debug` flip is what found it - and this unit's own declaration of the same name was
  what made the name ambiguous once the mirror carried it.  The declaration is gone; the mirror is
  the single declaration point (PORTING.md 0.1-3).
