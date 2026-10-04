// femacs: port of src/macfont.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header belongs to the **Cocoa** font backend (the one
// `configure.ac` 2762 selects as `macfont.o`; `src/nsfont.m`'s GNUstep backend is the other
// half of that choice, status 3.161), and nothing on this target compiles it: `HAVE_NS` is
// undefined, `src/macfont.c` is not in the frozen build's source list (its platform object
// variables are empty), and every type the file names below comes from CoreFoundation /
// CoreGraphics, which have no mirrors here.  The mirror carries the header's live text the way
// `src/w32select.h`/`src/xsettings.h` do, and P-72 decides the names: `struct frame *` and the
// Core pointers degrade to `*mut c_void` with C's prototype in a `/// C:` line, while
// `struct font *` is named because `include/font.rs` owns that record (status 3.158).
//
// The CoreFoundation/CoreGraphics half is carried **under `#[cfg (feature = "macos_sdk")]`**, the
// P-51 shape for a platform arm no build here enables: `struct mac_glyph_layout` keeps C's field
// names *and* C's field types (`CFRange`, `CFIndex`, `CGFloat`, `CGGlyph`), the three Core Text
// enum blocks keep C's members and their Core Text values, and the smoothing setter keeps
// `CGContextRef`.  Spelling those names instead of a stand-in is the point: a macOS target
// supplies them from the SDK (its own bindings are that target's job), and nothing here is
// compiled on this one - the gate reads the text, and `layout-check` never sees the arm.  No
// layout assertion is written for the record, because no compiler on this target could measure
// it (the `include-w32heap.md`/`include-emacsgtkfixed.md` reading).

// C 20-22: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const MACFONT_H: bool = true;

/// C 24-46: the per-glyph layout record.  Field names and types are C's; the macOS arm supplies
/// `CFRange`/`CFIndex`/`CGFloat`/`CGGlyph`.  No assertion: nothing here can measure it.
/// C: struct mac_glyph_layout { CFRange comp_range; CFIndex string_index; CGFloat advance_delta,
/// C: baseline_delta; CGFloat advance; CGGlyph glyph_id; };
#[cfg (feature = "macos_sdk")]
#[repr(C)]
pub struct mac_glyph_layout
{
  pub comp_range: CFRange,
  pub string_index: CFIndex,
  pub advance_delta: CGFloat,
  pub baseline_delta: CGFloat,
  pub advance: CGFloat,
  pub glyph_id: CGGlyph,
}

/// C 48-70: three `enum` blocks inside `#if MAC_OS_X_VERSION_MAX_ALLOWED < 1080` (undefined on
/// this target, so C's preprocessor reads `0 < 1080` and keeps them).  The members are Core
/// Text's own constants, and C's values are those identifiers - the port keeps both, under the
/// same arm; the `#if … >= 1070` alternative for `kCTFontTraitColorGlyphs` (`(1 << 13)`) is dead
/// on any SDK the port would build against and is recorded in the ledger.
#[cfg (feature = "macos_sdk")]
pub const kCTFontTraitItalic: c_int = kCTFontItalicTrait;
#[cfg (feature = "macos_sdk")]
pub const kCTFontTraitBold: c_int = kCTFontBoldTrait;
#[cfg (feature = "macos_sdk")]
pub const kCTFontTraitMonoSpace: c_int = kCTFontMonoSpaceTrait;
#[cfg (feature = "macos_sdk")]
pub const kCTFontTraitColorGlyphs: c_int = kCTFontColorGlyphsTrait;
#[cfg (feature = "macos_sdk")]
pub const kCTCharacterCollectionIdentityMapping: c_int = kCTIdentityMappingCharacterCollection;
#[cfg (feature = "macos_sdk")]
pub const kCTCharacterCollectionAdobeJapan1: c_int = kCTAdobeJapan1CharacterCollection;
#[cfg (feature = "macos_sdk")]
pub const kCTFontOrientationDefault: c_int = kCTFontDefaultOrientation;

/// C 71-73: `#if USE_CT_GLYPH_INFO` (undefined here) aliases the entry point to Core Text's.
/// C: #define mac_font_get_glyph_for_cid mac_ctfont_get_glyph_for_cid
macro_rules! mac_font_get_glyph_for_cid {
  () => { mac_ctfont_get_glyph_for_cid };
}

/// C 75-76: Core Text's version constant, guarded by `#ifndef` because older SDKs define it.
/// C: #define kCTVersionNumber10_9 0x00060000
pub const kCTVersionNumber10_9: u32 = 0x00060000;

/// C 77-78: `CFSTR` makes a `CFStringRef` constant from the literal; Rust has no counterpart, and
/// the macro body is never expanded here (a `macro_rules!` body is not resolved until it is).
/// C: #define MAC_FONT_CHARACTER_SET_STRING_ATTRIBUTE \
/// C:   CFSTR ("MAC_FONT_CHARACTER_SET_STRING_ATTRIBUTE")
macro_rules! MAC_FONT_CHARACTER_SET_STRING_ATTRIBUTE {
  () => { CFSTR ("MAC_FONT_CHARACTER_SET_STRING_ATTRIBUTE") };
}

/// C 80: `typedef const struct _EmacsScreenFont *ScreenFontRef;` - a pointer to a record this
/// header does not define (the `.m` file does), so only the pointer shape is carried.
/// C: typedef const struct _EmacsScreenFont *ScreenFontRef;
pub type ScreenFontRef = *const c_void;

/// C 82-88: the three entry points `src/macfont.c` defines, plus Core Graphics' smoothing setter.
/// C: extern void mac_register_font_driver (struct frame *f);
/// C: extern void *macfont_get_nsctfont (struct font *font);
/// C: extern void macfont_update_antialias_threshold (void);
/// C: extern void CGContextSetFontSmoothingStyle(CGContextRef, int) __attribute__ ((weak_import));
unsafe extern "C" {
  /// `struct frame *` is frame.rs's record but this mirror declares no include for it, so the
  /// parameter keeps P-55's untyped pointer (a consumer converts at the call site).
  pub fn mac_register_font_driver (f: *mut c_void);
  pub fn macfont_get_nsctfont (font: *mut font) -> *mut c_void;
  pub fn macfont_update_antialias_threshold ();
}

/// C 86-87: Core Graphics' smoothing setter - under the same arm, because C's `CGContextRef` is
/// only nameable where the SDK is.  C's declaration is `weak_import`; Rust has no spelling for
/// that, so a macOS target resolves the symbol with `dlsym` (recorded in the ledger).
#[cfg (feature = "macos_sdk")]
unsafe extern "C" {
  pub fn CGContextSetFontSmoothingStyle (ctx: CGContextRef, style: c_int);
}
