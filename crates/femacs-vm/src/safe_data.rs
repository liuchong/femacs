//! `#![forbid(unsafe_code)]` high-level Lisp type predicates, symbol/subr/bool-vector helpers,
//! and sequence/string/alist operations (`src/data.rs` + `src/fns.rs`, `U7` / `P-486`).
//!
//! Low-level forwarded-variable unions, buffer-local value cells, `Lisp_Subr` / `Lisp_Symbol`
//! raw layout access, `bits_word` backing arrays, and raw string byte slices stay behind the
//! controlled kernel boundary in `src/data.rs`, `src/fns.rs`, and `src/include/lisp.rs`.

#![forbid(unsafe_code)]

use crate::common::*;
use crate::roots::RootedVec;

// ---------------------------------------------------------------------------
// `src/data.rs` — type predicates, cons/symbol/subr/array/bool-vector ops
// ---------------------------------------------------------------------------

#[inline(never)]
pub fn slow_eq_impl(x: Lisp_Object, y: Lisp_Object) -> bool {
    let en = symbols_with_pos_enabled.get();
    let lhs = if en && SYMBOL_WITH_POS_P(x) {
        XSYMBOL_WITH_POS_SYM(x)
    } else {
        x
    };
    let rhs = if en && SYMBOL_WITH_POS_P(y) {
        XSYMBOL_WITH_POS_SYM(y)
    } else {
        y
    };
    BASE_EQ(lhs, rhs)
}

#[inline(always)]
pub fn eq_subr(obj1: Lisp_Object, obj2: Lisp_Object) -> Lisp_Object {
    if EQ(obj1, obj2) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn null_subr(object: Lisp_Object) -> Lisp_Object {
    if NILP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn type_of(object: Lisp_Object) -> Lisp_Object {
    if SYMBOLP(object) {
        Qsymbol()
    } else if INTEGERP(object) {
        Qinteger()
    } else if SUBRP(object) {
        Qsubr()
    } else {
        cl_type_of(object)
    }
}

pub fn cl_type_of(object: Lisp_Object) -> Lisp_Object {
    #[allow(unreachable_patterns)]
    match XTYPE(object) {
        Lisp_Type::Lisp_Int0 | Lisp_Type::Lisp_Int1 => Qfixnum(),
        Lisp_Type::Lisp_Symbol => {
            if NILP(object) {
                Qnull()
            } else if EQ(object, Qt()) {
                Qboolean()
            } else {
                Qsymbol()
            }
        }
        Lisp_Type::Lisp_String => Qstring(),
        Lisp_Type::Lisp_Cons => Qcons(),
        Lisp_Type::Lisp_Vectorlike => {
            match PSEUDOVECTOR_TYPE(XVECTOR(object)) {
                pvec_type::PVEC_NORMAL_VECTOR => return Qvector(),
                pvec_type::PVEC_BIGNUM => return Qbignum(),
                pvec_type::PVEC_MARKER => return Qmarker(),
                pvec_type::PVEC_SYMBOL_WITH_POS => return Qsymbol_with_pos(),
                pvec_type::PVEC_OVERLAY => return Qoverlay(),
                pvec_type::PVEC_FINALIZER => return Qfinalizer(),
                pvec_type::PVEC_USER_PTR => return Quser_ptr(),
                pvec_type::PVEC_WINDOW_CONFIGURATION => return Qwindow_configuration(),
                pvec_type::PVEC_PROCESS => return Qprocess(),
                pvec_type::PVEC_WINDOW => return Qwindow(),
                pvec_type::PVEC_SUBR => {
                    let (_, max_args) = crate::data::subr_arity_pair(object);
                    return if max_args as isize == UNEVALLED {
                        Qspecial_form()
                    } else if NATIVE_COMP_FUNCTIONP(object) {
                        Qnative_comp_function()
                    } else {
                        Qprimitive_function()
                    };
                }
                pvec_type::PVEC_CLOSURE => {
                    return if CONSP(AREF(object, CLOSURE_CODE as isize)) {
                        Qinterpreted_function()
                    } else {
                        Qbyte_code_function()
                    };
                }
                pvec_type::PVEC_BUFFER => return Qbuffer(),
                pvec_type::PVEC_CHAR_TABLE => return Qchar_table(),
                pvec_type::PVEC_BOOL_VECTOR => return Qbool_vector(),
                pvec_type::PVEC_FRAME => return Qframe(),
                pvec_type::PVEC_HASH_TABLE => return Qhash_table(),
                pvec_type::PVEC_OBARRAY => return Qobarray(),
                pvec_type::PVEC_FONT => {
                    if FONT_SPEC_P(object) {
                        return Qfont_spec();
                    }
                    if FONT_ENTITY_P(object) {
                        return Qfont_entity();
                    }
                    if FONT_OBJECT_P(object) {
                        return Qfont_object();
                    }
                    emacs_abort();
                }
                pvec_type::PVEC_THREAD => return Qthread(),
                pvec_type::PVEC_MUTEX => return Qmutex(),
                pvec_type::PVEC_CONDVAR => return Qcondition_variable(),
                pvec_type::PVEC_TERMINAL => return Qterminal(),
                pvec_type::PVEC_RECORD => {
                    let t_ = AREF(object, 0);
                    if RECORDP(t_) && 1 < PVSIZE(t_) {
                        return AREF(t_, 1);
                    }
                    return t_;
                }
                pvec_type::PVEC_MODULE_FUNCTION => return Qmodule_function(),
                pvec_type::PVEC_NATIVE_COMP_UNIT => return Qnative_comp_unit(),
                pvec_type::PVEC_XWIDGET => return Qxwidget(),
                pvec_type::PVEC_XWIDGET_VIEW => return Qxwidget_view(),
                pvec_type::PVEC_TS_PARSER => return Qtreesit_parser(),
                pvec_type::PVEC_TS_NODE => return Qtreesit_node(),
                pvec_type::PVEC_TS_COMPILED_QUERY => return Qtreesit_compiled_query(),
                pvec_type::PVEC_SQLITE => return Qsqlite(),
                pvec_type::PVEC_SUB_CHAR_TABLE => return Qsub_char_table(),
                pvec_type::PVEC_MISC_PTR | pvec_type::PVEC_OTHER | pvec_type::PVEC_FREE => {}
                _ => {}
            }
            emacs_abort();
        }
        Lisp_Type::Lisp_Float => Qfloat(),
        _ => emacs_abort(),
    }
}

#[inline(always)]
pub fn consp(object: Lisp_Object) -> Lisp_Object {
    if CONSP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn atom(object: Lisp_Object) -> Lisp_Object {
    if CONSP(object) { Qnil() } else { Qt() }
}

#[inline(always)]
pub fn listp(object: Lisp_Object) -> Lisp_Object {
    if CONSP(object) || NILP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn nlistp(object: Lisp_Object) -> Lisp_Object {
    if CONSP(object) || NILP(object) { Qnil() } else { Qt() }
}

#[inline(always)]
pub fn bare_symbol_p(object: Lisp_Object) -> Lisp_Object {
    if BARE_SYMBOL_P(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn symbol_with_pos_p(object: Lisp_Object) -> Lisp_Object {
    if SYMBOL_WITH_POS_P(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn symbolp(object: Lisp_Object) -> Lisp_Object {
    if SYMBOLP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn keywordp(object: Lisp_Object) -> Lisp_Object {
    if crate::data::symbol_is_keyword_safe(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn vectorp(object: Lisp_Object) -> Lisp_Object {
    if VECTORP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn recordp(object: Lisp_Object) -> Lisp_Object {
    if RECORDP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn stringp(object: Lisp_Object) -> Lisp_Object {
    if STRINGP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn multibyte_string_p(object: Lisp_Object) -> Lisp_Object {
    if STRINGP(object) && STRING_MULTIBYTE(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn char_table_p(object: Lisp_Object) -> Lisp_Object {
    if CHAR_TABLE_P(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn vector_or_char_table_p(object: Lisp_Object) -> Lisp_Object {
    if VECTORP(object) || CHAR_TABLE_P(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn bool_vector_p(object: Lisp_Object) -> Lisp_Object {
    if BOOL_VECTOR_P(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn arrayp(object: Lisp_Object) -> Lisp_Object {
    if ARRAYP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn sequencep(object: Lisp_Object) -> Lisp_Object {
    if CONSP(object) || NILP(object) || ARRAYP(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn bufferp(object: Lisp_Object) -> Lisp_Object {
    if BUFFERP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn markerp(object: Lisp_Object) -> Lisp_Object {
    if MARKERP(object) { Qt() } else { Qnil() }
}

#[cfg(feature = "have_modules")]
#[inline(always)]
pub fn user_ptrp(object: Lisp_Object) -> Lisp_Object {
    if USER_PTRP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn subrp(object: Lisp_Object) -> Lisp_Object {
    if SUBRP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn closurep(object: Lisp_Object) -> Lisp_Object {
    if CLOSUREP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn byte_code_function_p(object: Lisp_Object) -> Lisp_Object {
    if CLOSUREP(object) && STRINGP(AREF(object, CLOSURE_CODE as isize)) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn interpreted_function_p(object: Lisp_Object) -> Lisp_Object {
    if CLOSUREP(object) && CONSP(AREF(object, CLOSURE_CODE as isize)) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn module_function_p(object: Lisp_Object) -> Lisp_Object {
    if MODULE_FUNCTIONP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn char_or_string_p(object: Lisp_Object) -> Lisp_Object {
    if CHARACTERP(object) || STRINGP(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn integerp(object: Lisp_Object) -> Lisp_Object {
    if INTEGERP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn integer_or_marker_p(object: Lisp_Object) -> Lisp_Object {
    if MARKERP(object) || INTEGERP(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn natnump(object: Lisp_Object) -> Lisp_Object {
    let ok = if FIXNUMP(object) {
        0 <= XFIXNUM(object)
    } else {
        BIGNUMP(object) && 0 <= crate::data::bignum_sign_safe(object)
    };
    if ok { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn numberp(object: Lisp_Object) -> Lisp_Object {
    if NUMBERP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn number_or_marker_p(object: Lisp_Object) -> Lisp_Object {
    if NUMBERP(object) || MARKERP(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn floatp(object: Lisp_Object) -> Lisp_Object {
    if FLOATP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn threadp(object: Lisp_Object) -> Lisp_Object {
    if THREADP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn mutexp(object: Lisp_Object) -> Lisp_Object {
    if MUTEXP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn condition_variable_p(object: Lisp_Object) -> Lisp_Object {
    if CONDVARP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn car_subr(list: Lisp_Object) -> Lisp_Object {
    CAR(list)
}

#[inline(always)]
pub fn car_safe_subr(object: Lisp_Object) -> Lisp_Object {
    CAR_SAFE(object)
}

#[inline(always)]
pub fn cdr_subr(list: Lisp_Object) -> Lisp_Object {
    CDR(list)
}

#[inline(always)]
pub fn cdr_safe_subr(object: Lisp_Object) -> Lisp_Object {
    CDR_SAFE(object)
}

#[inline(always)]
pub fn setcar(cell: Lisp_Object, newcar: Lisp_Object) -> Lisp_Object {
    CHECK_CONS(cell);
    XSETCAR(cell, newcar);
    newcar
}

#[inline(always)]
pub fn setcdr(cell: Lisp_Object, newcdr: Lisp_Object) -> Lisp_Object {
    CHECK_CONS(cell);
    XSETCDR(cell, newcdr);
    newcdr
}

#[inline(always)]
pub fn fboundp(symbol: Lisp_Object) -> Lisp_Object {
    if symbol_fbound_p_safe(symbol) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn fmakunbound(symbol: Lisp_Object) -> Lisp_Object {
    CHECK_SYMBOL(symbol);
    if NILP(symbol) || EQ(symbol, Qt()) {
        xsignal1(Qsetting_constant(), symbol);
    }
    set_symbol_function(symbol, Qnil());
    symbol
}

#[inline(always)]
pub fn symbol_function(symbol: Lisp_Object) -> Lisp_Object {
    symbol_function_cell(symbol)
}

#[inline(always)]
pub fn symbol_plist(symbol: Lisp_Object) -> Lisp_Object {
    symbol_plist_cell(symbol)
}

#[inline(always)]
pub fn symbol_name(symbol: Lisp_Object) -> Lisp_Object {
    symbol_name_cell(symbol)
}

pub fn bare_symbol(sym: Lisp_Object) -> Lisp_Object {
    if BARE_SYMBOL_P(sym) {
        return sym;
    }
    if SYMBOL_WITH_POS_P(sym) {
        return XSYMBOL_WITH_POS_SYM(sym);
    }
    wrong_type_argument(list2(Qsymbolp(), Qsymbol_with_pos_p()), sym);
}

#[inline(always)]
pub fn symbol_with_pos_pos(sympos: Lisp_Object) -> Lisp_Object {
    CHECK_TYPE(SYMBOL_WITH_POS_P(sympos), Qsymbol_with_pos_p(), sympos);
    XSYMBOL_WITH_POS_POS(sympos)
}

#[inline(always)]
pub fn remove_pos_from_symbol(arg: Lisp_Object) -> Lisp_Object {
    if SYMBOL_WITH_POS_P(arg) {
        XSYMBOL_WITH_POS_SYM(arg)
    } else {
        arg
    }
}

pub fn position_symbol(sym: Lisp_Object, pos: Lisp_Object) -> Lisp_Object {
    let bare = bare_symbol(sym);
    let position = if FIXNUMP(pos) {
        pos
    } else if SYMBOL_WITH_POS_P(pos) {
        XSYMBOL_WITH_POS_POS(pos)
    } else {
        wrong_type_argument(Qfixnum_or_symbol_with_pos_p(), pos);
    };
    build_symbol_with_pos(bare, position)
}

pub fn fset(symbol: Lisp_Object, definition: Lisp_Object) -> Lisp_Object {
    CHECK_SYMBOL(symbol);
    if NILP(symbol) && !NILP(definition) {
        xsignal1(Qsetting_constant(), symbol);
    }

    let mut s = definition;
    while SYMBOLP(s) && !NILP(s) {
        if EQ(s, symbol) {
            xsignal1(Qcyclic_function_indirection(), symbol);
        }
        s = symbol_function_cell(s);
    }

    #[cfg(feature = "have_native_comp")]
    {
        let function = symbol_function_cell(symbol);
        if !NILP(Vnative_comp_enable_subr_trampolines.get())
            && SUBRP(function)
            && !NATIVE_COMP_FUNCTIONP(function)
            && !EQ(definition, symbol_function(symbol))
        {
            calln!(Qcomp_subr_trampoline_install(), symbol);
        }
    }

    set_symbol_function(symbol, definition);
    definition
}

#[inline(always)]
pub fn setplist(symbol: Lisp_Object, newplist: Lisp_Object) -> Lisp_Object {
    CHECK_SYMBOL(symbol);
    set_symbol_plist(symbol, newplist);
    newplist
}

pub fn subr_arity(subr: Lisp_Object) -> Lisp_Object {
    CHECK_SUBR(subr);
    let (minargs, maxargs) = crate::data::subr_arity_pair(subr);
    Fcons(
        make_fixnum(minargs as EMACS_INT),
        if maxargs as isize == MANY {
            Qmany()
        } else if maxargs as isize == UNEVALLED {
            Qunevalled()
        } else {
            make_fixnum(maxargs as EMACS_INT)
        },
    )
}

#[inline(always)]
pub fn subr_name(subr: Lisp_Object) -> Lisp_Object {
    CHECK_SUBR(subr);
    crate::data::subr_name_string(subr)
}

#[inline(always)]
pub fn native_comp_function_p(object: Lisp_Object) -> Lisp_Object {
    if NATIVE_COMP_FUNCTIONP(object) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn subr_native_lambda_list(subr: Lisp_Object) -> Lisp_Object {
    CHECK_SUBR(subr);
    crate::data::subr_native_lambda_list_or_t(subr)
}

#[inline(always)]
pub fn subr_type(subr: Lisp_Object) -> Lisp_Object {
    CHECK_SUBR(subr);
    crate::data::subr_type_obj(subr)
}

#[cfg(feature = "have_native_comp")]
#[inline(always)]
pub fn subr_native_comp_unit(subr: Lisp_Object) -> Lisp_Object {
    CHECK_SUBR(subr);
    crate::data::subr_native_comp_unit_obj(subr)
}

#[cfg(feature = "have_native_comp")]
#[inline(always)]
pub fn native_comp_unit_file(unit: Lisp_Object) -> Lisp_Object {
    crate::data::native_comp_unit_file_obj(unit)
}

#[cfg(feature = "have_native_comp")]
#[inline(always)]
pub fn native_comp_unit_set_file(unit: Lisp_Object, new_file: Lisp_Object) -> Lisp_Object {
    crate::data::set_native_comp_unit_file_obj(unit, new_file)
}

pub fn interactive_form(cmd: Lisp_Object) -> Lisp_Object {
    let mut fun = crate::data::indirect_function_safe(cmd);
    let mut genfun: bool = false;

    if NILP(fun) {
        return Qnil();
    }

    fun = cmd;
    while SYMBOLP(fun) {
        let tmp = Fget(fun, Qinteractive_form());
        if !NILP(tmp) {
            return tmp;
        }
        fun = symbol_function(fun);
    }

    if SUBRP(fun) {
        let form = crate::data::subr_interactive_form_or_nil(fun);
        if !NILP(form) {
            return form;
        }
    } else if CLOSUREP(fun) {
        if PVSIZE(fun) > CLOSURE_INTERACTIVE as isize {
            let form = AREF(fun, CLOSURE_INTERACTIVE as isize);
            return list2(
                Qinteractive(),
                if VECTORP(form) { AREF(form, 0) } else { form },
            );
        } else if PVSIZE(fun) > CLOSURE_DOC_STRING as isize {
            let doc = AREF(fun, CLOSURE_DOC_STRING as isize);
            genfun = !(NILP(doc) || VALID_DOCSTRING_P(doc));
        }
    } else if cfg!(feature = "have_modules") && MODULE_FUNCTIONP(fun) {
        #[cfg(feature = "have_modules")]
        {
            let form = crate::data::module_fn_interactive_form(fun);
            if !NILP(form) {
                return form;
            }
        }
    } else if AUTOLOADP(fun) {
        return interactive_form(Fautoload_do_load(fun, cmd, Qnil()));
    } else if CONSP(fun) {
        let funcar = XCAR(fun);
        if EQ(funcar, Qlambda()) {
            let form = cdr_subr(XCDR(fun));
            let spec = assq_subr(Qinteractive(), form);
            if NILP(cdr_subr(cdr_subr(spec))) {
                return spec;
            }
            return list2(Qinteractive(), car_subr(cdr_subr(spec)));
        }
    }

    if genfun && !NILP(symbol_function(Qoclosure_interactive_form())) {
        calln!(Qoclosure_interactive_form(), fun)
    } else {
        Qnil()
    }
}

pub fn command_modes(command: Lisp_Object) -> Lisp_Object {
    let mut fun = crate::data::indirect_function_safe(command);

    if NILP(fun) {
        return Qnil();
    }

    fun = command;
    while SYMBOLP(fun) {
        let modes = Fget(fun, Qcommand_modes());
        if !NILP(modes) {
            return modes;
        }
        fun = symbol_function(fun);
    }

    if SUBRP(fun) {
        return crate::data::subr_command_modes_obj(fun);
    } else if CLOSUREP(fun) {
        if PVSIZE(fun) <= CLOSURE_INTERACTIVE as isize {
            return Qnil();
        }
        let form = AREF(fun, CLOSURE_INTERACTIVE as isize);
        if VECTORP(form) {
            return AREF(form, 1);
        }
        return Qnil();
    } else if cfg!(feature = "have_modules") && MODULE_FUNCTIONP(fun) {
        #[cfg(feature = "have_modules")]
        {
            let form = crate::data::module_fn_command_modes(fun);
            if !NILP(form) {
                return form;
            }
        }
    } else if AUTOLOADP(fun) {
        let modes = nth_subr(make_int(3), fun);
        return if CONSP(modes) { modes } else { Qnil() };
    } else if CONSP(fun) {
        let funcar = XCAR(fun);
        if EQ(funcar, Qlambda()) {
            let form = cdr_subr(XCDR(fun));
            return cdr_subr(cdr_subr(assq_subr(Qinteractive(), form)));
        }
    }
    Qnil()
}

#[inline(always)]
pub fn indirect_variable(object: Lisp_Object) -> Lisp_Object {
    crate::data::symbol_follow_varalias_chain(object)
}

#[inline(always)]
pub fn indirect_function_subr(object: Lisp_Object, _noerror: Lisp_Object) -> Lisp_Object {
    crate::data::indirect_function_safe(object)
}

pub fn aref_subr(array: Lisp_Object, idx: Lisp_Object) -> Lisp_Object {
    if FIXNUMP(idx) && VECTORP(array) {
        let idxval = XFIXNUM(idx);
        let size = ASIZE(array);
        if idxval >= 0 && idxval < size {
            return AREF(array, idxval);
        }
    }

    CHECK_FIXNUM(idx);
    let idxval = XFIXNUM(idx);

    if STRINGP(array) {
        if idxval < 0 || idxval >= SCHARS(array) {
            args_out_of_range(array, idx);
        }
        if !STRING_MULTIBYTE(array) {
            return make_fixnum(SREF(array, idxval as usize) as EMACS_INT);
        }
        let idxval_byte = string_char_to_byte(array, idxval);
        let (c, _) = fetch_string_char_at(array, idxval, idxval_byte);
        make_fixnum(c as EMACS_INT)
    } else if BOOL_VECTOR_P(array) {
        if idxval < 0 || idxval >= bool_vector_size(array) {
            args_out_of_range(array, idx);
        }
        bool_vector_ref(array, idxval)
    } else if CHAR_TABLE_P(array) {
        CHECK_CHARACTER(idx);
        CHAR_TABLE_REF(array, idxval)
    } else {
        let size = if VECTORP(array) {
            ASIZE(array)
        } else if CLOSUREP(array) || RECORDP(array) {
            PVSIZE(array)
        } else {
            wrong_type_argument(Qarrayp(), array);
        };
        if idxval < 0 || idxval >= size {
            args_out_of_range(array, idx);
        }
        AREF(array, idxval)
    }
}

pub fn aset_subr(array: Lisp_Object, idx: Lisp_Object, newelt: Lisp_Object) -> Lisp_Object {
    if FIXNUMP(idx) && VECTORP(array) {
        let idxval = XFIXNUM(idx);
        let size = ASIZE(array);
        if idxval >= 0 && idxval < size {
            ASET(array, idxval, newelt);
            return newelt;
        }
    }

    CHECK_FIXNUM(idx);
    let idxval = XFIXNUM(idx);
    if !RECORDP(array) {
        CHECK_ARRAY(array, Qarrayp());
    }

    if VECTORP(array) {
        if idxval < 0 || idxval >= ASIZE(array) {
            args_out_of_range(array, idx);
        }
        ASET(array, idxval, newelt);
    } else if BOOL_VECTOR_P(array) {
        if idxval < 0 || idxval >= bool_vector_size(array) {
            args_out_of_range(array, idx);
        }
        bool_vector_set(array, idxval, !NILP(newelt));
    } else if CHAR_TABLE_P(array) {
        CHECK_CHARACTER(idx);
        CHAR_TABLE_SET(array, idxval, newelt);
    } else if RECORDP(array) {
        if idxval < 0 || idxval >= PVSIZE(array) {
            args_out_of_range(array, idx);
        }
        ASET(array, idxval, newelt);
    } else {
        if idxval < 0 || idxval >= SCHARS(array) {
            args_out_of_range(array, idx);
        }
        CHECK_CHARACTER(newelt);
        let c = XFIXNAT(newelt) as c_int;

        if STRING_MULTIBYTE(array) {
            if c > 0x7f {
                crate::data::error_store_non_ascii_in_multibyte_string();
            }
            let idxval_byte = string_char_to_byte(array, idxval) as usize;
            if SREF(array, idxval_byte) > 0x7f {
                crate::data::error_replace_non_ascii_in_multibyte_string();
            }
            SSET(array, idxval_byte, c as u8);
        } else {
            if c > 0xff {
                crate::data::error_store_non_byte_in_unibyte_string();
            }
            SSET(array, idxval as usize, c as u8);
        }
    }
    newelt
}

#[inline(always)]
pub fn number_to_string_subr(number: Lisp_Object) -> Lisp_Object {
    number_to_string_safe(number)
}

pub fn string_to_number_subr(string: Lisp_Object, base: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(string);
    let b: c_int = if NILP(base) {
        10
    } else {
        CHECK_FIXNUM(base);
        if !(XFIXNUM(base) >= 2 && XFIXNUM(base) <= 16) {
            xsignal1(Qargs_out_of_range(), base);
        }
        XFIXNUM(base) as c_int
    };
    let val = string_to_number_after_ws(string, b);
    if if IEEE_FLOATING_POINT {
        NILP(val)
    } else {
        !NUMBERP(val)
    } {
        make_fixnum(0)
    } else {
        val
    }
}

#[inline(always)]
pub fn byteorder_subr() -> Lisp_Object {
    let ch = if cfg!(target_endian = "big") {
        b'B'
    } else {
        b'l'
    };
    make_fixnum(ch as EMACS_INT)
}

pub fn bool_vector_binop_driver_safe(
    a: Lisp_Object,
    b: Lisp_Object,
    dest: Lisp_Object,
    op: c_int,
) -> Lisp_Object {
    CHECK_BOOL_VECTOR(a);
    CHECK_BOOL_VECTOR(b);
    let nr_bits = bool_vector_size(a);
    if bool_vector_size(b) != nr_bits {
        crate::data::wrong_length_argument_safe(a, b, dest);
    }

    let nr_words = bool_vector_words(nr_bits);
    let mut i: ptrdiff_t = 0;
    let dest = if NILP(dest) {
        make_uninit_bool_vector(nr_bits)
    } else {
        CHECK_BOOL_VECTOR(dest);
        if bool_vector_size(dest) != nr_bits {
            crate::data::wrong_length_argument_safe(a, b, dest);
        }

        'set_dest: loop {
            match op {
                x if x == crate::data::BOOL_VEC_EXCLUSIVE_OR => {
                    while i < nr_words {
                        if crate::data::bool_vector_word_get(dest, i)
                            != (crate::data::bool_vector_word_get(a, i)
                                ^ crate::data::bool_vector_word_get(b, i))
                        {
                            break 'set_dest;
                        }
                        i += 1;
                    }
                }
                x if x == crate::data::BOOL_VEC_SUBSETP => {
                    while i < nr_words {
                        if (crate::data::bool_vector_word_get(a, i)
                            & !crate::data::bool_vector_word_get(b, i))
                            != 0
                        {
                            return Qnil();
                        }
                        i += 1;
                    }
                    return Qt();
                }
                x if x == crate::data::BOOL_VEC_UNION => {
                    while i < nr_words {
                        if crate::data::bool_vector_word_get(dest, i)
                            != (crate::data::bool_vector_word_get(a, i)
                                | crate::data::bool_vector_word_get(b, i))
                        {
                            break 'set_dest;
                        }
                        i += 1;
                    }
                }
                x if x == crate::data::BOOL_VEC_INTERSECTION => {
                    while i < nr_words {
                        if crate::data::bool_vector_word_get(dest, i)
                            != (crate::data::bool_vector_word_get(a, i)
                                & crate::data::bool_vector_word_get(b, i))
                        {
                            break 'set_dest;
                        }
                        i += 1;
                    }
                }
                x if x == crate::data::BOOL_VEC_SET_DIFFERENCE => {
                    while i < nr_words {
                        if crate::data::bool_vector_word_get(dest, i)
                            != (crate::data::bool_vector_word_get(a, i)
                                & !crate::data::bool_vector_word_get(b, i))
                        {
                            break 'set_dest;
                        }
                        i += 1;
                    }
                }
                _ => emacs_abort(),
            }
            return Qnil();
        }
        dest
    };

    match op {
        x if x == crate::data::BOOL_VEC_EXCLUSIVE_OR => {
            while i < nr_words {
                let w = crate::data::bool_vector_word_get(a, i)
                    ^ crate::data::bool_vector_word_get(b, i);
                crate::data::bool_vector_word_set(dest, i, w);
                i += 1;
            }
        }
        x if x == crate::data::BOOL_VEC_UNION => {
            while i < nr_words {
                let w = crate::data::bool_vector_word_get(a, i)
                    | crate::data::bool_vector_word_get(b, i);
                crate::data::bool_vector_word_set(dest, i, w);
                i += 1;
            }
        }
        x if x == crate::data::BOOL_VEC_INTERSECTION => {
            while i < nr_words {
                let w = crate::data::bool_vector_word_get(a, i)
                    & crate::data::bool_vector_word_get(b, i);
                crate::data::bool_vector_word_set(dest, i, w);
                i += 1;
            }
        }
        x if x == crate::data::BOOL_VEC_SET_DIFFERENCE => {
            while i < nr_words {
                let w = crate::data::bool_vector_word_get(a, i)
                    & !crate::data::bool_vector_word_get(b, i);
                crate::data::bool_vector_word_set(dest, i, w);
                i += 1;
            }
        }
        _ => emacs_abort(),
    }

    dest
}

#[inline(always)]
pub fn bool_vector_exclusive_or_subr(
    a: Lisp_Object,
    b: Lisp_Object,
    c: Lisp_Object,
) -> Lisp_Object {
    bool_vector_binop_driver_safe(a, b, c, crate::data::BOOL_VEC_EXCLUSIVE_OR)
}

#[inline(always)]
pub fn bool_vector_union_subr(a: Lisp_Object, b: Lisp_Object, c: Lisp_Object) -> Lisp_Object {
    bool_vector_binop_driver_safe(a, b, c, crate::data::BOOL_VEC_UNION)
}

#[inline(always)]
pub fn bool_vector_intersection_subr(
    a: Lisp_Object,
    b: Lisp_Object,
    c: Lisp_Object,
) -> Lisp_Object {
    bool_vector_binop_driver_safe(a, b, c, crate::data::BOOL_VEC_INTERSECTION)
}

#[inline(always)]
pub fn bool_vector_set_difference_subr(
    a: Lisp_Object,
    b: Lisp_Object,
    c: Lisp_Object,
) -> Lisp_Object {
    bool_vector_binop_driver_safe(a, b, c, crate::data::BOOL_VEC_SET_DIFFERENCE)
}

#[inline(always)]
pub fn bool_vector_subsetp_subr(a: Lisp_Object, b: Lisp_Object) -> Lisp_Object {
    bool_vector_binop_driver_safe(a, b, b, crate::data::BOOL_VEC_SUBSETP)
}

pub fn bool_vector_not_subr(a: Lisp_Object, mut b: Lisp_Object) -> Lisp_Object {
    CHECK_BOOL_VECTOR(a);
    let nr_bits = bool_vector_size(a);

    if NILP(b) {
        b = make_uninit_bool_vector(nr_bits);
    } else {
        CHECK_BOOL_VECTOR(b);
        if bool_vector_size(b) != nr_bits {
            crate::data::wrong_length_argument_safe(a, b, Qnil());
        }
    }

    let mut i: ptrdiff_t = 0;
    let full_words = nr_bits / BITS_PER_BITS_WORD as EMACS_INT;
    while i < full_words {
        let w = BITS_WORD_MAX & !crate::data::bool_vector_word_get(a, i);
        crate::data::bool_vector_word_set(b, i, w);
        i += 1;
    }

    if nr_bits % BITS_PER_BITS_WORD as EMACS_INT != 0 {
        let mut mword = bits_word_to_host_endian(crate::data::bool_vector_word_get(a, i));
        mword = !mword;
        mword &= bool_vector_spare_mask(nr_bits);
        crate::data::bool_vector_word_set(b, i, bits_word_to_host_endian(mword));
    }

    b
}

pub fn bool_vector_count_population_subr(a: Lisp_Object) -> Lisp_Object {
    CHECK_BOOL_VECTOR(a);
    let nr_bits = bool_vector_size(a);
    let nwords = bool_vector_words(nr_bits);
    let mut count: EMACS_INT = 0;
    let mut i: ptrdiff_t = 0;
    while i < nwords {
        count += crate::data::bool_vector_word_get(a, i).count_ones() as EMACS_INT;
        i += 1;
    }
    make_fixnum(count)
}

pub fn bool_vector_count_consecutive_subr(
    a: Lisp_Object,
    b: Lisp_Object,
    i: Lisp_Object,
) -> Lisp_Object {
    CHECK_BOOL_VECTOR(a);
    CHECK_FIXNAT(i);

    let nr_bits = bool_vector_size(a);
    let start_bit = XFIXNAT(i);
    if start_bit > nr_bits {
        args_out_of_range(a, i);
    }

    let nr_words = bool_vector_words(nr_bits);
    let mut pos: ptrdiff_t = start_bit / BITS_PER_BITS_WORD as EMACS_INT;
    let offset: c_int = (start_bit % BITS_PER_BITS_WORD as EMACS_INT) as c_int;
    let mut count: EMACS_INT = 0;
    let twiddle: bits_word = if NILP(b) { 0 } else { BITS_WORD_MAX };

    if pos < nr_words && offset != 0 {
        let mut mword = bits_word_to_host_endian(crate::data::bool_vector_word_get(a, pos));
        mword ^= twiddle;
        mword >>= offset;
        mword |= (1 as bits_word) << ((BITS_PER_BITS_WORD as c_int - offset) as usize);
        count = mword.trailing_zeros() as EMACS_INT;
        pos += 1;
        if (count + offset as EMACS_INT) < (BITS_PER_BITS_WORD as EMACS_INT) {
            return make_fixnum(count);
        }
    }

    let pos0 = pos;
    while pos < nr_words && crate::data::bool_vector_word_get(a, pos) == twiddle {
        pos += 1;
    }
    count += (pos - pos0) * BITS_PER_BITS_WORD as EMACS_INT;

    if pos < nr_words {
        let mut mword = bits_word_to_host_endian(crate::data::bool_vector_word_get(a, pos));
        mword ^= twiddle;
        count += mword.trailing_zeros() as EMACS_INT;
    } else if nr_bits % BITS_PER_BITS_WORD as EMACS_INT != 0 {
        count -= BITS_PER_BITS_WORD as EMACS_INT - nr_bits % BITS_PER_BITS_WORD as EMACS_INT;
    }

    make_fixnum(count)
}

// ---------------------------------------------------------------------------
// `src/fns.rs` — sequence, string, list, and alist operations
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn identity(argument: Lisp_Object) -> Lisp_Object {
    argument
}

pub fn list_length_impl(mut list: Lisp_Object) -> ptrdiff_t {
    let mut i: ptrdiff_t = 0;
    let orig = list;
    while CONSP(list) && i < FOR_EACH_TAIL_THRESHOLD as ptrdiff_t {
        i += 1;
        list = XCDR(list);
    }
    if NILP(list) {
        return i;
    }
    list = orig;
    i = 0;
    let mut tortoise = list;
    while CONSP(list) {
        i += 1;
        list = XCDR(list);
        if BASE_EQ(list, tortoise) {
            circular_list(list);
        } else if (i & (FOR_EACH_TAIL_THRESHOLD as ptrdiff_t - 1)) == 0 {
            maybe_quit();
            if (i & (i - 1)) == 0 {
                tortoise = list;
            }
        }
    }
    CHECK_LIST_END(list, list);
    i
}

pub fn length_subr(sequence: Lisp_Object) -> Lisp_Object {
    let val = if NILP(sequence) {
        return make_fixnum(0);
    } else if CONSP(sequence) {
        list_length_impl(sequence)
    } else if STRINGP(sequence) {
        SCHARS(sequence)
    } else if VECTORP(sequence) {
        ASIZE(sequence)
    } else if CHAR_TABLE_P(sequence) {
        (MAX_CHAR + 1) as EMACS_INT
    } else if BOOL_VECTOR_P(sequence) {
        bool_vector_size(sequence)
    } else if CLOSUREP(sequence) || RECORDP(sequence) {
        PVSIZE(sequence)
    } else {
        wrong_type_argument(Qsequencep(), sequence);
    };
    make_fixnum(val)
}

pub fn safe_length(mut list: Lisp_Object) -> Lisp_Object {
    let mut len: ptrdiff_t = 0;
    FOR_EACH_TAIL_SAFE!(list, {
        len += 1;
    });
    make_fixnum(len)
}

fn length_internal(mut sequence: Lisp_Object, mut len: c_int) -> EMACS_INT {
    if len < 0xffff {
        while CONSP(sequence) {
            len -= 1;
            if len <= 0 {
                return -1;
            }
            sequence = XCDR(sequence);
        }
    } else {
        FOR_EACH_TAIL!(sequence, {
            len -= 1;
            if len <= 0 {
                return -1;
            }
        });
    }
    len as EMACS_INT
}

pub fn length_less(sequence: Lisp_Object, length: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(length);
    let len = XFIXNUM(length);
    if CONSP(sequence) {
        if length_internal(sequence, len as c_int) == -1 {
            Qnil()
        } else {
            Qt()
        }
    } else if XFIXNUM(length_subr(sequence)) < len {
        Qt()
    } else {
        Qnil()
    }
}

pub fn length_greater(sequence: Lisp_Object, length: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(length);
    let len = XFIXNUM(length);
    if CONSP(sequence) {
        if length_internal(sequence, (len + 1) as c_int) == -1 {
            Qt()
        } else {
            Qnil()
        }
    } else if XFIXNUM(length_subr(sequence)) > len {
        Qt()
    } else {
        Qnil()
    }
}

pub fn length_equal(sequence: Lisp_Object, length: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(length);
    let len = XFIXNUM(length);
    if len < 0 {
        return Qnil();
    }
    if CONSP(sequence) {
        if length_internal(sequence, (len + 1) as c_int) == 1 {
            Qt()
        } else {
            Qnil()
        }
    } else if XFIXNUM(length_subr(sequence)) == len {
        Qt()
    } else {
        Qnil()
    }
}

pub fn proper_list_p(object: Lisp_Object) -> Lisp_Object {
    let mut len: ptrdiff_t = 0;
    let mut last_tail = object;
    let mut tail = object;
    FOR_EACH_TAIL_SAFE!(tail, {
        len += 1;
        rarely_quit(len as u64);
        last_tail = XCDR(tail);
    });
    if !NILP(last_tail) {
        return Qnil();
    }
    make_fixnum(len)
}

#[inline(always)]
pub fn string_bytes(string: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(string);
    make_fixnum(SBYTES(string))
}

pub fn string_distance(
    string1: Lisp_Object,
    string2: Lisp_Object,
    bytecompare: Lisp_Object,
) -> Lisp_Object {
    CHECK_STRING(string1);
    CHECK_STRING(string2);

    let use_byte_compare =
        !NILP(bytecompare) || (!STRING_MULTIBYTE(string1) && !STRING_MULTIBYTE(string2));
    let len1 = if use_byte_compare {
        SBYTES(string1)
    } else {
        SCHARS(string1)
    };
    let len2 = if use_byte_compare {
        SBYTES(string2)
    } else {
        SCHARS(string2)
    };

    let n1 = len1 as usize;
    let n2 = len2 as usize;
    let mut column: Vec<ptrdiff_t> = (0..=len1).collect();

    if use_byte_compare {
        let mut x: usize = 1;
        while x <= n2 {
            column[0] = x as ptrdiff_t;
            let b2 = SREF(string2, x - 1);
            let mut lastdiag = (x - 1) as ptrdiff_t;
            let mut y: usize = 1;
            while y <= n1 {
                let olddiag = column[y];
                let cost = if SREF(string1, y - 1) == b2 { 0 } else { 1 };
                column[y] = (column[y] + 1).min(column[y - 1] + 1).min(lastdiag + cost);
                lastdiag = olddiag;
                y += 1;
            }
            x += 1;
        }
    } else {
        let mut i2: ptrdiff_t = 0;
        let mut i2_byte: ptrdiff_t = 0;
        let mut x: usize = 1;
        while x <= n2 {
            column[0] = x as ptrdiff_t;
            let (c2, next_i2, next_i2_byte) = fetch_string_char_step(string2, i2, i2_byte);
            i2 = next_i2;
            i2_byte = next_i2_byte;

            let mut i1: ptrdiff_t = 0;
            let mut i1_byte: ptrdiff_t = 0;
            let mut lastdiag = (x - 1) as ptrdiff_t;
            let mut y: usize = 1;
            while y <= n1 {
                let olddiag = column[y];
                let (c1, next_i1, next_i1_byte) = fetch_string_char_step(string1, i1, i1_byte);
                i1 = next_i1;
                i1_byte = next_i1_byte;
                let cost = if c1 == c2 { 0 } else { 1 };
                column[y] = (column[y] + 1).min(column[y - 1] + 1).min(lastdiag + cost);
                lastdiag = olddiag;
                y += 1;
            }
            x += 1;
        }
    }

    make_fixnum(column[n1])
}

pub fn string_equal(mut s1: Lisp_Object, mut s2: Lisp_Object) -> Lisp_Object {
    if SYMBOLP(s1) {
        s1 = SYMBOL_NAME(s1);
    }
    if SYMBOLP(s2) {
        s2 = SYMBOL_NAME(s2);
    }
    CHECK_STRING(s1);
    CHECK_STRING(s2);
    if lisp_string_memcmp_eq(s1, s2) {
        Qt()
    } else {
        Qnil()
    }
}

pub fn compare_strings(
    str1: Lisp_Object,
    start1: Lisp_Object,
    mut end1: Lisp_Object,
    str2: Lisp_Object,
    start2: Lisp_Object,
    mut end2: Lisp_Object,
    ignore_case: Lisp_Object,
) -> Lisp_Object {
    CHECK_STRING(str1);
    CHECK_STRING(str2);

    if FIXNUMP(end1) && SCHARS(str1) < XFIXNUM(end1) {
        end1 = make_fixnum(SCHARS(str1));
    }
    if FIXNUMP(end2) && SCHARS(str2) < XFIXNUM(end2) {
        end2 = make_fixnum(SCHARS(str2));
    }

    let (from1, to1) = validate_subarray_safe(str1, start1, end1, SCHARS(str1));
    let (from2, to2) = validate_subarray_safe(str2, start2, end2, SCHARS(str2));

    let mut i1 = from1;
    let mut i2 = from2;
    let mut i1_byte = string_char_to_byte(str1, i1);
    let mut i2_byte = string_char_to_byte(str2, i2);

    while i1 < to1 && i2 < to2 {
        let (mut c1, next_i1, next_i1_byte) =
            fetch_string_char_as_multibyte_step(str1, i1, i1_byte);
        let (mut c2, next_i2, next_i2_byte) =
            fetch_string_char_as_multibyte_step(str2, i2, i2_byte);
        i1 = next_i1;
        i1_byte = next_i1_byte;
        i2 = next_i2;
        i2_byte = next_i2_byte;

        if c1 == c2 {
            continue;
        }
        if !NILP(ignore_case) {
            c1 = XFIXNUM(Fupcase(make_fixnum(c1 as EMACS_INT))) as c_int;
            c2 = XFIXNUM(Fupcase(make_fixnum(c2 as EMACS_INT))) as c_int;
        }
        if c1 == c2 {
            continue;
        }
        return if c1 < c2 {
            make_fixnum(-i1 + from1)
        } else {
            make_fixnum(i1 - from1)
        };
    }

    if i1 < to1 {
        return make_fixnum(i1 - from1 + 1);
    }
    if i2 < to2 {
        return make_fixnum(-i1 + from1 - 1);
    }
    Qt()
}

pub fn string_cmp_safe(string1: Lisp_Object, string2: Lisp_Object) -> c_int {
    let chars1 = SCHARS(string1);
    let chars2 = SCHARS(string2);
    let n = chars1.min(chars2);

    if (!STRING_MULTIBYTE(string1) || chars1 == SBYTES(string1))
        && (!STRING_MULTIBYTE(string2) || chars2 == SBYTES(string2))
    {
        let mut idx: usize = 0;
        let lim = n as usize;
        while idx < lim {
            let b1 = SREF(string1, idx);
            let b2 = SREF(string2, idx);
            if b1 != b2 {
                return if b1 < b2 { -1 } else { 1 };
            }
            idx += 1;
        }
        if n < chars2 {
            -1
        } else if n < chars1 {
            1
        } else {
            0
        }
    } else if STRING_MULTIBYTE(string1) && STRING_MULTIBYTE(string2) {
        let nb1 = SBYTES(string1);
        let nb2 = SBYTES(string2);
        let nb = nb1.min(nb2);
        let mut b: ptrdiff_t = 0;

        while b < nb && SREF(string1, b as usize) == SREF(string2, b as usize) {
            b += 1;
        }
        if b >= nb {
            return if b < nb2 {
                -1
            } else if b < nb1 {
                1
            } else {
                0
            };
        }
        while (SREF(string1, b as usize) & 0xc0) == 0x80 {
            b -= 1;
        }
        let (c1, _) = fetch_string_char_at(string1, 0, b);
        let (c2, _) = fetch_string_char_at(string2, 0, b);
        if c1 < c2 {
            -1
        } else if c1 > c2 {
            1
        } else {
            0
        }
    } else if STRING_MULTIBYTE(string1) {
        let mut i1: ptrdiff_t = 0;
        let mut i1_byte: ptrdiff_t = 0;
        let mut i2: usize = 0;
        while i1 < n {
            let (c1, next_i1, next_i1_byte) = fetch_string_char_step(string1, i1, i1_byte);
            i1 = next_i1;
            i1_byte = next_i1_byte;
            let c2 = SREF(string2, i2) as c_int;
            i2 += 1;
            if c1 != c2 {
                return if c1 < c2 { -1 } else { 1 };
            }
        }
        if i1 < chars2 {
            -1
        } else if i1 < chars1 {
            1
        } else {
            0
        }
    } else {
        let mut i1: usize = 0;
        let mut i2: ptrdiff_t = 0;
        let mut i2_byte: ptrdiff_t = 0;
        while (i1 as ptrdiff_t) < n {
            let c1 = SREF(string1, i1) as c_int;
            i1 += 1;
            let (c2, next_i2, next_i2_byte) = fetch_string_char_step(string2, i2, i2_byte);
            i2 = next_i2;
            i2_byte = next_i2_byte;
            if c1 != c2 {
                return if c1 < c2 { -1 } else { 1 };
            }
        }
        let done = i1 as ptrdiff_t;
        if done < chars2 {
            -1
        } else if done < chars1 {
            1
        } else {
            0
        }
    }
}

pub fn string_lessp(mut string1: Lisp_Object, mut string2: Lisp_Object) -> Lisp_Object {
    if SYMBOLP(string1) {
        string1 = SYMBOL_NAME(string1);
    } else {
        CHECK_STRING(string1);
    }
    if SYMBOLP(string2) {
        string2 = SYMBOL_NAME(string2);
    } else {
        CHECK_STRING(string2);
    }
    if string_cmp_safe(string1, string2) < 0 {
        Qt()
    } else {
        Qnil()
    }
}

pub fn string_version_lessp(mut string1: Lisp_Object, mut string2: Lisp_Object) -> Lisp_Object {
    if SYMBOLP(string1) {
        string1 = SYMBOL_NAME(string1);
    }
    if SYMBOLP(string2) {
        string2 = SYMBOL_NAME(string2);
    }
    CHECK_STRING(string1);
    CHECK_STRING(string2);
    if crate::fns::filenvercmp_strings_safe(string1, string2) < 0 {
        Qt()
    } else {
        Qnil()
    }
}

pub fn string_collate_lessp(
    s1: Lisp_Object,
    s2: Lisp_Object,
    locale: Lisp_Object,
    ignore_case: Lisp_Object,
) -> Lisp_Object {
    #[cfg(feature = "stdc_iso_10646")]
    {
        let mut s1 = s1;
        let mut s2 = s2;
        if SYMBOLP(s1) {
            s1 = SYMBOL_NAME(s1);
        }
        if SYMBOLP(s2) {
            s2 = SYMBOL_NAME(s2);
        }
        CHECK_STRING(s1);
        CHECK_STRING(s2);
        if !NILP(locale) {
            CHECK_STRING(locale);
        }
        return if str_collate_safe(s1, s2, locale, ignore_case) < 0 {
            Qt()
        } else {
            Qnil()
        };
    }
    #[cfg(not(feature = "stdc_iso_10646"))]
    {
        let _ = (locale, ignore_case);
        string_lessp(s1, s2)
    }
}

pub fn string_collate_equalp(
    s1: Lisp_Object,
    s2: Lisp_Object,
    locale: Lisp_Object,
    ignore_case: Lisp_Object,
) -> Lisp_Object {
    #[cfg(feature = "stdc_iso_10646")]
    {
        let mut s1 = s1;
        let mut s2 = s2;
        if SYMBOLP(s1) {
            s1 = SYMBOL_NAME(s1);
        }
        if SYMBOLP(s2) {
            s2 = SYMBOL_NAME(s2);
        }
        CHECK_STRING(s1);
        CHECK_STRING(s2);
        if !NILP(locale) {
            CHECK_STRING(locale);
        }
        return if str_collate_safe(s1, s2, locale, ignore_case) == 0 {
            Qt()
        } else {
            Qnil()
        };
    }
    #[cfg(not(feature = "stdc_iso_10646"))]
    {
        let _ = (locale, ignore_case);
        string_equal(s1, s2)
    }
}

pub fn copy_sequence(arg: Lisp_Object) -> Lisp_Object {
    if NILP(arg) {
        return arg;
    }
    if CONSP(arg) {
        let val = Fcons(XCAR(arg), Qnil());
        let mut tail = XCDR(arg);
        if NILP(tail) {
            return val;
        }
        let mut prev = val;
        let mut fast_steps: c_int = 0;
        while CONSP(tail) && fast_steps < 64 {
            let c = Fcons(XCAR(tail), Qnil());
            XSETCDR(prev, c);
            prev = c;
            tail = XCDR(tail);
            fast_steps += 1;
        }
        if NILP(tail) {
            return val;
        }
        FOR_EACH_TAIL!(tail, {
            let c = Fcons(XCAR(tail), Qnil());
            XSETCDR(prev, c);
            prev = c;
        });
        CHECK_LIST_END(tail, tail);
        return val;
    }
    if STRINGP(arg) {
        return crate::fns::copy_sequence_string_safe(arg);
    }
    if VECTORP(arg) {
        return crate::fns::copy_sequence_vector_or_record_safe(arg, false);
    }
    if RECORDP(arg) {
        return crate::fns::copy_sequence_vector_or_record_safe(arg, true);
    }
    if CHAR_TABLE_P(arg) {
        return copy_char_table(arg);
    }
    if BOOL_VECTOR_P(arg) {
        return crate::fns::copy_sequence_bool_vector_safe(arg);
    }
    wrong_type_argument(Qsequencep(), arg);
}

pub fn copy_alist(mut alist: Lisp_Object) -> Lisp_Object {
    if NILP(alist) {
        return alist;
    }
    CHECK_CONS(alist);
    let first_car = XCAR(alist);
    let first_elt = if CONSP(first_car) {
        Fcons(XCAR(first_car), XCDR(first_car))
    } else {
        first_car
    };
    let val = Fcons(first_elt, Qnil());
    let mut prev = val;
    let mut tail = XCDR(alist);
    let mut fast_steps: c_int = 0;
    while CONSP(tail) && fast_steps < 64 {
        let car = XCAR(tail);
        let elt = if CONSP(car) {
            Fcons(XCAR(car), XCDR(car))
        } else {
            car
        };
        let c = Fcons(elt, Qnil());
        XSETCDR(prev, c);
        prev = c;
        tail = XCDR(tail);
        fast_steps += 1;
    }
    if NILP(tail) {
        return val;
    }
    alist = copy_sequence(tail);
    let mut tem = alist;
    while !NILP(tem) {
        let car = XCAR(tem);
        if CONSP(car) {
            XSETCAR(tem, Fcons(XCAR(car), XCDR(car)));
        }
        tem = XCDR(tem);
    }
    XSETCDR(prev, alist);
    val
}

pub fn substring_subr(string: Lisp_Object, from: Lisp_Object, to: Lisp_Object) -> Lisp_Object {
    let size = CHECK_VECTOR_OR_STRING(string);
    let (ifrom, ito) = validate_subarray_safe(string, from, to, size);
    if STRINGP(string) {
        let same_len = size == SBYTES(string);
        let from_byte = if same_len || ifrom == 0 {
            ifrom
        } else {
            string_char_to_byte(string, ifrom)
        };
        let to_byte = if same_len {
            ito
        } else if ito == size {
            SBYTES(string)
        } else {
            string_char_to_byte(string, ito)
        };
        crate::fns::substring_string_bytes_safe(string, ifrom, from_byte, ito, to_byte, true)
    } else {
        crate::fns::subvector_safe(string, ifrom, ito)
    }
}

pub fn substring_no_properties(
    string: Lisp_Object,
    from: Lisp_Object,
    to: Lisp_Object,
) -> Lisp_Object {
    CHECK_STRING(string);
    let size = SCHARS(string);
    let (from_char, to_char) = validate_subarray_safe(string, from, to, size);
    let same_len = size == SBYTES(string);
    let from_byte = if same_len || from_char == 0 {
        from_char
    } else {
        string_char_to_byte(string, from_char)
    };
    let to_byte = if same_len {
        to_char
    } else if to_char == size {
        SBYTES(string)
    } else {
        string_char_to_byte(string, to_char)
    };
    crate::fns::substring_string_bytes_safe(string, from_char, from_byte, to_char, to_byte, false)
}

pub fn substring_both_impl(
    string: Lisp_Object,
    from: ptrdiff_t,
    from_byte: ptrdiff_t,
    to: ptrdiff_t,
    to_byte: ptrdiff_t,
) -> Lisp_Object {
    let size = CHECK_VECTOR_OR_STRING(string);
    if !(0 <= from && from <= to && to <= size) {
        args_out_of_range_3(string, make_fixnum(from), make_fixnum(to));
    }
    if STRINGP(string) {
        crate::fns::substring_string_bytes_safe(string, from, from_byte, to, to_byte, true)
    } else {
        crate::fns::subvector_safe(string, from, to)
    }
}

pub fn take_subr(n: Lisp_Object, mut list: Lisp_Object) -> Lisp_Object {
    let mut m: EMACS_INT = if FIXNUMP(n) {
        let v = XFIXNUM(n);
        if v <= 0 {
            return Qnil();
        }
        v
    } else if BIGNUMP(n) {
        if crate::data::bignum_sign_safe(n) < 0 {
            return Qnil();
        }
        MOST_POSITIVE_FIXNUM
    } else {
        wrong_type_argument(Qintegerp(), n);
    };

    CHECK_LIST(list);
    if NILP(list) {
        return Qnil();
    }
    let ret = Fcons(XCAR(list), Qnil());
    let mut prev = ret;
    m -= 1;
    list = XCDR(list);
    while m > 0 && CONSP(list) {
        let p = Fcons(XCAR(list), Qnil());
        XSETCDR(prev, p);
        prev = p;
        m -= 1;
        list = XCDR(list);
    }
    if m > 0 && !NILP(list) {
        wrong_type_argument(Qlistp(), list);
    }
    ret
}

pub fn ntake_subr(n: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    let mut m: EMACS_INT = if FIXNUMP(n) {
        let v = XFIXNUM(n);
        if v <= 0 {
            return Qnil();
        }
        v
    } else if BIGNUMP(n) {
        if crate::data::bignum_sign_safe(n) < 0 {
            return Qnil();
        }
        MOST_POSITIVE_FIXNUM
    } else {
        wrong_type_argument(Qintegerp(), n);
    };

    CHECK_LIST(list);
    let mut tail = list;
    m -= 1;
    while m > 0 && CONSP(tail) {
        tail = XCDR(tail);
        m -= 1;
    }
    if CONSP(tail) {
        XSETCDR(tail, Qnil());
    } else if !NILP(tail) {
        wrong_type_argument(Qlistp(), list);
    }
    list
}

pub fn nth_subr(n: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    if FIXNUMP(n) {
        let mut num = XFIXNUM(n);
        if num <= SMALL_LIST_LEN_MAX as EMACS_INT {
            let mut tail = list;
            while 0 < num && CONSP(tail) {
                tail = XCDR(tail);
                num -= 1;
            }
            if CONSP(tail) {
                return XCAR(tail);
            } else if NILP(tail) {
                return Qnil();
            }
        }
    }
    car_subr(Fnthcdr(n, list))
}

#[inline(always)]
pub fn elt_subr(sequence: Lisp_Object, n: Lisp_Object) -> Lisp_Object {
    if CONSP(sequence) || NILP(sequence) {
        return car_subr(Fnthcdr(n, sequence));
    }
    CHECK_ARRAY(sequence, Qsequencep());
    aref_subr(sequence, n)
}

#[inline(always)]
fn eq_comparable_value(x: Lisp_Object) -> bool {
    SYMBOLP(x) || FIXNUMP(x)
}

#[inline(always)]
fn same_float_bits(x: Lisp_Object, y: Lisp_Object) -> bool {
    XFLOAT_DATA(x).to_bits() == XFLOAT_DATA(y).to_bits()
}

pub fn member_subr(elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    if eq_comparable_value(elt) {
        return memq_subr(elt, list);
    }
    let mut tail = list;
    FOR_EACH_TAIL!(tail, {
        if !NILP(Fequal(elt, XCAR(tail))) {
            return tail;
        }
    });
    CHECK_LIST_END(tail, list);
    Qnil()
}

#[cold]
#[inline(never)]
fn memq_slow(elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    let mut tail = list;
    FOR_EACH_TAIL!(tail, {
        if EQ(XCAR(tail), elt) {
            return tail;
        }
    });
    CHECK_LIST_END(tail, list);
    Qnil()
}

pub fn memq_subr(elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    if NILP(list) {
        return Qnil();
    }
    let mut tail = list;
    if !symbols_with_pos_enabled.get() {
        let mut steps: ptrdiff_t = 0;
        while CONSP(tail) && steps < 4096 {
            if BASE_EQ(XCAR(tail), elt) {
                return tail;
            }
            tail = XCDR(tail);
            steps += 1;
        }
        if NILP(tail) {
            return Qnil();
        }
    }
    memq_slow(elt, list)
}

#[inline(always)]
pub fn memq_no_quit_impl(elt: Lisp_Object, mut list: Lisp_Object) -> Lisp_Object {
    while CONSP(list) {
        if EQ(XCAR(list), elt) {
            return list;
        }
        list = XCDR(list);
    }
    Qnil()
}

pub fn memql_subr(elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    let mut tail = list;
    if FLOATP(elt) {
        FOR_EACH_TAIL!(tail, {
            let tem = XCAR(tail);
            if FLOATP(tem) && same_float_bits(elt, tem) {
                return tail;
            }
        });
    } else if BIGNUMP(elt) {
        FOR_EACH_TAIL!(tail, {
            let tem = XCAR(tail);
            if BIGNUMP(tem) && crate::data::bignum_equal_safe(elt, tem) {
                return tail;
            }
        });
    } else {
        return memq_subr(elt, list);
    }
    CHECK_LIST_END(tail, list);
    Qnil()
}

#[cold]
#[inline(never)]
fn assq_slow(key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object {
    let mut tail = alist;
    FOR_EACH_TAIL!(tail, {
        if CONSP(XCAR(tail)) && EQ(XCAR(XCAR(tail)), key) {
            return XCAR(tail);
        }
    });
    CHECK_LIST_END(tail, alist);
    Qnil()
}

pub fn assq_subr(key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object {
    if NILP(alist) {
        return Qnil();
    }
    let mut tail = alist;
    if !symbols_with_pos_enabled.get() {
        let mut steps: ptrdiff_t = 0;
        while CONSP(tail) && steps < 4096 {
            let car = XCAR(tail);
            if CONSP(car) && BASE_EQ(XCAR(car), key) {
                return car;
            }
            tail = XCDR(tail);
            steps += 1;
        }
        if NILP(tail) {
            return Qnil();
        }
    }
    assq_slow(key, alist)
}

#[inline(always)]
pub fn assq_no_quit_impl(key: Lisp_Object, mut alist: Lisp_Object) -> Lisp_Object {
    while !NILP(alist) {
        let car = XCAR(alist);
        if CONSP(car) && EQ(XCAR(car), key) {
            return car;
        }
        alist = XCDR(alist);
    }
    Qnil()
}

pub fn assq_no_signal_impl(key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object {
    let mut tail = alist;
    let mut found = Qnil();
    FOR_EACH_TAIL_SAFE!(tail, {
        let car = XCAR(tail);
        if CONSP(car) && EQ(XCAR(car), key) {
            found = car;
        }
    });
    found
}

pub fn assoc_subr(key: Lisp_Object, alist: Lisp_Object, testfn: Lisp_Object) -> Lisp_Object {
    if eq_comparable_value(key) && NILP(testfn) {
        return assq_subr(key, alist);
    }
    let mut tail = alist;
    FOR_EACH_TAIL!(tail, {
        let car = XCAR(tail);
        if CONSP(car) {
            let matched = if NILP(testfn) {
                EQ(XCAR(car), key) || !NILP(Fequal(XCAR(car), key))
            } else {
                !NILP(calln!(testfn, XCAR(car), key))
            };
            if matched {
                return car;
            }
        }
    });
    CHECK_LIST_END(tail, alist);
    Qnil()
}

pub fn rassq_subr(key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object {
    if NILP(alist) {
        return Qnil();
    }
    let mut tail = alist;
    if !symbols_with_pos_enabled.get() {
        let mut steps: ptrdiff_t = 0;
        while CONSP(tail) && steps < 4096 {
            let car = XCAR(tail);
            if CONSP(car) && BASE_EQ(XCDR(car), key) {
                return car;
            }
            tail = XCDR(tail);
            steps += 1;
        }
        if NILP(tail) {
            return Qnil();
        }
        tail = alist;
    }
    FOR_EACH_TAIL!(tail, {
        let car = XCAR(tail);
        if CONSP(car) && EQ(XCDR(car), key) {
            return car;
        }
    });
    CHECK_LIST_END(tail, alist);
    Qnil()
}

pub fn rassoc_subr(key: Lisp_Object, alist: Lisp_Object) -> Lisp_Object {
    if eq_comparable_value(key) {
        return rassq_subr(key, alist);
    }
    let mut tail = alist;
    FOR_EACH_TAIL!(tail, {
        let car = XCAR(tail);
        if CONSP(car) && (EQ(XCDR(car), key) || !NILP(Fequal(XCDR(car), key))) {
            return car;
        }
    });
    CHECK_LIST_END(tail, alist);
    Qnil()
}

pub fn delq_subr(elt: Lisp_Object, mut list: Lisp_Object) -> Lisp_Object {
    if NILP(list) {
        return Qnil();
    }
    let mut prev = Qnil();
    let mut tail = list;

    if !symbols_with_pos_enabled.get() {
        let mut steps: ptrdiff_t = 0;
        while CONSP(tail) && steps < 4096 {
            let next = XCDR(tail);
            if BASE_EQ(elt, XCAR(tail)) {
                if NILP(prev) {
                    list = next;
                } else {
                    XSETCDR(prev, next);
                }
            } else {
                prev = tail;
            }
            tail = next;
            steps += 1;
        }
        if NILP(tail) {
            return list;
        }
        prev = Qnil();
        tail = list;
    }

    FOR_EACH_TAIL!(tail, {
        let tem = XCAR(tail);
        if EQ(elt, tem) {
            if NILP(prev) {
                list = XCDR(tail);
            } else {
                setcdr(prev, XCDR(tail));
            }
        } else {
            prev = tail;
        }
    });
    CHECK_LIST_END(tail, list);
    list
}

pub fn delq_no_quit_impl(elt: Lisp_Object, mut list: Lisp_Object) -> Lisp_Object {
    let mut prev = Qnil();
    let mut tail = list;
    while !NILP(tail) {
        let tem = XCAR(tail);
        if EQ(elt, tem) {
            if NILP(prev) {
                list = XCDR(tail);
            } else {
                setcdr(prev, XCDR(tail));
            }
        } else {
            prev = tail;
        }
        tail = XCDR(tail);
    }
    list
}

pub fn delete_subr(elt: Lisp_Object, mut seq: Lisp_Object) -> Lisp_Object {
    if NILP(seq) {
        return seq;
    }
    if CONSP(seq) {
        let mut prev = Qnil();
        let mut tail = seq;
        FOR_EACH_TAIL!(tail, {
            if !NILP(Fequal(elt, XCAR(tail))) {
                if NILP(prev) {
                    seq = XCDR(tail);
                } else {
                    setcdr(prev, XCDR(tail));
                }
            } else {
                prev = tail;
            }
        });
        CHECK_LIST_END(tail, seq);
        return seq;
    }
    if VECTORP(seq) {
        let size = ASIZE(seq) as usize;
        let mut kept = RootedVec::with_capacity(size);
        let mut i: ptrdiff_t = 0;
        while (i as usize) < size {
            let item = AREF(seq, i);
            if NILP(Fequal(item, elt)) {
                kept.push(item);
            }
            i += 1;
        }
        if kept.len() != size {
            seq = Fvector_slice(&mut kept);
        }
        return seq;
    }
    if STRINGP(seq) {
        if !CHARACTERP(elt) {
            return seq;
        }
        return crate::fns::delete_string_char_safe(seq, XFIXNUM(elt) as c_int);
    }
    wrong_type_argument(Qsequencep(), seq);
}

pub fn nreverse_subr(mut seq: Lisp_Object) -> Lisp_Object {
    if NILP(seq) {
        return seq;
    }
    if CONSP(seq) {
        if NILP(XCDR(seq)) {
            return seq;
        }
        let mut prev = Qnil();
        let mut tail = seq;
        while CONSP(tail) {
            let next = XCDR(tail);
            if BASE_EQ(next, seq) {
                circular_list(seq);
            }
            XSETCDR(tail, prev);
            prev = tail;
            tail = next;
        }
        CHECK_LIST_END(tail, seq);
        seq = prev;
    } else if VECTORP(seq) {
        let size = ASIZE(seq);
        let mut i: ptrdiff_t = 0;
        while i < size / 2 {
            let tem = AREF(seq, i);
            ASET(seq, i, AREF(seq, size - i - 1));
            ASET(seq, size - i - 1, tem);
            i += 1;
        }
    } else if BOOL_VECTOR_P(seq) {
        let size = bool_vector_size(seq);
        let mut i: ptrdiff_t = 0;
        while i < size / 2 {
            let tem = bool_vector_bitref(seq, i);
            bool_vector_set(seq, i, bool_vector_bitref(seq, size - i - 1));
            bool_vector_set(seq, size - i - 1, tem);
            i += 1;
        }
    } else if STRINGP(seq) {
        return reverse_subr(seq);
    } else {
        wrong_type_argument(Qarrayp(), seq);
    }
    seq
}

pub fn reverse_subr(seq: Lisp_Object) -> Lisp_Object {
    let mut new = Qnil();
    if NILP(seq) {
        return Qnil();
    }
    if CONSP(seq) {
        let mut seq_loop = seq;
        let mut fast_steps: c_int = 0;
        while CONSP(seq_loop) && fast_steps < 64 {
            new = Fcons(XCAR(seq_loop), new);
            seq_loop = XCDR(seq_loop);
            fast_steps += 1;
        }
        if !NILP(seq_loop) {
            let mut seq_end = seq_loop;
            FOR_EACH_TAIL!(seq_loop, {
                new = Fcons(XCAR(seq_loop), new);
                seq_end = XCDR(seq_loop);
            });
            CHECK_LIST_END(seq_end, seq_end);
        }
    } else if VECTORP(seq) {
        let size = ASIZE(seq);
        new = make_uninit_vector(size);
        let mut i: ptrdiff_t = 0;
        while i < size {
            ASET(new, i, AREF(seq, size - i - 1));
            i += 1;
        }
    } else if BOOL_VECTOR_P(seq) {
        let nbits = bool_vector_size(seq);
        new = make_clear_bool_vector(nbits, true);
        let mut i: ptrdiff_t = 0;
        while i < nbits {
            if bool_vector_bitref(seq, nbits - i - 1) {
                bool_vector_set(new, i, true);
            }
            i += 1;
        }
    } else if STRINGP(seq) {
        let size = SCHARS(seq);
        let bytes = SBYTES(seq);
        if size == bytes {
            new = make_uninit_string(size);
            let mut i: ptrdiff_t = 0;
            while i < size {
                SSET(new, i as usize, SREF(seq, (size - i - 1) as usize));
                i += 1;
            }
        } else {
            new = crate::fns::reverse_multibyte_string_safe(seq);
        }
    } else {
        wrong_type_argument(Qsequencep(), seq);
    }
    new
}
