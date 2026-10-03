// femacs: port of lib/min-eloop-threshold.h @ fd7afced403 .
//
// One constant: the minimum number of symbolic links a path resolution must be able to
// traverse before ELOOP is diagnosed (canonicalize-lgpl.c's loop counter compares against
// it).  `SYMLOOP_MAX` is mentioned only in the C comment; the header defines no other
// macro.

/* #define MIN_ELOOP_THRESHOLD    40  */
pub const MIN_ELOOP_THRESHOLD: c_int = 40;

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _MIN_ELOOP_THRESHOLD_H: c_int = 1;
