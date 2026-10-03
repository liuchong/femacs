# emacs/src/chartab.c (1391 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 13
- [x] make-char-table  (Fmake_char_table, 1/2, doc 4f6ce4501f87, line 104)
- [x] char-table-subtype  (Fchar_table_subtype, 1/1, doc a3570d98b5b8, line 499)
- [x] char-table-parent  (Fchar_table_parent, 1/1, doc ec18a3dbbb90, line 510)
- [x] set-char-table-parent  (Fset_char_table_parent, 2/2, doc b2f66da24a80, line 524)
- [x] char-table-extra-slot  (Fchar_table_extra_slot, 2/2, doc ba6952226506, line 548)
- [x] set-char-table-extra-slot  (Fset_char_table_extra_slot, 3/3, doc 1404c78e518c, line 562)
- [x] char-table-range  (Fchar_table_range, 2/2, doc c446958a409d, line 578)
- [x] set-char-table-range  (Fset_char_table_range, 3/3, doc 88dd6a7696fc, line 609)
- [x] optimize-char-table  (Foptimize_char_table, 1/2, doc 875090ab8e6b, line 676)
- [x] map-char-table  (Fmap_char_table, 2/2, doc 67b81139fece, line 903)
- [x] unicode-property-table-internal  (Funicode_property_table_internal, 1/1, doc 5ca60c4f77fc, line 1303)
- [x] get-unicode-property-internal  (Fget_unicode_property_internal, 2/2, doc e216c1f26029, line 1326)
- [x] put-unicode-property-internal  (Fput_unicode_property_internal, 3/3, doc f4fbd4ce1210, line 1339)

## Functions defined — 31
- [x] CHECK_CHAR_TABLE  (line 88, static)
- [x] set_char_table_ascii  (line 94, static)
- [x] set_char_table_parent  (line 99, static)
- [x] make_sub_char_table  (line 139, static)
- [x] char_table_ascii  (line 150, static)
- [x] copy_sub_char_table  (line 167, static)
- [x] copy_char_table  (line 187)
- [x] sub_char_table_ref  (line 211, static)
- [x] char_table_ref  (line 226)
- [x] char_table_ref_simple  (line 253, static)
- [x] sub_char_table_ref_and_range  (line 270, static)
- [x] char_table_ref_and_range  (line 321)
- [x] sub_char_table_set  (line 371, static)
- [x] char_table_set  (line 400)
- [x] sub_char_table_set_range  (line 425, static)
- [x] char_table_set_range  (line 463)
- [x] optimize_sub_char_table  (line 644, static)
- [x] map_sub_char_table  (line 716, static)
- [x] map_char_table  (line 842)
- [x] map_sub_char_table_for_charset  (line 920, static)
- [x] map_char_table_for_charset  (line 1003)
- [x] uniprop_table_uncompress  (line 1091, static)
- [x] uniprop_decode_value_run_length  (line 1152, static)
- [x] uniprop_get_decoder  (line 1172, static)
- [x] uniprop_encode_value_character  (line 1189, static)
- [x] uniprop_encode_value_run_length  (line 1201, static)
- [x] uniprop_encode_value_numeric  (line 1219, static)
- [x] uniprop_get_encoder  (line 1247, static)
- [x] uniprop_table  (line 1264)
- [x] get_unicode_property  (line 1319)
- [x] syms_of_chartab  (line 1360)

## Macros defined — 4
- [x] CHARTAB_IDX  (line 53, body 15299ad99fff)
- [x] UNIPROP_COMPRESSED_FORM_P  (line 83, body de355ec71cce)
- [x] UNIPROP_GET_DECODER  (line 76, body 1996f98bc199)
- [x] UNIPROP_TABLE_P  (line 71, body f22ba75ffab6)

## Types and their fields — 0

## File-scope objects — 2
- [x] static const int uniprop_decoder_count  (line 1167, static)
- [x] static const int uniprop_encoder_count  (line 1242, static)

## Decisions

`emacs/src/chartab.c` @ `fd7afced403`; port `femacs/src/chartab.rs` (1,704 lines vs C's 1391,
of which **1387** are live - the 22.9% drift is declared in
`docs/ports/line-drift-tolerance.tsv` with the `live` basis and is explained under "Line
count" below).  The gates are
`python3 tools/extract-abi.py --check emacs/src/chartab.c femacs/src/chartab.rs` -> `OK`,
`build-tu src/chartab.c` -> `built` and
`python3 tools/check-symbols.py --pending docs/baseline/symbol-pending.txt` ->
`0 mismatched` (this unit is under `src/`, which that gate does not compare - it walks
`lib/`; the row is quoted because the round's acceptance list names it).

C's ten non-`static` definitions (`copy_char_table`, `char_table_ref`, `char_table_set`,
`char_table_ref_and_range`, `char_table_set_range`, `map_char_table`,
`map_char_table_for_charset`, `uniprop_table`, `get_unicode_property`, `syms_of_chartab`)
and its one non-`static` object (`chartab_size`) carry `#[unsafe(no_mangle)]` (PORTING.md
0.2).  The twenty-one `static` helpers are `unsafe fn` (every one of them reads or writes a
Lisp object through a pointer or a union - the "unsafe where C needs a pointer" rule), except
the four uniprop callbacks, which are `unsafe extern "C" fn` because C stores their addresses
in the `uniprop_decoder`/`uniprop_encoder` tables (PORTING.md 1.4).  The two `DEFVAR_*`-style
objects (`Vchar_code_property_alist`) and the thirteen subr records are this unit's own
definitions, shadowing the `globals.rs` declarations (P-21).

### Dead text (not transcribed)

`src/chartab.c` has exactly one preprocessor condition, and it is dead:

- C 1139-1144: `uniprop_table_uncompress`'s `#if 0` arm, the `WORD-LIST TABLE` case C's own
  comment above it says the file does not need.  It is recorded here, not carried (P-08);
  the port keeps the `else if (*p == 2)` arm and, next to where the arm sat, the comment C
  wrote above the `#if`.

`python3 tools/check-cond-eval.py src/chartab.c` reports
`1 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and `src/chartab.c` is line 253 of
`docs/baseline/config/cond-eval-verified.txt`, so the extractor's branch selection is the
clang-verified one and no `docs/baseline/config/source-header-macros.tsv` row is needed.

### Deviations recorded

1. **C's three forward declarations are deleted** (C 64-68: `uniprop_table_uncompress`,
   `uniprop_get_decoder`, `sub_char_table_ref_and_range`); the definitions keep their C place
   (PORTING.md 0.1-12).  Rust items are order-independent, and `UNIPROP_GET_DECODER`'s body
   names `uniprop_get_decoder` only at its call sites.
2. **`charset.h` has no mirror yet** (`docs/port-map.md` still lists
   `femacs/src/include/charset.rs` as unstarted), so the names this unit's live text takes
   from it are written at their use site, with C's own bodies and comments (PORTING.md
   0.1-2/P-72, the terminal.rs/filelock.rs shape).  That block is 206 of the port's lines and
   contains: `enum charset_method` and the two `enum charset_attr_index` members the code
   reaches (`charset_id` = 0, `charset_encoder` = 5); `struct charset` with its six
   `bool_bf` members packed into one byte named `bits` (PORTING.md 1.3) plus the six
   `CHARSET_*` masks; `struct charset_table`; the `charset_table`, `charset_work`,
   `encode_char` and `coding_inherit_eol_type` declarations (the last one because
   `coding.rs` is a WIP mirror that does not carry it yet - `coding.h` itself is included);
   the header inline `charset_attributes_getter`; the four accessor macros
   `CHARSET_ATTR_ID`, `CHARSET_ATTR_ENCODER`, `CHARSET_ATTRIBUTES` and `CHARSET_ENCODER`; and
   `ENCODE_CHAR`, which is the only reason any of it is needed.  **It moves into the mirror
   when charset.c lands.**  The layout was measured, not derived - a standalone clang
   translation of the header's declaration gives `sizeof (struct charset)` = 320, align 8,
   `id` 0, `dimension` 4, `code_space` 8, `code_space_mask` 72, the six bits at 80:0-80:5,
   `iso_final` 84, `iso_revision` 88, `emacs_mule_id` 92, `method` 96, `min_code` 100,
   `max_code` 104, `char_index_offset` 108, `min_char` 112, `max_char` 116, `invalid_code`
   120, `fast_map` 124, `code_offset` 316 - and the numbers are in the struct's doc comment.
   `bool_bf` is `bool`, i.e. a one-byte allocation unit, on this target (`conf_post.h`
   56-61), which is what puts the six bits in one byte.
   `ENCODE_CHAR`'s C body is wrapped in `verify_expr (sizeof (c) <= sizeof (int), ...)`, a
   compile-time guard (lib/verify.h 350-351) whose runtime value is just the expression; the
   port keeps C's expression and drops the guard, because the one use site passes a `c_int`
   (which is what the guard asserts) and the `verify.h` mirror is not landed.  Each use of
   `$c` and `$charset` keeps C's evaluation count.
3. **C's `bool` literals become `false`/`true`** in `optimize_sub_char_table`
   (`optimizable = SUB_CHAR_TABLE_P (elt) ? 0 : 1;`, `optimizable = 0;`) and in
   `map_sub_char_table` (`bool different_value = 1;`); the literals are exempted in
   `docs/ports/macro-body-notes.tsv` (the `word_boundary_p`/casetab class).
4. **C's `NULL` becomes `None`** where the value is a function pointer:
   `UNIPROP_GET_DECODER`, `uniprop_get_decoder`, `uniprop_get_encoder` and
   `Fmap_char_table`'s `map_char_table (NULL, ...)`.  The type is
   `Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object) -> Lisp_Object>` (PORTING.md 1.4),
   so the empty value cannot be spelled `NULL`; the four tokens are exempted in
   `docs/ports/macro-body-notes.tsv`.  C's `if (c_function) (*c_function) (...)` calls become
   `if let Some (c_function) = c_function { c_function (...) }` (the terminal.rs shape:
   `.unwrap ()` is not allowed).
5. **C's uninitialized local `int len;`** (C 1127, written by
   `string_char_and_length (p, &len)`) is `let mut len: c_int = 0;` (PORTING.md 1.5).  The
   other "declared then assigned" locals keep C's shape because Rust's definite-initialization
   analysis accepts them: `Lisp_Object val;`, `int i, c;`, `unsigned code;` and friends are
   `let mut` declarations whose first write is C's.
6. **C parameters that the body assigns are re-bound at the top of the body**, because Rust
   parameters are immutable: `c` in `sub_char_table_ref_and_range` and
   `char_table_ref_and_range`, `from` in `sub_char_table_set_range`, `idx` in
   `uniprop_table_uncompress`, `value` in the three encoders/decoders and in
   `map_sub_char_table`'s `val`.  In a `DEFUN!` invocation the parameter list cannot carry
   `mut` either (the macro's pattern is `$arg:ident : $ty:ty`), so
   `Fput_unicode_property_internal` re-declares `value` first (the `category.rs` precedent).
7. **`CHARTAB_SIZE_BITS_*` and `CHAR_TABLE_STANDARD_SLOTS` cross a width boundary**: the
   mirror gives them as `usize`/`isize`, C's are `int`, so `chartab_bits`' three sum elements
   and the `CHAR_TABLE_STANDARD_SLOTS + n_extras` / `PVSIZE (table)` / `XFIXNUM (n)` values
   carry an explicit `as c_int` (or `as EMACS_INT`) cast where C narrowed implicitly.
8. **Pointer and flexible-array idioms** keep the mirror's established spellings: flexible
   members are reached through `FAM_ELEM!` (it works for `Lisp_Char_Table.contents`' *sized*
   array too, as lisp.rs documents), the base of a vector's `contents` for
   `uniprop_encode_value_*`'s `Lisp_Object *` is
   `fam_base (addr_of_mut! ((*XVECTOR (…)).contents))` (the `bool_vector_data` idiom),
   `p++`/`p += len` are `p = p.offset (1)`/`p.offset (len as isize)`, and the
   `&p` argument of `string_char_advance` is `&raw mut p`.
9. **`CHARTAB_IDX`** indexes the port's `chartab_bits` array with `($depth) as usize`, since
   Rust array indexing takes a `usize` and C's `depth` is an `int`; the macro's C body is
   otherwise unchanged.
10. **`countof`** is gnulib's macro, so `uniprop_decoder_count`/`uniprop_encoder_count` keep
    C's derivation (`countof! (uniprop_decoder) as c_int`) rather than a literal `1`/`3`;
    that requires the two tables to be immutable `static`s (a shared reference to a
    `static mut` is not allowed), which is also their C type (neither is ever written).
11. **`chartab_size` is `pub static mut`** because the lisp.rs mirror declares it that way
    (`other_units`) and the definition must have the declaration's type; C's object is
    `const int [4]` with external linkage.

### Macro-body notes

`docs/ports/macro-body-notes.tsv` carries seven rows for this unit: `UNIPROP_GET_DECODER`
and `uniprop_get_decoder`/`uniprop_get_encoder`/`Fmap_char_table` (`null`, deviation 4),
`syms_of_chartab` (`doc`: the positional `DEFVAR_LISP!` docstring has no `doc:` label),
`optimize_sub_char_table` (`0,1`) and `map_sub_char_table` (`1`) for deviation 3.  No
`token-aliases.tsv` family and no `excluded-macros.txt` row was needed: the four macros C
defines in this file (`CHARTAB_IDX`, `UNIPROP_TABLE_P`, `UNIPROP_GET_DECODER`,
`UNIPROP_COMPRESSED_FORM_P`) all keep their names and bodies, and `calln` keeps C's
spelling as `calln!` (`CALLN!` for `Fvconcat`).

### Line count

`docs/ports/line-drift-tolerance.tsv`'s `chartab` row declares the `live` basis with a 0.25
tolerance: C's 1,387 live lines (of 1391; the 4 lines of the `#if 0` arm above are recorded,
not carried) against the port's 1,684 (+21.4%).  206 of the port's lines are the use-site
`charset.h` block of deviation 2; the rest is the unit's fixed costs (the `// femacs:` note,
the `#![allow]` block, the include block) less what the thirteen
docstrings save by collapsing C's comment lines into literals.

### Call-relationship differences (D7)

None.  Every call C makes from this file is to a name this file defines, to a mirror
declaration, or to one of the five names of deviation 2/3-4 written at its use site; the
reverse direction (who calls this file's functions) is unchanged, since all ten non-`static`
definitions keep their C names and signatures - `map_char_table`'s `c_function` parameter is
the `Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object, Lisp_Object)>` the lisp.rs
declaration already carries.

### Changes outside the unit

None to the tree's sources: this round added `femacs/src/chartab.rs`,
`docs/ports/chartab.md`, seven `docs/ports/macro-body-notes.tsv` rows and one
`docs/ports/line-drift-tolerance.tsv` row.  No new mirror was created (the `charset.h`
material is deliberately at the use site; see deviation 2) and no tool was changed.

**Note for the integrator**: at the first attempt `build-tu src/chartab.c` failed inside
`femacs/src/include/thread.rs`, a shared mirror another unit owns that was being rewritten in
the same worktree at the time (its `eassert!`/`XUNTAG!` calls sit before the `include!` that
defines those macros).  The failure was not specific to this unit - `build-tu src/casetab.c`,
green before, printed the same errors - so the `built` line was first taken in an isolated
`git worktree add --detach /tmp/femacs-chartab-check HEAD` with this unit's files copied in
(`built src/chartab.rs -> ../build/femacs/chartab.o`; the reference tree `emacs/` is not
tracked, so `--check` can only run in the shared worktree).  Once the mirror settled, the
same command in the shared worktree printed the `built` line quoted above.
