# emacs/lib/nproc.c (582 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] num_processors_via_affinity_mask  (line 72, static)
- [x] num_processors_available  (line 237, static)
- [x] cpu_quota  (line 488, static)
- [x] parse_omp_threads  (line 516, static)
- [x] num_processors  (line 545)

## Macros defined — 1
- [x] NPROC_MINIMUM  (line 65, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/nproc.c` @ `fd7afced403`; port `lib/nproc.rs` (279 lines).  All checklist
items ticked; `build-tu lib/nproc.c` -> `built lib/nproc.rs -> ../build/femacs/nproc.o`
(0 errors), `--check` -> `OK`.

- **Symbol-level 1:1 against the reference object** (`build/emacs-ref/lib/nproc.o`, the unit
  *is* compiled by the frozen build): the port's global set is `_num_processors` (T) with
  `_getenv`, `_strtoul`, `_sysconf`, `_sysctl` undefined — exactly the reference's.  The four
  C `static` functions stay local (`t parse_omp_threads` / `num_processors_available` /
  `num_processors_via_affinity_mask` / `cpu_quota`) and the function-local
  `static int mib[][2]` is a local data symbol, as in the reference
  (`d _num_processors_available.mib`).  The only additions are mangled Rust instantiations of
  `core::mem::size_of_val` / `core::ptr::const_ptr::is_null` (generic code, unique names, no
  C name claimed).
- **Which arms are live.**  The frozen table has `_SC_NPROCESSORS_ONLN` 58,
  `_SC_NPROCESSORS_CONF` 57, `HAVE_SYSCTL` 1, `CTL_HW` 6 and `HW_NCPU` 3, and no
  `HAVE_SCHED_GETAFFINITY_LIKE_GLIBC` / `HAVE_SCHED_GETAFFINITY_NP` /
  `HAVE_PTHREAD_GETAFFINITY_NP` / `HAVE_PSTAT_GETDYNAMIC` / `HW_NCPUONLINE` / `_WIN32`
  (and `__GLIBC__`/`__linux__`/`__ANDROID__` are undefined), so the live text is: the two
  `sysconf` branches, the `sysctl` mib fallback, the OpenMP environment parsing and
  `num_processors` itself.  `num_processors_via_affinity_mask`'s live body really is
  `return 0;` (C 230) and `cpu_quota`'s is `return quota;` with `quota` still `ULONG_MAX`
  (C 490/510) — that is what C compiles here, not a stub the port chose.
- **Dead arms are named, not carried** (P-51, the `lib/fsusage.c` treatment): C 74-208's five
  affinity-mask bodies (glibc `pthread_getaffinity_np`; NetBSD 5's; the
  `HAVE_SCHED_GETAFFINITY_LIKE_GLIBC` chain with its `CPU_ALLOC` retry loop and the
  statically sized `cpu_set_t`; `HAVE_SCHED_GETAFFINITY_NP`'s `cpuset_t`;
  native Windows' `GetProcessAffinityMask`), C 280-294's glibc `/sys`//`/proc` fixup inside
  the `_SC_NPROCESSORS_CONF` branch, C 300-321's HP-UX `pstat_getdynamic`, C 332-334's
  `HW_NCPUONLINE` row, C 347-354's `GetSystemInfo`, C 359-481's whole cgroup2 block
  (`cgroup2_mount` + `get_cgroup2_cpu_quota`, ~120 lines) and C 493-508's `SCHED_DEADLINE`
  switch.  Their types (`cpu_set_t`, `cpuset_t`, `DWORD_PTR`, `SYSTEM_INFO`,
  `struct pst_dynamic`, `FILE`, `struct mntent`), functions and headers (`<pthread.h>`,
  `<sched.h>`, `<windows.h>`, `<sys/pstat.h>`, `<mntent.h>`) are not on this target.  Drift
  declared: `line-drift-tolerance.tsv` records `nproc 0.53` (C 582 vs 279, 52.1%).
- **`NPROC_MINIMUM` is a constant**, not a `macro_rules!` (the `lib/malloc.rs` shape): the
  C body is the literal `1`, which a `const NPROC_MINIMUM: c_ulong = 1;` gives the macro-body
  comparison verbatim.
- **C's two assigned parameters carry `mut`** (`parse_omp_threads`'s `threads++`,
  `num_processors`'s `query = NPROC_CURRENT`); C's `char *endptr;` before `strtoul` is
  zero-initialised (PORTING.md 0.1-13) and C's post-increments become explicit steps
  (PORTING.md 2.2).
- **Mirrors.**  New: `include/nproc.rs` (`enum nproc_query` with its three members and the
  `num_processors` prototype in a glob-imported module, PORTING.md 0.1-10 — `--check` OK) and
  `include/sys_sysctl.rs` (`CTL_HW` 6, `HW_NCPU` 3 and the `<sys/sysctl.h>` `sysctl`
  prototype, values measured from the frozen table).  Appended to existing system-header
  mirrors (minimal additions, nothing rewritten): `unistd.in.rs`'s
  `_SC_NPROCESSORS_CONF`/`_SC_NPROCESSORS_ONLN` (57/58, measured) and `c-ctype.rs`'s missing
  `c_isspace` (C 299-309; written as an `if` chain because Rust has no `\v`/`\f` escapes and
  a `char` literal is not the `int` C compares).  `getenv`/`strtoul` live in that same mirror
  (`include/stdlib.rs`) as its single home — one declaration per name for the whole crate
  (PORTING.md 0.1-3, the `sys_stat.in.rs` `chmod` precedent).  Making that true needed one
  edit outside this pair: `lib/getopt.rs` declared `getenv` itself ("the entries those
  mirrors do not carry yet"), which is a *second* declaration of one name in one compilation
  unit — measured, `E0428 the name \`getenv\` is defined multiple times` in
  `build-tu lib/getopt.c`, and the same failure would hit this unit as soon as it included
  the mirror.  The local declaration was removed; with the mirror as the only home,
  `lib/getopt.c`, `lib/nproc.c`, `lib/strtoimax.c`, `lib/fcntl.c`, `lib/tempname.c`,
  `lib/utimensat.c` and `lib/fstatat.c` all build.
- **Mutation evidence for this pair** (the gate must fail on a deleted live call): deleting
  `cpu_quota ()` from `num_processors` -> `- function num_processors does not mention
  cpu_quota`; deleting `close (channel);` from `getloadavg` -> `- function getloadavg does
  not mention close`; deleting one enum member from `include/nproc.rs` -> `- missing constant
  NPROC_CURRENT_OVERRIDABLE`, and giving `NPROC_ALL` the value 5 -> `- constant NPROC_ALL:
  C 0 vs Rust enum 5`.  One mutation the token gate **cannot** see, recorded honestly:
  deleting C 558-560's `if (omp_env_limit == 0) omp_env_limit = ULONG_MAX;` still passes the
  token set (every token survives elsewhere in the body) — only the line budget notices, and
  the tolerance is set tight enough (0.53, ~5 lines) that a longer deletion trips it.
- Not verified: `gate-fast`/`gate-probes` (the main agent runs them) and any runtime test of
  `num_processors` (the unit is in the reference build, but exercising the OMP variables and
  the `sysctl` fallback needs a harness).
