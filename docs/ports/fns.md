# emacs/src/fns.c (6973 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 104
- [x] identity  (Fidentity, 1/1, doc 36fad1da8837, line 49)
- [x] random  (Frandom, 0/1, doc 0a667d88d480, line 77)
- [x] length  (Flength, 1/1, doc 3cb2462e76b0, line 127)
- [x] safe-length  (Fsafe_length, 1/1, doc 53d941f4c0b2, line 162)
- [x] length<  (Flength_less, 2/2, doc 1e1d1dc28d98, line 196)
- [x] length>  (Flength_greater, 2/2, doc 4df505502012, line 211)
- [x] length=  (Flength_equal, 2/2, doc 757ac8fa39e7, line 226)
- [x] proper-list-p  (Fproper_list_p, 1/1, doc 47223d7511a1, line 244)
- [x] string-bytes  (Fstring_bytes, 1/1, doc 8712900bd3cd, line 263)
- [x] string-distance  (Fstring_distance, 2/3, doc f9baa3d66f70, line 272)
- [x] string-equal  (Fstring_equal, 2/2, doc 4662c12a2019, line 339)
- [x] compare-strings  (Fcompare_strings, 6/7, doc c210579b61eb, line 361)
- [x] string-lessp  (Fstring_lessp, 2/2, doc f3c99a619f5d, line 559)
- [x] string-version-lessp  (Fstring_version_lessp, 2/2, doc 982612f065a7, line 577)
- [x] string-collate-lessp  (Fstring_collate_lessp, 2/4, doc 9332fe85bc69, line 603)
- [x] string-collate-equalp  (Fstring_collate_equalp, 2/4, doc 39f66c07cf85, line 650)
- [x] append  (Fappend, 0/MANY, doc a938ac05beb6, line 716)
- [x] concat  (Fconcat, 0/MANY, doc 1d7af11c22db, line 737)
- [x] vconcat  (Fvconcat, 0/MANY, doc f5aa3fb86978, line 750)
- [x] copy-sequence  (Fcopy_sequence, 1/1, doc f0b49e20f8db, line 761)
- [x] string-make-multibyte  (Fstring_make_multibyte, 1/1, doc fb244227dafd, line 1365)
- [x] string-make-unibyte  (Fstring_make_unibyte, 1/1, doc 0e264d561172, line 1393)
- [x] string-as-unibyte  (Fstring_as_unibyte, 1/1, doc e1f16c84f76b, line 1405)
- [x] string-as-multibyte  (Fstring_as_multibyte, 1/1, doc a78f5b063612, line 1427)
- [x] string-to-multibyte  (Fstring_to_multibyte, 1/1, doc 4d90bc2f1730, line 1465)
- [x] string-to-unibyte  (Fstring_to_unibyte, 1/1, doc 75f1e5750125, line 1484)
- [x] copy-alist  (Fcopy_alist, 1/1, doc 2b14462f4557, line 1516)
- [x] substring  (Fsubstring, 1/3, doc 8d02188a4ecc, line 1579)
- [x] substring-no-properties  (Fsubstring_no_properties, 1/3, doc bf61bb755a20, line 1619)
- [x] take  (Ftake, 2/2, doc bed3bbb93d25, line 1671)
- [x] ntake  (Fntake, 2/2, doc ee145176fbfd, line 1712)
- [x] nthcdr  (Fnthcdr, 2/2, doc a640a721293b, line 1749)
- [x] nth  (Fnth, 2/2, doc b79eee104043, line 1845)
- [x] elt  (Felt, 2/2, doc d18edffe886a, line 1853)
- [x] member  (Fmember, 2/2, doc 5f79ea42b139, line 1896)
- [x] memq  (Fmemq, 2/2, doc d855d5d49b3d, line 1911)
- [x] memql  (Fmemql, 2/2, doc 9ca694168e0c, line 1933)
- [x] assq  (Fassq, 2/2, doc a0c64028015e, line 1966)
- [x] assoc  (Fassoc, 2/3, doc 4789c549c643, line 2006)
- [x] rassq  (Frassq, 2/2, doc 727ef6b15bc6, line 2049)
- [x] rassoc  (Frassoc, 2/2, doc ed286908e780, line 2062)
- [x] delq  (Fdelq, 2/2, doc 6bc97d8e6d70, line 2081)
- [x] delete  (Fdelete, 2/2, doc 05698e5c5b37, line 2137)
- [x] nreverse  (Fnreverse, 1/1, doc f29587a834ae, line 2268)
- [x] reverse  (Freverse, 1/1, doc 05b652fee31a, line 2322)
- [x] sort  (Fsort, 1/MANY, doc d6b00e74b2e0, line 2447)
- [x] plist-get  (Fplist_get, 2/3, doc 196d06523522, line 2609)
- [x] get  (Fget, 2/2, doc 3a06a6ed4e90, line 2653)
- [x] plist-put  (Fplist_put, 3/4, doc ff671588886d, line 2667)
- [x] put  (Fput, 3/3, doc 558e24991e33, line 2734)
- [x] plist-member  (Fplist_member, 2/3, doc b098019a6fb5, line 2745)
- [x] eql  (Feql, 2/2, doc 09d70dba5aec, line 2790)
- [x] equal  (Fequal, 2/2, doc 23ef7bfe99d7, line 2808)
- [x] equal-including-properties  (Fequal_including_properties, 2/2, doc 2ce2a1e59fae, line 2822)
- [x] value<  (Fvaluelt, 2/2, doc 7fa657a6810a, line 3291)
- [x] fillarray  (Ffillarray, 2/2, doc 8d340145c8e8, line 3308)
- [x] clear-string  (Fclear_string, 1/1, doc 685f2fb00703, line 3364)
- [x] nconc  (Fnconc, 0/MANY, doc d799230c5f87, line 3389)
- [x] mapconcat  (Fmapconcat, 2/3, doc 0d7fb2a58352, line 3484)
- [x] mapcar  (Fmapcar, 2/2, doc 5a8ca0bcf369, line 3551)
- [x] mapc  (Fmapc, 2/2, doc 861681b583f2, line 3569)
- [x] mapcan  (Fmapcan, 2/2, doc d378db7a3578, line 3585)
- [x] yes-or-no-p  (Fyes_or_no_p, 1/1, doc 011197db2511, line 3612)
- [x] load-average  (Fload_average, 0/1, doc 5a4d3ad18712, line 3688)
- [x] featurep  (Ffeaturep, 1/2, doc 5c2985cfa486, line 3724)
- [x] provide  (Fprovide, 1/2, doc ca0bf6c80c42, line 3742)
- [x] require  (Frequire, 1/3, doc 81fc008b30af, line 3781)
- [x] locale-info  (Flocale_info, 1/1, doc f10be09951a8, line 3896)
- [x] base64-encode-region  (Fbase64_encode_region, 2/3, doc dabb424b5f30, line 4081)
- [x] base64url-encode-region  (Fbase64url_encode_region, 2/3, doc 396979baa3a1, line 4099)
- [x] base64-encode-string  (Fbase64_encode_string, 1/2, doc f1407bc9c236, line 4168)
- [x] base64url-encode-string  (Fbase64url_encode_string, 1/2, doc d9cedc9f7057, line 4179)
- [x] base64-decode-region  (Fbase64_decode_region, 2/4, doc 7489dd7b6f12, line 4334)
- [x] base64-decode-string  (Fbase64_decode_string, 1/3, doc 71f94d0ccb2b, line 4409)
- [x] sxhash-eq  (Fsxhash_eq, 1/1, doc 8e708f6a2b3f, line 5617)
- [x] sxhash-eql  (Fsxhash_eql, 1/1, doc 6139f39a19b4, line 5627)
- [x] sxhash-equal  (Fsxhash_equal, 1/1, doc 19b5dcff04c9, line 5638)
- [x] sxhash-equal-including-properties  (Fsxhash_equal_including_properties, 1/1, doc 76a89c29c3cc, line 5649)
- [x] make-hash-table  (Fmake_hash_table, 0/MANY, doc 1f2fc0ad5bab, line 5720)
- [x] copy-hash-table  (Fcopy_hash_table, 1/1, doc 8bf76201f11c, line 5808)
- [x] hash-table-count  (Fhash_table_count, 1/1, doc 8b207866c4f4, line 5816)
- [x] hash-table-rehash-size  (Fhash_table_rehash_size, 1/1, doc 347c861107b2, line 5825)
- [x] hash-table-rehash-threshold  (Fhash_table_rehash_threshold, 1/1, doc d7a7ccee5eb1, line 5837)
- [x] hash-table-size  (Fhash_table_size, 1/1, doc 1722f7aed0d6, line 5849)
- [x] hash-table-test  (Fhash_table_test, 1/1, doc da132c69e7a3, line 5865)
- [x] hash-table-weakness  (Fhash_table_weakness, 1/1, doc 2bfe2b0b2fea, line 5886)
- [x] hash-table-p  (Fhash_table_p, 1/1, doc 2cf4ea6b0a5c, line 5895)
- [x] clrhash  (Fclrhash, 1/1, doc a006f78a80a1, line 5903)
- [x] gethash  (Fgethash, 2/3, doc f69fa3ea11e2, line 5915)
- [x] puthash  (Fputhash, 3/3, doc 7cddf457d45f, line 5929)
- [x] remhash  (Fremhash, 2/2, doc d44c2b245750, line 5949)
- [x] maphash  (Fmaphash, 2/2, doc afb479925b6d, line 5960)
- [x] define-hash-table-test  (Fdefine_hash_table_test, 3/3, doc 2b429b667c02, line 5978)
- [x] internal--hash-table-histogram  (Finternal__hash_table_histogram, 1/1, doc 60c764a6a99a, line 5995)
- [x] internal--hash-table-buckets  (Finternal__hash_table_buckets, 1/1, doc 6ba71c3e22d6, line 6023)
- [x] internal--hash-table-index-size  (Finternal__hash_table_index_size, 1/1, doc 000100a3361d, line 6046)
- [x] secure-hash-algorithms  (Fsecure_hash_algorithms, 0/0, doc 6d679b016a44, line 6092)
- [x] md5  (Fmd5, 1/5, doc c35dcafed389, line 6374)
- [x] secure-hash  (Fsecure_hash, 2/5, doc 7cc60ca1f0c2, line 6415)
- [x] buffer-hash  (Fbuffer_hash, 0/1, doc bd306417c5dc, line 6447)
- [x] buffer-line-statistics  (Fbuffer_line_statistics, 0/1, doc aca82348cc40, line 6492)
- [x] string-search  (Fstring_search, 2/3, doc 617c7fc7e397, line 6576)
- [x] object-intervals  (Fobject_intervals, 1/1, doc 8eccb14b2b15, line 6663)
- [x] line-number-at-pos  (Fline_number_at_pos, 0/2, doc 07dcc03b3783, line 6688)

## Functions defined — 101
- [x] get_random_fixnum  (line 60, static)
- [x] list_length  (line 117)
- [x] length_internal  (line 176, static)
- [x] load_unaligned_size_t  (line 462, static)
- [x] string_cmp  (line 471, static)
- [x] concat2  (line 705)
- [x] concat3  (line 711)
- [x] concat_to_string  (line 835, static)
- [x] concat_to_list  (line 1031)
- [x] concat_to_vector  (line 1114)
- [x] string_char_to_byte  (line 1196)
- [x] string_byte_to_char  (line 1256)
- [x] string_to_multibyte  (line 1322)
- [x] string_make_unibyte  (line 1343)
- [x] validate_subarray  (line 1545)
- [x] substring_both  (line 1648)
- [x] same_float  (line 1877, static)
- [x] eq_comparable_value  (line 1891, static)
- [x] memq_no_quit  (line 1925)
- [x] assq_no_quit  (line 1984)
- [x] assq_no_signal  (line 1997)
- [x] assoc_no_quit  (line 2037)
- [x] delq_no_quit  (line 2115)
- [x] sort_list  (line 2392, static)
- [x] sort_vector  (line 2438, static)
- [x] merge  (line 2515)
- [x] merge_c  (line 2561)
- [x] plist_get  (line 2639)
- [x] plist_put  (line 2708)
- [x] plist_member  (line 2775)
- [x] equal_no_quit  (line 2837)
- [x] internal_equal_1  (line 2861, static)
- [x] internal_equal_cycle  (line 3027, static)
- [x] internal_equal  (line 3054, static)
- [x] bool_vector_cmp  (line 3062, static)
- [x] fixnum_float_cmp  (line 3094, static)
- [x] value_cmp  (line 3116, static)
- [x] nconc2  (line 3384)
- [x] mapcar1  (line 3430, static)
- [x] do_yes_or_no_p  (line 3607)
- [x] require_unwind  (line 3776, static)
- [x] base64_encode_region_1  (line 4112, static)
- [x] base64_encode_string_1  (line 4192, static)
- [x] base64_encode_1  (line 4232, static)
- [x] base64_decode_1  (line 4454, static)
- [x] CHECK_HASH_TABLE  (line 4613, static)
- [x] set_hash_next_slot  (line 4619, static)
- [x] set_hash_hash_slot  (line 4625, static)
- [x] set_hash_index_slot  (line 4631, static)
- [x] check_hash_table  (line 4641, static)
- [x] next_almost_prime  (line 4653)
- [x] larger_vector  (line 4667)
- [x] HASH_NEXT  (line 4698, static)
- [x] HASH_INDEX  (line 4708, static)
- [x] restore_mutability  (line 4717, static)
- [x] hash_table_user_defined_call  (line 4729, static)
- [x] cmpfn_eql  (line 4744, static)
- [x] cmpfn_equal  (line 4753, static)
- [x] cmpfn_user_defined  (line 4763, static)
- [x] sxhash_eq  (line 4771, static)
- [x] sxhash_eql  (line 4778, static)
- [x] hashfn_eq  (line 4786, static)
- [x] hashfn_equal  (line 4794, static)
- [x] hashfn_eql  (line 4801, static)
- [x] hashfn_user_defined  (line 4810, static)
- [x] allocate_hash_table  (line 4829, static)
- [x] compute_hash_index_bits  (line 4836, static)
- [x] make_hash_table  (line 4865)
- [x] copy_hash_table  (line 4921, static)
- [x] hash_index_index  (line 4952, static)
- [x] maybe_resize_hash_table  (line 4961, static)
- [x] hash_table_test_from_std  (line 5032, static)
- [x] hash_table_thaw  (line 5045)
- [x] hash_find_with_hash  (line 5094, static)
- [x] hash_find  (line 5111)
- [x] hash_find_get_hash  (line 5119)
- [x] check_mutable_hash_table  (line 5128, static)
- [x] hash_put  (line 5139)
- [x] hash_remove_from_table  (line 5168)
- [x] hash_clear  (line 5208, static)
- [x] keep_entry_p  (line 5238, static)
- [x] sweep_weak_table  (line 5258)
- [x] hash_char_array  (line 5349)
- [x] sxhash_float  (line 5410, static)
- [x] sxhash_list  (line 5423, static)
- [x] sxhash_vector  (line 5451, static)
- [x] sxhash_bool_vector  (line 5469, static)
- [x] sxhash_bignum  (line 5485, static)
- [x] sxhash  (line 5498)
- [x] sxhash_obj  (line 5507, static)
- [x] hash_interval  (line 5583, static)
- [x] collect_interval  (line 5594, static)
- [x] reduce_emacs_uint_to_fixnum  (line 5612, static)
- [x] mark_fns  (line 5679)
- [x] get_hash_table_user_test  (line 5693, static)
- [x] hash_table_weakness_symbol  (line 5873)
- [x] hexbuf_digest  (line 6072)
- [x] make_digest_string  (line 6086, static)
- [x] extract_data_from_object  (line 6106)
- [x] secure_hash  (line 6289, static)
- [x] syms_of_fns  (line 6741)

## Macros defined — 4
- [x] HAVE_FAST_UNALIGNED_ACCESS  (line 457, body b6589fc6ab0d)
- [x] MIME_LINE_LENGTH  (line 3983, body d54ad009d179)
- [x] SXHASH_MAX_DEPTH  (line 5338, body 77de68daecd8)
- [x] SXHASH_MAX_LEN  (line 5343, body 902ba3cda188)

## Types and their fields — 3
- [x] double_and_words: double val, EMACS_UINT word
- [x] hash_table_user_test: struct hash_table_test test, struct hash_table_user_test* next
- [x] textprop_rec: ptrdiff_t argnum, ptrdiff_t to

## File-scope objects — 6
- [x] static Lisp_Object string_char_byte_cache_string  (line 1189, static)
- [x] static ptrdiff_t string_char_byte_cache_charpos  (line 1190, static)
- [x] static ptrdiff_t string_char_byte_cache_bytepos  (line 1191, static)
- [x] static Lisp_Object require_nesting_list  (line 3773, static)
- [x] static const hash_idx_t empty_hash_index_vector  (line 4851, static)
- [x] static struct hash_table_user_test hash_table_user_tests  (line 5676, static)

## Decisions

Port of `emacs/src/fns.c` (6,973 lines) to `femacs/src/fns.rs` (7307 lines; 4.8% drift, inside the
5% rule, so no `line-drift-tolerance.tsv` row).  All 104 `DEFUN`s, 101 functions, 4 macros, 8
constants, 3 types and 6 file-scope objects are ported; `tools/extract-abi.py --check` reports `OK`.

### Carried arms (P-51) and the frozen conditions

`src/fns.c` is **not** in `docs/baseline/config/cond-eval-verified.txt`
(`13 conditions, 0 unknown, 1 disagreements`), so the extractor keeps *both* arms of every `#if`
live and the port carries the text the frozen configuration does not compile:

- C 40-42 / 2978-2981 `#ifdef HAVE_TREE_SITTER` (`undef`) -> `#[cfg (feature = "have_tree_sitter")]`
  (`treesit_node_eq`, the `PVEC_TS_NODE` arm).
- C 448-458: `HAVE_FAST_UNALIGNED_ACCESS` is `#define`d in both arms; the frozen build's
  `CFLAGS = -g3 -O2` define `__OPTIMIZE__`, so the `#if` arm (`1`) is the compiled text and the
  `#else` (`0`) is carried under `no_fast_unaligned_access` (`bool`, so `if (…)` keeps C's shape).
- C 632-647 / 681-696 `#if defined __STDC_ISO_10646__ || defined WINDOWSNT`: neither macro is
  defined here (the frozen table has `WINDOWSNT undef` and no `__STDC_ISO_10646__`), so the `#else`
  arms (`Fstring_lessp`/`Fstring_equal`) are live and the `str_collate` arms are carried under
  `stdc_iso_10646`.
- C 3223-3228 `#ifdef subprocesses` is live (`1`); its arm is ordinary text.
- C 3892-3894, 3923, 3976, 6836-6841: `HAVE_LANGINFO_CODESET` is `1` and `<langinfo.h>` defines
  `DAY_1` (7) and `MON_1` (21), so those arms are live; `HAVE_LANGINFO__NL_PAPER_WIDTH` is `undef`,
  so the `paper` arm is carried under `have_langinfo_nl_paper_width` (the `_NL_PAPER_WIDTH`/
  `_NL_PAPER_HEIGHT` names are inside it).
- C 5383-5391 `#if EMACS_INT_MAX > INT32_MAX` is live (clang True; the one `check-cond-eval`
  disagreement, the evaluator cannot see `EMACS_INT_MAX`) and has no `#else`.

`gnutls.h` (C 40) contributes nothing to this unit on this target: every use it could add is inside
its own `#ifdef HAVE_GNUTLS` (`undef`) and the file only mentions the name in a docstring (C 6104);
the header has no mirror and there is nothing to declare at a use site.

### Structural deviations (each is also in `macro-body-notes.tsv` where a token moved)

- **C's forward declarations are dropped** (PORTING.md 0.1-12): C 45-47's prototypes of
  `internal_equal` and `sxhash_obj`, C 700-702's three `concat_to_*`, C 2837/2846/2851's
  `hash_find_with_hash`/`internal_equal_cycle`/`internal_equal_1`, C 4072-4080's three
  `base64_*_1`.  Rust is order-independent and a declaration plus a definition is `E0428`.
- **`attributes: const` is dropped** from `Fidentity` (and C 4771's other `attributes` uses): it is
  a `DEFUN` argument the macro never expands (it is make-docfile metadata only), like data.rs's
  `Feq`; `--check` does not compare it.
- **`enum equal_kind` is `type equal_kind = c_int` + three `const`s** (PORTING.md 1.2/1.3's
  spelling for a C enum used as a parameter type).
- **`WORDS_PER_DOUBLE`'s `sizeof (double)` is `size_of::<f64> ()`** (the `double` token is
  exempted in the notes file; the four `sizeof`s keep the `sizeof`/`size_of` family count),
  and `double_and_words` is a `#[repr(C)]` union with `val: f64` / `word: [EMACS_UINT; N]`.
- **`FOR_EACH_TAIL`'s terminating non-cons is tracked in the body** (the `list_end`/`tail_end`
  idiom, 12 call sites: `list_length`, `Fcopy_sequence`, `concat_to_list`, `Fmember`, `Fmemq`,
  `Fmemql`, `Fassq`, `Fassoc`, `Frassq`, `Frassoc`, `Fdelq`, `Fdelete`, `Freverse`): C documents
  that the loop *sets the caller's variable* to the terminating non-cons, and its
  `CHECK_LIST_END (x, x)` depends on that.  The port's macro re-binds the loop variable inside its
  own block, so the caller's variable is unchanged; recording `XCDR` as the last statement of the
  body reproduces C's value exactly (the loop runs the body once per cons, so the last recorded
  value is the terminating non-cons).  Without it `(length '(1 2 3))` would signal.
- **`internal_equal_1`/`internal_equal_cycle`/`Fnthcdr` write C's `FOR_EACH_TAIL_BASIC` +
  `FOR_EACH_TAIL_STEP_CYCLEP` out** because their bodies read the macros' own `li` iterator state,
  which `macro_rules!` hygiene cannot expose to call-site tokens (the fileio.rs precedent).  The
  expansion is the mirror's, statement for statement.
- **`Fnreverse`/`Fsafe_length`/`Fproper_list_p`/`value_cmp`/`Fnconc`/`Frequire`/`Fassoc_no_quit`'s
  loops** keep C's own variable names; the `while (1)` loops are `loop` (the `1` token is exempted).
- **`Fassoc`'s `continue` becomes an if/else**: in C `continue` runs the `for` increment, which is
  the port macro's own stepper; a Rust `continue` inside the body would skip that stepper and loop
  forever.
- **`syms_of_fns`'s `DOHASH_SAFE` is written out** (the macro is `deferred` in
  `excluded-macros.txt`), and **`Fmapconcat`'s `goto concat`** is the `jumped_to_concat` flag.
- **`value_cmp`'s `goto tail_recurse`/`goto type_mismatch`**: the backward one is the
  `'tail_recurse` loop's `continue`, the forward one is a `break` out of a labelled block whose
  tail is C's `xsignal2` call (the back_comment/`jumped_to_*` shape).
- **`internal_equal_1`'s five-case `PVEC_CLOSURE`… group** (with its `FALLTHROUGH` into
  `PVEC_NORMAL_VECTOR`) is one arm group that applies `size &= PSEUDOVECTOR_SIZE_MASK` for the
  group's own members: one copy of the shared element-wise body, PORTING.md 2.2.
- **C 5383's `EMACS_INT_MAX > INT32_MAX` arm** is kept as live text (it has no `#else`).
- **`hash_char_array`'s `memcpy (&c, p, sizeof hash)`** is `std::ptr::copy_nonoverlapping` with
  `std::mem::size_of_val`, and `uint32_t`/`uint16_t` are `u32`/`u16`.
- **`concat_to_list` and `concat_to_vector` have internal linkage**: C 699/701 declare them
  `static` (the *declaration* carries the keyword, the definitions at C 1031/1114 do not), so the
  port defines them as plain `unsafe fn` with no `#[unsafe(no_mangle)]`.  The ledger's function
  list does not mark them, so only the reference object sees it: `check-symbols.py --dir src`
  reported them as *invented* exported symbols against `build/emacs-ref/src/fns.o` until fixed
  (and then the text extractor wanted the symbol back, since *it* also reads the definition):
  both rows are therefore recorded in `docs/ports/excluded-macros.txt` (kind `linkage`) with the
  reason, which is what keeps `--check` `OK` without inventing the symbol.
- **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `Lisp_Object tail UNINIT`
  in `Fnconc`, `x` in `load_unaligned_size_t`, the `c` temporaries in `hash_char_array`, and every
  `&raw mut <local>` out-parameter (Rust rejects `&raw mut` of an uninitialised binding).
- **`copy_hash_table`'s `*h2 = *h1`** is `std::ptr::read (h1)` (docs/status.md 3.51's
  coding_system note): a struct assignment through raw pointers cannot move out of `*h1`.

### Deviations forced by mirrors (reported, not fixed here)

- ~~**`hash_find`, `hash_put` and `string_make_unibyte` are defined inside
  `pub mod fns_root_shadowed`**~~ **Resolved in status 3.87**: the mirror's three declarations
  moved into `pub mod lisp_hash_decls` + glob import, so the definitions are back at their C
  position (crate root) and the wrapper module is gone.  While it stood, the names and symbols
  were unchanged and every internal call went through the unit's own `_hash_*` symbol.
- ~~**The seven `pub mod decls` lib mirrors cannot be included together**~~ **Resolved in
  status 3.88/3.89**: the seven modules are named per file now (`md5_decls` and friends), so this
  lane's six mirrors are in the include list here and their names come from them -
  `filenvercmp`, `md5_buffer`, `sha224_buffer`, `sha256_buffer`, `sha384_buffer`, `sha512_buffer`,
  `sha3_{224,256,384,512}_buffer`, the ten `*_DIGEST_SIZE` constants and `<langinfo.h>`'s
  `DAY_1..DAY_7`/`MON_1..MON_12` all moved from use-site copies into the mirrors, and
  `tools/check-mirror-dupes.py` now reports zero duplicates for this unit (it saw the constants
  all along; the include graph is what changed).  The copy of `CODESET`'s note below is what the
  lane found while doing it.
- **`hash_table_test.cmpfn` is `Lisp_Object` in the mirror** (the C field is a function pointer;
  same size, and C itself tests it as a pointer): the three `hashtest_*` definitions store
  `cmpfn_eql as … as *const () as Lisp_Object` (a const-evaluable cast) and the two lookup loops
  transmute it back to `unsafe extern "C" fn (Lisp_Object, Lisp_Object, *mut Lisp_Hash_Table)
  -> Lisp_Object` to call it.
- **`.`name = LISPSYM_INITIALLY (Qeq)`** is not const-evaluable through the mirror's
  `LISPSYM_INITIALLY` (a plain `fn`): the port spells the same value as
  `(iQ* * size_of::<Lisp_Symbol> ()) as EMACS_UINT + Lisp_Type::Lisp_Symbol as EMACS_UINT` cast to
  `Lisp_Object` (the thread.rs/`main_thread` precedent for a C constant initialiser).
- **`lib/include/langinfo.rs`'s `CODESET` was 14; the frozen target's `<langinfo.h>` defines it
  as 0** (`clang -E -dM`, and `docs/baseline/config/darwin-arm64.macros.tsv` agrees).  **Fixed in
  the mirror with the `DAY_*`/`MON_*` items (status 3.86).**  `Flocale_info`
  must use the name the C body uses (the token rule and the mirror's root declaration leave no
  alternative), so the `codeset` arm passes the wrong item until the mirror is corrected; reported
  here rather than worked around.

### Names declared at their use site (mirror gaps, P-72 rule 3)

Each carries C's own prototype in a `/// C:` line and belongs to an unported or already-ported
unit's mirror: `get_random`, `seed_random`, `init_random` (lisp.h 5297-5299), `str_collate`
(5341), `make_specified_string` (4566, alloc.c), `tim_sort` (4289-4290, sort.c), `bool_vector_fill`
(4539, alloc.c), `xlispstrdup` (5553), `parse_str_as_multibyte` (4426-4427), `copy_text` (4323),
`redisplay_preserve_echo_area` (4547, xdisp.c), `load_with_autoload_queue` (4840-4842, lread.c),
`synchronize_system_time_locale` (5143, emacs.c), `getloadavg` (`<stdlib.h>`), `insert_1_both`
(4331-4332, insdel.c), `del_range_byte` (4347), `del_range_both` (4348), `hash_table_alloc_bytes`
/`hash_table_free_bytes` (4663-4664, alloc.c), `survives_gc_p`/`mark_object` (4435-4436, alloc.c),
`code_convert_string` (4685-4686, coding.c), `preferred_coding_system` (coding.h 800),
`nsberror` (buffer.h 1795), `string_ascii_p` (4223, character.c), `count_lines` (4403, indent.c),
`memmem` (`<string.h>`, macOS libc).

`emacs/src/lisp.h`'s `struct Lisp_Hash_Table` bitfields (`weakness`, `frozen_test`, `mutable`) are
the mirror's `bits` masks (`HASH_WEAKNESS_MASK`/`HASH_FROZEN_TEST_MASK`/`HASH_MUTABLE_MASK`), which
`token-aliases.tsv` records.

### Verification (this round)

`extract-abi.py --check` `OK`; `build-tu src/fns.c` `built`; `rustc … --cfg enable_checking`
exit 0; `check-globals.py` 0 missing; `check-linkage.py` 0 missing; `check-declarations.py` 0
forbidden; `check-verbatim.py` 0 failures; `check-mirror-dupes.py` 0 failures (its one row is the
pre-existing `insdel` ratchet).  `check-undef.py` reports the two references this unit adds to
the frozen build's keyboard.c — `last_input_event` at this run's measurement, and
`Fdiscard_input` in the first run, which the integrator's ratchet refresh has since absorbed —
because that unit is not ported yet (13,086 C lines), and ratchet entries that are now satisfied
(`_Fsubstring_no_properties`, `_Vfeatures`, `_merge`, `_use_dialog_box`, `_string_ascii_p`, and the
coding.c four this unit's first run absorbed); the ratchet file is the integrator's to re-derive
(`--write-pending`), since `docs/baseline/*` is outside this lane.
