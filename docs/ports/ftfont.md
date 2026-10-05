# emacs/src/ftfont.c (3243 lines)

> **This file is P-86's unit: the frozen configuration compiles no `ftfont.o`.**  `configure.ac`
> 7199-7203 adds it to `FONT_OBJ` only inside the `HAVE_X_WINDOWS` branch, and only when
> `HAVE_CAIRO`, `HAVE_XFT` or `HAVE_FREETYPE` is set (4525: cairo requires libfreetype); 7208 does
> the same for the `window_system=pgtk` branch and 7213 for be-app + cairo.  **Every one of those
> branches has `HAVE_FREETYPE=yes`**, which is why this port's file gate is `have_freetype`; that
> macro is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1342, `FONT_OBJ=` is empty in
> `build/emacs-ref/src/Makefile` 342, and `docs/baseline/frozen-build-sources.txt` lists no
> `src/ftfont.o`.  `extract-abi.py`'s checklist for the file is therefore empty, so P-86 ②'s
> substitute is what follows: the **C line-range table**, the `## Evidence` readings and the
> per-`#if` `## Deviations`.
>
> Port: `femacs/src/ftfont.rs`, **slice 1 = C 1-1352**, both arms of every `#if` carried.  The slice
> ends on the `}` of `ftfont_entity_pattern` (C 1352) - the last of the three entry points
> `ftfont.h` 45-47 declares - and the old revision's conclusion ("not in the frozen build, so the
> port is the empty unit plus a note") is P-86 ①'s "not started" case; its measurements are still
> true and are still where the file gate comes from.
>
> **The next live definition is `ftfont_open` (C 1357), under C 1354's `#ifndef USE_CAIRO`.**

**Why 1352.**  P-86 ④ says to start at C 1 and stop on a whole-item boundary.  C 1354-1551's
`#ifndef USE_CAIRO` arm holds `ftfont_open` (C 1357), the file's largest function; the boundaries
nearest the 1,200-1,600-line target are the `}` of `ftfont_entity_pattern` (C 1352) and, one item
earlier, the `}` of `ftfont_add_rendering_parameters` (C 1302).  Landing the three `ftfont.h`
entry points (`ftfont_fix_match`, `ftfont_add_rendering_parameters`, `ftfont_entity_pattern`) also
makes the unit's *exported* surface the file's first four non-`static` definitions
(`ftfont_get_cache`, `ftfont_list2`, `ftfont_match2`, `ftfont_list_family` came earlier), and it is
what stops `extract-abi.py --check` being vacuous for the file (P-86 ⑩): before this slice the port
had no `fn` at all.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 7199-7213's `FONT_OBJ`) | `feature = "have_freetype"` (on every item and on C 22-47's include graph) | a FreeType-capable window-system build |
| C 29-32 `#ifdef HAVE_XFT` (`<X11/Xlib.h>`, `<X11/Xft/Xft.h>`) | `feature = "have_xft"` (already registered) | X11 with Xft |
| C 33-35 `#ifdef USE_CAIRO` (`<cairo-ft.h>`) | `feature = "use_cairo"` (already registered) | the cairo backend |
| C 50-52 `#ifdef HAVE_HARFBUZZ` (`fthbfont_driver`) | `feature = "harfbuzz"` (registered name of `HAVE_HARFBUZZ`) | HarfBuzz present |
| C 155-161 `#ifdef FC_FONTFORMAT` | `feature = "fc_fontformat"` | the Fontconfig headers define `FC_FONTFORMAT` |
| C 203-220 `#ifdef FC_VARIABLE` | `feature = "fc_variable"` | they define `FC_VARIABLE` |
| C 472-493, C 570-577 `#ifdef HAVE_LIBOTF` | `feature = "have_libotf"` | libotf present |
| C 578-590 `#elif defined HAVE_HARFBUZZ` | `all (not (feature = "have_libotf"), feature = "harfbuzz")` | libotf absent, HarfBuzz present |
| C 659-663 / 823-827 / 972-976 `#if defined HAVE_XFT && defined FC_COLOR` + the Xft-version test | `all (feature = "have_xft", feature = "fc_color", feature = "xft_version_lt_2_3_6")` | X11 + Xft older than 2.3.6 |
| C 921 / 1005 `#ifdef FC_CAPABILITY` | `feature = "fc_capability"` | they define `FC_CAPABILITY` |
| C 927 / 972 `FC_COLOR` in the object-set list | `all (feature = "have_xft", feature = "fc_color")` | X11 + Xft |
| C 930 `FC_VARIABLE` in the object-set list | `feature = "fc_variable"` | as C 203 |
| C 942-968 `#if 0` | **no gate**: dead in every configuration, recorded and not transcribed | never |
| C 1016-1041 `#if defined HAVE_LIBOTF || defined HAVE_HARFBUZZ` | `any (feature = "have_libotf", feature = "harfbuzz")` (each arm spells the callee C's `#define` leaves) | libotf or HarfBuzz |
| C 1227-1229 / 1235-1238 `#ifndef FC_HINT_STYLE` / `#ifndef FC_LCD_FILTER` | **no second copy**: the two names are declared once in the C 23-24 segment | — |
| C 1297-1300 `#ifdef FC_EMBOLDEN` | `feature = "fc_embolden"` | they define `FC_EMBOLDEN` |

Seven of those names are **not in the tree's feature table yet** and are Owed 5:
`have_libotf`, `fc_fontformat`, `fc_variable`, `fc_capability`, `fc_color`, `fc_embolden` and
`xft_version_lt_2_3_6`.

## Decisions

- **The file gate is `have_freetype`, measured from the build list rather than inferred.**  Every
  `configure.ac` branch that puts `ftfont.o` in `FONT_OBJ` (7199-7203, 7208, 7213) is inside a
  configuration with `HAVE_FREETYPE=yes`: cairo requires libfreetype (4525) and the X11 branch's
  own third arm tests `$HAVE_FREETYPE = "yes"`.  `HAVE_FREETYPE` is `undef` here, so the frozen
  configuration compiles nothing from the unit (Evidence ①), and `have_freetype` is the one
  already-registered name that stands for "this file is in the build" (the `xfns.rs` reading of
  `have_x_windows` for `WINDOW_SYSTEM_OBJ`).
- **`src/ftfont.c` is in `docs/baseline/config/cond-eval-verified.txt` (286)**, so
  `check-cond-eval.py src/ftfont.c` reads `62 conditions, 0 unknown, 0 disagreements [VERIFIED]`
  and `extract-abi.py`'s inventory of the file is a live statement, not an empty one: the frozen
  live set is exactly the FreeType arm (`have_xft`, `use_cairo`, `harfbuzz`, `have_libotf` and
  `HAVE_M17N_FLT` all `undef`), which is what Evidence ⑥'s `- ` rows count out.
- **The external SDK surfaces stand in the unit, at the C `#include` lines** (P-88 ①):
  `<fontconfig/fontconfig.h>`, `<fontconfig/fcfreetype.h>`, `<ft2build.h>`/`FT_FREETYPE_H`,
  `<otf.h>` and `<hb.h>` have no file in the reference tree, so the segment after C 22 declares
  exactly the names C 1-1352 uses - 24 `FC_*` string macros, the ten Fontconfig records/enums, 37
  Fontconfig entry points, and the FreeType/bdf/libotf/harfbuzz half.  Every declaration carries
  the header's own line and every record's layout is a clang reading (P-88 ②, Evidence ④).  All
  four surfaces have two or more consumers in the tree (`src/ftfont.c`, `src/ftfont.h`,
  `src/xftfont.c`, `src/ftcrfont.c`; Fontconfig also `src/xsettings.c` and `lwlib/lwlib-utils.h`),
  so P-90 ① wants a dedicated mirror each - Owed 4 names `include/fontconfig.rs`,
  `include/freetype.rs`, `include/xft.rs` and `include/hb.rs`, and the move-back is P-90 ③'s.
- **`<X11/Xlib.h>`/`<X11/Xft/Xft.h>` (C 30-31) and `<cairo-ft.h>` (C 34) need no declaration
  here.**  No name either header defines is used by C 1-1352's body: the Xft version macros of
  C 660-663 are the derived feature, and the Xft/cairo types reach `struct font_info`, which is
  `femacs/src/include/ftfont.rs`'s item.  The two include blocks are therefore notes at their C
  line, and the X11 surface itself has the dedicated mirror `femacs/src/include/xlib.rs`.
- **Three entry points `ftfont.h` declares are defined at C 1203-1352**, and each carries
  `#[unsafe(no_mangle)] pub unsafe extern "C"` (PORTING.md 0.2); the same is true of
  `ftfont_get_cache` (C 495), `ftfont_list2` (C 1091), `ftfont_match2` (C 1151) and
  `ftfont_list_family` (C 1161).  The `static` helpers stay plain `unsafe fn` - none of them has
  its address taken in this slice.
- **`struct frame *` is spelled `*mut c_void`** in all six signatures of the slice (P-55 /
  0.1-2): `ftfont.c` includes no `frame.h`, `lisp.h` 3928's `struct frame;` is the incomplete
  declaration C reaches it through, and Rust has no forward declaration for a type another file
  defines.  The `/// C: …` line over each declaration keeps C's prototype.
- **C's `goto`-with-cleanup shapes are labelled blocks** (PORTING.md 2.2): `err`/`finish` in
  `ftfont_resolve_generic_family`, `ftfont_lookup_cache`, `ftfont_spec_pattern`, `ftfont_list` and
  `ftfont_list_family`.  The six `macro-body-notes.tsv` rows this owes are written (the file is in
  this lane's write scope); each names the C line of the label and the reason the `goto` token
  cannot reappear.
- **C's `#ifdef`-guarded arguments of a varargs call cannot be gated one by one** (C 921-932's
  `FcObjectSetBuild` list).  The port keeps C's eleven fixed names in the call and appends the four
  conditional ones with `FcObjectSetAdd` - the same object set in the same order (they are C's last
  four before `NULL`, and C 944 already appends `FC_CHARSET` that way) - while the features keep
  their C conditions at C 927/972 (`FC_COLOR`), C 1005 (`FC_CAPABILITY`) and C 1297
  (`FC_EMBOLDEN`).  Deviation 6 states it at the site.

## C line ranges

Rows are **contiguous and ascending** (each absorbs the blank lines that follow the one before) and
cover C 1-3243; `docs/port-map.md` 443 pairs only `src/ftfont.c` with this unit, so one table is
what `check-dead-file-coverage.py` reads.  `transcribed` = in `femacs/src/ftfont.rs`; `owed` = a
later slice.  The `arm` cell gives the port gate, which `## Gate mapping` derives.

| C | block | arm |
| --- | --- | --- |
| 1-20 | the licence/declaration block | dropped (P-86 ①/0.1-19; the port opens with the `// femacs: port of …` line) |
| 21-35 | `#include <config.h>` (C 22), the two Fontconfig headers (C 23-24), C 26-35's two comment/`#ifdef` blocks (`<X11/Xlib.h>`, `<X11/Xft/Xft.h>`, `<cairo-ft.h>`) | `have_freetype` (C 22-24, where the whole Fontconfig+FreeType declaration segment stands); `have_xft` (C 29-32, note only); `use_cairo` (C 33-35, note only) - **transcribed** |
| 36-37 | the blank and `#include <c-strcase.h>` | file gate - **transcribed** |
| 38-47 | the blank and the nine Emacs headers (`lisp.h` … `pdumper.h`) | file gate - **transcribed** |
| 48-52 | C 49's `static struct font_driver const ftfont_driver;`, C 50-52's `#ifdef HAVE_HARFBUZZ` + `fthbfont_driver` | file gate; `harfbuzz` (C 50-52) - **recorded, not transcribed** (C tentative definitions have no Rust counterpart, 0.1-12; the objects land with C 3154/C 3235) |
| 53-64 | the four comments and the four state objects (`fc_initialized`, `ft_library`, `freetype_font_cache`, `ft_face_cache`) | file gate - **transcribed** |
| 65-71 | `enum ftfont_cache_for` | file gate - **transcribed** (the tag is `c_int` and the three members are its constants) |
| 72-74 | the `ftfont_lookup_cache` prototype | file gate - **recorded, not transcribed** (0.1-12; the item is at C 373) |
| 75-76 | `#define SYMBOL_FcChar8(SYM)` | file gate - **transcribed** (a `macro_rules!`) |
| 77-127 | the file-scope `fc_charset_table` (38 records, C 126's `{ NULL }` sentinel) | file gate - **transcribed** (the anonymous element type is `fc_charset_table_entry`, PORTING.md 1.3) |
| 128-133 | `matching_prefix` | file gate - **transcribed** |
| 134-186 | C 135-146's comment and `get_adstyle_property` (C 148-186) | file gate; C 155-161 `fc_fontformat` - **transcribed** |
| 187-309 | `ftfont_pattern_entity` | file gate; C 203-220 `fc_variable` - **transcribed** |
| 310-312 | the blank and `static Lisp_Object ftfont_generic_family_list;` | file gate - **transcribed** |
| 313-364 | `ftfont_resolve_generic_family` (C 314-364), with C 333-334's `goto err;` and C 361's `err:` | file gate - **transcribed** (`'err` block; `macro-body-notes.tsv`) |
| 365-371 | `struct ftfont_cache_data` | file gate - **transcribed** |
| 372-458 | `ftfont_lookup_cache` (C 373-458), with C 433/436/439's `goto finish;` and C 448's `finish:` | file gate - **transcribed** |
| 459-470 | `ftfont_get_fc_charset` | file gate - **transcribed** |
| 471-493 | C 472's `#ifdef HAVE_LIBOTF` and `ftfont_get_otf` (C 473-493) | `have_libotf` - **transcribed** (text only: see Owed 3 - `struct font_info`'s libotf members are not in the mirror) |
| 494-499 | `ftfont_get_cache` | file gate - **transcribed** (non-`static`) |
| 500-545 | `ftfont_get_charset` (C 501-545), with the `SAFE_ALLOCA`/`AUTO_STRING_WITH_LEN` sites | file gate - **transcribed** |
| 546-553 | `struct OpenTypeSpec` | file gate - **transcribed** |
| 554-568 | C 555-559's `OTF_SYM_TAG`, C 561-568's `OTF_TAG_STR` | file gate - **transcribed** (two `macro_rules!`) |
| 569-590 | C 570-577's `#ifdef HAVE_LIBOTF` `OTF_TAG_SYM`, C 578-590's `#elif defined HAVE_HARFBUZZ` arm (two `typedef`s, two `static` prototypes, three `#define`s) | `have_libotf` (C 570-577); `all (not have_libotf, harfbuzz)` (C 578-590) - **transcribed** (the arm's two callees are defined at C 3006/3024, P-91 ④) |
| 591-657 | `ftfont_get_open_type_spec` (C 593-657) | file gate - **transcribed** |
| 658-684 | C 659-663's `HAVE_XFT && FC_COLOR` + Xft-version arm and `xft_color_font_whitelisted_p` (C 665-681) | `all (have_xft, fc_color, xft_version_lt_2_3_6)` - **transcribed** |
| 685-862 | `ftfont_spec_pattern` (C 686-862), with C 334/732/754/758/764/772/792/796/802/806/810/813/816/819/822/834's `goto err;`, C 838's `goto finish;`, C 840's `err:` and C 858's `finish:`; C 823-836's `FC_COLOR` arm | file gate; `all (have_xft, fc_color, xft_version_lt_2_3_6)` (C 823-836) - **transcribed** |
| 863-1089 | `ftfont_list` (C 865-1089), with C 921-932's four conditional object-set members, C 942-968's `#if 0`, C 972-994's `FC_COLOR` filter, C 1005-1014's `FC_CAPABILITY` test and C 1016-1041's libotf/HarfBuzz arm | file gate; `have_xft && fc_color`; `fc_variable`; `fc_fontformat`; `fc_capability`; `any (have_libotf, harfbuzz)` - **transcribed** |
| 1090-1099 | `ftfont_list2` | file gate - **transcribed** (non-`static`) |
| 1100-1149 | `ftfont_match` (C 1102-1149) | file gate - **transcribed** |
| 1150-1159 | `ftfont_match2` | file gate - **transcribed** (non-`static`) |
| 1160-1201 | `ftfont_list_family` (C 1162-1201), with C 1180/1185/1189's `goto finish;` and C 1191's `finish:` | file gate - **transcribed** (non-`static`) |
| 1202-1254 | `ftfont_fix_match` (C 1204-1254), with C 1227-1229's and C 1235-1238's `#ifndef` fallback definitions | file gate - **transcribed** (the two names stand in the C 23-24 segment; `ftfont.h`'s first entry point) |
| 1255-1302 | `ftfont_add_rendering_parameters` (C 1257-1302), with C 1297-1300's `#ifdef FC_EMBOLDEN` | file gate; `fc_embolden` - **transcribed** (the second entry point) |
| 1303-1352 | `ftfont_entity_pattern` (C 1305-1352) - **the slice's last row** | file gate - **transcribed** (the third entry point; C 1354 opens the next slice) |
| 1353-1551 | C 1354's `#ifndef USE_CAIRO`, `ftfont_open` (C 1357-1517), `ftfont_close` (C 1520-1549) and C 1551's `#endif` | `not (use_cairo)` - **owed** (the next slice starts at C 1357) |
| 1552-1662 | `ftfont_has_char` (C 1554-1581), C 1583's `#ifndef USE_CAIRO`, `ftfont_encode_char` (C 1586-1594), `ftfont_glyph_metrics` (C 1597-1618), `ftfont_text_extents` (C 1621-1660) and C 1662's `#endif` | file gate; `not (use_cairo)` - **owed** |
| 1663-1725 | `ftfont_get_bitmap` (C 1665-1705) with C 1675-1679's `#ifdef FT_LOAD_TARGET_MONO` pair, `ftfont_anchor_point` (C 1708-1725) | file gate; `ft_load_target_mono` - **owed** |
| 1726-1790 | C 1727's `#ifdef HAVE_LIBOTF`, `ftfont_otf_features` (C 1730-1770), `ftfont_otf_capability` (C 1774-1790) | `have_libotf` - **owed** |
| 1791-2952 | C 1792's `#ifdef HAVE_M17N_FLT` block: the `MFLTGlyphFT`/`MFLTFont` records and the `M17N_FLT_USE_NEW_FEATURE` arms (C 1794-1800, C 1802-2757), `ftfont_get_glyph_id` (C 1824-1840), `ftfont_get_metrics` (C 1849-1903), `ftfont_check_otf` (C 1906-1976), `adjust_anchor` (C 1984-2009), `setup_otf_gstring` (C 2014-2026), `ftfont_drive_otf` (C 2044-2457), the `#else` arm's `ftfont_try_otf` (C 2460-2464) and `ftfont_drive_otf` (C 2469-2733), `ftfont_shape_by_flt` (C 2742-2937), `ftfont_shape` (C 2940-2948), C 2950's and C 2952's `#endif`s | `all (have_libotf, have_m17n_flt)` - **owed** |
| 2953-2986 | C 2954's `#if defined HAVE_OTF_GET_VARIATION_GLYPHS || defined HAVE_FT_FACE_GETCHARVARIANTINDEX`, `ftfont_variation_glyphs` (C 2956-2985), C 2986's `#endif` | `any (have_otf_get_variation_glyphs, have_ft_face_getcharvariantindex)` - **owed** |
| 2987-3087 | C 2988's `#ifdef HAVE_HARFBUZZ` block: `fthbfont_begin_hb_font` (C 2991-3000), C 3002-3003's `#ifndef HAVE_LIBOTF` + `<hb-ot.h>`, `hbotf_open` (C 3006-3021), `hbotf_check_features` (C 3024-3085), C 3086's and C 3087's `#endif`s | `harfbuzz` - **owed** (this is where C 3006/3024, the two call sites slice 1 waits on, live) |
| 3088-3131 | the two `const char *const` tables `ftfont_booleans` (C 3089-3100) and `ftfont_non_booleans` (C 3102-3131) | file gate - **owed** |
| 3132-3137 | `ftfont_filter_properties` (C 3134-3137) | file gate - **owed** (non-`static`) |
| 3138-3148 | `ftfont_combining_capability` (C 3141-3148) with C 3143/3145's `#ifdef HAVE_M17N_FLT` pair | file gate; `have_m17n_flt` - **owed** (non-`static`) |
| 3149-3182 | C 3150's `syms_of_ftfont_for_pdumper` prototype, C 3152's `#ifndef USE_CAIRO`, the `ftfont_driver` record (C 3154-3180) with C 3169/3172/3175's member arms and C 3182's `#endif` | file gate; `have_libotf`; `all (have_m17n_flt, have_libotf)`; `any (have_otf_get_variation_glyphs, have_ft_face_getcharvariantindex)` - **owed** (this row holds `ftfont_driver`, the object slice 1 owes) |
| 3183-3227 | `syms_of_ftfont` (C 3185-3227) with C 3189-3192's `#ifdef HAVE_HARFBUZZ` and C 3194-3196's `#ifdef HAVE_HAIKU` | file gate; `harfbuzz`; `have_haiku` - **owed** |
| 3228-3243 | `syms_of_ftfont_for_pdumper` (C 3230-3243) with C 3234-3242's `#ifdef HAVE_HARFBUZZ` (where `fthbfont_driver` is assigned and registered) | file gate; `harfbuzz` - **owed** |

## Findings

1. **A string macro the `.c` itself defines must be a literal the constant gate can read.**  C
   1227-1238's two `#ifndef` fallbacks make `FC_HINT_STYLE`/`FC_LCD_FILTER` part of this file's
   *own* inventory, and `extract-abi.py`'s constant rule compares a string value byte for byte:
   `pub const FC_HINT_STYLE: *const c_char = c"hintstyle".as_ptr ();` reads as
   `constant FC_HINT_STYLE: C "hintstyle" vs Rust c"hintstyle".as_ptr () (different bytes)`,
   because `is_string_literal` accepts anything starting with `c"` and `rust_string_bytes` then
   decodes the pointer expression's text.  `&CStr` (the `src/emacs.rs` 359 spelling) is both what
   the rule reads (`rust_string_bytes (c"hintstyle") == b"hintstyle"`) and a real NUL-terminated C
   string, at the price of `.as_ptr ()` at the six call sites.  The other 23 `FC_*` names are only
   *referenced* by this file, are not in its inventory, and keep the `*const c_char` shape
   (`src/comp.rs` 549's).
2. **Rust cannot put a `#[cfg]` on one argument of a variadic call**, and C 921-932's
   `FcObjectSetBuild` list has four `#ifdef`-guarded members.  Appending them with
   `FcObjectSetAdd` builds the same set in the same order (they are C's last four before `NULL`,
   and C 944 appends `FC_CHARSET` the same way), so the four features keep their C conditions and
   the freeze/`fc_*` arms both compile.
3. **An opaque `#[repr(C)] pub struct _X { _private: [u8; 0] }` needs the `repr(C)`**: without it
   every use of the type in an `extern` block is an `improper_ctypes` warning - this slice's first
   complete flip produced **51 such warnings** (28 `_FcPattern`, 9 `_FcCharSet`, 5 `_FcLangSet`,
   4 `_FcObjectSet`, 3 `_FcConfig`, 3 `FT_FaceRec_`, 2 `FT_LibraryRec_`, 1 `_FcRange`), all gone
   once the ten stand-ins carry `#[repr(C)]` (`src/callproc.rs` 200-201, `src/sound.rs` 272).
4. **A `///` doc comment directly above `#[cfg] unsafe extern "C" {` is an `unused doc comment`**
   (it is not attached to an item).  Four sites had it; the block-comment form the other segments
   use is the fixed spelling.  The remaining three warnings of the final reading are mirrors'.
5. **`TYPE_RANGED_FIXNUMP`'s C form takes a *type*, the port's takes two bounds** - `lisp.rs`
   3177's `TYPE_RANGED_FIXNUMP (lo, x, hi)`.  C's `TYPE_RANGED_FIXNUMP (FcChar32, AREF (chars, j))`
   is therefore `TYPE_RANGED_FIXNUMP (0, AREF (chars, j), UINT_MAX as EMACS_INT)` (the unsigned arm
   of `lisp.h` 3267-3270), and `UINT_MAX` arrives through `lisp.rs` -> `stdint.in.rs` ->
   `limits.in.rs`.
6. **Three mirror spellings the slice had to learn by compiling**: `eassert` is a macro
   (`eassert!`), `xstrcasecmp` lives in `pub mod dispextern_externs` (a module no glob import
   brings into a unit - the `pgtkterm.rs` 193-199 reading), and
   `fast_c_string_match_ignore_case`'s third parameter is `usize` (C's `strlen` result, so the port
   passes it without a cast).  `include/font.rs`'s own `FONT_ADD_LOG!` body then needed
   `font_add_log` in scope, which no mirror declares.
7. **The frozen build still *parses* the whole unit.**  Every item is behind `have_freetype`, but
   the first `build-tu` run reported two syntax errors from gated-off text (`expected expression,
   found keyword else`, and `a / b < c` parsing as `a / (b < c)`), so the empty-object run is a
   cheap syntax gate before any arm flip.
8. **`src/ftfont.c` is in `docs/baseline/config/cond-eval-verified.txt` (286)** - unlike the
   files whose whole body hangs on a macro the frozen configuration leaves false, this one's `#if`s
   are all decided (`62 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so `extract-abi.py`
   reads a real live inventory and `--check` is not vacuous here (Evidence ⑥).
9. **`docs/baseline/port-lines-pending.tsv` carries no `src/ftfont.c` row** even though the
   pre-slice port was 11 lines (0.3 % of 3,243, well inside the ten-percent class): the ratchet's
   reading is `0 new, 0 stale` both before and after this slice, and the file now has 2,304 lines
   (71.0 %), so P-86 ④'s `--write-pending` step has nothing to retire here.

## Evidence

**① The frozen configuration (`build-tu`).**  `cd femacs && CARGO_TARGET_DIR=/tmp/ftfont-lane
cargo run -q -p xtask -- build-tu src/ftfont.c` -> run 1 (the first revision) **exit 1**, rustc
reported **2 syntax errors** (the `else` after a `#[cfg]`-gated arm and the `a / b < c` comparison
of Finding 7); run 2 (this revision) -> `built src/ftfont.rs -> ../build/femacs/ftfont.o`,
**exit 0**, and `build/femacs/ftfont.o` is **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` -
an empty object, the same size and the same md5 the P-86 class's other units read
(`docs/ports/androidfns.md` Evidence ①).  That is the expected reading and not a proof of the new
text: every item sits behind
`have_freetype`, so under the frozen feature set rustc compiles only `include/config.rs` and the
`#![allow]` attribute (P-86 ⑧).

**② The arm flip (the FreeType arm).**

    cd femacs && CARGO_TARGET_DIR=/tmp/ftfont-lane cargo run -q -p xtask -- probe \
        src/ftfont.rs /tmp/ftfont-lane/x.rlib --crate-type=lib -C panic=abort \
        --cfg 'feature="have_freetype"' --cfg 'feature="fc_fontformat"' \
        --cfg 'feature="fc_variable"' --cfg 'feature="fc_capability"' \
        --cfg 'feature="fc_embolden"'

-> **exit 0, 0 errors, 3 warnings** (the last one of six runs; the reading is step ⑧'s).  The
artifact's own numbers move between runs because the mirrors in this unit's include graph belong to
other lanes that are editing them: run 5 wrote **11,304,816 bytes** (md5
`5580979655d9e39e0c0aa55ce4f6ca83`) and the final run **11,532,456 bytes** (md5
`2e9751af06027a0879b1d2fd9a27f6bc`); the **0 errors / 3 mirror-side warnings** reading is the same
in both.  The five `--cfg`s are the file gate plus the four Fontconfig
*header* macros this machine's `fontconfig.h` defines (measured, Finding 1: `FC_FONTFORMAT`,
`FC_VARIABLE`, `FC_CAPABILITY` and `FC_EMBOLDEN` are all present), so the flip is the arm set C's
own `#if`s select on a FreeType-only build; `have_xft`, `use_cairo`, `harfbuzz` and `have_libotf`
stay off (that is the frozen configuration's live set too, Evidence ⑥).

**Budget, reported rather than kept**: **six** `probe` runs (budget: three) and four direct-rustc
iterations on the materialisation (`build/probes/ftfont/_root.rs`, the file `probe` writes before it
invokes rustc).  The five, in order: (1) 18 errors + abort; (2) 0 errors, 59 warnings, an
11,303,504-byte rlib - the `improper_ctypes` reading of Finding 3; (3) 4 `E0308` rows (the four
`FcPatternAddInteger (pat, FC_HINT_STYLE …)` sites Finding 1's `.as_ptr ()` change had missed);
(4) 0 errors, 4 warnings, 11,304,808 bytes; (5) 0 errors, 3 warnings, 11,304,816 bytes (the gated
`///` comments of Finding 4); (6) the final re-run above, 0 errors, 3 warnings, 11,532,456 bytes -
the only difference from run 5 is the rlib's bytes, i.e. the other lanes' concurrent mirror
edits.  The four direct-rustc iterations are the same materialisation
read by the repository's own toolchain (`rustc +nightly-2026-10-01`, the pin
`femacs/rust-toolchain.toml` names) and are where the per-category counts below come from; a direct
`rustc` from outside the repository uses the stable toolchain and adds one **mirror-side** `E0658`
(`include/lisp.rs` 3756's `verror`, `c_variadic` - the `docs/ports/gnutls.md` 246 reading).

**③ The error classification of ② (P-86 ⑦), by file.**  Every row of run (1) is in this unit's own
text, and the classes are: 6 x `E0425 cannot find type frame` (Deviations 1), 4 x `E0425 cannot find
function xstrcasecmp` (Finding 6), 2 x `E0425 cannot find function font_add_log` (Finding 6), 2 x
`E0423 expected function, found macro eassert` (the same), 1 x `E0061` and 1 x `E0423` on
`TYPE_RANGED_FIXNUMP`/`FcChar32` (Finding 5), 2 x `E0308` (`FcLangSetAdd`'s `*const FcChar8` and
`fast_c_string_match_ignore_case`'s `usize`), and the mirror side's `E0658`.  The final reading has
**0 rows in any file**: all three warnings are mirror-side `unused doc comment` rows
(`lib/include/stdbit.in.rs` 644, `src/include/lisp.rs` 3934, `src/include/charset.rs` 697, all
pre-existing - the gnutls ledger names the first two).

**④ The mutation test (P-86 ⑦).**  With `let _mutation: c_int = c"injected type error".as_ptr ();`
injected as `matching_prefix`'s first statement, the same materialisation compiled with
`rustc +nightly-2026-10-01` returns **1 error** (`E0308: mismatched types`, expected `c_int`, found
`*const c_char`, at the injected line) where the unmutated text returns **0** - so the compile reads
this unit's text and the "0 errors of its own" reading is real.

**⑤ The clang probes the C 23-24 segment rests on** (P-88 ②: measured, not remembered).  The
program is `/tmp/ftfont-probe/layout.c`; command
`clang -std=gnu23 -I/opt/homebrew/include -I/opt/homebrew/include/freetype2 -o layout layout.c &&
./layout`; output, verbatim: `FC_VERSION 2 18 3`; `sizeof FcChar8 1 sizeof FcBool 4 sizeof FcChar32
4`; `sizeof FcFontSet 16 align 8 nfont 0 sfont 4 fonts 8`; `sizeof FcValue 16 align 8 type 0 u 8`
(`fcvalue_members d 8 s 8 i 8 b 8`); `FcResultMatch 0 FcResultNoMatch 1 FcResultTypeMismatch 2
FcResultNoId 3 FcResultOutOfMemory 4`; `FcTypeString 3 FcTypeInteger 1 FcTypeDouble 2 FcTypeBool 4
FcTypeCharSet 6 FcTypeLangSet 8 FcTypeRange 9`; `FcMatchPattern 0 FcMatchFont 1 FcMatchScan 2
FcTrue 1 FcFalse 0`; the `FC_*` string values (`FC_HINT_STYLE hintstyle`, `FC_LCD_FILTER lcdfilter`,
`FC_RGBA rgba`, `FC_DPI dpi`, `FC_AUTOHINT autohint`, `FC_FILE file`, `FC_INDEX index`,
`FC_FOUNDRY foundry`, `FC_FAMILY family`, `FC_WEIGHT weight`, `FC_SLANT slant`, `FC_WIDTH width`,
`FC_PIXEL_SIZE pixelsize`, `FC_SPACING spacing`, `FC_SCALABLE scalable`, `FC_STYLE style`,
`FC_CHARSET charset`, `FC_LANG lang`, `FC_ANTIALIAS antialias`, `FC_HINTING hinting`,
`FC_CAPABILITY capability`, `FC_FONTFORMAT fontformat`, `FC_COLOR color`, `FC_VARIABLE variable`,
`FC_EMBOLDEN embolden`); `sizeof BDF_PropertyRec 16 align 8 type 0 u 8 u.integer 8`;
`BDF_PROPERTY_TYPE_NONE 0 ATOM 1 INTEGER 2 CARDINAL 3`; `sizeof FT_Library 8 sizeof FT_Face 8
FT_LOAD_DEFAULT 0`.  The headers' own declarations are quoted at each `/// C:` line in the unit
(Fontconfig 2.18.3, FreeType 2.14.3; HarfBuzz's `hb_face_destroy` is `hb-face.h` 116-117).  The
include chain C 24 reaches `<string.h>`/`<stdlib.h>` through was measured with `clang -H` on a
two-line file (`freetype/config/ftstdlib.h` 36 and 88 pull them), which is why
`lib/include/string.rs`/`stdlib.rs` are included at C 23-24.

**⑥ Inventory (`extract-abi.py --check`).**
`python3 tools/extract-abi.py --check emacs/src/ftfont.c femacs/src/ftfont.rs` -> **FAIL, 14 `- `
rows**, none of them a slice item that is missing: **12 `missing function` rows** name the unported
remainder (`ftfont_open`, `ftfont_close`, `ftfont_has_char`, `ftfont_encode_char`,
`ftfont_glyph_metrics`, `ftfont_text_extents`, `ftfont_get_bitmap`, `ftfont_anchor_point`,
`ftfont_filter_properties`, `ftfont_combining_capability`, `syms_of_ftfont`,
`syms_of_ftfont_for_pdumper`), **1** is `missing file-scope object ftfont_driver` (C 49's tentative
declaration with C 3154's initialiser, Owed 7) and **1** is `line count drifted 29.0%: C 3243 vs
Rust 2304` (the slice's basis, Owed 1).

**⑦ The language fact behind Owed 3 (b).**  Two lines are enough:
`unsafe extern "C" { pub fn f (); }` plus `#[unsafe(no_mangle)] pub unsafe extern "C" fn f () {}` in
one crate is `error[E0428]: the name f is defined multiple times` - measured with the pinned nightly
(the tree's own P-39/P-21 ruling).  `include/ftfont.rs` declares this file's three `ftfont.h` entry
points under `have_x_windows`, so the X11 + FreeType configuration (which has both features on)
would collide with this unit's three definitions; the fix is that mirror's (a `pub mod`, as
`lisp.rs` does).

**⑧ The gates this lane ran.**  `python3 tools/check-dead-file-coverage.py` -> `67 ledger(s) bound
(a C range table), 0 failure(s)`, and its own line is `info docs/ports/ftfont.md: 48 range row(s)
cover src/ftfont.c (3243 lines; 0 blank line(s) between blocks need no row)` (and the same for
`src/ftfont.h`, which this ledger is also bound to). - `python3 tools/check-mirror-dupes.py` ->
`17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)`, no row naming `femacs/src/ftfont.rs`.
- `python3 tools/check-citations.py` -> **109 of its rows are this unit's and every one of them is
`external`** (the Fontconfig/FreeType/HarfBuzz header lines; 0 `drift`, 0 `range`, 0 `stale`).  The
run's own tail moved while this lane worked, and **neither reading names this unit**: the first run
printed `676 file(s) scanned, 1 drift, 0 range, 914 external, 35 allowed, 0 stale` (the row was
`femacs/src/include/font.rs:577`, `font.h 562-565`) and the last one prints `676 file(s) scanned, 2
drift, 0 range, 913 external, 34 allowed, 1 stale` (both rows and the stale allowlist row are in
`femacs/src/include/dispextern.rs` 587/593) - a mirror file this lane did not touch. - `python3 tools/check-declarations.py` -> `10 declaration sites (0 forbidden, 10
allowed literals), 0 failures` (exit 0). - `python3 tools/check-port-lines.py` -> `53 file(s) under
ten percent, 69,407 reference line(s); 53 ratchet row(s), 0 new, 0 stale` (exit 0), with no
`src/ftfont.c` row (Finding 9).

## Deviations

1. **`struct frame *` is `*mut c_void`** in all six signatures (`ftfont_get_cache` C 496,
   `ftfont_list` C 865, `ftfont_match` C 1102, `ftfont_list2` C 1092, `ftfont_match2` C 1152,
   `ftfont_list_family` C 1162).  C reaches the incomplete type through `lisp.h` 3928's
   `struct frame;`; Rust has no forward declaration for a record another file defines and this unit
   includes no `frame.h`, so the pointer is the ABI-equivalent untyped one and each declaration
   keeps C's prototype in its `/// C:` line (PORTING.md 0.1-2/P-55, the `include/font.rs` 533
   `start_for_frame` reading).
2. **C 29-35's three SDK includes are notes, not `include!`s.**  No name from `<X11/Xlib.h>`,
   `<X11/Xft/Xft.h>` or `<cairo-ft.h>` is used by C 1-1352's body: the Xft version macros are the
   derived feature `xft_version_lt_2_3_6`, and the Xft/cairo types belong to `struct font_info`
   (`femacs/src/include/ftfont.rs`'s item, under its own arm).  The X11 surface's own mirror is
   `include/xlib.rs` (P-90 ①).
3. **The Fontconfig, FreeType, Libotf and HarfBuzz surfaces stand in the unit at C 23-24 / C 472**
   (P-88 ①'s use-site rule: a third-party header with no file in the reference tree).  Every
   declaration carries the header's line and the records' layouts are the clang readings of
   Evidence ⑤; the libotf entry points' signatures are **UNVERIFIED** (that toolchain is not on
   this machine) and were read off C's own call sites (C 482-486, C 1024-1037).  Owed 4 names the
   four mirrors they belong in; when one lands, this segment moves into it and disappears from here
   (P-88 ⑤/P-90 ③).
4. **`FC_HINT_STYLE` and `FC_LCD_FILTER` are `&CStr`** and their six call sites take `.as_ptr ()`,
   while the other 23 `FC_*` names are `*const c_char`.  Reason: they are the two macros this file
   *defines* (C 1227-1238's `#ifndef` fallbacks), so `extract-abi.py`'s constant rule compares their
   bytes with C's literal (Finding 1).
5. **C 1227-1229's and C 1235-1238's `#ifndef` blocks are recorded, not transcribed twice.**  C
   spells a fallback definition; the port's single declaration in the C 23-24 segment is that
   definition's counterpart, and the two C comments are kept at the function's site.
6. **C 921-932's four `#ifdef`-guarded members of the `FcObjectSetBuild` list are appended with
   `FcObjectSetAdd`** under their own features (Finding 2), because Rust cannot gate one argument of
   a variadic call.  The resulting object set is C's, in C's order.
7. **C 942-968's `#if 0` is recorded, not transcribed** (dead in every configuration; the
   `src/xfns.rs` 958 / `androidfns.md` Deviation 2 shape).
8. **C's booleans are the port's `bool` and `1`/`0` are `true`/`false`**: `fc_initialized = 1`
   (C 882/1114) and the `fc_initialized` object itself.  The `macro-body-notes.tsv` row for
   `ftfont_match` records the `1` that this leaves unspelled.
9. **`enum ftfont_cache_for` is `pub type ftfont_cache_for = c_int` with its three members as
   constants** (the `include/gtk.rs` 373-378 shape, PORTING.md 1.3's enum ABI), which is also what
   `extract-abi.py`'s three `FTFONT_CACHE_FOR_*` constant rows ask for.
10. **The anonymous struct of the `fc_charset_table` array is named `fc_charset_table_entry`**
    (PORTING.md 1.3: a nested anonymous struct must be named; the `charset_map_entry`/`table_entry`
    shape), and C's partial aggregate initialisation is written out (the unused `uniquifier` slots
    and the `lang` pointers are explicit zeros).
11. **C's uninitialised locals are zero-initialised up front** (PORTING.md 0.1-13): `get_adstyle_
    property`'s `fcstr`; `ftfont_pattern_entity`'s `str_`/`idx`/`numeric`/`dbl`/`b`/`ft_face`/`rec`;
    `ftfont_lookup_cache`'s `cache`/`val`/`cache_data`/`ft_face`… ; `ftfont_spec_pattern`'s
    `pattern`/`charset`/`langset`/`tmp`/`script`/`fc_charset_idx`; `ftfont_list`'s eleven;
    `ftfont_match`'s `entity`/`pattern`/`match_`/`result`/`otlayout`/`otspec`/`langname` and its
    `FcValue value` (`unsafe { std::mem::zeroed () }`); `ftfont_fix_match`'s `b`/`i`/`dpi`;
    `ftfont_add_rendering_parameters`'s `ival`.  `ftfont_get_charset`'s `i`/`j` keep C's two
    initial assignments as statements.
12. **`FcValue`'s `type` member is `r#type`** (a Rust keyword; the `src/alloc.rs` 563 spelling), and
    the record is a `#[repr(C)]` struct over a `#[repr(C)]` `Copy` union (PORTING.md 0.1-9).
13. **C's chained assignments are two statements**: `spec->nfeatures[0] = spec->nfeatures[1] = 0;`
    (C 624) and `i = j = 0` in `ftfont_get_charset` (C 509), whose `for`-head increments become the
    loop body's `i += 1; j += 1;`.
14. **`min (PTRDIFF_MAX, SIZE_MAX)` is `min (PTRDIFF_MAX, SIZE_MAX as isize)`** (the port's `min` is
    `(isize, isize)`), and the allocation is `malloc ((len * size_of::<c_int> () as isize) as
    usize)` for C's `malloc (len * sizeof *spec->features[i])`.
15. **C's `for (i = 0; i < n; i++)` loops with `continue` are `while` loops with an explicit
    `i += 1; continue;`** at each site, with the C line of every `continue` named at the site
    (`ftfont_list`'s six, the sfntfont.rs 1136 shape).
16. **Under `harfbuzz` the two `hbotf_*` call sites wait on definitions later in the file**
    (C 3006's `hbotf_open`, C 3024's `hbotf_check_features`): P-91 ④'s allowed class, named at each
    site.  `OTF_open`/`OTF_close`/`OTF_check_features` are object-like `#define`s C 587-589 makes,
    so each arm spells the callee C's preprocessor leaves (`OTF_open` / `hbotf_open`); the port's
    own `#[cfg]`-selected statements are the two arms.
17. **`ftfont_get_otf` (C 472-493) is transcribed but cannot be typed here**: its parameter is the
    mirror's `struct font_info`, which `include/ftfont.rs` gates on `have_x_windows` and whose two
    `HAVE_LIBOTF` members (`maybe_otf`, `otf`) it records as dead text.  Owed 3 (a) is the mirror
    work; the text keeps C's member paths so the fix is the mirror's alone.
18. **C 24's transitive `<string.h>`/`<stdlib.h>`** (through `fcfreetype.h` -> `ftstdlib.h`) are
    included at that C line, and `strstr` is declared at the use site because the frozen
    `<string.h>` mirror does not carry it (Evidence ⑤, Owed 3 (d)).
19. **Line count**: C 3243 vs Rust 2304 (29.0 % drift) - the slice's basis, not a whole-file claim.
    A `docs/ports/line-drift-tolerance.tsv` row is owed (Owed 1).

## Owed by the integrator (nothing here is claimed as done)

1. `docs/ports/line-drift-tolerance.tsv`: one row, stem `ftfont`, tolerance **>= 0.30**, basis
   `total` (the file is 29.0 % short of C's line count in this slice).  The file is outside this
   lane's write scope, so `extract-abi.py --check`'s line-count row cannot clear (P-86 ⑩) even
   though the other 13 rows are the unported remainder's.  **DECLINED, deliberately (status 3.497,
   integrator):** a tolerance row is for a **completed** file (3.434 clause 5) - this is slice 1 of
   13 owed blocks, C 1353-3243 is unported, and the check's `12 missing function` + `missing
   file-scope object ftfont_driver` rows *are* that remainder's list.  Writing the row would clear
   the line-count reminder while the file is 59 % unwritten, which is exactly the mistake 3.489 had
   to withdraw for gnutls.
2. `docs/baseline/port-lines-pending.tsv`: nothing to retire - the file has left P-86's
   under-ten-percent class (2,304 lines against a 324-line threshold) and the ratchet already reads
   `0 new, 0 stale` (Finding 9).  Named here because P-86 ④'s closing step asks for the reading,
   not because a row moved.
3. **The mirror gaps this slice's compile readings name**, each in the file that owns it:
   (a) `femacs/src/include/ftfont.rs`: `struct font_info` is gated `have_x_windows` while C's record
   is unconditional and its two `HAVE_LIBOTF` members are recorded as dead text - so
   `ftfont_get_otf`'s `have_libotf` arm (C 472-493) cannot be typed (Deviation 17);
   (b) the same mirror's three declarations (`ftfont_fix_match`, `ftfont_add_rendering_parameters`,
   `ftfont_entity_pattern`, its C 45-54) collide with this unit's definitions in any configuration
   that has `have_x_windows` on (Evidence ⑦): they belong in a `pub mod` the unit does not
   glob-import (P-39's shape);
   (c) `femacs/src/include/font.rs`: declare `font_make_entity` (font.h 842), `font_style_to_value`
   (font.h 854), `font_intern_prop` (font.h 885), `font_put_extra` (font.h 913) and `font_add_log`
   (font.h 1001 - the mirror's own `FONT_ADD_LOG!` body calls it);
   (d) `femacs/lib/include/string.rs`: `strstr` (the rest of the family this slice calls is there);
   (e) `femacs/src/include/dispextern.rs`: `xstrcasecmp` is inside `pub mod dispextern_externs`,
   which no glob import reaches - the unit calls it by that path, and `src/font.rs` 298 keeps its
   own copy; the mirror's owner may prefer one spelling.
4. **The four SDK mirrors P-90 ① wants** (`docs/ports/include-fontconfig.md`, `-freetype.md`,
   `-xft.md`, `-hb.md`): the C 23-24 segment of the unit moves into `femacs/src/include/
   fontconfig.rs` and the C 472 / C 570-590 segment into `include/freetype.rs`, `include/xft.rs` and
   `include/hb.rs`, deleting the in-unit copies in the same commit (P-90 ③ / P-88 ⑤).  Xft has no
   headers on this machine, so that mirror's numbers would be UNVERIFIED here.
5. **Seven features to register** (`femacs/xtask/Cargo.toml` + `docs/baseline/feature-coverage.tsv`),
   each with its C criterion line: `have_libotf` (C 472, C 570), `fc_fontformat` (C 155),
   `fc_variable` (C 203, C 930), `fc_capability` (C 921, C 1005), `fc_color` (C 659, C 927, C 972),
   `fc_embolden` (C 1297) and `xft_version_lt_2_3_6` (C 660-663's `XFT_MAJOR`/`XFT_MINOR`/
   `XFT_REVISION` test, also at C 824-827 and C 973-976).  The last one is a *derived* condition with
   no single macro, so its name follows the `webkit_check_version_ge_2_16` /
   `sqlite_version_number_ge_3007015` convention; the other six are the macros' own lower-cased
   spellings.
6. `docs/ports/macro-body-notes.tsv`: the six rows this slice's `goto` blocks and the boolean
   spelling need - **written** (the file is in this lane's write scope): `ftfont_resolve_generic_
   family`, `ftfont_lookup_cache`, `ftfont_spec_pattern`, `ftfont_list`, `ftfont_list_family`
   (`goto`) and `ftfont_match` (`1`).
7. **`missing file-scope object ftfont_driver`**: C 49's tentative declaration plus C 3154-3180's
   initialiser (and C 3233/3241's two `register_font_driver` calls).  It lands with the C 3154 row;
   `fthbfont_driver` (C 51, C 3235-3241) with the `harfbuzz` arm of the same row.
8. **The next slice**: **C 1357's `ftfont_open`**, under C 1354's `#ifndef USE_CAIRO`.  After it the
   C line-range table's `owed` rows are the order: C 1554 `ftfont_has_char`, C 1665
   `ftfont_get_bitmap`, C 1730 `ftfont_otf_features`, C 2956 `ftfont_variation_glyphs`, C 2991
   `fthbfont_begin_hb_font` (which unblocks Deviation 16's two call sites), C 3134
   `ftfont_filter_properties`, C 3141 `ftfont_combining_capability`, C 3154's `ftfont_driver`
   record, C 3185 `syms_of_ftfont`, C 3230 `syms_of_ftfont_for_pdumper`.

## Not claimed

- **No behaviour.**  The frozen configuration compiles nothing from this file (the 336-byte object
  is empty), and the flip reading is a compile: no Fontconfig, FreeType, libotf, HarfBuzz or Xft
  call was made, and no font was listed, matched, opened, measured or shaped.
- The arm flip is `have_freetype` + four Fontconfig-header features (Evidence ②).  It is not a
  link, not a load and not a P-86 ③⑴ "the class can be verified" claim: the Xft, cairo, HarfBuzz
  and libotf arms are **not compiled at all** on this machine (no Xft headers; no libotf), and the
  libotf arm's text cannot be typed until Owed 3 (a) lands.
- The layouts and constants in the C 23-24 segment are this machine's Fontconfig 2.18.3 and
  FreeType 2.14.3 readings; the frozen configuration has neither header, so nothing here is a claim
  about what the file would compile against on another target.  The libotf entry points are
  UNVERIFIED (Owed 4).
- C 1353-3243 is unported: nothing in this ledger speaks for those 1,891 lines, which are exactly
  the 12 `missing function` rows and the `ftfont_driver` object of Evidence ⑥.
- Only `femacs/src/ftfont.rs`, `docs/ports/ftfont.md` and the six rows of
  `docs/ports/macro-body-notes.tsv` were written by this lane.

The integrator's remaining steps for this unit (status 3.497): the seven feature names of `## Owed`
4's table are registered in `femacs/xtask/Cargo.toml`
(`have_libotf`, `fc_capability`, `fc_color`, `fc_embolden`, `fc_fontformat`, `fc_variable`,
`xft_version_lt_2_3_6`), `tools/feature-coverage.py --write` has been re-run, and the round is
written up in `docs/status.md` 3.497.  `docs/baseline/port-lines-pending.tsv` needs nothing (item 2).
