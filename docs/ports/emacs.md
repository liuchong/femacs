# emacs/src/emacs.c (3653 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 5
- [x] invocation-name  (Finvocation_name, 0/0, doc 2ac74ba1db94, line 658)
- [x] invocation-directory  (Finvocation_directory, 0/0, doc 3e380b7ee0d2, line 666)
- [x] kill-emacs  (Fkill_emacs, 0/2, doc c794e55ea0b7, line 2953)
- [x] daemonp  (Fdaemonp, 0/0, doc c1ca42717657, line 3398)
- [x] daemon-initialized  (Fdaemon_initialized, 0/0, doc 2e64b2b091d7, line 3412)

## Functions defined — 20
- [x] using_utf8  (line 415, static)
- [x] terminate_due_to_signal  (line 441)
- [x] init_cmdargs  (line 489, static)
- [x] argmatch  (line 686, static)
- [x] default_PATH  (line 743, static)
- [x] follow_if_symlink  (line 794, static)
- [x] find_emacs_executable  (line 817, static)
- [x] dump_error_to_string  (line 907, static)
- [x] load_pdump  (line 938, static)
- [x] main  (line 1317)
- [x] copy_args  (line 2780, static)
- [x] sort_args  (line 2803, static)
- [x] shut_down_emacs  (line 3103)
- [x] fixup_locale  (line 3193)
- [x] synchronize_locale  (line 3203, static)
- [x] synchronize_system_time_locale  (line 3229)
- [x] synchronize_system_messages_locale  (line 3242)
- [x] emacs_strerror  (line 3253)
- [x] decode_env_path  (line 3261)
- [x] syms_of_emacs  (line 3477)

## Macros defined — 3
- [x] INLINE  (line 21, body 21a632056285)
- [x] MAIN_PROGRAM  (line 33, body da39a3ee5e6b)
- [x] SECCOMP_USABLE  (line 79, body b6589fc6ab0d)

## Types and their fields — 1
- [x] standard_args: const char* name, const char* longname, int priority, int nargs

## File-scope objects — 22
- [x] static const char emacs_version  (line 159, static)
- [x] static const char emacs_copyright  (line 160, static)
- [x] static const char emacs_bugreport  (line 161, static)
- [x] Lisp_Object empty_unibyte_string  (line 170)
- [x] Lisp_Object empty_multibyte_string  (line 170)
- [x] struct gflags gflags  (line 181)
- [x] bool initialized  (line 182)
- [x] bool inhibit_window_system  (line 186)
- [x] bool running_asynch_code  (line 190)
- [x] bool noninteractive  (line 218)
- [x] bool no_site_lisp  (line 221)
- [x] bool build_details  (line 224)
- [x] static char daemon_name  (line 227, static)
- [x] int daemon_type  (line 231)
- [x] static int daemon_pipe  (line 236, static)
- [x] char initial_argv  (line 255)
- [x] int initial_argc  (line 256)
- [x] char initial_argv0  (line 258)
- [x] static char initial_emacs_executable  (line 259, static)
- [x] bool fatal_error_in_progress  (line 411)
- [x] static Lisp_Object Vprevious_system_time_locale  (line 3225, static)
- [x] static Lisp_Object Vprevious_system_messages_locale  (line 3236, static)

## Decisions

- **What the file carries.** C's 5 `DEFUN`s, its 20 functions (9 non-`static`:
  `terminate_due_to_signal`, `main`, `shut_down_emacs`, `fixup_locale`,
  `synchronize_system_time_locale`, `synchronize_system_messages_locale`, `emacs_strerror`,
  `decode_env_path`, `syms_of_emacs`), the 3 file-local macros (`INLINE`, `MAIN_PROGRAM`,
  `SECCOMP_USABLE`), the one type it declares (`struct standard_args`, plus the two file-local
  arrays `standard_args` and `usage_message` that the extractor's object list does not key),
  the 22 file-scope objects, the 21 `DEFVAR_*` variables whose storage this unit owns
  (status 3.29 — the extractor cannot see them, which is why `tools/check-globals.py`
  exists), and two objects the extractor's list misses: `RCS_Id` (because of
  `EXTERNALLY_VISIBLE`) and `emacs_wd` (lisp.h 5131 declares it and the object is defined
  here; its `type *name` line is not a shape the field scanner keys).  The 9 non-`static`
  functions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the 11 `static` helpers stay plain
  `unsafe fn`.
- **`main` and the one tool change this unit needed.**  C's entry point is written as
  `#if !defined HAVE_ANDROID || defined ANDROID_STUBIFY` / `int` / `main (…)` / `#else` /
  `int` / `android_emacs_init (…)` / `#endif` / `{`, and
  `tools/cinventory.py`'s `function_spans` looks for the `{` past preprocessor lines but
  stopped at the *dead* arm's `int` line: the file's entry point was not one of its functions,
  so `#[unsafe(no_mangle)]` on the port's definition was reported as an invented exported
  symbol and no body check covered it.  The fix (this commit) skips the lines the frozen
  configuration disables while looking for the body; its impact was measured over every
  `*.c` of the reference tree outside `test/` by diffing the old and the new
  `inventory()['functions']`: **exactly one file changes, `src/emacs.c: +['main'] -[]`**.
  The ledger therefore lists `main` (line 1317) among the file's 20 functions and the port
  keeps C's linkage; `nm -g build/femacs/emacs.o` shows `_main`.
- **Dead arms recorded, not transcribed (P-08).**  `src/emacs.c` is in
  `docs/baseline/config/cond-eval-verified.txt` and
  `python3 tools/check-cond-eval.py src/emacs.c` reports
  `172 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the frozen configuration's
  branch selection is clang's and the arms it does not compile have **no Rust counterpart**.
  The families (C line numbers): `HAVE_ANDROID` (37-38, 41-42, 143-144, 424-429, 940-961,
  2448-2459, 2562-2566, 2623-2630, 2985-2986, 3042-3059, 3065-3070), `WINDOWSNT`/`HAVE_NTGUI`
  (45-51, 54-57, 172-178, 237-238, 497-507, 556-570, 823-827, 1054-1059, 1128-1134, 1395-1399,
  1401-1421, 1433-1440, 1732-1733, 1922-1930, 2181-2195, 2286-2288, 2353-2354, 2398-2402,
  2405-2410, 2480-2485, 2505-2515, 2575-2579, 3210-3218, 3268-3279, 3300-3301, 3359-3368,
  3463-3468, 3635-3643, 3649-3651), `MSDOS` (64-66, 584-589, 610-613, 964-965, 1635-1639,
  1922-1934, 2149-2156, 2413-2418, 3172-3173), `CYGWIN` (60-61, 2356-2357),
  `HAVE_LIBSYSTEMD` (69-71, 1793-1805, 3015-3019, 3432-3433), the seccomp pair
  `HAVE_LINUX_SECCOMP_H`/`HAVE_LINUX_FILTER_H` (74-77) and `SECCOMP_USABLE` (82-86, 304-307,
  1156-1312 — the whole `emacs_seccomp`/`read_full`/`load_seccomp`/`maybe_load_seccomp` region
  — plus 1337-1338, 2733-2734), `HAVE_WINDOW_SYSTEM` (89-90, 1682-1683, 2004-2005, 2363-2370,
  2551-2552), `HAVE_HAIKU` (120-121, 213-214, 2437-2446, 2519-2520, 3119-3121), `PROFILING`
  (124-131, 2603-2610), `_CS_PATH`'s `#elif DOS_NT` (776-780), `DOS_NT` (974-975, 3304-3347),
  `NS_SELF_CONTAINED` (981-982, 3296-3298), `HAVE_MODULES` (293-296, 2037-2046, 2342-2343,
  2671-2672), `HAVE_X_WINDOWS`/`HAVE_PGTK` (192-194, 2086-2118, 2372-2383, 2535-2536,
  3022-3024), `HAVE_NS` (1424-1426, 2049-2077, 2291-2295, 2420-2425, 2736-2742, 3176-3177),
  `HAVE_PTHREAD && !SYSTEM_MALLOC && !DOUG_LEA_MALLOC` (1939-1945), `DAEMON_MUST_EXEC`
  (1344-1345, 1834-1838, 1875-1914), `HAVE_TEXT_CONVERSION` (2007-2008), the notification and
  image back-ends `HAVE_KQUEUE`/`HAVE_GFILENOTIFY`/`HAVE_DBUS`/`HAVE_INOTIFY`/`HAVE_SOUND`/
  `HAVE_ZLIB`/`HAVE_LCMS2`/`HAVE_LIBXML2`/`HAVE_X_SM`/`HAVE_W32NOTIFY` (2159-2164, 2348-2349,
  2388-2393, 2464-2477, 2482-2484, 2546-2549, 3180-3185), `HAVE_NATIVE_COMP` (2496-2498,
  2587-2595, 3039-3040), and MSDOS's `set_binary_mode` (1625-1632).  One condition clang
  compiles that is *not* a configuration macro at all is `#ifdef LC_MESSAGES` (C 3235-3237,
  3244-3247, 3576-3579): it is a **system** macro (`<locale.h>` defines it as 6), so the port
  keeps the text live and declares the constant at the use site — it is not a `#[cfg]`.
- **The one tool/mirror defect no gate sees: `config.rs`'s `SYSTEM_TYPE`.**  `config.h`'s
  `#define SYSTEM_TYPE "darwin"` reaches `config.rs` as the macro **body**, i.e.
  `pub const SYSTEM_TYPE: &str = "\\\"darwin\\";`, whose value still carries C's quotes; that
  is not the C string value `intern_c_string` needs (and its type is `&str`, not a pointer).
  The port writes C's value at the use site and records the missing token in
  `docs/ports/macro-body-notes.tsv` (the `syms_of_emacs` row); the mirror owner's fix is to
  decode the body in `xtask gen-config` the way `extract-abi.py` decodes a C string literal.
- **Use-site declarations (the mirror-owner list).**  Names no mirror in this unit's include
  chain carries are declared in the unit's own `unsafe extern "C"` block, each with C's
  prototype in a `/// C:` line (PORTING.md 0.1-2/P-55).  By owner: **lisp.rs** needs
  `check_message_stack`, `clear_message_stack`, `emacs_backtrace`,
  `emacs_get_current_dir_name`, `file_access_p`, `hexbuf_digest`, `fatal`, `openp`, the nine
  `init_*_once` it lacks (`init_alloc_once`, `init_obarray_once`, `init_eval_once`,
  `init_charset_once`, `init_coding_once`, `init_syntax_once`, `init_category_once`,
  `init_casetab_once`, `init_buffer_once`, `init_minibuf_once`, `init_print_once`,
  `init_frame_once`, `init_window_once`), `init_alloc`, `init_buffer`, `init_editfns`,
  `init_eval`, `init_fileio`, `init_font`, `init_lread`, `init_process_emacs`, `init_random`,
  `init_standard_fds`, `init_xdisp`, `init_xfaces`, `init_charset`, `init_macros`,
  `init_keyboard`, `init_callproc`, `init_callproc_1`, `keys_of_keyboard`,
  `kill_buffer_processes`, `message_dolog`, `reset_all_sys_modes`, `safe_run_hooks`,
  `set_initial_environment`, `set_initial_minibuffer_mode`, `unlock_all_files`, the
  `syms_of_*` family (`alloc`, `buffer`, `bytecode`, `callint`, `callproc`, `casefiddle`,
  `casetab`, `category`, `ccl`, `character`, `charset`, `chartab`, `cmds`, `coding`, `comp`,
  `data`, `dired`, `display`, `doc`, `editfns`, `eval`, `fileio`, `filelock`, `floatfns`,
  `fns`, `font`, `frame`, `indent`, `insdel`, `json`, `keyboard`, `lread`, `macros`, `menu`,
  `minibuf`, `pdumper`, `print`, `process`, `profiler`, `search`, `sqlite`, `syntax`,
  `sysdep`, `term`, `terminal`, `textprop`, `threads`, `timefns`, `undo`, `window`, `xdisp`,
  `xfaces`, `xml`, `xwidget`), `emacs_pipe`, `emacs_perror`, `emacs_open_noquit`,
  `emacs_unlink`, `emacs_write` and the object `emacs_wd`; **dispextern.rs** needs
  `check_glyph_memory`, `unrequest_sigio`, `init_display`, `syms_of_display`; **window.rs**
  needs `init_window_once`, `init_window`, `syms_of_window`; **keyboard.rs** needs
  `stuff_buffered_input` and the `extern char const DEV_TTY[]` object; **sysstdio.rs** needs
  `close_output_streams`; **regex-emacs.rs** needs `emacs_re_max_failures` and
  `emacs_re_safe_alloca`; **process.rs** needs nothing here (it already carries
  `inhibit_sentinels`); `<stdio.h>`'s mirror needs `printf`, `fprintf`, `fflush`, `clearerr`
  and the three `FILE *` objects, `<stdlib.h>`'s `exit`, `atexit`, `confstr`, `realpath` and
  `EXIT_SUCCESS`/`EXIT_FAILURE`, `<string.h>`'s `strcspn`/`strerror`, `<unistd.h>`'s `fork`,
  `setsid`, `getpgrp`, `tcgetpgrp`, `execvp` and the three descriptor constants,
  `<signal.h>`'s `SIG_DFL`, `<sys/resource.h>`'s `RLIMIT_STACK`, `<limits.h>`'s `PIPE_BUF`,
  `<locale.h>`'s `setlocale` and four categories (there is no `<locale.h>` mirror at all),
  `<unistd.h>`'s `_CS_PATH`, and `treesit.h`/`xwidget.h`/`gnutls.h` (no mirror exists for any
  of the three) need `syms_of_treesit`, `syms_of_xwidget`, `syms_of_gnutls`.  `#include`s
  with no mirror whose names this file does *not* use (`<sys/file.h>`, `getpagesize.h`,
  `sysselect.h`, `xwidget.h`) are recorded here instead of being declared.
- **One mirror *signature* deviation, visible only at the call site.**  lisp.h 5274-5280's
  live arm is `INLINE int maybe_disable_address_randomization (int argc, char **argv)`, and
  lisp.rs implements it with the one parameter it reads (`argc`); C's call in `main`
  therefore becomes a one-argument call in the port, and this row is the record.
- **Spelling deviations (each recorded in one place).**
  * `switch (result)` in `dump_error_to_string` selects among `pdumper.h`'s enumeration
    constants; Rust cannot use a cast (`PDUMPER_LOAD_SUCCESS as c_int`) as a match pattern,
    so C's arms are an if/else chain in the same order with C's `default` as the tail
    (`macro-body-notes.tsv`: `case`, `default`, `switch`).
  * C's two `goto`s in `load_pdump` are labelled blocks (PORTING.md 2.2): `goto hardcoded`
    is `break 'hardcoded` out of the inner block that carries C's `hardcoded:` label, and
    `goto out` is `break 'out` out of the outer one; the `out:` label's own two statements
    follow the block, so both paths reach them.  `sort_args`'s `goto done` (its `done: ;`
    label sits at the enclosing block's tail, after the long-option scan) is the port's
    labelled block with `break 'done`, so the label's name survives and only the `goto`
    keyword is the notes row's.
  * C's `do { … } while (*path++ != '\0');` keeps the post-increment in its condition: the
    port's `loop` ends with `if ! { let c = *path; path = path.offset (1); c != 0 }
    { break; }` (the `compute_motion` shape the indent ledger records).
  * `memcpy`/`mempcpy` are the port's `std::ptr::copy_nonoverlapping` (the `memcpy` family in
    `token-aliases.tsv`, whose occurrence check keeps all four calls alive); `mempcpy`'s
    *return* value is spelled as the arithmetic it performs
    (`buf.as_mut_ptr ().offset (nlen as isize + sig_desclen)`).
  * `NULL` is `std::ptr::null ()`/`null_mut ()` (the `null` family's folding), except C's
    literal `0` third argument of `pthread_sigmask`, which stays `0 as *mut sigset_t` so the
    token survives (the `0`-for-`NULL` class).
  * C's `bool` values are Rust `true`/`false` (`fatal_error_in_progress = 1`,
    `inhibit_window_system = 1`, `inhibit_x_resources = 0`, `err |= …`); the `0` that leaves
    `syms_of_emacs`'s body with it is the notes row's.
  * `eassert`/`eassume` are the mirror's `eassert!`/`eassume!`; `AUTO_STRING`,
    `XSETFASTINT`, `countof`, `emacs_raise`, `calln`, `S_ISREG`, `IS_DIRECTORY_SEP`,
    `DEFSYM`, `DEFVAR_LISP`/`DEFVAR_BOOL` and `DEFUN` keep their C names as the mirror's
    macros (the `DEFVAR_*` docstrings are the macro's third argument, so C's `doc:` label has
    no counterpart — the `syms_of_indent` row).
  * `AVOID` is `-> !` (`terminate_due_to_signal`); `main` returns `c_int` and ends at C's
    `eassume (false)` with `std::hint::unreachable_unchecked ()`, the port's spelling of the
    unreachable end C's macro asserts.
  * C's `sizeof fingerprint` is `FINGERPRINT_SIZE` (`size_of::<[c_uchar; 32]> ()`) and the
    object is reached as `(&raw const fingerprint) as *const c_void`; C's VLA
    `char buf[max ((sizeof fmt - sizeof "%d" + INT_STRLEN_BOUND (int) + 1),
    min (PIPE_BUF, MAX_ALLOCA))];` is a `const buf_size` whose folds are an if/else (neither
    `Ord::min` nor `Ord::max` is const on this toolchain — the `BUF_BYTES_MAX` row's class,
    and the notes row carries `max`/`min`); `static char const fmt[]` is `static fmt: &CStr`.
  * `struct standard_args`'s array is `static mut`, not `static`: Rust's `static` requires
    `Sync` and a raw pointer is not (the callint.rs `callint_argfuns` shape); its six
    clang-measured assertions pin `sizeof` 24, `alignof` 8 and the four offsets (`printf`
    measurement: `24 8 0 8 16 20`).
  * `usage_message` is `[&CStr; 13]`: C's array has thirteen live elements and the two dead
    `#if` ones are recorded above; its `PACKAGE_BUGREPORT` concatenation is the frozen value,
    and the strings keep C's bytes (real newlines, the tab in the `-Q` entry).
  * `RCS_Id` keeps C's object (an `EXTERNALLY_VISIBLE` `char const[]`) as
    `#[unsafe(no_mangle)] pub static RCS_Id: &CStr`; `strings build/femacs/emacs.o` shows the
    `$Id: …$` bytes.
  * Uninitialised locals (PORTING.md 1.5): every "declared then assigned" local of
    `init_cmdargs`, `argmatch`, `load_pdump`, `main`, `sort_args`, `Fkill_emacs`,
    `decode_env_path` and `Fdaemon_initialized` starts at C's zero (`Qnil ()`, `0`,
    `std::ptr::null_mut ()`); the parameters C assigns to are shadowed (`dump_file`,
    `temp`, `data`) or `mut` in the signature (`main`'s `argc`).
  * Descending `for` loops are `while 0 <= i { …; i -= 1; }`; C's `argv[i]`/`argv[i+1]` are
    `*argv.offset (i as isize)`; `CStr::as_ptr`/`is_null` replace C's implicit conversions.
- **`line` and `column` counts:** C 3653 (live 2731) vs the port's 2969.  The dead region is
  recorded rather than transcribed (P-08), so the row in `docs/ports/line-drift-tolerance.tsv`
  declares the **`live`** basis with 0.10 (measured 8.7%); the per-function token comparison is
  the real guard and it is `OK`.

### Call-relationship differences (D7)

**None beyond the recorded spellings.**  Every call in the C text appears in the port with the
same callee and the same argument order — a sequence comparison of the call names in each of
the file's 25 bodies (C vs. the port, macro forms included) shows no reordering; the only
names that differ are the six the two entries above record (`memcpy`/`mempcpy` as the
`memcpy` family and `max`/`min` in the const fold).  The mirror accessors the port calls
(`Qnil ()`/`Qt ()`/`Qload ()` and the other `Q*` functions, `IS_DAEMON ()`, `DAEMON_RUNNING ()`,
`XSETFASTINT!`, `calln!`, `emacs_raise!`, `S_ISREG!`, `AUTO_STRING!`, `countof!`,
`eassert!`/`eassume!`) are C's own macros expanding at the same points, and the port adds no
call the reference does not make: the other Rust-only identifiers in the bodies are the
`as_ptr`/`offset`/`is_null`/`null_mut`/`zeroed`/`copy_nonoverlapping` spellings of C's pointer,
`NULL`, `sizeof` and `memcpy` operations.
