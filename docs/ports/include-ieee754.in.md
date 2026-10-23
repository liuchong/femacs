# emacs/lib/ieee754.in.h (215 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] IEEE754_DOUBLE_BIAS  (line 141, body c306c90e0c2f)
- [x] IEEE754_FLOAT_BIAS  (line 79, body a19ed90b4eb3)
- [x] IEEE854_LONG_DOUBLE_BIAS  (line 209, body 16d5cf9c21cc)
- [x] _IEEE754_BIG_ENDIAN  (line 33, body a10f69d43d8a)
- [x] _IEEE754_BYTE_ORDER  (line 32, body 852c3ad67ba1)
- [x] _IEEE754_FLOAT_WORD_ORDER  (line 35, body 852c3ad67ba1)
- [x] _IEEE754_H  (line 19, body 356a192b7913)
- [x] _IEEE754_LITTLE_ENDIAN  (line 34, body be4c98d225b1)

## Types and their fields — 3
- [x] ieee754_float: float f, `ieee` {mantissa:23, exponent:8, negative:1}, `ieee_nan` {mantissa:22, quiet_nan:1, exponent:8, negative:1}
- [x] ieee754_double: double d, `ieee` {mantissa1:32, mantissa0:20, exponent:11, negative:1}, `ieee_nan` {mantissa1:32, mantissa0:19, quiet_nan:1, exponent:11, negative:1}
- [x] ieee854_long_double: long double d, `ieee` {mantissa1:32, mantissa0:32, exponent:15, negative:1, empty:16}, `ieee_nan` {mantissa1:32, mantissa0:30, quiet_nan:1, one:1, exponent:15, negative:1, empty:16}

(the member lists above are the live little-endian / little-word-order arm; the big-endian arms of the same structs carry the same names in the reverse order, and the extractor's own row prints every branch's names in one line)

## File-scope objects — 0

- **Status: the unit is PORTED** — `lib/include/ieee754.in.rs` (224 lines, 2026-10-04).

# ----------------------------------------------------------------------
## Decisions

**Criterion.** The generated header, `build/emacs-ref/lib/ieee754.h` (5,660 bytes), spells the
template's glibc arm `#if 0` at its line 20 and the gnulib arm live at 24-31.  So the four order
knobs name `<endian.h>`'s `BYTE_ORDER` / `BIG_ENDIAN` / `LITTLE_ENDIAN` / `BYTE_ORDER`, whose
owner in this tree is `lib/include/endian.in.rs` 28-34 (`LITTLE_ENDIAN` 1234, `BIG_ENDIAN` 4321);
this mirror includes that file and keeps C's own name in each initializer
(`pub const _IEEE754_BYTE_ORDER: core::ffi::c_int = BYTE_ORDER;`), which is also what the macro
body check compares.  Measured on this target: `BYTE_ORDER` 1234, `BIG_ENDIAN` 4321,
`FLOAT_WORD_ORDER` 1234 - i.e. every `# if _IEEE754_FLOAT_WORD_ORDER == _IEEE754_BIG_ENDIAN`
arm below is the dead one.

**The include guard's body is `1`.**  `pub const _IEEE754_H: core::ffi::c_int = 1;` - a
`bool = true` spelling fails the body check (`- macro _IEEE754_H does not mention 1`), so the
const keeps C's value the way `byteswap.in.rs` 23 and `dirent.in.rs` 62 do.

**The three unions are bitfield carriers (PORTING.md P-80).**  Rust has no bitfields, so each
union is a storage unit - `u32`, `[u32; 2]`, `[u32; 3]` - behind `#[repr(C)]`, with the sizes and
alignments asserted from the frozen clang's own `sizeof`/`_Alignof` (4/4, 8/8, 16/8 for the
unions; 4/4, 8/4, 12/4 for their member structs).  Reads go through one accessor macro per
union whose **first argument is C's own member path** (`ieee754_double_bits! (ieee_nan.mantissa0, u)`).
That shape is required, not cosmetic: C's token set for a body that writes `u.ieee_nan.mantissa0`
contains `ieee_nan` and `mantissa0` separately, so a macro named after the *variant*
(`ieee754_double_ieee_nan_bits!`) drops the `ieee_nan` token and the gate reports
`- function float_to_string does not mention ieee_nan` (measured, then fixed this way).  A
one-bit member returns `bool` (C's `!` is logical; Rust's `!` on an integer is bitwise, and both
consumers write `!`), every other member returns its numeric field value (`exponent` shifted
down).

**`long double` is the 8-byte double on this target.**  The union's measured 16 bytes are
`ceil (12, 8)`: the member struct is 12 bytes with align 4 and the union's alignment comes from
the `d` member, which is `core::ffi::c_double` (the `editfns.rs` 197 `pub type c_longdouble =
c_double;` precedent, same measurement).

**FFI types are fully qualified.**  A bare `c_int` / `c_float` / `c_double` is `E0425` in the
single-crate layout the materializer produces (the `stdio.rs` 644-653 precedent), so this mirror
spells them `core::ffi::*`.

**Both consumers now point here.**  `src/print.rs` (C 1200-1208's NaN arm) and `src/lread.rs`
(C 4572) each declared their own copy of `union ieee754_double` with a comment saying the
template "has no mirror" (P-26/P-62); both blocks are deleted and replaced by
`include!("../lib/include/ieee754.in.rs")`, and the measurements they carried are the ones in
this ledger.  Their `--check` verdicts are unchanged (both `OK`) and both units build.

## Evidence

The criterion, the layouts and the bit positions were measured with the frozen clang against the
generated header (the probe and its output, verbatim):

```
$ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib \
      -o /tmp/ieee754_probe /tmp/ieee754_probe.c && /tmp/ieee754_probe
union ieee754_float: size 4 align 4
union ieee754_double: size 8 align 8
union ieee854_long_double: size 16 align 8
IEEE754_FLOAT_BIAS 127 DOUBLE_BIAS 1023 LONG_DOUBLE_BIAS 16383
BYTE_ORDER 1234 BIG 4321 LITTLE 1234 FLOAT_WORD_ORDER 1234
float ieee bytes: 01 00 80 bf
double ieee bytes: 01 00 00 00 ff ff ff bf
long double ieee bytes: 01 00 00 00 01 00 00 00 ff bf 00 00 01 00 00 00
variant float ieee: size 4 align 4
variant float ieee_nan: size 4 align 4
variant double ieee: size 8 align 4
variant double ieee_nan: size 8 align 4
variant ld ieee: size 12 align 4
variant ld ieee_nan: size 12 align 4
```

(The probe includes `config.h` first, sets the fields named on each dump line and prints the
object's bytes, which is what fixes each bit position recorded in the mirror's comments:
`mantissa` 0-22 / `exponent` 23-30 / `negative` 31 for the float; `mantissa1` in unit 0 and
`mantissa0` 0-19 / `exponent` 20-30 / `negative` 31 in unit 1 for the double; `mantissa1` unit 0,
`mantissa0` unit 1, `exponent` 0-14 / `negative` 15 / `empty` 16-31 in unit 2 for the long
double.)

```
$ python3 tools/extract-abi.py --check emacs/lib/ieee754.in.h lib/include/ieee754.in.rs
OK emacs/lib/ieee754.in.h -> lib/include/ieee754.in.rs            (0 problem rows)
$ python3 tools/extract-abi.py --check emacs/src/print.c src/print.rs
OK emacs/src/print.c -> src/print.rs
$ python3 tools/extract-abi.py --check emacs/src/lread.c src/lread.rs
OK emacs/src/lread.c -> src/lread.rs
$ cd femacs && cargo run -q -p xtask -- build-tu src/print.c
built src/print.rs -> ../build/femacs/print.o
$ cd femacs && cargo run -q -p xtask -- build-tu src/lread.c
built src/lread.rs -> ../build/femacs/lread.o
```
