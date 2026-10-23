# emacs/src/decompress.c (349 lines)

> **This ledger overturns the entry it replaces.**  The previous revision of this file recorded
> `decompress.c` as an empty unit under P-08 ("the whole body is inside a disabled `#if`, so
> nothing is demanded of the port"), with the checklist sections all at zero.  **P-86 rules that
> the other way**: a file whose正文 sits entirely under a condition the frozen configuration
> leaves false is *ported by transcription*, both arms of every `#if` carried, and its ledger
> records a **C line-range table** instead of ticked boxes, because `extract-abi.py`'s checklist
> for it is empty (P-73's live-line filter; the file *is* in
> `docs/baseline/config/cond-eval-verified.txt` 261) and the checkbox sections are therefore not
> coverage evidence.  The port is `src/decompress.rs` (627 lines), the file gate is
> `feature = "have_zlib"`, and the frozen `build-tu` reading below still has to be the empty
> object it was - the transcription must not leak a single line into this configuration.

`HAVE_ZLIB` is `undef` (`docs/baseline/config/darwin-arm64.macros.tsv` 1721) and `HAVE_NATIVE_COMP`
is `undef` (1473); `WINDOWSNT` is `undef` (4010).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`#ifdef HAVE_ZLIB`, C 21) | `feature = "have_zlib"` (on every item and on C 23-28's include graph) | a zlib configuration |
| C 30 `#ifdef WINDOWSNT` … C 65 (DLL loading, `fn_*` renames) | `all (feature = "have_zlib", feature = "windowsnt")` | a Windows configuration |
| C 68 `#ifdef HAVE_NATIVE_COMP` … C 167 (`md5_gz_stream` and its two helpers) | `all (feature = "have_zlib", feature = "have_native_comp")` | zlib + native compilation |
| C 108-116, C 249-257 (the `#ifdef WINDOWSNT` blocks inside the bodies) | `feature = "windowsnt"` (nested in the file gate) | a Windows configuration |
| C 206-220 (`zlib-available-p`'s two arms) | `feature = "windowsnt"` / `not (feature = "windowsnt")` | both arms carried |
| C 57-63 (`#undef`/`#define inflate …`) | `windowsnt`, realized as the arms' two call spellings (see Deviations) | - |

`have_native_comp` is the feature the tree already registered for `HAVE_NATIVE_COMP`
(`docs/baseline/feature-coverage.tsv`, `enabled by: xtask dead-arms`); `windowsnt` is the name
`src/vm-limit.rs` 95 uses.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, PORTING.md 0.1-19) |
| 19 | `#include <config.h>` | ungated (every unit includes it) |
| 21 | `#ifdef HAVE_ZLIB` | the file gate |
| 23 | `#include <zlib.h>` | file gate; the header has no file in the reference tree, so its declarations are carried in the unit (P-88) |
| 25-28 | `lisp.h`, `buffer.h`, `composite.h`, `md5.h` | file gate (plus `character.rs` for buffer.h's `CHAR_STRING` family, and `string.rs`/`limits.in.rs`, which reach C through lisp.h 28/32) |
| 30-65 | `#ifdef WINDOWSNT`: C 31-33 the three includes, C 35-39 the three `DEF_DLL_FN`, C 41 `zlib_initialized`, C 43-55 `init_zlib_functions`, C 57-59 the `#undef`s, C 61-63 the three `#define` renames | `windowsnt` (nested) |
| 68-167 | `#ifdef HAVE_NATIVE_COMP`: C 70 `MD5_BLOCKSIZE`, C 72-73 `acc_buff`/`acc_size`, C 75-89 `accumulate_and_process_md5`, C 91-99 `final_process_md5`, C 101-165 `md5_gz_stream`, C 166 `#undef MD5_BLOCKSIZE` | `have_native_comp` (nested) |
| 171-175 | `struct decompress_unwind_data` | file gate |
| 177-200 | `unwind_decompress` | file gate / `windowsnt` (the inner C 180 `inflateEnd` spelling) |
| 202-221 | `DEFUN ("zlib-available-p", …)` | file gate / `windowsnt` (C 206-220, whose `#else` arm is C 218-220) |
| 223-336 | `DEFUN ("zlib-decompress-region", …)` | file gate / `windowsnt` (C 249-257) |
| 338-347 | the `Initialization` banner and `syms_of_decompress` | file gate |
| 349 | `#endif /* HAVE_ZLIB */` | the file gate's end |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/femacs-dc-target cargo run -q -p xtask -- build-tu src/decompress.c`
→ `built src/decompress.rs -> ../build/femacs/decompress.o`, **exit 0**, and
`build/femacs/decompress.o` is **336 bytes** - an empty object, unchanged from the empty-unit
revision.  That is the expected reading, not a proof of the new text: every item sits behind
`have_zlib`, so under the frozen feature set rustc compiles only `include/config.rs` and the
`#![allow]` attribute.

**② The arm flip (P-86 ③⑴).**
`cargo run -q -p xtask -- probe src/decompress.rs /tmp/femacs-port-probe/decompress_x.o
--crate-type lib --emit obj -C panic=abort --cfg 'feature="have_zlib"'`
→ **exit 0**, **0 errors**, 5 warnings (2 `unused doc comment`, both in mirror copies -
`lib/include/stdbit.in.rs` 644 and `include/lisp.rs` 3934; 3 `` `extern` block uses type
`internal_state`, which is not FFI-safe ``), object **3,744 bytes**.
`nm -g` on it shows the unit's own definitions and the references they expect:
`T _Fzlib_available_p`, `T _Fzlib_decompress_region` and
`U _inflate`, `U _inflateEnd`, `U _inflateInit2_`, `U _del_range_2`, `U _insert_from_gap`,
`U _defsubr`, `U _error`, `U _current_buffer`, `U _make_gap`, `U _Vquit_flag`, `U _cons_to_signed`,
`U _cons_to_unsigned`, `U _make_bigint`, `U __setjmp` (the last five come from the mirror code the
materialized root also carries).  **0 of the errors (there are none) are in a mirror copy, and 0 in
the unit's own text** - unlike the first P-86 file, whose flip died on mirror name-resolution
before reaching its own body.

**③ The sub-gate flip, and why it is P-51's class.**
`… --cfg 'feature="have_zlib"' --cfg 'feature="have_native_comp"'` → **exit 1, 1 error**:
`error[E0080]: evaluation panicked: assertion failed: !cfg!(feature = "have_native_comp") ||
HAVE_NATIVE_COMP` at `src/include/lisp.rs` 2853 - the `const _: () = assert! (…)` P-51
requires to bind a carried feature to its configuration constant.  A bare `--cfg` therefore
**cannot** compile C 68-167's arm, by design; its compile reading belongs to `xtask dead-arms`
(which the task's budget excludes), so `md5_gz_stream`, `accumulate_and_process_md5` and
`final_process_md5` are carried text this round.

**④ The `windowsnt` arm is `text-only` (P-86 ③⑵).**  This machine has **no Windows cross
toolchain** (`which x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc` → empty), so C 30-65 cannot be
compiled from this host at all.  What is recorded instead: the arm's items are all under
`all (have_zlib, windowsnt)`, so the frozen reading ① is unaffected, and the flip ② does not
select them (`0 errors` there is *not* a reading of this arm).

**⑤ `<zlib.h>`'s layout, measured (not remembered).**  The unit's P-88 section carries the type
and constant declarations with clang readings from this machine's SDK header
(`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/
usr/include/zlib.h`, `ZLIB_VERSION` "1.2.12"):
```
$ clang -o /tmp/femacs-port-probe/zprobe /tmp/femacs-port-probe/zprobe.c && /tmp/femacs-port-probe/zprobe
ZLIB_VERSION=1.2.12
sizeof(z_stream)=112 alignof=8
offsetof next_in=0 avail_in=8 total_in=16 next_out=24 avail_out=32 total_out=40
offsetof msg=48 state=56 zalloc=64 zfree=72 opaque=80 data_type=88 adler=96 reserved=104
MAX_WBITS=15 Z_NULL=0 Z_OK=0 Z_STREAM_END=1 Z_NO_FLUSH=0
sizeof(Byte)=1 sizeof(uInt)=4 sizeof(uLong)=8 sizeof(voidpf)=8
sizeof(z_streamp)=8 sizeof(alloc_func)=8 sizeof(free_func)=8
sizeof(struct internal_state *)=8 sizeof(z_const char *)=8
```
(the same readings appear in the unit's section comment):

```c
/* /tmp/<prefix>/zprobe.c - the layout probe (the task's P-88 ② shape); the temporary
   directory is cleaned up, so the source is kept here.  */
#include <zlib.h>
#include <stdio.h>
#include <stddef.h>
int main (void)
{
  printf ("ZLIB_VERSION=%s\n", ZLIB_VERSION);
  printf ("sizeof(z_stream)=%zu alignof=%zu\n", sizeof (z_stream), _Alignof (z_stream));
  printf ("offsetof next_in=%zu avail_in=%zu total_in=%zu next_out=%zu avail_out=%zu total_out=%zu\n",
          offsetof (z_stream, next_in), offsetof (z_stream, avail_in), offsetof (z_stream, total_in),
          offsetof (z_stream, next_out), offsetof (z_stream, avail_out), offsetof (z_stream, total_out));
  printf ("offsetof msg=%zu state=%zu zalloc=%zu zfree=%zu opaque=%zu data_type=%zu adler=%zu reserved=%zu\n",
          offsetof (z_stream, msg), offsetof (z_stream, state), offsetof (z_stream, zalloc),
          offsetof (z_stream, zfree), offsetof (z_stream, opaque), offsetof (z_stream, data_type),
          offsetof (z_stream, adler), offsetof (z_stream, reserved));
  printf ("MAX_WBITS=%d Z_NULL=%d Z_OK=%d Z_STREAM_END=%d Z_NO_FLUSH=%d Z_FINISH=%d\n",
          MAX_WBITS, Z_NULL, Z_OK, Z_STREAM_END, Z_NO_FLUSH, Z_FINISH);
  printf ("sizeof(Byte)=%zu sizeof(uInt)=%zu sizeof(uLong)=%zu sizeof(voidpf)=%zu\n",
          sizeof (Byte), sizeof (uInt), sizeof (uLong), sizeof (voidpf));
  printf ("sizeof(z_streamp)=%zu sizeof(alloc_func)=%zu sizeof(free_func)=%zu\n",
          sizeof (z_streamp), sizeof (alloc_func), sizeof (free_func));
  printf ("sizeof(struct internal_state *)=%zu sizeof(z_const char *)=%zu\n",
          sizeof (struct internal_state *), sizeof (z_const char *));
  return 0;
}
```


**⑥ The gates (repo root, readings taken on this revision).**
The worktree is shared with other lanes (P-87 ②: the main worktree's number includes lane
in-flight changes), so the repo-wide counts move; the per-file readings are the ones below.
`python3 tools/check-dead-file-coverage.py` → `5 ledger(s) bound (a C range table), 0 failure(s)`
(this file's 13 rows cover `src/decompress.c`, 349 lines, 14 blank line(s) between blocks need no
row). — `python3 tools/check-port-lines.py` → `92 file(s) under ten percent, 243,405 reference
line(s); 95 ratchet row(s), 0 new, 3 stale`, one of the stale rows being
`src/decompress.c: no longer under the threshold` (this unit's row retires; the others are two
other lanes' rows).  `--write-pending` is the integrator's step. —
`python3 tools/check-citations.py` → `675 file(s) scanned, 1 drift, 0 range, 456 external, 33
allowed, 0 stale`; the one drift is `src/androidselect.rs:97` (another lane's file), **0
of the findings name this unit**, and the external count rose from 350 to 456 because the zlib.h
and zconf.h citations name a header the tree does not carry. — `python3
tools/check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed literals), 0
failures` (the reading is unchanged by this file; the ledger is scanned for the `decl`/`version`/
`title` classes and adds none). — `python3 tools/check-carried.py` → `11 carried file(s), 11
baseline row(s), 0 failure(s)`. — `python3 tools/check-ledger-ticks.py` → `235 ledger(s) bound
(a function tick to compare), 0 failure(s)`, unchanged for this file: its ledger carries no
function tick to bind, which is the P-86 class's shape.

**⑥a The `#if` decisions.**  `python3 tools/check-cond-eval.py src/decompress.c` → `6
conditions, 0 unknown, 0 disagreements [VERIFIED]`: the guard, `HAVE_ZLIB`, `WINDOWSNT`,
`HAVE_NATIVE_COMP` and the two nested `WINDOWSNT` blocks are all decided by the frozen table,
which is why the Gate mapping above and the port's `#[cfg]`s agree item for item.

**⑦ `extract-abi.py --check` FAILs, and the six lines are P-86 ⑩'s owed baseline row.**
`python3 tools/extract-abi.py --check emacs/src/decompress.c src/decompress.rs` → **FAIL**,
with exactly these six lines and no others:
```
- DEFUN! Fzlib_available_p: the reference file has no such DEFUN
- DEFUN! Fzlib_decompress_region: the reference file has no such DEFUN
- unexpected defsubr (&Szlib_available_p): the reference file does not register it
- unexpected defsubr (&Szlib_decompress_region): the reference file does not register it
- fn md5_gz_stream carries #[unsafe(no_mangle)] but the reference does not define a non-static function of that name (invented exported symbol)
- fn syms_of_decompress carries #[unsafe(no_mangle)] but the reference does not define a non-static function of that name (invented exported symbol)
```
Every line is the same finding: the file is in `cond-eval-verified.txt` (261), the frozen table
decides `HAVE_ZLIB` as `undef`, P-73's live-line filter therefore yields an **empty** item list for
the *reference*, while the port declares the items the dead arm contains - so each correct item
reads as invented.  The checklist cannot be the evidence here (P-86 ②) and this port does not
change that; what the integrator needs is the extractor rule (or baseline row) that says "this
file's whole body is dead, compare the port against the file's text, not the live lines".

## Deviations

- **The `<zlib.h>` section is the stand-in for an external header** (P-88 ④): the reference tree
  has no `zlib.h`, so the types, constants and prototypes this unit uses live in the unit at C 23's
  position, every declaration carrying the header's own line (P-55), with the layout measured by
  the clang probe in Evidence ⑤ rather than remembered.  A future tree that grows a `zlib.h` mirror
  moves this section into it and deletes the copy in the same commit (P-88 ⑤).
- **`DEF_DLL_FN`/`LOAD_DLL_FN` are written in long form** (C 35-39, C 51-53): C pastes
  `W32_PFN_##func`/`fn_##func` from one identifier and takes the parameter list as `__VA_ARGS__`,
  which Rust cannot express; `emacs/src/w32common.h` 77-97 is a `kind=deferred` row in
  `docs/ports/excluded-macros.txt` and `src/include/w32common.rs` keeps the C bodies as a
  comment (P-22 says the shape is the first consuming target's decision).  This unit is that
  consumer: the two generated names are macro parameters, the parameter list is a `ty` fragment,
  and the loader's cast `(W32_PFN_##func) get_proc_addr (…)` is a `transmute` (C's cast between two
  function-pointer types; Rust has no `as` cast to an `Option<fn>`), with C's `#func` passed as a
  `c"…"` literal.  Nothing of this is compiled on this host (Evidence ④); the Windows target must
  re-verify the arm.
- **`#define inflate fn_inflate` (C 61-63) has no Rust counterpart** - an object-like `#define`
  renames an identifier, and Rust's only renames (`use … as …`) cannot carry a callable meaning.
  The port therefore spells C's *preprocessed* text per arm at the seven call sites (`fn_inflate
  (…)` / `fn_inflateEnd (…)` / the `windowsnt` `inflateInit2` body against `inflate (…)` /
  `inflateEnd (…)` / `inflateInit2! (…)`), which is the `ccl.rs` rule for a macro-expanded token,
  while the three pointer objects keep C's own names.  The header's own function-like
  `inflateInit2` macro is carried as `macro_rules! inflateInit2` (PORTING.md 2.1) in two
  cfg-exclusive definitions, one per arm, because a `macro_rules!` body resolves its names where
  it is defined.
- **Uninitialised locals are zeroed (PORTING.md 1.5/0.1-13).**  Named here: `stream` (C 104),
  `in`/`out` (C 105-106), `ctx` (C 120) and `istart`/`iend`/`pos_byte`/`inflate_status`/
  `unwind_data` (C 238-241), each of which C fills before its first read; `static char acc_buff[]`
  and `static ptrdiff_t acc_size` (C 72-73) and `static bool zlib_initialized` (C 41) are C
  tentative definitions, i.e. zeroed by C's own rule and written as such.
- **`stream.zalloc`/`zfree = Z_NULL` is `None`** (C 124-125): C's `Z_NULL` (an `int` 0) assigned
  to a function pointer is the null function pointer, which the port's `Option<unsafe extern "C"
  fn>` types spell `None` (PORTING.md P-27); the two pointer members (`opaque`, `next_in`) keep the
  `Z_NULL as …` spelling.
- **Mirror gaps declared at the use site (P-72 rule 4).**  `insert_from_gap` (lisp.h 4334) and
  `del_range_2` (lisp.h 5273) are not in the frozen mirrors; the unit declares both beside the
  others (`coding.rs` 106-108 and `buffer.rs` 123-126 are where the same two are declared for other
  units).  They belong in `lisp.rs` when that mirror is next extended.
- **`include/character.rs` is included before `buffer.rs`**: `buffer.h`'s `CHAR_STRING` family
  lives in `character.h` and `buffer.rs` names it, exactly as `dired.rs`'s include list records
  (PORTING.md 0.1-4's include-graph rule).
- **Numeric conversions the port spells**: `min (iend - pos_byte, UINT_MAX as ptrdiff_t)` (C 298),
  `make_int ((iend - pos_byte) as intmax_t)` (C 321), `GAP_SIZE! () < avail_out as ptrdiff_t`
  (C 302-303), `avail_in as uInt` / `avail_out as uInt` (C 305/307) and the two `bool` arguments C
  passes as `0`/`false` (C 187-189, C 311, C 329-330).
- **Line count.**  C 349 vs Rust 627 (79.7%): P-86's per-item `#[cfg]` gates (both arms carried),
  the P-88 declaration section, the per-arm spellings of three renames, and Rust's expression
  verbosity.  `extract-abi`'s line-count row does not fire here (the reference's live set is empty,
  so it compares nothing - Evidence ⑦); a `docs/ports/line-drift-tolerance.tsv` row is owed by the
  integrator if the rule later binds this class.

## Owed by the integrator (nothing here is claimed as done)

1. `tools/extract-abi.py`: a rule (or a baseline row) for a file whose whole body is dead - see
   Evidence ⑦.  Six lines, all one finding.
2. `tools/check-port-lines.py --write-pending`: the `src/decompress.c` row is stale (retired).
3. `docs/ports/excluded-macros.txt`: the `DEF_DLL_FN` / `LOAD_DLL_FN` rows are `kind=deferred` and
   stay so (their first consumer is this unit, and the shape is recorded here), but the rows'
   reason text can now point at `src/decompress.rs` as the first consumer's shape.
4. `docs/baseline/feature-coverage.tsv` (generated): gains the `have_zlib` row.
5. `docs/port-map.md` (generated): the target's line count for `src/decompress.c` is stale (12).
