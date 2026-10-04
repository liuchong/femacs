// femacs: mirror of lib/alloca.in.h @ fd7afced403 (PORTING.md 0.1).
//
// Disposition mirror: the header's whole content is the `alloca` macro family, and the port has
// no `alloca` at all (PORTING.md P-05 and its section 3.2).  The judge is the *generated* header
// `build/emacs-ref/lib/alloca.h` (2,522 bytes; the same method `errno.in`/`dirent.in` use, status
// 3.159): the frozen configuration's preprocessor takes the `__GNUC__`/clang arm
// (`# define alloca __builtin_alloca`, generated header line 47), so C's other arms - `_AIX`,
// `_MSC_VER`, `__DECC`, `__TANDEM`, `__MVS__` and the trailing `extern void *alloca (size_t)`
// fallback - are dead text here, and the `include_next <alloca.h>` workaround (C 36-40) never
// fires because `_WIN32` is undefined.
//
// Why nothing is carried: `tools/extract-abi.py emacs/lib/alloca.in.h` lists **zero items**
// (there is no item the mirror owes), and a Rust `alloca` macro cannot exist - the port's
// stack-pool idiom is `sa_avail`/`sa_alloc` (PORTING.md 3.2), with the use-site spelling recorded
// in `docs/ports/excluded-macros.txt` (`AVAIL_ALLOCA alloca ...`).  A mirror that invented an
// `alloca!` would be a second mechanism, and the port is forbidden one.  This file exists so the
// header has its named counterpart and the decision is written where a reader looks first (the
// `lib/assert.in.h` precedent, status 3.157).

// C 23-24: the include guard has no Rust counterpart (PORTING.md 0.1-4); it is carried so a
// future judge that *can* decide this template finds the name.
pub const _GL_ALLOCA_H: bool = true;

/// C 27-45: `#ifndef alloca` + the platform arm chain.  Only the clang arm is live for the
/// frozen configuration, and it defines a *macro*, which Rust has no text counterpart for.
/// C: # if defined __GNUC__ || (__clang_major__ >= 4)
/// C: #  define alloca __builtin_alloca
/// C: # elif defined _AIX
/// C: #  define alloca __alloca
/// C: # elif defined _MSC_VER
/// C: #  include <malloc.h>
/// C: #  define alloca _alloca
/// C: # elif defined __DECC && defined __VMS
/// C: #  define alloca __ALLOCA
/// C: # elif defined __TANDEM && defined _TNS_E_TARGET
/// C: #  ifdef  __cplusplus
/// C: extern "C"
/// C: #  endif
/// C: void *_alloca (unsigned short);
/// C: #  pragma intrinsic (_alloca)
/// C: #  define alloca _alloca
/// C: # elif defined __MVS__
/// C: #  include <stdlib.h>
/// C: # else
/// C: #  include <stddef.h>
/// C: #  ifdef  __cplusplus
/// C: extern "C"
/// C: #  endif
/// C: void *alloca (size_t);
