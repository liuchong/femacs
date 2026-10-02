# emacs/src/category.c (506 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 13
- [x] make-category-set  (Fmake_category_set, 1/1, doc d600fe9d3041, line 72)
- [x] define-category  (Fdefine_category, 2/3, doc 3a551156c741, line 105)
- [x] category-docstring  (Fcategory_docstring, 1/2, doc e9c60897ca8a, line 126)
- [x] get-unused-category  (Fget_unused_category, 0/1, doc bca44ecdc21b, line 138)
- [x] category-table-p  (Fcategory_table_p, 1/1, doc 7b9978e34461, line 160)
- [x] category-table  (Fcategory_table, 0/0, doc 7a5666637af2, line 184)
- [x] standard-category-table  (Fstandard_category_table, 0/0, doc 7ff825e0fb0d, line 192)
- [x] copy-category-table  (Fcopy_category_table, 0/1, doc 8fa76c2df6e4, line 232)
- [x] make-category-table  (Fmake_category_table, 0/0, doc 1ce8376f91e6, line 246)
- [x] set-category-table  (Fset_category_table, 1/1, doc d57426056948, line 262)
- [x] char-category-set  (Fchar_category_set, 1/1, doc 70c199430b0d, line 283)
- [x] category-set-mnemonics  (Fcategory_set_mnemonics, 1/1, doc ef56b571deaf, line 292)
- [x] modify-category-entry  (Fmodify_category_entry, 2/4, doc fd1ce60297ed, line 314)

## Functions defined — 10
- [x] bset_category_table  (line 40, static)
- [x] hash_get_category_set  (line 49, static)
- [x] set_category_set  (line 67, static)
- [x] check_category_table  (line 176, static)
- [x] copy_category_entry  (line 203, static)
- [x] copy_category_table  (line 218, static)
- [x] char_category_set  (line 278)
- [x] word_boundary_p  (line 377)
- [x] init_category_once  (line 422)
- [x] syms_of_category  (line 436)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`Vstandard_category_table` is written as its expansion.**  category.h 103 defines it as
  `BVAR (&buffer_defaults, category_table)`, i.e. a macro that pastes the field name; the
  port writes `BVAR! (&raw mut buffer_defaults, category_table_)` at each of the five use
  sites, and the three bodies that then do not spell the macro's name carry a
  `macro-body-notes` row (the name itself is in `docs/ports/excluded-macros.txt`).
- **`x->extras[i]` is `FAM_ELEM! ((*XCHAR_TABLE (x)).extras, i)`.**  `extras` is the
  flexible array member, which the mirror declares as `[Lisp_Object; 0]`, so a Rust index
  would be an out-of-bounds read (the compiler rejects it outright); the port's `FAM_ELEM!`
  is the addressing idiom `set_char_table_extras` already used.
- **Five `DEFUN!` bodies re-bind their `table` parameter.**  C rewrites it
  (`table = check_category_table (table)`), and `DEFUN!`'s pattern takes plain idents, so
  the re-binds are at the body's top level (the recorded `let mut X = X;` convention).
- **`copy_category_entry` is an `unsafe extern "C" fn`** because `map_char_table` takes a C
  function pointer (the casetab callbacks' shape).
- **`ASET` is a function in this mirror, not a macro**, so `SET_CATEGORY_DOCSTRING!`'s body
  calls it directly; the macro stays a macro because C's form is an assignment.
- **Types the file crosses on:** `CHAR_TABLE_REF` takes `isize`, `CATEGORY_DOCSTRING`
  `c_int`, `set_char_table_contents` `isize`, `SREF` `usize`, `hash_*` `hash_idx_t`
  (lisp.h 2559's `int_least32_t`, not ptrdiff_t - review A's F1), so C's implicit
  conversions are written out.
- **The three cross-unit calls it needs were added to lisp.rs** (`make_hash_table`,
  `hash_find_get_hash`, `copy_char_table`, `char_table_set_range`, all lisp.h).
