// femacs: port of src/haiku_io.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// file-scope object, local), the same control flow and the same error model.
//
// **The frozen configuration compiles no `haiku_io.o`**: `build/emacs-ref/src/Makefile`'s platform
// object variables are empty for this target (`HAIKU_OBJ =`) and `docs/baseline/frozen-build-sources.txt`
// lists no such object.  PORTING.md P-86 governs the class: the whole body is transcribed as it stands
// (this file's previous revision was the P-08 note), the file gate `have_haiku` is carried on every
// item and on the include edges (P-86 ⑧), and the file has **no `#if` of its own**.  The Haiku SDK's
// `<OS.h>` is not on this machine, so the five port calls and the two values this file names are
// declared at their use sites with the standing `UNVERIFIED` mark (P-88 ①/②).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* The `c_*` names come from the `lisp.rs` mirror this unit includes (`include/lisp.rs` 14) - a unit
   that includes that mirror does not import them again, which would be `E0252` in one module.  */

#[cfg (feature = "have_haiku")]
include!("include/config.rs");              // C 20: `#include <config.h>`
#[cfg (feature = "have_haiku")]
include!("../lib/include/signal.rs");       // C 22: `#include <signal.h>`
#[cfg (feature = "have_haiku")]
include!("../lib/include/stdio.rs");        // C 23: `#include <stdio.h>`
/* C 24: `#include <pthread.h>` - the header has no mirror, and the two entries this file takes
   from it are the `<signal.h>` mirror's `pthread_sigmask` (`signal.rs` 158) and `sigfillset`,
   declared at the use site below (PORTING.md P-88 ①).  */
#[cfg (feature = "have_haiku")]
include!("../lib/include/unistd.in.rs");    // C 25: `#include <unistd.h>`
#[cfg (feature = "have_haiku")]
include!("include/haiku_support.rs");       // C 27: `#include "haiku_support.h"`
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                // C 28: `#include "lisp.h"` (`emacs_abort`, `eassert!`)
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");           // C 29: `#include "haikuterm.h"`
#[cfg (feature = "have_haiku")]
include!("include/blockinput.rs");          // C 30: `#include "blockinput.h"`

/* C 26: `#include <OS.h>` - the Haiku SDK's own header, which this machine does not have: the five
   calls and the two values below carry the runtime's own signatures and the standing UNVERIFIED mark
   (PORTING.md P-88 ①/②).  */
/// C: `port_id create_port (int32 capacity, const char *name);   [<OS.h>; UNVERIFIED here]`
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  pub fn create_port (capacity: int32, name: *const c_char) -> port_id;
  /// C: `ssize_t port_buffer_size_etc (port_id port, uint32 flags, bigtime_t timeout);`
  pub fn port_buffer_size_etc (port: port_id, flags: c_uint, timeout: bigtime_t) -> ssize_t;
  /// C: `ssize_t read_port (port_id port, int32 *code, void *buffer, ssize_t bufferSize);`
  pub fn read_port (port: port_id, code: *mut int32, buffer: *mut c_void,
                    buffer_size: ssize_t) -> ssize_t;
  /// C: `ssize_t read_port_etc (port_id port, int32 *code, void *buffer, ssize_t bufferSize,
  ///                            uint32 flags, bigtime_t timeout);`
  pub fn read_port_etc (port: port_id, code: *mut int32, buffer: *mut c_void,
                        buffer_size: ssize_t, flags: c_uint, timeout: bigtime_t) -> ssize_t;
  /// C: `ssize_t write_port (port_id port, int32 code, const void *buffer, ssize_t bufferSize);`
  pub fn write_port (port: port_id, code: int32, buffer: *const c_void,
                     buffer_size: ssize_t) -> ssize_t;

  /// C: `int sigfillset (sigset_t *set);   [<signal.h>; the mirror carries `sigemptyset` and
  /// `sigaddset` but not this one]`
  pub fn sigfillset (set: *mut sigset_t) -> c_int;
  /// C: `int kill (pid_t pid, int sig);   [<signal.h>; the mirror does not carry it]`
  pub fn kill (pid: pid_t, sig: c_int) -> c_int;
}

/// C: `#define SIGPOLL 7` (`<signal.h>`; **UNVERIFIED here** - this host's header does not define
/// `SIGPOLL` at all, the clang probe for it fails, and the value is the Haiku target's own; the
/// `<signal.h>` mirror does not carry it, and this is the one signal this file names).
#[cfg (feature = "have_haiku")]
pub const SIGPOLL: c_int = 7;

/// C: `#define B_OK 0` (`<OS.h>`; UNVERIFIED here - the value is the SDK's own documented one).
#[cfg (feature = "have_haiku")]
pub const B_OK: c_int = 0;
/// C: `#define B_TIMEOUT 8` (`<OS.h>`; UNVERIFIED here).
#[cfg (feature = "have_haiku")]
pub const B_TIMEOUT: c_uint = 8;

/// C 32: `#define PORT_CAP 1200`
#[cfg (feature = "have_haiku")]
pub const PORT_CAP: c_int = 1200;

/// C 36-37: "The port used to send messages from the application thread to Emacs."
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut port_application_to_emacs: port_id = 0;

/// C 40-41: "The port used to send popup menu messages from the application thread to Emacs."
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut port_popup_menu_to_emacs: port_id = 0;

/// C 44-45: "The port used to send replies to the application after a session management event."
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut port_emacs_to_session_manager: port_id = 0;

/// C 47-54: `void haiku_io_init (void)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_io_init ()
{
  port_application_to_emacs = create_port (PORT_CAP, c"application emacs port".as_ptr ());
  port_emacs_to_session_manager = create_port (1, c"session manager port".as_ptr ());
}

/// C 56-116: `static ssize_t haiku_len (enum haiku_event_type type)` - the event-type-to-size table,
/// C's `switch` (whose arms all `return sizeof (…)` and whose fall-through is `emacs_abort ()`)
/// spelled as the `match` the port uses for `switch` (PORTING.md 2.2).
#[cfg (feature = "have_haiku")]
unsafe fn haiku_len (type_: haiku_event_type) -> ssize_t
{
  return match type_
    {
      QUIT_REQUESTED => std::mem::size_of::<haiku_quit_requested_event> () as ssize_t,
      FRAME_RESIZED => std::mem::size_of::<haiku_resize_event> () as ssize_t,
      FRAME_EXPOSED => std::mem::size_of::<haiku_expose_event> () as ssize_t,
      KEY_DOWN | KEY_UP => std::mem::size_of::<haiku_key_event> () as ssize_t,
      ACTIVATION => std::mem::size_of::<haiku_activation_event> () as ssize_t,
      MOUSE_MOTION => std::mem::size_of::<haiku_mouse_motion_event> () as ssize_t,
      BUTTON_DOWN | BUTTON_UP => std::mem::size_of::<haiku_button_event> () as ssize_t,
      ICONIFICATION => std::mem::size_of::<haiku_iconification_event> () as ssize_t,
      MOVE_EVENT => std::mem::size_of::<haiku_move_event> () as ssize_t,
      SCROLL_BAR_VALUE_EVENT => std::mem::size_of::<haiku_scroll_bar_value_event> () as ssize_t,
      SCROLL_BAR_DRAG_EVENT => std::mem::size_of::<haiku_scroll_bar_drag_event> () as ssize_t,
      WHEEL_MOVE_EVENT => std::mem::size_of::<haiku_wheel_move_event> () as ssize_t,
      MENU_BAR_RESIZE => std::mem::size_of::<haiku_menu_bar_resize_event> () as ssize_t,
      MENU_BAR_CLICK => std::mem::size_of::<haiku_menu_bar_click_event> () as ssize_t,
      MENU_BAR_OPEN | MENU_BAR_CLOSE => std::mem::size_of::<haiku_menu_bar_state_event> () as ssize_t,
      MENU_BAR_SELECT_EVENT => std::mem::size_of::<haiku_menu_bar_select_event> () as ssize_t,
      MENU_BAR_HELP_EVENT => std::mem::size_of::<haiku_menu_bar_help_event> () as ssize_t,
      ZOOM_EVENT => std::mem::size_of::<haiku_zoom_event> () as ssize_t,
      DRAG_AND_DROP_EVENT => std::mem::size_of::<haiku_drag_and_drop_event> () as ssize_t,
      APP_QUIT_REQUESTED_EVENT => std::mem::size_of::<haiku_app_quit_requested_event> () as ssize_t,
      DUMMY_EVENT => std::mem::size_of::<haiku_dummy_event> () as ssize_t,
      MENU_BAR_LEFT => std::mem::size_of::<haiku_menu_bar_left_event> () as ssize_t,
      SCROLL_BAR_PART_EVENT => std::mem::size_of::<haiku_scroll_bar_part_event> () as ssize_t,
      SCREEN_CHANGED_EVENT => std::mem::size_of::<haiku_screen_changed_event> () as ssize_t,
      CLIPBOARD_CHANGED_EVENT => std::mem::size_of::<haiku_clipboard_changed_event> () as ssize_t,
      FONT_CHANGE_EVENT => std::mem::size_of::<haiku_font_change_event> () as ssize_t,
      NOTIFICATION_CLICK_EVENT => std::mem::size_of::<haiku_notification_click_event> () as ssize_t,

      _ => { emacs_abort (); }
    };
}

/// C 118-135: `void haiku_read_size (ssize_t *len, bool popup_menu_p)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_read_size (len: *mut ssize_t, popup_menu_p: bool)
{
  let from: port_id = if popup_menu_p
    { port_popup_menu_to_emacs }
  else
    { port_application_to_emacs };
  let mut size: ssize_t = 0;

  size = port_buffer_size_etc (from, B_TIMEOUT, 0);

  if size < B_OK as ssize_t
    { *len = -1; }
  else
    { *len = size; }
}

/// C 137-152: `int haiku_read (enum haiku_event_type *type, void *buf, ssize_t len)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_read (type_: *mut haiku_event_type, buf: *mut c_void,
				     len: ssize_t) -> c_int
{
  let mut typ: int32 = 0;
  let from: port_id = port_application_to_emacs;

  if read_port (from, &raw mut typ, buf, len) < B_OK as ssize_t
    { return -1; }

  *type_ = typ as haiku_event_type;
  eassert! (len >= haiku_len (typ));
  return 0;
}

/// C 154-181: `int haiku_read_with_timeout (enum haiku_event_type *type, void *buf, ssize_t len,
/// bigtime_t timeout, bool popup_menu_p)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_read_with_timeout (type_: *mut haiku_event_type, buf: *mut c_void,
						  len: ssize_t, timeout: bigtime_t,
						  popup_menu_p: bool) -> c_int
{
  let mut typ: int32 = 0;
  let from: port_id = if popup_menu_p
    { port_popup_menu_to_emacs }
  else
    { port_application_to_emacs };

  block_input ();
  if read_port_etc (from, &raw mut typ, buf, len,
		    B_TIMEOUT, timeout as bigtime_t) < B_OK as ssize_t
    {
      unblock_input ();
      return -1;
    }
  unblock_input ();
  *type_ = typ as haiku_event_type;
  eassert! (len >= haiku_len (typ));
  return 0;
}

/// C 183-195: `int haiku_write (enum haiku_event_type type, void *buf)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_write (type_: haiku_event_type, buf: *mut c_void) -> c_int
{
  let to: port_id = port_application_to_emacs;

  if write_port (to, type_ as int32, buf, haiku_len (type_)) < B_OK as ssize_t
    { return -1; }

  kill (getpid (), SIGPOLL);

  return 0;
}

/// C 197-209: `int haiku_write_without_signal (enum haiku_event_type type, void *buf,
/// bool popup_menu_p)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_write_without_signal (type_: haiku_event_type, buf: *mut c_void,
						     popup_menu_p: bool) -> c_int
{
  let to: port_id = if popup_menu_p
    { port_popup_menu_to_emacs }
  else
    { port_application_to_emacs };

  if write_port (to, type_ as int32, buf, haiku_len (type_)) < B_OK as ssize_t
    { return -1; }

  return 0;
}

/// C 211-217: `void haiku_io_init_in_app_thread (void)`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_io_init_in_app_thread ()
{
  let mut set: sigset_t = sigset_t { __val: 0 };
  sigfillset (&raw mut set);

  if pthread_sigmask (SIG_BLOCK, &raw const set, std::ptr::null_mut ()) != 0
    { perror (c"pthread_sigmask".as_ptr ()); }
}
