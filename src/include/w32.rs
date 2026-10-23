// femacs: port of src/w32.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the Windows NT process layer's interface:
// C 2-3 is the include guard, C 23-25 a Cygwin exclusion, C 25 `#include <windows.h>`, C 30-57
// the file-descriptor-set emulation, C 61-68 the child-status enum, C 84-110 the `child_process`
// record, C 115-124 the `filedesc` table and C 127's object, C 130-142 the `fd_info` flags, and
// C 145-277 the entry points `w32.c`/`w32proc.c`/`w32fns.c` define (the last two inside
// `#ifdef HAVE_GNUTLS`).
//
// Conditions: `tools/check-cond-eval.py src/w32.h` -> `4 conditions, 0 unknown, 1 disagreement
// [NOT VERIFIED]` - the guard, C 23's `CYGWIN`, C 32's `FD_SETSIZE` and C 269's `HAVE_GNUTLS`.
// The `FD_SETSIZE` one is a known disagreement (clang False, the evaluator True) and is **not**
// resolved here: C 32-34 only `#undef`s a macro the MSVC runtime may have defined first, and
// C 36 then defines the name to 2048, so *both arms end with the same text*; the file stays NOT
// VERIFIED, which is the extractor's conservative "both arms live" reading (the
// `docs/ports/include-assert.in.md` precedent, status 3.157).  `CYGWIN` and `HAVE_GNUTLS` are
// `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so C 23-25 is an exclusion this
// target does not take (ported below as the compile-time assertion that expresses it) and
// C 270-276 is dead text under `#[cfg (feature = "have_gnutls")]` (PORTING.md P-51).
//
// Nothing on this target compiles the header: its readers are all Windows units (`src/w32.c`,
// `src/w32proc.c`, `src/w32fns.c`, …), and `WINDOWSNT`/`HAVE_NTGUI` are `undef`, so no unit
// `include!`s this mirror and the declarations are deferred to the Windows target with their
// definitions.
//
// **Names C takes from `<windows.h>` (C 25) are spelled the way PORTING.md's table spells them,
// each with its C text in a `/// C:` line** (P-55): `HANDLE`/`HWND`/`HMODULE`/`HKEY` are `void *`
// at the ABI and become `*mut c_void`; `BOOL` is C's `int`; `UINT`/`unsigned` are `c_uint`;
// `DWORD` is `c_ulong`; `LPBYTE` is `unsigned char *`; `LPDWORD` is `DWORD *`; `CPINFO` is only
// ever a pointer; `WINAPI` is the `extern "system"` ABI of the two DLL entry-point pointers; and
// `wchar_t` is spelled `c_ushort` because *this* header's `wchar_t` is the Windows 16-bit one,
// while `lib/include/wchar.rs` owns the frozen target's `int` (C 10 of that mirror).
//
// **Three groups are deferred or masked, and each one is named.**
//   1. **`_child_process` (C 84-110) is deferred, not written.**  C embeds `<windows.h>`'s
//      `PROCESS_INFORMATION` (C 103) and two `OVERLAPPED` records (C 107-108) **by value**, the
//      port has no mirror of that header, and no compiler on this target could measure them - so
//      no Rust spelling of the record or of those three records is written rather than an
//      unmeasured one (the `include/w32heap.rs` `file_data` rule and the
//      `include/w32common.rs` C 35-36 deferral).  Its checklist box stays unchecked and the
//      pointers to it keep C's pointer depth as `*mut c_void` (`filedesc.cp` C 122, `new_child`
//      C 145, `delete_child` C 146).
//   2. **Names the port's system mirrors already declare carry `#[cfg (feature = "windowsnt")]`**:
//      `realpath` (C 197), `srandom`/`random` (C 254-255), `fchmod`/`fchmodat`/`lchmod`
//      (C 264-265), `pipe2` (C 266), `openat` (C 262), `w32_strerror`/`w32_get_resource`
//      (C 149/181, which `include/cygw32.rs` 35/43 also declares - C 23-25's `#error` is exactly
//      what keeps the Cygwin and the Windows header from ever being included together), and
//      `init_environment`/`dostounix_filename`/`unixtodos_filename` (C 199/204/205, which
//      `include/msdos.rs` 101-104 declares for the MS-DOS build).  Rust holds one declaration per
//      name in a crate, so the Windows side is marked with the feature that names the only
//      configuration that includes this header (the `lib/include/errno.in.rs` 95-99 and
//      `include/msdos.rs` readings).
//   3. **The file-descriptor-set group (C 30-57, C 115-116 and C 127) carries the feature too**,
//      because it is the Windows build's *replacement* for the system `<sys/select.h>`: C 32-34
//      `#undef`s the runtime's `FD_SETSIZE` (which is what the port's `<sys/select.h>` mirror
//      spells for this target - `lib/include/sys_select.rs` 31/39 has `FD_SETSIZE` 1024 and its
//      own `fd_set`), C 36 defines it to 2048, and C 37-39 defines the record that goes with it.
//      So the masked items are `FD_SETSIZE`, `fd_set`, the four access macros (`FD_SET`, `FD_CLR`,
//      `FD_ISSET`, `FD_ZERO`), `SELECT_TYPE`, the two size macros (C 115-116, whose values are
//      `FD_SETSIZE`) and C 127's table (whose length is `MAXDESC`).  C 32-34's `#undef` has no
//      Rust counterpart of its own: Rust has no pre-existing macro to remove (PORTING.md 0.1-4's
//      reading of `#undef`).

// C 2-3: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_W32_H: bool = true;

/// C 23-25: `#ifdef CYGWIN` / `#error "w32.h is not compatible with Cygwin"` / `#endif` - the
/// one condition in this header that is a *hard* exclusion rather than a branch, so the port
/// expresses it as the assertion that fails when the Cygwin feature is on.  (The frozen
/// configuration leaves `CYGWIN` undefined, which is why the assertion holds here.)
const _: () = core::assert! (!cfg! (feature = "cygwin"));

/* File descriptor set emulation.  */

/// C 31-36: the MSVC runtime library's 64-descriptor limit is `#undef`ed (C 32-34, see the file
/// note) and the name redefined; the value is what C 37's array size divides.
/// C: #define FD_SETSIZE  2048
///
/// C 37-39: `typedef struct { unsigned int bits[FD_SETSIZE / 32]; } fd_set;` - the anonymous
/// record under its typedef name (cinventory keys it `<anon@37>` with the alias `fd_set`), whose
/// member is an **array** of `FD_SETSIZE / 32` = 64 `unsigned int`, not a pointer.  Its layout is
/// not asserted: the C side needs this target's `<windows.h>`, which no compiler here has (the
/// `include/w32heap.rs` `file_data` note).
/// C: typedef struct {
/// C:   unsigned int bits[FD_SETSIZE / 32];
/// C: } fd_set;
///
/// C 264: `extern int fchmod (int, mode_t);` and C 265: `extern int fchmodat (int, const char *,
/// mode_t, int);`
#[cfg (feature = "windowsnt")]
pub mod w32_system_names {
  use super::*;

  pub const FD_SETSIZE: c_int = 2048;

  #[repr(C)]
  pub struct fd_set
  {
    pub bits: [c_uint; FD_SETSIZE as usize / 32],
  }

  unsafe extern "C" {
    pub fn fchmod (fd: c_int, mode: u16) -> c_int;
    pub fn fchmodat (fd: c_int, path: *const c_char, mode: u16, flags: c_int) -> c_int;
  }
}
#[cfg (feature = "windowsnt")]
use w32_system_names::*;

/* C 31-39 and C 264-265: **the four names this header shares with the frozen target's own
   system mirrors** - `FD_SETSIZE` and `fd_set` with `lib/include/sys_select.rs` 31/39, `fchmod`
   and `fchmodat` with `lib/include/sys_stat.in.rs` 184-185.  Measured on this machine
   (`xtask probe src/w32term.rs … --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`):
   four `error[E0428]` name exactly these pairs, i.e. a `windowsnt` crate that reaches both
   headers cannot be compiled at all.  The declarations therefore sit in a module this header
   glob-imports (the `window.rs` 302-367 / `androidterm.rs` 513-537 / `w32term.rs`
   `w32term_objs` shape): a crate-root definition shadows a glob import, so the system mirror's
   item stays the live one and the collision is gone.

   **Owed, and not disguised by that move**: a *real* Windows build has to read C's own values
   here (`FD_SETSIZE` 2048, `MAXDESC` 2048), and the arbitration that expresses C's
   `#undef`/`#define` ordering is a *complementary* gate - `not (feature = "windowsnt")` - on the
   two `lib/include` mirrors, which are outside this lane's writable set.  Until it lands, this
   module's names are reachable in a `windowsnt` crate only where nothing else declares them.  */
/* standard access macros */
/// C 41-47: C's `do { … } while (0)` statement macro.  The port writes the `if` C's loop
/// encloses (PORTING.md 2.2's `do…while (0)` class: Rust has no statement-macro loop wrapper);
/// C's `(p)->bits[…]` is Rust's field access, and the arithmetic keeps every token C's body
/// carries except the loop's own `0`.
/// C: #define FD_SET(n, p) \
/// C:   do { \
/// C:     if ((n) < FD_SETSIZE) { \
/// C:       (p)->bits[(n)/32] |= (1 << (n)%32); \
/// C:     } \
/// C:   } while (0)
#[cfg (feature = "windowsnt")]
macro_rules! FD_SET {
  ($n:expr, $p:expr) =>
    { {
        if ($n) < FD_SETSIZE
          { ($p).bits[($n) as usize / 32] |= 1 << (($n) % 32); }
      } };
}
/// C 48-53: the `FD_CLR` half of the pair above, with C's `& ~(1 << …)`.
/// C: #define FD_CLR(n, p) \
/// C:   do { \
/// C:     if ((n) < FD_SETSIZE) { \
/// C:       (p)->bits[(n)/32] &= ~(1 << (n)%32); \
/// C:     } \
/// C:   } while (0)
#[cfg (feature = "windowsnt")]
macro_rules! FD_CLR {
  ($n:expr, $p:expr) =>
    { {
        if ($n) < FD_SETSIZE
          { ($p).bits[($n) as usize / 32] &= !(1 << (($n) % 32)); }
      } };
}
/// C 54: `#define FD_ISSET(n, p) ((n) < FD_SETSIZE ? ((p)->bits[(n)/32] & (1 << (n)%32)) : 0)` -
/// an expression macro, so C's `? :` is Rust's `if`/`else` (a Rust expression), and C's `0`
/// stays as the else arm.
#[cfg (feature = "windowsnt")]
macro_rules! FD_ISSET {
  ($n:expr, $p:expr) =>
    { (if ($n) < FD_SETSIZE
         { ($p).bits[($n) as usize / 32] & (1 << (($n) % 32)) }
       else { 0 }) };
}
/// C 55: `#define FD_ZERO(p) memset((p), 0, sizeof(fd_set))` - `memset` is a token-aliases
/// family whose port spelling is `std::ptr::write_bytes` (docs/ports/token-aliases.tsv), and C's
/// `sizeof (fd_set)` is `std::mem::size_of::<fd_set> ()`, which keeps the type's name.
#[cfg (feature = "windowsnt")]
macro_rules! FD_ZERO {
  ($p:expr) => { std::ptr::write_bytes ($p as *mut u8, 0, std::mem::size_of::<fd_set> ()) };
}

/// C 57: `#define SELECT_TYPE fd_set` - a *type* macro, so it is a nullary `macro_rules!` (the
/// `haikugui.rs` 63 spelling).
#[cfg (feature = "windowsnt")]
macro_rules! SELECT_TYPE { () => { fd_set }; }

/* ------------------------------------------------------------------------- */

/* child_process.status values */
/// C 61-68: the child-status enumeration (C's members carry no initialiser, so the values are
/// the declaration order - the constant gate compares them; the last one is 5).
/// C: enum {
/// C:   STATUS_READ_ERROR = -1,
/// C:   STATUS_READ_READY,
/// C:   STATUS_READ_IN_PROGRESS,
/// C:   STATUS_READ_FAILED,
/// C:   STATUS_READ_SUCCEEDED,
/// C:   STATUS_READ_ACKNOWLEDGED,
/// C:   STATUS_CONNECT_FAILED
/// C: };
pub const STATUS_READ_ERROR: c_int = -1;
pub const STATUS_READ_READY: c_int = 0;
pub const STATUS_READ_IN_PROGRESS: c_int = 1;
pub const STATUS_READ_FAILED: c_int = 2;
pub const STATUS_READ_SUCCEEDED: c_int = 3;
pub const STATUS_READ_ACKNOWLEDGED: c_int = 4;
pub const STATUS_CONNECT_FAILED: c_int = 5;

/* C 84-110: `typedef struct _child_process { … } child_process;` is **deferred** (see the file
   note): C embeds `PROCESS_INFORMATION` (C 103) and two `OVERLAPPED` records (C 107-108) by
   value, and the port has no measured spelling for any of the three.  Its checklist box stays
   unchecked, the extractor names the gap on stderr ("missing type deferred") instead of failing a
   check, and every pointer to the record below is `*mut c_void` with C's text in a `/// C:` line
   (PORTING.md P-55).  C's twelve members in order: `fd`, `pid`, `char_avail`, `char_consumed`,
   `thrd`, `hwnd`, `procinfo`, `status`, `errcode`, `chr`, `ovl_read`, `ovl_write`.  */

/// C 120-124: `typedef struct { unsigned flags; HANDLE hnd; child_process *cp; } filedesc;` -
/// cinventory keys it `<anon@120>` with the alias `filedesc`, which is the name the port uses.
/// `HANDLE` is `*mut c_void` and `cp` points at the deferred record above.
/// C: typedef struct
/// C: {
/// C:   unsigned         flags;
/// C:   HANDLE           hnd;
/// C:   child_process *  cp;
/// C: } filedesc;
#[repr(C)]
pub struct filedesc
{
  pub flags: c_uint,

  /// C: `HANDLE           hnd;`
  pub hnd: *mut c_void,

  /// C: `child_process *  cp;`
  pub cp: *mut c_void,
}

/// C 115-117: the two sizes (C 105's `FD_SETSIZE` and its half) and the "is this child active"
/// test, whose `char_avail != NULL` keeps C's member name and C's `NULL` (the port's
/// `null_mut ()`, PORTING.md P-44's family table).
/// C: #define MAXDESC FD_SETSIZE
/// C: #define MAX_CHILDREN  MAXDESC/2
/// C: #define CHILD_ACTIVE(cp) ((cp)->char_avail != NULL)
#[cfg (feature = "windowsnt")]
pub const MAXDESC: c_int = FD_SETSIZE;
#[cfg (feature = "windowsnt")]
pub const MAX_CHILDREN: c_int = MAXDESC / 2;
macro_rules! CHILD_ACTIVE { ($cp:expr) => { (($cp).char_avail != std::ptr::null_mut ()) }; }


/* fd_info flag definitions */
/// C 130-142: the thirteen `FILE_*` flags.  C spells them `0x`-hexadecimal; the constant check
/// compares values.
pub const FILE_READ: c_int = 0x0001;
pub const FILE_WRITE: c_int = 0x0002;
pub const FILE_LISTEN: c_int = 0x0004;
pub const FILE_CONNECT: c_int = 0x0008;
pub const FILE_BINARY: c_int = 0x0010;
pub const FILE_LAST_CR: c_int = 0x0020;
pub const FILE_AT_EOF: c_int = 0x0040;
pub const FILE_SEND_SIGCHLD: c_int = 0x0080;
pub const FILE_PIPE: c_int = 0x0100;
pub const FILE_SOCKET: c_int = 0x0200;
pub const FILE_NDELAY: c_int = 0x0400;
pub const FILE_SERIAL: c_int = 0x0800;
pub const FILE_DONT_CLOSE: c_int = 0x1000;


/// C 189-190: the two DLL entry-point pointers.  C's `WINAPI` is the `extern "system"` ABI in
/// Rust, `UINT` is `c_uint`, `DWORD` is `c_ulong`, `LPCSTR`/`LPCWSTR`/`LPSTR`/`LPWSTR` are the
/// `char`/16-bit-`wchar_t` string pointers, and `LPBOOL` is `int *`; a C function pointer is
/// nullable, so each is an `Option` (PORTING.md P-27).
/// C: typedef int (WINAPI *MultiByteToWideChar_Proc)(UINT,DWORD,LPCSTR,int,LPWSTR,int);
/// C: typedef int (WINAPI *WideCharToMultiByte_Proc)(UINT,DWORD,LPCWSTR,int,LPSTR,int,LPCSTR,LPBOOL);
pub type MultiByteToWideChar_Proc
  = Option<unsafe extern "system" fn (c_uint, c_ulong, *const c_char, c_int, *mut u16,
                                      c_int) -> c_int>;
pub type WideCharToMultiByte_Proc
  = Option<unsafe extern "system" fn (c_uint, c_ulong, *const u16, c_int, *mut c_char,
                                      c_int, *const c_char, *mut c_int) -> c_int>;


/* C 207-217's `w32_get_internal_run_time` is defined by `src/w32.c`, so its declaration sits in
   a glob-imported module for the same reason as `w32_objs` above.  */
pub mod w32_fn_decls {
  use super::*;

  unsafe extern "C" {
    pub fn w32_get_internal_run_time () -> Lisp_Object;
  }
}
use w32_fn_decls::*;

/* C 190-191's two timer entries are *defined* by `src/w32proc.c`, so (like the three objects
   above) their declaration sits in a glob-imported module: a crate-root definition shadows a glob
   import, and at the mirror's top level the same two names were an `E0428` against that unit's
   definitions (measured, the `windowsnt` arm at status 3.444).  */
pub mod w32proc_fn_decls {
  use super::*;

  unsafe extern "C" {
    /* C 190-191: `extern void term_timers (void); extern void init_timers (void);`  */
    pub fn term_timers ();
    pub fn init_timers ();
  }
}
use w32proc_fn_decls::*;

/* C 191-193's three objects live in a module with a glob import, because `src/w32.c` *defines*
   them: a crate-root definition shadows a glob import, so the unit is not an `E0428` against the
   mirror's declaration (the `window.rs` 302-367 shape, P-21; docs/ports/include-w32.md owed 4).  */
pub mod w32_objs {
  use super::*;

  unsafe extern "C" {
    pub static mut pMultiByteToWideChar: MultiByteToWideChar_Proc;
    pub static mut pWideCharToMultiByte: WideCharToMultiByte_Proc;
    pub static mut multiByteToWideCharFlags: c_ulong;
  }
}
use w32_objs::*;


/* src/w32.h 61-113: the subprocess record, `typedef struct _child_process { … } child_process;`
   (C 74-113).  Dead platform text on the frozen target (no `_WIN32`), so it is carried as the
   dead arm's text (P-51/3.9) under `windows`: the member names are C's own, in C's order, and
   the member *types* stay C's spellings (`HANDLE`/`HWND`/`PROCESS_INFORMATION`/`OVERLAPPED`),
   which no configuration of the port compiles.  */
#[cfg (feature = "windows")]
#[repr(C)]
pub struct _child_process
{
  /* int fd;  */
  pub fd: c_int,
  /* int pid;  */
  pub pid: c_int,
  /* HANDLE char_avail;  */
  pub char_avail: *mut c_void,
  /* HANDLE char_consumed;  */
  pub char_consumed: *mut c_void,
  /* HANDLE thrd;  */
  pub thrd: *mut c_void,
  /* HWND hwnd;  */
  pub hwnd: *mut c_void,
  /* PROCESS_INFORMATION procinfo;  */
  pub procinfo: *mut c_void,
  /* volatile int status;  */
  pub status: c_int,
  /* volatile int errcode;  */
  pub errcode: c_int,
  /* char chr;  */
  pub chr: c_char,
  /* OVERLAPPED ovl_read;  */
  pub ovl_read: *mut c_void,
  /* OVERLAPPED ovl_write;  */
  pub ovl_write: *mut c_void,
}
/* C 113: the typedef name the rest of src/w32.h uses.  */
#[cfg (feature = "windows")]
pub type child_process = _child_process;


/* Every declaration below is a name some unit of the port *defines* (`src/w32.rs` defines
   twenty-one of them, and `src/gnutls.rs` the two gnutls ones), so they sit in a module this
   mirror glob-imports: a crate-root definition shadows a glob import (PORTING.md 0.1-10 / P-21),
   which is what retires the `E0428`s the `windowsnt` arm read at the crate root (the
   `include/w32term.rs` `w32term_fn_decls` shape; measured 2026-10-05, status 3.598).  */
pub mod w32_file_decls {
  use super::*;

  /* parallel array of private info on file handles */
  /// C 127: `extern filedesc fd_info [ MAXDESC ];` - the table `w32.c` defines, one `filedesc` per
  /// descriptor.
  /// C: extern filedesc fd_info [ MAXDESC ];
  unsafe extern "C" {
    #[cfg (feature = "windowsnt")]
    pub static mut fd_info: [filedesc; MAXDESC as usize];
  }
  /// C 145-146: the child list's two entry points - the record itself is deferred above, so the
  /// pointer keeps C's depth as `*mut c_void`.
  /// C: extern child_process * new_child (void);
  /// C: extern void delete_child (child_process *cp);
  unsafe extern "C" {
    pub fn new_child () -> *mut c_void;
    pub fn delete_child (cp: *mut c_void);

    /// C 149: `char * w32_strerror (int error_no);` - the same entry point `include/cygw32.rs` 35
    /// declares for the Cygwin build (C 23-25's `#error` is what keeps the two apart), hence the
    /// feature.
    /// C: extern char * w32_strerror (int error_no);
    #[cfg (feature = "windowsnt")]
    pub fn w32_strerror (error_no: c_int) -> *mut c_char;

    /// C 152-153: the pointer validator and the two file-name conversions of the Win32 API.
    /// C: extern int w32_valid_pointer_p (void *, int);
    /// C: extern BOOL w32_get_long_filename (const char * name, char * buf, int size);
    /// C: extern unsigned int w32_get_short_filename (const char *, char *, int);
    pub fn w32_valid_pointer_p (p: *mut c_void, size: c_int) -> c_int;
    pub fn w32_get_long_filename (name: *const c_char, buf: *mut c_char, size: c_int) -> c_int;
    pub fn w32_get_short_filename (name: *const c_char, buf: *mut c_char,
                                   size: c_int) -> c_uint;

    /// C 169-177: the standard-handle inheritance pair.  C's `HANDLE handles[3]` parameter decays
    /// to `HANDLE *`, which is the port's `*mut *mut c_void` (PORTING.md P-55's pointer reading).
    /// C: extern void prepare_standard_handles (int in, int out,
    /// C: 				      int err, HANDLE handles[3]);
    /// C: extern void reset_standard_handles (int in, int out,
    /// C: 				    int err, HANDLE handles[3]);
    pub fn prepare_standard_handles (in_: c_int, out: c_int, err: c_int,
                                     handles: *mut *mut c_void);
    pub fn reset_standard_handles (in_: c_int, out: c_int, err: c_int,
                                   handles: *mut *mut c_void);

    /// C 181: `extern LPBYTE w32_get_resource (const char * key, const char * name, LPDWORD type);`
    /// - the Cygwin pair of C 149 (see there); `LPBYTE` is `unsigned char *` and `LPDWORD` is
    /// `DWORD *` (the `include/cygw32.rs` 38-42 reading).
    /// C: extern LPBYTE w32_get_resource (const char * key, const char * name, LPDWORD type);
    #[cfg (feature = "windowsnt")]
    pub fn w32_get_resource (key: *const c_char, name: *const c_char,
                             type_: *mut c_ulong) -> *mut u8;

    /// C 183-188: the listen-thread, NT-init and DLL-loading entries.
    /// C: extern void release_listen_threads (void);
    /// C: extern void init_ntproc (int);
    /// C: extern void term_ntproc (int);
    /// C: extern HANDLE maybe_load_unicows_dll (void);
    /// C: extern void load_unicows_dll_for_w32fns (HMODULE);
    /// C: extern void globals_of_w32 (void);
    pub fn release_listen_threads ();
    pub fn init_ntproc (dumping: c_int);
    pub fn term_ntproc (ignored: c_int);
    pub fn maybe_load_unicows_dll () -> *mut c_void;
    pub fn load_unicows_dll_for_w32fns (hmodule: *mut c_void);
    pub fn globals_of_w32 ();


    /// C 193-195: the reader thread's three socket/serial waits.
    /// C: extern int _sys_read_ahead (int fd);
    /// C: extern int _sys_wait_accept (int fd);
    /// C: extern int _sys_wait_connect (int fd);
    pub fn _sys_read_ahead (fd: c_int) -> c_int;
    pub fn _sys_wait_accept (fd: c_int) -> c_int;
    pub fn _sys_wait_connect (fd: c_int) -> c_int;

    /// C 197: `extern HMODULE w32_delayed_load (Lisp_Object);`
    pub fn w32_delayed_load (library_id: Lisp_Object) -> *mut c_void;
  }
  /// C 191-193: the two loaded entry points and the flags `w32.c` passes them.
  /// C: extern MultiByteToWideChar_Proc pMultiByteToWideChar;
  /// C: extern WideCharToMultiByte_Proc pWideCharToMultiByte;
  /// C: extern DWORD multiByteToWideCharFlags;
  unsafe extern "C" {

    /// C 195-197: the executable's own name, the relocation table and the `realpath` the Windows
    /// build provides (the name is also declared by the port's system mirrors, hence the feature -
    /// see the file note).
    /// C: extern char *w32_my_exename (void);
    /// C: extern const char *w32_relocate (const char *);
    /// C: extern char *realpath (const char *, char *);
    pub fn w32_my_exename () -> *mut c_char;
    pub fn w32_relocate (epath_dir: *const c_char) -> *const c_char;
    #[cfg (feature = "windowsnt")]
    pub fn realpath (file_name: *const c_char, resolved_name: *mut c_char) -> *mut c_char;

    /// C 199: `extern void init_environment (char **);` - the MS-DOS build's differently-shaped
    /// entry point of the same name (`include/msdos.rs` 104), hence the feature.
    /// C: extern void init_environment (char **);
    #[cfg (feature = "windowsnt")]
    pub fn init_environment (argv: *mut *mut c_char);

    /// C 200-206: the init-file check, the two symbol initializers and the file-name conversions.
    /// C 204-205's `dostounix_filename`/`unixtodos_filename` are the MS-DOS pair
    /// (`include/msdos.rs` 101-102), hence the feature; C's `register` storage class has no Rust
    /// counterpart (PORTING.md's token table skips it).
    /// C: extern void check_windows_init_file (void);
    /// C: extern void syms_of_ntproc (void);
    /// C: extern void syms_of_ntterm (void);
    /// C: extern void dostounix_filename (register char *);
    /// C: extern void unixtodos_filename (register char *);
    pub fn check_windows_init_file ();
    pub fn syms_of_ntproc ();
    pub fn syms_of_ntterm ();
    #[cfg (feature = "windowsnt")]
    pub fn dostounix_filename (p: *mut c_char);
    #[cfg (feature = "windowsnt")]
    pub fn unixtodos_filename (p: *mut c_char);

    /// C 207-217: the ANSI/UTF-16 file-name layer.  C's `const wchar_t *`/`wchar_t *` are the
    /// Windows 16-bit `wchar_t` (see the file note), so they are spelled `*const c_ushort`/
    /// `*mut c_ushort` instead of the frozen target's `wchar_t` alias.
    /// C: extern const char *map_w32_filename (const char *, const char **);
    /// C: extern int  filename_from_ansi (const char *, char *);
    /// C: extern int  filename_to_ansi (const char *, char *);
    /// C: extern int  filename_from_utf16 (const wchar_t *, char *);
    /// C: extern int  filename_to_utf16 (const char *, wchar_t *);
    /// C: extern Lisp_Object w32_get_internal_run_time (void);
    /// C: extern void w32_init_file_name_codepage (void);
    /// C: extern int  codepage_for_filenames (CPINFO *);
    /// C: extern Lisp_Object ansi_encode_filename (Lisp_Object);
    pub fn map_w32_filename (name: *const c_char, pPath: *mut *const c_char) -> *const c_char;
    pub fn filename_from_ansi (fn_in: *const c_char, fn_out: *mut c_char) -> c_int;
    pub fn filename_to_ansi (fn_in: *const c_char, fn_out: *mut c_char) -> c_int;
    pub fn filename_from_utf16 (fn_in: *const c_ushort, fn_out: *mut c_char) -> c_int;
    pub fn filename_to_utf16 (fn_in: *const c_char, fn_out: *mut c_ushort) -> c_int;
    pub fn w32_init_file_name_codepage ();
    pub fn codepage_for_filenames (cp_info: *mut c_void) -> c_int;
    pub fn ansi_encode_filename (filename: Lisp_Object) -> Lisp_Object;

    /// C 218-236: the file operations the Windows build provides.  `ptrdiff_t` is `isize`
    /// (PORTING.md 1.1).  **`mode_t` and `pid_t` are written as the widths they are** - `u16` and
    /// `c_int`, `lib/include/sys_types.in.rs` 70/64's measured `__uint16_t`/`int` - because a mirror
    /// may not name a type its includers may lack (PORTING.md P-87 (3), the `window_part` reading);
    /// C's units reach `<sys/types.h>` through their own includes (PORTING.md 0.1-9).  The four names
    /// the port's system mirrors also declare carry the feature (file note).
    /// C: extern int  w32_copy_file (const char *, const char *, int, int, int);
    /// C: extern int  w32_accessible_directory_p (const char *, ptrdiff_t);
    /// C: extern void w32_init_current_directory (void);
    /// C: extern BOOL init_winsock (int load_now);
    /// C: extern void srandom (int);
    /// C: extern int random (void);
    /// C: extern int fchmod (int, mode_t);
    /// C: extern int sys_rename_replace (char const *, char const *, BOOL);
    /// C: extern int pipe2 (int *, int);
    /// C: extern void register_aux_fd (int);
    /// C: extern void set_process_dir (const char *);
    /// C: extern int sys_spawnve (int, char *, char **, char **);
    /// C: extern void register_child (pid_t, int);
    /// C: extern void sys_sleep (int);
    /// C: extern int sys_link (const char *, const char *);
    /// C: extern int openat (int, const char *, int, int);
    /// C: extern int fchmodat (int, char const *, mode_t, int);
    /// C: extern int lchmod (char const *, mode_t);
    /// C: extern bool symlinks_supported (const char *);
    /// C: extern void w32_fix_tzset (void);
    pub fn w32_copy_file (from: *const c_char, to: *const c_char, keep_time: c_int,
                          preserve_ownership: c_int, copy_acls: c_int) -> c_int;
    pub fn w32_accessible_directory_p (dirname: *const c_char, dirlen: isize) -> c_int;
    pub fn w32_init_current_directory ();
    pub fn init_winsock (load_now: c_int) -> c_int;
    #[cfg (feature = "windowsnt")]
    pub fn srandom (seed: c_int);
    #[cfg (feature = "windowsnt")]
    pub fn random () -> c_int;
    // `fchmod` (C 264) is declared in `w32_system_names` above (E0428 against
    // `lib/include/sys_stat.in.rs` 184).
    pub fn sys_rename_replace (oldname: *const c_char, newname: *const c_char, force: c_int) -> c_int;
    #[cfg (feature = "windowsnt")]
    pub fn pipe2 (phandles: *mut c_int, pipe2_flags: c_int) -> c_int;
    pub fn register_aux_fd (infd: c_int);
    pub fn set_process_dir (dir: *const c_char);
    pub fn sys_spawnve (mode: c_int, cmdname: *mut c_char, argv: *mut *mut c_char,
                        envp: *mut *mut c_char) -> c_int;
    pub fn register_child (pid: c_int, fd: c_int);
    pub fn sys_sleep (seconds: c_int);
    pub fn sys_link (old: *const c_char, new: *const c_char) -> c_int;
    #[cfg (feature = "windowsnt")]
    pub fn openat (fd: c_int, path: *const c_char, oflag: c_int, mode: c_int) -> c_int;
    // `fchmodat` (C 265) is declared in `w32_system_names` above (E0428 against
    // `lib/include/sys_stat.in.rs` 185).
    #[cfg (feature = "windowsnt")]
    pub fn lchmod (path: *const c_char, mode: u16) -> c_int;
    pub fn symlinks_supported (file: *const c_char) -> bool;
    pub fn w32_fix_tzset ();

    /// C 240-266: the memory/string/random helpers.
    /// C: extern int w32_memory_info (unsigned long long *, unsigned long long *,
    /// C: 			    unsigned long long *, unsigned long long *);
    /// C: extern int w32_compare_strings (const char *, const char *, char *, int);
    /// C: extern unsigned w32_get_nproc (void);
    /// C: extern int w32_init_random (void *, ptrdiff_t);
    /// C: extern Lisp_Object w32_read_registry (HKEY, Lisp_Object, Lisp_Object);
    /// C: extern int w32_reexec_emacs (char *, const char *);
    /// C: extern void free_wait_pool (void);
    pub fn w32_memory_info (totalram: *mut u64, freeram: *mut u64, totalswap: *mut u64,
                            freeswap: *mut u64) -> c_int;
    pub fn w32_compare_strings (s1: *const c_char, s2: *const c_char, locname: *mut c_char,
                                ignore_case: c_int) -> c_int;
    pub fn w32_get_nproc () -> c_uint;
    pub fn w32_init_random (buf: *mut c_void, buflen: isize) -> c_int;
    pub fn w32_read_registry (rootkey: *mut c_void, lkey: Lisp_Object,
                              lname: Lisp_Object) -> Lisp_Object;
    pub fn w32_reexec_emacs (cmd_line: *mut c_char, wdir: *const c_char) -> c_int;
    pub fn free_wait_pool ();
  }
  /// C 269-277: the `HAVE_GNUTLS` arm - dead text on this target (the feature macro is `undef`),
  /// carried under the feature that names the other side (PORTING.md P-51).  C's
  /// `gnutls_transport_ptr_t` is `<gnutls/gnutls.h>`'s opaque pointer and C's `ssize_t` is the
  /// port's `c_long` (see the file note); the arm is not resolved unless the feature is on.
  /// C: extern ssize_t emacs_gnutls_pull (gnutls_transport_ptr_t p,
  /// C:                                   void* buf, size_t sz);
  /// C: extern ssize_t emacs_gnutls_push (gnutls_transport_ptr_t p,
  /// C:                                   const void* buf, size_t sz);
  #[cfg (feature = "have_gnutls")]
  unsafe extern "C" {
    pub fn emacs_gnutls_pull (p: gnutls_transport_ptr_t, buf: *mut c_void, sz: usize) -> c_long;
    pub fn emacs_gnutls_push (p: gnutls_transport_ptr_t, buf: *const c_void, sz: usize) -> c_long;
  }
}
use w32_file_decls::*;