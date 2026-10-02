# emacs/src/json.c (1851 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 4
- [x] json-serialize  (Fjson_serialize, 1/MANY, doc 297db3dd47e2, line 589)
- [x] json-insert  (Fjson_insert, 1/MANY, doc b68c2ced8d49, line 627)
- [x] json-parse-string  (Fjson_parse_string, 1/MANY, doc 4be59a8668f7, line 1665)
- [x] json-parse-buffer  (Fjson_parse_buffer, 0/MANY, doc 8fc63522cdb9, line 1733)

## Functions defined — 63
- [x] json_parse_args  (line 54, static)
- [x] symset_size  (line 135, static)
- [x] make_symset_table  (line 141, static)
- [x] push_symset  (line 156, static)
- [x] pop_symset  (line 166, static)
- [x] cleanup_symset_tables  (line 174, static)
- [x] symset_hash  (line 185, static)
- [x] symset_expand  (line 192, static)
- [x] symset_add  (line 220, static)
- [x] json_out_grow_buf  (line 247, static)
- [x] cleanup_json_out  (line 255, static)
- [x] json_make_room  (line 265, static)
- [x] json_out_str  (line 276, static)
- [x] json_out_byte  (line 284, static)
- [x] json_out_fixnum  (line 291, static)
- [x] string_not_unicode  (line 300, static)
- [x] json_out_string  (line 320, static)
- [x] json_out_nest  (line 406, static)
- [x] json_out_unnest  (line 414, static)
- [x] json_out_object_cons  (line 422, static)
- [x] json_out_object_hash  (line 473, static)
- [x] json_out_array  (line 497, static)
- [x] json_out_float  (line 513, static)
- [x] json_out_bignum  (line 526, static)
- [x] json_out_something  (line 536, static)
- [x] json_serialize  (line 565, static)
- [x] json_signal_error  (line 725, static)
- [x] json_parser_init  (line 737, static)
- [x] json_parser_done  (line 791, static)
- [x] json_make_object_workspace_for_slow_path  (line 803, static)
- [x] json_make_object_workspace_for  (line 821)
- [x] json_byte_workspace_reset  (line 832, static)
- [x] json_byte_workspace_put_slow_path  (line 839, static)
- [x] json_byte_workspace_put  (line 861)
- [x] json_input_at_eof  (line 875, static)
- [x] json_input_switch_to_secondary  (line 884, static)
- [x] json_input_get_slow_path  (line 903, static)
- [x] json_input_get  (line 911, static)
- [x] json_input_get_if_possible  (line 921, static)
- [x] json_input_put_back  (line 933, static)
- [x] is_json_whitespace  (line 939, static)
- [x] json_skip_whitespace  (line 947, static)
- [x] json_skip_whitespace_if_possible  (line 961, static)
- [x] json_hex_value  (line 972, static)
- [x] json_parse_unicode  (line 986, static)
- [x] utf8_error  (line 1001, static)
- [x] json_parse_string  (line 1009, static)
- [x] json_create_integer  (line 1193, static)
- [x] json_create_float  (line 1220, static)
- [x] json_parse_number  (line 1238, static)
- [x] json_parse_array  (line 1347, static)
- [x] json_parse_object_member_value  (line 1425, static)
- [x] json_parse_object  (line 1438, static)
- [x] json_is_token_char  (line 1554, static)
- [x] json_parse_value  (line 1561, static)
- [x] json_parse  (line 1624, static)
- [x] count_chars  (line 1631, static)
- [x] count_newlines  (line 1641, static)
- [x] string_byte_to_pos  (line 1650, static)
- [x] string_byte_to_line  (line 1658, static)
- [x] buffer_byte_to_pos  (line 1716, static)
- [x] buffer_byte_to_line  (line 1723, static)
- [x] syms_of_json  (line 1803)

## Macros defined — 3
- [x] JSON_OUT_STR  (line 272, body 8195f95e6b58)
- [x] JSON_PARSER_INTERNAL_BYTE_WORKSPACE_SIZE  (line 673, body ce09b127d48f)
- [x] JSON_PARSER_INTERNAL_OBJECT_WORKSPACE_SIZE  (line 672, body c66c65175fec)

## Types and their fields — 3
- [x] json_configuration: enum json_object_type object_type, enum json_array_type array_type, Lisp_Object null_object, Lisp_Object false_object
- [x] json_parser: const unsigned char* input_current, const unsigned char* input_begin, const unsigned char* input_end, const unsigned char* secondary_input_begin, const unsigned char* secondary_input_end, int available_depth, struct json_configuration conf, ptrdiff_t additional_bytes_count, Lisp_Object internal_object_workspace, Lisp_Object* object_workspace, ptrdiff_t object_workspace_size, ptrdiff_t object_workspace_current, unsigned char internal_byte_workspace, unsigned char* byte_workspace, unsigned char* byte_workspace_end, unsigned char* byte_workspace_current, Lisp_Object obj, ptrdiff_t (*) byte_to_pos, ptrdiff_t (*) byte_to_line
- [x] symset_tbl: struct symset_tbl* up, Lisp_Object entries

## File-scope objects — 0

## Decisions

- **json.c has no `#if` at all.**  The only macros it defines are the two workspace sizes and
  JSON_OUT_STR, so nothing here is an arm decision.
- **The three macros**: C 672-673's workspace sizes are `pub const` items (`usize`, the array
  lengths of `struct json_parser`); C 272's `JSON_OUT_STR (jo, str)` is a `macro_rules!` whose
  body takes the byte count from the literal (`to_bytes ().len ()`), which is what C's
  `sizeof (str) - 1` computes - the `sizeof` and `1` tokens are in
  docs/ports/macro-body-notes.tsv.
- **`struct symset_tbl` ends in C's flexible array member** `Lisp_Object entries[]`; the port
  declares `entries: [Lisp_Object; 0]` and reaches the elements through `FAM_ELEM!` (P-40),
  which is what makes `st->entries[i] = Qunbound` a place expression.
- **The two `switch` pairs and the escape `switch` are `match`es** with C's arms in C's order
  (`json_object_type`'s and `json_array_type`'s variants are imported so C's bare enumerators
  keep their spelling); `default:` becomes `_ =>`.  Four notes rows record the `switch`/`case`/
  `default`/`break` tokens that a `match` cannot spell (the tparam1 precedent).
- **`struct json_parser`'s two function pointers are `Option<unsafe extern "C" fn (…)>`** and
  are called with `.unwrap () (…)`, the vm-limit.rs shape; the six functions whose address
  escapes (`cleanup_json_out`, `json_parser_done`, `string_byte_to_pos`, `string_byte_to_line`,
  `buffer_byte_to_pos`, `buffer_byte_to_line`) are therefore `pub unsafe extern "C" fn`.
- **`INT_BUFSIZE_BOUND` is instantiated locally** for a signed 64-bit `EMACS_INT` (= 21) with
  data.rs's derivation, because the C macro is type-generic and `<intprops.h>` has no generic
  mirror (P-26).
- **The double type is `c_double`** (the `double` token-aliases family), because the family's
  port-side scan skips short tokens and `f64` is one.
- **Byte literals that carry a brace or a quote are written numerically** (`0x7b`, `0x7d`,
  `0x22`): the C-side body scanner blanks literals before counting braces, but the Rust side
  does not, so `b'{'`/`b'"'` inside the port would unbalance the span.  (The same bug bit the C
  side for `case '{':` in json_parse_value and was fixed in tools/cinventory.py: the span
  scanner now counts braces on a literal-blanked view, and `static` is found wherever it
  appears in the declaration prefix, which is what C 802's `NO_INLINE static void` needs.)
- **The line count matches**: C 1,851 vs the port's 1,845 (no tolerance needed).
- **Call-relationship differences (D7)**: none.  The unit calls the mirror's allocator,
  buffer, hash-table, string and error entry points exactly as C does.
