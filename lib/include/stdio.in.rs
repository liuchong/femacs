// femacs: port of lib/stdio.in.h @ fd7afced403 (gnulib's <stdio.h>; PORTING.md P-26).
//
// **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/stdio.h`
// (107,390 bytes, 2,771 lines) is generated from this template by `gnulib.mk`'s rule, and it is
// not pre-digested: every `@PLACEHOLDER@` becomes a literal number there, which is what decides
// each arm.  The diff that names them all is
//
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/stdio.in.h) build/emacs-ref/lib/stdio.h
//     (1,150 lines, 877 of them changed; most of it is the `c++defs.h`/`arg-nonnull.h`/
//      `warn-on-use.h` text the build inlines, plus the build's own `@GNULIB_FOO@` legend)
//
// `tools/extract-abi.py` cannot read those numbers, so its item list is the union of every arm
// (P-66: `check-cond-eval.py lib/stdio.in.h` -> `360 conditions, 155 unknown, 214 disagreements
// [NOT VERIFIED]`).  The arms are decided by the **frozen preprocessor** on the generated header:
//
//     $ clang -E -dM -std=gnu23 -include build/emacs-ref/src/config.h \
//         -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src -I emacs/lib \
//         build/emacs-ref/lib/stdio.h | grep '^#define '
//     -> fourteen live macros: the eleven `_GL_ATTRIBUTE_*`/`_GL_STDIO_*`/`_PRINTF_NAN_LEN_MAX`
//        spellings this mirror carries below, `_GL_ATTRIBUTE_DEALLOC`/`…_DEALLOC_FREE`/
//        `…_FORMAT`/`…_MALLOC`/`…_NOTHROW` (which `attribute.rs` already has), and *no* `#define
//        X rpl_X` rename at all - so every `#  define fclose rpl_fclose` in the template is dead
//        text on this target and the live arm declares the *system's* function instead;
//
//     $ clang -E -std=gnu23 … build/emacs-ref/lib/stdio.h | grep -c '\bX\b'   (per function name)
//     -> 49 of the entry points are *declared* by the frozen translation unit (the SDK's
//        `<stdio.h>`/`<_stdio.h>`, or gnulib's own `_GL_FUNCDECL_SYS` where the system lacks one)
//        and 20 template names are absent from it altogether (the ledger's table).
//
// **`getline`'s judgement** (asked for this round, measured the same way): the generated header
// has `#if 1` at 1684 (C 1137's `#if @GNULIB_GETLINE@`, so **`@GNULIB_GETLINE@` is 1**) and
// `# if 0` at 1691 (C 1144's `# if @REPLACE_GETLINE@`, so **`@REPLACE_GETLINE@` is 0**).  The RPL
// arm - `_GL_FUNCDECL_RPL (getline, …)` (1685-1692), the C++ inline definition (1696-1706) that
// the extractor reads as the file's one *function*, and `#   define getline rpl_getline`
// (1711) - is therefore dead text, and the live arm is `# else` at 1709: `#  if !1` at 1717 (C
// 1157's `#  if !@HAVE_DECL_GETLINE@`, so `HAVE_DECL_GETLINE` is 1) skips even gnulib's own
// `_GL_FUNCDECL_SYS (getline, …)` at 1718 and leaves `_GL_CXXALIAS_SYS (getline, …)`: **`getline`
// is the system's**, which the declaration below says.
//
// **The system half is `stdio.rs`** - C 47/52's `# @INCLUDE_NEXT@ @NEXT_STDIO_H@`, the frozen
// `<stdio.h>` (which brings `FILE` and the 20 entry points it already carries) - and
// `attribute.rs` carries the five `_GL_ATTRIBUTE_*` names C 110-290's fallback guards define
// (each `#ifndef`-guarded, so `lib/attribute.h`'s own text wins when a unit includes it first).
//
// **What this mirror carries.**  The template adds to the system header: the `_POSIX_C_SOURCE`
// value C 41-42 sets for macOS (dead here - the SDK's version guard does not fire - and carried
// as the value C's arm writes), the six format/scan attribute spellings and the two
// stringification macros, `_PRINTF_NAN_LEN_MAX` (the BSD arm's `3`), the ten
// `GNULIB_overrides_*` markers (each in a dead arm, carried as the value C sets), the two
// `_GL_FSEEK_WARN`/`_GL_FTELL_WARN` category markers, and the 32 declarations of the entry points
// `stdio.rs` does not already have.
//
// **What it does not carry, and why (the ledger's table has the line numbers).**  Six names live
// only in arms the frozen configuration does not take and have no counterpart anywhere on this
// target, so the mirror carries nothing for them: `fcloseall` (C 426's `@GNULIB_MDA_FCLOSEALL@`),
// `fwrite_unlocked` (C 1035-1049's glibc `__REDIRECT` arm), `obstack_printf`/`obstack_vprintf`
// (C 1248's `@GNULIB_OBSTACK_PRINTF@ || @GNULIB_OBSTACK_PRINTF_POSIX@`), `rpl_getdelim`
// (C 1103-1105, dead because `getdelim` is the SDK's) and `__has_feature` (C 1096-1097, dead
// because the frozen compiler *has* `__has_feature` - measured: `#ifdef __has_feature` answers
// true).  P-66 rule 2 asks for the boundary and the line numbers instead of a transcription, so
// those six are recorded in `docs/ports/excluded-macros.txt` (the ledger lists the rows to add).

include!("stdio.rs");        // C 47/52: `# @INCLUDE_NEXT@ @NEXT_STDIO_H@` - the system half
include!("attribute.rs");    // C 110-290: the `_GL_ATTRIBUTE_*` spellings

/* ---- C 36-44: the `_POSIX_C_SOURCE` prologue -----------------------------------------------
   C 36-44 is `#if (defined __APPLE__ && defined __MACH__) && !defined _POSIX_C_SOURCE` …
   `#  if (defined MAC_OS_X_VERSION_MIN_REQUIRED && 130000 <= MAC_OS_X_VERSION_MIN_REQUIRED)` …
   `#  define _POSIX_C_SOURCE 200809L` / `#  define _GL_DEFINED__POSIX_C_SOURCE`.  The frozen SDK's
   `AvailabilityMacros.h` leaves `MAC_OS_X_VERSION_MIN_REQUIRED` at 150000, so this arm is
   **live** in the configuration's own terms - but the preprocessor's macro table has no
   `_POSIX_C_SOURCE` (measured), because `build/emacs-ref/lib/stdio.h` is reached *after* the
   system's `<stdio.h>` has already pulled its own headers in; the value C writes is 200809.  */
/// C 41: `#  define _POSIX_C_SOURCE 200809L` - C's suffix is Rust's absence of one.
/// C: #  define _POSIX_C_SOURCE 200809L
pub const _POSIX_C_SOURCE: c_long = 200809;
/// C 42: `#  define _GL_DEFINED__POSIX_C_SOURCE` - the marker C 54-57 `#undef`s again; it has no
/// value in C and no counterpart in the port, so the name is carried as a no-op macro (the
/// `attribute.rs` shape for a name whose expansion is nothing).
/// C: #  define _GL_DEFINED__POSIX_C_SOURCE
macro_rules! _GL_DEFINED__POSIX_C_SOURCE { () => {}; }

/* ---- C 191-247: the format/scan attribute spellings -----------------------------------------
   Each is an `#if`/`#else` pair over the compiler's capabilities, and the frozen clang takes the
   arm the `-dM` table shows: `__gnu_printf__` needs GCC, so C 195-199's `#else` gives
   `_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD` the value `__printf__`; C 203-208's
   `@GNULIB_PRINTF_ATTRIBUTE_FLAVOR_GNU@` is 0, so `…_SYSTEM` is `__printf__` too; C 227-233's
   `__GNUC__ > 4` arm gives `…_FORMAT_SCANF` `__gnu_scanf__` (its `#else` is the dead
   `__scanf__` spelling the extractor records, which is why the two `…_SCANF` names are branch
   macros and their bodies are not compared).  The port spells each name as the *macro* whose
   expansion is the attribute C's text names - `attribute.rs`'s convention is that a Rust
   attribute is a different mechanism, so this header's `_GL_ATTRIBUTE_FORMAT` (from
   `attribute.rs`) expands to nothing and these six only have to forward to it.  */
/// C 197: `#  define _GL_ATTRIBUTE_SPEC_PRINTF_STANDARD __printf__` (C 195's GCC arm is dead).
/// C: #  define _GL_ATTRIBUTE_SPEC_PRINTF_STANDARD __printf__
macro_rules! _GL_ATTRIBUTE_SPEC_PRINTF_STANDARD { () => { __printf__ }; }
/// C 206: `#  define _GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM __printf__` (C 203's GNU-flavor arm is dead).
/// C: #  define _GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM __printf__
macro_rules! _GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM { () => { __printf__ }; }
/// C 214-215: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD(formatstring_parameter, first_argument) _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD, formatstring_parameter, first_argument))`
/// C: #define _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD(formatstring_parameter, first_argument) \
/// C:   _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD, formatstring_parameter, first_argument))
macro_rules! _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD {
  ($formatstring_parameter:tt, $first_argument:tt) => {
    _GL_ATTRIBUTE_FORMAT! ((_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD, $formatstring_parameter,
                            $first_argument))
  };
}
/// C 221-222: the same shape over `_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM`.
/// C: #define _GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM(formatstring_parameter, first_argument) \
/// C:   _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM, formatstring_parameter, first_argument))
macro_rules! _GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM {
  ($formatstring_parameter:tt, $first_argument:tt) => {
    _GL_ATTRIBUTE_FORMAT! ((_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM, $formatstring_parameter,
                            $first_argument))
  };
}
/// C 229-230: `# define _GL_ATTRIBUTE_FORMAT_SCANF(formatstring_parameter, first_argument) _GL_ATTRIBUTE_FORMAT ((__gnu_scanf__, formatstring_parameter, first_argument))` - the frozen `__GNUC__ > 4` arm; C 232-233's `#else` spells `__scanf__` and is dead.
/// C: # define _GL_ATTRIBUTE_FORMAT_SCANF(formatstring_parameter, first_argument) \
/// C:    _GL_ATTRIBUTE_FORMAT ((__gnu_scanf__, formatstring_parameter, first_argument))
macro_rules! _GL_ATTRIBUTE_FORMAT_SCANF {
  ($formatstring_parameter:tt, $first_argument:tt) => {
    _GL_ATTRIBUTE_FORMAT! ((__gnu_scanf__, $formatstring_parameter, $first_argument))
  };
}
/// C 240-241: `#define _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM(formatstring_parameter, first_argument) _GL_ATTRIBUTE_FORMAT ((__scanf__, formatstring_parameter, first_argument))`.
/// C: #define _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM(formatstring_parameter, first_argument) \
/// C:   _GL_ATTRIBUTE_FORMAT ((__scanf__, formatstring_parameter, first_argument))
macro_rules! _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM {
  ($formatstring_parameter:tt, $first_argument:tt) => {
    _GL_ATTRIBUTE_FORMAT! ((__scanf__, $formatstring_parameter, $first_argument))
  };
}

/* ---- C 293-294: the stringification pair ----------------------------------------------------
   `#token` is C's stringification operator and Rust's `stringify!` is its exact counterpart, so
   both names are real macros here (the two callers are the printing paths that build a format
   string from a macro argument).  */
/// C 293: `#define _GL_STDIO_STRINGIZE(token) #token`
/// C: #define _GL_STDIO_STRINGIZE(token) #token
macro_rules! _GL_STDIO_STRINGIZE { ($token:tt) => { stringify! ($token) }; }
/// C 294: `#define _GL_STDIO_MACROEXPAND_AND_STRINGIZE(token) _GL_STDIO_STRINGIZE(token)`
/// C: #define _GL_STDIO_MACROEXPAND_AND_STRINGIZE(token) _GL_STDIO_STRINGIZE(token)
macro_rules! _GL_STDIO_MACROEXPAND_AND_STRINGIZE {
  ($token:tt) => { _GL_STDIO_STRINGIZE! ($token) };
}

/* ---- C 305-330: `_PRINTF_NAN_LEN_MAX` -------------------------------------------------------
   The chain's arms are `3` (BSD/Apple, C 310), `4` (glibc/musl/…), `5` (AIX), `10` (Windows)
   and `32` (the unknown-platform `#else` the extractor records last), so the macro has five
   definitions and no body is compared; the frozen arm is C 307-310's, measured with `clang -dM`
   (the frozen `<stdio.h>`'s table says `3`).  */
/// C 310: `#  define _PRINTF_NAN_LEN_MAX 3` - the frozen arm (BSD and macOS print a bare "nan").
/// C: #  define _PRINTF_NAN_LEN_MAX 3
pub const _PRINTF_NAN_LEN_MAX: c_int = 3;

/* ---- C 670-2200: the `GNULIB_overrides_*` markers -------------------------------------------
   Ten markers, each a `#  define GNULIB_overrides_X 1` inside the `#  if @REPLACE_X@` arm that
   prints a `#   define X rpl_X` rename.  Nine of the ten arms are dead here (the only live
   override on this target is the *function* redirect `free`, which `stdlib.in.rs` carries) and
   the port keeps each marker as the value C's arm sets - the `sys_types.in.rs`/`endian.in.rs`
   reading of a witness: the value is a fact about the header, and the port has no second
   inclusion for it to guard.  `GNULIB_overrides_asprintf` (C 1823) is the one C writes with *no*
   value, so it is a no-op macro rather than a constant.  */
/// C 670: `#  define GNULIB_overrides_fprintf 1` (the dead `@REPLACE_FPRINTF@` arm).
/// C: #  define GNULIB_overrides_fprintf 1
pub const GNULIB_overrides_fprintf: c_int = 1;
/// C 1422: `#  define GNULIB_overrides_printf 1` (the dead `@REPLACE_PRINTF@` arm).
/// C: #  define GNULIB_overrides_printf 1
pub const GNULIB_overrides_printf: c_int = 1;
/// C 1648: `#  define GNULIB_overrides_snprintf 1` (the dead `@REPLACE_SNPRINTF@` arm).
/// C: #  define GNULIB_overrides_snprintf 1
pub const GNULIB_overrides_snprintf: c_int = 1;
/// C 1713: `#  define GNULIB_overrides_sprintf 1` (the dead `@REPLACE_SPRINTF@` arm).
/// C: #  define GNULIB_overrides_sprintf 1
pub const GNULIB_overrides_sprintf: c_int = 1;
/// C 1847: `#  define GNULIB_overrides_vasprintf 1` (the dead `@REPLACE_VASPRINTF@` arm).
/// C: #  define GNULIB_overrides_vasprintf 1
pub const GNULIB_overrides_vasprintf: c_int = 1;
/// C 1968: `#  define GNULIB_overrides_vfprintf 1` (the dead vfprintf arm).
/// C: #  define GNULIB_overrides_vfprintf 1
pub const GNULIB_overrides_vfprintf: c_int = 1;
/// C 2058: `#  define GNULIB_overrides_vprintf 1` (the dead vprintf arm).
/// C: #  define GNULIB_overrides_vprintf 1
pub const GNULIB_overrides_vprintf: c_int = 1;
/// C 2139: `#  define GNULIB_overrides_vsnprintf 1` (the dead `@REPLACE_VSNPRINTF@` arm).
/// C: #  define GNULIB_overrides_vsnprintf 1
pub const GNULIB_overrides_vsnprintf: c_int = 1;
/// C 2195: `#  define GNULIB_overrides_vsprintf 1` (the dead `@REPLACE_VSPRINTF@` arm).
/// C: #  define GNULIB_overrides_vsprintf 1
pub const GNULIB_overrides_vsprintf: c_int = 1;
/// C 1823: `#  define GNULIB_overrides_asprintf` - the one marker C's arm writes with no value
/// (the extractor records an empty body), so the port keeps the name as a no-op macro.
/// C: #  define GNULIB_overrides_asprintf
macro_rules! GNULIB_overrides_asprintf { () => {}; }

/* ---- C 917-981: the two "category 1" warning markers ----------------------------------------
   `# define _GL_FSEEK_WARN /* Category 1, above.  */` and its `ftell` twin sit in
   `#elif defined GNULIB_POSIXCHECK` arms - dead on this target (`GNULIB_POSIXCHECK` is
   undefined) - and their bodies are a comment, i.e. nothing.  The port carries the names as the
   no-op macros their C bodies are (`attribute.rs`'s reading of a name whose expansion is
   nothing).  */
/// C 917: `# define _GL_FSEEK_WARN /* Category 1, above.  */` (the dead POSIXCHECK arm).
/// C: # define _GL_FSEEK_WARN /* Category 1, above.  */
macro_rules! _GL_FSEEK_WARN { () => {}; }
/// C 980: `# define _GL_FTELL_WARN /* Category 1, above.  */` (the dead POSIXCHECK arm).
/// C: # define _GL_FTELL_WARN /* Category 1, above.  */
macro_rules! _GL_FTELL_WARN { () => {}; }

/* ---- C 277-2210: the live declarations ------------------------------------------------------
   Every name here is declared by the frozen translation unit, measured by preprocesssing the
   generated header (`clang -E … | grep -c '\bX\b'`) and by dumping that unit's declarations
   (`clang -std=gnu23 -fsyntax-only -Xclang -ast-dump`); the signature in each `/// C:` line is the
   one that dump prints.  The 20 entry points `stdio.rs` already declares (`fclose`, `fopen`,
   `fread`, `fwrite`, `fputs`, `fputc`, `sprintf`, `getchar`, `putchar`, `puts`, `printf`,
   `perror`, `fprintf`, `putc`, `fflush`, `fileno`, `getc`, …) are *not* repeated: a second
   declaration of one name in one crate is `E0428`.  `off_t` and `ssize_t` are the system's types
   (`sys_types.in.rs` 59 spells `off_t` `i64`, `unistd.in.rs` 34 spells `ssize_t` `c_long`), spelled
   out here because C reaches them through the system `<stdio.h>`'s own includes rather than
   through this header, and `va_list` is the port's `core::ffi::VaList<'_>` (PORTING.md P-04, the
   `xdisp.rs`/`pdumper.rs` shape).  The whole-import module is `PORTING.md` 0.1-10's, and it has to be named
   `stdio_in_decls` rather than `stdio_decls`, because this mirror `include!`s `stdio.rs`, which
   already owns *that* module name: two top-level modules of one name in one crate are `E0428`,
   which the standalone probe measured (`the name `stdio_decls` is defined multiple times` at
   `stdio.rs` 23) before this rename.  The module is what lets a unit that declares one of these
   names itself - several do, `src/xdisp.rs` and `src/pdumper.rs` declare `vfprintf` - shadow the
   glob instead of colliding with it.  */
pub mod stdio_in_decls {
  use super::*;

  unsafe extern "C" {
    /// C 1085-1100: `_GL_FUNCDECL_SYS (getdelim, ssize_t, (char **restrict lineptr, size_t *restrict linesize, int delimiter, FILE *restrict stream), …);`
    pub fn getdelim (lineptr: *mut *mut c_char, linesize: *mut usize, delimiter: c_int,
                     stream: *mut FILE) -> c_long;
    /// C 1137-1182's live `# else` arm: the frozen translation unit declares `getline` itself
    /// (generated header 1718's `_GL_FUNCDECL_SYS` is skipped by `#  if !1` and `_GL_CXXALIAS_SYS`
    /// is what is left).
    pub fn getline (lineptr: *mut *mut c_char, linesize: *mut usize, stream: *mut FILE) -> c_long;
    /// C 375-410: `_GL_FUNCDECL_SYS (dprintf, int, (int fd, const char *restrict format, ...), …);`
    pub fn dprintf (fd: c_int, format: *const c_char, ...) -> c_int;
    /// C 1821: `_GL_FUNCDECL_RPL (asprintf, int, (char **result, const char *format, ...), …);`
    /// (the declaration is live; the `@REPLACE_VASPRINTF@` arm that renames it is not).
    pub fn asprintf (result: *mut *mut c_char, format: *const c_char, ...) -> c_int;
    /// C 1845: `_GL_FUNCDECL_RPL (vasprintf, int, (char **result, const char *format, va_list args), …);`
    pub fn vasprintf (result: *mut *mut c_char, format: *const c_char, args: core::ffi::VaList<'_>)
      -> c_int;
    /// C 517: `_GL_FUNCDECL_SYS (fdopen, FILE *, (int fd, const char *mode), );`
    pub fn fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
    /// C 556: `_GL_FUNCDECL_SYS (fgetc, int, (FILE *stream), …);`
    pub fn fgetc (stream: *mut FILE) -> c_int;
    /// C 572: `_GL_FUNCDECL_SYS (fgets, char *, (char *restrict s, int n, FILE *restrict stream), );`
    pub fn fgets (s: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    /// C 713-737: `_GL_FUNCDECL_SYS (fpurge, int, (FILE *stream), );`
    pub fn fpurge (stream: *mut FILE) -> c_int;
    /// C 796: `_GL_FUNCDECL_SYS (freopen, FILE *, (const char *restrict filename, const char *restrict mode, FILE *restrict stream), );`
    pub fn freopen (filename: *const c_char, mode: *const c_char, stream: *mut FILE) -> *mut FILE;
    /// C 824-881: `_GL_FUNCDECL_SYS (fscanf, int, (FILE *restrict stream, const char *restrict format, ...), …);`
    pub fn fscanf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
    /// C 881-904: `_GL_FUNCDECL_SYS (fseek, int, (FILE *stream, long offset, int whence), );`
    pub fn fseek (stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    /// C 904-918: `_GL_FUNCDECL_SYS (fseeko, int, (FILE *stream, off_t offset, int whence), …);`
    /// (`off_t` is `i64` on this target, `sys_types.in.rs` 59.)
    pub fn fseeko (stream: *mut FILE, offset: i64, whence: c_int) -> c_int;
    /// C 946-967: `_GL_FUNCDECL_SYS (ftell, long, (FILE *stream), );`
    pub fn ftell (stream: *mut FILE) -> c_long;
    /// C 967-981: `_GL_FUNCDECL_SYS (ftello, off_t, (FILE *stream), …);` (`off_t` is `i64`.)
    pub fn ftello (stream: *mut FILE) -> i64;
    /// C 1204-1240: `_GL_FUNCDECL_SYS (getw, int, (FILE *restrict stream), );`
    pub fn getw (stream: *mut FILE) -> c_int;
    /// C 1337: `_GL_FUNCDECL_SYS (popen, FILE *, (const char *command, const char *mode), );`
    pub fn popen (command: *const c_char, mode: *const c_char) -> *mut FILE;
    /// C 1497: `_GL_FUNCDECL_SYS (putw, int, (int w, FILE *restrict stream), );`
    pub fn putw (w: c_int, stream: *mut FILE) -> c_int;
    /// C 1518: `_GL_FUNCDECL_SYS (remove, int, (const char *filename), );`
    pub fn remove (filename: *const c_char) -> c_int;
    /// C 1538: `_GL_FUNCDECL_SYS (rename, int, (const char *old, const char *new), );`
    pub fn rename (old: *const c_char, new: *const c_char) -> c_int;
    /// C 1562: `_GL_FUNCDECL_SYS (renameat, int, (int oldfd, const char *old, int newfd, const char *new), );`
    pub fn renameat (oldfd: c_int, old: *const c_char, newfd: c_int, new: *const c_char) -> c_int;
    /// C 1604: `_GL_FUNCDECL_SYS (scanf, int, (const char *restrict format, ...), …);`
    pub fn scanf (format: *const c_char, ...) -> c_int;
    /// C 1646: `_GL_FUNCDECL_SYS (snprintf, int, (char *restrict str, size_t size, const char *restrict format, ...), …);`
    pub fn snprintf (s: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    /// C 1741: `_GL_FUNCDECL_SYS (tempnam, char *, (const char *dir, const char *pfx), );`
    pub fn tempnam (dir: *const c_char, pfx: *const c_char) -> *mut c_char;
    /// C 1753: `_GL_FUNCDECL_SYS (tmpfile, FILE *, (void), );`
    pub fn tmpfile () -> *mut FILE;
    /// C 1911: `_GL_FUNCDECL_SYS (vdprintf, int, (int fd, const char *restrict format, va_list args), …);`
    pub fn vdprintf (fd: c_int, format: *const c_char, args: core::ffi::VaList<'_>) -> c_int;
    /// C 1999: `_GL_FUNCDECL_SYS (vfprintf, int, (FILE *restrict stream, const char *restrict format, va_list args), …);`
    pub fn vfprintf (stream: *mut FILE, format: *const c_char, args: core::ffi::VaList<'_>)
      -> c_int;
    /// C 2013: `_GL_FUNCDECL_SYS (vfscanf, int, (FILE *restrict stream, const char *restrict format, va_list args), …);`
    pub fn vfscanf (stream: *mut FILE, format: *const c_char, args: core::ffi::VaList<'_>)
      -> c_int;
    /// C 2082: `_GL_FUNCDECL_SYS (vprintf, int, (const char *restrict format, va_list args), …);`
    pub fn vprintf (format: *const c_char, args: core::ffi::VaList<'_>) -> c_int;
    /// C 2096: `_GL_FUNCDECL_SYS (vscanf, int, (const char *restrict format, va_list args), …);`
    pub fn vscanf (format: *const c_char, args: core::ffi::VaList<'_>) -> c_int;
    /// C 2137: `_GL_FUNCDECL_SYS (vsnprintf, int, (char *restrict str, size_t size, const char *restrict format, va_list args), …);`
    pub fn vsnprintf (s: *mut c_char, size: usize, format: *const c_char,
                      args: core::ffi::VaList<'_>) -> c_int;
    /// C 2193: `_GL_FUNCDECL_SYS (vsprintf, int, (char *restrict str, const char *restrict format, va_list args), …);`
    pub fn vsprintf (s: *mut c_char, format: *const c_char, args: core::ffi::VaList<'_>) -> c_int;
  }
}
use stdio_in_decls::*;
