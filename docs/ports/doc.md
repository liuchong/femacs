# emacs/src/doc.c (746 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 6
- [x] documentation-stringp  (Fdocumentation_stringp, 1/1, doc 4cdfc3fe7ad4, line 319)
- [x] documentation  (Fdocumentation, 1/2, doc 55746fca8a3e, line 333)
- [x] internal-subr-documentation  (Fsubr_documentation, 1/1, doc 00770d4b8e65, line 383)
- [x] documentation-property  (Fdocumentation_property, 2/3, doc fd1120b74fa9, line 402)
- [x] Snarf-documentation  (Fsnarf_documentation, 1/1, doc fed5ad013a29, line 500)
- [x] text-quoting-style  (Ftext_quoting_style, 0/0, doc 22a0fc35c467, line 652)

## Functions defined — 5
- [x] get_doc_string  (line 107)
- [x] reread_doc_file  (line 311, static)
- [x] store_function_docstring  (line 459, static)
- [x] default_to_grave_quoting_style  (line 640, static)
- [x] syms_of_doc  (line 682)

## Macros defined — 5
- [x] doc_fd  (line 44, body 46f8ab7c0cff)
- [x] doc_fd_p  (line 45, body 3e4ff0024176)
- [x] doc_lseek  (line 48, body 0d5930e232f1)
- [x] doc_open  (line 46, body a2e752273417)
- [x] doc_read_quit  (line 47, body 6f0f5a8a7f7f)

## Types and their fields — 0

## File-scope objects — 2
- [x] static char get_doc_string_buffer  (line 74, static)
- [x] static ptrdiff_t get_doc_string_buffer_size  (line 75, static)

## Decisions

- **The five `doc_fd` macros (C 42-49)** are `macro_rules!` with C's names
  (`doc_fd`/`doc_fd_p`/`doc_open`/`doc_read_quit`/`doc_lseek`), because `HAVE_ANDROID` is
  undefined in the frozen configuration and C's first arm is the live one.  `doc_fd` is a
  *type* macro, so the one declaration it heads reads `let fd: doc_fd! () = doc_open! (…)`.
- **C 51-70's `#else` arm (the `struct android_fd_or_asset` set) and the three
  `USE_ANDROID_ASSETS` sites (C 148-153, 553-558 and `close_file_unwind_android_fd`) are
  gated text** under `#[cfg (feature = "android_assets")]` (P-51, the status §3.9 convention
  of naming the feature after the non-frozen side).  Nothing compiles them: the feature is
  not in `xtask dead-arms`'s FLIPS, and android.h has no mirror.
- **`#include "buildobj.h"` (C 520)** is a *generated* header, so its mirror is generated
  too: `femacs/src/include/buildobj.rs` is written by `tools/gen-buildobj.py` from the frozen
  build's `build/emacs-ref/src/buildobj.h` (which `make -C build/emacs-ref/src buildobj.h`
  produces from `$(ALLOBJS)`, `src/Makefile.in` 673-678; 75 objects).  The array name stays C's
  `buildobj`, its elements are `&CStr` so `buildobj[i].as_ptr ()` is C's `char const *`, and
  `countof!` is `lib/include/stdcountof.rs`'s macro for C's `countof`.
- **`while (true)` (C 197, 559)** is the port's `loop` and C's `goto retry` (C 350, 434) is
  `'retry: loop` with `continue 'retry`; `docs/ports/macro-body-notes.tsv` records both
  (`true` and `goto`) with PORTING.md 2.2.
- **`error ("\` + newline (C 297)**: the multi-line string literal is written *the same way*
  in Rust, because both languages remove exactly the newline (Rust also removes the next
  line's indentation, and this message starts at column 0).  That keeps the emitted bytes
  identical and, as a bonus, keeps the checker's string scanner in the same state on both
  sides (it cannot see across a backslash-newline inside a literal).
- **`TYPE_MAXIMUM (off_t)` (C 185)** is `off_t::MAX` (the frozen `off_t` is `i64`,
  `lib/include/sys_types.in.rs` 7): gnulib's intprops.h has no mirror (P-26), the same class
  the `Fash` row records; `037` (C 295) is `31`, Rust having no octal literal (the `Ctl` row).
- **C's `int offset = min (position, max (1024, position % (8 * 1024)))` (C 184)** keeps its
  narrowing: the port's `min`/`max` are `(isize, isize)` and the result is cast to `c_int`.
- **`close_file_unwind` / `record_unwind_protect_int`** are declared in lisp.rs's
  `other_units` group in the same change (lisp.h 4823, 4976), as are `concat3` (4269) and
  `emacs_read`/`emacs_read_quit` (5332-5333); `<string.h>`'s `stpcpy` (the frozen
  `HAVE_STPCPY=1`, so the system function, not `rpl_stpcpy`) was added to
  `lib/include/string.rs`.
- **`#ifdef HAVE_NATIVE_COMP` / `#ifdef HAVE_MODULES` (C 387-397)**: both are off in the
  frozen configuration, so the live body is C's `if (SUBRP (function)) … else return Qt;`.
  The native arm is gated as `#[cfg (feature = "native_comp")]` — *that* feature **is** in
  `xtask dead-arms`'s FLIPS, so the arm compiles there, and it declares
  `native_function_doc` (comp.h 78, no mirror carries it) inside itself, as the vm-limit dead
  arms declare their platform entries.  The modules arm is gated as
  `#[cfg (feature = "modules")]` (never compiled) and declares `MODULE_FUNCTIONP`,
  `XMODULE_FUNCTION` and `module_function_documentation` the same way.  Because the live
  `if/else` must stay one chain, the two gated arms precede it and each returns.
- **`intervals.h`**: doc.c includes it but calls nothing it declares (it only parses the DOC
  file's bytes), so the port carries no mirror for it; `<sys/file.h>` likewise has no mirror,
  its names (`O_RDONLY`, `lseek`) coming from the fcntl/unistd mirrors.
- **Docstrings**: the six `doc:` literals carry make-docfile's value; the `text-quoting-style`
  doc keeps C's `\\=` escapes decoded to `\=`, and the `‘…’` glyphs as the same bytes.
- **Call-relationship differences (D7)**: none.  Every callee is defined here or already
  declared (`Fsnarf_documentation` is this unit's own subr, called by `reread_doc_file` the way
  C calls it).
- Evidence: `xtask build-tu src/doc.c` -> `built src/doc.rs -> ../build/femacs/doc.o`;
  `tools/extract-abi.py --check emacs/src/doc.c femacs/src/doc.rs` -> OK.
