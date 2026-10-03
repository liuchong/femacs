// femacs: port of lib/close-stream.h @ fd7afced403 ; PORTING.md 0.1.
//
// Three lines of text: the `<stdio.h>` include, the `extern "C"` guard and the one
// prototype.  The declaration lives in `pub mod close_stream_decls` + glob import, because
// lib/close-stream.c *defines* `close_stream` (E0428; PORTING.md 0.1-10).
include!("stdio.rs");

pub mod close_stream_decls {
  use super::*;

  unsafe extern "C" {
    /* int close_stream (FILE *stream);  */
    pub fn close_stream (stream: *mut FILE) -> c_int;
  }
}
use close_stream_decls::*;
