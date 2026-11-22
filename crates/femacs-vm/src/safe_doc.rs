//! `#![forbid(unsafe_code)]` implementation of documentation string loading,
//! snarfing, and quoting style selection (`src/doc.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

const SIBLING_ETC: &[u8] = b"../etc/";

pub fn get_doc_string_impl(filepos: Lisp_Object, unibyte: bool) -> Lisp_Object {
    let (mut file, dir, pos) = if FIXNUMP(filepos) {
        (Vdoc_file_name.get(), Vdoc_directory.get(), filepos)
    } else if CONSP(filepos) {
        (XCAR(filepos), Fsymbol_value(Qlisp_directory()), XCDR(filepos))
    } else {
        return Qnil();
    };

    if !FIXNUMP(pos) {
        return Qnil();
    }
    let position: EMACS_INT = XFIXNUM(pos).wrapping_abs();

    if !STRINGP(dir) || !STRINGP(file) {
        return Qnil();
    }

    let tem = Ffile_name_absolute_p(file);
    file = ENCODE_FILE(file);
    let docdir = if NILP(tem) {
        ENCODE_FILE(dir)
    } else {
        empty_unibyte_string.get()
    };

    let file_bytes = lisp_string_bytes_vec(file);
    let mut name_bytes = lisp_string_bytes_vec(docdir);
    name_bytes.extend_from_slice(&file_bytes);

    let count = SPECPDL_INDEX();
    let mut fd = emacs_open_ro_bytes(&name_bytes);
    if fd < 0 {
        if will_dump_p() {
            name_bytes.clear();
            name_bytes.extend_from_slice(SIBLING_ETC);
            name_bytes.extend_from_slice(&file_bytes);
            fd = emacs_open_ro_bytes(&name_bytes);
        }
        if fd < 0 {
            let err = current_errno();
            if err != ENOENT && err != ENOTDIR {
                report_file_error_cstr(c"Read error on documentation file", file);
            }
            return concat3(
                build_cstr(c"Cannot open doc string file \""),
                file,
                build_cstr(c"\"\n"),
            );
        }
    }

    record_close_file_unwind_safe(fd);

    let offset: usize = core::cmp::min(position, core::cmp::max(1024, position % (8 * 1024))) as usize;
    let seek_pos = position - offset as EMACS_INT;
    if off_t::MAX < position as off_t || lseek_set_offset(fd, seek_pos as off_t) < 0 {
        error_doc_position_out_of_range(position, &name_bytes);
    }

    let mut raw_buf: Vec<u8> = Vec::with_capacity(16 * 1024);
    let mut chunk = [0u8; 8 * 1024];
    loop {
        let start_idx = raw_buf.len();
        let nread = emacs_read_quit_slice(fd, &mut chunk);
        if nread < 0 {
            report_file_error_cstr(c"Read error on documentation file", file);
        }
        if nread == 0 {
            break;
        }
        raw_buf.extend_from_slice(&chunk[..(nread as usize)]);
        let search_from = if start_idx == 0 { offset } else { start_idx };
        if search_from < raw_buf.len() {
            if let Some(rel) = raw_buf[search_from..].iter().position(|&b| b == 31) {
                raw_buf.truncate(search_from + rel);
                break;
            }
        }
    }

    unbind_to(count, Qnil());

    if raw_buf.len() < offset {
        return Qnil();
    }

    // Sanity check the header bytes preceding `offset`.
    if CONSP(filepos) {
        if offset == 0 {
            return Qnil();
        }
        let mut test: usize = 1;
        if raw_buf[offset - test] != 31 {
            let idx1 = offset - test;
            test += 1;
            if raw_buf[idx1] != b' ' {
                return Qnil();
            }
            while test <= offset && raw_buf[offset - test].is_ascii_digit() {
                test += 1;
            }
            if test + 1 > offset {
                return Qnil();
            }
            let idx2 = offset - test;
            test += 1;
            if raw_buf[idx2] != b'@' || raw_buf[offset - test] != b'#' {
                return Qnil();
            }
        }
    } else {
        if offset < 2 {
            return Qnil();
        }
        let mut test: usize = 1;
        let idx1 = offset - test;
        test += 1;
        if raw_buf[idx1] != b'\n' {
            return Qnil();
        }
        while test <= offset && raw_buf[offset - test] > b' ' {
            test += 1;
        }
        if test > offset || raw_buf[offset - test] != 31 {
            return Qnil();
        }
    }

    // Unquote `^A` escapes (`^A^A -> ^A`, `^A0 -> NUL`, `^A_ -> ^_`).
    let payload = &raw_buf[offset..];
    let mut decoded = Vec::with_capacity(payload.len());
    let mut i = 0usize;
    while i < payload.len() {
        let b = payload[i];
        i += 1;
        if b == 1 {
            let c = if i < payload.len() {
                let next = payload[i];
                i += 1;
                next
            } else {
                0
            };
            match c {
                1 => decoded.push(1),
                b'0' => decoded.push(0),
                b'_' => decoded.push(31),
                uc => error_doc_invalid_escape(uc),
            }
        } else {
            decoded.push(b);
        }
    }

    if unibyte {
        make_unibyte_string_bytes(&decoded)
    } else {
        let nchars = multibyte_chars_in_slice(&decoded);
        make_string_from_bytes_safe(&decoded, nchars)
    }
}

fn reread_doc_file(file: Lisp_Object) {
    if NILP(file) {
        Fsnarf_documentation(Vdoc_file_name.get());
    } else {
        save_match_data_load(file, Qt(), Qt(), Qt(), Qnil());
    }
}

pub fn documentation_stringp_subr(object: Lisp_Object) -> Lisp_Object {
    if STRINGP(object)
        || FIXNUMP(object)
        || (CONSP(object) && STRINGP(XCAR(object)) && FIXNUMP(XCDR(object)))
    {
        Qt()
    } else {
        Qnil()
    }
}

pub fn documentation_subr(function: Lisp_Object, raw: Lisp_Object) -> Lisp_Object {
    let mut try_reload = documentation_dynamic_reload.get();

    loop {
        if SYMBOLP(function) {
            let tem = Fget(function, Qfunction_documentation());
            if !NILP(tem) {
                return Fdocumentation_property(function, Qfunction_documentation(), raw);
            }
        }

        let mut fun = Findirect_function(function, Qnil());
        if NILP(fun) {
            xsignal1(Qvoid_function(), function);
        }
        if CONSP(fun) && EQ(XCAR(fun), Qmacro()) {
            fun = XCDR(fun);
        }
        let mut doc = calln!(Qfunction_documentation(), fun);

        if BASE_EQ(doc, make_fixnum(0)) {
            doc = Qnil();
        }
        if FIXNUMP(doc) || (CONSP(doc) && FIXNUMP(XCDR(doc))) {
            let tem = get_doc_string_impl(doc, false);
            if NILP(tem) && try_reload {
                reread_doc_file(Fcar_safe(doc));
                try_reload = false;
                continue;
            }
            doc = tem;
        }

        if NILP(raw) {
            doc = calln!(Qsubstitute_command_keys(), doc);
        }
        return doc;
    }
}

pub fn subr_documentation_subr(function: Lisp_Object) -> Lisp_Object {
    if let Some(doc) = native_or_module_subr_documentation(function) {
        return doc;
    }
    if SUBRP(function) {
        make_fixnum(subr_doc_raw(function))
    } else {
        Qt()
    }
}

pub fn documentation_property_subr(
    symbol: Lisp_Object,
    prop: Lisp_Object,
    raw: Lisp_Object,
) -> Lisp_Object {
    let mut try_reload = documentation_dynamic_reload.get();

    loop {
        let mut tem = Fget(symbol, prop);

        if EQ(prop, Qvariable_documentation()) && NILP(tem) {
            let indirect = Findirect_variable(symbol);
            if !NILP(indirect) {
                tem = Fget(indirect, prop);
            }
        }

        if BASE_EQ(tem, make_fixnum(0)) {
            tem = Qnil();
        }

        if FIXNUMP(tem) || (CONSP(tem) && FIXNUMP(XCDR(tem))) {
            let doc = tem;
            tem = get_doc_string_impl(tem, false);
            if NILP(tem) && try_reload {
                reread_doc_file(Fcar_safe(doc));
                try_reload = false;
                continue;
            }
        } else if !STRINGP(tem) {
            tem = Feval(tem, Qnil());
        }

        if NILP(raw) && STRINGP(tem) {
            tem = calln!(Qsubstitute_command_keys(), tem);
        }
        return tem;
    }
}

fn store_function_docstring(obj: Lisp_Object, offset: EMACS_INT) {
    let mut fun = if SYMBOLP(obj) {
        symbol_function_slot(obj)
    } else {
        obj
    };

    if CONSP(fun) && EQ(XCAR(fun), Qmacro()) {
        fun = XCDR(fun);
    }

    if SUBRP(fun) {
        set_subr_doc_slot(fun, offset);
    } else if CLOSUREP(fun) {
        if PVSIZE(fun) > CLOSURE_DOC_STRING as isize
            && VALID_DOCSTRING_P(AREF(fun, CLOSURE_DOC_STRING as isize))
        {
            set_closure_doc_slot_safe(fun, make_fixnum(offset));
        } else {
            let fmt = build_cstr(c"No doc string slot for compiled: %S");
            CALLN!(Fmessage, fmt, obj);
        }
    } else {
        let fmt = build_cstr(c"Ignoring DOC string on non-compiled non-subr: %S");
        CALLN!(Fmessage, fmt, obj);
    }
}

pub fn snarf_documentation_subr(filename: Lisp_Object) -> Lisp_Object {
    let mut delayed_init = find_symbol_value(Qcustom_delayed_init_variables());
    if !CONSP(delayed_init) {
        delayed_init = Qnil();
    }

    CHECK_STRING(filename);

    let mut name_bytes = if will_dump_p() {
        SIBLING_ETC.to_vec()
    } else {
        let doc_dir = Vdoc_directory.get();
        CHECK_STRING(doc_dir);
        lisp_string_bytes_vec(doc_dir)
    };
    name_bytes.extend_from_slice(&lisp_string_bytes_vec(filename));

    let count = SPECPDL_INDEX();

    if NILP(Vbuild_files.get()) {
        let items = &crate::buildobj::buildobj;
        let mut list = Qnil();
        for item in items.iter().rev() {
            list = Fcons(build_cstr(item), list);
        }
        Vbuild_files.set(list);
    }

    let fd = emacs_open_ro_bytes(&name_bytes);
    if fd < 0 {
        let open_errno = current_errno();
        report_file_errno_cstr(
            c"Opening doc string file",
            make_string_bytes(&name_bytes),
            open_errno,
        );
    }
    record_close_file_unwind_safe(fd);
    Vdoc_file_name.set(filename);

    let mut buf = [0u8; 1024 + 1];
    let mut filled: usize = 0;
    let mut pos: EMACS_INT = 0;

    loop {
        if filled < 512 {
            let max_read = 1024 - filled;
            let nread = emacs_read_quit_slice(fd, &mut buf[filled..(filled + max_read)]);
            if nread > 0 {
                filled += nread as usize;
            }
        }
        if filled == 0 {
            break;
        }

        let scan_limit = if filled < 512 { filled } else { filled - 128 };
        let mut end_idx = scan_limit;
        if let Some(p_idx) = buf[..scan_limit].iter().position(|&b| b == 31) {
            let nl_rel = buf[p_idx..filled]
                .iter()
                .position(|&b| b == b'\n')
                .unwrap_or_else(|| error_doc_invalid_pos(pos));
            end_idx = p_idx + nl_rel;

            if end_idx >= p_idx + 2 {
                let sym_bytes = &buf[(p_idx + 2)..end_idx];
                let sym = oblookup_bytes(Vobarray.get(), sym_bytes);
                if SYMBOLP(sym) {
                    let kind = buf[p_idx + 1];
                    let is_skip = filled >= end_idx + 5 && &buf[end_idx..(end_idx + 5)] == b"\nSKIP";
                    match kind {
                        b'V' => {
                            if (!NILP(Fboundp(sym)) || !NILP(Fmemq(sym, delayed_init))) && !is_skip
                            {
                                Fput(
                                    sym,
                                    Qvariable_documentation(),
                                    make_fixnum(pos + end_idx as EMACS_INT + 1),
                                );
                            }
                        }
                        b'F' => {
                            if !NILP(Ffboundp(sym)) && !is_skip {
                                store_function_docstring(sym, pos + end_idx as EMACS_INT + 1);
                            }
                        }
                        b'S' => {}
                        _ => error_doc_invalid_pos(pos),
                    }
                }
            }
        }

        pos += end_idx as EMACS_INT;
        buf.copy_within(end_idx..filled, 0);
        filled -= end_idx;
    }

    unbind_to(count, Qnil())
}

pub fn default_to_grave_quoting_style() -> bool {
    if !text_quoting_flag.get() {
        return true;
    }
    let std_disp = Vstandard_display_table.get();
    if !DISP_TABLE_P(std_disp) {
        return false;
    }
    let dv = DISP_CHAR_VECTOR(XCHAR_TABLE(std_disp), LEFT_SINGLE_QUOTATION_MARK);
    VECTORP(dv) && ASIZE(dv) == 1 && BASE_EQ(AREF(dv, 0), make_fixnum(b'`' as EMACS_INT))
}

pub fn text_quoting_style_subr() -> Lisp_Object {
    let style = Vtext_quoting_style.get();
    if if NILP(style) {
        default_to_grave_quoting_style()
    } else {
        EQ(style, Qgrave())
    } {
        Qgrave()
    } else if EQ(style, Qstraight()) {
        Qstraight()
    } else {
        Qcurve()
    }
}
