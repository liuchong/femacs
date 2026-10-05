# emacs/lib-src/asset-directory-tool.c (338 lines)

> **Not compiled by any configuration the port builds.**  The `lib-src` Makefile's program lists
> (`INSTALLABLES`, `UTILITIES`, `DONT_INSTALL`) do not name this program - it is built only in the
> Android configuration - and `docs/baseline/frozen-build-sources.txt` 92 records that derivation.
> PORTING.md P-86 governs the class: the body is **transcribed as it stands** (the previous revision
> was the P-08 note), the file gate is carried down to the include edges (P-86 ⑧), and P-86 ②'s C
> line-range table is below.  Port: `femacs/lib-src/asset-directory-tool.rs`, first line
> `// femacs: port of lib-src/asset-directory-tool.c @ fd7afced403.`

# ----------------------------------------------------------------------
## Gate mapping

`python3 tools/check-cond-eval.py lib-src/asset-directory-tool.c` reads `1 conditions, 0 unknown,
0 disagreements [VERIFIED]`: the file's only condition is C 231's `#ifdef WORDS_BIGENDIAN`.

| C | condition | arm taken here | feature |
| --- | --- | --- | --- |
| 231 | `#ifdef WORDS_BIGENDIAN` | false (`words_bigendian` is not defined in the frozen configuration) | `words_bigendian` (the positive arm) |

The file gate itself is external - the program's Makefile arm is the Android one - so the port carries
`have_android` on the include edges and on every item, which is what keeps the frozen translation unit
empty (measured: a 336-byte object).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | licence and ownership block | dropped (0.1-19) |
| 20-32 | the include block: `config.h`, `<stdio.h>`, `<assert.h>`, `<fcntl.h>`, `<errno.h>`, `<byteswap.h>`, `<stdlib.h>`, `<dirent.h>`, `<string.h>`, `<unistd.h>`, `<sys/stat.h>` | `have_android` |
| 33-59 | the program's banner and the "Such a file records the layout of the `assets' directory …" commentary | — |
| 60-78 | `struct directory_tree` | `have_android` |
| 79-82 | "Whether the size of each entry should be prepended to the start pointer." and `static bool need_file_size;` | `have_android` |
| 83-95 | `static _Noreturn void croak (const char *what)` | `have_android` |
| 96-112 | `static void *xmalloc (size_t size)` | `have_android` |
| 113-216 | `static void main_1 (DIR *dir, struct directory_tree *parent)` | `have_android` |
| 217-272 | `static void main_2 (int fd, struct directory_tree *tree, size_t *offset)` | `have_android` / `words_bigendian` |
| 273-338 | `int main (int argc, char **argv)` | `have_android` |

## Evidence

* Frozen: `cargo run -q -p xtask -- build-tu lib-src/asset-directory-tool.c` → `built
  lib-src/asset-directory-tool.rs -> ../build/femacs/asset-directory-tool.o`, **exit 0**, object
  **336 bytes** (the empty object the file gate must produce).
* Flipped arm (P-86 ③ ⑴, this host has the Android NDK): `probe lib-src/asset-directory-tool.rs …
  --cfg 'feature="have_android"'` → **exit 0, 0 errors, 4,544-byte object**, its 17 warnings all from
  included mirrors; the arm reading started at 254 errors (a preamble that did not carry the
  `c_*` family), went to 16, then 5, and 0 after the three fixes below.
* `python3 tools/extract-abi.py --check emacs/lib-src/asset-directory-tool.c
  femacs/lib-src/asset-directory-tool.rs` → one token row (`function main_2 does not mention
  static_assert`, exempted in `macro-body-notes.tsv`) plus the line drift declared in
  `line-drift-tolerance.tsv`.
* `python3 tools/check-cond-eval.py lib-src/asset-directory-tool.c` → `1 conditions, 0 unknown,
  0 disagreements [VERIFIED]`.

## Deviations

1. **`struct dirent` and `size_t` are use-site declarations**: the dirent mirror carries the opaque
   `DIR` and the five calls but not the record (`dirent.in.rs`'s own note says it is declared at the
   use site), so the port declares it here in the `src/dired.rs` 72-87 shape, whose clang probe
   measured `sizeof = 1048`, `d_type@20`, `d_name@21`; `size_t` is written out the
   `lib-src/emacsclient.rs` 68 way (both are P-88 ①).
2. **`abort` and `exit` are use-site declarations** - no mirror carries them yet (the same gap the
   `lib-src/emacsclient.rs` 292-295 and `src/sfnt.rs` notes record).
3. **C's `static_assert`** (C 239) is the port's `const _: () = core::assert! (…)`; the macro name has
   no Rust site (the `macro-body-notes.tsv` row).
4. **C's uninitialised locals** in `main_1`/`main_2`/`main` are written with C's own zero values
   (PORTING.md 0.1-13), and `readdir`'s `void *` is taken back to `struct dirent *` because the mirror
   declares the call the way the reference's header does.
5. **The variadic `open`** receives C's promoted `c_int` for `mode_t` (16-bit here), which is what C's
   default argument promotion does at that call (PORTING.md 2.4).
6. **C's `while ((dirent = readdir (dir)))`** and the `for (child = …; child; child = child->next)`
   loops are the port's `loop`/`while !….is_null ()` with the same assignment order.

## Owed

* No runtime evidence of any kind: neither object was ever run, and the directory-tree format itself
  was never produced or read (see `docs/status.md` 3.450).
* The arm's 17 warnings are all in the included mirrors (`unistd.in.rs`'s runtime-`read`/`write` lint
  and the `unused doc comment` pair), which the mirror round owns.

## 2026-10-05: the message's bug address is this repository's own (status 3.613)

The `fprintf` that reports a bad offset sends the user to a bug address, so the literal now spells
this repository's issue page (PORTING.md 0.1-19's ninth half).  The rest of the message, its
arguments and the `abort ()` are C's; the frozen configuration's measured address stays in
`docs/baseline/config/`.
