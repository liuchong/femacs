// femacs: WIP mirror of lib/getopt-pfx-ext.h @ fd7afced403 (PORTING.md P-26).
//
// Same rename layer as getopt-pfx-core.rs, for the GNU extensions: C 28-33 rename
// `getopt_long`, `getopt_long_only` and `_getopt_internal` to `rpl_getopt_long`,
// `rpl_getopt_long_only` and `rpl_getopt_internal` (measured on the reference object:
// `nm build/emacs-ref/lib/getopt.o` defines `_rpl_getopt_internal`, and getopt1.o
// `_rpl_getopt_long`), and C 47-53 define `__getopt_argv_const` as empty plus
// `__getopt_argv_const_is_empty 1` - which is why the prototypes in getopt-ext.h take
// `char **argv` and why lib/getopt1.c's `ARGV_CAST` is the identity.
//
// The renames are expressed at the declaration (`#[link_name = "rpl_getopt_long"]` in
// getopt-ext.rs) and the definition (`#[unsafe(export_name = ...)]` in lib/getopt1.rs); the
// file has no item of its own.  docs/ports/include-getopt-pfx-ext.md records the rest.
