// femacs: port of src/xsettings.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header's own includes are conditional and the frozen
// configuration decides them: `HAVE_PGTK` is undefined, so C 23-25's arm (`dispextern.h` +
// `<X11/Xlib.h>`) is the live one and C 26-28's cairo arm is dead text.  Nothing on this target
// compiles the header either - `src/xsettings.c` is not in the frozen build's source list, so
// `src/xsettings.rs` is the empty-shape note that names the exclusion - and the mirror
// carries the live declarations anyway, because the header's *text* is what a mirror mirrors
// (the `src/w32select.h` / `src/xgselect.h` precedent, status 3.158/3.162).
//
// P-72 decides the names: `Display_Info` is C 33-35's `#ifdef HAVE_PGTK` typedef (dead here) and
// reaches the live declarations only as dispextern.h's record; `XEvent` is Xlib's.  Neither is
// this header's own, so both parameters take the P-55 stand-in with C's text in a `/// C:` line,
// and a consumer on a PGTK/X build converts at the call site (PORTING.md 0.1-9).

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const XSETTINGS_H: bool = true;

/// C 30-31: forward declarations of two records this header does not define - the port's
/// consumers reach the definitions through their own includes, so the mirror names neither.
/// C: struct x_display_info;
/// C: struct pgtk_display_info;

/// C 33-35: `typedef struct pgtk_display_info Display_Info;` lives inside `#ifdef HAVE_PGTK`,
/// which the frozen configuration leaves undefined - dead text, recorded rather than mirrored.

/// C 37-44: the entry points `src/xsettings.c` defines.  C 39-40's `xft_settings_event` is under
/// `#ifndef HAVE_PGTK` (true here, so it is live text), and C 42-44's
/// `xsettings_get_system_normal_font` is under `#ifdef USE_LUCID` (undefined - dead text).
/// C: extern void xsettings_initialize (Display_Info *);
/// C: extern bool xft_settings_event (Display_Info *, const XEvent *);
/// C: extern const char *xsettings_get_system_font (void);
/// C: extern const char *xsettings_get_system_normal_font (void);   [USE_LUCID, dead here]
/// C: cairo_font_options_t *xsettings_get_font_options (void);       [HAVE_PGTK, dead here]
/// C: bool xg_get_system_dark_mode (void);                           [HAVE_PGTK, dead here]
unsafe extern "C" {
  pub fn xsettings_initialize (dpyinfo: *mut c_void);
  pub fn xft_settings_event (dpyinfo: *mut c_void, event: *const c_void) -> bool;
  pub fn xsettings_get_system_font () -> *const c_char;
}
