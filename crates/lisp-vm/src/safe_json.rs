//! `#![forbid(unsafe_code)]` RFC 8259 JSON serializer and parser (`src/json.rs`, `U7` / `P-486`).
//!
//! Replaces `xpalloc`/`xmalloc`/`symset_tbl` flexible-array tables and raw `object_workspace`
//! pointers with safe `Vec<u8>`, `HashSet<isize>` symbol deduplication, and GC-safe `RootedVec`
//! object/array workspaces.

#![forbid(unsafe_code)]

use std::collections::HashSet;

use crate::common::*;
use crate::roots::RootedVec;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JsonObjectType {
    Hashtable,
    Alist,
    Plist,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JsonArrayType {
    Array,
    List,
}

#[derive(Clone, Copy)]
struct JsonConfiguration {
    object_type: JsonObjectType,
    array_type: JsonArrayType,
    null_object: Lisp_Object,
    false_object: Lisp_Object,
}

impl Default for JsonConfiguration {
    fn default() -> Self {
        Self {
            object_type: JsonObjectType::Hashtable,
            array_type: JsonArrayType::Array,
            null_object: QCnull(),
            false_object: QCfalse(),
        }
    }
}

fn json_parse_args(args: &[Lisp_Object], conf: &mut JsonConfiguration, parse_object_types: bool) {
    if !args.len().is_multiple_of(2) {
        let mut copy = RootedVec::from_slice(args);
        wrong_type_argument(Qplistp(), Flist_slice(copy.as_mut_slice()));
    }

    let mut i = args.len();
    while i > 0 {
        let key = args[i - 2];
        let value = args[i - 1];
        if parse_object_types && EQ(key, QCobject_type()) {
            if EQ(value, Qhash_table()) {
                conf.object_type = JsonObjectType::Hashtable;
            } else if EQ(value, Qalist()) {
                conf.object_type = JsonObjectType::Alist;
            } else if EQ(value, Qplist()) {
                conf.object_type = JsonObjectType::Plist;
            } else {
                wrong_choice(list3(Qhash_table(), Qalist(), Qplist()), value);
            }
        } else if parse_object_types && EQ(key, QCarray_type()) {
            if EQ(value, Qarray()) {
                conf.array_type = JsonArrayType::Array;
            } else if EQ(value, Qlist()) {
                conf.array_type = JsonArrayType::List;
            } else {
                wrong_choice(list2(Qarray(), Qlist()), value);
            }
        } else if EQ(key, QCnull_object()) {
            conf.null_object = value;
        } else if EQ(key, QCfalse_object()) {
            conf.false_object = value;
        } else if parse_object_types {
            wrong_choice(
                list4(
                    QCobject_type(),
                    QCarray_type(),
                    QCnull_object(),
                    QCfalse_object(),
                ),
                value,
            );
        } else {
            wrong_choice(list2(QCnull_object(), QCfalse_object()), value);
        }
        i -= 2;
    }
}

static JSON_PLAIN_CHAR: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

struct JsonOut {
    buf: Vec<u8>,
    chars_delta: ptrdiff_t,
    maxdepth: c_int,
    conf: JsonConfiguration,
}

#[inline(always)]
fn string_not_unicode(obj: Lisp_Object) -> ! {
    wrong_type_argument(Qjson_value_p(), obj)
}

impl JsonOut {
    fn out_string(&mut self, str_obj: Lisp_Object, skip: usize) {
        const HEXCHAR: [u8; 16] = *b"0123456789ABCDEF";
        let bytes = lisp_string_bytes_vec(str_obj);
        let multibyte = STRING_MULTIBYTE(str_obj);
        self.buf.reserve(bytes.len() + 2);
        self.buf.push(b'"');

        let mut p = skip.min(bytes.len());
        let end = bytes.len();
        while p < end {
            let run = p;
            while p < end && JSON_PLAIN_CHAR[bytes[p] as usize] != 0 {
                p += 1;
            }
            if p > run {
                self.buf.extend_from_slice(&bytes[run..p]);
                continue;
            }

            let c = bytes[p];
            if c > 0x7f {
                if multibyte {
                    let n: usize = if c <= 0xc1 {
                        string_not_unicode(str_obj);
                    } else if c <= 0xdf {
                        if p + 2 > end {
                            string_not_unicode(str_obj);
                        }
                        2
                    } else if c <= 0xef {
                        if p + 3 > end {
                            string_not_unicode(str_obj);
                        }
                        let v = (((c as c_int) & 0x0f) << 12)
                            + (((bytes[p + 1] as c_int) & 0x3f) << 6)
                            + ((bytes[p + 2] as c_int) & 0x3f);
                        if char_surrogate_p(v) {
                            string_not_unicode(str_obj);
                        }
                        3
                    } else if c <= 0xf7 {
                        if p + 4 > end {
                            string_not_unicode(str_obj);
                        }
                        let v = (((c as c_int) & 0x07) << 18)
                            + (((bytes[p + 1] as c_int) & 0x3f) << 12)
                            + (((bytes[p + 2] as c_int) & 0x3f) << 6)
                            + ((bytes[p + 3] as c_int) & 0x3f);
                        if v > MAX_UNICODE_CHAR {
                            string_not_unicode(str_obj);
                        }
                        4
                    } else {
                        string_not_unicode(str_obj);
                    };
                    self.buf.extend_from_slice(&bytes[p..p + n]);
                    self.chars_delta += (n as ptrdiff_t) - 1;
                    p += n;
                } else {
                    string_not_unicode(str_obj);
                }
            } else {
                self.buf.push(b'\\');
                match c {
                    b'"' | b'\\' => self.buf.push(c),
                    0x08 => self.buf.push(b'b'),
                    0x09 => self.buf.push(b't'),
                    0x0a => self.buf.push(b'n'),
                    0x0c => self.buf.push(b'f'),
                    0x0d => self.buf.push(b'r'),
                    _ => {
                        self.buf.extend_from_slice(&[
                            b'u',
                            b'0',
                            b'0',
                            HEXCHAR[(c >> 4) as usize],
                            HEXCHAR[(c & 0x0f) as usize],
                        ]);
                    }
                }
                p += 1;
            }
        }
        self.buf.push(b'"');
    }

    #[inline(always)]
    fn nest(&mut self) {
        self.maxdepth -= 1;
        if self.maxdepth < 0 {
            error_json_max_depth();
        }
    }

    #[inline(always)]
    fn unnest(&mut self) {
        self.maxdepth += 1;
    }

    fn out_object_cons(&mut self, obj: Lisp_Object) {
        self.nest();
        let mut seen = HashSet::with_capacity(16);
        self.buf.push(b'{');
        let is_alist = CONSP(XCAR(obj));
        let mut first = true;
        let mut tail = obj;
        FOR_EACH_TAIL!(tail, {
            let (raw_key, value) = if is_alist {
                let pair = XCAR(tail);
                CHECK_CONS(pair);
                (XCAR(pair), XCDR(pair))
            } else {
                let k = XCAR(tail);
                tail = XCDR(tail);
                CHECK_CONS(tail);
                (k, XCAR(tail))
            };
            let key = maybe_remove_pos_from_symbol(raw_key);
            CHECK_TYPE(BARE_SYMBOL_P(key), Qsymbolp(), key);

            if seen.insert(XLI(key)) {
                if !first {
                    self.buf.push(b',');
                }
                first = false;

                let key_str = SYMBOL_NAME(key);
                let skip = if !is_alist && SBYTES(key_str) > 1 && SREF(key_str, 0) == b':' {
                    1
                } else {
                    0
                };
                self.out_string(key_str, skip);
                self.buf.push(b':');
                self.out_something(value);
            }
        });
        CHECK_LIST_END(tail, obj);
        self.buf.push(b'}');
        self.unnest();
    }

    fn out_object_hash(&mut self, obj: Lisp_Object) {
        self.nest();
        self.buf.push(b'{');
        let pairs = hash_table_collect_pairs(obj);
        let mut first = true;
        for (k, v) in pairs {
            if !first {
                self.buf.push(b',');
            }
            first = false;
            CHECK_STRING(k);
            self.out_string(k, 0);
            self.buf.push(b':');
            self.out_something(v);
        }
        self.buf.push(b'}');
        self.unnest();
    }

    fn out_array(&mut self, obj: Lisp_Object) {
        self.nest();
        self.buf.push(b'[');
        let n = ASIZE(obj);
        for i in 0..n {
            if i > 0 {
                self.buf.push(b',');
            }
            self.out_something(AREF(obj, i));
        }
        self.buf.push(b']');
        self.unnest();
    }

    fn out_float(&mut self, f: Lisp_Object) {
        let x = XFLOAT_DATA(f);
        if !x.is_finite() {
            signal_json_inf_or_nan(f);
        }
        float_append_decimal(x, &mut self.buf);
    }

    fn out_something(&mut self, obj: Lisp_Object) {
        if EQ(obj, self.conf.null_object) {
            self.buf.extend_from_slice(b"null");
        } else if EQ(obj, self.conf.false_object) {
            self.buf.extend_from_slice(b"false");
        } else if EQ(obj, Qt()) {
            self.buf.extend_from_slice(b"true");
        } else if NILP(obj) {
            self.buf.extend_from_slice(b"{}");
        } else if FIXNUMP(obj) {
            fixnum_append_decimal(XFIXNUM(obj), &mut self.buf);
        } else if STRINGP(obj) {
            self.out_string(obj, 0);
        } else if CONSP(obj) {
            self.out_object_cons(obj);
        } else if FLOATP(obj) {
            self.out_float(obj);
        } else if HASH_TABLE_P(obj) {
            self.out_object_hash(obj);
        } else if VECTORP(obj) {
            self.out_array(obj);
        } else if BIGNUMP(obj) {
            bignum_append_decimal(obj, &mut self.buf);
        } else {
            wrong_type_argument(Qjson_value_p(), obj);
        }
    }
}

fn serialize_to_out(object: Lisp_Object, kw_args: &[Lisp_Object]) -> JsonOut {
    let mut conf = JsonConfiguration::default();
    json_parse_args(kw_args, &mut conf, false);

    let mut jo = JsonOut {
        buf: Vec::with_capacity(64),
        chars_delta: 0,
        maxdepth: 50,
        conf,
    };

    let count = SPECPDL_INDEX();
    if !NILP(Vfloat_output_format.get()) {
        specbind(Qfloat_output_format(), Qnil());
    }
    jo.out_something(object);
    unbind_to(count, Qnil());
    jo
}

pub fn json_serialize_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let jo = serialize_to_out(args[0], &args[1..]);
    make_unibyte_string_bytes(&jo.buf)
}

pub fn json_insert_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let jo = serialize_to_out(args[0], &args[1..]);

    let pt = current_buffer_pt();
    let pt_byte = current_buffer_pt_byte();
    prepare_to_modify_buffer_simple(pt, pt);
    let pt = current_buffer_pt();
    let pt_byte = current_buffer_pt_byte();
    move_gap_both(pt, pt_byte);
    write_into_current_buffer_gap(&jo.buf);

    let ub_buffer = !current_buffer_multibyte_p();
    let inserted_bytes = jo.buf.len() as ptrdiff_t;
    let inserted = if ub_buffer {
        inserted_bytes
    } else {
        inserted_bytes - jo.chars_delta
    };
    eassert!(inserted > 0);

    insert_from_gap_1(inserted, inserted_bytes, false);
    invalidate_current_buffer_caches(pt, pt + inserted);
    adjust_after_insert(
        pt,
        pt_byte,
        pt + inserted,
        pt_byte + inserted_bytes,
        inserted,
    );
    signal_after_change(pt, 0, inserted);
    update_compositions(pt, pt, CHECK_BORDER);
    SET_PT_BOTH(pt + inserted, pt_byte + inserted_bytes);

    Qnil()
}

pub(crate) fn count_utf8_chars(bytes: &[u8]) -> ptrdiff_t {
    let mut nchars: ptrdiff_t = 0;
    for &b in bytes {
        if (b & 0xc0) != 0x80 {
            nchars += 1;
        }
    }
    nchars
}

pub(crate) fn count_newlines_slice(bytes: &[u8]) -> ptrdiff_t {
    let mut nls: ptrdiff_t = 0;
    for &b in bytes {
        if b == b'\n' {
            nls += 1;
        }
    }
    nls
}

enum JsonInputSource {
    Slice {
        bytes: Vec<u8>,
        multibyte: bool,
    },
    CurrentBuffer {
        start_pt: ptrdiff_t,
        start_pt_byte: ptrdiff_t,
        end_byte: ptrdiff_t,
        chunk: [u8; 256],
        chunk_base: usize,
        chunk_len: usize,
    },
}

struct SafeJsonParser {
    source: JsonInputSource,
    cursor: usize,
    total_len: usize,
    available_depth: c_int,
    conf: JsonConfiguration,
    object_workspace: RootedVec,
    byte_workspace: Vec<u8>,
}

impl SafeJsonParser {
    fn from_lisp_string(string: Lisp_Object, conf: JsonConfiguration) -> Self {
        let bytes = lisp_string_bytes_vec(string);
        let total_len = bytes.len();
        Self {
            source: JsonInputSource::Slice {
                bytes,
                multibyte: STRING_MULTIBYTE(string),
            },
            cursor: 0,
            total_len,
            available_depth: 10000,
            conf,
            object_workspace: RootedVec::with_capacity(64),
            byte_workspace: Vec::with_capacity(512),
        }
    }

    fn from_current_buffer(conf: JsonConfiguration) -> Self {
        let start_pt = current_buffer_pt();
        let start_pt_byte = current_buffer_pt_byte();
        let zv = current_buffer_zv();
        let end_byte = if start_pt == zv {
            start_pt_byte
        } else {
            current_buffer_zv_byte()
        };
        let total_len = (end_byte - start_pt_byte).max(0) as usize;
        Self {
            source: JsonInputSource::CurrentBuffer {
                start_pt,
                start_pt_byte,
                end_byte,
                chunk: [0; 256],
                chunk_base: 0,
                chunk_len: 0,
            },
            cursor: 0,
            total_len,
            available_depth: 10000,
            conf,
            object_workspace: RootedVec::with_capacity(64),
            byte_workspace: Vec::with_capacity(512),
        }
    }

    fn signal_error(&self, error_sym: Lisp_Object) -> ! {
        let byte = self.cursor as ptrdiff_t;
        let (pos, line) = match &self.source {
            JsonInputSource::Slice { bytes, multibyte } => {
                let b = (byte as usize).min(bytes.len());
                let pos = if *multibyte {
                    count_utf8_chars(&bytes[..b])
                } else {
                    b as ptrdiff_t
                };
                let line = count_newlines_slice(&bytes[..b]) + 1;
                (pos, line)
            }
            JsonInputSource::CurrentBuffer {
                start_pt,
                start_pt_byte,
                ..
            } => {
                let target_byte = *start_pt_byte + byte;
                let pos = BYTE_TO_CHAR(target_byte) - *start_pt;
                let line = current_buffer_count_newlines(*start_pt_byte, target_byte) + 1;
                (pos, line)
            }
        };
        xsignal3(error_sym, INT_TO_INTEGER(line), Qnil(), INT_TO_INTEGER(pos))
    }

    #[inline(always)]
    fn at_eof(&self) -> bool {
        self.cursor >= self.total_len
    }

    #[inline]
    fn get_if_possible(&mut self) -> c_int {
        if self.cursor >= self.total_len {
            return -1;
        }
        let idx = self.cursor;
        let b = match &mut self.source {
            JsonInputSource::Slice { bytes, .. } => bytes[idx],
            JsonInputSource::CurrentBuffer {
                start_pt_byte,
                end_byte,
                chunk,
                chunk_base,
                chunk_len,
                ..
            } => {
                if !(idx >= *chunk_base && idx < *chunk_base + *chunk_len) {
                    let from_byte = *start_pt_byte + idx as ptrdiff_t;
                    let got = current_buffer_copy_chunk(from_byte, *end_byte, chunk);
                    if got == 0 {
                        return -1;
                    }
                    *chunk_base = idx;
                    *chunk_len = got;
                }
                chunk[idx - *chunk_base]
            }
        };
        self.cursor += 1;
        b as c_int
    }

    #[inline]
    fn get_byte(&mut self) -> u8 {
        let c = self.get_if_possible();
        if c < 0 {
            self.signal_error(Qjson_end_of_file());
        }
        c as u8
    }

    #[inline(always)]
    fn put_back(&mut self) {
        debug_assert!(self.cursor > 0);
        self.cursor -= 1;
    }

    #[inline]
    fn try_copy_four_plain(&mut self) -> bool {
        if self.total_len.saturating_sub(self.cursor) < 4 {
            return false;
        }
        let idx = self.cursor;
        let four = match &self.source {
            JsonInputSource::Slice { bytes, .. } => {
                [bytes[idx], bytes[idx + 1], bytes[idx + 2], bytes[idx + 3]]
            }
            JsonInputSource::CurrentBuffer {
                chunk,
                chunk_base,
                chunk_len,
                ..
            } => {
                if idx >= *chunk_base && idx + 4 <= *chunk_base + *chunk_len {
                    let rel = idx - *chunk_base;
                    [
                        chunk[rel],
                        chunk[rel + 1],
                        chunk[rel + 2],
                        chunk[rel + 3],
                    ]
                } else {
                    return false;
                }
            }
        };
        if JSON_PLAIN_CHAR[four[0] as usize] != 0
            && JSON_PLAIN_CHAR[four[1] as usize] != 0
            && JSON_PLAIN_CHAR[four[2] as usize] != 0
            && JSON_PLAIN_CHAR[four[3] as usize] != 0
        {
            self.byte_workspace.extend_from_slice(&four);
            self.cursor += 4;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn is_whitespace(c: c_int) -> bool {
        c == 0x20 || c == 0x09 || c == 0x0d || c == 0x0a
    }

    fn skip_whitespace(&mut self) -> c_int {
        loop {
            let c = self.get_byte() as c_int;
            if !Self::is_whitespace(c) {
                return c;
            }
        }
    }

    fn skip_whitespace_if_possible(&mut self) -> c_int {
        loop {
            let c = self.get_if_possible();
            if !Self::is_whitespace(c) || c < 0 {
                return c;
            }
        }
    }

    #[inline(always)]
    fn hex_value(c: c_int) -> c_int {
        if (b'0' as c_int..=b'9' as c_int).contains(&c) {
            c - b'0' as c_int
        } else if (b'A' as c_int..=b'F' as c_int).contains(&c) {
            c - b'A' as c_int + 10
        } else if (b'a' as c_int..=b'f' as c_int).contains(&c) {
            c - b'a' as c_int + 10
        } else {
            -1
        }
    }

    fn parse_unicode(&mut self) -> c_int {
        let mut v = [0u8; 4];
        for slot in &mut v {
            let c = Self::hex_value(self.get_byte() as c_int);
            if c < 0 {
                self.signal_error(Qjson_escape_sequence_error());
            }
            *slot = c as u8;
        }
        ((v[0] as c_int) << 12)
            | ((v[1] as c_int) << 8)
            | ((v[2] as c_int) << 4)
            | (v[3] as c_int)
    }

    fn parse_string(&mut self, intern: bool, leading_colon: bool) -> Lisp_Object {
        self.byte_workspace.clear();
        if leading_colon {
            self.byte_workspace.push(b':');
        }
        let mut chars_delta: ptrdiff_t = 0;

        loop {
            if self.try_copy_four_plain() {
                continue;
            }

            let mut c = self.get_byte() as c_int;
            if JSON_PLAIN_CHAR[c as usize] != 0 {
                self.byte_workspace.push(c as u8);
                continue;
            }

            if c == b'"' as c_int {
                let nbytes = self.byte_workspace.len() as ptrdiff_t;
                let nchars = nbytes - chars_delta;
                return if intern {
                    intern_multibyte_bytes(&self.byte_workspace, nchars)
                } else {
                    make_multibyte_string_bytes(&self.byte_workspace, nchars)
                };
            }

            if (c & 0x80) != 0 {
                self.byte_workspace.push(c as u8);
                let c1 = self.get_byte();
                if (c1 as c_int & 0xc0) != 0x80 {
                    self.signal_error(Qjson_utf8_decode_error());
                }
                self.byte_workspace.push(c1);
                if c <= 0xc1 {
                    self.signal_error(Qjson_utf8_decode_error());
                } else if c <= 0xdf {
                    chars_delta += 1;
                } else if c <= 0xef {
                    let c2 = self.get_byte();
                    if (c2 as c_int & 0xc0) != 0x80 {
                        self.signal_error(Qjson_utf8_decode_error());
                    }
                    let v = ((c & 0x0f) << 12) + ((c1 as c_int & 0x3f) << 6) + (c2 as c_int & 0x3f);
                    if v < 0x800 || (0xd800..=0xdfff).contains(&v) {
                        self.signal_error(Qjson_utf8_decode_error());
                    }
                    self.byte_workspace.push(c2);
                    chars_delta += 2;
                } else if c <= 0xf7 {
                    let c2 = self.get_byte();
                    let c3 = self.get_byte();
                    if (c2 as c_int & 0xc0) != 0x80 || (c3 as c_int & 0xc0) != 0x80 {
                        self.signal_error(Qjson_utf8_decode_error());
                    }
                    let v = ((c & 0x07) << 18)
                        + ((c1 as c_int & 0x3f) << 12)
                        + ((c2 as c_int & 0x3f) << 6)
                        + (c3 as c_int & 0x3f);
                    if !(0x10000..=0x10ffff).contains(&v) {
                        self.signal_error(Qjson_utf8_decode_error());
                    }
                    self.byte_workspace.push(c2);
                    self.byte_workspace.push(c3);
                    chars_delta += 3;
                } else {
                    self.signal_error(Qjson_utf8_decode_error());
                }
            } else if c == b'\\' as c_int {
                c = self.get_byte() as c_int;
                match c as u8 {
                    b'"' => self.byte_workspace.push(b'"'),
                    b'\\' => self.byte_workspace.push(b'\\'),
                    b'/' => self.byte_workspace.push(b'/'),
                    b'b' => self.byte_workspace.push(0x08),
                    b'f' => self.byte_workspace.push(0x0c),
                    b'n' => self.byte_workspace.push(0x0a),
                    b'r' => self.byte_workspace.push(0x0d),
                    b't' => self.byte_workspace.push(0x09),
                    b'u' => {
                        let mut num = self.parse_unicode();
                        if (0xd800..0xdc00).contains(&num) {
                            if self.get_byte() != b'\\' {
                                self.signal_error(Qjson_invalid_surrogate_error());
                            }
                            if self.get_byte() != b'u' {
                                self.signal_error(Qjson_invalid_surrogate_error());
                            }
                            let num2 = self.parse_unicode();
                            if !(0xdc00..0xe000).contains(&num2) {
                                self.signal_error(Qjson_invalid_surrogate_error());
                            }
                            num = 0x10000 + (((num - 0xd800) << 10) | (num2 - 0xdc00));
                        } else if (0xdc00..0xe000).contains(&num) {
                            self.signal_error(Qjson_invalid_surrogate_error());
                        }

                        if num < 0x80 {
                            self.byte_workspace.push(num as u8);
                        } else if num < 0x800 {
                            self.byte_workspace.push((0xc0 | (num >> 6)) as u8);
                            self.byte_workspace.push((0x80 | (num & 0x3f)) as u8);
                            chars_delta += 1;
                        } else if num < 0x10000 {
                            self.byte_workspace.push((0xe0 | (num >> 12)) as u8);
                            self.byte_workspace.push((0x80 | ((num >> 6) & 0x3f)) as u8);
                            self.byte_workspace.push((0x80 | (num & 0x3f)) as u8);
                            chars_delta += 2;
                        } else {
                            self.byte_workspace.push((0xf0 | (num >> 18)) as u8);
                            self.byte_workspace
                                .push((0x80 | ((num >> 12) & 0x3f)) as u8);
                            self.byte_workspace.push((0x80 | ((num >> 6) & 0x3f)) as u8);
                            self.byte_workspace.push((0x80 | (num & 0x3f)) as u8);
                            chars_delta += 3;
                        }
                    }
                    _ => self.signal_error(Qjson_escape_sequence_error()),
                }
            } else {
                self.signal_error(Qjson_parse_error());
            }
        }
    }

    fn create_integer(
        &self,
        integer_overflow: bool,
        negative: bool,
        value: EMACS_UINT,
    ) -> Lisp_Object {
        if !integer_overflow {
            if negative {
                let v = value as u64;
                if v <= (INTMAX_MAX as u64) + 1 {
                    return INT_TO_INTEGER((0isize).wrapping_sub(v as isize));
                }
            } else if value <= INTMAX_MAX as EMACS_UINT {
                return INT_TO_INTEGER(value as isize);
            }
        }

        let (result, parsed_len) = string_to_number_bytes(&self.byte_workspace, 10);
        if parsed_len != self.byte_workspace.len() {
            self.signal_error(Qjson_error());
        }
        result
    }

    fn create_float(&self) -> Lisp_Object {
        let (value, parsed_len, out_of_range) = strtod_bytes(&self.byte_workspace);
        if out_of_range {
            self.signal_error(Qjson_number_out_of_range());
        } else if parsed_len != self.byte_workspace.len() {
            self.signal_error(Qjson_error());
        } else {
            make_float(value)
        }
    }

    fn parse_number(&mut self, c_in: c_int) -> Lisp_Object {
        let mut c = c_in;
        self.byte_workspace.clear();
        self.byte_workspace.push(c as u8);

        let mut negative = false;
        if c == b'-' as c_int {
            negative = true;
            c = self.get_byte() as c_int;
            self.byte_workspace.push(c as u8);
        }
        if !(b'0' as c_int..=b'9' as c_int).contains(&c) {
            self.signal_error(Qjson_parse_error());
        }

        let mut integer = (c - b'0' as c_int) as EMACS_UINT;
        let mut integer_overflow = false;

        if integer == 0 {
            if self.at_eof() {
                return INT_TO_INTEGER(0);
            }
            c = self.get_byte() as c_int;
        } else {
            loop {
                if self.at_eof() {
                    return self.create_integer(integer_overflow, negative, integer);
                }
                c = self.get_byte() as c_int;
                if !(b'0' as c_int..=b'9' as c_int).contains(&c) {
                    break;
                }
                self.byte_workspace.push(c as u8);

                let (prod, mul_ovf) = integer.overflowing_mul(10);
                integer = prod;
                integer_overflow |= mul_ovf;
                let (sum, add_ovf) = integer.overflowing_add((c - b'0' as c_int) as EMACS_UINT);
                integer = sum;
                integer_overflow |= add_ovf;
            }
        }

        let mut is_float = false;
        if c == b'.' as c_int {
            self.byte_workspace.push(c as u8);
            is_float = true;
            c = self.get_byte() as c_int;
            self.byte_workspace.push(c as u8);
            if !(b'0' as c_int..=b'9' as c_int).contains(&c) {
                self.signal_error(Qjson_parse_error());
            }
            loop {
                if self.at_eof() {
                    return self.create_float();
                }
                c = self.get_byte() as c_int;
                if !(b'0' as c_int..=b'9' as c_int).contains(&c) {
                    break;
                }
                self.byte_workspace.push(c as u8);
            }
        }
        if c == b'e' as c_int || c == b'E' as c_int {
            self.byte_workspace.push(c as u8);
            is_float = true;
            c = self.get_byte() as c_int;
            self.byte_workspace.push(c as u8);
            if c == b'-' as c_int || c == b'+' as c_int {
                c = self.get_byte() as c_int;
                self.byte_workspace.push(c as u8);
            }
            if !(b'0' as c_int..=b'9' as c_int).contains(&c) {
                self.signal_error(Qjson_parse_error());
            }
            loop {
                if self.at_eof() {
                    return self.create_float();
                }
                c = self.get_byte() as c_int;
                if !(b'0' as c_int..=b'9' as c_int).contains(&c) {
                    break;
                }
                self.byte_workspace.push(c as u8);
            }
        }

        self.put_back();
        if is_float {
            self.create_float()
        } else {
            self.create_integer(integer_overflow, negative, integer)
        }
    }

    fn parse_array(&mut self) -> Lisp_Object {
        let mut c = self.skip_whitespace();
        let first = self.object_workspace.len();
        let mut head = Qnil();
        let mut tail = Qnil();

        if c != b']' as c_int {
            self.available_depth -= 1;
            if self.available_depth < 0 {
                self.signal_error(Qjson_object_too_deep());
            }
            loop {
                let element = self.parse_value(c);
                match self.conf.array_type {
                    JsonArrayType::Array => {
                        self.object_workspace.push(element);
                    }
                    JsonArrayType::List => {
                        let nc = Fcons(element, Qnil());
                        if NILP(head) {
                            head = nc;
                        } else {
                            XSETCDR(tail, nc);
                        }
                        tail = nc;
                    }
                }

                c = self.skip_whitespace();
                if c == b']' as c_int {
                    self.available_depth += 1;
                    break;
                }
                if c != b',' as c_int {
                    self.signal_error(Qjson_parse_error());
                }
                c = self.skip_whitespace();
            }
        }

        match self.conf.array_type {
            JsonArrayType::Array => {
                let count = self.object_workspace.len() - first;
                let vec_obj = make_vector(count as ptrdiff_t, Qnil());
                for i in 0..count {
                    rarely_quit(i as u64);
                    ASET(vec_obj, i as ptrdiff_t, self.object_workspace[first + i]);
                }
                self.object_workspace.truncate(first);
                vec_obj
            }
            JsonArrayType::List => head,
        }
    }

    fn parse_object_member_value(&mut self) -> Lisp_Object {
        let mut c = self.skip_whitespace();
        if c != b':' as c_int {
            self.signal_error(Qjson_parse_error());
        }
        c = self.skip_whitespace();
        self.parse_value(c)
    }

    fn parse_object(&mut self) -> Lisp_Object {
        let mut c = self.skip_whitespace();
        let first = self.object_workspace.len();
        let mut head = Qnil();
        let mut tail = Qnil();

        if c != b'}' as c_int {
            self.available_depth -= 1;
            if self.available_depth < 0 {
                self.signal_error(Qjson_object_too_deep());
            }

            loop {
                if c != b'"' as c_int {
                    self.signal_error(Qjson_parse_error());
                }

                match self.conf.object_type {
                    JsonObjectType::Hashtable => {
                        let key = self.parse_string(false, false);
                        self.object_workspace.push(key);
                        let value = self.parse_object_member_value();
                        self.object_workspace.push(value);
                    }
                    JsonObjectType::Alist => {
                        let key = self.parse_string(true, false);
                        let value = self.parse_object_member_value();
                        let nc = Fcons(Fcons(key, value), Qnil());
                        if NILP(head) {
                            head = nc;
                        } else {
                            XSETCDR(tail, nc);
                        }
                        tail = nc;
                    }
                    JsonObjectType::Plist => {
                        let key = self.parse_string(true, true);
                        let value = self.parse_object_member_value();
                        let kcell = Fcons(key, Qnil());
                        let vcell = Fcons(value, Qnil());
                        XSETCDR(kcell, vcell);
                        if NILP(head) {
                            head = kcell;
                        } else {
                            XSETCDR(tail, kcell);
                        }
                        tail = vcell;
                    }
                }

                c = self.skip_whitespace();
                if c == b'}' as c_int {
                    self.available_depth += 1;
                    break;
                }
                if c != b',' as c_int {
                    self.signal_error(Qjson_parse_error());
                }
                c = self.skip_whitespace();
            }
        }

        match self.conf.object_type {
            JsonObjectType::Hashtable => {
                let npairs = ((self.object_workspace.len() - first) >> 1) as EMACS_INT;
                let tbl = make_equal_hash_table(npairs, Weak_None);
                let mut i = first;
                while i < self.object_workspace.len() {
                    let k = self.object_workspace[i];
                    let v = self.object_workspace[i + 1];
                    Fputhash(k, v, tbl);
                    i += 2;
                }
                self.object_workspace.truncate(first);
                tbl
            }
            JsonObjectType::Alist | JsonObjectType::Plist => head,
        }
    }

    #[inline(always)]
    fn is_token_char(c: c_int) -> bool {
        (b'A' as c_int..=b'Z' as c_int).contains(&c)
            || (b'a' as c_int..=b'z' as c_int).contains(&c)
            || (b'0' as c_int..=b'9' as c_int).contains(&c)
            || c == b'-' as c_int
    }

    fn parse_value(&mut self, c: c_int) -> Lisp_Object {
        match c as u8 {
            b'{' => return self.parse_object(),
            b'[' => return self.parse_array(),
            b'"' => return self.parse_string(false, false),
            b'0'..=b'9' | b'-' => return self.parse_number(c),
            b't' => {
                if self.get_if_possible() == b'r' as c_int
                    && self.get_if_possible() == b'u' as c_int
                    && self.get_if_possible() == b'e' as c_int
                {
                    let c2 = self.get_if_possible();
                    if !Self::is_token_char(c2) {
                        if c2 >= 0 {
                            self.put_back();
                        }
                        return Qt();
                    }
                }
            }
            b'f' => {
                if self.get_if_possible() == b'a' as c_int
                    && self.get_if_possible() == b'l' as c_int
                    && self.get_if_possible() == b's' as c_int
                    && self.get_if_possible() == b'e' as c_int
                {
                    let c2 = self.get_if_possible();
                    if !Self::is_token_char(c2) {
                        if c2 >= 0 {
                            self.put_back();
                        }
                        return self.conf.false_object;
                    }
                }
            }
            b'n' => {
                if self.get_if_possible() == b'u' as c_int
                    && self.get_if_possible() == b'l' as c_int
                    && self.get_if_possible() == b'l' as c_int
                {
                    let c2 = self.get_if_possible();
                    if !Self::is_token_char(c2) {
                        if c2 >= 0 {
                            self.put_back();
                        }
                        return self.conf.null_object;
                    }
                }
            }
            _ => {}
        }
        self.signal_error(Qjson_parse_error());
    }

    fn parse(&mut self) -> Lisp_Object {
        let first = self.skip_whitespace();
        self.parse_value(first)
    }
}

pub fn string_byte_to_pos_impl(obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t {
    eassert!(STRINGP(obj));
    eassert!(byte <= SBYTES(obj));
    if STRING_MULTIBYTE(obj) {
        let bytes = lisp_string_bytes_vec(obj);
        count_utf8_chars(&bytes[..(byte.max(0) as usize)])
    } else {
        byte
    }
}

pub fn string_byte_to_line_impl(obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t {
    eassert!(STRINGP(obj));
    eassert!(byte <= SBYTES(obj));
    let bytes = lisp_string_bytes_vec(obj);
    count_newlines_slice(&bytes[..(byte.max(0) as usize)])
}

pub fn buffer_byte_to_pos_impl(_obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t {
    BYTE_TO_CHAR(current_buffer_pt_byte() + byte) - current_buffer_pt()
}

pub fn buffer_byte_to_line_impl(_obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t {
    let pt_byte = current_buffer_pt_byte();
    current_buffer_count_newlines(pt_byte, pt_byte + byte)
}

pub fn json_parse_string_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let string = args[0];
    CHECK_STRING(string);
    let mut conf = JsonConfiguration::default();
    json_parse_args(&args[1..], &mut conf, true);

    let mut parser = SafeJsonParser::from_lisp_string(string, conf);
    let result = parser.parse();
    if parser.skip_whitespace_if_possible() >= 0 {
        parser.signal_error(Qjson_trailing_content());
    }
    result
}

pub fn json_parse_buffer_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let mut conf = JsonConfiguration::default();
    json_parse_args(args, &mut conf, true);

    let mut parser = SafeJsonParser::from_current_buffer(conf);
    let result = parser.parse();

    let byte = current_buffer_pt_byte() + parser.cursor as ptrdiff_t;
    let position = if !current_buffer_multibyte_p() {
        byte
    } else {
        BYTE_TO_CHAR(byte)
    };
    SET_PT_BOTH(position, byte);
    result
}
