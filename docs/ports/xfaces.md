# emacs/src/xfaces.c (7843 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 25
- [x] clear-face-cache  (Fclear_face_cache, 0/1, doc 6ffdead55fd2, line 794)
- [x] color-values-from-color-spec  (Fcolor_values_from_color_spec, 1/1, doc 7cdac59f272b, line 1029)
- [x] color-gray-p  (Fcolor_gray_p, 1/2, doc 0b4d4ecd3c08, line 1265)
- [x] color-supported-p  (Fcolor_supported_p, 1/3, doc 2aa2f0dc972f, line 1277)
- [x] internal-make-lisp-face  (Finternal_make_lisp_face, 1/2, doc 128b43811220, line 3029)
- [x] internal-lisp-face-p  (Finternal_lisp_face_p, 1/2, doc 6396ad41257e, line 3120)
- [x] internal-copy-lisp-face  (Finternal_copy_lisp_face, 4/4, doc e3a5b13fa098, line 3145)
- [x] internal-set-lisp-face-attribute  (Finternal_set_lisp_face_attribute, 3/4, doc 50bd6f101449, line 3217)
- [x] face-attribute-relative-p  (Fface_attribute_relative_p, 2/2, doc 280da801dd75, line 4193)
- [x] merge-face-attribute  (Fmerge_face_attribute, 3/3, doc e01f41ca2241, line 4214)
- [x] internal-get-lisp-face-attribute  (Finternal_get_lisp_face_attribute, 2/3, doc b88ee7ad79b3, line 4230)
- [x] internal-lisp-face-attribute-values  (Finternal_lisp_face_attribute_values, 1/1, doc 3347abab3a08, line 4295)
- [x] internal-merge-in-global-face  (Finternal_merge_in_global_face, 2/2, doc c3f24321cb8a, line 4316)
- [x] face-font  (Fface_font, 1/3, doc 160a7b4502cb, line 4423)
- [x] internal-lisp-face-equal-p  (Finternal_lisp_face_equal_p, 2/4, doc d4f6ff4bb1cb, line 4529)
- [x] internal-lisp-face-empty-p  (Finternal_lisp_face_empty_p, 1/2, doc 64d0bb36f580, line 4600)
- [x] frame--face-hash-table  (Fframe_face_hash_table, 0/1, doc 428bbd446485, line 4619)
- [x] color-distance  (Fcolor_distance, 2/4, doc f68a053ce468, line 4837)
- [x] face-attributes-as-vector  (Fface_attributes_as_vector, 1/1, doc 22718f8af056, line 5408)
- [x] display-supports-face-attributes-p  (Fdisplay_supports_face_attributes_p, 1/2, doc 78140bd0b17c, line 5764)
- [x] internal-set-font-selection-order  (Finternal_set_font_selection_order, 1/1, doc 8e916e6b5867, line 5850)
- [x] internal-set-alternative-font-family-alist  (Finternal_set_alternative_font_family_alist, 1/1, doc b84466119bd8, line 5911)
- [x] internal-set-alternative-font-registry-alist  (Finternal_set_alternative_font_registry_alist, 1/1, doc 3b5e875a1576, line 5940)
- [x] tty-suppress-bold-inverse-default-colors  (Ftty_suppress_bold_inverse_default_colors, 1/1, doc 2dc0a172ad85, line 6881)
- [x] x-load-color-file  (Fx_load_color_file, 1/1, doc db098be674c4, line 7307)

## Functions defined — 65
- [x] init_frame_faces  (line 671)
- [x] free_frame_faces  (line 696)
- [x] recompute_basic_faces  (line 731)
- [x] clear_face_cache  (line 755)
- [x] parse_hex_color_comp  (line 928, static)
- [x] parse_float_color_comp  (line 956, static)
- [x] parse_color_spec  (line 987)
- [x] parse_rgb_list  (line 1058, static)
- [x] tty_lookup_color  (line 1083, static)
- [x] tty_defined_color  (line 1144)
- [x] tty_color_name  (line 1180)
- [x] face_color_gray_p  (line 1215, static)
- [x] face_color_supported_p  (line 1242, static)
- [x] load_color2  (line 1294, static)
- [x] load_color  (line 1369, static)
- [x] push_named_merge_point  (line 1976, static)
- [x] resolve_face_name  (line 2014, static)
- [x] lface_from_face_name_no_resolve  (line 2060, static)
- [x] lface_from_face_name  (line 2086, static)
- [x] get_lface_attributes_no_remap  (line 2100, static)
- [x] get_lface_attributes  (line 2124, static)
- [x] lface_fully_specified_p  (line 2171, static)
- [x] merge_face_heights  (line 2253, static)
- [x] merge_face_vectors  (line 2298, static)
- [x] face_inherited_attr  (line 2395, static)
- [x] face_inheritance_cycle  (line 2447, static)
- [x] merge_named_face  (line 2504, static)
- [x] evaluate_face_filter  (line 2556, static)
- [x] filter_face_ref  (line 2621, static)
- [x] merge_face_ref  (line 2700, static)
- [x] update_face_from_frame_parameter  (line 3876)
- [x] face_attr_equal_p  (line 4483, static)
- [x] lface_equal_p  (line 4517, static)
- [x] hash_string_case_insensitive  (line 4633, static)
- [x] lface_hash  (line 4647, static)
- [x] make_realized_face  (line 4698, static)
- [x] free_realized_face  (line 4715, static)
- [x] color_distance  (line 4813, static)
- [x] make_face_cache  (line 4885, static)
- [x] free_realized_faces  (line 4934, static)
- [x] free_all_realized_faces  (line 4978)
- [x] free_face_cache  (line 4995, static)
- [x] cache_face  (line 5015, static)
- [x] uncache_face  (line 5085, static)
- [x] lookup_face  (line 5109, static)
- [x] lookup_named_face  (line 5190)
- [x] lookup_basic_face  (line 5228)
- [x] smaller_face  (line 5296)
- [x] face_with_height  (line 5355)
- [x] lookup_derived_face  (line 5384)
- [x] tty_supports_face_attributes_p  (line 5576, static)
- [x] realize_basic_faces  (line 5998, static)
- [x] realize_default_face  (line 6062, static)
- [x] realize_named_face  (line 6193, static)
- [x] realize_face  (line 6236, static)
- [x] realize_gui_face  (line 6331, static)
- [x] map_tty_color  (line 6674, static)
- [x] realize_tty_face  (line 6756, static)
- [x] compute_char_face  (line 6907)
- [x] face_at_buffer_position  (line 6955)
- [x] face_for_overlay_string  (line 7097)
- [x] face_at_string_position  (line 7169)
- [x] merge_faces  (line 7243)
- [x] init_xfaces  (line 7449)
- [x] syms_of_xfaces  (line 7503)

## Macros defined — 29
- [x] FACE_CACHE_BUCKETS_SIZE  (line 309, body ab68fc51497d)
- [x] HANDLE_INVALID_NIL_VALUE  (line 3208, body 8eddf4c21630)
- [x] IGNORE_DEFFACE_P  (line 300, body 441811e8384c)
- [x] LFACEP  (line 1819, body b8e2d32a651c)
- [x] LFACE_BACKGROUND  (line 1803, body d6d978fe50b0)
- [x] LFACE_BOX  (line 1808, body df54e8249b3c)
- [x] LFACE_DISTANT_FOREGROUND  (line 1813, body a31f8d521899)
- [x] LFACE_EXTEND  (line 1812, body c5df39c5fa1c)
- [x] LFACE_FAMILY  (line 1795, body 8559c85f2ac5)
- [x] LFACE_FONT  (line 1809, body 0bf18eee1d5f)
- [x] LFACE_FONTSET  (line 1811, body b03f7b11dc23)
- [x] LFACE_FOREGROUND  (line 1802, body 735d64f12c64)
- [x] LFACE_FOUNDRY  (line 1796, body e896f0726cad)
- [x] LFACE_HEIGHT  (line 1797, body f4dcf9e493cc)
- [x] LFACE_INHERIT  (line 1810, body aa5ed4be5a24)
- [x] LFACE_INVERSE  (line 1801, body 9f964f1667ba)
- [x] LFACE_OVERLINE  (line 1806, body 188371fde0c5)
- [x] LFACE_SLANT  (line 1799, body 63ba5355525b)
- [x] LFACE_STIPPLE  (line 1804, body 10248fe35b28)
- [x] LFACE_STRIKE_THROUGH  (line 1807, body afe642466ef9)
- [x] LFACE_SWIDTH  (line 1805, body 67a35c1ff83b)
- [x] LFACE_UNDERLINE  (line 1800, body 5c0ea35b885a)
- [x] LFACE_WEIGHT  (line 1798, body 290ec3bed639)
- [x] PARSE_RGB_LIST_FIELD  (line 1060, body a09b6f1b3fae)
- [x] RESET_P  (line 304, body 474cdaae555e)
- [x] TTY_SAME_COLOR_THRESHOLD  (line 5431, body 8a12a315082a)
- [x] UNSPECIFIEDP  (line 296, body 5721bc82edec)
- [x] check_lface  (line 1942, body 0facd856f519)
- [x] check_lface_attrs  (line 1941, body 0facd856f519)

## Types and their fields — 1
- [x] named_merge_point: Lisp_Object face_name, enum named_merge_point_kind named_merge_point_kind, struct named_merge_point* prev

## File-scope objects — 11
- [x] char unspecified_fg  (line 311)
- [x] Lisp_Object Vface_alternative_font_family_alist  (line 317)
- [x] Lisp_Object Vface_alternative_font_registry_alist  (line 323)
- [x] static int next_lface_id  (line 327, static)
- [x] static Lisp_Object lface_id_to_name  (line 331, static)
- [x] static ptrdiff_t lface_id_to_name_size  (line 332, static)
- [x] bool face_change  (line 349)
- [x] static bool tty_suppress_bold_inverse_default_colors_p  (line 356, static)
- [x] static bool menu_face_changed_default  (line 369, static)
- [x] static int font_sort_order  (line 1534, static)
- [x] static Lisp_Object face_attr_sym  (line 1826, static)

# ----------------------------------------------------------------------
## Decisions

The gates (`femacs/` is the working directory for the `xtask`/`rustc` commands):

    build-tu src/xfaces.c                     -> built src/xfaces.rs -> ../build/femacs/xfaces.o
    extract-abi.py --check                    -> OK emacs/src/xfaces.c -> femacs/src/xfaces.rs
    check-globals.py                          -> check-globals: 267 units, 0 DEFVAR_* variables without a definition in their own unit
    check-undef.py                            -> check-undef: 25 ratchet entries are stale (now satisfied): _Fcopy_hash_table
                                                 _Vface_alternative_font_family_alist _Vface_font_rescale_alist _Vface_ignored_fonts
                                                 _adjust_frame_glyphs _assoc_no_quit ...; the only NEW entry is callproc/process's
                                                 `_inherit_process_coding_system` (other lanes' objects), and no line names xfaces.o
    check-linkage.py                          -> check-linkage: 2 unit object(s) compared against the non-static objects of the C
                                                 sources the frozen build links, 0 with 0 missing symbol(s)
    check-symbols.py --dir src                -> check-symbols: 67 units compared, 0 mismatched, 0 still pending; 299 DEFVAR carrier
                                                 symbol(s) ...; 1567 Lisp-symbol/constant symbol(s) folded into Rust constants (P-17)
    layout-check                              -> layout-check passed (see build/femacs/layout_asserts.rs)
    rustc --cfg enable_checking (_root.rs)    -> exit 0
    check-declarations.py                     -> 8 declaration sites (0 forbidden, 8 allowed literals), 0 failures
    check-verbatim.py                         -> 6 trees, 4607 files, 0 failures
    check-cond-eval.py src/xfaces.c           -> src/xfaces.c: 94 conditions, 0 unknown, 0 disagreements [VERIFIED]
    ledger                                    -> 131 rows, 0 unchecked

`build/emacs-ref/src/xfaces.o` was built as well (`make -C build/emacs-ref/src xfaces.o`), which is
what makes `check-symbols.py --dir src` compare this unit's exports (67 units, 0 mismatched) and
what keeps `check-undef.py` from reporting this unit's three genuine references
(`_Fhash_table_count`, `_clear_current_matrices`, `_forget_escape_and_glyphless_faces`) as new: the
reference object references them too, so they are in the gate's domain (docs/status.md 3.72).

### The dead region is *recorded*, not transcribed (P-08/P-66, the `frame` row's precedent)

`src/xfaces.c` **is** in `docs/baseline/config/cond-eval-verified.txt`
(`python3 tools/check-cond-eval.py src/xfaces.c` -> `94 conditions, 0 unknown, 0 disagreements
[VERIFIED]`), so `tools/extract-abi.py` evaluates every `#if` against the frozen macro table and
compares only the text this target compiles.  The frozen configuration is the minimal macOS one
(`docs/baseline/config/darwin-arm64.macros.tsv`): `HAVE_WINDOW_SYSTEM`, `HAVE_X_WINDOWS`,
`HAVE_NTGUI`, `HAVE_NS`, `HAVE_PGTK`, `HAVE_HAIKU`, `HAVE_ANDROID`, `USE_MOTIF`, `USE_X_TOOLKIT`,
`MSDOS`, `GLYPH_DEBUG`, `DEBUG_X_COLORS`, `HAVE_XFT` and `HAVE_OTF_KANNADA_BUG` are all undefined;
`HAVE_PDUMPER` is defined (`init_xfaces`'s `#ifdef HAVE_PDUMPER` arm is live) and `O_BINARY` is 0
(so sysstdio.h's `FOPEN_TEXT` is `""`).

C compiles 5,489 of the file's 7,843 lines (`count("\n")`; 7,844 physical); **2,355 lines in 64 blocks are dead** and are
named here instead of being carried as `#[cfg]` text (the `comp.c`/`treesit.c`/`frame.c` shape):

- C 230-232, 235-236, 271-289: `USE_MOTIF`'s `<Xm/*>`, `MSDOS`'s `dosfns.h`, and `HAVE_X_WINDOWS`'s
  `<X11/Xos.h>` / `XOS_NEEDS_TIME_H` block.
- C 239-260: `HAVE_WINDOW_SYSTEM`'s `TERM_HEADER`/`fontset.h` and the five `GCGraphicsExposures`
  branches (`#ifdef HAVE_NTGUI`/`NS`/`PGTK`/`HAIKU`/`ANDROID`).
- C 334-343: `clear_font_table_count` and its two `#define`s.
- C 360-363, 1828-1938: `GLYPH_DEBUG` (`ncolors_allocated`/`npixmaps_allocated`/`ngcs`, the
  `check_lface_attrs` body with its twenty `eassert`s, and `check_lface`'s `XVECTOR` walk).  The
  live `#else` arm spells both macros as `(void) 0` and is carried.
- C 391-395, 637-665, 677-683, 705-720, 757-790, 810-914, 1377-1499, 1536-1785, 2184-2242,
  2936-2940, 3614-3622, 3641-3695, 3700-3713, 3787-3794, 3802-3834: the `HAVE_WINDOW_SYSTEM`
  blocks - `share_image_cache`, the `#ifdef HAVE_X_WINDOWS` GC helpers (`x_create_gc`, `x_free_gc`,
  `x_free_colors`, `free_face_colors`, `unload_color`, `color_count`/`register_color` and the
  `DEBUG_X_COLORS` machinery), the `HAVE_NTGUI`/`HAVE_NS`/`HAVE_PGTK`/`HAVE_ANDROID` GC shims,
  `load_face_colors`, the XLFD font-name family (`compare_fonts_by_sort_order`, `x-family-fonts`,
  `x-list-fonts`, `font_props_for_sorting`), `set_lface_from_font`, `set_lface_from_font`'s
  `load_face_colors` sibling in the `QCstipple`/`QCfont`/`QCfontset` arms, `set_font_frame_param`,
  the `Qscroll_bar`/`Qborder`/`Qcursor`/`Qmouse` frame-parameter arms, and
  `internal-face-x-get-resource` (`HAVE_X_WINDOWS && USE_X_TOOLKIT`).
- C 402-545: `HAVE_X_WINDOWS`'s `DEBUG_X_COLORS` reference counting, `x_free_colors`,
  `x_alloc_nearest_color`/`x_query_color`, `USE_X_TOOLKIT`'s `x_catch_errors`, and the
  `HAVE_NTGUI`/`HAVE_NS`/`HAVE_PGTK`/`HAVE_ANDROID` GC pair.
- C 1193-1197: `tty_color_name`'s `MSDOS` arm (`FRAME_MSDOS_P` / `msdos_stdcolor_name`).
- C 1250-1259: `face_color_supported_p`'s `HAVE_WINDOW_SYSTEM` arm.
- C 1348-1350: `load_color2`'s `GLYPH_DEBUG` counter.
- C 4083-4189: `internal-face-x-get-resource`'s body (`HAVE_X_WINDOWS && USE_X_TOOLKIT`).
- C 4384-4391: `gui_set_frame_parameters_1` inside `internal-merge-in-global-face`.
- C 4458-4467: `face-font`'s `HAVE_WINDOW_SYSTEM` arm.
- C 4659-4687: `lface_same_font_attributes_p`.
- C 4719-4747, 4750-4751, 4757-4807: `free_realized_face`'s GUI body (`free_face_fontset`,
  `font_done_for_face`, `x_free_gc`, `image_destroy_bitmap`) and `prepare_face_for_display`.
- C 4898-4925: `clear_face_gcs`.
- C 5054-5066, 5138-5139: `cache_face`'s and `lookup_face`'s `GLYPH_DEBUG` assertions.
- C 5145-5180: `face_for_font`.
- C 5298-5342, 5357-5369: `smaller_face`'s and `face_with_height`'s `HAVE_WINDOW_SYSTEM` bodies
  (`FRAME_TERMCAP_P`, `FONT_HEIGHT`, `PIXEL_TO_POINT`, the 1/2-pt walk).  Their live text is C's
  `#else`: `return face_id;`.
- C 5433-5556: `gui_supports_face_attributes_p`.
- C 5837-5839: `display-supports-face-attributes-p`'s `gui_supports_face_attributes_p` dispatch.
- C 5968-5984: `face_fontset`; C 6043-6045: `realize_basic_faces`' `USE_X_TOOLKIT` menu update;
  C 6077-6086: `realize_default_face`'s `set_lface_from_font` arm (`XSETFONT`, `FRAME_FONT`,
  `FRAME_FONTSET`, `fontset_name`).
- C 6169-6182: `realize_default_face`'s `#else` (`FRAME_X_P`, `gui_set_font`); the live text is
  `(void) face;`.
- C 6273-6320, 6334-6660: `realize_non_ascii_face`, `font_maybe_unset_attribute`, and
  `realize_gui_face`'s body down to C 6660.  `realize_gui_face` is *mostly* live: C 6331-6333
  (its head and `struct face *face = NULL;`) and C 6663's `return face;` are live text, so the port
  keeps the function with that pair and an empty middle - it returns NULL, exactly as the frozen
  build's does.
- C 6682-6684, 6708-6732: `map_tty_color`'s two `MSDOS` arms (`default_other_pixel`,
  `FRAME_MSDOS_P`).
- C 6770-6771: `realize_tty_face`'s `#if false` (`face->font_name`), dead in every configuration.
- C 7335-7336: `x-load-color-file`'s `HAVE_NTGUI` `RGB` arm.
- C 7359-7433: the `GLYPH_DEBUG` test block (`dump_realized_face`, `dump_face_lisp`, `dump-face`,
  `show-face-resources`).
- C 7633-7634, 7657-7659, 7664-7665, 7706-7708, 7712-7715, 7836-7840: `syms_of_xfaces`'s
  `HAVE_WINDOW_SYSTEM` `defsubr`s, the two `GLYPH_DEBUG` ones, `DUMP_COLORS`, the `HAVE_XFT` and
  `HAVE_OTF_KANNADA_BUG` arms of `face-ignored-fonts` (the live arm is `Qnil`), and the four
  `HAVE_WINDOW_SYSTEM` `defsubr`s at the tail.

### Use-site declarations (no mirror was asked to grow)

`dispextern.h`'s mirror (`include/dispextern.rs`) stops before its face block, `font.h`'s
(`include/font.rs`) is a WIP mirror that says "`struct font` lands with font.c", and the frame/
window/termhooks mirrors do not carry every accessor or writer this unit needs.  Everything below
is declared where C's translation unit gets it, with clang-measured layouts pinned by `const _`
(PORTING.md P-66/P-72; the `term.rs`/`font.rs` shape):

- `struct face` (`sizeof` 280 / align 8; `id`@160, `foreground`@168 … `box_color`@208, `font`@216,
  `fontset`@224, the bit-field run's `bits`@240, `hash`@248, `next`@256, `prev`@264,
  `ascii_face`@272; `HAVE_WINDOW_SYSTEM`'s `gc`/`stipple`, `HAVE_NS`'s `synth_ital` and
  `HAVE_XFT`/`HAVE_FREETYPE`'s `extra` are absent) and `struct face_cache` (`sizeof` 40).  The
  twenty-one `ENUM_BF`/`bool_bf` members live in one `bits: u32` storage unit read and written
  through the member-named `face_bits!` macro (P-80).
- `struct font` (`sizeof` 184; `props`@8 … `min_width`@144, `driver`@176) - this unit reads only
  `props` and the metrics.
- `FRAME_FACE_CACHE` as a function for reads (C's call shape) plus a two-argument *macro* of the
  same name for the three writes C spells as assignments (Rust will not assign to a cast);
  `FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`, `FRAME_TERMINAL` (the mirror's macro),
  `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL`, `FRAME_MSDOS_P`, `FRAME_W32_P`,
  `FRAME_INITIAL_P`, `FRAME_LIVE_P`, `XFRAME`, `WINDOW_FRAME`/`WINDOW_XFRAME`, `CHECK_LIVE_FRAME`,
  `FACE_SUITABLE_FOR_ASCII_CHAR_P` (live body `return true;`), `FACE_FOR_CHAR` (live body
  `return face->id;`), the `LFACE_*`/`LFACEP` macros, `TTY_CAP_*`, `FACE_TTY_DEFAULT_*` and
  `face_tty_specified_color` (unused here), `SET_FRAME_GARBAGED`, `AUTO_FRAME_ARG`,
  `FOPEN_TEXT`.
- `frame.h` 517's `bool_bf face_change : 1` is packed into the mirror's `bits2` and no mirror
  carries a setter, so this unit writes it through `frame_bits! (face_change, f, true)` - the
  member name stays the macro's first argument, which is what keeps C's token in the ported text.
- Externs declared here because no mirror in this unit's include graph carries them:
  `decode_live_frame`, `hashtest_eq`, `FUNCTIONP`, `font_style_symbolic`, `font_style_to_value`,
  `font_clear_prop`, `copy_font_spec`, `merge_font_spec`, `emacs_fopen`, `fgets`, `sscanf`,
  `lrint`, `add_to_log` (C-variadic), `assoc_no_quit`, `store_in_alist`, `is_tty_frame`,
  `tty_capable_p`, `font_update_sort_order`, `forget_escape_and_glyphless_faces`,
  `clear_current_matrices`.

### `DEFVAR_*` storages (status 3.29 / P-21)

C's storage is `globals.f_<name>`; the port's `include/globals.rs` declares the names
(`use variables::*`), so the ten carriers this file owns are defined at the crate root and the
definition shadows the glob import: `Vface_new_frame_defaults`, `Vface_default_stipple`,
`Vtty_defined_color_alist`, `Vscalable_fonts_allowed`, `Vface_ignored_fonts`,
`Vface_remapping_alist`, `Vface_font_rescale_alist`, `Vface_font_lax_matched_attributes`,
`face_filters_always_match` and `face_near_same_color_threshold`.  The two plain `Lisp_Object`
globals C defines here (C 317/323, `Vface_alternative_font_family_alist` /
`Vface_alternative_font_registry_alist`) are defined the same way, with `staticpro`; a module plus
`use module::*` would be *ambiguous* with `globals.rs`'s glob, which is why they are crate-root
items.  `char unspecified_fg[]`/`unspecified_bg[]` (C 311) are `[c_char; 15]` statics;
`face_change` and `unspecified_fg` are exported because C's are non-`static`.

### Spelling deviations, one by one

- C's ten forward declarations (C 371-389) are deleted and the definitions keep C's positions
  (PORTING.md 0.1-12): Rust items are order-independent.
- `resolve_face_name`'s `while (true)` is `loop`; `evaluate_face_filter`'s and `filter_face_ref`'s
  inner block + `goto err` is a labelled block whose `break 'err_block` lands on the same three
  statements; the three `switch`es (`load_color2`, `face_attr_equal_p`, `lookup_basic_face`,
  `map_tty_color`) are `match`es with `_` for `default:` (rows in `macro-body-notes.tsv`).
- `PARSE_RGB_LIST_FIELD` is a `macro_rules!` with one arm per member (`red`/`green`/`blue`) and the
  caller's `rgb_list`/`color` passed in: C's statement macro captures both of those *and* its
  `field` parameter, and Rust cannot paste an identifier onto `(*color).`; the `field` token is the
  one exemption in the notes file.  `HANDLE_INVALID_NIL_VALUE` keeps C's two macro parameters plus
  the captured `value`, which the port passes as the first metavariable.
- C's `Lisp_Object x = Qnil;` becomes `Qnil ()` (P-17) except in `static` initialisers, which need
  the `const fn` form `lisp_h_Qnil ()`.
- `f->face_change = true` is `frame_bits! (face_change, f, true)` (the bit-field write above);
  `FRAME_FACE_CACHE (f) = …` is `FRAME_FACE_CACHE! (f, …)`.
- `arrow` `fset_redisplay (f)` needs the mirror's `*mut c_void` parameter, so the five call sites
  pass `f as *mut c_void`; `sort_overlays`' `struct window *` is likewise `*mut c_void`.
- C's `0`/`NULL` passed for a `struct named_merge_point *` is kept as `0 as *mut
  named_merge_point` where C writes `0` and as `std::ptr::null_mut ()` where C writes `NULL`
  (the token check canonicalises `null_mut` to `null`).
- `SDATA` is `*mut u8` in the mirror, so `*SDATA (…) == ':'` compares against `':' as u8`.
- `color_distance`'s `long long` is `i64` (C's `long long` tokens are skipped on both sides).
- `XVECTOR (x).contents` is spelled `(*XVECTOR (x)).contents.as_mut_ptr ()`: the mirror's `XVECTOR`
  hands back a raw pointer whose `contents` is the flexible member.

### Line counts

C 7,843 lines / 5,489 live (the extractor's own count; the number above).  The port is 5,545 lines, i.e. +1.0%
against the **live** baseline, so `docs/ports/line-drift-tolerance.tsv`'s `xfaces` row asks for
`live` (against the total-line baseline the drift would be 29.3%, which would silence the check).
The excess is the use-site declaration block (about 150 lines with their `/// C:` lines and
`const _` assertions), the `#![allow]`/include block and the `DEFVAR_*` carriers; the savings are
the recorded dead text (2,355 lines) and the `DEFUN!` docstrings, which turn C's twenty-five
comment blocks into single-line literals.

### Call-relationship differences (D7)

The identifiers-after-`(` sets differ by exactly eight tokens, all of them C syntax the port cannot
spell with a call: `for`, `switch`, `sizeof`, `offsetof` (loops, `match`, `size_of`), `defined`
(the `#if` evaluator's own name), `HAVE_WINDOW_SYSTEM` (the same), and `defined_color_hook`, which
C calls through `FRAME_TERMINAL (f)->defined_color_hook (…)` and the port reads as the mirror's
field.  Everything else in C's live text - every `static` helper and every exported function,
including the twenty-one this unit exports (`init_frame_faces`, `free_frame_faces`,
`recompute_basic_faces`, `clear_face_cache`, `parse_color_spec`, `tty_defined_color`,
`tty_color_name`, `update_face_from_frame_parameter`, `free_all_realized_faces`,
`lookup_named_face`, `lookup_basic_face`, `smaller_face`, `face_with_height`,
`lookup_derived_face`, `compute_char_face`, `face_at_buffer_position`, `face_for_overlay_string`,
`face_at_string_position`, `merge_faces`, `init_xfaces`, `syms_of_xfaces`) - keeps C's name; the
`nm`-level check (`check-symbols.py --dir src`) reports 0 mismatched for the 67 units it can
compare.

### Residual (reported, not fixed)

- **`include/dispextern.rs` stops before its face block**, so `struct face`, `struct face_cache`,
  `FACE_FROM_ID`, `FACE_FROM_ID_OR_NULL`, `FACE_SUITABLE_FOR_ASCII_CHAR_P`, `FACE_FOR_CHAR`, the
  `LFACE_*` macros, `TTY_CAP_*`, `FACE_TTY_DEFAULT_*` and `set_lface_from_font` have no mirror
  home and are declared at their use sites in this unit (and in `term.rs`/`font.rs`/`frame.rs`,
  independently).  A `dispextern.h` face-section mirror would let all four units drop their copies
  (P-26/P-70).
- **`include/font.rs` is a WIP mirror**: it carries `FONTP`/`FONT_SPEC_P`/`FONT_OBJECT_P` and the
  `FONT_*_INDEX` constants but not `struct font`, the `FONT_*_FOR_FACE`/`FONT_*_NAME_NUMERIC`
  macros, or `font_style_symbolic`/`font_style_to_value` - all declared here.
- **`frame.h`'s `face_change` bit-field has no accessor** in `include/frame.rs` (only the shift),
  so this unit owns the writer; `FRAME_FACE_CACHE` has no write form (a cast cannot be an
  assignment target), hence the dual function/macro pair.
- **`fset_redisplay`/`bset_redisplay`/`sort_overlays` take `*mut c_void`** in the mirrors while C
  passes `struct frame *`/`struct window *`, so every call site casts.
- `xstrcasecmp` (dispextern.h's INLINE) is *not* carried: C only calls it from dead arms and
  `lface_same_font_attributes_p`, which is itself dead - the port has no counterpart and no
  unused helper.

### Integrator addition (2026-10-04, window.h's geometry family)

- The use-site declarations this ledger recorded for `WINDOW_FRAME` (frame.h 929) and `WINDOW_XFRAME` (window.h 620) are the mirrors' now: the window.h
  geometry macros (and, where they were here, the dispextern.h `MATRIX_*_LINE_HEIGHT`/
  `CURRENT_*_LINE_HEIGHT` family) moved into `include/window.rs`/`include/dispextern.rs` with
  window.h's third slice and the local copies were deleted (PORTING.md P-26;
  `docs/ports/include-window.md`'s "Decisions (third slice, 2026-10-04: the geometry family)"
  has the full list and the reason the family keeps C's `macro_rules!` shape).  Call sites keep
  C's text; `estimate_mode_line_height` (dispextern.h 3582) stays a use-site declaration here.
