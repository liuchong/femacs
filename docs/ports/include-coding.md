# emacs/src/coding.h (790 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [ ] surrogates_to_codepoint  (line 761)
- [ ] build_string_from_utf8  (line 774)

## Macros defined — 62
- [ ] CHECK_CODING_SYSTEM  (line 208, body 2ffb045b1793)
- [ ] CHECK_CODING_SYSTEM_GET_ID  (line 235, body 81607d2c1f37)
- [ ] CHECK_CODING_SYSTEM_GET_SPEC  (line 219, body 207be715477e)
- [ ] CODING_ANNOTATE_CHARSET_MASK  (line 510, body db132fb76e88)
- [ ] CODING_ANNOTATE_COMPOSITION_MASK  (line 508, body f2b1649be8f5)
- [ ] CODING_ANNOTATE_DIRECTION_MASK  (line 509, body 5f399b2ba212)
- [ ] CODING_ANNOTATION_MASK  (line 507, body 29a9e9dd5eda)
- [ ] CODING_ATTR_ASCII_COMPAT  (line 154, body 663dbede70a2)
- [ ] CODING_ATTR_BASE_NAME  (line 149, body a14396298f61)
- [ ] CODING_ATTR_CATEGORY  (line 163, body ee26339d0ee2)
- [ ] CODING_ATTR_CHARSET_LIST  (line 151, body 5de26c324942)
- [ ] CODING_ATTR_DECODE_TBL  (line 155, body d31c5d6bde6a)
- [ ] CODING_ATTR_DEFAULT_CHAR  (line 160, body ab8f8e2fa295)
- [ ] CODING_ATTR_DOCSTRING  (line 153, body c63909e1b14e)
- [ ] CODING_ATTR_ENCODE_TBL  (line 156, body f02616442d36)
- [ ] CODING_ATTR_FOR_UNIBYTE  (line 161, body 2daa7d81603b)
- [ ] CODING_ATTR_MNEMONIC  (line 152, body 45cafd7e60d4)
- [ ] CODING_ATTR_PLIST  (line 162, body f6efa5419872)
- [ ] CODING_ATTR_POST_READ  (line 158, body a7aff9ebc48e)
- [ ] CODING_ATTR_PRE_WRITE  (line 159, body 113ac8e075a7)
- [ ] CODING_ATTR_SAFE_CHARSETS  (line 164, body 2e7b2e70f619)
- [ ] CODING_ATTR_TRANS_TBL  (line 157, body 1a9375b59f5c)
- [ ] CODING_ATTR_TYPE  (line 150, body 55c60e99fc75)
- [ ] CODING_FOR_UNIBYTE  (line 525, body e08052b068bb)
- [ ] CODING_FOR_UNIBYTE_MASK  (line 511, body ecdf501256ce)
- [ ] CODING_ID_ALIASES  (line 178, body 92d80fd8f1bd)
- [ ] CODING_ID_ATTRS  (line 173, body c1475cf56456)
- [ ] CODING_ID_EOL_TYPE  (line 183, body c084d72fe4fd)
- [ ] CODING_ID_NAME  (line 168, body 00a08592b493)
- [ ] CODING_MAY_REQUIRE_DECODING  (line 558, body 82f1b3cd0795)
- [ ] CODING_MODE_DIRECTION  (line 275, body cc35a7395782)
- [ ] CODING_MODE_FIXED_DESTINATION  (line 277, body 432a3e71f41d)
- [ ] CODING_MODE_LAST_BLOCK  (line 267, body 2113f7bfab26)
- [ ] CODING_MODE_SAFE_ENCODING  (line 281, body 9042d4228c01)
- [ ] CODING_MODE_SELECTIVE_DISPLAY  (line 271, body d6c81010612a)
- [ ] CODING_REQUIRE_ANNOTATION  (line 520, body d815fe3127b4)
- [ ] CODING_REQUIRE_DECODING  (line 535, body 60635ed45e40)
- [ ] CODING_REQUIRE_DECODING_MASK  (line 513, body e0b28e99ef1e)
- [ ] CODING_REQUIRE_DETECTION  (line 553, body d9c678bad73d)
- [ ] CODING_REQUIRE_DETECTION_MASK  (line 515, body 987a3bdd4907)
- [ ] CODING_REQUIRE_ENCODING  (line 545, body a9ce51c01032)
- [ ] CODING_REQUIRE_ENCODING_MASK  (line 514, body 96d5e6218d19)
- [ ] CODING_REQUIRE_FLUSHING  (line 530, body ce21a2cfd830)
- [ ] CODING_REQUIRE_FLUSHING_MASK  (line 512, body d228a45608c3)
- [ ] CODING_RESET_AT_BOL_MASK  (line 516, body 3daa00115a66)
- [ ] CODING_SYSTEM_ID  (line 195, body 62fac8841cae)
- [ ] CODING_SYSTEM_P  (line 201, body 7fde64bbe9db)
- [ ] CODING_SYSTEM_SPEC  (line 189, body bac9589ab5b3)
- [ ] DECODE_FILE  (line 652, body e43de9d18bbe)
- [ ] DECODE_SYSTEM  (line 663, body 2bce1f98cd6e)
- [ ] EMACS_CODING_H  (line 27, body da39a3ee5e6b)
- [ ] ENCODE_FILE  (line 648, body 302a5815f669)
- [ ] ENCODE_SYSTEM  (line 656, body 5d515c9a2c96)
- [ ] ENCODE_UTF_8  (line 669, body d522c93407c1)
- [ ] JIS_TO_SJIS  (line 608, body 2e5a2b0f4695)
- [ ] JIS_TO_SJIS2  (line 622, body e3b77600e70c)
- [ ] SJIS_TO_JIS  (line 567, body 52c4e8dbeb6b)
- [ ] SJIS_TO_JIS2  (line 582, body 19415dc972e3)
- [ ] UTF_16_HIGH_SURROGATE_P  (line 674, body aac34e4ea84a)
- [ ] UTF_16_LOW_SURROGATE_P  (line 680, body d012dcd2bd0d)
- [ ] decode_coding_c_string  (line 750, body 16a9aff0bb3b)
- [ ] encode_coding_string  (line 743, body 44bccf16851e)

## Types and their fields — 7
- [ ] coding_detection_info: int checked, int found, int rejected
- [ ] coding_system: ptrdiff_t id, unsigned common_flags : 14, unsigned mode : 5, bool_bf src_multibyte : 1, bool_bf dst_multibyte : 1, bool_bf chars_at_source : 1, bool_bf raw_destination : 1, bool_bf annotated : 1, bool_bf insert_before_markers : 1, unsigned eol_seen : 3, ENUM_BF (coding_result_code) result : 3, int max_charset_id, struct iso_2022_spec iso_2022, struct ccl_spec* ccl, struct utf_16_spec utf_16, enum utf_bom_type utf_8_bom, struct emacs_mule_spec emacs_mule, struct undecided_spec undecided
- [ ] composition_status: enum composition_state state, enum composition_method method, bool old_form, int length, int nchars, int ncomps
- [ ] emacs_mule_spec: struct composition_status cmp_status
- [ ] iso_2022_spec: unsigned flags, int current_invocation, int current_designation, int ctext_extended_segment_len, bool_bf single_shifting : 1, bool_bf bol : 1, bool_bf embedded_utf_8 : 1, struct composition_status cmp_status
- [ ] undecided_spec: int inhibit_nbd, int inhibit_ied, bool prefer_utf_8
- [ ] utf_16_spec: enum utf_bom_type bom, enum utf_16_endian_type endian, int surrogate

## File-scope objects — 3
- [ ] extern Lisp_Object Vcoding_system_hash_table  (line 98)
- [ ] extern struct coding_system safe_terminal_coding  (line 784)
- [ ] extern char emacs_mule_bytes  (line 786)
