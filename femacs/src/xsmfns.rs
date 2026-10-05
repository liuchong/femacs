// femacs: port of src/xsmfns.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow.  The non-`static`
// C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers stay plain
// `unsafe fn`, except the ten whose *address* C stores in a callback slot (`x_session_check_input`
// in `add_non_keyboard_read_fd`'s `fd_callback`, the four `SmcCallbacks` members, the two SM/ICE
// error handlers and `ice_conn_watch_CB` as the ICE connection watch), which are
// `unsafe extern "C" fn` - the `androidfns.rs` `do_unwind_create_frame` shape (PORTING.md 0.1-6).
//
// **This file is P-86's class: the frozen configuration compiles none of it.**  `xsmfns.o` is a
// member of `MSDOS_OBJ`-style platform variables only through `configure.ac`'s
// `AC_DEFINE([HAVE_X_SM])` (emacs/src/Makefile.in's `XSMFNS_OBJ`-carrying list is empty on this
// target, and `HAVE_X_SM` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1122), so
// `docs/baseline/frozen-build-sources.txt` lists no object for the file, `extract-abi.py`'s
// checklist for it is empty (P-86 ②), and what a port of it needs is P-86 ②'s substitute: the C
// line-range table, the `## Evidence` readings and the `## Deviations` of `docs/ports/xsmfns.md`.
// The **file gate is C 23's `#ifdef HAVE_X_SM`**, spelled `feature = "have_x_sm"`, on every item
// *and on C 25-39's include graph* (P-86 ⑧: an include outside the gate fails the frozen build at
// once, and the object must stay the 336-byte empty one).  Both arms of the file's nested `#if`s
// are carried (C 41-43/474-480's `USE_GTK` pair, C 365-389's `#ifndef USE_GTK`, C 527-531's
// `#if false`); the two `#if`s whose arms have no Rust spelling are recorded in the ledger.
//
// The X11 and XSMP surfaces this file needs are named by `docs/ports/xsmfns.md`'s `## Owed`: the
// four Xlib entry points and the one atom `include/xlib.rs` (the X11 SDK's mirror, P-90 ②) does not
// carry yet, `<X11/Xutil.h>`'s record, and the whole XSMP/ICE surface, which has no header on this
// machine.  Names a mirror does not carry are declared at the use site with C's own header line in
// a `/// C:` comment (PORTING.md P-72 ③/P-55, the `src/xfns.rs` 236 shape); the XSMP segment is
// P-88 ①'s in-unit stand-in and is **UNVERIFIED**.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn,
         unused_assignments, unused_mut, unused_parens, unreachable_code, unused_comparisons,
         unused_must_use, clippy::all)]

include!("include/config.rs");                    // C 21: `#include <config.h>` (every unit)

// C 25: `#include <X11/SM/SMlib.h>` - the header this section stands for.
/* C 25's `<X11/SM/SMlib.h>`: the X session-management client library, with the ICE protocol
   library it is built on (C 109-114's `IceProcessMessages`/`IceCloseConnection`, C 327-333's
   `IcePointer`, C 349-361's `IceConnectionNumber`/`IceAddConnectionWatch`).

   **Neither header is on this machine**: `ls /opt/homebrew/include/X11/` has no `SM/` and no
   `ICE/` directory and no `SMlib.h`/`ICElib.h` exists under `/opt`, `/usr/local` or `/Library`
   (the two `find` commands are in `docs/ports/xsmfns.md` `## Evidence` ②), and CTests against a
   real SDK stay owed.  Every declaration below is therefore written from the protocol's published
   interface and the whole section is **UNVERIFIED** (PORTING.md P-88 ①/P-90 ④: where a layout or a
   value cannot be measured it is named as unverified, never guessed as measured).  What *is*
   measured is the Rust side's own side of the contract: the three records' sizes/offsets are
   asserted against a restatement of C's declaration compiled by `clang` on this machine, which
   proves the `#[repr (C)]` records match the shape stated here and nothing about the header
   itself.  */
/// C: SMlib.h: `typedef struct _SmcConn *SmcConn;`
#[cfg (feature = "have_x_sm")]
#[repr (C)]
pub struct _SmcConn
{
  _private: [u8; 0],
}
/// C: ICElib.h: `typedef struct _IceConn *IceConn;`
#[cfg (feature = "have_x_sm")]
#[repr (C)]
pub struct _IceConn
{
  _private: [u8; 0],
}
/// C: ICElib.h: `typedef struct _IceReplyWaitInfo IceReplyWaitInfo;` - the reply record
/// `IceProcessMessages` waits on; C 109 passes `0` for it.
#[cfg (feature = "have_x_sm")]
#[repr (C)]
pub struct _IceReplyWaitInfo
{
  _private: [u8; 0],
}
#[cfg (feature = "have_x_sm")]
pub type SmcConn = *mut _SmcConn;
#[cfg (feature = "have_x_sm")]
pub type IceConn = *mut _IceConn;
#[cfg (feature = "have_x_sm")]
pub type IceReplyWaitInfo = _IceReplyWaitInfo;
/// C: SMlib.h: `typedef void *SmPointer;` - the argument type every SM callback's `clientData`
/// has.  Only stored and passed here, so the pointee is not observable (UNVERIFIED).
#[cfg (feature = "have_x_sm")]
pub type SmPointer = *mut c_void;
/// C: ICElib.h: `typedef char *IcePointer;` - ICE's counterpart (C 333's and C 351's).  Same
/// caveat as `SmPointer`.
#[cfg (feature = "have_x_sm")]
pub type IcePointer = *mut c_void;

/// C: SMlib.h: `typedef void (*SmcSaveYourselfProc) (SmcConn, SmPointer, int, Bool, int, Bool);`
/// - `SmcCallbacks.save_yourself.callback`'s type; C 439 stores `smc_save_yourself_CB` in it.
#[cfg (feature = "have_x_sm")]
pub type SmcSaveYourselfProc =
  Option<unsafe extern "C" fn (SmcConn, SmPointer, c_int, Bool, c_int, Bool)>;
/// C: SMlib.h: `typedef void (*SmcDieProc) (SmcConn, SmPointer);` (C 441's `smc_die_CB`).
#[cfg (feature = "have_x_sm")]
pub type SmcDieProc = Option<unsafe extern "C" fn (SmcConn, SmPointer)>;
/// C: SMlib.h: `typedef void (*SmcSaveCompleteProc) (SmcConn, SmPointer);` (C 443).
#[cfg (feature = "have_x_sm")]
pub type SmcSaveCompleteProc = Option<unsafe extern "C" fn (SmcConn, SmPointer)>;
/// C: SMlib.h: `typedef void (*SmcShutdownCancelledProc) (SmcConn, SmPointer);` (C 445).
#[cfg (feature = "have_x_sm")]
pub type SmcShutdownCancelledProc = Option<unsafe extern "C" fn (SmcConn, SmPointer)>;
/// C: SMlib.h: `typedef void (*SmcInteractProc) (SmcConn, SmPointer);` (C 277's argument).
#[cfg (feature = "have_x_sm")]
pub type SmcInteractProc = Option<unsafe extern "C" fn (SmcConn, SmPointer)>;
/// C: SMlib.h: `typedef void (*SmcErrorHandler) (SmcConn, Bool, int, unsigned long, int, int,
/// SmPointer);` (C 315-321's `smc_error_handler`, C 449's argument).
#[cfg (feature = "have_x_sm")]
pub type SmcErrorHandler =
  Option<unsafe extern "C" fn (SmcConn, Bool, c_int, c_ulong, c_int, c_int, SmPointer)>;
/// C: ICElib.h: `typedef void (*IceErrorHandler) (IceConn, Bool, int, unsigned long, int, int,
/// IcePointer);` (C 327-333's `ice_error_handler`, C 450's argument).
#[cfg (feature = "have_x_sm")]
pub type IceErrorHandler =
  Option<unsafe extern "C" fn (IceConn, Bool, c_int, c_ulong, c_int, c_int, IcePointer)>;
/// C: ICElib.h: `typedef void (*IceIOErrorHandler) (IceConn);` (C 340's `ice_io_error_handler`,
/// C 451's argument).
#[cfg (feature = "have_x_sm")]
pub type IceIOErrorHandler = Option<unsafe extern "C" fn (IceConn)>;
/// C: ICElib.h: `typedef void (*IceWatchProc) (IceConn, IcePointer, int, IcePointer *);` - the
/// connection watch C 351 spells `ice_conn_watch_CB` and C 454 installs.
#[cfg (feature = "have_x_sm")]
pub type IceWatchProc = Option<unsafe extern "C" fn (IceConn, IcePointer, c_int, *mut IcePointer)>;

/// C: SMlib.h: `#define SmcSaveYourselfProcMask (1L << 0)`
#[cfg (feature = "have_x_sm")]
pub const SmcSaveYourselfProcMask: c_ulong = 1 << 0;
/// C: SMlib.h: `#define SmcDieProcMask (1L << 1)`
#[cfg (feature = "have_x_sm")]
pub const SmcDieProcMask: c_ulong = 1 << 1;
/// C: SMlib.h: `#define SmcSaveCompleteProcMask (1L << 2)`
#[cfg (feature = "have_x_sm")]
pub const SmcSaveCompleteProcMask: c_ulong = 1 << 2;
/// C: SMlib.h: `#define SmcShutdownCancelledProcMask (1L << 3)`
#[cfg (feature = "have_x_sm")]
pub const SmcShutdownCancelledProcMask: c_ulong = 1 << 3;
/// C: SMlib.h: `#define SmCloneCommand "CloneCommand"` (C 179)
#[cfg (feature = "have_x_sm")]
pub const SmCloneCommand: &core::ffi::CStr = c"CloneCommand";
/// C: SMlib.h: `#define SmProgram "Program"` (C 189)
#[cfg (feature = "have_x_sm")]
pub const SmProgram: &core::ffi::CStr = c"Program";
/// C: SMlib.h: `#define SmUserID "UserID"` (C 199)
#[cfg (feature = "have_x_sm")]
pub const SmUserID: &core::ffi::CStr = c"UserID";
/// C: SMlib.h: `#define SmCurrentDirectory "CurrentDirectory"` (C 211)
#[cfg (feature = "have_x_sm")]
pub const SmCurrentDirectory: &core::ffi::CStr = c"CurrentDirectory";
/// C: SMlib.h: `#define SmRestartCommand "RestartCommand"` (C 223)
#[cfg (feature = "have_x_sm")]
pub const SmRestartCommand: &core::ffi::CStr = c"RestartCommand";
/// C: SMlib.h: `#define SmLISTofARRAY8 "LISTofARRAY8"` (C 180/224)
#[cfg (feature = "have_x_sm")]
pub const SmLISTofARRAY8: &core::ffi::CStr = c"LISTofARRAY8";
/// C: SMlib.h: `#define SmARRAY8 "ARRAY8"` (C 190/200/212)
#[cfg (feature = "have_x_sm")]
pub const SmARRAY8: &core::ffi::CStr = c"ARRAY8";
/// C: SMlib.h: `#define SmSaveLocal 1` - the `SmSaveBoth`/`SmSaveGlobal` sibling of C 276's test.
#[cfg (feature = "have_x_sm")]
pub const SmSaveLocal: c_int = 1;
/// C: SMlib.h: `#define SmInteractStyleAny 2` (C 274's test)
#[cfg (feature = "have_x_sm")]
pub const SmInteractStyleAny: c_int = 2;
/// C: SMlib.h: `#define SmDialogNormal 1` (C 277's argument)
#[cfg (feature = "have_x_sm")]
pub const SmDialogNormal: c_int = 1;
/// C: ICElib.h: `typedef enum { IceProcessMessagesSuccess, IceProcessMessagesIOError,
/// IceProcessMessagesConnectionClosed } IceProcessMessagesStatus;` - declaration order, so 0/1/2
/// (C 110 and C 113 compare against the first two).
#[cfg (feature = "have_x_sm")]
pub type IceProcessMessagesStatus = c_int;
#[cfg (feature = "have_x_sm")]
pub const IceProcessMessagesSuccess: IceProcessMessagesStatus = 0;
#[cfg (feature = "have_x_sm")]
pub const IceProcessMessagesIOError: IceProcessMessagesStatus = 1;

/// C: SMlib.h: `typedef struct { int length; SmPointer value; } SmPropValue;`
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmPropValue
{
  pub length: c_int,        // C: int length;
  pub value: SmPointer,     // C: SmPointer value;
}
/// C: SMlib.h: `typedef struct { char *name; char *type; int num_vals; SmPropValue *vals; } SmProp;`
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmProp
{
  pub name: *mut c_char,        // C: char *name;
  pub r#type: *mut c_char,      // C: char *type;
  pub num_vals: c_int,          // C: int num_vals;
  pub vals: *mut SmPropValue,   // C: SmPropValue *vals;
}
/// C: SMlib.h: the four `{ callback, client_data }` records `SmcCallbacks` is built from.
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmcSaveYourselfCallback
{
  pub callback: SmcSaveYourselfProc,
  pub client_data: SmPointer,
}
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmcDieCallback
{
  pub callback: SmcDieProc,
  pub client_data: SmPointer,
}
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmcSaveCompleteCallback
{
  pub callback: SmcSaveCompleteProc,
  pub client_data: SmPointer,
}
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmcShutdownCancelledCallback
{
  pub callback: SmcShutdownCancelledProc,
  pub client_data: SmPointer,
}
/// C: SMlib.h: `typedef struct { SmcSaveYourselfCallback save_yourself; SmcDieCallback die;
/// SmcSaveCompleteCallback save_complete; SmcShutdownCancelledCallback shutdown_cancelled; }
/// SmcCallbacks;` - the record C 400 declares and C 439-446 fills.
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct SmcCallbacks
{
  pub save_yourself: SmcSaveYourselfCallback,
  pub die: SmcDieCallback,
  pub save_complete: SmcSaveCompleteCallback,
  pub shutdown_cancelled: SmcShutdownCancelledCallback,
}
/* The three records' Rust side of the contract: the assertions are measured against a clang
   reading of the same declaration (`docs/ports/xsmfns.md` `## Evidence` ②), so a Rust layout that
   stopped matching the stated C shape fails the *frozen* build as well.  */
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::size_of::<SmPropValue> () == 16);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmPropValue, length) == 0);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmPropValue, value) == 8);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::size_of::<SmProp> () == 32);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmProp, name) == 0);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmProp, r#type) == 8);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmProp, num_vals) == 16);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmProp, vals) == 24);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::size_of::<SmcCallbacks> () == 64);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmcCallbacks, save_yourself) == 0);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmcCallbacks, die) == 16);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmcCallbacks, save_complete) == 32);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (SmcCallbacks, shutdown_cancelled) == 48);

/// C: SMlib.h: `SmcConn SmcOpenConnection (char *, SmPointer, int, int, unsigned long,
/// SmcCallbacks *, char *, char **, int, char *);` (C 459-468)
#[cfg (feature = "have_x_sm")]
unsafe extern "C" {
  pub fn SmcOpenConnection (network_ids: *mut c_char, context: SmPointer, xsmp_major_rev: c_int,
                            xsmp_minor_rev: c_int, mask: c_ulong, callbacks: *mut SmcCallbacks,
                            previous_id: *mut c_char, client_id_ret: *mut *mut c_char,
                            error_length: c_int, error_string_ret: *mut c_char) -> SmcConn;
  /// C: SMlib.h: `IceConn SmcGetIceConnection (SmcConn);` (C 109/114)
  pub fn SmcGetIceConnection (smc_conn: SmcConn) -> IceConn;
  /// C: SMlib.h: `void SmcSetProperties (SmcConn, int, SmProp **);` (C 260)
  pub fn SmcSetProperties (smc_conn: SmcConn, num_props: c_int, props: *mut *mut SmProp);
  /// C: SMlib.h: `Bool SmcInteractRequest (SmcConn, int, SmcInteractProc, SmPointer);` (C 277)
  pub fn SmcInteractRequest (smc_conn: SmcConn, dialog_type: c_int, interact_proc: SmcInteractProc,
                             client_data: SmPointer) -> Bool;
  /// C: SMlib.h: `void SmcInteractDone (SmcConn, Bool);` (C 516)
  pub fn SmcInteractDone (smc_conn: SmcConn, cancel_shutdown: Bool);
  /// C: SMlib.h: `void SmcSaveYourselfDone (SmcConn, Bool);` (C 280/517)
  pub fn SmcSaveYourselfDone (smc_conn: SmcConn, success: Bool);
  /// C: SMlib.h: `int SmcCloseConnection (SmcConn, int, char **);` (C 529, inside C 527's
  /// `#if false`)
  pub fn SmcCloseConnection (smc_conn: SmcConn, num_reasons: c_int,
                             reason_msgs: *mut *mut c_char) -> c_int;
  /// C: SMlib.h: `SmcErrorHandler SmcSetErrorHandler (SmcErrorHandler);` (C 449)
  pub fn SmcSetErrorHandler (handler: SmcErrorHandler) -> SmcErrorHandler;
  /// C: ICElib.h: `IceProcessMessagesStatus IceProcessMessages (IceConn, IceReplyWaitInfo *, Bool
  /// *);` (C 109)
  pub fn IceProcessMessages (ice_conn: IceConn, reply_wait: *mut IceReplyWaitInfo,
                             reply_ready_return: *mut Bool) -> IceProcessMessagesStatus;
  /// C: ICElib.h: `int IceCloseConnection (IceConn);` (C 114)
  pub fn IceCloseConnection (ice_conn: IceConn) -> c_int;
  /// C: ICElib.h: `int IceConnectionNumber (IceConn);` (C 359)
  pub fn IceConnectionNumber (ice_conn: IceConn) -> c_int;
  /// C: ICElib.h: `IceErrorHandler IceSetErrorHandler (IceErrorHandler);` (C 450)
  pub fn IceSetErrorHandler (handler: IceErrorHandler) -> IceErrorHandler;
  /// C: ICElib.h: `IceIOErrorHandler IceSetIOErrorHandler (IceIOErrorHandler);` (C 451)
  pub fn IceSetIOErrorHandler (handler: IceIOErrorHandler) -> IceIOErrorHandler;
  /// C: ICElib.h: `int IceAddConnectionWatch (IceWatchProc, IcePointer);` (C 454)
  pub fn IceAddConnectionWatch (watch_proc: IceWatchProc, client_data: IcePointer) -> c_int;
}

#[cfg (feature = "have_x_sm")]
include!("include/xlib.rs");                      // C 26: `#include <X11/Xlib.h>` (P-90 ②)

/* C 26's `<X11/Xlib.h>` has its home in `include/xlib.rs`; the four entry points and the one atom
   this file names are not in it yet, so they stand at the use site with the header line each was
   read from (P-72 ③, the `src/xfns.rs` 236 shape).  `include/xlib.rs` is where they belong, and
   `docs/ports/xsmfns.md`'s `## Owed` names it.  All five are **measured on this machine** (libX11
   + xorgproto under `/opt/homebrew/include`, `## Evidence` ②).  */
/// C: Xlib.h 1617-1627: `extern Window XCreateSimpleWindow (Display *, Window, int, int, unsigned
/// int, unsigned int, unsigned int, unsigned long, unsigned long);` (C 372-375)
#[cfg (feature = "have_x_sm")]
unsafe extern "C" {
  pub fn XCreateSimpleWindow (display: *mut Display, parent: Window, x: c_int, y: c_int,
                              width: c_uint, height: c_uint, border_width: c_uint,
                              border: c_ulong, background: c_ulong) -> Window;
  /// C: Xlib.h 3405-3409: `extern int XStoreName (Display *, Window, _Xconst char *);` (C 381)
  pub fn XStoreName (display: *mut Display, w: Window, window_name: *const c_char) -> c_int;
  /// C: Xlib.h 2078-2087: `extern int XChangeProperty (Display *, Window, Atom, Atom, int, int,
  /// _Xconst unsigned char *, int);` (C 383-385)
  pub fn XChangeProperty (display: *mut Display, w: Window, property: Atom, r#type: Atom,
                          format: c_int, mode: c_int, data: *const c_uchar,
                          nelements: c_int) -> c_int;
}
/// C: Xatom.h 39: `#define XA_STRING ((Atom) 31)` - measured 31 (C 384, the property's type).
#[cfg (feature = "have_x_sm")]
pub const XA_STRING: Atom = 31;

// C 27: `#include <X11/Xutil.h>` - the header this section stands for.
/* C 27's `<X11/Xutil.h>`: the class-hint record and its setter.  `include/xlib.rs`'s heading names
   this header among the five it stands for, but only the *macros* of that list live in its
   `x11_constants` module (P-90 ②), so the record and the setter stand here (P-88 ①) and
   `docs/ports/xsmfns.md`'s `## Owed` names the mirror.  Measured on this machine (`## Evidence`
   ②): `sizeof (XClassHint) 16`, `res_name` at 0, `res_class` at 8.  */
/// C: Xutil.h 189-192: `typedef struct { char *res_name; char *res_class; } XClassHint;` (C 370)
#[cfg (feature = "have_x_sm")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct XClassHint
{
  pub res_name: *mut c_char,    // C: char *res_name;
  pub res_class: *mut c_char,   // C: char *res_class;
}
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::size_of::<XClassHint> () == 16);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (XClassHint, res_name) == 0);
#[cfg (feature = "have_x_sm")]
const _: () = core::assert! (core::mem::offset_of! (XClassHint, res_class) == 8);
/// C: Xutil.h 583-587: `extern int XSetClassHint (Display *, Window, XClassHint *);` (C 380)
#[cfg (feature = "have_x_sm")]
unsafe extern "C" {
  pub fn XSetClassHint (display: *mut Display, w: Window, class_hints: *mut XClassHint) -> c_int;
}

/// C: Xlib.h 79-80: `#define True 1` / `#define False 0` - `include/xterm.rs` 65/68 carries
/// the measured pair for this target at its top level (in scope here through C 37's edge),
/// so nothing is redeclared; the pair's home is `<X11/Xlib.h>`, i.e. `include/xlib.rs`
/// (`## Owed`).  C 372-375/384's `CopyFromParent`/`PropModeReplace` are `include/xlib.rs`'s
/// `x11_constants` (X.h 121/475), a module the mirror does not glob-import, so the function
/// that uses them takes them with its own `use` - the `src/widget.rs` 691 and
/// `src/emacsgtkfixed.rs` 439 shape.

#[cfg (feature = "have_x_sm")]
include!("../lib/include/unistd.in.rs");          // C 29: `#include <unistd.h>`
/* C 30: `#include <sys/param.h>` - the header has **no mirror in this tree** (`lib/getloadavg.rs`
   65-68 records the same gap for its two live macros) and this file takes no name from it, so its
   line is this comment and `docs/ports/xsmfns.md`'s `## Deviations` records it.  */
#[cfg (feature = "have_x_sm")]
include!("../lib/include/errno.rs");              // C 31: `#include <errno.h>` (`errno`, C 408)
#[cfg (feature = "have_x_sm")]
include!("../lib/include/stdio.rs");              // C 32: `#include <stdio.h>` (`fprintf`,
                                                  // `stderr`, C 407)
#[cfg (feature = "have_x_sm")]
include!("include/lisp.rs");                      // C 34: `#include "lisp.h"`
#[cfg (feature = "have_x_sm")]
include!("include/frame.rs");                     // C 35: `#include "frame.h"`
#[cfg (feature = "have_x_sm")]
include!("include/termhooks.rs");                 // C 36: `#include "termhooks.h"`
#[cfg (feature = "have_x_sm")]
include!("include/xterm.rs");                     // C 37: `#include "xterm.h"`
#[cfg (feature = "have_x_sm")]
include!("include/font.rs");                      // xterm.h's `struct font *font` field (the
                                                  // `struct font` definition is font.h's;
                                                  // the `src/xfns.rs` 80 edge)

/* C 37's `xterm.h` reaches `<X11/IntrinsicP.h>`/`<X11/CoreP.h>` (xterm.h 57-58) and
   `<X11/Xmu/Editres.h>` (62) under `USE_X_TOOLKIT`, and `struct x_display_info`'s
   `widget`/`column_widget`/`edit_widget`/`menubar_widget` fields are typed with what those
   bring (`include/xterm.rs` 730-737, on that same gate).  **Xt is not on this machine**
   (`ls /opt/homebrew/include/X11/` has no `Intrinsic.h`, `IntrinsicP.h`, `CoreP.h` or
   `Xmu/`), the reference tree has no header for it, and no mirror carries the surface, so
   `Widget` stands at the use site (P-88 ①) with the shape `src/widget.rs` 202-212 measured
   for this same surface; the pointee is Xt's own record, nothing here reads it, and the
   declaration is **UNVERIFIED** (`## Owed` names `include/gtkutil.rs`'s toolkit section as
   the home a machine with Xt would move it to).  */
/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetRec *Widget;` - the pointee is Xt's own
/// record, whose layout nothing here reads (the `include/xlib.rs` 881-883 `_XDisplay` shape).
#[cfg (all (feature = "have_x_sm", feature = "use_x_toolkit"))]
#[repr (C)]
pub struct WidgetRec
{
  _private: [u8; 0],
}
/// C: `<X11/Intrinsic.h>`: `typedef struct _WidgetRec *Widget;`
#[cfg (all (feature = "have_x_sm", feature = "use_x_toolkit"))]
pub type Widget = *mut WidgetRec;
#[cfg (feature = "have_x_sm")]
include!("include/process.rs");                   // C 38: `#include "process.h"`
#[cfg (feature = "have_x_sm")]
include!("include/keyboard.rs");                  // C 39: `#include "keyboard.h"`

/* Edges C 34-39's mirror chain reaches through other headers, spelled here the way
   `src/gfilenotify.rs` 44-52 spells its own (P-86 ⑧).  `termhooks.h` names systime.h's `Time`;
   lisp.h 30's `<string.h>` include is where this file's three string calls come from
   (`strlen`, `strcpy`, `stpcpy`), and lisp.h's `<stdckdint.h>` chain supplies the `ckd_*`
   family.  */
#[cfg (feature = "have_x_sm")]
include!("include/systime.rs");                   // termhooks.h's `Time`
#[cfg (feature = "have_x_sm")]
include!("include/character.rs");                // buffer.h/dispextern.h's `"character.h"` (its
                                                  // width helpers C 291/362/378 reach, the
                                                  // `xfns.rs` 23 edge)
#[cfg (feature = "have_x_sm")]
include!("../lib/include/string.rs");             // lisp.h 30's `<string.h>`
#[cfg (feature = "have_x_sm")]
include!("../lib/include/stdckdint.in.rs");       // lisp.h's `<stdckdint.h>` (`ckd_add`, C 226)

/* C 34-39's chain also reaches the two other units' entry points this file calls and the two
   `frame.h`/`lisp.h` prototypes no mirror carries yet (P-55/P-72 ③); the `## Owed` list names
   their mirrors.  */
/// C: frame.h 1873: `extern void validate_x_resource_name (void);` (C 377)
#[cfg (feature = "have_x_sm")]
unsafe extern "C" {
  pub fn validate_x_resource_name ();
  /// C: lisp.h 5284: `extern char *emacs_get_current_dir_name (void);` (C 207/404)
  pub fn emacs_get_current_dir_name () -> *mut c_char;
  /// C: lisp.h 5144: `extern char *emacs_strerror (int) ATTRIBUTE_RETURNS_NONNULL;` (C 408)
  pub fn emacs_strerror (error_number: c_int) -> *mut c_char;
}

/* termhooks.h 373-412's `enum event_kind kind : 16` shares `include/termhooks.rs`'s
   `input_event::bits` with `part : 16` (PORTING.md 1.3/P-80), so C 107/141/289's `event.kind = …`
   and C 121's read keep C's field name in a per-unit macro - the `src/gfilenotify.rs` 182-190
   shape, body-identical.  */
#[cfg (feature = "have_x_sm")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

/* C 41-43: `#if defined USE_GTK && !defined HAVE_GTK3` - GTK 2 spells the entry point
   `gdk_set_sm_client_id`; with GTK 3 the `x11` name is the header's own, so C's `#define` stays a
   macro and C 474-480's call site carries both spellings (P-86 ①).  Neither GTK nor GDK is on this
   machine (`docs/ports/include-gtk.md`), so the two prototypes are text-only and **UNVERIFIED**
   (P-90 ④); `include/gtk.rs` is their home (`## Owed`).  */
/// C: gtk/gtk.h: `void gdk_set_sm_client_id (const char *sm_client_id);` (GTK 2's name)
#[cfg (all (feature = "have_x_sm", feature = "use_gtk", not (feature = "have_gtk3")))]
unsafe extern "C" {
  pub fn gdk_set_sm_client_id (sm_client_id: *const c_char);
}
/// C: gdk/gdkx.h: `void gdk_x11_set_sm_client_id (const char *sm_client_id);` (GTK 3's name)
#[cfg (all (feature = "have_x_sm", feature = "use_gtk", feature = "have_gtk3"))]
unsafe extern "C" {
  pub fn gdk_x11_set_sm_client_id (sm_client_id: *const c_char);
}
#[cfg (all (feature = "have_x_sm", feature = "use_gtk", not (feature = "have_gtk3")))]
macro_rules! gdk_x11_set_sm_client_id {
  ($w:expr) => { gdk_set_sm_client_id ($w) };
}

/* C 45: `This is the event used when SAVE_SESSION_EVENT occurs.` and C 47: `static struct input_event emacs_event;` - a tentative definition, i.e. C's zero, which is
   `NO_EVENT`'s kind and the nil Lisp_Objects (PORTING.md 1.5; the `src/android.rs` 458 spelling).  */
#[cfg (feature = "have_x_sm")]
static mut emacs_event: input_event = unsafe { core::mem::zeroed () };

/* C 49-50: `The descriptor that we use to check for data from the session manager.` and C 51:
   `static int ice_fd = -1;` */
#[cfg (feature = "have_x_sm")]
static mut ice_fd: c_int = -1;

/* C 53-54: `A flag that says if we are in shutdown interactions or not.` and C 55:
   `static bool doing_interact;` */
#[cfg (feature = "have_x_sm")]
static mut doing_interact: bool = false;

/* C 57-58: `The session manager object for the session manager connection.` and C 59:
   `static SmcConn smc_conn;` */
#[cfg (feature = "have_x_sm")]
static mut smc_conn: SmcConn = core::ptr::null_mut ();

/* C 61-62: `The client session id for this session.` and C 63: `static char *client_id;` */
#[cfg (feature = "have_x_sm")]
static mut client_id: *mut c_char = core::ptr::null_mut ();

/* C 65-66: `The full path name to the Emacs program.` and C 67: `static char *emacs_program;` */
#[cfg (feature = "have_x_sm")]
static mut emacs_program: *mut c_char = core::ptr::null_mut ();

/* C 69-71: `The option we tell the session manager to start Emacs with when restarting Emacs.`
   `The client_id is appended.` and C 72: `#define SMID_OPT "--smid="` */
#[cfg (feature = "have_x_sm")]
const SMID_OPT: &core::ffi::CStr = c"--smid=";

/* C 75-77: `The option to start Emacs without the splash screen when restarting Emacs.` and C 78:
   `static char NOSPLASH_OPT[] = "--no-splash";` - NUL-terminated, twelve bytes; the port
   keeps C's array (`strlen` and the property value read it).  */
#[cfg (feature = "have_x_sm")]
static mut NOSPLASH_OPT: [c_char; 12] =
  [b'-' as c_char, b'-' as c_char, b'n' as c_char, b'o' as c_char, b'-' as c_char, b's' as c_char,
   b'p' as c_char, b'l' as c_char, b'a' as c_char, b's' as c_char, b'h' as c_char, 0];

/* C 80-81: `The option to make Emacs start in the given directory.` and C 82:
   `#define CHDIR_OPT "--chdir="` */
#[cfg (feature = "have_x_sm")]
const CHDIR_OPT: &core::ffi::CStr = c"--chdir=";

/* C 84-90: `ice_connection_closed`.  */
#[cfg (feature = "have_x_sm")]
unsafe fn ice_connection_closed ()
{
  if ice_fd >= 0
    { delete_read_fd (ice_fd); }
  ice_fd = -1;
}

/* C 93-94: `Handle any messages from the session manager.  If no connection is open to a session
   manager, just return.`  C 96-123: `x_session_check_input` - the `fd_callback` C 360 hands to
   `add_non_keyboard_read_fd`, so it is `unsafe extern "C" fn` (PORTING.md 0.1-6).  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn x_session_check_input (fd: c_int, data: *mut c_void)
{
  let mut ret: c_int;

  if ice_fd == -1 { return; }

  /* Reset this so wo can check kind after callbacks have been called by
     IceProcessMessages.  The smc_interact_CB sets the kind to
     SAVE_SESSION_EVENT, but we don't know beforehand if that callback
     will be called.  */
  input_event_bits! (kind, &raw mut emacs_event, NO_EVENT);

  ret = IceProcessMessages (SmcGetIceConnection (smc_conn), core::ptr::null_mut (),
                            core::ptr::null_mut ());
  if ret != IceProcessMessagesSuccess
    {
      /* Either IO error or Connection closed.  */
      if ret == IceProcessMessagesIOError
        { IceCloseConnection (SmcGetIceConnection (smc_conn)); }

      ice_connection_closed ();
    }

  /* Check if smc_interact_CB was called and we shall generate a
     SAVE_SESSION_EVENT.  */
  if input_event_bits! (kind, &raw mut emacs_event) != NO_EVENT
    { kbd_buffer_store_event (&raw mut emacs_event as *mut c_void); }
}

/* C 125: `Return true if we have a connection to a session manager.`  C 127-131:
   `x_session_have_connection` (xterm.h 1999's prototype; `include/xterm.rs` does not
   carry it yet - `## Owed`).  */
#[cfg (feature = "have_x_sm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_session_have_connection () -> bool
{
  return ice_fd != -1;
}

/* C 133-135: `This is called when the session manager says it is OK to interact with the user.
   Here we set the kind to SAVE_SESSION_EVENT so an event is generated.`
   `Then lisp code can interact with the user.`  C 137-143: `smc_interact_CB` - an `SmcInteractProc` (C 277 passes its address).  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn smc_interact_CB (smcConn: SmcConn, clientData: SmPointer)
{
  doing_interact = true;
  input_event_bits! (kind, &raw mut emacs_event, SAVE_SESSION_EVENT);
  emacs_event.arg = Qnil ();
}

/* C 145-151: `This is called when the session manager tells us to save ourselves.`
   `We set the required properties so the session manager can restart us,`
   `plus the current working directory property (not mandatory) so we`
   `are started in the correct directory.`
   `If this is a shutdown and we can request to interact with the user,`
   `we do so, because we don't know what the lisp code might do.`  C 153-282:
   `smc_save_yourself_CB` - the `SmcSaveYourselfProc` C 439 stores in the callbacks
   record.  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn smc_save_yourself_CB (smcConn: SmcConn, clientData: SmPointer, saveType: c_int,
                                           shutdown: Bool, interactStyle: c_int, fast: Bool)
{
  /* C 161: `#define NR_PROPS 5` */
  const NR_PROPS: usize = 5;

  let mut props: [*mut SmProp; NR_PROPS] = [core::ptr::null_mut (); NR_PROPS];
  let mut prop_ptr: [SmProp; NR_PROPS] = unsafe { core::mem::zeroed () };

  let mut values: [SmPropValue; 20] = unsafe { core::mem::zeroed () };
  let mut vp: *mut SmPropValue;
  let mut val_idx: c_int = 0;
  let mut vp_idx: c_int = 0;
  let mut props_idx: c_int = 0;
  /* C 169's `int i;` is indeterminate until C 226 writes it through `ckd_add`; the port needs a
     value at the declaration (PORTING.md 1.5).  */
  let mut i: c_int = 0;
  let mut smid_opt: *mut c_char;
  let mut chdir_opt: *mut c_char = core::ptr::null_mut ();
  let user_login_name: Lisp_Object = Fuser_login_name (Qnil ());

  /* Must have these.  */
  if !STRINGP (Vinvocation_name) || !STRINGP (user_login_name)
    { return; }

  /* C 177-185: `How to start a new instance of Emacs.`  */
  props[props_idx as usize] = &raw mut prop_ptr[props_idx as usize];
  (*props[props_idx as usize]).name = xstrdup (SmCloneCommand.as_ptr ());
  (*props[props_idx as usize]).r#type = xstrdup (SmLISTofARRAY8.as_ptr ());
  (*props[props_idx as usize]).num_vals = 1;
  /* `props[props_idx]->vals = &values[val_idx++];` - the index is the old `val_idx` and the
     increment follows the store, which the port writes as the two statements (the post-increment
     is a recorded deviation, the `src/pgtkterm.rs` 602 shape).  */
  (*props[props_idx as usize]).vals = &raw mut values[val_idx as usize];
  val_idx += 1;
  (*(*props[props_idx as usize]).vals.offset (0)).length = strlen (emacs_program) as c_int;
  (*(*props[props_idx as usize]).vals.offset (0)).value = emacs_program as *mut c_void;
  props_idx += 1;

  /* C 187-195: `The name of the program.`  */
  props[props_idx as usize] = &raw mut prop_ptr[props_idx as usize];
  (*props[props_idx as usize]).name = xstrdup (SmProgram.as_ptr ());
  (*props[props_idx as usize]).r#type = xstrdup (SmARRAY8.as_ptr ());
  (*props[props_idx as usize]).num_vals = 1;
  (*props[props_idx as usize]).vals = &raw mut values[val_idx as usize];
  val_idx += 1;
  (*(*props[props_idx as usize]).vals.offset (0)).length = SBYTES (Vinvocation_name) as c_int;
  (*(*props[props_idx as usize]).vals.offset (0)).value = SDATA (Vinvocation_name) as *mut c_void;
  props_idx += 1;

  /* C 197-205: `User id.`  */
  props[props_idx as usize] = &raw mut prop_ptr[props_idx as usize];
  (*props[props_idx as usize]).name = xstrdup (SmUserID.as_ptr ());
  (*props[props_idx as usize]).r#type = xstrdup (SmARRAY8.as_ptr ());
  (*props[props_idx as usize]).num_vals = 1;
  (*props[props_idx as usize]).vals = &raw mut values[val_idx as usize];
  val_idx += 1;
  (*(*props[props_idx as usize]).vals.offset (0)).length = SBYTES (user_login_name) as c_int;
  (*(*props[props_idx as usize]).vals.offset (0)).value = SDATA (user_login_name) as *mut c_void;
  props_idx += 1;

  /* C 207-218: the current-directory property, set only when the directory could be read.  */
  let cwd: *mut c_char = emacs_get_current_dir_name ();
  if !cwd.is_null ()
    {
      props[props_idx as usize] = &raw mut prop_ptr[props_idx as usize];
      (*props[props_idx as usize]).name = xstrdup (SmCurrentDirectory.as_ptr ());
      (*props[props_idx as usize]).r#type = xstrdup (SmARRAY8.as_ptr ());
      (*props[props_idx as usize]).num_vals = 1;
      (*props[props_idx as usize]).vals = &raw mut values[val_idx as usize];
      val_idx += 1;
      (*(*props[props_idx as usize]).vals.offset (0)).length = strlen (cwd) as c_int;
      (*(*props[props_idx as usize]).vals.offset (0)).value = cwd as *mut c_void;
      props_idx += 1;
    }

  /* C 221-232: `How to restart Emacs.` /
     `/path/to/emacs, --smid=xxx --no-splash --chdir=dir ...`  */
  props[props_idx as usize] = &raw mut prop_ptr[props_idx as usize];
  (*props[props_idx as usize]).name = xstrdup (SmRestartCommand.as_ptr ());
  (*props[props_idx as usize]).r#type = xstrdup (SmLISTofARRAY8.as_ptr ());
  /* C 226: `if (ckd_add (&i, initial_argc, 3)) memory_full_up ();` - the frozen `<stdckdint.h>`
     arm of the family is the `overflowing_*` pair with a write-back (P-15, the `ckd_add` row of
     `docs/ports/token-aliases.tsv`), so `i` and the overflow flag come out together.  */
  let overflow: bool;
  (i, overflow) = initial_argc.overflowing_add (3);
  if overflow
    { memory_full_up (); }
  (*props[props_idx as usize]).num_vals = i;
  vp = xnmalloc (i as isize, core::mem::size_of::<SmPropValue> () as isize) as *mut SmPropValue;
  (*props[props_idx as usize]).vals = vp;
  (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).length =
    strlen (emacs_program) as c_int;
  (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).value =
    emacs_program as *mut c_void;
  vp_idx += 1;

  smid_opt = xmalloc (SMID_OPT.to_bytes ().len () + strlen (client_id) + 1) as *mut c_char;
  strcpy (stpcpy (smid_opt, SMID_OPT.as_ptr ()), client_id);

  (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).length = strlen (smid_opt) as c_int;
  (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).value = smid_opt as *mut c_void;
  vp_idx += 1;

  (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).length =
    strlen (NOSPLASH_OPT.as_ptr ()) as c_int;
  (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).value =
    NOSPLASH_OPT.as_mut_ptr () as *mut c_void;
  vp_idx += 1;

  if !cwd.is_null ()
    {
      chdir_opt = xmalloc (CHDIR_OPT.to_bytes ().len () + strlen (cwd) + 1) as *mut c_char;
      strcpy (stpcpy (chdir_opt, CHDIR_OPT.as_ptr ()), cwd);

      (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).length =
        strlen (chdir_opt) as c_int;
      (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).value =
        chdir_opt as *mut c_void;
      vp_idx += 1;
    }

  i = 1;
  while i < initial_argc
    {
      (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).length =
        strlen (*initial_argv.offset (i as isize)) as c_int;
      (*(*props[props_idx as usize]).vals.offset (vp_idx as isize)).value =
        *initial_argv.offset (i as isize) as *mut c_void;
      vp_idx += 1;
      i += 1;
    }

  props_idx += 1;

  SmcSetProperties (smcConn, props_idx, props.as_mut_ptr ());

  xfree (smid_opt as *mut c_void);
  xfree (chdir_opt as *mut c_void);
  xfree (cwd as *mut c_void);
  xfree (vp as *mut c_void);

  i = 0;
  while i < props_idx
    {
      xfree ((*props[i as usize]).r#type as *mut c_void);
      xfree ((*props[i as usize]).name as *mut c_void);
      i += 1;
    }

  /* See if we maybe shall interact with the user.  */
  if interactStyle != SmInteractStyleAny
      || shutdown == 0
      || saveType == SmSaveLocal
      || SmcInteractRequest (smcConn, SmDialogNormal, Some (smc_interact_CB),
                             core::ptr::null_mut ()) == 0
    {
      /* No interaction, we are done saving ourself.  */
      SmcSaveYourselfDone (smcConn, True);
    }
}

/* C 284: `According to the SM specification, this shall close the connection.` and C 286-291:
   `smc_die_CB`.  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn smc_die_CB (smcConn: SmcConn, clientData: SmPointer)
{
  input_event_bits! (kind, &raw mut emacs_event, SAVE_SESSION_EVENT);
  emacs_event.arg = Qt ();
}

/* C 293-297: `We don't use the next two but they are mandatory, leave them empty.`
   `According to the SM specification, we should not interact with the`
   `user between smc_save_yourself_CB is called and until smc_save_complete_CB`
   `is called.  It seems like a lot of job to implement this and it doesn't`
   `even seem necessary.`  C 299-303 and C 305-309 are the two callbacks; C's bodies are the
   comment `/* Empty */` alone, so these bodies are empty as well - nothing is stubbed.  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn smc_save_complete_CB (smcConn: SmcConn, clientData: SmPointer)
{
  /* Empty */
}

#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn smc_shutdown_cancelled_CB (smcConn: SmcConn, clientData: SmPointer)
{
  /* Empty */
}

/* C 311-324: the SM error handler (`Error handlers for SM and ICE.  We don't want to exit Emacs
   just because there is some error in the session management.`); C's body is `/* Empty  */`.  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn smc_error_handler (smcConn: SmcConn, swap: Bool, offendingMinorOpcode: c_int,
                                        offendingSequence: c_ulong, errorClass: c_int,
                                        severity: c_int, values: SmPointer)
{
  /* Empty  */
}

/* C 326-336: the ICE error handler; C's body is `/* Empty  */`.  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn ice_error_handler (iceConn: IceConn, swap: Bool, offendingMinorOpcode: c_int,
                                        offendingSequence: c_ulong, errorClass: c_int,
                                        severity: c_int, values: IcePointer)
{
  /* Empty  */
}

/* C 339-344: `ice_io_error_handler`.  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn ice_io_error_handler (iceConn: IceConn)
{
  /* Connection probably gone.  */
  ice_connection_closed ();
}

/* C 346-361: `ice_conn_watch_CB` - the `IceWatchProc` C 454 installs
   (`This is called when the ICE connection is created or closed.  The SM library uses ICE as it
   transport protocol.`).  */
#[cfg (feature = "have_x_sm")]
unsafe extern "C" fn ice_conn_watch_CB (iceConn: IceConn, clientData: IcePointer, opening: c_int,
                                        watchData: *mut IcePointer)
{
  if opening == 0
    {
      ice_connection_closed ();
      return;
    }

  ice_fd = IceConnectionNumber (iceConn);
  add_non_keyboard_read_fd (ice_fd, Some (x_session_check_input), core::ptr::null_mut ());
}

/* C 363-389: `Create the client leader window.` - C's `#ifndef USE_GTK` (C 365), whose `#else`
   side is C 474-480's pair inside `x_session_initialize` (there is no second arm here).  */
#[cfg (all (feature = "have_x_sm", not (feature = "use_gtk")))]
unsafe fn create_client_leader_window (dpyinfo: *mut x_display_info, client_ID: *mut c_char)
{
  use x11_constants::{CopyFromParent, PropModeReplace};   // C 375/384's two X11 macros

  let mut w: Window;
  /* C 370 leaves the record indeterminate and fills both members at C 378-379; Rust needs a
     value before the first assignment (PORTING.md 1.5, the `src/androidfns.rs` 516 shape).  */
  let mut class_hints: XClassHint = unsafe { core::mem::zeroed () };

  w = XCreateSimpleWindow ((*dpyinfo).display, (*dpyinfo).root_window, -1, -1, 1, 1,
                           CopyFromParent as c_uint, CopyFromParent as c_ulong,
                           CopyFromParent as c_ulong);

  validate_x_resource_name ();
  class_hints.res_name = SSDATA (Vx_resource_name);
  class_hints.res_class = SSDATA (Vx_resource_class);
  XSetClassHint ((*dpyinfo).display, w, &raw mut class_hints);
  XStoreName ((*dpyinfo).display, w, class_hints.res_name);

  XChangeProperty ((*dpyinfo).display, w, (*dpyinfo).Xatom_SM_CLIENT_ID,
                   XA_STRING, 8, PropModeReplace,
                   client_ID as *const c_uchar, strlen (client_ID) as c_int);

  (*dpyinfo).client_leader_window = w;
}

/* C 392: `Try to open a connection to the session manager.`  C 394-482:
   `x_session_initialize` (xterm.h 1998's prototype; `## Owed`).  */
#[cfg (feature = "have_x_sm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_session_initialize (dpyinfo: *mut x_display_info)
{
  /* C 397: `#define SM_ERRORSTRING_LEN 512` */
  const SM_ERRORSTRING_LEN: usize = 512;
  /* C 398's `char errorstring[SM_ERRORSTRING_LEN];` is indeterminate in C; the buffer is only
     handed to `SmcOpenConnection`, which writes into it (PORTING.md 1.5).  */
  let mut errorstring: [c_char; SM_ERRORSTRING_LEN] = [0; SM_ERRORSTRING_LEN];
  let mut previous_id: *mut c_char = core::ptr::null_mut ();
  let mut callbacks: SmcCallbacks = core::mem::zeroed ();
  let mut name_len: ptrdiff_t = 0;

  /* libSM seems to crash if pwd is missing - see bug#18851.  */
  let pwd: *mut c_char = emacs_get_current_dir_name ();
  if pwd.is_null ()
    {
      fprintf (stderr, c"Disabling session management due to pwd error: %s\n".as_ptr (),
               /* C's `errno` is the macro `(*__error ())`; the port keeps the accessor
                  (`lib/include/errno.rs`), so the read is `*errno ()`.  */
               emacs_strerror (*errno ()));
      return;
    }
  xfree (pwd as *mut c_void);

  ice_fd = -1;
  doing_interact = false;

  /* Check if we where started by the session manager.  If so, we will
     have a previous id.  */
  if STRINGP (Vx_session_previous_id)
    { previous_id = SSDATA (Vx_session_previous_id); }

  /* Construct the path to the Emacs program.  */
  if STRINGP (Vinvocation_directory)
    { name_len += SBYTES (Vinvocation_directory); }
  if STRINGP (Vinvocation_name)
    { name_len += SBYTES (Vinvocation_name); }

  /* This malloc will not be freed, but it is only done once, and hopefully
     not very large   */
  emacs_program = xmalloc ((name_len + 1) as usize) as *mut c_char;
  let mut z: *mut c_char = emacs_program;

  if STRINGP (Vinvocation_directory)
    { z = lispstpcpy (z, Vinvocation_directory); }
  if STRINGP (Vinvocation_name)
    { lispstpcpy (z, Vinvocation_name); }

  /* The SM protocol says all callbacks are mandatory, so set up all
     here and in the mask passed to SmcOpenConnection.  */
  callbacks.save_yourself.callback = Some (smc_save_yourself_CB);
  callbacks.save_yourself.client_data = core::ptr::null_mut ();
  callbacks.die.callback = Some (smc_die_CB);
  callbacks.die.client_data = core::ptr::null_mut ();
  callbacks.save_complete.callback = Some (smc_save_complete_CB);
  callbacks.save_complete.client_data = core::ptr::null_mut ();
  callbacks.shutdown_cancelled.callback = Some (smc_shutdown_cancelled_CB);
  callbacks.shutdown_cancelled.client_data = core::ptr::null_mut ();

  /* Set error handlers.  */
  SmcSetErrorHandler (Some (smc_error_handler));
  IceSetErrorHandler (Some (ice_error_handler));
  IceSetIOErrorHandler (Some (ice_io_error_handler));

  /* Install callback for when connection status changes.  */
  IceAddConnectionWatch (Some (ice_conn_watch_CB), core::ptr::null_mut ());

  /* Open the connection to the session manager.  A failure is not
     critical, it usually means that no session manager is running.
     The errorstring is here for debugging.  */
  smc_conn = SmcOpenConnection (core::ptr::null_mut (), core::ptr::null_mut (), 1, 0,
                                (SmcSaveYourselfProcMask | SmcDieProcMask
                                 | SmcSaveCompleteProcMask | SmcShutdownCancelledProcMask),
                                &raw mut callbacks, previous_id, &raw mut client_id,
                                SM_ERRORSTRING_LEN as c_int, errorstring.as_mut_ptr ());

  if !smc_conn.is_null ()
    {
      Vx_session_id = build_string (client_id);

      #[cfg (feature = "use_gtk")]
      {
        /* GTK creates a leader window by itself, but we need to tell
           it about our client_id.  */
        /* C 41-43's `#define` renames this call in the GTK 2 arm (where the entry point is
           `gdk_set_sm_client_id`); both spellings stand here (P-86 ①).  */
        #[cfg (not (feature = "have_gtk3"))]
        { gdk_x11_set_sm_client_id! (client_id); }
        #[cfg (feature = "have_gtk3")]
        { gdk_x11_set_sm_client_id (client_id); }
      }
      #[cfg (not (feature = "use_gtk"))]
      { create_client_leader_window (dpyinfo, client_id); }
    }
}

/* C 484-490: `x_session_close` (`Ensure that the session manager is not contacted again.`;
   xterm.h 2000's prototype, `## Owed`).  */
#[cfg (feature = "have_x_sm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_session_close ()
{
  ice_connection_closed ();
}

/* C 493-535: `DEFUN ("handle-save-session", …)`.  C 527-531's `#if false` block is dead in every
   configuration (the `src/xfns.rs` 958 `#if false` precedent), so it is recorded in
   `docs/ports/xsmfns.md` and not transcribed - the ledger's line table names it.  */
#[cfg (feature = "have_x_sm")]
DEFUN! ("handle-save-session", Fhandle_save_session, Shandle_save_session, 1, 1, "e",
       doc: "Handle the save_yourself event from a session manager.\nA session manager can tell Emacs that the window system is shutting down\nby sending Emacs a save_yourself message.  Emacs executes this function when\nsuch an event occurs.  This function then executes `emacs-session-save'.\nAfter that, this function informs the session manager that it can continue\nor abort shutting down the window system depending on the return value\nfrom `emacs-session-save'  If the return value is non-nil the session manager\nis told to abort the window system shutdown.\n\nDo not call this function yourself.",
       (event: Lisp_Object),
       {
  let kill_emacs: bool = CONSP (event) && CONSP (XCDR (event))
                        && EQ (Qt (), XCAR (XCDR (event)));

  /* Check doing_interact so that we don't do anything if someone called
     this at the wrong time. */
  if doing_interact && ! kill_emacs
    {
      let cancel_shutdown: bool = ! NILP (call0 (Qemacs_session_save ()));

      SmcInteractDone (smc_conn, cancel_shutdown as Bool);
      SmcSaveYourselfDone (smc_conn, True);

      doing_interact = false;
    }
  else if kill_emacs
    {
      /* We should not do user interaction here, but it is not easy to
         prevent.  Fix this in next version.  */
      Fkill_emacs (Qnil (), Qnil ());
      /* C 527-531's `#if false` block follows here in C; see the comment above the DEFUN.  */
    }

  return Qnil ();
});

/* C 542-579: `syms_of_xsmfns`, with C's initialization banner above it
   (`/***********************************************************************             Initialization
   ***********************************************************************/`).
   The two variables C declares through `DEFVAR_LISP` live in `struct emacs_globals`, so the unit
   that owns them defines them (P-39's `tools/check-globals.py`): `include/globals.rs` declares
   both, and the definitions stand here in C's order.  */
#[cfg (feature = "have_x_sm")]
#[unsafe(no_mangle)]
pub static mut Vx_session_id: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_x_sm")]
#[unsafe(no_mangle)]
pub static mut Vx_session_previous_id: Lisp_Object = lisp_h_Qnil ();

#[cfg (feature = "have_x_sm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xsmfns ()
{
  DEFSYM! (Qemacs_session_save, "emacs-session-save");

  DEFVAR_LISP! (c"x-session-id", Vx_session_id,
    "The session id Emacs got from the session manager for this session.\nChanging the value does not change the session id used by Emacs.\nThe value is nil if no session manager is running.\nSee also `x-session-previous-id', `emacs-save-session-functions',\n`emacs-session-save' and `emacs-session-restore'.");
  Vx_session_id = Qnil ();

  DEFVAR_LISP! (c"x-session-previous-id", Vx_session_previous_id,
    "The previous session id Emacs got from session manager.\nIf Emacs is running on a window system that has a session manager, the\nsession manager gives Emacs a session id.  It is feasible for Emacs Lisp\ncode to use the session id to save configuration in, for example, a file\nwith a file name based on the session id.  If Emacs is running when the\nwindow system is shut down, the session manager remembers that Emacs was\nrunning and saves the session id Emacs had.\n\nWhen the window system is started again, the session manager restarts\nEmacs and hands Emacs the session id it had the last time it was\nrunning.  This is now the previous session id and the value of this\nvariable.  If configuration was saved in a file as stated above, the\nprevious session id shall be used to reconstruct the file name.\n\nThe session id Emacs has while it is running is in the variable\n`x-session-id'.  The value of this variable and `x-session-id' may be the\nsame, depending on how the session manager works.\n\nSee also `emacs-save-session-functions', `emacs-session-save' and\n`emacs-session-restore'.");
  Vx_session_previous_id = Qnil ();

  defsubr (&raw mut Shandle_save_session);
}
