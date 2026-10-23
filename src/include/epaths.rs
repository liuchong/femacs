// femacs: mirror of the frozen target's <epaths.h>; standalone builders generate checkout-local paths.
//
// C generates one `epaths.h` per build, from `src/epaths.in`: the *installed* tree's paths for a
// normal build, and the `assets/` pseudo-paths for Android (`epaths.in` 85-99, its
// `#ifdef HAVE_ANDROID` arm).  This mirror carries both arms at one landing point (P-26), because
// every unit that includes it must see the arm its target compiles.
//
// Three of the Android values are *runtime* expressions in C - `(android_lib_dir)`,
// `(android_site_load_path)` and `(android_game_path)` - so they are read from the objects
// `src/android.rs` exports rather than frozen into literals.  Freezing them was the defect
// `docs/status.md` 4.784 measured on the device: the reference looked for
// `/usr/local/share/emacs/.../lisp` instead of `/assets/lisp/` and exited 1 with "will not
// function correctly without the character map files".

/// C: `<epaths.h>`: `#define PATH_LOADSEARCH "..."` (installed tree / `/assets/lisp/`).
#[inline(always)]
pub fn PATH_LOADSEARCH () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/share/emacs/32.0.50/lisp".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return c"/assets/lisp/".as_ptr (); }
}

/// C: `<epaths.h>`: `#define PATH_DUMPLOADSEARCH "..."` (the build tree / `/assets/lisp/`).
#[inline(always)]
pub fn PATH_DUMPLOADSEARCH () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"../lisp".as_ptr(); }
  #[cfg (feature = "have_android")]
  { return c"/assets/lisp/".as_ptr (); }
}

/// C: `<epaths.h>`: `#define PATH_DUMPLOADSEARCH "/"` - C's adjacent-literal concatenation.
#[inline(always)]
pub fn PATH_DUMPLOADSEARCH_SLASH () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"../lisp/".as_ptr(); }
  #[cfg (feature = "have_android")]
  { return c"/assets/lisp//".as_ptr (); }
}

/// C: `<epaths.h>`: `#define PATH_SITELOADSEARCH "..."` / `(android_site_load_path)`.
#[inline(always)]
pub fn PATH_SITELOADSEARCH () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/share/emacs/32.0.50/site-lisp:/usr/local/share/emacs/site-lisp".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return unsafe { android_site_load_path }; }
}

/// C: `<epaths.h>`: `#define PATH_EXEC "..."` / `(android_lib_dir)`.
#[inline(always)]
pub fn PATH_EXEC () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return unsafe { android_lib_dir }; }
}

/// C: `<epaths.h>`: `#define PATH_DATA "..."` / `"/assets/etc/"`.
#[inline(always)]
pub fn PATH_DATA () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/share/emacs/32.0.50/etc".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return c"/assets/etc/".as_ptr (); }
}

/// C: `<epaths.h>`: `#define PATH_DOC "..."` / `"/assets/etc/"`.
#[inline(always)]
pub fn PATH_DOC () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/share/emacs/32.0.50/etc".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return c"/assets/etc/".as_ptr (); }
}

/// C: `<epaths.h>`: `#define PATH_INFO "..."` / `"/assets/info/"`.
#[inline(always)]
pub fn PATH_INFO () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/share/info".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return c"/assets/info/".as_ptr (); }
}

/// C: `<epaths.h>`: `#define PATH_GAME "..."` / `(android_game_path)`.  The frozen build's value
/// is the null pointer, which `callproc.rs` 2262 tests.
#[inline(always)]
pub fn PATH_GAME () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return std::ptr::null (); }
  #[cfg (feature = "have_android")]
  { return unsafe { android_game_path }; }
}

/// C: `<epaths.h>`: `#define PATH_BITMAPS "..."` / `"/assets/bitmaps/"`.
#[inline(always)]
pub fn PATH_BITMAPS () -> *const c_char
{
  #[cfg (not (feature = "have_android"))]
  { return c"/usr/local/share/emacs/32.0.50/etc/images".as_ptr (); }
  #[cfg (feature = "have_android")]
  { return c"/assets/bitmaps/".as_ptr (); }
}

/// The three Android objects `epaths.in` 96-100 names.  They are C globals in `src/android.c`
/// (`char *android_site_load_path;` …), so `src/android.rs` exports them with `#[unsafe(no_mangle)]`.
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub static mut android_site_load_path: *mut c_char;
  pub static mut android_lib_dir: *mut c_char;
  pub static mut android_game_path: *mut c_char;
}
