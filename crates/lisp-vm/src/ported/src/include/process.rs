// femacs: port of src/process.h @ fd7afced403.
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
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_PROCESS_H: bool = true;

// The historical offsets below describe the disabled-TLS layout. Enabling TLS
// inserts its traced fields and native tail in the original positions.
#[cfg(feature = "have_gnutls")]
pub mod process_gnutls_types {
  pub use super::h_src_include_gnutls_types::*;
}
#[cfg(feature = "have_gnutls")]
pub use self::process_gnutls_types::*;

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
  #[cfg(feature = "have_gnutls")]
  pub gnutls_cred_type: Lisp_Object,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_boot_parameters: Lisp_Object,
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
  #[cfg(feature = "have_gnutls")]
  pub gnutls_initstage: gnutls_initstage_t,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_state: gnutls_session_t,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_x509_cred: gnutls_certificate_client_credentials,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_anon_cred: gnutls_anon_client_credentials_t,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_certificates: *mut gnutls_x509_crt_t,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_certificates_length: c_int,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_peer_verification: c_uint,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_extra_peer_verification: c_uint,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_log_level: c_int,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_handshakes_tried: c_int,
  #[cfg(feature = "have_gnutls")]
  pub gnutls_bits: u32,
}

#[cfg(feature = "have_gnutls")]
impl Lisp_Process {
  #[inline(always)]
  pub fn is_non_blocking_client(&self) -> bool {
    self.bits2 & (1 << PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT) != 0
  }
  #[inline(always)]
  pub fn gnutls_p(&self) -> bool { self.gnutls_bits & 1 != 0 }
  #[inline(always)]
  pub fn set_gnutls_p(&mut self, value: bool) {
    self.gnutls_bits = (self.gnutls_bits & !1) | u32::from(value);
  }
  #[inline(always)]
  pub fn gnutls_complete_negotiation_p(&self) -> bool { self.gnutls_bits & 2 != 0 }
  #[inline(always)]
  pub fn set_gnutls_complete_negotiation_p(&mut self, value: bool) {
    self.gnutls_bits = (self.gnutls_bits & !2) | (u32::from(value) << 1);
  }
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
pub fn pset_childp (p: *mut Lisp_Process, val: Lisp_Object) { /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p).childp = val; } }

#[inline(always)]
pub fn pset_status (p: *mut Lisp_Process, val: Lisp_Object) { unsafe { (*p).status = val; } }

#[cfg(feature = "have_gnutls")]
#[inline(always)]
pub fn pset_gnutls_cred_type(p: *mut Lisp_Process, val: Lisp_Object) {
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p).gnutls_cred_type = val; }
}

/* process.h 263-268: the two objects C's `src/process.c` defines (C 161 and C 7674).  They
   sit in `pub mod process_objs` + glob import so the unit's definitions shadow the
   declarations where a same-named declaration at this scope would be `E0428` (PORTING.md
   0.1-10/P-21, the `composite_objs`/`charset_objs` shape).  */
pub mod process_objs {
  use super::*;

  /* True means don't run process sentinels.  This is used when exiting.  */
  unsafe extern "C-unwind" { pub static inhibit_sentinels: VmGlobal<bool>; }              // process.h 265
  /* True means that Glib clobbers Emacs SIGCHLD handler.  */
  unsafe extern "C-unwind" { pub static glib_installs_sigchld_handler: VmGlobal<bool>; }  // process.h 267
}
pub use self::process_objs::*;

/* Exit statuses for GNU programs that exec other programs.  */
// process.h 270-275.
pub const EXIT_CANCELED: c_int = 125;      // internal error prior to exec attempt
pub const EXIT_CANNOT_INVOKE: c_int = 126; // program located, but not usable
pub const EXIT_ENOENT: c_int = 127;        // could not find program to exec

/* process.h 298-305's five read/write-descriptor entries.  C declares them *here*, not in
   keyboard.h (status 3.5 recorded them under keyboard.h, and the mirror carried them there
   until status 3.78).  They sit in `pub mod process_fd_decls` + glob import for the same
   reason `process_objs` does (PORTING.md 0.1-10/P-21): `src/process.c` *defines* both of the
   ones it uses, and an explicit item shadows a glob import where a same-named declaration at
   this scope would be `E0428`.

   `fd_callback` (process.h 296) is the *unit's* `pub type` (its definition needs the nullable
   form, PORTING.md 1.4), so the mirror does not repeat it - a second `type` item in the same
   scope would be the one thing 0.1-10's shadowing rule does not cover (it is textual, not a
   glob).  */
pub mod process_fd_decls {
  use super::*;

  unsafe extern "C-unwind" {
    /// C: process.h 298: `extern void add_read_fd (int fd, fd_callback func, void *data);`
    pub fn add_read_fd (fd: c_int, func: Option<unsafe extern "C-unwind" fn (c_int, *mut c_void)>,
                        data: *mut c_void);
    /// C: process.h 299: `extern void add_non_keyboard_read_fd (int fd, fd_callback func, void *data);`
    pub fn add_non_keyboard_read_fd (fd: c_int,
                                     func: Option<unsafe extern "C-unwind" fn (c_int, *mut c_void)>,
                                     data: *mut c_void);
    /// C: process.h 300: `extern void delete_read_fd (int fd);`
    pub fn delete_read_fd (fd: c_int);
    /// C: process.h 301: `extern void add_write_fd (int fd, fd_callback func, void *data);`
    pub fn add_write_fd (fd: c_int, func: Option<unsafe extern "C-unwind" fn (c_int, *mut c_void)>,
                         data: *mut c_void);
    /// C: process.h 302: `extern void delete_write_fd (int fd);`
    pub fn delete_write_fd (fd: c_int);
  }
}
pub use self::process_fd_decls::*;

// C: process.h 296-308's remaining prototypes and the `fd_callback` typedef — (not mirrored
// yet; they land with process.c/sysdep.c, batch D).

/* C 279-316: the prototypes this header declares for functions
   other units define.  They stand in a module + glob import so a unit that writes the same
   prototype at its use site shadows the import instead of colliding with it (P-297, the P-39
   shape); a prototype carries the gate of the records it names.  */
pub mod process_fn_decls {
  use super::*;

  unsafe extern "C-unwind" {
    /// C: process.h 279: `extern Lisp_Object get_current_directory (bool);`
    pub fn get_current_directory (encode: bool) -> Lisp_Object;
    /// C: process.h 280: `extern void record_kill_process (struct Lisp_Process *, Lisp_Object);`
    pub fn record_kill_process (p: *mut Lisp_Process, tempfile: Lisp_Object);
    /// C: process.h 284: `extern Lisp_Object list_system_processes (void);`
    pub fn list_system_processes () -> Lisp_Object;
    /// C: process.h 285: `extern Lisp_Object system_process_attributes (Lisp_Object);`
    pub fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object;
    /// C: process.h 292: `extern void hold_keyboard_input (void);`
    pub fn hold_keyboard_input ();
    /// C: process.h 293: `extern void unhold_keyboard_input (void);`
    pub fn unhold_keyboard_input ();
    /// C: process.h 294: `extern bool kbd_on_hold_p (void);`
    pub fn kbd_on_hold_p () -> bool;
    /// C: process.h 303: `extern void catch_child_signal (void);`
    pub fn catch_child_signal ();
    /// C: process.h 304: `extern void restore_nofile_limit (void);`
    pub fn restore_nofile_limit ();
    /// C: process.h 311: `extern Lisp_Object remove_slash_colon (Lisp_Object);`
    pub fn remove_slash_colon (string: Lisp_Object) -> Lisp_Object;
    /// C: process.h 313: `extern void update_processes_for_thread_death (Lisp_Object);`
    pub fn update_processes_for_thread_death (thread: Lisp_Object);
    /// C: process.h 314: `extern void dissociate_controlling_tty (void);`
    pub fn dissociate_controlling_tty ();
    /// C: process.h 316: `extern int open_channel_for_module (Lisp_Object);`
    pub fn open_channel_for_module (pipe_process: Lisp_Object) -> c_int;
  }
}
pub use self::process_fn_decls::*;
