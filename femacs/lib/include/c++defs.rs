// femacs: port of lib/c++defs.h @ fd7afced403 (gnulib).
//
// **P-86 ①: the file's whole payload is C++-only text, and it is transcribed anyway.**  Every
// name here either sits in an `#if` the frozen configuration leaves false (`defined __cplusplus`
// at C 90/98/134; `defined __cplusplus && defined GNULIB_NAMESPACE` at C 21 and the seven alias
// blocks C 165-365) or spells a declaration only the C++ arms use (the nine unconditional
// `#define`s, C 125-128 / 151-152 / 163-164 / 192-193 / 225-226).  `tools/extract-abi.py
// lib/c++defs.h` therefore reports 0 subrs, 0 functions, 0 types, 0 file-scope objects and 23
// *macros*: the names are in C's text, and the `#else` arms the frozen guards take are what a C
// build sees.  The 22-line note this mirror used to be is the shape P-86 calls "not started".
//
// **Both arms of every `#if` are carried** (P-86 ①, P-51), in the order
// `lib/include/fcntl.in.rs` opens with: the arm C's text writes *first* (the C++ one) is
// written first here behind `#[cfg (feature = "<its condition>")]`, and the `#else` arm - the
// one the frozen configuration compiles - follows ungated.  The reason is the extractor's: it
// reads a name's *first* definition, so C's arm is what `--check` compares, while `rustc`
// compiles the ungated one.  Each name is a `macro_rules!` pair rather than a `const` pair
// because a macro definition may shadow another of the same name (`lib/include/cdefs.rs`'s
// `__BEGIN_DECLS`/`__END_DECLS` are the precedent), so both arms can live in one file; two
// `const`s of one name would be an `E0428`.
//
// The two names whose C++ arm is a bare brace pair - `_GL_BEGIN_NAMESPACE` (C 22:
// `namespace GNULIB_NAMESPACE {`) and `_GL_END_NAMESPACE` (C 23: `}`) - have no Rust item that
// can hold the arm: an item and a `macro_rules!` body must be balanced, and `}` alone is not
// a token tree.  They are carried as the same-name marker their (nonexistent) use site needs,
// the empty `()` (`lib/include/_Noreturn.rs`'s shape), with both C arms quoted at the item,
// listed in `## Deviations` and appended to `docs/ports/excluded-macros.txt`.
//
// The features are the reference's own conditions, lower-cased.  `cplusplus` is registered
// (`femacs/xtask/Cargo.toml` 80); the other three are new and are reported to the integrator:
//   * `cplusplus`                        - C 90/98/134's `defined __cplusplus`.
//   * `cplusplus_gnulib_namespace`       - C 21/165/200/238/263/289/318/345's `defined
//     __cplusplus && defined GNULIB_NAMESPACE`.
//   * `gnuc_optimize_without_clang`      - C 325/353's `defined __GNUC__ && !defined __clang__
//     && __OPTIMIZE__`, the two warning arms' inner `#if`.
//   * `gnuc3_or_clang_gnulib_strict_checking` - C 330's `(__GNUC__ >= 3 || defined __clang__)
//     && GNULIB_STRICT_CHECKING`.
//
// Measured, not assumed: `python3 tools/check-cond-eval.py lib/c++defs.h` reads `27 conditions,
// 0 unknown, 1 disagreements` - the single disagreement is C 17's own guard
// (`#ifndef _GL_CXXDEFS_H`, whose name the file defines one line below, so the evaluator's
// frozen table cannot see it), and it changes no item.  The header is not in
// `cond-eval-verified.txt` and the extractor reads it blindly (P-66), asking for both arms of
// every condition - which is what this mirror carries: nothing below is dropped as dead text.
// `python3 tools/extract-abi.py --check emacs/lib/c++defs.h femacs/lib/include/c++defs.rs`
// reads `OK`.  That reading is quiet for a recorded reason: all 23 names still carry a row in
// `docs/ports/excluded-macros.txt` (the boilerplate this lane inherits - the mirror now holds
// every one of them, so those rows are stale and the integrator should drop them), which skips
// the body token comparison.  With the rows lifted from a /tmp copy the run reports exactly one
// class of problem - C's `__VA_ARGS__` in the three variadic macros (`_GL_FUNCDECL_RPL`,
// `_GL_FUNCDECL_RPL_1`, `_GL_FUNCDECL_SYS`), which PORTING.md 2.1 spells as the `$($args:tt)*`
// tail - and with the three `docs/ports/macro-body-notes.tsv` rows appended with this file the
// lifted run is `OK` as well, i.e. every other token of every single-definition body matches.
// See `docs/ports/include-c++defs.md` `## Evidence` for both readings.
//
// These macros are C's spelling of *declarations*, not runtime constructs, and the port's
// declarations are the ones its own units write: the mirrors that answer for the generated
// gnulib headers record the resolution at each use site (`lib/include/fcntl.in.rs` 274-276 for
// creat/open/openat, `time.in.rs` 156, `unistd.in.rs` 24, `sys_stat.in.rs` 43/175,
// `dirent.in.rs` 48/115/134-135, `string.in.rs` 28-29/71, `stdio.in.rs` 27-37,
// `sys_time.in.rs` 27/50, `sys_select.in.rs` 26/53, `inttypes.in.rs` 19/65-67).  Measured: the
// only files in the reference that name this header are `lib/gnulib.mk.in` and the
// `build-aux/snippet` machinery that inlines its text into other generated headers - no
// translation unit includes it - so this mirror's compile evidence is that it stands as its
// own crate root (`xtask build-tu lib/c++defs.h`) with none of the four features set, which is
// the frozen arm of every pair.
//
// C 1-15 is the file's opening ownership/title block, dropped (PORTING.md 0.1-19); the port's
// first line is the revision line above, and C 16 is the blank line under the block.

/* C 17-18: `#ifndef _GL_CXXDEFS_H` / `#define _GL_CXXDEFS_H` - the include guard, whose C body
   is empty.  The port's include graph is what dedupes (PORTING.md 0.1-4), so the name is
   carried as the item `tools/extract-abi.py`'s macro list asks for; an empty `macro_rules!`
   body is the one thing the extractor refuses, which is why this one is a `const` - the
   marker shape `lib/include/gnulib_config.rs` uses for `_GL_INLINE`/`_GL_EXTERN_INLINE`.  */
pub const _GL_CXXDEFS_H: bool = true;

/* C 20-27: `_GL_BEGIN_NAMESPACE` / `_GL_END_NAMESPACE`, and C 20's own note: "Begin/end the
   GNULIB_NAMESPACE namespace."

   C 21's `#if defined __cplusplus && defined GNULIB_NAMESPACE` writes the C++ arm first - C 22
   is `namespace GNULIB_NAMESPACE {`, C 23 is `}` - and C 24-26's `#else` arm, the one the
   frozen configuration compiles, defines both names empty.  So the frozen arm *is* this file's
   value: `()`.

   The C++ arm has no Rust spelling: Rust has no namespace blocks (names stay flat, PORTING.md
   0.1), an item cannot carry an unbalanced `{` or `}`, and a `macro_rules!` body must be a
   balanced token tree - so `}` alone cannot be written at all, and writing `{` would leave the
   definition swallowing whatever follows it.  The arm's text is therefore carried here (and in
   the two `docs/ports/excluded-macros.txt` rows appended for this file) and the name is the
   marker its use site needs - the same shape C's own empty `#else` gives it.  */
pub const _GL_BEGIN_NAMESPACE: () = ();
pub const _GL_END_NAMESPACE: () = ();
/* C 29-86, verbatim: The three most frequent use cases of these macros are:

   * For providing a substitute for a function that is missing on some
     platforms, but is declared and works fine on the platforms on which
     it exists:

       #if @GNULIB_FOO@
       # if !@HAVE_FOO@
       _GL_FUNCDECL_SYS (foo, ...);
       # endif
       _GL_CXXALIAS_SYS (foo, ...);
       _GL_CXXALIASWARN (foo);
       #elif defined GNULIB_POSIXCHECK
       ...
       #endif

   * For providing a replacement for a function that exists on all platforms,
     but is broken/insufficient and needs to be replaced on some platforms:

       #if @GNULIB_FOO@
       # if @REPLACE_FOO@
       #  if !(defined __cplusplus && defined GNULIB_NAMESPACE)
       #   undef foo
       #   define foo rpl_foo
       #  endif
       _GL_FUNCDECL_RPL (foo, ...);
       _GL_CXXALIAS_RPL (foo, ...);
       # else
       _GL_CXXALIAS_SYS (foo, ...);
       # endif
       _GL_CXXALIASWARN (foo);
       #elif defined GNULIB_POSIXCHECK
       ...
       #endif

   * For providing a replacement for a function that exists on some platforms
     but is broken/insufficient and needs to be replaced on some of them and
     is additionally either missing or undeclared on some other platforms:

       #if @GNULIB_FOO@
       # if @REPLACE_FOO@
       #  if !(defined __cplusplus && defined GNULIB_NAMESPACE)
       #   undef foo
       #   define foo rpl_foo
       #  endif
       _GL_FUNCDECL_RPL (foo, ...);
       _GL_CXXALIAS_RPL (foo, ...);
       # else
       #  if !@HAVE_FOO@   or   if !@HAVE_DECL_FOO@
       _GL_FUNCDECL_SYS (foo, ...);
       #  endif
       _GL_CXXALIAS_SYS (foo, ...);
       # endif
       _GL_CXXALIASWARN (foo);
       #elif defined GNULIB_POSIXCHECK
       ...
       #endif

*/

/* C 88-94: `_GL_EXTERN_C`, under C 90's `#if defined __cplusplus`.  C 91's C++ arm is
   `extern "C"`, C 93's `#else` arm (the frozen one) is `extern`.  A declaration prefix rather
   than a value, so both arms are carried as C's tokens - the `lib/include/cdefs.rs` shape for
   the same construct (`__BEGIN_DECLS`) - and the port's own spelling of C linkage is the
   `unsafe extern "C" { … }` block of a declaration (PORTING.md 0.1-2), which is why no use
   site calls for this name.  */
/* C 88-89, verbatim: _GL_EXTERN_C declaration;
   performs the declaration with C linkage.  
*/
#[cfg (feature = "cplusplus")]
macro_rules! _GL_EXTERN_C { () => { extern "C" }; }
/// C 93: `# define _GL_EXTERN_C extern` - the frozen arm.
macro_rules! _GL_EXTERN_C { () => { extern }; }

/* C 96-105: `_GL_EXTERN_C_FUNC`, under C 98's `#if defined __cplusplus`.  C 99's C++ arm is
   `extern "C"`; C 104's `#else` arm (frozen) is empty, for the reason C 101-103 states (an
   attribute in bracket syntax may not sit between `extern` and the return type), so the name
   expands to nothing.  */
/* C 96-97, verbatim: _GL_EXTERN_C_FUNC declaration;
   performs the declaration of a function with C linkage.  
*/
#[cfg (feature = "cplusplus")]
macro_rules! _GL_EXTERN_C_FUNC { () => { extern "C" }; }

/* C 101-103, verbatim: In C mode, omit the 'extern' keyword, because attributes in bracket syntax
   are not allowed between 'extern' and the return type (see gnulib-common.m4).
 
*/
/// C 104: `# define _GL_EXTERN_C_FUNC` - the frozen arm, no tokens.
macro_rules! _GL_EXTERN_C_FUNC { () => {}; }

/* C 107-124, verbatim: _GL_FUNCDECL_RPL (func, rettype, parameters, [attributes]);
   declares a replacement function, named rpl_func, with the given prototype,
   consisting of return type, parameters, and attributes.
   Although attributes are optional, the comma before them is required
   for portability to C17 and earlier.  The attribute _GL_ATTRIBUTE_NOTHROW,
   if needed, must be placed after the _GL_FUNCDECL_RPL invocation,
   at the end of the declaration.
   Examples:
     _GL_FUNCDECL_RPL (free, void, (void *ptr), ) _GL_ATTRIBUTE_NOTHROW;
     _GL_FUNCDECL_RPL (open, int, (const char *filename, int flags, ...),
                                  _GL_ARG_NONNULL ((1)));

   Note: Attributes, such as _GL_ATTRIBUTE_DEPRECATED, are supported in front
   of a _GL_FUNCDECL_RPL invocation only in C mode, not in C++ mode.  (That's
   because
     [[...]] extern "C" <declaration>;
   is invalid syntax in C++.)
 
*/
/* C 125-128.  C's `rpl_##func` pastes the parameter into the name and its `__VA_ARGS__` is the
   trailing attribute list; the port spells the paste as two tokens (`rpl_ $func`) and the
   variadic tail as `$($args:tt)*` (PORTING.md 2.1), which is why `__VA_ARGS__` itself is
   recorded in `docs/ports/macro-body-notes.tsv`.  */
/// C 125-126: `#define _GL_FUNCDECL_RPL(func,rettype,parameters,...) _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters, __VA_ARGS__)`.
macro_rules! _GL_FUNCDECL_RPL {
  ($func:tt, $rettype:tt, $parameters:tt, $($args:tt)*) => {
    _GL_FUNCDECL_RPL_1! (rpl_ $func, $rettype, $parameters, $($args)*)
  };
}
/// C 127-128: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters,...) _GL_EXTERN_C_FUNC __VA_ARGS__ rettype rpl_func parameters`.
macro_rules! _GL_FUNCDECL_RPL_1 {
  ($rpl_func:tt, $rettype:tt, $parameters:tt, $($args:tt)*) => {
    _GL_EXTERN_C_FUNC $($args)* $rettype $rpl_func $parameters
  };
}

/* C 130-138: `_GL_FUNCDECL_SYS_NAME`, under C 134's `#ifdef __cplusplus`.  C 135's C++ arm is
   the bare `func`; C 137's `#else` arm (frozen) is the parenthesised `(func)` that C23's
   qualifier-generic macros need.  */
/* C 130-133, verbatim: _GL_FUNCDECL_SYS_NAME (func) expands to plain func if C++, and to
   parenthesized func otherwise.  Parenthesization is needed in C23 if
   the function is like strchr and so is a qualifier-generic macro
   that expands to something more complicated.  
*/
#[cfg (feature = "cplusplus")]
macro_rules! _GL_FUNCDECL_SYS_NAME { ($func:tt) => { $func }; }
/// C 137: `# define _GL_FUNCDECL_SYS_NAME(func) (func)` - the frozen arm.
macro_rules! _GL_FUNCDECL_SYS_NAME { ($func:tt) => { ($func) }; }

/* C 140-150, verbatim: _GL_FUNCDECL_SYS (func, rettype, parameters, [attributes]);
   declares the system function, named func, with the given prototype,
   consisting of return type, parameters, and attributes.
   Although attributes are optional, the comma before them is required
   for portability to C17 and earlier.  The attribute _GL_ATTRIBUTE_NOTHROW,
   if needed, must be placed after the _GL_FUNCDECL_RPL invocation,
   at the end of the declaration.
   Examples:
     _GL_FUNCDECL_SYS (getumask, mode_t, (void), ) _GL_ATTRIBUTE_NOTHROW;
     _GL_FUNCDECL_SYS (posix_openpt, int, (int flags), _GL_ATTRIBUTE_NODISCARD);
 
*/
/// C 151-152: `#define _GL_FUNCDECL_SYS(func,rettype,parameters,...) _GL_EXTERN_C_FUNC __VA_ARGS__ rettype _GL_FUNCDECL_SYS_NAME (func) parameters`.
macro_rules! _GL_FUNCDECL_SYS {
  ($func:tt, $rettype:tt, $parameters:tt, $($args:tt)*) => {
    _GL_EXTERN_C_FUNC $($args)* $rettype _GL_FUNCDECL_SYS_NAME ($func) $parameters
  };
}

/* C 154-162, verbatim: _GL_CXXALIAS_RPL (func, rettype, parameters);
   declares a C++ alias called GNULIB_NAMESPACE::func
   that redirects to rpl_func, if GNULIB_NAMESPACE is defined.
   Example:
     _GL_CXXALIAS_RPL (open, int, (const char *filename, int flags, ...));

   Wrapping rpl_func in an object with an inline conversion operator
   avoids a reference to rpl_func unless GNULIB_NAMESPACE::func is
   actually used in the program.  
*/
/// C 163-164: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`.
macro_rules! _GL_CXXALIAS_RPL {
  ($func:tt, $rettype:tt, $parameters:tt) => {
    _GL_CXXALIAS_RPL_1! ($func, rpl_ $func, $rettype, $parameters)
  };
}

/* C 165-183: `_GL_CXXALIAS_RPL_1`.  C 166-179's C++ arm is the namespace-local object whose
   inline conversion operator returns `::rpl_func` (so nothing refers to `rpl_func` unless the
   alias is used); C 181-182's `#else` arm, the frozen one, is the unused
   `_gl_cxxalias_dummy` declaration.  The C++ arm's braces balance (the namespace block's and
   the wrapper struct's cancel), so it is carried token for token, with C's pasted
   `_gl_ ## func ## _wrapper` spelled `_gl_ $func _wrapper`.  */
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIAS_RPL_1 {
  ($func:tt, $rpl_func:tt, $rettype:tt, $parameters:tt) => {
    namespace GNULIB_NAMESPACE { static const struct _gl_ $func _wrapper { typedef $rettype (*type) $parameters; inline operator type () const { return :: $rpl_func; } } $func = {}; } _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}
/// C 181-182: `# define _GL_CXXALIAS_RPL_1(func,rpl_func,rettype,parameters) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIAS_RPL_1 {
  ($func:tt, $rpl_func:tt, $rettype:tt, $parameters:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy };
}

/* C 185-191, verbatim: _GL_CXXALIAS_MDA (func, rettype, parameters);
   is to be used when func is a Microsoft deprecated alias, on native Windows.
   It declares a C++ alias called GNULIB_NAMESPACE::func
   that redirects to _func, if GNULIB_NAMESPACE is defined.
   Example:
     _GL_CXXALIAS_MDA (open, int, (const char *filename, int flags, ...));
 
*/
/// C 192-193: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)` - the Microsoft deprecated alias, pasted as `_ $func`.
macro_rules! _GL_CXXALIAS_MDA {
  ($func:tt, $rettype:tt, $parameters:tt) => {
    _GL_CXXALIAS_RPL_1! ($func, _ $func, $rettype, $parameters)
  };
}

/* C 195-199, verbatim: _GL_CXXALIAS_RPL_CAST_1 (func, rpl_func, rettype, parameters);
   is like  _GL_CXXALIAS_RPL_1 (func, rpl_func, rettype, parameters);
   except that the C function rpl_func may have a slightly different
   declaration.  A cast is used to silence the "invalid conversion" error
   that would otherwise occur.  
*/
/* C 200-218: `_GL_CXXALIAS_RPL_CAST_1` - the C++ arm (C 201-214) casts the rpl function to the
   wrapper's type, the frozen `#else` arm (C 216-217) is the dummy declaration again.  */
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIAS_RPL_CAST_1 {
  ($func:tt, $rpl_func:tt, $rettype:tt, $parameters:tt) => {
    namespace GNULIB_NAMESPACE { static const struct _gl_ $func _wrapper { typedef $rettype (*type) $parameters; inline operator type () const { return reinterpret_cast<type>(:: $rpl_func); } } $func = {}; } _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}
/// C 216-217: `# define _GL_CXXALIAS_RPL_CAST_1(func,rpl_func,rettype,parameters) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIAS_RPL_CAST_1 {
  ($func:tt, $rpl_func:tt, $rettype:tt, $parameters:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy };
}

/* C 220-224, verbatim: _GL_CXXALIAS_MDA_CAST (func, rettype, parameters);
   is like  _GL_CXXALIAS_MDA (func, rettype, parameters);
   except that the C function func may have a slightly different declaration.
   A cast is used to silence the "invalid conversion" error that would
   otherwise occur.  
*/
/// C 225-226: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`.
macro_rules! _GL_CXXALIAS_MDA_CAST {
  ($func:tt, $rettype:tt, $parameters:tt) => {
    _GL_CXXALIAS_RPL_CAST_1! ($func, _ $func, $rettype, $parameters)
  };
}

/* C 228-237, verbatim: _GL_CXXALIAS_SYS (func, rettype, parameters);
   declares a C++ alias called GNULIB_NAMESPACE::func
   that redirects to the system provided function func, if GNULIB_NAMESPACE
   is defined.
   Example:
     _GL_CXXALIAS_SYS (open, int, (const char *filename, int flags, ...));

   Wrapping func in an object with an inline conversion operator
   avoids a reference to func unless GNULIB_NAMESPACE::func is
   actually used in the program.  
*/
/* C 238-256: `_GL_CXXALIAS_SYS`.  The C++ arm (C 239-252) returns `::func`; the frozen `#else`
   arm (C 254-255) is the dummy declaration.  */
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIAS_SYS {
  ($func:tt, $rettype:tt, $parameters:tt) => {
    namespace GNULIB_NAMESPACE { static const struct _gl_ $func _wrapper { typedef $rettype (*type) $parameters; inline operator type () const { return :: $func; } } $func = {}; } _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}
/// C 254-255: `# define _GL_CXXALIAS_SYS(func,rettype,parameters) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIAS_SYS {
  ($func:tt, $rettype:tt, $parameters:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy };
}

/* C 258-262, verbatim: _GL_CXXALIAS_SYS_CAST (func, rettype, parameters);
   is like  _GL_CXXALIAS_SYS (func, rettype, parameters);
   except that the C function func may have a slightly different declaration.
   A cast is used to silence the "invalid conversion" error that would
   otherwise occur.  
*/
/* C 263-281: `_GL_CXXALIAS_SYS_CAST` - the C++ arm (C 264-277) casts the system function, the
   frozen `#else` arm (C 279-280) is the dummy declaration.  */
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIAS_SYS_CAST {
  ($func:tt, $rettype:tt, $parameters:tt) => {
    namespace GNULIB_NAMESPACE { static const struct _gl_ $func _wrapper { typedef $rettype (*type) $parameters; inline operator type () const { return reinterpret_cast<type>(:: $func); } } $func = {}; } _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}
/// C 279-280: `# define _GL_CXXALIAS_SYS_CAST(func,rettype,parameters) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIAS_SYS_CAST {
  ($func:tt, $rettype:tt, $parameters:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy };
}

/* C 283-288, verbatim: _GL_CXXALIAS_SYS_CAST2 (func, rettype, parameters, rettype2, parameters2);
   is like  _GL_CXXALIAS_SYS (func, rettype, parameters);
   except that the C function is picked among a set of overloaded functions,
   namely the one with rettype2 and parameters2.  Two consecutive casts
   are used to silence the "cannot find a match" and "invalid conversion"
   errors that would otherwise occur.  
*/
  /* C 290-294, verbatim: The outer cast must be a reinterpret_cast.
     The inner cast: When the function is defined as a set of overloaded
     functions, it works as a static_cast<>, choosing the designated variant.
     When the function is defined as a single variant, it works as a
     reinterpret_cast<>. The parenthesized cast syntax works both ways.  
*/
/* C 289-312: `_GL_CXXALIAS_SYS_CAST2` - the C++ arm (C 295-308) picks the overload with
   `rettype2`/`parameters2` through two casts (C 290-294's note); the frozen `#else` arm
   (C 310-311) is the dummy declaration.  */
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIAS_SYS_CAST2 {
  ($func:tt, $rettype:tt, $parameters:tt, $rettype2:tt, $parameters2:tt) => {
    namespace GNULIB_NAMESPACE { static const struct _gl_ $func _wrapper { typedef $rettype (*type) $parameters; inline operator type () const { return reinterpret_cast<type>(($rettype2 (*) $parameters2)(:: $func)); } } $func = {}; } _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}
/// C 310-311: `# define _GL_CXXALIAS_SYS_CAST2(func,rettype,parameters,rettype2,parameters2) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIAS_SYS_CAST2 {
  ($func:tt, $rettype:tt, $parameters:tt, $rettype2:tt, $parameters2:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy };
}

/* C 314-317, verbatim: _GL_CXXALIASWARN (func);
   causes a warning to be emitted when ::func is used but not when
   GNULIB_NAMESPACE::func is used.  func must be defined without overloaded
   variants.  
*/
/* C 318-340: `_GL_CXXALIASWARN` and its two helpers.  The C++ arm (C 319-320) forwards to
   `_GL_CXXALIASWARN_1 (func, GNULIB_NAMESPACE)`; the frozen `#else` arm (C 338-339) is the
   dummy declaration.  Inside the C++ arm, `_GL_CXXALIASWARN_2` has three definitions: the
   warning one (C 326-329, taken when C 325's condition is false), the `GNULIB_STRICT_CHECKING`
   one (C 331-332) and the dummy (C 334-335) - so all three are gated on the *nested*
   conditions, and `_GL_CXXALIASWARN_1` (C 321-322), which C defines only in the C++ arm, is
   gated on the C++ condition alone (the frozen configuration has no such macro, exactly as
   C's `#else` arm does not define it).  */
/* C 323-324, verbatim: To work around GCC bug <https://gcc.gnu.org/PR43881>,
   we enable the warning only when not optimizing.  
*/
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIASWARN {
  ($func:tt) => { _GL_CXXALIASWARN_1! ($func, GNULIB_NAMESPACE) };
}
/// C 338-339: `# define _GL_CXXALIASWARN(func) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIASWARN { ($func:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy }; }

/// C 321-322: `# define _GL_CXXALIASWARN_1(func,namespace) _GL_CXXALIASWARN_2 (func, namespace)` - C's C++ arm only.
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIASWARN_1 {
  ($func:tt, $namespace:tt) => { _GL_CXXALIASWARN_2! ($func, $namespace) };
}

/* C 325-329's warning arm - C's `# if !(defined __GNUC__ && !defined __clang__ &&
   __OPTIMIZE__)`, so it is the arm a C++ build takes when `__GNUC__` is absent, or `__clang__`
   is present, or the build is not optimising.  */
#[cfg (all (feature = "cplusplus_gnulib_namespace",
           not (feature = "gnuc_optimize_without_clang")))]
macro_rules! _GL_CXXALIASWARN_2 {
  ($func:tt, $namespace:tt) => {
    _GL_WARN_ON_USE ($func, "The symbol ::" # $func " refers to the system function. " "Use " # $namespace "::" # $func " instead.")
  };
}
/// C 330-332: `# elif (__GNUC__ >= 3 || defined __clang__) && GNULIB_STRICT_CHECKING` - the arm that verifies the function exists.
#[cfg (all (feature = "cplusplus_gnulib_namespace", feature = "gnuc_optimize_without_clang",
           feature = "gnuc3_or_clang_gnulib_strict_checking"))]
macro_rules! _GL_CXXALIASWARN_2 {
  ($func:tt, $namespace:tt) => { extern __typeof__ ($func) $func };
}
/// C 333-335: `# else` - the arm for a compiler with none of the three spellings.
#[cfg (all (feature = "cplusplus_gnulib_namespace", feature = "gnuc_optimize_without_clang",
           not (feature = "gnuc3_or_clang_gnulib_strict_checking")))]
macro_rules! _GL_CXXALIASWARN_2 {
  ($func:tt, $namespace:tt) => { _GL_EXTERN_C int _gl_cxxalias_dummy };
}

/* C 342-344, verbatim: _GL_CXXALIASWARN1 (func, rettype, parameters_and_attributes);
   causes a warning to be emitted when the given overloaded variant of ::func
   is used but not when GNULIB_NAMESPACE::func is used.  
*/
/* C 345-365: `_GL_CXXALIASWARN1` and its two helpers - the overloaded variant.  C 346-348's
   C++ arm forwards to `_GL_CXXALIASWARN1_1`; C 363-364's `#else` arm (frozen) is the dummy.
   `_GL_CXXALIASWARN1_1` (C 349-350) is C's C++ arm only, and `_GL_CXXALIASWARN1_2`'s two
   definitions (C 354-357 and C 359-360) split on the same optimising `#if` as the warning
   above.  */
/* C 351-352, verbatim: To work around GCC bug <https://gcc.gnu.org/PR43881>,
   we enable the warning only when not optimizing.  
*/
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIASWARN1 {
  ($func:tt, $rettype:tt, $parameters_and_attributes:tt) => {
    _GL_CXXALIASWARN1_1! ($func, $rettype, $parameters_and_attributes, GNULIB_NAMESPACE)
  };
}
/// C 363-364: `# define _GL_CXXALIASWARN1(func,rettype,parameters_and_attributes) _GL_EXTERN_C int _gl_cxxalias_dummy` - the frozen arm.
macro_rules! _GL_CXXALIASWARN1 {
  ($func:tt, $rettype:tt, $parameters_and_attributes:tt) => {
    _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}

/// C 349-350: `# define _GL_CXXALIASWARN1_1(func,rettype,parameters_and_attributes,namespace) _GL_CXXALIASWARN1_2 (func, rettype, parameters_and_attributes, namespace)` - C's C++ arm only.
#[cfg (feature = "cplusplus_gnulib_namespace")]
macro_rules! _GL_CXXALIASWARN1_1 {
  ($func:tt, $rettype:tt, $parameters_and_attributes:tt, $namespace:tt) => {
    _GL_CXXALIASWARN1_2! ($func, $rettype, $parameters_and_attributes, $namespace)
  };
}

/// C 353-357: `# if !(defined __GNUC__ && !defined __clang__ && __OPTIMIZE__)` - the warning arm for overloaded variants.
#[cfg (all (feature = "cplusplus_gnulib_namespace",
           not (feature = "gnuc_optimize_without_clang")))]
macro_rules! _GL_CXXALIASWARN1_2 {
  ($func:tt, $rettype:tt, $parameters_and_attributes:tt, $namespace:tt) => {
    _GL_WARN_ON_USE_CXX ($func, $rettype, $rettype, $parameters_and_attributes, "The symbol ::" # $func " refers to the system function. " "Use " # $namespace "::" # $func " instead.")
  };
}
/// C 358-360: `# else` - the arm whose expansion is nothing but the dummy declaration.
#[cfg (all (feature = "cplusplus_gnulib_namespace", feature = "gnuc_optimize_without_clang"))]
macro_rules! _GL_CXXALIASWARN1_2 {
  ($func:tt, $rettype:tt, $parameters_and_attributes:tt, $namespace:tt) => {
    _GL_EXTERN_C int _gl_cxxalias_dummy
  };
}

/* C 367: the trailing `#endif` whose comment names `_GL_CXXDEFS_H` closes C's include guard.
   The port has no preprocessor, so the guard is the `pub const` at the top of this file and
   nothing else (PORTING.md 0.1-4).  */
