// femacs: port of src/nsfont.m @ fd7afced403.
//
// **This file is the GNUstep font backend.**  `configure.ac` 2816 sets
// `ns_fontfile=nsfont.o` for the GNUstep configuration, while 2762 sets
// `ns_fontfile=macfont.o` for the Cocoa one: the two files are build alternatives, they both
// define a `font_driver` of their own, and src/nsfont.m has no `#ifdef` of any kind (the only
// conditional in it is the `#if 0` block at 659-662) - it is unconditionally
// `NS_IMPL_GNUSTEP` code, from its first line to the GNUstep-only calls listed in the ledger.
//
// The unit below carries the whole file: the descriptor/entity/charset/script layer
// (lines 35-830), the driver entry points, the glyph and metrics caches, the shaping, the
// `font_driver` record and `syms_of_nsfont`.  The core records the driver bodies dereference
// are named in Swift where this tree already owns a mirror of the C record - `struct font`
// (femacs/src/include/font.rs), `struct font_metrics`/`struct font_driver`/`struct Lisp_Fwd`
// (femacs/src/font.rs) - and the records that have none are named in
// `docs/ports/nsfont.md`'s `## Deferred` rather than invented here: `struct glyph_string`,
// which this tree declares only as the opaque P-70 placeholder `femacs/src/font.rs` 739
// because font.c never dereferences it, is what keeps `nsfont_draw` and
// `ns_dump_glyphstring` out of this file.
//
// The two halves share the file's own types, enums and macros, which are all here.
//
// GNUstep-only entries (there is no Cocoa arm in this file to take instead): the
// `GSFontEnumerator` class, `-[NSMutableAttributedString setFont:]` and the DPS drawing calls
// are declared as C-ABI entries of their own names; their definitions live in the GNUstep
// libraries, not in this repository.

import Foundation
import AppKit

// ==========================================================================
// Lane ABI: C types.
// ==========================================================================

/// C: `typedef struct Lisp_X *Lisp_Word; typedef Lisp_Word Lisp_Object;` (src/lisp.h
/// 311-323).  The pointee is incomplete in C too and a Swift pointer to a Swift type cannot
/// be a `@_cdecl` parameter, so the lane keeps the pointer alone (docs/ports/nsimage.md).
typealias Lisp_Object = UnsafeMutableRawPointer?

/// C: `union vectorlike_header { ptrdiff_t size; }` (src/lisp.h 923-955) - one member.
struct vectorlike_header {
  var size: Int
}

/// C: `struct Lisp_Char_Table`'s prefix (src/lisp.h 2053-2082), which this file reaches
/// through `XCHAR_TABLE (Vchar_script_table)->extras[0]` (src/nsfont.m 690).  The struct's
/// static_asserts (lisp.h 2254-2259) fix `extras`; the flexible `extras` array is the one
/// element the reference reads, so the mirror stops there.  The layout is measured in
/// `## Evidence`.
struct Lisp_Char_Table {
  var header: vectorlike_header
  var defalt: Lisp_Object
  var parent: Lisp_Object
  var purpose: Lisp_Object
  var ascii: Lisp_Object
  /// C: `Lisp_Object contents[(1 << CHARTAB_SIZE_BITS_0)];`
  var contents: contents64
  /// C: `Lisp_Object extras[FLEXIBLE_ARRAY_MEMBER];` - only `extras[0]` is read here.
  var extras: Lisp_Object
}
/// C: `Lisp_Object contents[(1 << CHARTAB_SIZE_BITS_0)];` - a fixed-size array is not a
/// Swift value type, so the 64 slots are 64 fields (`CHARTAB_SIZE_BITS_0` is 6); the probe
/// proves the array's size and `extras`'s offset.
struct contents64 {
  var v0: Lisp_Object
  var v1: Lisp_Object
  var v2: Lisp_Object
  var v3: Lisp_Object
  var v4: Lisp_Object
  var v5: Lisp_Object
  var v6: Lisp_Object
  var v7: Lisp_Object
  var v8: Lisp_Object
  var v9: Lisp_Object
  var v10: Lisp_Object
  var v11: Lisp_Object
  var v12: Lisp_Object
  var v13: Lisp_Object
  var v14: Lisp_Object
  var v15: Lisp_Object
  var v16: Lisp_Object
  var v17: Lisp_Object
  var v18: Lisp_Object
  var v19: Lisp_Object
  var v20: Lisp_Object
  var v21: Lisp_Object
  var v22: Lisp_Object
  var v23: Lisp_Object
  var v24: Lisp_Object
  var v25: Lisp_Object
  var v26: Lisp_Object
  var v27: Lisp_Object
  var v28: Lisp_Object
  var v29: Lisp_Object
  var v30: Lisp_Object
  var v31: Lisp_Object
  var v32: Lisp_Object
  var v33: Lisp_Object
  var v34: Lisp_Object
  var v35: Lisp_Object
  var v36: Lisp_Object
  var v37: Lisp_Object
  var v38: Lisp_Object
  var v39: Lisp_Object
  var v40: Lisp_Object
  var v41: Lisp_Object
  var v42: Lisp_Object
  var v43: Lisp_Object
  var v44: Lisp_Object
  var v45: Lisp_Object
  var v46: Lisp_Object
  var v47: Lisp_Object
  var v48: Lisp_Object
  var v49: Lisp_Object
  var v50: Lisp_Object
  var v51: Lisp_Object
  var v52: Lisp_Object
  var v53: Lisp_Object
  var v54: Lisp_Object
  var v55: Lisp_Object
  var v56: Lisp_Object
  var v57: Lisp_Object
  var v58: Lisp_Object
  var v59: Lisp_Object
  var v60: Lisp_Object
  var v61: Lisp_Object
  var v62: Lisp_Object
  var v63: Lisp_Object
}

/// C: `XCharStruct` (src/nsgui.h 57-64) - `int rbearing, lbearing, width, ascent, descent;`.
/// Only used by the deferred half (`struct nsfont_info`'s `max_bounds`), declared here
/// because it is a type of this file's environment (src/nsgui.h) rather than of the core.
struct XCharStruct {
  var rbearing: Int32
  var lbearing: Int32
  var width: Int32
  var ascent: Int32
  var descent: Int32
}

/// C: `struct ns_glyph_layout` (src/nsfont.m 51-71) - the record `ns_glyph_layout` C defines
/// here; its two users (`ns_font_shape`, src/nsfont.m 1214) are deferred, but the
/// type is this file's own definition.  (`nsfont_shape` is nsfont.m 1464.)
struct ns_glyph_layout {
  /// C: `NSRange comp_range;` - `{location, length}` of `NSUInteger`s.
  var comp_range: NSRange
  var string_index: UInt
  var advance_delta: CGFloat
  var baseline_delta: CGFloat
  var advance: CGFloat
  var glyph_id: NSGlyph
}

/// C: `enum lgstring_direction { DIR_R2L = -1, DIR_UNKNOWN = 0, DIR_L2R = 1 }` (src/nsfont.m
/// 73-77).
enum lgstring_direction: Int32 {
  case DIR_R2L = -1
  case DIR_UNKNOWN = 0
  case DIR_L2R = 1
}

/// C: `enum gs_font_slant` (src/nsfont.m 79-84).
enum gs_font_slant: Int32 {
  case GS_FONT_SLANT_ITALIC = 0
  case GS_FONT_SLANT_REVERSE_ITALIC = 1
  case GS_FONT_SLANT_NORMAL = 2
}

/// C: `enum gs_font_weight` (src/nsfont.m 86-91).
enum gs_font_weight: Int32 {
  case GS_FONT_WEIGHT_LIGHT = 0
  case GS_FONT_WEIGHT_BOLD = 1
  case GS_FONT_WEIGHT_NORMAL = 2
}

/// C: `enum gs_font_width` (src/nsfont.m 93-98).
enum gs_font_width: Int32 {
  case GS_FONT_WIDTH_CONDENSED = 0
  case GS_FONT_WIDTH_EXPANDED = 1
  case GS_FONT_WIDTH_NORMAL = 2
}

/// C: `enum gs_specified` (src/nsfont.m 100-107) - flags, kept as `Int32` constants with
/// C's names because they are combined with `|`.
let GS_SPECIFIED_SLANT: Int32 = 1
let GS_SPECIFIED_WEIGHT: Int32 = 1 << 1
let GS_SPECIFIED_WIDTH: Int32 = 1 << 2
let GS_SPECIFIED_FAMILY: Int32 = 1 << 3
let GS_SPECIFIED_SPACING: Int32 = 1 << 4

/// C: `struct gs_font_data` (src/nsfont.m 109-116).  `char *family_name` is storage this
/// file owns (`xstrdup`), so it stays an `UnsafeMutablePointer<CChar>?`.
struct gs_font_data {
  var specified: Int32
  var slant: gs_font_slant
  var weight: gs_font_weight
  var width: gs_font_width
  var monospace_p: Bool
  var family_name: UnsafeMutablePointer<CChar>?
}

/// C: `struct font` (src/font.h 275-404), the font-object record.  This mirror is the
/// **windowed** form: the eight members C guards with `#if defined (HAVE_WINDOW_SYSTEM)`
/// (font.h 323-397) are present because this file's target (the GNUstep NS build) is a
/// windowed one; the core's own mirror (`femacs/src/include/font.rs`) carries the same record
/// with the frozen configuration's layout (no window system: `sizeof` 184, `driver`@176) and
/// keeps those members under its `window_system` feature, while the numbers here are the
/// windowed layout, measured in `## Evidence`: `sizeof` 216, `props`@8, `min_width`@144,
/// `pixel_size`@152, `driver`@208.  No Swift enum is used for any field (a raw-value enum is
/// one byte - see the lane rule in Decisions).
struct font {
  var header: vectorlike_header
  /// C: `Lisp_Object props[FONT_OBJECT_MAX];` (font.h 284) - 17 slots, the length named by
  /// `FONT_OBJECT_MAX` (the enum at font.h 70-190).
  var props: props17
  var min_width: Int32
  var max_width: Int32
  /// C: `int pixel_size;` (font.h 296).
  var pixel_size: Int32
  var height: Int32
  var space_width: Int32
  var average_width: Int32
  var ascent: Int32
  var descent: Int32
  var underline_thickness: Int32
  var underline_position: Int32
  /// C: `bool vertical_centering;`
  var vertical_centering: Bool
  var baseline_offset: Int32
  var relative_compose: Int32
  var default_ascent: Int32
  var encoding_charset: Int32
  var repertory_charset: Int32
  /// C: `struct font_driver const *driver;` (font.h 400) - the driver record has no owner
  /// mirror yet (see `## Deferred` in the ledger), so the pointer keeps the untyped form with
  /// this line, exactly as the core's `font.rs` does for the same field (PORTING.md P-55/P-72).
  var driver: UnsafeRawPointer?
}

/// C: `Lisp_Object props[FONT_OBJECT_MAX];` - a fixed-size array of 17 slots.
struct props17 {
  var v0: Lisp_Object
  var v1: Lisp_Object
  var v2: Lisp_Object
  var v3: Lisp_Object
  var v4: Lisp_Object
  var v5: Lisp_Object
  var v6: Lisp_Object
  var v7: Lisp_Object
  var v8: Lisp_Object
  var v9: Lisp_Object
  var v10: Lisp_Object
  var v11: Lisp_Object
  var v12: Lisp_Object
  var v13: Lisp_Object
  var v14: Lisp_Object
  var v15: Lisp_Object
  var v16: Lisp_Object
}

/// C: `struct font_metrics` (src/font.h 414-417) - `short lbearing, rbearing, width, ascent,
/// descent;`.  The record's owner mirror is the core's font.c unit (`femacs/src/font.rs` 196,
/// which cites the frozen configuration's `font.h` 351-356 and asserts `sizeof` 10), so this
/// lane names it - as it names `struct font` - instead of hanging an opaque pointer.  The
/// windowed declaration is the same five 2-byte fields; measured in `## Evidence`.
struct font_metrics {
  var lbearing: Int16
  var rbearing: Int16
  var width: Int16
  var ascent: Int16
  var descent: Int16
}

/// C: `struct font_bitmap` (src/font.h 419-430) - the record `font_bitmap` named by the two
/// `font_driver` (src/font.h 400) slots this file leaves NULL.  Owner mirror:
/// `femacs/src/font.rs` 208 (`sizeof` 40, `buffer`@16,
/// `left`@24).
struct font_bitmap {
  var bits_per_pixel: Int32
  var rows: Int32
  var width: Int32
  var pitch: Int32
  var buffer: UnsafeMutablePointer<UInt8>?
  var left: Int32
  var top: Int32
  var advance: Int32
}

/// C: `struct font_driver` (src/font.h 589-821) - the record `font_driver` whose instance
/// `nsfont_driver` (src/font.h 985) is this lane's.  The core mirrors it for the frozen
/// configuration (`femacs/src/font.rs` 266:
/// `sizeof` 208, `draw`/`get_bitmap`/`free_bitmap` behind its `window_system` feature); this
/// lane mirrors the **windowed** form because this file's target is the NS build - measured in
/// `## Evidence`: `sizeof` 232, `type`@0, `case_sensitive`@8, `get_cache`@16, `list`@24,
/// `match`@32, `list_family`@40, `free_entity`@48, `open_font`@56, `close_font`@64,
/// `prepare_face`@72, `done_face`@80, `has_char`@88, `encode_char`@96, `text_extents`@104,
/// `draw`@112, `get_bitmap`@120, `free_bitmap`@128, `anchor_point`@136, `otf_capability`@144,
/// `otf_drive`@152, `start_for_frame`@160, `end_for_frame`@168, `shape`@176, `check`@184,
/// `get_variation_glyphs`@192, `filter_properties`@200, `cached_font_ok`@208,
/// `combining_capability`@216, `drop_xrender_surfaces`@224.  `HAVE_HARFBUZZ`'s
/// `begin_hb_font`/`end_hb_font` (font.h 813-819) are absent, as on the frozen target.  Every
/// slot is C's pointer-to-function.  **Every pointer parameter of a slot is spelled
/// `UnsafeMutableRawPointer?` even where this file does mirror the record**: in a file that
/// imports AppKit (Objective-C interop), a `@convention(c)` function type requires
/// Objective-C-representable parameter types, and a pointer to a Swift struct is not one -
/// `@convention(c) (UnsafeMutablePointer<font>?) -> Void` is rejected outright (Evidence).
/// The C prototype stays in each comment, and the initialiser below reinterprets the pointer
/// for the three entry points this file assigns (a pointer is a pointer; no behaviour sits in
/// between).
struct font_driver {
  /// C: `Lisp_Object type;` (font.h 592) - `LISPSYM_INITIALLY (Qns)` in the initialiser.
  var type: Lisp_Object
  var case_sensitive: Bool
  /// C: `Lisp_Object (*get_cache) (struct frame *f);` (font.h 600).
  var get_cache: (@convention(c) (UnsafeMutableRawPointer?) -> Lisp_Object)?
  /// C: `Lisp_Object (*list) (struct frame *, Lisp_Object);` (font.h 623).
  var list: (@convention(c) (UnsafeMutableRawPointer?, Lisp_Object) -> Lisp_Object)?
  /// C: `Lisp_Object (*match) (struct frame *, Lisp_Object);` (font.h 632).
  var match: (@convention(c) (UnsafeMutableRawPointer?, Lisp_Object) -> Lisp_Object)?
  /// C: `Lisp_Object (*list_family) (struct frame *);` (font.h 637).
  var list_family: (@convention(c) (UnsafeMutableRawPointer?) -> Lisp_Object)?
  /// C: `void (*free_entity) (Lisp_Object);` (font.h 641).
  var free_entity: (@convention(c) (Lisp_Object) -> Void)?
  /// C: `Lisp_Object (*open_font) (struct frame *, Lisp_Object, int);` (font.h 645-646).
  var open_font: (@convention(c) (UnsafeMutableRawPointer?, Lisp_Object, Int32) -> Lisp_Object)?
  /// C: `void (*close_font) (struct font *);` (font.h 649).
  var close_font: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
  /// C: `void (*prepare_face) (struct frame *, struct face *);` (font.h 653) - `struct face`
  /// has an owner mirror (`femacs/src/font.rs` 464) that is the *non-windowed* layout, so the
  /// slot keeps the untyped pointer (the windowed form is what the deferred `nsfont_draw`
  /// needs).
  var prepare_face: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void)?
  /// C: `void (*done_face) (struct frame *, struct face *);` (font.h 657).
  var done_face: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void)?
  /// C: `int (*has_char) (Lisp_Object, int);` (font.h 663).
  var has_char: (@convention(c) (Lisp_Object, Int32) -> Int32)?
  /// C: `unsigned (*encode_char) (struct font *, int);` (font.h 667).
  var encode_char: (@convention(c) (UnsafeMutableRawPointer?, Int32) -> UInt32)?
  /// C: `void (*text_extents) (struct font *, const unsigned *, int, struct font_metrics *);`
  /// (font.h 672-673).
  var text_extents: (@convention(c) (UnsafeMutableRawPointer?, UnsafePointer<UInt32>?, Int32,
                                     UnsafeMutableRawPointer?) -> Void)?
  /// C: `int (*draw) (struct glyph_string *, int, int, int, int, bool);` (font.h 683-684) -
  /// `struct glyph_string` has no owner mirror (see `## Deferred`), so its slot keeps the
  /// untyped form.
  var draw: (@convention(c) (UnsafeMutableRawPointer?, Int32, Int32, Int32, Int32, Bool) -> Int32)?
  /// C: `int (*get_bitmap) (struct font *, unsigned, struct font_bitmap *, int);` (font.h
  /// 690-691).
  var get_bitmap: (@convention(c) (UnsafeMutableRawPointer?, UInt32,
                                   UnsafeMutableRawPointer?, Int32) -> Int32)?
  /// C: `void (*free_bitmap) (struct font *, struct font_bitmap *);` (font.h 696).
  var free_bitmap: (@convention(c) (UnsafeMutableRawPointer?,
                                     UnsafeMutableRawPointer?) -> Void)?
  /// C: `int (*anchor_point) (struct font *, unsigned, int, int *, int *);` (font.h 704-705).
  var anchor_point: (@convention(c) (UnsafeMutableRawPointer?, UInt32, Int32,
                                     UnsafeMutablePointer<Int32>?,
                                     UnsafeMutablePointer<Int32>?) -> Int32)?
  /// C: `Lisp_Object (*otf_capability) (struct font *);` (font.h 713).
  var otf_capability: (@convention(c) (UnsafeMutableRawPointer?) -> Lisp_Object)?
  /// C: `int (*otf_drive) (struct font *, Lisp_Object, Lisp_Object, int, int, Lisp_Object,
  /// int, bool);` (font.h 734-735).
  var otf_drive: (@convention(c) (UnsafeMutableRawPointer?, Lisp_Object, Lisp_Object, Int32,
                                  Int32, Lisp_Object, Int32, Bool) -> Int32)?
  /// C: `int (*start_for_frame) (struct frame *);` (font.h 742).
  var start_for_frame: (@convention(c) (UnsafeMutableRawPointer?) -> Int32)?
  /// C: `int (*end_for_frame) (struct frame *);` (font.h 747).
  var end_for_frame: (@convention(c) (UnsafeMutableRawPointer?) -> Int32)?
  /// C: `Lisp_Object (*shape) (Lisp_Object, Lisp_Object);` (font.h 765).
  var shape: (@convention(c) (Lisp_Object, Lisp_Object) -> Lisp_Object)?
  /// C: `int (*check) (struct frame *, struct font *);` (font.h 771).
  var check: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Int32)?
  /// C: `int (*get_variation_glyphs) (struct font *, int, unsigned *);` (font.h 778-779).
  var get_variation_glyphs: (@convention(c) (UnsafeMutableRawPointer?, Int32,
                                              UnsafeMutablePointer<UInt32>?) -> Int32)?
  /// C: `void (*filter_properties) (Lisp_Object, Lisp_Object);` (font.h 787).
  var filter_properties: (@convention(c) (Lisp_Object, Lisp_Object) -> Void)?
  /// C: `bool (*cached_font_ok) (struct frame *, Lisp_Object, Lisp_Object);` (font.h 792-793).
  var cached_font_ok: (@convention(c) (UnsafeMutableRawPointer?, Lisp_Object,
                                       Lisp_Object) -> Bool)?
  /// C: `Lisp_Object (*combining_capability) (struct font *);` (font.h 799).
  var combining_capability: (@convention(c) (UnsafeMutableRawPointer?) -> Lisp_Object)?
  /// C: `void (*drop_xrender_surfaces) (struct frame *);` (font.h 805).
  var drop_xrender_surfaces: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
}

/// C: `struct Lisp_Fwd { ENUM_BF (Lisp_Fwd_Type) type : 8; union { … } u; }` (src/lisp.h
/// 3118-3133) - the record `DEFVAR_LISP` builds for `Vns_reg_to_script`.  The bitfield is one
/// byte and the union one slot, as in the lane's `nsselect` unit (docs/ports/nsselect.swift).
struct Lisp_Fwd_u {
  var objvar: UnsafeMutablePointer<Lisp_Object>?
}
struct Lisp_Fwd {
  var type: UInt8
  var u: Lisp_Fwd_u
}

/// C: `struct nsfont_info` (src/nsterm.h 824-849, the `NS_IMPL_GNUSTEP` arm) - the
/// per-font record the driver hangs off `struct font`.  It is a platform record, so this
/// lane mirrors it (as `struct font`'s owner mirror does for its own); the layout is measured
/// with the same declarations against the frozen headers (`## Evidence`).
struct nsfont_info {
  var font: font
  /// PostScript name, uniquely identifies on NS systems.
  var name: UnsafeMutablePointer<CChar>?
  /// The following metrics are stored as float rather than int.
  var width: Float
  var height: Float
  var underpos: Float
  var underwidth: Float
  var size: Float
  /// C: `NSFont *nsfont;` (the `__OBJC__` arm) - an object pointer.
  var nsfont: UnsafeMutableRawPointer?
  /// convenience flags
  var bold: CChar
  var ital: CChar
  var synthItal: CChar
  var max_bounds: XCharStruct
  /// We compute glyph codes and metrics on-demand in blocks of 256 indexed by hibyte,
  /// lobyte.
  var glyphs: UnsafeMutablePointer<UnsafeMutablePointer<UInt32>?>?
  /// C: `struct font_metrics **metrics;` (src/nsterm.h 848) - the record's owner mirror is
  /// `femacs/src/font.rs` 196, so it is named here rather than left untyped.
  var metrics: UnsafeMutablePointer<UnsafeMutablePointer<font_metrics>?>?
}

/// C: `struct ns_display_info`'s prefix (src/nsterm.h 853-928).  The only member this port
/// reads is `name_list_element` (in `nsfont_get_cache`), so the mirror stops there; the tail
/// needs the ObjC-header types of the same header.
struct ns_display_info {
  /// Chain of all ns_display_info structures.
  var next: UnsafeMutablePointer<ns_display_info>?
  /// The generic display parameters corresponding to this NS display.
  var terminal: UnsafeMutableRawPointer?
  /// This is a cons cell of the form (NAME . FONT-LIST-CACHE).
  var name_list_element: Lisp_Object
}

// ==========================================================================
// Lane ABI: the C side.
// ==========================================================================

// Core functions (src/lisp.h, src/alloc.c, src/font.c, src/chartab.c).
/// C: `void xfree (void *)` (src/lisp.h 5543); the definition is the gnulib allocator's, in
/// the port of src/alloc.c.
@_silgen_name("xfree") func xfree(_ p: UnsafeMutableRawPointer?)
/// C: `char *xstrdup (char const *)` (src/lisp.h 5551); the definition is the gnulib
/// allocator's, in the port of src/alloc.c.
@_silgen_name("xstrdup") func xstrdup(_ s: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?
/// C: `char *xlispstrdup (Lisp_Object)` (src/lisp.h 5553).
@_silgen_name("xlispstrdup") func xlispstrdup(_ string: Lisp_Object) -> UnsafeMutablePointer<CChar>?
/// C: `Lisp_Object font_make_entity (void)` (src/font.h 842).
@_silgen_name("font_make_entity") func font_make_entity() -> Lisp_Object
/// C: `Lisp_Object intern (const char *)` - `INLINE` in src/lisp.h 4757.
@_silgen_name("intern") func intern(_ str: UnsafePointer<CChar>) -> Lisp_Object
/// C: `void debug_print (Lisp_Object)` (src/lisp.h 4702).
@_silgen_name("debug_print") func debug_print(_ obj: Lisp_Object)
/// C: `Lisp_Object assq_no_quit (Lisp_Object, Lisp_Object)` (src/lisp.h 4272).
@_silgen_name("assq_no_quit") func assq_no_quit(_ key: Lisp_Object, _ list: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fcons (Lisp_Object, Lisp_Object)` (globals.h 4560).
@_silgen_name("Fcons") func Fcons(_ car: Lisp_Object, _ cdr: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fmemq (Lisp_Object, Lisp_Object)` (globals.h 5101).
@_silgen_name("Fmemq") func Fmemq(_ elt: Lisp_Object, _ list: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object Fnreverse (Lisp_Object)` (globals.h 5161).
@_silgen_name("Fnreverse") func Fnreverse(_ list: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object list1 (Lisp_Object)` (src/lisp.h 4486).
@_silgen_name("list1") func list1(_ a: Lisp_Object) -> Lisp_Object
/// C: `void map_char_table (void (*) (Lisp_Object, Lisp_Object, Lisp_Object), Lisp_Object,
/// Lisp_Object, Lisp_Object)` (declared src/lisp.h 4689, defined src/chartab.c 842).
@_silgen_name("map_char_table")
func map_char_table(_ fn: @convention(c) (Lisp_Object, Lisp_Object, Lisp_Object) -> Void,
                    _ table: Lisp_Object, _ subtable: Lisp_Object, _ arg: Lisp_Object)
/// C: `AVOID emacs_abort (void)` (src/lisp.h 5302) - it does not return.
@_silgen_name("emacs_abort") func emacs_abort() -> Never

// Header-only constructs (macros, INLINE functions, enumerators, globals entries); the core
// exports each C-ABI entry under the C construct's own name.
/// C: `#define NILP(x) lisp_h_NILP (x)` (src/lisp.h 450).
@_silgen_name("NILP") func NILP(_ x: Lisp_Object) -> Bool
/// C: `INLINE bool EQ (Lisp_Object, Lisp_Object)` (src/lisp.h 1315).
@_silgen_name("EQ") func EQ(_ x: Lisp_Object, _ y: Lisp_Object) -> Bool
/// C: `#define CONSP(x) lisp_h_CONSP (x)` (src/lisp.h 446).
@_silgen_name("CONSP") func CONSP(_ x: Lisp_Object) -> Bool
/// C: `#define FIXNUMP(x) lisp_h_FIXNUMP (x)` (src/lisp.h 449).
@_silgen_name("FIXNUMP") func FIXNUMP(_ x: Lisp_Object) -> Bool
/// C: `INLINE bool SYMBOLP (Lisp_Object)` (src/lisp.h 1075).
@_silgen_name("SYMBOLP") func SYMBOLP(_ x: Lisp_Object) -> Bool
/// C: `#define XCAR(c) lisp_h_XCAR (c)` (src/lisp.h 455).
@_silgen_name("XCAR") func XCAR(_ c: Lisp_Object) -> Lisp_Object
/// C: `#define XCDR(c) lisp_h_XCDR (c)` (src/lisp.h 456).
@_silgen_name("XCDR") func XCDR(_ c: Lisp_Object) -> Lisp_Object
/// C: `#define XSETCDR(c, v) (XCONS (c)->u.s.u.cdr = (v))` (src/lisp.h 1515) - it assigns
/// through its first argument, which is a cons object (as with `ASET`).
@_silgen_name("XSETCDR") func XSETCDR(_ c: Lisp_Object, _ v: Lisp_Object)
/// C: `INLINE Lisp_Object AREF (Lisp_Object, ptrdiff_t)` (src/lisp.h 1964).
@_silgen_name("AREF") func AREF(_ array: Lisp_Object, _ idx: Int) -> Lisp_Object
/// C: `INLINE void ASET (Lisp_Object, ptrdiff_t, Lisp_Object)` (src/lisp.h 1978).
@_silgen_name("ASET") func ASET(_ array: Lisp_Object, _ idx: Int, _ val: Lisp_Object)
/// C: `INLINE Lisp_Object SYMBOL_NAME (Lisp_Object)` (src/lisp.h 2350).
@_silgen_name("SYMBOL_NAME") func SYMBOL_NAME(_ sym: Lisp_Object) -> Lisp_Object
/// C: `INLINE char *SSDATA (Lisp_Object)` (src/lisp.h 1658).
@_silgen_name("SSDATA") func SSDATA(_ string: Lisp_Object) -> UnsafeMutablePointer<CChar>
/// C: `INLINE ptrdiff_t SBYTES (Lisp_Object)` (src/lisp.h 1697).
@_silgen_name("SBYTES") func SBYTES(_ string: Lisp_Object) -> Int
/// C: `INLINE Lisp_Object make_fixnum (EMACS_INT)` (src/lisp.h 1211).
@_silgen_name("make_fixnum") func make_fixnum(_ n: Int) -> Lisp_Object
/// C: `enum font_property_index`'s members (src/font.h 70-190), one entry each.
@_silgen_name("FONT_TYPE_INDEX") func FONT_TYPE_INDEX() -> Int
@_silgen_name("FONT_FOUNDRY_INDEX") func FONT_FOUNDRY_INDEX() -> Int
@_silgen_name("FONT_FAMILY_INDEX") func FONT_FAMILY_INDEX() -> Int
@_silgen_name("FONT_ADSTYLE_INDEX") func FONT_ADSTYLE_INDEX() -> Int
@_silgen_name("FONT_REGISTRY_INDEX") func FONT_REGISTRY_INDEX() -> Int
@_silgen_name("FONT_WEIGHT_INDEX") func FONT_WEIGHT_INDEX() -> Int
@_silgen_name("FONT_SLANT_INDEX") func FONT_SLANT_INDEX() -> Int
@_silgen_name("FONT_WIDTH_INDEX") func FONT_WIDTH_INDEX() -> Int
@_silgen_name("FONT_SIZE_INDEX") func FONT_SIZE_INDEX() -> Int
@_silgen_name("FONT_AVGWIDTH_INDEX") func FONT_AVGWIDTH_INDEX() -> Int
@_silgen_name("FONT_SPACING_INDEX") func FONT_SPACING_INDEX() -> Int
@_silgen_name("FONT_EXTRA_INDEX") func FONT_EXTRA_INDEX() -> Int
@_silgen_name("FONT_OBJLIST_INDEX") func FONT_OBJLIST_INDEX() -> Int
/// C: `FONT_SPACING_PROPORTIONAL` / `FONT_SPACING_MONO` (src/font.h 408-410).
@_silgen_name("FONT_SPACING_PROPORTIONAL") func FONT_SPACING_PROPORTIONAL() -> Int
@_silgen_name("FONT_SPACING_MONO") func FONT_SPACING_MONO() -> Int
/// C: `#define FONT_WEIGHT_SYMBOLIC(font) AREF ((font), FONT_WEIGHT_INDEX)` (src/font.h
/// 205-213, one macro per style).
@_silgen_name("FONT_WEIGHT_SYMBOLIC") func FONT_WEIGHT_SYMBOLIC(_ font: Lisp_Object) -> Lisp_Object
@_silgen_name("FONT_SLANT_SYMBOLIC") func FONT_SLANT_SYMBOLIC(_ font: Lisp_Object) -> Lisp_Object
@_silgen_name("FONT_WIDTH_SYMBOLIC") func FONT_WIDTH_SYMBOLIC(_ font: Lisp_Object) -> Lisp_Object
/// C: `#define FONT_SET_STYLE(font, prop, val) ASET ((font), prop, (val))` (src/font.h 236).
@_silgen_name("FONT_SET_STYLE") func FONT_SET_STYLE(_ font: Lisp_Object, _ prop: Int, _ val: Lisp_Object)
/// C: `INLINE struct Lisp_Char_Table *XCHAR_TABLE (Lisp_Object)` (src/lisp.h 2090).
@_silgen_name("XCHAR_TABLE") func XCHAR_TABLE(_ a: Lisp_Object) -> UnsafeMutablePointer<Lisp_Char_Table>
/// C: `#define Qfoo builtin_lisp_symbol (...)` in the generated globals.h (PORTING.md P-17);
/// called as `Qfoo ()`, the Rust mirror's shape.
@_silgen_name("Qnil") func Qnil() -> Lisp_Object
@_silgen_name("Qns") func Qns() -> Lisp_Object
@_silgen_name("Qitalic") func Qitalic() -> Lisp_Object
@_silgen_name("Qoblique") func Qoblique() -> Lisp_Object
@_silgen_name("Qreverse_italic") func Qreverse_italic() -> Lisp_Object
@_silgen_name("Qreverse_oblique") func Qreverse_oblique() -> Lisp_Object
@_silgen_name("Qnormal") func Qnormal() -> Lisp_Object
@_silgen_name("Qbold") func Qbold() -> Lisp_Object
@_silgen_name("Qlight") func Qlight() -> Lisp_Object
@_silgen_name("Qcondensed") func Qcondensed() -> Lisp_Object
@_silgen_name("Qexpanded") func Qexpanded() -> Lisp_Object
@_silgen_name("Qiso10646_1") func Qiso10646_1() -> Lisp_Object
@_silgen_name("QCscript") func QCscript() -> Lisp_Object
@_silgen_name("QClang") func QClang() -> Lisp_Object
@_silgen_name("QCotf") func QCotf() -> Lisp_Object
@_silgen_name("QL2R") func QL2R() -> Lisp_Object
@_silgen_name("QR2L") func QR2L() -> Lisp_Object

// Core variables this file reads.  They are fields of the generated `globals` record in C
// (`globals.h`'s `#define Vfoo globals.f_Vfoo`), and the unit that owns each defines it
// (docs/status.md 3.29); the core's Rust units export them as C data symbols, so the lane
// imports them.
@_silgen_name("Vchar_script_table") var Vchar_script_table: Lisp_Object
@_silgen_name("Votf_script_alist") var Votf_script_alist: Lisp_Object

/// C: `Vns_reg_to_script` is `DEFVAR_LISP`'d by *this* file (src/nsfont.m 1746-1748), so this
/// unit owns its storage; Swift cannot emit a C-named data symbol (`@_cdecl` refuses `var`
/// and `@_silgen_name` on a `var` imports), so it is a Swift global with C's initial `Qnil`
/// and `syms_of_nsfont` registers it through `defvar_lisp` (the `nsselect` unit's shape).
fileprivate var Vns_reg_to_script: Lisp_Object = Qnil()

// ==========================================================================
// The file's own macros (src/nsfont.m 47, 296, 1304) and objects (516).
// ==========================================================================

/// C: `#define NSFONT_TRACE 0` (src/nsfont.m 47) - the trace flag; the `if (NSFONT_TRACE)`
/// blocks are compiled but never run, as in C.
fileprivate let NSFONT_TRACE: Int32 = 0

/// C: `#define INVALID_GLYPH 0xFFFF` (src/nsfont.m 296).
fileprivate let INVALID_GLYPH: UInt32 = 0xFFFF

/// C: `#define RIGHT_TO_LEFT_P permutation` (src/nsfont.m 1304, `#undef`ed at 1447) - a
/// textual alias for the `permutation` pointer of the shaping function, which is deferred;
/// the macro is recorded in the ledger rather than spelled here, because a Swift alias for a
/// local of a deferred body would have no body to live in.

/// C: `static NSString *ascii_printable;` (src/nsfont.m 516).
fileprivate var ascii_printable: NSString? = nil

/* ==========================================================================

    Utilities

   ========================================================================== */

/// C: `ns_done_font_data` (src/nsfont.m 122-127).
fileprivate func ns_done_font_data(_ data: UnsafeMutablePointer<gs_font_data>) {
  if data.pointee.specified & GS_SPECIFIED_FAMILY != 0 {
    xfree(UnsafeMutableRawPointer(data.pointee.family_name))
  }
}

/// C: `ns_get_font_data` (src/nsfont.m 129-224).
fileprivate func ns_get_font_data(_ desc: NSFontDescriptor, _ dat: UnsafeMutablePointer<gs_font_data>) {
  var tem: NSNumber?
  let traits = desc.symbolicTraits
  let dict = desc.object(forKey: NSFontDescriptor.AttributeName.traits) as? NSDictionary
  let family = desc.object(forKey: NSFontDescriptor.AttributeName.family) as? NSString

  dat.pointee.specified = 0

  if family != nil {
    dat.pointee.specified |= GS_SPECIFIED_FAMILY
    dat.pointee.family_name = xstrdup(family!.utf8String!)
  }

  tem = desc.object(forKey: NSFontDescriptor.AttributeName.fixedAdvance) as? NSNumber

  if (tem != nil && tem!.boolValue != false)
      || traits.contains(.monoSpace) {
    dat.pointee.specified |= GS_SPECIFIED_SPACING
    dat.pointee.monospace_p = true
  } else if tem != nil && tem!.boolValue == false {
    dat.pointee.specified |= GS_SPECIFIED_SPACING
    dat.pointee.monospace_p = false
  }

  if traits.contains(.bold) {
    dat.pointee.specified |= GS_SPECIFIED_WEIGHT
    dat.pointee.weight = .GS_FONT_WEIGHT_BOLD
  }

  if traits.contains(.italic) {
    dat.pointee.specified |= GS_SPECIFIED_SLANT
    dat.pointee.slant = .GS_FONT_SLANT_ITALIC
  }

  if traits.contains(.condensed) {
    dat.pointee.specified |= GS_SPECIFIED_WIDTH
    dat.pointee.width = .GS_FONT_WIDTH_CONDENSED
  } else if traits.contains(.expanded) {
    dat.pointee.specified |= GS_SPECIFIED_WIDTH
    dat.pointee.width = .GS_FONT_WIDTH_EXPANDED
  }

  if dict != nil {
    tem = dict!.object(forKey: NSFontDescriptor.TraitKey.slant.rawValue) as? NSNumber

    if tem != nil {
      dat.pointee.specified |= GS_SPECIFIED_SLANT

      dat.pointee.slant = tem!.floatValue > 0
        ? .GS_FONT_SLANT_ITALIC
        : (tem!.floatValue < 0
           ? .GS_FONT_SLANT_REVERSE_ITALIC
           : .GS_FONT_SLANT_NORMAL)
    }

    tem = dict!.object(forKey: NSFontDescriptor.TraitKey.weight.rawValue) as? NSNumber

    if tem != nil {
      dat.pointee.specified |= GS_SPECIFIED_WEIGHT

      dat.pointee.weight = tem!.floatValue > 0
        ? .GS_FONT_WEIGHT_BOLD
        : (tem!.floatValue < -0.4
           ? .GS_FONT_WEIGHT_LIGHT
           : .GS_FONT_WEIGHT_NORMAL)
    }

    tem = dict!.object(forKey: NSFontDescriptor.TraitKey.width.rawValue) as? NSNumber

    if tem != nil {
      dat.pointee.specified |= GS_SPECIFIED_WIDTH

      dat.pointee.width = tem!.floatValue > 0
        ? .GS_FONT_WIDTH_EXPANDED
        : (tem!.floatValue < 0
           ? .GS_FONT_WIDTH_NORMAL
           : .GS_FONT_WIDTH_CONDENSED)
    }
  }
}

/// C: `ns_font_descs_match_p` (src/nsfont.m 225-305).
fileprivate func ns_font_descs_match_p(_ desc: NSFontDescriptor, _ target: NSFontDescriptor) -> Bool {
  var dat = gs_font_data(specified: 0, slant: .GS_FONT_SLANT_NORMAL,
                         weight: .GS_FONT_WEIGHT_NORMAL, width: .GS_FONT_WIDTH_NORMAL,
                         monospace_p: false, family_name: nil)
  var t = dat

  ns_get_font_data(desc, &dat)
  ns_get_font_data(target, &t)

  if t.specified & GS_SPECIFIED_WIDTH == 0 {
    t.width = .GS_FONT_WIDTH_NORMAL
  }
  if t.specified & GS_SPECIFIED_WEIGHT == 0 {
    t.weight = .GS_FONT_WEIGHT_NORMAL
  }
  if t.specified & GS_SPECIFIED_SPACING == 0 {
    t.monospace_p = false
  }
  if t.specified & GS_SPECIFIED_SLANT == 0 {
    t.slant = .GS_FONT_SLANT_NORMAL
  }

  if t.specified & GS_SPECIFIED_FAMILY == 0 {
    emacs_abort()
  }

  var match_p = true

  // C's `goto gout` (src/nsfont.m 298-304) leaves the checks for the label above the two
  // releases and the return; a labelled `do` block is that shape in Swift (PORTING.md 2.2 -
  // not an early `return`, which would skip the cleanup).
  gout: do {
    if dat.specified & GS_SPECIFIED_WIDTH != 0 && dat.width != t.width {
      match_p = false
      break gout
    }

    if dat.specified & GS_SPECIFIED_WEIGHT != 0 && dat.weight != t.weight {
      match_p = false
      break gout
    }

    if dat.specified & GS_SPECIFIED_SPACING != 0 && dat.monospace_p != t.monospace_p {
      match_p = false
      break gout
    }

    if dat.specified & GS_SPECIFIED_SLANT != 0 && dat.monospace_p != t.monospace_p {
      if NSFONT_TRACE != 0 {
        printf("Matching monospace for %s: %d %d\n",
               t.family_name!, Int32(dat.monospace_p ? 1 : 0), Int32(t.monospace_p ? 1 : 0))
      }
      match_p = false
      break gout
    }

    if dat.specified & GS_SPECIFIED_FAMILY != 0 && strcmp(dat.family_name!, t.family_name!) != 0 {
      match_p = false
    }
  }

  ns_done_font_data(&dat)
  ns_done_font_data(&t)

  return match_p
}

/// C: `ns_get_family` (src/nsfont.m 307-322).
fileprivate func ns_get_family(_ font_spec: Lisp_Object) -> NSString? {
  let tem = AREF(font_spec, FONT_FAMILY_INDEX())
  if NILP(tem) {
    return nil
  } else {
    let tmp = xlispstrdup(SYMBOL_NAME(tem))
    let family = NSString(utf8String: tmp!)
    xfree(UnsafeMutableRawPointer(tmp))
    return family
  }
}

/// C: `ns_spec_to_descriptor` (src/nsfont.m 325-421).
fileprivate func ns_spec_to_descriptor(_ font_spec: Lisp_Object) -> NSFontDescriptor? {
  var fdesc: NSFontDescriptor?
  let fdAttrs = NSMutableDictionary()
  let family = ns_get_family(font_spec)
  let tdict = NSMutableDictionary()

  var tem: Lisp_Object

  tem = FONT_SLANT_SYMBOLIC(font_spec)
  if !NILP(tem) {
    if EQ(tem, Qitalic()) || EQ(tem, Qoblique()) {
      tdict.setObject(NSNumber(value: Float(1.0)),
                      forKey: NSFontDescriptor.TraitKey.slant.rawValue as NSString)
    } else if EQ(tem, Qreverse_italic())
                || EQ(tem, Qreverse_oblique()) {
      tdict.setObject(NSNumber(value: Float(-1.0)),
                      forKey: NSFontDescriptor.TraitKey.slant.rawValue as NSString)
    } else {
      tdict.setObject(NSNumber(value: Float(0.0)),
                      forKey: NSFontDescriptor.TraitKey.slant.rawValue as NSString)
    }
  }

  tem = FONT_WIDTH_SYMBOLIC(font_spec)
  if !NILP(tem) {
    if EQ(tem, Qcondensed()) {
      tdict.setObject(NSNumber(value: Float(-1.0)),
                      forKey: NSFontDescriptor.TraitKey.width.rawValue as NSString)
    } else if EQ(tem, Qexpanded()) {
      tdict.setObject(NSNumber(value: Float(1.0)),
                      forKey: NSFontDescriptor.TraitKey.width.rawValue as NSString)
    } else {
      tdict.setObject(NSNumber(value: Float(0.0)),
                      forKey: NSFontDescriptor.TraitKey.width.rawValue as NSString)
    }
  }

  tem = FONT_WEIGHT_SYMBOLIC(font_spec)

  if !NILP(tem) {
    if EQ(tem, Qbold()) {
      tdict.setObject(NSNumber(value: Float(1.0)),
                      forKey: NSFontDescriptor.TraitKey.weight.rawValue as NSString)
    } else if EQ(tem, Qlight()) {
      tdict.setObject(NSNumber(value: Float(-1.0)),
                      forKey: NSFontDescriptor.TraitKey.weight.rawValue as NSString)
    } else {
      tdict.setObject(NSNumber(value: Float(0.0)),
                      forKey: NSFontDescriptor.TraitKey.weight.rawValue as NSString)
    }
  }

  tem = AREF(font_spec, FONT_SPACING_INDEX())

  if family != nil {
    fdAttrs.setObject(family!, forKey: NSFontDescriptor.AttributeName.family as NSString)
  }

  if FIXNUMP(tem) {
    if XFIXNUM_local(tem) != FONT_SPACING_PROPORTIONAL() {
      fdAttrs.setObject(NSNumber(value: true),
                        forKey: NSFontDescriptor.AttributeName.fixedAdvance as NSString)
    } else {
      fdAttrs.setObject(NSNumber(value: false),
                        forKey: NSFontDescriptor.AttributeName.fixedAdvance as NSString)
    }
  }

  /* Handle special families such as ``fixed'', ``monospace'' or
     ``Sans Serif''.  */

  if family?.isEqual(to: "fixed") == true
      || family?.isEqual(to: "monospace") == true {
    fdAttrs.setObject(NSFont.userFixedPitchFont(ofSize: 0)!.familyName!,
                      forKey: NSFontDescriptor.AttributeName.family as NSString)
  } else if family?.isEqual(to: "Sans Serif") == true {
    fdAttrs.setObject(NSFont.userFont(ofSize: 0)!.familyName!,
                      forKey: NSFontDescriptor.AttributeName.family as NSString)
  }

  fdAttrs.setObject(tdict, forKey: NSFontDescriptor.AttributeName.traits as NSString)

  fdesc = NSFontDescriptor(fontAttributes: fdAttrs as? [NSFontDescriptor.AttributeName: Any])

  return fdesc
}

/// C: `ns_descriptor_to_entity` (src/nsfont.m 424-489).
fileprivate func ns_descriptor_to_entity(_ desc: NSFontDescriptor, _ extra: Lisp_Object,
                                         _ style: UnsafePointer<CChar>?) -> Lisp_Object {
  let font_entity = font_make_entity()
  var data = gs_font_data(specified: 0, slant: .GS_FONT_SLANT_NORMAL,
                          weight: .GS_FONT_WEIGHT_NORMAL, width: .GS_FONT_WIDTH_NORMAL,
                          monospace_p: false, family_name: nil)
  ns_get_font_data(desc, &data)

  ASET(font_entity, FONT_TYPE_INDEX(), Qns())
  ASET(font_entity, FONT_FOUNDRY_INDEX(), Qns())
  if data.specified & GS_SPECIFIED_FAMILY != 0 {
    ASET(font_entity, FONT_FAMILY_INDEX(), intern(data.family_name!))
  }
  ASET(font_entity, FONT_ADSTYLE_INDEX(), style != nil ? intern(style!) : Qnil())
  ASET(font_entity, FONT_REGISTRY_INDEX(), Qiso10646_1())

  if data.specified & GS_SPECIFIED_WEIGHT != 0 {
    FONT_SET_STYLE(font_entity, FONT_WEIGHT_INDEX(),
                   data.weight == .GS_FONT_WEIGHT_BOLD
                   ? Qbold() : (data.weight == .GS_FONT_WEIGHT_LIGHT
                                ? Qlight() : Qnormal()))
  } else {
    FONT_SET_STYLE(font_entity, FONT_WEIGHT_INDEX(), Qnormal())
  }

  if data.specified & GS_SPECIFIED_SLANT != 0 {
    FONT_SET_STYLE(font_entity, FONT_SLANT_INDEX(),
                   data.slant == .GS_FONT_SLANT_ITALIC
                   ? Qitalic() : (data.slant == .GS_FONT_SLANT_REVERSE_ITALIC
                                  ? Qreverse_italic() : Qnormal()))
  } else {
    FONT_SET_STYLE(font_entity, FONT_SLANT_INDEX(), Qnormal())
  }

  if data.specified & GS_SPECIFIED_WIDTH != 0 {
    FONT_SET_STYLE(font_entity, FONT_WIDTH_INDEX(),
                   data.width == .GS_FONT_WIDTH_CONDENSED
                   ? Qcondensed() : (data.width == .GS_FONT_WIDTH_EXPANDED
                                     ? Qexpanded() : Qnormal()))
  } else {
    FONT_SET_STYLE(font_entity, FONT_WIDTH_INDEX(), Qnormal())
  }

  ASET(font_entity, FONT_SIZE_INDEX(), make_fixnum(0))
  ASET(font_entity, FONT_AVGWIDTH_INDEX(), make_fixnum(0))
  ASET(font_entity, FONT_SPACING_INDEX(),
       make_fixnum((data.specified & GS_SPECIFIED_SPACING != 0 && data.monospace_p)
                   ? FONT_SPACING_MONO() : FONT_SPACING_PROPORTIONAL()))

  ASET(font_entity, FONT_EXTRA_INDEX(), extra)
  ASET(font_entity, FONT_OBJLIST_INDEX(), Qnil())

  if NSFONT_TRACE != 0 {
    fputs("created font_entity:\n    ", stderr)
    debug_print(font_entity)
  }

  ns_done_font_data(&data)
  return font_entity
}

/// C: `ns_fallback_entity` (src/nsfont.m 491-496).  `Qnil`/`NULL` are C's second and third
/// arguments.
fileprivate func ns_fallback_entity() -> Lisp_Object {
  return ns_descriptor_to_entity(NSFont.userFixedPitchFont(ofSize: 1)!.fontDescriptor,
                                 Qnil(), nil)
}

/// C: `ns_char_width` (src/nsfont.m 499-514).
fileprivate func ns_char_width(_ sfont: NSFont, _ c: Int32) -> CGFloat {
  var w: CGFloat = -1.0
  let cstr = NSString(format: "%c", c)

  if w < 0.0 {
    let attrsDictionary: [NSAttributedString.Key: Any] = [.font: sfont]
    w = cstr.size(withAttributes: attrsDictionary).width
  }

  return max(w, 1.0)
}

/// C: `ns_ascii_average_width` (src/nsfont.m 519-549).
fileprivate func ns_ascii_average_width(_ sfont: NSFont) -> Int32 {
  var w: CGFloat = -1.0

  if ascii_printable == nil {
    var chars = [CChar](repeating: 0, count: 96)
    var ch: Int32
    ch = 0
    while ch < 95 {
      chars[Int(ch)] = CChar(truncatingIfNeeded: Int32(UInt8(ascii: " ")) + ch)
      ch += 1
    }
    chars[95] = 0

    // C: `[[NSString alloc] initWithFormat: @"%s", chars]` - the array decays to a C string
    // for the duration of the call.
    chars.withUnsafeBufferPointer { buf in
      ascii_printable = NSString(format: "%s", buf.baseAddress!)
    }
  }

  if w < 0.0 {
    let attrsDictionary: [NSAttributedString.Key: Any] = [.font: sfont]
    w = ascii_printable!.size(withAttributes: attrsDictionary).width
  }

  return Int32(lrint(w / 95.0))
}

/// C: `ns_charset_covers` (src/nsfont.m 552-585).
fileprivate func ns_charset_covers(_ set1: NSCharacterSet, _ set2: NSCharacterSet, _ pct: Float) -> Bool {
  let font = set1.bitmapRepresentation
  let script = set2.bitmapRepresentation

  var covered = 0
  var total = 0

  font.withUnsafeBytes { fontBytes in
    script.withUnsafeBytes { scriptBytes in
      var b: Int
      b = 0
      while b < script.count {
        var i: Int32
        i = 0
        while i < 8 {
          if scriptBytes[b] & UInt8(1 << i) != 0 {
            total += 1

            if b < font.count
                && fontBytes[b] & UInt8(1 << i) != 0 {
              covered += 1
            }
          }
          i += 1
        }
        b += 1
      }
    }
  }

  return Float(covered) / Float(total) >= 1.0 - pct
}

/// C: `ns_lang_to_script` (src/nsfont.m 587-600).  C's `strcmp` on the symbol's name.
fileprivate func ns_lang_to_script(_ lang: Lisp_Object) -> NSString {
  let name = SSDATA(SYMBOL_NAME(lang))
  if strcmp(name, "ja") == 0 {
    return "han"
  } else if strcmp(name, "zh") == 0 {
    return "han"
  } else if strcmp(name, "ko") == 0 {
    return "hangul"
  } else {
    return ""
  }
}

/// C: `ns_otf_to_script` (src/nsfont.m 602-610).
fileprivate func ns_otf_to_script(_ otf: Lisp_Object) -> NSString {
  let script = assq_no_quit(XCAR(otf), Votf_script_alist)
  return CONSP(script)
    ? stringWithLispString(SYMBOL_NAME(XCDR(script)))!
    : ""
}

/// C: `ns_registry_to_script` (src/nsfont.m 612-631).
fileprivate func ns_registry_to_script(_ reg: UnsafeMutablePointer<CChar>) -> NSString {
  var script: Lisp_Object
  var r: Lisp_Object
  var rts = Vns_reg_to_script
  while CONSP(rts) {
    r = XCAR(XCAR(rts))
    if strncmp(SSDATA(r), reg, SBYTES(r)) == 0 {
      script = XCDR(XCAR(rts))
      return stringWithLispString(SYMBOL_NAME(script))!
    }
    rts = XCDR(rts)
  }
  return ""
}

/// C: `ns_get_req_script` (src/nsfont.m 633-671).
fileprivate func ns_get_req_script(_ font_spec: Lisp_Object) -> NSString {
  let reg = AREF(font_spec, FONT_REGISTRY_INDEX())
  var extra = AREF(font_spec, FONT_EXTRA_INDEX())

  /* The extra-bundle properties have priority.  */
  while CONSP(extra) {
    let tmp = XCAR(extra)
    if CONSP(tmp) {
      let key = XCAR(tmp)
      let val = XCDR(tmp)
      if EQ(key, QCscript()) && SYMBOLP(val) {
        return stringWithLispString(SYMBOL_NAME(val))!
      }
      if EQ(key, QClang()) && SYMBOLP(val) {
        return ns_lang_to_script(val)
      }
      if EQ(key, QCotf()) && CONSP(val) && SYMBOLP(XCAR(val)) {
        return ns_otf_to_script(val)
      }
    }
    extra = XCDR(extra)
  }

  /* If we get here, check the charset portion of the registry.  */
  if !NILP(reg) {
    /* XXX: iso10646 is passed in for non-ascii latin-1 characters
       (which causes box rendering if we don't treat it like iso8858-1)
       but also for ascii (which causes unnecessary font substitution).  */
    // C's `#if 0` block at 657-662 (the `iso10646` rewrite) is dead text; recorded in the
    // ledger, not transcribed.
    return ns_registry_to_script(SSDATA(SYMBOL_NAME(reg)))
  }

  return ""
}

/// C: `accumulate_script_ranges` (src/nsfont.m 673-684).
fileprivate func accumulate_script_ranges(_ arg: Lisp_Object, _ range: Lisp_Object, _ val: Lisp_Object) {
  if EQ(XCAR(arg), val) {
    if CONSP(range) {
      XSETCDR(arg, Fcons(Fcons(XCAR(range), XCDR(range)), XCDR(arg)))
    } else {
      XSETCDR(arg, Fcons(Fcons(range, range), XCDR(arg)))
    }
  }
}

/// C: `ns_script_to_charset` (src/nsfont.m 686-719).
fileprivate func ns_script_to_charset(_ scriptName: NSString) -> NSCharacterSet {
  let charset = NSMutableCharacterSet()
  let script = intern(scriptName.utf8String!)
  let script_list = XCHAR_TABLE(Vchar_script_table).pointee.extras

  if !NILP(Fmemq(script, script_list)) {
    var ranges: Lisp_Object
    var range_list: Lisp_Object

    ranges = list1(script)
    map_char_table({ arg, range, val in accumulate_script_ranges(arg, range, val) },
                   Qnil(), Vchar_script_table, ranges)
    range_list = Fnreverse(XCDR(ranges))
    if !NILP(range_list) {
      while CONSP(range_list) {
        let start = XFIXNUM_local(XCAR(XCAR(range_list)))
        let end = XFIXNUM_local(XCDR(XCAR(range_list)))
        if NSFONT_TRACE != 0 {
          debug_print(XCAR(range_list))
        }
        if end < 0x10000 {
          charset.addCharacters(in: NSRange(location: Int(start), length: Int(end - start)))
        }
        range_list = XCDR(range_list)
      }
    }
  }
  return charset
}

/// C: `ns_get_covering_families` (src/nsfont.m 721-784).
fileprivate func ns_get_covering_families(_ script: NSString, _ pct_start: Float) -> NSSet {
  var pct = pct_start
  var families: NSMutableSet

  if NSFONT_TRACE != 0 {
    NSLog("Request covering families for script: '%@'", script)
  }

  if scriptToFamilies == nil {
    scriptToFamilies = NSMutableDictionary()
  }

  if let cached = scriptToFamilies!.object(forKey: script) as? NSMutableSet {
    families = cached
  } else {
    let fontMgr = NSFontManager.shared
    let allFamilies = fontMgr.availableFontFamilies

    if script.length == 0 {
      families = NSMutableSet(array: allFamilies)
    } else {
      let charset = ns_script_to_charset(script)
      families = NSMutableSet(capacity: 10)
      while true {
        for family in allFamilies {
          // C: `[[fontMgr fontWithFamily: family traits: 0 weight: 5 size: 12.0] coveredCharacterSet]`
          // - a `CharacterSet`; the rest of the function compares `NSCharacterSet *` as C does.
          var fset: NSCharacterSet?
          fset = fontMgr.font(withFamily: family, traits: [], weight: 5, size: 12.0)?
            .coveredCharacterSet as NSCharacterSet?
          /* Some fonts on macOS, maybe many on GNUstep, return nil.  */
          if fset == nil {
            let fallback = NSMutableCharacterSet()
            fallback.addCharacters(in: NSRange(location: 0, length: 127))
            fset = fallback
          }
          if ns_charset_covers(fset!, charset, pct) {
            families.add(family)
          }
        }
        pct -= 0.2
        if families.count > 0 || pct < 0.05 {
          break
        }
      }
    }
    scriptToFamilies!.setObject(families, forKey: script)
  }

  if NSFONT_TRACE != 0 {
    NSLog("    returning %lu families", families.count)
  }
  return families
}

/// C: `static NSMutableDictionary *scriptToFamilies = nil;` inside `ns_get_covering_families`
/// (src/nsfont.m 723).  Swift has no function-local statics, so the cache is a fileprivate
/// global, which is what preserves C's lifetime.
fileprivate var scriptToFamilies: NSMutableDictionary? = nil

// ==========================================================================
// Lane ABI: the rest of the C side this half needs.
// ==========================================================================

/// C's stdio calls.  Swift's imported `printf`/`fprintf`/`fputs` are marked unavailable
/// ("Variadic function is unavailable"), so the lane declares the C entry points; the format
/// strings are the reference's (see `pD`).
@_silgen_name("printf") func printf(_ format: UnsafePointer<CChar>, _ args: CVarArg...)
@_silgen_name("fprintf") func fprintf(_ stream: UnsafeMutablePointer<FILE>?, _ format: UnsafePointer<CChar>, _ args: CVarArg...)
@_silgen_name("fputs") func fputs(_ s: UnsafePointer<CChar>, _ stream: UnsafeMutablePointer<FILE>?)
/// C: `#define pD "t"` (src/lisp.h 150), a printf length modifier the reference splices into
/// format literals as `"%"pD"d"` (e.g. src/nsfont.m 903-904).  Swift has no compile-time
/// concatenation of literals, so the call site composes the same text and passes it as a C
/// string; the core owns this entry (the Rust mirror has `pub const pD: &str = "t"`).
@_silgen_name("pD") func pD() -> UnsafePointer<CChar>

/// C: `void *xmalloc (size_t)` (src/lisp.h 5535) - the ported core defines the gnulib
/// allocator it belongs to.
@_silgen_name("xmalloc") func xmalloc(_ size: Int) -> UnsafeMutableRawPointer?
/// C: `#define FONT_INVALID_CODE 0xFFFFFFFF` (src/font.h 585).
@_silgen_name("FONT_INVALID_CODE") func FONT_INVALID_CODE() -> UInt32
/// C: `#define FRAME_DISPLAY_INFO(f) ((struct ns_display_info *) (f)->output_data.ns->display_info)`
/// (src/nsterm.h 1032) - a construct of this lane's own header; the port of src/nsterm.m owns
/// the definition (it holds the `struct frame` mirror).
@_silgen_name("FRAME_DISPLAY_INFO")
func FRAME_DISPLAY_INFO(_ f: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<ns_display_info>
/// C: `+[GSFontEnumerator sharedEnumerator]` and
/// `-[GSFontEnumerator availableFontDescriptors]` (used at src/nsfont.m 790 and 799).
/// `GSFontEnumerator` is a **GNUstep** class: Apple's SDK has no such type, so the provider of
/// these two entries is the GNUstep libraries, not this repository (see the ledger).
@_silgen_name("GSFontEnumerator_sharedEnumerator")
func GSFontEnumerator_sharedEnumerator() -> UnsafeMutableRawPointer?
@_silgen_name("GSFontEnumerator_availableFontDescriptors")
func GSFontEnumerator_availableFontDescriptors(_ enumerator: UnsafeMutableRawPointer?) -> NSArray?

/// C's `NSTextStorage` (src/nsfont.m 1220) carries the `[textStorage setFont:font]` call at
/// src/nsfont.m 1235 as a GNUstep
/// extension; **macOS does not implement the selector** (an Objective-C message send is not
/// checked against the interface, which is why the reference compiles), so the port performs
/// the same effect with a **public API** rather than reaching for a symbol no provider in this
/// tree defines - the deviation docs/ports/nsfont.md's `## Decisions` records.
fileprivate func setFont(_ textStorage: NSTextStorage?, _ font: NSFont?) {
  textStorage!.addAttribute(.font, value: font as Any,
                            range: NSRange(location: 0, length: textStorage!.length))
}

/// C: `void block_input (void)` / `void unblock_input (void)` (src/blockinput.h) - `INLINE`
/// functions, so they are core C-ABI entries.
@_silgen_name("block_input") func block_input()
@_silgen_name("unblock_input") func unblock_input()
/// C: `INLINE EMACS_INT XFIXNUM (Lisp_Object)` (src/lisp.h 1268).
@_silgen_name("XFIXNUM") func XFIXNUM_local(_ a: Lisp_Object) -> Int
/// C: `ptrdiff_t list_length (Lisp_Object)` (src/lisp.h 4243).
@_silgen_name("list_length") func list_length_local(_ list: Lisp_Object) -> Int
/// C: `+[NSString stringWithLispString:]` (declared src/nsterm.h 367, defined src/nsfns.m
/// 4104) - a construct of this lane's own headers, reached as a C-ABI entry; the port of
/// src/nsfns.m owns the definition.
@_silgen_name("stringWithLispString") func stringWithLispString(_ string: Lisp_Object) -> NSString?

// --------------------------------------------------------------------------
// Lane ABI: what the driver entry points add to the list above.
// --------------------------------------------------------------------------

/// C: `void *xzalloc (size_t)` (src/lisp.h 5537, the gnulib allocator the port defines in
/// src/alloc.c).
@_silgen_name("xzalloc") func xzalloc(_ size: Int) -> UnsafeMutableRawPointer?
/// C: `Lisp_Object font_make_object (int, Lisp_Object, int)` (src/font.h 846, defined
/// src/font.c).
@_silgen_name("font_make_object")
func font_make_object(_ size: Int32, _ entity: Lisp_Object, _ pixel_size: Int32) -> Lisp_Object
/// C: `Lisp_Object Ffont_xlfd_name (Lisp_Object, Lisp_Object, Lisp_Object)` (globals.h 4765,
/// defined src/font.c).
@_silgen_name("Ffont_xlfd_name")
func Ffont_xlfd_name(_ font: Lisp_Object, _ foundry: Lisp_Object, _ family: Lisp_Object) -> Lisp_Object
/// C: `Lisp_Object get_frame_param (struct frame *, Lisp_Object)` (src/lisp.h 5120).
@_silgen_name("get_frame_param")
func get_frame_param(_ f: UnsafeMutableRawPointer?, _ param: Lisp_Object) -> Lisp_Object
/// C: `AVOID memory_full_up (void)` (src/lisp.h 4433) - it does not return.
@_silgen_name("memory_full_up") func memory_full_up() -> Never
/// C: `void register_font_driver (struct font_driver const *, struct frame *)` (src/font.h
/// 896, defined src/font.c).
@_silgen_name("register_font_driver")
func register_font_driver(_ driver: UnsafePointer<font_driver>?, _ f: UnsafeMutableRawPointer?)
/// C: `extern void defvar_lisp (struct Lisp_Fwd const *, char const *)` (src/lisp.h 3473).
@_silgen_name("defvar_lisp")
func defvar_lisp(_ o_fwd: UnsafePointer<Lisp_Fwd>?, _ namestring: UnsafePointer<CChar>?)

// Header-only constructs of the driver half (macros, INLINE functions, enumerators); the core
// exports each C-ABI entry under the C construct's own name, as in the list above.
/// C: `#define VECSIZE(type) ((sizeof (type) - header_size + word_size - 1) / word_size)`
/// (src/lisp.h 2015-2016); `header_size`/`word_size` are lisp.h's two enum constants
/// (2010-2013).  The core mirror spells the macro as a `const fn` over the type's size
/// (`femacs/src/include/lisp.rs` 2925), which is the entry this declaration matches.
@_silgen_name("VECSIZE") func VECSIZE(_ size: Int) -> Int
/// C: `INLINE struct font *XFONT_OBJECT (Lisp_Object)` (src/font.h 539).
@_silgen_name("XFONT_OBJECT")
func XFONT_OBJECT(_ p: Lisp_Object) -> UnsafeMutablePointer<font>?
/// C: `INLINE struct font *CHECK_FONT_GET_OBJECT (Lisp_Object)` (src/font.h 553).
@_silgen_name("CHECK_FONT_GET_OBJECT")
func CHECK_FONT_GET_OBJECT(_ x: Lisp_Object) -> UnsafeMutablePointer<font>?
/// C: `INLINE Lisp_Object build_unibyte_string (const char *)` (src/lisp.h 4556).
@_silgen_name("build_unibyte_string")
func build_unibyte_string(_ str: UnsafePointer<CChar>?) -> Lisp_Object
/// C: `INLINE EMACS_INT XFIXNAT (Lisp_Object)` (src/lisp.h 3246).
@_silgen_name("XFIXNAT") func XFIXNAT(_ a: Lisp_Object) -> Int
/// C: `enum font_property_index`'s `FONT_NAME_INDEX` (src/font.h 177) and
/// `FONT_FULLNAME_INDEX` (182) - the two slots `nsfont_open` fills.
@_silgen_name("FONT_NAME_INDEX") func FONT_NAME_INDEX() -> Int
@_silgen_name("FONT_FULLNAME_INDEX") func FONT_FULLNAME_INDEX() -> Int
/// C: `Lisp_Fwd_Obj` (src/lisp.h 527-532), the `enum Lisp_Fwd_Type` member `DEFVAR_LISP` stores.
@_silgen_name("Lisp_Fwd_Obj") func Lisp_Fwd_Obj() -> UInt8
/// C: `typedef void (*pdumper_hook) (void);` (src/pdumper.h 87) and
/// `INLINE void pdumper_do_now_and_after_load (pdumper_hook)` (92-99).
@_silgen_name("pdumper_do_now_and_after_load")
func pdumper_do_now_and_after_load(_ hook: (@convention(c) () -> Void)?)
/// C: `#define Qfoo builtin_lisp_symbol (...)`, one per symbol, generated into globals.h
/// (PORTING.md P-17); called as `Qfoo ()`, the Rust mirror's shape.
@_silgen_name("Qfontsize") func Qfontsize() -> Lisp_Object
/// C: `#define LGSTRING_HEADER(lgs) AREF (lgs, 0)` (src/composite.h 252) and the fifteen
/// LGSTRING/LGLYPH macros `nsfont_shape` reads (composite.h 258-325).  Each entry's
/// signature is C's own expression: `LGSTRING_GLYPH_LEN`'s `ASIZE` is the `ptrdiff_t` the
/// macro's callers use, `LGLYPH_CHAR`'s `XFIXNUM` is `EMACS_INT`, `LGLYPH_SET_CODE`'s value
/// is `unsigned long` (`INT_TO_INTEGER`), the index-taking entries take the `int` of
/// `LGLYPH_IX_*`, and `LGLYPH_SET_CHAR`'s value is the `UTF32Char` of its only C call site
/// (src/nsfont.m 1520).
@_silgen_name("LGSTRING_HEADER")
func LGSTRING_HEADER(_ lgs: Lisp_Object) -> Lisp_Object
@_silgen_name("LGSTRING_FONT")
func LGSTRING_FONT(_ lgs: Lisp_Object) -> Lisp_Object
@_silgen_name("LGSTRING_GLYPH_LEN")
func LGSTRING_GLYPH_LEN(_ lgs: Lisp_Object) -> Int
@_silgen_name("LGSTRING_GLYPH")
func LGSTRING_GLYPH(_ lgs: Lisp_Object, _ idx: Int) -> Lisp_Object
@_silgen_name("LGSTRING_SET_GLYPH")
func LGSTRING_SET_GLYPH(_ lgs: Lisp_Object, _ idx: Int, _ val: Lisp_Object)
@_silgen_name("LGLYPH_NEW") func LGLYPH_NEW() -> Lisp_Object
@_silgen_name("LGLYPH_CHAR") func LGLYPH_CHAR(_ g: Lisp_Object) -> Int
@_silgen_name("LGLYPH_SET_FROM") func LGLYPH_SET_FROM(_ g: Lisp_Object, _ val: Int)
@_silgen_name("LGLYPH_SET_TO") func LGLYPH_SET_TO(_ g: Lisp_Object, _ val: Int)
@_silgen_name("LGLYPH_SET_CHAR") func LGLYPH_SET_CHAR(_ g: Lisp_Object, _ val: UInt32)
@_silgen_name("LGLYPH_SET_CODE") func LGLYPH_SET_CODE(_ g: Lisp_Object, _ val: UInt)
@_silgen_name("LGLYPH_SET_WIDTH") func LGLYPH_SET_WIDTH(_ g: Lisp_Object, _ val: Int)
@_silgen_name("LGLYPH_SET_LBEARING") func LGLYPH_SET_LBEARING(_ g: Lisp_Object, _ val: Int)
@_silgen_name("LGLYPH_SET_RBEARING") func LGLYPH_SET_RBEARING(_ g: Lisp_Object, _ val: Int)
@_silgen_name("LGLYPH_SET_ASCENT") func LGLYPH_SET_ASCENT(_ g: Lisp_Object, _ val: Int)
@_silgen_name("LGLYPH_SET_DESCENT") func LGLYPH_SET_DESCENT(_ g: Lisp_Object, _ val: Int)

/* ==========================================================================

    Font driver implementation (src/nsfont.m 785-1765, the GNUstep arm), in C's own order.
    `nsfont_draw` (1151) and `ns_dump_glyphstring` (1711) are the two bodies this file cannot
    carry: both dereference `struct glyph_string`, and this tree declares that record only as
    the opaque P-70 placeholder (`femacs/src/font.rs` 739) because font.c never dereferences
    it.  They are absent here and named in docs/ports/nsfont.md's `## Deferred`.

   ========================================================================== */

/// C: `ns_findfonts` (src/nsfont.m 785-830).  `GSFontEnumerator` is a GNUstep class (there is
/// no Cocoa counterpart in this file - see the header comment), so the two messages are
/// declared as C-ABI entries whose provider is the GNUstep libraries.
fileprivate func ns_findfonts(_ font_spec: Lisp_Object, _ isMatch: Bool) -> Lisp_Object {
  var tem: Lisp_Object
  var list: Lisp_Object = Qnil()
  var fdesc: NSFontDescriptor?
  var all_descs: NSArray?
  let enumerator = GSFontEnumerator_sharedEnumerator()
  var cFamilies: NSSet

  block_input()
  if NSFONT_TRACE != 0 {
    fprintf(stderr, (isMatch ? "nsfont: match for fontspec:\n    " : "nsfont: list for fontspec:\n    "))
    debug_print(font_spec)
  }

  cFamilies = ns_get_covering_families(ns_get_req_script(font_spec), 0.90)

  fdesc = ns_spec_to_descriptor(font_spec)
  all_descs = GSFontEnumerator_availableFontDescriptors(enumerator)

  var i = 0
  while i < (all_descs?.count ?? 0) {
    let desc = all_descs!.object(at: i) as! NSFontDescriptor
    i += 1

    if !cFamilies.contains(desc.object(forKey: NSFontDescriptor.AttributeName.family)!) {
      continue
    }
    if !ns_font_descs_match_p(fdesc!, desc) {
      continue
    }

    tem = ns_descriptor_to_entity(desc, AREF(font_spec, FONT_EXTRA_INDEX()), nil)
    if isMatch {
      return tem
    }
    list = Fcons(tem, list)
  }

  unblock_input()

  /* Return something if was a match and nothing found.  */
  if isMatch {
    return ns_fallback_entity()
  }

  if NSFONT_TRACE != 0 {
    // C's format is `"    Returning %"pD"d entities.\n"` (src/nsfont.m 823-824).
    ("    Returning %" + String(cString: pD()) + "d entities.\n")
      .withCString { format in
        fprintf(stderr, format, list_length_local(list))
      }
  }

  return list
}

/// C: `nsfont_get_cache` (src/nsfont.m 848-853).
fileprivate func nsfont_get_cache(_ frame: UnsafeMutableRawPointer?) -> Lisp_Object {
  let dpyinfo = FRAME_DISPLAY_INFO(frame)
  return dpyinfo.pointee.name_list_element
}

/// C: `nsfont_list` (src/nsfont.m 868-871).
fileprivate func nsfont_list(_ f: UnsafeMutableRawPointer?, _ font_spec: Lisp_Object) -> Lisp_Object {
  _ = f
  return ns_findfonts(font_spec, false)
}

/// C: `nsfont_match` (src/nsfont.m 879-882).
fileprivate func nsfont_match(_ f: UnsafeMutableRawPointer?, _ font_spec: Lisp_Object) -> Lisp_Object {
  _ = f
  return ns_findfonts(font_spec, true)
}

/// C: `nsfont_list_family` (src/nsfont.m 888-909).
fileprivate func nsfont_list_family(_ f: UnsafeMutableRawPointer?) -> Lisp_Object {
  var list: Lisp_Object = Qnil()

  _ = f
  block_input()
  // C enumerates `[[[NSFontManager sharedFontManager] availableFontFamilies]
  // objectEnumerator]` and interns each `[family UTF8String]`.
  let families = NSFontManager.shared.availableFontFamilies
  for family in families {
    family.withCString { p in
      list = Fcons(intern(p), list)
    }
  }

  if NSFONT_TRACE != 0 {
    // C's format is `"nsfont: list families returning %"pD"d entries.\n"` (src/nsfont.m
    // 903-904): `pD` is a string macro, so the port composes the same text at runtime and
    // hands it to `fprintf` as a C string.
    ("nsfont: list families returning %" + String(cString: pD()) + "d entries.\n")
      .withCString { format in
        fprintf(stderr, format, list_length_local(list))
      }
  }

  unblock_input()
  return list
}

/// C: `nsfont_open` (src/nsfont.m 912-1047).
fileprivate func nsfont_open(_ f: UnsafeMutableRawPointer?, _ font_entity: Lisp_Object,
                             _ pixel_size_in: Int32) -> Lisp_Object {
  var font_info: UnsafeMutablePointer<nsfont_info>
  let fontDesc = ns_spec_to_descriptor(font_entity)
  let fontMgr = NSFontManager.shared
  var family: NSString?
  var nsfont: NSFont?
  var sfont: NSFont?
  var brect = NSRect()
  var font_object: Lisp_Object
  var tem: Lisp_Object
  var pixel_size = pixel_size_in

  block_input()

  if NSFONT_TRACE != 0 {
    fprintf(stderr, "nsfont: open size %d of fontentity:\n    ", pixel_size)
    debug_print(font_entity)
  }

  if pixel_size <= 0 {
    /* try to get it out of frame params */
    tem = get_frame_param(f, Qfontsize())
    pixel_size = NILP(tem) ? 0 : Int32(XFIXNAT(tem))
  }

  /* C: `tem = AREF (font_entity, FONT_ADSTYLE_INDEX);` (930) - the reference stores the
     adstyle in `tem` and never reads it; the (absent) effect is what the port keeps.  */
  _ = AREF(font_entity, FONT_ADSTYLE_INDEX())
  /* C: `family = ns_get_family (font_entity); if (family == nil) family = [[NSFont
     userFixedPitchFontOfSize: 0] familyName];` (931-933) - `family` is written and never
     read in the reference either; the two calls are the reference's, the value is not.  */
  family = ns_get_family(font_entity)
  if family == nil {
    family = NSFont.userFixedPitchFont(ofSize: 0)?.familyName as NSString?
  }
  _ = family

  /* C: `nsfont = [NSFont fontWithDescriptor: fontDesc size: pixel_size];` (935-936).  A nil
     `fontDesc` makes C's message return nil; the Swift initialiser takes a non-optional
     descriptor, so that case is spelled out.  */
  if fontDesc != nil {
    nsfont = NSFont(descriptor: fontDesc!, size: CGFloat(pixel_size))
  }

  if nsfont == nil {
    nsfont = NSFont.userFixedPitchFont(ofSize: CGFloat(pixel_size))
  }

  if NSFONT_TRACE != 0 {
    NSLog("%@\n", nsfont!)
  }

  font_object = font_make_object(Int32(VECSIZE(MemoryLayout<nsfont_info>.size)),
                                 font_entity, pixel_size)
  ASET(font_object, FONT_TYPE_INDEX(), Qns())
  let font_object_ptr = XFONT_OBJECT(font_object)
  if font_object_ptr == nil {
    unblock_input()
    return Qnil()
  }
  /* C: `font_info = (struct nsfont_info *) XFONT_OBJECT (font_object); font = (struct font *)
     font_info;` (952-953).  `struct nsfont_info`'s first member *is* the `struct font`, so
     the two C names are one address; the port drops the redundant alias and writes the
     `font->…` members as `font_info.pointee.font.…`.  */
  font_info = UnsafeMutableRawPointer(font_object_ptr!).assumingMemoryBound(to: nsfont_info.self)

  font_info.pointee.glyphs =
    xzalloc(0x100 * MemoryLayout<UnsafeMutablePointer<UInt32>?>.size)!
      .assumingMemoryBound(to: UnsafeMutablePointer<UInt32>?.self)
  font_info.pointee.metrics =
    xzalloc(0x100 * MemoryLayout<UnsafeMutablePointer<font_metrics>?>.size)!
      .assumingMemoryBound(to: UnsafeMutablePointer<font_metrics>?.self)

  /* for metrics */
  sfont = nsfont!.screen

  if sfont == nil {
    sfont = nsfont
  }

  /* non-metric backend font struct fields */
  font_info.pointee.font.pixel_size = Int32(sfont!.pointSize)
  font_info.pointee.font.driver = UnsafeRawPointer(nsfont_driver)
  font_info.pointee.font.encoding_charset = -1
  font_info.pointee.font.repertory_charset = -1
  font_info.pointee.font.default_ascent = 0
  font_info.pointee.font.vertical_centering = false
  font_info.pointee.font.baseline_offset = 0
  font_info.pointee.font.relative_compose = 0

  do {
    let fontName = nsfont!.fontName

    /* The values specified by fonts are not always exact. For
     * example, a 6x8 font could specify that the descender is
     * -2.00000405... (represented by 0xc000000220000000).  Without
     * adjustment, the code below would round the descender to -3,
     * resulting in a font that would be one pixel higher than
     * intended.  */
    let adjusted_descender = sfont!.descender + 0.0001

    font_info.pointee.nsfont = Unmanaged.passUnretained(sfont!).toOpaque()
    _ = Unmanaged<NSFont>.fromOpaque(font_info.pointee.nsfont!).retain()

    /* set up ns_font (defined in nsgui.h) */
    font_info.pointee.name = xstrdup(fontName)
    font_info.pointee.bold =
      CChar(truncatingIfNeeded:
        fontMgr.traits(of: nsfont!).rawValue & NSFontTraitMask.boldFontMask.rawValue)
    font_info.pointee.ital =
      CChar(truncatingIfNeeded:
        fontMgr.traits(of: nsfont!).rawValue & NSFontTraitMask.italicFontMask.rawValue)

    /* Metrics etc.; some fonts return an unusually large max advance, so we
       only use it for fonts that have wide characters.  */
    font_info.pointee.width = (sfont!.numberOfGlyphs > 2000)
      ? Float(sfont!.maximumAdvancement.width) : Float(ns_char_width(sfont!, 0x30))

    brect = sfont!.boundingRectForFont

    font_info.pointee.underpos = Float(sfont!.underlinePosition)
    font_info.pointee.underwidth = Float(sfont!.underlineThickness)
    font_info.pointee.size = Float(font_info.pointee.font.pixel_size)

    /* max bounds */
    font_info.pointee.max_bounds.ascent = Int32(lrint(sfont!.ascender))
    font_info.pointee.font.ascent = font_info.pointee.max_bounds.ascent
    /* Descender is usually negative.  Use floor to avoid
       clipping descenders.  */
    font_info.pointee.max_bounds.descent = -Int32(lrint(floor(adjusted_descender)))
    font_info.pointee.font.descent = font_info.pointee.max_bounds.descent
    font_info.pointee.height =
      Float(font_info.pointee.max_bounds.ascent + font_info.pointee.max_bounds.descent)
    font_info.pointee.max_bounds.width = Int32(lrint(Double(font_info.pointee.width)))
    font_info.pointee.max_bounds.lbearing = Int32(lrint(brect.origin.x))
    font_info.pointee.max_bounds.rbearing =
      Int32(lrint(brect.size.width - CGFloat(font_info.pointee.width)))

    /* set up metrics portion of font struct */
    font_info.pointee.font.space_width = Int32(lrint(ns_char_width(sfont!, 0x20)))
    font_info.pointee.font.max_width = Int32(lrint(Double(font_info.pointee.max_bounds.width)))
    font_info.pointee.font.min_width = font_info.pointee.font.space_width  /* Approximate.  */
    font_info.pointee.font.average_width = ns_ascii_average_width(sfont!)

    font_info.pointee.font.height = Int32(lrint(Double(font_info.pointee.height)))
    font_info.pointee.font.underline_position = Int32(lrint(Double(font_info.pointee.underpos)))
    font_info.pointee.font.underline_thickness = Int32(lrint(Double(font_info.pointee.underwidth)))

    /* C: `font->props[FONT_NAME_INDEX] = …` - `props` is C's fixed 17-slot array, so the
       port re-reads it as `Lisp_Object *` and keeps C's index expression.  */
    withUnsafeMutablePointer(to: &font_info.pointee.font.props) { propPtr in
      propPtr.withMemoryRebound(to: Lisp_Object.self, capacity: 17) { slots in
        slots[FONT_NAME_INDEX()] = Ffont_xlfd_name(font_object, Qnil(), Qnil())
        slots[FONT_FULLNAME_INDEX()] = build_unibyte_string(font_info.pointee.name)
      }
    }
  }
  unblock_input()

  return font_object
}

/// C: `nsfont_close` (src/nsfont.m 1049-1075).
fileprivate func nsfont_close(_ font: UnsafeMutablePointer<font>?) {
  let font_info = font.map { UnsafeMutableRawPointer($0).assumingMemoryBound(to: nsfont_info.self) }

  if let font_info = font_info, font_info.pointee.name != nil {
    var i: Int32

    i = 0
    while i < 0x100 {
      xfree(UnsafeMutableRawPointer(font_info.pointee.glyphs?[Int(i)]))
      xfree(UnsafeMutableRawPointer(font_info.pointee.metrics?[Int(i)]))
      i += 1
    }
    xfree(UnsafeMutableRawPointer(font_info.pointee.glyphs))
    xfree(UnsafeMutableRawPointer(font_info.pointee.metrics))
    // C: `[font_info->nsfont release]` - the +1 that `nsfont_open`'s `retain` created.
    if let nsfont = font_info.pointee.nsfont {
      Unmanaged<AnyObject>.fromOpaque(nsfont).release()
    }
    xfree(UnsafeMutableRawPointer(font_info.pointee.name))
    font_info.pointee.name = nil
  }
}

/// C: `nsfont_has_char` (src/nsfont.m 1077-1081).
fileprivate func nsfont_has_char(_ entity: Lisp_Object, _ c: Int32) -> Int32 {
  return -1
}

/// C: `nsfont_encode_char` (src/nsfont.m 1086-1100).
fileprivate func nsfont_encode_char(_ font: UnsafeMutablePointer<font>?, _ c: Int32) -> UInt32 {
  // C casts the pointer and dereferences it without a nil test.
  let font_info = UnsafeMutableRawPointer(font!).assumingMemoryBound(to: nsfont_info.self)
  let high = UInt8(truncatingIfNeeded: (c & 0xff00) >> 8)
  let low = UInt8(truncatingIfNeeded: c & 0x00ff)
  var g: UInt32

  if c > 0xFFFF {
    return FONT_INVALID_CODE()
  }

  /* Did we already cache this block?  */
  if font_info.pointee.glyphs?[Int(high)] == nil {
    ns_uni_to_glyphs(font_info, high)
  }

  g = font_info.pointee.glyphs![Int(high)]![Int(low)]
  return g == INVALID_GLYPH ? FONT_INVALID_CODE() : g
}

/// C: `nsfont_text_extents` (src/nsfont.m 1108-1147).
fileprivate func nsfont_text_extents(_ font: UnsafeMutablePointer<font>?,
                                     _ code: UnsafePointer<UInt32>?,
                                     _ nglyphs: Int32,
                                     _ metrics: UnsafeMutablePointer<font_metrics>?) {
  // C casts the pointer and dereferences it without a nil test.
  let font_info = UnsafeMutableRawPointer(font!).assumingMemoryBound(to: nsfont_info.self)
  var pcm: UnsafeMutablePointer<font_metrics>
  var high: UInt8
  var low: UInt8
  var totalWidth: Int32 = 0
  var i: Int32

  // C: `memset (metrics, 0, sizeof (struct font_metrics));` - an all-zero `font_metrics`.
  metrics!.pointee = font_metrics(lbearing: 0, rbearing: 0, width: 0, ascent: 0, descent: 0)

  i = 0
  while i < nglyphs {
    /* get metrics for this glyph, filling cache if need be */
    /* TODO: get metrics for whole string from an NSLayoutManager
             (if not too slow) */
    high = UInt8(truncatingIfNeeded: (code![Int(i)] & 0xFF00) >> 8)
    low = UInt8(truncatingIfNeeded: code![Int(i)] & 0x00FF)
    if font_info.pointee.metrics![Int(high)] == nil {
      ns_glyph_metrics(font_info, UInt32(high))
    }
    pcm = font_info.pointee.metrics![Int(high)]! + Int(low)

    if Int32(metrics!.pointee.lbearing) > totalWidth + Int32(pcm.pointee.lbearing) {
      metrics!.pointee.lbearing = Int16(truncatingIfNeeded: totalWidth + Int32(pcm.pointee.lbearing))
    }
    if Int32(metrics!.pointee.rbearing) < totalWidth + Int32(pcm.pointee.rbearing) {
      metrics!.pointee.rbearing = Int16(truncatingIfNeeded: totalWidth + Int32(pcm.pointee.rbearing))
    }
    if metrics!.pointee.ascent < pcm.pointee.ascent {
      metrics!.pointee.ascent = pcm.pointee.ascent
    }
    if metrics!.pointee.descent < pcm.pointee.descent {
      metrics!.pointee.descent = pcm.pointee.descent
    }

    totalWidth += Int32(pcm.pointee.width)
    i += 1
  }

  metrics!.pointee.width = Int16(truncatingIfNeeded: totalWidth)
}

/// C: `nsfont_draw` (src/nsfont.m 1151-1212) - the GNUstep driver's draw entry (C 1740's
/// `.draw = nsfont_draw`), and one of the two bodies this file was waiting for.  Readings:
/// `struct glyph_string` is this tree's opaque placeholder (`femacs/src/font.rs` 739, P-70:
/// font.c never dereferences it), so every access is a reader entry of the `glyph_string_*`
/// family (`## ABI` (2f)); `s->first_glyph->type` is the `glyph_type` reader against the
/// `COMPOSITE_GLYPH` enumerator; C's `alloca` is `withUnsafeTemporaryAllocation` (the C
/// 1491/1514 reading), and its `NSGlyph` is the SDK's `unsigned int` (NSFont.h 147) - the same
/// width as `char2b`'s `unsigned` (src/dispextern.h 1407), so C's `c[i] = s->char2b[i + from]`
/// is an exact element copy (no `CGGlyph`-style truncation); C 1166-1167's nil `s->font` arm
/// reads `FRAME_FONT (s->f)`, whose value is the same `struct nsfont_info` address (`struct
/// nsfont_info`'s first member *is* the `struct font`, the `nsfont_open` reading); and C
/// 1200-1207's DPS block is GNUstep-only text this file carries in full (`## Decisions`), its
/// two callees being declared entries.
fileprivate func nsfont_draw(_ s: UnsafeMutableRawPointer?, _ from: Int32, _ to: Int32,
                             _ x: Int32, _ y: Int32, _ with_background: Bool) -> Int32 {
  let len = to - from
  let isComposite = glyph_type(glyph_string_first_glyph(s)) == COMPOSITE_GLYPH()
  var face: UnsafeMutableRawPointer?
  var r = NSRect()
  var font: UnsafeMutableRawPointer?
  var col: NSColor?

  block_input()

  font = glyph_string_font(s)
  if font == nil {
    font = FRAME_FONT(glyph_string_f(s))
  }

  face = glyph_string_face(s)

  r.origin.x = CGFloat(x)
  r.origin.y = CGFloat(y)
  r.size.height = CGFloat(FONT_HEIGHT(font))

  return withUnsafeTemporaryAllocation(of: NSGlyph.self, capacity: max(Int(len), 1)) { buffer in
    let c = buffer.baseAddress!

    for i in 0..<Int(max(len, 0)) {
      c[i] = glyph_string_char2b(s)![Int(from) + i]
    }

    /* Fill background if requested.  */
    if with_background && !isComposite {
      let br = NSRect(x: CGFloat(x),
                      y: CGFloat(y) - CGFloat(FONT_BASE(glyph_string_font(s))),
                      width: CGFloat(glyph_string_width(s)),
                      height: CGFloat(FONT_HEIGHT(glyph_string_font(s))))
      if glyph_string_hl(s) != DRAW_CURSOR() {
        (NS_FACE_BACKGROUND(face) != 0
         ? colorWithUnsignedLong(unsafeBitCast(NSColor.self,
                                               to: UnsafeMutableRawPointer.self),
                                 NS_FACE_BACKGROUND(face))
         : FRAME_BACKGROUND_COLOR(glyph_string_f(s)))!.set()
      } else {
        FRAME_CURSOR_COLOR(glyph_string_f(s))!.set()
      }
      NSRectFill(br)
    }

    /* set up for character rendering */
    if glyph_string_hl(s) == DRAW_CURSOR() {
      col = FRAME_BACKGROUND_COLOR(glyph_string_f(s))
    } else {
      col = (NS_FACE_FOREGROUND(face) != 0
             ? colorWithUnsignedLong(unsafeBitCast(NSColor.self,
                                                   to: UnsafeMutableRawPointer.self),
                                     NS_FACE_FOREGROUND(face))
             : FRAME_FOREGROUND_COLOR(glyph_string_f(s)))
    }

    /* render under GNUstep using DPS */
    do {
      let context = NSGraphicsContext.current
      let font_info = UnsafeMutableRawPointer(font!).assumingMemoryBound(to: nsfont_info.self)
      Unmanaged<NSFont>.fromOpaque(font_info.pointee.nsfont!).takeUnretainedValue().set()
      col!.set()
      DPSmoveto(context, r.origin.x, r.origin.y)
      GSShowGlyphs(context, c, Int(len))
    }

    unblock_input()
    return to - from
  }
}

/// C: `ns_font_shape` (src/nsfont.m 1214-1462).  `[textStorage setFont:]` is GNUstep's own
/// `NSMutableAttributedString` extension, so it is a declared C-ABI entry like
/// `GSFontEnumerator`; everything else is the SDK's own `NSLayoutManager` glyph API (some of it
/// deprecated - the warnings it raises are the reference's own API choice).
///
/// C's `#define RIGHT_TO_LEFT_P permutation` (1304, `#undef` 1454) is a textual alias for the
/// `permutation` pointer.  The reference **never allocates it** (`permutation = NULL` at 1303
/// is its only assignment, and the `dir` parameter is never read), so `RIGHT_TO_LEFT_P` is
/// constant-false and every branch behind it is dead text; the port keeps the same expressions
/// (`permutation != nil`), which is why they are written but never taken.
fileprivate func ns_font_shape(_ font: NSFont, _ string: NSString,
                               _ glyph_layouts: UnsafeMutablePointer<ns_glyph_layout>,
                               _ glyph_len: Int, _ dir: lgstring_direction) -> Int {
  var result = 0
  var textStorage: NSTextStorage
  var layoutManager: NSLayoutManager
  var textContainer: NSTextContainer
  let stringLength: Int
  var spaceLocation = NSPoint()
  /* numberOfGlyphs can't actually be 0, but this pacifies GCC */
  var used: Int = 0
  var numberOfGlyphs: Int = 0

  _ = dir

  textStorage = NSTextStorage(string: string as String)
  layoutManager = NSLayoutManager()
  textContainer = NSTextContainer()

  /* Append a trailing space to measure baseline position.  */
  textStorage.append(NSAttributedString(string: " "))
  setFont(textStorage, font)
  textContainer.lineFragmentPadding = 0

  layoutManager.addTextContainer(textContainer)
  textStorage.addLayoutManager(layoutManager)

  // C: `if (!(textStorage && layoutManager && textContainer)) emacs_abort ();` (src/nsfont.m
  // 1243-1244) - the three objects are non-optional here (the SDK's initializers do not
  // return nil), so C's only failure mode, a failed allocation, has no Swift spelling; the
  // check is recorded, not transcribed.

  stringLength = string.length

  /* Force layout.  */
  _ = layoutManager.glyphRange(for: textContainer)

  spaceLocation = layoutManager.location(forGlyphAt: stringLength)

  /* Remove the appended trailing space because otherwise it may
     generate a wrong result for a right-to-left text.  */
  textStorage.beginEditing()
  textStorage.deleteCharacters(in: NSRange(location: stringLength, length: 1))
  textStorage.endEditing()
  _ = layoutManager.glyphRange(for: textContainer)

  var i = 0
  while i < stringLength {
    var range = NSRange()
    let fontInTextStorage =
      textStorage.attribute(.font, at: i, longestEffectiveRange: &range,
                            in: NSRange(location: 0, length: stringLength)) as? NSFont

    if !((fontInTextStorage as AnyObject?) === font
         || fontInTextStorage?.fontName == font.fontName) {
      break
    }
    i = NSMaxRange(range)
  }
  if i < stringLength {
    /* Make the test `used <= glyph_len' below fail if textStorage
       contained some fonts other than the specified one.  */
    used = glyph_len + 1
  } else {
    var range = NSRange(location: 0, length: stringLength)

    range = layoutManager.glyphRange(forCharacterRange: range, actualCharacterRange: nil)
    numberOfGlyphs = NSMaxRange(range)
    used = numberOfGlyphs
    i = 0
    while i < numberOfGlyphs {
      if layoutManager.notShownAttribute(forGlyphAt: i) {
        used -= 1
      }
      i += 1
    }
  }

  if 0 < used && used <= glyph_len {
    var glyphIndex: Int
    var prevGlyphIndex: Int
    var permutation: UnsafeMutablePointer<Int>?
    var compRange = NSRange()
    var range = NSRange()
    var totalAdvance: CGFloat

    /* C: `#define RIGHT_TO_LEFT_P permutation`.  The reference's only assignment is the NULL
       below, so this is always false and the branches behind it are dead text.  */
    permutation = nil

    glyphIndex = 0
    while layoutManager.notShownAttribute(forGlyphAt: glyphIndex) {
      glyphIndex += 1
    }

    /* Fill the `comp_range' member of struct ns_glyph_layout, and
       setup a permutation for right-to-left text.  */
    compRange = NSRange(location: 0, length: 0)
    range = NSRange(location: 0, length: 0)
    while NSMaxRange(range) < used {
      let gl = glyph_layouts + NSMaxRange(range)
      var characterIndex = layoutManager.characterIndexForGlyph(at: glyphIndex)

      gl.pointee.string_index = UInt(characterIndex)

      if characterIndex >= NSMaxRange(compRange) {
        compRange.location = NSMaxRange(compRange)
        repeat {
          var characterRange = string.rangeOfComposedCharacterSequence(at: characterIndex)

          compRange.length = NSMaxRange(characterRange) - compRange.location
          _ = layoutManager.glyphRange(forCharacterRange: compRange,
                                       actualCharacterRange: &characterRange)
          characterIndex = NSMaxRange(characterRange) - 1
        } while characterIndex >= NSMaxRange(compRange)

        if permutation != nil {
          var n = 0
          while n < range.length {
            permutation![range.location + n] = NSMaxRange(range) - n - 1
            n += 1
          }
        }

        range = NSRange(location: NSMaxRange(range), length: 0)
      }

      gl.pointee.comp_range.location = compRange.location
      gl.pointee.comp_range.length = compRange.length

      glyphIndex += 1
      while glyphIndex < numberOfGlyphs {
        if !layoutManager.notShownAttribute(forGlyphAt: glyphIndex) {
          break
        }
        glyphIndex += 1
      }
      range.length += 1
    }
    if permutation != nil {
      var n = 0
      while n < range.length {
        permutation![range.location + n] = NSMaxRange(range) - n - 1
        n += 1
      }
    }

    /* Then fill the remaining members.  */
    glyphIndex = 0
    prevGlyphIndex = 0
    while layoutManager.notShownAttribute(forGlyphAt: glyphIndex) {
      glyphIndex += 1
    }

    if permutation == nil {
      totalAdvance = 0
    } else {
      var nrects: Int = 0
      let glyphRects = layoutManager.rectArray(forGlyphRange: NSRange(location: 0, length: numberOfGlyphs),
                                               withinSelectedGlyphRange: NSRange(location: NSNotFound, length: 0),
                                               in: textContainer, rectCount: &nrects)

      totalAdvance = NSMaxX(glyphRects![0])
    }

    i = 0
    while i < used {
      var gl: UnsafeMutablePointer<ns_glyph_layout>
      var location = NSPoint()
      var nextGlyphIndex: Int
      var glyphRange = NSRange()
      var nrects: Int = 0

      if permutation == nil {
        gl = glyph_layouts + i
      } else {
        let dest = permutation![i]

        gl = glyph_layouts + dest
        if i < dest {
          let tmp = gl.pointee.string_index

          gl.pointee.string_index = glyph_layouts[i].string_index
          glyph_layouts[i].string_index = tmp
        }
      }
      gl.pointee.glyph_id = layoutManager.glyph(at: glyphIndex)

      location = layoutManager.location(forGlyphAt: glyphIndex)
      gl.pointee.baseline_delta = spaceLocation.y - location.y

      nextGlyphIndex = glyphIndex + 1
      while nextGlyphIndex < numberOfGlyphs {
        if !layoutManager.notShownAttribute(forGlyphAt: nextGlyphIndex) {
          break
        }
        nextGlyphIndex += 1
      }

      if permutation == nil {
        var maxX: CGFloat

        if prevGlyphIndex == 0 {
          glyphRange = NSRange(location: 0, length: nextGlyphIndex)
        } else {
          glyphRange = NSRange(location: glyphIndex, length: nextGlyphIndex - glyphIndex)
        }
        let glyphRects = layoutManager.rectArray(forGlyphRange: glyphRange,
                                                 withinSelectedGlyphRange: NSRange(location: NSNotFound, length: 0),
                                                 in: textContainer, rectCount: &nrects)
        maxX = max(NSMaxX(glyphRects![0]), totalAdvance)
        gl.pointee.advance_delta = location.x - totalAdvance
        gl.pointee.advance = maxX - totalAdvance
        totalAdvance = maxX
      } else {
        var minX: CGFloat

        if nextGlyphIndex == numberOfGlyphs {
          glyphRange = NSRange(location: prevGlyphIndex, length: numberOfGlyphs - prevGlyphIndex)
        } else {
          glyphRange = NSRange(location: prevGlyphIndex, length: glyphIndex + 1 - prevGlyphIndex)
        }
        let glyphRects = layoutManager.rectArray(forGlyphRange: glyphRange,
                                                 withinSelectedGlyphRange: NSRange(location: NSNotFound, length: 0),
                                                 in: textContainer, rectCount: &nrects)
        minX = min(NSMinX(glyphRects![0]), totalAdvance)
        gl.pointee.advance = totalAdvance - minX
        totalAdvance = minX
        gl.pointee.advance_delta = location.x - totalAdvance
      }

      prevGlyphIndex = glyphIndex + 1
      glyphIndex = nextGlyphIndex
      i += 1
    }

    if permutation != nil {
      xfree(UnsafeMutableRawPointer(permutation))
    }

    result = used
  }

  return result
}

/// C: `nsfont_shape` (src/nsfont.m 1464-1574).  C's two `alloca` scratch arrays are Swift's
/// `withUnsafeTemporaryAllocation` - the same scoped, stack-first temporary storage.  The
/// capacities are `max (_, 1)` because C's `alloca (0)` never yields NULL while Swift's
/// zero-capacity buffer has no `baseAddress` to hand to the SDK.
fileprivate func nsfont_shape(_ lgstring: Lisp_Object, _ direction: Lisp_Object) -> Lisp_Object {
  let font = CHECK_FONT_GET_OBJECT(LGSTRING_FONT(lgstring))!
  let font_info = UnsafeMutableRawPointer(font).assumingMemoryBound(to: nsfont_info.self)
  var glyph_layouts: UnsafeMutablePointer<ns_glyph_layout>? = nil
  let nsfont = Unmanaged<NSFont>.fromOpaque(font_info.pointee.nsfont!).takeUnretainedValue()
  var glyph_len: Int
  var len: Int
  var i: Int
  var tem: Lisp_Object
  var mb_buf: UnsafeMutablePointer<unichar>? = nil
  var used: UInt = 0

  glyph_len = LGSTRING_GLYPH_LEN(lgstring)
  i = 0
  while i < glyph_len {
    tem = LGSTRING_GLYPH(lgstring, i)

    if NILP(tem) {
      break
    }
    i += 1
  }

  len = i

  if Int(Int32.max) / 2 < len {
    memory_full_up()
  }

  block_input()

  return withUnsafeTemporaryAllocation(of: unichar.self, capacity: max(len, 1)) { mbBuf in
    mb_buf = mbBuf.baseAddress!
    i = 0
    while i < len {
      let c = UInt32(truncatingIfNeeded: LGLYPH_CHAR(LGSTRING_GLYPH(lgstring, i)))

      mb_buf![Int(i)] = unichar(truncatingIfNeeded: c)
      i += 1
    }

    let string = NSString(characters: mb_buf!, length: len)
    unblock_input()

    // C: `if (!string) return Qnil;` (1497-1498).  `+[NSString stringWithCharacters:length:]`
    // returns nil only on failure, and Swift's initialiser is non-failable, so the reference's
    // only failure mode has no Swift spelling; the check is recorded, not transcribed.

    block_input()

    var dir = lgstring_direction.DIR_UNKNOWN

    if EQ(direction, QL2R()) {
      dir = lgstring_direction.DIR_L2R
    } else if EQ(direction, QR2L()) {
      dir = lgstring_direction.DIR_R2L
    }
    return withUnsafeTemporaryAllocation(of: ns_glyph_layout.self,
                                         capacity: max(glyph_len, 1)) { glBuf in
      glyph_layouts = glBuf.baseAddress!
      used = UInt(ns_font_shape(nsfont, string, glyph_layouts!, glyph_len, dir))

      i = 0
      while i < Int(used) {
        var lglyph = LGSTRING_GLYPH(lgstring, i)
        let gl = glyph_layouts! + i
        var from: Int
        var to: Int
        var metrics = font_metrics(lbearing: 0, rbearing: 0, width: 0, ascent: 0, descent: 0)

        if NILP(lglyph) {
          lglyph = LGLYPH_NEW()
          LGSTRING_SET_GLYPH(lgstring, i, lglyph)
        }

        from = gl.pointee.comp_range.location
        LGLYPH_SET_FROM(lglyph, from)

        to = gl.pointee.comp_range.location + gl.pointee.comp_range.length
        LGLYPH_SET_TO(lglyph, to - 1)

        /* LGLYPH_CHAR is used in `describe-char' for checking whether
           the composition is trivial.  */
        do {
          var c: UInt32
          let si = Int(gl.pointee.string_index)

          if mb_buf![si] >= 0xD800 && mb_buf![si] < 0xDC00 {
            c = UInt32(((Int32(mb_buf![si]) - 0xD800) << 10)
                       + (Int32(mb_buf![si + 1]) - 0xDC00) + 0x10000)
          } else {
            c = UInt32(mb_buf![si])
          }

          LGLYPH_SET_CHAR(lglyph, c)
        }

        do {
          let cc = UInt(gl.pointee.glyph_id)

          LGLYPH_SET_CODE(lglyph, cc)
        }

        nsfont_text_extents(font, &gl.pointee.glyph_id, 1, &metrics)
        LGLYPH_SET_WIDTH(lglyph, Int(metrics.width))
        LGLYPH_SET_LBEARING(lglyph, Int(metrics.lbearing))
        LGLYPH_SET_RBEARING(lglyph, Int(metrics.rbearing))
        LGLYPH_SET_ASCENT(lglyph, Int(metrics.ascent))
        LGLYPH_SET_DESCENT(lglyph, Int(metrics.descent))

        i += 1
      }
      unblock_input()

      return make_fixnum(Int(used))
    }
  }
}

/// C: `ns_uni_to_glyphs_1` (src/nsfont.m 1576-1620).  C wraps the `glyphAtIndex:` call in
/// `@try`/`@catch`; Swift has no way to catch an Objective-C exception, so the call is made
/// plain and that guard is recorded as a deviation in the ledger (and in the report).
fileprivate func ns_uni_to_glyphs_1(_ info: UnsafeMutablePointer<nsfont_info>, _ c: UInt32) -> NSGlyph {
  let characters: [unichar] = [unichar(truncatingIfNeeded: c)]
  let string = NSString(characters: characters, length: 1)
  let attributes: [NSAttributedString.Key: Any] =
    [.font: Unmanaged<NSFont>.fromOpaque(info.pointee.nsfont!).takeUnretainedValue()]
  let storage = NSTextStorage(string: string as String, attributes: attributes)
  let text_container = NSTextContainer()
  let manager = NSLayoutManager()

  manager.addTextContainer(text_container)
  storage.addLayoutManager(manager)

  let nsfont = Unmanaged<NSFont>.fromOpaque(info.pointee.nsfont!).takeUnretainedValue()
  let font_in_storage = storage.attribute(.font, at: 0, effectiveRange: nil) as? NSFont
  var glyph: NSGlyph = FONT_INVALID_CODE()

  if (font_in_storage as AnyObject?) === nsfont
      || font_in_storage?.fontName == nsfont.fontName {
    // C wraps this call in `@try`/`@catch (NSException *e) { glyph = 'X'; }`; Swift cannot
    // catch an Objective-C exception, so the call is plain and that guard is a recorded
    // deviation (docs/ports/nsfont.md, Decisions).
    glyph = manager.glyph(at: 0)
  }

  return glyph
}

/// C: `ns_uni_to_glyphs` (src/nsfont.m 1622-1662).
fileprivate func ns_uni_to_glyphs(_ font_info: UnsafeMutablePointer<nsfont_info>, _ block: UInt8) {
  let unichars = xmalloc(0x101 * MemoryLayout<unichar>.size)!.assumingMemoryBound(to: unichar.self)
  var i: UInt32
  var g: UInt32
  var idx: UInt32

  if NSFONT_TRACE != 0 {
    fprintf(stderr, "%p\tFinding glyphs for glyphs in block %d\n",
            font_info, Int32(block))
  }

  block_input()

  font_info.pointee.glyphs![Int(block)] = xmalloc(0x100 * MemoryLayout<UInt32>.size)!
    .assumingMemoryBound(to: UInt32.self)
  if font_info.pointee.glyphs![Int(block)] == nil {
    emacs_abort()
  }

  /* Create a string containing all Unicode characters in this block.  */
  idx = UInt32(block) << 8
  i = 0
  while i < 0x100 {
    if idx < 0xD800 || idx > 0xDFFF {
      unichars[Int(i)] = unichar(truncatingIfNeeded: idx)
    } else {
      unichars[Int(i)] = 0xFEFF
    }
    idx += 1
    i += 1
  }
  unichars[0x100] = 0

  do {
    // C: `glyphs = font_info->glyphs[block];` (non-null after the xmalloc above).
    var glyphs = font_info.pointee.glyphs![Int(block)]!
    i = 0
    while i < 0x100 {
      g = UInt32(unichars[Int(i)])
      let glyph = ns_uni_to_glyphs_1(font_info, g)
      glyphs.pointee = glyph
      glyphs += 1
      i += 1
    }
  }

  unblock_input()
  xfree(UnsafeMutableRawPointer(unichars))
}

/// C: `ns_glyph_metrics` (src/nsfont.m 1664-1709).
fileprivate func ns_glyph_metrics(_ font_info: UnsafeMutablePointer<nsfont_info>, _ block: UInt32) {
  var i: UInt32
  var g: NSGlyph
  var numGlyphs = UInt32(truncatingIfNeeded:
    Unmanaged<NSFont>.fromOpaque(font_info.pointee.nsfont!).takeUnretainedValue().numberOfGlyphs)
  var sfont: NSFont
  var metrics: UnsafeMutablePointer<font_metrics>

  if NSFONT_TRACE != 0 {
    fprintf(stderr, "%p\tComputing metrics for glyphs in block %u\n",
            font_info, block)
  }

  /* not implemented yet (as of startup 0.18), so punt */
  if numGlyphs == 0 {
    numGlyphs = 0x10000
  }

  block_input()
  sfont = Unmanaged<NSFont>.fromOpaque(font_info.pointee.nsfont!).takeUnretainedValue().screen

  font_info.pointee.metrics![Int(block)] =
    xzalloc(0x100 * MemoryLayout<font_metrics>.size)!.assumingMemoryBound(to: font_metrics.self)
  if font_info.pointee.metrics![Int(block)] == nil {
    emacs_abort()
  }

  metrics = font_info.pointee.metrics![Int(block)]!
  g = NSGlyph(block << 8)
  i = 0
  while i < 0x100 && g < NSGlyph(numGlyphs) {
    var w: CGFloat
    var lb: CGFloat
    var rb: CGFloat
    let r = sfont.boundingRect(forGlyph: g)

    w = max(sfont.advancement(forGlyph: g).width, 2.0)
    metrics.pointee.width = Int16(lrint(w))

    lb = NSMinX(r)
    rb = NSMaxX(r)

    metrics.pointee.rbearing = Int16(lrint(rb))
    metrics.pointee.lbearing = Int16(lrint(lb))

    metrics.pointee.descent = Int16(-NSMaxY(r))
    metrics.pointee.ascent = Int16(-NSMinY(r))

    g += 1
    i += 1
    metrics += 1
  }
  unblock_input()
}

/// C: `ns_dump_glyphstring` (src/nsfont.m 1710-1722) - the reference's debugging dumper,
/// exported (declared src/nsterm.h 1118).  Readings: every `s->…` read is a `glyph_string_*`
/// entry (the family `nsfont_draw` above declares), `s->row->overlapping_p` and
/// `s->first_glyph[i].u.ch` are the two reads no earlier body needed (`glyph_row_overlapping_p`
/// and the indexed `glyph_u_ch`, whose index lives in the entry because `struct glyph`'s stride
/// is the core mirror's), C's two `%d` bit-field arguments take the 0/1 C's own promotion
/// produces, and `fprintf`/`putc`/`stderr` are the standard streams (C 1180's reading).
@_cdecl("ns_dump_glyphstring")
func ns_dump_glyphstring(_ s: UnsafeMutableRawPointer?) {
  fprintf(stderr,
          "Glyph string len = %d at (%d, %d) overhang (%d, %d),overlap = %d, bg_filled = %d:",
          glyph_string_nchars(s), glyph_string_x(s), glyph_string_y(s),
          glyph_string_left_overhang(s), glyph_string_right_overhang(s),
          glyph_row_overlapping_p(glyph_string_row(s)) ? Int32(1) : Int32(0),
          glyph_string_background_filled_p(s) ? Int32(1) : Int32(0))
  for i in 0..<Int(max(glyph_string_nchars(s), 0)) {
    /* C's `putc (…)` result is discarded; Swift wants the discard spelled out.  */
    _ = putc(Int32(glyph_u_ch(glyph_string_first_glyph(s), i)), stderr)
  }
  _ = putc(Int32(UInt8(ascii: "\n")), stderr)
}

/// C: `struct font_driver const nsfont_driver = { … };` (src/nsfont.m 1726-1742).  Swift cannot
/// emit a C data symbol (`@_cdecl` refuses `var` and `@_silgen_name` on a `var` imports), so
/// the record lives in a Swift global at a process-stable address - C's `static` storage,
/// which is never freed either - and the *export* of the `nsfont_driver` symbol that
/// src/nsfns.m 1316/3001 take the address of is a lane-level decision, the same one
/// `docs/ports/nsselect.md`'s `## ABI` (3) item 3 records for PORTING.md P-31's five `S…`
/// records.  The initialiser keeps C's order; the members C leaves unmentioned are zero, so
/// `nil` is C's value for them.  `draw` was the one member this half could not carry while
/// `nsfont_draw` was deferred; **round 83 lands it**, so C's `.draw = nsfont_draw` (C 1740) is
/// the function reference below.
fileprivate let nsfont_driver: UnsafeMutablePointer<font_driver> = {
  let p = UnsafeMutablePointer<font_driver>.allocate(capacity: 1)

  p.initialize(to: font_driver(
    type: Qns(),
    case_sensitive: true,
    get_cache: nsfont_get_cache,
    list: nsfont_list,
    match: nsfont_match,
    list_family: nsfont_list_family,
    free_entity: nil,
    open_font: nsfont_open,
    // The three slots below are C's `&nsfont_close`/`&nsfont_encode_char`/
    // `&nsfont_text_extents`; the value only reinterprets the pointer parameters that
    // `@convention(c)` cannot name (see the record's comment).
    close_font: { f in nsfont_close(f?.assumingMemoryBound(to: font.self)) },
    prepare_face: nil,
    done_face: nil,
    has_char: nsfont_has_char,
    encode_char: { f, c in nsfont_encode_char(f?.assumingMemoryBound(to: font.self), c) },
    text_extents: { f, code, n, metrics in
      nsfont_text_extents(f?.assumingMemoryBound(to: font.self), code, n,
                          metrics?.assumingMemoryBound(to: font_metrics.self))
    },
    draw: nsfont_draw,
    get_bitmap: nil,
    free_bitmap: nil,
    anchor_point: nil,
    otf_capability: nil,
    otf_drive: nil,
    start_for_frame: nil,
    end_for_frame: nil,
    shape: nsfont_shape,
    check: nil,
    get_variation_glyphs: nil,
    filter_properties: nil,
    cached_font_ok: nil,
    combining_capability: nil,
    drop_xrender_surfaces: nil))
  return p
}()

/// C: `DEFVAR_LISP`'s function-local `static struct Lisp_Fwd` (src/lisp.h 3495-3500); Swift
/// has no function-local statics, so the record is a fileprivate global and `syms_of_nsfont`
/// fills it in and hands it to `defvar_lisp` (the shape `docs/ports/nsselect.md` uses).
fileprivate var O_FWD_Vns_reg_to_script: Lisp_Fwd =
  Lisp_Fwd(type: 0, u: Lisp_Fwd_u(objvar: nil))

/* C: `static void syms_of_nsfont_for_pdumper (void);` (src/nsfont.m 1724) - C's forward
   declaration of the hook below; Swift resolves file-scope functions without one.  */

/// C: `void syms_of_nsfont (void)` (src/nsfont.m 1744-1760; declared src/font.h 986).
@_cdecl("syms_of_nsfont")
func syms_of_nsfont() {
  // C: `DEFSYM (Qcondensed, "condensed"); DEFSYM (Qmedium, "medium");` (1745-1746) and the
  // three at 1755-1757: `#define DEFSYM(sym, name) /* empty */` (src/lisp.h 2394) is only a
  // marker for make-docfile, so those call sites carry no statement; the symbols come from
  // the generated globals.

  // C: `DEFVAR_LISP ("ns-reg-to-script", Vns_reg_to_script, doc)` (1748-1749): the macro
  // builds its `struct Lisp_Fwd` with `Lisp_Fwd_Obj` and the variable's address (src/lisp.h
  // 3495-3500) and hands it to `defvar_lisp`; the docstring is the macro's third argument.
  O_FWD_Vns_reg_to_script =
    Lisp_Fwd(type: Lisp_Fwd_Obj(), u: Lisp_Fwd_u(objvar: &Vns_reg_to_script))
  defvar_lisp(&O_FWD_Vns_reg_to_script, "ns-reg-to-script")
  Vns_reg_to_script = Qnil()

  pdumper_do_now_and_after_load(syms_of_nsfont_for_pdumper)

  /* Font slant styles.  */
}

/// C: `syms_of_nsfont_for_pdumper` (src/nsfont.m 1762-1765), the hook `syms_of_nsfont` hands
/// to `pdumper_do_now_and_after_load`.
fileprivate func syms_of_nsfont_for_pdumper() {
  register_font_driver(UnsafePointer(nsfont_driver), nil)
}

/// The C data symbol `nsfont_driver` (src/nsfont.m 1726) as a **getter**: Swift cannot emit a
/// data symbol, so the lane exports a function of the variable's own C name returning its
/// address - the device `docs/ports/nsselect.md`'s `## ABI` (3) item 3 records for
/// PORTING.md P-31's five `S…` records, and the route this lane adopted for the driver record
/// (rc 3.166, decision 2): the Rust side writes C's `register_font_driver (&nsfont_driver, f)`
/// as a call to this entry, and src/nsfns.m's two `&nsfont_driver` sites with it.  The address
/// is the process-stable one `nsfont_driver` is allocated at, and nothing frees it, exactly as
/// C's `static` storage is never freed.
@_cdecl("nsfont_driver")
func nsfont_driver_getter() -> UnsafeMutableRawPointer? {
  UnsafeMutableRawPointer(nsfont_driver)
}

// ==========================================================================
// Round 83: the entries `nsfont_draw` and `ns_dump_glyphstring` need (`## ABI` (2f))
// ==========================================================================

/// C: `struct glyph_string`'s members the two bodies read - `s->first_glyph` and the glyph's
/// `type` (C 1161), `s->font`/`s->f`/`s->face` (C 1165-1169), `s->width` (C 1182), `s->hl`
/// (C 1183/1193), `s->char2b` (C 1176) and, for the dumper, `s->nchars`, `s->x`, `s->y`,
/// `s->left_overhang`, `s->right_overhang`, `s->row` and `s->background_filled_p` (C 1716-1720).
/// The record is the tree's opaque placeholder (`femacs/src/font.rs` 739, P-70), so each access
/// is its own reader entry - the shape `nsterm.swift` 3578-5016 already spells, declared here
/// as well because each unit spells what it reads.  **Open**: the core's to export.
@_silgen_name("glyph_string_first_glyph")
func glyph_string_first_glyph(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_type") func glyph_type(_ g: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_font")
func glyph_string_font(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_f")
func glyph_string_f(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_face")
func glyph_string_face(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_width") func glyph_string_width(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_hl") func glyph_string_hl(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_char2b")
func glyph_string_char2b(_ s: UnsafeMutableRawPointer?) -> UnsafePointer<UInt32>?
@_silgen_name("glyph_string_nchars") func glyph_string_nchars(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_x") func glyph_string_x(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_y") func glyph_string_y(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_left_overhang")
func glyph_string_left_overhang(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_right_overhang")
func glyph_string_right_overhang(_ s: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("glyph_string_row")
func glyph_string_row(_ s: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?
@_silgen_name("glyph_string_background_filled_p")
func glyph_string_background_filled_p(_ s: UnsafeMutableRawPointer?) -> Bool

/// C: `s->row->overlapping_p` (C 1718, the row's bit-field) and `s->first_glyph[i].u.ch`
/// (C 1720, src/dispextern.h 593's `unsigned ch`) - the two reads no earlier body needed.
/// `glyph_u_ch` takes the index because `struct glyph`'s stride belongs to the core's mirror
/// (`femacs/src/include/dispextern.rs`), which this unit never sees.  **Open**: the core's to
/// export.
@_silgen_name("glyph_row_overlapping_p")
func glyph_row_overlapping_p(_ row: UnsafeMutableRawPointer?) -> Bool
@_silgen_name("glyph_u_ch") func glyph_u_ch(_ g: UnsafeMutableRawPointer?, _ i: Int) -> UInt32

/// C: the enumerators the two bodies compare against - `COMPOSITE_GLYPH` (C 1161, `enum
/// glyph_type`) and `DRAW_CURSOR` (C 1183/1187/1193, `enum draw_glyphs_face`, src/dispextern.h
/// 1348).  **Open**: the core's to export.
@_silgen_name("COMPOSITE_GLYPH") func COMPOSITE_GLYPH() -> Int32
@_silgen_name("DRAW_CURSOR") func DRAW_CURSOR() -> Int32

/// C: `#define FONT_HEIGHT(f) ((f)->height)` and `#define FONT_BASE(f) ((f)->ascent)`
/// (src/font.h 244-245) - C 1173/1181/1182's four reads.  **Provided**: the core's
/// (`macfont.swift` 745-746 imports the same two names).
@_silgen_name("FONT_HEIGHT") func FONT_HEIGHT(_ f: UnsafeMutableRawPointer?) -> Int32
@_silgen_name("FONT_BASE") func FONT_BASE(_ f: UnsafeMutableRawPointer?) -> Int32

/// C: `NS_FACE_BACKGROUND`/`NS_FACE_FOREGROUND` (src/nsterm.h 1031-1041) and the three frame
/// colours (src/nsterm.h 1037-1038) - C 1184-1198's reads.  **Provided**: the port of
/// src/nsterm.m owns the definitions (`macfont.swift` 752-757 imports the same names).
@_silgen_name("NS_FACE_BACKGROUND")
func NS_FACE_BACKGROUND(_ face: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("NS_FACE_FOREGROUND")
func NS_FACE_FOREGROUND(_ face: UnsafeMutableRawPointer?) -> UInt
@_silgen_name("FRAME_BACKGROUND_COLOR")
func FRAME_BACKGROUND_COLOR(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("FRAME_CURSOR_COLOR")
func FRAME_CURSOR_COLOR(_ f: UnsafeMutableRawPointer?) -> NSColor?
@_silgen_name("FRAME_FOREGROUND_COLOR")
func FRAME_FOREGROUND_COLOR(_ f: UnsafeMutableRawPointer?) -> NSColor?

/// C: `#define FRAME_FONT(f) ((struct nsfont_info *) (f)->output_data.ns->font)` (src/nsterm.h)
/// - C 1167's fallback read.  **Provided**: the port of src/nsterm.m owns the definition.
@_silgen_name("FRAME_FONT")
func FRAME_FONT(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?

/// C: `+[NSColor colorWithUnsignedLong: (unsigned long)]` (declared src/nsterm.h 361, defined
/// src/nsterm.m 157, the `NSColor (EmacsColor)` category) - C 1185/1197's two calls, reached as
/// the class-first entry `macfont.swift` 1321-1327 declares.  **Provided**: the port of
/// src/nsterm.m owns the definition.
@_silgen_name("colorWithUnsignedLong")
func colorWithUnsignedLong(_ cls: UnsafeMutableRawPointer?, _ c: UInt) -> NSColor?

/// C: `void NSRectFill (NSRect)` (AppKit's NSGraphics.h) - C 1189's fill.  **Provided**: the SDK
/// (Swift marks the `NSRect*` C family unavailable, the `nsterm.swift` 3362 reading).
@_silgen_name("NSRectFill") func NSRectFill(_ rect: NSRect)

/// C: `int putc (int, FILE *)` (stdio.h) - C 1720/1721's two puts (`fprintf`/`stderr` are C
/// 1180's declarations above).  **Provided**: libSystem.
@_silgen_name("putc") func putc(_ c: Int32, _ stream: UnsafeMutablePointer<FILE>?) -> Int32

/// C: `void DPSmoveto (NSGraphicsContext *ctxt, CGFloat x, CGFloat y)` and
/// `void GSShowGlyphs (NSGraphicsContext *ctxt, const NSGlyph *glyphs, NSInteger count)`
/// (GNUstep's NSGraphicsContext.h) - C 1205-1206's two calls.  **Open**: these are the two
/// callees the file's `## Decisions` names as GNUstep-only - Apple's SDK has neither, so their
/// definitions live in the GNUstep libraries (`## ABI` (2e)).
@_silgen_name("DPSmoveto")
func DPSmoveto(_ context: NSGraphicsContext?, _ x: CGFloat, _ y: CGFloat)
@_silgen_name("GSShowGlyphs")
func GSShowGlyphs(_ context: NSGraphicsContext?, _ glyphs: UnsafePointer<NSGlyph>?, _ count: Int)
