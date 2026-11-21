// femacs: mirror of src/region-cache.h @ fd7afced403.
//
// Mirror (PORTING.md 0.1-2): the six entry points are declared in `region_cache_decls` + glob
// import (0.1-10, the intervals.rs shape) because `src/region-cache.c`'s port
// (`src/region-cache.rs`) defines every one of them; nothing here emits a symbol.
//
// Two pointer types appear in C's prototypes.  `struct region_cache` is incomplete in the
// header - the record is `src/region-cache.c`'s and is only ever reached through a pointer -
// so the port spells it `*mut c_void`, the ABI-equal form 0.1-2/P-72 rule 3 asks for - the one
// buffer.rs's `newline_cache`, `width_run_cache` and `bidi_paragraph_cache` fields already use,
// and what the definition unit's own `pub struct region_cache` (the record itself) is spelled
// as at every interface.  search.rs keeps one local `pub type region_cache = c_void;` because
// its bodies name C's type in six local declarations (P-72 rule 3's use-site spelling).  `struct buffer` is *not* hidden that way: C 59 forward-declares it, buffer.rs's mirror
// owns the record, and C's own prototypes spell `struct buffer *BUF`, so this mirror includes
// buffer.rs and keeps the real type - the intervals.rs precedent, whose two `struct buffer *`
// parameters do the same.  (C's inclusion order is preserved: every consumer includes
// buffer.h before region-cache.h, and 0.1-4's include graph makes the second copy a no-op.)
//
// The four units that name these entries - bidi.rs, insdel.rs, search.rs and the definition
// unit region-cache.rs - include this mirror at C's own position, and the copies of the
// declarations that stood in them went away with it (two same-named declarations in one unit
// are an `E0428`; 0.1-10's glob import is what makes the mirror's copy visible).

// [U1-MODULE-MIRROR]
use super::*;
// [U1] buffer.rs imported via super

pub const EMACS_REGION_CACHE_H: c_int = 1;

pub mod region_cache_decls {
  use super::*;

  unsafe extern "C-unwind" {
    /* Allocate, initialize and return a new, empty region cache.  */
    /// C: region-cache.h 68: `struct region_cache *new_region_cache (void);`
    pub fn new_region_cache () -> *mut c_void;

    /* Free a region cache.  */
    /// C: region-cache.h 71: `void free_region_cache (struct region_cache *);`
    pub fn free_region_cache (c: *mut c_void);

    /* Assert that the region of BUF between START and END (absolute
       buffer positions) is "known," for the purposes of CACHE (e.g. "has
       no newlines", in the case of the line cache).  */
    /// C: region-cache.h 76-78: `extern void know_region_cache (struct buffer *BUF,
    /// struct region_cache *CACHE, ptrdiff_t START, ptrdiff_t END);`
    pub fn know_region_cache (buf: *mut buffer, c: *mut c_void, start: ptrdiff_t,
                              end: ptrdiff_t);

    /* Indicate that a section of BUF has changed, to invalidate CACHE.
       HEAD is the number of chars unchanged at the beginning of the buffer.
       TAIL is the number of chars unchanged at the end of the buffer.  */
    /// C: region-cache.h 88-90: `extern void invalidate_region_cache (struct buffer *BUF,
    /// struct region_cache *CACHE, ptrdiff_t HEAD, ptrdiff_t TAIL);`
    pub fn invalidate_region_cache (buf: *mut buffer, c: *mut c_void, head: ptrdiff_t,
                                    tail: ptrdiff_t);

    /* Return the value for the text immediately after POS in BUF if the value
       is known, for the purposes of CACHE, and return zero otherwise.
       If NEXT is non-zero, set *NEXT to the nearest
       position after POS where the knowledge changes.  */
    /// C: region-cache.h 105-107: `extern int region_cache_forward (struct buffer *buf,
    /// struct region_cache *c, ptrdiff_t pos, ptrdiff_t *next);`
    pub fn region_cache_forward (buf: *mut buffer, c: *mut c_void, pos: ptrdiff_t,
                                 next: *mut ptrdiff_t) -> c_int;

    /* Likewise, except before POS rather than after POS.  */
    /// C: region-cache.h 110-111: `extern int region_cache_backward (struct buffer *buf,
    /// struct region_cache *c, ptrdiff_t pos, ptrdiff_t *next);`
    pub fn region_cache_backward (buf: *mut buffer, c: *mut c_void, pos: ptrdiff_t,
                                  next: *mut ptrdiff_t) -> c_int;
  }
}
pub use self::region_cache_decls::*;
