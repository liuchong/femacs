// femacs: port of lib/malloc/dynarray-skeleton.c @ fd7afced403.
//
// The reference is a *template*: `DYNARRAY_STRUCT`, `DYNARRAY_ELEMENT` and `DYNARRAY_PREFIX`
// (plus the optional `DYNARRAY_ELEMENT_FREE`, `DYNARRAY_ELEMENT_INIT`, `DYNARRAY_INITIAL_SIZE`
// and `DYNARRAY_FINAL_TYPE`) are the parameters, and C's includers are the instantiations.
// Rust has no textual include or `##` pasting, so PORTING.md 10.13(2) applies: the shared
// source lands as a prelude `macro_rules!` and every instantiation point goes through it.
// The frozen build has exactly one - `lib/regexec.c` 1437-1441, with
// `DYNARRAY_STRUCT regmatch_list`, `DYNARRAY_ELEMENT regmatch_t`, `DYNARRAY_PREFIX
// regmatch_list_` - and `lib/regexec.rs` invokes `dynarray_skeleton_body!` there.
// `lib/malloc/dynarray_*.c` do *not* instantiate it: they include `<dynarray.h>` with none of
// the three names defined, so the gnulib wrapper takes its `#else` arm (`<malloc/dynarray.h>`).
//
// `##` has no Rust counterpart (`concat_idents!` is not stable; PORTING.md P-37's BVAR
// decision), so the macro takes C's `DYNARRAY_NAME (x)` results as `ident` arguments: the
// binding passes `regmatch_list_init`, `regmatch_list_free`, ... explicitly, which is the
// same shape `DEFUN!` uses for its `$cname` and `ftoastr_body!`/`at_func_body!` use for their
// entries.  The three `DYNARRAY_CONCAT0/1`/`DYNARRAY_NAME` records below keep C's spelling of
// the pasting so the ledger and a reader can see where each name comes from.
//
// The arms this file's one binding selects, with C's condition named at the site:
//   * `DYNARRAY_ELEMENT_FREE` and `DYNARRAY_ELEMENT_INIT`: both undefined, so
//     `free__elements__` is empty, `emplace__tail__` has no `memset` arm, and `resize` takes
//     C's `#else` (`__libc_dynarray_resize`).
//   * `DYNARRAY_INITIAL_SIZE`: not defined by any includer, so C 115-121's default applies
//     (`sizeof (regmatch_t) > 64 ? 2 : 128 / sizeof (regmatch_t)` = 8; the measured
//     `sizeof (regmatch_t)` is 16) and `DYNARRAY_HAVE_SCRATCH` is 1.
//   * `DYNARRAY_FINAL_TYPE`: not defined, so `finalize` is C 488-513's `size_t *` variant.
// The non-selected arms are one-line notes at their C positions; docs/ports/dynarray-skeleton.md
// records them and the instance.

/* Pre-processor macros which act as parameters:

   DYNARRAY_STRUCT
      The struct tag of dynamic array to be defined.
   DYNARRAY_ELEMENT
      The type name of the element type.  Elements are copied
      as if by memcpy, and can change address as the dynamic
      array grows.
   DYNARRAY_PREFIX
      The prefix of the functions which are defined.

   The following parameters are optional:

   DYNARRAY_ELEMENT_FREE
      DYNARRAY_ELEMENT_FREE (E) is evaluated to deallocate the
      contents of elements. E is of type  DYNARRAY_ELEMENT *.
   DYNARRAY_ELEMENT_INIT
      DYNARRAY_ELEMENT_INIT (E) is evaluated to initialize a new
      element.  E is of type  DYNARRAY_ELEMENT *.
      If DYNARRAY_ELEMENT_FREE but not DYNARRAY_ELEMENT_INIT is
      defined, new elements are automatically zero-initialized.
      Otherwise, new elements have undefined contents.
   DYNARRAY_INITIAL_SIZE
      The size of the statically allocated array (default:
      at least 2, more elements if they fit into 128 bytes).
      Must be a preprocessor constant.  If DYNARRAY_INITIAL_SIZE is 0,
      there is no statically allocated array at, and all non-empty
      arrays are heap-allocated.
   DYNARRAY_FINAL_TYPE
      The name of the type which holds the final array.  If not
      defined, is PREFIX##finalize not provided.  DYNARRAY_FINAL_TYPE
      must be a struct type, with members of type DYNARRAY_ELEMENT and
      size_t at the start (in this order).

   These macros are undefined after this header file has been
   included.

   The following types are provided (their members are private to the
   dynarray implementation):

     struct DYNARRAY_STRUCT

   The following functions are provided:

     void DYNARRAY_PREFIX##init (struct DYNARRAY_STRUCT *);
     void DYNARRAY_PREFIX##free (struct DYNARRAY_STRUCT *);
     bool DYNARRAY_PREFIX##has_failed (const struct DYNARRAY_STRUCT *);
     void DYNARRAY_PREFIX##mark_failed (struct DYNARRAY_STRUCT *);
     size_t DYNARRAY_PREFIX##size (const struct DYNARRAY_STRUCT *);
     DYNARRAY_ELEMENT *DYNARRAY_PREFIX##begin (const struct DYNARRAY_STRUCT *);
     DYNARRAY_ELEMENT *DYNARRAY_PREFIX##end (const struct DYNARRAY_STRUCT *);
     DYNARRAY_ELEMENT *DYNARRAY_PREFIX##at (struct DYNARRAY_STRUCT *, size_t);
     void DYNARRAY_PREFIX##add (struct DYNARRAY_STRUCT *, DYNARRAY_ELEMENT);
     DYNARRAY_ELEMENT *DYNARRAY_PREFIX##emplace (struct DYNARRAY_STRUCT *);
     bool DYNARRAY_PREFIX##resize (struct DYNARRAY_STRUCT *, size_t);
     void DYNARRAY_PREFIX##remove_last (struct DYNARRAY_STRUCT *);
     void DYNARRAY_PREFIX##clear (struct DYNARRAY_STRUCT *);

   The following functions are provided are provided if the
   prerequisites are met:

     bool DYNARRAY_PREFIX##finalize (struct DYNARRAY_STRUCT *,
                                     DYNARRAY_FINAL_TYPE *);
       (if DYNARRAY_FINAL_TYPE is defined)
     DYNARRAY_ELEMENT *DYNARRAY_PREFIX##finalize (struct DYNARRAY_STRUCT *,
                                                  size_t *);
       (if DYNARRAY_FINAL_TYPE is not defined)
*/

/* C 88's `#include <malloc/dynarray.h>` and C 92-94's `<errno.h>`, `<stdlib.h>` and
   `<string.h>` are the *instantiation site's* mirrors: the skeleton's own text names no
   function of its own, so - like lib/at-func.rs - this file carries no `include!`, and the
   expanded body's `dynarray_header`, `__dynarray_error_marker`, `__libc_dynarray_*`, `free`,
   `memset`, `errno` and `ENOMEM` are in scope where the macro is invoked (C 88 is a no-op
   there: `lib/regexec.c` already reaches the shared header through `regex_internal.h` 35).
   `docs/ports/dynarray-skeleton.md` records the arrangement.  */

/* Internal use only: Helper macros.  (C 143-161.)  C's `DYNARRAY_NAME (name)` is
   `DYNARRAY_CONCAT1 (DYNARRAY_PREFIX, name)`; Rust cannot paste, so the binding passes the
   pasted name (see the file note).  `DYNARRAY_FREE` is `DYNARRAY_NAME (f)` + `ree`, spelled so
   gnulib does not turn `free` into `rpl_free`.  */
macro_rules! DYNARRAY_CONCAT0 {
  ($prefix:ident, $name:ident) => { $name };
}
macro_rules! DYNARRAY_CONCAT1 {
  ($prefix:ident, $name:ident) => { DYNARRAY_CONCAT0! ($prefix, $name) };
}
macro_rules! DYNARRAY_NAME {
  ($name:ident) => { DYNARRAY_CONCAT1! (DYNARRAY_PREFIX, $name) };
}
macro_rules! DYNARRAY_FREE {
  ($list:expr) => { DYNARRAY_CONCAT1! (DYNARRAY_NAME! (f), ree) };
}
/* C 157-161's `#if DYNARRAY_HAVE_SCRATCH / # define DYNARRAY_SCRATCH(list) (list)->scratch /
   #else # define DYNARRAY_SCRATCH(list) NULL / #endif`: the frozen reading is the first arm.
   C's `(list)->scratch` is an array that decays to `DYNARRAY_ELEMENT *`; the port's array does
   not decay, so the macro spells the same pointer with `as_mut_ptr`.  */
macro_rules! DYNARRAY_SCRATCH {
  ($list:expr) => { (*$list).scratch.as_mut_ptr () };
}
/* C 109-121: `DYNARRAY_HAVE_SCRATCH` is 1 for this binding (the default arm), and
   `DYNARRAY_INITIAL_SIZE` is C's `sizeof` expression - the token `DYNARRAY_ELEMENT` is the
   parameter's own name, kept so the ledger's comparison sees C's tokens.  */
macro_rules! DYNARRAY_HAVE_SCRATCH {
  () => { 1 };
}
macro_rules! DYNARRAY_INITIAL_SIZE {
  ($DYNARRAY_ELEMENT:ty) => {
    if std::mem::size_of::<$DYNARRAY_ELEMENT> () > 64 { 2 }
    else { 128 / std::mem::size_of::<$DYNARRAY_ELEMENT> () }
  };
}

/* C 128-528, the template's whole body, parameterised on the binding.  `$struct` is
   `DYNARRAY_STRUCT`, `$u` is the anonymous union's port name, `$header` the anonymous
   `struct { size_t used; size_t allocated; DYNARRAY_ELEMENT *array; }`'s, `$element`
   `DYNARRAY_ELEMENT`, `$initial_size` the pasted `DYNARRAY_INITIAL_SIZE`, and the `$name_*`
   arguments are `DYNARRAY_NAME (…)` / `DYNARRAY_FREE` pasted by the caller, in C's
   declaration order (free__elements__, free__array__, init, free, has_failed, mark_failed,
   size, at, begin, end, add__, add, emplace__tail__, emplace__, emplace, resize, remove_last,
   clear, finalize).  */
macro_rules! dynarray_skeleton_body {
  ($struct:ident, $u:ident, $header:ident, $element:ty, $initial_size:expr,
   $name_free__elements__:ident, $name_free__array__:ident, $name_init:ident,
   $name_free:ident, $name_has_failed:ident, $name_mark_failed:ident, $name_size:ident,
   $name_at:ident, $name_begin:ident, $name_end:ident, $name_add__:ident, $name_add:ident,
   $name_emplace__tail__:ident, $name_emplace__:ident, $name_emplace:ident,
   $name_resize:ident, $name_remove_last:ident, $name_clear:ident,
   $name_finalize:ident) => {

/* Public type definitions.  */

/* All fields of this struct are private to the implementation.  (C 130-141.)
   `DYNARRAY_HAVE_SCRATCH` is 1 (C 120), so the `scratch` member is present.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union $u {
  pub dynarray_abstract: dynarray_header,
  pub dynarray_header: $header,
}
/// C: the anonymous `struct { size_t used; size_t allocated; DYNARRAY_ELEMENT *array; }` of
/// the union above (C 133-138); C's tag is anonymous, so the port gives it a name.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct $header {
  pub used: usize,
  pub allocated: usize,
  pub array: *mut $element,
}
#[repr(C)]
pub struct $struct {
  pub u: $u,
  /* Initial inline allocation.  (C 139-140.)  */
  pub scratch: [$element; $initial_size],
}

/* Internal use only: Helper functions.  */

/* Internal function.  Call DYNARRAY_ELEMENT_FREE with the array
   elements.  Name mangling needed due to the DYNARRAY_ELEMENT_FREE
   macro expansion.  (C 170-178.)  `DYNARRAY_ELEMENT_FREE` is undefined for this binding, so
   C's loop - `__dynarray_i`, its `0` and `++`, and the `[__dynarray_i]` subscript - is named
   here for the ledger rather than carried as text (P-51).  */
unsafe fn $name_free__elements__ (__dynarray_array: *mut $element, __dynarray_used: usize)
{
}

/* Internal function.  Free the non-scratch array allocation.  (C 181-190.)  */
unsafe fn $name_free__array__ (list: *mut $struct)
{
  if (*list).u.dynarray_header.array != DYNARRAY_SCRATCH! (list)
    { free ((*list).u.dynarray_header.array as *mut c_void); }
}

/* Public functions.  */

/* Initialize a dynamic array object.  This must be called before any
   use of the object.  (C 194-201.)  */
unsafe fn $name_init (list: *mut $struct)
{
  (*list).u.dynarray_header.used = 0;
  (*list).u.dynarray_header.allocated = $initial_size;
  (*list).u.dynarray_header.array = DYNARRAY_SCRATCH! (list);
}

/* Deallocate the dynamic array and its elements.  (C 204-211.)  */
unsafe fn $name_free (list: *mut $struct)
{
  $name_free__elements__
    ((*list).u.dynarray_header.array, (*list).u.dynarray_header.used);
  $name_free__array__ (list);
  $name_init (list);
}

/* Return true if the dynamic array is in an error state.  (C 214-219.)  */
unsafe fn $name_has_failed (list: *const $struct) -> bool
{
  return (*list).u.dynarray_header.allocated == __dynarray_error_marker ();
}

/* Mark the dynamic array as failed.  All elements are deallocated as
   a side effect.  (C 222-231.)  */
unsafe fn $name_mark_failed (list: *mut $struct)
{
  $name_free__elements__
    ((*list).u.dynarray_header.array, (*list).u.dynarray_header.used);
  $name_free__array__ (list);
  (*list).u.dynarray_header.array = DYNARRAY_SCRATCH! (list);
  (*list).u.dynarray_header.used = 0;
  (*list).u.dynarray_header.allocated = __dynarray_error_marker ();
}

/* Return the number of elements which have been added to the dynamic
   array.  (C 234-240.)  */
unsafe fn $name_size (list: *const $struct) -> usize
{
  return (*list).u.dynarray_header.used;
}

/* Return a pointer to the array element at INDEX.  Terminate the
   process if INDEX is out of bounds.  (C 243-251.)  */
unsafe fn $name_at (list: *mut $struct, index: usize) -> *mut $element
{
  if __glibc_unlikely (index >= $name_size (list))
    { __libc_dynarray_at_failure ($name_size (list), index); }
  return (*list).u.dynarray_header.array.add (index);
}

/* Return a pointer to the first array element, if any.  For a
   zero-length array, the pointer can be NULL even though the dynamic
   array has not entered the failure state.  (C 254-260.)  */
unsafe fn $name_begin (list: *mut $struct) -> *mut $element
{
  return (*list).u.dynarray_header.array;
}

/* Return a pointer one element past the last array element.  For a
   zero-length array, the pointer can be NULL even though the dynamic
   array has not entered the failure state.  (C 263-269.)  */
unsafe fn $name_end (list: *mut $struct) -> *mut $element
{
  return (*list).u.dynarray_header.array.add ((*list).u.dynarray_header.used);
}

/* Internal function.  Slow path for the add function below.  (C 272-285.)  */
unsafe fn $name_add__ (list: *mut $struct, item: $element)
{
  if __glibc_unlikely
      (!__libc_dynarray_emplace_enlarge (&mut (*list).u.dynarray_abstract,
                                         DYNARRAY_SCRATCH! (list) as *mut c_void,
                                         std::mem::size_of::<$element> ()))
    {
      $name_mark_failed (list);
      return;
    }

  /* Copy the new element and increase the array length.  (C's
     `list->u.dynarray_header.array[list->u.dynarray_header.used++] = item;`, whose `++` the
     port writes as the store plus the increment.)  */
  *(*list).u.dynarray_header.array.add ((*list).u.dynarray_header.used) = item;
  (*list).u.dynarray_header.used += 1;
}

/* Add ITEM at the end of the array, enlarging it by one element.
   Mark *LIST as failed if the dynamic array allocation size cannot be
   increased.  (C 292-312.)  */
unsafe fn $name_add (list: *mut $struct, item: $element)
{
  /* Do nothing in case of previous error.  */
  if $name_has_failed (list)
    { return; }

  /* Enlarge the array if necessary.  */
  if __glibc_unlikely ((*list).u.dynarray_header.used
                       == (*list).u.dynarray_header.allocated)
    {
      $name_add__ (list, item);
      return;
    }

  /* Copy the new element and increase the array length.  */
  *(*list).u.dynarray_header.array.add ((*list).u.dynarray_header.used) = item;
  (*list).u.dynarray_header.used += 1;
}

/* Internal function.  Building block for the emplace functions below.
   Assumes space for one more element in *LIST.  (C 317-333.)  C 325-332's
   `#if defined (DYNARRAY_ELEMENT_INIT) / #elif defined (DYNARRAY_ELEMENT_FREE)
   memset (result, 0, sizeof (*result)); / #endif` is empty for this binding: neither name is
   defined.  */
unsafe fn $name_emplace__tail__ (list: *mut $struct) -> *mut $element
{
  let result: *mut $element
    = (*list).u.dynarray_header.array.add ((*list).u.dynarray_header.used);
  (*list).u.dynarray_header.used += 1;
  return result;
}

/* Internal function.  Slow path for the emplace function below.  (C 336-349.)  */
unsafe fn $name_emplace__ (list: *mut $struct) -> *mut $element
{
  if __glibc_unlikely
      (!__libc_dynarray_emplace_enlarge (&mut (*list).u.dynarray_abstract,
                                         DYNARRAY_SCRATCH! (list) as *mut c_void,
                                         std::mem::size_of::<$element> ()))
    {
      $name_mark_failed (list);
      return null_mut ();
    }
  return $name_emplace__tail__ (list);
}

/* Allocate a place for a new element in *LIST and return a pointer to
   it.  The pointer can be NULL if the dynamic array cannot be
   enlarged due to a memory allocation failure.  (C 352-370.)  */
unsafe fn $name_emplace (list: *mut $struct) -> *mut $element
{
  /* Do nothing in case of previous error.  */
  if $name_has_failed (list)
    { return null_mut (); }

  /* Enlarge the array if necessary.  */
  if __glibc_unlikely ((*list).u.dynarray_header.used
                       == (*list).u.dynarray_header.allocated)
    { return $name_emplace__ (list); }
  return $name_emplace__tail__ (list);
}

/* Change the size of *LIST to SIZE.  If SIZE is larger than the
   existing size, new elements are added (which can be initialized).
   Otherwise, the list is truncated, and elements are freed.  Return
   false on memory allocation failure (and mark *LIST as failed).  (C 373-413.)  */
unsafe fn $name_resize (list: *mut $struct, size: usize) -> bool
{
  if size > (*list).u.dynarray_header.used
    {
      let ok: bool;
      /* C 383-406's `#if defined (DYNARRAY_ELEMENT_INIT) / #elif defined
         (DYNARRAY_ELEMENT_FREE)` arms are both dead for this binding, so the live arm is
         C's `#else`: `__libc_dynarray_resize`.  */
      ok = __libc_dynarray_resize (&mut (*list).u.dynarray_abstract, size,
                                   DYNARRAY_SCRATCH! (list) as *mut c_void,
                                   std::mem::size_of::<$element> ());
      if __glibc_unlikely (!ok)
        { $name_mark_failed (list); }
      return ok;
    }
  else
    {
      /* The list has shrunk in size.  Free the removed elements.  */
      $name_free__elements__
        ((*list).u.dynarray_header.array.add (size),
         (*list).u.dynarray_header.used - size);
      (*list).u.dynarray_header.used = size;
      return true;
    }
}

/* Remove the last element of LIST if it is present.  (C 416-431.)  */
unsafe fn $name_remove_last (list: *mut $struct)
{
  /* used > 0 implies that the array is the non-failed state.  */
  if (*list).u.dynarray_header.used > 0
    {
      let new_length: usize = (*list).u.dynarray_header.used - 1;
      /* C 429-431's `#ifdef DYNARRAY_ELEMENT_FREE` arm is dead for this binding.  */
      (*list).u.dynarray_header.used = new_length;
    }
}

/* Remove all elements from the list.  The elements are freed, but the
   list itself is not.  (C 434-444.)  */
unsafe fn $name_clear (list: *mut $struct)
{
  /* free__elements__ does nothing if the list is in the failed
     state.  */
  $name_free__elements__
    ((*list).u.dynarray_header.array, (*list).u.dynarray_header.used);
  (*list).u.dynarray_header.used = 0;
}

/* C 447-513: `#ifdef DYNARRAY_FINAL_TYPE` is dead for this binding (`DYNARRAY_FINAL_TYPE` is
   not defined), so C 488-513's `#else` arm is the live one: it returns the heap array and
   writes the length through `size_t *lengthp`.  */
/* Transfer the dynamic array to a heap-allocated array and return a
   pointer to it.  The pointer is NULL if memory allocation fails, or
   if the array is empty, so this function should be used only for
   arrays which are known not be empty (usually because they always
   have a sentinel at the end).  If LENGTHP is not NULL, the array
   length is written to *LENGTHP.  *LIST is re-initialized and can be
   reused.  */
unsafe fn $name_finalize (list: *mut $struct, lengthp: *mut usize) -> *mut $element
{
  let mut res: dynarray_finalize_result = std::mem::zeroed ();
  if __libc_dynarray_finalize (&mut (*list).u.dynarray_abstract,
                               DYNARRAY_SCRATCH! (list) as *mut c_void,
                               std::mem::size_of::<$element> (), &mut res)
    {
      /* On success, the result owns all the data.  */
      $name_init (list);
      if !lengthp.is_null ()
        { *lengthp = res.length; }
      return res.array as *mut $element;
    }
  else
    {
      /* On error, we need to free all data.  */
      $name_free (list);
      *errno () = ENOMEM;
      return null_mut ();
    }
}

  };
}
