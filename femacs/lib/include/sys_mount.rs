// femacs: mirror of the frozen target's <sys/mount.h> (system header, P-26).
//
// lib/fsusage.c's live arm needs it: `STAT_STATFS2_BSIZE` is 1 while `STAT_STATVFS`
// (`STAT_STATVFS || STAT_STATVFS64`) and `HAVE_SYS_VFS_H`/`HAVE_SYS_STATFS_H` are undefined,
// so C 28-49's `#else` arm is the live text and C 38 takes `#include <sys/mount.h>` because
// `HAVE_SYS_MOUNT_H` is 1.  The C 211-213 block then reads `fsd.f_bsize`, `f_blocks`,
// `f_bfree`, `f_bavail`, `f_files` and `f_ffree` - the six fields the port names.
//
// The frozen target's layout is measured, not guessed:
//
//     $ cat > /tmp/stfsprobe.c <<'EOF'
//     #include <sys/mount.h>
//     #include <sys/param.h>
//     #include <stddef.h>
//     #include <stdio.h>
//     int main (void) {
//       printf ("sizeof=%zu align=%zu\n", sizeof (struct statfs), _Alignof (struct statfs));
//       printf ("f_bsize@%zu f_iosize@%zu f_blocks@%zu f_bfree@%zu f_bavail@%zu f_files@%zu f_ffree@%zu\n",
//               offsetof (struct statfs, f_bsize), offsetof (struct statfs, f_iosize),
//               offsetof (struct statfs, f_blocks), offsetof (struct statfs, f_bfree),
//               offsetof (struct statfs, f_bavail), offsetof (struct statfs, f_files),
//               offsetof (struct statfs, f_ffree));
//       printf ("f_fsid@%zu f_owner@%zu f_type@%zu f_flags@%zu f_fssubtype@%zu f_fstypename@%zu f_mntonname@%zu f_mntfromname@%zu\n",
//               offsetof (struct statfs, f_fsid), offsetof (struct statfs, f_owner),
//               offsetof (struct statfs, f_type), offsetof (struct statfs, f_flags),
//               offsetof (struct statfs, f_fssubtype), offsetof (struct statfs, f_fstypename),
//               offsetof (struct statfs, f_mntonname), offsetof (struct statfs, f_mntfromname));
//       printf ("MFSTYPENAMELEN=%d MAXPATHLEN=%d MNAMELEN=%d\n", (int) MFSTYPENAMELEN, (int) MAXPATHLEN, (int) MNAMELEN);
//       printf ("sizes: uint32=%zu int32=%zu uint64=%zu fsid=%zu\n", sizeof (uint32_t), sizeof (int32_t), sizeof (uint64_t), sizeof (fsid_t));
//       return 0; }
//     EOF
//     $ clang -o /tmp/stfsprobe /tmp/stfsprobe.c && /tmp/stfsprobe
//     sizeof=2168 align=8
//     f_bsize@0 f_iosize@4 f_blocks@8 f_bfree@16 f_bavail@24 f_files@32 f_ffree@40
//     f_fsid@48 f_owner@56 f_type@60 f_flags@64 f_fssubtype@68 f_fstypename@72 f_mntonname@88 f_mntfromname@1112
//     MFSTYPENAMELEN=16 MAXPATHLEN=1024 MNAMELEN=1024
//     sizes: uint32=4 int32=4 uint64=8 fsid=8
//
// `f_fstypename`/`f_mntonname`/`f_mntfromname` are `MFSTYPENAMELEN`/`MAXPATHLEN` byte arrays
// (the latter two from <sys/param.h>, which the system <sys/mount.h> includes); the port
// carries the three lengths here rather than adding a mirror for <sys/param.h>, whose only
// other use in this unit would be these constants.

pub const MFSTYPENAMELEN: usize = 16;
pub const MAXPATHLEN: usize = 1024;
pub const MNAMELEN: usize = 1024;

/* <sys/mount.h>'s `fsid_t` (8 bytes: two int32_t, measured).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fsid_t {
  pub val: [i32; 2],
}

/// <sys/mount.h>'s `struct statfs` on the host: 2168 bytes with the three name arrays (the
/// offsets above).
#[cfg (not (feature = "have_android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statfs {
  pub f_bsize: u32,              /* fundamental file system block size */
  pub f_iosize: i32,             /* optimal transfer block size */
  pub f_blocks: u64,             /* total data blocks in file system */
  pub f_bfree: u64,              /* free blocks in fs */
  pub f_bavail: u64,             /* free blocks avail to non-superuser */
  pub f_files: u64,              /* total file nodes in file system */
  pub f_ffree: u64,              /* free file nodes in fs */
  pub f_fsid: fsid_t,            /* file system id */
  pub f_owner: uid_t,            /* user that mounted the filesystem */
  pub f_type: u32,               /* type of filesystem */
  pub f_flags: u32,              /* copy of mount exported flags */
  pub f_fssubtype: u32,          /* fs sub-type (flavor) */
  pub f_fstypename: [c_char; MFSTYPENAMELEN], /* fs type name */
  pub f_mntonname: [c_char; MAXPATHLEN],      /* directory on which mounted */
  pub f_mntfromname: [c_char; MAXPATHLEN],    /* mounted filesystem */
  pub f_flags_ext: u32,
  pub f_reserved: [u32; 7],
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<statfs> () == 2168);

/// bionic's `struct statfs`: 120 bytes and a different field set and order (NDK,
/// aarch64-linux-android21: size 120, `f_bsize` 8, `f_blocks` 16, `f_bavail` 32, `f_fsid` 56,
/// `f_namelen` 64, `f_frsize` 72, `f_flags` 80, `f_spare[4]` 88 - status 4.795).  The host's
/// `f_iosize`/`f_owner`/`f_fstypename`/`f_mntonname`/`f_mntfromname`/`f_flags_ext`/`f_reserved`
/// members do not exist here.
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statfs {
  pub f_type: c_long,            /* 0 */
  pub f_bsize: c_long,           /* 8 */
  pub f_blocks: u64,             /* 16 */
  pub f_bfree: u64,              /* 24 */
  pub f_bavail: u64,             /* 32 */
  pub f_files: u64,              /* 40 */
  pub f_ffree: u64,              /* 48 */
  pub f_fsid: fsid_t,            /* 56 */
  pub f_namelen: c_long,         /* 64 */
  pub f_frsize: c_long,          /* 72 */
  pub f_flags: c_long,           /* 80 */
  pub f_spare: [c_long; 4],      /* 88 */
}

#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<statfs> () == 120);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_bsize) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_blocks) == 16);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_bavail) == 32);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_fsid) == 56);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_namelen) == 64);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_flags) == 80);

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_bsize) == 0);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_iosize) == 4);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_blocks) == 8);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_bfree) == 16);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_bavail) == 24);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_files) == 32);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_ffree) == 40);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_fsid) == 48);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_owner) == 56);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_type) == 60);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_flags) == 64);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_fssubtype) == 68);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_fstypename) == 72);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_mntonname) == 88);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_mntfromname) == 1112);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (statfs, f_flags_ext) == 2136);

unsafe extern "C" {
  /* int statfs (const char *path, struct statfs *buf);  */
  pub fn statfs (path: *const c_char, buf: *mut statfs) -> c_int;
}
