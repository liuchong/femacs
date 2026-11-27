// femacs: port of src/json.c @ fd7afced403.
//
// Serializer and parser live in `#![forbid(unsafe_code)]` `crate::safe_json` (`U7` / `P-486`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum json_object_type
{
  json_object_hashtable,
  json_object_alist,
  json_object_plist,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum json_array_type
{
  json_array_array,
  json_array_list,
}

#[repr(C)]
pub struct json_configuration
{
  pub object_type: json_object_type,
  pub array_type: json_array_type,
  pub null_object: Lisp_Object,
  pub false_object: Lisp_Object,
}

pub extern "C-unwind" fn string_byte_to_pos (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  crate::safe_json::string_byte_to_pos_impl (obj, byte)
}

pub extern "C-unwind" fn string_byte_to_line (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  crate::safe_json::string_byte_to_line_impl (obj, byte)
}

pub extern "C-unwind" fn buffer_byte_to_pos (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  crate::safe_json::buffer_byte_to_pos_impl (obj, byte)
}

pub extern "C-unwind" fn buffer_byte_to_line (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  crate::safe_json::buffer_byte_to_line_impl (obj, byte)
}

DEFUN! ("json-serialize", Fjson_serialize, Sjson_serialize, 1, MANY,
       NULL,
       doc: "Return the JSON representation of OBJECT as a unibyte string.\n\nOBJECT is translated as follows:\n\n`t'        -- the JSON `true' value.\nnumber     -- a JSON number.\nstring     -- a JSON string.\nvector     -- a JSON array.\nhash-table -- a JSON object.  Keys must be strings.\nalist      -- a JSON object.  Keys must be symbols.\nplist      -- a JSON object.  Keys must be symbols.\n              A leading colon in plist key names is elided.\n\nFor duplicate object keys, the first value is used.\n\nThe Lisp equivalents to the JSON null and false values are\nconfigurable in the arguments ARGS, a list of keyword/argument pairs:\n\n:null-object OBJ -- use OBJ to represent a JSON null value.\n  It defaults to `:null'.\n\n:false-object OBJ -- use OBJ to represent a JSON false value.\n  It defaults to `:false'.\n\nIn you specify the same value for `:null-object' and `:false-object',\na potentially ambiguous situation, the JSON output will not contain\nany JSON false values.\nusage: (json-serialize OBJECT &rest ARGS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  // SAFETY: `MANY` subr ABI guarantees `args` points to `nargs >= 1` rooted `Lisp_Object`s.
  let slice = unsafe { core::slice::from_raw_parts (args, nargs as usize) };
  crate::safe_json::json_serialize_slice (slice)
});

DEFUN! ("json-insert", Fjson_insert, Sjson_insert, 1, MANY,
       NULL,
       doc: "Insert the JSON representation of OBJECT before point.\nThis is the same as (insert (json-serialize OBJECT ...)), but potentially\nfaster, and with the difference that Unicode characters are inserted as\nthemselves into multibyte buffers, and as UTF-8 byte sequences into\nunibyte buffers.\nSee the function `json-serialize' for allowed values of OBJECT and ARGS.\nusage: (json-insert OBJECT &rest ARGS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  // SAFETY: `MANY` subr ABI guarantees `args` points to `nargs >= 1` rooted `Lisp_Object`s.
  let slice = unsafe { core::slice::from_raw_parts (args, nargs as usize) };
  crate::safe_json::json_insert_slice (slice)
});

DEFUN! ("json-parse-string", Fjson_parse_string, Sjson_parse_string, 1, MANY,
       NULL,
       doc: "Parse the JSON STRING into a Lisp value.\nThis is essentially the reverse operation of `json-serialize', which\nsee.  The returned value will be the JSON null value, the JSON false\nvalue, t, a number, a string, a vector, a list, a hash-table, an alist,\nor a plist.  Its elements will be further values of these types.\nIf STRING doesn't contain a valid JSON value, this function\nsignals an error of type `json-parse-error'.\n\nThe arguments ARGS are a list of keyword/argument pairs:\n\n:object-type TYPE -- use TYPE to represent JSON objects.\n  TYPE can be `hash-table' (the default), `alist' or `plist'.\n  If an object has members with the same key, `hash-table' keeps only\n  the last value of such keys, while `alist' and `plist' keep all the\n  members.\n\n:array-type TYPE -- use TYPE to represent JSON arrays.\n  TYPE can be `array' (the default) or `list'.\n\n:null-object OBJ -- use OBJ to represent a JSON null value.\n  It defaults to `:null'.\n\n:false-object OBJ -- use OBJ to represent a JSON false value.\n  It defaults to `:false'.\nusage: (json-parse-string STRING &rest ARGS) ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  // SAFETY: `MANY` subr ABI guarantees `args` points to `nargs >= 1` rooted `Lisp_Object`s.
  let slice = unsafe { core::slice::from_raw_parts (args, nargs as usize) };
  crate::safe_json::json_parse_string_slice (slice)
});

DEFUN! ("json-parse-buffer", Fjson_parse_buffer, Sjson_parse_buffer,
       0, MANY,
       NULL,
       doc: "Read a JSON value from current buffer starting at point.\nMove point after the end of the value if parsing was successful.\nOn error, don't move point.\n\nThe returned value will be a vector, list, hashtable, alist, or\nplist.  Its elements will be the JSON null value, the JSON false\nvalue, t, numbers, strings, or further vectors, lists, hashtables,\nalists, or plists.\n\nIf the current buffer doesn't contain a valid JSON value, the\nfunction signals an error of type `json-parse-error'.\n\nThe arguments ARGS are a list of keyword/argument pairs:\n\n:object-type TYPE -- use TYPE to represent JSON objects.\n  TYPE can be `hash-table' (the default), `alist' or `plist'.\n  If an object has members with the same key, `hash-table' keeps only\n  the last value of such keys, while `alist' and `plist' keep all the\n  members.\n\n:array-type TYPE -- use TYPE to represent JSON arrays.\n  TYPE can be `array' (the default) or `list'.\n\n:null-object OBJ -- use OBJ to represent a JSON null value.\n  It defaults to `:null'.\n\n:false-object OBJ -- use OBJ to represent a JSON false value.\n  It defaults to `:false'.\nusage: (json-parse-buffer &rest args) ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  // SAFETY: `MANY` subr ABI guarantees `args` points to `nargs >= 0` rooted `Lisp_Object`s (or empty).
  let slice = if nargs <= 0 || args.is_null () {
    &[]
  } else {
    unsafe { core::slice::from_raw_parts (args, nargs as usize) }
  };
  crate::safe_json::json_parse_buffer_slice (slice)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_json ()
{
  DEFSYM! (QCnull, ":null");
  DEFSYM! (QCfalse, ":false");

  DEFSYM! (Qjson_value_p, "json-value-p");

  DEFSYM! (Qjson_error, "json-error");
  DEFSYM! (Qjson_out_of_memory, "json-out-of-memory");
  DEFSYM! (Qjson_parse_error, "json-parse-error");
  DEFSYM! (Qjson_end_of_file, "json-end-of-file");
  DEFSYM! (Qjson_trailing_content, "json-trailing-content");
  DEFSYM! (Qjson_object_too_deep, "json-object-too-deep");
  DEFSYM! (Qjson_utf8_decode_error, "json-utf8-decode-error");
  DEFSYM! (Qjson_invalid_surrogate_error, "json-invalid-surrogate-error");
  DEFSYM! (Qjson_number_out_of_range, "json-number-out-of-range-error");
  DEFSYM! (Qjson_escape_sequence_error, "json-escape-sequence-error");
  define_error_cstr (Qjson_error (), c"generic JSON error", Qerror ());
  define_error_cstr (Qjson_out_of_memory (), c"not enough memory for creating JSON object", Qjson_error ());
  define_error_cstr (Qjson_parse_error (), c"could not parse JSON stream", Qjson_error ());
  define_error_cstr (Qjson_end_of_file (), c"end of JSON stream", Qjson_parse_error ());
  define_error_cstr (Qjson_trailing_content (), c"trailing content after JSON stream", Qjson_parse_error ());
  define_error_cstr (Qjson_object_too_deep (), c"object cyclic or Lisp evaluation too deep", Qjson_error ());
  define_error_cstr (Qjson_utf8_decode_error (), c"invalid utf-8 encoding", Qjson_error ());
  define_error_cstr (Qjson_invalid_surrogate_error (), c"invalid surrogate pair", Qjson_error ());
  define_error_cstr (Qjson_number_out_of_range (), c"number out of range", Qjson_error ());
  define_error_cstr (Qjson_escape_sequence_error (), c"invalid escape sequence", Qjson_parse_error ());

  DEFSYM! (QCobject_type, ":object-type");
  DEFSYM! (QCarray_type, ":array-type");
  DEFSYM! (QCnull_object, ":null-object");
  DEFSYM! (QCfalse_object, ":false-object");
  DEFSYM! (Qalist, "alist");
  DEFSYM! (Qplist, "plist");
  DEFSYM! (Qarray, "array");

  defsubr_global (&Sjson_serialize);
  defsubr_global (&Sjson_insert);
  defsubr_global (&Sjson_parse_string);
  defsubr_global (&Sjson_parse_buffer);
}
