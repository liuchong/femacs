// femacs: mirror of the frozen target's <sys/resource.h> (system header, P-26/P-50).
//
// There is no reference file to pair with and gnulib has no wrapper for this header, so
// this carries what the ported units name: `struct rlimit`, `rlim_t`, `RLIMIT_AS` and
// `getrlimit`, which is what src/vm-limit.c's live `#ifdef HAVE_GETRLIMIT` branch uses.
//
// The layout is the frozen target's, measured:
//
//     $ cat > /tmp/rl.c <<'EOF'
//     #include <stddef.h>
//     #include <sys/resource.h>
//     #include <stdio.h>
//     int main (void) { printf ("%zu %zu %zu | %d %d\n", sizeof (struct rlimit),
//         offsetof (struct rlimit, rlim_cur), offsetof (struct rlimit, rlim_max),
//         (int) RLIMIT_AS, (int) sizeof (rlim_t)); return 0; }
//     EOF
//     $ clang -o /tmp/rl /tmp/rl.c && /tmp/rl
//     16 0 8 | 5 8
//
// i.e. two 8-byte limits at 0 and 8, and `RLIMIT_AS` is 5 - the same value
// docs/baseline/config/darwin-arm64.macros.tsv records for the macro (1556).  A lib mirror
// is compiled without lisp.rs in spikes/syshdrs-probe.sh, so the assertions use `const _`.

/// `__uint64_t` on the frozen target.
/// `__uint64_t` on the frozen target.
pub type rlim_t = u64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

const _: () = assert! (std::mem::size_of::<rlimit> () == 16);
const _: () = assert! (std::mem::offset_of! (rlimit, rlim_cur) == 0);
const _: () = assert! (std::mem::offset_of! (rlimit, rlim_max) == 8);
const _: () = assert! (std::mem::size_of::<rlim_t> () == 8);

/* RLIMIT_AS is 5 on this target (the macro table records the same value).  */
pub const RLIMIT_AS: c_int = 5;
/* RLIMIT_DATA is 2; vm-limit.c's `#ifndef RLIMIT_AS` fallback would have used it, but the
   target defines RLIMIT_AS, so that branch is dead.  */
pub const RLIMIT_DATA: c_int = 2;

unsafe extern "C" {
  pub fn getrlimit (resource: c_int, rlp: *mut rlimit) -> c_int;
  pub fn setrlimit (resource: c_int, rlp: *const rlimit) -> c_int;
}

// getrlimit's resource selector kqueue.c uses (RLIMIT_NOFILE = 8 on the frozen target).
pub const RLIMIT_NOFILE: c_int = 8;

/// `<sys/resource.h>`'s `RLIM_INFINITY` for `rlim_t`: the all-ones value (measured with clang:
/// `printf ("%llu", (unsigned long long) RLIM_INFINITY)` -> 9223372036854775807).
pub const RLIM_INFINITY: rlim_t = 9223372036854775807;
