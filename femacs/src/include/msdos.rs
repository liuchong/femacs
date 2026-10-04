// femacs: port of src/msdos.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the MS-DOS (djgpp) layer's interface: C 19-20
// is the include guard, C 22-25 the platform includes and one forward declaration, C 27-28 the
// `_dos_commit` rename, C 30-50 the terminal, file-name, mouse and frame-parameter entry points,
// C 52-67 the system headers plus the MS-DOS build's own prototypes for the POSIX calls djgpp
// lacks, C 69-76 the signal/keyboard/child entry points, C 79-87 two constants, and C 90-154 the
// `#ifndef HAVE_X_WINDOWS` arm: the dummy X types, the two `tty` records C 104-105 declares, the
// `FRAME_*` macros and the XMenu façade.
//
// **The whole header is included only by MS-DOS builds.**  C's own readers are all inside
// `#ifdef MSDOS` (`src/dired.c` 43-45, `src/sysdep.c` 96-98, `src/keyboard.c` 61-63,
// `src/atimer.c` 36-38, …), and `docs/baseline/config/darwin-arm64.macros.tsv` records `MSDOS`
// as `undef`, so nothing on this target compiles any of its lines - which is also why the
// `#ifndef HAVE_X_WINDOWS` arm is live *for this header* even though the frozen target has no
// window system (C's guard is about X, not about MS-DOS).
//
// Conditions: `tools/check-cond-eval.py src/msdos.h` -> `4 conditions, 0 unknown, 1 disagreement
// [NOT VERIFIED]`: the guard, `HAVE_X_WINDOWS` (C 90), `O_CLOEXEC` (C 84) and
// `__DJGPP__ == 2 && __DJGPP_MINOR__ < 4` (C 57).  The `O_CLOEXEC` one is a known disagreement -
// clang reads the macro as undefined there and the evaluator does not - and it is **not**
// resolved here: C 84-86 only `#undef`s a macro the system may have defined first, and C 87 then
// defines it to 0, so both arms end with the same text (`O_CLOEXEC 0`); the disagreement is
// recorded in docs/ports/include-msdos.md with both readings.  The `__DJGPP__` one is false
// for this table (the macro is undefined), so C 58-59 is the file's one dead arm and is carried
// under `#[cfg (feature = "djgpp_2_3")]` (PORTING.md P-51 names the non-frozen side).
//
// **Names this mirror shares with the port's other mirrors carry `#[cfg (feature = "msdos")]`.**
// Two classes: C 84-87's `O_CLOEXEC 0` and C 80's `EINPROGRESS 112`, which are the MS-DOS build's
// *overrides* of the portable values the port already spells (`lib/include/fcntl.in.rs` 118,
// `lib/include/errno.rs` 92 - the `lib/include/errno.in.rs` 95-99 `feature = "windows"` shape);
// and C 57-67/C 71's prototypes for calls the MS-DOS build defines itself (`readlink`,
// `readlinkat`, `fstatat`, `unsetenv`, `faccessat`, `openat`, `fchmodat`, `futimens`,
// `utimensat`, `pthread_sigmask`), which the port already declares in its system mirrors
// (`lib/include/unistd.in.rs` 39-41, `lib/include/sys_stat.in.rs` 181-196,
// `lib/include/fcntl.in.rs` 287, `lib/include/signal.rs` 157) - Rust holds one declaration per
// name in a crate, so the MS-DOS side is marked with the feature rather than duplicated
// unmarked.  C 90-96's five dummy X typedefs are the third class: `Display`/`Window` are declared
// by the other platform mirrors (`include/w32gui.rs` 57-58, `include/nsgui.rs` 60), so they carry
// the same feature.  Everything else is live text and is carried without a `cfg`.
//
// **P-72/P-55 readings.**  The header reaches `struct tty_display_info`/`struct tty_output` for
// the two *by value* objects of C 104-105 through its own units' includes (`src/msdos.c` 72,
// `src/dosfns.c` 38 both include termchar.h), so the mirror includes `termchar.rs`;
// `<sys/types.h>` (C 52) is included for `mode_t`.  Pointers to records this file never defines
// (`struct terminal`, `struct frame`, `struct window`, `struct stat`, `struct timespec`,
// `sigset_t`) keep C's pointer depth as `*mut c_void`/`*const c_void` with the C text in a
// `/// C:` line; `Lisp_Object` stays named (lisp.h is every unit's include, PORTING.md 0.1-9).
// C's `ssize_t` (C 60) is the port's `c_long`: unistd.in.rs owns the alias and C's msdos.h does
// not include <unistd.h>.
//
// Parameter names the header leaves out come from the definitions the mirror's declarations
// point at (`src/msdos.c`, `src/xfns.c` 1683, `src/w32proc.c` 293, `src/w16select.c` 675), as
// PORTING.md 0.2's "same names" rule wants (the `include/dosfns.rs` reading).

// C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_MSDOS_H: bool = true;

include!("termchar.rs");                            // the two records C 104-105 declare by value
include!("../../lib/include/sys_types.in.rs");      // C 52: `#include <sys/types.h>` (`mode_t`)

/// C 25: `struct terminal;` is a forward declaration of a record this header does not include, so
/// it has no item of its own (the `haikuterm.rs` reading) and the pointer of C 42 stays
/// `*mut c_void`.

/// C 27-28: djgpp's own `_dos_commit`, which the header renames `tcdrain` (C 28).
/// C: extern unsigned int _dos_commit(int);
/// C: #define tcdrain(f) _dos_commit(f)
unsafe extern "C" {
  pub fn _dos_commit (f: c_int) -> c_uint;
}
/// C 28.
macro_rules! tcdrain { ($f:expr) => { _dos_commit ($f) }; }

/// C 30-34: the raw/cooked terminal pair, the screen saver's saved screen, the keyboard layout
/// switch and the window-size setter.
/// C: int dos_ttraw (struct tty_display_info *);
/// C: int dos_ttcooked (void);
/// C: int dos_get_saved_screen (char **, int *, int *);
/// C: int dos_set_keyboard (int, int);
/// C: void dos_set_window_size (int *, int *);
unsafe extern "C" {
  pub fn dos_ttraw (tty: *mut tty_display_info) -> c_int;
  pub fn dos_ttcooked () -> c_int;
  pub fn dos_get_saved_screen (screen: *mut *mut c_char, rows: *mut c_int,
                               cols: *mut c_int) -> c_int;
  pub fn dos_set_keyboard (code: c_int, always: c_int) -> c_int;
  pub fn dos_set_window_size (rows: *mut c_int, cols: *mut c_int);
}

/// C 36-42: the MS-DOS file-name primitives and the environment/terminal initializers.
/// C: int getdefdir (int, char*);
/// C: void unixtodos_filename (char *);
/// C: void dostounix_filename (char *);
/// C: char *rootrelativepath (char *);
/// C: void init_environment (int, char **, int);
/// C: void internal_terminal_init (void);
/// C: void initialize_msdos_display (struct terminal *);
unsafe extern "C" {
  pub fn getdefdir (drive: c_int, dst: *mut c_char) -> c_int;
  pub fn unixtodos_filename (p: *mut c_char);
  pub fn dostounix_filename (p: *mut c_char);
  pub fn rootrelativepath (rel: *mut c_char) -> *mut c_char;
  pub fn init_environment (argc: c_int, argv: *mut *mut c_char, skip_args: c_int);
  pub fn internal_terminal_init ();
  pub fn initialize_msdos_display (term: *mut c_void);
}

/// C 44-48: the mouse.  C 44's `have_mouse` is the one object this header owns here.
/// C: extern int have_mouse;
/// C: void mouse_init (void);
/// C: void mouse_on (void);
/// C: void mouse_off (void);
/// C: void mouse_moveto (int, int);
unsafe extern "C" {
  pub static mut have_mouse: c_int;
  pub fn mouse_init ();
  pub fn mouse_on ();
  pub fn mouse_off ();
  pub fn mouse_moveto (x: c_int, y: c_int);
}

/// C 50: the frame-parameter hook the MS-DOS display shares with the X builds.
/// C: void IT_set_frame_parameters (struct frame *, Lisp_Object);
unsafe extern "C" {
  pub fn IT_set_frame_parameters (f: *mut c_void, alist: Lisp_Object);
}

/// C 57-59: the arm for djgpp 2.3 and older, which has no `readlink` in `<unistd.h>`.  Dead
/// text on this target (the condition is false in the frozen macro table), carried under the
/// feature that names the other side (PORTING.md P-51); the parameter names are
/// `lib/include/unistd.in.rs` 39's (`readlink`'s own prototype lives there for every other
/// target).
/// C: int readlink (const char *, char *, size_t);
#[cfg (feature = "djgpp_2_3")]
unsafe extern "C" {
  pub fn readlink (file: *const c_char, buf: *mut c_char, bufsize: usize) -> c_long;
}

/// C 60-67 and C 71: the MS-DOS build's own prototypes for the calls djgpp lacks.  The port's
/// system mirrors already declare these names (see the file note), so the MS-DOS side carries
/// them under its feature; C's `ssize_t` (C 60) is the port's `c_long` and C's
/// `const struct timespec[2]` a pointer to the record this header does not include.
/// C: ssize_t readlinkat (int, const char *, char *, size_t);
/// C: int fstatat (int, char const *, struct stat *, int);
/// C: int unsetenv (const char *);
/// C: int faccessat (int, const char *, int, int);
/// C: int openat (int, const char *, int, int);
/// C: int fchmodat (int, const char *, mode_t, int);
/// C: int futimens (int, const struct timespec[2]);
/// C: int utimensat (int, const char *, const struct timespec[2], int);
/// C: int pthread_sigmask (int, const sigset_t *, sigset_t *);
#[cfg (feature = "msdos")]
unsafe extern "C" {
  pub fn readlinkat (fd: c_int, name: *const c_char, buffer: *mut c_char,
                     buffer_size: usize) -> c_long;
  pub fn fstatat (fd: c_int, name: *const c_char, st: *mut c_void, flags: c_int) -> c_int;
  pub fn unsetenv (name: *const c_char) -> c_int;
  pub fn faccessat (dirfd: c_int, path: *const c_char, mode: c_int, flags: c_int) -> c_int;
  pub fn openat (fd: c_int, path: *const c_char, oflag: c_int, mode: c_int) -> c_int;
  pub fn fchmodat (fd: c_int, path: *const c_char, mode: mode_t, flags: c_int) -> c_int;
  pub fn futimens (fd: c_int, times: *const c_void) -> c_int;
  pub fn utimensat (dirfd: c_int, pathname: *const c_char, times: *const c_void,
                    flags: c_int) -> c_int;
  pub fn pthread_sigmask (how: c_int, set: *const c_void, oset: *mut c_void) -> c_int;
}

/// C 69-76: the signal handler, the Lisp-symbol initializers and the child-process runner.
/// C: void msdos_fatal_signal (int);
/// C: void syms_of_msdos (void);
/// C: int dos_keysns (void);
/// C: int dos_keyread (void);
/// C: int run_msdos_command (char **, const char *, int, int, int, char **);
/// C: void syms_of_win16select (void);
unsafe extern "C" {
  pub fn msdos_fatal_signal (sig: c_int);
  pub fn syms_of_msdos ();
  pub fn dos_keysns () -> c_int;
  pub fn dos_keyread () -> c_int;
  pub fn run_msdos_command (argv: *mut *mut c_char, working_dir: *const c_char,
                            tempin: c_int, tempout: c_int, temperr: c_int,
                            envv: *mut *mut c_char) -> c_int;
  pub fn syms_of_win16select ();
}

/* Constants.  */

/// C 80: `#define EINPROGRESS 112` - djgpp's `<errno.h>` has no `EINPROGRESS`; the MS-DOS
/// build's value is marked with the feature because the port's `<errno.h>` mirror already owns
/// the name (`lib/include/errno.rs` 92, 36; the `errno.in.rs` 95-99 shape).
/// C: #define EINPROGRESS 112
#[cfg (feature = "msdos")]
pub const EINPROGRESS: c_int = 112;

/// C 81-87: gnulib sets `O_CLOEXEC` to `O_NOINHERIT`, "which gets in the way when we need to
/// redirect standard handles for subprocesses using temporary files created by mkostemp" - so
/// the header `#undef`s it (C 84-86, the condition `check-cond-eval.py` cannot decide; see the
/// file note) and defines it to 0.  Both arms end at the same text.  The name is the MS-DOS
/// build's override of `lib/include/fcntl.in.rs` 118, hence the feature.
/// C: #define O_CLOEXEC 0
#[cfg (feature = "msdos")]
pub const O_CLOEXEC: c_int = 0;

/// C 90-96: `#ifndef HAVE_X_WINDOWS`'s dummy types - the MS-DOS build has no X, so the names the
/// shared code uses are `int`.  `Display`/`Window` are declared by the port's other platform
/// mirrors (`include/w32gui.rs` 57-58, `include/nsgui.rs` 60), so the five carry the MS-DOS
/// feature (C's guard keeps them apart from an X build; Rust needs the feature to keep them
/// apart from the other platforms' mirrors).
/// C: typedef int XFontStruct;
/// C: typedef int GC;
/// C: typedef int Pixmap;
/// C: typedef int Display;
/// C: typedef int Window;
#[cfg (feature = "msdos")]
pub type XFontStruct = c_int;
#[cfg (feature = "msdos")]
pub type GC = c_int;
#[cfg (feature = "msdos")]
pub type Pixmap = c_int;
#[cfg (feature = "msdos")]
pub type Display = c_int;
#[cfg (feature = "msdos")]
pub type Window = c_int;

/// C 98: `#define FRAME_X_DISPLAY(ignored) NULL` - C's `NULL` is the port's `null_mut ()`
/// (canonicalised by the token check, PORTING.md P-44's family table).
/// C: #define FRAME_X_DISPLAY(ignored) NULL
macro_rules! FRAME_X_DISPLAY { ($ignored:expr) => { std::ptr::null_mut () }; }

/// C 99: `#define PIX_TYPE unsigned long` - a *type* macro, so it is a `macro_rules!` with no
/// metavariables (the `haikugui.rs` 63 / `include/w32gui.rs` `NativeRectangle` spelling); C's
/// `unsigned long` is PORTING.md 1.1's `c_ulong`.
macro_rules! PIX_TYPE { () => { c_ulong }; }

/// C 100: `#define XDISPLAY` - a marker macro with no body, so the port spells it as the
/// constant that says "defined" (the `nt/inc/include/ms-w32.rs` 147 `_WINSOCK_H` spelling; a
/// `macro_rules!` with an empty body is not what the mirror can write - the check reports an
/// empty body).
pub const XDISPLAY: bool = true;

/// C 102: `typedef struct tty_display_info Display_Info;` - the same record, under the name the
/// shared code uses.
pub type Display_Info = tty_display_info;

/// C 104-105: the MS-DOS build's one display and its `tty_output`, declared with the records
/// `termchar.rs` defines (the two objects' *definitions* are `src/msdos.c`'s).
/// C: extern struct tty_display_info the_only_display_info;
/// C: extern struct tty_output the_only_tty_output;
unsafe extern "C" {
  pub static mut the_only_display_info: tty_display_info;
  pub static mut the_only_tty_output: tty_output;
}

/// C 107-108: the two accessors.  C 107's body is `((f)->output_data.tty->font)`, which the port
/// reads with `.` field access (Rust has no `->`).
/// C: #define FRAME_FONT(f) ((f)->output_data.tty->font)
macro_rules! FRAME_FONT { ($f:expr) => { (($f).output_data.tty.font) }; }
/// C 108: `#define FRAME_DISPLAY_INFO(f) (&the_only_display_info)` - the record C 104 declares
/// above.
/// C: #define FRAME_DISPLAY_INFO(f) (&the_only_display_info)
macro_rules! FRAME_DISPLAY_INFO { ($f:expr) => { (&raw mut the_only_display_info) }; }

/// C 113-114: `struct frame;` and `struct window;` are forward declarations of records this
/// header does not include, so they have no items of their own (the C 25 reading above).

/// C 118: the menu-bar setter the MS-DOS display emulates (`src/xfns.c` 1683 gives the
/// parameter names).
/// C: extern void x_set_menu_bar_lines (struct frame *, Lisp_Object, Lisp_Object);
unsafe extern "C" {
  pub fn x_set_menu_bar_lines (f: *mut c_void, value: Lisp_Object, oldval: Lisp_Object);
}

/* Defined in xfns.c; emulated on msdos.c */
/* C 120-131: the no-op X entries and the two display-size accessors.  C's bodies are `(void)`
   (C 120, 125, 126) or empty (C 121, 122), so the port's bodies are the unit expression; the
   two size accessors read `SELECTED_FRAME ()`, which the port spells with the same name
   (lisp.h's macro, PORTING.md 0.1-9).  */
/// C: #define XFreeGC (void)
macro_rules! XFreeGC { () => { () }; }
/// C: #define x_destroy_bitmap(p1,p2)
macro_rules! x_destroy_bitmap { ($p1:expr, $p2:expr) => { () }; }
/// C: #define XGetGeometry(p1,p2,p3,p4,p5,p6,p7,p8,p9)
macro_rules! XGetGeometry {
  ($p1:expr, $p2:expr, $p3:expr, $p4:expr, $p5:expr, $p6:expr, $p7:expr, $p8:expr, $p9:expr) =>
    { () };
}
/// C: #define DisplayWidth(p1,p2) (SELECTED_FRAME()->text_cols)
macro_rules! DisplayWidth { ($p1:expr, $p2:expr) => { (SELECTED_FRAME ()).text_cols }; }
/// C: #define DisplayHeight(p1,p2) (SELECTED_FRAME()->text_lines)
macro_rules! DisplayHeight { ($p1:expr, $p2:expr) => { (SELECTED_FRAME ()).text_lines }; }
/// C: #define XMenuSetFreeze (void)
macro_rules! XMenuSetFreeze { () => { () }; }
/// C: #define XMenuRecompute (void)
macro_rules! XMenuRecompute { () => { () }; }

/// C 127-130: the four XMenu status values.
/// C: #define XM_FAILURE -1
pub const XM_FAILURE: c_int = -1;
/// C 128: `#define XM_SUCCESS 1`
pub const XM_SUCCESS: c_int = 1;
/// C 129: `#define XM_NO_SELECT 2`
pub const XM_NO_SELECT: c_int = 2;
/// C 130: `#define XM_IA_SELECT 3`
pub const XM_IA_SELECT: c_int = 3;
/// C 131: `#define ButtonReleaseMask 0`
pub const ButtonReleaseMask: c_int = 0;

/// C 133-143: `struct x_menu_struct` - the menu record the MS-DOS XMenu façade fills in.  The
/// field order and names are C's; `submenu` points at the record itself (C 137), `text` and
/// `help_text` are C's `char **`/`const char **`.
/// C: typedef struct x_menu_struct
/// C: {
/// C:   int count;
/// C:   char **text;
/// C:   struct x_menu_struct **submenu;
/// C:   int *panenumber; /* Also used as enable.  */
/// C:   int allocated;
/// C:   int panecount;
/// C:   int width;
/// C:   const char **help_text;
/// C: } XMenu;
#[repr(C)]
pub struct x_menu_struct
{
  pub count: c_int,
  pub text: *mut *mut c_char,
  pub submenu: *mut *mut x_menu_struct,
  pub panenumber: *mut c_int,
  pub allocated: c_int,
  pub panecount: c_int,
  pub width: c_int,
  pub help_text: *mut *const c_char,
}
/// C 143's typedef name.
pub type XMenu = x_menu_struct;

/// C 145-152: the XMenu façade.  C 151's callback is a plain function pointer, which is
/// nullable, so it is an `Option` of an `extern "C"` fn (PORTING.md P-27); `Display`/`Window`
/// are this header's own dummy types (C 95-96) and `unsigned` (C 150) is `c_uint`.
/// C: XMenu *XMenuCreate (Display *, Window, char *);
/// C: int XMenuAddPane (Display *, XMenu *, char const *, int);
/// C: int XMenuAddSelection (Display *, XMenu *, int, int, char *, int, char const *);
/// C: void XMenuLocate (Display *, XMenu *, int, int, int, int,
/// C: 		  int *, int *, int *, int *);
/// C: int XMenuActivate (Display *, XMenu *, int *, int *, int, int, unsigned,
/// C: 		   char **, void (*callback)(char const *, int, int));
/// C: void XMenuDestroy (Display *, XMenu *);
unsafe extern "C" {
  pub fn XMenuCreate (foo1: *mut Display, foo2: Window, foo3: *mut c_char) -> *mut XMenu;
  pub fn XMenuAddPane (foo: *mut Display, menu: *mut XMenu, txt: *const c_char,
                       enable: c_int) -> c_int;
  pub fn XMenuAddSelection (bar: *mut Display, menu: *mut XMenu, pane: c_int, foo: c_int,
                            txt: *mut c_char, enable: c_int, help_text: *const c_char) -> c_int;
  pub fn XMenuLocate (foo0: *mut Display, menu: *mut XMenu, foo1: c_int, foo2: c_int,
                      x: c_int, y: c_int, ulx: *mut c_int, uly: *mut c_int,
                      width: *mut c_int, height: *mut c_int);
  pub fn XMenuActivate (foo: *mut Display, menu: *mut XMenu, pane: *mut c_int,
                        selidx: *mut c_int, x0: c_int, y0: c_int, ButtonMask: c_uint,
                        txt: *mut *mut c_char,
                        help_callback: Option<unsafe extern "C" fn (*const c_char, c_int,
                                                                    c_int)>);
  pub fn XMenuDestroy (foo: *mut Display, menu: *mut XMenu);
}
