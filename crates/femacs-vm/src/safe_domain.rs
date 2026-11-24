//! Safe high-level Buffer, Reader, File I/O, Window, and Evaluation domain layer (U4).
//!
//! Enforced with `#![forbid(unsafe_code)]`: every operation uses `GcContext`, `Rooted`, `Handle`,
//! `PersistentRoot`, and `NoGcToken` from `crate::roots` + `crate::vm_primitives`, demonstrating
//! complete high-level safe Rust composition across GC safepoints and preparing `C1` `femacs-host`.

#![forbid(unsafe_code)]

use crate::nonlocal::VmResult;
use crate::roots::{GcContext, PersistentRoot, Rooted};
use crate::vm_primitives;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferSummary {
    pub name: String,
    pub chars: usize,
    pub bytes: usize,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowSummary {
    pub selected_buffer_name: String,
    pub live_window_count: usize,
}

/// Create or look up a named buffer, replace its text, optionally run GC while holding the buffer
/// in a `Rooted` handle, and read back its UTF-8 content and character/byte metrics.
pub fn write_and_read_named_buffer(
    ctx: &mut GcContext,
    buffer_name: &str,
    content: &str,
    force_gc: bool,
) -> VmResult<BufferSummary> {
    let name_str = vm_primitives::make_utf8_string(ctx, buffer_name)?;
    let buf = vm_primitives::get_buffer_create(ctx, name_str.handle())?;
    let payload = vm_primitives::make_utf8_string(ctx, content)?;

    vm_primitives::with_current_buffer(ctx, buf.handle(), |ctx| {
        vm_primitives::erase_current_buffer(ctx)?;
        vm_primitives::insert_lisp_string(ctx, payload.handle())?;
        if force_gc {
            vm_primitives::trigger_garbage_collect(ctx)?;
        }
        let (chars, bytes) = vm_primitives::current_buffer_char_and_byte_len(ctx);
        let text_val = vm_primitives::current_buffer_string_no_props(ctx)?;
        let actual_name_val = vm_primitives::buffer_name_val(ctx, buf.handle())?;
        let text = vm_primitives::lisp_string_to_owned(ctx, text_val.handle()).unwrap_or_default();
        let name =
            vm_primitives::lisp_string_to_owned(ctx, actual_name_val.handle()).unwrap_or_default();
        Ok(BufferSummary {
            name,
            chars,
            bytes,
            text,
        })
    })
}

/// Execute a temporary scratch buffer workflow in safe Rust and deterministically kill the buffer
/// on both normal completion and `NonLocalExit`.
pub fn with_ephemeral_buffer<R>(
    ctx: &mut GcContext,
    temp_name: &str,
    initial_text: &str,
    f: impl FnOnce(&mut GcContext) -> VmResult<R>,
) -> VmResult<R> {
    let name_val = vm_primitives::make_utf8_string(ctx, temp_name)?;
    let buf = vm_primitives::get_buffer_create(ctx, name_val.handle())?;
    let init_val = vm_primitives::make_utf8_string(ctx, initial_text)?;

    let outcome = vm_primitives::with_current_buffer(ctx, buf.handle(), |ctx| {
        vm_primitives::erase_current_buffer(ctx)?;
        vm_primitives::insert_lisp_string(ctx, init_val.handle())?;
        f(ctx)
    });
    let _ = vm_primitives::kill_buffer_val(ctx, buf.handle());
    outcome
}

/// Read a single Lisp expression from UTF-8 source, evaluate it lexically across a GC safepoint,
/// hold the result in a `PersistentRoot` across an explicit GC cycle, and format with `prin1`.
pub fn read_eval_and_format_utf8(ctx: &mut GcContext, source: &str) -> VmResult<String> {
    let src_val = vm_primitives::make_utf8_string(ctx, source)?;
    let (form, _end) = vm_primitives::read_from_lisp_string(ctx, src_val.handle(), 0)?;
    let evaluated = vm_primitives::eval_lexical(ctx, form.handle())?;

    let mut persistent = PersistentRoot::from_handle(ctx, evaluated.handle());
    drop(evaluated);
    vm_primitives::trigger_garbage_collect(ctx)?;
    let printed = vm_primitives::prin1_to_lisp_string(ctx, persistent.handle())?;
    let rendered = vm_primitives::lisp_string_to_owned(ctx, printed.handle()).unwrap_or_default();
    persistent.release();
    Ok(rendered)
}

/// Write UTF-8 text to `path` and read it back through VM file I/O primitives (`expand-file-name`,
/// `write-region`, `insert-file-contents`) using exclusively safe `Rooted` handles.
pub fn roundtrip_utf8_file(ctx: &mut GcContext, path: &str, content: &str) -> VmResult<String> {
    let raw_path = vm_primitives::make_utf8_string(ctx, path)?;
    let expanded = vm_primitives::expand_file_name_val(ctx, raw_path.handle())?;

    with_ephemeral_buffer(ctx, " *u4-safe-io-write*", content, |ctx| {
        vm_primitives::write_current_buffer_to_file(ctx, expanded.handle())
    })?;

    with_ephemeral_buffer(ctx, " *u4-safe-io-read*", "", |ctx| {
        let _inserted = vm_primitives::insert_file_into_current_buffer(ctx, expanded.handle())?;
        let text_val = vm_primitives::current_buffer_string_no_props(ctx)?;
        Ok(vm_primitives::lisp_string_to_owned(ctx, text_val.handle()).unwrap_or_default())
    })
}

/// Inspect the selected window's buffer name and live window count on the selected frame.
pub fn inspect_selected_window(ctx: &mut GcContext) -> VmResult<WindowSummary> {
    let win = vm_primitives::selected_window_val(ctx);
    let buf = vm_primitives::window_buffer_val(ctx, win.handle())?;
    let bname = vm_primitives::buffer_name_val(ctx, buf.handle())?;
    let selected_buffer_name =
        vm_primitives::lisp_string_to_owned(ctx, bname.handle()).unwrap_or_default();
    let live_window_count = vm_primitives::selected_frame_live_window_count(ctx)?;
    Ok(WindowSummary {
        selected_buffer_name,
        live_window_count,
    })
}

/// Run an end-to-end self-check across all four migrated high-level safe domains (Buffer, Reader,
/// File I/O, and Window) on a live VM thread, returning a rooted Lisp string for `AU4`/`AU7`.
pub fn exercise_all_safe_domains(ctx: &mut GcContext, temp_file_path: &str) -> VmResult<Rooted> {
    ctx.with_narrowed_stack_bottom(|ctx| {
        let buf_summary = write_and_read_named_buffer(
            ctx,
            " *u4-safe-buffer*",
            "safe-buffer-αβγ",
            true,
        )?;
        let kill_name = vm_primitives::make_utf8_string(ctx, " *u4-safe-buffer*")?;
        let kill_buf = vm_primitives::get_buffer_create(ctx, kill_name.handle())?;
        let _ = vm_primitives::kill_buffer_val(ctx, kill_buf.handle());

        let eval_out = read_eval_and_format_utf8(ctx, "(mapcar (lambda (x) (* x x)) '(2 3 5 7))")?;
        let io_out = roundtrip_utf8_file(ctx, temp_file_path, "u4-io-roundtrip-你好\n")?;
        let win_out = inspect_selected_window(ctx)?;

        let report = format!(
            "buf={}:{}:{};eval={};io_ok={};win={}:{}",
            buf_summary.name,
            buf_summary.chars,
            buf_summary.bytes,
            eval_out,
            io_out == "u4-io-roundtrip-你好\n",
            !win_out.selected_buffer_name.is_empty(),
            win_out.live_window_count,
        );
        vm_primitives::make_utf8_string(ctx, &report)
    })
}
