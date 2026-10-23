// The batch-F header mirrors under `nt/inc/` had no compiler: no translation unit of the
// frozen configuration includes one (no `nt/` or `msdos/` unit is compiled - see
// `docs/baseline/frozen-build-sources.txt`'s batch-F section), `xtask layout-check` compiles only
// the `src/include` and `lib/include` mirrors, and `tools/extract-abi.py --check` compares text.
// Two real defects survived that gap until this probe existed: `nt/inc/sys/include/acl.rs` spelled
// its `<sys/types.h>` include as `sys_types.rs.in` (the file is `sys_types.in.rs`), and
// `nt/inc/sys/include/time.rs` included the frozen target's own `<sys/time.h>` mirror, whose
// `ITIMER_PROF`/`ITIMER_VIRTUAL`/`itimerval`/`setitimer` contradict the Windows shim's text
// (`E0428` four times, and the wrong value on `ITIMER_PROF`), and `acl.rs` asked the
// `<sys/types.h>` mirror for a name it does not carry.  This probe is the consumer those mirrors
// never had: it supplies the names their C headers take from the system, and includes all eight
// into one flat namespace, which is what a Windows translation unit taking them at once is
// (docs/status.md 3.102).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, clashing_extern_declarations)]
use std::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_void};
/// `<sys/types.h>`'s `ssize_t`, which the port's `<unistd.h>` mirror spells
/// (`lib/include/unistd.in.rs` 14: `c_long`).  `acl_to_text` takes a pointer to one.
type ssize_t = c_long;

/// MinGW's `<sys/time.h>`, the header C's `#include_next <sys/time.h>` reaches on the Windows
/// build.  The shim's own text adds everything else, and the frozen target's mirror of the *same*
/// header name (lib/include/sys_time.rs) is a different header, so it is not included anywhere.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval
{
  pub tv_sec: c_long,
  pub tv_usec: c_long,
}

// One flat namespace, the way a unit includes its mirrors: a Windows translation unit that takes
// all eight headers at once is exactly this.  The names do not overlap (each guard, constant and
// prototype is the header's own), and `acl.rs` brings `<sys/types.h>` in itself.
include!("../nt/inc/sys/include/param.rs");
include!("../nt/inc/sys/include/time.rs");
include!("../nt/inc/sys/include/file.rs");
include!("../nt/inc/sys/include/ioctl.rs");
include!("../nt/inc/sys/include/acl.rs");
include!("../nt/inc/sys/include/mman.rs");
include!("../nt/inc/sys/include/resource.rs");
include!("../nt/inc/sys/include/wait.rs");

// The probe is a compile proof, not a program: `include!` expands each mirror's text into its
// module and the compiler checks every item, used or not (`nt/inc/sys/include/ioctl.rs` carries no
// item at all - it is the empty header NT needs on the include path).  There is no C side to
// compare against, because no configuration compiles a `nt/inc/` header on this target.
fn main () {}
