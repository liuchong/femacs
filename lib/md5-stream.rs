// femacs: port of lib/md5-stream.c @ fd7afced403.
//! Stream wrapper feeding `FILE *` chunks into safe `femacs_sys::md5::Md5Ctx` with RAII buffer ownership.
#![deny(unsafe_op_in_unsafe_fn)]
use crate::common::*;

#[cfg(feature = "libc")]
pub const WORDS_BIGENDIAN: c_int = 1;
#[cfg(feature = "libc")]
macro_rules! md5_stream { () => { __md5_stream } }

pub const BLOCKSIZE: usize = 32768;
const _: () = core::assert!(BLOCKSIZE % 64 == 0, "invalid BLOCKSIZE");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_stream(stream: *mut FILE, resblock: *mut c_void) -> c_int {
    let mut buffer = vec![0u8; BLOCKSIZE];
    let mut ctx = ::femacs_sys::md5::Md5Ctx::new();
    let mut sum: usize;

    'process_partial_block: loop {
        sum = 0;
        loop {
            // SAFETY: `stream` is a valid open `FILE *` provided by the caller.
            if unsafe { feof(stream) } != 0 {
                break 'process_partial_block;
            }
            // SAFETY: `buffer[sum..BLOCKSIZE]` is uniquely owned writable storage and `stream` is valid.
            let n = unsafe {
                fread(
                    buffer[sum..].as_mut_ptr().cast::<c_void>(),
                    1,
                    BLOCKSIZE - sum,
                    stream,
                )
            };
            sum += n;
            if sum == BLOCKSIZE {
                break;
            }
            if n == 0 {
                // SAFETY: `stream` is valid; check stream error indicator on 0-byte read.
                if unsafe { ferror(stream) } != 0 {
                    return 1;
                }
                break 'process_partial_block;
            }
        }
        ctx.process_block(&buffer[..BLOCKSIZE]);
    }

    if sum > 0 {
        ctx.process_bytes(&buffer[..sum]);
    }

    let mut out = [0u8; 16];
    ctx.finish_into(&mut out);
    // SAFETY: Caller provides a 16-byte writable `resblock` output buffer.
    unsafe {
        core::ptr::copy_nonoverlapping(out.as_ptr(), resblock.cast::<u8>(), 16);
    }
    0
}
