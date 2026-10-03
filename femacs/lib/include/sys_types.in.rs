// femacs: port of lib/sys_types.in.h @ fd7afced403 .
//
// WIP mirror: only `off_t`, which buffer.h names; the rest of the header is
// tracked in docs/ports/include-sys_types.in.md (PORTING.md P-26).

/// The frozen target's `off_t` (a 64-bit signed integer).
pub type off_t = i64;

/// `pid_t` — `process.h` 160 uses it as `Lisp_Process.pid`.  The frozen target's
/// `<sys/types.h>` makes it a 32-bit signed integer (`sizeof (pid_t) == 4`, measured).
pub type pid_t = c_int;

// sys/types.h: the group id type gnulib's group-member.c and getgroups.c take.
pub type gid_t = c_uint;

// sys/types.h + gnulib: the type getgroups.c's groups come back in (gid_t on this target).
pub type GETGROUPS_T = gid_t;

// ---- The rest of the frozen target's <sys/types.h> (PORTING.md P-26, WIP) ---------------
//
// `lib/sys_stat.in.rs` needs `struct stat`'s field types, and C gets every one of them from
// <sys/types.h> on this target: a probe that includes only <sys/types.h> and prints
// `sizeof` for each compiles, and prints
//
//     $ cat > /tmp/tprobe.c <<'EOF'
//     #include <sys/types.h>
//     #include <stdio.h>
//     int main(void){
//       mode_t m = 1; nlink_t n = 1; blksize_t b = 1; blkcnt_t c = 1; ino_t i = 1; dev_t d = 1; off_t o = 1;
//       printf("%zu %zu %zu %zu %zu %zu %zu\n", sizeof m, sizeof n, sizeof b, sizeof c, sizeof i, sizeof d, sizeof o);
//       return 0; }
//     EOF
//     $ clang -o /tmp/tprobe /tmp/tprobe.c && /tmp/tprobe
//     2 2 4 8 8 4 8
//
// i.e. mode_t and nlink_t are 16-bit, blksize_t and dev_t 32-bit, blkcnt_t and ino_t 64-bit
// and off_t (above) 64-bit - the widths the six aliases below carry.  Typos in the C arms
// the rest of the header writes (`GNULIB_defined_dev_t` and friends, which this target does
// not define) are the unchecked boxes of docs/ports/include-sys_types.in.md.

/// The frozen target's `mode_t` (`__uint16_t`, measured above).
pub type mode_t = u16;
/// The frozen target's `nlink_t` (`__uint16_t`, measured above).
pub type nlink_t = u16;
/// The frozen target's `blksize_t` (`__int32_t`, measured above).
pub type blksize_t = c_int;
/// The frozen target's `blkcnt_t` (`__int64_t`, measured above).
pub type blkcnt_t = i64;
/// The frozen target's `ino_t` (`__uint64_t`, measured above).
pub type ino_t = u64;
/// The frozen target's `dev_t` (`__int32_t`, measured above).
pub type dev_t = c_int;
/// The frozen target's `uid_t` (`__uint32_t`; `struct stat`'s `st_uid`).
pub type uid_t = c_uint;
