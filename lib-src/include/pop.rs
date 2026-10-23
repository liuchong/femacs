// femacs: mirror of lib-src/pop.h @ fd7afced403.
//
// The header is two size constants, two extern objects, the `_popserver` record with its two
// `bool_bf` bitfields, the `popserver` typedef, three flag macros and eighteen prototypes.  Its
// two consumers are lib-src/movemail.c (which links pop.o) and lib-src/pop.c itself, both of
// which the frozen configuration builds (`lib-src/Makefile`'s `UTILITIES` and the movemail
// object list), so this is a live mirror - it is simply not reachable from `src/include`, which
// is why `xtask layout-check` does not compile it (the lib-src units that include it do).
//
// `bool_bf` is the frozen `conf_post.h` 61's `typedef bool bool_bf;` (measured `sizeof` 1), so
// the two 1-bit members pack into one byte at offset 24; the mirror keeps the storage unit as
// `bits: u8` and names the members in accessors (PORTING.md 1.3/P-80, the coding.rs shape).

include!("../../lib/include/stdio.rs");   // C 20: `#include <stdio.h>`

/// C 23: `#define GETLINE_MIN 1024`
pub const GETLINE_MIN: c_int = 1024;
/// C 24: `#define GETLINE_INCR 1024`
pub const GETLINE_INCR: c_int = 1024;

pub mod pop_objs {
  use super::*;

  unsafe extern "C" {
    /* C 26: `extern char pop_error[];` - an array of *unknown* length in the header, so the
       mirror declares the zero-length stand-in the port uses for that shape (lisp.rs's
       `my_edata`/`my_endbss` precedent): lib-src/pop.c 133 defines it as
       `char pop_error[ERROR_MAX]` with `ERROR_MAX` 160 (pop.c 123), and the difference is what
       the `clashing_extern_declarations` allowance covers.  */
    pub static mut pop_error: [c_char; 0];
    /* C 27: `extern bool pop_debug;`  */
    pub static mut pop_debug: bool;
  }
}
use pop_objs::*;

/// C 29-37: `struct _popserver` - measured with clang against the frozen configuration:
/// `sizeof` 32, `file`@0, `data`@4, `buffer`@8, `buffer_size`@16, `buffer_index`@20, the two
/// bitfields at 24.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _popserver
{
  pub file: c_int,
  pub data: c_int,
  pub buffer: *mut c_char,
  pub buffer_size: c_int,
  pub buffer_index: c_int,
  pub bits: u8,             /* C 34-35: `bool_bf in_multi : 1; bool_bf trash_started : 1;` */
}

const _: () = core::assert! (std::mem::size_of::<_popserver> () == 32);
const _: () = core::assert! (std::mem::offset_of! (_popserver, file) == 0);
const _: () = core::assert! (std::mem::offset_of! (_popserver, data) == 4);
const _: () = core::assert! (std::mem::offset_of! (_popserver, buffer) == 8);
const _: () = core::assert! (std::mem::offset_of! (_popserver, buffer_size) == 16);
const _: () = core::assert! (std::mem::offset_of! (_popserver, buffer_index) == 20);
const _: () = core::assert! (std::mem::offset_of! (_popserver, bits) == 24);

/// The two `bool_bf` members of `_popserver`, read and written by name (PORTING.md 1.3/P-80:
/// C's bitfield access is a load, a mask and a store, which is what this expands to).
macro_rules! popserver_bits {
  (in_multi, $s:expr) => { (($s).bits & 1u8) != 0 };
  (in_multi, $s:expr, $v:expr) => { { let b = ($s).bits & !1u8; ($s).bits = b | (if $v { 1u8 } else { 0u8 }); } };
  (trash_started, $s:expr) => { (($s).bits & 2u8) != 0 };
  (trash_started, $s:expr, $v:expr) => { { let b = ($s).bits & !2u8; ($s).bits = b | (if $v { 2u8 } else { 0u8 }); } };
}

/// C 39: `typedef struct _popserver *popserver;`
pub type popserver = *mut _popserver;

/// C 45: `#define POP_NO_KERBEROS (1<<0)`
pub const POP_NO_KERBEROS: c_int = 1 << 0;
/// C 46: `#define POP_NO_HESIOD (1<<1)`
pub const POP_NO_HESIOD: c_int = 1 << 1;
/// C 47: `#define POP_NO_GETPASS (1<<2)`
pub const POP_NO_GETPASS: c_int = 1 << 2;

// C 49-73: the eighteen prototypes.  lib-src/pop.c defines every one of them, so they live in a
// glob-imported module the defining unit's own definitions shadow (PORTING.md 0.1-10/P-21).
pub mod pop_decls {
  use super::*;

  unsafe extern "C" {
    pub fn pop_open (host: *mut c_char, username: *mut c_char, password: *mut c_char,
                     flags: c_int) -> popserver;
    pub fn pop_stat (server: popserver, count: *mut c_int, size: *mut c_int) -> c_int;
    pub fn pop_list (server: popserver, message: c_int, IDs: *mut *mut c_int,
                     size: *mut *mut c_int) -> c_int;
    pub fn pop_retrieve (server: popserver, message: c_int, markfrom: c_int,
                         response: *mut *mut c_char) -> c_int;
    pub fn pop_retrieve_first (server: popserver, message: c_int,
                               response: *mut *mut c_char) -> c_int;
    pub fn pop_retrieve_next (server: popserver, line: *mut *mut c_char) -> c_int;
    pub fn pop_retrieve_flush (server: popserver) -> c_int;
    pub fn pop_top_first (server: popserver, message: c_int, lines: c_int,
                          response: *mut *mut c_char) -> c_int;
    pub fn pop_top_next (server: popserver, line: *mut *mut c_char) -> c_int;
    pub fn pop_top_flush (server: popserver) -> c_int;
    pub fn pop_multi_first (server: popserver, command: *const c_char,
                            response: *mut *mut c_char) -> c_int;
    pub fn pop_multi_next (server: popserver, line: *mut *mut c_char) -> c_int;
    pub fn pop_multi_flush (server: popserver) -> c_int;
    pub fn pop_delete (server: popserver, message: c_int) -> c_int;
    pub fn pop_noop (server: popserver) -> c_int;
    pub fn pop_last (server: popserver) -> c_int;
    pub fn pop_reset (server: popserver) -> c_int;
    pub fn pop_quit (server: popserver) -> c_int;
    pub fn pop_close (server: popserver);
  }
}
use pop_decls::*;
