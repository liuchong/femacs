// femacs: port of src/decompress.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  `HAVE_ZLIB` is `undef` in the frozen configuration
// (docs/baseline/config/darwin-arm64.macros.tsv 1721), so C compiles nothing from this file but
// C 19's `<config.h>`; this is a P-86 unit, and **both arms of every `#if` are carried**, each item
// under the gate that names C's own macro:
//
//   * the file gate (`#ifdef HAVE_ZLIB`, C 21) is `feature = "have_zlib"`, on every item *and on
//     C 23-28's include graph* - which is why the frozen configuration compiles nothing here;
//   * C 30-65's `#ifdef WINDOWSNT` arm is `feature = "windowsnt"` (nested inside the file gate),
//     the name `src/vm-limit.rs` 95 already uses for that configuration;
//   * C 68-167's `#ifdef HAVE_NATIVE_COMP` arm is `feature = "have_native_comp"` (nested too), the
//     feature `docs/baseline/feature-coverage.tsv` records as `xtask dead-arms`-flippable.
//
// C 23's `<zlib.h>` has no file in the reference tree, so this unit carries the declarations it
// uses itself (PORTING.md P-88; the layout numbers are clang readings, not memory, and
// `docs/ports/decompress.md` records them).  C 30-65's `DEF_DLL_FN`/`LOAD_DLL_FN` are two of the
// macros `emacs/src/w32common.h` 77-97 defines and `src/include/w32common.rs` keeps
// `deferred` (P-22); this unit is their first consumer and writes their expansion in long form -
// see that arm's own notes.  C 61-63's `#define inflate fn_inflate` and its two siblings rename
// three identifiers; Rust has no object-like `#define`, so the two arms spell the call sites the
// way C's preprocessor makes them differ (`fn_inflate (...)` against `inflate (...)`), and
// `docs/ports/decompress.md` records that class with the rest.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

/* C 25-28's four headers sit inside the file gate, as C's own `#ifdef HAVE_ZLIB` does; `string.h`
   and `limits.h` reach this file through `lisp.h` 28 and 32, and the two system mirrors are
   included here the way `src/sysdep.rs` 59 includes them.  */
#[cfg (feature = "have_zlib")]
const _: () = ();
/* `character.h` reaches C through `buffer.h`, whose `CHAR_STRING` family lives in it (`dired.rs`
   includes the mirror in the same order for the same reason, PORTING.md 0.1-4).  */
#[cfg (feature = "have_zlib")]
const _: () = ();
#[cfg (feature = "have_zlib")]
const _: () = ();
#[cfg (feature = "have_zlib")]
const _: () = ();
#[cfg (feature = "have_zlib")]
const _: () = ();
#[cfg (feature = "have_zlib")]
const _: () = ();
#[cfg (feature = "have_zlib")]
const _: () = ();

/* C 31-33's `#include <windows.h>`, `"w32common.h"` and `"w32.h"`.  `<windows.h>` has no mirror,
   so `HMODULE` is the pointer PORTING.md 0.2 maps it to (`*mut c_void`, the spelling
   `src/include/w32common.rs` 137 already uses for `hbfont_init_w32_funcs`).  */
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
const _: () = ();
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
const _: () = ();

/* ----------------------------------------------------------------------------------------------
   C 23: `#include <zlib.h>` - the reference tree has no `zlib.h` (P-88: a third-party header with
   no file anywhere in the tree), so the declarations this unit uses stand here rather than in a
   new file under `src/include/`, which is the *reference* headers' mirror directory.  Every
   declaration carries the header's own line (P-55).

   Measured with clang against this machine's SDK header
   (`/Applications/Xcode.app/.../MacOSX.sdk/usr/include/zlib.h`, `ZLIB_VERSION` 1.2.12;
   `ZEXTERN`/`ZEXPORT` are `extern`, `z_const` and `FAR` are empty):
   `sizeof (z_stream)` **112**, alignof **8**, offsets `next_in` **0**, `avail_in` **8**,
   `total_in` **16**, `next_out` **24**, `avail_out` **32**, `total_out` **40**, `msg` **48**,
   `state` **56**, `zalloc` **64**, `zfree` **72**, `opaque` **80**, `data_type` **88**,
   `adler` **96**, `reserved` **104**; `MAX_WBITS` **15**, `Z_NULL` **0**, `Z_OK` **0**,
   `Z_STREAM_END` **1**, `Z_NO_FLUSH` **0**; `sizeof (Byte)` **1**, `uInt` **4**, `uLong` **8**,
   `voidpf` **8**, each of the three function pointers **8**.  The probe source, its command and
   its output are in docs/ports/decompress.md's `## Evidence`.  */

/// C: zconf.h 390: `typedef unsigned char  Byte;  /* 8 bits */`
#[cfg (feature = "have_zlib")]
pub type Byte = c_uchar;
/// C: zconf.h 400: `typedef Byte  FAR Bytef;` (`FAR` is empty)
#[cfg (feature = "have_zlib")]
pub type Bytef = Byte;
/// C: zconf.h 393: `typedef unsigned int   uInt;  /* 16 bits or more */`
#[cfg (feature = "have_zlib")]
pub type uInt = c_uint;
/// C: zconf.h 394: `typedef unsigned long  uLong; /* 32 bits or more */`
#[cfg (feature = "have_zlib")]
pub type uLong = c_ulong;
/// C: zconf.h 409: `typedef void FAR   *voidpf;`
#[cfg (feature = "have_zlib")]
pub type voidpf = *mut c_void;

/// C: zlib.h 87: `typedef voidpf (*alloc_func) OF((voidpf opaque, uInt items, uInt size));`
#[cfg (feature = "have_zlib")]
pub type alloc_func = Option<unsafe extern "C" fn (opaque: voidpf, items: uInt, size: uInt)
                                            -> voidpf>;
/// C: zlib.h 88: `typedef void   (*free_func)  OF((voidpf opaque, voidpf address));`
#[cfg (feature = "have_zlib")]
pub type free_func = Option<unsafe extern "C" fn (opaque: voidpf, address: voidpf)>;

/// C: zlib.h 90: `struct internal_state;` - an incomplete type only ever named through a pointer
/// (PORTING.md 0.1-2's zero-sized stand-in; its size never enters a layout).
#[cfg (feature = "have_zlib")]
pub struct internal_state { _private: [u8; 0] }

/// C: zlib.h 92-112, the state record.  C's `z_const` (zconf.h 289-293) is empty unless
/// `ZLIB_CONST` is defined, so `next_in` is a plain `Bytef *` here.
#[repr(C)]
#[cfg (feature = "have_zlib")]
pub struct z_stream_s
{
  /// C: zlib.h 93: `    z_const Bytef *next_in;     /* next input byte */`
  pub next_in: *mut Bytef,
  /// C: zlib.h 94: `    uInt     avail_in;  /* number of bytes available at next_in */`
  pub avail_in: uInt,
  /// C: zlib.h 95: `    uLong    total_in;  /* total number of input bytes read so far */`
  pub total_in: uLong,
  /// C: zlib.h 97: `    Bytef    *next_out; /* next output byte will go here */`
  pub next_out: *mut Bytef,
  /// C: zlib.h 98: `    uInt     avail_out; /* remaining free space at next_out */`
  pub avail_out: uInt,
  /// C: zlib.h 99: `    uLong    total_out; /* total number of bytes output so far */`
  pub total_out: uLong,
  /// C: zlib.h 101: `    z_const char *msg;  /* last error message, NULL if no error */`
  pub msg: *const c_char,
  /// C: zlib.h 102: `    struct internal_state FAR *state; /* not visible to applications */`
  pub state: *mut internal_state,
  /// C: zlib.h 104: `    alloc_func zalloc;  /* used to allocate the internal state */`
  pub zalloc: alloc_func,
  /// C: zlib.h 105: `    free_func  zfree;   /* used to free the internal state */`
  pub zfree: free_func,
  /// C: zlib.h 106: `    voidpf     opaque;  /* private data object passed to zalloc and zfree */`
  pub opaque: voidpf,
  /// C: zlib.h 108-109: `    int     data_type;  /* best guess about the data type: binary or text`
  /// `                           for deflate, or the decoding state for inflate */`
  pub data_type: c_int,
  /// C: zlib.h 110: `    uLong   adler;      /* Adler-32 or CRC-32 value of the uncompressed data */`
  pub adler: uLong,
  /// C: zlib.h 111: `    uLong   reserved;   /* reserved for future use */`
  pub reserved: uLong,
}
/// C: zlib.h 112: `} z_stream;`
#[cfg (feature = "have_zlib")]
pub type z_stream = z_stream_s;
/// C: zlib.h 114: `typedef z_stream FAR *z_streamp;`
#[cfg (feature = "have_zlib")]
pub type z_streamp = *mut z_stream;

/// C: zlib.h 46: `#define ZLIB_VERSION "1.2.12"` - the value the header on this machine carries,
/// which is what C's `inflateInit2` hands the library at every call.
#[cfg (feature = "have_zlib")]
pub const ZLIB_VERSION: *const c_char = c"1.2.12".as_ptr ();

/// C: zconf.h 269-270: `#ifndef MAX_WBITS` / `#  define MAX_WBITS   15 /* 32K LZ77 window */`
#[cfg (feature = "have_zlib")]
pub const MAX_WBITS: c_int = 15;
/// C: zlib.h 174: `#define Z_NO_FLUSH      0`
#[cfg (feature = "have_zlib")]
pub const Z_NO_FLUSH: c_int = 0;
/// C: zlib.h 183: `#define Z_OK            0`
#[cfg (feature = "have_zlib")]
pub const Z_OK: c_int = 0;
/// C: zlib.h 184: `#define Z_STREAM_END    1`
#[cfg (feature = "have_zlib")]
pub const Z_STREAM_END: c_int = 1;
/// C: zlib.h 218: `#define Z_NULL  0  /* for initializing zalloc, zfree, opaque */`
#[cfg (feature = "have_zlib")]
pub const Z_NULL: c_int = 0;

// C: zlib.h 406: `ZEXTERN int ZEXPORT inflate OF((z_streamp strm, int flush));`
// C: zlib.h 526: `ZEXTERN int ZEXPORT inflateEnd OF((z_streamp strm));`
// C: zlib.h 1814-1815: `ZEXTERN int ZEXPORT inflateInit2_ OF((z_streamp strm, int  windowBits,` /
// `                                      const char *version, int stream_size));`
#[cfg (feature = "have_zlib")]
unsafe extern "C" {
  pub fn inflate (strm: z_streamp, flush: c_int) -> c_int;
  pub fn inflateEnd (strm: z_streamp) -> c_int;
  pub fn inflateInit2_ (strm: z_streamp, windowBits: c_int, version: *const c_char,
                        stream_size: c_int) -> c_int;
}

/* C: zlib.h 1842-1844: `#  define inflateInit2(strm, windowBits) \` /
   `          inflateInit2_((strm), (windowBits), ZLIB_VERSION, \` /
   `                        (int)sizeof(z_stream))` - a function-like macro of `<zlib.h>`, carried
   with C's own name (PORTING.md 2.1).  Its body names `inflateInit2_`, which C 63's
   `#define inflateInit2_ fn_inflateInit2_` renames on the `windowsnt` arm; a `macro_rules!` body
   resolves names where it is *defined*, so the two arms need the two definitions below - which is
   what C's preprocessor does to the one body.  */
#[cfg (all (feature = "have_zlib", not (feature = "windowsnt")))]
macro_rules! inflateInit2 {
  ($strm:expr, $windowBits:expr) => {
    inflateInit2_ ($strm, $windowBits, ZLIB_VERSION, size_of::<z_stream> () as c_int)
  };
}
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
macro_rules! inflateInit2 {
  ($strm:expr, $windowBits:expr) => {
    (fn_inflateInit2_.unwrap ()) ($strm, $windowBits, ZLIB_VERSION,
                                  size_of::<z_stream> () as c_int)
  };
}

/* ----------------------------------------------------------------------------------------------
   C 30-65: the `#ifdef WINDOWSNT` arm - the lazy loading of the three zlib entry points from the
   DLL.  `WINDOWSNT` is `undef` here, so no line below is compiled on this target (P-51 names the
   non-frozen side), and this machine has no Windows cross toolchain (no `*-w64-mingw32-gcc`), so
   this arm's evidence is `text-only` (P-86 clause 3 (2)): there is no compile reading of it.  */

/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func)`
/// `__VA_ARGS__; static W32_PFN_##func fn_##func` - the macro
/// `src/include/w32common.rs` keeps `deferred` (P-22).  This unit is its first consumer
/// and writes the expansion in long form: C pastes `W32_PFN_##func` and `fn_##func` together,
/// which Rust cannot do, so both generated names are parameters here and C's `__VA_ARGS__`
/// (the parameter list, with any trailing attributes) is the `$args` fragment.  The two items the
/// macro makes are C's, name for name.
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func)       do { fn_##func = (W32_PFN_##func)`
/// `get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)`.  `do { … } while
/// (false)` is a block expression (PORTING.md 2.1); C's `#func` is the stringified name, which is
/// also a parameter here for the same reason as above; the assignment's cast between two function
/// pointer types is C's, and Rust has no `as` cast to an `Option<fn>` type, so it is the
/// `transmute` that the equal widths make the same operation.  `get_proc_addr` is the mirror's own
/// inline definition (`src/include/w32common.rs` 128-136).
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}

/* C 35-39: the three `DEF_DLL_FN` uses, each expanding to the two names C's macro would paste -
   `W32_PFN_inflateInit2_`/`fn_inflateInit2_`, `W32_PFN_inflate`/`fn_inflate` and
   `W32_PFN_inflateEnd`/`fn_inflateEnd`.  C's `CDECL` is the `__cdecl` attribute, which is empty on
   this target's ABI, so the pointer type is the plain `extern "C"` one.  */
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, inflateInit2_, W32_PFN_inflateInit2_, fn_inflateInit2_,
             (z_streamp, c_int, *const c_char, c_int));
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, inflate, W32_PFN_inflate, fn_inflate, (z_streamp, c_int));
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, inflateEnd, W32_PFN_inflateEnd, fn_inflateEnd, (z_streamp));

/// C 41: `static bool zlib_initialized;` - C's tentative definition, i.e. zero (PORTING.md 0.1-13).
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
static mut zlib_initialized: bool = false;

/// C 43-55: `init_zlib_functions`.  `HMODULE` is a pointer; `w32_delayed_load` is the declaration
/// `src/include/w32.rs` 313-314 carries for `w32.h` 197.
#[cfg (all (feature = "have_zlib", feature = "windowsnt"))]
unsafe fn init_zlib_functions () -> bool
{
  let library: *mut c_void = w32_delayed_load (Qzlib ());

  if library.is_null ()
    { return false; }

  LOAD_DLL_FN! (library, inflateInit2_, W32_PFN_inflateInit2_, fn_inflateInit2_, c"inflateInit2_");
  LOAD_DLL_FN! (library, inflate, W32_PFN_inflate, fn_inflate, c"inflate");
  LOAD_DLL_FN! (library, inflateEnd, W32_PFN_inflateEnd, fn_inflateEnd, c"inflateEnd");
  return true;
}

/* C 57-63: the four `#undef`s and the three `#define`s.  The `#undef`s have no counterpart - the
   four `W32_PFN_`/`fn_` names above are already the names C's macros make - and an object-like
   `#define` has none in Rust either: `#define inflate fn_inflate` renames an identifier, so the
   arms spell the call sites the way C's preprocessor leaves them (`fn_inflate (...)` under this
   gate, `inflate (...)` otherwise; the `ccl.rs` "C's preprocessed text" rule), while the three
   pointer objects and the two `inflateInit2` bodies above keep C's own names.  */

/* ----------------------------------------------------------------------------------------------
   C 68-167: `#ifdef HAVE_NATIVE_COMP`, the md5-stream driver `fileio.c` and `dired.c` call under
   native compilation.  `HAVE_NATIVE_COMP` is `undef` here too (macros.tsv 1473), so this arm is
   dead text on this target and its gate is the feature that names the other side (P-51).  */

/// C 70: `# define MD5_BLOCKSIZE 32768 /* From md5.c  */` - C `#undef`s it at C 166; Rust's
/// `macro_rules!` scope would end with the item, so the constant is gated on the same feature the
/// arm is (the `FIND_METHOD` note's shape in `androidmenu.rs`, whose macros C `#undef`s too).
#[cfg (all (feature = "have_zlib", feature = "have_native_comp"))]
const MD5_BLOCKSIZE: usize = 32768;

/// C 72-73: `static char acc_buff[2 * MD5_BLOCKSIZE];` and `static ptrdiff_t acc_size;` - both
/// C tentative definitions, i.e. zeroed.
#[cfg (all (feature = "have_zlib", feature = "have_native_comp"))]
static mut acc_buff: [c_char; 2 * MD5_BLOCKSIZE] = [0; 2 * MD5_BLOCKSIZE];
#[cfg (all (feature = "have_zlib", feature = "have_native_comp"))]
static mut acc_size: ptrdiff_t = 0;

/// C 75-89: `accumulate_and_process_md5`.
#[cfg (all (feature = "have_zlib", feature = "have_native_comp"))]
unsafe fn accumulate_and_process_md5 (data: *mut c_void, len: ptrdiff_t, ctxt: *mut md5_ctx)
{
  eassert! (len <= MD5_BLOCKSIZE as ptrdiff_t);
  /* We may optimize this saving some of these memcpy/move using
     directly the outer buffers but so far don't bother.  */
  memcpy (acc_buff.as_mut_ptr ().offset (acc_size) as *mut c_void, data, len as usize);
  acc_size += len;
  if acc_size >= MD5_BLOCKSIZE as ptrdiff_t
    {
      acc_size -= MD5_BLOCKSIZE as ptrdiff_t;
      md5_process_block (acc_buff.as_ptr () as *const c_void, MD5_BLOCKSIZE, ctxt);
      memmove (acc_buff.as_mut_ptr () as *mut c_void,
               acc_buff.as_ptr ().offset (MD5_BLOCKSIZE as isize) as *const c_void,
               acc_size as usize);
    }
}

/// C 91-99: `final_process_md5`.
#[cfg (all (feature = "have_zlib", feature = "have_native_comp"))]
unsafe fn final_process_md5 (ctxt: *mut md5_ctx)
{
  if acc_size != 0
    {
      md5_process_bytes (acc_buff.as_ptr () as *const c_void, acc_size as usize, ctxt);
      acc_size = 0;
    }
}

/// C 101-165: `md5_gz_stream`, one of the file's two non-`static` definitions.  The two 32K
/// buffers and the `z_stream` are C locals without initializers, which the port zeroes
/// (PORTING.md 1.5/0.1-13; `docs/ports/decompress.md` names them).
#[cfg (all (feature = "have_zlib", feature = "have_native_comp"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_gz_stream (source: *mut FILE, resblock: *mut c_void) -> c_int
{
  let mut stream: z_stream = std::mem::zeroed ();
  let mut r#in: [c_uchar; MD5_BLOCKSIZE] = std::mem::zeroed ();
  let mut out: [c_uchar; MD5_BLOCKSIZE] = std::mem::zeroed ();

  #[cfg (feature = "windowsnt")]
  {
    if !zlib_initialized
      { zlib_initialized = init_zlib_functions (); }
    if !zlib_initialized
      {
        message1 (c"zlib library not found".as_ptr ());
        return -1;
      }
  }

  eassert! (acc_size == 0);

  let mut ctx: md5_ctx = std::mem::zeroed ();
  md5_init_ctx (&mut ctx);

  /* allocate inflate state */
  stream.zalloc = None;
  stream.zfree = None;
  stream.opaque = Z_NULL as voidpf;
  stream.avail_in = 0;
  stream.next_in = Z_NULL as *mut Bytef;
  let mut res: c_int = inflateInit2! (&mut stream, MAX_WBITS + 32);
  if res != Z_OK
    { return -1; }

  loop {
    stream.avail_in = fread (r#in.as_mut_ptr () as *mut c_void, 1, MD5_BLOCKSIZE, source) as uInt;
    if ferror (source) != 0 {
      #[cfg (not (feature = "windowsnt"))]
      { inflateEnd (&mut stream); }
      #[cfg (feature = "windowsnt")]
      { (fn_inflateEnd.unwrap ()) (&mut stream); }
      return -1;
    }
    if stream.avail_in == 0
      { break; }
    stream.next_in = r#in.as_mut_ptr ();

    loop {
      stream.avail_out = MD5_BLOCKSIZE as uInt;
      stream.next_out = out.as_mut_ptr ();
      #[cfg (not (feature = "windowsnt"))]
      { res = inflate (&mut stream, Z_NO_FLUSH); }
      #[cfg (feature = "windowsnt")]
      { res = (fn_inflate.unwrap ()) (&mut stream, Z_NO_FLUSH); }

      if res != Z_OK && res != Z_STREAM_END
	{ return -1; }

      accumulate_and_process_md5 (out.as_mut_ptr () as *mut c_void,
                                  MD5_BLOCKSIZE as ptrdiff_t - stream.avail_out as ptrdiff_t,
                                  &mut ctx);
      if ! (stream.avail_in != 0 && stream.avail_out == 0)
        { break; }
    }

    if res == Z_STREAM_END
      { break; }
  }

  final_process_md5 (&mut ctx);
  #[cfg (not (feature = "windowsnt"))]
  { inflateEnd (&mut stream); }
  #[cfg (feature = "windowsnt")]
  { (fn_inflateEnd.unwrap ()) (&mut stream); }

  if res != Z_STREAM_END
    { return -1; }

  md5_finish_ctx (&mut ctx, resblock);

  return 0;
}

/* ----------------------------------------------------------------------------------------------
   C 171-347: the rest of the file, under the file gate alone.  */

/// C 171-175: `struct decompress_unwind_data`.
#[cfg (feature = "have_zlib")]
#[repr(C)]
pub struct decompress_unwind_data
{
  pub old_point: ptrdiff_t,
  pub orig: ptrdiff_t,
  pub start: ptrdiff_t,
  pub nbytes: ptrdiff_t,
  pub stream: *mut z_stream,
}

/// C 177-200: `unwind_decompress` - C's `static void (void *)`, which is the shape
/// `record_unwind_protect_ptr` takes (PORTING.md 0.3).
#[cfg (feature = "have_zlib")]
unsafe extern "C" fn unwind_decompress (ddata: *mut c_void)
{
  let data: *mut decompress_unwind_data = ddata as *mut decompress_unwind_data;
  #[cfg (not (feature = "windowsnt"))]
  { inflateEnd ((*data).stream); }
  #[cfg (feature = "windowsnt")]
  { (fn_inflateEnd.unwrap ()) ((*data).stream); }

  /* Delete any uncompressed data already inserted on error, but
     without calling the change hooks.  */
  if (*data).start != 0
    {
      del_range_2 ((*data).start, (*data).start, /* byte, char offsets the same */
		   (*data).start + (*data).nbytes, (*data).start + (*data).nbytes,
		   false);
      update_compositions ((*data).start, (*data).start, CHECK_HEAD);
      /* "Balance" the before-change-functions call, which would
	 otherwise be left "hanging".  */
      signal_after_change ((*data).orig, (*data).start - (*data).orig,
			   (*data).start - (*data).orig);
    }
  /* Put point where it was, or if the buffer has shrunk because the
     compressed data is bigger than the uncompressed, at
     point-max.  */
  SET_PT (min ((*data).old_point, ZV! ()));
}

#[cfg (feature = "have_zlib")]
DEFUN! ("zlib-available-p", Fzlib_available_p, Szlib_available_p, 0, 0, 0,
       doc: "Return t if zlib decompression is available in this instance of Emacs.",
       (),
       {
  #[cfg (feature = "windowsnt")]
  {
    let mut found: Lisp_Object = Fassq (Qzlib (), Vlibrary_cache);
    if CONSP (found)
      { return XCDR (found); }
    else
      {
        let mut status: Lisp_Object;
        zlib_initialized = init_zlib_functions ();
        status = if zlib_initialized { Qt () } else { Qnil () };
        Vlibrary_cache = Fcons (Fcons (Qzlib (), status), Vlibrary_cache);
        return status;
      }
  }
  #[cfg (not (feature = "windowsnt"))]
  { return Qt (); }
});

#[cfg (feature = "have_zlib")]
DEFUN! ("zlib-decompress-region", Fzlib_decompress_region, Szlib_decompress_region, 2, 3, 0,
       doc: "Decompress a gzip- or zlib-compressed region.\nReplace the text in the region by the decompressed data.\n\nIf optional parameter ALLOW-PARTIAL is nil or omitted, then on\nfailure, return nil and leave the data in place.  Otherwise, return\nthe number of bytes that were not decompressed and replace the region\ntext by whatever data was successfully decompressed (similar to gzip).\nIf decompression is completely successful return t.\n\nThis function can be called only in unibyte buffers.",
       (start: Lisp_Object, end: Lisp_Object, allow_partial: Lisp_Object),
       {
  /* C declares `ptrdiff_t istart, iend, pos_byte;`, an uninitialized `z_stream`,
     `int inflate_status;` and an uninitialized `struct decompress_unwind_data`, and fills each
     before its first read; the port zeroes them first (PORTING.md 1.5/0.1-13).  */
  let mut istart: ptrdiff_t = 0;
  let mut iend: ptrdiff_t = 0;
  let mut pos_byte: ptrdiff_t = 0;
  let mut stream: z_stream = std::mem::zeroed ();
  let mut inflate_status: c_int = 0;
  let mut unwind_data: decompress_unwind_data = std::mem::zeroed ();
  let count: specpdl_ref = SPECPDL_INDEX ();

  let mut start = start;
  let mut end = end;

  validate_region (&raw mut start, &raw mut end);

  if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { error (c"This function can be called only in unibyte buffers".as_ptr ()); }

  #[cfg (feature = "windowsnt")]
  {
    if !zlib_initialized
      { zlib_initialized = init_zlib_functions (); }
    if !zlib_initialized
      {
        message1 (c"zlib library not found".as_ptr ());
        return Qnil ();
      }
  }

  /* This is a unibyte buffer, so character positions and bytes are
     the same.  */
  istart = XFIXNUM (start);
  iend = XFIXNUM (end);

  /* Do the following before manipulating the gap.  */
  modify_text (istart, iend);

  move_gap_both (iend, iend);

  stream.zalloc = None;
  stream.zfree = None;
  stream.opaque = Z_NULL as voidpf;
  stream.avail_in = 0;
  stream.next_in = Z_NULL as *mut Bytef;

  /* The magic number 32 apparently means "autodetect both the gzip and
     zlib formats" according to zlib.h.  */
  if inflateInit2! (&mut stream, MAX_WBITS + 32) != Z_OK
    { return Qnil (); }

  unwind_data.orig = istart;
  unwind_data.start = iend;
  unwind_data.stream = &raw mut stream;
  unwind_data.old_point = PT! ();
  unwind_data.nbytes = 0;
  record_unwind_protect_ptr (Some (unwind_decompress), &raw mut unwind_data as *mut c_void);

  /* Insert the decompressed data at the end of the compressed data.  */
  SET_PT (iend);

  pos_byte = istart;

  /* Keep calling 'inflate' until it reports an error or end-of-input.  */
  loop
    {
      /* Maximum number of bytes that one 'inflate' call should read and write.
	 Do not make avail_out too large, as that might unduly delay C-g.
	 zlib requires that avail_in and avail_out not exceed UINT_MAX.  */
      let avail_in: ptrdiff_t = min (iend - pos_byte, UINT_MAX as ptrdiff_t);
      let avail_out: c_int = 16 * 1024;
      let mut decompressed: c_int;

      if GAP_SIZE! () < avail_out as ptrdiff_t
	{ make_gap (avail_out as ptrdiff_t - GAP_SIZE! ()); }
      stream.next_in = BYTE_POS_ADDR (pos_byte);
      stream.avail_in = avail_in as uInt;
      stream.next_out = GPT_ADDR! ();
      stream.avail_out = avail_out as uInt;
      #[cfg (not (feature = "windowsnt"))]
      { inflate_status = inflate (&mut stream, Z_NO_FLUSH); }
      #[cfg (feature = "windowsnt")]
      { inflate_status = (fn_inflate.unwrap ()) (&mut stream, Z_NO_FLUSH); }
      pos_byte += avail_in - stream.avail_in as ptrdiff_t;
      decompressed = avail_out - stream.avail_out as c_int;
      insert_from_gap (decompressed as ptrdiff_t, decompressed as ptrdiff_t, false, false);
      unwind_data.nbytes += decompressed as ptrdiff_t;
      maybe_quit ();
      if ! (inflate_status == Z_OK)
        { break; }
    }

  let mut ret: Lisp_Object = Qt ();
  if inflate_status != Z_STREAM_END
    {
      if !NILP (allow_partial)
        { ret = make_int ((iend - pos_byte) as intmax_t); }
      else
        { return unbind_to (count, Qnil ()); }
    }

  unwind_data.start = 0;

  /* Delete the compressed data.  */
  del_range_2 (istart, istart, /* byte and char offsets are the same */
               iend, iend, false);

  signal_after_change (istart, iend - istart, unwind_data.nbytes);
  update_compositions (istart, istart, CHECK_HEAD);

  return unbind_to (count, ret);
});

/* Three declarations the frozen mirrors do not carry (P-72 rule 4: a mirror must reach what it
   needs through its own `include!`s).  C reaches them through `lisp.h`; until they land there the
   unit supplies them at the use site - the `coding.rs` 106-108 / `buffer.rs` 123-126 shape, which
   is where these same two functions are declared for other units.  `docs/ports/decompress.md`
   records the gap for the integrator.  */
#[cfg (feature = "have_zlib")]
unsafe extern "C" {

  /// C: lisp.h 5273: `extern void del_range_2 (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool);`
  pub fn del_range_2 (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t, to_byte: ptrdiff_t,
                      prepare: bool);
}

/* ----------------------------------------------------------------------------------------------
   C 338-347: `syms_of_decompress`.  */

/// C 342-347: the file's one remaining non-`static` definition.
#[cfg (feature = "have_zlib")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_decompress ()
{
  defsubr (&raw mut Szlib_decompress_region);
  defsubr (&raw mut Szlib_available_p);
}
