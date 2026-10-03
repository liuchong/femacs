// femacs: port of lib/getloadavg.c @ fd7afced403.
//
// `lib/getloadavg.c` is in `docs/baseline/config/cond-eval-verified.txt` (82 conditions, 0
// disagreements with clang — adding it needed two fixes in `tools/cinventory.py`, which are
// recorded in docs/status.md §3.29), so the gate evaluates its `#if`s against the frozen
// configuration and compares only the live text.  The live arm is C 785's
// `#if ! defined LDAV_DONE && defined LOAD_AVE_TYPE && ! defined __VMS`: this target's
// `<sys/param.h>` (HAVE_SYS_PARAM_H is 1) defines `BSD 199506`, so C 225's condition holds
// and C 226 makes `LOAD_AVE_TYPE` `double`; `NLIST_STRUCT`/`N_NAME_POINTER` stay undefined,
// so C 795's `strcpy` arm is the one the port writes and C 421-431's `n_un.n_name` /
// pointer forms are dead.  Every other platform arm is dead and is *named* at its site
// below (P-51): C 377-381 `NO_GET_LOAD_AVG`, C 384-427 `HAVE_LIBKSTAT`,
// C 430-446 `hpux && HAVE_PSTAT_GETDYNAMIC`, C 448-461 `HAVE_LIBPERFSTAT`,
// C 464-478 `__linux__`/`__ANDROID__`'s `sysinfo`, C 481-531 `__CYGWIN__`,
// C 534-571 `__NetBSD__`, C 574-607 `NeXT`, C 610-675 `UMAX`, C 678-692 `DGUX`,
// C 695-717 `apollo`, C 720-728 `OSF_MIPS`, C 731-739 `__MSDOS__ || WINDOWS32`,
// C 742-782 `__VMS`, plus C 816-820 `FIXUP_KERNEL_SYMBOL_ADDR` (sgi/mips) and
// C 838-846/864-870/891-894 `SUNOS_5`'s kvm pair inside the live arm.
//
// Those arms are named rather than carried as `#[cfg (feature = …)]` text for the reason
// `lib/fsusage.c`'s statvfs/statfs4 arms were (docs/ports/fsusage.md): their types
// (`kstat_ctl_t`, `struct pst_dynamic`, `perfstat_cpu_total_t`, `struct sysinfo`,
// `processor_set_t`, `struct processor_set_basic_info`, `struct cpu_config`,
// `struct proc_summary`, `struct dg_sys_info_load_info`, `struct tbl_loadavg`, `kvm_t`),
// their functions and their headers (`<kstat.h>`, `<sys/pstat.h>`, `<libperfstat.h>`,
// `<mach/mach.h>`, `<inq_stats/*>`, `<sys/dg_sys_info.h>`, `<iodef.h>`, `<descrip.h>`,
// `<kvm.h>`) are not on this target, and the VMS arm's `$DESCRIPTOR`/`sys$assign` are not
// even C identifiers the port could spell.  The line drift this costs is declared in
// docs/ports/line-drift-tolerance.tsv.
//
// The reference build proves the file's own status on this target: `HAVE_GETLOADAVG` is 1,
// so gnulib does not build the module (`build/emacs-ref/lib/` has `nproc.o`, no
// `getloadavg.o`), and the live arm's `#include <a.out.h>` cannot resolve because this
// target has no such header — see femacs/lib/include/a_out.rs for the mirror and the
// measured facts behind it.  `--check` therefore has no reference object to compare
// against; its live-text inventory and the TU compile are the evidence (P-42).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* C 23-84's "Compile-time symbols that this file uses" block is C's own documentation of the
   arms listed in this file's `// femacs:` note: `HAVE_PSTAT_GETDYNAMIC`, `HAVE_LIBPERFSTAT`,
   `FIXUP_KERNEL_SYMBOL_ADDR()`, `KERNEL_FILE`, `LDAV_CVT()`, `LDAV_SYMBOL`, `LOAD_AVE_TYPE`,
   `HAVE_NLIST_H`, `NLIST_STRUCT`, `N_NAME_POINTER`, `HAVE_STRUCT_NLIST_N_UN_N_NAME` and the
   system predefines (`apollo`, `BSD`, `DGUX`, `eunice`, `hpux`, `__MSDOS__`, `NeXT`, `sgi`,
   `UMAX`, `VMS`, `_WIN32`, `__linux__`, `__ANDROID__`, `__CYGWIN__`, `__NetBSD__`), with
   `LDAV_DONE`/`LDAV_PRIVILEGED` as the internal markers.  The frozen configuration's
   answers are in the arm list above; the macros this file *defines* are at the bottom.  */

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar,
              c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/stdint.in.rs");
include!("include/sys_types.in.rs");
include!("include/fcntl.in.rs");
include!("include/unistd.in.rs");
include!("include/errno.rs");
include!("include/string.rs");     // <string.h> (`strcpy`)
include!("include/a_out.rs");      // <a.out.h> (`struct nlist`, `nlist`)

/* C 88-98's includes: <config.h>, <stdlib.h>, <errno.h>, <stdio.h>, <sys/types.h>,
   <sys/param.h> (HAVE_SYS_PARAM_H) and "intprops.h" are the mirrors above except that
   <sys/param.h> has no mirror yet — the two macros the live arm uses from it, `FSHIFT` and
   `FSCALE`, are the measured constants below (the same arrangement lib/fsusage.rs used for
   <sys/param.h>'s MAXPATHLEN/MNAMELEN, which went into include/sys_mount.rs with the type
   they size).  `"intprops.h"` is only reached by dead arms (C 112's `INT_STRLEN_BOUND` in
   the Cygwin arm, C 163's `INT_BUFSIZE_BOUND` in the NetBSD arm), so no mirror of it is
   needed.  C 100-115's `WINDOWS32` define and the NeXT/HURD `#undef BSD`/`#undef FSCALE`
   fixups are preprocessor text the frozen values already carry (the `#undef`s sit in
   branches the configuration disables), and C 325-331's `<fcntl.h>` include is live.  */

/// <sys/param.h>: `# define FSHIFT 11` (measured on the frozen target; the frozen macro
/// table records `FSHIFT	11`).
const FSHIFT: c_int = 11;

/// <sys/param.h>: `# define FSCALE (1 << FSHIFT)`.  C's `FSCALE` is an `int` constant that
/// `LDAV_CVT` divides a `double` by.
const FSCALE: c_int = 1 << FSHIFT;

/* C 226's `# define LOAD_AVE_TYPE double`, reached through C 225's
   `#if !defined (LOAD_AVE_TYPE) && (defined (BSD) || …)`: the type name is a macro in C, so
   the port keeps it as one (the `struct_stat64` shape lib/tempname.rs uses) and every use
   site spells `LOAD_AVE_TYPE! ()`.  */
macro_rules! LOAD_AVE_TYPE {
  () => { c_double };
}

/* C 249's `# define KERNEL_FILE "/vmunix"` and C 253's `# define LDAV_SYMBOL "_avenrun"`
   (C 246/252's `#ifndef` guards are taken because neither macro is predefined here).  The
   port keeps the two names as the C strings the `nlist`/`strcpy` calls pass.  */
const KERNEL_FILE: &core::ffi::CStr = c"/vmunix";
const LDAV_SYMBOL: &core::ffi::CStr = c"_avenrun";

/* C 192's `# define LDAV_CVT(n) (((double) (n)) / FSCALE)`, from C 191's
   `#if !defined (LDAV_CVT) && defined (FSCALE)`.  */
macro_rules! LDAV_CVT {
  ($n:expr) => { (($n) as c_double) / (FSCALE as c_double) };
}

/* C 889's `# define LDAV_DONE` and C 790's `# define LDAV_PRIVILEGED`: empty object-like
   macros whose only job is to make the arms exclusive in the preprocessor (C 375, 384, 430,
   443, 457, 476, 487, 490, 573, 608, 675, 693, 718, 741, 783, 785, 881 and 889 test or set
   `LDAV_DONE`; the frozen configuration's answers are the arm list in the header note).  The
   port keeps both names (the inventory requires them) as nullary macros.  */
macro_rules! LDAV_DONE { () => {} }
macro_rules! LDAV_PRIVILEGED { () => {} }

/* C 336-360: "Avoid static vars inside a function since in HPUX they dump as pure."  The
   three live ones are C 352-360 (`channel`, `getloadavg_initialized`, `offset` and
   `name_list`, under `#if !defined (HAVE_LIBKSTAT) && defined (LOAD_AVE_TYPE)`); the NeXT
   (`default_set`), UMAX (`cpus`, `samples`), DGUX (`load_info`) and SUNOS_5 (`kd`) statics
   are in dead arms.  C's internal-linkage `static` → Rust statics that are neither `pub` nor
   `#[unsafe(no_mangle)]` (PORTING.md 0.2).  */
static mut channel: c_int = 0;
static mut getloadavg_initialized: bool = false;
static mut offset: c_long = 0;
static mut name_list: [nlist; 2] = [nlist { n_name: [0; NCPLN], n_type: 0, n_other: 0,
                                             n_desc: 0, n_value: 0 }; 2];

/* Put the 1 minute, 5 minute and 15 minute load averages
   into the first NELEM elements of LOADAVG.
   Return the number written (never more than 3, but may be less than NELEM),
   or -1 (setting errno) if an error occurred.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getloadavg (loadavg: *mut c_double, nelem: c_int) -> c_int
{                                    /* C 375 */
  let mut elem: c_int = 0;           /* C 376 */

  /* C 377-381's `#ifdef NO_GET_LOAD_AVG` arm (`errno = ENOSYS; elem = -1;`), C 384-427's
     `HAVE_LIBKSTAT` arm (`kstat_open`/`kstat_lookup`/`kstat_read`/`kstat_data_lookup`/
     `kstat_close` over `kstat_ctl_t`/`kstat_t`/`kstat_named_t`), C 430-446's
     `hpux && HAVE_PSTAT_GETDYNAMIC` arm (`struct pst_dynamic`, `pstat_getdynamic`),
     C 448-461's `HAVE_LIBPERFSTAT` arm (`perfstat_cpu_total_t`/`perfstat_cpu_total` and
     C 275's `SBITS`), C 464-478's `__linux__`/`__ANDROID__` arm (`struct sysinfo`/`sysinfo`
     and C 276's `SI_LOAD_SHIFT`), C 481-531's `__CYGWIN__` arm (`/proc/loadavg` and its
     `ldavgbuf` parse), C 534-571's `__NetBSD__` arm (`NETBSD_LDAV_FILE`/`sscanf`),
     C 574-607's `NeXT` arm (`processor_set_default`/`processor_set_info`/`LOAD_SCALE`),
     C 610-675's `UMAX` arm (`inq_stats`, `struct cpu_config`, `struct proc_summary`),
     C 678-692's `DGUX` arm (`dg_sys_info`, `struct dg_sys_info_load_info`), C 695-717's
     `apollo` arm (`proc1_$get_loadav`), C 720-728's `OSF_MIPS` arm (`table`/
     `struct tbl_loadavg`), C 731-739's `__MSDOS__ || WINDOWS32` arm (the zero fill under
     "A faithful emulation is going to have to be saved for a rainy day.") and C 742-782's
     `__VMS` arm (`$DESCRIPTOR`/`sys$assign`/`sys$qiow`/`sys$dassgn`) are all dead: every one
     of those macros is undefined on this target.  They are named here, not carried (see the
     `// femacs:` note at the top of this file).  C 570's `errno = ENOSYS; elem = -1;` tail
     (C 892-895) is dead too, because C 881's arm defines `LDAV_DONE`.  */

  /* C 785's arm is the live one: the UNIX path that reads the average out of /dev/kmem
     (C 785-879).  C 790's `# define LDAV_PRIVILEGED` is the marker macro at the top.  */
  if offset == 0                              /* C 793 */
    {
      /* C 795's `#if ! defined NLIST_STRUCT || ! defined N_NAME_POINTER` is the live arm:
         both are undefined, so `n_name` is the array a.out.h field (C 796-797).  C 798-805's
         `#else` pair (`n_un.n_name = LDAV_SYMBOL;` and `n_name = LDAV_SYMBOL;` under
         C 799's `#ifdef HAVE_STRUCT_NLIST_N_UN_N_NAME`) is dead.  */
      strcpy (name_list[0].n_name.as_mut_ptr (), LDAV_SYMBOL.as_ptr ());
      strcpy (name_list[1].n_name.as_mut_ptr (), c"".as_ptr ());

      /* C 808's `#ifndef SUNOS_5` is live; C 810's `# if !defined (_AIX)` picks `nlist`.
         C 816-820's `#ifdef FIXUP_KERNEL_SYMBOL_ADDR` (sgi/mips's `n_value &= ~(1 << 31)`)
         is dead, and so is C 812-813's `knlist (name_list, 1, sizeof (name_list[0]))`.  */
      if nlist (KERNEL_FILE.as_ptr (), name_list.as_mut_ptr ()) >= 0
          {
            offset = name_list[0].n_value as c_long;
          }
    }

  /* C 827's `if (!getloadavg_initialized)`; C 829's `#ifndef SUNOS_5` is live and
     C 838-846's `#else` (`kvm_open`/`kvm_nlist` into `kd`) is dead.  */
  if !getloadavg_initialized
    {
      let fd = open (c"/dev/kmem".as_ptr (), O_RDONLY | O_CLOEXEC);
      if 0 <= fd
        {
          channel = fd;
          getloadavg_initialized = true;
        }
    }

  /* C 850's `LOAD_AVE_TYPE load_ave[3];`.  C leaves the array uninitialised and only reads
     it after the `read` below succeeded (a failed read returns -1 two blocks down), so the
     port's zero fill is C's value on every path that reads it (PORTING.md 0.1-13).  */
  let mut load_ave: [LOAD_AVE_TYPE! (); 3] = [0.0; 3];

  /* C 853's `if (offset && getloadavg_initialized)`; C 856's `#ifndef SUNOS_5` is live and
     C 864-870's `#else` (`kvm_read`/`kvm_close`) is dead.  */
  if offset != 0 && getloadavg_initialized
    {
      /* C's `read (...) != sizeof (load_ave)` compares `ssize_t` with `size_t`, i.e. after
         the usual arithmetic conversions, so the port casts the result to `usize` (an
         error's -1 becomes a huge unsigned value, which differs from the size exactly as it
         does in C).  */
      if lseek (channel, offset, 0) == -1
          || read (channel, load_ave.as_mut_ptr () as *mut c_void,
                   std::mem::size_of_val (&load_ave)) as usize
             != std::mem::size_of_val (&load_ave)
        {
          close (channel);
          getloadavg_initialized = false;
        }
    }

  if offset == 0 || !getloadavg_initialized   /* C 874 */
    {
      *errno () = ENOTSUP;                    /* C 876 */
      return -1;
    }

  /* C 881's `# if !defined (LDAV_DONE) && defined (LOAD_AVE_TYPE)` arm, the one that hands
     the kernel's scaled values out through `LDAV_CVT` and then defines `LDAV_DONE` so that
     C 892's `errno = ENOSYS` tail stays dead.  C's `loadavg[elem++] = …` writes with the
     index's old value and then steps it, which the port keeps as two statements (PORTING.md
     2.2, the lib/stpcpy.rs precedent).  */
  if nelem > 0                                /* C 882 */
    {
      *loadavg.add (elem as usize) = LDAV_CVT! (load_ave[0]);
      elem += 1;
    }
  if nelem > 1                                /* C 884 */
    {
      *loadavg.add (elem as usize) = LDAV_CVT! (load_ave[1]);
      elem += 1;
    }
  if nelem > 2                                /* C 886 */
    {
      *loadavg.add (elem as usize) = LDAV_CVT! (load_ave[2]);
      elem += 1;
    }

  return elem;                                /* C 896 */
}                                             /* C 897 */
