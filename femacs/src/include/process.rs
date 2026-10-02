// femacs: port of src/process.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): the type and the five inlines the frozen configuration's
// live text carries.  `process.h` includes `lisp.h` and `coding.h`-adjacent headers by way of
// its users; its own live text needs only `Lisp_Object`, `pid_t` and the vectorlike header.
// The `HAVE_GNUTLS` and `HAVE_GETADDRINFO_A` members are dead on this target (both are
// `#undef` in the frozen `config.h`), and `tools/check-cond-eval.py` reports zero
// disagreements for the header, so the ledger's branch set is the frozen one.  The
// `ChannelMask`/inline layer lands here; the prototypes in
// docs/ports/include-process.md arrive with process.c (batch D).
//
// C's `#include "gnutls.h"`/`<gnutls/…>` are inside those dead branches, which is why this
// mirror names no gnutls type.

// process.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_PROCESS_H: bool = true;

// process.h 37: C spells it as an anonymous enum, so the port declares the same constant.
pub const PROCESS_OPEN_FDS: c_int = 6;

/* Most code should use these functions to set Lisp fields in struct process.  */
// process.h 238: `#define ChannelMask(n) (1 << (n))`.
#[inline(always)]
pub fn ChannelMask (n: c_int) -> c_int { 1 << n }

// process.h 39-190: the process pseudovector.  The layout is measured: 272 bytes, alignment
// 8 — 19 Lisp_Objects from offset 8, `pid` at 160, `open_fd[6]` at 180, the two `EMACS_INT`
// ticks at 208/216, the first bitfield unit at 232 and the second at 248 (see below).
#[repr(C)]
pub struct Lisp_Process
{
  pub header: vectorlike_header,                        // process.h 41

  /* Name of subprocess terminal.  */
  pub tty_name: Lisp_Object,                            // 44
  /* Name of this process.  */
  pub name: Lisp_Object,
  /* List of command arguments that this process was run with.  */
  pub command: Lisp_Object,
  /* (funcall FILTER PROC STRING) to dispose of a bunch of chars at once.  */
  pub filter: Lisp_Object,
  /* (funcall SENTINEL PROCESS) when process state changes.  */
  pub sentinel: Lisp_Object,
  /* (funcall LOG SERVER CLIENT MESSAGE) when a server accepts a connection.  */
  pub log: Lisp_Object,
  /* Buffer that output is going to.  */
  pub buffer: Lisp_Object,
  pub childp: Lisp_Object,
  /* Plist for programs to keep per-process state information.  */
  pub plist: Lisp_Object,
  /* Symbol indicating the type of process: real, network, serial.  */
  pub r#type: Lisp_Object,                              // C spells it `type`
  /* Marker set to end of last buffer-inserted output from this process.  */
  pub mark: Lisp_Object,
  pub status: Lisp_Object,
  /* Coding-system for decoding the input from this process.  */
  pub decode_coding_system: Lisp_Object,
  /* Working buffer for decoding.  */
  pub decoding_buf: Lisp_Object,
  /* Coding-system for encoding the output to this process.  */
  pub encode_coding_system: Lisp_Object,
  /* Working buffer for encoding.  */
  pub encoding_buf: Lisp_Object,
  /* Queue for storing waiting writes.  */
  pub write_queue: Lisp_Object,
  /* Pipe process attached to the standard error of this process.  */
  pub stderrproc: Lisp_Object,
  /* The thread a process is linked to, or nil for any thread.  */
  pub thread: Lisp_Object,                              // 152

  /* Process ID.  */
  pub pid: pid_t,                                       // 160
  /* File descriptor for the process.  */
  pub infd: c_int,                                      // 164
  /* Bytes read since the last filtering.  */
  pub nbytes_read: uintmax_t,                           // 168
  pub outfd: c_int,                                     // 176
  /* The file descriptors opened for reading from the process.  */
  pub open_fd: [c_int; PROCESS_OPEN_FDS as usize],      // 180 (6 ints, 24 bytes)

  /* The process's update tick counters (EMACS_INT, so 8-aligned).  */
  pub tick: EMACS_INT,                                  // 208
  pub update_tick: EMACS_INT,                           // 216
  pub decoding_carryover: c_int,                        // 224
  pub read_output_delay: c_int,                         // 228

  // process.h 149-151: `unsigned int adaptive_read_buffering : 2` and
  // `bool_bf read_output_skip : 1` share a 4-byte unit at 232.  Measured with
  // adaptive_read_buffering 3 and read_output_skip 1: `07 00 00 00`; with only the skip
  // flag set: `04 00 00 00` — so the skip flag is bit 2 (PORTING.md 1.3).
  pub bits1: u32,                                       // 232

  /* After a change in a process's output, wait until this many bytes are read.  */
  pub readmax: ptrdiff_t,                               // 240

  // process.h 152-164: eight `bool_bf : 1` members in a 4-byte unit at 248 — all eight set
  // measures `ff 00 00 00`.
  pub bits2: u32,                                       // 248

  pub raw_status: c_int,                                // 166
  pub backlog: c_int,                                   // 187
  pub port: c_int,                                      // 188
  pub socktype: c_int,                                  // 189
}

// The two bitfield units' members, named the way `glyph_slice`'s are (PORTING.md 1.3).  C
// reads and writes these as bitfields; the port masks `bits1`/`bits2`.
pub const PROCESS_ADAPTIVE_READ_BUFFERING_SHIFT: u32 = 0;
pub const PROCESS_ADAPTIVE_READ_BUFFERING_MASK: u32 = 0x03;   // 2 bits
pub const PROCESS_READ_OUTPUT_SKIP_SHIFT: u32 = 2;
pub const PROCESS_KILL_WITHOUT_QUERY_SHIFT: u32 = 0;          // bits2
pub const PROCESS_PTY_IN_SHIFT: u32 = 1;
pub const PROCESS_PTY_OUT_SHIFT: u32 = 2;
pub const PROCESS_INHERIT_CODING_SYSTEM_FLAG_SHIFT: u32 = 3;
pub const PROCESS_ALIVE_SHIFT: u32 = 4;
pub const PROCESS_RAW_STATUS_NEW_SHIFT: u32 = 5;
pub const PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT: u32 = 6;
pub const PROCESS_IS_SERVER_SHIFT: u32 = 7;

// process.h 217-232.
#[inline(always)]
pub fn PROCESSP (a: Lisp_Object) -> bool { PSEUDOVECTORP (a, pvec_type::PVEC_PROCESS) }

#[inline(always)]
pub fn CHECK_PROCESS (x: Lisp_Object) { CHECK_TYPE (PROCESSP (x), Qprocessp (), x) }

#[inline(always)]
pub fn XPROCESS (a: Lisp_Object) -> *mut Lisp_Process
{
  eassert! (PROCESSP (a));
  XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_Process)
}

// process.h 244-254: the setters C's code uses for the two Lisp fields GC cares about.
#[inline(always)]
pub fn pset_childp (p: *mut Lisp_Process, val: Lisp_Object) { unsafe { (*p).childp = val; } }

#[inline(always)]
pub fn pset_status (p: *mut Lisp_Process, val: Lisp_Object) { unsafe { (*p).status = val; } }

/* True means don't run process sentinels.  This is used when exiting.  */
unsafe extern "C" { pub static mut inhibit_sentinels: bool; }              // process.h 265
/* True means that Glib clobbers Emacs SIGCHLD handler.  */
unsafe extern "C" { pub static mut glib_installs_sigchld_handler: bool; }  // process.h 267

/* Exit statuses for GNU programs that exec other programs.  */
// process.h 270-275.
pub const EXIT_CANCELED: c_int = 125;      // internal error prior to exec attempt
pub const EXIT_CANNOT_INVOKE: c_int = 126; // program located, but not usable
pub const EXIT_ENOENT: c_int = 127;        // could not find program to exec

// C: process.h 277-320's prototypes and the `fd_callback` typedef — (not mirrored yet; they
// land with process.c/sysdep.c, batch D).
