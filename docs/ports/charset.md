# emacs/src/charset.c (2464 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 23
- [x] charsetp  (Fcharsetp, 1/1, doc c1ed7c90f557, line 642)
- [x] map-charset-chars  (Fmap_charset_chars, 2/5, doc d27fb03a4ee5, line 787)
- [x] define-charset-internal  (Fdefine_charset_internal, charset_arg_max/MANY, doc 2e2d8e5b52ad, line 838)
- [x] define-charset-alias  (Fdefine_charset_alias, 2/2, doc 37a77a00f254, line 1281)
- [x] charset-plist  (Fcharset_plist, 1/1, doc 511fced6848a, line 1295)
- [x] set-charset-plist  (Fset_charset_plist, 2/2, doc 5656ad81baaf, line 1306)
- [x] unify-charset  (Funify_charset, 1/3, doc 1e801c21b93e, line 1318)
- [x] get-unused-iso-final-char  (Fget_unused_iso_final_char, 2/2, doc ec869e4ba7d0, line 1399)
- [x] declare-equiv-charset  (Fdeclare_equiv_charset, 4/4, doc b5b08a69646e, line 1420)
- [x] find-charset-region  (Ffind_charset_region, 2/3, doc 35f262aa3a91, line 1517)
- [x] find-charset-string  (Ffind_charset_string, 1/2, doc 21ab9e6b2c14, line 1569)
- [x] decode-char  (Fdecode_char, 2/2, doc 6d6f37aa6748, line 1828)
- [x] encode-char  (Fencode_char, 2/2, doc d913f93f163c, line 1848)
- [x] make-char  (Fmake_char, 1/5, doc 34581bc70a56, line 1876)
- [x] split-char  (Fsplit_char, 1/1, doc a9da8161eac3, line 1993)
- [x] char-charset  (Fchar_charset, 1/2, doc 58493dbc4ebd, line 2024)
- [x] charset-after  (Fcharset_after, 0/1, doc b1445c24343e, line 2063)
- [x] iso-charset  (Fiso_charset, 3/3, doc c24e7b4abb4b, line 2081)
- [x] clear-charset-maps  (Fclear_charset_maps, 0/0, doc 903018ae3471, line 2124)
- [x] charset-priority-list  (Fcharset_priority_list, 0/1, doc d4674c30efac, line 2146)
- [x] set-charset-priority  (Fset_charset_priority, 1/MANY, doc 3ac6ad140695, line 2165)
- [x] charset-id-internal  (Fcharset_id_internal, 0/1, doc e613ccefa4a2, line 2218)
- [x] sort-charsets  (Fsort_charsets, 1/1, doc d33d82b8badb, line 2246)

## Functions defined — 20
- [x] load_charset_map  (line 239, static)
- [x] read_hex  (line 414, static)
- [x] load_charset_map_from_file  (line 472, static)
- [x] load_charset_map_from_vector  (line 551, static)
- [x] load_charset  (line 617, static)
- [x] map_charset_for_dump  (line 651, static)
- [x] map_charset_chars  (line 704)
- [x] define_charset_internal  (line 1225, static)
- [x] check_iso_charset_parameter  (line 1377, static)
- [x] string_xstring_p  (line 1452)
- [x] find_charsets_in_text  (line 1479, static)
- [x] maybe_unify_char  (line 1597, static)
- [x] decode_char  (line 1639)
- [x] encode_char  (line 1722)
- [x] char_charset  (line 1961)
- [x] shrink_charset_table  (line 2100, static)
- [x] charset_compare  (line 2238, static)
- [x] init_charset  (line 2295)
- [x] init_charset_once  (line 2323)
- [x] syms_of_charset  (line 2359)

## Macros defined — 7
- [x] CHARSET_TABLE_INIT_SIZE  (line 2356, body 3a2dc677d8e8)
- [x] CODE_POINT_TO_INDEX  (line 105, body 37bb939bf3f2)
- [x] GET_TEMP_CHARSET_WORK_DECODER  (line 196, body 6d62df86dd17)
- [x] GET_TEMP_CHARSET_WORK_ENCODER  (line 186, body b1cde7806350)
- [x] INDEX_TO_CODE_POINT  (line 127, body 85fcd6a60f95)
- [x] SET_TEMP_CHARSET_WORK_DECODER  (line 193, body e3dbc5a7b3f7)
- [x] SET_TEMP_CHARSET_WORK_ENCODER  (line 176, body 67c751ee41ae)

## Types and their fields — 3
- [x] <anon@162>: int decoder, unsigned short encoder
- [x] charset_map_entries: unsigned from, unsigned to, int c, <aggregate> entry, struct charset_map_entries* next
- [x] charset_sort_data: Lisp_Object charset, int id, ptrdiff_t priority

## File-scope objects — 21
- [x] Lisp_Object Vcharset_hash_table  (line 61)
- [x] struct charset_table charset_table  (line 64)
- [x] int charset_ascii  (line 67)
- [x] int charset_eight_bit  (line 68)
- [x] static int charset_iso_8859_1  (line 69, static)
- [x] int charset_unicode  (line 70)
- [x] static int charset_emacs  (line 71, static)
- [x] int charset_jisx0201_roman  (line 74)
- [x] int charset_jisx0208_1978  (line 75)
- [x] int charset_jisx0208  (line 76)
- [x] int charset_ksc5601  (line 77)
- [x] int charset_unibyte  (line 80)
- [x] Lisp_Object Vcharset_ordered_list  (line 83)
- [x] Lisp_Object Vcharset_non_preferred_head  (line 87)
- [x] EMACS_UINT charset_ordered_list_tick  (line 91)
- [x] Lisp_Object Viso_2022_charset_list  (line 94)
- [x] Lisp_Object Vemacs_mule_charset_list  (line 97)
- [x] int emacs_mule_charset  (line 99)
- [x] int iso_charset_table  (line 103)
- [x] bool charset_map_loaded  (line 202)
- [x] Lisp_Object charset_work  (line 1715)

## Decisions

- **What the file carries.** C's 23 `DEFUN`s, its 20 functions (8 non-`static`:
  `string_xstring_p`, `map_charset_chars`, `decode_char`, `encode_char`, `char_charset`,
  `init_charset`, `init_charset_once`, `syms_of_charset`), the 7 file-local macros
  (`CODE_POINT_TO_INDEX`, `INDEX_TO_CODE_POINT`, the four
  `SET_`/`GET_TEMP_CHARSET_WORK_*`, `CHARSET_TABLE_INIT_SIZE` — the last one is a `pub
  const` here, because `tools/extract-abi.py` counts it as a *constant* of value 190), the
  three types, the 21 file-scope objects and the four `DEFVAR_*` variables this unit owns.
  The 8 non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the 12
  `static` helpers stay plain `unsafe fn`.
- **`charset.h`'s mirror landed with this unit** (`femacs/src/include/charset.rs`, the
  composite.rs/intervals.rs precedent): the three enums, both structs, the 63 macros, the
  two INLINE functions, all nineteen `extern` objects and every prototype.  This unit
  defines five of the header's prototypes, so that block moved into a `charset_decls`
  module + glob import (PORTING.md 0.1-10) and the nineteen objects into `charset_objs`
  (P-21); the unit's own definitions shadow both globs.  Three units had declared the
  names they take from this header at their use sites (chartab.rs, ccl.rs); their
  declarations keep shadowing the globs, and neither file was touched.  `build-tu` was
  re-run for `src/ccl.c` and `src/chartab.c` to confirm it.
- **Reported mirror gaps (declared at the use site, not by editing the mirrors).**
  `lisp.rs` has no `openp` (lisp.h 4743), `emacs_fdopen` (5320), `emacs_fclose` (5316),
  `set_unwind_protect_ptr` (4832), `fclose_unwind` (4977), `record_unwind_protect_nothing`
  (4827), `clear_unwind_protect` (4829), `add_to_log` (4378), `nconc2` (4271),
  `file_accessible_directory_p` (4988) or `map_char_table_for_charset` (4692);
  `coding.rs` has no `coding_system_charset_list` (coding.h 688); `<stdio.h>`'s mirror has
  no `getc`/`fprintf`/`stderr`; `<stdlib.h>`'s has no `exit`/`qsort`; `<string.h>`'s has no
  `strerror`.  All fourteen are declared in the unit's own `unsafe extern "C"` block with
  C's prototype (`/// C:` line) and are listed here for the mirror owners.
- **Dead arms recorded, not transcribed (P-08).** `src/charset.c` is in
  `docs/baseline/config/cond-eval-verified.txt`, so the frozen configuration's branch
  selection is clang's.  The file's only `#if`s are the two `#ifdef REL_ALLOC` arms in
  `maybe_unify_char` (C 1607-1612, 1628-1630: the two
  `r_alloc_inhibit_buffer_relocation` calls); `REL_ALLOC` is a source-level macro the
  frozen table does not know and `check-cond-eval.py` reports `0 unknown` /
  `0 disagreements`, so both arms are dead and neither is carried.  There is no `#if 0`
  and no other configuration arm in the file.
- **The one structural deviation.** `struct charset_map_entries`'s `entry` member is an
  array of a C *anonymous* struct, and `tools/extract-abi.py` attributes that struct's
  members (`from`, `to`, `c`) to the outer container; a Rust array field cannot recurse
  into its element type, so the port declares the element as `charset_map_entry` **and**
  three zero-sized `[u8; 0]` markers named `from`/`to`/`c` at the end of the outer struct.
  They add no storage and move no field: `const _` assertions pin `entry` at 0, `next` at
  0xC0000 and `sizeof` at 0xC0008.  The code (`(*entries).entry[idx].from`) is unchanged.
- **The six `bool_bf : 1` members** of `struct charset` are read and written through the
  mirror's `charset_bits!` macro, so every C read and write keeps its C field name; the six
  occurrence-checked families are in `docs/ports/token-aliases.tsv`.  **The lvalue uses of
  the header's accessor macros** (`CHARSET_MIN_CHAR (charset) = …`,
  `CHARSET_MAX_CHAR (charset) = …`) are written `(*charset).min_char = …`, and the
  `UNINIT` locals of `load_charset_map` and the `bool maybe_null = 0/1` of
  `char_charset` are recorded in `docs/ports/macro-body-notes.tsv` (5 rows for this unit).
- **Fourteen use-site declarations** (the list above) and **one local macro**:
  `<intprops.h>` has no mirror (P-26), so `INT_LEFT_SHIFT_OVERFLOW` is written at the use
  site with intprops.h 210-213's unsigned branch (`UINT_MAX >> (b) < (a)`), the
  canonicalize-lgpl.rs shape.
- **Uninitialised locals (PORTING.md 1.5):** C 239-240's `Lisp_Object vec UNINIT;` /
  `table UNINIT;` are `= Qnil ()` (recorded in macro-body-notes.tsv), and every
  declared-then-assigned local of `define_charset_internal`, `load_charset_map*`,
  `find-charsets-in-text`'s callers and `sort-charsets` starts at 0/`Qnil ()`.
  `struct charset charset = {0}` (0.1-13) is the seventeen-field zero value.
- **Line count:** C 2464 vs the port's 2791 (13.3%), declared in
  `docs/ports/line-drift-tolerance.tsv` with the `total` basis (the port carries more text
  than C, and no dead arm is carried); the per-function token comparison is the real guard
  and it is OK.
