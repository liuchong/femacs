// femacs: WIP mirror of lib/getopt.in.h @ fd7afced403, i.e. of the generated
// `build/emacs-ref/lib/getopt.h` (PORTING.md P-26).
//
// The generated header's decisions, all measured in build/emacs-ref/lib/getopt.h:
//   * HAVE_GETOPT_H is 1, so C 28-38's arm is live: it `#define`s `_GL_SYSTEM_GETOPT`,
//     renames `option` to `sys_option` for the duration of the `#include_next <getopt.h>`,
//     and includes the *system* header.  The port's one `struct option` is gnulib's
//     (getopt-ext.rs); the system header's declaration is only needed by C's
//     `#include_next`/C++ dance, and macOS's `struct option` has the same four fields.
//   * `__GETOPT_PREFIX` is `rpl_`, so C 47-58's `#if defined __GETOPT_PREFIX` arm is live
//     and the `# include <stdlib.h>` inside it is dead (`!1` in the generated header) -
//     the prefix renames are the pfx-core/pfx-ext layer, documented there.
//   * `_GL_ARG_NONNULL` (C 60-75) is an attribute; PORTING.md 0.3 drops attributes.
//
// The includes follow the generated header: getopt-cdefs.h (C 77), then getopt-pfx-core.h
// (C 78) and getopt-pfx-ext.h (C 79), each of which includes the core/ext header it prefixes.

include!("getopt-cdefs.in.rs");
include!("getopt-pfx-core.rs");
include!("getopt-pfx-ext.rs");
include!("getopt-core.rs");
include!("getopt-ext.rs");
