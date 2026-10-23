# emacs/exec/mipsfpu.c (289 lines)

> **Not compiled by any configuration the port builds.**  `docs/baseline/frozen-build-sources.txt`
> lists no `exec/` object (the loader belongs to the unexec dump path; this build is
> `HAVE_PDUMPER=1`).  PORTING.md P-86 governs this class - the body is **transcribed as it stands**
> (the previous revision was the three-line P-08 note) - and P-86 ②'s C line-range table is below.
> Port: `exec/mipsfpu.rs` (310 lines), first line
> `// femacs: port of exec/mipsfpu.c @ fd7afced403.`

# ----------------------------------------------------------------------
## Gate mapping

`python3 tools/check-cond-eval.py exec/mipsfpu.c` reads `1 conditions, 0 unknown,
0 disagreements [VERIFIED]`: the file's only `#if` is C 140-145's
`#if defined __mips_isa_rev && __mips_isa_rev >= 6` inside `cpu_supports_fr0_p`.  It is a compiler
builtin rather than a measured configuration macro, so the port names its two arms with the feature
`mips_isa_rev_ge_6` (the `android_api_ge_9` naming precedent, PORTING.md 0.1-7) and the frozen
target - which has no `__mips_isa_rev` - compiles the `#else` arm.

| C | condition | arm taken here | feature |
| --- | --- | --- | --- |
| 140 | `#if defined __mips_isa_rev && __mips_isa_rev >= 6` | false (the evaluator's reading; this host has no `__mips_isa_rev`) | `mips_isa_rev_ge_6` (the positive arm) |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | licence and ownership block | dropped (0.1-19) |
| 20-23 | the include block: `config.h`, `<errno.h>`, `"mipsfpu.h"` | — |
| 24-40 | the "OABI MIPS systems support several different modes …" commentary | transcribed in the C's order as comments where the port needs them |
| 41-50 | the eight `MIPS_ABI_FP_*` numbers | — |
| 51-61 | the eleven `EF_MIPS_*` ELF flag words | — |
| 62-67 | the `struct mode_description` banner | — |
| 68-90 | `struct mode_description` | — |
| 91-104 | `static struct mode_description fpu_reqs[8]` | — |
| 105-118 | `static bool valid_abi_p (int abi)` | — |
| 119-133 | `static int fp_mode_for_abi (int abi)` | — |
| 134-146 | `bool cpu_supports_fr0_p (void)` | `mips_isa_rev_ge_6` |
| 147-289 | `int determine_fpu_mode (…)` | — |

## Evidence

* `cargo run -q -p xtask -- build-tu exec/mipsfpu.c` → `built exec/mipsfpu.rs ->
  ../build/femacs/mipsfpu.o`, **exit 0**, object **10,904 bytes**; `nm -g` shows
  `_cpu_supports_fr0_p` and `_determine_fpu_mode` defined (C's two non-`static` definitions).
* `python3 tools/extract-abi.py --check emacs/exec/mipsfpu.c exec/mipsfpu.rs` → one token
  row (`function valid_abi_p does not mention case, default, switch`, exempted in
  `macro-body-notes.tsv`) and a 7.3% line drift (declared in `line-drift-tolerance.tsv`).
* `python3 tools/check-cond-eval.py exec/mipsfpu.c` → `1 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`.

## Deviations

1. **C's `switch` in `valid_abi_p`** is the port's `match` (eight `case`s that all return `true`
   plus a `default` that returns `false`); the `case`/`default`/`switch` tokens have no site
   (PORTING.md 2.2, the `macro-body-notes.tsv` row).
2. **`fpu_reqs[abi]`** is spelled as a raw-pointer `offset`, so a negative `abi` is the undefined
   behaviour C has rather than a Rust panic (PORTING.md P-72); the same for the two descriptor
   lookups in `determine_fpu_mode`.
3. **C's five uninitialised locals** (C 176-178) are written with C's own zero values
   (PORTING.md 0.1-13).
4. **The designated initialisers** of `fpu_reqs` (`[MIPS_ABI_FP_ANY] = { … }`, indices 0-7) are an
   array literal in C's order.
5. **`ELIBBAD`** is Linux's errno and is *not* defined by this host's `<errno.h>`, so it is declared
   in the unit with the target's value (80) and marked `UNVERIFIED`; `ENOEXEC` comes from the
   `<errno.h>` mirror (measured 8).  The reference's own typo at C 241 (`abiflags1` tested for
   `NULL`, then `abiflags->fp_abi` read) is carried bug for bug, with the comment in place.

## Owed

* `exec/include/mipsfpu.rs` declared `cpu_supports_fr0_p`/`determine_fpu_mode` at the
  mirror's top level while this unit defines them: they now sit in a glob-imported
  `mipsfpu_decls` module (PORTING.md 0.1-10, measured here - the two `E0428`s are gone).
* No runtime evidence of any kind: the object was never linked and never run (see
  `docs/status.md` 3.446).
