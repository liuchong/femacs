# emacs/src/alloc.c (7639 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 22
- [x] make-string  (Fmake_string, 2/3, doc 2a352c841272, line 2090)
- [x] make-bool-vector  (Fmake_bool_vector, 2/2, doc b62d0fda5a34, line 2202)
- [x] bool-vector  (Fbool_vector, 0/MANY, doc baa458bdfd05, line 2215)
- [x] cons  (Fcons, 2/2, doc 927fba7e91dc, line 2625)
- [x] list  (Flist, 0/MANY, doc 19fce85ffbed, line 2725)
- [x] make-list  (Fmake_list, 2/2, doc d6a5312fca50, line 2743)
- [x] make-record  (Fmake_record, 3/3, doc cc428a4d6f49, line 3499)
- [x] record  (Frecord, 1/MANY, doc 3e3fb1208f6f, line 3516)
- [x] make-vector  (Fmake_vector, 2/2, doc 1d15985953ba, line 3530)
- [x] vector  (Fvector, 0/MANY, doc 797623c286dc, line 3553)
- [x] make-byte-code  (Fmake_byte_code, 4/MANY, doc 65afe51b2585, line 3565)
- [x] make-closure  (Fmake_closure, 1/MANY, doc 7df35377ec4a, line 3600)
- [x] make-symbol  (Fmake_symbol, 1/1, doc 68bdcb2063cf, line 3701)
- [x] make-marker  (Fmake_marker, 0/0, doc 0cf208b4d934, line 3782)
- [x] make-finalizer  (Fmake_finalizer, 1/1, doc 2dba751b9998, line 3978)
- [x] garbage-collect  (Fgarbage_collect, 0/0, doc 0daeba8a7389, line 6030)
- [x] garbage-collect-heapsize  (Fgarbage_collect_heapsize, 0/0, doc f4f999621396, line 6054)
- [x] garbage-collect-maybe  (Fgarbage_collect_maybe, 1/1, doc bdd70dd9c033, line 6110)
- [x] memory-info  (Fmemory_info, 0/0, doc 472264f5cef7, line 7154)
- [x] memory-use-counts  (Fmemory_use_counts, 0/0, doc 1452a36058ba, line 7212)
- [x] malloc-info  (Fmalloc_info, 0/0, doc 9683ee74f308, line 7237)
- [x] malloc-trim  (Fmalloc_trim, 0/1, doc ac0a84990f21, line 7251)

## Functions defined — 209
- [x] malloc_initialize_hook  (line 221, static)
- [x] no_sanitize_memcpy  (line 368, static)
- [x] deadp  (line 426, static)
- [x] XPNTR  (line 512, static)
- [x] XFLOAT_INIT  (line 520, static)
- [x] tally_consing  (line 529, static)
- [x] pointers_fit_in_lispobj_p  (line 536, static)
- [x] mmap_lisp_allowed_p  (line 542, static)
- [x] malloc_warning  (line 570)
- [x] display_malloc_warning  (line 580)
- [x] buffer_memory_full  (line 592)
- [x] xmalloc  (line 658)
- [x] xzalloc  (line 673)
- [x] xcalloc  (line 685)
- [x] xrealloc  (line 700)
- [x] xfree  (line 713)
- [x] xnmalloc  (line 735)
- [x] xnrealloc  (line 749)
- [x] xpalloc  (line 783)
- [x] xstrdup  (line 831)
- [x] xlispstrdup  (line 842)
- [x] dupstring  (line 854)
- [x] xputenv  (line 866)
- [x] record_xmalloc  (line 875)
- [x] lisp_malloc  (line 902, static)
- [x] lisp_free  (line 944, static)
- [x] aligned_alloc  (line 976, static)
- [x] pointer_align  (line 1075, static)
- [x] lisp_align_malloc  (line 1086, static)
- [x] lisp_align_free  (line 1183, static)
- [x] make_interval  (line 1285)
- [x] mark_interval_tree_1  (line 1322, static)
- [x] mark_interval_tree  (line 1334, static)
- [x] sdata_size  (line 1510, static)
- [x] init_strings  (line 1545, static)
- [x] string_bytes  (line 1609)
- [x] check_sblock  (line 1623, static)
- [x] check_string_bytes  (line 1642, static)
- [x] check_string_free_list  (line 1674, static)
- [x] allocate_string  (line 1694, static)
- [x] allocate_string_data  (line 1755, static)
- [x] sweep_strings  (line 1852, static)
- [x] free_large_strings  (line 1959, static)
- [x] compact_small_strings  (line 1985, static)
- [x] string_overflow  (line 2082)
- [x] bool_vector_fill  (line 2152)
- [x] make_clear_bool_vector  (line 2171)
- [x] make_uninit_bool_vector  (line 2197)
- [x] make_string  (line 2235)
- [x] make_unibyte_string  (line 2254)
- [x] make_multibyte_string  (line 2267)
- [x] make_string_from_bytes  (line 2281)
- [x] make_specified_string  (line 2299)
- [x] make_clear_string  (line 2325, static)
- [x] make_uninit_string  (line 2340)
- [x] make_clear_multibyte_string  (line 2351, static)
- [x] make_uninit_multibyte_string  (line 2373)
- [x] make_formatted_string  (line 2381)
- [x] pin_string  (line 2399)
- [x] make_float  (line 2507)
- [x] free_cons  (line 2615)
- [x] list1  (line 2665)
- [x] list2  (line 2671)
- [x] list3  (line 2678)
- [x] list4  (line 2684)
- [x] list5  (line 2690)
- [x] cons_listn  (line 2700, static)
- [x] listn  (line 2716)
- [x] next_vector  (line 2771, static)
- [x] set_next_vector  (line 2777, static)
- [x] ADVANCE  (line 2825, static)
- [x] VINDEX  (line 2836, static)
- [x] large_vector_vec  (line 2863, static)
- [x] setup_on_free_list  (line 2921, static)
- [x] allocate_vector_block  (line 2939, static)
- [x] pseudovector_nbytes  (line 2958, static)
- [x] init_vectors  (line 2970, static)
- [x] allocate_vector_from_block  (line 2995, static)
- [x] vectorlike_nbytes  (line 3069)
- [x] cleanup_vector  (line 3106, static)
- [x] sweep_vectors  (line 3267, static)
- [x] allocate_vectorlike  (line 3379, static)
- [x] allocate_clear_vector  (line 3421, static)
- [x] allocate_vector  (line 3435)
- [x] allocate_nil_vector  (line 3443)
- [x] allocate_pseudovector  (line 3452)
- [x] allocate_buffer  (line 3472)
- [x] allocate_record  (line 3487, static)
- [x] make_vector  (line 3543)
- [x] set_symbol_name  (line 3680, static)
- [x] init_symbol  (line 3686)
- [x] make_misc_ptr  (line 3742)
- [x] build_symbol_with_pos  (line 3752)
- [x] build_overlay  (line 3768)
- [x] build_marker  (line 3801)
- [x] make_event_array  (line 3829)
- [x] make_user_ptr  (line 3862)
- [x] init_finalizer_list  (line 3873, static)
- [x] finalizer_insert  (line 3881, static)
- [x] unchain_finalizer  (line 3893, static)
- [x] mark_finalizer_list  (line 3905, static)
- [x] queue_doomed_finalizers  (line 3921, static)
- [x] run_finalizer_handler  (line 3940, static)
- [x] run_finalizer_function  (line 3947, static)
- [x] run_finalizers  (line 3960, static)
- [x] vector_marked_p  (line 4008, static)
- [x] set_vector_marked  (line 4025, static)
- [x] vectorlike_marked_p  (line 4037, static)
- [x] set_vectorlike_marked  (line 4043, static)
- [x] cons_marked_p  (line 4049, static)
- [x] set_cons_marked  (line 4057, static)
- [x] string_marked_p  (line 4066, static)
- [x] set_string_marked  (line 4074, static)
- [x] symbol_marked_p  (line 4083, static)
- [x] set_symbol_marked  (line 4091, static)
- [x] interval_marked_p  (line 4100, static)
- [x] set_interval_marked  (line 4108, static)
- [x] memory_full  (line 4130)
- [x] memory_full_up  (line 4176)
- [x] refill_memory_reserve  (line 4189)
- [x] mem_init  (line 4235, static)
- [x] mem_find  (line 4249, static)
- [x] mem_insert  (line 4272, static)
- [x] mem_insert_fixup  (line 4330, static)
- [x] mem_rotate_left  (line 4408, static)
- [x] mem_rotate_right  (line 4447, static)
- [x] mem_delete  (line 4476, static)
- [x] mem_delete_fixup  (line 4530, static)
- [x] live_string_holding  (line 4617, static)
- [x] live_string_p  (line 4653, static)
- [x] live_cons_holding  (line 4663, static)
- [x] live_cons_p  (line 4700, static)
- [x] live_symbol_holding  (line 4711, static)
- [x] live_symbol_p  (line 4755, static)
- [x] live_float_holding  (line 4766, static)
- [x] live_float_p  (line 4800, static)
- [x] live_vector_pointer  (line 4808, static)
- [x] live_large_vector_holding  (line 4841, static)
- [x] live_large_vector_p  (line 4848, static)
- [x] live_small_vector_holding  (line 4858, static)
- [x] live_small_vector_p  (line 4881, static)
- [x] mark_maybe_pointer  (line 4890, static)
- [x] mark_memory  (line 5028)
- [x] test_setjmp  (line 5087, static)
- [x] test_setjmp  (line 5132, static)
- [x] mark_c_stack  (line 5248)
- [x] flush_stack_call_func1  (line 5288)
- [x] valid_pointer_p  (line 5300, static)
- [x] valid_lisp_object_p  (line 5340)
- [x] hash_table_alloc_bytes  (line 5406)
- [x] hash_table_free_bytes  (line 5417)
- [x] staticpro  (line 5433)
- [x] allow_garbage_collection  (line 5451, static)
- [x] inhibit_garbage_collection  (line 5458)
- [x] object_bytes  (line 5471, static)
- [x] total_bytes_of_live_objects  (line 5480, static)
- [x] compact_font_cache_entry  (line 5500, static)
- [x] compact_font_caches  (line 5567, static)
- [x] compact_undo_list  (line 5599, static)
- [x] android_make_lisp_symbol  (line 5623, static)
- [x] visit_vectorlike_root  (line 5639, static)
- [x] visit_buffer_root  (line 5653, static)
- [x] visit_static_gc_roots  (line 5673)
- [x] mark_object_root_visitor  (line 5693, static)
- [x] mark_and_sweep_weak_table_contents  (line 5706, static)
- [x] consing_threshold  (line 5740, static)
- [x] bump_consing_until_gc  (line 5768, static)
- [x] watch_gc_cons_threshold  (line 5782, static)
- [x] watch_gc_cons_percentage  (line 5794, static)
- [x] maybe_garbage_collect  (line 5805)
- [x] garbage_collect  (line 5815)
- [x] mark_glyph_matrix  (line 6138, static)
- [x] mark_vectorlike  (line 6182, static)
- [x] mark_char_table  (line 6208, static)
- [x] mark_overlay  (line 6236, static)
- [x] mark_overlays  (line 6248, static)
- [x] mark_buffer  (line 6260, static)
- [x] mark_face_cache  (line 6291, static)
- [x] mark_frame  (line 6311, static)
- [x] mark_window  (line 6350, static)
- [x] mark_stack_empty_p  (line 6388, static)
- [x] mark_stack_pop  (line 6395, static)
- [x] grow_mark_stack  (line 6413, static)
- [x] mark_stack_push_value  (line 6424, static)
- [x] mark_stack_push_values  (line 6433, static)
- [x] check_live  (line 6451, static)
- [x] check_allocated_and_live  (line 6466, static)
- [x] check_allocated_and_live_symbol  (line 6487, static)
- [x] check_allocated_and_live_vectorlike  (line 6496, static)
- [x] process_mark_stack  (line 6519, static)
- [x] mark_object  (line 6769)
- [x] mark_objects  (line 6777)
- [x] mark_terminals  (line 6788, static)
- [x] survives_gc_p  (line 6803)
- [x] sweep_conses  (line 6850, static)
- [x] sweep_floats  (line 6930, static)
- [x] sweep_intervals  (line 6982, static)
- [x] sweep_symbols  (line 7033, static)
- [x] unchain_dead_markers  (line 7107, static)
- [x] sweep_buffers  (line 7123, static)
- [x] gc_sweep  (line 7140, static)
- [x] symbol_uses_obj  (line 7282, static)
- [x] which_symbols  (line 7300)
- [x] die  (line 7349)
- [x] verify_alloca  (line 7364, static)
- [x] init_alloc_once  (line 7387)
- [x] init_alloc_once_for_pdumper  (line 7408, static)
- [x] init_alloc  (line 7425)
- [x] syms_of_alloc  (line 7432)

## Macros defined — 94
- [x] ABLOCKS_BASE  (line 1055, body 338e2aaca2bc)
- [x] ABLOCKS_BUSY  (line 1049, body ea0910f5656c)
- [x] ABLOCKS_BYTES  (line 1041, body 30c3d8241f9a)
- [x] ABLOCKS_SIZE  (line 1005, body 1574bddb75c7)
- [x] ABLOCK_ABASE  (line 1043, body eb6a885d0ca4)
- [x] ASAN_POISON_ABLOCK  (line 1065, body a29646b15239)
- [x] ASAN_POISON_CONS  (line 2608, body a29646b15239)
- [x] ASAN_POISON_CONS_BLOCK  (line 2607, body a29646b15239)
- [x] ASAN_POISON_FLOAT  (line 2488, body a29646b15239)
- [x] ASAN_POISON_FLOAT_BLOCK  (line 2486, body a29646b15239)
- [x] ASAN_POISON_INTERVAL  (line 1278, body a29646b15239)
- [x] ASAN_POISON_INTERVAL_BLOCK  (line 1276, body a29646b15239)
- [x] ASAN_POISON_SBLOCK_DATA  (line 1594, body a29646b15239)
- [x] ASAN_POISON_STRING  (line 1597, body a29646b15239)
- [x] ASAN_POISON_STRING_BLOCK  (line 1595, body a29646b15239)
- [x] ASAN_POISON_SYMBOL  (line 3665, body a29646b15239)
- [x] ASAN_POISON_SYMBOL_BLOCK  (line 3663, body a29646b15239)
- [x] ASAN_POISON_VECTOR_CONTENTS  (line 2913, body a29646b15239)
- [x] ASAN_PREPARE_DEAD_SDATA  (line 1592, body a29646b15239)
- [x] ASAN_PREPARE_LIVE_SDATA  (line 1593, body a29646b15239)
- [x] ASAN_UNPOISON_ABLOCK  (line 1066, body a29646b15239)
- [x] ASAN_UNPOISON_CONS  (line 2609, body a29646b15239)
- [x] ASAN_UNPOISON_FLOAT  (line 2489, body a29646b15239)
- [x] ASAN_UNPOISON_FLOAT_BLOCK  (line 2487, body a29646b15239)
- [x] ASAN_UNPOISON_INTERVAL  (line 1279, body a29646b15239)
- [x] ASAN_UNPOISON_INTERVAL_BLOCK  (line 1277, body a29646b15239)
- [x] ASAN_UNPOISON_STRING  (line 1598, body a29646b15239)
- [x] ASAN_UNPOISON_STRING_BLOCK  (line 1596, body a29646b15239)
- [x] ASAN_UNPOISON_SYMBOL  (line 3666, body a29646b15239)
- [x] ASAN_UNPOISON_SYMBOL_BLOCK  (line 3664, body a29646b15239)
- [x] ASAN_UNPOISON_VECTOR_BLOCK  (line 2915, body a29646b15239)
- [x] ASAN_UNPOISON_VECTOR_CONTENTS  (line 2914, body a29646b15239)
- [x] BLOCK_ALIGN  (line 964, body d71c75f9f01d)
- [x] BLOCK_BYTES  (line 1000, body 7f0fd1a5c2d7)
- [x] BLOCK_PADDING  (line 999, body b6589fc6ab0d)
- [x] COMMON_MULTIPLE  (line 613, body fbbebf5022ce)
- [x] CONS_BLOCK  (line 2558, body 6b9bab22d820)
- [x] CONS_BLOCK_SIZE  (line 2552, body 4e69c0dac633)
- [x] CONS_INDEX  (line 2562, body b826715447d4)
- [x] FLOAT_BLOCK  (line 2450, body 3108a678adb9)
- [x] FLOAT_BLOCK_SIZE  (line 2431, body a2a5952039bd)
- [x] FLOAT_INDEX  (line 2454, body 11790861ee7d)
- [x] GC_ASAN_POISON_OBJECTS  (line 113, body b6589fc6ab0d)
- [x] GC_CDR_COUNT  (line 6171, body b6589fc6ab0d)
- [x] GC_CHECK_MARKED_OBJECTS  (line 124, body b6589fc6ab0d)
- [x] GC_DEFAULT_THRESHOLD  (line 275, body 016761336932)
- [x] GC_POINTER_ALIGNMENT  (line 5022, body b8ce573a6bd4)
- [x] GC_STRING_EXTRA  (line 1521, body b6edeb975e71)
- [x] GC_STRING_OVERRUN_COOKIE_SIZE  (line 1502, body b6589fc6ab0d)
- [x] GETMARKBIT  (line 2437, body 20d1ae0b45a7)
- [x] HI_THRESHOLD  (line 304, body 6a0e3c4f1180)
- [x] LARGE_STRING_BYTES  (line 1374, body 128351137a9c)
- [x] MALLOC_PROBE  (line 649, body 93a68942377d)
- [x] MALLOC_SIZE_NEAR  (line 202, body 15c4740726cb)
- [x] MAX_SAVE_STACK  (line 355, body 7c2b08a8ef93)
- [x] MEM_NIL  (line 491, body 6703845e2c9a)
- [x] MMAP_MAX_AREAS  (line 215, body b1a65867cc11)
- [x] NEXT_FREE_LISP_STRING  (line 1482, body 2cea0efce177)
- [x] PSEUDOVEC_STRUCT  (line 3097, body 5245fe11524c)
- [x] SDATA_DATA  (line 1422, body fc59874d03d0)
- [x] SDATA_NBYTES  (line 1421, body 38cae05bebc4)
- [x] SDATA_OF_STRING  (line 1489, body 71eccd6def2f)
- [x] SETJMP_WILL_LIKELY_WORK  (line 5095, body 93b26c9a85c1)
- [x] SETJMP_WILL_NOT_WORK  (line 5108, body 50204da3b098)
- [x] SETMARKBIT  (line 2442, body 7fed163638c8)
- [x] SET_STACK_TOP_ADDRESS  (line 5199, body 204c3be352f8)
- [x] SPARE_MEMORY  (line 337, body 74bb80ab44da)
- [x] SYMBOL_BLOCK_SIZE  (line 3642, body 16097b1d89d9)
- [x] UNSETMARKBIT  (line 2446, body 66d28e7809b4)
- [x] USE_ALIGNED_ALLOC  (line 973, body 356a192b7913)
- [x] USE_VALGRIND  (line 77, body 356a192b7913)
- [x] VECTOR_ELTS_MAX  (line 3368, body 7d115ac9dfca)
- [x] VECTOR_IN_BLOCK  (line 3062, body 144895cbed3c)
- [x] XCONS_MARKED_P  (line 2573, body 643d263c381d)
- [x] XFLOAT_MARK  (line 2468, body 95dcb78b41cf)
- [x] XFLOAT_MARKED_P  (line 2465, body 6f4a9a3027db)
- [x] XFLOAT_UNMARK  (line 2471, body db87589ec56e)
- [x] XMARK_CONS  (line 2576, body a78ffd0bff34)
- [x] XMARK_STRING  (line 265, body 3c48818646c9)
- [x] XMARK_VECTOR  (line 269, body e2dfac745df9)
- [x] XSTRING_MARKED_P  (line 267, body fbbc2340e9e4)
- [x] XUNMARK_CONS  (line 2579, body 663269f7e523)
- [x] XUNMARK_STRING  (line 266, body c035b7fd131b)
- [x] XUNMARK_VECTOR  (line 270, body 4fd5b660cb45)
- [x] XVECTOR_MARKED_P  (line 271, body 275d1f51dcea)
- [x] __MALLOC_HOOK_VOLATILE  (line 255, body da39a3ee5e6b)
- [x] aligned_alloc  (line 974, body 4f7ee4d85459)
- [x] check_string_bytes  (line 1664, body a29646b15239)
- [x] check_string_free_list  (line 1688, body da39a3ee5e6b)
- [x] compact_font_caches  (line 5591, body 75022a406384)
- [x] make_lisp_symbol  (line 5620, body e46662896d74)
- [x] verify_alloca  (line 7378, body a29646b15239)
- [x] vroundup  (line 2797, body df82d7f6c66a)
- [x] vroundup_ct  (line 2795, body a9ddd272b6b7)

## Types and their fields — 16
- [x] <anon@1414>: struct Lisp_String* string, ptrdiff_t nbytes
- [x] ablock: char payload, struct ablock* next_free, <aggregate> x, struct ablocks* abase, char padding
- [x] ablocks: struct ablock blocks
- [x] cons_block: struct Lisp_Cons conses, bits_word gcmarkbits, struct cons_block* next
- [x] emacs_align_type: struct frame frame, struct Lisp_Bignum Lisp_Bignum, struct Lisp_CondVar Lisp_CondVar, struct Lisp_Finalizer Lisp_Finalizer, struct Lisp_Float Lisp_Float, struct Lisp_Hash_Table Lisp_Hash_Table, struct Lisp_Marker Lisp_Marker, struct Lisp_Misc_Ptr Lisp_Misc_Ptr, struct Lisp_Mutex Lisp_Mutex, struct Lisp_Overlay Lisp_Overlay, struct Lisp_Subr Lisp_Subr, struct Lisp_Sqlite Lisp_Sqlite, struct Lisp_User_Ptr Lisp_User_Ptr, struct terminal terminal, struct thread_state thread_state, struct window window
- [x] float_block: struct Lisp_Float floats, bits_word gcmarkbits, struct float_block* next
- [x] interval_block: struct interval intervals, struct interval_block* next
- [x] large_vector: struct large_vector* next
- [x] mark_entry: ptrdiff_t n, Lisp_Object value, Lisp_Object* values, <aggregate> u
- [x] mark_stack: struct mark_entry* stack, ptrdiff_t size, ptrdiff_t sp
- [x] mem_node: struct mem_node* left, struct mem_node* right, struct mem_node* parent, void* start, void* end, enum { MEM_BLACK, enum { color, enum mem_type type
- [x] sblock: struct sblock* next, sdata* next_free, sdata data
- [x] sdata: struct Lisp_String* string, ptrdiff_t nbytes, unsigned char data
- [x] string_block: struct Lisp_String strings, struct string_block* next
- [x] symbol_block: struct Lisp_Symbol symbols, struct symbol_block* next
- [x] vector_block: char data, struct vector_block* next

## File-scope objects — 51
- [x] struct emacs_globals globals  (line 278)
- [x] EMACS_INT consing_until_gc  (line 282)
- [x] int number_finalizers_run  (line 287)
- [x] bool gc_in_progress  (line 292)
- [x] static ptrdiff_t hash_table_allocated_bytes  (line 326, static)
- [x] static char spare_memory  (line 332, static)
- [x] intptr_t garbage_collection_inhibited  (line 341)
- [x] static EMACS_INT gc_threshold  (line 345, static)
- [x] const char pending_malloc_warning  (line 350)
- [x] static char stack_copy  (line 361, static)
- [x] static ptrdiff_t stack_copy_size  (line 362, static)
- [x] enum mem_type allocated_mem_type  (line 433)
- [x] static struct mem_node mem_root  (line 482, static)
- [x] static void min_heap_address  (line 486, static)
- [x] static void max_heap_address  (line 486, static)
- [x] static struct mem_node mem_z  (line 490, static)
- [x] int staticidx  (line 507)
- [x] struct Lisp_Finalizer finalizers  (line 552)
- [x] struct Lisp_Finalizer doomed_finalizers  (line 558)
- [x] extern void lisp_malloc_loser  (line 884)
- [x] static struct ablock free_ablock  (line 1070, static)
- [x] static struct interval_block interval_block  (line 1253, static)
- [x] static int interval_block_index  (line 1258, static)
- [x] static INTERVAL interval_free_list  (line 1262, static)
- [x] static struct sblock oldest_sblock  (line 1464, static)
- [x] static struct sblock current_sblock  (line 1464, static)
- [x] static struct sblock large_sblocks  (line 1468, static)
- [x] static struct string_block string_blocks  (line 1472, static)
- [x] static struct Lisp_String string_free_list  (line 1476, static)
- [x] static int check_string_bytes_count  (line 1603, static)
- [x] static struct float_block float_block  (line 2494, static)
- [x] static int float_block_index  (line 2498, static)
- [x] static struct Lisp_Float float_free_list  (line 2502, static)
- [x] static struct cons_block cons_block  (line 2589, static)
- [x] static int cons_block_index  (line 2593, static)
- [x] static struct Lisp_Cons cons_free_list  (line 2597, static)
- [x] static struct vector_block vector_blocks  (line 2879, static)
- [x] static struct Lisp_Vector vector_free_lists  (line 2890, static)
- [x] static ptrdiff_t last_inserted_vector_free_idx  (line 2895, static)
- [x] static struct large_vector large_vectors  (line 2899, static)
- [x] Lisp_Object zero_vector  (line 2903)
- [x] static struct symbol_block symbol_block  (line 3672, static)
- [x] static int symbol_block_index  (line 3673, static)
- [x] static struct Lisp_Symbol symbol_free_list  (line 3677, static)
- [x] static bool setjmp_tested_p  (line 5092, static)
- [x] static int longjmps_done  (line 5093, static)
- [x] static struct Lisp_Hash_Table weak_hash_tables  (line 5702, static)
- [x] extern Lisp_Object last_marked  (line 6165)
- [x] static int last_marked_index  (line 6167, static)
- [x] static struct mark_stack mark_stk  (line 6385, static)
- [x] bool suppress_checking  (line 7346)

## Decisions

- **The file is unverified, so both arms of every `#if` are carried.**
  `python3 tools/check-cond-eval.py src/alloc.c` reports `138 conditions, 0 unknown,
  4 disagreements [NOT VERIFIED]`: the frozen macro table has no `USE_LSB_TAG`, which `lisp.h`
  requires to be 1, so the four `! USE_LSB_TAG` conditions read the other way round for the
  evaluator than for clang.  `tools/extract-abi.py` therefore keeps *both* arms live and the
  per-function token check demands the text of the arms the frozen build does not compile
  (docs/status.md 3.49; `term.rs`/`font.rs` are the precedents).  The live arms are ordinary
  text; the dead ones are carried as `#[cfg (feature = "…")]` text and no such feature is ever
  set.  Which arm is dead was **measured with clang**, not read off the evaluator: the file's
  166 conditional lines were probed with `clang -dM -E` against `build/emacs-ref/src/config.h`
  (one `#define EMACS_LIVE_<n>` marker per branch; the same probe `check-cond-eval.py` uses).
  The dead features, in C's order: `have_malloc_h`, `have_window_system`, `have_android`,
  `have_tree_sitter`, `have_linux_sysinfo`, `msdos`, `windowsnt`, `use_gtk`, `have_pgtk`,
  `have_haiku`, `have_x_windows`, `have_ns`, `have_text_conversion`, `enable_checking`,
  `use_valgrind`, `address_sanitizer`, `doug_lea_malloc`, `posix_memalign`,
  `gc_malloc_check`, `gc_check_string_bytes`, `gc_check_string_overrun`,
  `gc_check_string_free_list`, `gc_asan_poison_objects`, `gc_mark_secondary_stack`,
  `gc_cdr_count`, `have_modules`, `have_native_comp`, `have_malloc_trim`, `use_lsb_tag`,
  `sigdanger`, `i386`, `rel_alloc`, `no_use_aligned_alloc` and `no_have_builtin_unwind_init`.
  Live: `HAVE_PTHREAD`, `HAVE_PDUMPER`, `MAX_SAVE_STACK > 0`, `HAVE___BUILTIN_UNWIND_INIT`,
  `GC_REMEMBER_LAST_MARKED`, `GC_STRING_EXTRA` and C 7620's `__GNUC__`.
- **`#if 0` regions** (C 176-186 inside `union emacs_align_type`, C 1409-1410 in `sdata`) are
  dead in every configuration and are recorded here, not transcribed (docs/status.md 3.55).
- **`struct emacs_globals globals` (C 278).**  `globals.h` is generated by `make-docfile`, has
  no mirror, and no reader in this tree indexes `globals`; the port declares the record at its
  use site and defines the object as the opaque cell C needs for the symbol.  Each `V*` this
  unit owns is the separate `#[unsafe(no_mangle)] pub static mut` of §3.29's shape ①, which is
  what every other unit's ledger records; §3.29 ④'s field-access model needs the `globals.h`
  mirror first (integrator's call).
- **Dead-arm text that the gates demand and the ledger therefore carries**: the two
  `SETJMP_WILL_*` messages and `test_setjmp`, `verify_alloca`, `malloc_initialize_hook` and
  `__malloc_initialize_hook`, `pointer_align`, `aligned_alloc`/`my_aligned_alloc`,
  `string_bytes`/`check_sblock`/`check_string_bytes`/`check_string_free_list`,
  `android_make_lisp_symbol`, the window-system `compact_font_cache_entry`/
  `compact_font_caches`, `Fmemory_info`'s three platform arms, `Fmalloc_info`/`Fmalloc_trim`,
  `Fgarbage_collect_heapsize`'s tenth entry, `die`/`suppress_checking`,
  `allocated_mem_type`, `lisp_malloc_loser`, and the GC_ASAN_POISON_OBJECTS family.
- **`.unwrap ()` on a function-pointer *call* is C's own dereference.**  The seven sites
  (`valid_font_driver`-checked `drv->close_font (font)`, `uptr->finalizer (uptr->p)`,
  `flush_stack_call_func1`'s `func (arg)`, the four `visitor.visit (…)`/`livep (…)` calls in
  `visit_vectorlike_root`/`visit_static_gc_roots`/`check_live`) are C's direct call of a
  pointer C assumes non-null; the port's `Option<…>` declaration makes the call explicit and
  panics where C would have crashed (the frame.rs/term.rs convention).  No other `.unwrap ()`
  appears in the file.
- **Two C namespaces, one Rust type namespace.**  C's `struct sdata` (the nonnull-string
  layout) and the union *typedef* `sdata` (the type of `sblock.next_free` and `sblock.data`)
  are one spelling in C and cannot be in Rust.  The port keeps `sdata` for the struct (which
  the `SDATA_DATA` macro casts to, keeping C's token) and names the union **`sdata_union`**;
  the record is pinned by `const _` assertions against clang's measured `sizeof 8` (struct)
  and `sizeof 16, alignof 8` (union).  Three zero-sized markers reproduce the extractor's
  flattened field view without moving a field (the `charset.rs` precedent):
  `struct sdata.nbytes` (C 1416-1418's `#ifdef GC_CHECK_STRING_BYTES` member, dead here),
  `struct ablock.padding` (C 1029's `#if BLOCK_PADDING`, zero) and `struct mem_node.MEM_BLACK`
  (C's anonymous `enum {MEM_BLACK, MEM_RED} color` reports its first member as a field).
- **Layouts.**  Every record this unit defines is `#[repr(C)]` and pinned with `const _`
  assertions against the frozen clang's own numbers (measured by compiling a TU that
  `#include "alloc.c"` and dumping its record layouts): `emacs_align_type` 568/8, `gcstat` 128,
  `mem_node` 48 (`color`@40, `type`@44), `ablock` 32768, `ablocks` 524288, `interval_block`
  1016, `sdata` 8 (`data`@8), `sdata_union` 16/8, `sblock` 16, `string_block` 1000,
  `float_block` 32760, `cons_block` 32744, `vector_block` 4096, `symbol_block` 1016,
  `mark_entry` 16, `mark_stack` 24, plus the derived constants `BLOCK_BYTES` 32760,
  `VECTOR_BLOCK_BYTES` 4088, `VBLOCK_BYTES_MIN` 16, `VBLOCK_BYTES_MAX` 2040,
  `VECTOR_FREE_LIST_ARRAY_SIZE` 256, `FLOAT_BLOCK_SIZE` 4031, `CONS_BLOCK_SIZE` 2030,
  `SYMBOL_BLOCK_SIZE` 21, `INTERVAL_BLOCK_SIZE` 18, `STRING_BLOCK_SIZE` 31,
  `MALLOC_ALIGNMENT` 16, `SBLOCK_SIZE` 8184, `VECTOR_ELTS_MAX` 1152921504606846973 and
  `memory_full_cons_threshold` 32744.
- **Use-site declarations (mirror gaps, P-72 ③; no mirror was edited).**  `emacs/src/alloc.h`
  **does not exist in the reference checkout** (checked), so there is no such mirror to
  report; the names C declares in `lisp.h` are in the `lisp.rs` mirror
  (`valid_lisp_object_p`, `consing_until_gc`, `build_symbol_with_pos`, …).  Declared at their
  use site here, each with a `/// C:` line: `struct font`, `struct font_driver` (both in the
  *mirror* `include/font.rs` are WIP — they exist in `femacs/src/font.rs`, the font *unit*,
  which cannot be included), `struct face`, `struct face_cache`, `FRAME_FACE_CACHE`,
  `FACE_FROM_ID_OR_NULL` (frame.h 1656), `valid_font_driver`, `aligned_alloc` (lisp.h 4671),
  `putenv`, `malloc_probe` (profiler.h), `parse_str_as_multibyte` (character.h 1046),
  `record_unwind_protect_void` (lisp.h 5619), `record_unwind_protect_intmax`,
  `internal_condition_case_1` (eval.c 2255), `sweep_weak_table` (lisp.h 4246), `fatal`
  (emacs.c), `add_to_log` (print.c), `evxprintf` (lisp.h 4722), `push_message` /
  `pop_message_unwind` / `restore_message` (xdisp.c), `shrink_regexp_cache` (regex-emacs.c),
  `mark_lread`, `mark_kboards`, `mark_threads` / `unmark_main_thread`, `mark_profiler`,
  `mark_fns`, `safe_run_hooks` (eval.c), `emacs_pipe` (emacs.c), `emacs_write` (sysdep.c),
  `UINTPTR_MAX` (`<limits.h>`, missing from `lib/include/limits.in.rs`) and the macro
  `FUNCTIONP` (lisp.h 3772).
- **Two mirror declarations sit at *crate root* and forced a module.**  `include/intervals.rs`
  declares `make_interval` at crate root and `include/buffer.rs` declares `build_overlay`
  there ("Declared in alloc.c"), so the two definitions live in the glob-imported modules
  `alloc_defs` / `alloc_defs_overlay`: the crate-root declaration shadows the import while the
  definition still emits `_make_interval` / `_build_overlay` (PORTING.md 0.1-10/P-21, the
  `term.rs` `term_defs` shape).  Both are **mirror gaps for the integrator**: those two
  declarations belong in the mirrors' own `*_decls` modules when the next commit touches them.
- **`call0` needs a C-ABI thunk.**  C's `call0` is a C-ABI inline function and is passed to
  `internal_condition_case_1` as a pointer; the port's `lisp.rs` `call0` is Rust-ABI, so
  `run_finalizer_function` passes `call0_thunk`, a two-line `unsafe extern "C"` wrapper.
- **`defsubr` liveness is decided with the frozen table, not blindly.**  C's
  `defsubr (&Smalloc_info)` and `defsubr (&Smalloc_trim)` live in dead arms; the extractor's
  `c_defsubr_live` consults the frozen macro table even for a blind file (the font.rs rule),
  so the port does **not** transcribe those two calls — the subr *records* (`Fmalloc_info` /
  `Fmalloc_trim`) are carried, since the subr table is read blind.
- **Extractor quirk (recorded, not worked around in the tool).**  `cinventory.scan_const_value`
  reads a `\`-continued string literal's continuations as single spaces, so the value it
  records for `SETJMP_WILL_LIKELY_WORK`/`SETJMP_WILL_NOT_WORK` has a leading space and a space
  after every `\n` (C's translation phase 2 splices them away).  The constant gate compares
  bytes, so the port spells the bytes the gate records, and the constants are typed
  `&CStr` (not `*const c_char`) so that the literal is the whole initialiser — a trailing
  `.as_ptr ()` would be read as part of the value.  Both are dead-arm text used only by the
  dead `test_setjmp`.  **For the integrator**: this is an extractor gap, not a port error.
- **Spelling deviations recorded in `docs/ports/macro-body-notes.tsv`** (one row each, with
  the reason): `MALLOC_PROBE`, `PSEUDOVEC_STRUCT`, `SDATA_OF_STRING`, `check_string_bytes`,
  `compact_font_caches`, `verify_alloca`, `LISP_ALIGNMENT`, `MALLOC_ALIGNMENT`,
  `MALLOC_IS_LISP_ALIGNED`, `aligned_alloc`, `make_formatted_string`, `listn`,
  `cleanup_vector`, `init_symbol`, `set_symbol_marked`, `build_marker`,
  `run_finalizer_function`, `mark_maybe_pointer`, `valid_lisp_object_p`, `survives_gc_p`,
  `process_mark_stack`, `garbage_collect`, `sweep_buffers`, `which_symbols`, `syms_of_alloc`.
  They cover C's `switch`/`case`/`default`/`break`/`goto` → `match`/labeled loops, the
  `do…while (0)` statement macros, `va_list`/`va_start`/`va_end` → the C-variadic `…`
  parameter (P-04), `alloca` → P-05's stack pool, `FOR_EACH_LIVE_BUFFER` → the P-22 hand
  expansion, the `bool_bf` fields the mirrors pack into `bits` (P-80), `verify_expr`/
  `ATTRIBUTE_MAY_ALIAS` (P-13), and the clang-measured constant folds.
  `docs/ports/va-arg-types.tsv` gained the `Lisp_Object` row for `cons_listn`'s read.
  `docs/ports/line-drift-tolerance.tsv` gained the `alloc` row (0.08, `total` basis): carrying
  both arms of 166 conditional lines costs the `#[cfg]` attribute, the braces and the comments
  C does not spell.
- **What the gates cannot see.**  (a) `check-linkage.py` flags `_allocated_mem_type` and
  `_suppress_checking` as C objects the port does not export — both are inside dead arms
  (`#ifdef GC_MALLOC_CHECK` / `#ifdef ENABLE_CHECKING`), and the tool itself asks for triage on
  an unverified file; the port defines both under the matching `#[cfg]`.  (b)
  `check-undef.py`: this unit **adds six new orphans** (`_mark_fns`, `_mark_kboards`,
  `_push_message`, `_pop_message_unwind`, `_restore_message`, `_sweep_weak_table`), each a
  dependency on a unit that is not ported yet (fns.c, keyboard.c, xdisp.c); it also makes 11
  ratchet rows stale (`_Vgc_cons_percentage`, `_Vpost_gc_hook`, `_Vpurify_flag`, …), which the
  unit now defines.  The ratchet (`docs/baseline/undef-pending.txt`) is the integrator's file:
  `--write-pending` is what records both movements.  (c) `layout-check` has no assertions for
  this file's records (the frozen `c-layout-darwin-arm64.tsv` was generated from a header list
  that does not reach `alloc.c`), so the assertions above are this unit's own clang
  measurements.  (d) The GC's semantics — the mark stack, the red-black tree, the sweepers, the
  weak-table protocol — are *not* exercised by any gate; the tests that would are the ones
  `docs/status.md` P-30 asks for.
- **D7 call relationships.**  Measured with `clang -c -std=gnu23` of `emacs/src/alloc.c`
  against the frozen configuration vs `nm -u build/femacs/alloc.o`: **98 undefined references
  in C, 103 in the port (96 after the seven `__RNv*` Rust panic symbols), 93 shared.**  Both
  C-only differences and port-only differences, and nothing else:
  * **C-only.**  `___chkstk_darwin`/`___stack_chk_fail`/`___stack_chk_guard` are clang's
    codegen support (stack probes and stack protector); `_FUNCTIONP` is `lisp.h` 4799's
    out-of-line `extern bool FUNCTIONP (Lisp_Object)`, which the port spells as the
    `FUNCTIONP` macro (no symbol); `_timespectod` is systime.c's out-of-line function, while
    the port's `lib/include/timespec.rs` copy is an inline `fn`, so the port calls its own.
  * **Port-only.**  `_Ffboundp`, `_Vquit_flag`, `_cons_to_signed`, `_cons_to_unsigned`,
    `_current_buffer`, `_specpdl`, `_specpdl_end`, `_specpdl_ptr` and
    `_symbols_with_pos_enabled` come from the *mirror* helpers that this translation unit
    compiles (the `EQ`/`NILP`/`make_int` and `specpdl`/`ConsToInteger` helpers in `lisp.rs`):
    C spells those as macros or inline functions, so its object has no such reference, while
    `--crate-type=lib` emits every `pub` helper of the included mirrors.  `__setjmp` is
    `lisp.rs`'s `pub unsafe fn sys_setjmp`, whose body calls `_setjmp`; C's alloc.o never
    reaches it (`HAVE___BUILTIN_UNWIND_INIT` is defined, so the `#ifndef` arm is dead).
  * No callee of C's source text is missing from the port's set, and the only *renamed*
    callee is `free` → `rpl_free` (above), which the port spells the same way.
