# emacs/src/coding.h (790 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] surrogates_to_codepoint  (line 761)
- [x] build_string_from_utf8  (line 774)

## Macros defined — 62
- [x] CHECK_CODING_SYSTEM  (line 208, body 2ffb045b1793)
- [x] CHECK_CODING_SYSTEM_GET_ID  (line 235, body 81607d2c1f37)
- [x] CHECK_CODING_SYSTEM_GET_SPEC  (line 219, body 207be715477e)
- [x] CODING_ANNOTATE_CHARSET_MASK  (line 510, body db132fb76e88)
- [x] CODING_ANNOTATE_COMPOSITION_MASK  (line 508, body f2b1649be8f5)
- [x] CODING_ANNOTATE_DIRECTION_MASK  (line 509, body 5f399b2ba212)
- [x] CODING_ANNOTATION_MASK  (line 507, body 29a9e9dd5eda)
- [x] CODING_ATTR_ASCII_COMPAT  (line 154, body 663dbede70a2)
- [x] CODING_ATTR_BASE_NAME  (line 149, body a14396298f61)
- [x] CODING_ATTR_CATEGORY  (line 163, body ee26339d0ee2)
- [x] CODING_ATTR_CHARSET_LIST  (line 151, body 5de26c324942)
- [x] CODING_ATTR_DECODE_TBL  (line 155, body d31c5d6bde6a)
- [x] CODING_ATTR_DEFAULT_CHAR  (line 160, body ab8f8e2fa295)
- [x] CODING_ATTR_DOCSTRING  (line 153, body c63909e1b14e)
- [x] CODING_ATTR_ENCODE_TBL  (line 156, body f02616442d36)
- [x] CODING_ATTR_FOR_UNIBYTE  (line 161, body 2daa7d81603b)
- [x] CODING_ATTR_MNEMONIC  (line 152, body 45cafd7e60d4)
- [x] CODING_ATTR_PLIST  (line 162, body f6efa5419872)
- [x] CODING_ATTR_POST_READ  (line 158, body a7aff9ebc48e)
- [x] CODING_ATTR_PRE_WRITE  (line 159, body 113ac8e075a7)
- [x] CODING_ATTR_SAFE_CHARSETS  (line 164, body 2e7b2e70f619)
- [x] CODING_ATTR_TRANS_TBL  (line 157, body 1a9375b59f5c)
- [x] CODING_ATTR_TYPE  (line 150, body 55c60e99fc75)
- [x] CODING_FOR_UNIBYTE  (line 525, body e08052b068bb)
- [x] CODING_FOR_UNIBYTE_MASK  (line 511, body ecdf501256ce)
- [x] CODING_ID_ALIASES  (line 178, body 92d80fd8f1bd)
- [x] CODING_ID_ATTRS  (line 173, body c1475cf56456)
- [x] CODING_ID_EOL_TYPE  (line 183, body c084d72fe4fd)
- [x] CODING_ID_NAME  (line 168, body 00a08592b493)
- [x] CODING_MAY_REQUIRE_DECODING  (line 558, body 82f1b3cd0795)
- [x] CODING_MODE_DIRECTION  (line 275, body cc35a7395782)
- [x] CODING_MODE_FIXED_DESTINATION  (line 277, body 432a3e71f41d)
- [x] CODING_MODE_LAST_BLOCK  (line 267, body 2113f7bfab26)
- [x] CODING_MODE_SAFE_ENCODING  (line 281, body 9042d4228c01)
- [x] CODING_MODE_SELECTIVE_DISPLAY  (line 271, body d6c81010612a)
- [x] CODING_REQUIRE_ANNOTATION  (line 520, body d815fe3127b4)
- [x] CODING_REQUIRE_DECODING  (line 535, body 60635ed45e40)
- [x] CODING_REQUIRE_DECODING_MASK  (line 513, body e0b28e99ef1e)
- [x] CODING_REQUIRE_DETECTION  (line 553, body d9c678bad73d)
- [x] CODING_REQUIRE_DETECTION_MASK  (line 515, body 987a3bdd4907)
- [x] CODING_REQUIRE_ENCODING  (line 545, body a9ce51c01032)
- [x] CODING_REQUIRE_ENCODING_MASK  (line 514, body 96d5e6218d19)
- [x] CODING_REQUIRE_FLUSHING  (line 530, body ce21a2cfd830)
- [x] CODING_REQUIRE_FLUSHING_MASK  (line 512, body d228a45608c3)
- [x] CODING_RESET_AT_BOL_MASK  (line 516, body 3daa00115a66)
- [x] CODING_SYSTEM_ID  (line 195, body 62fac8841cae)
- [x] CODING_SYSTEM_P  (line 201, body 7fde64bbe9db)
- [x] CODING_SYSTEM_SPEC  (line 189, body bac9589ab5b3)
- [x] DECODE_FILE  (line 652, body e43de9d18bbe)
- [x] DECODE_SYSTEM  (line 663, body 2bce1f98cd6e)
- [x] EMACS_CODING_H  (line 27, body da39a3ee5e6b)
- [x] ENCODE_FILE  (line 648, body 302a5815f669)
- [x] ENCODE_SYSTEM  (line 656, body 5d515c9a2c96)
- [x] ENCODE_UTF_8  (line 669, body d522c93407c1)
- [x] JIS_TO_SJIS  (line 608, body 2e5a2b0f4695)
- [x] JIS_TO_SJIS2  (line 622, body e3b77600e70c)
- [x] SJIS_TO_JIS  (line 567, body 52c4e8dbeb6b)
- [x] SJIS_TO_JIS2  (line 582, body 19415dc972e3)
- [x] UTF_16_HIGH_SURROGATE_P  (line 674, body aac34e4ea84a)
- [x] UTF_16_LOW_SURROGATE_P  (line 680, body d012dcd2bd0d)
- [x] decode_coding_c_string  (line 750, body 16a9aff0bb3b)
- [x] encode_coding_string  (line 743, body 44bccf16851e)

## Types and their fields — 7
- [x] coding_detection_info: int checked, int found, int rejected
- [x] coding_system: ptrdiff_t id, unsigned common_flags : 14, unsigned mode : 5, bool_bf src_multibyte : 1, bool_bf dst_multibyte : 1, bool_bf chars_at_source : 1, bool_bf raw_destination : 1, bool_bf annotated : 1, bool_bf insert_before_markers : 1, unsigned eol_seen : 3, ENUM_BF (coding_result_code) result : 3, int max_charset_id, struct iso_2022_spec iso_2022, struct ccl_spec* ccl, struct utf_16_spec utf_16, enum utf_bom_type utf_8_bom, struct emacs_mule_spec emacs_mule, struct undecided_spec undecided
- [x] composition_status: enum composition_state state, enum composition_method method, bool old_form, int length, int nchars, int ncomps
- [x] emacs_mule_spec: struct composition_status cmp_status
- [x] iso_2022_spec: unsigned flags, int current_invocation, int current_designation, int ctext_extended_segment_len, bool_bf single_shifting : 1, bool_bf bol : 1, bool_bf embedded_utf_8 : 1, struct composition_status cmp_status
- [x] undecided_spec: int inhibit_nbd, int inhibit_ied, bool prefer_utf_8
- [x] utf_16_spec: enum utf_bom_type bom, enum utf_16_endian_type endian, int surrogate

## File-scope objects — 3
- [x] extern Lisp_Object Vcoding_system_hash_table  (line 98)
- [x] extern struct coding_system safe_terminal_coding  (line 784)
- [x] extern char emacs_mule_bytes  (line 786)

# ----------------------------------------------------------------------
## Decisions

- **Bitfield storage.**  `struct coding_system` packs ten members (`common_flags:14`, `mode:5`,
  six `bool_bf:1`, `eol_seen:3`, `ENUM_BF (coding_result_code) result:3`) into one 4-byte unit
  at offset 8; the frozen clang's word for `common_flags` 5, `mode` 7, all six bools set,
  `eol_seen` 3, `result` 4 is `0x47f9c005`, which fixes every window.  The port stores the unit
  as `bits: u32` and names each field's mask/shift (PORTING.md 1.3, the `glyph_slice`
  precedent); `CODING_REQUIRE_*`/`CODING_FOR_UNIBYTE` read it through those constants.  The
  three `iso_2022_spec` flags are the same story (`bits: u32` at offset 32).
- **Incomplete type.**  `struct ccl_spec` is defined in `ccl.h`, which this header does not
  include; C names it only as a pointer.  The mirror declares the stand-in `pub struct ccl_spec;`
  and deletes it when `ccl.rs` lands (PORTING.md P-70).
- **Out-parameter macros.**  `CHECK_CODING_SYSTEM_GET_SPEC` and `CHECK_CODING_SYSTEM_GET_ID`
  assign a local in C; the port returns that value.  `SJIS_TO_JIS`/`SJIS_TO_JIS2`/
  `JIS_TO_SJIS`/`JIS_TO_SJIS2` rewrite their `code` argument; the port returns it.
- **`do { … } while (false)`.**  Rust has no do-while, so those macros become functions that
  run the body once; the missing `false` token is recorded in `docs/ports/macro-body-notes.tsv`,
  together with the `common_flags`/`mode`/`src_multibyte`/`dst_multibyte` tokens the packed unit
  cannot mention and the `Qnil` the port spells `lisp_h_Qnil ()` (P-17).
- **Declarations the header itself needs.**  `hash_find`/`hash_put` (lisp.h 4254-4258) were
  added to `lisp.rs` because `CODING_SYSTEM_ID` calls the first; the prototypes this header
  declares are mirrored as `unsafe extern "C"` blocks (`code_convert_string_norecord`,
  `encode_file_name`, `decode_file_name`, `decode_coding_gap`, `decode_coding_object`,
  `encode_coding_object`, `make_string_from_utf8`).
- **Not yet done: none.**  `python3 tools/extract-abi.py --check emacs/src/coding.h
  src/include/coding.rs` reports `OK` for every function, macro, type, extern and global
  of the frozen configuration.

# ----------------------------------------------------------------------
## Decision and Evidence (`composite.rs`, the include C has at 284, 2026-10-05)

- `coding.h` 284 includes `composite.h`, and the type layer below takes two names from it:
  composite.h 50's `MAX_COMPOSITION_COMPONENTS` (it sizes `composition_status.carryover`,
  306-310) and `composition_method` (which types that record's `method`).  This mirror names
  both, so it needs the include for any unit whose own list does not supply `composite.rs` -
  and `src/androidmenu.c` is such a unit (it reaches this mirror through `keyboard.h` 23 once
  the fix recorded in `docs/ports/include-keyboard.md` lands).
- **The line sits on the header block's blank line** (between the guard constant and the
  `ccl_spec` note), not in a block of its own: `docs/baseline/citations-allowlist.tsv` keys four
  rows of this file by line (170, 215, 222, 311), so an insertion above them would move the
  findings they allow and hand the integrator a shared-table retarget (the 3.283 precedent in
  `docs/ports/include-dispextern.md`).  The file keeps C's count of lines, and C's order is
  kept too - `composite.h` before the record that reads it.

## Evidence (`composite.rs`)

- `python3 tools/extract-abi.py --check emacs/src/coding.h src/include/coding.rs` -> `OK
  emacs/src/coding.h -> src/include/coding.rs` (the include adds no item the comparison
  can see).
- `python3 tools/check-citations.py` -> `675 file(s) scanned, 0 drift, 0 range, 351 external,
  33 allowed, 0 stale` (the placement is what keeps the four rows above matching).
- `build-tu src/coding.c` -> `built src/coding.rs -> ../build/femacs/coding.o` (unchanged: that
  unit includes `composite.rs` at 64, before `coding.rs` at 65, so this line is deduped there).
- The unit the include exists for: `--cfg 'feature="have_android"'` arm-flip of
  `build/femacs/units/androidmenu/_root.rs` -> exit 0, 0 `^error` lines (89,808-byte object).

## Declarations (status 4.996)

C's prototypes for functions other units define now stand in `coding_fn_decls` (module + glob
import, P-297/P-39): **12** rows, each with its C line and text, `pub` so the glob brings them out,
and carrying the gate of the records it names.

The use-site copies were withdrawn only where `tools/check-decl-withdraw.py` could prove both
things P-323 asks for: the unit includes this mirror itself and the mirror's declaration carries no
cfg, and the two signatures are identical after whitespace and `pub` are normalised. Copies that
fail either test stay, and are ratcheted in `docs/baseline/mirror-dupes.tsv` with the reason
(the four readings in the batch: 342 withdrawn, 57 duplicates, 0 failures).
