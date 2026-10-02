# spikes/shim

Compile-only stand-ins for C23 headers this host does not have.

**Rule (PORTING.md P-46):** the shim may only carry a header whose `HAVE_*_H` is
undefined in the frozen configuration.  A stand-in for a header the system *does*
provide silently replaces the real implementation in every C reference built with
this directory on the include path — review A's F3: `stdckdint.h` was shadowed by a
no-op `ckd_add`, so `parse_format_integer`'s overflow check became unreachable in
the C side of the differential probes (`%99999999999999999999d` wrapped into 63
spaces instead of raising "Format width or precision too large").

The probes pass this directory with `-idirafter`, so a system or gnulib header of
the same name always wins, and the two shadowing stand-ins (`stdckdint.h`,
`stdcountof.h`, both present on this host and both `#define HAVE_*_H 1` in
`docs/baseline/config/darwin-arm64.json`) are gone.
