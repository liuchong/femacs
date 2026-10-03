# emacs/lib/getloadavg.c (897 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] getloadavg  (line 374)

## Macros defined — 6
- [x] KERNEL_FILE  (line 249, body 9ea37e09e547)
- [x] LDAV_CVT  (line 192, body e6942245b342)
- [x] LDAV_DONE  (line 889, body da39a3ee5e6b)
- [x] LDAV_PRIVILEGED  (line 790, body da39a3ee5e6b)
- [x] LDAV_SYMBOL  (line 253, body 9ff57e6b9ea2)
- [x] LOAD_AVE_TYPE  (line 226, body bdb36bb22deb)

## Types and their fields — 0

## File-scope objects — 4
- [x] static int channel  (line 352, static)
- [x] static bool getloadavg_initialized  (line 354, static)
- [x] static long offset  (line 356, static)
- [x] static struct nlist name_list  (line 359, static)

## Decisions

`emacs/lib/getloadavg.c` @ `fd7afced403`; port `femacs/lib/getloadavg.rs` (255 lines).  All
checklist items ticked; `build-tu lib/getloadavg.c` -> `built lib/getloadavg.rs ->
../build/femacs/getloadavg.o` (0 errors), `--check` -> `OK`.

- **The file is now in `cond-eval-verified.txt`, which took two `tools/cinventory.py` fixes.**
  It was previously blind (20 of its 82 conditions disagreed with clang, so the gate demanded
  *both* arms of every platform `#if`).  The causes were both about the file-local macro
  state: (1) `#undef` was applied even inside a dead branch, while `#define` already checked
  it — C 104-110's `#undef BSD`/`#undef FSCALE` sit in `#ifdef NeXT` and
  `#if defined __gnu_hurd__ || defined NeXT`, both false, and applying them un-defined the
  `<sys/param.h>` macros that decide the whole nlist chain; (2) a `#define NAME (params)`
  recorded the name as `NAME (params)`, so `defined (NAME)` answered false — C 190's
  `# define LDAV_CVT(n) …` never satisfied C 267's `#ifndef LDAV_CVT`.  With both fixed,
  `tools/check-cond-eval.py lib/getloadavg.c` is 82 conditions / 0 unknown / **0
  disagreements**, and the whole 431-file verified list re-ran with **0 disagreements**
  (5,601 conditions), so the fix regresses nothing (both facts recorded in docs/status.md
  §3.29).
- **Which arm is live.**  The frozen table has `HAVE_SYS_PARAM_H` = 1 and `<sys/param.h>`
  defines `BSD` (199506) and `FSCALE` = `(1<<FSHIFT)` (2048), so C 225's disjunction holds,
  C 226 makes `LOAD_AVE_TYPE` `double` and C 785's nlist arm
  (`# if ! defined LDAV_DONE && defined LOAD_AVE_TYPE && ! defined __VMS`) plus C 881's
  `LDAV_CVT` arm are the file's whole live body (111 of the 523 lines inside `getloadavg`).
  `NLIST_STRUCT` and `N_NAME_POINTER` are undefined, so C 795's `strcpy` arm is live and
  C 798-805's pointer/`n_un.n_name` pair is dead; `SUNOS_5`, `_AIX` and
  `FIXUP_KERNEL_SYMBOL_ADDR` are undefined, so the `kvm_*`/`knlist`/`n_value &= ~(1<<31)`
  texts inside the live arm are dead too.
- **Dead arms are named, not carried** (P-51, the `lib/fsusage.c` treatment): C 377-381
  `NO_GET_LOAD_AVG`, C 384-427 `HAVE_LIBKSTAT`, C 430-446 `hpux && HAVE_PSTAT_GETDYNAMIC`,
  C 448-461 `HAVE_LIBPERFSTAT`, C 464-478 `__linux__`/`__ANDROID__`, C 481-531 `__CYGWIN__`,
  C 534-571 `__NetBSD__`, C 574-607 `NeXT`, C 610-675 `UMAX`, C 678-692 `DGUX`, C 695-717
  `apollo`, C 720-728 `OSF_MIPS`, C 731-739 `__MSDOS__ || WINDOWS32`, C 742-782 `__VMS`,
  plus the dead branches inside the live arm (C 816-820, C 838-846, C 864-870, C 891-894) and
  C 570/892-895's `errno = ENOSYS; elem = -1;` tail.  Their types (`kstat_ctl_t`,
  `struct pst_dynamic`, `perfstat_cpu_total_t`, `struct sysinfo`, `processor_set_t`,
  `struct cpu_config`, `struct proc_summary`, `struct dg_sys_info_load_info`,
  `struct tbl_loadavg`, `kvm_t`), their functions and their headers (`<kstat.h>`,
  `<sys/pstat.h>`, `<libperfstat.h>`, `<mach/mach.h>`, `<inq_stats/*>`, `<sys/dg_sys_info.h>`,
  `<kvm.h>`) are not on this target, and the VMS arm's `$DESCRIPTOR`/`sys$assign`/`sys$qiow`
  are not C identifiers a Rust port could spell; carrying them as `#[cfg (feature = …)]` text
  would have to invent every one of those declarations.  The drift is declared:
  `line-drift-tolerance.tsv` records `getloadavg 0.72` (C 897 vs 255, 71.6%).
- **The unit's own `<a.out.h>` is a new mirror** (`femacs/lib/include/a_out.rs`): the frozen
  target has no such header, so the shape of `struct nlist` is the one the *C text's own
  configuration* selects (`NLIST_STRUCT`/`N_NAME_POINTER` undefined → `strcpy` into
  `n_name` → the 4.3BSD/SunOS array form, `NCPLN` 8), documented in the mirror with the
  measured facts (`HAVE_GETLOADAVG` 1, no `getloadavg.o` in `build/emacs-ref/lib/`, clang
  cannot open the header).  Because no C compiler on this target can measure it, the type is
  deliberately **not** in `tools/gen-layout-fields.py`'s list — the layout gate reports it as
  pending instead of asserting an invented offset.
- **Type macros keep C's names**: `LOAD_AVE_TYPE` is a `macro_rules!` used in type position
  (`[LOAD_AVE_TYPE! (); 3]`, the `struct_stat64` shape of `lib/tempname.rs`), because
  `extract-abi.py` looks for a macro/const/fn of that name and a bare `type` alias would be
  reported as a missing macro.  `KERNEL_FILE`/`LDAV_SYMBOL` are `&CStr` constants (the
  constants rule compares their bytes), so the `nlist`/`strcpy` calls pass the same
  NUL-terminated strings.  `FSCALE`/`FSHIFT` are the two `<sys/param.h>` macros the live arm
  needs; they are unit-local constants with the frozen values, since `<sys/param.h>` has no
  mirror yet (the note in the file records where lib/fsusage.rs put its two).
- **C's uninitialised `LOAD_AVE_TYPE load_ave[3];` (C 850) is zero-filled** (PORTING.md
  0.1-13): every path that reads the array is behind the successful `read`, so C's value on
  those paths is exactly what the port writes.
- **C's post-increments keep C's order**: `loadavg[elem++] = …` is an assignment followed by
  `elem += 1` (PORTING.md 2.2, the `lib/stpcpy.rs` precedent), and `read (...) != sizeof
  (load_ave)` carries the `as usize` cast C's usual arithmetic conversions perform (a failed
  read's -1 becomes a huge unsigned value, which differs from the size exactly as in C).
- **The comments C's preprocessor carries are summarised, not copied**: C 23-84's 60-line
  "Compile-time symbols that this file uses" block and the per-arm prose (including C 733's
  "A faithful emulation is going to have to be saved for a rainy day.") are folded into the
  file's `// femacs:` note; the license header is verbatim.
- Not verified: `gate-fast`/`gate-probes` and any behavioural test (the unit is not compiled
  by the reference build on this target, so there is no `getloadavg.o` to compare symbols
  with, and `/dev/kmem` does not exist on modern Darwin — the live arm's runtime behaviour is
  the same `ENOTSUP` path C would take there).
