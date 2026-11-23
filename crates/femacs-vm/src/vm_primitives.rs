//! Narrow safe-interface primitives bridging `LispVal`/`Handle`/`GcContext` to `femacs-vm` (`U4`/`U6`).
//!
//! Raw C-string/slice boundaries and `specpdl` pointer indexing are isolated here behind
//! `GcContext` + `Handle` contracts with explicit `// SAFETY:` proofs, while fixed-arity `DEFUN!`
//! subr calls execute directly as safe Rust (`P-485`).

#![deny(unsafe_op_in_unsafe_fn, static_mut_refs)]

use core::slice;

use crate::common::{
    BUFFERP, CONSP, EMACS_INT, Lisp_Object, Qt, SBYTES, SDATA, STRINGP, WINDOWP, XCAR, XCDR,
    XFIXNUM, build_utf8_str, current_thread, lisp_h_Qnil, make_fixnum,
    record_unwind_current_buffer, unbind_to_safe, wrap_specpdl_ref,
};
use crate::nonlocal::{VmResult, catch_vm_nonlocal};
use crate::roots::{GcContext, Handle, LispVal, UnrootedBorrow};

#[inline]
fn current_specpdl_index() -> isize {
    // SAFETY: `current_thread` is non-null when `GcContext` exists; computes byte offset of
    // `m_specpdl_ptr` from `m_specpdl` exactly matching `SPECPDL_INDEX()`.
    unsafe {
        let ct = current_thread.get();
        (*ct).m_specpdl_ptr as isize - (*ct).m_specpdl as isize
    }
}

/// Allocate a multibyte Lisp string from a Rust UTF-8 `&str` across a GC safepoint.
pub fn make_utf8_string(ctx: &mut GcContext, text: &str) -> VmResult<LispVal> {
    let _ = ctx;
    catch_vm_nonlocal(|| LispVal::from_raw(build_utf8_str(text)))
}

/// Copy a Lisp string into an owned Rust `String` using a short `NoGcToken` borrow that ends
/// before any subsequent Lisp/GC safepoint.
pub fn lisp_string_to_owned(ctx: &GcContext, val: Handle<'_>) -> Option<String> {
    let nogc = ctx.nogc();
    let borrowed = UnrootedBorrow::new(&nogc, val.get());
    let raw = borrowed.get().into_raw();
    if !STRINGP(raw) {
        return None;
    }
    let len = SBYTES(raw);
    if len < 0 {
        return None;
    }
    if len == 0 {
        return Some(String::new());
    }
    let ptr = SDATA(raw) as *const u8;
    // SAFETY: `NoGcToken` is live and `STRINGP(raw)` with `len > 0` verified above, so `ptr`
    // is valid for `len` bytes until copied into an owned Rust `String`.
    let bytes = unsafe { slice::from_raw_parts(ptr, len as usize) };
    Some(String::from_utf8_lossy(bytes).into_owned())
}

pub fn get_buffer_create(ctx: &mut GcContext, name: Handle<'_>) -> VmResult<LispVal> {
    let _ = ctx;
    let raw_name = name.get().into_raw();
    catch_vm_nonlocal(|| LispVal::from_raw(crate::buffer::Fget_buffer_create(raw_name, lisp_h_Qnil())))
}

pub fn buffer_name_val(ctx: &mut GcContext, buffer: Handle<'_>) -> VmResult<LispVal> {
    let _ = ctx;
    let raw_buf = buffer.get().into_raw();
    catch_vm_nonlocal(|| LispVal::from_raw(crate::buffer::Fbuffer_name(raw_buf)))
}

pub fn kill_buffer_val(ctx: &mut GcContext, buffer: Handle<'_>) -> VmResult<bool> {
    let _ = ctx;
    let raw_buf = buffer.get().into_raw();
    catch_vm_nonlocal(|| crate::buffer::Fkill_buffer(raw_buf) != lisp_h_Qnil())
}

/// Run `body` with `buffer` temporarily selected as `current_buffer`, guaranteed to restore the
/// previous current buffer via `specpdl` (`record_unwind_current_buffer` + `unbind_to_safe`) on both
/// normal return and `NonLocalExit`.
pub fn with_current_buffer<R>(
    ctx: &mut GcContext,
    buffer: Handle<'_>,
    body: impl FnOnce(&mut GcContext) -> VmResult<R>,
) -> VmResult<R> {
    let raw_buf = buffer.get().into_raw();
    if !BUFFERP(raw_buf) {
        return catch_vm_nonlocal(|| {
            crate::buffer::Fset_buffer(raw_buf);
            unreachable!()
        });
    }
    let count_raw = current_specpdl_index();
    catch_vm_nonlocal(|| {
        {
            record_unwind_current_buffer();
        }
        crate::buffer::Fset_buffer(raw_buf);
    })?;

    let result = body(ctx);
    let unbind_res = catch_vm_nonlocal(|| {
        unbind_to_safe(wrap_specpdl_ref(count_raw), lisp_h_Qnil());
    });
    match (result, unbind_res) {
        (_, Err(cleanup_exit)) => Err(cleanup_exit),
        (Ok(v), Ok(())) => Ok(v),
        (Err(e), Ok(())) => Err(e),
    }
}

pub fn erase_current_buffer(ctx: &mut GcContext) -> VmResult<()> {
    let _ = ctx;
    catch_vm_nonlocal(|| {
        crate::buffer::Ferase_buffer();
    })
}

pub fn insert_lisp_string(ctx: &mut GcContext, text: Handle<'_>) -> VmResult<()> {
    let _ = ctx;
    let mut arg = text.get().into_raw();
    catch_vm_nonlocal(|| {
        // SAFETY: `Finsert` (`MANY`) reads 1 `Lisp_Object` from `&raw mut arg` on the active Lisp thread.
        unsafe {
            crate::editfns::Finsert(1, &raw mut arg);
        }
    })
}

pub fn current_buffer_string_no_props(ctx: &mut GcContext) -> VmResult<LispVal> {
    let _ = ctx;
    catch_vm_nonlocal(|| {
        let beg = crate::editfns::Fpoint_min();
        let end = crate::editfns::Fpoint_max();
        LispVal::from_raw(crate::editfns::Fbuffer_substring_no_properties(beg, end))
    })
}

pub fn current_buffer_char_and_byte_len(ctx: &GcContext) -> (usize, usize) {
    let _nogc = ctx.nogc();
    let chars_obj = crate::editfns::Fbuffer_size(lisp_h_Qnil());
    let max_pt = crate::editfns::Fpoint_max();
    let min_pt = crate::editfns::Fpoint_min();
    let end_bytes = crate::editfns::Fposition_bytes(max_pt);
    let beg_bytes = crate::editfns::Fposition_bytes(min_pt);
    let chars = XFIXNUM(chars_obj).max(0) as usize;
    let bytes = (XFIXNUM(end_bytes) - XFIXNUM(beg_bytes)).max(0) as usize;
    (chars, bytes)
}

/// Parse one Lisp form starting at byte/char index `start_char` from `source`, returning
/// `(parsed_form, next_char_index)`.
pub fn read_from_lisp_string(
    ctx: &mut GcContext,
    source: Handle<'_>,
    start_char: usize,
) -> VmResult<(LispVal, usize)> {
    let _ = ctx;
    let raw_src = source.get().into_raw();
    let start_fix = LispVal::from_fixnum(start_char as isize).into_raw();
    catch_vm_nonlocal(|| {
        let pair = crate::lread::Fread_from_string(raw_src, start_fix, lisp_h_Qnil());
        if !CONSP(pair) {
            return (LispVal::nil(), start_char);
        }
        let obj = LispVal::from_raw(XCAR(pair));
        let next_idx = LispVal::from_raw(XCDR(pair)).as_fixnum().unwrap_or(start_char as isize);
        (obj, next_idx.max(0) as usize)
    })
}

pub fn eval_lexical(ctx: &mut GcContext, form: Handle<'_>) -> VmResult<LispVal> {
    let _ = ctx;
    let raw_form = form.get().into_raw();
    catch_vm_nonlocal(|| LispVal::from_raw(crate::eval::Feval(raw_form, Qt())))
}

pub fn prin1_to_lisp_string(ctx: &mut GcContext, value: Handle<'_>) -> VmResult<LispVal> {
    let _ = ctx;
    let raw_val = value.get().into_raw();
    catch_vm_nonlocal(|| {
        LispVal::from_raw(crate::print::Fprin1_to_string(raw_val, lisp_h_Qnil(), lisp_h_Qnil()))
    })
}

pub fn expand_file_name_val(ctx: &mut GcContext, path: Handle<'_>) -> VmResult<LispVal> {
    let _ = ctx;
    let raw_path = path.get().into_raw();
    catch_vm_nonlocal(|| LispVal::from_raw(crate::fileio::Fexpand_file_name(raw_path, lisp_h_Qnil())))
}

pub fn write_current_buffer_to_file(ctx: &mut GcContext, filename: Handle<'_>) -> VmResult<()> {
    let _ = ctx;
    let raw_file = filename.get().into_raw();
    catch_vm_nonlocal(|| {
        let beg = crate::editfns::Fpoint_min();
        let end = crate::editfns::Fpoint_max();
        let quiet = make_fixnum(0 as EMACS_INT);
        crate::fileio::Fwrite_region(
            beg,
            end,
            raw_file,
            lisp_h_Qnil(),
            quiet,
            lisp_h_Qnil(),
            lisp_h_Qnil(),
        );
    })
}

pub fn insert_file_into_current_buffer(ctx: &mut GcContext, filename: Handle<'_>) -> VmResult<usize> {
    let _ = ctx;
    let raw_file = filename.get().into_raw();
    catch_vm_nonlocal(|| {
        let ret = crate::fileio::Finsert_file_contents(
            raw_file,
            lisp_h_Qnil(),
            lisp_h_Qnil(),
            lisp_h_Qnil(),
            lisp_h_Qnil(),
        );
        if CONSP(ret) && CONSP(XCDR(ret)) {
            LispVal::from_raw(XCAR(XCDR(ret))).as_fixnum().unwrap_or(0).max(0) as usize
        } else {
            0
        }
    })
}

pub fn selected_window_val(ctx: &GcContext) -> LispVal {
    let _nogc = ctx.nogc();
    LispVal::from_raw(crate::window::Fselected_window())
}

pub fn window_buffer_val(ctx: &mut GcContext, window: Handle<'_>) -> VmResult<LispVal> {
    let _ = ctx;
    let raw_win = window.get().into_raw();
    catch_vm_nonlocal(|| {
        let arg = if WINDOWP(raw_win) { raw_win } else { lisp_h_Qnil() };
        LispVal::from_raw(crate::window::Fwindow_buffer(arg))
    })
}

pub fn selected_frame_live_window_count(ctx: &mut GcContext) -> VmResult<usize> {
    let _ = ctx;
    catch_vm_nonlocal(|| {
        let mut list: Lisp_Object =
            crate::window::Fwindow_list(lisp_h_Qnil(), make_fixnum(0), lisp_h_Qnil());
        let mut n = 0usize;
        while CONSP(list) {
            n += 1;
            list = XCDR(list);
        }
        n
    })
}

pub fn trigger_garbage_collect(ctx: &mut GcContext) -> VmResult<()> {
    let _ = ctx;
    catch_vm_nonlocal(|| {
        crate::alloc::Fgarbage_collect();
    })
}

/// Safe `#[cfg(enable_checking)]` assertion failure trampoline for `eassert!`/`eassume!`.
#[cfg(enable_checking)]
#[inline(never)]
#[cold]
pub fn die_checking(msg_nul: &'static str, file_nul: &'static str, line: u32) -> ! {
    let msg = if msg_nul.as_bytes().last() == Some(&0) {
        msg_nul
    } else {
        "assertion failed\0"
    };
    let file = if file_nul.as_bytes().last() == Some(&0) {
        file_nul
    } else {
        "<unknown>\0"
    };
    // SAFETY: `msg` and `file` are `'static str` slices verified above to terminate with a NUL byte (`\0`).
    unsafe {
        crate::common::die(
            msg.as_ptr().cast::<crate::common::c_char>(),
            file.as_ptr().cast::<crate::common::c_char>(),
            line as crate::common::c_int,
        )
    }
}
