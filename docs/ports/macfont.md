# emacs/src/macfont.m (4257 lines)

> **This file is the Cocoa (Core Text) font backend.**  `configure.ac` 2762 sets
> `ns_fontfile=macfont.o` for the Cocoa configuration and 2816 sets `nsfont.o` for the GNUstep
> one; src/Makefile.in 480 lists both.  The two are **build alternatives**, each defines its own
> `font_driver` (`macfont_driver` 1765-1791, `nsfont_driver` in nsfont.m), and both are rows of
> `docs/port-map.md`'s Swift table, so a single Swift module cannot hold the pair (the mirror of
> the finding docs/ports/nsfont.md's header records).
>
> Unlike src/nsfont.m, this file **is** conditional - 23 `#if`s, every one of them on
> `MAC_OS_X_VERSION_MIN_REQUIRED`/`MAX_ALLOWED`, `__LP64__` or `USE_CT_GLYPH_INFO`.  The port
> follows the file's own target (the Cocoa build on a current macOS, the rule the lane adopted
> from nsfont's GNUstep arm): `USE_CT_GLYPH_INFO` is defined nowhere in this tree, so the
> `#if USE_CT_GLYPH_INFO`/`#if !USE_CT_GLYPH_INFO` pair takes the **`NSGlyphInfo` arm**
> (macfon.m 219-265 lives, 3993-4066 does not), and every `MAC_OS_X_VERSION_MIN_REQUIRED <
> 10.x` arm is dead.  `tools/check-cond-eval.py src/macfont.m` reports `23 conditions, 0
> unknown, 5 disagreements [NOT VERIFIED]`: the frozen configuration neither compiles this file
> nor defines the `MAC_OS_X_*` macros, so the frozen table cannot decide its arms and the port
> decides them by its target and records each one below.
>
> **The unit's state: 71 of the 74 functions are ported, and the unit is closed.**  Round 1 carried the engine layer
> (111-1535) and the three helpers C forward-declares at 49-56; round 2 carried the charset
> layer (1536-1756), the UVS layer (3301-3592), seven small entry points and the whole UVS data
> of `src/macuvs.h`; round 3 added `macfont_get_charset` and the `cf_charset_table` it reads;
> round 4 added the `glyph_string` mirror, `macfont_draw` and the five `CG_SET_*` macros;
> round 5 added the OpenType layer (`macfont_get_open_type_spec`,
> `macfont_create_attributes_with_spec`, `macfont_supports_charset_and_languages_p`), the two
> spec entry points `macfont_list`/`macfont_match`, and the two CoreText helpers they call
> (`mac_font_descriptor_supports_languages`, `mac_font_create_preferred_family_for_attributes`);
> round 6 (the approved 3a cut) added the rest of the driver core's entry points:
> `macfont_list_family`, `macfont_open`, `macfont_close`, `macfont_text_extents`; round 7 added
> `macfont_shape` and the two CoreText helpers it needs
> (`mac_font_create_line_with_string_and_font`, `mac_font_shape`); round 8 (this round, the
> closure) added the driver record `macfont_driver` with its `@_cdecl ("macfont_driver")`
> getter, `macfont_get_nsctfont`, `mac_register_font_driver`, `syms_of_macfont` and
> `syms_of_macfont_for_pdumper`.
> The remaining 3 are listed with a reason each under `## Deferred`; all three are in arms this
> target does not take, can never land, and are the only rows this unit leaves in the manifest.
> Nothing is stubbed.
>
> **The UVS table is Swift data** (rc 3.166, decision 1): the file carries
> `mac_uvs_table_adobe_japan1_bytes` - 73,655 bytes, generated from `emacs/src/macuvs.h` by
> `python3 tools/gen-uvs-swift.py` into the block between the `// gen-uvs-swift: begin` and
> `// gen-uvs-swift: end` markers - with no runtime file read and no resource bundle.  The
> generator and the command are in `## Evidence`.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 74

Ported (71 of 74):

- [x] cfdata_to_lisp  (line 138, static)
- [x] cfstring_to_lisp_nodecode  (line 154, static)
- [x] cfstring_create_with_string_noencode  (line 188, static)
- [x] mac_font_get_weight  (line 202, static)
- [x] mac_screen_font_get_advance_width_for_glyph  (line 210, static)
- [x] mac_font_get_glyph_for_cid  (line 219, static)
- [x] mac_screen_font_create_with_name  (line 268, static)
- [x] mac_screen_font_get_metrics  (line 280, static)
- [x] mac_font_shape_1  (line 329, static)
- [x] mac_screen_font_shape  (line 593, static)
- [x] get_cgcolor  (line 603, static)
- [x] get_cgcolor_from_nscolor  (line 619, static)
- [x] macfont_update_antialias_threshold  (line 747)
- [x] macfont_intern_prop_cfstring  (line 761, static)
- [x] macfont_store_utf32char_to_unichars  (line 769, static)
- [x] cfnumber_get_font_symbolic_traits_value  (line 788, static)
- [x] mac_font_descriptor_get_adjusted_weight  (line 806, static)
- [x] macfont_store_descriptor_attributes  (line 837, static)
- [x] macfont_descriptor_entity  (line 943, static)
- [x] macfont_invalidate_family_cache  (line 996, static)
- [x] macfont_get_family_cache_if_present  (line 1016, static)
- [x] macfont_set_family_cache  (line 1037, static)
- [x] macfont_invalidate_available_families_cache  (line 1071, static)
- [x] macfont_handle_font_change_notification  (line 1081, static)
- [x] macfont_init_font_change_handler  (line 1091, static)
- [x] macfont_copy_available_families_cache  (line 1107, static)
- [x] macfont_create_family_with_symbol  (line 1119, static)
- [x] macfont_monospace_width_multiplier  (line 1211, static)
- [x] macfont_glyph_extents  (line 1229, static)
- [x] macfont_lookup_cache  (line 1463, static)
- [x] macfont_retain_cache  (line 1509, static)
- [x] macfont_release_cache  (line 1517, static)
- [x] macfont_get_cf_charset  (line 1536, static)
- [x] macfont_get_cf_charset_for_name  (line 1544, static)
- [x] macfont_get_glyph_for_character  (line 1552, static)
- [x] macfont_get_glyph_for_cid  (line 1694, static)
- [x] macfont_get_uvs_table  (line 1705, static)
- [x] macfont_get_cache  (line 1793, static)
- [x] macfont_get_charset  (line 1801, static)
- [x] macfont_get_open_type_spec  (line 1885, static)
- [x] macfont_create_attributes_with_spec  (line 1951, static)
- [x] macfont_supports_charset_and_languages_p  (line 2190, static)
- [x] macfont_traits_distance  (line 2230, static)
- [x] macfont_closest_traits_index_p  (line 2250, static)
- [x] macfont_list  (line 2275, static)
- [x] macfont_match  (line 2595, static)
- [x] macfont_list_family  (line 2627, static)
- [x] macfont_free_entity  (line 2650, static)
- [x] macfont_open  (line 2662, static)
- [x] macfont_close  (line 2824, static)
- [x] macfont_has_char  (line 2849, static)
- [x] macfont_encode_char  (line 2878, static)
- [x] macfont_text_extents  (line 2890, static)
- [x] macfont_draw  (line 2923, static)
- [x] macfont_shape  (line 3094, static)
- [x] mac_font_copy_uvs_table  (line 3301, static)
- [x] mac_font_get_glyphs_for_variants  (line 3429, static)
- [x] macfont_variation_glyphs  (line 3520, static)
- [x] macfont_filter_properties  (line 3575, static)
- [x] mac_font_descriptor_supports_languages  (line 3581, static)
- [x] mac_font_create_preferred_family_for_attributes  (line 3617, static)
- [x] mac_font_get_advance_width_for_glyph  (line 3707, static)
- [x] mac_font_get_bounding_rect_for_glyph  (line 3714, static)
- [x] mac_font_create_available_families  (line 3721, static)
- [x] mac_font_equal_in_postscript_name  (line 3750, static)
- [x] mac_font_create_line_with_string_and_font  (line 3775, static)
- [x] mac_font_shape  (line 3839, static)
- [ ] mac_ctfont_get_glyph_for_cid  (line 3995, static)
- [ ] mac_font_copy_default_descriptors_for_language  (line 4070, static)
- [ ] mac_font_copy_default_name_for_charset_and_languages  (line 4162, static)
- [x] macfont_get_nsctfont  (line 4206)
- [x] mac_register_font_driver  (line 4215)
- [x] syms_of_macfont  (line 4225)
- [x] syms_of_macfont_for_pdumper  (line 4248, static)

The list above is the one `tools/extract-abi.py` produces; `tools/check-swift-units.py` (3.164)
reads the same inventory and now counts 74 C definitions with 23 unported for this target -
the rows in `docs/baseline/swift-units.tsv`.

## Macros defined — 26
- [x] WIDTH_FRAC_BITS (1162, body 10720c7cfed2) -> `fileprivate let WIDTH_FRAC_BITS: Int32`
- [x] WIDTH_FRAC_SCALE (1163, body e210bc0f03c5) -> computed from it, as C does
- [x] METRICS_VALUE (1184, body a5c7698585b6) -> `fileprivate func METRICS_VALUE (high, low)`
      (C's `(unsigned int) high << 8 | low`; `member_high` is a **signed** 4-bit field, so the
      port sign-extends first)
- [x] METRICS_SET_VALUE (1187, body 96a4ea1d2271) -> `fileprivate func METRICS_SET_VALUE`
      returning the `(low, high)` pair the C macro writes into the two members
- [x] METRICS_STATUS (1197, body c96775e00435) -> `fileprivate func METRICS_STATUS`
- [x] METRICS_SET_STATUS (1199, body a958d76fd21e) -> `fileprivate func METRICS_SET_STATUS`
- [x] METRICS_NCOLS_PER_ROW (1203, body f92d5e0f18f9) -> `fileprivate let` (128)
- [x] LCD_FONT_SMOOTHING_LEFT_MARGIN / _RIGHT_ (1204-1205, body 07f13e96972e) ->
      `fileprivate let` `CGFloat` 0.396 each
- [x] ROW_PERM_OFFSET (1408) / NGLYPHS_IN_VALUE (1412) (bodies 630d9ccc0aa5 / 6e10b4b23a8e) ->
      `fileprivate let`s (8, and `sizeof (void *) / sizeof (CGGlyph)` = 4)
- [x] RIGHT_TO_LEFT_P (3870, body 005fcb61ed11) -> `#define RIGHT_TO_LEFT_P permutation` (the
      alias nsfont.m has at 1304); **here it is live text** - `permutation` is `xmalloc`'d when
      the first glyph's bidi level is odd - so the port keeps C's `permutation != nil`
      expressions
- [x] BUINT8_VALUE / BUINT16_VALUE / BUINT24_VALUE / BUINT32_VALUE (3291-3295, bodies
      0270b40f1fea / c751199eb0ee / 483dd83134df / 4e8b3bb1cab9) -> four `fileprivate func`s
      over the member's address, which is what C's macros take (`OSReadBigInt*` is the
      big-endian read; `BUINT24_VALUE` reads four bytes from the three-byte field, as C's own
      comment requires)
- [x] SIZEOF_UVS_TABLE_HEADER (3258) / SIZEOF_NON_DEFAULT_UVS_TABLE_HEADER (3283) /
      SIZEOF_DEFAULT_UVS_TABLE_HEADER (3270) (bodies ae104b9d306b / 90be0aad6424 /
      4ab941ba6212) -> the same `sizeof (record) - sizeof (member)` expressions over the mirrors
- [x] OTF_SYM_TAG (1869-1873) -> `fileprivate func OTF_SYM_TAG (sym) -> UInt32` (C's macro
      assigns its second argument; the port returns the tag).  `macfont_get_open_type_spec`
      (1885) is its first reader; `macfont_create_attributes_with_spec` (1951) reaches it
      through that function
- [x] OTF_TAG_STR (1875-1882) -> **no reader in this file**: `grep -n OTF_TAG_STR
      emacs/src/macfont.m` finds only the definition, so there is no call site to transcribe and
      a Swift copy would be dead code (recorded, not written).  This corrects round 5's row,
      which claimed `macfont_shape` (3094) and `mac_font_shape` (3839) were its readers - they
      are not, and round 7 landed both and checked
- [x] CG_SET_FILL_COLOR_WITH_FACE_FOREGROUND / _FACE_BACKGROUND / _FRAME_CURSOR /
      _FRAME_BACKGROUND / CG_SET_STROKE_COLOR_WITH_FACE_FOREGROUND (633-667) -> five
      fileprivate functions of the same names taking `(context, face|frame)`, each the C
      `do { … } while (0)` body (`get_cgcolor*` + one `CGContext.set*Color`); `macfont_draw`
      calls them where C writes the macro (`## Decisions`)

## Types and their fields — 12
- [x] mac_glyph_layout (src/macfont.h 25-49) -> `struct mac_glyph_layout`.  Measured: C
      `sizeof` 56, Swift size 50/**stride 56**; `comp_range`@0, `string_index`@16,
      `advance_delta`@24, `baseline_delta`@32, `advance`@40, `glyph_id`@48 - every offset = C
- [x] macfont_info (79-94) -> `struct macfont_info`.  C `sizeof` 264, Swift size 262/**stride
      264**; `font`@0, `macfont`@216, `cgfont`@224, `screen_font`@232, `cache`@240, `metrics`@248,
      `metrics_nrows`@256, the five-bit run in the 4-byte unit at **258** (measured bit
      positions: `synthetic_italic_p` 0, `synthetic_bold_p` 1, `spacing` 2-3, `antialias` 4-5,
      `color_bitmap_p` 6), stored as two `UInt16`s because Swift would align a `UInt32` to 260
- [x] macfont_metrics (1162-1183) -> `struct macfont_metrics`.  C `sizeof` 8, Swift size 8
      (align 2 versus C's 4 - `## Decisions`); the six byte offsets of the `## Evidence` probe
- [x] macfont_cache (1413-1461) -> `struct macfont_cache` + `macfont_cache_glyph` +
      `macfont_cache_uvs`.  C `sizeof` 312 = Swift size/stride; `reference_count`@0,
      `cf_charset`@8, `glyph.row_nkeys_or_perm`@16, `glyph.matrix`@272, `glyph.nrows`@280,
      `glyph.dictionary`@288, `uvs.table`@296, `uvs.collection`@304 (stored as the raw
      `NSUInteger` the lane rule requires); the 256-byte `row_nkeys_or_perm` is a 256-element
      tuple, indexed through `row_nkeys_or_perm_at`/`set_row_nkeys_or_perm`
- [x] glyph_string (src/dispextern.h 1362-1507) -> `struct glyph_string`.  The owner mirror is
      `femacs/src/include/dispextern.rs` 1366 (behind `feature = "have_window_system"`, rc
      3.183) but is not layout-asserted there, so this lane measured the windowed form itself:
      `sizeof` **272**, align 8, and the 27 offsets of `## Evidence`; the eight-bit run at 120
      is one `UInt8` with `padding_p`/`for_overlaps` accessors (measured bit positions), and
      `struct glyph_slice` (455-462) is recorded as its 8-byte unit.  `NativeRectangle` is
      `NSRect` on this target, so `clip[2]` is a 2-tuple of `NSRect`
- [x] <anon@855> / <anon@1962> -> two fileprivate structs: `macfont_numeric_trait` (`index`,
      `trait`, `points`, `adjust_func`) and `macfont_spec_numeric_trait` (`index`, `trait`,
      `points`).  The two tables' `CGPoint` pairs are read on **opposite** components - C's
      descriptor loop walks `point->x` and answers `y`, the spec loop walks `point->y` and
      answers `x` - and the port keeps each loop's own component.  `macfont_store_descriptor_
      attributes` (its user 2923) is still deferred, so the first table is record-only for now
- [x] variation_selector_record / uvs_table / unicode_value_range / default_uvs_table /
      uvs_mapping / non_default_uvs_table (3236-3281, inside `#pragma pack(push, 1)`) ->
      mirrors whose 4-byte/2-byte members are **byte tuples**
      (`packed_uint32`/`packed_uint16`) because Swift aligns those types and C's packed layout
      does not; every read goes through the `BUINT*_VALUE` helpers.  `UINT24` is
      `typedef UInt8 UINT24[3]` (3244) -> a 3-byte tuple
- [x] `sfntCMapEncoding` / `sfntCMapHeader` and their `sizeof_*` constants (CoreText's
      `SFNTTypes.h` 269-287) - SDK records Swift does not import, so this lane mirrors them
- [x] ns_display_info's prefix (src/nsterm.h 853-928) -> `struct ns_display_info`, the same
      prefix mirror nsfont.swift carries (`name_list_element` is all `macfont_get_cache` reads)
- [x] OpenTypeSpec (1866-1872) -> `struct OpenTypeSpec` (`script`, `script_tag`, `langsys_tag`,
      the two 2-element arrays `nfeatures`/`features` as tuples with `otspec_*` accessors).  Its
      user `macfont_get_open_type_spec` (1885) landed in round 5; `features[i]` is `malloc`'d and
      `free`'d exactly as C does (uninitialised until the loop fills it, read only through
      `nfeatures[i]`)
- [x] `enum lgstring_direction` (44-48, `DIR_R2L = -1, DIR_UNKNOWN = 0, DIR_L2R = 1`) ->
      `enum lgstring_direction: Int32`, the file's **one Swift raw-value enum** and the one
      exception the "no raw-value enums" rule needs: the rule guards *mirrors of C records*
      (whose C enum field is 4 bytes where a Swift enum is 1), and this enum is a parameter type
      of this file's own `fileprivate` functions - it never crosses the ABI, so no C compiler
      sees its layout (`nsfont.swift` keeps the same one).  `mac_font_shape_1` (329),
      `mac_screen_font_shape` (593) and `macfont_shape` (3094) read it; its three members ride
      on the `DIR_*` names C gives them.  The extractor's "12" above counts the file's structs
      only, so this row is outside it

## File-scope objects — 6
- [x] static const CGAffineTransform synthetic_italic_atfm (111) -> `fileprivate let
      synthetic_italic_atfm = CGAffineTransform (a: 1, b: 0, c: 0.25, d: 1, tx: 0, ty: 0)`
- [x] static const CGFloat synthetic_bold_factor (112) -> `fileprivate let` 0.024
- [x] static CGFloat macfont_antialias_threshold (744) -> `fileprivate var`
- [x] static Lisp_Object macfont_family_cache (993) -> `fileprivate var`
- [x] static CFArrayRef macfont_available_families_cache (1068) -> `fileprivate var ... CFArray?`
- [x] static CFMutableDictionaryRef macfont_cache_dictionary (1403) -> `fileprivate var ...
      CFMutableDictionary?`
- [x] `mac_uvs_table_adobe_japan1_bytes` (src/macuvs.h 9-9218; 73,655 bytes) -> the generated
      `fileprivate let mac_uvs_table_adobe_japan1_bytes: [UInt8]` block of this file
- [x] the two function-local statics the port moved to file scope (Swift has none):
      `mac_uvs_table_adobe_japan1` (1724, inside `macfont_get_uvs_table`) and
      `macfont_font_change_handler_initialized` (1093, inside `macfont_init_font_change_handler`)
- [x] `macfont_driver` (1765-1791) - `static struct font_driver macfont_driver = { … };`, the
      record this unit's closure round lands: a Swift global
      (`fileprivate let macfont_driver: UnsafeMutablePointer<font_driver>`, allocated once at a
      process-stable address - C's `static` storage) plus the `@_cdecl ("macfont_driver")`
      getter that exports the C name (see `## ABI` (1)/(2)f).  The extractor's "6" counts other
      objects, so this row sits outside it like `cf_charset_table`'s
- [x] `cf_charset_table[]` (681-739) - a 37-row `static struct { … }` array the extractor's
      inventory does not list (its scan wants `name;`/`name =`, and this one is `name[] =`; the
      blind spot is reported): `fileprivate var cf_charset_table: [cf_charset_table_entry]` with
      C's `{ NULL }` terminator row, whose `uniquifier` is a 6-slot tuple (the written values +
      C's zero fill and terminator) and whose `name` is a `strdup` of C's literal (the same
      process lifetime, never freed as C's static storage is not).  `macfont_get_charset` reads
      it; `macfont_create_attributes_with_spec` (1951) is its other reader

## Decisions

**The file's target and its conditional arms.**  This is the Cocoa backend (`configure.ac`
2762/2816 choose between it and nsfont.m).  The port takes the arms a current macOS Cocoa build
takes and records the others; the frozen configuration neither compiles the file nor defines
`MAC_OS_X_*` (evidence below), so `tools/check-cond-eval.py` cannot verify them.

| condition | arms | taken |
| --- | --- | --- |
| 60-63 `MIN_REQUIRED < 1090` | the old-SDK `kCTFontTrait*` compat defines | no |
| 64-67 and 3993-4066 `USE_CT_GLYPH_INFO` | `mac_ctfont_get_glyph_for_cid` (3995) | **no** - defined nowhere in this tree, so macfont.h 70's `#define mac_font_get_glyph_for_cid mac_ctfont_get_glyph_for_cid` is not taken either |
| 217-265 `!USE_CT_GLYPH_INFO` | `mac_font_get_glyph_for_cid`'s `NSGlyphInfo` body | **yes** (ported) |
| 722-742 `MIN_REQUIRED < 1080` | `macfont_language_default_font_names` | no |
| 2973-2984 / 3033-3046 / 3061-3075 `MAX_ALLOWED >= 101000`/`>= 1070` with their `<` twins | the modern CoreText calls in `macfont_draw`/`macfont_shape` | the modern arms (both users deferred) |
| 3314 `__LP64__` | `mac_font_copy_uvs_table`'s length check | yes (arm64) - **ported** |
| 3626-3648 `MIN_REQUIRED >= 1090` | `mac_font_create_preferred_family_for_attributes` | yes (deferred) |
| 4068-4199 `MIN_REQUIRED < 1090` | `mac_font_copy_default_descriptors_for_language` (4070), `mac_font_copy_default_name_for_charset_and_languages` (4162) | **no** |

**Mirrors of the records this file dereferences.**  Each is measured, and the lane rule that a
mirror uses **raw integers, never Swift raw-value enums** is what keeps them C's:
`macfont_cache.uvs.collection` is C's `NSCharacterCollection` (an `NSUInteger`) stored as `UInt`
and converted at the call sites; the `struct font`/`font_metrics`/`ns_display_info` mirrors are
the same ones nsfont.swift carries (one spelling once the 9 files are one module, PORTING.md
0.1-5, the lane's open item).  Two Swift-vs-C differences are the compiler's, not the port's
choice: `MemoryLayout.size` omits a record's trailing padding, so where C's `sizeof` is the
*stride* this ledger says so (`mac_glyph_layout` 50/56, `macfont_info` 262/264,
`macfont_cache` 312/312), and `macfont_metrics`'s alignment is 2 in Swift against C's 4 (its
widest member is `Int16`, C's bit-field unit is an `int`) - the `sizeof` (8) and every offset
match.

**The bit-field runs are storage units plus accessors** (PORTING.md 1.3/P-80): the C macros
`METRICS_VALUE`/`METRICS_SET_VALUE` and the five `macfont_info` flags read and write the
measured nibbles, and `sign_extend_4bit` is C's `signed x : 4` sign extension, not an
invention.

**The five `CG_SET_*` macros' Swift shape (rc 3.184's approval).**  Each C macro is a
`do { … } while (0)` that reads `NS_FACE_FOREGROUND`/`NS_FACE_BACKGROUND`/`FRAME_*_COLOR`,
calls `get_cgcolor*` and one `CGContextSet{Fill,Stroke}ColorWithColor`; the port spells them as
five fileprivate functions of the same names (`(context, face|frame)`), each carrying C's body
with the SDK's own Swift setter replacement, and `macfont_draw` calls them exactly where C
writes the macro.  The `NS_FACE_*`/`FRAME_*_COLOR` accessors stay C-ABI entries (the port of
src/nsterm.m owns them), so no `struct face`/`struct frame` mirror is invented for them.

**`struct glyph_string` is mirrored from a measurement, not from the owner.**  The owner mirror
exists but its own comment records that the record is *not* layout-asserted (this target has no
window-system configuration to measure it with), so the lane measured the windowed form with the
same clang probe the other mirrors use and wrote the numbers into `## Evidence`; the storage
unit of the eight-bit run is the byte at 120 with `padding_p`/`for_overlaps` accessors.

**The CoreGraphics C setters are the SDK's own Swift replacements.**  `CGContextSetFillColorWithColor`
-> `context.setFillColor(_:)`, `CGContextSetStrokeColorWithColor` -> `.setStrokeColor(_:)`,
`CGContextSaveGState`/`RestoreGState` -> `.saveGState()`/`.restoreGState()`,
`CGContextFillRects` -> `.fill([rect])`, `CGContextScaleCTM` -> `.scaleBy(x:y:)`,
`CGContextSetTextDrawingMode` -> `.setTextDrawingMode(_:)`, `CGContextSetLineWidth` ->
`.setLineWidth(_:)`, `CGContextSetShouldAntialias` -> `.setShouldAntialias(_:)`,
`CGContextSetShouldSmoothFonts` -> `.setShouldSmoothFonts(_:)`, `CGContextSetTextMatrix` ->
`.textMatrix`, `CGContextSetTextPosition` -> `.textPosition`, `CGContextSetFont` ->
`.setFont(_:)`, `CGContextSetFontSize` -> `.setFontSize(_:)`,
`CGContextShowGlyphsAtPositions` -> `.showGlyphs(_:at:)` (which takes arrays).  Each is marked
obsoleted in Swift; the port uses the replacement, which is the same CG call.

**The packed UVS records, and the SDK records Swift does not import.**  `#pragma pack(push, 1)`
means Swift cannot place a `UInt32` at an odd offset, so the UVS records' 4-byte and 2-byte
members are byte tuples and every read goes through the `BUINT*_VALUE` helpers, which take the
member's address - exactly what C's macros do - with the three packed offsets of
`variation_selector_record` named as `RECORD_*` constants.  `sfntCMapEncoding`/
`sfntCMapHeader` and their `sizeof_*` enumerators are in CoreText's `SFNTTypes.h`, which the
Swift module does not export, so this lane mirrors them like any other SDK-side construct
(`mac_glyph_layout` is the same case).

**The UVS table is Swift data, generated from the C source (rc 3.166, decision 1).**  The
Adobe-Japan1 UVS table is 73,655 bytes; the port carries it as a `fileprivate let [UInt8]`
inside this file, between the `// gen-uvs-swift: begin`/`end` markers, written by
`tools/gen-uvs-swift.py` from `emacs/src/macuvs.h`.  No runtime file read and no resource
bundle.  The fragment is 18 literal chunks joined by `+`: one 73,655-element literal is what
the Swift type checker cannot do in reasonable time, and the chunks are the same values in the
same order.  `macfont_get_uvs_table` hands the array to `CFDataCreateWithBytesNoCopy` with
`kCFAllocatorNull`, C's own call, so nothing copies or owns the bytes.

**`[NSTextStorage setFont:]` has no macOS implementation, so the port uses the public API
(rc 3.166, decision 2) - a recorded deviation.**  AppKit declares no
`-[NSTextStorage setFont:]`; the reference compiles because an Objective-C message send is not
checked against the interface, and on macOS the selector is not recognised at runtime, so C's
own call is a no-op/crash there.  The port performs the same effect with
`addAttribute(.font, value: font, range: NSRange (location: 0, length: textStorage.length))` in
a fileprivate `setFont (_:_:)` (macfont.m 295 and 349; nsfont.swift's 1254 has the same
treatment, so the lane has one spelling).  No symbol provider is invented: the alternatives
were a thunk nobody defines or linking a C object, and the lane does neither.

**C's manual CoreFoundation bookkeeping is ARC's, except where it crosses into Lisp.**  Every
`CFRelease` of a `Create`/`Copy` result is dropped (Swift owns those), as are the `CFRetain`
+1s handed to another Swift value (`macfont_copy_available_families_cache` 1115,
`macfont_create_family_with_symbol` 1152, `mac_font_descriptor_get_adjusted_weight`'s
`font`/`name`, `mac_font_copy_uvs_table`'s `cmap_table`, `mac_font_equal_in_postscript_name`'s
two names, `macfont_get_uvs_table`'s `CFRetain` of the fallback table).  Three retains **stay**,
because the pointer they protect lives in a Lisp object or a cache field this lane cannot make
ARC see: `macfont_set_family_cache` (1054) `make_mint_ptr (CFRetain (string))`,
`macfont_descriptor_entity`'s `make_mint_ptr ((void *) name)` (966), and their release sides
`macfont_invalidate_family_cache` (1004-1010) and `macfont_free_entity` (2656) - the port
spells them `Unmanaged.passRetained (...).toOpaque ()` /
`Unmanaged<CFString>.fromOpaque (...).release ()`.

**The `NSTextStorage`/`NSLayoutManager` compositions are the SDK's own.**  C's alloc/init +
release pairs are ARC's, the objects are `let`s, and the
`if (!(textStorage && layoutManager && textContainer))` checks have no Swift spelling (the
SDK's initialisers do not return nil); they are recorded, not transcribed.

**Mechanical spellings.**  `CFSTR ("…")` -> `"…" as CFString`; `kCFStringEncoding*` ->
`CFStringBuiltInEncodings.*.rawValue`; a CFString key for `CFDictionary*` ->
`UnsafeRawPointer(Unmanaged.passUnretained (key).toOpaque ())` (C's implicit `const void *`),
and a value read back with `Unmanaged<CFNumber>.fromOpaque (p).takeUnretainedValue ()`;
`[font screenFont]` -> `.screen`, `[font printerFont]` -> `.printer` (the Swift 3 renames);
`(NSFont *) font` on a `CTFontRef` is the unwrap (toll-free bridged); `CGRectGetMinX`/
`CGRectIntegral`/`CGRectInset`/`CGPointApplyAffineTransform` -> `bounds.minX`/`.integral`/
`.insetBy (dx:dy:)`/`.applying (_:)`; `CGFLOAT_MAX` -> `CGFloat.greatestFiniteMagnitude`;
`lround`'s `long` takes an explicit `Int32 (...)` at the bit-field stores; `CGColorCreate` ->
`CGColor (colorSpace:components:)`; `Boolean` -> `DarwinBoolean` at the two CF calls that want
one; `kCFNull` compares with `===`; `NSCharacterCollection`'s cases are
`.identityMappingCharacterCollection`/`.adobeJapan1CharacterCollection`;
`CTFontTableOptions.noOptions` -> `[]`; `strlen`/`lround`/`round`/`strdup` are Darwin's;
`CFNotificationSuspensionBehavior.coalesce` is `CFNotificationSuspensionBehaviorCoalesce`;
`goto finish` (3301-3408) and `goto` in general are labelled `do` blocks with `break`, so every
error path leaves through C's own exit.

**Two recorded-not-transcribed items.**  The two non-nil composition checks above, and
`macfont_init_font_change_handler`'s function-local `static bool xinitialized` (1093) - Swift
has no function-local statics, so it is a fileprivate global, which preserves C's lifetime
(the shape nsfont.swift uses for `scriptToFamilies`).

**Round 5's decisions (the OpenType layer and the two spec entry points).**

* **`OpenTypeSpec` and `OTF_SYM_TAG`.**  C's `#define OTF_SYM_TAG (SYM, TAG)` assigns into its
  second argument; the port returns the tag (`fileprivate func OTF_SYM_TAG (_ sym) -> UInt32`)
  and assigns at the call sites, the shape the round-1 macro ports use.  `features[i]` is
  allocated with **libc `malloc`** and freed with **libc `free`**, exactly as C does there -
  not this lane's `xmalloc`/`xfree`, because the function is written to tolerate allocation
  failure and answer NULL.  C's bound `min (PTRDIFF_MAX, SIZE_MAX) / sizeof (int) < len` is
  `Int.max / MemoryLayout<UInt32>.size < len` on this target (PTRDIFF_MAX is the smaller of the
  two), and C's `int j` counter becomes the loop's own `j` (the port keeps `j`, `i` and
  `negative` as C declares them).
* **The second anonymous numeric-traits table** (`<anon@1962>`) is a fileprivate struct,
  `macfont_spec_numeric_trait`, the shape the other table's `<anon@855>` port already uses.  The
  two tables hold the same `{a, b}` literals but their walks read **opposite** components: the
  descriptor loop searches `point->x` and interpolates `point->y`, the spec loop searches
  `point->y` and interpolates `point->x`.  The port keeps each walk's own component and records
  the asymmetry here rather than "fixing" either one.  C's `points[12]` zero fill past the
  listed entries is unreachable (the `CGFLOAT_MAX` row stops the walk), so the port lists the
  reference's own entries - the same reading the other table's port recorded.
* **`macfont_create_attributes_with_spec`.**  C's `goto err` is a labelled `do` with `break err`
  (the label the file already uses elsewhere); the `err` arm drops `attributes` and falls into
  the same tail, which is why both paths leave through the one `return attributes`.  Every
  `CFRelease` is ARC's - including the `charset`/`charset_string` pair C releases only when
  `cf_charset_idx < 0` (there they are this function's own `Create` results).  C's odd
  `if (! string || !cs) { if (string) CFRelease (string); else if (cs) CFRelease (cs); goto err; }`
  is kept as the plain `if string == nil || cs == nil { break err }`: the `else if` can never
  run (both being non-NULL is the only case that gets there), and ARC owns both objects.
  `CFNumberCreate` uses `CFNumberType.cgFloatType`, C's `kCFNumberCGFloatType`.
* **`macfont_list`'s mask arithmetic.**  `CTFontSymbolicTraits` is Swift's `OptionSet` and the
  stdlib gives it no `&`/`^`/`|`/`~` operators (they resolve against `BinaryInteger`/`SIMD` and
  fail), so the port spells C's operators as the `SetAlgebra` methods of the same meaning
  (`symmetricDifference`, `intersection`, `subtracting`, `subtract`, `formUnion`, `union`,
  `isEmpty`) - the shape `macfont_traits_distance` above already uses.  The three nested
  `mmask`/`bmask`/`imask` loops need C's `<=` and `+=` on the traits value, which an `OptionSet`
  does not have either, so those three counters run on the raw `UInt32` values of C's
  `CTFontSymbolicTraits` (a `uint32_t` in C) and the loop's `imask | bmask | mmask` result is
  converted back once with `CTFontSymbolicTraits (rawValue:)` - a pure type reinterpretation at
  the point C's own operators sit.
* **`traits_array` stores the value as a pointer.**  C writes
  `CFArrayAppendValue (traits_array, (const void *) (uintptr_t) sym_traits)`, so the port
  appends `UnsafeRawPointer (bitPattern: UInt (sym_traits.rawValue))` and reads it back with
  `UInt (bitPattern: CFArrayGetValueAtIndex (…))` (the reader `macfont_closest_traits_index_p`
  already ported): a zero traits value stores NULL and reads back as 0, exactly as C's cast
  does.
* **`CFDictionaryGetValue`/`CTFontDescriptorCopyAttribute` results** are converted where C
  implicitly casts: a pointer result with
  `Unmanaged<T>.fromOpaque (…).takeUnretainedValue ()`, an object result with `as! T`.  The
  Swift reference taken that way is where C's `CFRetain (charset)` / `CFRetain (languages)` went.
  The `as!` casts that C writes inline are hoisted into a `let` where the result is passed
  straight to a C function whose parameter Swift imports as an optional (that form draws
  "treating a forced downcast as optional will never produce 'nil'", and this unit is a
  0-warning gate).
* **`CTRunGetAttributes` has no NULL result in Swift** (src/macfont.m 3681-3682): the SDK imports
  it as a non-optional `CFDictionary`, so C's `CFDictionaryRef attributes = CTRunGetAttributes
  (run); if (attributes == NULL) break;` keeps only the loop's body - C returns
  NULL there only for a NULL `run`, and every `run` comes from a live array element.  Recorded
  as a deviation the SDK's own import forces.
* **`CFSTR ("zh-Hans")` / `CFSTR ("zh")`** are spelled `"zh-Hans" as CFString` / `"zh" as
  CFString` (the file's existing spelling for `CFSTR` literals); `CFEqual` compares by value, so
  the test is the reference's own.  In `mac_font_create_preferred_family_for_attributes` the
  `MAC_OS_X_VERSION_MIN_REQUIRED >= 1090` arm is taken (3626-3630), so `keys[0]` is
  `kCTLanguageAttributeName` and the dead `CTGetCoreTextVersion ()` /
  `mac_font_copy_default_name_for_charset_and_languages` branch (3639-3648) is not written (the
  `result != NULL` early exit that branch needs is not either).
* **A finding this round settled**: `mac_font_create_preferred_family_for_attributes` is the
  function that contains the `MAC_OS_X_VERSION_MIN_REQUIRED < 1090` arm, and its dead branch
  (3639-3648) is the **only** caller of `mac_font_copy_default_name_for_charset_and_languages`
  (4162), which is in turn the only caller of
  `mac_font_copy_default_descriptors_for_language` (4070).  So on this target those two are not
  merely unported but unreferenced, and the manifest rows' `permanent` reason now has the
  stronger reading `## Deferred` records.
* **`macfont_match`'s literal `0`** for `synth_sym_traits` is
  `CTFontSymbolicTraits (rawValue: 0)`, and `FONT_ADD_LOG`'s action string is passed through
  `withCString` (the entry takes the `const char *` C's macro passes).

**Round 6's decisions (the rest of the driver core, rc 3.190's 3a cut).**

* **`font_build_object` and `VECSIZE`.**  C's `VECSIZE (struct macfont_info)` is the macro over
  the record's `sizeof`, and this record's C `sizeof` is **264** - its Swift mirror's *stride*
  (`MemoryLayout.size` is 262, because Swift's `size` omits the record's four trailing padding
  bytes; the `## Types` row records both numbers).  The port therefore passes
  `MemoryLayout<macfont_info>.stride`, which is C's number, and `VECSIZE` is the core's
  `const fn` entry (`nsfont.swift` declares the same one).  The C parameter types are kept:
  `Int32` for `int vectorsize`, `Double` for `double pixelsize`.
* **`font->driver = &macfont_driver` is a call to this file's own getter entry.**  The record
  is a C data symbol Swift cannot emit, so the lane exports it as a function of its own C name
  (rc 3.166, decision 2 - the `nsfont_driver` shape).  Every `&macfont_driver` site in C
  becomes a call to that entry, and this one is declared `@_silgen_name("macfont_driver")`
  right above `macfont_list_family` (the ledger's `## ABI` (2)f).  The closure round defines it
  `@_cdecl("macfont_driver")`, so until then the object has an undefined `_macfont_driver` -
  C's own forward reference, unresolved until the record below it.  The record is not written
  here because its initialiser names `macfont_shape` (the 3b round) and
  `macfont_open`/`macfont_text_extents`; a record with a `nil` slot would be a stub.
* **`macfont_close`'s releases.**  `CFRelease (macfont_info->macfont)` and
  `CGFontRelease (macfont_info->cgfont)` are the ports' `nil` assignments: the mirror's two
  fields are strong Swift references (ARC owns the `Create`/`Copy` +1 each), and assigning nil
  is the release - the reading `macfont_release_cache` already records for a mirror record's
  CF fields.  `screen_font` is a raw pointer holding the +1 `mac_screen_font_create_with_name`
  handed over, so its release stays explicit (`Unmanaged<NSFont>.fromOpaque (…).release ()`),
  the shape `macfont_free_entity` uses for the other mint-pointer payload.  C's
  `macfont_info->metrics` loop keeps its two `xfree`s and the final `cache = NULL`.
* **`macfont_text_extents`'s two call sites.**  C writes `metrics ? &m : nil` as the third
  argument; Swift allows `&m` only in an argument position, so the port spells the ternary as
  its two branches (each branch's call is C's own).  `struct font_metrics m` is uninitialised
  in C and read only when `metrics` is non-NULL (the call fills it), so the port gives it the
  record's trivial value.  Two conversions are C's own: `code[i]`'s `unsigned int` -> `CGGlyph`
  is `CGGlyph (truncatingIfNeeded:)`, and each `int` value stored into a `short` field is
  `Int16 (truncatingIfNeeded:)`.
* **`macfont_open`'s three SDK-forced spellings.**  `CTFontCopyFamilyName` imports as a
  **non-optional** `CFString`, so C's `if (family_name)` guard has no Swift spelling (NULL
  comes only from a NULL font, which this function has already ruled out) - the same family as
  round 5's `CTRunGetAttributes`.  `ascent`/`descent`/`leading` are uninitialised locals in C,
  read only after the metrics call fills them; Swift needs a local initialised before `&ascent`
  may appear in an argument position, so they start at the record's trivial value (the one
  return-true path of `mac_screen_font_get_metrics` writes all three).  `(int) [((NSFont *)
  macfont) pointSize]` is the toll-free bridge (`let nsFont: NSFont = macfont!`, the shape
  `mac_font_get_weight` uses) and C's implicit `double` -> `int` truncations are spelled
  `Int32 (…)` (`Int32 (ascent + 0.5)`, `Int32 (-CTFontGetUnderlinePosition (…) + 0.5)`, …).
* **Small spellings this round fixed by the language**: C's discarded
  `macfont_retain_cache (…)` is `_ = macfont_retain_cache (…)`; the five `macfont_info` bit
  fields are written through the port's accessors (`set_synthetic_italic_p`,
  `set_synthetic_bold_p`, `set_spacing`, `set_antialias`, `set_color_bitmap_p`) rather than by
  naming a storage unit; `' '` and `' ' + i` are `UTF32Char (32)` and `UTF32Char (32 + i)`; and
  the cast pointer keeps C's own name (`let macfont_info = …`), the shape
  `macfont_monospace_width_multiplier` established.

**Round 7's decisions (3b: `macfont_shape` and the two CoreText helpers it needs).**

* **The three `alloca` scratch arrays are nested `withUnsafeTemporaryAllocation` closures.**
  C allocates `unichars`, `nonbmp_indices` and `glyph_layouts` on the stack and keeps each one
  alive to the end of the function; Swift's scoped temporary allocation is the same
  stack-first storage (the device `nsfont_shape` established), so the three closures nest with
  the second phase inside the innermost, and each capacity is `max (_, 1)` because C's
  `alloca (0)` never yields NULL while Swift's zero-capacity buffer has no `baseAddress` to
  hand to the SDK.  The pointers live in outer `var`s (`unichars`, `nonbmp_indices`,
  `glyph_layouts`) so every later statement keeps C's own spelling.
* **`RIGHT_TO_LEFT_P` is live text here** (the Macros row records it; this is the file's
  *second* definition of the alias, at 3870 - `mac_font_shape_1` has the first at 433):
  `permutation` is
  `xmalloc`'d for a right-to-left run and `xfree`'d at the end of the run, so every branch
  behind C's alias is a `permutation != nil` test in the port - the same reading
  `mac_font_shape_1`'s port already uses.
* **One Swift-forced initialisation in `macfont_shape`**: C declares `j` uninitialised with
  `i`/`len`/`glyph_len` and gives it its first value inside the loop; Swift requires a variable
  captured by a closure to be initialised before the capture, so `j` starts at the record's
  trivial value (`j = 0`).  The same reason `ascent`/`descent`/`leading` got theirs in round 6.
* **C's discarded call is `_ = macfont_glyph_extents (…)`** in the second phase (C ignores the
  returned advance; only `metrics` is read), and `CALLN (Fvector, xoff, yoff, wadjust)` is the
  stack argument array the `Fmake_hash_table` call already established.
* **`if (string == NULL)` collapses to one path.**  The string is the only thing that can set
  `used`, so C's `unblock_input (); if (used == 0) return Qnil;` on that path always returns
  `Qnil`; the port writes `unblock_input (); return Qnil ()` there and keeps C's own two
  statements in the `string != NULL` branch (they sit after the shape call, before the second
  phase's `block_input`).
* **The `CTRunGetAttributes` deviation repeats** in
  `mac_font_create_line_with_string_and_font`: the SDK imports that copy as non-optional, so
  only C's `font_in_run == NULL` guard is written (the `attributes == NULL` one has no Swift
  spelling).  `CFTypeRef values[]` is spelled as the port's `[UnsafeRawPointer?]` and keeps
  C's own role for `values[1]` (the `CFNumberCreate` result whose NULL is C's test).
* **`mac_font_shape`'s `dir` parameter is unused in C** (the function never reads it; only the
  `mac_screen_font_shape` arm of `macfont_shape` does), so the port keeps the parameter as C
  declares it and writes no reference to it.
* **A finding that corrects round 5's ledger**: `OTF_TAG_STR` (1875-1882) has **no call site
  anywhere in src/macfont.m** - `grep -n OTF_TAG_STR` finds only the definition - so there is
  no reader to transcribe and the port writes none (a Swift copy would be dead code).  The
  Macros row now says so instead of pointing at the two shaping functions, and the same
  re-read corrected `OTF_SYM_TAG`'s range (1869-1873, not 1875-1880).
* **`enum lgstring_direction` stays a Swift raw-value enum** (the Types row records why): it is
  a parameter type of this file's own `fileprivate` functions and never crosses the ABI, so the
  one-byte Swift layout no C compiler ever sees; the "raw integers, never raw-value enums" rule
  guards record mirrors, whose C enum *field* is four bytes.

**Round 8's decisions (the closure: the driver record, its export, and `syms_of_macfont`).**

* **The record's storage and its export.** C's `static struct font_driver macfont_driver` is a
  Swift global (`fileprivate let macfont_driver: UnsafeMutablePointer<font_driver>`) allocated
  once, lazily, at a process-stable address - C's `static` storage, never freed either - and the
  **export** of the C name is the lane's getter, `@_cdecl("macfont_driver") func
  macfont_driver_getter () -> UnsafeMutableRawPointer?` (rc 3.166/3.220, decision (3) item 1;
  `nsfont.swift`'s `T _nsfont_driver` is the same device).  The initialiser keeps C's order and
  C's zeros: `case_sensitive: false` and the nine capability slots are `nil`/false because C's
  designated initialiser does not mention them, and `type: Qmac_ct ()` stands for C's
  `LISPSYM_INITIALLY (Qmac_ct)` (the pdumper hook rewrites it at runtime, as C does).
* **Five slots are one-time pointer reinterpretations.** `close_font`, `encode_char`,
  `text_extents`, `draw` and `get_variation_glyphs` hold Swift functions whose pointer
  parameters `@convention(c)` cannot name in a file that imports AppKit (the rule
  `nsfont.swift`'s record established, rc 3.166 decision 3): each wrapper reinterprets the
  untyped pointer with `assumingMemoryBound (to:)` and calls the typed body.  The other ten
  slots are the Swift functions themselves, their signatures already C-representable.
* **`@_cdecl` cannot take `struct font *`.** `macfont_get_nsctfont`'s parameter is C's
  `struct font *`; the attribute refuses a pointer to a Swift struct (the same constraint), so
  the exported entry takes `UnsafeMutableRawPointer?` and reinterprets it once at the top.  The
  C ABI is unchanged (a pointer is a pointer) and the body keeps the typed name; the four
  `macfont.h` prototypes' other three exports have C-representable parameters as they stand.
* **The two `&macfont_driver` sites keep the getter spelling.** 3a's `macfont_open` (2701) calls
  the getter entry because Swift has no forward declaration (rc 3.220's approval), and the
  closure round writes `mac_register_font_driver` (4217) the same way, so this file has one
  spelling for `&macfont_driver` - the entry the Rust side calls too.
  `nsfont.swift`'s own site references its global directly because there the record and the
  exports landed in one round; the address is the same either way.
* **`DEFSYM` is empty in this tree.**  `#define DEFSYM(sym, name) /* empty */` (src/lisp.h
  2394) is only a marker for make-docfile, so `syms_of_macfont`'s three call sites
  (`Qmac_ct` 4228, `QCdestination` 4236, `QCminspace` 4239) carry no statement; the symbols come
  from the generated globals, and this file's own three are the ones the core must export
  (`## ABI` (2)c).
* **The hook's four core entries keep the shapes the other units use**: `macfont_family_cache =
  Qnil ()` then `staticpro (&macfont_family_cache)` (nsselect's `Vselection_alist` shape),
  `pdumper_do_now_and_after_load (syms_of_macfont_for_pdumper)` with the Swift function as the
  `(@convention(c) () -> Void)?` hook (nsfont's shape), and C's
  `eassert (NILP (macfont_family_cache))` as the disabled-arm `fileprivate func eassert`
  (nsimage/nsmenu/nsselect's shape: the argument must compile, is never evaluated).

## Deferred

3 functions, and the unit is **closed** on them: all three are inside `#if` arms this target
does not take, so no Swift body can ever exist and the manifest keeps their rows with the
`permanent` reason the integrator added.  Nothing is stubbed.

| functions (C line) | why |
| --- | --- |
| `mac_ctfont_get_glyph_for_cid` (3995) | **the `USE_CT_GLYPH_INFO` arm is not taken**; permanent (the manifest row says so) |
| `mac_font_copy_default_descriptors_for_language` (4070), `mac_font_copy_default_name_for_charset_and_languages` (4162) | **the `MAC_OS_X_VERSION_MIN_REQUIRED < 1090` arm is not taken**; permanent.  Both forward declarations (61-62) and both definitions (4070-4204) sit inside that arm, and the pair's only caller is 3645, also inside it - round 5 landed the function that holds that arm, so neither has a live reference on this target |

## ABI

### (1) C symbols this file defines and exports

| C name | C prototype | where | how |
| --- | --- | --- | --- |
| `macfont_update_antialias_threshold` | `void (void)` | 747, declared src/macfont.h 85 | `@_cdecl("macfont_update_antialias_threshold")` - `nm -g` shows `T _macfont_update_antialias_threshold` |
| `macfont_get_nsctfont` | `void *(struct font *)` | 4206, declared src/macfont.h 83 | `@_cdecl("macfont_get_nsctfont")` - `nm -g` shows `T _macfont_get_nsctfont`; the parameter is the untyped pointer (`@_cdecl` cannot name `struct font *`, see `## Decisions`) |
| `mac_register_font_driver` | `void (struct frame *)` | 4215, declared src/macfont.h 82 | `@_cdecl("mac_register_font_driver")` - `T _mac_register_font_driver`; its `&macfont_driver` is a call to the getter below |
| `macfont_driver` | `struct font_driver const` | 1765-1791 | **a C data symbol, exported as a getter of its own name** (decision (3) item 1): `@_cdecl("macfont_driver") func macfont_driver_getter () -> UnsafeMutableRawPointer?` returns the Swift global's process-stable address - `nm -g` shows `T _macfont_driver` (the shape `nsfont.swift`'s `T _nsfont_driver` has) |
| `syms_of_macfont` | `void (void)` | 4225, declared src/macfont.h 87 | `@_cdecl("syms_of_macfont")` - `T _syms_of_macfont` |

`nm -g` on the closed unit's object shows **five** non-mangled exports, all of them C's own
names: `T _macfont_update_antialias_threshold`, `T _macfont_get_nsctfont`,
`T _mac_register_font_driver`, `T _syms_of_macfont` (the four `macfont.h` declares) and
`T _macfont_driver` (the getter above - the one deviation, agreed at rc 3.166/3.220).
`nm -u` no longer carries `_macfont_driver`: 3a's `@_silgen_name` self-reference now resolves
against this round's `@_cdecl` definition in the same object (`## Evidence`).

### (2) C symbols this half calls

**a. Symbols the reference defines out of line**: `xfree` (lisp.h 5543), `xmalloc` (5535),
`xzalloc` (5537), `xrealloc` (5539), `make_uninit_string` (4563), `make_unibyte_string` (4544),
`font_intern_prop` (font.h 885), `font_make_entity` (842), `font_put_extra` (913),
`font_filter_properties` (font.h 953, src/font.c), `Fcons` (globals.h 4560), `Fcopy_sequence`
(4572), `Fmake_hash_table` (the `CALLN` target), `hash_find`/`hash_find_get_hash`/`hash_put`
(lisp.h 4254-4258), `assq_no_quit` (4272), `fast_c_string_match_ignore_case` (lisp.h 4325),
`strlen`/`lround`/`round`/`strdup` (libc).

**b. Header-only constructs** (the core exports each C-ABI entry under the C construct's own
name): `SDATA` (1651), `SBYTES` (1697), `SYMBOL_NAME` (2350), `AREF` (1964), `ASET` (1978),
`BASE_EQ` (393), `HASH_TABLE_P` (2664), `XHASH_TABLE` (2670), `HASH_TABLE_SIZE` (2709),
`HASH_KEY` (2685), `HASH_VALUE` (2693), `hash_unused_entry_key_p` (2658),
`set_hash_value_slot` (3982), `make_mint_ptr` (2893), `mint_ptrp` (2900), `xmint_pointer`
(2906), `MAX_UNICODE_CHAR` (character.h 52), the ten `FONT_*_INDEX` members (font.h 70-190),
`FONT_SPACING_PROPORTIONAL`/`FONT_SPACING_MONO` (408-410), `FONT_SET_STYLE` (236),
`FONT_ENTITY_P` (439), `XFONT_OBJECT` (539), `FONT_INVALID_CODE` (585), `make_fixnum` (1211),
`XCAR`/`XCDR` (455/456), `NILP` (450), `FONT_HEIGHT`/`FONT_BASE` (font.h 244-245),
`DRAW_CURSOR` (dispextern.h 1030), `block_input`/`unblock_input` (src/blockinput.h), the
`SIZEOF_*_UVS_TABLE_HEADER` mirrors' C counterparts (they are this file's own macros, so they
are Swift expressions over `MemoryLayout`), and the `Q*` entries below.

Round 8's closure added `register_font_driver` (font.h 896, the two ends that hand the record
over), `staticpro` (lisp.h 3919, for `macfont_family_cache`), `pdumper_do_now_and_after_load`
(pdumper.h 92-99, the hook `syms_of_macfont` hands over) and `dumped_with_pdumper_p`
(lisp.h 679-681) - the last three the shapes `nsselect.swift`/`nsfont.swift` already declare.

Round 7's segment added `CHECK_FONT_GET_OBJECT` (font.h 553), the sixteen
`LGSTRING_*`/`LGLYPH_*` entries `macfont_shape` reads (composite.h 252-325; the core's
`femacs/src/include/composite.rs` mirrors every one of them as a `pub fn`, so the Swift
declarations are the only ones this lane writes - no second definition), `QL2R`/`QR2L`
(src/font.c 5897-5898), `Fvector` (globals.h, the `CALLN` target) and `memory_full_up`
(lisp.h 4433).

Round 6's segment added `font_build_object` (src/font.h 848, defined src/font.c 250 - the
entry `macfont_open` builds the font object with) and `VECSIZE` (src/lisp.h 2015-2016, the
macro over the record's `sizeof`; the core mirror spells it as a `const fn` over that size,
`femacs/src/include/lisp.rs` 2925, and `nsfont.swift` declares the same entry), plus
`QCdestination`, `QCantialias` and `QCminspace` (see (2)c).

Round 5's segment added the entries the OpenType and spec functions call: `EQ` (lisp.h 1315),
`CONSP` (446), `SYMBOLP` (1075), `FIXNUMP` (449), `XFIXNUM` (1268), `XFIXNAT` (3246),
`RANGED_FIXNUMP` (1262), `CHARACTERP` (src/character.h), `VECTORP` (1581), `ASIZE` (1687),
`CDR` (462) / `CDR_SAFE` (844), `list_length` (4243), `Fnreverse` (globals.h 5161),
`FONT_WEIGHT_NUMERIC` / `FONT_SLANT_NUMERIC` (font.h 193-201) and `FONT_ADD_LOG` (font.h
1004-1006).  Two of them are **not** generated-globals entries: `Votf_script_alist` and
`Vscript_representative_chars` are `DEFVAR_LISP`'d variables (src/font.c), i.e. `globals.h`
fields with no C symbol of their own, so the core exports an entry under each name and the
port imports it as a data symbol (`@_silgen_name (…) var`), the shape `nsimage.swift`'s
`ns_use_srgb_colorspace` already uses.  `FONT_ADD_LOG` is called as C's macro does, with the
action string passed through `withCString`.

**c. Generated-globals entries**: `Qnil`, `Qeq`, `QCtest`, `QCfont_entity`, `Qiso10646_1`,
`QClang`, `QCotf`, `QCscript`, `Qascii_0`, `Qunicode_bmp`, `Qemoji` (all six added in round 5,
and all six present in `docs/baseline/globals/symbols.tsv`, defined by src/font.c), and round
6's `QCantialias` (font.c, in that table), `QCminspace` (font.c, in that table) and
`QCdestination` (**this file's own** `DEFSYM`, absent from the table) - and
**`Qmac_ct`** (src/macfont.m 4228), which is **not** in
`docs/baseline/globals/symbols.tsv`: that table is generated from the frozen build set, which
does not compile macfont.m, so this file's own `DEFSYM`s (`Qmac_ct` 4228, `QCdestination` 4236,
`QCminspace` 4239) have to be exported by the core the same way as the rest.  Reported.

**d. This lane's own header constructs**: `+[NSColor colorWithUnsignedLong:]` (declared
src/nsterm.h 361, defined src/nsterm.m 157 in the `NSColor (EmacsColor)` category) - a *class*
method, so the receiver (the class object) is the entry's first parameter; and
`FRAME_DISPLAY_INFO` (src/nsterm.h 1032, owned by the port of src/nsterm.m) for
`macfont_get_cache`; `NS_FACE_FOREGROUND`/`NS_FACE_BACKGROUND` (nsterm.h 1040-1041),
`FRAME_CURSOR_COLOR`/`FRAME_BACKGROUND_COLOR` (1037-1038), `FRAME_NS_VIEW` (1045) and
`-[NSColor unsignedLong]` (nsterm.h 363, src/nsterm.m 157) for `macfont_draw`;
`ns_antialias_text`/`ns_use_thin_smoothing` (`DEFVAR_LISP`'d by src/nsterm.m 11685-11691)
reached as C-ABI functions of their names (nsselect.md's device); and
`CGContextSetFontSmoothingStyle` (declared src/macfont.h 88, an undocumented CoreGraphics entry).  `-[NSTextStorage setFont:]` is **not** an entry any more: the port
performs its effect with the public API (decision 2).

**e. This file's own variables**: `macfont_antialias_threshold` (744, filled by the exported
`macfont_update_antialias_threshold`), `macfont_family_cache` (993, a `DEFVAR`-time Lisp value
set by `syms_of_macfont` next round), `macfont_available_families_cache` (1068),
`macfont_cache_dictionary` (1403) - all Swift globals, because Swift cannot emit C data
symbols; `cf_charset_table` (681) is the data table recorded above.

**f. This file's own `macfont_driver` (the getter route of (3) item 1)**: C writes
`&macfont_driver` twice inside this file (`macfont_open` 2701 and `mac_register_font_driver`
4217) and src/nsfns.m writes it twice more.  The lane exports the record as a function of its
own C name, so every one of those sites is a **call to that entry**: this file declares it
`@_silgen_name("macfont_driver") func macfont_driver_address () -> UnsafeMutableRawPointer?`
and the closure round defines it `@_cdecl("macfont_driver")`.  Until that definition lands the
object carries an undefined `_macfont_driver` (the closure round's own check: it must
disappear from `nm -u`), which is exactly C's forward reference, unresolved until the record's
definition below `macfont_open`.

### (3) What the lane still has to settle (flagged for the integrator)

1. **`macfont_driver` took the getter route** (settled): the lane adopted it at rc 3.166,
   decision 2 (`nsfont_driver` is already so exported - `@_cdecl("nsfont_driver")` in
   nsfont.swift, `nm -g` shows `T _nsfont_driver`), and the closure round landed it: a
   `@_cdecl("macfont_driver")` function returning the record's process-stable address, which
   the Rust side calls where C writes `register_font_driver (&macfont_driver, f)` and src/nsfns.m
   writes its two `&macfont_driver` sites.  This file's two sites (2701, 4217) call the same
   entry.
2. **The UVS tables are settled** by rc 3.166, decision 1 and this round's implementation: the
   73,655 bytes are Swift source, generated by `tools/gen-uvs-swift.py`; 1705/3301/3429/3520 are
   ported, so nothing waits on a data decision any more.
3. **Three `docs/baseline/swift-units.tsv` rows can never be cleared** by a Swift func:
   `mac_ctfont_get_glyph_for_cid` (3995) and the two 4070/4162 helpers are in arms this target
   does not take (the rows now carry the integrator's `reason` column saying so).
4. **`cf_charset_table` (681) is invisible to the extractor** (the `name[] =` array form);
   recorded in the ledger's File-scope objects and reported as a tool blind spot.  One more such
   blind spot surfaced this round: `<anon>` **packed records inside `#pragma pack`** are listed
   by the inventory under `Types and their fields` only by their member names, which is why the
   port measures them itself.

**The `--write-manifest` note is closed**: rc 477434c fixed
`python3 tools/check-swift-units.py --write-manifest` to read back and preserve the
integrator's fourth (`reason`) column, so this round's removal of the seven landed rows used
the writer directly and was verified by diffing against a copy (the only change was the seven
rows leaving, with the three permanent rows' `reason` text intact).

## Evidence

- `swiftc --version` -> `swift-driver version: 1.168.6 Apple Swift version 6.4
  (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`, `Target: arm64-apple-macosx26.0`.
- `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/macfont.o femacs/platform/macos/macfont.swift` -> **exit 0**, no warnings, no
  `-swift-version` flag (swiftc 6.4's default language mode is 5).
- `nm -g /tmp/macfont.o` -> one non-mangled defined symbol, `T _macfont_update_antialias_threshold`
  (every other `T`/`D`/`S` row is Swift runtime or `$s`-mangled); `nm -u` lists 379 undefined
  entries (the `@_silgen_name` imports and Swift's own).
- The C side of every mirror, measured with the windowed declarations this file's target
  implies: `/opt/homebrew/opt/llvm/bin/clang -std=gnu23 -DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1
  -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib` (Homebrew clang
  23.1.1) on a probe that copies the file's own declarations and includes `<CoreText/CoreText.h>`,
  `<config.h>`, `"lisp.h"`, `"font.h"`:
  - `struct mac_glyph_layout` -> `sizeof` 56, `comp_range`@0, `string_index`@16,
    `advance_delta`@24, `baseline_delta`@32, `advance`@40, `glyph_id`@48
  - `struct macfont_metrics` -> `sizeof` 8, align 4, the offsets of `## Types`
  - `struct macfont_info` -> `sizeof` 264, `font`@0, `macfont`@216, `cgfont`@224,
    `screen_font`@232, `cache`@240, `metrics`@248, `metrics_nrows`@256; a byte dump with
    `synthetic_italic_p = 1` / `spacing = 3` / `antialias = 3 + color_bitmap_p = 1` shows the
    4-byte bit-field unit at **258** with the members at bits 0, 1, 2-3, 4-5 and 6
  - `struct macfont_cache` -> `sizeof` 312, `reference_count`@0, `cf_charset`@8,
    `row_nkeys_or_perm`@16, `matrix`@272, `nrows`@280, `dictionary`@288, `table`@296,
    `collection`@304
  - the local `numeric_traits` struct -> `sizeof` 216, `index`@0, `trait`@8, `points`@16,
    `adjust_func`@208
- The same records, Swift side (`swiftc -o … femacs/platform/macos/macfont.swift <a main that
  prints MemoryLayout> -Xlinker -undefined -Xlinker dynamic_lookup`, then the program):
  `mac_glyph_layout` size 50/stride 56 with all six offsets = C; `macfont_metrics` size 8
  (align 2) with all six offsets = C; `macfont_info` size 262/stride 264 with every offset = C
  (the bit-field unit at 258); `macfont_cache` size/stride 312 with every offset = C;
  `row_nkeys_or_perm_256` = 256 bytes; `font` 216 with `props`@8/`driver`@208; `font_metrics`
  10/stride 10.
- `python3 tools/check-cond-eval.py src/macfont.m` -> `23 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (the disagreements are `MAC_OS_X_VERSION_MAX_ALLOWED >= …`
  and `__LP64__`, which the frozen clang probe sees as undefined macros; `src/macfont.m` is not
  in `docs/baseline/config/cond-eval-verified.txt`), and
  `grep -nE "^\s*#\s*(if|ifdef|ifndef)" emacs/src/macfont.m` -> the 23 conditions of the
  Decisions table.
- `grep -rn "USE_CT_GLYPH_INFO" emacs/src/*.h emacs/src/*.m emacs/configure.ac
  emacs/src/Makefile.in` -> only the four uses in macfont.h/macfont.m; nothing defines it.
- `python3 tools/extract-abi.py emacs/src/macfont.m` -> 0 subrs, 74 functions, 26 macros,
  12 types, 6 file-scope objects (`cf_charset_table` absent - the blind spot).
- `python3 tools/check-swift-units.py` -> `femacs/platform/macos/macfont.swift: 74 C
  definition(s), 39 unported, 39 row(s); 54 C symbol import(s)` and `0 new, 0 stale, 4 Swift
  unit(s) landed` (this unit's 39 rows were in `docs/baseline/swift-units.tsv` when the gate
  ran - a `--write-manifest` run outside this lane wrote them).
- Round 2's additions, same commands: `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, no warnings, with the 73,655-byte
  generated data block in the file (`wc -l` 8,794 after the round's last edit); `nm -g` still
  shows only `T _macfont_update_antialias_threshold` (the round added no export - the driver
  record's getter comes with the entry points); `nm -u` 426 entries.
- `python3 tools/gen-uvs-swift.py --write` -> `gen-uvs-swift --write: macfont.swift <-
  emacs/src/macuvs.h (73655 values, 18 chunks)`, and `python3 tools/gen-uvs-swift.py --check` ->
  `the block in macfont.swift matches src/macuvs.h (73655 values)`.  The generated block is
  `// gen-uvs-swift: begin` … `// gen-uvs-swift: end`.
- Round 4's readings: `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, no warnings; `nm -g` unchanged (only
  `T _macfont_update_antialias_threshold`).
- **The `struct glyph_string` measurement** (`clang -std=gnu23 -DHAVE_WINDOW_SYSTEM=1
  -DHAVE_NS=1 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib`):
  `sizeof` **272**, align 8; `x`@0, `y`@4, `ybase`@8, `width`@12, `background_width`@16,
  `height`@20, `left_overhang`@24, `right_overhang`@28, `f`@32, `w`@40, `row`@48, `area`@56,
  `char2b`@64, `nchars`@72, `hl`@76, `face`@80, `font`@88, `cmp`@96, `cmp_id`@104,
  `cmp_from`@112, `cmp_to`@116, the eight-bit run at **120**, `first_glyph`@128, `img`@136,
  `xwidget`@144, `slice`@152, `clip_head`@160, `clip_tail`@168, `clip`@176,
  `num_clips`@240, `underline_position`@244, `underline_thickness`@248, `next`@256,
  `prev`@264; a byte dump with `padding_p = 1` / `for_overlaps = 3` /
  `extends_to_end_of_line_p = 1` gives `120: 0x80 / 0x30 / 0x01`, which is the bit order the
  accessors use (`NativeRectangle` is `NSRect`, 32 bytes; `struct glyph_slice` 8).
- `python3 tools/check-swift-units.py` -> `macfont.swift: 74 C definition(s), 21 unported, 21
  row(s); 71 C symbol import(s)` and `0 new, 0 stale, 4 Swift unit(s) landed`.  The 16 rows of
  round 2's landed functions, and round 3's `macfont_get_charset` row, were removed with the
  tool's `--write-manifest` (the writer now preserves the integrator's `reason` column, rc
  477434c - verified by diffing: the only change is the landed row leaving).
- Round 3's readings: `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, no warnings; the file is 8,942 lines at
  the reading; `nm -g` unchanged (only `T _macfont_update_antialias_threshold`); `nm -u` 434.
- **Groundwork for the next round, measured**: `struct glyph_string` under this target's
  windowed configuration (`clang -std=gnu23 -DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1 -I
  build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib`) -> `sizeof` **272**,
  align 8; `x`@0, `y`@4, `ybase`@8, `width`@12, `background_width`@16, `height`@20,
  `left_overhang`@24, `right_overhang`@28, `f`@32, `w`@40, `row`@48, `area`@56, `char2b`@64,
  `nchars`@72, `hl`@76, `face`@80, `font`@88, `cmp`@96, `cmp_id`@104, `cmp_from`@112,
  `cmp_to`@116, `first_glyph`@128, `img`@136, `xwidget`@144, `clip_head`@160, `clip_tail`@168
  (the platform `gc` blocks contribute no member on this target).
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0
  failures`.
- `emacs/` untouched; the probe files under `/tmp/mflay` and `/tmp/macfont.o` were deleted
  after the readings.
- Round 5's readings: `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, 0 warnings, no `-swift-version` flag;
  `wc -l` on `femacs/platform/macos/macfont.swift` -> **10,266** lines at the reading;
  `nm -g /tmp/macfont.o` unchanged - exactly one non-mangled `T`, `_macfont_update_antialias_threshold`
  (the round's seven functions are `fileprivate`, so it adds no export); `nm -u` -> 520 entries,
  including the round's new entries `_EQ`, `_CONSP`, `_SYMBOLP`, `_FIXNUMP`, `_XFIXNUM`,
  `_XFIXNAT`, `_RANGED_FIXNUMP`, `_CHARACTERP`, `_VECTORP`, `_ASIZE`, `_CDR`, `_CDR_SAFE`,
  `_list_length`, `_Fnreverse`, `_FONT_WEIGHT_NUMERIC`, `_FONT_SLANT_NUMERIC`, `_FONT_ADD_LOG`,
  `_QClang`, `_QCotf`, `_QCscript`, `_Qascii_0`, `_Qunicode_bmp`, `_Qemoji`, and the two data
  symbols `_Votf_script_alist` / `_Vscript_representative_chars`.
- `python3 tools/check-swift-units.py` -> `femacs/platform/macos/macfont.swift: 74 C
  definition(s), 14 unported, 14 row(s); 100 C symbol import(s)` and `0 new, 0 stale, 4 Swift
  unit(s) landed`; the seven landed rows were removed with the writer
  (`python3 tools/check-swift-units.py --write-manifest`), diffed against a copy first.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0
  failures`.
- `emacs/` untouched; `/tmp` probe files removed after the readings.
- Round 6's readings: `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, 0 warnings (the compile's stdout and
  stderr are both empty), no `-swift-version` flag; `wc -l` -> **10,596** lines; `nm -g /tmp/macfont.o`
  -> still exactly one non-mangled `T`, `_macfont_update_antialias_threshold`; `nm -u` -> 535
  entries, the round's new ones being `_font_build_object`, `_VECSIZE`, `_QCdestination`,
  `_QCantialias`, `_QCminspace` and **`_macfont_driver`** (the 3a self-reference of `## ABI`
  (2)f, which the closure round's `@_cdecl("macfont_driver")` must resolve).
- `python3 tools/check-swift-units.py` -> `femacs/platform/macos/macfont.swift: 74 C
  definition(s), 10 unported, 10 row(s); 106 C symbol import(s)` and `0 new, 0 stale, 4 Swift
  unit(s) landed`; the four landed rows were removed with
  `python3 tools/check-swift-units.py --write-manifest` (12 rows), diffed against a copy first
  (the only change was the four rows leaving, the three `reason` texts intact).
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0
  failures`; `python3 tools/gen-uvs-swift.py --check` -> `the block in macfont.swift matches
  src/macuvs.h (73655 values)`; `python3 tools/check-globals.py` -> `283 units, 0 DEFVAR_*
  variables without a definition in their own unit`.
- Round 7's readings: `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, 0 warnings (stdout and stderr both
  empty), no `-swift-version` flag; `wc -l` -> **11,069** lines; `nm -g /tmp/macfont.o` -> still
  exactly one non-mangled `T`, `_macfont_update_antialias_threshold`; `nm -u` -> 568 entries, of
  which 16 are the round's `LGSTRING_*`/`LGLYPH_*` imports (plus `_CHECK_FONT_GET_OBJECT`,
  `_QL2R`, `_QR2L`, `_Fvector`, `_memory_full_up`, and 3a's `_macfont_driver`, still unresolved
  until the closure round).
- `python3 tools/check-swift-units.py` -> `femacs/platform/macos/macfont.swift: 74 C
  definition(s), 7 unported, 7 row(s); 127 C symbol import(s)` and `0 new, 0 stale, 7 Swift
  unit(s) landed` (the tool now sees seven units).  **Cross-lane note**: the tree gained
  `femacs/platform/macos/nsterm.swift` *during* this round (142 C definitions), so the
  sanctioned writer wrote its 140 rows into the shared manifest along with the removal of this
  round's three; the earlier backup already carried another lane's 21 `nsfns.swift` rows, so
  whole-tree refreshes by the writer are what the shared file has been getting.  The 140 nsterm
  rows are the writer's deterministic output, not hand-written here; reverting that part is a
  writer run or a restore of the previous manifest plus a hand removal of this round's three.
  The shared file churned again before this reading: a concurrent write put this round's three
  rows **back**, the gate reported `0 new, 3 stale`, and the writer was run a second time - the
  `0 new, 0 stale` above is that final state (170 rows: 7 macfont + 21 nsfns + 2 nsfont + 140
  nsterm).
- `python3 tools/gen-uvs-swift.py --check` -> `the block in macfont.swift matches
  src/macuvs.h (73655 values)`; `python3 tools/check-declarations.py` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `python3 tools/check-verbatim.py` -> `6
  trees, 4607 files, 0 failures`.
- Round 8's readings (**the closure**): `swiftc -parse-as-library -c -o /tmp/macfont.o
  femacs/platform/macos/macfont.swift` -> **exit 0**, 0 warnings (stdout and stderr both
  empty), no `-swift-version` flag; `wc -l` -> **11,313** lines.
- **The acceptance check rc 3.220 attached to the same-file `@_silgen_name` self-reference**:
  `nm -u /tmp/macfont.o | grep -E '_macfont_driver|_font_build_object|_VECSIZE|_QC'` ->
  `_QCantialias _QCdestination _QCfont_entity _QClang _QCminspace _QCotf _QCscript _QCtest
  _VECSIZE _font_build_object` - **`_macfont_driver` is gone**: 3a's self-reference now
  resolves against this round's `@_cdecl ("macfont_driver")` in the same object.
- `nm -g /tmp/macfont.o | grep -E ' T _[a-zA-Z]'` -> exactly five non-mangled exports, all of
  them C's own names: `_mac_register_font_driver`, `_macfont_driver`, `_macfont_get_nsctfont`,
  `_macfont_update_antialias_threshold`, `_syms_of_macfont`.  `nm -u` -> 572 entries.
- `python3 tools/check-swift-units.py` -> `femacs/platform/macos/macfont.swift: 74 C
  definition(s), **3 unported, 3 row(s)**; 131 C symbol import(s)` and `0 new, 0 stale, 7 Swift
  unit(s) landed`; the writer removed this round's four rows (162 rows total), diffed against a
  copy first.  The three rows left are the unit's permanent ones (`mac_ctfont_get_glyph_for_cid`,
  the two `< 1090` helpers).  **Shared-file note**: the same writer run also removed four
  `nsterm.swift` rows (`append2`, `hold_event`, `ns_finish_events`, `ns_init_events`) - that
  lane's functions landed in its in-flight file between this round's start and the run, so its
  rows had become stale; the writer's whole-tree output is the deterministic answer for the
  current tree and the only change to the file is those eight deletions (this round's four plus
  theirs).
- `python3 tools/gen-uvs-swift.py --check` -> `the block in macfont.swift matches
  src/macuvs.h (73655 values)`; `python3 tools/check-declarations.py` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `python3 tools/check-verbatim.py` -> `6
  trees, 4607 files, 0 failures`.
- `emacs/` untouched; the round's `/tmp` probe files were removed after the readings.

