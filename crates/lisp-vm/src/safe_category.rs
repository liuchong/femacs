//! `#![forbid(unsafe_code)]` implementation of character categories, category sets,
//! and category tables (`src/category.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

#[inline(always)]
pub fn set_category_set(category_set: Lisp_Object, category: EMACS_INT, val: bool) {
    bool_vector_set(category_set, category, val);
}

pub fn make_category_set(categories: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(categories);
    let val = MAKE_CATEGORY_SET();

    if STRING_MULTIBYTE(categories) {
        error_cstr(c"Multibyte string in `make-category-set'");
    }

    let mut len = SCHARS(categories) - 1;
    while len >= 0 {
        let cat = SREF(categories, len as usize);
        let category = make_fixnum(cat as EMACS_INT);
        CHECK_CATEGORY(category);
        set_category_set(val, cat as EMACS_INT, true);
        len -= 1;
    }
    val
}

pub fn category_table_p(arg: Lisp_Object) -> Lisp_Object {
    if CHAR_TABLE_P(arg) && EQ(char_table_purpose(arg), Qcategory_table()) {
        Qt()
    } else {
        Qnil()
    }
}

pub fn check_category_table(table: Lisp_Object) -> Lisp_Object {
    if NILP(table) {
        return current_buffer_category_table();
    }
    CHECK_TYPE(!NILP(Fcategory_table_p(table)), Qcategory_table_p(), table);
    table
}

pub fn define_category(
    category: Lisp_Object,
    docstring: Lisp_Object,
    table: Lisp_Object,
) -> Lisp_Object {
    CHECK_CATEGORY(category);
    CHECK_STRING(docstring);
    let table = check_category_table(table);

    if !NILP(CATEGORY_DOCSTRING(table, XFIXNAT(category) as c_int)) {
        error_category_already_defined(XFIXNAT(category) as c_int);
    }
    SET_CATEGORY_DOCSTRING!(table, XFIXNAT(category), docstring);
    Qnil()
}

pub fn category_docstring(category: Lisp_Object, table: Lisp_Object) -> Lisp_Object {
    CHECK_CATEGORY(category);
    let table = check_category_table(table);
    CATEGORY_DOCSTRING(table, XFIXNAT(category) as c_int)
}

pub fn get_unused_category(table: Lisp_Object) -> Lisp_Object {
    let table = check_category_table(table);
    for i in (b' ' as c_int)..=(b'~' as c_int) {
        if NILP(CATEGORY_DOCSTRING(table, i)) {
            return make_fixnum(i as EMACS_INT);
        }
    }
    Qnil()
}

pub fn category_table() -> Lisp_Object {
    current_buffer_category_table()
}

pub fn standard_category_table_subr() -> Lisp_Object {
    standard_category_table()
}

pub extern "C-unwind" fn copy_category_entry(
    table: Lisp_Object,
    c: Lisp_Object,
    val: Lisp_Object,
) {
    let copied = Fcopy_sequence(val);
    if CONSP(c) {
        char_table_set_range(
            table,
            XFIXNUM(XCAR(c)) as c_int,
            XFIXNUM(XCDR(c)) as c_int,
            copied,
        );
    } else {
        char_table_set(table, XFIXNUM(c) as c_int, copied);
    }
}

pub fn copy_category_table_internal(table: Lisp_Object) -> Lisp_Object {
    let copied = copy_char_table(table);
    let def = char_table_defalt(copied);
    if !NILP(def) {
        set_char_table_defalt(copied, Fcopy_sequence(def));
    }
    set_char_table_extras(copied, 0, Fcopy_sequence(char_table_extra_slot_ref(copied, 0)));
    map_char_table_c_fn(copy_category_entry, copied, copied);
    copied
}

pub fn copy_category_table_subr(table: Lisp_Object) -> Lisp_Object {
    let src = if !NILP(table) {
        check_category_table(table)
    } else {
        standard_category_table()
    };
    copy_category_table_internal(src)
}

pub fn make_category_table() -> Lisp_Object {
    let val = Fmake_char_table(Qcategory_table(), Qnil());
    set_char_table_defalt(val, MAKE_CATEGORY_SET());
    for i in 0..(1 << CHARTAB_SIZE_BITS_0) {
        set_char_table_contents(val, i as isize, MAKE_CATEGORY_SET());
    }
    Fset_char_table_extra_slot(val, make_fixnum(0), make_nil_vector(95));
    val
}

pub fn set_category_table(table: Lisp_Object) -> Lisp_Object {
    let table = check_category_table(table);
    set_current_buffer_category_table(table);
    table
}

pub fn char_category_set_impl(c: c_int) -> Lisp_Object {
    CHAR_TABLE_REF(current_buffer_category_table(), c as isize)
}

pub fn char_category_set_subr(ch: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(ch);
    CATEGORY_SET(XFIXNAT(ch) as c_int)
}

pub fn category_set_mnemonics(category_set: Lisp_Object) -> Lisp_Object {
    CHECK_CATEGORY_SET(category_set);
    let mut bytes = Vec::with_capacity(95);
    for i in 32..127 {
        if CATEGORY_MEMBER(i as EMACS_INT, category_set) {
            bytes.push(i as u8);
        }
    }
    make_string_bytes(&bytes)
}

pub fn modify_category_entry(
    character: Lisp_Object,
    category: Lisp_Object,
    table: Lisp_Object,
    reset: Lisp_Object,
) -> Lisp_Object {
    let (mut start, end) = if FIXNUMP(character) {
        CHECK_CHARACTER(character);
        let c = XFIXNAT(character) as c_int;
        (c, c)
    } else {
        CHECK_CONS(character);
        CHECK_CHARACTER_CAR(character);
        CHECK_CHARACTER_CDR(character);
        (XFIXNAT(XCAR(character)) as c_int, XFIXNAT(XCDR(character)) as c_int)
    };

    CHECK_CATEGORY(category);
    let table = check_category_table(table);

    if NILP(CATEGORY_DOCSTRING(table, XFIXNAT(category) as c_int)) {
        error_undefined_category(XFIXNAT(category) as c_int);
    }

    let set_value = NILP(reset);
    while start <= end {
        let (mut category_set, _from, to) = char_table_ref_and_range_safe(table, start, start, end);
        if CATEGORY_MEMBER(XFIXNAT(category), category_set) != NILP(reset) {
            category_set = Fcopy_sequence(category_set);
            set_category_set(category_set, XFIXNAT(category), set_value);
            category_set = hash_intern_category_set(table, category_set);
            char_table_set_range(table, start, to, category_set);
        }
        start = to + 1;
    }

    Qnil()
}

pub fn word_boundary_p_impl(c1: c_int, c2: c_int) -> bool {
    let script_table = Vchar_script_table.get();
    let (mut tail, default_result) =
        if EQ(CHAR_TABLE_REF(script_table, c1 as isize), CHAR_TABLE_REF(script_table, c2 as isize))
        {
            (Vword_separating_categories.get(), false)
        } else {
            (Vword_combining_categories.get(), true)
        };

    let category_set1 = CATEGORY_SET(c1);
    if NILP(category_set1) {
        return default_result;
    }
    let category_set2 = CATEGORY_SET(c2);
    if NILP(category_set2) {
        return default_result;
    }

    while CONSP(tail) {
        let elt = XCAR(tail);
        if CONSP(elt)
            && (NILP(XCAR(elt))
                || (CATEGORYP(XCAR(elt))
                    && CATEGORY_MEMBER(XFIXNAT(XCAR(elt)), category_set1)
                    && !CATEGORY_MEMBER(XFIXNAT(XCAR(elt)), category_set2)))
            && (NILP(XCDR(elt))
                || (CATEGORYP(XCDR(elt))
                    && !CATEGORY_MEMBER(XFIXNAT(XCDR(elt)), category_set1)
                    && CATEGORY_MEMBER(XFIXNAT(XCDR(elt)), category_set2)))
        {
            return !default_result;
        }
        tail = XCDR(tail);
    }
    default_result
}

pub fn init_category_once_impl() {
    Fput(Qcategory_table(), Qchar_table_extra_slots(), make_fixnum(2));
    let std_table = Fmake_char_table(Qcategory_table(), Qnil());
    set_standard_category_table(std_table);
    set_char_table_defalt(std_table, MAKE_CATEGORY_SET());
    Fset_char_table_extra_slot(std_table, make_fixnum(0), make_nil_vector(95));
}
