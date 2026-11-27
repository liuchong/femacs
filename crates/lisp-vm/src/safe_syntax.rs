//! `#![forbid(unsafe_code)]` implementation of syntax-table inspection, entry parsing,
//! word scanning, and list/sexp Lisp entrypoints (`src/syntax.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

pub const SYNTAX_CODE_SPEC: [c_char; 16] = [
    b' ' as c_char,
    b'.' as c_char,
    b'w' as c_char,
    b'_' as c_char,
    b'(' as c_char,
    b')' as c_char,
    b'\'' as c_char,
    b'"' as c_char,
    b'$' as c_char,
    b'\\' as c_char,
    b'/' as c_char,
    b'<' as c_char,
    b'>' as c_char,
    b'@' as c_char,
    b'!' as c_char,
    b'|' as c_char,
];

#[inline(always)]
pub fn check_syntax_table_safe(obj: Lisp_Object) {
    CHECK_TYPE(
        CHAR_TABLE_P(obj) && EQ(char_table_purpose_ref(obj), Qsyntax_table()),
        Qsyntax_table_p(),
        obj,
    );
}

#[inline(always)]
fn syntax_via_gl(c: c_int) -> syntaxcode {
    syntax_property(c, true)
}

#[inline(always)]
fn syntax_match_via_gl(c: c_int) -> Lisp_Object {
    let ent = syntax_property_entry(c, true);
    if CONSP(ent) { XCDR(ent) } else { Qnil() }
}

pub fn syntax_table_p(object: Lisp_Object) -> Lisp_Object {
    if CHAR_TABLE_P(object) && EQ(char_table_purpose_ref(object), Qsyntax_table()) {
        Qt()
    } else {
        Qnil()
    }
}

pub fn syntax_table() -> Lisp_Object {
    current_buffer_syntax_table()
}

pub fn standard_syntax_table_subr() -> Lisp_Object {
    standard_syntax_table()
}

pub fn copy_syntax_table(mut table: Lisp_Object) -> Lisp_Object {
    if !NILP(table) {
        check_syntax_table_safe(table);
    } else {
        table = standard_syntax_table();
    }

    let copy = Fcopy_sequence(table);
    set_char_table_defalt(copy, Qnil());
    if NILP(char_table_parent_ref(copy)) {
        Fset_char_table_parent(copy, standard_syntax_table());
    }
    copy
}

pub fn set_syntax_table(table: Lisp_Object) -> Lisp_Object {
    check_syntax_table_safe(table);
    set_current_buffer_syntax_table_local(table);
    table
}

pub fn char_syntax(character: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(character);
    let mut char_int = XFIXNAT(character) as c_int;
    SETUP_BUFFER_SYNTAX_TABLE();
    if !current_buffer_multibyte_p() && SINGLE_BYTE_CHAR_P(char_int as intmax_t) {
        char_int = make_char_multibyte(char_int);
    }
    make_fixnum(SYNTAX_CODE_SPEC[syntax_via_gl(char_int) as usize] as EMACS_INT)
}

pub fn syntax_class_to_char(syntax: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(syntax);
    let syn = XFIXNUM(syntax) as c_int;
    if syn < 0 || syn >= SYNTAX_CODE_SPEC.len() as c_int {
        args_out_of_range(make_fixnum(SYNTAX_CODE_SPEC.len() as EMACS_INT - 1), syntax);
    }
    make_fixnum(SYNTAX_CODE_SPEC[syn as usize] as EMACS_INT)
}

pub fn matching_paren(character: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(character);
    let char_int = XFIXNUM(character) as c_int;
    SETUP_BUFFER_SYNTAX_TABLE();
    let code = syntax_via_gl(char_int);
    if code == Sopen || code == Sclose {
        syntax_match_via_gl(char_int)
    } else {
        Qnil()
    }
}

pub fn string_to_syntax(string: Lisp_Object, syntax_code_object: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(string);
    let nbytes = SBYTES(string);
    let first_byte = lisp_string_first_or_byte_at(string, 0);
    let mut val = syntax_spec_code[first_byte as usize] as c_int;
    if val == 0o377 {
        error_invalid_syntax_letter(first_byte as c_int);
    }
    if val == Sinherit {
        return Qnil();
    }

    let mut byte_idx: ptrdiff_t = 1;
    let mut match_lisp = if byte_idx < nbytes && SREF(string, byte_idx as usize) != 0 {
        let (character, clen) = lisp_string_char_and_length_at(string, byte_idx);
        byte_idx += clen;
        let m = make_fixnum(character as EMACS_INT);
        if XFIXNAT(m) == b' ' as EMACS_INT {
            Qnil()
        } else {
            m
        }
    } else {
        Qnil()
    };
    let _ = &mut match_lisp;

    while byte_idx < nbytes {
        let ch = SREF(string, byte_idx as usize);
        if ch == 0 {
            break;
        }
        byte_idx += 1;
        match ch {
            b'1' => val |= 1 << 16,
            b'2' => val |= 1 << 17,
            b'3' => val |= 1 << 18,
            b'4' => val |= 1 << 19,
            b'p' => val |= 1 << 20,
            b'b' => val |= 1 << 21,
            b'n' => val |= 1 << 22,
            b'c' => val |= 1 << 23,
            _ => {}
        }
    }

    if (val as isize) < ASIZE(syntax_code_object) && NILP(match_lisp) {
        AREF(syntax_code_object, val as isize)
    } else {
        Fcons(make_fixnum(val as EMACS_INT), match_lisp)
    }
}

pub fn modify_syntax_entry(
    c: Lisp_Object,
    newentry: Lisp_Object,
    mut syntax_table: Lisp_Object,
) -> Lisp_Object {
    if CONSP(c) {
        CHECK_CHARACTER_CAR(c);
        CHECK_CHARACTER_CDR(c);
    } else {
        CHECK_CHARACTER(c);
    }

    if NILP(syntax_table) {
        syntax_table = current_buffer_syntax_table();
    } else {
        check_syntax_table_safe(syntax_table);
    }

    let entry = Fstring_to_syntax(newentry);
    if CONSP(c) {
        SET_RAW_SYNTAX_ENTRY_RANGE(syntax_table, c, entry);
    } else {
        SET_RAW_SYNTAX_ENTRY(syntax_table, XFIXNUM(c) as c_int, entry);
    }

    clear_regexp_cache();
    Qnil()
}

pub fn scan_words_impl(from: ptrdiff_t, mut count: EMACS_INT) -> ptrdiff_t {
    let beg = current_buffer_begv();
    let end = current_buffer_zv();
    let mut from = from;
    let mut from_byte = current_buffer_char_to_byte(from);
    let mut ch0: c_int;
    let mut ch1: c_int;

    crate::syntax::setup_syntax_table_safe(from, clip_to_bounds(PTRDIFF_MIN, count, PTRDIFF_MAX));

    while count > 0 {
        loop {
            if from == end {
                return 0;
            }
            UPDATE_SYNTAX_TABLE_FORWARD(from);
            ch0 = FETCH_CHAR_AS_MULTIBYTE(from_byte);
            let code = syntax_via_gl(ch0);
            let (_, step, _) = current_buffer_char_len_and_next_at(from_byte, false);
            from += 1;
            from_byte += if current_buffer_multibyte_p() {
                step as ptrdiff_t
            } else {
                1
            };
            if words_include_escapes.get() && (code == Sescape || code == Scharquote) {
                break;
            }
            if code == Sword {
                break;
            }
            rarely_quit(from as u64);
        }
        let func = CHAR_TABLE_REF(Vfind_word_boundary_function_table.get(), ch0 as isize);
        if !NILP(Ffboundp(func)) {
            let pos = calln!(func, make_fixnum(from - 1), make_fixnum(end));
            if FIXNUMP(pos) && from < XFIXNUM(pos) && XFIXNUM(pos) <= current_buffer_zv() {
                from = XFIXNUM(pos);
                from_byte = current_buffer_char_to_byte(from);
            }
        } else {
            loop {
                if from == end {
                    break;
                }
                UPDATE_SYNTAX_TABLE_FORWARD(from);
                ch1 = FETCH_CHAR_AS_MULTIBYTE(from_byte);
                let code = syntax_via_gl(ch1);
                if (code != Sword
                    && (!words_include_escapes.get() || (code != Sescape && code != Scharquote)))
                    || word_boundary_p(ch0, ch1)
                {
                    break;
                }
                let (_, step, _) = current_buffer_char_len_and_next_at(from_byte, false);
                from += 1;
                from_byte += if current_buffer_multibyte_p() {
                    step as ptrdiff_t
                } else {
                    1
                };
                ch0 = ch1;
                rarely_quit(from as u64);
            }
        }
        count -= 1;
    }

    while count < 0 {
        loop {
            if from == beg {
                return 0;
            }
            from -= 1;
            from_byte -= if current_buffer_multibyte_p() {
                current_buffer_prev_char_len(from_byte) as ptrdiff_t
            } else {
                1
            };
            UPDATE_SYNTAX_TABLE_BACKWARD(from);
            ch1 = FETCH_CHAR_AS_MULTIBYTE(from_byte);
            let code = syntax_via_gl(ch1);
            if words_include_escapes.get() && (code == Sescape || code == Scharquote) {
                break;
            }
            if code == Sword {
                break;
            }
            rarely_quit(from as u64);
        }
        let func = CHAR_TABLE_REF(Vfind_word_boundary_function_table.get(), ch1 as isize);
        if !NILP(Ffboundp(func)) {
            let pos = calln!(func, make_fixnum(from), make_fixnum(beg));
            if FIXNUMP(pos) && current_buffer_begv() <= XFIXNUM(pos) && XFIXNUM(pos) < from {
                from = XFIXNUM(pos);
                from_byte = current_buffer_char_to_byte(from);
            }
        } else {
            loop {
                if from == beg {
                    break;
                }
                from -= 1;
                from_byte -= if current_buffer_multibyte_p() {
                    current_buffer_prev_char_len(from_byte) as ptrdiff_t
                } else {
                    1
                };
                UPDATE_SYNTAX_TABLE_BACKWARD(from);
                ch0 = FETCH_CHAR_AS_MULTIBYTE(from_byte);
                let code = syntax_via_gl(ch0);
                if (code != Sword
                    && (!words_include_escapes.get() || (code != Sescape && code != Scharquote)))
                    || word_boundary_p(ch0, ch1)
                {
                    let (_, step, _) = current_buffer_char_len_and_next_at(from_byte, false);
                    from += 1;
                    from_byte += if current_buffer_multibyte_p() {
                        step as ptrdiff_t
                    } else {
                        1
                    };
                    break;
                }
                ch1 = ch0;
                rarely_quit(from as u64);
            }
        }
        count += 1;
    }

    from
}

pub fn forward_word(arg: Lisp_Object) -> Lisp_Object {
    let cnt = if NILP(arg) {
        1
    } else {
        CHECK_FIXNUM(arg);
        XFIXNUM(arg)
    };

    let mut val = scan_words(current_buffer_pt(), cnt);
    let orig_val = val;
    if orig_val == 0 {
        val = if cnt > 0 {
            current_buffer_zv()
        } else {
            current_buffer_begv()
        };
    }

    let tmp = Fconstrain_to_field(
        make_fixnum(val),
        make_fixnum(current_buffer_pt()),
        Qnil(),
        Qnil(),
        Qnil(),
    );
    val = XFIXNAT(tmp);
    SET_PT(val);
    if val == orig_val { Qt() } else { Qnil() }
}

pub fn internal_describe_syntax_value(syntax: Lisp_Object) -> Lisp_Object {
    let value = syntax;
    if NILP(value) {
        insert_ascii_bytes_safe(b"default");
        return syntax;
    }
    if CHAR_TABLE_P(value) {
        insert_ascii_bytes_safe(b"deeper char-table ...");
        return syntax;
    }
    if !CONSP(value) {
        insert_ascii_bytes_safe(b"invalid");
        return syntax;
    }

    let first = XCAR(value);
    let match_lisp = XCDR(value);
    if !FIXNUMP(first) || !(NILP(match_lisp) || CHARACTERP(match_lisp)) {
        insert_ascii_bytes_safe(b"invalid");
        return syntax;
    }

    let syntax_code = XFIXNUM(first) as c_int & INT_MAX;
    let code = syntax_code & 0o377;
    let start1 = SYNTAX_FLAGS_COMSTART_FIRST(syntax_code);
    let start2 = SYNTAX_FLAGS_COMSTART_SECOND(syntax_code);
    let end1 = SYNTAX_FLAGS_COMEND_FIRST(syntax_code);
    let end2 = SYNTAX_FLAGS_COMEND_SECOND(syntax_code);
    let prefix = SYNTAX_FLAGS_PREFIX(syntax_code);
    let comstyleb = SYNTAX_FLAGS_COMMENT_STYLEB(syntax_code);
    let comstylec = SYNTAX_FLAGS_COMMENT_STYLEC(syntax_code);
    let comnested = SYNTAX_FLAGS_COMMENT_NESTED(syntax_code);

    if Smax <= code {
        insert_ascii_bytes_safe(b"invalid");
        return syntax;
    }

    let spec_byte = [SYNTAX_CODE_SPEC[code as usize] as u8];
    insert_ascii_bytes_safe(&spec_byte);

    if NILP(match_lisp) {
        insert_ascii_bytes_safe(b" ");
    } else {
        insert_char_safe(XFIXNUM(match_lisp) as c_int);
    }

    if start1 {
        insert_ascii_bytes_safe(b"1");
    }
    if start2 {
        insert_ascii_bytes_safe(b"2");
    }
    if end1 {
        insert_ascii_bytes_safe(b"3");
    }
    if end2 {
        insert_ascii_bytes_safe(b"4");
    }
    if prefix {
        insert_ascii_bytes_safe(b"p");
    }
    if comstyleb {
        insert_ascii_bytes_safe(b"b");
    }
    if comstylec {
        insert_ascii_bytes_safe(b"c");
    }
    if comnested {
        insert_ascii_bytes_safe(b"n");
    }

    insert_ascii_bytes_safe(b"\twhich means: ");
    match code {
        Swhitespace => insert_ascii_bytes_safe(b"whitespace"),
        Spunct => insert_ascii_bytes_safe(b"punctuation"),
        Sword => insert_ascii_bytes_safe(b"word"),
        Ssymbol => insert_ascii_bytes_safe(b"symbol"),
        Sopen => insert_ascii_bytes_safe(b"open"),
        Sclose => insert_ascii_bytes_safe(b"close"),
        Squote => insert_ascii_bytes_safe(b"prefix"),
        Sstring => insert_ascii_bytes_safe(b"string"),
        Smath => insert_ascii_bytes_safe(b"math"),
        Sescape => insert_ascii_bytes_safe(b"escape"),
        Scharquote => insert_ascii_bytes_safe(b"charquote"),
        Scomment => insert_ascii_bytes_safe(b"comment"),
        Sendcomment => insert_ascii_bytes_safe(b"endcomment"),
        Sinherit => insert_ascii_bytes_safe(b"inherit"),
        Scomment_fence => insert_ascii_bytes_safe(b"comment fence"),
        Sstring_fence => insert_ascii_bytes_safe(b"string fence"),
        _ => {
            insert_ascii_bytes_safe(b"invalid");
            return syntax;
        }
    }

    if !NILP(match_lisp) {
        insert_ascii_bytes_safe(b", matches ");
        insert_char_safe(XFIXNUM(match_lisp) as c_int);
    }

    if start1 {
        insert_ascii_bytes_safe(b",\n\t  is the first character of a comment-start sequence");
    }
    if start2 {
        insert_ascii_bytes_safe(b",\n\t  is the second character of a comment-start sequence");
    }
    if end1 {
        insert_ascii_bytes_safe(b",\n\t  is the first character of a comment-end sequence");
    }
    if end2 {
        insert_ascii_bytes_safe(b",\n\t  is the second character of a comment-end sequence");
    }
    if comstyleb {
        insert_ascii_bytes_safe(b" (comment style b)");
    }
    if comstylec {
        insert_ascii_bytes_safe(b" (comment style c)");
    }
    if comnested {
        insert_ascii_bytes_safe(b" (nestable)");
    }
    if prefix {
        let prefixdoc = build_cstr(c",\n\t  is a prefix character for `backward-prefix-chars'");
        insert1_safe(calln!(Qsubstitute_command_keys(), prefixdoc));
    }

    syntax
}

pub fn scan_lists_subr(from: Lisp_Object, count: Lisp_Object, depth: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(from);
    CHECK_FIXNUM(count);
    CHECK_FIXNUM(depth);
    crate::syntax::scan_lists_safe(XFIXNUM(from), XFIXNUM(count), XFIXNUM(depth), false)
}

pub fn scan_sexps_subr(from: Lisp_Object, count: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(from);
    CHECK_FIXNUM(count);
    crate::syntax::scan_lists_safe(XFIXNUM(from), XFIXNUM(count), 0, true)
}
