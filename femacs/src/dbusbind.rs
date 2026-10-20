// femacs: port of src/dbusbind.c @ fd7afced403.
//
// One-to-one port (PORTING.md P-86 ①): the reference is transcribed in C's order, both arms of
// every `#if` are carried, and each arm's gate is the lower-case feature of the macro the
// reference tests.  The **file gate is `feature = "have_dbus"`** (C 21's `#ifdef HAVE_DBUS`;
// `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1259), so the frozen configuration
// compiles nothing from this file beyond C 19's `<config.h>` - and the reference compiles the same
// nothing (`docs/baseline/frozen-build-sources.txt` has no `dbusbind.o`; P-86 ⑧ measures the empty
// object at 336 bytes).
//
// The sub-gates.  None of them is decided by the frozen table (every `HAVE_DBUS_*` macro it carries
// is `undef`, macros.tsv 1260-1266, and the whole file is inside the false file gate anyway), so
// each was decided by hand and is recorded in `docs/ports/dbusbind.md`:
//
//   * C 104/2173's `#ifdef DBUS_DEBUG`                                  -> `dbus_debug`
//   * C 138's `#ifdef HAVE_DBUS_TYPE_IS_VALID`                          -> `have_dbus_type_is_valid`
//   * C 142/195/222/421/707/929/1577/1855/2044/2061's `#ifdef DBUS_TYPE_UNIX_FD`
//                                                                       -> `dbus_type_unix_fd`
//   * C 321/338/346/354/362's `HAVE_DBUS_VALIDATE_*`                    -> one feature per name
//   * C 1051's `HAVE_DBUS_WATCH_GET_UNIX_FD`                            -> `have_dbus_watch_get_unix_fd`
//   * C 1553's `HAVE_DBUS_MESSAGE_SET_ALLOW_INTERACTIVE_AUTHORIZATION`
//                     -> `have_dbus_message_set_allow_interactive_authorization`
//   * C 2083/2093's `DBUS_VERSION_STRING` / `DBUS_VERSION`              -> `dbus_version_string` /
//                                                                          `dbus_version`
//   * C 32's `#ifndef O_RDONLY` and C 37's `#ifndef DBUS_NUM_MESSAGE_TYPES` -> `o_rdonly` /
//     `dbus_num_message_types`, the `comp.rs` shape: the feature names the macro's *being defined*,
//     so the reference's arm is the `not (…)` side.
//
// `have_dbus` is the tree's (`docs/baseline/feature-coverage.tsv` 138, first used by
// `keyboard.rs` 4768); **every other name above is new** and is Owed 1 in the ledger.
//
// `<dbus/dbus.h>` has no file in the reference tree and no copy on this machine (no dbus directory
// under `/opt/homebrew/include`, `/usr/include`, the Xcode SDK or the cargo registry - Evidence
// ②), so its surface is declared **in this unit** at C 24's `#include` line (P-88 ①) and every item
// is **UNVERIFIED**.  What *is* measured is the Rust side's own side of the contract: the two
// records' sizes/offsets and the `PRIdMAX`/`PRIuMAX` spellings come from a `clang` restatement of
// the declarations below (Evidence ③), which proves the `#[repr (C)]` records match the shape
// stated here and nothing about the real header.  Nothing links `libdbus`.
//
// `docs/ports/dbusbind.md` carries the C line ranges (C 1-2190, the reference's every line), the
// acceptance readings, every deviation and the items the integrator owes.  No behaviour of this
// file was executed anywhere: no bus connection was opened, no message sent or received, and no
// Lisp call made.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 19: `#include <config.h>`

/* C 21: `#ifdef HAVE_DBUS` - the file gate.  C 22 through C 2189 sit inside it, and every
   `include!` and item below carries it (P-86 ⑧: the gate goes on the include graph too).  */

/* C 22-24: `<stdio.h>`, `<stdlib.h>` and `<dbus/dbus.h>`.  The first two are the target's own
   headers; the third has no mirror and the section below is its stand-in.  */
#[cfg (feature = "have_dbus")]
include!("../lib/include/stdio.rs");           // C 22: `#include <stdio.h>`
#[cfg (feature = "have_dbus")]
include!("../lib/include/stdlib.rs");          // C 23: `#include <stdlib.h>`

/* ---------------------------------------------------------------------------------------------
   C 24: `#include <dbus/dbus.h>` - the header this section stands for (P-88 ①).

   **The header is not on this machine** and the reference tree has no copy (the four `find`/`ls`
   readings are in `docs/ports/dbusbind.md` `## Evidence` ②), so every declaration below is
   written from the library's published interface and is **UNVERIFIED**; no value or layout here is
   a measurement of the real header.  What the ledger's Evidence ③ *does* measure is this text's
   own consistency: a `clang` restatement of `DBusError`, `DBusMessageIter` and the scalars, which
   is what `xtask`'s `## C line ranges` row for C 24 points at.  Nothing here is a stub: each item
   is named by the C statement that uses it.
   --------------------------------------------------------------------------------------------- */

/* `dbus/dbus-types.h`'s fixed-width scalars.  The header picks `long long` for the 64-bit pair on
   a target with 64-bit `long long` (`DBUS_HAVE_INT64`), which is this one.  */
/// C: dbus/dbus-types.h: `typedef int dbus_bool_t;`
#[cfg (feature = "have_dbus")]
pub type dbus_bool_t = c_int;
/// C: dbus/dbus-types.h: `typedef short dbus_int16_t;` / `unsigned short dbus_uint16_t;`
#[cfg (feature = "have_dbus")]
pub type dbus_int16_t = c_short;
#[cfg (feature = "have_dbus")]
pub type dbus_uint16_t = c_ushort;
/// C: dbus/dbus-types.h: `typedef int dbus_int32_t;` / `unsigned int dbus_uint32_t;`
#[cfg (feature = "have_dbus")]
pub type dbus_int32_t = c_int;
#[cfg (feature = "have_dbus")]
pub type dbus_uint32_t = c_uint;
/// C: dbus/dbus-types.h: `typedef long long dbus_int64_t;` / `unsigned long long
/// dbus_uint64_t;` (`DBUS_HAVE_INT64`).
#[cfg (feature = "have_dbus")]
pub type dbus_int64_t = core::ffi::c_longlong;
#[cfg (feature = "have_dbus")]
pub type dbus_uint64_t = core::ffi::c_ulonglong;

/* The opaque handles.  C only ever holds pointers to them here.  */
/// C: dbus/dbus-connection.h: `typedef struct DBusConnection DBusConnection;`
#[cfg (feature = "have_dbus")]
#[repr (C)]
pub struct DBusConnection { _private: [u8; 0] }
/// C: dbus/dbus-message.h: `typedef struct DBusMessage DBusMessage;`
#[cfg (feature = "have_dbus")]
#[repr (C)]
pub struct DBusMessage { _private: [u8; 0] }
/// C: dbus/dbus-connection.h: `typedef struct DBusPendingCall DBusPendingCall;` - C 1613 passes
/// `NULL` for it.
#[cfg (feature = "have_dbus")]
#[repr (C)]
pub struct DBusPendingCall { _private: [u8; 0] }
/// C: dbus/dbus-watch.h: `typedef struct DBusWatch DBusWatch;`
#[cfg (feature = "have_dbus")]
#[repr (C)]
pub struct DBusWatch { _private: [u8; 0] }
/// C: dbus/dbus-address.h: `typedef struct DBusAddressEntry DBusAddressEntry;` - C 292's
/// `dbus_parse_address` fills a `DBusAddressEntry **`.
#[cfg (feature = "have_dbus")]
#[repr (C)]
pub struct DBusAddressEntry { _private: [u8; 0] }

/// C: dbus/dbus-message.h: `typedef struct DBusMessageIter { void *dummy1; void *dummy2;
/// dbus_uint32_t dummy3; int dummy4; … int dummy11; int pad1; int pad2; void *pad3; }
/// DBusMessageIter;` - the iterator C 638's `xd_append_arg` and 875's `xd_retrieve_arg` take by
/// pointer.  Measured on the restatement: `sizeof == 72`, `alignof == 8`, `dummy1` at 0,
/// `dummy2` at 8, `dummy3` at 16, `dummy4` at 20, `dummy11` at 48, `pad1` at 52, `pad2` at 56 and
/// `pad3` at 64 (Evidence ③).  UNVERIFIED against the real header.
#[cfg (feature = "have_dbus")]
#[repr (C)]
#[derive(Clone, Copy)]
pub struct DBusMessageIter
{
  pub dummy1: *mut c_void,
  pub dummy2: *mut c_void,
  pub dummy3: dbus_uint32_t,
  pub dummy4: c_int,
  pub dummy5: c_int,
  pub dummy6: c_int,
  pub dummy7: c_int,
  pub dummy8: c_int,
  pub dummy9: c_int,
  pub dummy10: c_int,
  pub dummy11: c_int,
  pub pad1: c_int,
  pub pad2: c_int,
  pub pad3: *mut c_void,
}

/// C: dbus/dbus-errors.h: `typedef struct DBusError { const char *name; const char *message;
/// unsigned int dummy1 : 1; … unsigned int dummy5 : 1; void *padding1; } DBusError;` - C 92's
/// `XD_ERROR` reads `.message`, C 284/323's validate macros read nothing.  Measured on the
/// restatement: `sizeof == 32`, `alignof == 8`, `name` at 0, `message` at 8, the five 1-bit
/// fields share the 4-byte unit at 16, `padding1` at 24 (Evidence ③).  The five bits are one
/// storage unit (PORTING.md 1.3) - C reads none of them.  UNVERIFIED.
#[cfg (feature = "have_dbus")]
#[repr (C)]
#[derive(Clone, Copy)]
pub struct DBusError
{
  pub name: *const c_char,
  pub message: *const c_char,
  pub bits: c_uint,                    // dummy1:1 | dummy2:1 | dummy3:1 | dummy4:1 | dummy5:1
  pub padding1: *mut c_void,
}

/// C: dbus/dbus-shared.h: `typedef enum { DBUS_BUS_SESSION, DBUS_BUS_SYSTEM, DBUS_BUS_STARTER }
/// DBusBusType;` - C 1257 picks between the first two.
#[cfg (feature = "have_dbus")]
pub type DBusBusType = c_int;
/// C: dbus/dbus-connection.h: `typedef enum { DBUS_DISPATCH_DATA_REMAINS,
/// DBUS_DISPATCH_COMPLETE, DBUS_DISPATCH_NEED_MEMORY } DBusDispatchStatus;`
#[cfg (feature = "have_dbus")]
pub type DBusDispatchStatus = c_int;

/* The three watch callbacks and the free function C 1160's `dbus_connection_set_watch_functions`
/// takes (dbus/dbus-connection.h's `DBusAddWatchFunction`, `DBusRemoveWatchFunction`,
`DBusWatchToggledFunction`, `DBusFreeFunction`).  Each parameter is `Option<unsafe extern "C"
fn>`: C's `NULL` free function (C 1603) has no other Rust spelling (the `src/process.rs`
`fd_callback` shape).  */
/// C: dbus/dbus-connection.h: `typedef dbus_bool_t (*DBusAddWatchFunction) (DBusWatch *watch,
/// void *data);`
#[cfg (feature = "have_dbus")]
pub type DBusAddWatchFunction = Option<unsafe extern "C" fn (*mut DBusWatch, *mut c_void)
                                                                  -> dbus_bool_t>;
/// C: dbus/dbus-connection.h: `typedef void (*DBusRemoveWatchFunction) (DBusWatch *watch,
/// void *data);`
#[cfg (feature = "have_dbus")]
pub type DBusRemoveWatchFunction = Option<unsafe extern "C" fn (*mut DBusWatch, *mut c_void)>;
/// C: dbus/dbus-connection.h: `typedef void (*DBusWatchToggledFunction) (DBusWatch *watch,
/// void *data);`
#[cfg (feature = "have_dbus")]
pub type DBusWatchToggledFunction = Option<unsafe extern "C" fn (*mut DBusWatch, *mut c_void)>;
/// C: dbus/dbus-memory.h: `typedef void (*DBusFreeFunction) (void *memory);`
#[cfg (feature = "have_dbus")]
pub type DBusFreeFunction = Option<unsafe extern "C" fn (*mut c_void)>;

/* `dbus/dbus-protocol.h`'s type codes.  C spells each one as the ASCII code of the signature
   character it stands for (C 418's `sprintf (signature, "%c", dtype)` and C 439's
   `signature[0] = dtype` print them).  */
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_INVALID ((int) '\0')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_INVALID: c_int = 0;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_BYTE ((int) 'y')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_BYTE: c_int = b'y' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_BOOLEAN ((int) 'b')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_BOOLEAN: c_int = b'b' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_INT16 ((int) 'n')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_INT16: c_int = b'n' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_UINT16 ((int) 'q')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_UINT16: c_int = b'q' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_INT32 ((int) 'i')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_INT32: c_int = b'i' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_UINT32 ((int) 'u')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_UINT32: c_int = b'u' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_INT64 ((int) 'x')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_INT64: c_int = b'x' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_UINT64 ((int) 't')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_UINT64: c_int = b't' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_DOUBLE ((int) 'd')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_DOUBLE: c_int = b'd' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_STRING ((int) 's')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_STRING: c_int = b's' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_OBJECT_PATH ((int) 'o')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_OBJECT_PATH: c_int = b'o' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_SIGNATURE ((int) 'g')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_SIGNATURE: c_int = b'g' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_UNIX_FD ((int) 'h')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_UNIX_FD: c_int = b'h' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_ARRAY ((int) 'a')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_ARRAY: c_int = b'a' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_VARIANT ((int) 'v')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_VARIANT: c_int = b'v' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_STRUCT ((int) 'r')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_STRUCT: c_int = b'r' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_DICT_ENTRY ((int) 'e')`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_DICT_ENTRY: c_int = b'e' as c_int;

/// C: dbus/dbus-protocol.h: `#define DBUS_TYPE_STRING_AS_STRING "s"`
#[cfg (feature = "have_dbus")]
pub const DBUS_TYPE_STRING_AS_STRING: *const c_char = c"s".as_ptr ();
/// C: dbus/dbus-protocol.h: `#define DBUS_STRUCT_BEGIN_CHAR_AS_STRING "("`
#[cfg (feature = "have_dbus")]
pub const DBUS_STRUCT_BEGIN_CHAR_AS_STRING: *const c_char = c"(".as_ptr ();
/// C: dbus/dbus-protocol.h: `#define DBUS_STRUCT_END_CHAR_AS_STRING ")"`
#[cfg (feature = "have_dbus")]
pub const DBUS_STRUCT_END_CHAR_AS_STRING: *const c_char = c")".as_ptr ();
/// C: dbus/dbus-protocol.h: `#define DBUS_DICT_ENTRY_BEGIN_CHAR_AS_STRING "{"`
#[cfg (feature = "have_dbus")]
pub const DBUS_DICT_ENTRY_BEGIN_CHAR_AS_STRING: *const c_char = c"{".as_ptr ();
/// C: dbus/dbus-protocol.h: `#define DBUS_DICT_ENTRY_END_CHAR_AS_STRING "}"`
#[cfg (feature = "have_dbus")]
pub const DBUS_DICT_ENTRY_END_CHAR_AS_STRING: *const c_char = c"}".as_ptr ();

/// C: dbus/dbus-protocol.h: `#define DBUS_STRUCT_BEGIN_CHAR ((int) '(')`
#[cfg (feature = "have_dbus")]
pub const DBUS_STRUCT_BEGIN_CHAR: c_int = b'(' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_STRUCT_END_CHAR ((int) ')')`
#[cfg (feature = "have_dbus")]
pub const DBUS_STRUCT_END_CHAR: c_int = b')' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_DICT_ENTRY_BEGIN_CHAR ((int) '{')`
#[cfg (feature = "have_dbus")]
pub const DBUS_DICT_ENTRY_BEGIN_CHAR: c_int = b'{' as c_int;
/// C: dbus/dbus-protocol.h: `#define DBUS_DICT_ENTRY_END_CHAR ((int) '}')`
#[cfg (feature = "have_dbus")]
pub const DBUS_DICT_ENTRY_END_CHAR: c_int = b'}' as c_int;

/// C: dbus/dbus-protocol.h: `#define DBUS_MAXIMUM_SIGNATURE_LENGTH 255` - C 375's
/// `xd_signature_cat` is the one user.
#[cfg (feature = "have_dbus")]
pub const DBUS_MAXIMUM_SIGNATURE_LENGTH: usize = 255;

/// C: dbus/dbus-message.h: `#define DBUS_MESSAGE_TYPE_INVALID 0`
#[cfg (feature = "have_dbus")]
pub const DBUS_MESSAGE_TYPE_INVALID: c_int = 0;
/// C: dbus/dbus-message.h: `#define DBUS_MESSAGE_TYPE_METHOD_CALL 1`
#[cfg (feature = "have_dbus")]
pub const DBUS_MESSAGE_TYPE_METHOD_CALL: c_int = 1;
/// C: dbus/dbus-message.h: `#define DBUS_MESSAGE_TYPE_METHOD_RETURN 2`
#[cfg (feature = "have_dbus")]
pub const DBUS_MESSAGE_TYPE_METHOD_RETURN: c_int = 2;
/// C: dbus/dbus-message.h: `#define DBUS_MESSAGE_TYPE_ERROR 3`
#[cfg (feature = "have_dbus")]
pub const DBUS_MESSAGE_TYPE_ERROR: c_int = 3;
/// C: dbus/dbus-message.h: `#define DBUS_MESSAGE_TYPE_SIGNAL 4`
#[cfg (feature = "have_dbus")]
pub const DBUS_MESSAGE_TYPE_SIGNAL: c_int = 4;

/// C: dbus/dbus-connection.h: `#define DBUS_WATCH_READABLE (1 << 0)`
#[cfg (feature = "have_dbus")]
pub const DBUS_WATCH_READABLE: c_uint = 1 << 0;
/// C: dbus/dbus-connection.h: `#define DBUS_WATCH_WRITABLE (1 << 1)`
#[cfg (feature = "have_dbus")]
pub const DBUS_WATCH_WRITABLE: c_uint = 1 << 1;

/// C: dbus/dbus-connection.h: `#define DBUS_DISPATCH_DATA_REMAINS 0`
#[cfg (feature = "have_dbus")]
pub const DBUS_DISPATCH_DATA_REMAINS: c_int = 0;
/// C: dbus/dbus-connection.h: `#define DBUS_DISPATCH_COMPLETE 1`
#[cfg (feature = "have_dbus")]
pub const DBUS_DISPATCH_COMPLETE: c_int = 1;

/// C: dbus/dbus-shared.h: `DBUS_BUS_SESSION` (the first enumerator of `DBusBusType`)
#[cfg (feature = "have_dbus")]
pub const DBUS_BUS_SESSION: DBusBusType = 0;
/// C: dbus/dbus-shared.h: `DBUS_BUS_SYSTEM` (the second enumerator)
#[cfg (feature = "have_dbus")]
pub const DBUS_BUS_SYSTEM: DBusBusType = 1;

/// C: dbus/dbus-macros.h: `#define FALSE 0` (the library's own `dbus_bool_t` pair).
#[cfg (feature = "have_dbus")]
pub const FALSE: dbus_bool_t = 0;
/// C: dbus/dbus-macros.h: `#define TRUE 1`
#[cfg (feature = "have_dbus")]
pub const TRUE: dbus_bool_t = 1;
/// C: dbus/dbus-macros.h: `#define DBUS_VERSION_STRING "1.16.0"` - the library's own released
/// version string, the value C 2084 hands `build_string`.  **UNVERIFIED**: the header that would
/// carry the target's value is not on this machine, and the arm that reads it is compiled by no
/// configuration here (Evidence ②); a machine with libdbus measures its own value.
#[cfg (all (feature = "have_dbus", feature = "dbus_version_string"))]
pub const DBUS_VERSION_STRING: *const c_char = c"1.16.0".as_ptr ();
/// C: dbus/dbus-macros.h: `#define DBUS_VERSION (…)` - C 2093 tests *whether* the macro is
/// defined and never its value, so the port's stand-in records `0` and says the encoding (the
/// library packs major/minor/micro into an integer) is **UNVERIFIED**.
#[cfg (all (feature = "have_dbus", feature = "dbus_version"))]
pub const DBUS_VERSION: c_int = 0;

/* The entry points.  Each is named by the C statement that calls it, in C's order of first use.  */
#[cfg (feature = "have_dbus")]
unsafe extern "C" {
  /// C: dbus/dbus-errors.h: `void dbus_error_init (DBusError *error);`
  pub fn dbus_error_init (error: *mut DBusError);
  /// C: dbus/dbus-errors.h: `void dbus_error_free (DBusError *error);`
  pub fn dbus_error_free (error: *mut DBusError);
  /// C: dbus/dbus-errors.h: `dbus_bool_t dbus_error_is_set (const DBusError *error);`
  pub fn dbus_error_is_set (error: *const DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-address.h: `dbus_bool_t dbus_parse_address (const char *address,
  /// DBusAddressEntry ***entry, int *array_len, DBusError *error);`
  pub fn dbus_parse_address (address: *const c_char, entry: *mut *mut *mut DBusAddressEntry,
                             array_len: *mut c_int, error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-address.h: `void dbus_address_entries_free (DBusAddressEntry **entries);`
  pub fn dbus_address_entries_free (entries: *mut *mut DBusAddressEntry);
  /// C: dbus/dbus-signature.h: `dbus_bool_t dbus_type_is_valid (int ty);`
  pub fn dbus_type_is_valid (ty: c_int) -> dbus_bool_t;
  /// C: dbus/dbus-signature.h: `dbus_bool_t dbus_type_is_basic (int ty);`
  pub fn dbus_type_is_basic (ty: c_int) -> dbus_bool_t;
  /// C: dbus/dbus-syntax.h: `dbus_bool_t dbus_validate_bus_name (const char *name,
  /// DBusError *error);`
  pub fn dbus_validate_bus_name (name: *const c_char, error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-syntax.h: `dbus_bool_t dbus_validate_path (const char *path,
  /// DBusError *error);`
  pub fn dbus_validate_path (path: *const c_char, error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-syntax.h: `dbus_bool_t dbus_validate_interface (const char *name,
  /// DBusError *error);`
  pub fn dbus_validate_interface (name: *const c_char, error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-syntax.h: `dbus_bool_t dbus_validate_member (const char *name,
  /// DBusError *error);`
  pub fn dbus_validate_member (name: *const c_char, error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_iter_append_basic (DBusMessageIter *iter,
  /// int type, const void *value);`
  pub fn dbus_message_iter_append_basic (iter: *mut DBusMessageIter, type_: c_int,
                                         value: *const c_void) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_iter_open_container (DBusMessageIter *iter,
  /// int type, const char *contained_signature, DBusMessageIter *sub);`
  pub fn dbus_message_iter_open_container (iter: *mut DBusMessageIter, type_: c_int,
                                          contained_signature: *const c_char,
                                          sub: *mut DBusMessageIter) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_iter_close_container (DBusMessageIter *iter,
  /// DBusMessageIter *sub);`
  pub fn dbus_message_iter_close_container (iter: *mut DBusMessageIter, sub: *mut DBusMessageIter)
                                            -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `void dbus_message_iter_get_basic (DBusMessageIter *iter,
  /// void *value);`
  pub fn dbus_message_iter_get_basic (iter: *mut DBusMessageIter, value: *mut c_void);
  /// C: dbus/dbus-message.h: `int dbus_message_iter_get_arg_type (DBusMessageIter *iter);`
  pub fn dbus_message_iter_get_arg_type (iter: *mut DBusMessageIter) -> c_int;
  /// C: dbus/dbus-message.h: `void dbus_message_iter_recurse (DBusMessageIter *iter,
  /// DBusMessageIter *sub);`
  pub fn dbus_message_iter_recurse (iter: *mut DBusMessageIter, sub: *mut DBusMessageIter);
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_iter_next (DBusMessageIter *iter);`
  pub fn dbus_message_iter_next (iter: *mut DBusMessageIter) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_iter_init (DBusMessage *message,
  /// DBusMessageIter *iter);`
  pub fn dbus_message_iter_init (message: *mut DBusMessage, iter: *mut DBusMessageIter)
                                 -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `void dbus_message_iter_init_append (DBusMessage *message,
  /// DBusMessageIter *iter);`
  pub fn dbus_message_iter_init_append (message: *mut DBusMessage, iter: *mut DBusMessageIter);
  /// C: dbus/dbus-connection.h: `dbus_bool_t dbus_connection_get_is_connected
  /// (DBusConnection *connection);`
  pub fn dbus_connection_get_is_connected (connection: *mut DBusConnection) -> dbus_bool_t;
  /// C: dbus/dbus-connection.h: `void dbus_connection_close (DBusConnection *connection);`
  pub fn dbus_connection_close (connection: *mut DBusConnection);
  /// C: dbus/dbus-connection.h: `void dbus_connection_unref (DBusConnection *connection);`
  pub fn dbus_connection_unref (connection: *mut DBusConnection);
  /// C: dbus/dbus-connection.h: `DBusConnection *dbus_connection_ref (DBusConnection *connection);`
  pub fn dbus_connection_ref (connection: *mut DBusConnection) -> *mut DBusConnection;
  /// C: dbus/dbus-connection.h: `DBusConnection *dbus_connection_open (const char *address,
  /// DBusError *error);`
  pub fn dbus_connection_open (address: *const c_char, error: *mut DBusError) -> *mut DBusConnection;
  /// C: dbus/dbus-connection.h: `DBusConnection *dbus_connection_open_private (const char *address,
  /// DBusError *error);`
  pub fn dbus_connection_open_private (address: *const c_char, error: *mut DBusError)
                                       -> *mut DBusConnection;
  /// C: dbus/dbus-bus.h: `DBusConnection *dbus_bus_get (DBusBusType type, DBusError *error);`
  pub fn dbus_bus_get (type_: DBusBusType, error: *mut DBusError) -> *mut DBusConnection;
  /// C: dbus/dbus-bus.h: `DBusConnection *dbus_bus_get_private (DBusBusType type,
  /// DBusError *error);`
  pub fn dbus_bus_get_private (type_: DBusBusType, error: *mut DBusError) -> *mut DBusConnection;
  /// C: dbus/dbus-bus.h: `dbus_bool_t dbus_bus_register (DBusConnection *connection,
  /// DBusError *error);`
  pub fn dbus_bus_register (connection: *mut DBusConnection, error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-connection.h: `void dbus_connection_set_exit_on_disconnect
  /// (DBusConnection *connection, dbus_bool_t exit_on_disconnect);`
  pub fn dbus_connection_set_exit_on_disconnect (connection: *mut DBusConnection,
                                                  exit_on_disconnect: dbus_bool_t);
  /// C: dbus/dbus-connection.h: `dbus_bool_t dbus_connection_set_watch_functions
  /// (DBusConnection *connection, DBusAddWatchFunction add_function, DBusRemoveWatchFunction
  /// remove_function, DBusWatchToggledFunction toggled_function, void *data,
  /// DBusFreeFunction free_data_function);`
  pub fn dbus_connection_set_watch_functions (connection: *mut DBusConnection,
                                              add_function: DBusAddWatchFunction,
                                              remove_function: DBusRemoveWatchFunction,
                                              toggled_function: DBusWatchToggledFunction,
                                              data: *mut c_void,
                                              free_data_function: DBusFreeFunction) -> dbus_bool_t;
  /// C: dbus/dbus-bus.h: `const char *dbus_bus_get_unique_name (DBusConnection *connection);`
  pub fn dbus_bus_get_unique_name (connection: *mut DBusConnection) -> *const c_char;
  /// C: dbus/dbus-bus.h: `dbus_bool_t dbus_bus_name_has_owner (DBusConnection *connection,
  /// const char *name, DBusError *error);`
  pub fn dbus_bus_name_has_owner (connection: *mut DBusConnection, name: *const c_char,
                                  error: *mut DBusError) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `DBusMessage *dbus_message_new (int message_type);`
  pub fn dbus_message_new (message_type: c_int) -> *mut DBusMessage;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_set_destination (DBusMessage *message,
  /// const char *destination);`
  pub fn dbus_message_set_destination (message: *mut DBusMessage, destination: *const c_char)
                                       -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_set_path (DBusMessage *message,
  /// const char *object_path);`
  pub fn dbus_message_set_path (message: *mut DBusMessage, object_path: *const c_char)
                                -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_set_interface (DBusMessage *message,
  /// const char *interface);`
  pub fn dbus_message_set_interface (message: *mut DBusMessage, interface: *const c_char)
                                     -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_set_member (DBusMessage *message,
  /// const char *member);`
  pub fn dbus_message_set_member (message: *mut DBusMessage, member: *const c_char) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_set_reply_serial (DBusMessage *message,
  /// dbus_uint32_t reply_serial);`
  pub fn dbus_message_set_reply_serial (message: *mut DBusMessage, reply_serial: dbus_uint32_t)
                                        -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_bool_t dbus_message_set_error_name (DBusMessage *message,
  /// const char *error_name);`
  pub fn dbus_message_set_error_name (message: *mut DBusMessage, error_name: *const c_char)
                                      -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `void dbus_message_set_allow_interactive_authorization
  /// (DBusMessage *message, dbus_bool_t allow);`
  pub fn dbus_message_set_allow_interactive_authorization (message: *mut DBusMessage,
                                                           allow: dbus_bool_t);
  /// C: dbus/dbus-connection.h: `dbus_bool_t dbus_connection_send_with_reply
  /// (DBusConnection *connection, DBusMessage *message, DBusPendingCall **pending_return,
  /// int timeout_milliseconds);`
  pub fn dbus_connection_send_with_reply (connection: *mut DBusConnection,
                                          message: *mut DBusMessage,
                                          pending_return: *mut *mut DBusPendingCall,
                                          timeout_milliseconds: c_int) -> dbus_bool_t;
  /// C: dbus/dbus-connection.h: `dbus_bool_t dbus_connection_send (DBusConnection *connection,
  /// DBusMessage *message, dbus_uint32_t *client_serial);`
  pub fn dbus_connection_send (connection: *mut DBusConnection, message: *mut DBusMessage,
                               client_serial: *mut dbus_uint32_t) -> dbus_bool_t;
  /// C: dbus/dbus-message.h: `dbus_uint32_t dbus_message_get_serial (DBusMessage *message);`
  pub fn dbus_message_get_serial (message: *mut DBusMessage) -> dbus_uint32_t;
  /// C: dbus/dbus-message.h: `void dbus_message_unref (DBusMessage *message);`
  pub fn dbus_message_unref (message: *mut DBusMessage);
  /// C: dbus/dbus-connection.h: `DBusMessage *dbus_connection_pop_message
  /// (DBusConnection *connection);`
  pub fn dbus_connection_pop_message (connection: *mut DBusConnection) -> *mut DBusMessage;
  /// C: dbus/dbus-message.h: `int dbus_message_get_type (DBusMessage *message);`
  pub fn dbus_message_get_type (message: *mut DBusMessage) -> c_int;
  /// C: dbus/dbus-message.h: `dbus_uint32_t dbus_message_get_reply_serial (DBusMessage
  /// *message);`
  pub fn dbus_message_get_reply_serial (message: *mut DBusMessage) -> dbus_uint32_t;
  /// C: dbus/dbus-message.h: `const char *dbus_message_get_sender (DBusMessage *message);`
  pub fn dbus_message_get_sender (message: *mut DBusMessage) -> *const c_char;
  /// C: dbus/dbus-message.h: `const char *dbus_message_get_destination (DBusMessage *message);`
  pub fn dbus_message_get_destination (message: *mut DBusMessage) -> *const c_char;
  /// C: dbus/dbus-message.h: `const char *dbus_message_get_path (DBusMessage *message);`
  pub fn dbus_message_get_path (message: *mut DBusMessage) -> *const c_char;
  /// C: dbus/dbus-message.h: `const char *dbus_message_get_interface (DBusMessage *message);`
  pub fn dbus_message_get_interface (message: *mut DBusMessage) -> *const c_char;
  /// C: dbus/dbus-message.h: `const char *dbus_message_get_member (DBusMessage *message);`
  pub fn dbus_message_get_member (message: *mut DBusMessage) -> *const c_char;
  /// C: dbus/dbus-message.h: `const char *dbus_message_get_error_name (DBusMessage *message);`
  pub fn dbus_message_get_error_name (message: *mut DBusMessage) -> *const c_char;
  /// C: dbus/dbus-connection.h: `dbus_bool_t dbus_connection_read_write
  /// (DBusConnection *connection, int timeout_milliseconds);`
  pub fn dbus_connection_read_write (connection: *mut DBusConnection, timeout_milliseconds: c_int)
                                     -> dbus_bool_t;
  /// C: dbus/dbus-connection.h: `DBusDispatchStatus dbus_connection_get_dispatch_status
  /// (DBusConnection *connection);`
  pub fn dbus_connection_get_dispatch_status (connection: *mut DBusConnection)
                                              -> DBusDispatchStatus;
  /// C: dbus/dbus-connection.h: `unsigned int dbus_watch_get_flags (DBusWatch *watch);`
  pub fn dbus_watch_get_flags (watch: *mut DBusWatch) -> c_uint;
  /// C: dbus/dbus-connection.h: `dbus_bool_t dbus_watch_get_enabled (DBusWatch *watch);`
  pub fn dbus_watch_get_enabled (watch: *mut DBusWatch) -> dbus_bool_t;
  /// C: dbus/dbus-connection.h: `int dbus_watch_get_unix_fd (DBusWatch *watch);`
  pub fn dbus_watch_get_unix_fd (watch: *mut DBusWatch) -> c_int;
  /// C: dbus/dbus-connection.h: `int dbus_watch_get_socket (DBusWatch *watch);`
  pub fn dbus_watch_get_socket (watch: *mut DBusWatch) -> c_int;
  /// C: dbus/dbus-connection.h: `int dbus_watch_get_fd (DBusWatch *watch);`
  pub fn dbus_watch_get_fd (watch: *mut DBusWatch) -> c_int;
  /// C: dbus/dbus-misc.h: `void dbus_get_version (int *major_version, int *minor_version,
  /// int *micro_version);`
  pub fn dbus_get_version (major_version: *mut c_int, minor_version: *mut c_int,
                           micro_version: *mut c_int);
}

/* Five names the frozen mirrors do not carry, declared at the use site (P-72 ③): each line is
   C's own prototype and the unit is its first consumer in this file's include graph.  They are
   what the body below reaches through `lisp.h` (C 26's `#include`).  */
/// C: lisp.h 4389: `extern void message (const char *, ...) ATTRIBUTE_FORMAT_PRINTF (1, 2);`
#[cfg (feature = "have_dbus")]
unsafe extern "C" {
  pub fn message (format: *const c_char, ...);
  /// C: lisp.h 4803: `Lisp_Object internal_catch (Lisp_Object, Lisp_Object (*) (Lisp_Object),
  /// Lisp_Object);` (`eval.c` defines it; C 1990 is the call site).
  pub fn internal_catch (tag: Lisp_Object, func: Option<unsafe extern "C" fn (Lisp_Object)
							   -> Lisp_Object>,
                         arg: Lisp_Object) -> Lisp_Object;


  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (`eval.c` defines it; C 1470-1472
  /// are the three call sites).
  pub fn FUNCTIONP (object: Lisp_Object) -> bool;
  /// C: lisp.h 4542: `extern Lisp_Object make_formatted_string (const char *, ...)` (`editfns.c`
  /// defines it; C 2096 is the call site).
  pub fn make_formatted_string (format: *const c_char, ...) -> Lisp_Object;
  /// C: lisp.h 5579: `extern void xputenv (const char *);` (`alloc.c` defines it; C 2000 is the
  /// call site).
  pub fn xputenv (string: *const c_char);
}

/* C 27's `termhooks.h` 249: `, DBUS_EVENT` - the one member `termhooks.h` adds to `enum
   event_kind` under `HAVE_DBUS`, and the value the frozen mirror does not carry (its numbering
   stops at the members the frozen configuration has).  C 1744 stores it into `event.kind`.  The
   mirror gap is Owed 3 in the ledger, together with the shifted numbering a `have_dbus` build
   gives the members after it.  */
/// C: termhooks.h 249: `DBUS_EVENT` - the member the list adds under `HAVE_DBUS`, i.e. 27 with
/// `HAVE_DBUS` and without `THREADS_ENABLED`/`HAVE_NTGUI` (`SAVE_SESSION_EVENT` is at 246).
#[cfg (feature = "have_dbus")]
pub const DBUS_EVENT: event_kind = 27;

/* C 26-30: the reference's five headers.  All five edges carry the file gate (P-86 ⑧).
   `lisp.h` is the one that reaches this section's users (`Q*`, the checkers, `DEFUN!`), and
   `ENCODE_FILE` (C 1679) is `coding.h`'s in the port (`include/coding.rs`), so that mirror is
   included here as well - C reaches it through lisp.h.  */
#[cfg (feature = "have_dbus")]
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
#[cfg (feature = "have_dbus")]
include!("include/termhooks.rs");              // C 27: `#include "termhooks.h"`
#[cfg (feature = "have_dbus")]
include!("include/keyboard.rs");               // C 28: `#include "keyboard.h"`
#[cfg (feature = "have_dbus")]
include!("include/pdumper.rs");                // C 29: `#include "pdumper.h"`
#[cfg (feature = "have_dbus")]
include!("include/process.rs");                // C 30: `#include "process.h"`
#[cfg (feature = "have_dbus")]
include!("include/coding.rs");                 // C 26's `lisp.h` reaches `coding.h` (`ENCODE_FILE`)
#[cfg (feature = "have_dbus")]
include!("../lib/include/string.rs");          // C 26's `lisp.h` reaches `<string.h>`

/* C 32-35: `#ifndef O_RDONLY` / `/* WINDOWSNT.  */` / `#include <fcntl.h>` / `#endif`.  The
   guard is **dead on this target**: `O_RDONLY` is defined (`darwin-arm64.macros.tsv`;
   `tools/check-cond-eval.py src/dbusbind.c` reports the 27 conditions with 0 disagreements), so
   the reference takes no arm here - C reaches the value through `config.h`'s system headers, and
   `C 1685`'s `emacs_open (…, O_RDONLY, 0)` needs it.  No mirror in this unit's include graph
   declares it, so the port takes it from the `<fcntl.h>` mirror (whose own note records the
   measurement), exactly as `src/dired.rs` 60 does at its C 31; the `#ifndef` itself is recorded
   in `docs/ports/dbusbind.md` (the filelock `ELOOP` shape, Deviation 3).  */
#[cfg (feature = "have_dbus")]
include!("../lib/include/fcntl.in.rs");        // C 34: `#include <fcntl.h>`

/* C 37-39: `#ifndef DBUS_NUM_MESSAGE_TYPES` / `#define DBUS_NUM_MESSAGE_TYPES 5` / `#endif` -
   the fallback for a library whose `<dbus/dbus.h>` does not define it.  C 1355 tests
   `XFIXNAT (message_type) < DBUS_NUM_MESSAGE_TYPES`.  The two arms are the `#ifndef`'s and the
   header's own definition; both spell 5, and exactly one of the two items below is defined.  */
/// C: dbus/dbus-message.h: `#define DBUS_NUM_MESSAGE_TYPES 5` (the header's own definition).
#[cfg (all (feature = "have_dbus", feature = "dbus_num_message_types"))]
pub const DBUS_NUM_MESSAGE_TYPES: c_int = 5;
#[cfg (all (feature = "have_dbus", not (feature = "dbus_num_message_types")))]
pub const DBUS_NUM_MESSAGE_TYPES: c_int = 5;

/* C 42-47: `/* Some platforms define the symbol "interface", but we want to use it as a variable
   name below.  */` and the `#ifdef interface` / `#undef interface` / `#endif` pair.  The guard
   is against a macro the platform's headers define (WINDOWSNT's); Rust has no macro named
   `interface` to undo, so there is no arm to carry and the pair is recorded here (Deviation 6).  */

/* C 50-55's alist note, C 56's `static Lisp_Object xd_registered_buses;` and C 58-59's
   `static bool xd_in_read_queued_messages = 0;`.  Both are C's tentative definitions, i.e. the
   all-zero object; the port uses `Qnil` for the Lisp object, the value `syms_of_dbusbind_for_pdumper`
   (C 2007) stores anyway (Deviation 7).  */
/* Alist of D-Bus buses we are polling for messages.
   The key is the symbol or string of the bus, and the value is the
   connection address.  For every bus, just one connection is counted.
   If there shall be a second connection to the same bus, a different
   symbol or string for the bus must be chosen.  On Lisp level, a bus
   stands for the associated connection.  */
#[cfg (feature = "have_dbus")]
static mut xd_registered_buses: Lisp_Object = lisp_h_Qnil ();

/* Whether we are reading a D-Bus event.  */
#[cfg (feature = "have_dbus")]
static mut xd_in_read_queued_messages: bool = false;

/* C 61-64: `/* We use "xd_" and "XD_" as prefix for all internal symbols, because we don't want
   to poison other namespaces with "dbus_".  */`  */

/* C 67-90: `XD_SIGNAL1`, `XD_SIGNAL2` and `XD_SIGNAL3` - the three `do { … } while (0)` macros
   that either throw to `xd_read_queued_messages` or signal.  Each call site passes a `Lisp_Object`
   for every `arg`, so the parameters are `expr`s (PORTING.md 2.1: a macro body resolves its names
   where it is defined, so `Fthrow`/`xsignal*` are named directly).  */
/// C 67-73: `#define XD_SIGNAL1(arg) do { if (xd_in_read_queued_messages) Fthrow (Qdbus_error,
/// Qnil); else xsignal1 (Qdbus_error, arg); } while (0)`
#[cfg (feature = "have_dbus")]
macro_rules! XD_SIGNAL1 {
  ($arg:expr) => {
    {
      if xd_in_read_queued_messages
	{ Fthrow (Qdbus_error (), Qnil ()); }
      else
	{ xsignal1 (Qdbus_error (), $arg); }
    }
  };
}
/// C 75-81: `#define XD_SIGNAL2(arg1, arg2) …` (the `xsignal2` form)
#[cfg (feature = "have_dbus")]
macro_rules! XD_SIGNAL2 {
  ($arg1:expr, $arg2:expr) => {
    {
      if xd_in_read_queued_messages
	{ Fthrow (Qdbus_error (), Qnil ()); }
      else
	{ xsignal2 (Qdbus_error (), $arg1, $arg2); }
    }
  };
}
/// C 83-89: `#define XD_SIGNAL3(arg1, arg2, arg3) …` (the `xsignal3` form)
#[cfg (feature = "have_dbus")]
macro_rules! XD_SIGNAL3 {
  ($arg1:expr, $arg2:expr, $arg3:expr) => {
    {
      if xd_in_read_queued_messages
	{ Fthrow (Qdbus_error (), Qnil ()); }
      else
	{ xsignal3 (Qdbus_error (), $arg1, $arg2, $arg3); }
    }
  };
}

/* C 91-98: `#define XD_ERROR(error) do { /* Remove the trailing newline.  */ char const *mess =
   error.message; char const *nl = strchr (mess, '\n'); Lisp_Object err = make_string (mess, nl ?
   nl - mess : strlen (mess)); dbus_error_free (&error); XD_SIGNAL1 (err); } while (0)`.  The
   parameter is the lvalue `DBusError` C names (`derror` at every call site), so the macro takes
   it by reference and `&mut $error` is C's `&error`.  */
#[cfg (feature = "have_dbus")]
macro_rules! XD_ERROR {
  ($error:expr) => {
    {
      let mut error = $error;
      let mess: *const c_char = (*error).message;
      let nl: *mut c_char = strchr (mess, b'\n' as c_int);
      let err: Lisp_Object
	= make_string (mess,
		       (if ! nl.is_null () { nl.offset_from (mess) }
			else { strlen (mess) as isize }) as ptrdiff_t);
      dbus_error_free (error);
      XD_SIGNAL1! (err);
    }
  };
}

/* C 100-133: the debugging macros, both arms of C 104's `#ifdef DBUS_DEBUG`.
   C's `XD_DEBUG_MESSAGE(...)` is variadic (`__VA_ARGS__` is the format followed by its
   arguments), so the port's macro takes the format as one `expr` and the rest as token trees
   (`concat!`-built formats are `expr`s).  The disabled arm C 122-132 keeps its own condition,
   `Vdbus_debug` (C 123-131's `if (!NILP (Vdbus_debug))`), and its `snprintf`/`message` pair;
   the enabled arm C 105-121 uses `noninteractive` and `printf`.  `__func__` is the function's own
   name as a function-scope array, so each arm's expansion site names it through the port's own
   `let __func__` (the `androidmenu.rs` shape).  */
/// C 105-112: `#define XD_DEBUG_MESSAGE(...) do { char s[1024]; snprintf (s, sizeof s,
/// __VA_ARGS__); if (!noninteractive) printf ("%s: %s\n", __func__, s); message ("%s: %s",
/// __func__, s); } while (0)`.  C's `__func__` is the *expansion site's* function name; a Rust
/// `macro_rules!` body cannot see a local of the expansion site (macro hygiene), so the name C's
/// preprocessor supplies implicitly is the port's explicit first argument, and every call site
/// passes the `__func__` its function defines (Deviation 8).
#[cfg (all (feature = "have_dbus", feature = "dbus_debug"))]
macro_rules! XD_DEBUG_MESSAGE {
  ($func:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
    {
      let mut s: [c_char; 1024] = [0; 1024];
      snprintf (s.as_mut_ptr (), 1024, $fmt, $($arg),*);
      if ! noninteractive
	{ printf (c"%s: %s\n".as_ptr (), $func, s.as_ptr ()); }
      message (c"%s: %s".as_ptr (), $func, s.as_ptr ());
    }
  };
}
/// C 113-120: `#define XD_DEBUG_VALID_LISP_OBJECT_P(object) do { if (!valid_lisp_object_p
/// (object)) { XD_DEBUG_MESSAGE ("%d Assertion failure", __LINE__); XD_SIGNAL1 (build_string
/// ("Assertion failure")); } } while (0)`
#[cfg (all (feature = "have_dbus", feature = "dbus_debug"))]
macro_rules! XD_DEBUG_VALID_LISP_OBJECT_P {
  ($func:expr, $object:expr) => {
    {
      if valid_lisp_object_p ($object) == 0
	{
	  XD_DEBUG_MESSAGE! ($func, c"%d Assertion failure".as_ptr (), line! ());
	  XD_SIGNAL1! (build_string (c"Assertion failure".as_ptr ()));
	}
    }
  };
}
/// C 123-131: `# define XD_DEBUG_MESSAGE(...) do { if (!NILP (Vdbus_debug)) { char s[1024];
/// snprintf (s, sizeof s, __VA_ARGS__); message ("%s: %s", __func__, s); } } while (0)`
#[cfg (all (feature = "have_dbus", not (feature = "dbus_debug")))]
macro_rules! XD_DEBUG_MESSAGE {
  ($func:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
    {
      if ! NILP (Vdbus_debug)
	{
	  let mut s: [c_char; 1024] = [0; 1024];
	  snprintf (s.as_mut_ptr (), 1024, $fmt, $($arg),*);
	  message (c"%s: %s".as_ptr (), $func, s.as_ptr ());
	}
    }
  };
}
/// C 132: `# define XD_DEBUG_VALID_LISP_OBJECT_P(object)` - the disabled arm is empty.
#[cfg (all (feature = "have_dbus", not (feature = "dbus_debug")))]
macro_rules! XD_DEBUG_VALID_LISP_OBJECT_P {
  ($func:expr, $object:expr) => {};
}

/// C: `<inttypes.h>`: `#define PRIdMAX "jd"` - measured with clang on this target (`intmax_t` is
/// `long long`), the `dispnew.rs` 69-72 / `editfns.rs` 188-190 shape.  C 686 prints it through
/// `XD_DEBUG_MESSAGE ("%c %"PRIdMAX, dtype, pval)`.
#[cfg (feature = "have_dbus")]
macro_rules! PRIdMAX { () => { "jd" } }
/// C: `<inttypes.h>`: `#define PRIuMAX "ju"` - measured with clang on this target.
#[cfg (feature = "have_dbus")]
macro_rules! PRIuMAX { () => { "ju" } }

/// C: `lib/intprops.h` 68-69: `# define TYPE_MINIMUM(t) ((t) ~ TYPE_MAXIMUM (t))` /
/// `# define TYPE_MAXIMUM(t) …` (`lisp.h` reaches the header; the frozen mirror
/// `femacs/lib/include/intprops.rs` keeps C's own text and is not compiled, and the units that
/// need the semantics write the Rust primitive - `androidmenu.rs` 330-335's `TYPE_MAXIMUM`).
#[cfg (feature = "have_dbus")]
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
/// C: `lib/intprops.h`'s `TYPE_MINIMUM (dbus_int16_t)` / `dbus_int16_t` uses (C 675-678).
#[cfg (feature = "have_dbus")]
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }

/* C 135-172: the three arms of `XD_BASIC_DBUS_TYPE`.  C's condition tree is
   `#ifdef HAVE_DBUS_TYPE_IS_VALID` -> the library predicate; else `#ifdef DBUS_TYPE_UNIX_FD` ->
   the twelve-member disjunction; else the eleven-member one.  */
/// C 139-140: `#define XD_BASIC_DBUS_TYPE(type) (dbus_type_is_valid (type) &&
/// dbus_type_is_basic (type))`
#[cfg (all (feature = "have_dbus", feature = "have_dbus_type_is_valid"))]
macro_rules! XD_BASIC_DBUS_TYPE {
  ($type:expr) => { (dbus_type_is_valid ($type) != FALSE && dbus_type_is_basic ($type) != FALSE) };
}
/// C 143-156: the `DBUS_TYPE_UNIX_FD` arm of the disjunction, thirteen members.
#[cfg (all (feature = "have_dbus", not (feature = "have_dbus_type_is_valid"),
	    feature = "dbus_type_unix_fd"))]
macro_rules! XD_BASIC_DBUS_TYPE {
  ($type:expr) => {
    (($type == DBUS_TYPE_BYTE)
     || ($type == DBUS_TYPE_BOOLEAN)
     || ($type == DBUS_TYPE_INT16)
     || ($type == DBUS_TYPE_UINT16)
     || ($type == DBUS_TYPE_INT32)
     || ($type == DBUS_TYPE_UINT32)
     || ($type == DBUS_TYPE_INT64)
     || ($type == DBUS_TYPE_UINT64)
     || ($type == DBUS_TYPE_DOUBLE)
     || ($type == DBUS_TYPE_STRING)
     || ($type == DBUS_TYPE_OBJECT_PATH)
     || ($type == DBUS_TYPE_SIGNATURE)
     || ($type == DBUS_TYPE_UNIX_FD))
  };
}
/// C 158-170: the innermost arm, twelve members (`DBUS_TYPE_UNIX_FD` absent).
#[cfg (all (feature = "have_dbus", not (feature = "have_dbus_type_is_valid"),
	    not (feature = "dbus_type_unix_fd")))]
macro_rules! XD_BASIC_DBUS_TYPE {
  ($type:expr) => {
    (($type == DBUS_TYPE_BYTE)
     || ($type == DBUS_TYPE_BOOLEAN)
     || ($type == DBUS_TYPE_INT16)
     || ($type == DBUS_TYPE_UINT16)
     || ($type == DBUS_TYPE_INT32)
     || ($type == DBUS_TYPE_UINT32)
     || ($type == DBUS_TYPE_INT64)
     || ($type == DBUS_TYPE_UINT64)
     || ($type == DBUS_TYPE_DOUBLE)
     || ($type == DBUS_TYPE_STRING)
     || ($type == DBUS_TYPE_OBJECT_PATH)
     || ($type == DBUS_TYPE_SIGNATURE))
  };
}

/* C 174-178: `/* This was a macro.  On Solaris 2.11 it was said to compile for hours, when
   optimization is enabled.  So we have transferred it into a function.  */` and
   `/* Determine the DBusType of a given Lisp symbol.  OBJECT must be one of the predefined D-Bus
   type symbols.  */`.  */
/// C 179-204: `static int xd_symbol_to_dbus_type (Lisp_Object object)`.  C writes the chain as one
/// `return` of nested `?:` with C 195-197's `#ifdef DBUS_TYPE_UNIX_FD` member inside it; Rust has
/// no attribute that can sit on an `else if` arm, so the port keeps C's order and first-match
/// semantics as one `if … return` per member (Deviation 5).
#[cfg (feature = "have_dbus")]
unsafe fn xd_symbol_to_dbus_type (object: Lisp_Object) -> c_int
{
  if EQ (object, QCbyte ()) { return DBUS_TYPE_BYTE; }
  if EQ (object, QCboolean ()) { return DBUS_TYPE_BOOLEAN; }
  if EQ (object, QCint16 ()) { return DBUS_TYPE_INT16; }
  if EQ (object, QCuint16 ()) { return DBUS_TYPE_UINT16; }
  if EQ (object, QCint32 ()) { return DBUS_TYPE_INT32; }
  if EQ (object, QCuint32 ()) { return DBUS_TYPE_UINT32; }
  if EQ (object, QCint64 ()) { return DBUS_TYPE_INT64; }
  if EQ (object, QCuint64 ()) { return DBUS_TYPE_UINT64; }
  if EQ (object, QCdouble ()) { return DBUS_TYPE_DOUBLE; }
  if EQ (object, QCstring ()) { return DBUS_TYPE_STRING; }
  if EQ (object, QCobject_path ()) { return DBUS_TYPE_OBJECT_PATH; }
  if EQ (object, QCsignature ()) { return DBUS_TYPE_SIGNATURE; }
  /* C 195-197: `#ifdef DBUS_TYPE_UNIX_FD` / `: EQ (object, QCunix_fd) ? DBUS_TYPE_UNIX_FD` /
     `#endif`.  */
  #[cfg (feature = "dbus_type_unix_fd")]
  if EQ (object, QCunix_fd ()) { return DBUS_TYPE_UNIX_FD; }
  if EQ (object, QCarray ()) { return DBUS_TYPE_ARRAY; }
  if EQ (object, QCvariant ()) { return DBUS_TYPE_VARIANT; }
  if EQ (object, QCstruct ()) { return DBUS_TYPE_STRUCT; }
  if EQ (object, QCdict_entry ()) { return DBUS_TYPE_DICT_ENTRY; }
  return DBUS_TYPE_INVALID;
}

/* C 205: the `/* Determine the Lisp symbol of DBusType.  */` note.  */
/// C 206-231: `static Lisp_Object xd_dbus_type_to_symbol (int type)` - the same chain shape as
/// C 179-204, and the same Deviation 5 (C 222-224's gated member).
#[cfg (feature = "have_dbus")]
unsafe fn xd_dbus_type_to_symbol (type_: c_int) -> Lisp_Object
{
  if type_ == DBUS_TYPE_BYTE { return QCbyte (); }
  if type_ == DBUS_TYPE_BOOLEAN { return QCboolean (); }
  if type_ == DBUS_TYPE_INT16 { return QCint16 (); }
  if type_ == DBUS_TYPE_UINT16 { return QCuint16 (); }
  if type_ == DBUS_TYPE_INT32 { return QCint32 (); }
  if type_ == DBUS_TYPE_UINT32 { return QCuint32 (); }
  if type_ == DBUS_TYPE_INT64 { return QCint64 (); }
  if type_ == DBUS_TYPE_UINT64 { return QCuint64 (); }
  if type_ == DBUS_TYPE_DOUBLE { return QCdouble (); }
  if type_ == DBUS_TYPE_STRING { return QCstring (); }
  if type_ == DBUS_TYPE_OBJECT_PATH { return QCobject_path (); }
  if type_ == DBUS_TYPE_SIGNATURE { return QCsignature (); }
  /* C 222-224: `#ifdef DBUS_TYPE_UNIX_FD` / `: (type == DBUS_TYPE_UNIX_FD) ? QCunix_fd` /
     `#endif`.  */
  #[cfg (feature = "dbus_type_unix_fd")]
  if type_ == DBUS_TYPE_UNIX_FD { return QCunix_fd (); }
  if type_ == DBUS_TYPE_ARRAY { return QCarray (); }
  if type_ == DBUS_TYPE_VARIANT { return QCvariant (); }
  if type_ == DBUS_TYPE_STRUCT { return QCstruct (); }
  if type_ == DBUS_TYPE_DICT_ENTRY { return QCdict_entry (); }
  return Qnil ();
}

/// C 232: `#define XD_KEYWORDP(object) !NILP (Fkeywordp (object))`
#[cfg (feature = "have_dbus")]
macro_rules! XD_KEYWORDP {
  ($object:expr) => { ! NILP (Fkeywordp ($object)) };
}

/// C 234-236: `/* Check whether a Lisp symbol is a predefined D-Bus type symbol.  */`
/// `#define XD_DBUS_TYPE_P(object) XD_KEYWORDP (object) && ((xd_symbol_to_dbus_type (object) !=
/// DBUS_TYPE_INVALID))`
#[cfg (feature = "have_dbus")]
macro_rules! XD_DBUS_TYPE_P {
  ($object:expr) => { XD_KEYWORDP! ($object) && (xd_symbol_to_dbus_type ($object) != DBUS_TYPE_INVALID) };
}

/* C 238-242: the note on `XD_OBJECT_TO_DBUS_TYPE` (the conversion of the arguments of
   `dbus-call-method` / `dbus-send-signal` into C values).  */
/// C 243-257: `#define XD_OBJECT_TO_DBUS_TYPE(object) ((EQ (object, Qt) || NILP (object)) ?
/// DBUS_TYPE_BOOLEAN : (FIXNATP (object)) ? DBUS_TYPE_UINT32 : …)`.  The port binds the argument
/// once (`let object = $object;`): every call site passes a pure expression, and C's own macro
/// re-evaluates it, so the two are the same value.
#[cfg (feature = "have_dbus")]
macro_rules! XD_OBJECT_TO_DBUS_TYPE {
  ($object:expr) => {
    {
      let object = $object;
      if EQ (object, Qt ()) || NILP (object) { DBUS_TYPE_BOOLEAN }
      else if FIXNATP (object) { DBUS_TYPE_UINT32 }
      else if FIXNUMP (object) { DBUS_TYPE_INT32 }
      else if FLOATP (object) { DBUS_TYPE_DOUBLE }
      else if STRINGP (object) { DBUS_TYPE_STRING }
      else if XD_DBUS_TYPE_P! (object) { xd_symbol_to_dbus_type (object) }
      else if CONSP (object)
	{
	  if XD_DBUS_TYPE_P! (XCAR (object))
	    {
	      if XD_BASIC_DBUS_TYPE! (xd_symbol_to_dbus_type (XCAR (object)))
		{ DBUS_TYPE_ARRAY }
	      else { xd_symbol_to_dbus_type (XCAR (object)) }
	    }
	  else { DBUS_TYPE_ARRAY }
	}
      else { DBUS_TYPE_INVALID }
    }
  };
}

/// C 258-260: `#define XD_NEXT_VALUE(object) ((XD_DBUS_TYPE_P (CAR_SAFE (object))) ?
/// CDR_SAFE (object) : object)`
#[cfg (feature = "have_dbus")]
macro_rules! XD_NEXT_VALUE {
  ($object:expr) => {
    {
      let object = $object;
      if XD_DBUS_TYPE_P! (CAR_SAFE (object)) { CDR_SAFE (object) } else { object }
    }
  };
}

/* C 262-263: `/* Transform the message type to its string representation for debug messages.
   */`  */
/// C 264-274: `#define XD_MESSAGE_TYPE_TO_STRING(mtype) ((mtype == DBUS_MESSAGE_TYPE_INVALID) ?
/// "DBUS_MESSAGE_TYPE_INVALID" : …)`
#[cfg (feature = "have_dbus")]
macro_rules! XD_MESSAGE_TYPE_TO_STRING {
  ($mtype:expr) => {
    {
      let mtype = $mtype;
      if mtype == DBUS_MESSAGE_TYPE_INVALID { c"DBUS_MESSAGE_TYPE_INVALID".as_ptr () }
      else if mtype == DBUS_MESSAGE_TYPE_METHOD_CALL { c"DBUS_MESSAGE_TYPE_METHOD_CALL".as_ptr () }
      else if mtype == DBUS_MESSAGE_TYPE_METHOD_RETURN
	{ c"DBUS_MESSAGE_TYPE_METHOD_RETURN".as_ptr () }
      else if mtype == DBUS_MESSAGE_TYPE_ERROR { c"DBUS_MESSAGE_TYPE_ERROR".as_ptr () }
      else { c"DBUS_MESSAGE_TYPE_SIGNAL".as_ptr () }
    }
  };
}

/* C 276-277: `/* Transform the object to its string representation for debug messages.  */`  */
/// C 277-281: `static char * XD_OBJECT_TO_STRING (Lisp_Object object)`
#[cfg (feature = "have_dbus")]
unsafe fn XD_OBJECT_TO_STRING (object: Lisp_Object) -> *mut c_char
{
  AUTO_STRING! (format, c"%s".as_ptr ());
  return SSDATA (CALLN! (Fformat, format, object));
}

/* C 284-319: `#define XD_DBUS_VALIDATE_BUS_ADDRESS(bus) do { … } while (0)`.  The parameter is
   the variable C reassigns (`bus = QCsession;`), so the port spells it `$bus:ident` and C's
   assignment stays an assignment of the caller's local.  */
#[cfg (feature = "have_dbus")]
macro_rules! XD_DBUS_VALIDATE_BUS_ADDRESS {
  ($bus:ident) => {
    {
      let session_bus_address: *const c_char = egetenv (c"DBUS_SESSION_BUS_ADDRESS".as_ptr ());
      if STRINGP ($bus)
	{
	  let mut entries: *mut *mut DBusAddressEntry = std::ptr::null_mut ();
	  let mut len: c_int = 0;
	  let mut derror: DBusError = std::mem::zeroed ();
	  dbus_error_init (&raw mut derror);
	  if dbus_parse_address (SSDATA ($bus), &raw mut entries, &raw mut len,
				 &raw mut derror) == FALSE
	    { XD_ERROR! (&raw mut derror); }
	  /* Cleanup.  */
	  dbus_error_free (&raw mut derror);
	  dbus_address_entries_free (entries);
	  /* Canonicalize session bus address.  */
	  if (! session_bus_address.is_null ()
	      && ! NILP (Fstring_equal ($bus, build_string (session_bus_address))))
	    { $bus = QCsession (); }
	}

      else
	{
	  CHECK_SYMBOL ($bus);
	  if ! (EQ ($bus, QCsystem ()) || EQ ($bus, QCsession ())
		|| EQ ($bus, QCsystem_private ()) || EQ ($bus, QCsession_private ()))
	    { XD_SIGNAL2! (build_string (c"Wrong bus name".as_ptr ()), $bus); }
	  /* We do not want to have an autolaunch for the session bus.  */
	  if (EQ ($bus, QCsession ()) || EQ ($bus, QCsession_private ()))
	    && session_bus_address.is_null ()
	    { XD_SIGNAL2! (build_string (c"No connection to bus".as_ptr ()), $bus); }
	}
    }
  };
}

/* C 321-336: the `TODO: Shouldn't we assume, that recent D-Bus implementations carry
   HAVE_DBUS_VALIDATE_*?  See configure.ac.` note and the `#if (HAVE_DBUS_VALIDATE_BUS_NAME ||
   HAVE_DBUS_VALIDATE_PATH || HAVE_DBUS_VALIDATE_INTERFACE || HAVE_DBUS_VALIDATE_MEMBER)` region
   around `XD_DBUS_VALIDATE_OBJECT`.  C's condition is the disjunction; the port spells it at
   each of the four consumers (the `sqlite.rs` Deviation-12 shape) and defines the shared body
   once.  */
/// C 323-335: `#define XD_DBUS_VALIDATE_OBJECT(object, func) do { if (!NILP (object)) { DBusError
/// derror; CHECK_STRING (object); dbus_error_init (&derror); if (!func (SSDATA (object),
/// &derror)) XD_ERROR (derror); /* Cleanup.  */ dbus_error_free (&derror); } } while (0)`
#[cfg (all (feature = "have_dbus",
	    any (feature = "have_dbus_validate_bus_name", feature = "have_dbus_validate_path",
		 feature = "have_dbus_validate_interface", feature = "have_dbus_validate_member")))]
macro_rules! XD_DBUS_VALIDATE_OBJECT {
  ($object:expr, $func:expr) => {
    {
      let object = $object;
      if ! NILP (object)
	{
	  let mut derror: DBusError = std::mem::zeroed ();
	  CHECK_STRING (object);
	  dbus_error_init (&raw mut derror);
	  if $func (SSDATA (object), &raw mut derror) == FALSE
	    { XD_ERROR! (&raw mut derror); }
	  /* Cleanup.  */
	  dbus_error_free (&raw mut derror);
	}
    }
  };
}

/* C 338-368: the four validating macros, each with the `HAVE_DBUS_VALIDATE_*` arm and the
   `CHECK_STRING`-only `#else` arm.  C 338/346/354/362's conditions are the four names.  */
/// C 338-344: `#if HAVE_DBUS_VALIDATE_BUS_NAME` / `#define XD_DBUS_VALIDATE_BUS_NAME(bus_name)
/// XD_DBUS_VALIDATE_OBJECT (bus_name, dbus_validate_bus_name)` / `#else` / `#define … do { if
/// (!NILP (bus_name)) CHECK_STRING (bus_name); } while (false)` / `#endif`
#[cfg (all (feature = "have_dbus", feature = "have_dbus_validate_bus_name"))]
macro_rules! XD_DBUS_VALIDATE_BUS_NAME {
  ($bus_name:expr) => { XD_DBUS_VALIDATE_OBJECT! ($bus_name, dbus_validate_bus_name) };
}
#[cfg (all (feature = "have_dbus", not (feature = "have_dbus_validate_bus_name")))]
macro_rules! XD_DBUS_VALIDATE_BUS_NAME {
  ($bus_name:expr) => { { if ! NILP ($bus_name) { CHECK_STRING ($bus_name); } } };
}
/// C 346-352: the `HAVE_DBUS_VALIDATE_PATH` pair (the function is `dbus_validate_path`).
#[cfg (all (feature = "have_dbus", feature = "have_dbus_validate_path"))]
macro_rules! XD_DBUS_VALIDATE_PATH {
  ($path:expr) => { XD_DBUS_VALIDATE_OBJECT! ($path, dbus_validate_path) };
}
#[cfg (all (feature = "have_dbus", not (feature = "have_dbus_validate_path")))]
macro_rules! XD_DBUS_VALIDATE_PATH {
  ($path:expr) => { { if ! NILP ($path) { CHECK_STRING ($path); } } };
}
/// C 354-360: the `HAVE_DBUS_VALIDATE_INTERFACE` pair (`dbus_validate_interface`).
#[cfg (all (feature = "have_dbus", feature = "have_dbus_validate_interface"))]
macro_rules! XD_DBUS_VALIDATE_INTERFACE {
  ($interface:expr) => { XD_DBUS_VALIDATE_OBJECT! ($interface, dbus_validate_interface) };
}
#[cfg (all (feature = "have_dbus", not (feature = "have_dbus_validate_interface")))]
macro_rules! XD_DBUS_VALIDATE_INTERFACE {
  ($interface:expr) => { { if ! NILP ($interface) { CHECK_STRING ($interface); } } };
}
/// C 362-368: the `HAVE_DBUS_VALIDATE_MEMBER` pair (`dbus_validate_member`).
#[cfg (all (feature = "have_dbus", feature = "have_dbus_validate_member"))]
macro_rules! XD_DBUS_VALIDATE_MEMBER {
  ($member:expr) => { XD_DBUS_VALIDATE_OBJECT! ($member, dbus_validate_member) };
}
#[cfg (all (feature = "have_dbus", not (feature = "have_dbus_validate_member")))]
macro_rules! XD_DBUS_VALIDATE_MEMBER {
  ($member:expr) => { { if ! NILP ($member) { CHECK_STRING ($member); } } };
}

/* C 370-371: `/* Append to SIGNATURE a copy of X, making sure SIGNATURE does not become too
   long.  */`  */
/// C 372-380: `static void xd_signature_cat (char *signature, char const *x)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_signature_cat (signature: *mut c_char, x: *const c_char)
{
  let siglen: ptrdiff_t = strlen (signature) as ptrdiff_t;
  let xlen: ptrdiff_t = strlen (x) as ptrdiff_t;
  if (DBUS_MAXIMUM_SIGNATURE_LENGTH as ptrdiff_t) - xlen <= siglen
    { string_overflow (); }
  strcpy (signature.offset (siglen), x);
}

/* C 382-388: `/* Compute SIGNATURE of OBJECT.  It must have a form that it can be used in
   dbus_message_iter_open_container.  DTYPE is the DBusType the object is related to.  It is
   passed as argument, because it cannot be detected in basic type objects, when they are
   preceded by a type symbol.  PARENT_TYPE is the DBusType of a container this signature is
   embedded, or DBUS_TYPE_INVALID.  It is needed for the check that DBUS_TYPE_DICT_ENTRY occurs
   only as array element.  */`  */
/// C 389-575: `static void xd_signature (char *signature, int dtype, int parent_type,
/// Lisp_Object object)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_signature (signature: *mut c_char, mut dtype: c_int, parent_type: c_int,
			object: Lisp_Object)
{
  let __func__: *const c_char = c"xd_signature".as_ptr ();
  let mut subtype: c_int;
  let mut elt: Lisp_Object;
  let mut subsig: *const c_char;
  let mut x: [c_char; 255] = [0; 255];

  elt = object;

  /* C 388's `switch (dtype)` becomes this `match`; C's `case` order is kept.  */
  match dtype
    {
      DBUS_TYPE_BYTE | DBUS_TYPE_UINT16 =>
	{
	  CHECK_FIXNAT (object);
	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      DBUS_TYPE_BOOLEAN =>
	{
	  /* There must be an argument.  */
	  if EQ (QCboolean (), object)
	    { wrong_type_argument (Qbooleanp (), object); }
	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      DBUS_TYPE_INT16 =>
	{
	  CHECK_FIXNUM (object);
	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      DBUS_TYPE_UINT32 | DBUS_TYPE_UINT64 | DBUS_TYPE_INT32 | DBUS_TYPE_INT64 | DBUS_TYPE_DOUBLE =>
	{
	  CHECK_NUMBER (object);
	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      /* C 421-423: C's case group is `case DBUS_TYPE_UINT32: case DBUS_TYPE_UINT64: #ifdef
	 DBUS_TYPE_UNIX_FD case DBUS_TYPE_UNIX_FD: #endif case DBUS_TYPE_INT32: …` with one shared
	 body.  A `#[cfg]` cannot sit on one branch of an or-pattern, so the gated member is its own
	 arm with C's body (Deviation 5).  */
      #[cfg (feature = "dbus_type_unix_fd")]
      DBUS_TYPE_UNIX_FD =>
	{
	  CHECK_NUMBER (object);
	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      DBUS_TYPE_STRING | DBUS_TYPE_OBJECT_PATH | DBUS_TYPE_SIGNATURE =>
	{
	  /* We don't check the syntax of signature.  This will be done by libdbus.  */
	  if dtype == DBUS_TYPE_OBJECT_PATH
	    { XD_DBUS_VALIDATE_PATH! (object); }
	  else
	    { CHECK_STRING (object); }
	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      DBUS_TYPE_ARRAY =>
	{
	  /* Check that all list elements have the same D-Bus type.  For complex element types, we
	     just check the container type, not the whole element's signature.  */
	  CHECK_CONS (object);

	  /* Type symbol is optional.  */
	  if EQ (QCarray (), XCAR (elt))
	    { elt = XD_NEXT_VALUE! (elt); }

	  /* If the array is empty, DBUS_TYPE_STRING is the default element type.  */
	  if NILP (elt)
	    {
	      subtype = DBUS_TYPE_STRING;
	      subsig = DBUS_TYPE_STRING_AS_STRING;
	    }
	  else
	    {
	      subtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (elt));
	      xd_signature (x.as_mut_ptr (), subtype, dtype, CAR_SAFE (XD_NEXT_VALUE! (elt)));
	      subsig = x.as_ptr ();
	    }

	  /* If the element type is DBUS_TYPE_SIGNATURE, and this is the only element, the value
	     of this element is used as the array's element signature.  */
	  if subtype == DBUS_TYPE_SIGNATURE
	    {
	      let elt1 = XD_NEXT_VALUE! (elt);
	      if CONSP (elt1) && STRINGP (XCAR (elt1)) && NILP (XCDR (elt1))
		{
		  subsig = SSDATA (XCAR (elt1)) as *const c_char;
		  elt = Qnil ();
		}
	    }

	  while ! NILP (elt)
	    {
	      let mut x: [c_char; 255] = [0; 255];
	      subtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (elt));
	      xd_signature (x.as_mut_ptr (), subtype, dtype, CAR_SAFE (XD_NEXT_VALUE! (elt)));
	      if strcmp (subsig, x.as_ptr ()) != 0
		{ wrong_type_argument (QD_Bus (), CAR_SAFE (elt)); }
	      elt = CDR_SAFE (XD_NEXT_VALUE! (elt));
	    }

	  *signature = dtype as c_char;
	  *signature.add (1) = 0;
	  xd_signature_cat (signature, subsig);
	}

      DBUS_TYPE_VARIANT =>
	{
	  /* Check that there is exactly one list element.  */
	  CHECK_CONS (object);

	  elt = XD_NEXT_VALUE! (elt);
	  CHECK_CONS (elt);
	  subtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (elt));
	  xd_signature (x.as_mut_ptr (), subtype, dtype, CAR_SAFE (XD_NEXT_VALUE! (elt)));

	  if ! NILP (CDR_SAFE (XD_NEXT_VALUE! (elt)))
	    { wrong_type_argument (QD_Bus (), CAR_SAFE (CDR_SAFE (XD_NEXT_VALUE! (elt)))); }

	  sprintf (signature, c"%c".as_ptr (), dtype);
	}

      DBUS_TYPE_STRUCT =>
	{
	  /* A struct list might contain any (but zero) number of elements with different types.
	     No further check needed.  */
	  CHECK_CONS (object);

	  elt = XD_NEXT_VALUE! (elt);
	  CHECK_CONS (elt);

	  /* Compose the signature from the elements.  It is enclosed by parentheses.  */
	  sprintf (signature, c"%c".as_ptr (), DBUS_STRUCT_BEGIN_CHAR);
	  while ! NILP (elt)
	    {
	      subtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (elt));
	      xd_signature (x.as_mut_ptr (), subtype, dtype, CAR_SAFE (XD_NEXT_VALUE! (elt)));
	      xd_signature_cat (signature, x.as_ptr ());
	      elt = CDR_SAFE (XD_NEXT_VALUE! (elt));
	    }
	  xd_signature_cat (signature, DBUS_STRUCT_END_CHAR_AS_STRING);
	}

      DBUS_TYPE_DICT_ENTRY =>
	{
	  /* Check that there are exactly two list elements, and the first one is of basic type.
	     The dictionary entry itself must be an element of an array.  */
	  CHECK_CONS (object);

	  /* Check the parent object type.  */
	  if parent_type != DBUS_TYPE_ARRAY
	    { wrong_type_argument (QD_Bus (), object); }

	  /* Compose the signature from the elements.  It is enclosed by curly braces.  */
	  sprintf (signature, c"%c".as_ptr (), DBUS_DICT_ENTRY_BEGIN_CHAR);

	  /* First element.  */
	  elt = XD_NEXT_VALUE! (elt);
	  CHECK_CONS (elt);
	  subtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (elt));
	  xd_signature (x.as_mut_ptr (), subtype, dtype, CAR_SAFE (XD_NEXT_VALUE! (elt)));
	  xd_signature_cat (signature, x.as_ptr ());

	  if ! XD_BASIC_DBUS_TYPE! (subtype)
	    { wrong_type_argument (QD_Bus (), CAR_SAFE (XD_NEXT_VALUE! (elt))); }

	  /* Second element.  */
	  elt = CDR_SAFE (XD_NEXT_VALUE! (elt));
	  CHECK_CONS (elt);
	  subtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (elt));
	  xd_signature (x.as_mut_ptr (), subtype, dtype, CAR_SAFE (XD_NEXT_VALUE! (elt)));
	  xd_signature_cat (signature, x.as_ptr ());

	  if ! NILP (CDR_SAFE (XD_NEXT_VALUE! (elt)))
	    { wrong_type_argument (QD_Bus (), CAR_SAFE (CDR_SAFE (XD_NEXT_VALUE! (elt)))); }

	  /* Closing signature.  */
	  xd_signature_cat (signature, DBUS_DICT_ENTRY_END_CHAR_AS_STRING);
	}

      _ =>
	{
	  wrong_type_argument (QD_Bus (), object);
	}
    }

  XD_DEBUG_MESSAGE! (__func__, c"%s".as_ptr (), signature);
}

/* C 576: `/* Convert X to a signed integer with bounds LO and HI.  */`  */
/// C 577-603: `static intmax_t xd_extract_signed (Lisp_Object x, intmax_t lo, intmax_t hi)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_extract_signed (x: Lisp_Object, lo: intmax_t, hi: intmax_t) -> intmax_t
{
  CHECK_NUMBER (x);
  if INTEGERP (x)
    {
      let mut i: intmax_t = 0;
      if integer_to_intmax (x, &raw mut i) && lo <= i && i <= hi
	{ return i; }
    }
  else
    {
      let d: f64 = XFLOAT_DATA (x);
      if lo as f64 <= d && d < 1.0 + (hi as f64)
	{
	  let n: intmax_t = d as intmax_t;
	  if (n as f64) == d
	    { return n; }
	}
    }

  if xd_in_read_queued_messages
    { Fthrow (Qdbus_error (), Qnil ()); }
  else
    { args_out_of_range_3 (x, INT_TO_INTEGER (lo as EMACS_INT),
			   INT_TO_INTEGER (hi as EMACS_INT)); }
}

/* C 604: `/* Convert X to an unsigned integer with bounds 0 and HI.  */`  */
/// C 605-634: `static uintmax_t xd_extract_unsigned (Lisp_Object x, uintmax_t hi)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_extract_unsigned (x: Lisp_Object, hi: uintmax_t) -> uintmax_t
{
  CHECK_NUMBER (x);
  if INTEGERP (x)
    {
      let mut i: uintmax_t = 0;
      if integer_to_uintmax (x, &raw mut i) && i <= hi
	{ return i; }
    }
  else
    {
      let d: f64 = XFLOAT_DATA (x);
      if 0.0 <= d && d < 1.0 + (hi as f64)
	{
	  let n: uintmax_t = d as uintmax_t;
	  if (n as f64) == d
	    { return n; }
	}
    }

  if xd_in_read_queued_messages
    { Fthrow (Qdbus_error (), Qnil ()); }
  else
    { args_out_of_range_3 (x, make_fixnum (0), INT_TO_INTEGER (hi as EMACS_INT)); }
}

/* C 636: `/* Append C value, extracted from Lisp OBJECT, to iteration ITER.  DTYPE must be a valid
   DBusType.  It is used to convert Lisp objects, being arguments of `dbus-call-method' or
   `dbus-send-signal', into corresponding C values appended as arguments to a D-Bus message.  */`
   (C 635-636).  */
/// C 637-872: `static void xd_append_arg (int dtype, Lisp_Object object, DBusMessageIter *iter)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_append_arg (mut dtype: c_int, mut object: Lisp_Object, iter: *mut DBusMessageIter)
{
  let __func__: *const c_char = c"xd_append_arg".as_ptr ();
  /* C's two uninitialised locals (`char signature[DBUS_MAXIMUM_SIGNATURE_LENGTH]` and
     `DBusMessageIter subiter`) carry PORTING.md 1.5's zero initialiser (Deviation 9).  */
  let mut signature: [c_char; 255] = [0; 255];
  let mut subiter: DBusMessageIter = std::mem::zeroed ();

  if XD_BASIC_DBUS_TYPE! (dtype)
    {
      /* C 650-705's `switch (dtype)`: every arm returns, so the `match` diverges like C.  */
      match dtype
	{
	  DBUS_TYPE_BYTE =>
	    {
	      CHECK_FIXNAT (object);
	      {
		let val: u8 = (XFIXNAT (object) & 0xFF) as u8;
		XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, val as c_uint);
		if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		  == FALSE
		  { XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
		return;
	      }
	    }

	  DBUS_TYPE_BOOLEAN =>
	    {
	      /* There must be an argument.  */
	      if EQ (QCboolean (), object)
		{ wrong_type_argument (Qbooleanp (), object); }
	      {
		let val: dbus_bool_t = if NILP (object) { FALSE } else { TRUE };
		XD_DEBUG_MESSAGE! (__func__, c"%c %s".as_ptr (), dtype,
				   if val == FALSE { c"false".as_ptr () } else { c"true".as_ptr () });
		if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		  == FALSE
		  { XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
		return;
	      }
	    }

	  DBUS_TYPE_INT16 =>
	    {
	      let val: dbus_int16_t
		= xd_extract_signed (object, TYPE_MINIMUM! (dbus_int16_t) as intmax_t,
				     TYPE_MAXIMUM! (dbus_int16_t) as intmax_t) as dbus_int16_t;
	      let pval: c_int = val as c_int;
	      XD_DEBUG_MESSAGE! (__func__, c"%c %d".as_ptr (), dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_UINT16 =>
	    {
	      let val: dbus_uint16_t
		= xd_extract_unsigned (object, TYPE_MAXIMUM! (dbus_uint16_t) as uintmax_t)
		  as dbus_uint16_t;
	      let pval: c_uint = val as c_uint;
	      XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_INT32 =>
	    {
	      let val: dbus_int32_t
		= xd_extract_signed (object, TYPE_MINIMUM! (dbus_int32_t) as intmax_t,
				     TYPE_MAXIMUM! (dbus_int32_t) as intmax_t) as dbus_int32_t;
	      let pval: c_int = val as c_int;
	      XD_DEBUG_MESSAGE! (__func__, c"%c %d".as_ptr (), dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_UINT32 =>
	    {
	      let val: dbus_uint32_t
		= xd_extract_unsigned (object, TYPE_MAXIMUM! (dbus_uint32_t) as uintmax_t)
		  as dbus_uint32_t;
	      let pval: c_uint = val as c_uint;
	      XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  /* C 707-709: `#ifdef DBUS_TYPE_UNIX_FD` / `case DBUS_TYPE_UNIX_FD:` / `#endif` - C shares
	     C 706-716's body with `DBUS_TYPE_UINT32`; the gated member is its own arm (Deviation 5).  */
	  #[cfg (feature = "dbus_type_unix_fd")]
	  DBUS_TYPE_UNIX_FD =>
	    {
	      let val: dbus_uint32_t
		= xd_extract_unsigned (object, TYPE_MAXIMUM! (dbus_uint32_t) as uintmax_t)
		  as dbus_uint32_t;
	      let pval: c_uint = val as c_uint;
	      XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_INT64 =>
	    {
	      let val: dbus_int64_t
		= xd_extract_signed (object, TYPE_MINIMUM! (dbus_int64_t) as intmax_t,
				     TYPE_MAXIMUM! (dbus_int64_t) as intmax_t) as dbus_int64_t;
	      let pval: intmax_t = val as intmax_t;
	      XD_DEBUG_MESSAGE! (__func__, concat! ("%c %", PRIdMAX! (), "\0" ).as_ptr () as *const c_char,
				 dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_UINT64 =>
	    {
	      let val: dbus_uint64_t
		= xd_extract_unsigned (object, TYPE_MAXIMUM! (dbus_uint64_t) as uintmax_t)
		  as dbus_uint64_t;
	      let pval: uintmax_t = val as uintmax_t;
	      XD_DEBUG_MESSAGE! (__func__, concat! ("%c %", PRIuMAX! (), "\0" ).as_ptr () as *const c_char,
				 dtype, pval);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_DOUBLE =>
	    {
	      let val: f64 = extract_float (object);
	      XD_DEBUG_MESSAGE! (__func__, c"%c %f".as_ptr (), dtype, val);
	      if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		== FALSE
		{ XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
	      return;
	    }

	  DBUS_TYPE_STRING | DBUS_TYPE_OBJECT_PATH | DBUS_TYPE_SIGNATURE =>
	    {
	      /* We don't check the syntax of signature.  This will be done by libdbus.  */
	      if dtype == DBUS_TYPE_OBJECT_PATH
		{ XD_DBUS_VALIDATE_PATH! (object); }
	      else
		{ CHECK_STRING (object); }
	      {
		/* We need to send a valid UTF-8 string.  We could encode `object' but by not
		   encoding it, we guarantee it's valid utf-8, even if it contains
		   eight-bit-bytes.  Of course, you can still send manually-crafted junk by passing a
		   unibyte string.  */
		let val: *mut c_char = SSDATA (object);
		XD_DEBUG_MESSAGE! (__func__, c"%c %s".as_ptr (), dtype, val);
		if dbus_message_iter_append_basic (iter, dtype, &raw const val as *const c_void)
		  == FALSE
		  { XD_SIGNAL2! (build_string (c"Unable to append argument".as_ptr ()), object); }
		return;
	      }
	    }

	  _ =>
	    {
	      /* C's switch has no `default`: a `DBusType` outside the basic set cannot reach here
		 because `XD_BASIC_DBUS_TYPE` gates the switch (Deviation 10).  */
	    }
	}
    }

  else /* Compound types.  */
    {
      /* All compound types except array have a type symbol.  For array, it is optional.
	 Skip it.  */
      if ! XD_BASIC_DBUS_TYPE! (XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (object)))
	{ object = XD_NEXT_VALUE! (object); }

      /* Open new subiteration.  */
      match dtype
	{
	  DBUS_TYPE_ARRAY =>
	    {
	      /* An array has only elements of the same type.  So it is sufficient to check the
		 first element's signature only.  */

	      if NILP (object)
		/* If the array is empty, DBUS_TYPE_STRING is the default element type.  */
		{ strcpy (signature.as_mut_ptr (), DBUS_TYPE_STRING_AS_STRING); }

	      else
		{
		  /* If the element type is DBUS_TYPE_SIGNATURE, and this is the only element, the
		     value of this element is used as the array's element signature.  */
		  if CONSP (object)
		    && (XD_OBJECT_TO_DBUS_TYPE! (XCAR (object)) == DBUS_TYPE_SIGNATURE)
		    {
		      let val = XD_NEXT_VALUE! (object);
		      if CONSP (val) && STRINGP (XCAR (val)) && NILP (XCDR (val))
			&& (SBYTES (XCAR (val)) as usize) < DBUS_MAXIMUM_SIGNATURE_LENGTH
			{
			  lispstpcpy (signature.as_mut_ptr (), XCAR (val));
			  object = Qnil ();
			}
		    }

		  if ! NILP (object)
		    { xd_signature (signature.as_mut_ptr (),
				    XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (object)),
				    dtype, CAR_SAFE (XD_NEXT_VALUE! (object))); }
		}

	      XD_DEBUG_MESSAGE! (__func__, c"%c %s %s".as_ptr (), dtype, signature.as_ptr (),
				 XD_OBJECT_TO_STRING (object));
	      if dbus_message_iter_open_container (iter, dtype, signature.as_ptr (),
						   &raw mut subiter) == FALSE
		{ XD_SIGNAL3! (build_string (c"Cannot open container".as_ptr ()),
			       make_fixnum (dtype as EMACS_INT),
			       build_string (signature.as_ptr ())); }
	    }

	  DBUS_TYPE_VARIANT =>
	    {
	      /* A variant has just one element.  */
	      xd_signature (signature.as_mut_ptr (), XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (object)),
			    dtype, CAR_SAFE (XD_NEXT_VALUE! (object)));

	      XD_DEBUG_MESSAGE! (__func__, c"%c %s %s".as_ptr (), dtype, signature.as_ptr (),
				 XD_OBJECT_TO_STRING (object));
	      if dbus_message_iter_open_container (iter, dtype, signature.as_ptr (),
						   &raw mut subiter) == FALSE
		{ XD_SIGNAL3! (build_string (c"Cannot open container".as_ptr ()),
			       make_fixnum (dtype as EMACS_INT),
			       build_string (signature.as_ptr ())); }
	    }

	  DBUS_TYPE_STRUCT | DBUS_TYPE_DICT_ENTRY =>
	    {
	      /* These containers do not require a signature.  */
	      XD_DEBUG_MESSAGE! (__func__, c"%c %s".as_ptr (), dtype, XD_OBJECT_TO_STRING (object));
	      if dbus_message_iter_open_container (iter, dtype, std::ptr::null (),
						   &raw mut subiter) == FALSE
		{ XD_SIGNAL2! (build_string (c"Cannot open container".as_ptr ()),
			       make_fixnum (dtype as EMACS_INT)); }
	    }

	  _ =>
	    {
	      /* C's switch has no `default` here either; `dtype` is one of the four compound
		 types by construction (Deviation 10).  */
	    }
	}

      /* Loop over list elements.  */
      while ! NILP (object)
	{
	  dtype = XD_OBJECT_TO_DBUS_TYPE! (CAR_SAFE (object));
	  object = XD_NEXT_VALUE! (object);

	  xd_append_arg (dtype, CAR_SAFE (object), &raw mut subiter);

	  object = CDR_SAFE (object);
	}

      /* Close the subiteration.  */
      if dbus_message_iter_close_container (iter, &raw mut subiter) == FALSE
	{ XD_SIGNAL2! (build_string (c"Cannot close container".as_ptr ()),
		       make_fixnum (dtype as EMACS_INT)); }
    }
}

/* C 873-874: `/* Retrieve C value from a DBusMessageIter structure ITER, and return a converted
   Lisp object.  The type DTYPE of the argument of the D-Bus message must be a valid DBusType.
   Compound D-Bus types result always in a Lisp list.  */`  */
/// C 874-1002: `static Lisp_Object xd_retrieve_arg (int dtype, DBusMessageIter *iter)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_retrieve_arg (dtype: c_int, iter: *mut DBusMessageIter) -> Lisp_Object
{
  let __func__: *const c_char = c"xd_retrieve_arg".as_ptr ();
  match dtype
    {
      DBUS_TYPE_BYTE =>
	{
	  let mut val: c_uint = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  val = val & 0xFF;
	  XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, val);
	  return list2 (xd_dbus_type_to_symbol (dtype), make_fixnum (val as EMACS_INT));
	}

      DBUS_TYPE_BOOLEAN =>
	{
	  let mut val: dbus_bool_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  XD_DEBUG_MESSAGE! (__func__, c"%c %s".as_ptr (), dtype,
			     if val == FALSE { c"false".as_ptr () } else { c"true".as_ptr () });
	  return list2 (xd_dbus_type_to_symbol (dtype),
			if val == FALSE { Qnil () } else { Qt () });
	}

      DBUS_TYPE_INT16 =>
	{
	  let mut val: dbus_int16_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: c_int = val as c_int;
	  XD_DEBUG_MESSAGE! (__func__, c"%c %d".as_ptr (), dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), make_fixnum (val as EMACS_INT));
	}

      DBUS_TYPE_UINT16 =>
	{
	  let mut val: dbus_uint16_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: c_int = val as c_int;
	  XD_DEBUG_MESSAGE! (__func__, c"%c %d".as_ptr (), dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), make_fixnum (val as EMACS_INT));
	}

      DBUS_TYPE_INT32 =>
	{
	  let mut val: dbus_int32_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: c_int = val as c_int;
	  XD_DEBUG_MESSAGE! (__func__, c"%c %d".as_ptr (), dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), INT_TO_INTEGER (val as EMACS_INT));
	}

      DBUS_TYPE_UINT32 =>
	{
	  let mut val: dbus_uint32_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: c_uint = val as c_uint;
	  XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), INT_TO_INTEGER (val as EMACS_INT));
	}

      /* C 929-931: `#ifdef DBUS_TYPE_UNIX_FD` / `case DBUS_TYPE_UNIX_FD:` / `#endif` - shared with
	 `DBUS_TYPE_UINT32`'s body in C; its own arm here (Deviation 5).  */
      #[cfg (feature = "dbus_type_unix_fd")]
      DBUS_TYPE_UNIX_FD =>
	{
	  let mut val: dbus_uint32_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: c_uint = val as c_uint;
	  XD_DEBUG_MESSAGE! (__func__, c"%c %u".as_ptr (), dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), INT_TO_INTEGER (val as EMACS_INT));
	}

      DBUS_TYPE_INT64 =>
	{
	  let mut val: dbus_int64_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: intmax_t = val as intmax_t;
	  XD_DEBUG_MESSAGE! (__func__, concat! ("%c %", PRIdMAX! (), "\0" ).as_ptr () as *const c_char,
			     dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), INT_TO_INTEGER (val as EMACS_INT));
	}

      DBUS_TYPE_UINT64 =>
	{
	  let mut val: dbus_uint64_t = 0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  let pval: uintmax_t = val as uintmax_t;
	  XD_DEBUG_MESSAGE! (__func__, concat! ("%c %", PRIuMAX! (), "\0" ).as_ptr () as *const c_char,
			     dtype, pval);
	  return list2 (xd_dbus_type_to_symbol (dtype), INT_TO_INTEGER (val as EMACS_INT));
	}

      DBUS_TYPE_DOUBLE =>
	{
	  let mut val: f64 = 0.0;
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  XD_DEBUG_MESSAGE! (__func__, c"%c %f".as_ptr (), dtype, val);
	  return list2 (xd_dbus_type_to_symbol (dtype), make_float (val));
	}

      DBUS_TYPE_STRING | DBUS_TYPE_OBJECT_PATH | DBUS_TYPE_SIGNATURE =>
	{
	  let mut val: *mut c_char = std::ptr::null_mut ();
	  dbus_message_iter_get_basic (iter, &raw mut val as *mut c_void);
	  XD_DEBUG_MESSAGE! (__func__, c"%c %s".as_ptr (), dtype, val);
	  return list2 (xd_dbus_type_to_symbol (dtype), build_string (val));
	}

      DBUS_TYPE_ARRAY | DBUS_TYPE_VARIANT | DBUS_TYPE_STRUCT | DBUS_TYPE_DICT_ENTRY =>
	{
	  let mut result: Lisp_Object = Qnil ();
	  let mut subiter: DBusMessageIter = std::mem::zeroed ();
	  let mut subtype: c_int;
	  dbus_message_iter_recurse (iter, &raw mut subiter);
	  loop
	    {
	      subtype = dbus_message_iter_get_arg_type (&raw mut subiter);
	      if subtype == DBUS_TYPE_INVALID
		{ break; }
	      result = Fcons (xd_retrieve_arg (subtype, &raw mut subiter), result);
	      dbus_message_iter_next (&raw mut subiter);
	    }
	  XD_DEBUG_MESSAGE! (__func__, c"%c %s".as_ptr (), dtype, XD_OBJECT_TO_STRING (result));
	  return Fcons (xd_dbus_type_to_symbol (dtype), Fnreverse (result));
	}

      _ =>
	{
	  XD_DEBUG_MESSAGE! (__func__, c"DBusType '%c' not supported".as_ptr (), dtype);
	  return Qnil ();
	}
    }
}

/* C 1003: `/* Return the number of references of the shared CONNECTION.  */`  */
/// C 1004-1016: `static ptrdiff_t xd_get_connection_references (DBusConnection *connection)`.
/// C reads the first field of the library's private structure through the parameter's own slot;
/// the port keeps C's two reinterpreting assignments and says so (Deviation 11 - the reference's
/// own note records that the layout is not public).
#[cfg (feature = "have_dbus")]
unsafe fn xd_get_connection_references (mut connection: *mut DBusConnection) -> ptrdiff_t
{
  let mut refcount: *mut ptrdiff_t;

  refcount = &raw mut connection as *mut ptrdiff_t;
  refcount = *refcount as *mut ptrdiff_t;
  return *refcount;
}

/* C 1017: `/* Convert a Lisp D-Bus object to a pointer.  */`  */
/// C 1018-1021: `static DBusConnection *xd_lisp_dbus_to_dbus (Lisp_Object bus)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_lisp_dbus_to_dbus (bus: Lisp_Object) -> *mut DBusConnection
{
  return xmint_pointer (bus) as *mut DBusConnection;
}

/* C 1023-1025: `/* Return D-Bus connection address.  BUS is either a Lisp symbol, :system,
   :session, :system-private or :session-private, or a string denoting the bus address.  */`  */
/// C 1027-1047: `static DBusConnection *xd_get_connection_address (Lisp_Object bus)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_get_connection_address (bus: Lisp_Object) -> *mut DBusConnection
{
  let connection: *mut DBusConnection;
  let val: Lisp_Object;

  val = CDR_SAFE (Fassoc (bus, xd_registered_buses, Qnil ()));
  if NILP (val)
    { XD_SIGNAL2! (build_string (c"No connection to bus".as_ptr ()), bus); }
  else
    { connection = xd_lisp_dbus_to_dbus (val); }

  if dbus_connection_get_is_connected (connection) == FALSE
    { XD_SIGNAL2! (build_string (c"No connection to bus".as_ptr ()), bus); }

  return connection;
}

/* C 1049-1050: `/* Return the file descriptor for WATCH, -1 if not found.  */` and the
   `TODO: Shouldn't we assume, that recent D-Bus implementations carry
   HAVE_DBUS_WATCH_GET_UNIX_FD?  See configure.ac.` note.  */
/// C 1048-1060: `static int xd_find_watch_fd (DBusWatch *watch)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_find_watch_fd (watch: *mut DBusWatch) -> c_int
{
  let mut fd: c_int;
  /* C 1051-1058's `#if HAVE_DBUS_WATCH_GET_UNIX_FD` / `#else` pair.  */
  #[cfg (feature = "have_dbus_watch_get_unix_fd")]
  {
    /* TODO: Reverse these on w32, which prefers the opposite.  */
    fd = dbus_watch_get_unix_fd (watch);
    if fd == -1
      { fd = dbus_watch_get_socket (watch); }
  }
  #[cfg (not (feature = "have_dbus_watch_get_unix_fd"))]
  {
    fd = dbus_watch_get_fd (watch);
  }
  return fd;
}

/* C 1062-1063: `/* Prototype.  */` and `static void xd_read_queued_messages (int fd, void
   *data);`.  C's forward declaration has no Rust counterpart (PORTING.md 0.1-12): the item is
   defined at its C position below (C 1969-1995) and Rust's items are order-independent
   (Deviation 4).  The `/* Prototype.  */` note is C 1062's.  */

/// C 1065-1090: `static dbus_bool_t xd_add_watch (DBusWatch *watch, void *data)`.  C passes it to
/// `dbus_connection_set_watch_functions` (C 1160), so the item is `unsafe extern "C" fn` (the
/// `androidmenu.rs` three-function shape) rather than a plain `fn`.
#[cfg (feature = "have_dbus")]
unsafe extern "C" fn xd_add_watch (watch: *mut DBusWatch, data: *mut c_void) -> dbus_bool_t
{
  let __func__: *const c_char = c"xd_add_watch".as_ptr ();
  let flags: c_uint = dbus_watch_get_flags (watch);
  let fd: c_int = xd_find_watch_fd (watch);

  XD_DEBUG_MESSAGE! (__func__, c"fd %d, write %u, enabled %u".as_ptr (), fd,
		     flags & DBUS_WATCH_WRITABLE, dbus_watch_get_enabled (watch) as c_uint);

  if fd == -1
    { return FALSE; }

  if dbus_watch_get_enabled (watch) != FALSE
    {
      if (flags & DBUS_WATCH_WRITABLE) != 0
	{ add_write_fd (fd, Some (xd_read_queued_messages), data); }
      if (flags & DBUS_WATCH_READABLE) != 0
	{ add_read_fd (fd, Some (xd_read_queued_messages), data); }
    }
  return TRUE;
}

/* C 1092: `/* Stop monitoring WATCH for possible I/O.  DATA is the used bus, either a string or
   QCsystem, QCsession, QCsystem_private or QCsession_private.  */`  */
/// C 1092-1118: `static void xd_remove_watch (DBusWatch *watch, void *data)`
#[cfg (feature = "have_dbus")]
unsafe extern "C" fn xd_remove_watch (watch: *mut DBusWatch, data: *mut c_void)
{
  let __func__: *const c_char = c"xd_remove_watch".as_ptr ();
  let flags: c_uint = dbus_watch_get_flags (watch);
  let fd: c_int = xd_find_watch_fd (watch);

  XD_DEBUG_MESSAGE! (__func__, c"fd %d".as_ptr (), fd);

  if fd == -1
    { return; }

  /* Unset session environment.  */
  /* C 1104-1111: `#if 0` / `/* This is buggy, since unsetenv is not thread-safe.  */` /
     `if (XSYMBOL (QCsession) == data) || (XSYMBOL (QCsession_private) == data) { … unsetenv
     ("DBUS_SESSION_BUS_ADDRESS"); }` / `#endif`.  C's own dead text (`#if 0`), recorded here and
     not transcribed (Deviation 12; the `alloc.rs`/`android.rs` `#if 0` reading).  */

  if (flags & DBUS_WATCH_WRITABLE) != 0
    { delete_write_fd (fd); }
  if (flags & DBUS_WATCH_READABLE) != 0
    { delete_read_fd (fd); }
}

/* C 1120: `/* Toggle monitoring WATCH for possible I/O.  */`  */
/// C 1120-1128: `static void xd_toggle_watch (DBusWatch *watch, void *data)`
#[cfg (feature = "have_dbus")]
unsafe extern "C" fn xd_toggle_watch (watch: *mut DBusWatch, data: *mut c_void)
{
  if dbus_watch_get_enabled (watch) != FALSE
    { xd_add_watch (watch, data); }
  else
    { xd_remove_watch (watch, data); }
}

/* C 1130: `/* Close connection to D-Bus BUS.  */`  */
/// C 1130-1167: `static void xd_close_bus (Lisp_Object bus)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_close_bus (bus: Lisp_Object)
{
  let __func__: *const c_char = c"xd_close_bus".as_ptr ();
  let connection: *mut DBusConnection;
  let val: Lisp_Object;
  let busobj: Lisp_Object;

  /* Check whether we are connected.  */
  val = Fassoc (bus, xd_registered_buses, Qnil ());
  if NILP (val)
    { return; }

  busobj = CDR_SAFE (val);
  if NILP (busobj)
    {
      xd_registered_buses = Fdelete (val, xd_registered_buses);
      return;
    }

  /* Retrieve bus address.  */
  connection = xd_lisp_dbus_to_dbus (busobj);

  if xd_get_connection_references (connection) == 1
    {
      /* Close connection, if there isn't another shared application.  */
      XD_DEBUG_MESSAGE! (__func__, c"Close connection to bus %s".as_ptr (),
			 XD_OBJECT_TO_STRING (bus));
      dbus_connection_close (connection);

      xd_registered_buses = Fdelete (val, xd_registered_buses);
    }

  else
    /* Decrement reference count.  */
    { dbus_connection_unref (connection); }

  /* Return.  */
  return;
}

/* C 1168: blank.  C 1169-1293's `DEFUN ("dbus--init-bus", …)`.  */
#[cfg (feature = "have_dbus")]
DEFUN! ("dbus--init-bus", Fdbus__init_bus, Sdbus__init_bus, 1, 2, 0,
       doc: "Establish the connection to D-Bus BUS.\n\nThis function is dbus internal.  You almost certainly want to use\n`dbus-init-bus'.\n\nBUS can be either the symbol `:system' or the symbol `:session', or it\ncan be a string denoting the address of the corresponding bus.  For\nthe system and session buses, this function is called when loading\n`dbus.el', there is no need to call it again.\n\nA special case is BUS being the symbol `:system-private' or\n`:session-private'.  These symbols still denote the system or session\nbus, but using a private connection.  They should not be used outside\ndbus.el.\n\nThe function returns a number, which counts the connections this Emacs\nsession has established to the BUS under the same unique name (see\n`dbus-get-unique-name').  It depends on the libraries Emacs is linked\nwith, and on the environment Emacs is running.  For example, if Emacs\nis linked with the gtk toolkit, and it runs in a GTK-aware environment\nlike Gnome, another connection might already be established.\n\nWhen PRIVATE is non-nil, a new connection is established instead of\nreusing an existing one.  It results in a new unique name at the bus.\nThis can be used, if it is necessary to distinguish from another\nconnection used in the same Emacs process, like the one established by\nGTK+.  It should be used with care for at least the `:system' and\n`:session' buses, because other Emacs Lisp packages might already use\nthis connection to those buses.  ",
       (bus: Lisp_Object, private: Lisp_Object),
       {
  let __func__: *const c_char = c"Fdbus__init_bus".as_ptr ();
  /* C 1174-1177: `DBusConnection *connection; DBusError derror; Lisp_Object val; ptrdiff_t
     refcount;` - `derror` carries PORTING.md 1.5's zero initialiser (Deviation 9).  */
  let connection: *mut DBusConnection;
  let mut derror: DBusError = std::mem::zeroed ();
  let mut val: Lisp_Object;
  let refcount: ptrdiff_t;
  let mut bus = bus;

  /* Check parameter.  */
  if ! NILP (private)
    { bus = if EQ (bus, QCsystem ()) { QCsystem_private () }
	    else if EQ (bus, QCsession ()) { QCsession_private () } else { bus }; }
  XD_DBUS_VALIDATE_BUS_ADDRESS! (bus);

  /* Close bus if it is already open.  */
  xd_close_bus (bus);

  /* Check, whether we are still connected.  */
  val = Fassoc (bus, xd_registered_buses, Qnil ());
  if ! NILP (val)
    {
      connection = xd_get_connection_address (bus);
      dbus_connection_ref (connection);
    }

  else
    {
      /* Initialize.  */
      dbus_error_init (&raw mut derror);

      /* Open the connection.  */
      if STRINGP (bus)
	{
	  if NILP (private)
	    { connection = dbus_connection_open (SSDATA (bus), &raw mut derror); }
	  else
	    { connection = dbus_connection_open_private (SSDATA (bus), &raw mut derror); }
	}

      else
	{
	  let bustype: DBusBusType
	    = if EQ (bus, QCsystem ()) || EQ (bus, QCsystem_private ())
	      { DBUS_BUS_SYSTEM } else { DBUS_BUS_SESSION };
	  if NILP (private)
	    { connection = dbus_bus_get (bustype, &raw mut derror); }
	  else
	    { connection = dbus_bus_get_private (bustype, &raw mut derror); }
	}

      if dbus_error_is_set (&raw const derror) != FALSE
	{ XD_ERROR! (&raw mut derror); }

      if connection.is_null ()
	{ XD_SIGNAL2! (build_string (c"No connection to bus".as_ptr ()), bus); }

      /* If it is not the system or session bus, we must register ourselves.  Otherwise, we have
	 called dbus_bus_get{_private}, which has configured us to exit if the connection closes -
	 we undo this setting.  */
      if STRINGP (bus)
	{ dbus_bus_register (connection, &raw mut derror); }
      else
	{ dbus_connection_set_exit_on_disconnect (connection, FALSE); }

      if dbus_error_is_set (&raw const derror) != FALSE
	{ XD_ERROR! (&raw mut derror); }

      /* Add the watch functions.  We pass also the bus as data, in order to distinguish between
	 the buses in xd_remove_watch.  */
      if dbus_connection_set_watch_functions (connection,
					      Some (xd_add_watch),
					      Some (xd_remove_watch),
					      Some (xd_toggle_watch),
					      if XD_KEYWORDP! (bus)
					      { XSYMBOL (bus) as *mut c_void }
					      else { XSTRING (bus) as *mut c_void },
					      None) == FALSE
	{ XD_SIGNAL1! (build_string (c"Cannot add watch functions".as_ptr ())); }

      /* Add bus to list of registered buses.  */
      val = make_mint_ptr (connection as *mut c_void);
      xd_registered_buses = Fcons (Fcons (bus, val), xd_registered_buses);

      /* Cleanup.  */
      dbus_error_free (&raw mut derror);
    }

  XD_DEBUG_MESSAGE! (__func__, c"Registered buses: %s".as_ptr (),
		     XD_OBJECT_TO_STRING (xd_registered_buses));

  /* Return reference counter.  */
  refcount = xd_get_connection_references (connection);
  XD_DEBUG_MESSAGE! (__func__, c"Bus %s, Reference counter %td".as_ptr (),
		     XD_OBJECT_TO_STRING (bus), refcount);
  return make_fixnum (refcount as EMACS_INT);
});

/* C 1294: blank.  C 1295-1316's `DEFUN ("dbus-get-unique-name", …)`.  */
#[cfg (feature = "have_dbus")]
DEFUN! ("dbus-get-unique-name", Fdbus_get_unique_name, Sdbus_get_unique_name,
       1, 1, 0,
       doc: "Return the unique name of Emacs registered at D-Bus BUS.  ",
       (bus: Lisp_Object),
       {
  let __func__: *const c_char = c"Fdbus_get_unique_name".as_ptr ();
  /* C's macro reassigns its argument (`bus = QCsession;`), so the body rebinds the parameter
     (Deviation 8's sibling: the DEFUN!'s own binding is not `mut`).  */
  let mut bus = bus;
  let connection: *mut DBusConnection;
  let name: *const c_char;

  /* Check parameter.  */
  XD_DBUS_VALIDATE_BUS_ADDRESS! (bus);

  /* Retrieve bus address.  */
  connection = xd_get_connection_address (bus);

  /* Request the name.  */
  name = dbus_bus_get_unique_name (connection);
  if name.is_null ()
    { XD_SIGNAL1! (build_string (c"No unique name available".as_ptr ())); }

  /* Return.  */
  return build_string (name);
});

/* C 1317: blank.  C 1318-1666's `DEFUN ("dbus-message-internal", …)`.  */
#[cfg (feature = "have_dbus")]
DEFUN! ("dbus-message-internal", Fdbus_message_internal, Sdbus_message_internal,
       4, MANY, 0,
       doc: "Send a D-Bus message.\nThis is an internal function, it shall not be used outside dbus.el.\n\nThe following usages are expected:\n\n`dbus-call-method', `dbus-call-method-asynchronously':\n  (dbus-message-internal\n    dbus-message-type-method-call BUS SERVICE PATH INTERFACE METHOD HANDLER\n    &optional :timeout TIMEOUT :authorizable AUTH &rest ARGS)\n\n`dbus-send-signal':\n  (dbus-message-internal\n    dbus-message-type-signal BUS SERVICE PATH INTERFACE SIGNAL &rest ARGS)\n\n`dbus-method-return-internal':\n  (dbus-message-internal\n    dbus-message-type-method-return BUS SERVICE SERIAL &rest ARGS)\n\n`dbus-method-error-internal':\n  (dbus-message-internal\n    dbus-message-type-error BUS SERVICE SERIAL ERROR-NAME &rest ARGS)\n\n`dbus-check-arguments': (does not send a message)\n  (dbus-message-internal\n    dbus-message-type-invalid BUS SERVICE &rest ARGS)\n\nusage: (dbus-message-internal &rest REST)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let __func__: *const c_char = c"Fdbus_message_internal".as_ptr ();
  let mut message_type: Lisp_Object;
  let mut bus: Lisp_Object;
  let mut service: Lisp_Object;
  let mut handler: Lisp_Object;
  let mut path: Lisp_Object = Qnil ();
  let mut interface: Lisp_Object = Qnil ();
  let mut member: Lisp_Object = Qnil ();
  let mut error_name: Lisp_Object = Qnil ();
  let result: Lisp_Object;
  let connection: *mut DBusConnection;
  let dmessage: *mut DBusMessage;
  let mut iter: DBusMessageIter = std::mem::zeroed ();
  let mut dtype: c_int;
  let mtype: c_int;
  let mut serial: dbus_uint32_t = 0;
  let ui_serial: c_uint;
  let mut timeout: c_int = -1;
  let mut keepfd: dbus_bool_t = FALSE;
  let mut count: ptrdiff_t;
  let count0: ptrdiff_t;
  let mut signature: [c_char; 255] = [0; 255];

  /* Initialize parameters.  */
  message_type = *args.offset (0);
  bus = *args.offset (1);
  service = *args.offset (2);
  handler = Qnil ();

  CHECK_FIXNAT (message_type);
  if ! (DBUS_MESSAGE_TYPE_INVALID as EMACS_INT <= XFIXNAT (message_type)
	&& XFIXNAT (message_type) < DBUS_NUM_MESSAGE_TYPES as EMACS_INT)
    { XD_SIGNAL2! (build_string (c"Invalid message type".as_ptr ()), message_type); }
  mtype = XFIXNAT (message_type) as c_int;

  if (mtype == DBUS_MESSAGE_TYPE_METHOD_CALL) || (mtype == DBUS_MESSAGE_TYPE_SIGNAL)
    {
      path = *args.offset (3);
      interface = *args.offset (4);
      member = *args.offset (5);
      if mtype == DBUS_MESSAGE_TYPE_METHOD_CALL
	{ handler = *args.offset (6); }
      count = if mtype == DBUS_MESSAGE_TYPE_METHOD_CALL { 7 } else { 6 };
    }
  else if (mtype == DBUS_MESSAGE_TYPE_METHOD_RETURN) || (mtype == DBUS_MESSAGE_TYPE_ERROR)
    {
      serial = xd_extract_unsigned (*args.offset (3), TYPE_MAXIMUM! (dbus_uint32_t) as uintmax_t)
	as dbus_uint32_t;
      if mtype == DBUS_MESSAGE_TYPE_ERROR
	{ error_name = *args.offset (4); }
      count = if mtype == DBUS_MESSAGE_TYPE_ERROR { 5 } else { 4 };
    }
  else /* DBUS_MESSAGE_TYPE_INVALID  */
    { count = 3; }

  /* Check parameters.  */
  XD_DBUS_VALIDATE_BUS_ADDRESS! (bus);
  XD_DBUS_VALIDATE_BUS_NAME! (service);
  if (nargs as ptrdiff_t) < count
    { xsignal2 (Qwrong_number_of_arguments (), Qdbus_message_internal (),
		make_fixnum (nargs as EMACS_INT)); }

  if (mtype == DBUS_MESSAGE_TYPE_METHOD_CALL) || (mtype == DBUS_MESSAGE_TYPE_SIGNAL)
    {
      XD_DBUS_VALIDATE_PATH! (path);
      XD_DBUS_VALIDATE_INTERFACE! (interface);
      XD_DBUS_VALIDATE_MEMBER! (member);
      if ! NILP (handler)
	&& ! (FUNCTIONP (handler)
	      || (CONSP (handler)
		  && FUNCTIONP (CAR_SAFE (handler))
		  && FUNCTIONP (CDR_SAFE (handler))))
	{ wrong_type_argument (Qinvalid_function (), handler); }
    }

  /* Trace parameters.  */
  match mtype
    {
      DBUS_MESSAGE_TYPE_METHOD_CALL =>
	{
	  XD_DEBUG_MESSAGE! (__func__, c"%s %s %s %s %s %s %s".as_ptr (),
			     XD_MESSAGE_TYPE_TO_STRING! (mtype),
			     XD_OBJECT_TO_STRING (bus),
			     XD_OBJECT_TO_STRING (service),
			     XD_OBJECT_TO_STRING (path),
			     XD_OBJECT_TO_STRING (interface),
			     XD_OBJECT_TO_STRING (member),
			     XD_OBJECT_TO_STRING (handler));
	}
      DBUS_MESSAGE_TYPE_SIGNAL =>
	{
	  XD_DEBUG_MESSAGE! (__func__, c"%s %s %s %s %s %s".as_ptr (),
			     XD_MESSAGE_TYPE_TO_STRING! (mtype),
			     XD_OBJECT_TO_STRING (bus),
			     XD_OBJECT_TO_STRING (service),
			     XD_OBJECT_TO_STRING (path),
			     XD_OBJECT_TO_STRING (interface),
			     XD_OBJECT_TO_STRING (member));
	}
      DBUS_MESSAGE_TYPE_METHOD_RETURN =>
	{
	  ui_serial = serial as c_uint;
	  XD_DEBUG_MESSAGE! (__func__, c"%s %s %s %u".as_ptr (),
			     XD_MESSAGE_TYPE_TO_STRING! (mtype),
			     XD_OBJECT_TO_STRING (bus),
			     XD_OBJECT_TO_STRING (service),
			     ui_serial);
	}
      DBUS_MESSAGE_TYPE_ERROR =>
	{
	  ui_serial = serial as c_uint;
	  XD_DEBUG_MESSAGE! (__func__, c"%s %s %s %u %s".as_ptr (),
			     XD_MESSAGE_TYPE_TO_STRING! (mtype),
			     XD_OBJECT_TO_STRING (bus),
			     XD_OBJECT_TO_STRING (service),
			     ui_serial,
			     XD_OBJECT_TO_STRING (error_name));
	}
      _ => /* DBUS_MESSAGE_TYPE_INVALID  */
	{
	  XD_DEBUG_MESSAGE! (__func__, c"%s %s %s".as_ptr (),
			     XD_MESSAGE_TYPE_TO_STRING! (mtype),
			     XD_OBJECT_TO_STRING (bus),
			     XD_OBJECT_TO_STRING (service));
	}
    }

  /* Retrieve bus address.  */
  connection = xd_get_connection_address (bus);

  /* Create the D-Bus message.  Since DBUS_MESSAGE_TYPE_INVALID is not a valid message type, we
     mockup it with DBUS_MESSAGE_TYPE_SIGNAL.  */
  dmessage = dbus_message_new (if mtype == DBUS_MESSAGE_TYPE_INVALID
			       { DBUS_MESSAGE_TYPE_SIGNAL } else { mtype });
  if dmessage.is_null ()
    { XD_SIGNAL1! (build_string (c"Unable to create a new message".as_ptr ())); }

  if STRINGP (service) && (mtype != DBUS_MESSAGE_TYPE_INVALID)
    {
      if mtype != DBUS_MESSAGE_TYPE_SIGNAL
	/* Set destination.  */
	{
	  if dbus_message_set_destination (dmessage, SSDATA (service)) == FALSE
	    { XD_SIGNAL2! (build_string (c"Unable to set the destination".as_ptr ()),
			   service); }
	}

      else
	/* Set destination for unicast signals.  */
	{
	  let uname: Lisp_Object;

	  /* If it is the same unique name as we are registered at the bus or an unknown name, we
	     regard it as broadcast message due to backward compatibility.  */
	  if dbus_bus_name_has_owner (connection, SSDATA (service), std::ptr::null_mut ())
	    != FALSE
	    { uname = calln! (Qdbus_get_name_owner (), bus, service); }
	  else
	    { uname = Qnil (); }

	  if STRINGP (uname)
	    && (strcmp (dbus_bus_get_unique_name (connection), SSDATA (uname)) != 0)
	    && (dbus_message_set_destination (dmessage, SSDATA (service)) == FALSE)
	    { XD_SIGNAL2! (build_string (c"Unable to set signal destination".as_ptr ()),
			   service); }
	}
    }

  /* Set message parameters.  */
  if (mtype == DBUS_MESSAGE_TYPE_METHOD_CALL) || (mtype == DBUS_MESSAGE_TYPE_SIGNAL)
    {
      if (dbus_message_set_path (dmessage, SSDATA (path)) == FALSE
	  || dbus_message_set_interface (dmessage, SSDATA (interface)) == FALSE
	  || dbus_message_set_member (dmessage, SSDATA (member)) == FALSE)
	{ XD_SIGNAL1! (build_string (c"Unable to set the message parameter".as_ptr ())); }
    }

  else if (mtype == DBUS_MESSAGE_TYPE_METHOD_RETURN) || (mtype == DBUS_MESSAGE_TYPE_ERROR)
    {
      if dbus_message_set_reply_serial (dmessage, serial) == FALSE
	{ XD_SIGNAL1! (build_string (c"Unable to create a return message".as_ptr ())); }

      if (mtype == DBUS_MESSAGE_TYPE_ERROR)
	&& (dbus_message_set_error_name (dmessage, SSDATA (error_name)) == FALSE)
	{ XD_SIGNAL1! (build_string (c"Unable to create an error message".as_ptr ())); }
    }

  while count + 2 <= nargs as ptrdiff_t
    {
      /* Check for timeout parameter.  */
      if EQ (*args.offset (count), QCtimeout ())
	{
	  if mtype != DBUS_MESSAGE_TYPE_METHOD_CALL
	    { XD_SIGNAL1! (build_string (c":timeout is only supported on method calls".as_ptr ())); }

	  CHECK_FIXNAT (*args.offset (count + 1));
	  timeout = min (XFIXNAT (*args.offset (count + 1)) as isize, INT_MAX as isize) as c_int;
	  count = count + 2;
	}

      /* Check for authorizable parameter.  */
      else if EQ (*args.offset (count), QCauthorizable ())
	{
	  if mtype != DBUS_MESSAGE_TYPE_METHOD_CALL
	    { XD_SIGNAL1! (build_string (c":authorizable is only supported on method calls".as_ptr ())); }

	  /* Ignore this keyword if unsupported.  */
	  /* C 1553-1558: `#ifdef HAVE_DBUS_MESSAGE_SET_ALLOW_INTERACTIVE_AUTHORIZATION` /
	     `dbus_message_set_allow_interactive_authorization (dmessage, NILP (args[count+1]) ?
	     FALSE : TRUE);` / `#else` / `XD_DEBUG_MESSAGE (":authorizable not supported");` /
	     `#endif`.  */
	  #[cfg (feature = "have_dbus_message_set_allow_interactive_authorization")]
	  {
	    dbus_message_set_allow_interactive_authorization
	      (dmessage, if NILP (*args.offset (count + 1)) { FALSE } else { TRUE });
	  }
	  #[cfg (not (feature = "have_dbus_message_set_allow_interactive_authorization"))]
	  {
	    XD_DEBUG_MESSAGE! (__func__, c":authorizable not supported".as_ptr ());
	  }

	  count = count + 2;
	}

      /* Check for keepfd parameter.  */
      else if EQ (*args.offset (count), QCkeep_fd ())
	{
	  if mtype != DBUS_MESSAGE_TYPE_METHOD_CALL
	    { XD_SIGNAL1! (build_string (c":keep-fd is only supported on method calls".as_ptr ())); }
	  /* This is because the error handler and the keepfd path use the same slot in
	     Vdbus_registered_objects_table.  */
	  if CONSP (handler)
	    { XD_SIGNAL1! (build_string (c":keep-fd cannot be used when there is an error handler".as_ptr ())); }

	  /* Ignore this keyword if unsupported.  */
	  /* C 1577-1581: `#ifdef DBUS_TYPE_UNIX_FD` / `keepfd = TRUE;` / `#else` /
	     `XD_DEBUG_MESSAGE (":keep-fd not supported");` / `#endif`.  */
	  #[cfg (feature = "dbus_type_unix_fd")]
	  { keepfd = TRUE; }
	  #[cfg (not (feature = "dbus_type_unix_fd"))]
	  { XD_DEBUG_MESSAGE! (__func__, c":keep-fd not supported".as_ptr ()); }

	  count = count + 1;
	}

      else { break; }
    }

  /* Initialize parameter list of message.  */
  dbus_message_iter_init_append (dmessage, &raw mut iter);

  /* Append parameters to the message.  */
  count0 = count - 1;
  while count < nargs as ptrdiff_t
    {
      dtype = XD_OBJECT_TO_DBUS_TYPE! (*args.offset (count));
      if (count + 1 < nargs as ptrdiff_t) && XD_DBUS_TYPE_P! (*args.offset (count))
	{
	  XD_DEBUG_VALID_LISP_OBJECT_P! (__func__, *args.offset (count));
	  XD_DEBUG_VALID_LISP_OBJECT_P! (__func__, *args.offset (count + 1));
	  XD_DEBUG_MESSAGE! (__func__, c"Parameter%td: %s Parameter%td: %s".as_ptr (),
			     count - count0,
			     XD_OBJECT_TO_STRING (*args.offset (count)),
			     count + 1 - count0,
			     XD_OBJECT_TO_STRING (*args.offset (count + 1)));
	  count = count + 1;
	}
      else
	{
	  XD_DEBUG_VALID_LISP_OBJECT_P! (__func__, *args.offset (count));
	  XD_DEBUG_MESSAGE! (__func__, c"Parameter%td: %s".as_ptr (), count - count0,
			     XD_OBJECT_TO_STRING (*args.offset (count)));
	}

      /* Check for valid signature.  We use DBUS_TYPE_INVALID as indication that there is no
	 parent type.  */
      xd_signature (signature.as_mut_ptr (), dtype, DBUS_TYPE_INVALID, *args.offset (count));

      xd_append_arg (dtype, *args.offset (count), &raw mut iter);
      count = count + 1;
    }

  if mtype == DBUS_MESSAGE_TYPE_INVALID
    { result = Qt (); }

  else if ! NILP (handler)
    {
      /* Send the message.  The message is just added to the outgoing message queue.  */
      if dbus_connection_send_with_reply (connection, dmessage, std::ptr::null_mut (), timeout)
	== FALSE
	{ XD_SIGNAL1! (build_string (c"Cannot send message".as_ptr ())); }

      /* The result is the key in Vdbus_registered_objects_table.  */
      serial = dbus_message_get_serial (dmessage);
      result = list3 (QCserial (), bus, INT_TO_INTEGER (serial as EMACS_INT));

      /* Create a hash table entry.  */
      Fputhash (result, if keepfd != FALSE { Fcons (handler, path) } else { handler },
		Vdbus_registered_objects_table);
    }
  else
    {
      /* Send the message.  The message is just added to the outgoing message queue.  */
      if dbus_connection_send (connection, dmessage, std::ptr::null_mut ()) == FALSE
	{ XD_SIGNAL1! (build_string (c"Cannot send message".as_ptr ())); }

      result = Qnil ();
    }

  if mtype != DBUS_MESSAGE_TYPE_INVALID
    { XD_DEBUG_MESSAGE! (__func__, c"Message sent: %s".as_ptr (), XD_OBJECT_TO_STRING (result)); }

  /* Cleanup.  */
  dbus_message_unref (dmessage);

  /* Return the result.  */
  return result;
});

/* C 1668-1669: `/* Alist of registered file descriptors for D-Bus.  … */` and `static Lisp_Object
   xd_registered_fds;` (zeroed like `xd_registered_buses`, Deviation 7).  */
/* Alist of registered file descriptors for D-Bus.
   The key is an open file descriptor, retrieved via `dbus-call-method'
   or `dbus--open-fd'.  The value is a string OBJECT-PATH or FILENAME,
   which represents the arguments the function was called with.  Those
   values are not needed for further operations; they are just shown for
   information.  */
#[cfg (feature = "have_dbus")]
static mut xd_registered_fds: Lisp_Object = lisp_h_Qnil ();

/* C 1670-1693's `DEFUN ("dbus--fd-open", …)`.  */
#[cfg (feature = "have_dbus")]
DEFUN! ("dbus--fd-open", Fdbus__fd_open, Sdbus__fd_open, 1, 1, 0,
       doc: "Open FILENAME and return the respective read-only file descriptor.  ",
       (filename: Lisp_Object),
       {
  let mut filename = filename;
  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());
  filename = ENCODE_FILE (filename);

  /* Check, whether it is registered already.  */
  let registered: Lisp_Object = Frassoc (filename, xd_registered_fds);
  if ! NILP (registered)
    { return CAR_SAFE (registered); }

  /* Open file descriptor.  */
  let fd: c_int = emacs_open (SSDATA (filename), O_RDONLY, 0);

  if fd <= 0
    { XD_SIGNAL2! (build_string (c"Cannot open file".as_ptr ()), filename); }

  /* Register file descriptor.  */
  xd_registered_fds
    = Fcons (Fcons (INT_TO_INTEGER (fd as EMACS_INT), filename), xd_registered_fds);
  return INT_TO_INTEGER (fd as EMACS_INT);
});

/* C 1694: blank.  C 1695-1714's `DEFUN ("dbus--fd-close", …)`.  */
#[cfg (feature = "have_dbus")]
DEFUN! ("dbus--fd-close", Fdbus__fd_close, Sdbus__fd_close, 1, 1, 0,
       doc: "Close file descriptor FD.\nFD must be the result of a `dbus-call-method' or `dbus--fd-open' call,\nsee `dbus--registered-fds'.  It returns t in case of success, or nil if\nit isn't be possible to close the file descriptor, or if the file\ndescriptor is closed already.  ",
       (fd: Lisp_Object),
       {
  CHECK_FIXNUM (fd);

  /* Check, whether it is registered.  */
  let registered: Lisp_Object = assoc_no_quit (fd, xd_registered_fds);
  if NILP (registered)
    { return Qnil (); }
  else
    {
      xd_registered_fds = Fdelete (registered, xd_registered_fds);
      return if emacs_close (XFIXNAT (fd) as c_int) == 0 { Qt () } else { Qnil () };
    }
});

/* C 1715: blank.  C 1716-1739's `DEFUN ("dbus--registered-fds", …)`.  */
#[cfg (feature = "have_dbus")]
DEFUN! ("dbus--registered-fds", Fdbus__registered_fds, Sdbus__registered_fds,
       0, 0, 0,
       doc: "Return registered file descriptors, an alist.\nThe key is an open file descriptor, retrieved via `dbus-call-method' or\n`dbus--open-fd'.  The value is a string OBJECT-PATH or FILENAME, which\nrepresents the arguments the function was called with.  Those values are\nnot needed for further operations; they are just shown for information.\n\nThis alist allows to check, whether other packages of the running Emacs\ninstance have acquired a file descriptor as well.  ",
       (),
       {
  /* We return a copy of xd_registered_fds, in order to protect it against malicious
     manipulation.  */
  let mut registered: Lisp_Object = xd_registered_fds;
  let mut result: Lisp_Object = Qnil ();
  while ! NILP (registered)
    {
      let tem: Lisp_Object = CAR_SAFE (registered);
      result = Fcons (Fcons (CAR_SAFE (tem), CDR_SAFE (tem)), result);
      registered = CDR_SAFE (registered);
    }
  return Fnreverse (result);
});

/* `termhooks.h` 373-412's `enum event_kind kind : 16` shares the mirror's `input_event::bits`
   unit with `part : 16` (PORTING.md 1.3/P-80's member-named shape); the `keyboard.rs`/
   `androidselect.rs` per-unit macro, body-identical.  C 1744's `event.kind = DBUS_EVENT;` is the
   one use.  */
#[cfg (feature = "have_dbus")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* C 1741: `/* Construct a D-Bus event, and store it into the input event queue.  */`  */
/// C 1741-1760: `static void xd_store_event (Lisp_Object handler, Lisp_Object handler_args,
/// Lisp_Object event_args)`
#[cfg (feature = "have_dbus")]
unsafe fn xd_store_event (handler: Lisp_Object, handler_args: Lisp_Object,
			  event_args: Lisp_Object)
{
  let __func__: *const c_char = c"xd_store_event".as_ptr ();
  /* C's `struct input_event event; EVENT_INIT (event);` - C's own compound literal zero-fills
     the record (termhooks.rs's `EVENT_INIT`), so the port's initialiser is the same call.  */
  let mut event: input_event = std::mem::zeroed ();
  EVENT_INIT (&raw mut event);
  input_event_bits! (kind, &raw mut event, DBUS_EVENT);
  event.frame_or_window = Qnil ();
  /* Handler and handler args.  */
  event.arg = Fcons (handler, handler_args);
  /* Event args.  */
  event.arg = CALLN! (Fappend, event_args, event.arg);
  /* Store it into the input event queue.  */
  kbd_buffer_store_event (&raw mut event as *mut c_void);

  XD_DEBUG_MESSAGE! (__func__, c"Event stored: %s".as_ptr (), XD_OBJECT_TO_STRING (event.arg));
}

/* C 1761-1762: `/* Read one queued incoming message of the D-Bus BUS.  BUS is either a Lisp
   symbol, :system, :session, :system-private or :session-private, or a string denoting the bus
   address.  */`  */
/// C 1762-1951: `static void xd_read_message_1 (DBusConnection *connection, Lisp_Object bus)`.
/// C's `goto cleanup` (C 1846, 1943) and `goto monitor` (C 1852, 1880) are the two labelled
/// blocks below: `break 'cleanup` is `goto cleanup`, `break 'first` is `goto monitor`, and the
/// first block's end is C's fall-through into `monitor:` (PORTING.md 2.2; the `macro-body-notes`
/// row records the token).
#[cfg (feature = "have_dbus")]
unsafe fn xd_read_message_1 (connection: *mut DBusConnection, bus: Lisp_Object)
{
  let __func__: *const c_char = c"xd_read_message_1".as_ptr ();
  let mut args: Lisp_Object;
  let mut event_args: Lisp_Object;
  let mut key: Lisp_Object;
  let mut value: Lisp_Object;
  let dmessage: *mut DBusMessage;
  let mut iter: DBusMessageIter = std::mem::zeroed ();
  let mut dtype: c_int;
  let mtype: c_int;
  let mut serial: dbus_uint32_t;
  let mut ui_serial: c_uint;
  let uname: *const c_char;
  let destination: *const c_char;
  let path: *const c_char;
  let interface: *const c_char;
  let member: *const c_char;
  let error_name: *const c_char;

  dmessage = dbus_connection_pop_message (connection);

  /* Return if there is no queued message.  */
  if dmessage.is_null ()
    { return; }

  /* Collect the parameters.  */
  args = Qnil ();

  /* Loop over the resulting parameters.  Construct a list.  */
  if dbus_message_iter_init (dmessage, &raw mut iter) != FALSE
    {
      loop
	{
	  dtype = dbus_message_iter_get_arg_type (&raw mut iter);
	  if dtype == DBUS_TYPE_INVALID
	    { break; }
	  args = Fcons (xd_retrieve_arg (dtype, &raw mut iter), args);
	  dbus_message_iter_next (&raw mut iter);
	}
      /* The arguments are stored in reverse order.  Reorder them.  */
      args = Fnreverse (args);
    }

  /* Read message type, message serial, unique name, object path, interface, member and error
     name from the message.  */
  mtype = dbus_message_get_type (dmessage);
  serial = if (mtype == DBUS_MESSAGE_TYPE_METHOD_RETURN) || (mtype == DBUS_MESSAGE_TYPE_ERROR)
	   { dbus_message_get_reply_serial (dmessage) }
	   else { dbus_message_get_serial (dmessage) };
  ui_serial = serial as c_uint;
  uname = dbus_message_get_sender (dmessage);
  destination = dbus_message_get_destination (dmessage);
  path = dbus_message_get_path (dmessage);
  interface = dbus_message_get_interface (dmessage);
  member = dbus_message_get_member (dmessage);
  error_name = dbus_message_get_error_name (dmessage);

  XD_DEBUG_MESSAGE! (__func__, c"Event received: %s %u %s %s %s %s %s %s".as_ptr (),
		     XD_MESSAGE_TYPE_TO_STRING! (mtype),
		     ui_serial, uname, destination, path, interface,
		     if mtype == DBUS_MESSAGE_TYPE_ERROR { error_name } else { member },
		     XD_OBJECT_TO_STRING (args));

  /* Add type, serial, uname, destination, path, interface and member or error_name to the
     event_args.  */
  event_args
    = Fcons (if mtype == DBUS_MESSAGE_TYPE_ERROR
	     { if error_name.is_null () { Qnil () } else { build_string (error_name) } }
	     else { if member.is_null () { Qnil () } else { build_string (member) } },
	     Qnil ());
  event_args = Fcons (if interface.is_null () { Qnil () } else { build_string (interface) },
		      event_args);
  event_args = Fcons (if path.is_null () { Qnil () } else { build_string (path) },
		     event_args);
  event_args = Fcons (if destination.is_null () { Qnil () } else { build_string (destination) },
		     event_args);
  event_args = Fcons (if uname.is_null () { Qnil () } else { build_string (uname) },
		     event_args);
  event_args = Fcons (INT_TO_INTEGER (serial as EMACS_INT), event_args);
  event_args = Fcons (make_fixnum (mtype as EMACS_INT), event_args);

  /* Add the bus symbol to the event.  */
  event_args = Fcons (bus, event_args);

  'cleanup: {
    'first: {
      if mtype == DBUS_MESSAGE_TYPE_INVALID
	{ break 'cleanup; }

      else if (mtype == DBUS_MESSAGE_TYPE_METHOD_RETURN) || (mtype == DBUS_MESSAGE_TYPE_ERROR)
	{
	  /* Search for a registered function of the message.  */
	  key = list3 (QCserial (), bus, INT_TO_INTEGER (serial as EMACS_INT));
	  value = Fgethash (key, Vdbus_registered_objects_table, Qnil ());

	  /* There shall be exactly one entry.  Construct an event.  */
	  if NILP (value)
	    { break 'first; }

	  /* Remove the entry.  */
	  Fremhash (key, Vdbus_registered_objects_table);

	  /* C 1855-1869: `#ifdef DBUS_TYPE_UNIX_FD` / `/* Check, whether there is a file
	     descriptor to be kept.  value is (handler . path) args is ((:unix-fd NN) ...)  */`
	     / the `xd_registered_fds` block / `#endif`.  */
	  #[cfg (feature = "dbus_type_unix_fd")]
	  {
	    if CONSP (value)
	      && CONSP (CAR_SAFE (args))
	      && EQ (CAR_SAFE (CAR_SAFE (args)), QCunix_fd ())
	      {
		xd_registered_fds
		  = Fcons (Fcons (CAR_SAFE (CDR_SAFE (CAR_SAFE (args))),
				  CDR_SAFE (value)),
			   xd_registered_fds);
		value = CAR_SAFE (value);
	      }
	  }

	  /* Store the event.  */
	  xd_store_event (value, args, event_args);
	}

      else /* DBUS_MESSAGE_TYPE_METHOD_CALL, DBUS_MESSAGE_TYPE_SIGNAL.  */
	{
	  /* Vdbus_registered_objects_table requires non-nil interface and member.  */
	  if interface.is_null () || member.is_null ()
	    { break 'first; }

	  /* Search for a registered function of the message.  */
	  key = list4 (if mtype == DBUS_MESSAGE_TYPE_METHOD_CALL { QCmethod () } else { QCsignal () },
		       bus, build_string (interface), build_string (member));
	  value = Fgethash (key, Vdbus_registered_objects_table, Qnil ());

	  /* A signal could be registered with a nil interface or member.  */
	  if mtype == DBUS_MESSAGE_TYPE_SIGNAL
	    {
	      key = list4 (QCsignal (), bus, Qnil (), build_string (member));
	      value = CALLN! (Fappend, value,
			      Fgethash (key, Vdbus_registered_objects_table, Qnil ()));

	      key = list4 (QCsignal (), bus, build_string (interface), Qnil ());
	      value = CALLN! (Fappend, value,
			      Fgethash (key, Vdbus_registered_objects_table, Qnil ()));

	      key = list4 (QCsignal (), bus, Qnil (), Qnil ());
	      value = CALLN! (Fappend, value,
			      Fgethash (key, Vdbus_registered_objects_table, Qnil ()));
	    }

	  let mut called_handlers: Lisp_Object = Qnil ();
	  /* Loop over the registered functions.  Construct an event.  */
	  while ! NILP (value)
	    {
	      key = CAR_SAFE (value);
	      let key_uname: Lisp_Object = CAR_SAFE (key);
	      /* key has the structure (UNAME SERVICE PATH HANDLER).  */
	      if ! uname.is_null () && ! NILP (key_uname)
		&& strcmp (uname, SSDATA (key_uname)) != 0
		{ value = CDR_SAFE (value); continue; }
	      let key_service_etc: Lisp_Object = CDR_SAFE (key);
	      let key_path_etc: Lisp_Object = CDR_SAFE (key_service_etc);
	      let key_path: Lisp_Object = CAR_SAFE (key_path_etc);
	      if ! path.is_null () && ! NILP (key_path)
		&& strcmp (path, SSDATA (key_path)) != 0
		{ value = CDR_SAFE (value); continue; }
	      let handler: Lisp_Object = CAR_SAFE (CDR_SAFE (key_path_etc));
	      if NILP (handler)
		{ value = CDR_SAFE (value); continue; }
	      if ! NILP (memq_no_quit (handler, called_handlers))
		{ value = CDR_SAFE (value); continue; }
	      called_handlers = Fcons (handler, called_handlers);

	      /* Store the event.  */
	      xd_store_event (handler, args, event_args);
	      value = CDR_SAFE (value);
	    }
	}
    }

    /* Monitor.  */
    /* monitor:  */
    /* Search for a registered function of the message.  */
    key = list2 (QCmonitor (), bus);
    value = Fgethash (key, Vdbus_registered_objects_table, Qnil ());

    /* There shall be exactly one entry.  Construct an event.  */
    if NILP (value)
      { break 'cleanup; }

    /* Store the event.  */
    xd_store_event (CAR_SAFE (CDR_SAFE (CDR_SAFE (CDR_SAFE (CAR_SAFE (value))))),
		    args, event_args);
  }
  /* Cleanup.  */
  /* cleanup:  */
  dbus_message_unref (dmessage);
}

/* C 1952-1953: `/* Read queued incoming messages of the D-Bus BUS.  BUS is either a Lisp symbol,
   :system, :session, :system-private or :session-private, or a string denoting the bus address.
   */`  */
/// C 1953-1967: `static Lisp_Object xd_read_message (Lisp_Object bus)`.  C passes it to
/// `internal_catch` (C 1990), so the item is `unsafe extern "C" fn`.
#[cfg (feature = "have_dbus")]
unsafe extern "C" fn xd_read_message (bus: Lisp_Object) -> Lisp_Object
{
  let __func__: *const c_char = c"xd_read_message".as_ptr ();
  /* Retrieve bus address.  */
  let connection: *mut DBusConnection = xd_get_connection_address (bus);

  /* Non blocking read of the next available message.  */
  dbus_connection_read_write (connection, 0);

  while dbus_connection_get_dispatch_status (connection) != DBUS_DISPATCH_COMPLETE
    { xd_read_message_1 (connection, bus); }
  return Qnil ();
}

/* C 1968: `/* Callback called when something is ready to read or write.  */`  */
/// C 1969-1995: `static void xd_read_queued_messages (int fd, void *data)`.  C's prototype at
/// C 1063 came before it (Deviation 4); C passes it to `add_write_fd`/`add_read_fd` (C 1074,
/// 1076), so the item is `unsafe extern "C" fn`.
#[cfg (feature = "have_dbus")]
unsafe extern "C" fn xd_read_queued_messages (fd: c_int, data: *mut c_void)
{
  let mut busp: Lisp_Object = xd_registered_buses;
  let mut bus: Lisp_Object = Qnil ();
  let mut key: Lisp_Object;

  /* Find bus related to fd.  */
  if ! data.is_null ()
    {
      while ! NILP (busp)
	{
	  key = CAR_SAFE (CAR_SAFE (busp));
	  if (XD_KEYWORDP! (key) && (XSYMBOL (key) as *mut c_void) == data)
	    || (STRINGP (key) && (XSTRING (key) as *mut c_void) == data)
	    { bus = key; }
	  busp = CDR_SAFE (busp);
	}
    }

  if NILP (bus)
    { return; }

  /* We ignore all Lisp errors during the call.  */
  xd_in_read_queued_messages = true;
  internal_catch (Qdbus_error (), Some (xd_read_message), bus);
  xd_in_read_queued_messages = false;
}

/* C 1996-1997: blank and `void init_dbusbind (void)`.  */
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_dbusbind ()
{
  let __func__: *const c_char = c"init_dbusbind".as_ptr ();
  /* We do not want to abort.  */
  xputenv (c"DBUS_FATAL_WARNINGS=0".as_ptr ());
}

/// C 2004-2009: `static void syms_of_dbusbind_for_pdumper (void)` - C passes it to
/// `pdumper_do_now_and_after_load` (C 2185), so the item is `unsafe extern "C" fn`.
#[cfg (feature = "have_dbus")]
unsafe extern "C" fn syms_of_dbusbind_for_pdumper ()
{
  xd_registered_buses = Qnil ();
  xd_registered_fds = Qnil ();
}

/* The storage this unit owns (docs/status.md 3.29): C's `DEFVAR_LISP` names a field of the single
   `globals` object (`lisp.h` 3495-3530), whose declaration the port mirrors in
   `include/globals.rs`'s `variables` module; the unit whose `syms_of_dbusbind` registers the
   variable defines the object here, and its address must be stable for the forwarding records.
   C zero-initialises the `globals` object, so the port's initialisers are `lisp_h_Qnil ()`; the
   values `syms_of_dbusbind` sets below are C's own.  */
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_compiled_version: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_debug: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_message_type_error: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_message_type_invalid: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_message_type_method_call: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_message_type_method_return: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_message_type_signal: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_registered_objects_table: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub static mut Vdbus_runtime_version: Lisp_Object = lisp_h_Qnil ();

/* C 2011-2188: `void syms_of_dbusbind (void)` - the unit's registration function.  */
#[cfg (feature = "have_dbus")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_dbusbind ()
{
  let __func__: *const c_char = c"syms_of_dbusbind".as_ptr ();
  defsubr (&raw mut Sdbus__init_bus);
  defsubr (&raw mut Sdbus_get_unique_name);
  defsubr (&raw mut Sdbus__fd_open);
  defsubr (&raw mut Sdbus__fd_close);
  defsubr (&raw mut Sdbus__registered_fds);

  DEFSYM! (Qdbus_message_internal, "dbus-message-internal");
  defsubr (&raw mut Sdbus_message_internal);

  /* D-Bus error symbol.  */
  DEFSYM! (Qdbus_error, "dbus-error");
  Fput (Qdbus_error (), Qerror_conditions (),
	list2 (Qdbus_error (), Qerror ()));
  Fput (Qdbus_error (), Qerror_message (),
	build_string (c"D-Bus error".as_ptr ()));
  DEFSYM! (QD_Bus, "D-Bus");

  /* Lisp symbols of the system and session buses.  */
  DEFSYM! (QCsystem, ":system");
  DEFSYM! (QCsession, ":session");
  DEFSYM! (QCsystem_private, ":system-private");
  DEFSYM! (QCsession_private, ":session-private");

  /* Lisp symbol for method call timeout.  */
  DEFSYM! (QCtimeout, ":timeout");

  /* Lisp symbol for method interactive authorization.  */
  DEFSYM! (QCauthorizable, ":authorizable");

  /* Lisp symbol for file descriptor kept.  */
  /* C 2044-2046: `#ifdef DBUS_TYPE_UNIX_FD` / `DEFSYM (QCkeep_fd, ":keep-fd");` / `#endif`.  */
  #[cfg (feature = "dbus_type_unix_fd")]
  DEFSYM! (QCkeep_fd, ":keep-fd");

  /* Lisp symbols of D-Bus types.  */
  DEFSYM! (QCbyte, ":byte");
  DEFSYM! (QCboolean, ":boolean");
  DEFSYM! (QCint16, ":int16");
  DEFSYM! (QCuint16, ":uint16");
  DEFSYM! (QCint32, ":int32");
  DEFSYM! (QCuint32, ":uint32");
  DEFSYM! (QCint64, ":int64");
  DEFSYM! (QCuint64, ":uint64");
  DEFSYM! (QCdouble, ":double");
  DEFSYM! (QCstring, ":string");
  DEFSYM! (QCobject_path, ":object-path");
  DEFSYM! (QCsignature, ":signature");
  /* C 2061-2063: `#ifdef DBUS_TYPE_UNIX_FD` / `DEFSYM (QCunix_fd, ":unix-fd");` / `#endif`.  */
  #[cfg (feature = "dbus_type_unix_fd")]
  DEFSYM! (QCunix_fd, ":unix-fd");
  DEFSYM! (QCarray, ":array");
  DEFSYM! (QCvariant, ":variant");
  DEFSYM! (QCstruct, ":struct");
  DEFSYM! (QCdict_entry, ":dict-entry");

  /* Lisp symbols of objects in `dbus-registered-objects-table'.
     `:property', which does exist there as well, is not declared here.  */
  DEFSYM! (QCserial, ":serial");
  DEFSYM! (QCmethod, ":method");
  DEFSYM! (QCsignal, ":signal");
  DEFSYM! (QCmonitor, ":monitor");

  /* Miscellaneous Lisp symbols.  */
  DEFSYM! (Qdbus_get_name_owner, "dbus-get-name-owner");
  DEFSYM! (Qdbus_call_method, "dbus-call-method");

  DEFVAR_LISP! (c"dbus-compiled-version", Vdbus_compiled_version,
		c"The version of D-Bus Emacs is compiled against.  ");
  /* C 2083-2087: `#ifdef DBUS_VERSION_STRING` / `Vdbus_compiled_version = build_string
     (DBUS_VERSION_STRING);` / `#else` / `Vdbus_compiled_version = Qnil;` / `#endif`.  */
  #[cfg (feature = "dbus_version_string")]
  { Vdbus_compiled_version = build_string (DBUS_VERSION_STRING); }
  #[cfg (not (feature = "dbus_version_string"))]
  { Vdbus_compiled_version = Qnil (); }

  DEFVAR_LISP! (c"dbus-runtime-version", Vdbus_runtime_version,
		c"The version of D-Bus Emacs runs with.  ");
  {
    /* C 2093-2100: `#ifdef DBUS_VERSION` / `int major, minor, micro; dbus_get_version
       (&major, &minor, &micro); Vdbus_runtime_version = make_formatted_string ("%d.%d.%d",
       major, minor, micro);` / `#else` / `Vdbus_runtime_version = Qnil;` / `#endif`.  */
    #[cfg (feature = "dbus_version")]
    {
      let mut major: c_int = 0;
      let mut minor: c_int = 0;
      let mut micro: c_int = 0;
      dbus_get_version (&raw mut major, &raw mut minor, &raw mut micro);
      Vdbus_runtime_version
	= make_formatted_string (c"%d.%d.%d".as_ptr (), major, minor, micro);
    }
    #[cfg (not (feature = "dbus_version"))]
    { Vdbus_runtime_version = Qnil (); }
  }

  DEFVAR_LISP! (c"dbus-message-type-invalid", Vdbus_message_type_invalid,
		c"This value is never a valid message type.  ");
  Vdbus_message_type_invalid = make_fixnum (DBUS_MESSAGE_TYPE_INVALID as EMACS_INT);

  DEFVAR_LISP! (c"dbus-message-type-method-call", Vdbus_message_type_method_call,
		c"Message type of a method call message.  ");
  Vdbus_message_type_method_call = make_fixnum (DBUS_MESSAGE_TYPE_METHOD_CALL as EMACS_INT);

  DEFVAR_LISP! (c"dbus-message-type-method-return", Vdbus_message_type_method_return,
		c"Message type of a method return message.  ");
  Vdbus_message_type_method_return
    = make_fixnum (DBUS_MESSAGE_TYPE_METHOD_RETURN as EMACS_INT);

  DEFVAR_LISP! (c"dbus-message-type-error", Vdbus_message_type_error,
		c"Message type of an error reply message.  ");
  Vdbus_message_type_error = make_fixnum (DBUS_MESSAGE_TYPE_ERROR as EMACS_INT);

  DEFVAR_LISP! (c"dbus-message-type-signal", Vdbus_message_type_signal,
		c"Message type of a signal message.  ");
  Vdbus_message_type_signal = make_fixnum (DBUS_MESSAGE_TYPE_SIGNAL as EMACS_INT);

  DEFVAR_LISP! (c"dbus-registered-objects-table", Vdbus_registered_objects_table,
		c"Hash table of registered functions for D-Bus.\n\nThere are two different uses of the hash table: for accessing\nregistered interfaces properties, targeted by signals, method calls or\nmonitors, and for calling handlers in case of non-blocking method call\nreturns.\n\nIn the first case, the key in the hash table is the list (TYPE BUS\n[INTERFACE MEMBER]).  TYPE is one of the Lisp symbols `:method',\n`:signal', `:property' or `:monitor'.  BUS is either a Lisp symbol,\n`:system', `:session', `:system-private' or `:session-private', or a\nstring denoting the bus address.  INTERFACE is a string which denotes\na D-Bus interface, and MEMBER, also a string, is either a method, a\nsignal or a property INTERFACE is offering.  All arguments can be nil.\n\nThe value in the hash table is a list of quadruple lists ((UNAME\nSERVICE PATH OBJECT [RULE]) ...).  SERVICE is the service name as\nregistered, UNAME is the corresponding unique name.  In case of\nregistered methods, properties and monitors, UNAME is nil.  PATH is\nthe object path of the sending object.  All of them can be nil, which\nmeans a wildcard then.\n\nOBJECT is either the handler to be called when a D-Bus message, which\nmatches the key criteria, arrives (TYPE `:method', `:signal' and\n`:monitor'), or a list (ACCESS EMITS-SIGNAL VALUE) for TYPE `:property'.\nFor type `:message', the handler slot can also be a cons cell (HANDLER\n. ERROR-HANDLER) or (HANDLER . KEEP-FD-PATH).\n\nFor entries of type `:signal' or `:monitor', there is also a fifth\nelement RULE, which keeps the match string the signal or monitor is\nregistered with.\n\nIn the second case, the key in the hash table is the list (:serial BUS\nSERIAL).  BUS is either a Lisp symbol, `:system' or `:session', or a\nstring denoting the bus address.  SERIAL is the serial number of the\nnon-blocking method call, a reply is expected.  Both arguments must\nnot be nil.  The value in the hash table is HANDLER, the function to\nbe called when the D-Bus reply message arrives.  ");
  Vdbus_registered_objects_table = CALLN! (Fmake_hash_table, QCtest (), Qequal ());

  DEFVAR_LISP! (c"dbus-debug", Vdbus_debug,
		c"If non-nil, debug messages of D-Bus bindings are raised.  ");
  /* C 2173-2180: `#ifdef DBUS_DEBUG` / `Vdbus_debug = Qt;` and the `DBUS_VERBOSE=1` note /
     `#else` / `Vdbus_debug = Qnil;` / `#endif`.  */
  #[cfg (feature = "dbus_debug")]
  {
    Vdbus_debug = Qt ();
    /* We can also set environment variable DBUS_VERBOSE=1 in order to see more traces.  This
       requires libdbus-1 to be configured with --enable-verbose-mode.  */
  }
  #[cfg (not (feature = "dbus_debug"))]
  { Vdbus_debug = Qnil (); }

  /* Initialize internal objects.  */
  pdumper_do_now_and_after_load (Some (syms_of_dbusbind_for_pdumper));
  staticpro (&raw const xd_registered_buses);
  staticpro (&raw const xd_registered_fds);

  Fprovide (intern_c_string (c"dbusbind".as_ptr ()), Qnil ());
}
