# emacs/src/coding.c (12312 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 32
- [x] coding-system-p  (Fcoding_system_p, 1/1, doc fabfd75d0c26, line 8594)
- [x] read-non-nil-coding-system  (Fread_non_nil_coding_system, 1/1, doc fe154bc90ce5, line 8609)
- [x] read-coding-system  (Fread_coding_system, 1/2, doc 89d55ed3ef03, line 8625)
- [x] check-coding-system  (Fcheck_coding_system, 1/1, doc 8b48782393bc, line 8647)
- [x] detect-coding-region  (Fdetect_coding_region, 2/3, doc d9968666af7a, line 9000)
- [x] detect-coding-string  (Fdetect_coding_string, 1/2, doc d9c949b4c720, line 9035)
- [x] find-coding-systems-region-internal  (Ffind_coding_systems_region_internal, 2/3, doc 0b7ae33f1141, line 9087)
- [x] unencodable-char-position  (Funencodable_char_position, 3/5, doc b19151aa1cef, line 9209)
- [x] check-coding-systems-region  (Fcheck_coding_systems_region, 3/3, doc 96db5e1db738, line 9322)
- [x] decode-coding-region  (Fdecode_coding_region, 3/4, doc 8c4a641b857c, line 9501)
- [x] encode-coding-region  (Fencode_coding_region, 3/4, doc 49c3d3b2ee5f, line 9533)
- [x] decode-coding-string  (Fdecode_coding_string, 2/4, doc 9b49af4a4fff, line 10487)
- [x] encode-coding-string  (Fencode_coding_string, 2/4, doc 01630b4f09fa, line 10509)
- [x] decode-sjis-char  (Fdecode_sjis_char, 1/1, doc a888c8caf420, line 10530)
- [x] encode-sjis-char  (Fencode_sjis_char, 1/1, doc 25cd26365677, line 10583)
- [x] decode-big5-char  (Fdecode_big5_char, 1/1, doc 04eb9658eef6, line 10611)
- [x] encode-big5-char  (Fencode_big5_char, 1/1, doc d7e6b0d213df, line 10655)
- [x] set-terminal-coding-system-internal  (Fset_terminal_coding_system_internal, 1/2, doc 0b7ae33f1141, line 10682)
- [x] set-safe-terminal-coding-system-internal  (Fset_safe_terminal_coding_system_internal, 1/1, doc 0b7ae33f1141, line 10704)
- [x] terminal-coding-system  (Fterminal_coding_system, 0/1, doc abb02a3d618b, line 10720)
- [x] set-keyboard-coding-system-internal  (Fset_keyboard_coding_system_internal, 1/2, doc 0b7ae33f1141, line 10735)
- [x] keyboard-coding-system  (Fkeyboard_coding_system, 0/1, doc 2bb359cfe415, line 10753)
- [x] find-operation-coding-system  (Ffind_operation_coding_system, 1/MANY, doc 8455320581ca, line 10763)
- [x] set-coding-system-priority  (Fset_coding_system_priority, 0/MANY, doc 579e5d504109, line 10875)
- [x] coding-system-priority-list  (Fcoding_system_priority_list, 0/1, doc e3612b412178, line 10933)
- [x] define-coding-system-internal  (Fdefine_coding_system_internal, coding_arg_max/MANY, doc 3f0cdbd9fb29, line 10981)
- [x] coding-system-put  (Fcoding_system_put, 3/3, doc e056d6535f95, line 11505)
- [x] define-coding-system-alias  (Fdefine_coding_system_alias, 2/2, doc 7b3916df156d, line 11575)
- [x] coding-system-base  (Fcoding_system_base, 1/1, doc 66cfb18f75aa, line 11614)
- [x] coding-system-plist  (Fcoding_system_plist, 1/1, doc b63238f60a3c, line 11629)
- [x] coding-system-aliases  (Fcoding_system_aliases, 1/1, doc 1e886bbcc9e4, line 11644)
- [x] coding-system-eol-type  (Fcoding_system_eol_type, 1/1, doc 285d8d12b6e1, line 11657)

## Functions defined — 93
- [x] encode_inhibit_flag  (line 595, static)
- [x] inhibit_flag  (line 604, static)
- [x] growable_destination  (line 618, static)
- [x] record_conversion_result  (line 784, static)
- [x] coding_set_source  (line 908, static)
- [x] coding_change_source  (line 935, static)
- [x] coding_set_destination  (line 946, static)
- [x] coding_change_destination  (line 979, static)
- [x] coding_alloc_by_realloc  (line 988, static)
- [x] coding_alloc_by_making_gap  (line 999, static)
- [x] alloc_destination  (line 1022, static)
- [x] detect_coding_utf_8  (line 1130, static)
- [x] decode_coding_utf_8  (line 1240, static)
- [x] encode_coding_utf_8  (line 1429)
- [x] detect_coding_utf_16  (line 1495, static)
- [x] decode_coding_utf_16  (line 1576, static)
- [x] encode_coding_utf_16  (line 1699, static)
- [x] detect_coding_emacs_mule  (line 1836, static)
- [x] emacs_mule_char  (line 1925, static)
- [x] emacs_mule_finish_composition  (line 2244, static)
- [x] decode_coding_emacs_mule  (line 2300, static)
- [x] encode_coding_emacs_mule  (line 2560, static)
- [x] setup_iso_safe_charsets  (line 2855, static)
- [x] detect_coding_iso_2022  (line 2923, static)
- [x] finish_composition  (line 3312, static)
- [x] decode_coding_iso_2022  (line 3449, static)
- [x] encode_invocation_designation  (line 4216, static)
- [x] encode_designation_at_bol  (line 4311, static)
- [x] encode_coding_iso_2022  (line 4364, static)
- [x] detect_coding_sjis  (line 4576, static)
- [x] detect_coding_big5  (line 4631, static)
- [x] decode_coding_sjis  (line 4677, static)
- [x] decode_coding_big5  (line 4794, static)
- [x] encode_coding_sjis  (line 4896, static)
- [x] encode_coding_big5  (line 4990, static)
- [x] detect_coding_ccl  (line 5069, static)
- [x] decode_coding_ccl  (line 5109, static)
- [x] encode_coding_ccl  (line 5188, static)
- [x] decode_coding_raw_text  (line 5265, static)
- [x] encode_coding_raw_text  (line 5286, static)
- [x] detect_coding_charset  (line 5364, static)
- [x] decode_coding_charset  (line 5467, static)
- [x] encode_coding_charset  (line 5594, static)
- [x] setup_coding_system  (line 5666)
- [x] coding_charset_list  (line 5876)
- [x] coding_system_charset_list  (line 5899)
- [x] raw_text_coding_system  (line 5932)
- [x] raw_text_coding_system_p  (line 5958)
- [x] coding_inherit_eol_type  (line 5970)
- [x] complement_process_encoding_system  (line 6016)
- [x] check_ascii  (line 6169, static)
- [x] check_utf_8  (line 6241, static)
- [x] make_string_from_utf8  (line 6336)
- [x] detect_eol  (line 6374, static)
- [x] adjust_coding_eol_type  (line 6470, static)
- [x] detect_coding  (line 6501, static)
- [x] decode_eol  (line 6759, static)
- [x] get_translation_table  (line 6860, static)
- [x] get_translation  (line 6961, static)
- [x] produce_chars  (line 6992, static)
- [x] produce_composition  (line 7207, static)
- [x] produce_charset  (line 7251, static)
- [x] produce_annotation  (line 7276, static)
- [x] decode_coding  (line 7337, static)
- [x] handle_composition_annotation  (line 7502, static)
- [x] handle_charset_annotation  (line 7583, static)
- [x] consume_chars  (line 7605, static)
- [x] encode_coding  (line 7757, static)
- [x] code_conversion_restore  (line 7832, static)
- [x] code_conversion_save  (line 7849)
- [x] coding_restore_undo_list  (line 7893, static)
- [x] decode_coding_gap  (line 7903)
- [x] decode_coding_object  (line 8063)
- [x] encode_coding_object  (line 8297)
- [x] preferred_coding_system  (line 8522)
- [x] detect_coding_system  (line 8686)
- [x] char_encodable_p  (line 9061, static)
- [x] code_convert_region  (line 9451, static)
- [x] string_ascii_p  (line 9566)
- [x] code_convert_string  (line 9576)
- [x] code_convert_string_norecord  (line 9652)
- [x] get_buffer_gap_address  (line 9663, static)
- [x] get_char_bytes  (line 9687, static)
- [x] encode_string_utf_8  (line 9776)
- [x] decode_string_utf_8  (line 10052)
- [x] convert_string_nocopy  (line 10421, static)
- [x] decode_file_name  (line 10430)
- [x] encode_file_name_1  (line 10450, static)
- [x] encode_file_name  (line 10476)
- [x] make_subsidiaries  (line 10962, static)
- [x] init_coding_once  (line 11689)
- [x] syms_of_coding  (line 11738)
- [x] reset_coding_after_pdumper_load  (line 12289, static)

## Macros defined — 140
- [x] ADD_ANNOTATION_DATA  (line 1074, body 840235f4816c)
- [x] ADD_CHARSET_DATA  (line 1090, body 88acd831367c)
- [x] ADD_COMPOSITION_DATA  (line 1082, body ba36f7dc6643)
- [x] ALLOC_CONVERSION_WORK_AREA  (line 7267, body 83d534dc3c63)
- [x] ASSURE_DESTINATION  (line 879, body 308e8dfbaa7b)
- [x] CATEGORY_MASK_ANY  (line 527, body 9e1efff66c8b)
- [x] CATEGORY_MASK_BIG5  (line 520, body d940283a71ff)
- [x] CATEGORY_MASK_CCL  (line 521, body 7ddb2cc1f73c)
- [x] CATEGORY_MASK_CHARSET  (line 518, body 111e12bd3b83)
- [x] CATEGORY_MASK_EMACS_MULE  (line 522, body 435f81f58c1f)
- [x] CATEGORY_MASK_ISO  (line 564, body fa10e7614384)
- [x] CATEGORY_MASK_ISO_7  (line 504, body 4ffa033026c6)
- [x] CATEGORY_MASK_ISO_7BIT  (line 549, body 0ca7f0d6088b)
- [x] CATEGORY_MASK_ISO_7_ELSE  (line 508, body 7ba90a8b471a)
- [x] CATEGORY_MASK_ISO_7_TIGHT  (line 505, body d4aff427df4f)
- [x] CATEGORY_MASK_ISO_8BIT  (line 552, body ab7eefa89762)
- [x] CATEGORY_MASK_ISO_8_1  (line 506, body ab5d5ab51f75)
- [x] CATEGORY_MASK_ISO_8_2  (line 507, body be676d92e474)
- [x] CATEGORY_MASK_ISO_8_ELSE  (line 509, body 949b354de1ab)
- [x] CATEGORY_MASK_ISO_ELSE  (line 555, body 94e074f389c4)
- [x] CATEGORY_MASK_ISO_ESCAPE  (line 558, body e90fc9d4417c)
- [x] CATEGORY_MASK_RAW_TEXT  (line 523, body e4c2cd4b015a)
- [x] CATEGORY_MASK_SJIS  (line 519, body d42bad6696c9)
- [x] CATEGORY_MASK_UTF_16  (line 569, body a4145f351974)
- [x] CATEGORY_MASK_UTF_16_AUTO  (line 513, body ed97422a95c7)
- [x] CATEGORY_MASK_UTF_16_BE  (line 514, body 88b9a0f30dd0)
- [x] CATEGORY_MASK_UTF_16_BE_NOSIG  (line 516, body 4eac78d15848)
- [x] CATEGORY_MASK_UTF_16_LE  (line 515, body 36d0b07cd580)
- [x] CATEGORY_MASK_UTF_16_LE_NOSIG  (line 517, body ca69e0ddd18b)
- [x] CATEGORY_MASK_UTF_8  (line 576, body 8c4f1b62fc2d)
- [x] CATEGORY_MASK_UTF_8_AUTO  (line 510, body aa36893a534a)
- [x] CATEGORY_MASK_UTF_8_NOSIG  (line 511, body 4167b5dd4978)
- [x] CATEGORY_MASK_UTF_8_SIG  (line 512, body ff1ade8bdd20)
- [x] CHAR_STRING_ADVANCE_NO_UNIFY  (line 896, body c498ced130e2)
- [x] CODING_CCL_DECODER  (line 467, body 33241b01a16b)
- [x] CODING_CCL_ENCODER  (line 469, body 345055dee37e)
- [x] CODING_CCL_VALIDS  (line 471, body 14d5ac7a3807)
- [x] CODING_CHAR_CHARSET  (line 845, body 4f54638f6a49)
- [x] CODING_CHAR_CHARSET_P  (line 859, body 1a8922e22a46)
- [x] CODING_DECODE_CHAR  (line 816, body 2cedcec0b448)
- [x] CODING_ENCODE_CHAR  (line 831, body 49b1180457f2)
- [x] CODING_GET_INFO  (line 609, body 2db78bc31b37)
- [x] CODING_INHIBIT_CHARACTER_SUBSTITUTION  (line 449, body 3893911e7110)
- [x] CODING_ISO_BOL  (line 341, body 74b24b8848df)
- [x] CODING_ISO_CMP_STATUS  (line 346, body 98915dfe5a54)
- [x] CODING_ISO_DESIGNATION  (line 335, body c221e70143b3)
- [x] CODING_ISO_EMBEDDED_UTF_8  (line 350, body 022601fea41b)
- [x] CODING_ISO_EXTSEGMENT_LEN  (line 348, body cbcdf9013138)
- [x] CODING_ISO_FLAGS  (line 333, body c4d1d25192e6)
- [x] CODING_ISO_FLAG_COMPOSITION  (line 435, body 3daa00115a66)
- [x] CODING_ISO_FLAG_DESIGNATE_AT_BOL  (line 426, body e0b28e99ef1e)
- [x] CODING_ISO_FLAG_DESIGNATION  (line 412, body 7232895c5fbc)
- [x] CODING_ISO_FLAG_DIRECTION  (line 418, body ecdf501256ce)
- [x] CODING_ISO_FLAG_FULL_SUPPORT  (line 445, body 2d91fb920a7c)
- [x] CODING_ISO_FLAG_INIT_AT_BOL  (line 422, body d228a45608c3)
- [x] CODING_ISO_FLAG_LATIN_EXTRA  (line 433, body 987a3bdd4907)
- [x] CODING_ISO_FLAG_LEVEL_4  (line 443, body 07780177d9da)
- [x] CODING_ISO_FLAG_LOCKING_SHIFT  (line 405, body d73afeb43f0b)
- [x] CODING_ISO_FLAG_LONG_FORM  (line 391, body f2b1649be8f5)
- [x] CODING_ISO_FLAG_RESET_AT_CNTL  (line 399, body eeed498aee81)
- [x] CODING_ISO_FLAG_RESET_AT_EOL  (line 395, body 5f399b2ba212)
- [x] CODING_ISO_FLAG_REVISION  (line 415, body 3be01b55ecdc)
- [x] CODING_ISO_FLAG_SAFE  (line 429, body 96d5e6218d19)
- [x] CODING_ISO_FLAG_SEVEN_BITS  (line 402, body 92df71781c5c)
- [x] CODING_ISO_FLAG_SINGLE_SHIFT  (line 409, body 19fbaf699b05)
- [x] CODING_ISO_FLAG_USE_OLDJIS  (line 441, body 67e6c786c9b7)
- [x] CODING_ISO_FLAG_USE_ROMAN  (line 439, body 2cedb007191f)
- [x] CODING_ISO_INITIAL  (line 319, body 695605c0a962)
- [x] CODING_ISO_INVOCATION  (line 337, body 2f34960871ab)
- [x] CODING_ISO_INVOKED_CHARSET  (line 343, body 4ba77612b89e)
- [x] CODING_ISO_REQUEST  (line 325, body 23c3e94ae364)
- [x] CODING_ISO_SINGLE_SHIFTING  (line 339, body b98b353521ac)
- [x] CODING_UTF_16_BOM  (line 456, body 879bf8cb1a08)
- [x] CODING_UTF_16_ENDIAN  (line 459, body 61aa547d148f)
- [x] CODING_UTF_16_SURROGATE  (line 462, body a802e480fc86)
- [x] CODING_UTF_8_BOM  (line 452, body bd379eedb230)
- [x] DECODE_COMPOSITION_END  (line 3400, body f13db0f78baf)
- [x] DECODE_COMPOSITION_RULE  (line 3264, body b3eb852e3d54)
- [x] DECODE_COMPOSITION_START  (line 3368, body 56ddb24b6cf7)
- [x] DECODE_DESIGNATION  (line 3186, body d1bb5f63847e)
- [x] DECODE_EMACS_MULE_20_RELATIVE_COMPOSITION  (line 2183, body a438cc57122e)
- [x] DECODE_EMACS_MULE_20_RULEBASE_COMPOSITION  (line 2196, body b4492439c718)
- [x] DECODE_EMACS_MULE_21_COMPOSITION  (line 2153, body e08a0aad9007)
- [x] DECODE_EMACS_MULE_COMPOSITION_RULE_20  (line 2115, body 58105937a337)
- [x] DECODE_EMACS_MULE_COMPOSITION_RULE_21  (line 2133, body 173f06d0404e)
- [x] DECODE_EMACS_MULE_COMPOSITION_START  (line 2207, body a5ed02a5d731)
- [x] EMACS_MULE_COMPOSITION_END  (line 2231, body 28dcb7701884)
- [x] EMACS_MULE_LEADING_CODES  (line 2544, body b127549a0498)
- [x] EMACS_MULE_MAYBE_FINISH_COMPOSITION  (line 2292, body ebb7acb23665)
- [x] EMIT_FOUR_BYTES  (line 776, body 95ac0cf1ad3b)
- [x] EMIT_ONE_ASCII_BYTE  (line 706, body 73fd66c6ce97)
- [x] EMIT_ONE_BYTE  (line 728, body d85bb93601d8)
- [x] EMIT_THREE_BYTES  (line 769, body cdb4b30e485e)
- [x] EMIT_TWO_ASCII_BYTES  (line 715, body 2d277a8b093e)
- [x] EMIT_TWO_BYTES  (line 745, body 3a0b63ff4df3)
- [x] ENCODE_COMPOSITION_RULE  (line 3289, body 065d709028da)
- [x] ENCODE_DESIGNATION  (line 4011, body d23d5e2df106)
- [x] ENCODE_ISO_CHARACTER  (line 4199, body e140ab202db8)
- [x] ENCODE_ISO_CHARACTER_DIMENSION1  (line 4115, body 6f8db51d5283)
- [x] ENCODE_ISO_CHARACTER_DIMENSION2  (line 4159, body b661d87c6636)
- [x] ENCODE_LOCKING_SHIFT_2  (line 4097, body 2ebfa082777d)
- [x] ENCODE_LOCKING_SHIFT_3  (line 4104, body 26fe4faac7e5)
- [x] ENCODE_RESET_PLANE_AND_REGISTER  (line 4284, body 3a8bcd05b4ad)
- [x] ENCODE_SHIFT_IN  (line 4083, body bb6f6e5157f5)
- [x] ENCODE_SHIFT_OUT  (line 4090, body 9ff3636bae9c)
- [x] ENCODE_SINGLE_SHIFT_2  (line 4059, body aaec470c3b8d)
- [x] ENCODE_SINGLE_SHIFT_3  (line 4069, body dbdefc3d3d41)
- [x] EOL_SEEN_CR  (line 1101, body da4b9237bacc)
- [x] EOL_SEEN_CRLF  (line 1102, body 1b6453892473)
- [x] EOL_SEEN_LF  (line 1100, body 356a192b7913)
- [x] EOL_SEEN_NONE  (line 1099, body b6589fc6ab0d)
- [x] ISO_CODE_CSI  (line 361, body 4c3a37daadc2)
- [x] ISO_CODE_ESC  (line 358, body 8a4251cdadf6)
- [x] ISO_CODE_SI  (line 356, body 3a6398cbb2aa)
- [x] ISO_CODE_SO  (line 355, body 44dd34a91ebe)
- [x] ISO_CODE_SS2  (line 359, body 06cb323c9c4a)
- [x] ISO_CODE_SS2_7  (line 357, body f331dcff7d8d)
- [x] ISO_CODE_SS3  (line 360, body fae0d267259b)
- [x] LOOKUP_TRANSLATION_TABLE  (line 6926, body d06a12d7ddeb)
- [x] MAX_ANNOTATION_LENGTH  (line 1072, body ac3478d69a3c)
- [x] MAX_CHARBUF_EXTRA_SIZE  (line 7265, body 35e3edecd320)
- [x] MAX_CHARBUF_SIZE  (line 7261, body da533c139833)
- [x] MAX_EOL_CHECK_COUNT  (line 6371, body 77de68daecd8)
- [x] MAYBE_FINISH_COMPOSITION  (line 3350, body 248c100edff4)
- [x] ONE_MORE_BYTE  (line 633, body 808d518e4d1a)
- [x] SAFE_CHARSET_P  (line 2850, body e00f7dd2f073)
- [x] STORE_COMPOSITION_CHAR  (line 3431, body 15a11b57258c)
- [x] STORE_COMPOSITION_RULE  (line 3420, body 7ed81ee97237)
- [x] STRING_CHAR_ADVANCE_NO_UNIFY  (line 903, body 5132b89faa31)
- [x] TWO_MORE_BYTES  (line 669, body 2c094337398a)
- [x] UTF_8_1_OCTET_P  (line 1115, body 77f47833456b)
- [x] UTF_8_2_OCTET_LEADING_P  (line 1117, body b8a79ba4342d)
- [x] UTF_8_3_OCTET_LEADING_P  (line 1118, body 53d9a6efc0be)
- [x] UTF_8_4_OCTET_LEADING_P  (line 1119, body 9dc71a9c1aac)
- [x] UTF_8_5_OCTET_LEADING_P  (line 1120, body 17eb28bf6df3)
- [x] UTF_8_BOM_1  (line 1122, body 03b3c6d8930d)
- [x] UTF_8_BOM_2  (line 1123, body 15cc51e52f66)
- [x] UTF_8_BOM_3  (line 1124, body 35d0ae57f853)
- [x] UTF_8_EXTRA_OCTET_P  (line 1116, body 07e2d824cf5b)
- [x] UTF_8_SEQUENCE_LENGTH  (line 10058, body 456df155de9b)

## Types and their fields — 0

## File-scope objects — 12
- [x] Lisp_Object Vcoding_system_hash_table  (line 305)
- [x] struct coding_system safe_terminal_coding  (line 311)
- [x] static Lisp_Object Vsjis_coding_system  (line 314, static)
- [x] static Lisp_Object Vbig5_coding_system  (line 315, static)
- [x] static Lisp_Object Vcoding_category_table  (line 583, static)
- [x] static enum coding_category coding_priorities  (line 586, static)
- [x] static struct coding_system coding_categories  (line 590, static)
- [x] char emacs_mule_bytes  (line 1829)
- [x] static enum iso_code_class_type iso_code_class  (line 2848, static)
- [x] static Lisp_Object Vcode_conversion_workbuf_name  (line 7823, static)
- [x] static Lisp_Object Vcode_conversion_reused_workbuf  (line 7826, static)
- [x] static bool reused_workbuf_in_use  (line 7829, static)

# ----------------------------------------------------------------------
## Decisions

- **`coding.h`'s mirror is used as it stands.**  `python3 tools/extract-abi.py --check
  emacs/src/coding.h src/include/coding.rs` reports `OK`, so the header's types,
  constants, accessors and the three `extern` objects come from the mirror; the macros the
  header does *not* carry (`CODING_ISO_*`, `CODING_UTF_8_BOM`/`CODING_UTF_16_*`,
  `CODING_CCL_DECODER`/`_ENCODER`/`_VALIDS`, `SAFE_CHARSET_P`, `MAX_ANNOTATION_LENGTH`,
  `EOL_SEEN_*`, `ISO_CODE_*`, `UTF_8_*` and the `CATEGORY_MASK_*` set) are coding.c's own
  (C 316-472, 1067-1124) and are defined in the unit.

- **The three file-scope objects `coding.h` declares are defined in `coding_objs`.**  The
  mirror keeps `Vcoding_system_hash_table`, `safe_terminal_coding` and `emacs_mule_bytes` as
  top-level `unsafe extern "C"` declarations instead of putting them in an `*_objs` module
  (the `charset_objs` shape), so a crate-root definition is `E0428`; the definitions live in
  a module that is glob-imported, which is P-21's mechanism (the crate-root declaration
  shadows the import for name resolution while the definition still emits C's own symbol).
  **Mirror gap, reported:** moving those three into a `coding_objs`-style module in
  `src/include/coding.rs` would let the unit define them at the crate root like every
  other unit.

- **The `iso_2022_spec` lvalue flags.**  The mirror packs `single_shifting`, `bol` and
  `embedded_utf_8` into `bits: u32`; C reads and writes them as lvalues.  `CODING_ISO_BOL`,
  `CODING_ISO_SINGLE_SHIFTING` and `CODING_ISO_EMBEDDED_UTF_8` therefore yield the
  `iso_2022_bit` proxy and the call sites use `.get ()`/`.set (v)`; the three field names are
  recorded in `docs/ports/macro-body-notes.tsv` (PORTING.md 1.3).

- **The statement macros that capture the caller's locals and jump to its labels**
  (`ONE_MORE_BYTE`, `TWO_MORE_BYTES`, `EMIT_*`, `ASSURE_DESTINATION`,
  `CODING_DECODE_CHAR`/`CODING_ENCODE_CHAR`/`CODING_CHAR_CHARSET[_P]`, `ADD_*`, the
  `DECODE_*`/`ENCODE_*`/`STORE_*` composition macros and `MAYBE_FINISH_COMPOSITION`) are
  defined **inside every function that uses them**, after the locals they capture and inside
  the label's scope, exactly as `src/ccl.rs` does (docs/status.md 3.55, PORTING.md P-57):
  a `macro_rules!` resolves local variables *and labels* at its definition site.  Macros that
  capture nothing (`CHAR_STRING_ADVANCE_NO_UNIFY`, `STRING_CHAR_ADVANCE_NO_UNIFY`,
  `ckd_add`) stay at file scope.

- **`goto`/labels.**  Every `goto <label>` is `break '<label>` into a labelled block (or
  labelled `loop`) that carries C's own label name; the three `while (1)`/`for (;;)` bodies
  whose `continue` is reached from inside such a block are labelled too, because Rust's
  `continue` inside a labelled block is `E0695` (PORTING.md 2.2).  `switch`/`case`/`default`
  are `match` arms (with `_` for `default`); the `case A: case B:` pairs are `A | B` arms, and
  the three `FALLTHROUGH` pairs are merged arms with the first arm's extra test as an `if`.

- **`coding_system`'s bitfields.**  `mode`, `common_flags`, `annotated`, `result`,
  `src_multibyte`/`dst_multibyte` are read through the mirror's `mode_of`/`src_multibyte_p`/
  `dst_multibyte_p` and written through three unit helpers (`coding_bits_get`,
  `coding_bits_set`, `coding_mode_set`/`coding_mode_clear`) built on the mirror's
  `CODING_SYSTEM_*_SHIFT`/`_MASK`; the field names are in
  `docs/ports/macro-body-notes.tsv`.

- **`CHAR_CHARSET_P` — mirror defect, worked around at the use site.**  The
  `src/include/charset.rs` mirror's `CHAR_CHARSET_P` compares the boolean
  `CHARSET_FAST_MAP_REF!` against `0` (`error[E0308]: expected bool, found integer`), so any
  use of it fails to compile.  This unit is its first user; `encode_coding_emacs_mule` and
  `encode_coding_iso_2022` define a faithful transcription of charset.h 519-529 locally and
  shadow the mirror inside those functions.  **The mirror is not edited** (the charset lane
  owns it); the one-token fix is to drop the `!= 0` in `src/include/charset.rs:599`.

- **`CODING_INHIBIT_CHARACTER_SUBSTITUTION`.**  C's `#define … '?'` is a `char` const in the
  port (the constant gate compares the written value, and `c_int = '?' as c_int` does not
  compare equal); its uses cast it back to `c_int`.

- **`ckd_add`.**  gnulib's `<stdckdint.h>` `ckd_add (&dst, a, b)` (C 988-991) has no Rust
  form; a file-scope `ckd_add! (dst, a, b)` macro expands to
  `let (v, o) = a.overflowing_add (b); dst = v; o` (P-15's `ckd_add` family).

- **Dead regions (P-08, this file is in `cond-eval-verified.txt`).**  Recorded, not
  transcribed: `#if 0` at C 154-189, 205-243, 262-284 (the general-note templates) and
  C 6338-6361 (the `dcstruct`/`#else` pair, whose `#else` arm *is* live and is
  transcribed); `#ifdef DOS_NT` at C 5987-5989; the
  `WINDOWSNT || CYGWIN || HAVE_ANDROID` block at C 8529-8589 (with the nested
  `WINDOWSNT || CYGWIN` at C 8547-8569) and the two `WINDOWSNT` pairs at C 10432-10446 and
  10458-10472 (`#else` arms live); `ENABLE_UTF_8_CONVERTER_TEST` at C 10292-10415 and
  11911-11914.  `check-cond-eval.py src/coding.c` reports `13 conditions, 0 unknown,
  0 disagreements` (the three `#if 0`s are counted by the evaluator as conditions).

- **The eight `DEFVAR_*` carriers of the ported half (placement deviation).**  `Venable_character_translation`, `Vlatin_extra_code_table`, `Vstandard_translation_table_for_decode`, `Vstandard_translation_table_for_encode`, `disable_ascii_optimization`, `inhibit_eol_conversion`, `inhibit_iso_escape_detection` and `inhibit_null_byte_detection` are read by sections 0-9 but C defines their storage (through `struct emacs_globals`) where `syms_of_coding` registers them (C 11738+).  That section is not ported yet, so their `pub static mut` storage is written next to `Vlast_code_conversion_error` — the same precedent — and `syms_of_coding` must register (not re-define) them when it lands.  Without this, the unit object has eight own-unit references `check-undef.py` can only report as NEW.

### Decisions added by the closing window (§10 tail — §12)

- **The five functions the `coding.rs` mirror declares at its crate root stay in
  `*_def` modules.**  `code_convert_string_norecord`, `decode_file_name`,
  `encode_file_name`, `decode_coding_object` and `encode_coding_object` are crate-root
  `unsafe extern "C"` declarations in `src/include/coding.rs` (coding.h 691-713), so a
  crate-root definition is `E0428`; each definition lives in a glob-imported module
  (`decode_coding_gap_def`'s device) and the crate-root declaration still resolves to it.
  **Mirror gap, reported** (the same class as the `coding_objs` note above): moving those
  five into a `coding_fwd`-style module in the mirror would remove the deviation.

- **`preferred_coding_system` and `decode_string_utf_8` are defined at the crate root.**
  Both are declared inside this unit's own `pub mod coding_fwd` (which exists because the
  mirror lacks them), and an explicit item shadows a glob import (§0.1-10/P-39), so their
  definitions need no module and no mirror change.

- **`UNINIT` and C's "declared then assigned in the loop" locals are zero-initialised**
  (PORTING.md 1.5): `decode_coding_object`'s `destination`/`dst_bytes`/`saved_pt_byte`,
  `detect_coding_system`'s `category`/`this`/`c`/`i`/`base_category`, `char_encodable_p`'s
  `charset`, `Funencodable_char_position`'s `from`/`to` (C's `validate_subarray` writes them
  through pointers), `code_convert_string`'s `chars`/`bytes`, `encode_string_utf_8`'s
  `outbytes`/`val`, `decode_string_utf_8`'s `val`, `Fset_coding_system_priority`'s
  `priorities[]`, `Fdefine_coding_system_internal`'s `category` and `reset_...`'s locals.
  Every such site is marked in the source where the value is not obviously overwritten first.

- **`get_char_bytes`'s function-local statics live in a private module.**  C's `chars`,
  `bytes`, `nbytes` and `last_index` are function statics; Rust has none, and a crate-root
  item of those names makes every mirror parameter of the same name `E0530` ("a `let`
  binding or parameter may not shadow a `static`").  `mod get_char_bytes_caches` holds them
  (`pub static mut`, C's own names) and the function glob-imports the module.

- **`make_subsidiaries`'s `suffixes`, and the two "C has no function statics" sites.**  The
  `static char const suffixes[][8]` is a file-scope `static [[u8; 8]; 3]` with C's bytes
  (C's bound `8` therefore lives outside the body — recorded in
  `docs/ports/macro-body-notes.tsv`); `memcpy`/`strcpy` keep C's own call shape.

- **C's assignment-as-expression plist entries.**  `syms_of_coding`'s
  `Lisp_Object plist[] = { …, args[coding_arg_name] = Qno_conversion, … }` uses the value of
  an assignment, which Rust has no form for; each such entry is
  `{ let v = …; args[i] = v; v }` (one local per entry, the only deviation in that list; the
  `plist[n] = …; args[i] = plist[n];` lines are C's own two statements and keep their shape).

- **`detect_coding_charset`'s two `sizeof ("…") - 1` counts** are `std::mem::size_of_val
  (b"…") - 1`: the `sizeof` family's occurrence check wants the port to spell the member as
  often as C does, and its number check wants the literal's digits inside the call
  (docs/ports/token-aliases.tsv's `sizeof` row).

- **C's forward declaration of `reset_coding_after_pdumper_load` is deleted** (PORTING.md
  0.1-12): Rust items are order-independent, and `pdumper_do_now_and_after_load (Some
  (reset_coding_after_pdumper_load))` still names the definition below it.  The hook is
  `Option<unsafe extern "C" fn ()>` in the mirror, so the `Some` is the port's spelling of
  C's function-pointer value.

- **The twenty remaining `DEFVAR_*` carriers** (`Vcoding_system_list`, `Vcoding_system_alist`,
  `Vcoding_category_list`, `Vcoding_system_for_read`/`_write`, `Vlast_coding_system_used`,
  `inherit_process_coding_system`, `Vfile_coding_system_alist`,
  `Vprocess_coding_system_alist`, `Vnetwork_coding_system_alist`, `Vlocale_coding_system`,
  the four `eol_mnemonic_*`, `Vcharset_revision_table`, `Vdefault_process_coding_system`,
  `Vselect_safe_coding_system_function`, `coding_system_require_warning`,
  `Vtranslation_table_for_input`) are written next to the first eight (`syms_of_coding` only
  registers them, `check-globals.py`: 274 units, 0 DEFVAR without a definition).  This is the
  placement deviation the second window recorded, now for all twenty-nine.

- **The `doc: /* … */` literals decode C's escapes.**  `make-docfile` reads a comment with
  C escape semantics, so `"Decoding"` is `"Decoding"` and `\(in case …)` is `(in case …)`:
  the Rust literal must decode to *that* value, not to the raw comment text
  (`tools/cinventory.py`'s `doc_comment_value`).  Three subrs differ if it does not
  (`decode-coding-region`, `encode-coding-region`, `find-operation-coding-system`) — the
  gate reports `subr …: docstring differs`.

- **A Rust string-literal line continuation hides the rest of the body from the token
  gate.**  `tools/extract-abi.py`'s string scanner blanks `"…"` with `(?:[^"\]|\.)*`, and
  `.` does not match a newline, so `"…\<newline>…"` is not seen as a literal: the next
  quote on the *far* side starts a blanked region and every token between two literals
  disappears from the body's token set (measured on `syms_of_coding`'s first `build_string`:
  `callmany`, `flist`, `list1`, the `coding_arg_*` names and the whole plist/args tail were
  reported "does not mention").  The port keeps such literals on one physical line with
  `\n` escapes.

## Integrator note (2026-10-04, second window) — unit NOT complete

This ledger's sections 0-9 plus the first half of §10 are written; the second half of §10
(C 8063-11689: `decode_coding_object`, `encode_coding_object`, the 32 subrs, and the
coding-system API they need), §11 (`init_coding_once`) and §12 (`syms_of_coding`,
`reset_coding_after_pdumper_load`) are still `[ ]` — **54 items**: the 32 subrs, 21 functions
(`decode_coding_object`, `encode_coding_object`, `detect_coding_system`, `char_encodable_p`,
`code_convert_region`, `string_ascii_p`, `code_convert_string`,
`code_convert_string_norecord`, `get_buffer_gap_address`, `get_char_bytes`,
`encode_string_utf_8`, `convert_string_nocopy`, `decode_file_name`, `encode_file_name_1`,
`encode_file_name`, `make_subsidiaries`, `init_coding_once`, `syms_of_coding`,
`reset_coding_after_pdumper_load`) and the `UTF_8_SEQUENCE_LENGTH` macro (C 10058).

- `src/coding.rs` **compiles** (`cargo run -p xtask -- build-tu src/coding.c` →
  `built src/coding.rs -> ../build/femacs/coding.o`).
- `extract-abi.py --check emacs/src/coding.c src/coding.rs` reports **0**
  `does not mention` lines (every landed function's and macro's body tokens are accounted
  for, with the reasons in `docs/ports/macro-body-notes.tsv`) and **86** `missing` lines, all
  of them the items listed above; the `line count drifted` line still follows from the
  missing half.
- `check-undef.py` reports **0** coding-owned NEW references (the eight `DEFVAR_*` carriers
  the ported half reads are defined, see `## Decisions`); `check-symbols.py --dir src` still
  lists the unit's unported C symbols as `missing` with `invented=[]`, as expected while the
  second half is absent.
- The deviations the new windows settled are in `## Decisions` above (the mirror-gap
  `*_def`/`coding_fwd` devices, the `unsafe extern "C"` handler functions, the packed-member
  helper spellings, the `c_int` narrowing, the **open P-05 `alloca` site** in `consume_chars`,
  and the per-function macro definitions).
- `docs/ports/line-drift-tolerance.tsv`'s `coding` row is still the provisional one; it must
  be re-derived when the remainder lands.

## Integrator note (2026-10-04, third window) — unit COMPLETE, gates green

`src/coding.rs` now carries all of `emacs/src/coding.c` (C 1-12,312): the second half
of §10 (`decode_coding_object`, `encode_coding_object`, `preferred_coding_system`,
`detect_coding_system`, the `char_encodable_p` / `code_convert_region` / `code_convert_string`
/ `code_convert_string_norecord` / `string_ascii_p` helpers, `get_buffer_gap_address`,
`get_char_bytes`, `encode_string_utf_8` + `UTF_8_SEQUENCE_LENGTH` + `decode_string_utf_8`,
`convert_string_nocopy`, the file-name encoders, the 32 subrs and the coding-system API they
need), §11 `init_coding_once` and §12 `syms_of_coding` + `reset_coding_after_pdumper_load`.
The ledger above has no `[ ]` left (54 ticked).

Gate lines, verbatim (last line of each command):

- ① `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-coding3 cargo run -q -p xtask -- build-tu src/coding.c`
  → `built src/coding.rs -> ../build/femacs/coding.o`
- ② `python3 tools/extract-abi.py --check emacs/src/coding.c src/coding.rs`
  → `OK emacs/src/coding.c -> src/coding.rs` (the 86 `missing` lines are 0 and
  `does not mention` stays 0; the only problems the first run had beyond those were the three
  `docstring differs` lines the doc-escape rule above explains and the `sizeof` family's
  `8859` count)
- ③ `python3 tools/check-globals.py`
  → `check-globals: 274 units, 0 DEFVAR_* variables without a definition in their own unit`
  (274, not the 273 of the task text: the count has grown with the other lanes' units; the
  anomaly count is what matters and it is 0)
- ④ `python3 tools/check-undef.py` → exit 1, and **no `coding.o` reference appears in either
  list**: 36 NEW references (all `window.o`/`keyboard.o`/`xdisp.o`) and 12 stale ratchet
  entries, of which five are this unit's (`_Vcoding_system_for_read`,
  `_Vcoding_system_for_write`, `_Vdefault_process_coding_system`,
  `_Vlast_coding_system_used`, `_Vlocale_coding_system`, `_Vselect_safe_coding_system_function`,
  `_Vtranslation_table_for_input` — the DEFVAR carriers this window landed).  **Follow-up for
  the integrator:** those seven rows are now satisfied and must be dropped from
  `docs/baseline/undef-pending.txt` (this lane may not edit `docs/baseline/**`).
- ⑤ `python3 tools/check-mirror-dupes.py` → `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)` (unchanged from before this window); `python3 tools/check-verbatim.py`
  → `6 trees, 4607 files, 0 failures`
- ⑥ `cd femacs && rustc --edition 2024 --crate-type lib --emit obj -C panic=abort
  -C codegen-units=1 -C opt-level=0 --cfg enable_checking
  ../build/femacs/units/coding/_root.rs -o /tmp/coding-checking.o` → exit 0 (9 warnings, the
  pre-existing `unpredictable_function_pointer_comparisons` family)
- ⑦ `python3 tools/check-symbols.py --dir src` → exit 1 with
  `check-symbols: 73 units compared, 1 mismatched, 0 still pending; 387 DEFVAR carrier
  symbol(s) …`; the one mismatched unit is `src/keyboard` (this lane's `src/coding` row is
  gone from the report, i.e. `missing=[] invented=[]`; the DEFVAR carrier count went 367 → 387,
  exactly this unit's twenty new carriers, which the tool classifies as the documented
  deviation).

- **Five more names are declared at their use site** (P-72 ③, the block at the head of the
  unit that the first window started): `safe_eval` (lisp.h 4412), `strcpy` (`<string.h>`),
  `plist_put` (lisp.h 4281), `decode_live_terminal` (termhooks.h 961) and the four
  buffer-edit entry points (`insert_1_both`, `insert_from_string`, `insert_from_buffer`,
  `del_range_both`; lisp.h 4331-4348) whose mirrors do not carry them — plus `del_range_2`'s
  return type, which the first window had declared `()` and C declares `Lisp_Object`
  (lisp.h 4349).  Each carries C's own prototype in a `/// C:` line.

### What the gates above cannot see

- **The two internal UTF-8 converter subrs** (`Finternal_encode_string_utf_8`,
  `Finternal_decode_string_utf_8`), the `WINDOWSNT`/`CYGWIN`/`HAVE_ANDROID` functions
  (`from_unicode`, `from_unicode_buffer`, `to_unicode`) and the `#ifdef DOS_NT` arm are
  recorded, not transcribed; their absence is invisible to ②-⑦ because the frozen
  configuration does not compile them.  `fnmatch`-style token notes for `syms_of_coding` name
  the three dead-arm tokens the body scanner still sees.
- **No behavioural test of any kind.**  This unit has never been executed: there is no
  `coding.o` linked into a runnable Emacs, so `setup_coding_system`, the detection paths and
  `syms_of_coding`'s symbol plumbing are proven only by the compiler, the token gate and the
  symbol table.  In particular `init_coding_once`'s `iso_code_class` fills, the
  `pdumper_do_now_and_after_load` hook and the `args`/`plist` array indices in
  `syms_of_coding` are unchecked constants (a wrong index is a runtime effect the gates cannot
  reach).
- **`check-symbols.py` compares this unit's object with the reference object only for the
  symbols the reference exports**: the DEFVAR carriers are exempt (387 of them tree-wide), so a
  carrier that is *never assigned* (e.g. one C initialises in `startup.el`) still matches.
- **Two mirror defects this unit works around, still open** (the mirror is not edited here):
  `src/include/charset.rs`'s `CHAR_CHARSET_P` compares a `bool` against `0`
  (`E0308`), so `char_encodable_p` is now its *second* use site with a local transcription
  (the two `encode_coding_*` functions are the first); and `src/include/coding.rs`
  declares `code_convert_string_norecord`, `decode_file_name`, `encode_file_name`,
  `decode_coding_object` and `encode_coding_object` at its crate root, which is why five
  definitions sit in `*_def` modules.

### D7 call-relationship check

Measured with `nm -u` on both objects (`build/emacs-ref/src/coding.o` vs
`build/femacs/coding.o`): **144 undefined references in C, 164 in the port, 138 shared**.
No callee of C's source text is missing from the port's set, and the port adds no call to a
C function that C's body does not make.

The differences are three classes, none of them a changed implementation path:

* **C-only (3).**  `_globals` and `_lispsym`-style aggregation is C's single
  `struct emacs_globals` object plus the `Q*` symbol array (the port defines each `V*`
  carrier as its own symbol — the documented deviation, status 3.60 — and folds `Q*` into
  constants, P-17); `_current_thread` is how C reaches `current_buffer`/`specpdl` (thread-local
  macros; P-19/P-20 are still open for the port); `_memset_pattern16` is clang's lowering of
  `memset (SDATA (safe_charsets), 255, …)`, a libSystem optimisation helper, not a source
  callee.
* **Port-only (11).**  `_Vdeactivate_mark`, `_Vfile_name_coding_system` and
  `_Vdefault_file_name_coding_system` are other units' `DEFVAR_*` carriers reached as one
  symbol each (C reaches the same storage through `_globals`); `_current_buffer`, `_specpdl`
  and `_specpdl_ptr` are the port's extern stand-ins for C's thread-local macros (P-19's
  recorded pending item); `_sa_stack_alloc` is the `SAFE_ALLOCA` pool (P-05; C's `alloca`
  has no symbol); `_strcpy` is a libc call clang inlined in C (`strcpy (buf + len, "-unix")`
  with a constant suffix); `_cons_to_signed`/`_cons_to_unsigned` come from the mirror's
  `ConsToInteger` trait (lisp.rs 3583+), which routes the fixnum accessors' bignum slow path
  through the two helpers C keeps inline; `_symbols_with_pos_enabled` is a pre-existing
  port-side mirror variable.
* **The rest of both lists** is the mirror spelling of the same calls at the same points
  (`BVAR`→`BVAR!`, `XSETCAR`→`XSETCAR`, `SPECPDL_INDEX`→`SPECPDL_INDEX`, `CALLN`→`CALLN!`,
  `calln`→`calln!`, `USE_SAFE_ALLOCA`→`USE_SAFE_ALLOCA!`, `SAFE_ALLOCA`→`SAFE_ALLOCA!`,
  `SAFE_FREE`→`SAFE_FREE!`, `PDUMPER_REMEMBER_SCALAR`→`PDUMPER_REMEMBER_SCALAR!`,
  `DEFSYM`→`DEFSYM!`, `DEFVAR_*`→`DEFVAR_*!`, `defsubr (&Sx)`→`defsubr (&raw mut Sx)`,
  `staticpro (&x)`→`staticpro (&raw const x)`, `goto <label>`→`break '<label>`, and the
  `*mut c_void` spellings P-72 forces), plus the Rust panic/`setjmp` machinery on the port
  side.
