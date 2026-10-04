# emacs/lib/stdint.in.h (740 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 136
- [x] GL_INT64_T  (line 192, body da39a3ee5e6b)
- [x] GL_UINT64_T  (line 218, body da39a3ee5e6b)
- [x] GNULIB_defined_stdint_types  (line 370, body 356a192b7913)
- [x] INT16_C  (line 630, body 11f6ad8ec52a)
- [x] INT16_MAX  (line 391, body 757c3591bcea)
- [x] INT16_MIN  (line 390, body 1ccada68c25b)
- [x] INT16_WIDTH  (line 690, body 7e84d0b9c4e4)
- [x] INT32_C  (line 635, body 11f6ad8ec52a)
- [x] INT32_MAX  (line 398, body 75878664309b)
- [x] INT32_MIN  (line 397, body 39775a91b4f3)
- [x] INT32_WIDTH  (line 696, body 29525061eaa5)
- [x] INT64_C  (line 645, body b765a38be534)
- [x] INT64_MAX  (line 405, body 5d3b6b2a086b)
- [x] INT64_MIN  (line 404, body 48a448038317)
- [x] INT64_WIDTH  (line 702, body a9ab14cbe570)
- [x] INT8_C  (line 625, body 11f6ad8ec52a)
- [x] INT8_MAX  (line 384, body 008451a05e1e)
- [x] INT8_MIN  (line 383, body 877c4ea21608)
- [x] INT8_WIDTH  (line 684, body 218be2858f5c)
- [x] INTMAX_C  (line 663, body e119cb724a8e)
- [x] INTMAX_MAX  (line 520, body 94b7ff4d363a)
- [x] INTMAX_MIN  (line 519, body 58bb802332f9)
- [x] INTMAX_WIDTH  (line 725, body eabff0889cb5)
- [x] INTPTR_MAX  (line 507, body f5c01bcbc752)
- [x] INTPTR_MIN  (line 506, body afdafb6274ed)
- [x] INTPTR_WIDTH  (line 723, body 59e4585e405f)
- [x] INT_FAST16_MAX  (line 468, body 5c970dd6f6de)
- [x] INT_FAST16_MIN  (line 467, body 00a8fcedeeec)
- [x] INT_FAST16_WIDTH  (line 717, body 834a04db3abf)
- [x] INT_FAST32_MAX  (line 480, body f5c01bcbc752)
- [x] INT_FAST32_MIN  (line 479, body afdafb6274ed)
- [x] INT_FAST32_WIDTH  (line 719, body 64cb17de5872)
- [x] INT_FAST64_MAX  (line 488, body cc7133c2a015)
- [x] INT_FAST64_MIN  (line 487, body 2757a48699b1)
- [x] INT_FAST64_WIDTH  (line 721, body c247f525e783)
- [x] INT_FAST8_MAX  (line 461, body 834043819b4f)
- [x] INT_FAST8_MIN  (line 460, body 2a8a1c243636)
- [x] INT_FAST8_WIDTH  (line 715, body f808065d3e2b)
- [x] INT_LEAST16_MAX  (line 429, body f095f23b3a46)
- [x] INT_LEAST16_MIN  (line 428, body bcb1ef5d904a)
- [x] INT_LEAST16_WIDTH  (line 709, body e29166634d6e)
- [x] INT_LEAST32_MAX  (line 436, body 94b7ff4d363a)
- [x] INT_LEAST32_MIN  (line 435, body 58bb802332f9)
- [x] INT_LEAST32_WIDTH  (line 711, body ebe59749c910)
- [x] INT_LEAST64_MAX  (line 443, body cc7133c2a015)
- [x] INT_LEAST64_MIN  (line 442, body 2757a48699b1)
- [x] INT_LEAST64_WIDTH  (line 713, body 19a40474be9d)
- [x] INT_LEAST8_MAX  (line 422, body 79274b596d41)
- [x] INT_LEAST8_MIN  (line 421, body 7a3d6daab86e)
- [x] INT_LEAST8_WIDTH  (line 707, body 2fbf136564ae)
- [x] PTRDIFF_MAX  (line 548, body 04186047aba4)
- [x] PTRDIFF_MIN  (line 546, body 0dc5c198ce58)
- [x] PTRDIFF_WIDTH  (line 727, body 647f9dc97e47)
- [x] SIG_ATOMIC_MAX  (line 562, body eac1534e9c3b)
- [x] SIG_ATOMIC_MIN  (line 559, body 7505615b07c1)
- [x] SIG_ATOMIC_WIDTH  (line 734, body b38fc43de8c4)
- [x] SIZE_MAX  (line 576, body 47d30c340fac)
- [x] SIZE_WIDTH  (line 728, body 2bdbe2195b6b)
- [x] UINT16_C  (line 631, body 11f6ad8ec52a)
- [x] UINT16_MAX  (line 392, body 0cca08dd76e2)
- [x] UINT16_WIDTH  (line 693, body 9ed7d091121b)
- [x] UINT32_C  (line 636, body 81de21fef84f)
- [x] UINT32_MAX  (line 399, body f1711e6df3e6)
- [x] UINT32_WIDTH  (line 699, body 2eaf4ba2168c)
- [x] UINT64_C  (line 652, body d637c6031d76)
- [x] UINT64_MAX  (line 409, body d433b87adbb3)
- [x] UINT64_WIDTH  (line 705, body 4d36eb503827)
- [x] UINT8_C  (line 626, body 11f6ad8ec52a)
- [x] UINT8_MAX  (line 385, body 3028f51407d8)
- [x] UINT8_WIDTH  (line 687, body 2e1e5f19a39f)
- [x] UINTMAX_C  (line 673, body bd46c8d74b65)
- [x] UINTMAX_MAX  (line 528, body 21b1af79b013)
- [x] UINTMAX_WIDTH  (line 726, body 35775edae271)
- [x] UINTPTR_MAX  (line 508, body 7531eda0d2d7)
- [x] UINTPTR_WIDTH  (line 724, body b05afb5b22dc)
- [x] UINT_FAST16_MAX  (line 469, body 5a5e4496bd6e)
- [x] UINT_FAST16_WIDTH  (line 718, body 4cc63b4c3aca)
- [x] UINT_FAST32_MAX  (line 481, body 7531eda0d2d7)
- [x] UINT_FAST32_WIDTH  (line 720, body 28fc1ba244fa)
- [x] UINT_FAST64_MAX  (line 493, body dff23bdb413c)
- [x] UINT_FAST64_WIDTH  (line 722, body f71b9f9a76c7)
- [x] UINT_FAST8_MAX  (line 462, body b9fa37885cbf)
- [x] UINT_FAST8_WIDTH  (line 716, body 03d78e1302dc)
- [x] UINT_LEAST16_MAX  (line 430, body 63201f5596bc)
- [x] UINT_LEAST16_WIDTH  (line 710, body bc9334004934)
- [x] UINT_LEAST32_MAX  (line 437, body 21b1af79b013)
- [x] UINT_LEAST32_WIDTH  (line 712, body 1bef90a56bd6)
- [x] UINT_LEAST64_MAX  (line 448, body dff23bdb413c)
- [x] UINT_LEAST64_WIDTH  (line 714, body db46def2cb68)
- [x] UINT_LEAST8_MAX  (line 423, body c53d6d4dd0a2)
- [x] UINT_LEAST8_WIDTH  (line 708, body 0ff028d49ba0)
- [x] WCHAR_MAX  (line 595, body 71104a970764)
- [x] WCHAR_MIN  (line 592, body 7e4717ccc0db)
- [x] WCHAR_WIDTH  (line 729, body c7b170b50241)
- [x] WINT_MAX  (line 611, body 8713aa3dc556)
- [x] WINT_MIN  (line 608, body 52eb87f1065e)
- [x] WINT_WIDTH  (line 731, body 9e59460a2185)
- [x] _  (line 73, body 357e0afe4d15)
- [x] _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H  (line 34, body da39a3ee5e6b)
- [x] _GL_JUST_INCLUDE_SYSTEM_WCHAR_H  (line 582, body da39a3ee5e6b)
- [x] _STDINT_MAX  (line 132, body e9abc444d40c)
- [x] _STDINT_SIGNED_MIN  (line 129, body a56d97899d33)
- [x] _STDINT_UNSIGNED_MIN  (line 127, body f7c4a77e27bb)
- [x] _UINT32_T  (line 224, body da39a3ee5e6b)
- [x] _UINT64_T  (line 225, body da39a3ee5e6b)
- [x] _UINT8_T  (line 223, body da39a3ee5e6b)
- [x] __STDC_CONSTANT_MACROS  (line 57, body 356a192b7913)
- [x] __STDC_LIMIT_MACROS  (line 60, body 356a192b7913)
- [x] int16_t  (line 153, body dad711f6402a)
- [x] int32_t  (line 160, body c95c143ffdfd)
- [x] int64_t  (line 191, body e52459cf0262)
- [x] int8_t  (line 146, body 6abc798e7652)
- [x] int_fast16_t  (line 291, body ab81467eb101)
- [x] int_fast32_t  (line 293, body 508ef45aea24)
- [x] int_fast64_t  (line 296, body f63111689337)
- [x] int_fast8_t  (line 289, body 742e282746fa)
- [x] int_least16_t  (line 244, body 19baa746e806)
- [x] int_least32_t  (line 246, body 8fcdcefeb8f1)
- [x] int_least64_t  (line 249, body f63111689337)
- [x] int_least8_t  (line 242, body fb861bce0ed1)
- [x] intmax_t  (line 346, body 7478b72bc176)
- [x] intptr_t  (line 323, body d23cd04d026b)
- [x] uint16_t  (line 154, body b4d3446d4d59)
- [x] uint32_t  (line 161, body e14cbe526625)
- [x] uint64_t  (line 217, body 507128832b01)
- [x] uint8_t  (line 147, body 34ddf2d13572)
- [x] uint_fast16_t  (line 292, body e97b891c0fad)
- [x] uint_fast32_t  (line 294, body 69c40e10105f)
- [x] uint_fast64_t  (line 299, body 1423d427c67f)
- [x] uint_fast8_t  (line 290, body fec314404f52)
- [x] uint_least16_t  (line 245, body 4e85f56cdbea)
- [x] uint_least32_t  (line 247, body b48de5001320)
- [x] uint_least64_t  (line 252, body 1423d427c67f)
- [x] uint_least8_t  (line 243, body 124420692e28)
- [x] uintmax_t  (line 360, body a8a2dee879ec)
- [x] uintptr_t  (line 324, body 66b7e83b068d)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/stdint.in.h` @ `fd7afced403`; port `femacs/lib/include/stdint.in.rs`.

**The template.**  The reference is a gnulib template: `@GUARD_PREFIX@`,
`@PRAGMA_SYSTEM_HEADER@`, `@PRAGMA_COLUMNS@`, `@INCLUDE_NEXT@`, `@NEXT_STDINT_H@`,
`@HAVE_STDINT_H@`, `@HAVE_C99_STDINT_H@`, `@GNULIBHEADERS_OVERRIDE_WINT_T@` and the
`@BITSIZEOF_*@` / `@*_T_SUFFIX@` / `@HAVE_SIGNED_*@` family are configured in.  The frozen
substitution is the generated `build/emacs-ref/lib/stdint.h` (guard `_GL_STDINT_H`,
`#include_next <stdint.h>`, `#if 1` for `@HAVE_STDINT_H@`, `#if ! 1` for
`@HAVE_C99_STDINT_H@`, `#if 0` for the `WINT_MIN`/`WINT_MAX` override).  The port has neither
`#include_next` nor placeholders, so the mirror is both halves: the names the template defines
itself and the values the system `<stdint.h>` the template includes next supplies.

**The whole type-and-limit section is dead text.**  `#if ! @HAVE_C99_STDINT_H@` (C 88) is
false for this configuration, so C 88-677 never compiles and the system header is where every
typedef, limit and `_C` macro comes from.  `extract-abi.py` reads a gnulib template blind
(P-08/P-66: the frozen macro table cannot decide a `@PLACEHOLDER@` condition and the file is
not in `docs/baseline/config/cond-eval-verified.txt`), so the mirror still carries every name
that dead text can define; the values and the types are the frozen ones, measured with the
frozen clang rather than read off the template:

- the exact/least/fast limits: `INT8_MIN` -128, `INT8_MAX` 127, `UINT8_MAX` 255, `INT16_MIN`
  -32768, `INT16_MAX` 32767, `UINT16_MAX` 65535, `INT32_MIN` -2147483648, `INT32_MAX`
  2147483647, `UINT32_MAX` 4294967295, `INT64_MIN`/`INT64_MAX` the `long long` pair,
  `UINT64_MAX` 18446744073709551615, the `INT_LEAST8..64`/`UINT_LEAST8..64` and
  `INT_FAST8..64`/`UINT_FAST8..64` families, `INTPTR_MIN`/`INTPTR_MAX` the 64-bit `long` pair,
  `UINTPTR_MAX` 18446744073709551615, `INTMAX_MIN`/`INTMAX_MAX`/`UINTMAX_MAX` the 64-bit pair,
  `PTRDIFF_MIN`/`PTRDIFF_MAX`/`SIZE_MAX`, `SIG_ATOMIC_MIN` -2147483648, `SIG_ATOMIC_MAX`
  2147483647, `WCHAR_MIN`/`WCHAR_MAX` -2147483648/2147483647, `WINT_MIN`/`WINT_MAX` the same
  pair;
- the types (each measured with `sizeof` and `_Generic`, and with clang's own
  `__INT_FAST16_TYPE__`-style macros): `int8_t` `signed char`, `uint8_t` `unsigned char`,
  `int16_t` `short`, …, `int64_t` `long long`, `int_fast16_t` **`short`** and `int_fast32_t`
  **`int`** (the template's dead arm widens them to `gl_int_fast32_t` and `long` — the frozen
  target does not), `uintptr_t` `unsigned long`;
- the widths: all 8/16/32/64 as `_GL_INTEGER_WIDTH (MIN, MAX)` of the type, and
  `WCHAR_WIDTH`/`WINT_WIDTH`/`SIG_ATOMIC_WIDTH` 32.

**The helpers and the `_C` family.**  `_GL_INTEGER_WIDTH` comes from `limits.in.rs` (the
template includes `<limits.h>` at C 77); its signature `(isize, isize) -> usize` is kept, and
`PTRDIFF_WIDTH` is the one width whose port type is `usize` because the units that size a
buffer with it do so in `usize` arithmetic (`src/sort.rs` 124, `src/buffer.rs` 1358).  The
three `_STDINT_*` helpers (C 124-133) are `macro_rules!` in C's own shape; `_STDINT_MAX` is
defined before `_STDINT_SIGNED_MIN` because a Rust macro body is only visible after its
definition (P-57).  The ten `_C` macros (C 623-675) are `macro_rules!` with a cast instead of
C's literal suffix (a `macro_rules!` cannot append one), and they are defined *before* the
limits that expand them (`INT64_MAX`'s body names `INTMAX_C`, C 405), which is the same P-57
reordering; the values are exercised in the probe (`INTMAX_C! (5) == 5`, …).

**The 31 names that need a `docs/ports/excluded-macros.txt` row** (that file is the
integrator's; the rows are quoted below and were verified by appending them temporarily — with
them `--check` is `OK`, without them it reports exactly these 31 names).  The mirror's own
text for each of them is in place; what has no Rust spelling is C's *body*:

- **26 type renames** (kind=syntax): the template's `# define int8_t gl_int8_t` family, which
  the port spells `pub type int8_t = core::ffi::c_schar;` — an item the extractor's scan does
  not read.  The rows:

```
int8_t	syntax	stdint.in.h 146: `# define int8_t gl_int8_t` is a type rename, and the typedef it renames is dead on the frozen target (`HAVE_C99_STDINT_H` is 1, so the system `<stdint.h>` the template includes next supplies the type, measured `sizeof` 1); the port writes `pub type int8_t = core::ffi::c_schar;` (the intmax_t/Pixel file_offset shape), which the gate's item scan does not read
uint8_t	syntax	stdint.in.h 147: `# define uint8_t gl_uint8_t` is the same type rename (measured: 1 byte, unsigned); the port writes `pub type uint8_t = core::ffi::c_uchar;`
int16_t	syntax	stdint.in.h 153: `# define int16_t gl_int16_t` is a type rename (measured: 2 bytes); the port writes `pub type int16_t = core::ffi::c_short;`
uint16_t	syntax	stdint.in.h 154: `# define uint16_t gl_uint16_t` is the same type rename (measured: 2 bytes, unsigned); the port writes `pub type uint16_t = core::ffi::c_ushort;`
int32_t	syntax	stdint.in.h 160: `# define int32_t gl_int32_t` is a type rename (measured: 4 bytes); the port writes `pub type int32_t = core::ffi::c_int;`
uint32_t	syntax	stdint.in.h 161: `# define uint32_t gl_uint32_t` is the same type rename (measured: 4 bytes, unsigned); the port writes `pub type uint32_t = core::ffi::c_uint;`
int64_t	syntax	stdint.in.h 191: `# define int64_t gl_int64_t` is a type rename (the template defines it four times, one per platform arm; measured: 8 bytes, `long long`); the port writes `pub type int64_t = core::ffi::c_longlong;`
uint64_t	syntax	stdint.in.h 217: `# define uint64_t gl_uint64_t` is the same type rename (measured: 8 bytes, `unsigned long long`); the port writes `pub type uint64_t = core::ffi::c_ulonglong;`
int_least8_t	syntax	stdint.in.h 242: `# define int_least8_t int8_t` is a type rename to the same-width exact type; the port writes `pub type int_least8_t = int8_t;`
uint_least8_t	syntax	stdint.in.h 243: `# define uint_least8_t uint8_t` is the same rename; the port writes `pub type uint_least8_t = uint8_t;`
int_least16_t	syntax	stdint.in.h 244: `# define int_least16_t int16_t` is the same rename; the port writes `pub type int_least16_t = int16_t;`
uint_least16_t	syntax	stdint.in.h 245: `# define uint_least16_t uint16_t` is the same rename; the port writes `pub type uint_least16_t = uint16_t;`
int_least32_t	syntax	stdint.in.h 246: `# define int_least32_t int32_t` is the same rename; the port writes `pub type int_least32_t = int32_t;`
uint_least32_t	syntax	stdint.in.h 247: `# define uint_least32_t uint32_t` is the same rename; the port writes `pub type uint_least32_t = uint32_t;`
int_least64_t	syntax	stdint.in.h 249: `# define int_least64_t int64_t` is the same rename; the port writes `pub type int_least64_t = int64_t;`
uint_least64_t	syntax	stdint.in.h 252: `# define uint_least64_t uint64_t` is the same rename; the port writes `pub type uint_least64_t = uint64_t;`
int_fast8_t	syntax	stdint.in.h 289: `# define int_fast8_t gl_int_fast8_t` is a type rename (measured: 1 byte); the port writes `pub type int_fast8_t = core::ffi::c_schar;`
uint_fast8_t	syntax	stdint.in.h 290: `# define uint_fast8_t gl_uint_fast8_t` is the same rename (measured: 1 byte, unsigned); the port writes `pub type uint_fast8_t = core::ffi::c_uchar;`
int_fast16_t	syntax	stdint.in.h 291: `# define int_fast16_t gl_int_fast16_t` is a type rename (the template's dead arm makes the 16-bit fast type the 32-bit one, but the frozen `int_fast16_t` is `short`, 2 bytes, measured); the port writes `pub type int_fast16_t = core::ffi::c_short;`
uint_fast16_t	syntax	stdint.in.h 292: `# define uint_fast16_t gl_uint_fast16_t` is the same rename (measured: 2 bytes, `unsigned short`); the port writes `pub type uint_fast16_t = core::ffi::c_ushort;`
int_fast32_t	syntax	stdint.in.h 293: `# define int_fast32_t gl_int_fast32_t` is a type rename (the template's non-Sun arm spells `long`, but the frozen type is 4 bytes, `int`, measured); the port writes `pub type int_fast32_t = core::ffi::c_int;`
uint_fast32_t	syntax	stdint.in.h 294: `# define uint_fast32_t gl_uint_fast32_t` is the same rename (measured: 4 bytes, `unsigned int`); the port writes `pub type uint_fast32_t = core::ffi::c_uint;`
int_fast64_t	syntax	stdint.in.h 296: `# define int_fast64_t int64_t` is a type rename to the 64-bit exact type; the port writes `pub type int_fast64_t = int64_t;`
uint_fast64_t	syntax	stdint.in.h 299: `# define uint_fast64_t uint64_t` is the same rename; the port writes `pub type uint_fast64_t = uint64_t;`
intptr_t	syntax	stdint.in.h 323: `# define intptr_t gl_intptr_t` is a type rename (measured: `long`, 8 bytes); the port spells it `pub type intptr_t = isize;` in the `lisp.h` mirror (`src/include/lisp.rs` 1067), which is where C's `lisp.h` 1113 uses the name too (P-26 keeps one spelling)
uintptr_t	syntax	stdint.in.h 324: `# define uintptr_t gl_uintptr_t` is a type rename (measured: `unsigned long`, 8 bytes); the port writes `pub type uintptr_t = usize;`, which the gate's item scan does not read
```

  Two of those rows are already in the file for the other platform shim (`intmax_t`,
  `uintmax_t`), which is why the check reports them as recorded; `intptr_t` is the same shape
  with its Rust spelling in the `lisp.h` mirror.

- **5 dead-text rows** (kind=dead-text): three bodies name the 32-bit fast macros and two are
  m4 placeholders, so neither can be the body of the frozen value:

```
INT_FAST16_MIN	dead-text	stdint.in.h 467's `# define INT_FAST16_MIN INT_FAST32_MIN` is dead on the frozen target (the template's whole limit section is under `#if ! @HAVE_C99_STDINT_H@`, which is false here) and the frozen `int_fast16_t` is `short` (`__INT_FAST16_TYPE__`, 2 bytes, measured), so the value is the 16-bit one (-32768); the mirror spells the frozen limit (`pub const INT_FAST16_MIN: core::ffi::c_int = INT16_MIN;` in `femacs/lib/include/stdint.in.rs`) and docs/ports/include-stdint.in.md records the measurement
INT_FAST16_MAX	dead-text	stdint.in.h 468's `# define INT_FAST16_MAX INT_FAST32_MAX` is dead on the frozen target and the frozen value is 32767 (measured); the mirror spells `pub const INT_FAST16_MAX: core::ffi::c_int = INT16_MAX;`, and docs/ports/include-stdint.in.md records the measurement
UINT_FAST16_MAX	dead-text	stdint.in.h 469's `# define UINT_FAST16_MAX UINT_FAST32_MAX` is dead on the frozen target and the frozen value is 65535 (measured); the mirror spells `pub const UINT_FAST16_MAX: core::ffi::c_int = UINT16_MAX;`, and docs/ports/include-stdint.in.md records the measurement
SIG_ATOMIC_MAX	dead-text	stdint.in.h 562's `_STDINT_MAX (@HAVE_SIGNED_SIG_ATOMIC_T@, @BITSIZEOF_SIG_ATOMIC_T@, 0@SIG_ATOMIC_T_SUFFIX@)` is dead on the frozen target (`HAVE_C99_STDINT_H` is 1, so the system `<stdint.h>` the template includes next supplies the value, measured 2147483647) and its body is the template's own `@PLACEHOLDER@` text, which has no Rust spelling; the mirror spells the frozen value (`pub const SIG_ATOMIC_MAX: core::ffi::c_int = core::ffi::c_int::MAX;` in `femacs/lib/include/stdint.in.rs`) and docs/ports/include-stdint.in.md records the measurement
WCHAR_MAX	dead-text	stdint.in.h 595's `_STDINT_MAX (@HAVE_SIGNED_WCHAR_T@, @BITSIZEOF_WCHAR_T@, 0@WCHAR_T_SUFFIX@)` is dead on the frozen target (`HAVE_C99_STDINT_H` is 1; the frozen `wchar_t` is a 32-bit signed `int`, measured 2147483647) and its body is the template's own `@PLACEHOLDER@` text, which has no Rust spelling; the mirror spells the frozen value (`pub const WCHAR_MAX: core::ffi::c_int = core::ffi::c_int::MAX;` in `femacs/lib/include/stdint.in.rs`) and docs/ports/include-stdint.in.md records the measurement
```

  The counterpart names whose C body the gate does *not* compare (`PTRDIFF_MIN`/`MAX`,
  `SIG_ATOMIC_MIN`, `SIZE_MAX`, `WCHAR_MIN`, `WINT_MIN`/`MAX`, `INTMAX_MIN`/`MAX`,
  `UINTMAX_MAX`, `INTPTR_MIN`/`MAX`, `INT_FAST32_MIN`/`MAX`, `UINT_FAST32_MAX`,
  `GL_INT64_T`, `_UINT*_T`, `INT64_C`/`UINT64_C`/`INTMAX_C`/`UINTMAX_C`) need no row: C defines
  each of them more than once (P-66's `macro_duplicates`) or the name is already recorded
  (`_`, the guard, `intmax_t`/`uintmax_t`).

**`INTMAX_WIDTH`/`UINTMAX_WIDTH` moved here from `limits.in.rs`.**  C defines them in
`lib/stdint.in.h` 725-726, not in `<limits.h>`, and the mirror's `limits.in.rs` carried them
only as a stopgap while no home existed (`docs/ports/include-limits.in.md` says so).  Their C
body is `_GL_INTEGER_WIDTH (INTMAX_MIN, INTMAX_MAX)` / `(0, UINTMAX_MAX)`, which
`limits.in.rs` cannot spell (its text precedes the `INTMAX_*` definitions and the units that
include only `limits.in.rs` — `lib/rawmemchr.rs` and 23 more, none of which names them — would
get an unresolved name), so the two constants now live in this mirror and were deleted from
`limits.in.rs`.  `--check` on `limits.in.h`/`limits.in.rs` is still `OK` afterwards.  **The
sentence in `docs/ports/include-limits.in.md` that lists them as carried by `limits.in.rs` is
now stale and wants the integrator's hand.**

**Convergence** (PORTING.md 0.1-3/0.1-8, the same batch): the two units that repeated a name
this mirror now carries lost the copies, and both still pass `build-tu` and `--check` -

| unit | copy | now |
| --- | --- | --- |
| `src/alloc.rs` 6235-6236 | `pub const UINTPTR_MAX: uintptr_t` (a "gap": "`lib/limits.in.rs` does not carry") | the mirror's `UINTPTR_MAX` |
| `src/pdumper.rs` 209-211 | `pub const UINT_FAST32_WIDTH: c_int = 32` | the mirror's |
| `src/pdumper.rs` 216-219 | `pub const INTPTR_MAX: intptr_t` | the mirror's (`c_long`, read as `INTPTR_MAX as off_t`) |
| `src/pdumper.rs` 222-226 | `pub type int_least32_t`, `INT_LEAST32_MIN`/`_MAX`/`_WIDTH` | the mirror's (`DUMP_OFF_MIN`/`_MAX`/`_WIDTH` still read them, and `dump_off = int_least32_t` now resolves to the mirror's alias) |

`check-mirror-dupes.py` reports no hit for either unit (`1 duplicate declaration(s), 1 ratchet
row(s), 0 failure(s)`, the row being `src/insdel.rs`'s pre-existing `bset_redisplay`).

**Not edited** (another lane's files, reported instead of patched):

- `src/include/lisp.rs` 52-53/1067 declares `intmax_t`/`uintmax_t`/`intptr_t`; this mirror
  declares the first two names nowhere and the third not at all, because a second declaration
  of the same type name in one crate root is an `E0428` and every unit on `stdint.in.rs` is
  also on `lisp.rs` (P-26: one spelling).
- `nt/inc/include/stdint.rs` (the other platform shim's mirror) defines `INT32_MAX`,
  `UINT32_MAX`, `INT64_MAX`, `INTPTR_MAX`, `UINTPTR_MAX`, `INTMAX_MAX`, `SIZE_MAX`, … and
  `intptr_t`/`uintmax_t`/`uint32_t`/`uint64_t`; a future `nt/` unit that includes both mirrors
  would collide (`E0428`).  No unit includes both today (`nt/inc/stdint.h` has no unit yet).
- `_GL_JUST_INCLUDE_SYSTEM_INTTYPES_H` is defined here because `lib/stdint.in.h` 34 defines it
  and `lib/inttypes.in.h` 32/46 is the *reader*; that header has no mirror yet, so nothing
  redefines the marker today.

## Evidence

```
# the frozen substitutions (the generated header the port mirrors)
$ sed -n '74,90p' build/emacs-ref/lib/stdint.h
#define _GL_STDINT_H
#include <limits.h>
#if 0                    <- the WINT_MIN/WINT_MAX override
#if ! 1                  <- HAVE_C99_STDINT_H is 1: the type/limit section is dead

# the values and the types, measured with the frozen clang
$ clang -std=gnu23 -I build/emacs-ref/src -I emacs/src -I emacs/lib -I build/emacs-ref/lib \
    -include config.h /tmp/femacs-stdint-probe.c -o /tmp/femacs-stdint-probe && /tmp/femacs-stdint-probe
INT8_MIN -128             INT8_MAX 127            UINT8_MAX 255
INT16_MIN -32768          INT16_MAX 32767         UINT16_MAX 65535
INT32_MIN -2147483648     INT32_MAX 2147483647    UINT32_MAX 4294967295
INT64_MIN -9223372036854775808   INT64_MAX 9223372036854775807
UINT64_MAX 18446744073709551615
INT_LEAST8_MIN -128  INT_LEAST8_MAX 127  UINT_LEAST8_MAX 255        (least16/32/64 likewise)
INT_FAST8_MIN -128   INT_FAST8_MAX 127   UINT_FAST8_MAX 255         (fast16/32/64 = 16/32/64-bit)
INTPTR_MIN -9223372036854775808  INTPTR_MAX 9223372036854775807  UINTPTR_MAX 18446744073709551615
INTMAX_MIN -9223372036854775808  INTMAX_MAX 9223372036854775807  UINTMAX_MAX 18446744073709551615
PTRDIFF_MIN/MAX the 64-bit pair  SIZE_MAX 18446744073709551615
SIG_ATOMIC_MIN -2147483648  SIG_ATOMIC_MAX 2147483647
WCHAR_MIN -2147483648  WCHAR_MAX 2147483647  WINT_MIN -2147483648  WINT_MAX 2147483647
INT8_WIDTH 8  INT16_WIDTH 16  INT32_WIDTH 32  INT64_WIDTH 64   (all three families alike)
INTPTR_WIDTH 64  UINTPTR_WIDTH 64  INTMAX_WIDTH 64  UINTMAX_WIDTH 64
PTRDIFF_WIDTH 64  SIZE_WIDTH 64  WCHAR_WIDTH 32  WINT_WIDTH 32  SIG_ATOMIC_WIDTH 32
sizeof int_fast8 1  int_fast16 2  int_fast32 4  int_fast64 8

# the same probe's `_Generic` pass (the type of each macro) and clang's own type macros:
INT8_MAX int … UINT32_MAX uint … INT64_MAX llong … UINT64_MAX ullong
INTPTR_MAX long  UINTPTR_MAX ulong  INTMAX_MAX long  UINTMAX_MAX ulong
$ clang -dM -E -std=gnu23 … | grep __INT_FAST16_TYPE__
#define __INT_FAST16_TYPE__ short      (__INT_FAST32_TYPE__ int, __UINT_FAST16_TYPE__ unsigned short)

# the mirror itself compiles and every measured value is pinned in it
$ rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    /tmp/femacs-mir4-probe/probe.rs -o /tmp/femacs-mir4-probe/probe.o
exit=0        # the probe `include!`s the mirror and asserts every value, every width, every
              # alias's `size_of`, the markers and the `_C`/`_STDINT_*` macros

# three real consumers of the mirror, plus the two units this batch converged
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir4 cargo run -q -p xtask -- build-tu <unit.c>
lib/byteswap.c   -> built lib/byteswap.rs    (byteswap.in.rs 16 includes stdint.in.rs)
lib/regcomp.c    -> built lib/regcomp.rs
src/bignum.c     -> built src/bignum.rs      (INTMAX_WIDTH/UINTMAX_WIDTH/UINTMAX_MAX)
src/fileio.c     -> built src/fileio.rs
src/pdumper.c    -> built src/pdumper.rs     (converged; DUMP_OFF_*/DUMP_BITSET_WORD_WIDTH read the mirror)
src/alloc.c      -> built src/alloc.rs       (converged; UINTPTR_MAX read from the mirror)

# the gates (with the 31 rows appended to docs/ports/excluded-macros.txt)
$ python3 tools/extract-abi.py --check emacs/lib/stdint.in.h femacs/lib/include/stdint.in.rs
OK emacs/lib/stdint.in.h -> femacs/lib/include/stdint.in.rs
$ python3 tools/extract-abi.py --check emacs/lib/limits.in.h femacs/lib/include/limits.in.rs
OK emacs/lib/limits.in.h -> femacs/lib/include/limits.in.rs
$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit

# the same three checks on the two converged units
$ python3 tools/extract-abi.py --check emacs/src/alloc.c femacs/src/alloc.rs
OK emacs/src/alloc.c -> femacs/src/alloc.rs
$ python3 tools/extract-abi.py --check emacs/src/pdumper.c femacs/src/pdumper.rs
OK emacs/src/pdumper.c -> femacs/src/pdumper.rs
$ python3 tools/extract-abi.py --check emacs/lib/endian.in.h femacs/lib/include/endian.in.rs
OK emacs/lib/endian.in.h -> femacs/lib/include/endian.in.rs
$ python3 tools/extract-abi.py --check emacs/lib/byteswap.in.h femacs/lib/include/byteswap.in.rs
OK emacs/lib/byteswap.in.h -> femacs/lib/include/byteswap.in.rs

# without those 31 rows (the state this lane leaves behind: the file is the integrator's)
$ python3 tools/extract-abi.py --check emacs/lib/stdint.in.h femacs/lib/include/stdint.in.rs
FAIL emacs/lib/stdint.in.h -> femacs/lib/include/stdint.in.rs   (31 problem lines: the 26 type
renames, the three 16-bit fast limits and the two `@PLACEHOLDER@` bodies listed above)
```
