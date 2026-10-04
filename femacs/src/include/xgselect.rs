// femacs: port of src/xgselect.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header has no platform guard of its own - C 20-21 is the
// include guard and C 28-33 are live text - but nothing on this target compiles it:
// `src/xgselect.c`'s whole body is inside `#ifdef HAVE_GLIB` (undefined in the frozen
// configuration, docs/baseline/config/darwin-arm64.macros.tsv), so `femacs/src/xgselect.rs` is
// the empty-shape note and no unit links these names (the `src/w32select.h` precedent,
// status 3.158).  The mirror carries them anyway, because the header's *text* is what a mirror
// mirrors.
//
// P-72 decides every name here: the header declares no type of its own and its two `#include`s
// (C 23-24) supply `fd_set` only through `sysselect.h`, which a *consumer* reaches itself
// (PORTING.md 0.1-9) - so every pointer parameter takes the P-55 stand-in with C's text in a
// `/// C:` line, and a consumer on a GLib build converts at the call site.

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const XGSELECT_H: bool = true;

/// C 26: `struct timespec;` - a forward declaration of a type this header does not define
/// (time.h's record), which is why the fifth parameter below is untyped rather than named.
/// C: struct timespec;

/// C 28-33: the four entry points `src/xgselect.c` defines - all of them inside that file's
/// dead `#ifdef HAVE_GLIB` body, so their definitions are deferred to the GLib target the way
/// the unit's own note records.
/// C: extern int xg_select (int, fd_set *, fd_set *, fd_set *, struct timespec *, sigset_t *);
/// C: extern void suppress_xg_select (void);
/// C: extern void release_xg_select (void);
/// C: extern void release_select_lock (void);
unsafe extern "C" {
  pub fn xg_select (max_fds: c_int, rfds: *mut c_void, wfds: *mut c_void, efds: *mut c_void,
                    timeout: *mut c_void, sigmask: *mut c_void) -> c_int;
  pub fn suppress_xg_select ();
  pub fn release_xg_select ();
  pub fn release_select_lock ();
}
