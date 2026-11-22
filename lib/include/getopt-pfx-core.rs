// femacs: WIP mirror of lib/getopt-pfx-core.h @ fd7afced403 (PORTING.md P-26).
#![forbid(unsafe_code)]
//
// The whole header is the `__GETOPT_PREFIX` rename layer: the frozen configuration defines
// __GETOPT_PREFIX as `rpl_` (docs/baseline/config/darwin-arm64.macros.tsv 4028), so C 45-49
// turn `getopt`, `optarg`, `opterr`, `optind` and `optopt` into `rpl_getopt`, `rpl_optarg`,
// `rpl_opterr`, `rpl_optind` and `rpl_optopt` for everything after the header.
//
// The port expresses a rename where the name is *used* (PORTING.md 10.13, the mempcpy/lstat
// precedent): `#[link_name = "rpl_optarg"]` on the declaration in getopt-core.rs and
// `#[unsafe(export_name = "rpl_optarg")]` on the definition in lib/getopt.rs.  C 60's
// `#undef _GETOPT_CORE_H` + `#include <getopt-core.h>` (C 64) is what makes C's translation
// unit see the declarations twice - once unprefixed from the system <getopt.h>, once
// prefixed; the port declares the prefixed ones only, which is what the frozen build links.
// The file has no item of its own; docs/ports/include-getopt-pfx-core.md records the rest.

// [U1-MODULE-MIRROR]
use super::*;
/* C 21: the include guard, whose C body is `1`.  */
pub const _GETOPT_PFX_CORE_H: core::ffi::c_int = 1;
