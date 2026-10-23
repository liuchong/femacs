# emacs/src/lisp.h (5924 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 275
- [x] will_dump_p  (line 650)
- [x] will_bootstrap_p  (line 660)
- [x] will_dump_with_pdumper_p  (line 670)
- [x] dumped_with_pdumper_p  (line 680)
- [x] XLI  (line 701)
- [x] XIL  (line 707)
- [x] XLP  (line 713)
- [x] XTYPE  (line 721)
- [x] TAGGEDP  (line 735)
- [x] CHECK_TYPE  (line 741)
- [x] PSEUDOVECTORP  (line 1054)
- [x] BARE_SYMBOL_P  (line 1063)
- [x] SYMBOL_WITH_POS_P  (line 1069)
- [x] SYMBOLP  (line 1075)
- [x] XSYMBOL_WITH_POS  (line 1082)
- [x] XSYMBOL_WITH_POS_SYM  (line 1089)
- [x] XSYMBOL_WITH_POS_POS  (line 1097)
- [x] maybe_remove_pos_from_symbol  (line 1103)
- [x] XBARE_SYMBOL  (line 1110)
- [x] XSYMBOL  (line 1119)
- [x] make_lisp_symbol_internal  (line 1131)
- [x] make_lisp_symbol  (line 1143)
- [x] builtin_lisp_symbol  (line 1151)
- [x] c_symbol_p  (line 1157)
- [x] CHECK_SYMBOL  (line 1171)
- [x] make_fixnum  (line 1184)
- [x] XFIXNUM_RAW  (line 1191)
- [x] make_ufixnum  (line 1197)
- [x] make_fixnum  (line 1211)
- [x] XFIXNUM_RAW  (line 1233)
- [x] make_ufixnum  (line 1245)
- [x] FIXNUMP  (line 1263)
- [x] XFIXNUM  (line 1269)
- [x] XUFIXNUM_RAW  (line 1277)
- [x] XUFIXNUM  (line 1283)
- [x] XHASH  (line 1291)
- [x] make_fixed_natnum  (line 1298)
- [x] BASE_EQ  (line 1307)
- [x] EQ  (line 1315)
- [x] clip_to_bounds  (line 1326)
- [x] make_lisp_ptr  (line 1334)
- [x] dead_object  (line 1353)
- [x] XFIXNUMPTR  (line 1401)
- [x] make_pointer_integer_unsafe  (line 1407)
- [x] make_pointer_integer  (line 1414)
- [x] NILP  (line 1449)
- [x] CONSP  (line 1455)
- [x] CHECK_CONS  (line 1461)
- [x] XCONS  (line 1467)
- [x] xcar_addr  (line 1481)
- [x] xcdr_addr  (line 1486)
- [x] XCAR  (line 1494)
- [x] XCDR  (line 1500)
- [x] XSETCAR  (line 1510)
- [x] XSETCDR  (line 1515)
- [x] CAR  (line 1522)
- [x] CDR  (line 1531)
- [x] CAR_SAFE  (line 1542)
- [x] CDR_SAFE  (line 1547)
- [x] STRINGP  (line 1582)
- [x] CHECK_STRING  (line 1588)
- [x] XSTRING  (line 1594)
- [x] STRING_MULTIBYTE  (line 1602)
- [x] STRING_SET_MULTIBYTE  (line 1635)
- [x] SDATA  (line 1653)
- [x] SSDATA  (line 1658)
- [x] SREF  (line 1664)
- [x] SSET  (line 1669)
- [x] SCHARS  (line 1674)
- [x] STRING_BYTES  (line 1685)
- [x] SBYTES  (line 1697)
- [x] STRING_SET_CHARS  (line 1702)
- [x] CHECK_STRING_NULL_BYTES  (line 1713)
- [x] string_immovable_p  (line 1721)
- [x] VECTORLIKEP  (line 1735)
- [x] XVECTOR  (line 1741)
- [x] ASIZE  (line 1748)
- [x] gc_asize  (line 1756)
- [x] PVSIZE  (line 1763)
- [x] VECTORP  (line 1769)
- [x] CHECK_VECTOR  (line 1775)
- [x] PSEUDOVECTOR_TYPE  (line 1784)
- [x] PSEUDOVECTOR_TYPEP  (line 1794)
- [x] bool_vector_words  (line 1852)
- [x] bool_vector_bytes  (line 1859)
- [x] bits_word_to_host_endian  (line 1866)
- [x] BOOL_VECTOR_P  (line 1890)
- [x] CHECK_BOOL_VECTOR  (line 1896)
- [x] XBOOL_VECTOR  (line 1902)
- [x] bool_vector_size  (line 1909)
- [x] bool_vector_data  (line 1917)
- [x] bool_vector_uchar_data  (line 1923)
- [x] bool_vector_bitref  (line 1931)
- [x] bool_vector_ref  (line 1940)
- [x] bool_vector_set  (line 1948)
- [x] AREF  (line 1964)
- [x] aref_addr  (line 1971)
- [x] ASET  (line 1978)
- [x] gc_aset  (line 1985)
- [x] memclear  (line 2004)
- [x] ASCII_CHAR_P  (line 2028)
- [x] CHAR_TABLE_P  (line 2085)
- [x] XCHAR_TABLE  (line 2091)
- [x] SUB_CHAR_TABLE_P  (line 2119)
- [x] XSUB_CHAR_TABLE  (line 2125)
- [x] CHAR_TABLE_REF_ASCII  (line 2132)
- [x] CHAR_TABLE_REF  (line 2150)
- [x] CHAR_TABLE_SET  (line 2160)
- [x] SUBRP  (line 2216)
- [x] XSUBR  (line 2222)
- [x] VALID_DOCSTRING_P  (line 2232)
- [x] CHAR_TABLE_EXTRA_SLOTS  (line 2263)
- [x] SYMBOL_VAL  (line 2298)
- [x] SYMBOL_ALIAS  (line 2305)
- [x] SYMBOL_BLV  (line 2311)
- [x] SYMBOL_FWD  (line 2317)
- [x] SET_SYMBOL_VAL  (line 2324)
- [x] SET_SYMBOL_ALIAS  (line 2331)
- [x] SET_SYMBOL_BLV  (line 2337)
- [x] SET_SYMBOL_FWD  (line 2343)
- [x] SYMBOL_NAME  (line 2350)
- [x] SYMBOL_INTERNED_P  (line 2358)
- [x] SYMBOL_INTERNED_IN_INITIAL_OBARRAY_P  (line 2366)
- [x] SYMBOL_TRAPPED_WRITE_P  (line 2376)
- [x] SYMBOL_CONSTANT_P  (line 2387)
- [x] OBARRAYP  (line 2411)
- [x] XOBARRAY  (line 2417)
- [x] CHECK_OBARRAY  (line 2424)
- [x] make_lisp_obarray  (line 2430)
- [x] obarray_size  (line 2437)
- [x] check_obarray  (line 2446)
- [x] make_obarray_iter  (line 2461)
- [x] obarray_iter_at_end  (line 2469)
- [x] obarray_iter_step  (line 2488)
- [x] obarray_iter_symbol  (line 2495)
- [x] hash_unused_entry_key_p  (line 2658)
- [x] HASH_TABLE_P  (line 2664)
- [x] XHASH_TABLE  (line 2670)
- [x] make_lisp_hash_table  (line 2677)
- [x] HASH_KEY  (line 2685)
- [x] HASH_VALUE  (line 2693)
- [x] HASH_HASH  (line 2701)
- [x] HASH_TABLE_SIZE  (line 2709)
- [x] hash_table_index_size  (line 2716)
- [x] hash_from_key  (line 2723)
- [x] sxhash_combine  (line 2772)
- [x] SXHASH_REDUCE  (line 2780)
- [x] reduce_emacs_uint_to_hash_hash  (line 2787)
- [x] knuth_hash  (line 2797)
- [x] make_mint_ptr  (line 2893)
- [x] mint_ptrp  (line 2900)
- [x] xmint_pointer  (line 2906)
- [x] FINALIZERP  (line 2951)
- [x] XFINALIZER  (line 2957)
- [x] MARKERP  (line 2964)
- [x] XMARKER  (line 2970)
- [x] OVERLAYP  (line 2977)
- [x] XOVERLAY  (line 2983)
- [x] USER_PTRP  (line 2990)
- [x] XUSER_PTR  (line 2996)
- [x] SQLITEP  (line 3003)
- [x] SQLITE  (line 3009)
- [x] CHECK_SQLITE  (line 3015)
- [x] XSQLITE  (line 3021)
- [x] BIGNUMP  (line 3028)
- [x] INTEGERP  (line 3034)
- [x] make_int  (line 3041)
- [x] make_uint  (line 3046)
- [x] XFWDTYPE  (line 3136)
- [x] BUFFER_OBJFWDP  (line 3142)
- [x] XBUFFER_OFFSET  (line 3148)
- [x] KBOARD_OBJFWDP  (line 3155)
- [x] FLOATP  (line 3174)
- [x] XFLOAT  (line 3180)
- [x] XFLOAT_DATA  (line 3187)
- [x] FIXNATP  (line 3239)
- [x] XFIXNAT  (line 3246)
- [x] NUMBERP  (line 3256)
- [x] RANGED_FIXNUMP  (line 3262)
- [x] AUTOLOADP  (line 3273)
- [x] WINDOW_CONFIGURATIONP  (line 3282)
- [x] CLOSUREP  (line 3288)
- [x] FRAMEP  (line 3294)
- [x] RECORDP  (line 3300)
- [x] CHECK_RECORD  (line 3306)
- [x] IMAGEP  (line 3313)
- [x] ARRAYP  (line 3320)
- [x] CHECK_LIST  (line 3326)
- [x] CHECK_LIST_END  (line 3332)
- [x] CHECK_FIXNUM  (line 3338)
- [x] CHECK_STRING_CAR  (line 3344)
- [x] CHECK_VECTOR_OR_STRING  (line 3350)
- [x] CHECK_ARRAY  (line 3359)
- [x] CHECK_FIXNAT  (line 3364)
- [x] XFLOATINT  (line 3370)
- [x] CHECK_NUMBER  (line 3378)
- [x] CHECK_INTEGER  (line 3384)
- [x] CHECK_SUBR  (line 3390)
- [x] call0  (line 3467)
- [x] wrap_specpdl_ref  (line 3653)
- [x] unwrap_specpdl_ref  (line 3664)
- [x] specpdl_count_to_ref  (line 3674)
- [x] specpdl_ref_to_count  (line 3680)
- [x] specpdl_ref_eq  (line 3687)
- [x] specpdl_ref_lt  (line 3694)
- [x] specpdl_ref_valid_p  (line 3700)
- [x] make_invalid_specpdl_ref  (line 3706)
- [x] specpdl_ref_add  (line 3714)
- [x] specpdl_ref_to_ptr  (line 3721)
- [x] SPECPDL_INDEX  (line 3728)
- [x] backtrace_debug_on_exit  (line 3734)
- [x] grow_specpdl  (line 3752)
- [x] record_in_backtrace  (line 3760)
- [x] maybe_quit  (line 3897)
- [x] rarely_quit  (line 3908)
- [x] xvector_contents_addr  (line 3944)
- [x] xvector_contents  (line 3957)
- [x] vcopy  (line 3965)
- [x] set_hash_key_slot  (line 3975)
- [x] set_hash_value_slot  (line 3982)
- [x] set_symbol_function  (line 3992)
- [x] set_symbol_plist  (line 3998)
- [x] set_symbol_next  (line 4004)
- [x] make_symbol_constant  (line 4010)
- [x] blv_found  (line 4018)
- [x] set_overlay_plist  (line 4027)
- [x] string_intervals  (line 4035)
- [x] set_string_intervals  (line 4043)
- [x] set_char_table_defalt  (line 4052)
- [x] set_char_table_purpose  (line 4057)
- [x] set_char_table_extras  (line 4065)
- [x] set_char_table_contents  (line 4072)
- [x] set_sub_char_table_contents  (line 4079)
- [x] integer_to_intmax  (line 4098)
- [x] integer_to_uintmax  (line 4113)
- [x] elogb  (line 4130)
- [x] modiff_incr  (line 4143)
- [x] modiff_to_integer  (line 4157)
- [x] flush_stack_call_func  (line 4465)
- [x] list1i  (line 4514)
- [x] list2i  (line 4520)
- [x] list3i  (line 4526)
- [x] list4i  (line 4532)
- [x] vector_nbytes  (line 4548)
- [x] build_unibyte_string  (line 4556)
- [x] build_string  (line 4574)
- [x] make_uninit_vector  (line 4598)
- [x] make_uninit_sub_char_table  (line 4606)
- [x] make_nil_vector  (line 4621)
- [x] LOADHIST_ATTACH  (line 4735)
- [x] intern  (line 4757)
- [x] intern_c_string  (line 4763)
- [x] xsignal  (line 4788)
- [x] CHECK_KEYWORD_ARGS  (line 4850)
- [x] MODULE_FUNCTIONP  (line 4873)
- [x] XMODULE_FUNCTION  (line 4879)
- [x] fast_string_match  (line 5005)
- [x] fast_string_match_ignore_case  (line 5011)
- [x] fast_c_string_match  (line 5017)
- [x] fast_c_string_match_ignore_case  (line 5024)
- [x] get_act_rec  (line 5234)
- [x] set_act_rec  (line 5240)
- [x] maybe_disable_address_randomization  (line 5277)
- [x] NATIVE_COMP_FUNCTIONP  (line 5492)
- [x] NATIVE_COMP_FUNCTION_DYNP  (line 5498)
- [x] SUBR_TYPE  (line 5504)
- [x] allocate_native_comp_unit  (line 5510)
- [x] NATIVE_COMP_FUNCTIONP  (line 5517)
- [x] NATIVE_COMP_FUNCTION_DYNP  (line 5523)
- [x] lispstpcpy  (line 5561)
- [x] __lsan_ignore_object  (line 5574)
- [x] egetenv  (line 5584)
- [x] safe_free  (line 5661)
- [x] safe_free_unbind_to  (line 5689)
- [x] maybe_gc  (line 5916)

## Macros defined — 201
- [x] ALLOCATE_PLAIN_PSEUDOVECTOR  (line 4632, body 71cfd554321c)
- [x] ALLOCATE_PSEUDOVECTOR  (line 4638, body d1d2c38d275a)
- [x] ALLOCATE_ZEROED_PSEUDOVECTOR  (line 4646, body 2161819fffc4)
- [x] ARRAY_MARK_FLAG  (line 967, body 68e510603696)
- [x] AUTO_CONS  (line 5771, body 071de34380a1)
- [x] AUTO_CONS_EXPR  (line 5763, body fa303065f904)
- [x] AUTO_LIST1  (line 5772, body 9563b5d4451d)
- [x] AUTO_LIST2  (line 5774, body 8b43e2d35d4e)
- [x] AUTO_LIST3  (line 5778, body 5d7c2912e10b)
- [x] AUTO_LIST4  (line 5782, body 041ece12371f)
- [x] AUTO_STR  (line 5813, body e88cf05c7a7e)
- [x] AUTO_STRING  (line 5796, body ad430022735d)
- [x] AUTO_STRING_WITH_LEN  (line 5806, body f8c79cb941ca)
- [x] AUTO_STR_WITH_LEN  (line 5819, body 52b86fcdc3fb)
- [x] AVAIL_ALLOCA  (line 5625, body e5e1b4f42db5)
- [x] AVOID  (line 153, body 671cea88212a)
- [x] BARE_SYMBOL_P  (line 442, body 473d5db71e75)
- [x] BASE_EQ  (line 447, body c2d4e54a31a6)
- [x] BITS_WORD_MAX  (line 140, body 6e5db472423f)
- [x] BOOL_VECTOR_BITS_PER_CHAR  (line 126, body fe5dbbcea5ce)
- [x] BOOL_VECTOR_LENGTH_MAX  (line 1840, body 7eae29123b5f)
- [x] CALLMANY  (line 3453, body fb473b914364)
- [x] CALLN  (line 3461, body 01e37d1d4236)
- [x] CHECK_FIXNUM  (line 443, body b97322f311c1)
- [x] CHECK_SYMBOL  (line 444, body ce21af9ae071)
- [x] CHECK_TYPE  (line 445, body 00c28f470b04)
- [x] CHILD_SETUP_ERROR_DESC  (line 5208, body 24c362c7debd)
- [x] CONSP  (line 446, body 2a16439e3220)
- [x] CONS_TO_INTEGER  (line 4188, body 43dc623e3a94)
- [x] DAEMON_RUNNING  (line 5166, body 465f115d4396)
- [x] DECLARE_GDB_SYM  (line 58, body 5c34c70be31e)
- [x] DEFINE_GDB_SYMBOL_BEGIN  (line 64, body 52a4d286fe58)
- [x] DEFINE_GDB_SYMBOL_END  (line 65, body 2d14ab97cc3d)
- [x] DEFINE_KEY_OPS_AS_MACROS  (line 434, body 7cb6efb98ba5)
- [x] DEFINE_LISP_SYMBOL  (line 883, body 68e47b6d932f)
- [x] DEFINE_NON_NIL_Q_SYMBOL_MACROS  (line 896, body 5ffe533b830f)
- [x] DEFSYM  (line 2394, body da39a3ee5e6b)
- [x] DEFUN  (line 3434, body fe55eaa09712)
- [x] DEFUN_ARGS_0  (line 842, body cbf9e406f246)
- [x] DEFUN_ARGS_1  (line 843, body eda9e3284475)
- [x] DEFUN_ARGS_2  (line 844, body 089c483a7459)
- [x] DEFUN_ARGS_3  (line 845, body bf3ff48f3ccc)
- [x] DEFUN_ARGS_4  (line 846, body 5d3a578ef9f7)
- [x] DEFUN_ARGS_5  (line 847, body 420e9b6831a3)
- [x] DEFUN_ARGS_6  (line 849, body a7e52c5d7c27)
- [x] DEFUN_ARGS_7  (line 851, body 2409e1a4a591)
- [x] DEFUN_ARGS_8  (line 853, body 279db0e33c2e)
- [x] DEFUN_ARGS_MANY  (line 840, body a681b281b194)
- [x] DEFUN_ARGS_UNEVALLED  (line 841, body eda9e3284475)
- [x] DEFVAR_BOOL  (line 3507, body 0c7a673f5b24)
- [x] DEFVAR_INT  (line 3513, body f9daaf5f7a90)
- [x] DEFVAR_KBOARD  (line 3519, body e3ba9fb5bf44)
- [x] DEFVAR_LISP  (line 3495, body 790a2191a477)
- [x] DEFVAR_LISP_NOPRO  (line 3501, body 29372037c487)
- [x] DOHASH  (line 2731, body c48776d4dca9)
- [x] DOHASH_SAFE  (line 2756, body bca7853b36c9)
- [x] DOOBARRAY  (line 2504, body c4bf199c066d)
- [x] EMACS_INT_MAX  (line 103, body 964b7883e05a)
- [x] EMACS_LISP_H  (line 21, body da39a3ee5e6b)
- [x] ENUM_BF  (line 482, body 22415dc4275d)
- [x] EXFUN  (line 835, body 7c434adb9381)
- [x] FIXNUMP  (line 449, body fbf79bcee64e)
- [x] FIXNUM_OVERFLOW_P  (line 1178, body fb2e0e798052)
- [x] FLOATP  (line 448, body 43e2ef70bec4)
- [x] FLOAT_TO_STRING_BUFSIZE  (line 4709, body 89a1c105a472)
- [x] FOR_EACH_ALIST_VALUE  (line 5908, body f219ec1aa28e)
- [x] FOR_EACH_TAIL  (line 5839, body 438a08a27348)
- [x] FOR_EACH_TAIL_BASIC  (line 5876, body 5d0718b6c633)
- [x] FOR_EACH_TAIL_INTERNAL  (line 5871, body cb0bd049a834)
- [x] FOR_EACH_TAIL_SAFE  (line 5845, body ce8a1057f531)
- [x] FOR_EACH_TAIL_STEP_CYCLEP  (line 5899, body c1ab2c38e2e6)
- [x] FOR_EACH_TAIL_THRESHOLD  (line 5886, body 6124cb94c33d)
- [x] GCALIGNED  (line 309, body 078a056b39b0)
- [x] GCALIGNED_STRUCT  (line 307, body da39a3ee5e6b)
- [x] GCALIGNED_UNION_MEMBER  (line 303, body 215eb87b91ae)
- [x] GCALIGNMENT  (line 280, body 356a192b7913)
- [x] GCTYPEBITS  (line 76, body 77de68daecd8)
- [x] HASH_UNUSED_ENTRY_KEY  (line 2654, body 303d8c67fb5f)
- [x] HAVE_EXT_MENU_BAR  (line 3933, body 5ffe533b830f)
- [x] HAVE_EXT_TOOL_BAR  (line 3938, body 5ffe533b830f)
- [x] IDEAL_GCALIGNMENT  (line 243, body fe5dbbcea5ce)
- [x] IEEE_FLOATING_POINT  (line 3201, body 05c9460d5cba)
- [x] INTMASK  (line 474, body 46ceaec96a15)
- [x] INTTYPEBITS  (line 231, body aac19e828c18)
- [x] INT_TO_INTEGER  (line 3052, body bdeacbd43218)
- [x] INVALID_LISP_VALUE  (line 2651, body 0e0deb978c4e)
- [x] IS_DAEMON  (line 5165, body dfeb28da6908)
- [x] LISPSYM_INITIALLY  (line 876, body fe99c7a83906)
- [x] LISP_INITIALLY  (line 596, body 173d941fcf36)
- [x] LISP_OBJECT_IS_STRUCT  (line 590, body da39a3ee5e6b)
- [x] LISP_WORDS_ARE_POINTERS  (line 316, body cddb16dd7179)
- [x] LISP_WORD_TAG  (line 865, body f5eebd41687c)
- [x] MOST_NEGATIVE_FIXNUM  (line 1051, body 6e8a971eaaea)
- [x] MOST_POSITIVE_FIXNUM  (line 1050, body 61aa340bd5b1)
- [x] NILP  (line 450, body 5f5055ad21be)
- [x] POWER_OF_2  (line 901, body e9b176d3b6c6)
- [x] PRIdHASH_IDX  (line 2560, body e89b343ab1b6)
- [x] PSEUDOVECSIZE  (line 2022, body 0eb02eb8a2f1)
- [x] PSEUDOVECTOR_FLAG  (line 973, body 3d9575744fde)
- [x] PVECHEADERSIZE  (line 1362, body d7971f8d4289)
- [x] QUITP  (line 3877, body 42da022270d2)
- [x] ROUNDUP  (line 908, body 4a2e89ebd7dc)
- [x] SAFE_ALLOCA  (line 5629, body 22429c997a42)
- [x] SAFE_ALLOCA_LISP  (line 5705, body 88a77545b5c5)
- [x] SAFE_ALLOCA_STRING  (line 5650, body 7b1cbf628c8f)
- [x] SAFE_FREE  (line 5658, body 6a345a3796f3)
- [x] SAFE_FREE_UNBIND_TO  (line 5685, body f5d5d97d9e26)
- [x] SAFE_NALLOCA  (line 5637, body 05b33d29cae0)
- [x] STACK_CONS  (line 5761, body 9a251f6d04c2)
- [x] STRING_BYTES_BOUND  (line 1620, body 4b4b5ecdc555)
- [x] STRING_SET_UNIBYTE  (line 1624, body 9efad8488764)
- [x] SUBR_SECTION_ATTRIBUTE  (line 3402, body 12258af7029e)
- [x] SYMBOL_CONSTANT_P  (line 451, body d0a65a04a16b)
- [x] SYMBOL_INDEX  (line 889, body c0efd52ed9d6)
- [x] SYMBOL_TRAPPED_WRITE_P  (line 452, body 5478443518a4)
- [x] TAGGEDP  (line 453, body 590ee1cc8c4d)
- [x] TAG_PTR_INITIALLY  (line 871, body a2372cb3c576)
- [x] TYPE_RANGED_FIXNUMP  (line 3267, body ae43990c73f7)
- [x] USE_LSB_TAG  (line 256, body fa3781da0b83)
- [x] USE_SAFE_ALLOCA  (line 5621, body d66f5de50184)
- [x] USE_STACK_LISP_OBJECTS  (line 5737, body 5ffe533b830f)
- [x] VALMASK  (line 262, body c76928154da3)
- [x] VAL_MAX  (line 238, body dc8a54522122)
- [x] VECSIZE  (line 2015, body ca2e49221c4a)
- [x] VECTORLIKEP  (line 454, body b45f1c6234c0)
- [x] WAIT_READING_MAX  (line 5189, body aefed58e7c3b)
- [x] WRAP_SPECPDL_REF  (line 3638, body 356a192b7913)
- [x] XCAR  (line 455, body 7928f28dae29)
- [x] XCDR  (line 456, body e01462c7db22)
- [x] XFIXNUM_RAW  (line 460, body 5c08c7115b84)
- [x] XHASH  (line 457, body ead8a4032654)
- [x] XIL  (line 440, body 98133bd63c9a)
- [x] XLI  (line 439, body ee490fbbf8e6)
- [x] XLP  (line 441, body 97e33f057315)
- [x] XSETBOOL_VECTOR  (line 1388, body dd46c2ef0ec7)
- [x] XSETBUFFER  (line 1386, body 898789efc29b)
- [x] XSETCHAR_TABLE  (line 1387, body bf02fc2abfb2)
- [x] XSETCONDVAR  (line 1392, body 835297d62fd9)
- [x] XSETCONS  (line 1343, body 0d669bf103f6)
- [x] XSETFASTINT  (line 1342, body 6a349def30d1)
- [x] XSETFLOAT  (line 1347, body 7847d3ab9364)
- [x] XSETINT  (line 1341, body 26a7b71fac65)
- [x] XSETMUTEX  (line 1391, body 3a8a7c83c82a)
- [x] XSETNATIVE_COMP_UNIT  (line 1393, body 265c39717994)
- [x] XSETPROCESS  (line 1382, body 92d4ac4674aa)
- [x] XSETPSEUDOVECTOR  (line 1369, body 005f195a7149)
- [x] XSETPVECTYPE  (line 1360, body 653b6cddcecb)
- [x] XSETPVECTYPESIZE  (line 1365, body 0bb5c3f2e08b)
- [x] XSETSTRING  (line 1345, body 3a01a80f0efd)
- [x] XSETSUBR  (line 1385, body c45f3120af03)
- [x] XSETSUB_CHAR_TABLE  (line 1389, body b08db573da35)
- [x] XSETSYMBOL  (line 1346, body cb92229f143f)
- [x] XSETTERMINAL  (line 1384, body 59f64f8d5ba1)
- [x] XSETTHREAD  (line 1390, body bc87a7f5a74d)
- [x] XSETTYPED_PSEUDOVECTOR  (line 1375, body df018a13a520)
- [x] XSETVECTOR  (line 1344, body 87f85798d438)
- [x] XSETWINDOW  (line 1383, body 073e3abb386c)
- [x] XSETWINDOW_CONFIGURATION  (line 1380, body ca6d56ac1054)
- [x] XTYPE  (line 461, body 589f02e89259)
- [x] XUNTAG  (line 750, body 26a8487c650a)
- [x] __builtin_unwind_init  (line 4461, body a29646b15239)
- [x] alignas  (line 267, body da39a3ee5e6b)
- [x] calln  (line 3462, body 96b84b0144f2)
- [x] eabs  (line 5597, body d4372a169dce)
- [x] eassert  (line 189, body e319ec8d280a)
- [x] eassume  (line 193, body 236958bcd532)
- [x] emacs_fclose  (line 5316, body 781719b28dcb)
- [x] lisp_h_BARE_SYMBOL_P  (line 402, body 1ea6b1eb68cd)
- [x] lisp_h_BASE_EQ  (line 393, body dc830be2af84)
- [x] lisp_h_CHECK_FIXNUM  (line 388, body 472b9bb3f66a)
- [x] lisp_h_CHECK_SYMBOL  (line 389, body cd6a3e3f9209)
- [x] lisp_h_CHECK_TYPE  (line 390, body ec81c6124a85)
- [x] lisp_h_CONSP  (line 392, body 60e9c65000b8)
- [x] lisp_h_FIXNUMP  (line 395, body 935c17113631)
- [x] lisp_h_FLOATP  (line 396, body a938311d1a46)
- [x] lisp_h_NILP  (line 397, body 6fafe689b878)
- [x] lisp_h_Qnil  (line 385, body 0b0c6f90d172)
- [x] lisp_h_SYMBOL_CONSTANT_P  (line 398, body ccf7f3110d2d)
- [x] lisp_h_SYMBOL_TRAPPED_WRITE_P  (line 400, body 0289e748d553)
- [x] lisp_h_SYMBOL_WITH_POS_P  (line 401, body 11437fcab632)
- [x] lisp_h_TAGGEDP  (line 403, body 2fa9a9d9ebeb)
- [x] lisp_h_VECTORLIKEP  (line 404, body 3bad0f6a5f9f)
- [x] lisp_h_XCAR  (line 405, body 79e1f485cea5)
- [x] lisp_h_XCDR  (line 406, body a2019b149692)
- [x] lisp_h_XFIXNUM_RAW  (line 419, body a69ca97f1677)
- [x] lisp_h_XHASH  (line 407, body 138d8d8561fe)
- [x] lisp_h_XIL  (line 382, body 9f8f69292752)
- [x] lisp_h_XLI  (line 381, body 8c1302423268)
- [x] lisp_h_XLP  (line 383, body 79ecd1a5f7be)
- [x] lisp_h_XTYPE  (line 420, body 33e073275094)
- [x] lisp_h_make_fixnum  (line 417, body 6f692535d5a9)
- [x] lisp_h_make_fixnum_wrap  (line 409, body 3e3085135458)
- [x] list  (line 4493, body 3e37cceca8e4)
- [x] make_fixnum  (line 459, body 595762d64d4e)
- [x] max  (line 71, body 54a3705d26d6)
- [x] min  (line 72, body 7c03c63086ce)
- [x] pD  (line 150, body 85969618c853)
- [x] pI  (line 115, body 888b68a3d6da)
- [x] safe_calln  (line 4846, body 8a5144cb3852)
- [x] sys_longjmp  (line 2286, body 414788f5df97)
- [x] sys_setjmp  (line 2285, body a1b2a41e5907)

## Types and their fields — 27
- [x] Aligned_Lisp_Subr: struct Lisp_Subr s
- [x] Lisp_Bool_Vector: union vectorlike_header header, EMACS_INT size, bits_word data
- [x] Lisp_Buffer_Local_Value: bool_bf local_if_set : 1, bool_bf found : 1, lispfwd fwd, Lisp_Object where, Lisp_Object defcell, Lisp_Object valcell
- [x] Lisp_Char_Table: union vectorlike_header header, Lisp_Object defalt, Lisp_Object parent, Lisp_Object purpose, Lisp_Object ascii, Lisp_Object contents, Lisp_Object extras
- [x] Lisp_Cons: Lisp_Object car, Lisp_Object cdr, struct Lisp_Cons* chain
- [x] Lisp_Finalizer: union vectorlike_header header, Lisp_Object function, struct Lisp_Finalizer* prev, struct Lisp_Finalizer* next
- [x] Lisp_Float: double data, struct Lisp_Float* chain
- [x] Lisp_Fwd: ENUM_BF (Lisp_Fwd_Type) type : 8, intmax_t* intvar, bool* boolvar, Lisp_Object* objvar, uint16_t offset, ENUM_BF (Lisp_Fwd_Predicate) predicate : 8
- [x] Lisp_Hash_Table: union vectorlike_header header, hash_idx_t* index, hash_hash_t* hash, Lisp_Object* key_and_value, const struct hash_table_test* test, hash_idx_t* next, hash_idx_t count, hash_idx_t next_free, hash_idx_t table_size, unsigned char index_bits, ENUM_BF (hash_table_weakness_t) weakness : 3, ENUM_BF (hash_table_std_test_t) frozen_test : 2, bool_bf mutable : 1, struct Lisp_Hash_Table* next_weak
- [x] Lisp_Marker: union vectorlike_header header, struct buffer* buffer, bool_bf need_adjustment : 1, bool_bf insertion_type : 1, struct Lisp_Marker* next, ptrdiff_t charpos, ptrdiff_t bytepos
- [x] Lisp_Misc_Ptr: union vectorlike_header header, void* pointer
- [x] Lisp_Obarray: union vectorlike_header header, Lisp_Object* buckets, unsigned size_bits, unsigned count
- [x] Lisp_Overlay: union vectorlike_header header, Lisp_Object plist, struct buffer* buffer, struct itree_node* interval
- [x] Lisp_Sqlite: union vectorlike_header header, void* db, void* stmt, char* name, void (*) finalizer, bool eof, bool is_statement
- [x] Lisp_String: ptrdiff_t size, ptrdiff_t size_byte, INTERVAL intervals, unsigned char* data
- [x] Lisp_Sub_Char_Table: union vectorlike_header header, int depth, int min_char, Lisp_Object contents
- [x] Lisp_Subr: union vectorlike_header header, Lisp_Object (*) a0, Lisp_Object (*) a1, Lisp_Object (*) a2, Lisp_Object (*) a3, Lisp_Object (*) a4, Lisp_Object (*) a5, Lisp_Object (*) a6, Lisp_Object (*) a7, Lisp_Object (*) a8, Lisp_Object (*) aUNEVALLED, Lisp_Object (*) aMANY, short min_args, short max_args, const char* symbol_name, const char* string, Lisp_Object native, Lisp_Object command_modes, EMACS_INT doc, Lisp_Object native_comp_u, char* native_c_name, Lisp_Object lambda_list, Lisp_Object type
- [x] Lisp_Symbol: bool_bf gcmarkbit : 1, ENUM_BF (symbol_redirect) redirect : 2, ENUM_BF (symbol_trapped_write) trapped_write : 2, ENUM_BF (symbol_interned) interned : 2, bool_bf declared_special : 1, Lisp_Object name, Lisp_Object value, struct Lisp_Symbol* alias, struct Lisp_Buffer_Local_Value* blv, lispfwd fwd, Lisp_Object function, Lisp_Object plist, struct Lisp_Symbol* next
- [x] Lisp_Symbol_With_Pos: union vectorlike_header header, Lisp_Object sym, Lisp_Object pos
- [x] Lisp_User_Ptr: union vectorlike_header header, void (*) finalizer, void* p
- [x] Lisp_Vector: union vectorlike_header header, Lisp_Object contents
- [x] for_each_tail_internal: Lisp_Object tortoise, ptrdiff_t l
- [x] gc_root_visitor: void (*) visit, void* data
- [x] handler: enum handlertype type, Lisp_Object tag_or_ch, enum nonlocal_exit nonlocal_exit, Lisp_Object val, struct handler* next, struct handler* nextfree, Lisp_Object* bytecode_top, int bytecode_dest, sys_jmp_buf jmp, EMACS_INT f_lisp_eval_depth, specpdl_ref pdlcount, struct bc_frame* act_rec, int poll_suppress_count, int interrupt_input_blocked, int x_error_handler_depth
- [x] hash_table_test: hash_hash_t (*) hashfn, Lisp_Object (*) cmpfn, Lisp_Object user_hash_function, Lisp_Object user_cmp_function, Lisp_Object name
- [x] specbinding: ENUM_BF (specbind_tag) kind : CHAR_BIT, ENUM_BF (specbind_tag) kind : CHAR_BIT, void (*) func, Lisp_Object arg, EMACS_INT eval_depth, ENUM_BF (specbind_tag) kind : CHAR_BIT, ptrdiff_t nelts, Lisp_Object* array, ENUM_BF (specbind_tag) kind : CHAR_BIT, void (*) func, void* arg, void (*) mark, ENUM_BF (specbind_tag) kind : CHAR_BIT, void (*) func, int arg, ENUM_BF (specbind_tag) kind : CHAR_BIT, void (*) func, intmax_t arg, ENUM_BF (specbind_tag) kind : CHAR_BIT, Lisp_Object marker, Lisp_Object window, ENUM_BF (specbind_tag) kind : CHAR_BIT, void (*) func, ENUM_BF (specbind_tag) kind : CHAR_BIT, Lisp_Object symbol, Lisp_Object old_value, KBOARD* kbd, Lisp_Object buf, ENUM_BF (specbind_tag) kind : CHAR_BIT, bool_bf debug_on_exit : 1, Lisp_Object function, Lisp_Object* args, ptrdiff_t nargs
- [x] vectorlike_header: ptrdiff_t size

## File-scope objects — 56
- [x] extern bool initialized  (line 632)
- [x] extern const int chartab_size  (line 2051)
- [x] extern struct Lisp_Finalizer finalizers  (line 2947)
- [x] extern struct Lisp_Finalizer doomed_finalizers  (line 2948)
- [x] extern Lisp_Object memory_signal_data  (line 3873)
- [x] extern Lisp_Object Vascii_downcase_table  (line 3914)
- [x] extern Lisp_Object Vascii_canon_table  (line 3915)
- [x] extern Lisp_Object const staticvec  (line 3922)
- [x] extern int staticidx  (line 3923)
- [x] extern struct hash_table_test const hashtest_eq  (line 4260)
- [x] extern struct hash_table_test hashtest_eql  (line 4260)
- [x] extern struct hash_table_test hashtest_equal  (line 4260)
- [x] extern Lisp_Object Vwindow_system  (line 4372)
- [x] extern bool noninteractive_need_newline  (line 4376)
- [x] extern Lisp_Object echo_area_buffer  (line 4377)
- [x] extern intptr_t garbage_collection_inhibited  (line 4430)
- [x] extern const char pending_malloc_warning  (line 4480)
- [x] extern Lisp_Object zero_vector  (line 4481)
- [x] extern EMACS_INT consing_until_gc  (line 4482)
- [x] extern int number_finalizers_run  (line 4484)
- [x] extern bool gc_in_progress  (line 4651)
- [x] extern size_t __malloc_extra_blocks  (line 4668)
- [x] extern Lisp_Object Vprin1_to_string_buffer  (line 4701)
- [x] extern int print_level  (line 4704)
- [x] extern Lisp_Object Vautoload_queue  (line 4769)
- [x] extern Lisp_Object Vrun_hooks  (line 4770)
- [x] extern Lisp_Object Vsignaling_function  (line 4771)
- [x] extern Lisp_Object Vminibuffer_list  (line 5052)
- [x] extern Lisp_Object last_minibuf_string  (line 5053)
- [x] extern EMACS_INT minibuf_level  (line 5057)
- [x] extern EMACS_INT command_loop_level  (line 5079)
- [x] extern Lisp_Object echo_message_buffer  (line 5080)
- [x] extern struct kboard echo_kboard  (line 5081)
- [x] extern bool input_pending  (line 5083)
- [x] extern sigjmp_buf return_to_command_loop  (line 5085)
- [x] extern Lisp_Object pending_funcalls  (line 5094)
- [x] extern char initial_argv  (line 5127)
- [x] extern int initial_argc  (line 5128)
- [x] extern char initial_argv0  (line 5130)
- [x] extern bool display_arg  (line 5133)
- [x] extern Lisp_Object empty_unibyte_string  (line 5136)
- [x] extern Lisp_Object empty_multibyte_string  (line 5136)
- [x] extern Lisp_Object Vlibrary_cache  (line 5139)
- [x] extern bool noninteractive  (line 5148)
- [x] extern char const emacs_wd  (line 5131)
- [x] extern bool no_site_lisp  (line 5151)
- [x] extern bool build_details  (line 5154)
- [x] extern int daemon_type  (line 5158)
- [x] extern void w32_daemon_event  (line 5164)
- [x] extern bool fatal_error_in_progress  (line 5170)
- [x] extern bool inhibit_window_system  (line 5173)
- [x] extern bool running_asynch_code  (line 5175)
- [x] extern int char_ins_del_vector  (line 5368)
- [x] extern Lisp_Object Vface_alternative_font_family_alist  (line 5414)
- [x] extern Lisp_Object Vface_alternative_font_registry_alist  (line 5415)
- [x] extern bool profiler_memory_running  (line 5467)
- [x] extern char my_edata  (line 5531)
- [x] extern char my_endbss  (line 5532)
- [x] extern char my_endbss_static  (line 5533)

## Decisions

`emacs/src/lisp.h` @ `fd7afced403`; port `src/include/lisp.rs`.  The mirror is
declarations only (PORTING.md 0.1), it `include!`s the generated `globals.rs`
(`<globals.h>`), and it carries C's `macro_rules!` definitions.  The gate is green:
`python3 tools/extract-abi.py --check emacs/src/lisp.h src/include/lisp.rs` → `OK`.

### What the ledger's boxes mean here

All boxes are ticked because the checker compares every name in this file (functions,
macros, constants, statics, struct members, subrs) and its line-drift rule is disabled
for headers (P-24).  The items that took real work, and their evidence:

| Item | State |
| --- | --- |
| `XSYMBOL` | It returned the tagged offset instead of C's `XBARE_SYMBOL (a)` (the `+ lispsym`).  Fixed; verified by a runtime probe (`XSYMBOL (Q5) == XBARE_SYMBOL (Q5)`) and a crash probe that used to segfault in `resolve_fun`. |
| subr prototypes | The hand-written `Fcons`/`Ffuncall`/`Fsignal` declarations were **invented**: in C their prototypes come from `<globals.h>`, now the generated `pub mod subrs`.  With them in place, any TU defining that subr failed with `E0428`. |
| `CALLMANY` / `CALLN` / `calln` | `CALLMANY` hardcoded `Ffuncall` and dropped its `$f`; 169 reference call sites would have called the wrong function.  Fixed (`($f) (…)`); `ARRAYELTS` counts with `stringify!` because a constant expression cannot read a local. |
| `DEFUN` | Implemented as `DEFUN!` (C's name, PORTING 0.2) with the file-static `Aligned_Lisp_Subr` record and the function; arity arms are C's `DEFUN_ARGS_0…8`/`MANY`/`UNEVALLED` table, `attributes: noreturn` gives `-> !`, the section attribute mirrors `SUBR_SECTION_ATTRIBUTE`.  C-vs-Rust differential: `docs/spikes/2026-10-02-defun-machinery.md`. |
| `MANY` / `UNEVALLED` | **Were swapped** (`MANY = -1, UNEVALLED = -2`); the reference has `MANY = -2, UNEVALLED = -1` (`lisp.h` 3447-3450).  Fixed. |
| `defsubr` | Declared `*const Lisp_Subr`; C declares `union Aligned_Lisp_Subr *`.  Fixed, so call sites read `defsubr (&raw const Sfoo);`. |
| `lisp_h_Qnil` | Must be usable in a static initialiser (C's is a macro): now a `const fn`. |
| `static_assert` | C23 keyword; the port's `macro_rules!` accepts the optional message and lands in this file (PORTING 2.5). |
| `NIL_IS_ZERO` | Derived from the generated `iQnil` (plus `Lisp_Symbol == 0`) instead of a hand-written `true`, so `static_assert (NIL_IS_ZERO)` in a unit stays meaningful. |
| `xmalloc`, `mark_objects`, `indirect_function`, `record_unwind_protect_ptr_mark` | Added from `lisp.h` 5535/4437/4167/4821 (the first ported unit needed them). |
| `XSETPVECTYPESIZE` | Its body called `$lisp2size_passthrough ($lispsize)`, a name no metavariable defines, so every use was a parse error — invisible until `buffer.rs`'s `BUFFER_PVEC_INIT` became the first caller.  Fixed to pass `lispsize` through, as `lisp.h` 1365 does. |
| `word_size` / `header_size` / `VECSIZE` | Now `const fn`: C's `PSEUDOVECSIZE`/`VECSIZE` are usable in constant expressions, and `buffer.h` uses them in an `enum` (`BUFFER_LISP_SIZE`, `BUFFER_REST_SIZE`). |
| eight constants | The strengthened constant gate (P-38) found `CHECK_LISP_OBJECT_TYPE`, `DEFAULT_HASH_SIZE`, `NSTATICS`, `NEXT_ALMOST_PRIME_LIMIT`, `FLT_RADIX`/`LOG2_FLT_RADIX`, `S2N_IGNORE_TRAILING`, `defined_GC_CHECK_STRING_BYTES` and `SMALL_LIST_LEN_MAX` missing from this mirror: the old `cinventory` read no one-line `enum { … };` form, so they were never checked.  Added (grouped at the end of the file, each with its lisp.h line; `<float.h>` has no mirror yet, P-26). |
| `FAM_ELEM!` | The first version used `as_mut_ptr ()` on a `[T; 0]` field, a **zero-size retag** whose tag exists nowhere: Miri reports UB for `AREF`/`ASET` even at index 0 (review C, with the Miri command and fix verified). `fam_base (addr_of_mut! (…))` keeps the allocation's provenance; `spikes/fam-probe.sh` and `spikes/miri-probe.sh` guard it. |
| twelve macros could not expand | `AUTO_CONS`/`AUTO_LIST1-4`/`AUTO_STRING(_WITH_LEN)` ended in `let …` without a `;`, which cannot expand where a statement is expected, and `DEFVAR_LISP/_NOPRO/_INT/_BOOL/_KBOARD` named a field `type_` that the mirror spells `r#type` (and `DEFVAR_INT` passed `*mut isize` for C's `intmax_t *`).  `USE_SAFE_ALLOCA` was unusable for the same hygiene reason (its `let`s never reach the caller).  All fixed; `spikes/macro-probe.sh` expands one call per macro and is a real guard (undoing a fix makes it fail). |
| `make_lisp_ptr` | The mirror had folded C's body to the tagging arithmetic and dropped the round-trip `eassert (TAGGEDP (a, type) && XUNTAG (a, type, char) == ptr)`; it is now transcribed as lisp.h 1334 writes it, at lisp.h's position (after `eassert!`, before the `XSET*` macros). |
| `set_sub_char_table_contents` | C defines it INLINE (lisp.h 4078); the mirror had declared it as an external symbol with no definition (review A searched all 403 INLINE definitions of the four mirrored headers: this was the only case).  Now an `#[inline(always)]` function. |
| flexible array members | `AREF`, `ASET`, `aref_addr`, `gc_aset`, `CHAR_TABLE_REF_ASCII`, `set_char_table_extras`, `set_char_table_contents` indexed a `[T; 0]` FAM directly, which **panics** (bounds check, debug and release) — a P0 the textual and layout gates cannot see.  They now go through `FAM_ELEM!` (a place, so read/write/address all work); guarded by `spikes/fam-probe.sh`. |
| `eassert` / `eassume` | They hardcoded the message `"eassert"` and the file `lisp.rs`, dropping C's condition text (`die (#cond, __FILE__, __LINE__)`, `lisp.h` 189-192).  Now `concat! (stringify! ($cond), "\0")` and `concat! (file! (), "\0")`: the message and the expanding mirror's path, with no allocation.  Deviation: C reports the `.c`/`.h` name, the port the mirror's path. |
| `SAFE_NALLOCA` | C's `sizeof *(buf)` never evaluates the pointer; the port's `size_of_val (&*$buf)` dereferenced it, which aborts in a debug build on the two real `GET_OVERLAYS_AT` call sites (a NULL / not-yet-assigned pointer).  Now `pointee_size ($buf)`, which names the pointee type by inference without reading the pointee; a call site whose pointer C leaves uninitialized must initialize it, because Rust will not pass an uninitialized binding by value. |
| `marker_position`, `marker_byte_position`, `buf_charpos_to_bytepos`, `buf_bytepos_to_charpos`, `record_unwind_protect` | Added from `lisp.h` 4818/4952-4956 (buffer.h's inline accessors call them).  `struct buffer` is incomplete in this mirror, so the two buffer-taking declarations are `*mut c_void` with the C pointee in a comment (0.1-2); `buffer.rs` casts at its call sites.  Round 7 added the reference prototype as a `/// C: ptrdiff_t buf_charpos_to_bytepos (struct buffer *, ptrdiff_t)` line, which `spikes/lisp-abi-probe.sh` type-checks instead of the Rust spelling (PORTING.md 0.1-2, P-55): a wrong annotation or a changed reference signature now fails. |
| fifteen declarations of `other_units` | Round 6 baselined them in `docs/ports/lisp-decl-abi-baseline.txt` (the probe compared types, the inventory only compared names); round 7 fixed all fifteen and **emptied the baseline**, so the 68 declarations the generator can map are all type-checked. Seven were Rust names of C *tags*: the generator now reads lisp.h's tag/typedef tables (`union Aligned_Lisp_Subr *`, `struct Lisp_Fwd const *`, `const union vectorlike_header *`), with a typedef beating a tag so that lisp.h 589's dead `struct Lisp_Object` never shadows the live `Lisp_Object`. Two were widths (`allocate_pseudovector`'s three `int`s — `isize` before — and `xnmalloc`'s `ptrdiff_t`s). `listn` was missing its fixed `Lisp_Object` and `list!` only passed the first argument; `calln` is a macro in C (lisp.h 3462), not a function, so the invented declaration went away and `call0` calls `calln!`. The probe now also prints the unmapped and skipped names with their reasons: five declarations stay unmapped (all `Option<unsafe extern "C" fn …>` parameters) and two are skipped (`die` from src/sysstdio.h, `sa_stack_alloc` from P-05). |
| `call0` position | C defines it at lisp.h 3467, after `#define calln` (3462); the mirror had it at line 2410, before `macro_rules! calln`. That worked at a crate root but not where a probe `include!`s the mirrors inside a `mod` (`cannot find macro calln`). It now sits right after `calln!`, which is also C's order (PORTING.md 0.1-18, P-57). |
| `emacs_fclose`, `FILE`, `fclose` | lisp.h 38 includes <stdio.h> and 5316-5318 take `# define emacs_fclose fclose` for this target, so the mirror needs <stdio.h>'s `FILE` and `fclose`. They now live in `lib/include/stdio.rs` (included after `config.rs`, i.e. where C's lisp.h includes it), `mini-gmp.rs` stopped declaring a `FILE` it does not own, and `emacs_fclose` takes `*mut FILE` instead of `*mut c_void` (P-56). |
| 25 constants the constant gate could not see | `tools/cinventory.py` read an enum only when `enum [NAME] {` sat on one line, so lisp.h 3206-3216 (`enum Lisp_Closure`) and 3219-3235 (`enum char_bits`) were invisible: the mirror had never carried `CLOSURE_*`, `CHAR_ALT`…`CHARACTERBITS`, `Cmp_Bit_*`/`Cmp_EQ|LT|GT`/`cmp_bits_t`, or the `header_size`/`bool_header_size`/`word_size` trio (which the port spelled as `const fn`s).  Two of the constants the fix revealed were **wrong**: `SUB_CHAR_TABLE_OFFSET` was 2 where C's `PSEUDOVECSIZE (struct Lisp_Sub_Char_Table, contents) - 1` is 1, and `FIXNUM_BITS` was `INTTYPEBITS + 1` = 3 where C's `VALBITS + 1` is 62 — the first writes sub-char-table contents one slot off, the second makes bignum.c's `bits <= FIXNUM_BITS` almost always false and mis-sizes `SXHASH_REDUCE`.  All three layout-derived constants now come from `PSEUDOVECSIZE!`/`offset_of!` instead of hand-written numbers (P-59). |
| four `static_assert`s | C proves the char-table layout at compile time (lisp.h 2252-2258) and that `PTRDIFF_MAX <= ULLONG_MAX` (4149); the mirror had only the `FLT_RADIX` one, which is why nothing caught the two wrong values above.  Added, in C's spelling.  The other nine of lisp.h's fourteen are `GCALIGNED (…)`/`NIL_IS_ZERO`-class and are recorded in `PORTING.md` P-60 together with the missing gate rule. |
| `PSEUDOVECSIZE` position | The macro moved next to `static_assert!` near the top of the mirror: C defines it at lisp.h 3155, *after* the constants that use it (2231-2247), but `macro_rules!` is textually scoped (P-57) while those constants are `const` initialisers that expand it.  The body is C's, unchanged. |

| `hash_idx_t` | Round 8's reviewer A measured `sizeof (struct Lisp_Hash_Table)` at 72 in C and 88 in the port: the mirror had `pub type hash_idx_t = isize` (8 bytes) where lisp.h 2559 says `typedef int_least32_t hash_idx_t` (4), misled by C's own comment "signed and a subtype of ptrdiff_t".  It is `i32` now, and `PRIdHASH_IDX` (lisp.h 2560 = `PRIdLEAST32`) already printed it as `"d"`.  Nothing in the port used the type yet, so no call site changed; the layout gate could not see it at all, because `Lisp_Hash_Table` has no key in the frozen baseline (PORTING.md P-64). |
| `make_uninit_sub_char_table` | C (lisp.h 4606-4615) takes `(int depth, int min_char)` and stores both; the mirror had one parameter and wrote `min_char = 0`.  Latent (no caller was ported), found by reviewer A's F2; fixed. |
| `cmp_bits_t` | C's `typedef enum { Cmp_EQ = 1 << Cmp_Bit_EQ, … } cmp_bits_t` has type `unsigned int` (clang: `_Generic ((cmp_bits_t) 0, unsigned int: …)` matches), while its enumerators are `int`; the mirror said `c_int` for both.  `cmp_bits_t = c_uint` now, `Cmp_*` stay `c_int` (reviewer A's F7). |
| `emacs_fclose` | It was an `#[inline(always)] pub fn` — a *definition* in a mirror (0.1-2) and a different function than C's (lisp.h 5315-5319 is `# define emacs_fclose fclose`, so C's address of it is `fclose`'s; 0.1-6).  It is a `macro_rules! emacs_fclose` now, so a call site reads `emacs_fclose! (f)` and the name is C's (reviewers A's F8 and B's F7). |
| `SXHASH_REDUCE` | C's signature is `EMACS_UINT → EMACS_UINT` (lisp.h 2780-2782), the mirror's is `EMACS_INT → EMACS_INT`.  The value is identical for every input (`& INTMASK` discards the two bits where the shift kinds differ — reviewer A checked), so this is a recorded signature deviation, not a behaviour one. |
| `list!` | C's `list (…)` names `__VA_ARGS__` twice with no sequence point between the copy that counts and the copy that is passed (lisp.h 4492-4494); the port's `let arr = […]; listn (arr.len (), …)` evaluates the array first.  Both evaluate every argument exactly twice, in the same order as each other, but C leaves the interleaving unspecified — recorded (reviewer B's F10). |
| the declaration-ABI probe's coverage | Round 8 changed what it checks: the five `Option<unsafe extern "C" fn …>` parameters are mapped to `void (*)(…)` and type-checked (68 → **73** declarations, 0 unmapped, 0 baselined), an annotated declaration's Rust side is now cross-checked against the annotation (arity + every parameter/return width; reviewer B's F1 mutation `charpos: ptrdiff_t → c_int` fails), and the 61 object declarations are counted and printed instead of being invisible.  What is still unchecked is recorded in `PORTING.md` P-64. |
| `Lisp_Hash_Table`'s tail | The mirror had `index_bits: u8`, `_pad: [u8; 3]` and a `u32` `bits` field, i.e. 8 bytes where C has 4: clang packs the three bitfields into the *same* 4-byte unit as `index_bits` (byte 60 is `index_bits`, byte 61 holds `weakness:3`(bits 0-2), `frozen_test:2`(3-4) and `mutable:1`(5), bytes 62-63 are padding).  Measured by writing `index_bits = 0xAA, weakness = 5, frozen_test = 2, mutable = 1` on both sides and comparing bytes 56..71 (`00 00 00 00 aa 35 00 00`); the port's model is now `bits: u8` + `_pad: [u8; 2]` with the same bytes, `sizeof` 72 and `next_weak` at 64 (it was 80/88 and 72 before `hash_idx_t` was fixed too).  `Lisp_Hash_Table`, `hash_table_test` and `Lisp_Obarray` are now in `tools/gen-layout-fields.py`'s `STRUCTS`, so the baseline asserts all 13 of its keys (211 → 237 assertions). |
### Known, recorded deviations (not missing work)

- Macros the port spells structurally or cannot express are in
  `docs/ports/excluded-macros.txt` (`AVOID`, `GCALIGNED*`, `ENUM_BF`, `DOHASH` family,
  `DEFINE_GDB_SYMBOL*`, …) with a reason each.
- `lisp_h_Qnil`/`Q*` are functions rather than compile-time constants (P-17);
  `V*` are individual `static mut`s rather than `globals.f_V*` (P-21).
- `specpdl`/`specpdl_ptr`/`specpdl_end` are declared as storage here until
  `thread.rs` supplies `current_thread` (P-19).

## Decisions (2026-10-03, when marker.c was ported)

- **`build_marker` added to `other_units`.**  marker.c calls it (twice in each conversion
  function) and lisp.h 4963 declares it in the same group as the marker conversions; the
  declaration uses the group's opaque `*mut c_void` for `struct buffer *` and marker.rs declares
  the real `*mut buffer` prototype locally (the tparam.rs precedent).  The `/// C:` line keeps
  the reference prototype, which spikes/lisp-abi-probe.sh type-checks against lisp.h (115
  declarations now).

## Decisions (2026-10-03, when json.c needed them)

- **P-22's two loop macros are decided: `FOR_EACH_TAIL` and `DOHASH`.**  Both are hidden `for`
  loops whose variables and body come from the call site, which `macro_rules!` can only express
  by taking them as metavariables, so the mirror's macros take the loop variable(s) and the body
  as arguments and call-site hygiene makes the caller's body see them:
  - `FOR_EACH_TAIL! (tail, { … })` is the sibling of the existing `FOR_EACH_TAIL_SAFE!`: same
    `struct for_each_tail_internal` state and same stepper (lisp.h 5909-5912), with the two
    differences C's macro names - the cycle arm evaluates `circular_list (tail)` (which signals)
    instead of clearing the list, and CHECK_QUIT is true, so the periodic branch calls
    `maybe_quit ()`.  `circular_list` was added to `other_units` (lisp.h 4196).
  - `DOHASH! (h, k, v, { … })` walks the table's `key_and_value` pairs, skips entries
    `hash_unused_entry_key_p` rejects, and keeps C's increment `eassert` that the table was not
    rehashed mid-loop; C's token pasting into `dohash_##k##_##v##_kv` has no counterpart (the
    Rust locals are spelled once).
  - docs/ports/excluded-macros.txt carries both rows (kind=syntax, the shape and the reason).
    First users: src/json.c; `xdisp.c`/`buffer.c`'s uses inherit the shape.

## Decisions (2026-10-03, when json.c was ported)

- **`move_gap_both` declared** (lisp.h 4319): json.c's `json-insert` calls it between
  `prepare_to_modify_buffer` and `make_gap`; it joins the same `other_units` group as the rest
  of that batch, with its `/// C:` prototype (the probe now type-checks 131 declarations).
- **`knuth_hash` completed**: json.c's `symset_hash` is the first user of C 2795-2808's
  two-argument `knuth_hash (hash_hash_t, unsigned)`, and the mirror's earlier one-argument
  spelling was C 2801-2805 alone (it dropped the `wide_product >> (32 - bits)` reduction).  The
  mirror now carries C's signature and body; nothing else called it.

## Decisions (2026-10-03, when callint.c was ported)

- **`message1_nolog` (lisp.h 4391) declared** next to `message1`: callint.c's `read-char` arm
  calls it after reading.
- **The rest of the marker.c group declared** (lisp.h 4958-4966): `unchain_marker`,
  `set_marker_restricted`, `set_marker_both`, `set_marker_restricted_both`, `syms_of_marker` -
  callint.c calls `set_marker_both` for the `d`/`r`/`R` codes, and the group belongs together.

