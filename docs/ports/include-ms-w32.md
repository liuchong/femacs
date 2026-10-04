# emacs/nt/inc/ms-w32.h (501 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 107
- [x] BROKEN_DATAGRAM_SOCKETS  (line 94, body 356a192b7913)
- [x] DATA_START  (line 493, body dcf8d045cd5a)
- [x] EMACS_CONFIGURATION  (line 394, body b25589bc8215)
- [x] EMACS_CONFIG_OPTIONS  (line 395, body e0e72f4bd3d1)
- [x] ENOTSUP  (line 337, body 2e4c602e5ae5)
- [x] EOPNOTSUPP  (line 342, body 2a7541babb57)
- [x] HAVE_ATTRIBUTE_ALIGNED  (line 100, body 356a192b7913)
- [x] HAVE_C99_STRTOLD  (line 105, body 356a192b7913)
- [x] HAVE_LONG_LONG_INT  (line 483, body 356a192b7913)
- [x] HAVE_TIMEVAL  (line 90, body 356a192b7913)
- [x] HAVE_UNSIGNED_LONG_LONG_INT  (line 488, body 356a192b7913)
- [x] HAVE_WINDOW_SYSTEM  (line 160, body 356a192b7913)
- [x] HAVE___BUILTIN_UNWIND_INIT  (line 110, body 356a192b7913)
- [x] MAIL_USE_SYSTEM_LOCK  (line 96, body 356a192b7913)
- [x] MAXPATHLEN  (line 152, body d342b266157f)
- [x] MAX_UTF8_PATH  (line 156, body 92f9c8afd1e0)
- [x] MINGW_W64  (line 35, body da39a3ee5e6b)
- [x] NSIG  (line 333, body d435a6cdd786)
- [x] NULL  (line 455, body c21114673039)
- [x] PENDING_OUTPUT_COUNT  (line 76, body a0315cb688aa)
- [x] POLL_FOR_INPUT  (line 161, body 356a192b7913)
- [x] SA_RESTART  (line 359, body b6589fc6ab0d)
- [x] SIGALRM  (line 328, body fa35e192121e)
- [x] SIGCHLD  (line 329, body 9e6a55b6b456)
- [x] SIGHUP  (line 323, body 356a192b7913)
- [x] SIGKILL  (line 326, body 0ade7c2cf97f)
- [x] SIGPIPE  (line 327, body bd307a3ec329)
- [x] SIGPROF  (line 330, body b3f0c7f6bb76)
- [x] SIGQUIT  (line 324, body 77de68daecd8)
- [x] SIGTRAP  (line 325, body ac3478d69a3c)
- [x] SIG_BLOCK  (line 360, body 356a192b7913)
- [x] SIG_SETMASK  (line 361, body da4b9237bacc)
- [x] SIG_UNBLOCK  (line 362, body 77de68daecd8)
- [x] W32_DAEMON_EVENT  (line 499, body 141d5f85b641)
- [x] WIN32_LEAN_AND_MEAN  (line 147, body da39a3ee5e6b)
- [x] WINDOWSNT  (line 24, body da39a3ee5e6b)
- [x] Wcm_clear  (line 266, body 65d61abc37a8)
- [x] _CALLBACK_  (line 86, body 8dcd8ad2fee6)
- [x] _VARARGS_  (line 81, body 8dcd8ad2fee6)
- [x] _WIN32_WINNT  (line 143, body e0b28e99ef1e)
- [x] _WIN32_WINNT_VISTA  (line 54, body 723bc119dec1)
- [x] _WIN32_WINNT_WIN10  (line 66, body 4bc66cdbb61c)
- [x] _WIN32_WINNT_WIN2K  (line 45, body 5b7ae6d1c8ca)
- [x] _WIN32_WINNT_WIN7  (line 57, body 5da23f1519ac)
- [x] _WIN32_WINNT_WIN8  (line 60, body 7796089e0c69)
- [x] _WIN32_WINNT_WINBLUE  (line 63, body e6d7b34d15c7)
- [x] _WIN32_WINNT_WINXP  (line 48, body 2de6604acfd3)
- [x] _WIN32_WINNT_WS03  (line 51, body a7509f2abb2b)
- [x] _WINSOCKAPI_  (line 401, body 356a192b7913)
- [x] _WINSOCK_H  (line 406, body da39a3ee5e6b)
- [x] __POSIX_2008_DEPRECATED  (line 40, body da39a3ee5e6b)
- [x] _longjmp  (line 284, body db80fe1858b7)
- [x] calloc  (line 446, body e0b3adf9fd92)
- [x] chcheckmagic  (line 262, body 0b8ed836d7f2)
- [x] chdir  (line 207, body b137868f13b0)
- [x] chmod  (line 209, body 18b30d090c01)
- [x] clock  (line 247, body 273caebed7c8)
- [x] close  (line 211, body 1b4131c956e0)
- [x] closedir  (line 233, body f884593db480)
- [x] cmcostinit  (line 263, body 57c8505af4e5)
- [x] cmgoto  (line 264, body ad1e1ca2efab)
- [x] cmputc  (line 265, body f396fc36b59a)
- [x] creat  (line 213, body bd9858f2db78)
- [x] ctime  (line 214, body 16d17a940763)
- [x] dup  (line 216, body a0ad296bd613)
- [x] dup2  (line 218, body 5cf00f23d540)
- [x] emacs_raise  (line 255, body 8a6628861184)
- [x] execvp  (line 285, body a9e6a75c7101)
- [x] fdopen  (line 279, body 3b19e8fce51e)
- [x] fileno  (line 308, body f3da3d29faba)
- [x] fopen  (line 219, body 3464c539b7de)
- [x] free  (line 444, body ffebd37e9cf2)
- [x] fsync  (line 280, body 14237b233918)
- [x] ftruncate  (line 281, body 3eac18ca78c8)
- [x] getdefdir  (line 388, body 634357f1a456)
- [x] getdisk  (line 384, body 1d57dab07219)
- [x] getpid  (line 282, body 440456939c68)
- [x] isatty  (line 283, body a9e4af0e4432)
- [x] kill  (line 251, body 38c342c04c8d)
- [x] link  (line 220, body a0fb79909231)
- [x] localtime  (line 221, body 918e301880ac)
- [x] malloc  (line 443, body 16266941245c)
- [x] mkdir  (line 417, body 14080d1dd563)
- [x] open  (line 319, body aec07c3bbc4a)
- [x] opendir  (line 231, body 26f542b6b968)
- [x] pselect  (line 226, body 890de6aab39c)
- [x] putenv  (line 468, body dec24e83e66b)
- [x] read  (line 313, body b1b03607d3d5)
- [x] readdir  (line 235, body c438402d0b67)
- [x] realloc  (line 445, body 0b8907178464)
- [x] rename  (line 223, body d3ba17db60c4)
- [x] rmdir  (line 224, body e5506fe4b7ae)
- [x] seekdir  (line 237, body da6fed530a1e)
- [x] select  (line 225, body 890de6aab39c)
- [x] signal  (line 252, body 0e797ea536e7)
- [x] sigsetjmp  (line 201, body eb3e9305c655)
- [x] sleep  (line 227, body 713abf5c2e9b)
- [x] spawnve  (line 250, body f276849689d5)
- [x] strerror  (line 272, body 6764750dd091)
- [x] sys_nerr  (line 461, body 96348974bfc3)
- [x] tcdrain  (line 278, body 14237b233918)
- [x] tgetstr  (line 259, body 9c50282a457a)
- [x] tputs  (line 258, body 63703d02f014)
- [x] tzname  (line 298, body 7adeb914e850)
- [x] umask  (line 245, body bac4f3e4624c)
- [x] unlink  (line 229, body a370699a5755)
- [x] write  (line 243, body ec2711839143)

## Types and their fields — 1
- [x] sigaction: int sa_flags, sigset_t sa_mask

## File-scope objects — 3
- [x] extern malloc_fn the_malloc_fn  (line 439)
- [x] extern realloc_fn the_realloc_fn  (line 440)
- [x] extern free_fn the_free_fn  (line 441)
## Decisions

Mirror of nt/inc/ms-w32.h, complete for this target: 107 macros, `struct sigaction`'s three
members and the three allocator-switch objects, plus the declarations of every emulated entry
point (the `sys_*` family, the `*_before_dump`/`*_after_dump` family, the signal and allocator
entry points) grouped in `ms_w32_decls` with a glob import.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so the extractor
resolves no `#if` in it - `WINDOWSNT`, `MINGW_W64`, `__GNUC__`, `emacs`, `__MINGW32__` and
`_WIN64` are all live text and the mirror carries every arm's statements.  C defines four names
twice (`NULL`, `PENDING_OUTPUT_COUNT`, `getdefdir`, `sigsetjmp`); the extractor leaves their
bodies uncompared, and the mirror spells the arm C keeps, with the other named in a `///` note
above it.  `WIN32_LEAN_AND_MEAN` already has a row in `docs/ports/excluded-macros.txt`; the
mirror still carries the name, because C declares it here.

**Conversions**: an object-like rename (`#define close sys_close`) becomes a `macro_rules!` whose
body calls the target; a value alias keeps the alias in the initialiser (`MAXPATHLEN` is
`_MAX_PATH`, `ENOTSUP` is `ENOSYS`, `MAX_UTF8_PATH` is `MAXPATHLEN * 4`), so the C token is still
visible; `W32_DAEMON_EVENT` is a `&core::ffi::CStr` constant (PORTING.md 2.1, status 3.91); the
two `__cdecl` macros (`_VARARGS_`, `_CALLBACK_`) have no Rust spelling - the port writes the ABI
as `extern "C"` on the block, and both are attribute prefixes a `macro_rules!` cannot produce -
so they are two of the rows this round appends to `docs/ports/excluded-macros.txt`.

**Names with no mirror**: `sigset_t` (the conditional `typedef _sigset_t sigset_t`, which C only
makes under `MINGW_W64 && !_POSIX`), `mode_t`, `pid_t`, `time_t`, `intptr_t`, `size_t`,
`struct tm`, `_MAX_PATH` and the CRT's `_ptr`/`_base` fields - all MinGW runtime names, spelled as
C spells them and named from the including unit (`nt/inc/sys/include/acl.rs`'s `ssize_t` shape).

**Known deviation**: C's rename macros also rewrite *bare identifier uses* (an `&close` becomes
`&sys_close`); a Rust `macro_rules!` covers only the call form, so a ported unit that passes one
of these names as a value must name the target itself.  Recorded here rather than papered over.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/ms-w32.h
femacs/nt/inc/include/ms-w32.rs` -> `OK` (with the expected notes that three macros have recorded
counterparts and four are defined twice).  This mirror needs no `docs/ports/macro-body-notes.tsv`
row.
