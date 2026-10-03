// femacs: port of src/atimer.h @ fd7afced403 .
//
// Header mirror (PORTING.md 0.1): the two type definitions, the three enum members and the
// seven prototypes; the one conditional is `#ifdef HAVE_TIMERFD` (C 81-83), which the frozen
// configuration leaves undefined (docs/baseline/config/darwin-arm64.macros.tsv records it as
// `undef`), so `timerfd_callback` is kept as P-51 text and its token is visible to the ledger.
//
// The field C spells `fn` is the raw identifier `r#fn` (`fn` is a Rust keyword), so the C
// name survives verbatim, as `r#type` does in dispextern.rs/lisp.rs.

include!("../../lib/include/time.in.rs");     // atimer.h 21: `#include <time.h>`

// atimer.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_ATIMER_H: bool = true;

/* Types of timers.  */

// atimer.h 28-40: C's `enum atimer_type` (PORTING.md 1.3's shape: the C type plus one
// constant per member).
pub type atimer_type = c_int;
pub const ATIMER_ABSOLUTE: atimer_type = 0;
pub const ATIMER_RELATIVE: atimer_type = 1;
pub const ATIMER_CONTINUOUS: atimer_type = 2;

/* Type of timer callback functions.  */

// atimer.h 44: `typedef void (* atimer_callback) (struct atimer *timer);` — the pointer is
// nullable in C, so it is an `Option` of an `extern "C"` fn (P-27).
pub type atimer_callback = Option<unsafe extern "C" fn (timer: *mut atimer)>;

/* Structure describing an asynchronous timer.  */

// atimer.h 48-72.  `struct atimer` is a complete type in this header, so the layout assertions
// the C compiler proves (tools/gen-layout-fields.py's STRUCTS) apply to it directly.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct atimer
{
  /* The type of this timer.  */
  pub r#type: atimer_type,

  /* Time when this timer is ripe.  */
  pub expiration: timespec,

  /* Interval of this timer.  */
  pub interval: timespec,

  /* Function to call when timer is ripe.  Interrupt input is
     guaranteed to not be blocked when this function is called.  */
  pub r#fn: atimer_callback,

  /* Additional user-specified data to pass to FN.  */
  pub client_data: *mut c_void,

  /* Next in list of active or free atimers.  */
  pub next: *mut atimer,
}

/* Function prototypes.  */

// The seven prototypes atimer.h 74-83 declares are what `src/atimer.c` *defines*, and Rust
// cannot declare and define one name in one module: they live in a module the unit
// glob-imports, so the definition in the unit shadows the import — C's prototype-plus-
// definition pair (PORTING.md P-17/P-39, the `lisp.rs`/`signal.rs` shape).
pub mod atimer_decls {
  use super::*;

  unsafe extern "C" {
    pub fn start_atimer (type_: atimer_type, expiration: timespec,
                         fn_: atimer_callback, client_data: *mut c_void) -> *mut atimer;
    pub fn cancel_atimer (timer: *mut atimer);
    pub fn do_pending_atimers ();
    pub fn init_atimer ();
    pub fn turn_on_atimers (on: bool);
    pub fn stop_other_atimers (timer: *mut atimer);
    pub fn run_all_atimers ();
    // atimer.h 81-83's `#ifdef HAVE_TIMERFD` (dead here, P-51):
    //   void timerfd_callback (int, void *);
    #[cfg (feature = "timerfd")]
    pub fn timerfd_callback (fd: c_int, arg: *mut c_void);
  }
}
use atimer_decls::*;
