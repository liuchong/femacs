// femacs: mirror of lib/malloc/scratch_buffer.h @ fd7afced403.
//
// The header the frozen build's generated `lib/malloc/scratch_buffer.gl.h` is produced
// from: the struct (C 64-70) and the five `static inline` bodies (C 72-133) below are that
// text line for line, C 1-18's declaration block is not carried (PORTING.md 0.1-19), and
// the three `bool __libc_scratch_buffer_*` prototypes (C 94/106/122) sit in the `unsafe
// extern` blocks where C declares them, with C 95/107/124's `libc_hidden_proto` markers
// empty in this build (libc-config.h's `# define libc_hidden_proto(name)`).
//
// The header that a *unit* includes is the wrapper, `lib/scratch_buffer.h` (mirrored as
// `lib/include/scratch_buffer.rs`); C 104-107 of the wrapper renames the three
// entries to `_gl_scratch_buffer_*` before it includes the generated gl.h, which is why the
// frozen objects export `_gl_scratch_buffer_grow` and none of them exports the unrenamed
// name (`nm build/emacs-ref/lib/malloc/scratch_buffer_grow.o` prints `T
// __gl_scratch_buffer_grow`, Mach-O's spelling of `_gl_scratch_buffer_grow`).  This mirror
// keeps the header's own, unrenamed names, as its text has them.
//
// `free` is the generated `<stdlib.h>`'s spelling - `build/emacs-ref/lib/stdlib.h` 883-884
// has `# undef free` / `# define free rpl_free` - and the frozen objects call it by that
// name (`nm build/emacs-ref/lib/malloc/scratch_buffer_grow.o` prints `U _rpl_free`), so the
// port spells `rpl_free` here too (PORTING.md 10.13, the rpl_mempcpy precedent).
//
// C 98/110/127's `_GL_ATTRIBUTE_ALWAYS_INLINE` is the generated header's spelling of
// `__always_inline`; the port's functions are ordinary items, so the qualifier is
// P-12/P-37's `#[inline(always)]`.  The scalar names (`c_char`, `c_void`) come from the
// including unit's scope, as the other lib mirrors expect.

/* Scratch buffers with a default stack allocation and fallback to
   heap allocation.  It is expected that this function is used in this
   way:

     struct scratch_buffer tmpbuf;
     scratch_buffer_init (&tmpbuf);

     while (!function_that_uses_buffer (tmpbuf.data, tmpbuf.length))
       if (!scratch_buffer_grow (&tmpbuf))
	 return -1;

     scratch_buffer_free (&tmpbuf);
     return 0;

   The allocation functions (scratch_buffer_grow,
   scratch_buffer_grow_preserve, scratch_buffer_set_array_size) make
   sure that the heap allocation, if any, is freed, so that the code
   above does not have a memory leak.  The buffer still remains in a
   state that can be deallocated using scratch_buffer_free, so a loop
   like this is valid as well:

     struct scratch_buffer tmpbuf;
     scratch_buffer_init (&tmpbuf);

     while (!function_that_uses_buffer (tmpbuf.data, tmpbuf.length))
       if (!scratch_buffer_grow (&tmpbuf))
	 break;

     scratch_buffer_free (&tmpbuf);

   scratch_buffer_grow and scratch_buffer_grow_preserve are guaranteed
   to grow the buffer by at least 512 bytes.  This means that when
   using the scratch buffer as a backing store for a non-character
   array whose element size, in bytes, is 512 or smaller, the scratch
   buffer only has to grow once to make room for at least one more
   element.
*/

/* C 60-62's `<stdbool.h>`, `<stddef.h>` and `<stdlib.h>`: `bool`/`size_t` are Rust's own,
   and `rpl_free` is the one entry `scratch_buffer_free` needs.  */
unsafe extern "C" {
  /* void rpl_free (void *ptr); - <stdlib.h>'s `free` under REPLACE_FREE.  */
  pub fn rpl_free (ptr: *mut c_void);
}

/// `<stddef.h>`'s `max_align_t` on the frozen target (`long double`, 8 bytes and 8-aligned:
/// `printf ("%zu %zu", sizeof (max_align_t), _Alignof (max_align_t))` -> `8 8`).  Only its
/// alignment matters to `__space`.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct MaxAlign {
  pub __opaque: [u8; 8],
}

/// C 69's `char __c[1024]`: the union's size (`sizeof (buffer->__space)`).
pub const SCRATCH_BUFFER_SPACE_SIZE: usize = 1024;

#[repr(C)]
#[derive(Clone, Copy)]
pub union scratch_buffer_space {
  /* max_align_t __align;  */
  pub __align: MaxAlign,
  /* char __c[1024];  */
  pub __c: [c_char; SCRATCH_BUFFER_SPACE_SIZE],
}

/* Scratch buffer.  Must be initialized with scratch_buffer_init
   before its use.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct scratch_buffer {
  /* void *data; - Pointer to the beginning of the scratch area.  */
  pub data: *mut c_void,
  /* size_t length; - Allocated space at the data pointer, in bytes.  */
  pub length: usize,
  /* union { max_align_t __align; char __c[1024]; } __space;  */
  pub __space: scratch_buffer_space,
}

/* The C compiler's layout for the type, measured with the frozen target's compiler on the
   struct above: size 1040, alignment 8, `data` at 0, `length` at 8, `__space` at 16
   (`printf ("%zu %zu %zu %zu", sizeof (struct scratch_buffer), offsetof (…, data),
   offsetof (…, length), offsetof (…, __space))` -> `1040 0 8 16`).  `tools/gen-layout-fields.py`'s
   key set does not carry the type, so the mirror states the facts here (PORTING.md 0.1-17's
   generator covers the types it does key).  */
const _: () = core::assert!(std::mem::size_of::<scratch_buffer> () == 1040);
const _: () = core::assert!(std::mem::align_of::<scratch_buffer> () == 8);
const _: () = core::assert!(std::mem::offset_of! (scratch_buffer, data) == 0);
const _: () = core::assert!(std::mem::offset_of! (scratch_buffer, length) == 8);
const _: () = core::assert!(std::mem::offset_of! (scratch_buffer, __space) == 16);

/* `<sys/cdefs.h>`'s branch hints (PORTING.md 2.6): the names and the identity, as
   lib/regcomp.rs spells them.  */
fn __glibc_likely<T> (cond: T) -> T { cond }
fn __glibc_unlikely<T> (cond: T) -> T { cond }

/* Initializes *BUFFER so that BUFFER->data points to BUFFER->__space
   and BUFFER->length reflects the available space.  */
#[inline(always)]
pub unsafe fn scratch_buffer_init (buffer: *mut scratch_buffer)
{
  (*buffer).data = &mut (*buffer).__space.__c as *mut [c_char] as *mut c_void;
  (*buffer).length = std::mem::size_of_val (&(*buffer).__space);
}

/* Deallocates *BUFFER (if it was heap-allocated).  */
#[inline(always)]
pub unsafe fn scratch_buffer_free (buffer: *mut scratch_buffer)
{
  if (*buffer).data != &mut (*buffer).__space.__c as *mut [c_char] as *mut c_void
    { rpl_free ((*buffer).data); }
}

/* Grow *BUFFER by some arbitrary amount.  The buffer contents is NOT
   preserved.  Return true on success, false on allocation failure (in
   which case the old buffer is freed).  On success, the new buffer is
   larger than the previous size.  On failure, *BUFFER is deallocated,
   but remains in a free-able state, and errno is set.  */
unsafe extern "C" {
  /* bool __libc_scratch_buffer_grow (struct scratch_buffer *buffer);  */
  pub fn __libc_scratch_buffer_grow (buffer: *mut scratch_buffer) -> bool;
}
/* C 95's `libc_hidden_proto (__libc_scratch_buffer_grow)`: empty in this build.  */

/* Alias for __libc_scratch_buffer_grow.  */
#[inline(always)]
pub unsafe fn scratch_buffer_grow (buffer: *mut scratch_buffer) -> bool
{
  return __glibc_likely (__libc_scratch_buffer_grow (buffer));
}

/* Like __libc_scratch_buffer_grow, but preserve the old buffer
   contents on success, as a prefix of the new buffer.  */
unsafe extern "C" {
  /* bool __libc_scratch_buffer_grow_preserve (struct scratch_buffer *buffer);  */
  pub fn __libc_scratch_buffer_grow_preserve (buffer: *mut scratch_buffer) -> bool;
}
/* C 107's `libc_hidden_proto (__libc_scratch_buffer_grow_preserve)`: empty here.  */

/* Alias for __libc_scratch_buffer_grow_preserve.  */
#[inline(always)]
pub unsafe fn scratch_buffer_grow_preserve (buffer: *mut scratch_buffer) -> bool
{
  return __glibc_likely (__libc_scratch_buffer_grow_preserve (buffer));
}

/* Grow *BUFFER so that it can store at least NELEM elements of SIZE
   bytes.  The buffer contents are NOT preserved.  Both NELEM and SIZE
   can be zero.  Return true on success, false on allocation failure
   (in which case the old buffer is freed, but *BUFFER remains in a
   free-able state, and errno is set).  It is unspecified whether this
   function can reduce the array size.  */
unsafe extern "C" {
  /* bool __libc_scratch_buffer_set_array_size (struct scratch_buffer *buffer,
                                                size_t nelem, size_t size);  */
  pub fn __libc_scratch_buffer_set_array_size (buffer: *mut scratch_buffer, nelem: usize,
                                               size: usize) -> bool;
}
/* C 124's `libc_hidden_proto (__libc_scratch_buffer_set_array_size)`: empty here.  */

/* Alias for __libc_scratch_set_array_size.  */
#[inline(always)]
pub unsafe fn scratch_buffer_set_array_size (buffer: *mut scratch_buffer, nelem: usize,
                                             size: usize) -> bool
{
  return __glibc_likely (__libc_scratch_buffer_set_array_size (buffer, nelem, size));
}

/* The include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const _SCRATCH_BUFFER_H: bool = true;
