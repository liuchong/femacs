//! `#![forbid(unsafe_code)]` implementation of high-level character, string, Unicode-category,
//! and char-table Lisp primitives (`src/character.rs`, `src/chartab.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;
use crate::common::unicode_category_t::*;

#[inline(always)]
fn check_char_table(x: Lisp_Object) {
    CHECK_TYPE(CHAR_TABLE_P(x), Qchar_table_p(), x);
}

#[inline(always)]
pub fn uniprop_table_p(table: Lisp_Object) -> bool {
    CHAR_TABLE_P(table)
        && EQ(char_table_purpose_ref(table), Qchar_code_property_table())
        && char_table_extra_slots_count(table) == 5
}

pub fn char_resolve_modifier_mask_impl(mut c: EMACS_INT) -> EMACS_INT {
    if !ASCII_CHAR_P((c & !(CHAR_MODIFIER_MASK as EMACS_INT)) as isize) {
        return c;
    }

    if (c & CHAR_SHIFT as EMACS_INT) != 0 {
        if (c & 0o377) >= b'A' as EMACS_INT && (c & 0o377) <= b'Z' as EMACS_INT {
            c &= !(CHAR_SHIFT as EMACS_INT);
        } else if (c & 0o377) >= b'a' as EMACS_INT && (c & 0o377) <= b'z' as EMACS_INT {
            c = (c & !(CHAR_SHIFT as EMACS_INT)) - (b'a' as EMACS_INT - b'A' as EMACS_INT);
        } else if (c & !(CHAR_MODIFIER_MASK as EMACS_INT)) <= 0x20 {
            c &= !(CHAR_SHIFT as EMACS_INT);
        }
    }
    if (c & CHAR_CTL as EMACS_INT) != 0 {
        if (c & 0o377) == b' ' as EMACS_INT {
            c &= !0o177 & !(CHAR_CTL as EMACS_INT);
        } else if (c & 0o377) == b'?' as EMACS_INT {
            c = 0o177 | (c & !0o177 & !(CHAR_CTL as EMACS_INT));
        } else if (c & 0o137) >= 0o101 && (c & 0o137) <= 0o132 {
            c &= 0o37 | (!0o177 & !(CHAR_CTL as EMACS_INT));
        } else if (c & 0o177) >= 0o100 && (c & 0o177) <= 0o137 {
            c &= 0o37 | (!0o177 & !(CHAR_CTL as EMACS_INT));
        }
    }
    c
}

pub fn translate_char_impl(mut table: Lisp_Object, mut c: c_int) -> c_int {
    if CHAR_TABLE_P(table) {
        let ch = CHAR_TABLE_REF(table, c as isize);
        if CHARACTERP(ch) {
            c = XFIXNUM(ch) as c_int;
        }
    } else {
        while CONSP(table) {
            c = translate_char_impl(XCAR(table), c);
            table = XCDR(table);
        }
    }
    c
}

pub fn characterp(object: Lisp_Object, _ignore: Lisp_Object) -> Lisp_Object {
    if CHARACTERP(object) { Qt() } else { Qnil() }
}

pub fn max_char(unicode: Lisp_Object) -> Lisp_Object {
    if !NILP(unicode) {
        make_fixnum(MAX_UNICODE_CHAR as EMACS_INT)
    } else {
        make_fixnum(MAX_CHAR as EMACS_INT)
    }
}

pub fn unibyte_char_to_multibyte(ch: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(ch);
    let c = XFIXNAT(ch) as c_int;
    if c >= 0x100 {
        error_not_unibyte_char(c);
    }
    make_fixnum(make_char_multibyte(c) as EMACS_INT)
}

pub fn multibyte_char_to_unibyte(ch: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(ch);
    let cm = XFIXNAT(ch) as c_int;
    if cm < 256 {
        ch
    } else {
        let cu = CHAR_TO_BYTE_SAFE(cm);
        make_fixnum(cu as EMACS_INT)
    }
}

pub fn char_width_for_disp_vector(c: c_int, disp_opt: Option<Lisp_Object>) -> ptrdiff_t {
    let mut width = CHARACTER_WIDTH(c) as ptrdiff_t;
    if let Some(disp) = disp_opt {
        if VECTORP(disp) {
            width = 0;
            let len = ASIZE(disp);
            for i in 0..len {
                let ch = AREF(disp, i);
                let mut gc: c_int = -1;
                if GLYPH_CODE_P(ch) {
                    gc = GLYPH_CODE_CHAR(ch);
                } else if CHARACTERP(ch) {
                    gc = XFIXNUM(ch) as c_int;
                }
                if gc >= 0 {
                    let w = CHARACTER_WIDTH(gc);
                    let (sum, over) = width.overflowing_add(w as ptrdiff_t);
                    width = sum;
                    if over {
                        string_overflow();
                    }
                }
            }
        }
    }
    width
}

pub fn char_width_subr(ch: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(ch);
    let c = XFIXNUM(ch) as c_int;
    let width = char_width_for_disp_vector(c, buffer_display_table_char_vector(c));
    make_fixnum(width as EMACS_INT)
}

pub fn string_width_subr(str_obj: Lisp_Object, from: Lisp_Object, to: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(str_obj);
    let (ifrom, ito) = validate_subarray_safe(str_obj, from, to, SCHARS(str_obj));
    let w = crate::character::lisp_string_width_subr_safe(str_obj, ifrom, ito);
    make_fixnum(w as EMACS_INT)
}

pub fn unibyte_string_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let str_obj = make_uninit_string(args.len() as ptrdiff_t);
    for (idx, &arg) in args.iter().enumerate() {
        let b = check_integer_range(arg, 0, 255) as u8;
        SSET(str_obj, idx, b);
    }
    str_obj
}

pub fn string_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let nargs = args.len() as ptrdiff_t;
    let mut nbytes: ptrdiff_t = 0;
    for &arg in args {
        CHECK_CHARACTER(arg);
        nbytes += CHAR_BYTES(XFIXNUM(arg) as c_int) as ptrdiff_t;
    }
    if nbytes == nargs {
        return unibyte_string_slice(args);
    }
    let mut bytes = Vec::with_capacity(nbytes as usize);
    for &arg in args {
        let c = XFIXNUM(arg) as c_int;
        let (tmp, len) = encode_char_multibyte_buf(c);
        bytes.extend_from_slice(&tmp[..len]);
    }
    make_specified_string_bytes(&bytes, nargs, true)
}

pub fn char_resolve_modifiers(character: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(character);
    make_fixnum(char_resolve_modifier_mask_impl(XFIXNUM(character)))
}

pub fn get_byte(position: Lisp_Object, string: Lisp_Object) -> Lisp_Object {
    let c: c_int = if NILP(string) {
        let bytepos = if NILP(position) {
            current_buffer_pt_byte()
        } else {
            let fixed_pos = fix_position_safe(position);
            let begv = current_buffer_begv();
            let zv = current_buffer_zv();
            if !(begv <= fixed_pos && fixed_pos < zv) {
                args_out_of_range_3(position, make_fixnum(begv), make_fixnum(zv));
            }
            current_buffer_char_to_byte(fixed_pos)
        };
        if !current_buffer_multibyte_p() {
            return make_fixnum(current_buffer_fetch_byte(bytepos) as EMACS_INT);
        }
        if bytepos >= current_buffer_z_byte() {
            0
        } else {
            current_buffer_fetch_char(bytepos)
        }
    } else {
        CHECK_STRING(string);
        let bytepos = if NILP(position) {
            0
        } else {
            CHECK_FIXNAT(position);
            if XFIXNUM(position) >= SCHARS(string) {
                args_out_of_range(string, position);
            }
            string_char_to_byte(string, XFIXNAT(position))
        };
        if !STRING_MULTIBYTE(string) {
            return make_fixnum(lisp_string_first_or_byte_at(string, bytepos) as EMACS_INT);
        }
        if bytepos == 0 && SBYTES(string) == 0 {
            0
        } else {
            let (ch, _) = fetch_string_char_at(string, 0, bytepos);
            ch
        }
    };

    let out = if CHAR_BYTE8_P(c) {
        CHAR_TO_BYTE8(c)
    } else if !ASCII_CHAR_P(c as isize) {
        error_not_ascii_or_8bit_char(c);
    } else {
        c
    };
    make_fixnum(out as EMACS_INT)
}

pub fn alphabeticp_impl(c: c_int) -> bool {
    let category = CHAR_TABLE_REF(Vunicode_category_table.get(), c as isize);
    if !FIXNUMP(category) {
        return false;
    }
    let gen_cat = XFIXNUM(category);
    gen_cat == UNICODE_CATEGORY_Lu as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Ll as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Lt as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Lm as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Lo as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Mn as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Mc as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Me as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Nl as EMACS_INT
}

pub fn alphanumericp_impl(c: c_int) -> bool {
    let category = CHAR_TABLE_REF(Vunicode_category_table.get(), c as isize);
    if !FIXNUMP(category) {
        return false;
    }
    let gen_cat = XFIXNUM(category);
    gen_cat == UNICODE_CATEGORY_Lu as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Ll as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Lt as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Lm as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Lo as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Mn as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Mc as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Me as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Nl as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Nd as EMACS_INT
}

pub fn graphicp_impl(c: c_int) -> bool {
    let category = CHAR_TABLE_REF(Vunicode_category_table.get(), c as isize);
    if !FIXNUMP(category) {
        return false;
    }
    let gen_cat = XFIXNUM(category);
    !(gen_cat == UNICODE_CATEGORY_Zs as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Zl as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Zp as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cc as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cs as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cn as EMACS_INT)
}

pub fn printablep_impl(c: c_int) -> bool {
    let category = CHAR_TABLE_REF(Vunicode_category_table.get(), c as isize);
    if !FIXNUMP(category) {
        return false;
    }
    let gen_cat = XFIXNUM(category);
    !(gen_cat == UNICODE_CATEGORY_Cc as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cs as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cn as EMACS_INT)
}

pub fn graphic_base_p_impl(c: c_int) -> bool {
    let category = CHAR_TABLE_REF(Vunicode_category_table.get(), c as isize);
    if !FIXNUMP(category) {
        return false;
    }
    let gen_cat = XFIXNUM(category);
    !(gen_cat == UNICODE_CATEGORY_Mn as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Mc as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Me as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Zs as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Zl as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Zp as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cc as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cs as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cf as EMACS_INT
        || gen_cat == UNICODE_CATEGORY_Cn as EMACS_INT)
}

pub fn blankp_impl(c: c_int) -> bool {
    let category = CHAR_TABLE_REF(Vunicode_category_table.get(), c as isize);
    FIXNUMP(category) && XFIXNUM(category) == UNICODE_CATEGORY_Zs as EMACS_INT
}

// --- Char-table Lisp primitives (`src/chartab.rs`) ---

pub fn make_char_table(purpose: Lisp_Object, init: Lisp_Object) -> Lisp_Object {
    CHECK_SYMBOL(purpose);
    let n = Fget(purpose, Qchar_table_extra_slots());
    let n_extras = if NILP(n) {
        0
    } else {
        CHECK_FIXNAT(n);
        XFIXNUM(n) as c_int
    };
    let size = CHAR_TABLE_STANDARD_SLOTS as c_int + n_extras;
    let vector = make_vector(size as isize, init);
    set_vector_to_char_table(vector, purpose)
}

pub fn char_table_subtype(char_table: Lisp_Object) -> Lisp_Object {
    check_char_table(char_table);
    char_table_purpose_ref(char_table)
}

pub fn char_table_parent(char_table: Lisp_Object) -> Lisp_Object {
    check_char_table(char_table);
    char_table_parent_ref(char_table)
}

pub fn set_char_table_parent_subr(char_table: Lisp_Object, parent: Lisp_Object) -> Lisp_Object {
    check_char_table(char_table);
    if !NILP(parent) {
        check_char_table(parent);
        let mut temp = parent;
        while !NILP(temp) {
            if EQ(temp, char_table) {
                error_chartab_self_parent();
            }
            temp = char_table_parent_ref(temp);
        }
    }
    set_char_table_parent_slot(char_table, parent);
    parent
}

pub fn char_table_extra_slot(char_table: Lisp_Object, n: Lisp_Object) -> Lisp_Object {
    check_char_table(char_table);
    CHECK_FIXNUM(n);
    let idx = XFIXNUM(n);
    if idx < 0 || idx >= char_table_extra_slots_count(char_table) {
        args_out_of_range(char_table, n);
    }
    char_table_extra_slot_ref(char_table, idx)
}

pub fn set_char_table_extra_slot(
    char_table: Lisp_Object,
    n: Lisp_Object,
    value: Lisp_Object,
) -> Lisp_Object {
    check_char_table(char_table);
    CHECK_FIXNUM(n);
    let idx = XFIXNUM(n);
    if idx < 0 || idx >= char_table_extra_slots_count(char_table) {
        args_out_of_range(char_table, n);
    }
    set_char_table_extras(char_table, idx, value);
    value
}

pub fn map_char_table_subr(function: Lisp_Object, char_table: Lisp_Object) -> Lisp_Object {
    check_char_table(char_table);
    crate::chartab::map_char_table_lisp_fn(function, char_table);
    Qnil()
}

pub fn unicode_property_table_internal(prop: Lisp_Object) -> Lisp_Object {
    let table = crate::chartab::uniprop_table_safe(prop);
    if CHAR_TABLE_P(table) {
        return table;
    }
    Fcdr(Fassq(prop, Vchar_code_property_alist.get()))
}

pub fn get_unicode_property_internal(char_table: Lisp_Object, ch: Lisp_Object) -> Lisp_Object {
    check_char_table(char_table);
    CHECK_CHARACTER(ch);
    if !uniprop_table_p(char_table) {
        error_invalid_uniprop_table();
    }
    crate::chartab::get_unicode_property_safe(char_table, XFIXNUM(ch) as c_int)
}
