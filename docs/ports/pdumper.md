# emacs/src/pdumper.c (5920 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 4
- [x] dump-emacs-portable  (Fdump_emacs_portable, 1/2, doc 75465330d977, line 4069)
- [x] dump-emacs-portable--sort-predicate  (Fdump_emacs_portable__sort_predicate, 2/2, doc 4eff09920db8, line 4367)
- [x] dump-emacs-portable--sort-predicate-copied  (Fdump_emacs_portable__sort_predicate_copied, 2/2, doc 4eff09920db8, line 4379)
- [x] pdumper-stats  (Fpdumper_stats, 0/0, doc da6d08f30070, line 5836)

## Functions defined — 202
- [x] divide_round_up  (line 111, static)
- [x] dump_trace  (line 144, static)
- [x] ptrdiff_t_to_dump_off  (line 158, static)
- [x] dump_get_max_page_size  (line 168, static)
- [x] emacs_reloc_set_type  (line 250, static)
- [x] dump_reloc_set_type  (line 300, static)
- [x] dump_reloc_get_offset  (line 307, static)
- [x] dump_reloc_set_offset  (line 313, static)
- [x] dump_fingerprint  (line 323)
- [x] dump_push  (line 620, static)
- [x] dump_pop  (line 628, static)
- [x] dump_tracking_referrers_p  (line 636, static)
- [x] dump_set_have_current_referrer  (line 642, static)
- [x] dump_set_referrer  (line 660, static)
- [x] dump_clear_referrer  (line 669, static)
- [x] dump_ptr_referrer  (line 678, static)
- [x] error_unsupported_dump_object  (line 694, static)
- [x] emacs_basis  (line 704, static)
- [x] emacs_ptr_at  (line 710, static)
- [x] emacs_offset  (line 718, static)
- [x] dump_builtin_symbol_p  (line 732, static)
- [x] dump_object_self_representing_p  (line 746, static)
- [x] intmax_t_from_lisp  (line 752, static)
- [x] intmax_t_to_lisp  (line 761, static)
- [x] dump_off_from_lisp  (line 767, static)
- [x] dump_off_to_lisp  (line 775, static)
- [x] dump_write  (line 781, static)
- [x] make_eq_hash_table  (line 801, static)
- [x] dump_tailq_init  (line 807, static)
- [x] dump_tailq_length  (line 814, static)
- [x] dump_tailq_prepend  (line 820, static)
- [x] dump_tailq_empty_p  (line 830, static)
- [x] dump_tailq_peek  (line 836, static)
- [x] dump_tailq_pop  (line 843, static)
- [x] dump_seek  (line 856, static)
- [x] dump_write_zero  (line 865, static)
- [x] dump_align_output  (line 879, static)
- [x] dump_object_start  (line 886, static)
- [x] dump_object_finish  (line 902, static)
- [x] dump_recall_object  (line 918, static)
- [x] dump_remember_object  (line 926, static)
- [x] dump_note_reachable  (line 936, static)
- [x] dump_object_emacs_ptr  (line 950, static)
- [x] dump_queue_init  (line 964, static)
- [x] dump_queue_empty_p  (line 976, static)
- [x] dump_queue_push_weight  (line 998, static)
- [x] dump_queue_enqueue  (line 1009, static)
- [x] dump_calc_link_score  (line 1094, static)
- [x] dump_queue_compute_score  (line 1113, static)
- [x] dump_queue_scan_fancy  (line 1144, static)
- [x] dump_queue_sequence  (line 1177, static)
- [x] dump_queue_find_score_of_one_weight_queue  (line 1192, static)
- [x] dump_queue_dequeue  (line 1228, static)
- [x] dump_object_needs_dumping_p  (line 1352, static)
- [x] dump_enqueue_object  (line 1358, static)
- [x] print_paths_to_root_1  (line 1390, static)
- [x] print_paths_to_root  (line 1409, static)
- [x] dump_remember_cold_op  (line 1415, static)
- [x] dump_reloc_dump_to_emacs_ptr_raw  (line 1430, static)
- [x] dump_reloc_dump_to_dump_lv  (line 1446, static)
- [x] dump_reloc_dump_to_dump_ptr_raw  (line 1479, static)
- [x] dump_reloc_dump_to_emacs_lv  (line 1496, static)
- [x] dump_emacs_reloc_copy_from_dump  (line 1530, static)
- [x] dump_emacs_reloc_immediate  (line 1559, static)
- [x] dump_emacs_reloc_to_dump_ptr_raw  (line 1596, static)
- [x] dump_emacs_reloc_to_lv  (line 1614, static)
- [x] dump_emacs_reloc_to_emacs_ptr_raw  (line 1642, static)
- [x] dump_remember_fixup_lv  (line 1681, static)
- [x] dump_remember_fixup_ptr_raw  (line 1701, static)
- [x] dump_root_visitor  (line 1722, static)
- [x] dump_roots  (line 1756, static)
- [x] field_relpos  (line 1766, static)
- [x] cpyptr  (line 1785, static)
- [x] dump_field_lv_or_rawptr  (line 1795, static)
- [x] dump_field_lv_rawptr  (line 1887, static)
- [x] dump_field_lv  (line 1910, static)
- [x] dump_field_fixup_later  (line 1922, static)
- [x] dump_field_ptr_to_dump_offset  (line 1934, static)
- [x] dump_field_emacs_ptr  (line 1960, static)
- [x] dump_object_start_pseudovector  (line 1982, static)
- [x] finish_dump_pvec  (line 1999, static)
- [x] dump_pseudovector_lisp_fields  (line 2006, static)
- [x] dump_cons  (line 2020, static)
- [x] dump_interval_tree  (line 2033, static)
- [x] dump_string  (line 2079, static)
- [x] dump_marker  (line 2125, static)
- [x] dump_interval_node  (line 2148, static)
- [x] dump_overlay  (line 2190, static)
- [x] dump_field_finalizer_ref  (line 2207, static)
- [x] dump_finalizer  (line 2221, static)
- [x] dump_treesit_compiled_query  (line 2239, static)
- [x] dump_bignum  (line 2259, static)
- [x] dump_float  (line 2297, static)
- [x] dump_field_fwd  (line 2310, static)
- [x] dump_blv  (line 2344, static)
- [x] dump_recall_symbol_aux  (line 2367, static)
- [x] dump_remember_symbol_aux  (line 2376, static)
- [x] dump_pre_dump_symbol  (line 2392, static)
- [x] dump_symbol  (line 2411, static)
- [x] dump_vectorlike_generic  (line 2497, static)
- [x] hash_table_contents  (line 2594, static)
- [x] dump_hash_table_list  (line 2611, static)
- [x] hash_table_std_test  (line 2618, static)
- [x] hash_table_freeze  (line 2634, static)
- [x] dump_hash_table_contents  (line 2647, static)
- [x] dump_hash_table  (line 2670, static)
- [x] dump_obarray_buckets  (line 2701, static)
- [x] dump_obarray  (line 2724, static)
- [x] dump_buffer  (line 2746, static)
- [x] dump_bool_vector  (line 2884, static)
- [x] dump_subr  (line 2901, static)
- [x] dump_native_comp_unit  (line 2958, static)
- [x] fill_pseudovec  (line 2981, static)
- [x] dump_nilled_pseudovec  (line 2991, static)
- [x] dump_vectorlike  (line 3000, static)
- [x] dump_object  (line 3111, static)
- [x] dump_object_for_offset  (line 3207, static)
- [x] dump_charset  (line 3215, static)
- [x] dump_charset_table  (line 3261, static)
- [x] dump_finalizer_list_head_ptr  (line 3279, static)
- [x] dump_metadata_for_pdumper  (line 3291, static)
- [x] dump_sort_copied_objects  (line 3317, static)
- [x] dump_hot_parts_of_discardable_objects  (line 3333, static)
- [x] dump_drain_copied_objects  (line 3348, static)
- [x] dump_cold_string  (line 3394, static)
- [x] dump_cold_charset  (line 3411, static)
- [x] dump_cold_buffer  (line 3425, static)
- [x] dump_cold_bignum  (line 3450, static)
- [x] dump_cold_native_subr  (line 3470, static)
- [x] dump_drain_cold_data  (line 3492, static)
- [x] read_ptr_raw_and_lv  (line 3548, static)
- [x] dump_drain_user_remembered_data_hot  (line 3576, static)
- [x] dump_drain_user_remembered_data_cold  (line 3601, static)
- [x] dump_unwind_cleanup  (line 3674, static)
- [x] dump_check_dump_off  (line 3690, static)
- [x] dump_check_emacs_off  (line 3697, static)
- [x] dump_decode_dump_reloc  (line 3703, static)
- [x] dump_emit_dump_reloc  (line 3715, static)
- [x] dump_check_overlap_dump_reloc  (line 3731, static)
- [x] decode_emacs_reloc  (line 3747, static)
- [x] dump_emit_emacs_reloc  (line 3831, static)
- [x] dump_merge_emacs_relocs  (line 3841, static)
- [x] drain_reloc_list  (line 3886, static)
- [x] dump_do_fixup  (line 3921, static)
- [x] dump_do_fixups  (line 4015, static)
- [x] dump_drain_normal_queue  (line 4033, static)
- [x] dump_drain_deferred_hash_tables  (line 4040, static)
- [x] dump_drain_deferred_symbols  (line 4055, static)
- [x] pdumper_do_now_and_after_load_impl  (line 4392)
- [x] pdumper_do_now_and_after_late_load_impl  (line 4401)
- [x] pdumper_remember_user_data_1  (line 4410, static)
- [x] pdumper_remember_scalar_impl  (line 4420)
- [x] pdumper_remember_lv_ptr_raw_impl  (line 4428)
- [x] dump_anonymous_allocate_w32  (line 4454, static)
- [x] dump_anonymous_allocate_posix  (line 4498, static)
- [x] dump_anonymous_release  (line 4552, static)
- [x] dump_anonymous_allocate  (line 4571, static)
- [x] dump_map_file_w32  (line 4591, static)
- [x] dump_map_file_posix  (line 4666, static)
- [x] dump_map_file  (line 4703, static)
- [x] dump_unmap_file  (line 4721, static)
- [x] dump_discard_mem  (line 4757, static)
- [x] dump_mmap_discard_contents  (line 4778, static)
- [x] dump_mmap_reset  (line 4785, static)
- [x] dump_mmap_release  (line 4793, static)
- [x] dump_mm_heap_cb_release  (line 4808, static)
- [x] dump_mmap_release_heap  (line 4822, static)
- [x] dump_mmap_contiguous_heap  (line 4829, static)
- [x] dump_mmap_release_vm  (line 4884, static)
- [x] needs_mmap_retry_p  (line 4893, static)
- [x] dump_mmap_contiguous_vm  (line 4904, static)
- [x] dump_mmap_contiguous  (line 5006, static)
- [x] dump_bitsets_init  (line 5040, static)
- [x] dump_bitset__bit_slot  (line 5056, static)
- [x] dump_bitset_bit_set_p  (line 5065, static)
- [x] dump_bitset__set_bit_value  (line 5074, static)
- [x] dump_bitset_set_bit  (line 5088, static)
- [x] dump_bitset_clear  (line 5094, static)
- [x] dump_ptr  (line 5125, static)
- [x] dump_read_word_from_dump  (line 5137, static)
- [x] dump_write_word_to_dump  (line 5149, static)
- [x] dump_write_lv_to_dump  (line 5161, static)
- [x] dump_find_relocation  (line 5178, static)
- [x] dump_loaded_p  (line 5209, static)
- [x] pdumper_cold_object_p_impl  (line 5215)
- [x] pdumper_find_object_type_impl  (line 5224)
- [x] pdumper_marked_p_impl  (line 5242)
- [x] pdumper_set_marked_impl  (line 5254)
- [x] pdumper_clear_marks_impl  (line 5267)
- [x] dump_read_all  (line 5276, static)
- [x] dump_reloc_size  (line 5298, static)
- [x] dump_make_lv_from_reloc  (line 5309, static)
- [x] dump_do_dump_relocation  (line 5344, static)
- [x] dump_do_all_dump_reloc_for_phase  (line 5502, static)
- [x] dump_do_emacs_relocation  (line 5513, static)
- [x] dump_do_all_emacs_relocations  (line 5563, static)
- [x] pdumper_set_emacs_execdir  (line 5576, static)
- [x] pdumper_load  (line 5622)
- [x] pdumper_record_wd  (line 5824)
- [x] thaw_hash_tables  (line 5870, static)
- [x] init_pdumper_once  (line 5881)
- [x] syms_of_pdumper  (line 5889)

## Macros defined — 16
- [x] DEFINE_EMACS_IMMEDIATE_FN  (line 1577, body 484513ba3914)
- [x] DUMP_BITSET_WORD_WIDTH  (line 5031, body 7d7f1a7cd8f6)
- [x] DUMP_FIELD_COPY  (line 1791, body f0f7a1a3e73f)
- [x] DUMP_OFF_MAX  (line 137, body 177f4b9163c2)
- [x] DUMP_OFF_MIN  (line 136, body e738b154238b)
- [x] DUMP_OFF_WIDTH  (line 138, body 92dc5221ada0)
- [x] INFINITY  (line 93, body 147ca149deac)
- [x] MAP_ANONYMOUS  (line 4494, body f8832fcd40ad)
- [x] MAP_POPULATE  (line 87, body b6589fc6ab0d)
- [x] PRIdDUMP_OFF  (line 139, body e89b343ab1b6)
- [x] START_DUMP_PVEC  (line 1993, body 2fda8dbf777b)
- [x] VM_MS_WINDOWS  (line 78, body da4b9237bacc)
- [x] VM_POSIX  (line 77, body 356a192b7913)
- [x] VM_SUPPORTED  (line 97, body b6589fc6ab0d)
- [x] dump_offsetof  (line 173, body e579805e547a)
- [x] nr_candidates  (line 1250, body 77de68daecd8)

## Types and their fields — 16
- [x] bignum_reload_info: dump_off data_location, dump_off nlimbs
- [x] candidate: float score, dump_off sequence
- [x] dump_bitset: dump_bitset_word *restrict bits, ptrdiff_t number_words
- [x] dump_context: struct dump_header header, void* buf, dump_off buf_size, dump_off max_offset, Lisp_Object old_purify_flag, Lisp_Object old_post_gc_hook, Lisp_Object old_process_environment, bool blocked_ralloc, int fd, Lisp_Object dump_filename, dump_off offset, dump_off obj_offset, struct dump_flags flags, dump_off end_heap, Lisp_Object objects_dumped, Lisp_Object referrers, Lisp_Object current_referrer, bool have_current_referrer, struct dump_queue dump_queue, Lisp_Object deferred_hash_tables, Lisp_Object deferred_symbols, Lisp_Object fixups, Lisp_Object staticpro_table, Lisp_Object symbol_aux, Lisp_Object copied_queue, Lisp_Object cold_queue, Lisp_Object dump_relocs, Lisp_Object object_starts, Lisp_Object emacs_relocs, Lisp_Object bignum_data, Lisp_Object hash_tables, dump_off number_hot_relocations, dump_off number_discardable_relocations
- [x] dump_flags: bool_bf dump_object_contents : 1, bool_bf record_object_starts : 1, bool_bf pack_objects : 1, bool_bf assert_already_seen : 1, bool_bf defer_hash_tables : 1, bool_bf defer_symbols : 1, bool_bf defer_cold_objects : 1, bool_bf defer_copied_objects : 1
- [x] dump_header: char magic, unsigned char fingerprint, struct dump_table_locator dump_relocs, struct dump_table_locator object_starts, struct dump_table_locator emacs_relocs, dump_off discardable_start, dump_off cold_start, dump_off hash_list
- [x] dump_memory_map: struct dump_memory_map_spec spec, void* mapping, void (*) release, void* private
- [x] dump_memory_map_heap_control_block: int refcount, void* mem
- [x] dump_memory_map_spec: int fd, size_t size, off_t offset, enum dump_memory_protection protection
- [x] dump_queue: struct dump_tailq zero_weight_objects, struct dump_tailq one_weight_normal_objects, struct dump_tailq one_weight_strong_objects, struct dump_tailq fancy_weight_objects, Lisp_Object link_weights, Lisp_Object sequence_numbers, dump_off next_sequence_number
- [x] dump_reloc: unsigned int raw_offset : DUMP_RELOC_OFFSET_BITS, ENUM_BF (dump_reloc_type) type : DUMP_RELOC_TYPE_BITS
- [x] dump_table_locator: dump_off offset, dump_off nr_entries
- [x] dump_tailq: Lisp_Object head, Lisp_Object tail, intptr_t length
- [x] emacs_reloc: ENUM_BF (emacs_reloc_type) type : EMACS_RELOC_TYPE_BITS, dump_off length : EMACS_RELOC_LENGTH_BITS, dump_off emacs_offset, dump_off dump_offset, dump_off emacs_offset2, intmax_t immediate, <aggregate> u
- [x] link_weight: int value
- [x] pdumper_loaded_dump_private: struct dump_header header, struct dump_bitset mark_bits, struct dump_bitset last_mark_bits, double load_time, char* dump_filename

## File-scope objects — 11
- [x] static pdumper_hook dump_hooks  (line 122, static)
- [x] static int nr_dump_hooks  (line 123, static)
- [x] static pdumper_hook dump_late_hooks  (line 125, static)
- [x] static int nr_dump_late_hooks  (line 126, static)
- [x] static int nr_remembered_data  (line 133, static)
- [x] static char emacs_execdir  (line 4439, static)
- [x] static ptrdiff_t execdir_size  (line 4440, static)
- [x] static ptrdiff_t execdir_len  (line 4441, static)
- [x] struct pdumper_loaded_dump dump_public  (line 5114)
- [x] static struct pdumper_loaded_dump_private dump_private  (line 5115, static)
- [x] static Lisp_Object pdumper_hashes  (line 5615, static)

## Decisions

`emacs/src/pdumper.c` @ `fd7afced403`; port `femacs/src/pdumper.rs` (**6,997 lines** vs C's
5,920 — +18.2% on the default `total` basis, so the file has a row in
`docs/ports/line-drift-tolerance.tsv`: `pdumper  0.20  …  total`).  The extra ~1,077 lines are
the carried dead arms (below), the clang-measured layout block (146 `const _` assertions) and
the use-site declarations the missing mirrors need.  All 249 ledger rows are checked
(`grep -c '^- \[ \]' docs/ports/pdumper.md` → 0); the per-function token comparison is the real
guard and it is `OK`.  The gates, verbatim:

    build-tu src/pdumper.c         -> built src/pdumper.rs -> ../build/femacs/pdumper.o
    extract-abi.py --check         -> OK emacs/src/pdumper.c -> femacs/src/pdumper.rs
    check-globals.py               -> check-globals: 260 units, 0 DEFVAR_* variables without a definition in their own unit
    check-linkage.py               -> check-linkage: 10 unit object(s) compared against the non-static objects of the C sources the frozen build links, 0 with 0 missing symbol(s)
    check-undef.py                 -> 2 new, both from build/femacs/lread.o (another worker's unit: `_Vload_file_rep_suffixes`, `_Vload_suffixes`); pdumper.o itself has 0 orphans (measured with the same tool's universe: `nm -u build/femacs/pdumper.o` minus universe minus the ratchet is empty)
    (extra) rustc --cfg enable_checking on the materialized unit -> exit 0
      `cd femacs && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort
       -C codegen-units=1 -C opt-level=0 --cfg enable_checking -o /tmp/pdumper-ec.o
       ../build/femacs/units/pdumper/_root.rs`; the only diagnostics are the three
       pre-existing mirror warnings (include/lisp.rs 3844 and include/charset.rs 697's unused
       doc comments, include/frame.rs 589's duplicate `#[inline(always)]`), none of them this
       unit's.
    (extra) the `have_native_comp` arm: the same rustc with `--cfg 'feature = "have_native_comp"'` and
      `HAVE_NATIVE_COMP` flipped to `true` in the *generated* copy of config.rs (what
      `xtask dead-arms` does) -> exit 0.  This unit is a finished unit, so `gate-probes`'
      dead-arms step compiles it with that feature on: the `#ifdef HAVE_NATIVE_COMP` arms
      are not stubs, they typecheck.

### Dead arms (P-08 / §3.49 — the file is *not* in cond-eval-verified.txt)

`python3 tools/check-cond-eval.py src/pdumper.c` reports **77 conditions, 0 unknown,
13 disagreements [NOT VERIFIED]** (the `HAVE_MADVISE`/`MAP_POPULATE` entries the frozen
macro table cannot see, the function-like `min`/`max` in the `DUMP_ALIGNMENT` enum, the
`VM_SUPPORTED` comparisons and `!USE_LSB_TAG`).  `src/pdumper.c` is therefore handled like
`src/timefns.c` (docs/status.md 3.49): **the frozen macro table is not consulted for it, every
`#if`/`#ifdef`/`#elif`/`#else` arm is carried as text**, the arm clang compiles as plain text
and each clang-dead arm under `#[cfg (feature = "…")]` naming C's own condition.  94 `#[cfg]`
sites, in these families:

- `have_pduper`-style liveness: `HAVE_PDUMPER` is 1, so C 71-5877 and the three `#ifdef`s
  inside `syms_of_pdumper`/`init_pdumper_once` are live text and their empty `#else` arms
  carry nothing.
- `have_native_comp` (`HAVE_NATIVE_COMP` undef): the `HAVE_NATIVE_COMP` bodies of `dump_subr`,
  `dump_native_comp_unit`, `dump_cold_native_subr`, `dump_vectorlike`,
  `dump_do_dump_relocation` (`RELOC_NATIVE_COMP_UNIT`, `RELOC_NATIVE_SUBR`),
  `pdumper_set_emacs_execdir`, the `emacs_execdir` trio, `COLD_OP_NATIVE_SUBR`'s
  `dump_drain_cold_data` arm, the `Lisp_Native_Comp_Unit` record and its two inlines, and
  `calln (intern_c_string ("load--fixup-all-elns"))` in `Fdump_emacs_portable`.
- `vm_ms_windows` / `no_vm_supported`: `dump_anonymous_allocate_w32`,
  `dump_map_file_w32`, `dump_anonymous_release`, `dump_unmap_file`,
  `dump_discard_mem`'s Windows arm, `needs_mmap_retry_p`'s `CYGWIN`/`_AIX` arm; the Windows
  names they need (`DWORD`, `HANDLE`, the `MEM_*`/`PAGE_*`/`FILE_MAP_*` constants and the
  seven Win32 calls) are declared in the same `#[cfg]` block.
- the other platform/macro arms: `msdos`, `windowsnt` (`INFINITY`, `dump_unmap_file`,
  `Fpdumper_stats`), `cygwin` (the `EINVAL` retry test, as a `#[cfg]`-selected predicate),
  `have_android` (the batch-mode error), `rel_alloc` (`blocked_ralloc`,
  `r_alloc_inhibit_buffer_relocation`), `have_tree_sitter` (`dump_treesit_compiled_query`,
  `XTS_COMPILED_QUERY`, `Lisp_TS_Query`), `no_use_lsb_tag`, `no_map_anonymous`,
  `not_use_lsb_tag_not_wide_emacs_int` / `have_mmap_map_fixed` / `no_map_populate*` (the
  `VM_SUPPORTED` chain of C 80-97) and `enable_checking` (C's `#ifdef ENABLE_CHECKING`
  blocks, which the extra compile pass above exercises).
- The naming convention is callproc.rs's: the feature names the C condition's non-frozen
  side; the `vm_*`/`vn` features are mutually exclusive by construction (they stand for one
  `#if/#elif/#else` chain), which is why the live arm of such a chain is unguarded and the
  dead arms carry the feature.
- **There is no `#if 0` in this file** (`grep -c '#if 0' emacs/src/pdumper.c` → 0), so nothing
  is recorded as dead text instead of transcribed: the only text not carried is inside the
  arms above, and it *is* carried, under its `#[cfg]`.

### `dump_fingerprint`-to-`pdumper_load` behaviour kept 1:1

The dump format is frozen (PORTING.md 4.4, P-09): `struct dump_header`'s field order,
`dump_table_locator`'s 3-entry array and the two relocation records are clang-measured and
asserted below, and `dump_fingerprint` (non-static in C, called by `emacs.c`) is exported.
`pdumper_load` keeps C's single `out:` cleanup path — a Rust `'out: { … }` labelled block whose
`break 'out` are C's `goto out`, so the cleanup statements (`dump_mmap_release` loop,
`emacs_close`) and the final `err` value keep their order; the `#if !USE_LSB_TAG` abort path
is the dead arm above.  C's `goto`-free functions (`dump_mmap_contiguous_heap`,
`dump_mmap_contiguous_vm`, `dump_map_file_w32`) use early `return` through the same cleanup
statements (recorded in macro-body-notes.tsv).

### Bitfields

Every `bool_bf`/bitfield group C reads by name is a packed byte or word in the mirror, with
`PORTING.md` 1.3's mask spelling and a family in `docs/ports/token-aliases.tsv`:

| C member(s) | storage | port |
| --- | --- | --- |
| `dump_flags`' eight `bool_bf : 1` (C 468-491) | one byte | `bits: u8` + `DUMP_FLAGS_*` masks; measured: the three set flags → byte `a1` |
| `emacs_reloc`'s `type : 3` + `length : 29` (C 245-254) | the 4-byte word at 0 | `raw: u32` + the `emacs_reloc_*!` accessors; measured: `type = RELOC_EMACS_EMACS_LV`, `length = -3` → bytes `ed ff ff ff` |
| `dump_reloc`'s `raw_offset : 27` + `type : 5` (C 292-296) | 4 bytes (C's own `static_assert` says so) | `raw: u32`; measured: `raw_offset = 0x123456`, `type = RELOC_DUMP_TO_EMACS_LV` → bytes `56 34 12 68` |
| `struct symbol`'s `gcmarkbit/redirect/trapped_write/interned/declared_special` | one word at 0 | `u.s.bits` + `SYMBOL_*` masks (lisp.rs); measured → `cf` |
| `struct interval`'s six bits (lisp.h 52-64) | byte at 40 | `bits` + `INTERVAL_*_BIT`; measured → `3f` |
| `struct Lisp_Hash_Table`'s `weakness/frozen_test/mutable` | byte at 61 | `bits` + `HASH_*_MASK`; measured → `35` |
| `struct Lisp_Marker`'s `need_adjustment/insertion_type` | byte at 16 | `bits` + `MARKER_*`; measured → `03` |
| `struct Lisp_Buffer_Local_Value`'s `local_if_set/found` | word at 0 | `bits` + `BLV_*`; measured → `03` |
| `struct buffer_text`'s `inhibit_shrinking/redisplay` | byte at 136 | `bits` + `BUFFER_TEXT_*`; measured → `03` |
| `struct buffer`'s four bits | byte at 960 | `bits` + `BUFFER_*`; measured → `0f` |
| `struct charset`'s six bits | byte at 80 | `bits` + `CHARSET_*`; measured → `3f` |
| `struct itree_node`'s three bits | byte at 72 | `bits` + `ITREE_*`; measured → `07` |

The byte dumps come from a probe that sets each flag and prints the record's non-zero bytes
(the numbers are in the table's last column and in this unit's `const _` block).

### Unions, function pointers, flexible arrays, `alloca`

- Unions: C's two anonymous unions are named `emacs_reloc_u` and the mirror's existing
  `Lisp_Fwd_u`; `union vectorlike_header` and `Lisp_Cons_u` come from lisp.rs.  The one
  anonymous *struct* in this file (`remembered_data[32]`'s element) is named
  `dump_remembered_data` (PORTING.md 1.3: a nested record needs a name); its measured layout
  is 16/8, `mem`@0, `sz`@8.
- Function pointers: `pdumper_hook` is the header mirror's `Option<unsafe extern "C" fn ()>`;
  `dump_memory_map.release` is `Option<unsafe extern "C" fn (*mut dump_memory_map)>` because C
  assigns two static bodies to it; the two `drain_reloc_*` typedefs are Rust `fn` pointers
  (the handlers are this unit's own functions and C never crosses that boundary).
- Flexible arrays: none declared here (`dump_char_table`-style members live in other files).
- `alloca`: one call site, `START_DUMP_PVEC` (C 1992-1995, used by
  `dump_marker`/`dump_overlay`/`dump_finalizer`/`dump_vectorlike`'s pvec arms).  PORTING.md
  P-05 is not yet decided, so the port takes the mechanism the tree already has — lisp.rs's
  `sa_stack_alloc` stack pool, the same call `AVAIL_ALLOCA!` makes — and records the open
  question here: **the pool must release a frame's allocation on return, as `alloca` does,
  or a full dump (millions of objects, each with one such buffer) grows without bound.**  That
  rule belongs to `alloc.rs`'s pool, which is not ported; this unit passes it the same
  `vectorlike_nbytes (in_hdr)` C's `alloca` took.
- Uninitialized locals (PORTING.md 1.5): `struct dump_header header;`,
  `struct dump_context ctx_buf = {0}`, `struct dump_reloc reloc;`,
  `struct emacs_reloc reloc = {0}`, `struct dump_memory_map sections[…] = {0}`,
  `struct stat stat;`, `struct dump_bitset mark_bits[2]`, `struct timespec`,
  `struct bignum_reload_info reload_info` and the out-objects every `dump_*` function
  `memcpy`s into are `mem::zeroed ()`/`{ … }`, named in the body's comment where C's text has
  no initializer.

### Mirror gaps and use-site declarations (27 names, no mirror asked to grow)

Declared with C's own prototype in `pdumper_use_site_decls` (plus the `#[cfg]`-gated blocks
for the dead arms), and reported for the integrator:

| name | C source | why not a mirror |
| --- | --- | --- |
| `mmap`, `munmap`, `madvise`, `posix_madvise`, `mprotect`, `MAP_*`, `PROT_*`, `MADV_*`, `POSIX_MADV_*`, `MAP_FAILED` | `<sys/mman.h>` | no mirror; values measured with `clang -E -dM` on the frozen target |
| `fprintf`, `vfprintf`, `snprintf`, `putc`, `stderr` | `<stdio.h>` | `lib/include/stdio.rs` carries the four md5-stream entries, `sprintf` and `fclose` |
| `labs`, `qsort`, `abort` | `<stdlib.h>` | `lib/include/stdlib.rs` carries malloc/realloc/calloc/free/getenv/strtol/strtod/strtoul |
| `strcat` | `<string.h>` | `lib/include/string.rs` carries mem*/str* without it |
| `int_least32_t`, `INT_LEAST32_MIN/MAX/WIDTH`, `PRIdLEAST32`, `UINT_FAST32_WIDTH`, `INTPTR_MAX` | `<stdint.h>`/`<inttypes.h>` | `lib/include/stdint.in.rs` is a WIP mirror (the WIDTH section only) |
| `hexbuf_digest`, `emacs_write`, `emacs_open_noquit`, `garbage_collect`, `fatal`, `staticvec`, `sys_fstat`, `splice_dir_file`, `file_name_absolute_p`, `hash_table_alloc_bytes`, `hash_table_thaw`, `visit_static_gc_roots`, `gc_root_visitor` | `lisp.h`/`fns.c` | `lisp.rs`/`globals.rs` do not reach them yet (`sys_fstat` is *defined* by `sysdep.rs`, which is ported — its prototype is what is missing) |
| `MAP_ANONYMOUS`, `MAXPATHLEN`, `S_ISREG`, `IS_DIRECTORY_SEP`/`DIRECTORY_SEP` | `<sys/mman.h>`, `<sys/param.h>`, `<sys/stat.h>`, `config.h` | no mirrors for the first two; the last two follow callproc.rs's use-site pair |
| `Lisp_Native_Comp_Unit` + `NATIVE_COMP_UNITP`/`XNATIVE_COMP_UNIT`, `dynlib_handle_ptr`, `dynlib_*`, `load_comp_unit`, `fixup_eln_load_path`, `file_access_p` | `comp.h`, `dynlib.h` | `include/comp.rs` is a 22-line stub (only `maybe_defer_native_compilation`); the whole block is `#[cfg (feature = "have_native_comp")]` |

`lib/include/fingerprint.rs` (the header mirror) and `include/pdumper.rs` (this unit's header)
were complete and were **not** modified.  Two names the *mirror import list* lacks are imported
at the top of the unit (`use std::ffi::c_float;`, `use core::ffi::VaList;`), which is an
import, not a declaration.

### Spelling deviations from the reference

- `type` is a Rust keyword: C's `type` parameters/locals are `r#type` (PORTING.md 0.2), which
  is the same name and the same token family.
- `struct candidate` is a function-local record in C and a unit-level `#[repr(C)]` struct here
  (one definition only); `START_DUMP_PVEC`'s expansion names the record's fields through
  `&raw mut (*out).header` where C writes `&out->header`, because the port's allocation is a
  raw pointer (the C text has an lvalue).
- `new object %0*"pI"x` and the three trace families: C's `PRIdDUMP_OFF`/`PRIdPTR`/
  `PRIdHASH_IDX` splices are the port's `concat!` pieces (same bytes, the same macros named).
- `dump_subr`'s `out.function.a0 = NULL` is `None` (the union member is an `Option`) and the
  `DUMP_FIELD_COPY` calls on bitfields are mask copies — both recorded in
  `docs/ports/macro-body-notes.tsv` with the other token-level differences (`switch`/`case`/
  `default`/`fallthrough` becoming `match`, `goto out` becoming `break 'out`, `countof`
  becoming the array length, `true`/`false` on the flags byte going through `set_flag!`).
- `defsubr (&raw mut Sdump_emacs_portable);` etc. keep C's call shape (the four subr records
  are this unit's `DEFUN!` static records).

### The `DEFUN`s

The four subrs (`dump-emacs-portable`, its two sort predicates, `pdumper-stats`) are `DEFUN!`
definitions with C's Lisp names, min/max and docstrings (the two predicates share C's
`Internal relocation sorting function.` text).  `Fdump_emacs_portable`'s body keeps C's order,
including the `record_unwind_protect_ptr (dump_unwind_cleanup, ctx)` registration, the
`block_input`/`unblock_input` pair and the byte-count `fprintf` at the end.  The
`Vpdumper_fingerprint` carrier `DEFVAR_LISP` writes is defined **by this unit**
(`#[unsafe(no_mangle)] pub static mut Vpdumper_fingerprint`, docs/status.md 3.29/3.60: the unit
that owns the `DEFVAR` defines the object).

### Call-relationship differences (D7)

None in the direction that matters: every function this unit calls that C calls is declared by
the mirror the reference header names, and the two directions the gate can see are clean —
`check-linkage.py` reports **0 missing symbols** for this unit's object, and `check-undef.py`
finds **0 orphans** in `build/femacs/pdumper.o` (the two `new` entries it prints are
`build/femacs/lread.o`'s, another worker's unit).  Three relationships are *shaped*
differently and are recorded above: the two `drain_reloc_*` handlers/mergers (Rust `fn`
pointers, never passed to C), the `dump_memory_map.release` bodies (C function pointers to
this unit's own statics), and `dump_object_start_pseudovector`/`START_DUMP_PVEC` (the
allocation is the port's stack pool).  Functions C leaves to other units
(`hexbuf_digest`, `hash_table_alloc_bytes`, `hash_table_thaw`, `visit_static_gc_roots`,
`garbage_collect`, `fatal`, `sys_fstat`, `file_name_absolute_p`, `splice_dir_file`,
`emacs_write`, `emacs_open_noquit`, `emacs_close`, `emacs_open`) stay declared, never
re-implemented here.
