//! `#![forbid(unsafe_code)]` implementation of character, string, region, and word
//! case-conversion operations (`src/casefiddle.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

pub type CaseAction = c_int;
pub const CASE_UP: CaseAction = 0;
pub const CASE_DOWN: CaseAction = 1;
pub const CASE_CAPITALIZE: CaseAction = 2;
pub const CASE_CAPITALIZE_UP: CaseAction = 3;

const CASING_STR_BUF_DATA_LEN: usize = if 6 > MAX_MULTIBYTE_LENGTH {
    6
} else {
    MAX_MULTIBYTE_LENGTH as usize
};

const GREEK_CAPITAL_LETTER_SIGMA: c_int = 0x03A3;
const GREEK_SMALL_LETTER_FINAL_SIGMA: c_int = 0x03C2;

#[derive(Clone, Copy)]
struct CasingContext {
    titlecase_char_table: Lisp_Object,
    specialcase_char_tables: [Lisp_Object; 3],
    flag: CaseAction,
    inbuffer: bool,
    inword: bool,
    downcase_last: bool,
}

#[derive(Clone, Copy)]
struct CasingStrBuf {
    data: [u8; CASING_STR_BUF_DATA_LEN],
    len_chars: u8,
    len_bytes: u8,
}

impl Default for CasingStrBuf {
    fn default() -> Self {
        Self {
            data: [0; CASING_STR_BUF_DATA_LEN],
            len_chars: 0,
            len_bytes: 0,
        }
    }
}

fn prepare_casing_context(flag: CaseAction, inbuffer: bool) -> CasingContext {
    let ctx = CasingContext {
        titlecase_char_table: if flag < CASE_CAPITALIZE {
            Qnil()
        } else {
            uniprop_table(Qtitlecase())
        },
        specialcase_char_tables: [
            if flag == CASE_DOWN {
                Qnil()
            } else {
                uniprop_table(Qspecial_uppercase())
            },
            if flag == CASE_UP {
                Qnil()
            } else {
                uniprop_table(Qspecial_lowercase())
            },
            if flag < CASE_CAPITALIZE {
                Qnil()
            } else {
                uniprop_table(Qspecial_titlecase())
            },
        ],
        flag,
        inbuffer,
        inword: false,
        downcase_last: false,
    };

    let down_tbl = current_buffer_downcase_table();
    if NILP(char_table_extra_slot_ref(down_tbl, 1)) {
        Fset_case_table(down_tbl);
    }

    if inbuffer && flag >= CASE_CAPITALIZE {
        SETUP_BUFFER_SYNTAX_TABLE();
    }

    ctx
}

#[inline(always)]
fn case_ch_is_word(syntax: syntaxcode) -> bool {
    syntax == Sword || (case_symbols_as_words.get() && syntax == Ssymbol)
}

fn case_character_impl(
    mut buf: Option<&mut CasingStrBuf>,
    ctx: &mut CasingContext,
    ch: c_int,
) -> c_int {
    let was_inword = ctx.inword;
    ctx.inword =
        case_ch_is_word(SYNTAX(ch)) && (!ctx.inbuffer || was_inword || !syntax_prefix_flag_p(ch));

    let cased: c_int = 'done: {
        let flag = if ctx.flag == CASE_CAPITALIZE {
            ctx.flag - was_inword as CaseAction
        } else if ctx.flag != CASE_CAPITALIZE_UP {
            ctx.flag
        } else if !was_inword {
            CASE_CAPITALIZE
        } else {
            break 'done ch;
        };

        if let Some(ref mut out_buf) = buf {
            let spec_tbl = ctx.specialcase_char_tables[flag as usize];
            if !NILP(spec_tbl) {
                let prop = CHAR_TABLE_REF(spec_tbl, ch as isize);
                if STRINGP(prop) {
                    let sbytes = SBYTES(prop) as usize;
                    if sbytes <= CASING_STR_BUF_DATA_LEN {
                        out_buf.len_chars = SCHARS(prop) as u8;
                        out_buf.len_bytes = sbytes as u8;
                        let bytes = lisp_string_bytes_vec(prop);
                        out_buf.data[..sbytes].copy_from_slice(&bytes);
                        return 1;
                    }
                }
            }
        }

        if flag == CASE_DOWN {
            ctx.downcase_last = true;
            downcase(ch)
        } else {
            ctx.downcase_last = false;
            let mut result = None;
            if !NILP(ctx.titlecase_char_table) {
                let prop = CHAR_TABLE_REF(ctx.titlecase_char_table, ch as isize);
                if CHARACTERP(prop) {
                    result = Some(XFIXNAT(prop) as c_int);
                }
            }
            result.unwrap_or_else(|| upcase(ch))
        }
    };

    match buf {
        None => cased,
        Some(out_buf) => {
            out_buf.len_chars = 1;
            let mut enc = [0u8; MAX_MULTIBYTE_LENGTH as usize];
            let nbytes = encode_char_bytes(cased, &mut enc);
            out_buf.data[..nbytes].copy_from_slice(&enc[..nbytes]);
            out_buf.len_bytes = nbytes as u8;
            (cased != ch) as c_int
        }
    }
}

#[inline(always)]
fn case_single_character(ctx: &mut CasingContext, ch: c_int) -> c_int {
    case_character_impl(None, ctx, ch)
}

fn case_character(
    buf: &mut CasingStrBuf,
    ctx: &mut CasingContext,
    ch: c_int,
    next_ch: Option<c_int>,
) -> bool {
    let was_inword = ctx.inword;
    let changed = case_character_impl(Some(buf), ctx, ch) != 0;

    if was_inword
        && ch == GREEK_CAPITAL_LETTER_SIGMA
        && changed
        && next_ch.is_none_or(|nch| !case_ch_is_word(SYNTAX(nch)))
    {
        let mut enc = [0u8; MAX_MULTIBYTE_LENGTH as usize];
        let nbytes = encode_char_bytes(GREEK_SMALL_LETTER_FINAL_SIGMA, &mut enc);
        buf.data[..nbytes].copy_from_slice(&enc[..nbytes]);
        buf.len_bytes = nbytes as u8;
        buf.len_chars = 1;
    }

    changed
}

#[inline(always)]
fn make_char_unibyte(c: c_int) -> c_int {
    if ASCII_CHAR_P(c as isize) {
        c
    } else {
        CHAR_TO_BYTE8(c)
    }
}

fn do_casify_natnum(ctx: &mut CasingContext, obj: Lisp_Object) -> Lisp_Object {
    let flagbits: c_int = CHAR_ALT | CHAR_SUPER | CHAR_HYPER | CHAR_SHIFT | CHAR_CTL | CHAR_META;
    let mut ch = XFIXNAT(obj) as c_int;

    if !(0 <= ch && ch <= flagbits + MAX_CHAR) {
        return obj;
    }

    let flags = ch & flagbits;
    ch &= !flagbits;

    let multibyte = ch >= 256 || current_buffer_multibyte_p();
    if !multibyte {
        ch = make_char_multibyte(ch);
    }
    let cased = case_single_character(ctx, ch);
    if cased == ch {
        return obj;
    }

    let final_cased = if !multibyte {
        make_char_unibyte(cased)
    } else {
        cased
    };
    make_fixed_natnum(final_cased as EMACS_INT | flags as EMACS_INT)
}

fn do_casify_multibyte_string(ctx: &mut CasingContext, obj: Lisp_Object) -> Lisp_Object {
    let mut size = SCHARS(obj);
    let (n_prod, mul_overflowed) = size.overflowing_mul(MAX_MULTIBYTE_LENGTH as ptrdiff_t);
    let (n_max, add_overflowed) =
        n_prod.overflowing_add(core::mem::size_of::<CasingStrBuf>() as ptrdiff_t);
    if mul_overflowed || add_overflowed || n_max >= PTRDIFF_MAX {
        string_overflow();
    }

    let src_bytes = lisp_string_bytes_vec(obj);
    let mut out = Vec::with_capacity(src_bytes.len());
    let mut byte_idx: usize = 0;
    let mut nchars: ptrdiff_t = 0;
    let max_bytes = n_max as usize;

    while size != 0 {
        if max_bytes.saturating_sub(out.len()) < core::mem::size_of::<CasingStrBuf>() {
            string_overflow();
        }
        let (ch, ch_len) = string_char_and_length_slice(&src_bytes[byte_idx..]);
        byte_idx += ch_len;
        let next_ch = if size > 1 && byte_idx < src_bytes.len() {
            Some(string_char_and_length_slice(&src_bytes[byte_idx..]).0)
        } else {
            None
        };
        let mut buf = CasingStrBuf::default();
        case_character(&mut buf, ctx, ch, next_ch);
        nchars += buf.len_chars as ptrdiff_t;
        out.extend_from_slice(&buf.data[..buf.len_bytes as usize]);
        size -= 1;
    }

    make_multibyte_string_bytes(&out, nchars)
}

fn ascii_casify_character(downcase_flag: bool, c: c_int) -> c_int {
    let tbl = if downcase_flag {
        uniprop_table(Qlowercase())
    } else {
        uniprop_table(Quppercase())
    };
    let cased = CHAR_TABLE_REF(tbl, c as isize);
    if FIXNATP(cased) {
        XFIXNAT(cased) as c_int
    } else {
        c
    }
}

fn do_casify_unibyte_string(ctx: &mut CasingContext, obj: Lisp_Object) -> Lisp_Object {
    let size = SCHARS(obj) as usize;
    let copied = Fcopy_sequence(obj);
    for i in 0..size {
        let ch = make_char_multibyte(SREF(copied, i) as c_int);
        let mut cased = case_single_character(ctx, ch);
        if ch == cased {
            continue;
        }
        if ASCII_CHAR_P(ch as isize) && !SINGLE_BYTE_CHAR_P(cased as intmax_t) {
            cased = ascii_casify_character(ctx.downcase_last, ch);
        }
        SSET(copied, i, make_char_unibyte(cased) as u8);
    }
    copied
}

pub fn casify_object(flag: CaseAction, obj: Lisp_Object) -> Lisp_Object {
    let mut ctx = prepare_casing_context(flag, false);

    if FIXNATP(obj) {
        do_casify_natnum(&mut ctx, obj)
    } else if !STRINGP(obj) {
        wrong_type_argument(Qchar_or_string_p(), obj)
    } else if SCHARS(obj) == 0 {
        obj
    } else if STRING_MULTIBYTE(obj) {
        do_casify_multibyte_string(&mut ctx, obj)
    } else {
        do_casify_unibyte_string(&mut ctx, obj)
    }
}

fn do_casify_unibyte_region(
    ctx: &mut CasingContext,
    startp: &mut ptrdiff_t,
    endp: &mut ptrdiff_t,
) -> ptrdiff_t {
    let mut first: ptrdiff_t = -1;
    let mut last: ptrdiff_t = -1;
    let end = *endp;
    let mut pos = *startp;

    while pos < end {
        let ch = make_char_multibyte(current_buffer_fetch_byte(pos) as c_int);
        let cased = case_single_character(ctx, ch);
        if cased == ch {
            pos += 1;
            continue;
        }
        last = pos + 1;
        if first < 0 {
            first = pos;
        }
        current_buffer_set_byte_at(pos, make_char_unibyte(cased) as u8);
        pos += 1;
    }

    *startp = first;
    *endp = last;
    0
}

fn do_casify_multibyte_region(
    ctx: &mut CasingContext,
    startp: &mut ptrdiff_t,
    endp: &mut ptrdiff_t,
) -> ptrdiff_t {
    let mut first: ptrdiff_t = -1;
    let mut last: ptrdiff_t = -1;
    let mut pos = *startp;
    let mut pos_byte = CHAR_TO_BYTE(pos);
    let mut size = *endp - pos;
    let mut opoint = current_buffer_pt();
    let mut added: ptrdiff_t = 0;

    while size != 0 {
        let (ch, len, next_ch) = current_buffer_char_len_and_next_at(pos_byte, size > 1);
        let mut buf = CasingStrBuf::default();
        if !case_character(&mut buf, ctx, ch, next_ch) {
            pos_byte += len as ptrdiff_t;
            pos += 1;
            size -= 1;
            continue;
        }

        last = pos + buf.len_chars as ptrdiff_t;
        if first < 0 {
            first = pos;
        }

        if buf.len_chars == 1 && buf.len_bytes as usize == len {
            current_buffer_write_same_len_bytes_at(pos_byte, &buf.data[..len]);
        } else {
            replace_range_2_bytes(
                pos,
                pos_byte,
                pos + 1,
                pos_byte + len as ptrdiff_t,
                &buf.data[..buf.len_bytes as usize],
                buf.len_chars as ptrdiff_t,
                false,
            );
            added += buf.len_chars as ptrdiff_t - 1;
            if opoint > pos {
                opoint += buf.len_chars as ptrdiff_t - 1;
            }
        }

        pos_byte += buf.len_bytes as ptrdiff_t;
        pos += buf.len_chars as ptrdiff_t;
        size -= 1;
    }

    if current_buffer_pt() != opoint {
        TEMP_SET_PT_BOTH(opoint, CHAR_TO_BYTE(opoint));
    }

    *startp = first;
    *endp = last;
    added
}

fn casify_region(flag: CaseAction, b: Lisp_Object, e: Lisp_Object) -> ptrdiff_t {
    let (b, e) = validate_region_safe(b, e);
    let mut start = XFIXNAT(b);
    let mut end = XFIXNAT(e);
    if start == end {
        return end;
    }
    modify_text(start, end);
    let mut ctx = prepare_casing_context(flag, true);

    const _: () = core::assert!(!cfg!(feature = "have_tree_sitter") || HAVE_TREE_SITTER);

    #[cfg(feature = "have_tree_sitter")]
    let start_byte = CHAR_TO_BYTE(start);
    #[cfg(feature = "have_tree_sitter")]
    let old_end_byte = CHAR_TO_BYTE(end);
    #[cfg(feature = "have_tree_sitter")]
    let start_linecol =
        treesit_linecol_maybe(start, start_byte, current_buffer_ts_linecol_point());
    #[cfg(feature = "have_tree_sitter")]
    let old_end_linecol =
        treesit_linecol_maybe(end, old_end_byte, current_buffer_ts_linecol_point());

    let orig_end = end;
    record_delete(start, make_buffer_string(start, end, true), false);
    let added = if !current_buffer_multibyte_p() {
        record_insert(start, end - start);
        do_casify_unibyte_region(&mut ctx, &mut start, &mut end)
    } else {
        let len = end - start;
        let ostart = start;
        let delta = do_casify_multibyte_region(&mut ctx, &mut start, &mut end);
        record_insert(ostart, len + delta);
        delta
    };

    if start >= 0 {
        signal_after_change(start, end - start - added, end - start);
        update_compositions(start, end, CHECK_ALL);
    }

    #[cfg(feature = "have_tree_sitter")]
    {
        let new_end = orig_end + added;
        let new_end_byte = CHAR_TO_BYTE(new_end);
        treesit_record_change(
            start_byte,
            old_end_byte,
            new_end_byte,
            start_linecol,
            old_end_linecol,
            new_end,
        );
    }

    orig_end + added
}

pub fn casify_pnc_region(
    flag: CaseAction,
    beg: Lisp_Object,
    end: Lisp_Object,
    region_noncontiguous_p: Lisp_Object,
) -> Lisp_Object {
    if !NILP(region_noncontiguous_p) {
        let mut bounds = calln!(Vregion_extract_function.get(), Qbounds());
        FOR_EACH_TAIL!(bounds, {
            CHECK_CONS(XCAR(bounds));
            casify_region(flag, XCAR(XCAR(bounds)), XCDR(XCAR(bounds)));
        });
        CHECK_LIST_END(bounds, bounds);
    } else {
        casify_region(flag, beg, end);
    }
    Qnil()
}

pub fn casify_word(flag: CaseAction, arg: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(arg);
    let pt = current_buffer_pt();
    let count = XFIXNUM(arg);
    let mut farend = scan_words(pt, count);
    if farend == 0 {
        farend = if count <= 0 {
            current_buffer_begv()
        } else {
            current_buffer_zv()
        };
    }
    SET_PT(casify_region(flag, make_fixnum(pt), make_fixnum(farend)));
    Qnil()
}
