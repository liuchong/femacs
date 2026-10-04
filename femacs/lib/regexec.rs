// femacs: port of lib/regexec.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The file
// is in `docs/baseline/config/cond-eval-verified.txt`, so the gate reads it with the frozen
// configuration: the `_LIBC` arms (regexec.c 157-160, 219-239, 276-278, 287-289, 299-301,
// 311-313, 338-343, 549-551, 556-565, 3692-3694, 3778-3783, 3806-3895, 3921-3979) and the
// three `#if 0` blocks (regexec.c 119-123, 2293-2297, 2469-2509) are dead; only their
// *live* text is ported here (a comment marks each spot).
//
// C's `static` forward declarations (regexec.c 20-181) have no Rust counterpart (0.1-12),
// so they are dropped.  `weak_alias`/`libc_hidden_def` only affect symbol binding; the port
// keeps the call text as no-op macros.  The generated <regex.h> renames the seven exported
// definitions (`regexec` -> `rpl_regexec`, `re_match` -> `rpl_re_match`, ...), so each keeps
// C's source name and emits the renamed symbol (PORTING.md 10.13, the regcomp.rs precedent).
//
// `#include <malloc/dynarray-skeleton.c>` (regexec.c 1438-1441) is the inclusion shim
// PORTING.md 10.13's second clause covers: the template's three bindings are the
// `DYNARRAY_STRUCT`/`DYNARRAY_ELEMENT`/`DYNARRAY_PREFIX` macros defined at that spot, and the
// port instantiates the template once through `femacs/lib/malloc/dynarray-skeleton.rs`'s
// `dynarray_skeleton_body!` (the strftime_body!/ftoastr_body! shape) in the `regmatch_list`
// block below; the template's generic helpers belong to `lib/malloc/dynarray_*.c`, and the
// shared header's declarations arrive from the mirror `include/dynarray.rs` instead of the
// file's own `unsafe extern "C"` block.  `docs/ports/regexec.md` records the choice.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr::null_mut;

include!("include/regex_internal.rs");
/* C: `regex_internal.h` 35's `# ifndef _LIBC / # include <dynarray.h>` - the gnulib wrapper,
   whose `#else` arm (`<malloc/dynarray.h>`) reaches the shared header.  Its declarations and
   the renamed symbols' `#[link_name]`s now arrive from this mirror rather than from the
   file's own block below (PORTING.md 0.1-3).  */
include!("include/dynarray.rs");
/* C: the `malloc/dynarray-skeleton.c` expansion below includes <errno.h> (skeleton 91).  */
include!("include/errno.rs");

/* C: `__always_inline`.  */
macro_rules! __always_inline {
    ($($x:tt)*) => {};
}

/* C: the two binding helpers, kept as call text (PORTING.md 8: attributes are no-ops).  */
macro_rules! weak_alias {
    ($a:ident, $b:ident) => {};
}
macro_rules! libc_hidden_def {
    ($a:ident) => {};
}

/* C: `__glibc_unlikely`/`__glibc_likely` are branch hints (PORTING.md 2.6).  */
#[inline(always)]
fn __glibc_unlikely<T> (cond: T) -> T { cond }
#[inline(always)]
fn __glibc_likely<T> (cond: T) -> T { cond }

/* C's token-type and bracket-kind enums are in scope at every use site in this file
   (`regexec.c` reads `ANCHOR`, `OP_BACK_REF`, ...); the mirror declares them, and these
   glob imports are the port's spelling of that scope (PORTING.md 0.1-10, the regcomp.rs
   shape).  */
use re_token_type_t::*;
use bracket_elem_type::*;
use re_context_type::*;

/* The functions below are `static` in the *aggregate* translation unit C builds for the
   regex cluster (`lib/regex.c` includes regex_internal.c, regcomp.c and regexec.c into one
   TU, so each file's `static` forward declarations cover the others' definitions).  The
   port keeps one file per unit, so `regex_internal.c`'s definitions are named here the way
   C's prototype block names them; `docs/ports/regexec.md` records the shape (the
   regcomp.rs precedent).  */
unsafe extern "C" {
  /* static reg_errcode_t re_string_allocate (re_string_t *pstr, const char *str,
                                              Idx len, Idx init_len,
                                              RE_TRANSLATE_TYPE trans, bool icase,
                                              const re_dfa_t *dfa);  */
  pub fn re_string_allocate (pstr: *mut re_string_t, str_: *const c_char, len: Idx,
                             init_len: Idx, trans: RE_TRANSLATE_TYPE, icase: bool,
                             dfa: *const re_dfa_t) -> reg_errcode_t;
  /* static reg_errcode_t re_string_realloc_buffers (re_string_t *pstr, Idx new_buf_len);  */
  pub fn re_string_realloc_buffers (pstr: *mut re_string_t, new_buf_len: Idx)
    -> reg_errcode_t;
  /* static reg_errcode_t re_string_reconstruct (re_string_t *pstr, Idx idx, int eflags);  */
  pub fn re_string_reconstruct (pstr: *mut re_string_t, idx: Idx, eflags: c_int)
    -> reg_errcode_t;
  /* static void re_string_destruct (re_string_t *pstr);  */
  pub fn re_string_destruct (pstr: *mut re_string_t);
  /* static unsigned int re_string_context_at (const re_string_t *input, Idx idx,
                                               int eflags);  */
  pub fn re_string_context_at (input: *const re_string_t, idx: Idx, eflags: c_int) -> c_uint;
  /* static reg_errcode_t re_string_translate_buffer (re_string_t *pstr);  */
  pub fn re_string_translate_buffer (pstr: *mut re_string_t) -> reg_errcode_t;
  /* static reg_errcode_t re_node_set_alloc (re_node_set *set, Idx size);  */
  pub fn re_node_set_alloc (set: *mut re_node_set, size: Idx) -> reg_errcode_t;
  /* static reg_errcode_t re_node_set_init_1 (re_node_set *set, Idx elem);  */
  pub fn re_node_set_init_1 (set: *mut re_node_set, elem: Idx) -> reg_errcode_t;
  /* static reg_errcode_t re_node_set_init_union (re_node_set *dest,
                                                   const re_node_set *src1,
                                                   const re_node_set *src2);  */
  pub fn re_node_set_init_union (dest: *mut re_node_set, src1: *const re_node_set,
                                 src2: *const re_node_set) -> reg_errcode_t;
  /* static bool re_node_set_insert (re_node_set *set, Idx elem);  */
  pub fn re_node_set_insert (set: *mut re_node_set, elem: Idx) -> bool;
  /* static reg_errcode_t re_node_set_init_copy (re_node_set *dest,
                                                 const re_node_set *src);  */
  pub fn re_node_set_init_copy (dest: *mut re_node_set, src: *const re_node_set)
    -> reg_errcode_t;
  /* static reg_errcode_t re_node_set_merge (re_node_set *dest, const re_node_set *src);  */
  pub fn re_node_set_merge (dest: *mut re_node_set, src: *const re_node_set)
    -> reg_errcode_t;
  /* static void re_node_set_remove_at (re_node_set *set, Idx idx);  (lib/regex_internal.c 1371)  */
  pub fn re_node_set_remove_at (set: *mut re_node_set, idx: Idx);
  /* static Idx re_node_set_contains (const re_node_set *set, Idx elem);  */
  pub fn re_node_set_contains (set: *const re_node_set, elem: Idx) -> Idx;
  /* static reg_errcode_t re_node_set_add_intersect (re_node_set *dest,
                                                      const re_node_set *src1,
                                                      const re_node_set *src2);  */
  pub fn re_node_set_add_intersect (dest: *mut re_node_set, src1: *const re_node_set,
                                    src2: *const re_node_set) -> reg_errcode_t;
  /* static re_dfastate_t *re_acquire_state (reg_errcode_t *err, const re_dfa_t *dfa,
                                              const re_node_set *nodes);  */
  pub fn re_acquire_state (err: *mut reg_errcode_t, dfa: *const re_dfa_t,
                           nodes: *const re_node_set) -> *mut re_dfastate_t;
  /* static re_dfastate_t *re_acquire_state_context (reg_errcode_t *err,
                                                     const re_dfa_t *dfa,
                                                     const re_node_set *nodes,
                                                     unsigned int context);  */
  pub fn re_acquire_state_context (err: *mut reg_errcode_t, dfa: *const re_dfa_t,
                                   nodes: *const re_node_set, context: c_uint)
    -> *mut re_dfastate_t;
  /* static void free_state (re_dfastate_t *state);  (lib/regex_internal.c 1579)  */
  pub fn free_state (state: *mut re_dfastate_t);
  /* static void build_wcs_buffer (re_string_t *pstr);  (lib/regex_internal.c 180)  */
  pub fn build_wcs_buffer (pstr: *mut re_string_t);
  /* static reg_errcode_t build_wcs_upper_buffer (re_string_t *pstr);
     (lib/regex_internal.c 260)  */
  pub fn build_wcs_upper_buffer (pstr: *mut re_string_t) -> reg_errcode_t;
  /* static void build_upper_buffer (re_string_t *pstr);  (lib/regex_internal.c 563)  */
  pub fn build_upper_buffer (pstr: *mut re_string_t);
  /* int re_compile_fastmap (struct re_pattern_buffer *__buffer);  (regex.h 556;
     regcomp.c 257, whose definition emits the renamed `rpl_re_compile_fastmap`).  */
  #[link_name = "rpl_re_compile_fastmap"]
  pub fn re_compile_fastmap (bufp: *mut re_pattern_buffer) -> c_int;
  /* The `malloc/dynarray-skeleton.c` instantiation's generic helpers (`__libc_dynarray_*`,
     defined by lib/malloc/dynarray_*.c) and the shared header's two structs are the mirror
     `include/dynarray.rs` included above: the port's own `unsafe extern "C"` block does not
     restate them (PORTING.md 0.1-3), and the mirror's declarations carry the renamed
     `#[link_name = "_gl_dynarray_…"]` symbols C's wrapper macros spell (PORTING.md 10.13(1)).  */
}

/* C: the generated <regex.h> renames this definition (`# define regexec rpl_regexec`), and
   the frozen build's lib/regex.o exports the renamed symbol; the port keeps C's source name
   and emits that symbol (PORTING.md 10.13, the regcomp.rs precedent).  */
#[unsafe(export_name = "rpl_regexec")]
pub unsafe extern "C" fn regexec (preg: *const regex_t, string: *const c_char,
                                  nmatch: usize, pmatch: *mut regmatch_t,
                                  eflags: c_int) -> c_int
{
  let mut err: reg_errcode_t;
  let mut start: Idx;
  let mut length: Idx;
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;

  if eflags & !(REG_NOTBOL | REG_NOTEOL | REG_STARTEND) != 0
    { return REG_BADPAT as c_int; }

  if eflags & REG_STARTEND != 0
    {
      start = (*pmatch).rm_so;
      length = (*pmatch).rm_eo;
    }
  else
    {
      start = 0;
      length = strlen (string) as Idx;
    }

  lock_lock! ((*dfa).lock);
  if no_sub_ (preg) != 0
    { err = re_search_internal (preg, string, length, start, length,
                                length, 0, null_mut (), eflags); }
  else
    { err = re_search_internal (preg, string, length, start, length,
                                length, nmatch, pmatch, eflags); }
  lock_unlock! ((*dfa).lock);
  return (err != REG_NOERROR) as c_int;
}
libc_hidden_def! (__regexec);
// C: `#ifdef _LIBC` (regexec.c 219-239) -- the glibc versioned/compat symbols are dead in
// the frozen configuration; only the two `libc_hidden_def`/`weak_alias` call texts above
// and below survive on this target.

/* C: the generated <regex.h> renames this definition (`# define re_match rpl_re_match`).  */
#[unsafe(export_name = "rpl_re_match")]
pub unsafe extern "C" fn re_match (bufp: *mut re_pattern_buffer, string: *const c_char,
                                   length: Idx, start: Idx, regs: *mut re_registers)
                                   -> regoff_t
{
  return re_search_stub (bufp, string, length, start, 0, length, regs, true);
}
// C: `#ifdef _LIBC` (regexec.c 276-278) `weak_alias (__re_match, re_match)` -- dead.

/* C: the generated <regex.h> renames this definition (`# define re_search rpl_re_search`).  */
#[unsafe(export_name = "rpl_re_search")]
pub unsafe extern "C" fn re_search (bufp: *mut re_pattern_buffer, string: *const c_char,
                                    length: Idx, start: Idx, range: regoff_t,
                                    regs: *mut re_registers) -> regoff_t
{
  return re_search_stub (bufp, string, length, start, range, length, regs, false);
}
// C: `#ifdef _LIBC` (regexec.c 287-289) `weak_alias (__re_search, re_search)` -- dead.

/* C: the generated <regex.h> renames this definition (`# define re_match_2 rpl_re_match_2`).  */
#[unsafe(export_name = "rpl_re_match_2")]
pub unsafe extern "C" fn re_match_2 (bufp: *mut re_pattern_buffer, string1: *const c_char,
                                     length1: Idx, string2: *const c_char, length2: Idx,
                                     start: Idx, regs: *mut re_registers, stop: Idx)
                                     -> regoff_t
{
  return re_search_2_stub (bufp, string1, length1, string2, length2, start, 0, regs, stop,
                           true);
}
// C: `#ifdef _LIBC` (regexec.c 299-301) `weak_alias (__re_match_2, re_match_2)` -- dead.

/* C: the generated <regex.h> renames this definition (`# define re_search_2 rpl_re_search_2`).  */
#[unsafe(export_name = "rpl_re_search_2")]
pub unsafe extern "C" fn re_search_2 (bufp: *mut re_pattern_buffer, string1: *const c_char,
                                      length1: Idx, string2: *const c_char, length2: Idx,
                                      start: Idx, range: regoff_t,
                                      regs: *mut re_registers, stop: Idx) -> regoff_t
{
  return re_search_2_stub (bufp, string1, length1, string2, length2, start, range, regs,
                           stop, false);
}
// C: `#ifdef _LIBC` (regexec.c 311-313) `weak_alias (__re_search_2, re_search_2)` -- dead.

unsafe fn re_search_2_stub (bufp: *mut re_pattern_buffer, string1: *const c_char,
                            length1: Idx, string2: *const c_char, length2: Idx, start: Idx,
                            range: regoff_t, regs: *mut re_registers, stop: Idx,
                            ret_len: bool) -> regoff_t
{
  let str_: *const c_char;
  let mut rval: regoff_t;
  let mut len: Idx = 0;
  let mut s: *mut c_char = null_mut ();

  /* C: `ckd_add (&len, length1, length2)`.  */
  let (len_sum, len_overflowed) = length1.overflowing_add (length2);
  len = len_sum;
  if __glibc_unlikely (length1 < 0 || length2 < 0 || stop < 0 || len_overflowed)
    { return -2; }

  /* Concatenate the strings.  */
  if length2 > 0
    {
      if length1 > 0
        {
          s = re_malloc! (c_char, len);

          if __glibc_unlikely (s.is_null ())
            { return -2; }
          // C: `#ifdef _LIBC` (regexec.c 338-343) `memcpy (__mempcpy (...), ...)` -- dead;
          // the live arm is the two-copy concatenation below.
          memcpy (s as *mut c_void, string1 as *const c_void, length1 as usize);
          memcpy (s.add (length1 as usize) as *mut c_void, string2 as *const c_void,
                  length2 as usize);
          str_ = s;
        }
      else
        { str_ = string2; }
    }
  else
    { str_ = string1; }

  rval = re_search_stub (bufp, str_, len, start, range, stop, regs, ret_len);
  re_free! (s);
  return rval;
}

/* The parameters have the same meaning as those of re_search.
   Additional parameters:
   If RET_LEN is true the length of the match is returned (re_match style);
   otherwise the position of the match is returned.  */

unsafe fn re_search_stub (bufp: *mut re_pattern_buffer, string: *const c_char, length: Idx,
                          start: Idx, range: regoff_t, stop: Idx, mut regs: *mut re_registers,
                          ret_len: bool) -> regoff_t
{
  let result: reg_errcode_t;
  let pmatch: *mut regmatch_t;
  let mut nregs: Idx;
  let mut rval: regoff_t;
  let mut eflags: c_int = 0;
  let dfa: *mut re_dfa_t = (*bufp).buffer as *mut re_dfa_t;
  let mut last_start: Idx = start + range;

  /* Check for out-of-range.  */
  if __glibc_unlikely (start < 0 || start > length)
    { return -1; }
  if __glibc_unlikely (length < last_start
                       || (0 <= range && last_start < start))
    { last_start = length; }
  else if __glibc_unlikely (last_start < 0
                            || (range < 0 && start <= last_start))
    { last_start = 0; }

  lock_lock! ((*dfa).lock);

  eflags |= if not_bol_ (bufp) != 0 { REG_NOTBOL } else { 0 };
  eflags |= if not_eol_ (bufp) != 0 { REG_NOTEOL } else { 0 };

  /* Compile fastmap if we haven't yet.  */
  if start < last_start && !(*bufp).fastmap.is_null () && fastmap_accurate_ (bufp) == 0
    { let _ = re_compile_fastmap (bufp); }

  if __glibc_unlikely (no_sub_ (bufp) != 0)
    { regs = null_mut (); }

  /* We need at least 1 register.  */
  if regs.is_null ()
    { nregs = 1; }
  else if __glibc_unlikely (regs_allocated_ (bufp) == REGS_FIXED as c_uint
                            && (*regs).num_regs <= (*bufp).re_nsub)
    {
      nregs = (*regs).num_regs as Idx;
      if __glibc_unlikely (nregs < 1)
        {
          /* Nothing can be copied to regs.  */
          regs = null_mut ();
          nregs = 1;
        }
    }
  else
    { nregs = (*bufp).re_nsub as Idx + 1; }

  'out: {
    pmatch = re_malloc! (regmatch_t, nregs);
    if __glibc_unlikely (pmatch.is_null ())
      {
        rval = -2;
        break 'out;
      }

    result = re_search_internal (bufp, string, length, start, last_start, stop,
                                 nregs as usize, pmatch, eflags);

    rval = 0;

    /* I hope we needn't fill their regs with -1's when no match was found.  */
    if result != REG_NOERROR
      { rval = if result == REG_NOMATCH { -1 } else { -2 }; }
    else if !regs.is_null ()
      {
        /* If caller wants register contents data back, copy them.  */
        set_regs_allocated_ (bufp, re_copy_regs (regs, pmatch, nregs,
                                                  regs_allocated_ (bufp) as c_int));
        if __glibc_unlikely (regs_allocated_ (bufp) == REGS_UNALLOCATED as c_uint)
          { rval = -2; }
      }

    if __glibc_likely (rval == 0)
      {
        if ret_len
          {
            DEBUG_ASSERT! ((*pmatch).rm_so == start);
            rval = (*pmatch).rm_eo - start;
          }
        else
          { rval = (*pmatch).rm_so; }
      }
    re_free! (pmatch);
  }
  lock_unlock! ((*dfa).lock);
  return rval;
}

unsafe fn re_copy_regs (regs: *mut re_registers, pmatch: *mut regmatch_t, nregs: Idx,
                        regs_allocated: c_int) -> c_uint
{
  let mut rval: c_int = REGS_REALLOCATE;
  let mut i: Idx;
  let need_regs: Idx = nregs + 1;
  /* We need one extra element beyond 'num_regs' for the '-1' marker GNU code
     uses.  */

  /* Have the register data arrays been allocated?  */
  if regs_allocated == REGS_UNALLOCATED
    { /* No.  So allocate them with malloc.  */
      (*regs).start = re_malloc! (regoff_t, need_regs);
      if __glibc_unlikely ((*regs).start.is_null ())
        { return REGS_UNALLOCATED as c_uint; }
      (*regs).end = re_malloc! (regoff_t, need_regs);
      if __glibc_unlikely ((*regs).end.is_null ())
        {
          re_free! ((*regs).start);
          return REGS_UNALLOCATED as c_uint;
        }
      (*regs).num_regs = need_regs as __re_size_t;
    }
  else if regs_allocated == REGS_REALLOCATE
    { /* Yes.  If we need more elements than were already
         allocated, reallocate them.  If we need fewer, just
         leave it alone.  */
      if __glibc_unlikely (need_regs as __re_size_t > (*regs).num_regs)
        {
          let new_start: *mut regoff_t =
            re_realloc! ((*regs).start, regoff_t, need_regs);
          let new_end: *mut regoff_t;
          if __glibc_unlikely (new_start.is_null ())
            { return REGS_UNALLOCATED as c_uint; }
          new_end = re_realloc! ((*regs).end, regoff_t, need_regs);
          if __glibc_unlikely (new_end.is_null ())
            {
              re_free! (new_start);
              return REGS_UNALLOCATED as c_uint;
            }
          (*regs).start = new_start;
          (*regs).end = new_end;
          (*regs).num_regs = need_regs as __re_size_t;
        }
    }
  else
    {
      DEBUG_ASSERT! (regs_allocated == REGS_FIXED);
      /* This function may not be called with REGS_FIXED and nregs too big.  */
      DEBUG_ASSERT! (nregs <= (*regs).num_regs as Idx);
      rval = REGS_FIXED;
    }

  /* Copy the regs.  */
  i = 0;
  while i < nregs
    {
      *(*regs).start.add (i as usize) = (*pmatch.add (i as usize)).rm_so;
      *(*regs).end.add (i as usize) = (*pmatch.add (i as usize)).rm_eo;
      i += 1;
    }
  while i < (*regs).num_regs as Idx
    {
      *(*regs).start.add (i as usize) = -1;
      *(*regs).end.add (i as usize) = -1;
      i += 1;
    }

  return rval as c_uint;
}

/* Set REGS to hold NUM_REGS registers, storing them in STARTS and
   ENDS.  Subsequent matches using PATTERN_BUFFER and REGS will use
   this memory for recording register information.  STARTS and ENDS
   must be allocated using the malloc library routine, and must each
   be at least NUM_REGS * sizeof (regoff_t) bytes long.

   If NUM_REGS == 0, then subsequent matches should allocate their own
   register data.

   Unless this function is called, the first search or match using
   PATTERN_BUFFER will allocate its own register data, without
   freeing the old data.  */

/* C: the generated <regex.h> renames this definition
   (`# define re_set_registers rpl_re_set_registers`).  */
#[unsafe(export_name = "rpl_re_set_registers")]
pub unsafe extern "C" fn re_set_registers (bufp: *mut re_pattern_buffer,
                                           regs: *mut re_registers,
                                           num_regs: __re_size_t, starts: *mut regoff_t,
                                           ends: *mut regoff_t)
{
  if num_regs != 0
    {
      set_regs_allocated_ (bufp, REGS_REALLOCATE as c_uint);
      (*regs).num_regs = num_regs;
      (*regs).start = starts;
      (*regs).end = ends;
    }
  else
    {
      set_regs_allocated_ (bufp, REGS_UNALLOCATED as c_uint);
      (*regs).num_regs = 0;
      (*regs).start = null_mut ();
      (*regs).end = null_mut ();
    }
}
// C: `#ifdef _LIBC` (regexec.c 549-551) `weak_alias (__re_set_registers, re_set_registers)` -- dead.

// C: `#if defined _REGEX_RE_COMP || defined _LIBC` (regexec.c 556-565): `re_exec` is dead
// in the frozen configuration (`_REGEX_RE_COMP` and `_LIBC` are both undefined).

/* Internal entry point.  */

/* Searches for a compiled pattern PREG in the string STRING, whose
   length is LENGTH.  NMATCH, PMATCH, and EFLAGS have the same
   meaning as with regexec.  LAST_START is START + RANGE, where
   START and RANGE have the same meaning as with re_search.
   Return REG_NOERROR if we find a match, and REG_NOMATCH if not,
   otherwise return the error code.
   Note: We assume front end functions already check ranges.
   (0 <= LAST_START && LAST_START <= LENGTH)  */

unsafe fn re_search_internal (preg: *const regex_t, string: *const c_char, length: Idx,
                              mut start: Idx, mut last_start: Idx, stop: Idx,
                              mut nmatch: usize, pmatch: *mut regmatch_t, eflags: c_int)
                              -> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let dfa: *const re_dfa_t = (*preg).buffer as *const re_dfa_t;
  let mut left_lim: Idx;
  let mut right_lim: Idx;
  let mut incr: c_int;
  let fl_longest_match: bool;
  let mut match_kind: c_int;
  let mut match_first: Idx;
  let mut match_last: Idx = -1;
  let extra_nmatch: Idx;
  let sb: bool;
  let mut ch: c_int;
  /* C 594: `re_match_context_t mctx = { .dfa = dfa };` zero-initializes the rest
     (PORTING.md 1.5).  */
  let mut mctx: re_match_context_t = std::mem::zeroed ();
  mctx.dfa = dfa;
  let fastmap: *mut c_char = if !(*preg).fastmap.is_null () && fastmap_accurate_ (preg) != 0
                               && start != last_start && can_be_null_ (preg) == 0
                               { (*preg).fastmap } else { null_mut () };
  let t: RE_TRANSLATE_TYPE = (*preg).translate;

  extra_nmatch = if nmatch > (*preg).re_nsub
                   { nmatch as Idx - ((*preg).re_nsub + 1) as Idx } else { 0 };
  nmatch -= extra_nmatch as usize;

  /* Check if the DFA haven't been compiled.  */
  if __glibc_unlikely ((*preg).used == 0 || (*dfa).init_state.is_null ()
                       || (*dfa).init_state_word.is_null ()
                       || (*dfa).init_state_nl.is_null ()
                       || (*dfa).init_state_begbuf.is_null ())
    { return REG_NOMATCH; }

  /* We assume front-end functions already check them.  */
  DEBUG_ASSERT! (0 <= last_start && last_start <= length);

  /* If initial states with non-begbuf contexts have no elements,
     the regex must be anchored.  If preg->newline_anchor is set,
     we'll never use init_state_nl, so do not check it.  */
  if (*(*dfa).init_state).nodes.nelem == 0
      && (*(*dfa).init_state_word).nodes.nelem == 0
      && ((*(*dfa).init_state_nl).nodes.nelem == 0
          || newline_anchor_ (preg) == 0)
    {
      if start != 0 && last_start != 0
        { return REG_NOMATCH; }
      start = 0;
      last_start = 0;
    }

  /* We must check the longest matching, if nmatch > 0.  */
  fl_longest_match = (nmatch != 0 || (*dfa).nbackref != 0);

  let mut save_state_log: *mut *mut re_dfastate_t = null_mut ();

  'free_return: {
  err = re_string_allocate (&mut mctx.input, string, length,
                            (*dfa).nodes_len as Idx + 1,
                            (*preg).translate, (*preg).syntax & RE_ICASE != 0,
                            dfa);
  if __glibc_unlikely (err != REG_NOERROR)
    { break 'free_return; }

  mctx.input.stop = stop;
  mctx.input.raw_stop = stop;
  mctx.input.newline_anchor = newline_anchor_ (preg) as u8;

  err = match_ctx_init (&mut mctx, eflags, (*dfa).nbackref * 2);
  if __glibc_unlikely (err != REG_NOERROR)
    { break 'free_return; }

  /* We will log all the DFA states through which the dfa pass,
     if nmatch > 1, or this dfa has "multibyte node", which is a
     back-reference or a node which can accept multibyte character or
     multi character collating element.  */
  if nmatch > 1 || has_mb_node_ (dfa) != 0
    {
      /* Avoid overflow.  */
      if __glibc_unlikely (MIN! (IDX_MAX,
                                 (SIZE_MAX / std::mem::size_of::<*mut re_dfastate_t> ())
                                   as Idx) <= mctx.input.bufs_len)
        {
          err = REG_ESPACE;
          break 'free_return;
        }

      mctx.state_log = re_malloc! (*mut re_dfastate_t, mctx.input.bufs_len + 1);
      if __glibc_unlikely (mctx.state_log.is_null ())
        {
          err = REG_ESPACE;
          break 'free_return;
        }
    }

  match_first = start;
  mctx.input.tip_context = if eflags & REG_NOTBOL != 0 { CONTEXT_BEGBUF as c_uint }
                           else { (CONTEXT_NEWLINE | CONTEXT_BEGBUF) as c_uint };

  /* Check incrementally whether the input string matches.  */
  incr = if last_start < start { -1 } else { 1 };
  left_lim = if last_start < start { last_start } else { start };
  right_lim = if last_start < start { start } else { last_start };
  sb = (*dfa).mb_cur_max == 1;
  match_kind =
    if !fastmap.is_null ()
      { (if sb || !((*preg).syntax & RE_ICASE != 0 || !t.is_null ()) { 4 } else { 0 })
        | (if start <= last_start { 2 } else { 0 })
        | (if !t.is_null () { 1 } else { 0 }) }
    else
      { 8 };

  loop
    {
      err = REG_NOMATCH;
      if match_first < left_lim || right_lim < match_first
        { break 'free_return; }

      /* Advance as rapidly as possible through the string, until we
	 find a plausible place to start matching.  This may be done
	 with varying efficiency, so there are various possibilities:
	 only the most common of them are specialized, in order to
	 save on code size.  We use a switch statement for speed.  */
      if match_kind == 8
        {
	  /* No fastmap.  */
        }
      else if match_kind == 7 || match_kind == 6
        {
          if match_kind == 7
            {
	      /* Fastmap with single-byte translation, match forward.  */
	      while __glibc_likely (match_first < right_lim)
                && *fastmap.add (if !t.is_null ()
                                   { *t.add (*(string.add (match_first as usize)) as u8
                                             as usize) as usize }
                                   else
                                   { *(string.add (match_first as usize)) as u8
                                     as usize }) == 0
	        { match_first += 1; }
              /* C: `goto forward_match_found_start_or_reached_end` (PORTING.md 2.2):
                 `case 7` jumps into `case 6`'s shared tail, which follows here.  */
            }
          else
            {
	      /* Fastmap without translation, match forward.  */
	      while __glibc_likely (match_first < right_lim)
                && *fastmap.add (*(string.add (match_first as usize)) as u8 as usize) == 0
	        { match_first += 1; }
            }

	  /* C: `forward_match_found_start_or_reached_end:`.  */
	  if __glibc_unlikely (match_first == right_lim)
	    {
	      ch = if match_first >= length
		     { 0 } else { *(string.add (match_first as usize)) as u8 as c_int };
	      if *fastmap.add (if !t.is_null () { *t.add (ch as usize) as usize }
                               else { ch as usize }) == 0
		{ break 'free_return; }
	    }
        }
      else if match_kind == 4 || match_kind == 5
        {
	  /* Fastmap without multi-byte translation, match backwards.  */
	  while match_first >= left_lim
	    {
	      ch = if match_first >= length
		     { 0 } else { *(string.add (match_first as usize)) as u8 as c_int };
	      if *fastmap.add (if !t.is_null () { *t.add (ch as usize) as usize }
                               else { ch as usize }) != 0
		{ break; }
	      match_first -= 1;
	    }
	  if match_first < left_lim
	    { break 'free_return; }
        }
      else
        {
	  /* In this case, we can't determine easily the current byte,
	     since it might be a component byte of a multibyte
	     character.  Then we use the constructed buffer instead.  */
	  loop
	    {
	      /* If MATCH_FIRST is out of the valid range, reconstruct the
		 buffers.  */
	      let mut offset: __re_size_t = (match_first - mctx.input.raw_mbs_idx)
	                                    as __re_size_t;
	      if __glibc_unlikely (offset
	                           >= mctx.input.valid_raw_len as __re_size_t)
		{
		  err = re_string_reconstruct (&mut mctx.input, match_first,
					       eflags);
		  if __glibc_unlikely (err != REG_NOERROR)
		    { break 'free_return; }

		  offset = (match_first - mctx.input.raw_mbs_idx) as __re_size_t;
		}
	      /* Use buffer byte if OFFSET is in buffer, otherwise '\0'.  */
	      ch = if offset < mctx.input.valid_len as __re_size_t
		    { re_string_byte_at! (&mctx.input, offset) as c_int } else { 0 };
	      if *fastmap.add (ch as usize) != 0
		{ break; }
	      match_first += incr as Idx;
	      if match_first < left_lim || match_first > right_lim
		{
		  err = REG_NOMATCH;
		  break 'free_return;
		}
	    }
        }

      /* Reconstruct the buffers so that the matcher can assume that
	 the matching starts from the beginning of the buffer.  */
      err = re_string_reconstruct (&mut mctx.input, match_first, eflags);
      if __glibc_unlikely (err != REG_NOERROR)
	{ break 'free_return; }

      /* Don't consider this char as a possible match start if it part,
         yet isn't the head, of a multibyte character.  */
      if !sb && !re_string_first_byte! (&mctx.input, 0)
	{ match_first += incr as Idx; continue; }

      /* It seems to be appropriate one, then use the matcher.  */
      /* We assume that the matching starts from 0.  */
      mctx.state_log_top = 0;
      mctx.nbkref_ents = 0;
      mctx.max_mb_elem_len = 0;
      match_last = check_matching (&mut mctx, fl_longest_match,
				   if start <= last_start { &mut match_first }
                                   else { null_mut () });
      if match_last != -1
	{
	  if __glibc_unlikely (match_last == -2)
	    {
	      err = REG_ESPACE;
	      break 'free_return;
	    }
	  else
	    {
	      mctx.match_last = match_last;
	      if (no_sub_ (preg) == 0 && nmatch > 1) || (*dfa).nbackref != 0
		{
		  let pstate: *mut re_dfastate_t =
		    *mctx.state_log.add (match_last as usize);
		  mctx.last_node = check_halt_state_context (&mctx, pstate,
							     match_last);
		}
	      if (no_sub_ (preg) == 0 && nmatch > 1 && has_plural_match_ (dfa) != 0)
		  || (*dfa).nbackref != 0
		{
		  /* Save state_log before pruning, in case set_regs
		     later fails and we need to retry with a shorter
		     match.  */
		  re_free! (save_state_log);
		  save_state_log = null_mut ();
		  if no_sub_ (preg) == 0 && nmatch > 1 && (*dfa).nbackref != 0
		    {
		      save_state_log
			= re_malloc! (*mut re_dfastate_t,
				     mctx.match_last + 1);
		      if __glibc_unlikely (save_state_log.is_null ())
			{
			  err = REG_ESPACE;
			  break 'free_return;
			}
		      memcpy (save_state_log as *mut c_void, mctx.state_log as *const c_void,
			      std::mem::size_of::<*mut re_dfastate_t> ()
			      * (mctx.match_last + 1) as usize);
		    }
		  err = prune_impossible_nodes (&mut mctx);
		  if err == REG_NOERROR
		    { break; }
		  if __glibc_unlikely (err != REG_NOMATCH)
		    { break 'free_return; }
		  re_free! (save_state_log);
		  save_state_log = null_mut ();
		  match_last = -1;
		}
	      else
		{ break; } /* We found a match.  */
	    }
	}

      match_ctx_clean (&mut mctx);
      match_first += incr as Idx;
    }

  DEBUG_ASSERT! (match_last != -1);
  DEBUG_ASSERT! (err == REG_NOERROR);

  /* Set pmatch[] if we need.  */
  if nmatch > 0
    {
      let mut reg_idx: Idx;

      /* When set_regs fails for a backref pattern, the structural
	 match at match_last has no valid register assignment.  Try
	 shorter match lengths, since a valid shorter match may
	 exist (e.g., all groups matching empty).  */
      loop
	{
	  /* Initialize registers.  */
	  reg_idx = 1;
	  while reg_idx < nmatch as Idx
	    {
	      (*pmatch.add (reg_idx as usize)).rm_so = -1;
	      (*pmatch.add (reg_idx as usize)).rm_eo = -1;
	      reg_idx += 1;
	    }
	  (*pmatch.add (0)).rm_so = 0;
	  (*pmatch.add (0)).rm_eo = mctx.match_last;

	  if no_sub_ (preg) != 0 || nmatch <= 1
	    { break; }

	  err = set_regs (preg, &mctx, nmatch, pmatch,
			  has_plural_match_ (dfa) != 0 && (*dfa).nbackref > 0);
	  if __glibc_likely (err == REG_NOERROR)
	      || save_state_log.is_null ()
	      || err != REG_NOMATCH
	    { break; }

	  /* set_regs failed; try a shorter match_last.  */
	  let mut ml: Idx = mctx.match_last;
	  re_free! (mctx.state_log);
	  loop
	    {
	      ml -= 1;
	      if ml < 0
		{ break; }
	      if !(*save_state_log.add (ml as usize)).is_null ()
                  && halt_ (*save_state_log.add (ml as usize)) != 0
                  && check_halt_state_context
                       (&mctx, *save_state_log.add (ml as usize), ml) != 0
                { break; }
	    }
	  if ml < 0
	    {
	      err = REG_NOMATCH;
	      mctx.state_log = save_state_log;
	      save_state_log = null_mut ();
	      break;
	    }
	  mctx.state_log
	    = re_malloc! (*mut re_dfastate_t, ml + 1);
	  if __glibc_unlikely (mctx.state_log.is_null ())
	    {
	      mctx.state_log = save_state_log;
	      save_state_log = null_mut ();
	      err = REG_ESPACE;
	      break;
	    }
	  memcpy (mctx.state_log as *mut c_void, save_state_log as *const c_void,
		  std::mem::size_of::<*mut re_dfastate_t> () * (ml + 1) as usize);
	  mctx.match_last = ml;
	  mctx.last_node
	    = check_halt_state_context
		(&mctx, *save_state_log.add (ml as usize), ml);
	  err = prune_impossible_nodes (&mut mctx);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      if err == REG_NOMATCH
		{
		  re_free! (mctx.state_log);
		  mctx.state_log = save_state_log;
		  save_state_log = null_mut ();
		}
	      break;
	    }
	}
      re_free! (save_state_log);
      save_state_log = null_mut ();
      if __glibc_unlikely (err != REG_NOERROR)
	{ break 'free_return; }

      /* At last, add the offset to each register, since we slid
	 the buffers so that we could assume that the matching starts
	 from 0.  */
      reg_idx = 0;
      while reg_idx < nmatch as Idx
	{
	  if (*pmatch.add (reg_idx as usize)).rm_so != -1
	    {
	      if __glibc_unlikely (mctx.input.offsets_needed != 0)
		{
		  (*pmatch.add (reg_idx as usize)).rm_so =
		    if (*pmatch.add (reg_idx as usize)).rm_so == mctx.input.valid_len
		      { mctx.input.valid_raw_len }
		      else
		      { *mctx.input.offsets.add ((*pmatch.add (reg_idx as usize)).rm_so
                                                 as usize) };
		  (*pmatch.add (reg_idx as usize)).rm_eo =
		    if (*pmatch.add (reg_idx as usize)).rm_eo == mctx.input.valid_len
		      { mctx.input.valid_raw_len }
		      else
		      { *mctx.input.offsets.add ((*pmatch.add (reg_idx as usize)).rm_eo
                                                 as usize) };
		}
	      (*pmatch.add (reg_idx as usize)).rm_so += match_first;
	      (*pmatch.add (reg_idx as usize)).rm_eo += match_first;
	    }
	  reg_idx += 1;
	}
      reg_idx = 0;
      while reg_idx < extra_nmatch
	{
	  (*pmatch.add (nmatch + reg_idx as usize)).rm_so = -1;
	  (*pmatch.add (nmatch + reg_idx as usize)).rm_eo = -1;
	  reg_idx += 1;
	}

      if !(*dfa).subexp_map.is_null ()
	{
	  reg_idx = 0;
	  while reg_idx + 1 < nmatch as Idx
	    {
	      if *(*dfa).subexp_map.add (reg_idx as usize) != reg_idx
		{
		  (*pmatch.add (reg_idx as usize + 1)).rm_so
		    = (*pmatch.add ((*(*dfa).subexp_map.add (reg_idx as usize) + 1) as usize)).rm_so;
		  (*pmatch.add (reg_idx as usize + 1)).rm_eo
		    = (*pmatch.add ((*(*dfa).subexp_map.add (reg_idx as usize) + 1) as usize)).rm_eo;
		}
	      reg_idx += 1;
	    }
	}
    }
  } /* 'free_return */

  /* free_return: */
  re_free! (save_state_log);
  re_free! (mctx.state_log);
  if (*dfa).nbackref != 0
    { match_ctx_free (&mut mctx); }
  re_string_destruct (&mut mctx.input);
  return err;
}

unsafe fn prune_impossible_nodes (mctx: *mut re_match_context_t) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut halt_node: Idx;
  let mut match_last: Idx;
  let mut ret: reg_errcode_t;
  let mut sifted_states: *mut *mut re_dfastate_t;
  let mut lim_states: *mut *mut re_dfastate_t = null_mut ();
  let mut sctx: re_sift_context_t = std::mem::zeroed ();
  DEBUG_ASSERT! (!(*mctx).state_log.is_null ());
  match_last = (*mctx).match_last;
  halt_node = (*mctx).last_node;

  /* Avoid overflow.  */
  if __glibc_unlikely (MIN! (IDX_MAX, (SIZE_MAX / std::mem::size_of::<*mut re_dfastate_t> ())
                                          as Idx) <= match_last)
    { return REG_ESPACE; }

  'free_return: {
  sifted_states = re_malloc! (*mut re_dfastate_t, match_last + 1);
  if __glibc_unlikely (sifted_states.is_null ())
    {
      ret = REG_ESPACE;
      break 'free_return;
    }
  if (*dfa).nbackref != 0
    {
      lim_states = re_malloc! (*mut re_dfastate_t, match_last + 1);
      if __glibc_unlikely (lim_states.is_null ())
	{
	  ret = REG_ESPACE;
	  break 'free_return;
	}
      loop
	{
	  memset (lim_states as *mut c_void, b'\0' as c_int,
		  std::mem::size_of::<*mut re_dfastate_t> () * (match_last + 1) as usize);
	  sift_ctx_init (&mut sctx, sifted_states, lim_states, halt_node,
			 match_last);
	  ret = sift_states_backward (mctx, &mut sctx);
	  re_node_set_free! (&mut sctx.limits);
	  if __glibc_unlikely (ret != REG_NOERROR)
	      { break 'free_return; }
	  if !(*sifted_states).is_null () || !(*lim_states).is_null ()
	    { break; }
	  loop
	    {
	      match_last -= 1;
	      if match_last < 0
		{
		  ret = REG_NOMATCH;
		  break 'free_return;
		}
              if !(*(*mctx).state_log.add (match_last as usize)).is_null ()
                  && halt_ (*(*mctx).state_log.add (match_last as usize)) != 0
                {
                  halt_node
                    = check_halt_state_context (mctx,
                                                *(*mctx).state_log.add (match_last as usize),
                                                match_last);
                  if halt_node != 0
                    { break; }
                }
	    }
	}
      ret = merge_state_array (dfa, sifted_states, lim_states,
			       match_last + 1);
      re_free! (lim_states);
      lim_states = null_mut ();
      if __glibc_unlikely (ret != REG_NOERROR)
	{ break 'free_return; }
    }
  else
    {
      sift_ctx_init (&mut sctx, sifted_states, lim_states, halt_node, match_last);
      ret = sift_states_backward (mctx, &mut sctx);
      re_node_set_free! (&mut sctx.limits);
      if __glibc_unlikely (ret != REG_NOERROR)
	{ break 'free_return; }
      if (*sifted_states).is_null ()
	{
	  ret = REG_NOMATCH;
	  break 'free_return;
	}
    }
  re_free! ((*mctx).state_log);
  (*mctx).state_log = sifted_states;
  sifted_states = null_mut ();
  (*mctx).last_node = halt_node;
  (*mctx).match_last = match_last;
  ret = REG_NOERROR;
  } /* 'free_return */

  /* free_return: */
  re_free! (sifted_states);
  re_free! (lim_states);
  return ret;
}

/* Acquire an initial state and return it.
   We must select appropriate initial state depending on the context,
   since initial states may have constraints like "\<", "^", etc..  */

unsafe fn acquire_init_state_context (err: *mut reg_errcode_t,
                                      mctx: *const re_match_context_t, idx: Idx)
                                      -> *mut re_dfastate_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  if has_constraint_ ((*dfa).init_state) != 0
    {
      let mut context: c_uint;
      context = re_string_context_at (&(*mctx).input, idx - 1, (*mctx).eflags);
      if IS_WORD_CONTEXT! (context) != 0
	{ return (*dfa).init_state_word; }
      else if IS_ORDINARY_CONTEXT! (context)
	{ return (*dfa).init_state; }
      else if IS_BEGBUF_CONTEXT! (context) != 0 && IS_NEWLINE_CONTEXT! (context) != 0
	{ return (*dfa).init_state_begbuf; }
      else if IS_NEWLINE_CONTEXT! (context) != 0
	{ return (*dfa).init_state_nl; }
      else if IS_BEGBUF_CONTEXT! (context) != 0
	{
	  /* It is relatively rare case, then calculate on demand.  */
	  return re_acquire_state_context (err, dfa,
					   (*(*dfa).init_state).entrance_nodes,
					   context);
	}
      else
	/* Must not happen?  */
	{ return (*dfa).init_state; }
    }
  else
    { return (*dfa).init_state; }
}

unsafe fn check_matching (mctx: *mut re_match_context_t, fl_longest_match: bool,
                          mut p_match_first: *mut Idx) -> Idx
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t;
  let mut match_: Idx = 0;
  let mut match_last: Idx = -1;
  let cur_str_idx: Idx = re_string_cur_idx! (&(*mctx).input);
  let mut cur_state: *mut re_dfastate_t;
  let mut at_init_state: bool = !p_match_first.is_null ();
  let mut next_start_idx: Idx = cur_str_idx;

  err = REG_NOERROR;
  cur_state = acquire_init_state_context (&mut err, mctx, cur_str_idx);
  /* An initial state must not be NULL (invalid).  */
  if __glibc_unlikely (cur_state.is_null ())
    {
      DEBUG_ASSERT! (err == REG_ESPACE);
      return -2;
    }

  if !(*mctx).state_log.is_null ()
    {
      *(*mctx).state_log.add (cur_str_idx as usize) = cur_state;

      /* Check OP_OPEN_SUBEXP in the initial state in case that we use them
	 later.  E.g. Processing back references.  */
      if __glibc_unlikely ((*dfa).nbackref != 0)
	{
	  at_init_state = false;
	  err = check_subexp_matching_top (mctx, &mut (*cur_state).nodes, 0);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err as Idx; }

	  if has_backref_ (cur_state) != 0
	    {
	      err = transit_state_bkref (mctx, &(*cur_state).nodes);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ return err as Idx; }
	    }
	}
    }

  /* If the RE accepts NULL string.  */
  if __glibc_unlikely (halt_ (cur_state) != 0)
    {
      if has_constraint_ (cur_state) == 0
	  || check_halt_state_context (mctx, cur_state, cur_str_idx) != 0
	{
	  if !fl_longest_match
	    { return cur_str_idx; }
	  else
	    {
	      match_last = cur_str_idx;
	      match_ = 1;
	    }
	}
    }

  while !re_string_eoi! (&(*mctx).input)
    {
      let old_state: *mut re_dfastate_t = cur_state;
      let next_char_idx: Idx = re_string_cur_idx! (&(*mctx).input) + 1;

      if (__glibc_unlikely (next_char_idx >= (*mctx).input.bufs_len)
	   && (*mctx).input.bufs_len < (*mctx).input.len)
	  || (__glibc_unlikely (next_char_idx >= (*mctx).input.valid_len)
	      && (*mctx).input.valid_len < (*mctx).input.len)
	{
	  err = extend_buffers (mctx, (next_char_idx + 1) as c_int);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      DEBUG_ASSERT! (err == REG_ESPACE);
	      return -2;
	    }
	}

      cur_state = transit_state (&mut err, mctx, cur_state);
      if !(*mctx).state_log.is_null ()
	{ cur_state = merge_state_with_log (&mut err, mctx, cur_state); }

      if cur_state.is_null ()
	{
	  /* Reached the invalid state or an error.  Try to recover a valid
	     state using the state log, if available and if we have not
	     already found a valid (even if not the longest) match.  */
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return -2; }

	  if (*mctx).state_log.is_null ()
	      || (match_ != 0 && !fl_longest_match)
	      || ({ cur_state = find_recover_state (&mut err, mctx); cur_state.is_null () })
	    { break; }
	}

      if __glibc_unlikely (at_init_state)
	{
	  if old_state == cur_state
	    { next_start_idx = next_char_idx; }
	  else
	    { at_init_state = false; }
	}

      if halt_ (cur_state) != 0
	{
	  /* Reached a halt state.
	     Check the halt state can satisfy the current context.  */
	  if has_constraint_ (cur_state) == 0
	      || check_halt_state_context (mctx, cur_state,
					   re_string_cur_idx! (&(*mctx).input)) != 0
	    {
	      /* We found an appropriate halt state.  */
	      match_last = re_string_cur_idx! (&(*mctx).input);
	      match_ = 1;

	      /* We found a match, do not modify match_first below.  */
	      p_match_first = null_mut ();
	      if !fl_longest_match
		{ break; }
	    }
	}
    }

  if !p_match_first.is_null ()
    { *p_match_first += next_start_idx; }

  return match_last;
}

/* Check NODE match the current context.  */

unsafe fn check_halt_node_context (dfa: *const re_dfa_t, node: Idx, context: c_uint)
                                   -> bool
{
  let r#type: c_uint = type_ (&*(*dfa).nodes.add (node as usize));
  let constraint: c_uint = constraint_ (&*(*dfa).nodes.add (node as usize));
  if r#type != END_OF_RE as c_uint
    { return false; }
  if constraint == 0
    { return true; }
  if NOT_SATISFY_NEXT_CONSTRAINT! (constraint, context)
    { return false; }
  return true;
}

unsafe fn check_halt_state_context (mctx: *const re_match_context_t,
                                    state: *const re_dfastate_t, idx: Idx) -> Idx
{
  let mut i: Idx;
  let context: c_uint;
  DEBUG_ASSERT! (halt_ (state) != 0);
  context = re_string_context_at (&(*mctx).input, idx, (*mctx).eflags);
  i = 0;
  while i < (*state).nodes.nelem
    {
      if check_halt_node_context ((*mctx).dfa, *(*state).nodes.elems.add (i as usize),
                                  context)
        { return *(*state).nodes.elems.add (i as usize); }
      i += 1;
    }
  return 0;
}

/* Compute the next node to which "NFA" transit from NODE("NFA" is a NFA
   corresponding to the DFA).
   Return the destination node, and update EPS_VIA_NODES;
   return -1 on match failure, -2 on error.  */

unsafe fn proceed_next_node (mctx: *const re_match_context_t, nregs: Idx,
                             regs: *mut regmatch_t, prevregs: *mut regmatch_t,
                             pidx: *mut Idx, node: Idx, eps_via_nodes: *mut re_node_set,
                             fs: *mut re_fail_stack_t) -> Idx
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  if IS_EPSILON_NODE! (type_ (&*(*dfa).nodes.add (node as usize))) != 0
    {
      let cur_nodes: *mut re_node_set =
        &mut (*(*((*mctx).state_log).add (*pidx as usize))).nodes;
      let edests: *mut re_node_set = &mut *(*dfa).edests.add (node as usize);

      if re_node_set_contains (eps_via_nodes, node) == 0
        {
          let ok: bool = re_node_set_insert (eps_via_nodes, node);
          if __glibc_unlikely (! ok)
            { return -2; }
        }

      /* Pick a valid destination, or return -1 if none is found.  */
      let mut dest_node: Idx = -1;
      let mut i: Idx = 0;
      while i < (*edests).nelem
	{
	  let candidate: Idx = *(*edests).elems.add (i as usize);
	  if re_node_set_contains (cur_nodes, candidate) == 0
	    { i += 1; continue; }
          if dest_node == -1
	    { dest_node = candidate; }

	  else
	    {
	      /* In order to avoid infinite loop like "(a*)*", return the second
		 epsilon-transition if the first was already considered.  */
	      if re_node_set_contains (eps_via_nodes, dest_node) != 0
		{ return candidate; }

	      /* Otherwise, push the second epsilon-transition on the fail stack.  */
	      else if !fs.is_null ()
		       && push_fail_stack (fs, *pidx, candidate, nregs, regs,
					   prevregs, eps_via_nodes) != REG_NOERROR
		{ return -2; }

	      /* We know we are going to exit.  */
	      break;
	    }
	  i += 1;
	}
      return dest_node;
    }
  else
    {
      let mut naccepted: Idx = 0;
      let r#type: c_uint = type_ (&*(*dfa).nodes.add (node as usize));

      if accept_mb_ (&*(*dfa).nodes.add (node as usize)) != 0
	{ naccepted = check_node_accept_bytes (dfa, node, &(*mctx).input, *pidx) as Idx; }
      else if r#type == OP_BACK_REF as c_uint
	{
	  let subexp_idx: Idx = (*(*dfa).nodes.add (node as usize)).opr.idx + 1;
	  if subexp_idx < nregs
	    { naccepted = (*regs.add (subexp_idx as usize)).rm_eo
	                  - (*regs.add (subexp_idx as usize)).rm_so; }
	  if !fs.is_null ()
	    {
	      if subexp_idx >= nregs
		  || (*regs.add (subexp_idx as usize)).rm_so == -1
		  || (*regs.add (subexp_idx as usize)).rm_eo == -1
		{ return -1; }
	      else if naccepted != 0
		{
		  let buf: *mut c_char = re_string_get_buffer! (&(*mctx).input) as *mut c_char;
		  if (*mctx).input.valid_len - *pidx < naccepted
		      || (memcmp (buf.add ((*regs.add (subexp_idx as usize)).rm_so as usize)
                                    as *const c_void,
				  buf.add (*pidx as usize) as *const c_void,
				  naccepted as usize)
			  != 0)
		    { return -1; }
		}
	    }

	  if naccepted == 0
	    {
	      let mut dest_node: Idx;
	      let ok: bool = re_node_set_insert (eps_via_nodes, node);
	      if __glibc_unlikely (! ok)
		{ return -2; }
	      dest_node = *(*(*dfa).edests.add (node as usize)).elems;
	      if re_node_set_contains (&mut (*(*((*mctx).state_log).add (*pidx as usize))).nodes,
					dest_node) != 0
		{ return dest_node; }
	    }
	}

      if naccepted != 0
	  || check_node_accept (mctx, (*dfa).nodes.add (node as usize), *pidx)
	{
	  let dest_node: Idx = *(*dfa).nexts.add (node as usize);
	  *pidx = if naccepted == 0 { *pidx + 1 } else { *pidx + naccepted };
	  if !fs.is_null ()
             && (*pidx > (*mctx).match_last
                 || *(*mctx).state_log.add (*pidx as usize) == null_mut ()
		 || re_node_set_contains (&mut (*(*((*mctx).state_log).add (*pidx as usize))).nodes,
					  dest_node) == 0)
	    { return -1; }
	  re_node_set_empty! (eps_via_nodes);
	  return dest_node;
	}
    }
  return -1;
}

unsafe fn push_fail_stack (fs: *mut re_fail_stack_t, str_idx: Idx, dest_node: Idx,
			   nregs: Idx, regs: *mut regmatch_t, prevregs: *mut regmatch_t,
			   eps_via_nodes: *mut re_node_set) -> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let num: Idx = (*fs).num;
  if num == (*fs).alloc
    {
      let new_array: *mut re_fail_stack_ent_t;
      new_array = re_realloc! ((*fs).stack, re_fail_stack_ent_t,
                              (*fs).alloc * 2);
      if new_array.is_null ()
	{ return REG_ESPACE; }
      (*fs).alloc *= 2;
      (*fs).stack = new_array;
    }
  (*((*fs).stack).add (num as usize)).idx = str_idx;
  (*((*fs).stack).add (num as usize)).node = dest_node;
  (*((*fs).stack).add (num as usize)).regs = re_malloc! (regmatch_t, 2 * nregs);
  if (*((*fs).stack).add (num as usize)).regs.is_null ()
    { return REG_ESPACE; }
  (*fs).num = num + 1;
  memcpy ((*((*fs).stack).add (num as usize)).regs as *mut c_void, regs as *const c_void,
          std::mem::size_of::<regmatch_t> () * nregs as usize);
  memcpy ((*((*fs).stack).add (num as usize)).regs.add (nregs as usize) as *mut c_void,
          prevregs as *const c_void, std::mem::size_of::<regmatch_t> () * nregs as usize);
  err = re_node_set_init_copy (&mut (*((*fs).stack).add (num as usize)).eps_via_nodes,
                               eps_via_nodes);
  return err;
}

unsafe fn pop_fail_stack (fs: *mut re_fail_stack_t, pidx: *mut Idx, nregs: Idx,
			  regs: *mut regmatch_t, prevregs: *mut regmatch_t,
			  eps_via_nodes: *mut re_node_set) -> Idx
{
  if fs.is_null () || (*fs).num == 0
    { return -1; }
  let num: Idx = { (*fs).num -= 1; (*fs).num };
  *pidx = (*((*fs).stack).add (num as usize)).idx;
  memcpy (regs as *mut c_void, (*((*fs).stack).add (num as usize)).regs as *const c_void,
          std::mem::size_of::<regmatch_t> () * nregs as usize);
  memcpy (prevregs as *mut c_void,
          (*((*fs).stack).add (num as usize)).regs.add (nregs as usize) as *const c_void,
          std::mem::size_of::<regmatch_t> () * nregs as usize);
  re_node_set_free! (eps_via_nodes);
  re_free! ((*((*fs).stack).add (num as usize)).regs);
  *eps_via_nodes = (*((*fs).stack).add (num as usize)).eps_via_nodes;
  DEBUG_ASSERT! (0 <= (*((*fs).stack).add (num as usize)).node);
  return (*((*fs).stack).add (num as usize)).node;
}

/* C: `#define DYNARRAY_STRUCT  regmatch_list`, `#define DYNARRAY_ELEMENT regmatch_t`,
   `#define DYNARRAY_PREFIX  regmatch_list_` (regexec.c 1438-1440), then
   `#include <malloc/dynarray-skeleton.c>` (regexec.c 1441).  The three bindings stay as the
   macros the ledger compares; `##` pasting has no Rust counterpart, so the template's body -
   `femacs/lib/malloc/dynarray-skeleton.rs`'s `dynarray_skeleton_body!` - is instantiated once
   below with the pasted names C's `DYNARRAY_NAME (…)` would build (PORTING.md 10.13(2), the
   strftime_body!/ftoastr_body! shape).  */
macro_rules! DYNARRAY_STRUCT {
    () => { regmatch_list };
}
macro_rules! DYNARRAY_ELEMENT {
    () => { regmatch_t };
}
macro_rules! DYNARRAY_PREFIX {
    () => { regmatch_list_ };
}

/* C: `#include <malloc/dynarray-skeleton.c>` (regexec.c 1441).  */
include!("malloc/dynarray-skeleton.rs");

/* C 1441's inclusion, expanded for this binding: `DYNARRAY_HAVE_SCRATCH` is 1 and neither
   `DYNARRAY_ELEMENT_FREE`/`DYNARRAY_ELEMENT_INIT` nor `DYNARRAY_FINAL_TYPE` is defined, so
   the skeleton's `scratch` member, its empty `free__elements__`, its `#else` resize arm and
   its `size_t *lengthp` finalize arm are the ones the macro emits; the built object's
   undefineds are the four `_gl_dynarray_*` entries (the reference `regex.o` undefineds
   `_gl_dynarray_resize`).  docs/ports/regexec.md and docs/ports/dynarray-skeleton.md record
   the arms.  */
dynarray_skeleton_body! (
  regmatch_list,                    /* DYNARRAY_STRUCT */
  regmatch_list_u,                  /* the anonymous union */
  regmatch_list_header,             /* the anonymous struct { used; allocated; array; } */
  regmatch_t,                       /* DYNARRAY_ELEMENT */
  DYNARRAY_INITIAL_SIZE! (regmatch_t),
  regmatch_list_free__elements__, regmatch_list_free__array__, regmatch_list_init,
  regmatch_list_free, regmatch_list_has_failed, regmatch_list_mark_failed,
  regmatch_list_size, regmatch_list_at, regmatch_list_begin, regmatch_list_end,
  regmatch_list_add__, regmatch_list_add, regmatch_list_emplace__tail__,
  regmatch_list_emplace__, regmatch_list_emplace, regmatch_list_resize,
  regmatch_list_remove_last, regmatch_list_clear, regmatch_list_finalize);

/* Set the positions where the subexpressions are starts/ends to registers
   PMATCH.
   Note: We assume that pmatch[0] is already set, and
   pmatch[i].rm_so == pmatch[i].rm_eo == -1 for 0 < i < nmatch.  */

unsafe fn set_regs (preg: *const regex_t, mctx: *const re_match_context_t, nmatch: usize,
		    pmatch: *mut regmatch_t, fl_backtrack: bool) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*preg).buffer as *const re_dfa_t;
  let mut idx: Idx;
  let mut cur_node: Idx;
  let mut eps_via_nodes: re_node_set = std::mem::zeroed ();
  let mut fs: *mut re_fail_stack_t;
  let mut fs_body: re_fail_stack_t = re_fail_stack_t { num: 0, alloc: 2, stack: null_mut () };
  let mut prev_match: regmatch_list = std::mem::zeroed ();
  regmatch_list_init (&mut prev_match);

  DEBUG_ASSERT! (nmatch > 1);
  DEBUG_ASSERT! (!(*mctx).state_log.is_null ());
  if fl_backtrack
    {
      fs = &mut fs_body;
      (*fs).stack = re_malloc! (re_fail_stack_ent_t, (*fs).alloc);
      if (*fs).stack.is_null ()
	{ return REG_ESPACE; }
    }
  else
    { fs = null_mut (); }

  cur_node = (*dfa).init_node;
  re_node_set_init_empty! (&raw mut eps_via_nodes);

  if !regmatch_list_resize (&mut prev_match, nmatch)
    {
      regmatch_list_free (&mut prev_match);
      let _ = free_fail_stack_return (fs);
      return REG_ESPACE;
    }
  let prev_idx_match: *mut regmatch_t = regmatch_list_begin (&mut prev_match);
  memcpy (prev_idx_match as *mut c_void, pmatch as *const c_void,
          std::mem::size_of::<regmatch_t> () * nmatch);

  idx = (*pmatch).rm_so;
  while idx <= (*pmatch).rm_eo
    {
      update_regs (dfa, pmatch, prev_idx_match, cur_node, idx, nmatch as Idx);

      if (idx == (*pmatch).rm_eo && cur_node == (*mctx).last_node)
	  || (!fs.is_null () && re_node_set_contains (&eps_via_nodes, cur_node) != 0)
	{
	  let mut reg_idx: Idx;
	  cur_node = -1;
	  if !fs.is_null ()
	    {
	      reg_idx = 0;
	      while reg_idx < nmatch as Idx
		{
		  if (*pmatch.add (reg_idx as usize)).rm_so > -1
		      && (*pmatch.add (reg_idx as usize)).rm_eo == -1
		    {
		      cur_node = pop_fail_stack (fs, &mut idx, nmatch as Idx, pmatch,
						 prev_idx_match, &mut eps_via_nodes);
		      break;
		    }
		  reg_idx += 1;
		}
	    }
	  if cur_node < 0
	    {
	      re_node_set_free! (&mut eps_via_nodes);
	      regmatch_list_free (&mut prev_match);
	      return free_fail_stack_return (fs);
	    }
	}

      /* Proceed to next node.  */
      cur_node = proceed_next_node (mctx, nmatch as Idx, pmatch, prev_idx_match,
				    &mut idx, cur_node,
				    &mut eps_via_nodes, fs);

      if __glibc_unlikely (cur_node < 0)
	{
	  if __glibc_unlikely (cur_node == -2)
	    {
	      re_node_set_free! (&mut eps_via_nodes);
	      regmatch_list_free (&mut prev_match);
	      let _ = free_fail_stack_return (fs);
	      return REG_ESPACE;
	    }
	  cur_node = pop_fail_stack (fs, &mut idx, nmatch as Idx, pmatch,
				     prev_idx_match, &mut eps_via_nodes);
	  if cur_node < 0
	    {
	      re_node_set_free! (&mut eps_via_nodes);
	      regmatch_list_free (&mut prev_match);
	      let _ = free_fail_stack_return (fs);
	      return REG_NOMATCH;
	    }
	}
    }
  re_node_set_free! (&mut eps_via_nodes);
  regmatch_list_free (&mut prev_match);
  return free_fail_stack_return (fs);
}

unsafe fn free_fail_stack_return (fs: *mut re_fail_stack_t) -> reg_errcode_t
{
  if !fs.is_null ()
    {
      let mut fs_idx: Idx;
      fs_idx = 0;
      while fs_idx < (*fs).num
	{
	  re_node_set_free! (&mut (*((*fs).stack).add (fs_idx as usize)).eps_via_nodes);
	  re_free! ((*((*fs).stack).add (fs_idx as usize)).regs);
	  fs_idx += 1;
	}
      re_free! ((*fs).stack);
    }
  return REG_NOERROR;
}

unsafe fn update_regs (dfa: *const re_dfa_t, pmatch: *mut regmatch_t,
		       prev_idx_match: *mut regmatch_t, cur_node: Idx, cur_idx: Idx,
		       nmatch: Idx)
{
  let r#type: c_uint = type_ (&*(*dfa).nodes.add (cur_node as usize));
  if r#type == OP_OPEN_SUBEXP as c_uint
    {
      let reg_num: Idx = (*(*dfa).nodes.add (cur_node as usize)).opr.idx + 1;

      /* We are at the first node of this sub expression.  */
      if reg_num < nmatch
	{
	  (*pmatch.add (reg_num as usize)).rm_so = cur_idx;
	  (*pmatch.add (reg_num as usize)).rm_eo = -1;
	}
    }
  else if r#type == OP_CLOSE_SUBEXP as c_uint
    {
      /* We are at the last node of this sub expression.  */
      let reg_num: Idx = (*(*dfa).nodes.add (cur_node as usize)).opr.idx + 1;
      if reg_num < nmatch
	{
	  if (*pmatch.add (reg_num as usize)).rm_so < cur_idx
	    {
	      (*pmatch.add (reg_num as usize)).rm_eo = cur_idx;
	      /* This is a non-empty match or we are not inside an optional
		 subexpression.  Accept this right away.  */
	      memcpy (prev_idx_match as *mut c_void, pmatch as *const c_void,
                      std::mem::size_of::<regmatch_t> () * nmatch as usize);
	    }
	  else
	    {
	      if opt_subexp_ (&*(*dfa).nodes.add (cur_node as usize)) != 0
		  && (*prev_idx_match.add (reg_num as usize)).rm_so != -1
		/* We transited through an empty match for an optional
		   subexpression, like (a?)*, and this is not the subexp's
		   first match.  Copy back the old content of the registers
		   so that matches of an inner subexpression are undone as
		   well, like in ((a?))*.  */
		{ memcpy (pmatch as *mut c_void, prev_idx_match as *const c_void,
                          std::mem::size_of::<regmatch_t> () * nmatch as usize); }
	      else
		/* We completed a subexpression, but it may be part of
		   an optional one, so do not update PREV_IDX_MATCH.  */
		{ (*pmatch.add (reg_num as usize)).rm_eo = cur_idx; }
	    }
	}
    }
}

/* This function checks the STATE_LOG from the SCTX->last_str_idx to 0
   and sift the nodes in each states according to the following rules.
   Updated state_log will be wrote to STATE_LOG.

   Rules: We throw away the Node 'a' in the STATE_LOG[STR_IDX] if...
     1. When STR_IDX == MATCH_LAST(the last index in the state_log):
	If 'a' isn't the LAST_NODE and 'a' can't epsilon transit to
	the LAST_NODE, we throw away the node 'a'.
     2. When 0 <= STR_IDX < MATCH_LAST and 'a' accepts
	string 's' and transit to 'b':
	i. If 'b' isn't in the STATE_LOG[STR_IDX+strlen('s')], we throw
	   away the node 'a'.
	ii. If 'b' is in the STATE_LOG[STR_IDX+strlen('s')] but 'b' is
	    thrown away, we throw away the node 'a'.
     3. When 0 <= STR_IDX < MATCH_LAST and 'a' epsilon transit to 'b':
	i. If 'b' isn't in the STATE_LOG[STR_IDX], we throw away the
	   node 'a'.
	ii. If 'b' is in the STATE_LOG[STR_IDX] but 'b' is thrown away,
	    we throw away the node 'a'.  */

macro_rules! STATE_NODE_CONTAINS {
    ($state:expr, $node:expr) => {
        (!($state).is_null () && re_node_set_contains (&mut (*$state).nodes, $node) != 0)
    };
}

unsafe fn sift_states_backward (mctx: *const re_match_context_t, sctx: *mut re_sift_context_t)
				-> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let mut null_cnt: c_int = 0;
  let mut str_idx: Idx = (*sctx).last_str_idx;
  let mut cur_dest: re_node_set = std::mem::zeroed ();

  DEBUG_ASSERT! (!(*mctx).state_log.is_null ()
                 && !(*(*mctx).state_log.add (str_idx as usize)).is_null ());

  /* Build sifted state_log[str_idx].  It has the nodes which can epsilon
     transit to the last_node and the last_node itself.  */
  err = re_node_set_init_1 (&mut cur_dest, (*sctx).last_node);
  if __glibc_unlikely (err != REG_NOERROR)
    { return err; }

  'free_return: {
  err = update_cur_sifted_state (mctx, sctx, str_idx, &mut cur_dest);
  if __glibc_unlikely (err != REG_NOERROR)
    { break 'free_return; }

  /* Then check each states in the state_log.  */
  while str_idx > 0
    {
      /* Update counters.  */
      null_cnt = if (*(*sctx).sifted_states.add (str_idx as usize)).is_null ()
                   { null_cnt + 1 } else { 0 };
      if null_cnt > (*mctx).max_mb_elem_len
	{
	  memset ((*sctx).sifted_states as *mut c_void, b'\0' as c_int,
		  std::mem::size_of::<*mut re_dfastate_t> () * str_idx as usize);
	  re_node_set_free! (&mut cur_dest);
	  return REG_NOERROR;
	}
      re_node_set_empty! (&mut cur_dest);
      str_idx -= 1;

      if !(*(*mctx).state_log.add (str_idx as usize)).is_null ()
	{
	  err = build_sifted_states (mctx, sctx, str_idx, &mut cur_dest);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { break 'free_return; }
	}

      /* Add all the nodes which satisfy the following conditions:
	 - It can epsilon transit to a node in CUR_DEST.
	 - It is in CUR_SRC.
	 And update state_log.  */
      err = update_cur_sifted_state (mctx, sctx, str_idx, &mut cur_dest);
      if __glibc_unlikely (err != REG_NOERROR)
	{ break 'free_return; }
    }
  err = REG_NOERROR;
  } /* 'free_return */

  /* free_return: */
  re_node_set_free! (&mut cur_dest);
  return err;
}

unsafe fn build_sifted_states (mctx: *const re_match_context_t, sctx: *mut re_sift_context_t,
			       str_idx: Idx, cur_dest: *mut re_node_set) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let cur_src: *const re_node_set =
    &(*(*((*mctx).state_log).add (str_idx as usize))).non_eps_nodes;
  let mut i: Idx;

  /* Then build the next sifted state.
     We build the next sifted state on 'cur_dest', and update
     'sifted_states[str_idx]' with 'cur_dest'.
     Note:
     'cur_dest' is the sifted state from 'state_log[str_idx + 1]'.
     'cur_src' points the node_set of the old 'state_log[str_idx]'
     (with the epsilon nodes pre-filtered out).  */
  i = 0;
  while i < (*cur_src).nelem
    {
      let prev_node: Idx = *(*cur_src).elems.add (i as usize);
      let mut naccepted: c_int = 0;
      let ok: bool;
      DEBUG_ASSERT! (IS_EPSILON_NODE! (type_ (&*(*dfa).nodes.add (prev_node as usize))) == 0);

      /* If the node may accept "multi byte".  */
      if accept_mb_ (&*(*dfa).nodes.add (prev_node as usize)) != 0
	{ naccepted = sift_states_iter_mb (mctx, sctx, prev_node,
					   str_idx, (*sctx).last_str_idx); }

      /* We don't check backreferences here.
	 See update_cur_sifted_state().  */
      if naccepted == 0
	  && check_node_accept (mctx, (*dfa).nodes.add (prev_node as usize), str_idx)
	  && STATE_NODE_CONTAINS! (*(*sctx).sifted_states.add ((str_idx + 1) as usize),
				   *(*dfa).nexts.add (prev_node as usize))
	{ naccepted = 1; }

      if naccepted == 0
	{ i += 1; continue; }

      if (*sctx).limits.nelem != 0
	{
	  let to_idx: Idx = str_idx + naccepted as Idx;
	  if check_dst_limits (mctx, &(*sctx).limits,
			       *(*dfa).nexts.add (prev_node as usize), to_idx,
			       prev_node, str_idx)
	    { i += 1; continue; }
	}
      ok = re_node_set_insert (cur_dest, prev_node);
      if __glibc_unlikely (! ok)
	{ return REG_ESPACE; }
      i += 1;
    }

  return REG_NOERROR;
}

unsafe fn clean_state_log_if_needed (mctx: *mut re_match_context_t, next_state_log_idx: Idx)
				     -> reg_errcode_t
{
  let top: Idx = (*mctx).state_log_top;

  if (next_state_log_idx >= (*mctx).input.bufs_len
       && (*mctx).input.bufs_len < (*mctx).input.len)
      || (next_state_log_idx >= (*mctx).input.valid_len
	  && (*mctx).input.valid_len < (*mctx).input.len)
    {
      let mut err: reg_errcode_t;
      err = extend_buffers (mctx, (next_state_log_idx + 1) as c_int);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }
    }

  if top < next_state_log_idx
    {
      DEBUG_ASSERT! (!(*mctx).state_log.is_null ());
      memset ((*mctx).state_log.add ((top + 1) as usize) as *mut c_void, b'\0' as c_int,
	      std::mem::size_of::<*mut re_dfastate_t> ()
              * (next_state_log_idx - top) as usize);
      (*mctx).state_log_top = next_state_log_idx;
    }
  return REG_NOERROR;
}

unsafe fn merge_state_array (dfa: *const re_dfa_t, dst: *mut *mut re_dfastate_t,
			     src: *mut *mut re_dfastate_t, num: Idx) -> reg_errcode_t
{
  let mut st_idx: Idx;
  let mut err: reg_errcode_t;
  st_idx = 0;
  while st_idx < num
    {
      if (*dst.add (st_idx as usize)).is_null ()
	{ *dst.add (st_idx as usize) = *src.add (st_idx as usize); }
      else if !(*src.add (st_idx as usize)).is_null ()
	{
	  let mut merged_set: re_node_set = std::mem::zeroed ();
	  err = re_node_set_init_union (&mut merged_set,
                                        &(*(*dst.add (st_idx as usize))).nodes,
					&(*(*src.add (st_idx as usize))).nodes);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	  *dst.add (st_idx as usize) = re_acquire_state (&mut err, dfa, &merged_set);
	  re_node_set_free! (&mut merged_set);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	}
      st_idx += 1;
    }
  return REG_NOERROR;
}

unsafe fn update_cur_sifted_state (mctx: *const re_match_context_t, sctx: *mut re_sift_context_t,
				   str_idx: Idx, dest_nodes: *mut re_node_set)
				   -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t = REG_NOERROR;
  let candidates: *const re_node_set;
  candidates = if (*(*mctx).state_log.add (str_idx as usize)).is_null () { null_mut () }
	       else { &(*(*(*mctx).state_log.add (str_idx as usize))).nodes };

  if (*dest_nodes).nelem == 0
    { *(*sctx).sifted_states.add (str_idx as usize) = null_mut (); }
  else
    {
      if !candidates.is_null ()
	{
	  /* At first, add the nodes which can epsilon transit to a node in
	     DEST_NODE.  */
	  err = add_epsilon_src_nodes (dfa, dest_nodes, candidates);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }

	  /* Then, check the limitations in the current sift_context.  */
	  if (*sctx).limits.nelem != 0
	    {
	      err = check_subexp_limits (dfa, dest_nodes, candidates, &mut (*sctx).limits,
					 (*mctx).bkref_ents, str_idx);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ return err; }
	    }
	}

      *(*sctx).sifted_states.add (str_idx as usize) = re_acquire_state (&mut err, dfa, dest_nodes);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }
    }

  if !candidates.is_null () && has_backref_ (&*(*(*mctx).state_log.add (str_idx as usize))) != 0
    {
      err = sift_states_bkref (mctx, sctx, str_idx, candidates);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }
    }
  return REG_NOERROR;
}

unsafe fn add_epsilon_src_nodes (dfa: *const re_dfa_t, dest_nodes: *mut re_node_set,
				 candidates: *const re_node_set) -> reg_errcode_t
{
  let mut err: reg_errcode_t = REG_NOERROR;
  let mut i: Idx;

  let state: *mut re_dfastate_t = re_acquire_state (&mut err, dfa, dest_nodes);
  if __glibc_unlikely (err != REG_NOERROR)
    { return err; }

  if (*state).inveclosure.alloc == 0
    {
      err = re_node_set_alloc (&mut (*state).inveclosure, (*dest_nodes).nelem);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return REG_ESPACE; }
      i = 0;
      while i < (*dest_nodes).nelem
	{
	  err = re_node_set_merge (&mut (*state).inveclosure,
				   (*dfa).inveclosures.add
                                     (*(*dest_nodes).elems.add (i as usize) as usize));
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return REG_ESPACE; }
	  i += 1;
	}
    }
  return re_node_set_add_intersect (dest_nodes, candidates,
				    &mut (*state).inveclosure);
}

unsafe fn sub_epsilon_src_nodes (dfa: *const re_dfa_t, node: Idx, dest_nodes: *mut re_node_set,
				 candidates: *const re_node_set) -> reg_errcode_t
{
    let mut ecl_idx: Idx;
    let mut err: reg_errcode_t;
    let inv_eclosure: *mut re_node_set = (*dfa).inveclosures.add (node as usize);
    let mut except_nodes: re_node_set = std::mem::zeroed ();
    re_node_set_init_empty! (&raw mut except_nodes);
    ecl_idx = 0;
    while ecl_idx < (*inv_eclosure).nelem
      {
	let cur_node: Idx = *(*inv_eclosure).elems.add (ecl_idx as usize);
	if cur_node == node
	  { ecl_idx += 1; continue; }
	if IS_EPSILON_NODE! (type_ (&*(*dfa).nodes.add (cur_node as usize))) != 0
	  {
	    let edst1: Idx = *(*(*dfa).edests.add (cur_node as usize)).elems;
	    let edst2: Idx = if (*(*dfa).edests.add (cur_node as usize)).nelem > 1
			       { *(*(*dfa).edests.add (cur_node as usize)).elems.add (1) } else { -1 };
	    if (re_node_set_contains (inv_eclosure, edst1) == 0
		 && re_node_set_contains (dest_nodes, edst1) != 0)
		|| (edst2 > 0
		    && re_node_set_contains (inv_eclosure, edst2) == 0
		    && re_node_set_contains (dest_nodes, edst2) != 0)
	      {
		err = re_node_set_add_intersect (&mut except_nodes, candidates,
						 (*dfa).inveclosures.add (cur_node as usize));
		if __glibc_unlikely (err != REG_NOERROR)
		  {
		    re_node_set_free! (&mut except_nodes);
		    return err;
		  }
	      }
	  }
	ecl_idx += 1;
      }
    ecl_idx = 0;
    while ecl_idx < (*inv_eclosure).nelem
      {
	let cur_node: Idx = *(*inv_eclosure).elems.add (ecl_idx as usize);
	if re_node_set_contains (&except_nodes, cur_node) == 0
	  {
	    let idx: Idx = re_node_set_contains (dest_nodes, cur_node) - 1;
	    re_node_set_remove_at (dest_nodes, idx);
	  }
	ecl_idx += 1;
      }
    re_node_set_free! (&mut except_nodes);
    return REG_NOERROR;
}

unsafe fn check_dst_limits (mctx: *const re_match_context_t, limits: *const re_node_set,
			    dst_node: Idx, dst_idx: Idx, src_node: Idx, src_idx: Idx)
			    -> bool
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut lim_idx: Idx;
  let mut src_pos: c_int;
  let mut dst_pos: c_int;

  let dst_bkref_idx: Idx = search_cur_bkref_entry (mctx, dst_idx);
  let src_bkref_idx: Idx = search_cur_bkref_entry (mctx, src_idx);
  lim_idx = 0;
  while lim_idx < (*limits).nelem
    {
      let subexp_idx: Idx;
      let ent: *mut re_backref_cache_entry;
      ent = (*mctx).bkref_ents.add (*(*limits).elems.add (lim_idx as usize) as usize);
      subexp_idx = (*(*dfa).nodes.add ((*ent).node as usize)).opr.idx;

      dst_pos = check_dst_limits_calc_pos (mctx, *(*limits).elems.add (lim_idx as usize),
					   subexp_idx, dst_node, dst_idx,
					   dst_bkref_idx);
      src_pos = check_dst_limits_calc_pos (mctx, *(*limits).elems.add (lim_idx as usize),
					   subexp_idx, src_node, src_idx,
					   src_bkref_idx);

      /* In case of:
	 <src> <dst> ( <subexp> )
	 ( <subexp> ) <src> <dst>
	 ( <subexp1> <src> <subexp2> <dst> <subexp3> )  */
      if src_pos == dst_pos
	{ lim_idx += 1; continue; } /* This is unrelated limitation.  */
      else
	{ return true; }
    }
  return false;
}

unsafe fn check_dst_limits_calc_pos_1 (mctx: *const re_match_context_t, boundaries: c_int,
				       subexp_idx: Idx, from_node: Idx, bkref_idx: Idx)
				       -> c_int
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let eclosures: *const re_node_set = (*dfa).eclosures.add (from_node as usize);
  let mut node_idx: Idx;

  /* Else, we are on the boundary: examine the nodes on the epsilon
     closure.  */
  node_idx = 0;
  while node_idx < (*eclosures).nelem
    {
      let node: Idx = *(*eclosures).elems.add (node_idx as usize);
      let r#type: c_uint = type_ (&*(*dfa).nodes.add (node as usize));
      if r#type == OP_BACK_REF as c_uint
	{
	  if bkref_idx != -1
	    {
	      let mut ent: *mut re_backref_cache_entry =
		(*mctx).bkref_ents.add (bkref_idx as usize);
	      loop
		{
		  let mut dst: Idx;
		  let mut cpos: c_int;

		  'do_body: {
		  if (*ent).node != node
		    { break 'do_body; }   /* C: `continue` -> the `ent++->more` test.  */

		  if subexp_idx < BITSET_WORD_BITS
		      && ((*ent).eps_reachable_subexps_map
			  & ((1 as bitset_word_t) << subexp_idx)) == 0
		    { break 'do_body; }   /* C: `continue` -> the test.  */

		  /* Recurse trying to reach the OP_OPEN_SUBEXP and
		     OP_CLOSE_SUBEXP cases below.  But, if the
		     destination node is the same node as the source
		     node, don't recurse because it would cause an
		     infinite loop: a regex that exhibits this behavior
		     is ()\1*\1*  */
		  dst = *(*(*dfa).edests.add (node as usize)).elems;
		  if dst == from_node
		    {
		      if boundaries & 1 != 0
			{ return -1; }
		      else /* if (boundaries & 2) */
			{ return 0; }
		    }

		  cpos =
		    check_dst_limits_calc_pos_1 (mctx, boundaries, subexp_idx,
						 dst, bkref_idx);
		  if cpos == -1 /* && (boundaries & 1) */
		    { return -1; }
		  if cpos == 0 && boundaries & 2 != 0
		    { return 0; }

		  if subexp_idx < BITSET_WORD_BITS
		    { (*ent).eps_reachable_subexps_map
			&= !((1 as bitset_word_t) << subexp_idx); }
		  }
		  /* C: `while (ent++->more)` -- read `more`, then advance `ent`.  */
		  let more: c_char = (*ent).more;
		  ent = ent.add (1);
		  if more == 0
		    { break; }
		}
	    }
	}
      else if r#type == OP_OPEN_SUBEXP as c_uint
	{
	  if boundaries & 1 != 0 && subexp_idx == (*(*dfa).nodes.add (node as usize)).opr.idx
	    { return -1; }
	}
      else if r#type == OP_CLOSE_SUBEXP as c_uint
	{
	  if boundaries & 2 != 0 && subexp_idx == (*(*dfa).nodes.add (node as usize)).opr.idx
	    { return 0; }
	}
      else
	{
	}
      node_idx += 1;
    }

  return if boundaries & 2 != 0 { 1 } else { 0 };
}

unsafe fn check_dst_limits_calc_pos (mctx: *const re_match_context_t, limit: Idx,
				     subexp_idx: Idx, from_node: Idx, str_idx: Idx,
				     bkref_idx: Idx) -> c_int
{
  let lim: *mut re_backref_cache_entry = (*mctx).bkref_ents.add (limit as usize);
  let mut boundaries: c_int;

  /* If we are outside the range of the subexpression, return -1 or 1.  */
  if str_idx < (*lim).subexp_from
    { return -1; }

  if (*lim).subexp_to < str_idx
    { return 1; }

  /* If we are within the subexpression, return 0.  */
  boundaries = (str_idx == (*lim).subexp_from) as c_int;
  boundaries |= ((str_idx == (*lim).subexp_to) as c_int) << 1;
  if boundaries == 0
    { return 0; }

  /* Else, examine epsilon closure.  */
  return check_dst_limits_calc_pos_1 (mctx, boundaries, subexp_idx,
				      from_node, bkref_idx);
}

/* Check the limitations of sub expressions LIMITS, and remove the nodes
   which are against limitations from DEST_NODES. */

unsafe fn check_subexp_limits (dfa: *const re_dfa_t, dest_nodes: *mut re_node_set,
			       candidates: *const re_node_set, limits: *mut re_node_set,
			       bkref_ents: *mut re_backref_cache_entry, str_idx: Idx)
			       -> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let mut node_idx: Idx;
  let mut lim_idx: Idx;

  lim_idx = 0;
  while lim_idx < (*limits).nelem
    {
      let subexp_idx: Idx;
      let ent: *mut re_backref_cache_entry;
      ent = bkref_ents.add (*(*limits).elems.add (lim_idx as usize) as usize);

      if str_idx <= (*ent).subexp_from || (*ent).str_idx < str_idx
	{ lim_idx += 1; continue; } /* This is unrelated limitation.  */

      subexp_idx = (*(*dfa).nodes.add ((*ent).node as usize)).opr.idx;
      if (*ent).subexp_to == str_idx
	{
	  let mut ops_node: Idx = -1;
	  let mut cls_node: Idx = -1;
	  node_idx = 0;
	  while node_idx < (*dest_nodes).nelem
	    {
	      let node: Idx = *(*dest_nodes).elems.add (node_idx as usize);
	      let r#type: c_uint = type_ (&*(*dfa).nodes.add (node as usize));
	      if r#type == OP_OPEN_SUBEXP as c_uint
		  && subexp_idx == (*(*dfa).nodes.add (node as usize)).opr.idx
		{ ops_node = node; }
	      else if r#type == OP_CLOSE_SUBEXP as c_uint
		       && subexp_idx == (*(*dfa).nodes.add (node as usize)).opr.idx
		{ cls_node = node; }
	      node_idx += 1;
	    }

	  /* Check the limitation of the open subexpression.  */
	  /* Note that (ent->subexp_to = str_idx != ent->subexp_from).  */
	  if ops_node >= 0
	    {
	      err = sub_epsilon_src_nodes (dfa, ops_node, dest_nodes,
					   candidates);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ return err; }
	    }

	  /* Check the limitation of the close subexpression.  */
	  if cls_node >= 0
	    {
	    node_idx = 0;
	    while node_idx < (*dest_nodes).nelem
	      {
		let node: Idx = *(*dest_nodes).elems.add (node_idx as usize);
		if re_node_set_contains ((*dfa).inveclosures.add (node as usize),
					 cls_node) == 0
		    && re_node_set_contains ((*dfa).eclosures.add (node as usize),
					      cls_node) == 0
		  {
		    /* It is against this limitation.
		       Remove it form the current sifted state.  */
		    err = sub_epsilon_src_nodes (dfa, node, dest_nodes,
						 candidates);
		    if __glibc_unlikely (err != REG_NOERROR)
		      { return err; }
		    node_idx -= 1;
		  }
		node_idx += 1;
	      }
	    }
	}
      else /* (ent->subexp_to != str_idx)  */
	{
	  node_idx = 0;
	  while node_idx < (*dest_nodes).nelem
	    {
	      let node: Idx = *(*dest_nodes).elems.add (node_idx as usize);
	      let r#type: c_uint = type_ (&*(*dfa).nodes.add (node as usize));
	      if r#type == OP_CLOSE_SUBEXP as c_uint || r#type == OP_OPEN_SUBEXP as c_uint
		{
		  if subexp_idx != (*(*dfa).nodes.add (node as usize)).opr.idx
		    { node_idx += 1; continue; }
		  /* It is against this limitation.
		     Remove it form the current sifted state.  */
		  err = sub_epsilon_src_nodes (dfa, node, dest_nodes,
					       candidates);
		  if __glibc_unlikely (err != REG_NOERROR)
		    { return err; }
		}
	      node_idx += 1;
	    }
	}
      lim_idx += 1;
    }
  return REG_NOERROR;
}

unsafe fn sift_states_bkref (mctx: *const re_match_context_t, sctx: *mut re_sift_context_t,
			     str_idx: Idx, candidates: *const re_node_set) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t;
  let mut node_idx: Idx;
  let mut node: Idx;
  let mut local_sctx: re_sift_context_t = std::mem::zeroed ();
  let first_idx: Idx = search_cur_bkref_entry (mctx, str_idx);

  if first_idx == -1
    { return REG_NOERROR; }

  local_sctx.sifted_states = null_mut (); /* Mark that it hasn't been initialized.  */

  'free_return: {
  node_idx = 0;
  while node_idx < (*candidates).nelem
    {
      let mut enabled_idx: Idx;
      let r#type: c_uint;
      let mut entry: *mut re_backref_cache_entry;
      node = *(*candidates).elems.add (node_idx as usize);
      r#type = type_ (&*(*dfa).nodes.add (node as usize));
      /* Avoid infinite loop for the REs like "()\1+".  */
      if node == (*sctx).last_node && str_idx == (*sctx).last_str_idx
	{ node_idx += 1; continue; }
      if r#type != OP_BACK_REF as c_uint
	{ node_idx += 1; continue; }

      entry = (*mctx).bkref_ents.add (first_idx as usize);
      enabled_idx = first_idx;
      loop
	{
	  let mut subexp_len: Idx;
	  let mut to_idx: Idx;
	  let mut dst_node: Idx;
	  let ok: bool;
	  let mut cur_state: *mut re_dfastate_t;

	  'do_body: {
	  if (*entry).node != node
	    { break 'do_body; }   /* C: `continue` -> the loop test below.  */
	  subexp_len = (*entry).subexp_to - (*entry).subexp_from;
	  to_idx = str_idx + subexp_len;
	  dst_node = if subexp_len != 0 { *(*dfa).nexts.add (node as usize) }
		     else { *(*(*dfa).edests.add (node as usize)).elems };

	  if to_idx > (*sctx).last_str_idx
	      || *(*sctx).sifted_states.add (to_idx as usize) == null_mut ()
	      || !STATE_NODE_CONTAINS! (*(*sctx).sifted_states.add (to_idx as usize), dst_node)
	      || check_dst_limits (mctx, &(*sctx).limits, node,
				   str_idx, dst_node, to_idx)
	    { break 'do_body; }   /* C: `continue` -> the loop test below.  */

	  if local_sctx.sifted_states.is_null ()
	    {
	      /* C: `local_sctx = *sctx;` (a struct copy).  */
	      local_sctx = std::ptr::read (sctx);
	      err = re_node_set_init_copy (&mut local_sctx.limits, &(*sctx).limits);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break 'free_return; }
	    }
	  local_sctx.last_node = node;
	  local_sctx.last_str_idx = str_idx;
	  ok = re_node_set_insert (&mut local_sctx.limits, enabled_idx);
	  if __glibc_unlikely (! ok)
	    {
	      err = REG_ESPACE;
	      break 'free_return;
	    }
	  cur_state = *local_sctx.sifted_states.add (str_idx as usize);
	  err = sift_states_backward (mctx, &mut local_sctx);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { break 'free_return; }
	  if (*sctx).limited_states != null_mut ()
	    {
	      err = merge_state_array (dfa, (*sctx).limited_states,
				       local_sctx.sifted_states,
				       str_idx + 1);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break 'free_return; }
	    }
	  *local_sctx.sifted_states.add (str_idx as usize) = cur_state;
	  re_node_set_remove! (&mut local_sctx.limits, enabled_idx);

	  /* mctx->bkref_ents may have changed, reload the pointer.  */
	  entry = (*mctx).bkref_ents.add (enabled_idx as usize);
	  }
	  /* C: `while (enabled_idx++, entry++->more);` */
	  enabled_idx += 1;
	  let more: c_char = (*entry).more;
	  entry = entry.add (1);
	  if more == 0
	    { break; }
	}
      node_idx += 1;
    }
  err = REG_NOERROR;
  } /* 'free_return */

  /* free_return: */
  if !local_sctx.sifted_states.is_null ()
    {
      re_node_set_free! (&mut local_sctx.limits);
    }

  return err;
}

unsafe fn sift_states_iter_mb (mctx: *const re_match_context_t, sctx: *mut re_sift_context_t,
			       node_idx: Idx, str_idx: Idx, max_str_idx: Idx) -> c_int
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut naccepted: c_int;
  /* Check the node can accept "multi byte".  */
  naccepted = check_node_accept_bytes (dfa, node_idx, &(*mctx).input, str_idx);
  if naccepted > 0 && str_idx + naccepted as Idx <= max_str_idx
      && !STATE_NODE_CONTAINS! (*(*sctx).sifted_states.add ((str_idx + naccepted as Idx) as usize),
			       *(*dfa).nexts.add (node_idx as usize))
    /* The node can't accept the "multi byte", or the
       destination was already thrown away, then the node
       couldn't accept the current input "multi byte".   */
    { naccepted = 0; }
  /* Otherwise, it is sure that the node could accept
     'naccepted' bytes input.  */
  return naccepted;
}

/* Functions for state transition.  */

/* Return the next state to which the current state STATE will transit by
   accepting the current input byte, and update STATE_LOG if necessary.
   Return NULL on failure.
   If STATE can accept a multibyte char/collating element/back reference
   update the destination of STATE_LOG.  */

unsafe fn transit_state (err: *mut reg_errcode_t, mctx: *mut re_match_context_t,
			 state: *mut re_dfastate_t) -> *mut re_dfastate_t
{
  let mut trtable: *mut *mut re_dfastate_t;
  let ch: u8;

  /* If the current state can accept multibyte.  */
  if __glibc_unlikely (state_accept_mb_ (state) != 0)
    {
      *err = transit_state_mb (mctx, state);
      if __glibc_unlikely (*err != REG_NOERROR)
	{ return null_mut (); }
    }

  /* Then decide the next state with the single byte.  */
  // C: `#if 0` (regexec.c 2293-2297) -- the `transit_state_sb` arm is dead.

  /* Use transition table  */
  ch = re_string_fetch_byte! (&mut (*mctx).input);
  loop
    {
      trtable = (*state).trtable;
      if __glibc_likely (!trtable.is_null ())
	{ return *trtable.add (ch as usize); }

      trtable = (*state).word_trtable;
      if __glibc_likely (!trtable.is_null ())
	{
	  let context: c_uint;
	  context
	    = re_string_context_at (&mut (*mctx).input,
				    re_string_cur_idx! (&(*mctx).input) - 1,
				    (*mctx).eflags);
	  if IS_WORD_CONTEXT! (context) != 0
	    { return *trtable.add ((ch as Idx + SBC_MAX) as usize); }
	  else
	    { return *trtable.add (ch as usize); }
	}

      if !build_trtable ((*mctx).dfa, state)
	{
	  *err = REG_ESPACE;
	  return null_mut ();
	}

      /* Retry, we now have a transition table.  */
    }
}

/* Update the state_log if we need */
unsafe fn merge_state_with_log (err: *mut reg_errcode_t, mctx: *mut re_match_context_t,
				next_state_in: *mut re_dfastate_t) -> *mut re_dfastate_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let cur_idx: Idx = re_string_cur_idx! (&(*mctx).input);
  let mut next_state: *mut re_dfastate_t = next_state_in;

  if cur_idx > (*mctx).state_log_top
    {
      *(*mctx).state_log.add (cur_idx as usize) = next_state;
      (*mctx).state_log_top = cur_idx;
    }
  else if (*(*mctx).state_log.add (cur_idx as usize)).is_null ()
    {
      *(*mctx).state_log.add (cur_idx as usize) = next_state;
    }
  else
    {
      let mut pstate: *mut re_dfastate_t;
      let mut context: c_uint;
      let mut next_nodes: re_node_set = std::mem::zeroed ();
      let mut log_nodes: *mut re_node_set;
      let mut table_nodes: *mut re_node_set = null_mut ();
      /* If (state_log[cur_idx] != 0), it implies that cur_idx is
	 the destination of a multibyte char/collating element/
	 back reference.  Then the next state is the union set of
	 these destinations and the results of the transition table.  */
      pstate = *(*mctx).state_log.add (cur_idx as usize);
      log_nodes = (*pstate).entrance_nodes;
      if !next_state.is_null () && !(*next_state).entrance_nodes.is_null ()
	{
	  table_nodes = (*next_state).entrance_nodes;
	  *err = re_node_set_init_union (&mut next_nodes, table_nodes,
					 log_nodes);
	  if __glibc_unlikely (*err != REG_NOERROR)
	    { return null_mut (); }
	}
      else
	{/* C: `next_nodes = *log_nodes;` (a struct copy).  */
	 next_nodes = std::ptr::read (log_nodes); }
      /* Note: We already add the nodes of the initial state,
	 then we don't need to add them here.  */

      context = re_string_context_at (&mut (*mctx).input,
				      re_string_cur_idx! (&(*mctx).input) - 1,
				      (*mctx).eflags);
      /* C: `next_state = mctx->state_log[cur_idx] = re_acquire_state_context (...);`  */
      let acquired: *mut re_dfastate_t =
        re_acquire_state_context (err, dfa, &next_nodes, context);
      *(*mctx).state_log.add (cur_idx as usize) = acquired;
      next_state = acquired;
      /* We don't need to check errors here, since the return value of
	 this function is next_state and ERR is already set.  */

      if !table_nodes.is_null ()
	{ re_node_set_free! (&mut next_nodes); }
    }

  if __glibc_unlikely ((*dfa).nbackref != 0) && !next_state.is_null ()
    {
      /* Check OP_OPEN_SUBEXP in the current state in case that we use them
	 later.  We must check them here, since the back references in the
	 next state might use them.  */
      *err = check_subexp_matching_top (mctx, &mut (*next_state).nodes,
					cur_idx);
      if __glibc_unlikely (*err != REG_NOERROR)
	{ return null_mut (); }

      /* If the next state has back references.  */
      if has_backref_ (next_state) != 0
	{
	  *err = transit_state_bkref (mctx, &(*next_state).nodes);
	  if __glibc_unlikely (*err != REG_NOERROR)
	    { return null_mut (); }
	  next_state = *(*mctx).state_log.add (cur_idx as usize);
	}
    }

  return next_state;
}

/* Skip bytes in the input that correspond to part of a
   multi-byte match, then look in the log for a state
   from which to restart matching.  */
unsafe fn find_recover_state (err: *mut reg_errcode_t, mctx: *mut re_match_context_t)
			      -> *mut re_dfastate_t
{
  let mut cur_state: *mut re_dfastate_t;
  loop
    {
      let max: Idx = (*mctx).state_log_top;
      let mut cur_str_idx: Idx = re_string_cur_idx! (&(*mctx).input);

      loop
	{
	  cur_str_idx += 1;
	  if cur_str_idx > max
	    { return null_mut (); }
	  re_string_skip_bytes! (&mut (*mctx).input, 1);
	  if !(*(*mctx).state_log.add (cur_str_idx as usize)).is_null ()
	    { break; }
	}

      cur_state = merge_state_with_log (err, mctx, null_mut ());
      if !(*err == REG_NOERROR && cur_state.is_null ())
	{ break; }
    }
  return cur_state;
}

/* Helper functions for transit_state.  */

/* From the node set CUR_NODES, pick up the nodes whose types are
   OP_OPEN_SUBEXP and which have corresponding back references in the regular
   expression. And register them to use them later for evaluating the
   corresponding back references.  */

unsafe fn check_subexp_matching_top (mctx: *mut re_match_context_t, cur_nodes: *mut re_node_set,
				     str_idx: Idx) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut node_idx: Idx;
  let mut err: reg_errcode_t;

  /* TODO: This isn't efficient.
	   Because there might be more than one nodes whose types are
	   OP_OPEN_SUBEXP and whose index is SUBEXP_IDX, we must check all
	   nodes.
	   E.g. RE: (a){2}  */
  node_idx = 0;
  while node_idx < (*cur_nodes).nelem
    {
      let node: Idx = *(*cur_nodes).elems.add (node_idx as usize);
      if type_ (&*(*dfa).nodes.add (node as usize)) == OP_OPEN_SUBEXP as c_uint
	  && (*(*dfa).nodes.add (node as usize)).opr.idx < BITSET_WORD_BITS
	  && ((*dfa).used_bkref_map
	      & ((1 as bitset_word_t) << (*(*dfa).nodes.add (node as usize)).opr.idx)) != 0
	{
	  err = match_ctx_add_subtop (mctx, node, str_idx);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	}
      node_idx += 1;
    }
  return REG_NOERROR;
}

// C: `#if 0` (regexec.c 2469-2509) -- `transit_state_sb` is dead.

unsafe fn transit_state_mb (mctx: *mut re_match_context_t, pstate: *mut re_dfastate_t)
			    -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t;
  let mut i: Idx;

  i = 0;
  while i < (*pstate).nodes.nelem
    {
      let mut dest_nodes: re_node_set = std::mem::zeroed ();
      let new_nodes: *mut re_node_set;
      let cur_node_idx: Idx = *(*pstate).nodes.elems.add (i as usize);
      let mut naccepted: c_int;
      let dest_idx: Idx;
      let mut context: c_uint;
      let dest_state: *mut re_dfastate_t;

      if accept_mb_ (&*(*dfa).nodes.add (cur_node_idx as usize)) == 0
	{ i += 1; continue; }

      if constraint_ (&*(*dfa).nodes.add (cur_node_idx as usize)) != 0
	{
	  context = re_string_context_at (&mut (*mctx).input,
					  re_string_cur_idx! (&(*mctx).input),
					  (*mctx).eflags);
	  if NOT_SATISFY_NEXT_CONSTRAINT! (constraint_ (&*(*dfa).nodes.add (cur_node_idx as usize)),
					   context)
	    { i += 1; continue; }
	}

      /* How many bytes the node can accept?  */
      naccepted = check_node_accept_bytes (dfa, cur_node_idx, &(*mctx).input,
					   re_string_cur_idx! (&(*mctx).input));
      if naccepted == 0
	{ i += 1; continue; }

      /* The node can accepts 'naccepted' bytes.  */
      dest_idx = re_string_cur_idx! (&(*mctx).input) + naccepted as Idx;
      (*mctx).max_mb_elem_len = if (*mctx).max_mb_elem_len < naccepted { naccepted }
			         else { (*mctx).max_mb_elem_len };
      err = clean_state_log_if_needed (mctx, dest_idx);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }
      DEBUG_ASSERT! (*(*dfa).nexts.add (cur_node_idx as usize) != -1);
      new_nodes = (*dfa).eclosures.add (*(*dfa).nexts.add (cur_node_idx as usize) as usize);

      dest_state = *(*mctx).state_log.add (dest_idx as usize);
      if dest_state.is_null ()
	{/* C: `dest_nodes = *new_nodes;` (a struct copy).  */
	 dest_nodes = std::ptr::read (new_nodes); }
      else
	{
	  err = re_node_set_init_union (&mut dest_nodes,
					(*dest_state).entrance_nodes, new_nodes);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	}
      context = re_string_context_at (&mut (*mctx).input, dest_idx - 1,
				      (*mctx).eflags);
      *(*mctx).state_log.add (dest_idx as usize)
	= re_acquire_state_context (&mut err, dfa, &dest_nodes, context);
      if !dest_state.is_null ()
	{ re_node_set_free! (&mut dest_nodes); }
      if __glibc_unlikely (*(*mctx).state_log.add (dest_idx as usize) == null_mut ()
			    && err != REG_NOERROR)
	{ return err; }
      i += 1;
    }
  return REG_NOERROR;
}

unsafe fn transit_state_bkref (mctx: *mut re_match_context_t, nodes: *const re_node_set)
			       -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t;
  let mut i: Idx;
  let cur_str_idx: Idx = re_string_cur_idx! (&(*mctx).input);

  'free_return: {
  i = 0;
  while i < (*nodes).nelem
    {
      let mut dest_str_idx: Idx;
      let mut prev_nelem: Idx;
      let mut bkc_idx: Idx;
      let node_idx: Idx = *(*nodes).elems.add (i as usize);
      let mut context: c_uint;
      let node: *const re_token_t = (*dfa).nodes.add (node_idx as usize) as *const re_token_t;
      let mut new_dest_nodes: *mut re_node_set;

      /* Check whether 'node' is a backreference or not.  */
      if type_ (node) != OP_BACK_REF as c_uint
	{ i += 1; continue; }

      if constraint_ (node) != 0
	{
	  context = re_string_context_at (&mut (*mctx).input, cur_str_idx,
					  (*mctx).eflags);
	  if NOT_SATISFY_NEXT_CONSTRAINT! (constraint_ (node), context)
	    { i += 1; continue; }
	}

      /* 'node' is a backreference.
	 Check the substring which the substring matched.  */
      bkc_idx = (*mctx).nbkref_ents;
      err = get_subexp (mctx, node_idx, cur_str_idx);
      if __glibc_unlikely (err != REG_NOERROR)
	{ break 'free_return; }

      /* And add the epsilon closures (which is 'new_dest_nodes') of
	 the backreference to appropriate state_log.  */
      DEBUG_ASSERT! (*(*dfa).nexts.add (node_idx as usize) != -1);
      while bkc_idx < (*mctx).nbkref_ents
	{
	  let subexp_len: Idx;
	  let dest_state: *mut re_dfastate_t;
	  let bkref_ent: *mut re_backref_cache_entry;
	  bkref_ent = (*mctx).bkref_ents.add (bkc_idx as usize);
	  if (*bkref_ent).node != node_idx || (*bkref_ent).str_idx != cur_str_idx
	    { bkc_idx += 1; continue; }
	  subexp_len = (*bkref_ent).subexp_to - (*bkref_ent).subexp_from;
	  new_dest_nodes = if subexp_len == 0
			     { (*dfa).eclosures.add
                                 (*(*(*dfa).edests.add (node_idx as usize)).elems as usize) }
			   else
			     { (*dfa).eclosures.add
                                 (*(*dfa).nexts.add (node_idx as usize) as usize) };
	  dest_str_idx = cur_str_idx + (*bkref_ent).subexp_to
			  - (*bkref_ent).subexp_from;
	  context = re_string_context_at (&mut (*mctx).input, dest_str_idx - 1,
					  (*mctx).eflags);
	  dest_state = *(*mctx).state_log.add (dest_str_idx as usize);
	  prev_nelem = if (*(*mctx).state_log.add (cur_str_idx as usize)).is_null ()
	                 { 0 }
			 else
			   { (*(*(*mctx).state_log.add (cur_str_idx as usize))).nodes.nelem };
	  /* Add 'new_dest_node' to state_log.  */
	  if dest_state.is_null ()
	    {
	      *(*mctx).state_log.add (dest_str_idx as usize)
		= re_acquire_state_context (&mut err, dfa, new_dest_nodes,
					    context);
	      if __glibc_unlikely (*(*mctx).state_log.add (dest_str_idx as usize) == null_mut ()
				    && err != REG_NOERROR)
		{ break 'free_return; }
	    }
	  else
	    {
	      let mut dest_nodes: re_node_set = std::mem::zeroed ();
	      err = re_node_set_init_union (&mut dest_nodes,
					    (*dest_state).entrance_nodes,
					    new_dest_nodes);
	      if __glibc_unlikely (err != REG_NOERROR)
		{
		  re_node_set_free! (&mut dest_nodes);
		  break 'free_return;
		}
	      *(*mctx).state_log.add (dest_str_idx as usize)
		= re_acquire_state_context (&mut err, dfa, &dest_nodes, context);
	      re_node_set_free! (&mut dest_nodes);
	      if __glibc_unlikely (*(*mctx).state_log.add (dest_str_idx as usize) == null_mut ()
				    && err != REG_NOERROR)
		{ break 'free_return; }
	    }
	  /* We need to check recursively if the backreference can epsilon
	     transit.  */
	  if subexp_len == 0
	      && (*(*(*mctx).state_log.add (cur_str_idx as usize))).nodes.nelem > prev_nelem
	    {
	      err = check_subexp_matching_top (mctx, new_dest_nodes,
					       cur_str_idx);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break 'free_return; }
	      err = transit_state_bkref (mctx, new_dest_nodes);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break 'free_return; }
	    }
	  bkc_idx += 1;
	}
      i += 1;
    }
  err = REG_NOERROR;
  } /* 'free_return */

  /* free_return: */
  return err;
}

/* Enumerate all the candidates which the backreference BKREF_NODE can match
   at BKREF_STR_IDX, and register them by match_ctx_add_entry().
   Note that we might collect inappropriate candidates here.
   However, the cost of checking them strictly here is too high, then we
   delay these checking for prune_impossible_nodes().  */

unsafe fn get_subexp (mctx: *mut re_match_context_t, bkref_node: Idx, bkref_str_idx: Idx)
		      -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut subexp_num: Idx;
  let mut sub_top_idx: Idx;
  let mut buf: *const c_char = re_string_get_buffer! (&(*mctx).input) as *const c_char;
  /* Return if we have already checked BKREF_NODE at BKREF_STR_IDX.  */
  let cache_idx: Idx = search_cur_bkref_entry (mctx, bkref_str_idx);
  if cache_idx != -1
    {
      let mut entry: *const re_backref_cache_entry
	= (*mctx).bkref_ents.add (cache_idx as usize);
      loop
	{
	  if (*entry).node == bkref_node
	    { return REG_NOERROR; } /* We already checked it.  */
	  /* C: `while (entry++->more)` */
	  let more: c_char = (*entry).more;
	  entry = entry.add (1);
	  if more == 0
	    { break; }
	}
    }

  subexp_num = (*(*dfa).nodes.add (bkref_node as usize)).opr.idx;

  /* For each sub expression  */
  sub_top_idx = 0;
  while sub_top_idx < (*mctx).nsub_tops
    {
      let mut err: reg_errcode_t;
      let sub_top: *mut re_sub_match_top_t = *(*mctx).sub_tops.add (sub_top_idx as usize);
      let mut sub_last: *mut re_sub_match_last_t;
      let mut sub_last_idx: Idx;
      let mut sl_str: Idx;
      let mut bkref_str_off: Idx;

      if (*(*dfa).nodes.add ((*sub_top).node as usize)).opr.idx != subexp_num
	{ sub_top_idx += 1; continue; } /* It isn't related.  */

      sl_str = (*sub_top).str_idx;
      bkref_str_off = bkref_str_idx;
      /* At first, check the last node of sub expressions we already
	 evaluated.  */
      sub_last_idx = 0;
      while sub_last_idx < (*sub_top).nlasts
	{
	  let sl_str_diff: regoff_t;
	  sub_last = *(*sub_top).lasts.add (sub_last_idx as usize);
	  sl_str_diff = (*sub_last).str_idx - sl_str;
	  /* The matched string by the sub expression match with the substring
	     at the back reference?  */
	  if sl_str_diff > 0
	    {
	      if __glibc_unlikely (bkref_str_off + sl_str_diff
				    > (*mctx).input.valid_len)
		{
		  /* Not enough chars for a successful match.  */
		  if bkref_str_off + sl_str_diff > (*mctx).input.len
		    { break; }

		  err = clean_state_log_if_needed (mctx,
						   bkref_str_off
						   + sl_str_diff);
		  if __glibc_unlikely (err != REG_NOERROR)
		    { return err; }
		  buf = re_string_get_buffer! (&(*mctx).input) as *const c_char;
		}
	      /* C: `memcmp (buf + bkref_str_off, buf + sl_str, sl_str_diff) != 0`  */
	      if memcmp (buf.add (bkref_str_off as usize) as *const c_void,
                         buf.add (sl_str as usize) as *const c_void,
                         sl_str_diff as usize) != 0
		/* We don't need to search this sub expression any more.  */
		{ break; }
	    }
	  bkref_str_off += sl_str_diff;
	  sl_str += sl_str_diff;
	  err = get_subexp_sub (mctx, sub_top, sub_last, bkref_node,
				bkref_str_idx);

	  /* Reload buf, since the preceding call might have reallocated
	     the buffer.  */
	  buf = re_string_get_buffer! (&(*mctx).input) as *const c_char;

	  if err == REG_NOMATCH
	    { sub_last_idx += 1; continue; }
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	  sub_last_idx += 1;
	}

      if sub_last_idx < (*sub_top).nlasts
	{ sub_top_idx += 1; continue; }
      if sub_last_idx > 0
	{ sl_str += 1; }
      /* Then, search for the other last nodes of the sub expression.  */
      while sl_str <= bkref_str_idx
	{
	  let mut cls_node: Idx;
	  let sl_str_off: regoff_t;
	  let nodes: *const re_node_set;
	  sl_str_off = sl_str - (*sub_top).str_idx;
	  /* The matched string by the sub expression match with the substring
	     at the back reference?  */
	  if sl_str_off > 0
	    {
	      if __glibc_unlikely (bkref_str_off >= (*mctx).input.valid_len)
		{
		  /* If we are at the end of the input, we cannot match.  */
		  if bkref_str_off >= (*mctx).input.len
		    { break; }

		  err = extend_buffers (mctx, (bkref_str_off + 1) as c_int);
		  if __glibc_unlikely (err != REG_NOERROR)
		    { return err; }

		  buf = re_string_get_buffer! (&(*mctx).input) as *const c_char;
		}
	      /* C: `buf [bkref_str_off++] != buf[sl_str - 1]` (the post-increment is
	         the two statements below).  */
	      let b: c_char = *buf.add (bkref_str_off as usize);
	      bkref_str_off += 1;
	      if b != *buf.add ((sl_str - 1) as usize)
		{ break; } /* We don't need to search this sub expression
			      any more.  */
	    }
	  if (*(*mctx).state_log.add (sl_str as usize)).is_null ()
	    { sl_str += 1; continue; }
	  /* Does this state have a ')' of the sub expression?  */
	  nodes = &(*(*(*mctx).state_log.add (sl_str as usize))).nodes;
	  cls_node = find_subexp_node (dfa, nodes, subexp_num,
				       OP_CLOSE_SUBEXP as c_int);
	  if cls_node == -1
	    { sl_str += 1; continue; } /* No.  */
	  if (*sub_top).path.is_null ()
	    {
	      (*sub_top).path = calloc ((sl_str - (*sub_top).str_idx + 1) as usize,
                                        std::mem::size_of::<state_array_t> ())
                                as *mut state_array_t;
	      if (*sub_top).path.is_null ()
		{ return REG_ESPACE; }
	    }
	  /* Can the OP_OPEN_SUBEXP node arrive the OP_CLOSE_SUBEXP node
	     in the current context?  */
	  err = check_arrival (mctx, (*sub_top).path, (*sub_top).node,
			       (*sub_top).str_idx, cls_node, sl_str,
			       OP_CLOSE_SUBEXP as c_int);
	  if err == REG_NOMATCH
	      { sl_str += 1; continue; }
	  if __glibc_unlikely (err != REG_NOERROR)
	      { return err; }
	  sub_last = match_ctx_add_sublast (sub_top, cls_node, sl_str);
	  if __glibc_unlikely (sub_last.is_null ())
	    { return REG_ESPACE; }
	  err = get_subexp_sub (mctx, sub_top, sub_last, bkref_node,
				bkref_str_idx);
	  buf = re_string_get_buffer! (&(*mctx).input) as *const c_char;
	  if err == REG_NOMATCH
	    { sl_str += 1; continue; }
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	  sl_str += 1;
	}
      sub_top_idx += 1;
    }
  return REG_NOERROR;
}

/* Helper functions for get_subexp().  */

/* Check SUB_LAST can arrive to the back reference BKREF_NODE at BKREF_STR.
   If it can arrive, register the sub expression expressed with SUB_TOP
   and SUB_LAST.  */

unsafe fn get_subexp_sub (mctx: *mut re_match_context_t,
			  sub_top: *const re_sub_match_top_t,
			  sub_last: *mut re_sub_match_last_t, bkref_node: Idx,
			  bkref_str: Idx) -> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let to_idx: Idx;
  /* Can the subexpression arrive the back reference?  */
  err = check_arrival (mctx, &mut (*sub_last).path, (*sub_last).node,
		       (*sub_last).str_idx, bkref_node, bkref_str,
		       OP_OPEN_SUBEXP as c_int);
  if err != REG_NOERROR
    { return err; }
  err = match_ctx_add_entry (mctx, bkref_node, bkref_str, (*sub_top).str_idx,
			     (*sub_last).str_idx);
  if __glibc_unlikely (err != REG_NOERROR)
    { return err; }
  to_idx = bkref_str + (*sub_last).str_idx - (*sub_top).str_idx;
  return clean_state_log_if_needed (mctx, to_idx);
}

/* Find the first node which is '(' or ')' and whose index is SUBEXP_IDX.
   Search '(' if FL_OPEN, or search ')' otherwise.
   TODO: This function isn't efficient...
	 Because there might be more than one nodes whose types are
	 OP_OPEN_SUBEXP and whose index is SUBEXP_IDX, we must check all
	 nodes.
	 E.g. RE: (a){2}  */

unsafe fn find_subexp_node (dfa: *const re_dfa_t, nodes: *const re_node_set,
			    subexp_idx: Idx, r#type: c_int) -> Idx
{
  let mut cls_idx: Idx;
  cls_idx = 0;
  while cls_idx < (*nodes).nelem
    {
      let cls_node: Idx = *(*nodes).elems.add (cls_idx as usize);
      let node: *const re_token_t = (*dfa).nodes.add (cls_node as usize) as *const re_token_t;
      if type_ (node) == r#type as c_uint
	  && (*node).opr.idx == subexp_idx
	{ return cls_node; }
      cls_idx += 1;
    }
  return -1;
}

/* Check whether the node TOP_NODE at TOP_STR can arrive to the node
   LAST_NODE at LAST_STR.  We record the path onto PATH since it will be
   heavily reused.
   Return REG_NOERROR if it can arrive, REG_NOMATCH if it cannot,
   REG_ESPACE if memory is exhausted.  */

unsafe fn check_arrival (mctx: *mut re_match_context_t, path: *mut state_array_t,
			 top_node: Idx, top_str: Idx, last_node: Idx, last_str: Idx,
			 r#type: c_int) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t = REG_NOERROR;
  let subexp_num: Idx;
  let backup_cur_idx: Idx;
  let mut str_idx: Idx;
  let mut null_cnt: Idx;
  let mut cur_state: *mut re_dfastate_t = null_mut ();
  let cur_nodes: *const re_node_set;
  let mut next_nodes: re_node_set = std::mem::zeroed ();
  let backup_state_log: *mut *mut re_dfastate_t;
  let mut context: c_uint;

  subexp_num = (*(*dfa).nodes.add (top_node as usize)).opr.idx;
  /* Extend the buffer if we need.  */
  if __glibc_unlikely ((*path).alloc < last_str + (*mctx).max_mb_elem_len as Idx + 1)
    {
      let new_array: *mut *mut re_dfastate_t;
      let old_alloc: Idx = (*path).alloc;
      let incr_alloc: Idx = last_str + (*mctx).max_mb_elem_len as Idx + 1;
      let new_alloc: Idx;
      if __glibc_unlikely (IDX_MAX - old_alloc < incr_alloc)
	{ return REG_ESPACE; }
      new_alloc = old_alloc + incr_alloc;
      if __glibc_unlikely (((SIZE_MAX / std::mem::size_of::<*mut re_dfastate_t> ()) as Idx)
                           < new_alloc)
	{ return REG_ESPACE; }
      new_array = re_realloc! ((*path).array, *mut re_dfastate_t, new_alloc);
      if __glibc_unlikely (new_array.is_null ())
	{ return REG_ESPACE; }
      (*path).array = new_array;
      (*path).alloc = new_alloc;
      memset (new_array.add (old_alloc as usize) as *mut c_void, b'\0' as c_int,
	      std::mem::size_of::<*mut re_dfastate_t> ()
              * ((*path).alloc - old_alloc) as usize);
    }

  str_idx = if (*path).next_idx != 0 { (*path).next_idx } else { top_str };

  /* Temporary modify MCTX.  */
  backup_state_log = (*mctx).state_log;
  backup_cur_idx = (*mctx).input.cur_idx;
  (*mctx).state_log = (*path).array;
  (*mctx).input.cur_idx = str_idx;

  /* Setup initial node set.  */
  context = re_string_context_at (&mut (*mctx).input, str_idx - 1, (*mctx).eflags);
  if str_idx == top_str
    {
      err = re_node_set_init_1 (&mut next_nodes, top_node);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }
      err = check_arrival_expand_ecl (dfa, &mut next_nodes, subexp_num, r#type);
      if __glibc_unlikely (err != REG_NOERROR)
	{
	  re_node_set_free! (&mut next_nodes);
	  return err;
	}
    }
  else
    {
      cur_state = *(*mctx).state_log.add (str_idx as usize);
      if !cur_state.is_null () && has_backref_ (cur_state) != 0
	{
	  err = re_node_set_init_copy (&mut next_nodes, &(*cur_state).nodes);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	}
      else
	{ re_node_set_init_empty! (&raw mut next_nodes); }
    }
  if str_idx == top_str || (!cur_state.is_null () && has_backref_ (cur_state) != 0)
    {
      if next_nodes.nelem != 0
	{
	  err = expand_bkref_cache (mctx, &mut next_nodes, str_idx,
				    subexp_num, r#type);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut next_nodes);
	      return err;
	    }
	}
      cur_state = re_acquire_state_context (&mut err, dfa, &next_nodes, context);
      if __glibc_unlikely (cur_state.is_null () && err != REG_NOERROR)
	{
	  re_node_set_free! (&mut next_nodes);
	  return err;
	}
      *(*mctx).state_log.add (str_idx as usize) = cur_state;
    }

  null_cnt = 0;
  while str_idx < last_str && null_cnt <= (*mctx).max_mb_elem_len as Idx
    {
      re_node_set_empty! (&mut next_nodes);
      if !(*(*mctx).state_log.add ((str_idx + 1) as usize)).is_null ()
	{
	  err = re_node_set_merge (&mut next_nodes,
				   &(*(*(*mctx).state_log.add ((str_idx + 1) as usize))).nodes);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut next_nodes);
	      return err;
	    }
	}
      if !cur_state.is_null ()
	{
	  err = check_arrival_add_next_nodes (mctx, str_idx,
					      &mut (*cur_state).non_eps_nodes,
					      &mut next_nodes);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut next_nodes);
	      return err;
	    }
	}
      str_idx += 1;
      if next_nodes.nelem != 0
	{
	  err = check_arrival_expand_ecl (dfa, &mut next_nodes, subexp_num, r#type);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut next_nodes);
	      return err;
	    }
	  err = expand_bkref_cache (mctx, &mut next_nodes, str_idx,
				    subexp_num, r#type);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut next_nodes);
	      return err;
	    }
	}
      context = re_string_context_at (&mut (*mctx).input, str_idx - 1, (*mctx).eflags);
      cur_state = re_acquire_state_context (&mut err, dfa, &next_nodes, context);
      if __glibc_unlikely (cur_state.is_null () && err != REG_NOERROR)
	{
	  re_node_set_free! (&mut next_nodes);
	  return err;
	}
      *(*mctx).state_log.add (str_idx as usize) = cur_state;
      null_cnt = if cur_state.is_null () { null_cnt + 1 } else { 0 };
    }
  re_node_set_free! (&mut next_nodes);
  cur_nodes = if (*(*mctx).state_log.add (last_str as usize)).is_null () { null_mut () }
	      else { &(*(*(*mctx).state_log.add (last_str as usize))).nodes };
  (*path).next_idx = str_idx;

  /* Fix MCTX.  */
  (*mctx).state_log = backup_state_log;
  (*mctx).input.cur_idx = backup_cur_idx;

  /* Then check the current node set has the node LAST_NODE.  */
  if !cur_nodes.is_null () && re_node_set_contains (cur_nodes, last_node) != 0
    { return REG_NOERROR; }

  return REG_NOMATCH;
}

/* Helper functions for check_arrival.  */

/* Calculate the destination nodes of CUR_NODES at STR_IDX, and append them
   to NEXT_NODES.
   TODO: This function is similar to the functions transit_state*(),
	 however this function has many additional works.
	 Can't we unify them?  */

unsafe fn check_arrival_add_next_nodes (mctx: *mut re_match_context_t, str_idx: Idx,
					cur_nodes: *mut re_node_set,
					next_nodes: *mut re_node_set) -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut ok: bool;
  let mut cur_idx: Idx;
  let mut err: reg_errcode_t = REG_NOERROR;
  let mut union_set: re_node_set = std::mem::zeroed ();
  re_node_set_init_empty! (&raw mut union_set);
  cur_idx = 0;
  while cur_idx < (*cur_nodes).nelem
    {
      let mut naccepted: c_int = 0;
      let cur_node: Idx = *(*cur_nodes).elems.add (cur_idx as usize);
      DEBUG_ASSERT! (IS_EPSILON_NODE! (type_ (&*(*dfa).nodes.add (cur_node as usize))) == 0);

      /* If the node may accept "multi byte".  */
      if accept_mb_ (&*(*dfa).nodes.add (cur_node as usize)) != 0
	{
	  naccepted = check_node_accept_bytes (dfa, cur_node, &(*mctx).input,
					       str_idx);
	  if naccepted > 1
	    {
	      let dest_state: *mut re_dfastate_t;
	      let next_node: Idx = *(*dfa).nexts.add (cur_node as usize);
	      let next_idx: Idx = str_idx + naccepted as Idx;
	      dest_state = *(*mctx).state_log.add (next_idx as usize);
	      re_node_set_empty! (&mut union_set);
	      if !dest_state.is_null ()
		{
		  err = re_node_set_merge (&mut union_set, &(*dest_state).nodes);
		  if __glibc_unlikely (err != REG_NOERROR)
		    {
		      re_node_set_free! (&mut union_set);
		      return err;
		    }
		}
	      ok = re_node_set_insert (&mut union_set, next_node);
	      if __glibc_unlikely (! ok)
		{
		  re_node_set_free! (&mut union_set);
		  return REG_ESPACE;
		}
	      *(*mctx).state_log.add (next_idx as usize) = re_acquire_state (&mut err, dfa,
							    &union_set);
	      if __glibc_unlikely (*(*mctx).state_log.add (next_idx as usize) == null_mut ()
				    && err != REG_NOERROR)
		{
		  re_node_set_free! (&mut union_set);
		  return err;
		}
	    }
	}

      if naccepted != 0
	  || check_node_accept (mctx, (*dfa).nodes.add (cur_node as usize), str_idx)
	{
	  ok = re_node_set_insert (next_nodes, *(*dfa).nexts.add (cur_node as usize));
	  if __glibc_unlikely (! ok)
	    {
	      re_node_set_free! (&mut union_set);
	      return REG_ESPACE;
	    }
	}
      cur_idx += 1;
    }
  re_node_set_free! (&mut union_set);
  return REG_NOERROR;
}

/* For all the nodes in CUR_NODES, add the epsilon closures of them to
   CUR_NODES, however exclude the nodes which are:
    - inside the sub expression whose number is EX_SUBEXP, if FL_OPEN.
    - out of the sub expression whose number is EX_SUBEXP, if !FL_OPEN.
*/

unsafe fn check_arrival_expand_ecl (dfa: *const re_dfa_t, cur_nodes: *mut re_node_set,
				    ex_subexp: Idx, r#type: c_int) -> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let mut idx: Idx;
  let mut outside_node: Idx;
  let mut new_nodes: re_node_set = std::mem::zeroed ();
  DEBUG_ASSERT! ((*cur_nodes).nelem != 0);
  err = re_node_set_alloc (&mut new_nodes, (*cur_nodes).nelem);
  if __glibc_unlikely (err != REG_NOERROR)
    { return err; }
  /* Create a new node set NEW_NODES with the nodes which are epsilon
     closures of the node in CUR_NODES.  */

  idx = 0;
  while idx < (*cur_nodes).nelem
    {
      let cur_node: Idx = *(*cur_nodes).elems.add (idx as usize);
      let eclosure: *const re_node_set = (*dfa).eclosures.add (cur_node as usize);
      outside_node = find_subexp_node (dfa, eclosure, ex_subexp, r#type);
      if outside_node == -1
	{
	  /* There are no problematic nodes, just merge them.  */
	  err = re_node_set_merge (&mut new_nodes, eclosure);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut new_nodes);
	      return err;
	    }
	}
      else
	{
	  /* There are problematic nodes, re-calculate incrementally.  */
	  err = check_arrival_expand_ecl_sub (dfa, &mut new_nodes, cur_node,
					      ex_subexp, r#type);
	  if __glibc_unlikely (err != REG_NOERROR)
	    {
	      re_node_set_free! (&mut new_nodes);
	      return err;
	    }
	}
      idx += 1;
    }
  re_node_set_free! (cur_nodes);
  *cur_nodes = new_nodes;
  return REG_NOERROR;
}

/* Helper function for check_arrival_expand_ecl.
   Check incrementally the epsilon closure of TARGET, and if it isn't
   problematic append it to DST_NODES.  */

unsafe fn check_arrival_expand_ecl_sub (dfa: *const re_dfa_t, dst_nodes: *mut re_node_set,
					target: Idx, ex_subexp: Idx, r#type: c_int)
					-> reg_errcode_t
{
  let mut cur_node: Idx;
  cur_node = target;
  while re_node_set_contains (dst_nodes, cur_node) == 0
    {
      let ok: bool;

      if type_ (&*(*dfa).nodes.add (cur_node as usize)) == r#type as c_uint
	  && (*(*dfa).nodes.add (cur_node as usize)).opr.idx == ex_subexp
	{
	  if r#type == OP_CLOSE_SUBEXP as c_int
	    {
	      ok = re_node_set_insert (dst_nodes, cur_node);
	      if __glibc_unlikely (! ok)
		{ return REG_ESPACE; }
	    }
	  break;
	}
      ok = re_node_set_insert (dst_nodes, cur_node);
      if __glibc_unlikely (! ok)
	{ return REG_ESPACE; }
      if (*(*dfa).edests.add (cur_node as usize)).nelem == 0
	{ break; }
      if (*(*dfa).edests.add (cur_node as usize)).nelem == 2
	{
	  let mut err: reg_errcode_t;
	  err = check_arrival_expand_ecl_sub (dfa, dst_nodes,
					      *(*(*dfa).edests.add (cur_node as usize)).elems.add (1),
					      ex_subexp, r#type);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	}
      cur_node = *(*(*dfa).edests.add (cur_node as usize)).elems;
    }
  return REG_NOERROR;
}


/* For all the back references in the current state, calculate the
   destination of the back references by the appropriate entry
   in MCTX->BKREF_ENTS.  */

unsafe fn expand_bkref_cache (mctx: *mut re_match_context_t, cur_nodes: *mut re_node_set,
			      cur_str: Idx, subexp_num: Idx, r#type: c_int)
			      -> reg_errcode_t
{
  let dfa: *const re_dfa_t = (*mctx).dfa;
  let mut err: reg_errcode_t;
  let cache_idx_start: Idx = search_cur_bkref_entry (mctx, cur_str);
  let mut ent: *mut re_backref_cache_entry;

  if cache_idx_start == -1
    { return REG_NOERROR; }

 'restart: loop
  {
  ent = (*mctx).bkref_ents.add (cache_idx_start as usize);
  loop
    {
      let mut to_idx: Idx;
      let mut next_node: Idx;

      'do_body: {
      /* Is this entry ENT is appropriate?  */
      if re_node_set_contains (cur_nodes, (*ent).node) == 0
	{ break 'do_body; } /* No.  */

      to_idx = cur_str + (*ent).subexp_to - (*ent).subexp_from;
      /* Calculate the destination of the back reference, and append it
	 to MCTX->STATE_LOG.  */
      if to_idx == cur_str
	{
	  /* The backreference did epsilon transit, we must re-check all the
	     node in the current state.  */
	  let mut new_dests: re_node_set = std::mem::zeroed ();
	  let err2: reg_errcode_t;
	  let err3: reg_errcode_t;
	  next_node = *(*(*dfa).edests.add ((*ent).node as usize)).elems;
	  if re_node_set_contains (cur_nodes, next_node) != 0
	    { break 'do_body; }
	  err = re_node_set_init_1 (&mut new_dests, next_node);
	  err2 = check_arrival_expand_ecl (dfa, &mut new_dests, subexp_num, r#type);
	  err3 = re_node_set_merge (cur_nodes, &new_dests);
	  re_node_set_free! (&mut new_dests);
	  if __glibc_unlikely (err != REG_NOERROR || err2 != REG_NOERROR
				|| err3 != REG_NOERROR)
	    {
	      err = if err != REG_NOERROR { err }
		     else if err2 != REG_NOERROR { err2 } else { err3 };
	      return err;
	    }
	  /* TODO: It is still inefficient...  */
	  continue 'restart;
	}
      else
	{
	  let mut union_set: re_node_set = std::mem::zeroed ();
	  next_node = *(*dfa).nexts.add ((*ent).node as usize);
	  if !(*(*mctx).state_log.add (to_idx as usize)).is_null ()
	    {
	      let ok: bool;
	      if re_node_set_contains (&(*(*(*mctx).state_log.add (to_idx as usize))).nodes,
					next_node) != 0
		{ break 'do_body; }
	      err = re_node_set_init_copy (&mut union_set,
					   &(*(*(*mctx).state_log.add (to_idx as usize))).nodes);
	      ok = re_node_set_insert (&mut union_set, next_node);
	      if __glibc_unlikely (err != REG_NOERROR || ! ok)
		{
		  re_node_set_free! (&mut union_set);
		  err = if err != REG_NOERROR { err } else { REG_ESPACE };
		  return err;
		}
	    }
	  else
	    {
	      err = re_node_set_init_1 (&mut union_set, next_node);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ return err; }
	    }
	  *(*mctx).state_log.add (to_idx as usize) = re_acquire_state (&mut err, dfa, &union_set);
	  re_node_set_free! (&mut union_set);
	  if __glibc_unlikely (*(*mctx).state_log.add (to_idx as usize) == null_mut ()
				&& err != REG_NOERROR)
	    { return err; }
	}
      }
      /* C: `while (ent++->more)` */
      let more: c_char = (*ent).more;
      ent = ent.add (1);
      if more == 0
	{ break; }
    }
  }
  return REG_NOERROR;
}

/* Build transition table for the state.
   Return true if successful.  */

unsafe fn build_trtable (dfa: *const re_dfa_t, state: *mut re_dfastate_t) -> bool
{
  let mut err: reg_errcode_t;
  let mut i: Idx;
  let mut j: Idx;
  let mut ch: c_int;
  let mut need_word_trtable: bool = false;
  let mut elem: bitset_word_t;
  let mut mask: bitset_word_t;
  let mut ndests: Idx; /* Number of the destination states from 'state'.  */
  let mut trtable: *mut *mut re_dfastate_t;
  let mut dest_states: [*mut re_dfastate_t; SBC_MAX as usize] = std::mem::zeroed ();
  let mut dest_states_word: [*mut re_dfastate_t; SBC_MAX as usize] = std::mem::zeroed ();
  let mut dest_states_nl: [*mut re_dfastate_t; SBC_MAX as usize] = std::mem::zeroed ();
  let mut follows: re_node_set = std::mem::zeroed ();
  let mut acceptable: bitset_t = std::mem::zeroed ();

  /* We build DFA states which corresponds to the destination nodes
     from 'state'.  'dests_node[i]' represents the nodes which i-th
     destination state contains, and 'dests_ch[i]' represents the
     characters which i-th destination state accepts.  */
  let mut dests_node: [re_node_set; SBC_MAX as usize] = std::mem::zeroed ();
  let mut dests_ch: [bitset_t; SBC_MAX as usize] = std::mem::zeroed ();

  /* Initialize transition table.  */
  (*state).word_trtable = null_mut ();
  (*state).trtable = null_mut ();

  /* At first, group all nodes belonging to 'state' into several
     destinations.  */
  ndests = group_nodes_into_DFAstates (dfa, state, dests_node.as_mut_ptr (),
                                       dests_ch.as_mut_ptr ());
  if __glibc_unlikely (ndests <= 0)
    {
      /* Return false in case of an error, true otherwise.  */
      if ndests == 0
	{
	  (*state).trtable = calloc (SBC_MAX as usize,
                                     std::mem::size_of::<*mut re_dfastate_t> ())
                             as *mut *mut re_dfastate_t;
          if __glibc_unlikely ((*state).trtable.is_null ())
            { return false; }
	  return true;
	}
      return false;
    }

  'out_free: {
  err = re_node_set_alloc (&mut follows, ndests + 1);
  if __glibc_unlikely (err != REG_NOERROR)
    {
      break 'out_free;
    }

  bitset_empty (acceptable.as_mut_ptr ());

  /* Then build the states for all destinations.  */
  i = 0;
  while i < ndests
    {
      let mut next_node: Idx;
      re_node_set_empty! (&mut follows);
      /* Merge the follows of this destination states.  */
      j = 0;
      while j < dests_node[i as usize].nelem
	{
	  next_node = *(*dfa).nexts.add (dests_node[i as usize].elems.offset (j) as usize);
	  if next_node != -1
	    {
	      err = re_node_set_merge (&mut follows, (*dfa).eclosures.add (next_node as usize));
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break 'out_free; }
	    }
	  j += 1;
	}
      dest_states[i as usize] = re_acquire_state_context (&mut err, dfa, &follows, 0);
      if __glibc_unlikely (dest_states[i as usize].is_null () && err != REG_NOERROR)
	{ break 'out_free; }
      /* If the new state has context constraint,
	 build appropriate states for these contexts.  */
      if has_constraint_ (dest_states[i as usize]) != 0
	{
	  dest_states_word[i as usize] = re_acquire_state_context (&mut err, dfa, &follows,
							  CONTEXT_WORD as c_uint);
	  if __glibc_unlikely (dest_states_word[i as usize].is_null ()
				&& err != REG_NOERROR)
	    { break 'out_free; }

	  if dest_states[i as usize] != dest_states_word[i as usize] && (*dfa).mb_cur_max > 1
	    { need_word_trtable = true; }

	  dest_states_nl[i as usize] = re_acquire_state_context (&mut err, dfa, &follows,
							CONTEXT_NEWLINE as c_uint);
	  if __glibc_unlikely (dest_states_nl[i as usize].is_null () && err != REG_NOERROR)
	    { break 'out_free; }
	}
      else
	{
	  dest_states_word[i as usize] = dest_states[i as usize];
	  dest_states_nl[i as usize] = dest_states[i as usize];
	}
      bitset_merge (acceptable.as_mut_ptr (), dests_ch[i as usize].as_ptr ());
      i += 1;
    }

  if !__glibc_unlikely (need_word_trtable)
    {
      /* We don't care about whether the following character is a word
	 character, or we are in a single-byte character set so we can
	 discern by looking at the character code: allocate a
	 256-entry transition table.  */
      trtable = calloc (SBC_MAX as usize, std::mem::size_of::<*mut re_dfastate_t> ())
                as *mut *mut re_dfastate_t;
      (*state).trtable = trtable;
      if __glibc_unlikely (trtable.is_null ())
	{ break 'out_free; }

      /* For all characters ch...:  */
      i = 0;
      while i < BITSET_WORDS as Idx
	{
	  ch = (i * BITSET_WORD_BITS) as c_int;
	  elem = acceptable[i as usize];
	  mask = 1;
	  while elem != 0
	    {
	      if __glibc_unlikely (elem & 1 != 0)
		{
		  /* There must be exactly one destination which accepts
		     character ch.  See group_nodes_into_DFAstates.  */
		  j = 0;
		  while dests_ch[j as usize][i as usize] & mask == 0
		    { j += 1; }

		  /* j-th destination accepts the word character ch.  */
		  if (*dfa).word_char[i as usize] & mask != 0
		    { *trtable.add (ch as usize) = dest_states_word[j as usize]; }
		  else
		    { *trtable.add (ch as usize) = dest_states[j as usize]; }
		}
	      mask <<= 1;
	      elem >>= 1;
	      ch += 1;
	    }
	  i += 1;
	}
    }
  else
    {
      /* We care about whether the following character is a word
	 character, and we are in a multi-byte character set: discern
	 by looking at the character code: build two 256-entry
	 transition tables, one starting at trtable[0] and one
	 starting at trtable[SBC_MAX].  */
      trtable = calloc (2 * SBC_MAX as usize,
                        std::mem::size_of::<*mut re_dfastate_t> ())
                as *mut *mut re_dfastate_t;
      (*state).word_trtable = trtable;
      if __glibc_unlikely (trtable.is_null ())
	{ break 'out_free; }

      /* For all characters ch...:  */
      i = 0;
      while i < BITSET_WORDS as Idx
	{
	  ch = (i * BITSET_WORD_BITS) as c_int;
	  elem = acceptable[i as usize];
	  mask = 1;
	  while elem != 0
	    {
	      if __glibc_unlikely (elem & 1 != 0)
		{
		  /* There must be exactly one destination which accepts
		     character ch.  See group_nodes_into_DFAstates.  */
		  j = 0;
		  while dests_ch[j as usize][i as usize] & mask == 0
		    { j += 1; }

		  /* j-th destination accepts the word character ch.  */
		  *trtable.add (ch as usize) = dest_states[j as usize];
		  *trtable.add ((ch as Idx + SBC_MAX) as usize) = dest_states_word[j as usize];
		}
	      mask <<= 1;
	      elem >>= 1;
	      ch += 1;
	    }
	  i += 1;
	}
    }

  /* new line */
  if bitset_contain (acceptable.as_ptr (), NEWLINE_CHAR as Idx)
    {
      /* The current state accepts newline character.  */
      j = 0;
      while j < ndests
	{
	  if bitset_contain (dests_ch[j as usize].as_ptr (), NEWLINE_CHAR as Idx)
	    {
	      /* k-th destination accepts newline character.  */
	      *trtable.add (NEWLINE_CHAR as usize) = dest_states_nl[j as usize];
	      if need_word_trtable
		{ *trtable.add (NEWLINE_CHAR as usize + SBC_MAX as usize) = dest_states_nl[j as usize]; }
	      /* There must be only one destination which accepts
		 newline.  See group_nodes_into_DFAstates.  */
	      break;
	    }
	  j += 1;
	}
    }

  re_node_set_free! (&mut follows);
  i = 0;
  while i < ndests
    {
      re_node_set_free! (dests_node.as_mut_ptr ().add (i as usize));
      i += 1;
    }
  return true;
  } /* 'out_free */

  /* out_free: */
  re_node_set_free! (&mut follows);
  i = 0;
  while i < ndests
    {
      re_node_set_free! (dests_node.as_mut_ptr ().add (i as usize));
      i += 1;
    }
  return false;
}

/* This static array is used for the map to single-byte characters when
   UTF-8 is used.  */
/* C: `static const bitset_t utf8_sb_map` is *defined* in regcomp.c 554 (owner:
   `femacs/lib/regcomp.rs`) and regexec.c 3551 reads it; the two files are separate objects in
   the port, so the measured initializer is repeated here, in the constant-false arm of
   `ASCII_CHARS % BITSET_WORD_BITS == 0` (`docs/ports/regexec.md` records it).  */
pub const utf8_sb_map: bitset_t =
  /* Set the first 128 bits.  */
  [BITSET_WORD_MAX, BITSET_WORD_MAX, 0, 0];

unsafe fn group_nodes_into_DFAstates (dfa: *const re_dfa_t, state: *const re_dfastate_t,
				      dests_node: *mut re_node_set,
				      dests_ch: *mut bitset_t) -> Idx
{
  let mut err: reg_errcode_t;
  let mut ok: bool;
  let mut i: Idx;
  let mut j: Idx;
  let mut k: Idx;
  let mut ndests: Idx; /* Number of the destinations from 'state'.  */
  let mut accepts: bitset_t = std::mem::zeroed (); /* Characters a node can accept.  */
  let cur_nodes: *const re_node_set = &(*state).nodes;
  bitset_empty (accepts.as_mut_ptr ());
  ndests = 0;

  'error_return: {
  /* For all the nodes belonging to 'state',  */
  i = 0;
  while i < (*cur_nodes).nelem
    {
      let node: *mut re_token_t =
        &mut *(*dfa).nodes.add (*(*cur_nodes).elems.add (i as usize) as usize);
      let r#type: c_uint = type_ (node);
      let constraint: c_uint = constraint_ (node);

      /* Enumerate all single byte character this node can accept.  */
      if r#type == CHARACTER as c_uint
	{ bitset_set (accepts.as_mut_ptr (), (*node).opr.c as Idx); }
      else if r#type == SIMPLE_BRACKET as c_uint
	{
	  bitset_merge (accepts.as_mut_ptr (), (*node).opr.sbcset);
	}
      else if r#type == OP_PERIOD as c_uint
	{
	  if (*dfa).mb_cur_max > 1
	    { bitset_merge (accepts.as_mut_ptr (), (*dfa).sb_char); }
	  else
	    { bitset_set_all (accepts.as_mut_ptr ()); }
	  if (*dfa).syntax & RE_DOT_NEWLINE == 0
	    { bitset_clear (accepts.as_mut_ptr (), b'\n' as Idx); }
	  if (*dfa).syntax & RE_DOT_NOT_NULL != 0
	    { bitset_clear (accepts.as_mut_ptr (), b'\0' as Idx); }
	}
      else if r#type == OP_UTF8_PERIOD as c_uint
	{
	  if (ASCII_CHARS as Idx) % BITSET_WORD_BITS == 0
	    { memset (accepts.as_mut_ptr () as *mut c_void, -1, (ASCII_CHARS / CHAR_BIT) as usize); }
	  else
	    { bitset_merge (accepts.as_mut_ptr (), utf8_sb_map.as_ptr ()); }
	  if (*dfa).syntax & RE_DOT_NEWLINE == 0
	    { bitset_clear (accepts.as_mut_ptr (), b'\n' as Idx); }
	  if (*dfa).syntax & RE_DOT_NOT_NULL != 0
	    { bitset_clear (accepts.as_mut_ptr (), b'\0' as Idx); }
	}
      else
	{ i += 1; continue; }

      /* Check the 'accepts' and sift the characters which are not
	 match it the context.  */
      if constraint != 0
	{
	  if constraint & NEXT_NEWLINE_CONSTRAINT as c_uint != 0
	    {
	      let accepts_newline: bool = bitset_contain (accepts.as_ptr (), NEWLINE_CHAR as Idx);
	      bitset_empty (accepts.as_mut_ptr ());
	      if accepts_newline
		{ bitset_set (accepts.as_mut_ptr (), NEWLINE_CHAR as Idx); }
	      else
		{ i += 1; continue; }
	    }
	  if constraint & NEXT_ENDBUF_CONSTRAINT as c_uint != 0
	    {
	      bitset_empty (accepts.as_mut_ptr ());
	      i += 1; continue;
	    }

	  if constraint & NEXT_WORD_CONSTRAINT as c_uint != 0
	    {
	      let mut any_set: bitset_word_t = 0;
	      if r#type == CHARACTER as c_uint && word_char_ (node) == 0
		{
		  bitset_empty (accepts.as_mut_ptr ());
		  i += 1; continue;
		}
	      if (*dfa).mb_cur_max > 1
		{
		  j = 0;
		  while j < BITSET_WORDS as Idx
		    {
		      accepts[j as usize] &= (*dfa).word_char[j as usize]
                                             | !*(*dfa).sb_char.add (j as usize);
		      any_set |= accepts[j as usize];
		      j += 1;
		    }
		}
	      else
		{
		  j = 0;
		  while j < BITSET_WORDS as Idx
		    {
		      accepts[j as usize] &= (*dfa).word_char[j as usize];
		      any_set |= accepts[j as usize];
		      j += 1;
		    }
		}
	      if any_set == 0
		{ i += 1; continue; }
	    }
	  if constraint & NEXT_NOTWORD_CONSTRAINT as c_uint != 0
	    {
	      let mut any_set: bitset_word_t = 0;
	      if r#type == CHARACTER as c_uint && word_char_ (node) != 0
		{
		  bitset_empty (accepts.as_mut_ptr ());
		  i += 1; continue;
		}
	      if (*dfa).mb_cur_max > 1
		{
		  j = 0;
		  while j < BITSET_WORDS as Idx
		    {
		      accepts[j as usize] &= !((*dfa).word_char[j as usize]
                                               & *(*dfa).sb_char.add (j as usize));
		      any_set |= accepts[j as usize];
		      j += 1;
		    }
		}
	      else
		{
		  j = 0;
		  while j < BITSET_WORDS as Idx
		    {
		      accepts[j as usize] &= !(*dfa).word_char[j as usize];
		      any_set |= accepts[j as usize];
		      j += 1;
		    }
		}
	      if any_set == 0
		{ i += 1; continue; }
	    }
	}

      /* Then divide 'accepts' into DFA states, or create a new
	 state.  Above, we make sure that accepts is not empty.  */
      j = 0;
      while j < ndests
	{
	  let mut intersec: bitset_t = std::mem::zeroed (); /* Intersection sets, see below.  */
	  let mut remains: bitset_t = std::mem::zeroed ();
	  /* Flags, see below.  */
	  let mut has_intersec: bitset_word_t;
	  let mut not_subset: bitset_word_t;
	  let mut not_consumed: bitset_word_t;

	  /* Optimization, skip if this state doesn't accept the character.  */
	  if r#type == CHARACTER as c_uint
	      && !bitset_contain ((*dests_ch.add (j as usize)).as_ptr (), (*node).opr.c as Idx)
	    { j += 1; continue; }

	  /* Enumerate the intersection set of this state and 'accepts'.  */
	  has_intersec = 0;
	  k = 0;
	  while k < BITSET_WORDS as Idx
	    {
	      intersec[k as usize] = accepts[k as usize]
                                     & (*dests_ch.add (j as usize))[k as usize];
	      has_intersec |= intersec[k as usize];
	      k += 1;
	    }
	  /* And skip if the intersection set is empty.  */
	  if has_intersec == 0
	    { j += 1; continue; }

	  /* Then check if this state is a subset of 'accepts'.  */
	  not_subset = 0;
	  not_consumed = 0;
	  k = 0;
	  while k < BITSET_WORDS as Idx
	    {
	      remains[k as usize] = !accepts[k as usize]
                                    & (*dests_ch.add (j as usize))[k as usize];
	      not_subset |= remains[k as usize];
	      accepts[k as usize] = accepts[k as usize]
                                    & !(*dests_ch.add (j as usize))[k as usize];
	      not_consumed |= accepts[k as usize];
	      k += 1;
	    }

	  /* If this state isn't a subset of 'accepts', create a
	     new group state, which has the 'remains'. */
	  if not_subset != 0
	    {
	      bitset_copy ((*dests_ch.add (ndests as usize)).as_mut_ptr (), remains.as_ptr ());
	      bitset_copy ((*dests_ch.add (j as usize)).as_mut_ptr (), intersec.as_ptr ());
	      err = re_node_set_init_copy (dests_node.add (ndests as usize),
                                           dests_node.add (j as usize));
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break 'error_return; }
	      ndests += 1;
	    }

	  /* Put the position in the current group. */
	  ok = re_node_set_insert (dests_node.add (j as usize),
                                   *(*cur_nodes).elems.add (i as usize));
	  if __glibc_unlikely (! ok)
	    { break 'error_return; }

	  /* If all characters are consumed, go to next node. */
	  if not_consumed == 0
	    { break; }
	  j += 1;
	}
      /* Some characters remain, create a new group. */
      if j == ndests
	{
	  bitset_copy ((*dests_ch.add (ndests as usize)).as_mut_ptr (), accepts.as_ptr ());
	  err = re_node_set_init_1 (dests_node.add (ndests as usize),
                                    *(*cur_nodes).elems.add (i as usize));
	  if __glibc_unlikely (err != REG_NOERROR)
	    { break 'error_return; }
	  ndests += 1;
	  bitset_empty (accepts.as_mut_ptr ());
	}
      i += 1;
    }
  assume! (ndests <= SBC_MAX);
  return ndests;
  } /* 'error_return */

  /* error_return: */
  j = 0;
  while j < ndests
    {
      re_node_set_free! (dests_node.add (j as usize));
      j += 1;
    }
  return -1;
}

unsafe fn check_node_accept_bytes (dfa: *const re_dfa_t, node_idx: Idx,
				   input: *const re_string_t, str_idx: Idx) -> c_int
{
  let node: *const re_token_t = (*dfa).nodes.add (node_idx as usize) as *const re_token_t;
  let mut char_len: c_int;
  let mut elem_len: c_int;
  let mut i: Idx;

  if __glibc_unlikely (type_ (node) == OP_UTF8_PERIOD as c_uint)
    {
      let c: u8 = re_string_byte_at! (input, str_idx);
      let mut d: u8;
      if __glibc_likely (c < 0xc2)
	{ return 0; }

      if str_idx + 2 > (*input).len
	{ return 0; }

      d = re_string_byte_at! (input, str_idx + 1);
      if c < 0xe0
	{ return if d < 0x80 || d > 0xbf { 0 } else { 2 }; }
      else if c < 0xf0
	{
	  char_len = 3;
	  if c == 0xe0 && d < 0xa0
	    { return 0; }
	}
      else if c < 0xf8
	{
	  char_len = 4;
	  if c == 0xf0 && d < 0x90
	    { return 0; }
	}
      else if c < 0xfc
	{
	  char_len = 5;
	  if c == 0xf8 && d < 0x88
	    { return 0; }
	}
      else if c < 0xfe
	{
	  char_len = 6;
	  if c == 0xfc && d < 0x84
	    { return 0; }
	}
      else
	{ return 0; }

      if str_idx + char_len as Idx > (*input).len
	{ return 0; }

      i = 1;
      while i < char_len as Idx
	{
	  d = re_string_byte_at! (input, str_idx + i);
	  if d < 0x80 || d > 0xbf
	    { return 0; }
	  i += 1;
	}
      return char_len;
    }

  char_len = re_string_char_size_at (input, str_idx);
  if type_ (node) == OP_PERIOD as c_uint
    {
      if char_len <= 1
	{ return 0; }
      /* FIXME: I don't think this if is needed, as both '\n'
	 and '\0' are char_len == 1.  */
      /* '.' accepts any one character except the following two cases.  */
      if ((*dfa).syntax & RE_DOT_NEWLINE == 0
	   && re_string_byte_at! (input, str_idx) == b'\n' as u8)
	  || ((*dfa).syntax & RE_DOT_NOT_NULL != 0
	      && re_string_byte_at! (input, str_idx) == b'\0' as u8)
	{ return 0; }
      return char_len;
    }

  elem_len = re_string_elem_size_at (input, str_idx);
  if (elem_len <= 1 && char_len <= 1) || char_len == 0
    { return 0; }

  if type_ (node) == COMPLEX_BRACKET as c_uint
    {
      let cset: *const re_charset_t = (*node).opr.mbcset as *const re_charset_t;
      // C: `#ifdef _LIBC` (regexec.c 3778-3805) -- dead in the frozen configuration.
      let mut match_len: c_int = 0;
      let wc: wchar_t = (if (*cset).nranges != 0 || (*cset).nchar_classes != 0
                            || (*cset).nmbchars != 0
			   { re_string_wchar_at (input, str_idx) } else { 0 }) as wchar_t;

      'check_node_accept_bytes_match: {
      /* match with multibyte character?  */
      i = 0;
      while i < (*cset).nmbchars
	{
	  if wc == *(*cset).mbchars.add (i as usize)
	    {
	      match_len = char_len;
	      break 'check_node_accept_bytes_match;
	    }
	  i += 1;
	}
      /* match with character_class?  */
      i = 0;
      while i < (*cset).nchar_classes
	{
	  let wt: wctype_t = *(*cset).char_classes.add (i as usize);
	  if __iswctype! (wc as wint_t, wt) != 0
	    {
	      match_len = char_len;
	      break 'check_node_accept_bytes_match;
	    }
	  i += 1;
	}

      // C: `#ifdef _LIBC` (regexec.c 3806-3895) -- the `nrules != 0` arm is dead; the live
      // text is the `#else` arm's range-expression test below.
	{
	  /* match with range expression?  */
	  i = 0;
	  while i < (*cset).nranges
	    {
	      if *(*cset).range_starts.add (i as usize) <= wc
		  && wc <= *(*cset).range_ends.add (i as usize)
		{
		  match_len = char_len;
		  break 'check_node_accept_bytes_match;
		}
	      i += 1;
	    }
	}
      }
      /* C: `check_node_accept_bytes_match:`.  */
      if non_match_ (cset) == 0
	{ return match_len; }
      else
	{
	  if match_len > 0
	    { return 0; }
	  else
	    { return if elem_len > char_len { elem_len } else { char_len }; }
	}
    }
  return 0;
}

// C: `#ifdef _LIBC` (regexec.c 3921-3979) `find_collation_sequence_value` -- dead.

/* Check whether the node accepts the byte which is IDX-th
   byte of the INPUT.  */

unsafe fn check_node_accept (mctx: *const re_match_context_t, node: *const re_token_t,
			     idx: Idx) -> bool
{
  let ch: u8;
  ch = re_string_byte_at! (&(*mctx).input, idx);
  let r#type: c_uint = type_ (node);
  if r#type == CHARACTER as c_uint
    {
      if (*node).opr.c != ch
        { return false; }
    }
  else if r#type == SIMPLE_BRACKET as c_uint
    {
      if !bitset_contain ((*node).opr.sbcset, ch as Idx)
        { return false; }
    }
  else if r#type == OP_UTF8_PERIOD as c_uint || r#type == OP_PERIOD as c_uint
    {
      /* C: `FALLTHROUGH` (regexec.c 3952): the OP_UTF8_PERIOD arm shares this body.  */
      if r#type == OP_UTF8_PERIOD as c_uint && ch as c_int >= ASCII_CHARS
        { return false; }
      if (ch == b'\n' as u8 && (*((*mctx).dfa)).syntax & RE_DOT_NEWLINE == 0)
	  || (ch == b'\0' as u8 && (*((*mctx).dfa)).syntax & RE_DOT_NOT_NULL != 0)
	{ return false; }
    }
  else
    { return false; }

  if constraint_ (node) != 0
    {
      /* The node has constraints.  Check whether the current context
	 satisfies the constraints.  */
      let context: c_uint = re_string_context_at (&(*mctx).input, idx,
						   (*mctx).eflags);
      if NOT_SATISFY_NEXT_CONSTRAINT! (constraint_ (node), context)
	{ return false; }
    }

  return true;
}

/* Extend the buffers, if the buffers have run out.  */

unsafe fn extend_buffers (mctx: *mut re_match_context_t, min_len: c_int) -> reg_errcode_t
{
  let mut ret: reg_errcode_t;
  let pstr: *mut re_string_t = &mut (*mctx).input;

  /* Avoid overflow.  */
  if __glibc_unlikely (MIN! (IDX_MAX, (SIZE_MAX / std::mem::size_of::<*mut re_dfastate_t> ())
                                          as Idx) / 2
			<= (*pstr).bufs_len)
    { return REG_ESPACE; }

  /* Double the lengths of the buffers, but allocate at least MIN_LEN.  */
  ret = re_string_realloc_buffers (pstr,
				   MAX! (min_len as Idx,
					MIN! ((*pstr).len, (*pstr).bufs_len * 2)));
  if __glibc_unlikely (ret != REG_NOERROR)
    { return ret; }

  if !(*mctx).state_log.is_null ()
    {
      /* And double the length of state_log.  */
      /* XXX We have no indication of the size of this buffer.  If this
	 allocation fail we have no indication that the state_log array
	 does not have the right size.  */
      let new_array: *mut *mut re_dfastate_t =
        re_realloc! ((*mctx).state_log, *mut re_dfastate_t, (*pstr).bufs_len + 1);
      if __glibc_unlikely (new_array.is_null ())
	{ return REG_ESPACE; }
      (*mctx).state_log = new_array;
    }

  /* Then reconstruct the buffers.  */
  if (*pstr).icase != 0
    {
      if (*pstr).mb_cur_max > 1
	{
	  ret = build_wcs_upper_buffer (pstr);
	  if __glibc_unlikely (ret != REG_NOERROR)
	    { return ret; }
	}
      else
	{ build_upper_buffer (pstr); }
    }
  else
    {
      if (*pstr).mb_cur_max > 1
	{ build_wcs_buffer (pstr); }
      else
	{
	  if !(*pstr).trans.is_null ()
	    { let _ = re_string_translate_buffer (pstr); }
	}
    }
  return REG_NOERROR;
}


/* Functions for matching context.  */

/* Initialize MCTX.  */

unsafe fn match_ctx_init (mctx: *mut re_match_context_t, eflags: c_int, n: Idx)
			  -> reg_errcode_t
{
  (*mctx).eflags = eflags;
  (*mctx).match_last = -1;
  if n > 0
    {
      /* Avoid overflow.  */
      let max_object_size: usize =
	MAX! (std::mem::size_of::<re_backref_cache_entry> (),
	      std::mem::size_of::<*mut re_sub_match_top_t> ());
      if __glibc_unlikely (MIN! (IDX_MAX, (SIZE_MAX / max_object_size) as Idx) < n)
	{ return REG_ESPACE; }

      (*mctx).bkref_ents = re_malloc! (re_backref_cache_entry, n);
      (*mctx).sub_tops = re_malloc! (*mut re_sub_match_top_t, n);
      if __glibc_unlikely ((*mctx).bkref_ents.is_null () || (*mctx).sub_tops.is_null ())
	{ return REG_ESPACE; }
    }
  /* Already zero-ed by the caller.
     else
       mctx->bkref_ents = NULL;
     mctx->nbkref_ents = 0;
     mctx->nsub_tops = 0;  */
  (*mctx).abkref_ents = n;
  (*mctx).max_mb_elem_len = 1;
  (*mctx).asub_tops = n;
  return REG_NOERROR;
}

/* Clean the entries which depend on the current input in MCTX.
   This function must be invoked when the matcher changes the start index
   of the input, or changes the input string.  */

unsafe fn match_ctx_clean (mctx: *mut re_match_context_t)
{
  let mut st_idx: Idx;
  st_idx = 0;
  while st_idx < (*mctx).nsub_tops
    {
      let mut sl_idx: Idx;
      let top: *mut re_sub_match_top_t = *(*mctx).sub_tops.add (st_idx as usize);
      sl_idx = 0;
      while sl_idx < (*top).nlasts
	{
	  let last: *mut re_sub_match_last_t = *(*top).lasts.add (sl_idx as usize);
	  re_free! ((*last).path.array);
	  re_free! (last);
	  sl_idx += 1;
	}
      re_free! ((*top).lasts);
      if !(*top).path.is_null ()
	{
	  re_free! ((*(*top).path).array);
	  re_free! ((*top).path);
	}
      re_free! (top);
      st_idx += 1;
    }

  (*mctx).nsub_tops = 0;
  (*mctx).nbkref_ents = 0;
}

/* Free all the memory associated with MCTX.  */

unsafe fn match_ctx_free (mctx: *mut re_match_context_t)
{
  /* First, free all the memory associated with MCTX->SUB_TOPS.  */
  match_ctx_clean (mctx);
  re_free! ((*mctx).sub_tops);
  re_free! ((*mctx).bkref_ents);
}

/* Add a new backreference entry to MCTX.
   Note that we assume that caller never call this function with duplicate
   entry, and call with STR_IDX which isn't smaller than any existing entry.
*/

unsafe fn match_ctx_add_entry (mctx: *mut re_match_context_t, node: Idx, str_idx: Idx,
			       from: Idx, to: Idx) -> reg_errcode_t
{
  if (*mctx).nbkref_ents >= (*mctx).abkref_ents
    {
      let new_entry: *mut re_backref_cache_entry;
      new_entry = re_realloc! ((*mctx).bkref_ents, re_backref_cache_entry,
			      (*mctx).abkref_ents * 2);
      if __glibc_unlikely (new_entry.is_null ())
	{
	  re_free! ((*mctx).bkref_ents);
	  return REG_ESPACE;
	}
      (*mctx).bkref_ents = new_entry;
      memset ((*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize) as *mut c_void,
              b'\0' as c_int,
	      std::mem::size_of::<re_backref_cache_entry> ()
              * (*mctx).abkref_ents as usize);
      (*mctx).abkref_ents *= 2;
    }
  if (*mctx).nbkref_ents > 0
      && (*(*mctx).bkref_ents.add (((*mctx).nbkref_ents - 1) as usize)).str_idx == str_idx
    { (*(*mctx).bkref_ents.add (((*mctx).nbkref_ents - 1) as usize)).more = 1; }

  (*(*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize)).node = node;
  (*(*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize)).str_idx = str_idx;
  (*(*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize)).subexp_from = from;
  (*(*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize)).subexp_to = to;

  /* This is a cache that saves negative results of check_dst_limits_calc_pos.
     If bit N is clear, means that this entry won't epsilon-transition to
     an OP_OPEN_SUBEXP or OP_CLOSE_SUBEXP for the N+1-th subexpression.  If
     it is set, check_dst_limits_calc_pos_1 will recurse and try to find one
     such node.

     A backreference does not epsilon-transition unless it is empty, so set
     to all zeros if FROM != TO.  */
  (*(*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize)).eps_reachable_subexps_map
    = if from == to { (-1isize) as bitset_word_t } else { 0 };

  (*(*mctx).bkref_ents.add ((*mctx).nbkref_ents as usize)).more = 0;
  (*mctx).nbkref_ents += 1;
  if (*mctx).max_mb_elem_len < (to - from) as c_int
    { (*mctx).max_mb_elem_len = (to - from) as c_int; }
  return REG_NOERROR;
}

/* Return the first entry with the same str_idx, or -1 if none is
   found.  Note that MCTX->BKREF_ENTS is already sorted by MCTX->STR_IDX.  */

unsafe fn search_cur_bkref_entry (mctx: *const re_match_context_t, str_idx: Idx) -> Idx
{
  let mut left: Idx;
  let mut right: Idx;
  let mut mid: Idx;
  let last: Idx;
  last = (*mctx).nbkref_ents;
  right = (*mctx).nbkref_ents;
  left = 0;
  while left < right
    {
      mid = (left + right) / 2;
      if (*(*mctx).bkref_ents.add (mid as usize)).str_idx < str_idx
	{ left = mid + 1; }
      else
	{ right = mid; }
    }
  if left < last && (*(*mctx).bkref_ents.add (left as usize)).str_idx == str_idx
    { return left; }
  else
    { return -1; }
}

/* Register the node NODE, whose type is OP_OPEN_SUBEXP, and which matches
   at STR_IDX.  */

unsafe fn match_ctx_add_subtop (mctx: *mut re_match_context_t, node: Idx, str_idx: Idx)
				-> reg_errcode_t
{
  DEBUG_ASSERT! (!(*mctx).sub_tops.is_null ());
  DEBUG_ASSERT! ((*mctx).asub_tops > 0);
  if __glibc_unlikely ((*mctx).nsub_tops == (*mctx).asub_tops)
    {
      let new_asub_tops: Idx = (*mctx).asub_tops * 2;
      let new_array: *mut *mut re_sub_match_top_t =
        re_realloc! ((*mctx).sub_tops, *mut re_sub_match_top_t, new_asub_tops);
      if __glibc_unlikely (new_array.is_null ())
	{ return REG_ESPACE; }
      (*mctx).sub_tops = new_array;
      (*mctx).asub_tops = new_asub_tops;
    }
  *(*mctx).sub_tops.add ((*mctx).nsub_tops as usize) =
    calloc (1, std::mem::size_of::<re_sub_match_top_t> ()) as *mut re_sub_match_top_t;
  if __glibc_unlikely ((*(*mctx).sub_tops.add ((*mctx).nsub_tops as usize)).is_null ())
    { return REG_ESPACE; }
  (**(*mctx).sub_tops.add ((*mctx).nsub_tops as usize)).node = node;
  (**(*mctx).sub_tops.add ((*mctx).nsub_tops as usize)).str_idx = str_idx;
  (*mctx).nsub_tops += 1;
  return REG_NOERROR;
}

/* Register the node NODE, whose type is OP_CLOSE_SUBEXP, and which matches
   at STR_IDX, whose corresponding OP_OPEN_SUBEXP is SUB_TOP.
   Return the new entry if successful, NULL if memory is exhausted.  */

unsafe fn match_ctx_add_sublast (subtop: *mut re_sub_match_top_t, node: Idx, str_idx: Idx)
				 -> *mut re_sub_match_last_t
{
  let mut new_entry: *mut re_sub_match_last_t;
  if __glibc_unlikely ((*subtop).nlasts == (*subtop).alasts)
    {
      let new_alasts: Idx = 2 * (*subtop).alasts + 1;
      let new_array: *mut *mut re_sub_match_last_t =
        re_realloc! ((*subtop).lasts, *mut re_sub_match_last_t, new_alasts);
      if __glibc_unlikely (new_array.is_null ())
	{ return null_mut (); }
      (*subtop).lasts = new_array;
      (*subtop).alasts = new_alasts;
    }
  new_entry = calloc (1, std::mem::size_of::<re_sub_match_last_t> ())
              as *mut re_sub_match_last_t;
  if __glibc_likely (!new_entry.is_null ())
    {
      *(*subtop).lasts.add ((*subtop).nlasts as usize) = new_entry;
      (*new_entry).node = node;
      (*new_entry).str_idx = str_idx;
      (*subtop).nlasts += 1;
    }
  return new_entry;
}

unsafe fn sift_ctx_init (sctx: *mut re_sift_context_t, sifted_sts: *mut *mut re_dfastate_t,
			 limited_sts: *mut *mut re_dfastate_t, last_node: Idx,
			 last_str_idx: Idx)
{
  (*sctx).sifted_states = sifted_sts;
  (*sctx).limited_states = limited_sts;
  (*sctx).last_node = last_node;
  (*sctx).last_str_idx = last_str_idx;
  re_node_set_init_empty! (&raw mut (*sctx).limits);
}
