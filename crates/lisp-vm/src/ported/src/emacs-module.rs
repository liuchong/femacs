// femacs: port of src/emacs-module.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same
// error model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`.
//
// This file is one of P-86's class: the frozen configuration compiles no `emacs-module.o`.  Its
// object sits behind `HAVE_MODULES` in `emacs/src/Makefile.in`'s module object list, and
// `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
// (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so
// `docs/baseline/frozen-build-sources.txt` lists no `emacs-module.o` and `extract-abi.py`'s
// checklist for the file is empty (P-86 ②).  `docs/ports/emacs-module.md` therefore carries the
// substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
//
// The gates, and both arms of each (P-86 ①):
//
//   * the **file gate** is `feature = "have_modules"` (`HAVE_MODULES`, `undef` in the frozen
//     macro table), on every item *and on C 78-96's include graph* (P-86 ⑧) - which is why the
//     frozen configuration compiles nothing here;
//   * C 112-115's `#ifdef WINDOWSNT` arm is `feature = "windowsnt"`;
//   * C 970-974's `#ifdef HAVE_WINDOW_SYSTEM` arm is `feature = "have_window_system"`;
//   * C 1076-1083's `#if GCC_LINT && __GNUC__ && !__clang__` arm is
//     `feature = "gcc_lint_not_clang"` (the whole triple is one feature: `GCC_LINT` is 1 in the
//     frozen macro table but `__clang__` is also defined, so the condition is false there - and
//     `src/eval.rs` 1604-1606 already spells this exact triple that way).
//
// **Acquisition note - this port is `text-only` (P-86 ③⑵).**  This machine has no Windows SDK
// and no mingw toolchain, so the two `windowsnt` lines (C 113-114) cannot be checked against a
// real `<windows.h>`; but the file's own body needs no Windows header for anything below, and
// every other gate is a `cfg` over text this unit already carries.  Nothing here is compiled or
// linked against a module host, so the `emacs_env` field assignments in
// `initialize_environment` are not ABI-measured here: their judge is the generated
// `emacs-module.h` the header mirror is built from (`docs/ports/include-emacs-module.in.md`).
//
// `docs/ports/emacs-module.md` carries the C line ranges, the acceptance readings, every
// deviation and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unused_parens, unused_mut, unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals, unused_must_use, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

/*
The public module API is defined in the header emacs-module.h.  The
configure script generates emacs-module.h from emacs-module.in.h and
the version-specific environment fragments in module-env-*.h.

If you want to change the module API, please abide to the following
rules:

- Don't remove publicly documented declarations from the headers.

- Don't remove, reorder, or rename structure fields, as such changes
  break ABI compatibility.

- Don't change the types of structure fields.

- Likewise, the presence, order, and type of structure fields may not
  depend on preprocessor macros.

- Add structure fields only at the end of structures.

- For every Emacs major version there is a new fragment file
  module-env-VER.h.  Add functions solely at the end of the fragment
  file for the next (not yet released) major version of Emacs.  For
  example, if the current Emacs release is 26.2, add functions only to
  module-env-27.h.

- emacs-module.h should only depend on standard C headers.  In
  particular, don't include config.h or lisp.h from emacs-module.h.

- The contents of emacs-module.h should be the same on all platforms
  and architectures.

- emacs-module.h may not depend on Emacs configuration options.

- Prefix all names in emacs-module.h with "emacs_" or "EMACS_".

To add a new module function, proceed as follows:

1. Add a new function pointer field at the end of the module-env-*.h
   file for the next major version of Emacs.

2. Run config.status or configure to regenerate emacs-module.h.

3. Create a corresponding implementation function in this file.  See
   "Implementation of runtime and environment functions" below for
   further rules.

4. Assign the new field in the initialize_environment function.

5. Add a test function that calls your new function to
   test/data/emacs-module/mod-test.c.  Add a unit test that invokes
   your new test function to test/src/emacs-module-tests.el.

6. Document your new function in the manual and in etc/NEWS.
*/

// [U1] config.rs provided by common

/* C 78-96: the file's own include list.  Every header sits inside the file gate, as the C build
   does - no translation unit the frozen configuration builds reaches any one of them.  */
#[cfg (feature = "have_modules")]
const _: () = ();
/* C 80: `#include <stdarg.h>` - the port's equivalent is `core::ffi::VaList` (PORTING.md P-04;
   the `src/doprnt.rs` 56-57 reading), so there is no mirror and the `...` bodies below
   name `VaList` directly.  */
#[cfg (feature = "have_modules")]
use core::ffi::VaList;                                // C 80: `#include <stdarg.h>`
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
// The terminal include graph uses only keyboard.h's two opaque buffer pointers.
// Window-system headers already provide the complete record.
#[cfg (all (feature = "have_modules", not (feature = "have_window_system")))]
type buffer = c_void;
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();
#[cfg (feature = "have_modules")]
const _: () = ();

/* ------------------------------------------------------------------------------------------
   C 112-115: `#ifdef WINDOWSNT` / `#include <windows.h>` / `#include "w32term.h"` / `#endif`.
   The Windows SDK header has no mirror and nothing below reads a name out of it (C's only use
   of the two lines is to make the Win32 declarations visible to the rest of the file), so the
   arm is carried as its two comments and one mirror (P-88 ①: one consumer, declared here).
   ------------------------------------------------------------------------------------------ */
/* C 113: `#include <windows.h>` - the Windows SDK header, whose declarations the port reaches
   through `w32term.rs` and the unit's own use sites; no mirror exists for it.  */
#[cfg (feature = "windowsnt")]
const _: () = ();

/* C 98-107: the comment on the two allocation strategies (assertions off -> the stack;
   assertions on -> the free store, never freed, so every object has a distinct address).  */

/* C 117-118: `typedef int (*emacs_init_function) (struct emacs_runtime *);` - the module init
   function's type; C's bare function pointer is the port's nullable `Option` (PORTING.md 1.4,
   P-27), and C's `if (!module_init)` at C 1243 reads it as such.  */
#[cfg (feature = "have_modules")]
pub type emacs_init_function
  = Option<unsafe extern "C" fn (runtime: *mut emacs_runtime) -> c_int>;

/* C 120-155: `struct emacs_value_tag`, `enum { value_frame_size }` and the two frame
   structures.  `emacs_value` is `*mut c_void` in the header mirror (the tag is C 125's record,
   defined by *this* unit - `include/emacs-module.in.rs` 97-102), so every `v->v` below is
   `(*v.cast::<emacs_value_tag> ()).v` and every `&…->value` is an `as emacs_value` cast
   (`## Deviations` 1).  */
#[cfg (feature = "have_modules")]
#[repr(C)]
pub struct emacs_value_tag { pub v: Lisp_Object }

/* C 133: `enum { value_frame_size = 512 };` */
#[cfg (feature = "have_modules")]
pub const value_frame_size: c_int = 512;

#[cfg (feature = "have_modules")]
#[repr(C)]
pub struct emacs_value_frame
{
  /* Storage for values.  */
  pub objects: [emacs_value_tag; value_frame_size as usize],

  /* Index of the next free value in `objects'.  */
  pub offset: c_int,

  /* Pointer to next frame, if any.  */
  pub next: *mut emacs_value_frame,
}

#[cfg (feature = "have_modules")]
#[repr(C)]
pub struct emacs_value_storage
{
  pub initial: emacs_value_frame,
  pub current: *mut emacs_value_frame,
}

/* C 160-180: the private halves.  `emacs_env_private::pending_non_local_exit` is C's
   `enum emacs_funcall_exit` (a plain `c_int` alias in the header mirror).  */
#[cfg (feature = "have_modules")]
#[repr(C)]
pub struct emacs_env_private
{
  pub pending_non_local_exit: emacs_funcall_exit,

  /* Dedicated storage for non-local exit symbol and data so that
     storage is always available for them, even in an out-of-memory
     situation.  */
  pub non_local_exit_symbol: Lisp_Object,
  pub non_local_exit_data: Lisp_Object,

  pub storage: emacs_value_storage,

  /* C4-R1: Non-ABI host-exit propagation flag recorded when a VM `HostExit(code)`
     hits an `extern "C"` module vtable boundary; prevents `non_local_exit_clear` from
     swallowing headless termination and resumes unwind cleanly after C returns.  */
  pub host_exit_pending: bool,
  pub host_exit_code: i32,
}

#[cfg (feature = "have_modules")]
#[repr(C)]
pub struct emacs_runtime_private
{
  pub env: *mut emacs_env,
}

/* C 183-206: C's forward declarations.  PORTING.md 0.1-12: Rust has no prototypes, so the list
   is dropped and each definition stands at its own C position (`## Deviations` 3).  */
/* C 207: `static bool module_assertions = false;` */
#[cfg (feature = "have_modules")]
static module_assertions: VmGlobal<bool> = VmGlobal::<bool>::new (false);

/* C 210-233: the two small helpers.  */
// C: coding.h 695-697: Lisp_Object decode_string_utf_8 (Lisp_Object, const char *, ptrdiff_t, Lisp_Object, bool, Lisp_Object, Lisp_Object)
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn decode_string_utf_8 (string: Lisp_Object, str_: *const c_char, len: ptrdiff_t,
                              handle_8_bit: Lisp_Object, handle_over_uni: bool,
                              handle_8_bit_coding: Lisp_Object,
                              handle_over_uni_coding: Lisp_Object) -> Lisp_Object;
}

/* C 212-225: `module_decode_utf_8`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_decode_utf_8 (str_: *const c_char, len: ptrdiff_t) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  /* We set HANDLE-8-BIT and HANDLE-OVER-UNI to nil to signal an error
     if the argument is not a valid UTF-8 string.  While it isn't
     documented how make_string and make_function behave in this case,
     signaling an error is the most defensive and obvious reaction. */
  let s: Lisp_Object
    = decode_string_utf_8 (Qnil (), str_, len, Qnil (), false, Qnil (), Qnil ());
  CHECK_TYPE (!NILP (s), Qutf_8_string_p (), make_string_from_utf8 (str_, len));
  return s;

  }
}

/* C 227-233: `module_memory_buffer_too_small`.  */
#[cfg (feature = "have_modules")]
fn module_memory_buffer_too_small (actual: ptrdiff_t, required: ptrdiff_t)
{
  {
  xsignal2 (Qmemory_buffer_too_small (), INT_TO_INTEGER (actual),
            INT_TO_INTEGER (required));

  }
}

/* C 236-264: the comment on the non-local-exit policy (module frames cannot be skipped by a
   long jump, so every exit is caught and the handler is pushed without ever signaling).  */

/* U3 explicit non-local exit boundary for external C dynamic modules:
   Every `emacs_env` vtable entry point remains `extern "C"` and intercepts VM `NonLocalExit`
   unwinds via `catch_vm_nonlocal`, translating them into `pending_non_local_exit` on `env` so
   zero stack unwinds ever cross C dynamic module frames.  */
#[cfg (feature = "have_modules")]
#[inline]
unsafe fn module_with_nonlocal_catch<R> (
  env: *mut emacs_env,
  error_retval: R,
  body: impl FnOnce () -> R,
) -> R
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  if module_non_local_exit_check (env) != emacs_funcall_exit_return
    { return error_retval; }
  let internal_handler: *mut handler =
    push_handler_nosignal (Qt (), CATCHER_ALL_DEBUGGABLE);
  if internal_handler.is_null ()
    {
      module_out_of_memory (env);
      return error_retval;
    }
  match crate::nonlocal::catch_vm_nonlocal_or_host_exit (body)
    {
      crate::nonlocal::HostUnwindOutcome::Ok (val) =>
        {
          module_reset_handlerlist (internal_handler);
          val
        }
      crate::nonlocal::HostUnwindOutcome::NonLocal (exit) =>
        {
          eassert! (exit.target == internal_handler);
          let _root = crate::nonlocal::ScopedLispRoot::new (&[exit.value]);
          module_handle_nonlocal_exit (env, exit.kind.to_raw (), exit.value);
          module_reset_handlerlist (internal_handler);
          error_retval
        }
      crate::nonlocal::HostUnwindOutcome::HostExit (code) =>
        {
          crate::eval::restore_vm_state_to_host_handler (internal_handler);
          module_reset_handlerlist (internal_handler);
          crate::host_runtime::clear_host_exit_unwinding ();
          let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
          (*p).host_exit_pending = true;
          (*p).host_exit_code = code;
          (*p).pending_non_local_exit = emacs_funcall_exit_signal;
          (*p).non_local_exit_symbol = Qerror ();
          (*p).non_local_exit_data = Qnil ();
          error_retval
        }
    }

  }
}

#[cfg (feature = "have_modules")]
#[inline]
unsafe fn module_with_nonlocal_catch_void (
  env: *mut emacs_env,
  body: impl FnOnce (),
)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, (), body);

  }
}

/* C 336-342: `MODULE_FUNCTION_BEGIN_NO_CATCH` (steps 2 and 3; `env` is passed, see above).  */
#[cfg (feature = "have_modules")]
macro_rules! MODULE_FUNCTION_BEGIN_NO_CATCH {
  ($env:ident, $error_retval:expr) => {
    module_assert_thread ();
    module_assert_env ($env);
    if module_non_local_exit_check ($env) != emacs_funcall_exit_return
      { return $error_retval; }
  };
  ($env:ident) => {
    module_assert_thread ();
    module_assert_env ($env);
    if module_non_local_exit_check ($env) != emacs_funcall_exit_return
      { return; }
  };
}

/* C 352-356: `CHECK_MODULE_FUNCTION`.  */
#[cfg (feature = "have_modules")]
fn CHECK_MODULE_FUNCTION (obj: Lisp_Object)
{
  CHECK_TYPE (MODULE_FUNCTIONP (obj), Qmodule_function_p (), obj);
}

/* C 358-362: `CHECK_USER_PTR`.  */
#[cfg (feature = "have_modules")]
fn CHECK_USER_PTR (obj: Lisp_Object)
{
  CHECK_TYPE (USER_PTRP (obj), Quser_ptrp (), obj);
}

/* C 364-374: `module_get_environment` (`private_members` is `*mut c_void` in the header
   mirror, so the cast names the record this unit's C 177 defines).  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_get_environment (runtime: *mut emacs_runtime) -> *mut emacs_env
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_runtime (runtime);
  return (*(*runtime).private_members.cast::<emacs_runtime_private> ()).env;

  }
}

/* C 376-381: `Vmodule_refs_hash` (C's `static` keeps C's internal linkage: not `pub`, no
   symbol - PORTING.md 0.2).  */
#[cfg (feature = "have_modules")]
static Vmodule_refs_hash: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (core::ptr::null_mut ());

/* C 383-397: `struct module_global_reference`.  */
#[cfg (feature = "have_modules")]
#[repr(C)]
pub struct module_global_reference
{
  /* Pseudovector header, must come first. */
  pub header: vectorlike_header,

  /* Holds the emacs_value for the object.  The Lisp_Object stored
     therein must be the same as the hash key.  */
  pub value: emacs_value_tag,

  /* Reference count, always positive.  */
  pub refcount: ptrdiff_t,
}

/* C 399-404: `XMODULE_GLOBAL_REFERENCE`.  */
#[cfg (feature = "have_modules")]
unsafe fn XMODULE_GLOBAL_REFERENCE (o: Lisp_Object) -> *mut module_global_reference
{
  eassert! (PSEUDOVECTORP (o, pvec_type::PVEC_OTHER));
  return XUNTAG! (o, Lisp_Type::Lisp_Vectorlike, module_global_reference);
}

/* C 406-423: `module_global_reference_p`.  C 421's `ckd_add (n, *n, h->count)` is inlined:
   the `ckd_add!` macro takes a bare identifier, and C's semantics leave `*n` alone on
   overflow (`src/buffer.rs` 3602-3607's reading).  */
#[cfg (feature = "have_modules")]
unsafe fn module_global_reference_p (v: emacs_value, n: *mut ptrdiff_t) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let h: *mut Lisp_Hash_Table = XHASH_TABLE ((*Vmodule_refs_hash.as_mut_ptr ()));
  /* Note that we can't use `hash_find' because V might be a local
     reference that's identical to some global reference.  */
  DOHASH! (h, k, val, {
    /* C 417: `if (&XMODULE_GLOBAL_REFERENCE (val)->value == v)` - the tag pointer is
       `emacs_value` again (the header mirror's `*mut c_void`, `## Deviations` 1).  */
    if &raw mut (*XMODULE_GLOBAL_REFERENCE (val)).value as emacs_value == v
      { return true; }
  });
  /* Only used for debugging, so we don't care about overflow, just
     make sure the operation is defined.  */
  let (n_new, n_overflow) = (*n).overflowing_add ((*h).count as ptrdiff_t);
  if !n_overflow
    { *n = n_new; }
  return false;

  }
}

/* C 425-460: `module_make_global_ref`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_global_ref (env: *mut emacs_env, value: emacs_value) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || unsafe {
    let h: *mut Lisp_Hash_Table = XHASH_TABLE ((*Vmodule_refs_hash.as_mut_ptr ()));
    let new_obj: Lisp_Object = value_to_lisp (value);
    let mut hashcode: hash_hash_t = 0;
    let i: ptrdiff_t = hash_find_get_hash (h, new_obj, &mut hashcode);

    /* Note: This approach requires the garbage collector to never move
       objects.  */

    if i >= 0
      {
        let value: Lisp_Object = HASH_VALUE (h, i as hash_idx_t);
        let r#ref: *mut module_global_reference = XMODULE_GLOBAL_REFERENCE (value);
        /* C 441: `bool overflow = ckd_add (&ref->refcount, ref->refcount, 1);` */
        let (refcount_new, overflow) = (*r#ref).refcount.overflowing_add (1);
        if overflow
          { overflow_error (); }
        (*r#ref).refcount = refcount_new;
        return &raw mut (*r#ref).value as emacs_value;
      }
    else
      {
        let r#ref: *mut module_global_reference
          = ALLOCATE_PLAIN_PSEUDOVECTOR! (module_global_reference, pvec_type::PVEC_OTHER);
        (*r#ref).value.v = new_obj;
        (*r#ref).refcount = 1;
        let mut value: Lisp_Object;
        XSETPSEUDOVECTOR! (value, r#ref, pvec_type::PVEC_OTHER);
        hash_put (h, new_obj, value, hashcode);
        return &raw mut (*r#ref).value as emacs_value;
      }
  })

  }
}

/* C 462-491: `module_free_global_ref`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_free_global_ref (env: *mut emacs_env, global_value: emacs_value)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  /* TODO: This probably never signals.  */
  /* FIXME: Wait a minute.  Shouldn't this function report an error if
     the hash lookup fails?  */
  module_with_nonlocal_catch_void (env, || unsafe {
    let h: *mut Lisp_Hash_Table = XHASH_TABLE ((*Vmodule_refs_hash.as_mut_ptr ()));
    let obj: Lisp_Object = value_to_lisp (global_value);
    let i: ptrdiff_t = hash_find (h, obj);

    if (*module_assertions.as_mut_ptr ())
      {
        let mut n: ptrdiff_t = 0;
        if ! module_global_reference_p (global_value, &mut n)
          { module_abort (c"Global value was not found in list of %td globals".as_ptr (),
                          n); }
      }

    if i >= 0
      {
        let value: Lisp_Object = HASH_VALUE (h, i as hash_idx_t);
        let r#ref: *mut module_global_reference = XMODULE_GLOBAL_REFERENCE (value);
        eassert! (0 < (*r#ref).refcount);
        (*r#ref).refcount -= 1;
        if (*r#ref).refcount == 0
          { hash_remove_from_table (h, obj); }
      }
  });

  }
}

/* C 493-499: `module_non_local_exit_check`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_non_local_exit_check (env: *mut emacs_env) -> emacs_funcall_exit
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  return (*(*env).private_members.cast::<emacs_env_private> ()).pending_non_local_exit;

  }
}

/* C 501-507: `module_non_local_exit_clear`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_non_local_exit_clear (env: *mut emacs_env)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  if !(*p).host_exit_pending
    { (*p).pending_non_local_exit = emacs_funcall_exit_return; }

  }
}

/* C 509-510: the two dedicated storage slots.  */
#[cfg (feature = "have_modules")]
static module_out_of_memory_symbol: VmGlobal<emacs_value_tag> = VmGlobal::<emacs_value_tag>::new (emacs_value_tag { v: core::ptr::null_mut () });
#[cfg (feature = "have_modules")]
static module_out_of_memory_data: VmGlobal<emacs_value_tag> = VmGlobal::<emacs_value_tag>::new (emacs_value_tag { v: core::ptr::null_mut () });

/* C 512-536: `module_non_local_exit_get`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_non_local_exit_get (env: *mut emacs_env,
                                     symbol: *mut emacs_value,
                                     data: *mut emacs_value) -> emacs_funcall_exit
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  let mut ret: emacs_funcall_exit = (*p).pending_non_local_exit;
  if ret != emacs_funcall_exit_return
    {
      let mut sym: emacs_value
        = allocate_emacs_value (env, (*p).non_local_exit_symbol);
      let mut dat: emacs_value
        = allocate_emacs_value (env, (*p).non_local_exit_data);
      if sym.is_null () || dat.is_null ()
        {
          sym = module_out_of_memory_symbol.as_mut_ptr () as emacs_value;
          dat = module_out_of_memory_data.as_mut_ptr () as emacs_value;
          ret = emacs_funcall_exit_signal;
        }
      *symbol = sym;
      *data = dat;
    }
  return ret;

  }
}

/* C 538-548: `module_non_local_exit_signal`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_non_local_exit_signal (env: *mut emacs_env,
                                        symbol: emacs_value, data: emacs_value)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  if module_non_local_exit_check (env) == emacs_funcall_exit_return
    { module_non_local_exit_signal_1 (env, value_to_lisp (symbol),
                                      value_to_lisp (data)); }

  }
}

/* C 550-558: `module_non_local_exit_throw`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_non_local_exit_throw (env: *mut emacs_env, tag: emacs_value,
                                       value: emacs_value)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  if module_non_local_exit_check (env) == emacs_funcall_exit_return
    { module_non_local_exit_throw_1 (env, value_to_lisp (tag),
                                     value_to_lisp (value)); }

  }
}

/* C 560-579: `struct Lisp_Module_Function`.  `include/lisp.rs` 2350-2355 spells the type as an
   incomplete pointer target at the *crate root* (lisp.h only uses it through pointers, C 4869's
   comment), so the definition takes the `make_string_from_utf8_def` device (`src/coding.rs`
   7372-7383): it lives in this module and the unit's own references spell the path.  Reported as
   a mirror gap in `## Owed` 1; `## Deviations` 4.  */
#[cfg (feature = "have_modules")]
pub mod lisp_module_function_def
{
  use super::*;

  #[repr(C, align(8))]
  pub struct Lisp_Module_Function
  {
    pub header: vectorlike_header,

    /* Fields traced by GC; these must come first.  */
    pub documentation: Lisp_Object, pub interactive_form: Lisp_Object,
    pub command_modes: Lisp_Object,

    /* Fields ignored by GC.  */
    pub min_arity: ptrdiff_t, pub max_arity: ptrdiff_t,
    pub subr: emacs_function,
    pub data: *mut c_void,
    pub finalizer: emacs_finalizer,
  }
}

/* C 581-586: `allocate_module_function`.  */
#[cfg (feature = "have_modules")]
unsafe fn allocate_module_function () -> *mut lisp_module_function_def::Lisp_Module_Function
{
  return ALLOCATE_PSEUDOVECTOR! (lisp_module_function_def::Lisp_Module_Function,
                                 command_modes, pvec_type::PVEC_MODULE_FUNCTION);
}

/* C 588-589: `XSET_MODULE_FUNCTION`.  */
#[cfg (feature = "have_modules")]
macro_rules! XSET_MODULE_FUNCTION {
  ($var:expr, $ptr:expr) => {
    XSETPSEUDOVECTOR! ($var, $ptr, pvec_type::PVEC_MODULE_FUNCTION)
  };
}

/* C 591-627: `module_make_function`.  C's `Lisp_Object result;` (a declaration with no
   initializer) gets C's zero value (PORTING.md 0.1-13).  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_function (env: *mut emacs_env, min_arity: ptrdiff_t,
                                max_arity: ptrdiff_t, func: emacs_function,
                                docstring: *const c_char, data: *mut c_void) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || unsafe {
    if ! (0 <= min_arity
          && (if max_arity < 0
                { min_arity <= MOST_POSITIVE_FIXNUM
                  && max_arity == emacs_variadic_function as ptrdiff_t }
              else
                { min_arity <= max_arity
                  && max_arity <= MOST_POSITIVE_FIXNUM }))
      { xsignal2 (Qinvalid_arity (), make_fixnum (min_arity), make_fixnum (max_arity)); }

    let function: *mut lisp_module_function_def::Lisp_Module_Function
      = allocate_module_function ();
    (*function).min_arity = min_arity;
    (*function).max_arity = max_arity;
    (*function).subr = func;
    (*function).data = data;
    (*function).finalizer = None;

    if !docstring.is_null ()
      { (*function).documentation
          = module_decode_utf_8 (docstring, strlen (docstring) as ptrdiff_t); }

    let mut result: Lisp_Object = core::ptr::null_mut ();
    XSET_MODULE_FUNCTION! (result, function);
    eassert! (MODULE_FUNCTIONP (result));

    lisp_to_value (env, result)
  })

  }
}

/* C 629-637: `module_get_function_finalizer`.  `XMODULE_FUNCTION` (the mirror) returns the
   crate-root incomplete-type pointer, so the use site casts it (P-72 ③).  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_get_function_finalizer (env: *mut emacs_env,
                                         arg: emacs_value) -> emacs_finalizer
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, None, || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_MODULE_FUNCTION (lisp);
    (*XMODULE_FUNCTION (lisp).cast::<lisp_module_function_def::Lisp_Module_Function> ()).finalizer
  })

  }
}

/* C 639-648: `module_set_function_finalizer`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_set_function_finalizer (env: *mut emacs_env, arg: emacs_value,
                                         fin: emacs_finalizer)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch_void (env, || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_MODULE_FUNCTION (lisp);
    (*XMODULE_FUNCTION (lisp).cast::<lisp_module_function_def::Lisp_Module_Function> ()).finalizer = fin;
  });

  }
}

/* C 650-655: `module_finalize_function` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_finalize_function
  (func: *const lisp_module_function_def::Lisp_Module_Function)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  if (*func).finalizer.is_some ()
    { ((*func).finalizer).unwrap () ((*func).data); }

  }
}

/* C 657-668: `module_make_interactive`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_interactive (env: *mut emacs_env, function: emacs_value,
                                   spec: emacs_value)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch_void (env, || {
    let lisp_fun: Lisp_Object = value_to_lisp (function);
    CHECK_MODULE_FUNCTION (lisp_fun);
    let lisp_spec: Lisp_Object = value_to_lisp (spec);
    /* Normalize (interactive nil) to (interactive). */
    (*XMODULE_FUNCTION (lisp_fun).cast::<lisp_module_function_def::Lisp_Module_Function> ()).interactive_form
      = if NILP (lisp_spec) { list1 (Qinteractive ()) }
        else { list2 (Qinteractive (), lisp_spec) };
  });

  }
}

/* C 670-674: `module_function_interactive_form` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_function_interactive_form
  (fun: *const lisp_module_function_def::Lisp_Module_Function) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  return (*fun).interactive_form;

  }
}

/* C 676-680: `module_function_command_modes` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_function_command_modes
  (fun: *const lisp_module_function_def::Lisp_Module_Function) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  return (*fun).command_modes;

  }
}

/* C 682-703: `module_funcall`.  C's `Lisp_Object *newargs;` gets C's zero value (0.1-13).  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_funcall (env: *mut emacs_env, func: emacs_value, nargs: ptrdiff_t,
                          args: *mut emacs_value) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || unsafe {
    /* Make a new Lisp_Object array starting with the function as the
       first arg, because that's what Ffuncall takes.  */
    let mut newargs: *mut Lisp_Object = core::ptr::null_mut ();
    USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
    /* C 693: `if (ckd_add (&nargs1, nargs, 1)) overflow_error ();` */
    let (nargs1_new, nargs1_overflow) = nargs.overflowing_add (1);
    if nargs1_overflow
      { overflow_error (); }
    let nargs1: ptrdiff_t = nargs1_new;
    SAFE_ALLOCA_LISP! (sa_avail, newargs, nargs1);
    *newargs.offset (0) = value_to_lisp (func);
    let mut i: ptrdiff_t = 0;
    while i < nargs
      {
        *newargs.offset ((1 + i) as isize) = value_to_lisp (*args.offset (i as isize));
        i += 1;
      }
    let result: emacs_value = lisp_to_value (env, Ffuncall (nargs1, newargs));
    SAFE_FREE! (sa_count, sa_mark);
    result
  })

  }
}

#[cfg (feature = "have_modules")]
#[inline(never)]
unsafe extern "C-unwind" fn call0_with_host_drop_guard_inner (lisp_fun: Lisp_Object) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let _host_guard = crate::nonlocal::HostDropGuard::new ();
  let _root_guard = crate::nonlocal::ScopedLispRoot::new (&[lisp_fun]);
  let mut args = [lisp_fun];
  Ffuncall (1, args.as_mut_ptr ())

  }
}

/* Host C-ABI probe entry point for AU3 dynamic-module verification: invokes a zero-argument
   Lisp callable across an `#[inline(never)]` Rust frame holding `HostDropGuard` + `ScopedLispRoot`,
   catching any `NonLocalExit` inside `module_with_nonlocal_catch` so zero unwinds cross C FFI
   and `obarray` remains completely untouched.  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_vm_module_call0_with_host_guard (
  env: *mut emacs_env,
  func: emacs_value,
) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    let res = call0_with_host_drop_guard_inner (value_to_lisp (func));
    lisp_to_value (env, res)
  })

  }
}

#[cfg (feature = "have_modules")]
pub(crate) fn u4_module_refs_table_live () -> bool
{
  // SAFETY: reads `Vmodule_refs_hash` Lisp_Object word under the VM global lock.
  unsafe { HASH_TABLE_P ((*Vmodule_refs_hash.as_mut_ptr ())) }
}

#[cfg (not (feature = "have_modules"))]
pub(crate) fn u4_module_refs_table_live () -> bool
{
  false
}

#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_vm_u4_exercise_safe_domains (
  env: *mut emacs_env,
  path_val: emacs_value,
) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    let raw_path = value_to_lisp (path_val);
    CHECK_STRING (raw_path);
    let len = SBYTES (raw_path).max (0) as usize;
    let slice = core::slice::from_raw_parts (SDATA (raw_path) as *const u8, len);
    let path_str = String::from_utf8_lossy (slice).into_owned ();
    let mut ctx = crate::roots::GcContext::current ().expect ("active VM thread required");
    match crate::safe_domain::exercise_all_safe_domains (&mut ctx, &path_str) {
      Ok (rooted) => lisp_to_value (env, rooted.get_raw ().into_raw ()),
      Err (exit) => crate::nonlocal::resume_vm_nonlocal (exit),
    }
  })

  }
}

/* C 705-714: `module_intern`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_intern (env: *mut emacs_env, name: *const c_char) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    lisp_to_value (env, intern (name))
  })

  }
}

/* C 716-725: `module_type_of`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_type_of (env: *mut emacs_env, arg: emacs_value) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    lisp_to_value (env, Ftype_of (value_to_lisp (arg)))
  })

  }
}

/* C 727-732: `module_is_not_nil`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_is_not_nil (env: *mut emacs_env, arg: emacs_value) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  MODULE_FUNCTION_BEGIN_NO_CATCH! (env, false);
  return ! NILP (value_to_lisp (arg));

  }
}

/* C 734-739: `module_eq`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_eq (env: *mut emacs_env, a: emacs_value, b: emacs_value) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  MODULE_FUNCTION_BEGIN_NO_CATCH! (env, false);
  return EQ (value_to_lisp (a), value_to_lisp (b));

  }
}

/* C 741-752: `module_extract_integer`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_extract_integer (env: *mut emacs_env, arg: emacs_value) -> intmax_t
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, 0, || unsafe {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_INTEGER (lisp);
    let mut i: intmax_t = 0;
    if ! integer_to_intmax (lisp, &mut i)
      { xsignal1 (Qoverflow_error (), lisp); }
    i
  })

  }
}

/* C 754-764: `module_make_integer`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_integer (env: *mut emacs_env, n: intmax_t) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    lisp_to_value (env, make_int (n))
  })

  }
}

/* C 766-775: `module_extract_float`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_extract_float (env: *mut emacs_env, arg: emacs_value) -> c_double
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, 0.0, || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_TYPE (FLOATP (lisp), Qfloatp (), lisp);
    XFLOAT_DATA (lisp)
  })

  }
}

/* C 777-787: `module_make_float`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_float (env: *mut emacs_env, d: c_double) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    lisp_to_value (env, make_float (d))
  })

  }
}

/* C 789-834: `module_copy_string_contents`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_copy_string_contents (env: *mut emacs_env, value: emacs_value,
                                       buf: *mut c_char, len: *mut ptrdiff_t) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, false, || {
    let lisp_str: Lisp_Object = value_to_lisp (value);
    CHECK_STRING (lisp_str);

    /* We can set NOCOPY to true here because we only use the byte
       sequence starting at SDATA and don't modify the original string
       before copying out the data.

       We set HANDLE-8-BIT and HANDLE-OVER-UNI to nil to signal an error
       if the argument is not a valid Unicode string.  While it isn't
       documented how copy_string_contents behaves in this case,
       signaling an error is the most defensive and obvious reaction. */
    let lisp_str_utf8: Lisp_Object
      = encode_string_utf_8 (lisp_str, Qnil (), true, Qnil (), Qnil ());

    /* Since we set HANDLE-8-BIT and HANDLE-OVER-UNI to nil, the return
       value can be nil, and we have to check for that. */
    CHECK_TYPE (!NILP (lisp_str_utf8), Qunicode_string_p (), lisp_str);

    let raw_size: ptrdiff_t = SBYTES (lisp_str_utf8);
    let required_buf_size: ptrdiff_t = raw_size + 1;

    if buf.is_null ()
      {
        *len = required_buf_size;
        return true;
      }

    if *len < required_buf_size
      {
        let actual: ptrdiff_t = *len;
        *len = required_buf_size;
        module_memory_buffer_too_small (actual, required_buf_size);
      }

    *len = required_buf_size;
    memcpy (buf as *mut c_void, SDATA (lisp_str_utf8) as *const c_void,
            (raw_size + 1) as usize);

    true
  })

  }
}

/* C 836-849: `module_make_string`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_string (env: *mut emacs_env, str_: *const c_char,
                              len: ptrdiff_t) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || unsafe {
    if ! (0 <= len && len <= STRING_BYTES_BOUND)
      { overflow_error (); }
    let lstr: Lisp_Object
      = if len == 0 { (*empty_multibyte_string.as_mut_ptr ()) } else { module_decode_utf_8 (str_, len) };
    lisp_to_value (env, lstr)
  })

  }
}

/* C 851-865: `module_make_unibyte_string`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_unibyte_string (env: *mut emacs_env, str_: *const c_char,
                                      length: ptrdiff_t) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || unsafe {
    if ! (0 <= length && length <= STRING_BYTES_BOUND)
      { overflow_error (); }
    let lstr: Lisp_Object
      = if length == 0 { (*empty_unibyte_string.as_mut_ptr ()) }
        else { make_unibyte_string (str_, length) };
    lisp_to_value (env, lstr)
  })

  }
}

/* C 867-877: `module_make_user_ptr`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_user_ptr (env: *mut emacs_env, fin: emacs_finalizer,
                                ptr: *mut c_void) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    lisp_to_value (env, make_user_ptr (fin, ptr))
  })

  }
}

/* C 879-888: `module_get_user_ptr`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_get_user_ptr (env: *mut emacs_env, arg: emacs_value) -> *mut c_void
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_USER_PTR (lisp);
    (*XUSER_PTR (lisp)).p
  })

  }
}

/* C 890-898: `module_set_user_ptr`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_set_user_ptr (env: *mut emacs_env, arg: emacs_value, ptr: *mut c_void)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch_void (env, || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_USER_PTR (lisp);
    (*XUSER_PTR (lisp)).p = ptr;
  });

  }
}

/* C 900-908: `module_get_user_finalizer`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_get_user_finalizer (env: *mut emacs_env, arg: emacs_value) -> emacs_finalizer
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, None, || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_USER_PTR (lisp);
    (*XUSER_PTR (lisp)).finalizer
  })

  }
}

/* C 910-919: `module_set_user_finalizer`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_set_user_finalizer (env: *mut emacs_env, arg: emacs_value,
                                     fin: emacs_finalizer)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch_void (env, || {
    let lisp: Lisp_Object = value_to_lisp (arg);
    CHECK_USER_PTR (lisp);
    (*XUSER_PTR (lisp)).finalizer = fin;
  });

  }
}

/* C 921-927: `check_vec_index`.  */
#[cfg (feature = "have_modules")]
unsafe fn check_vec_index (lvec: Lisp_Object, i: ptrdiff_t)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  CHECK_VECTOR (lvec);
  if ! (0 <= i && i < ASIZE (lvec))
    { args_out_of_range (lvec, INT_TO_INTEGER (i)); }

  }
}

/* C 929-938: `module_vec_set`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_vec_set (env: *mut emacs_env, vector: emacs_value, index: ptrdiff_t,
                          value: emacs_value)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch_void (env, || {
    let lisp: Lisp_Object = value_to_lisp (vector);
    check_vec_index (lisp, index);
    ASET (lisp, index, value_to_lisp (value));
  });

  }
}

/* C 940-952: `module_vec_get`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_vec_get (env: *mut emacs_env, vector: emacs_value,
                          index: ptrdiff_t) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    let lisp: Lisp_Object = value_to_lisp (vector);
    check_vec_index (lisp, index);
    lisp_to_value (env, AREF (lisp, index))
  })

  }
}

/* C 954-963: `module_vec_size`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_vec_size (env: *mut emacs_env, vector: emacs_value) -> ptrdiff_t
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, 0, || {
    let lisp: Lisp_Object = value_to_lisp (vector);
    CHECK_VECTOR (lisp);
    ASIZE (lisp)
  })

  }
}

/* C 965-977: `module_canvas_data` - C's `#ifdef HAVE_WINDOW_SYSTEM` pair, both arms.  */
// C: dispextern.h 3725: uint32_t *canvas_data (Lisp_Object)
#[cfg (all (feature = "have_modules", feature = "have_window_system"))]
unsafe extern "C-unwind"
{
  pub fn canvas_data (obj: Lisp_Object) -> *mut u32;
}

#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_canvas_data (env: *mut emacs_env, canvas: emacs_value) -> *mut u32
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || unsafe {
    #[cfg (feature = "have_window_system")]
    {
      canvas_data (value_to_lisp (canvas))
    }
    #[cfg (not (feature = "have_window_system"))]
    {
      error (c"Canvas: No window system".as_ptr ());
      core::ptr::null_mut ()
    }
  })

  }
}

/* C 979-986: `module_should_quit`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_should_quit (env: *mut emacs_env) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_assert_thread ();
  module_assert_env (env);
  let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  if (*p).host_exit_pending
    { return true; }
  if module_non_local_exit_check (env) != emacs_funcall_exit_return
    { return false; }
  return QUITP () || crate::host_runtime::has_pending_host_exit ();

  }
}

/* C 988-998: `module_process_input`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_process_input (env: *mut emacs_env) -> emacs_process_input_result
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, emacs_process_input_quit, || {
    maybe_quit ();
    emacs_process_input_continue
  })

  }
}

/* C 1000-1010: `module_extract_time`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_extract_time (env: *mut emacs_env, arg: emacs_value) -> timespec
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, timespec { tv_sec: 0, tv_nsec: 0 }, || {
    lisp_time_argument (value_to_lisp (arg))
  })

  }
}

/* C 1012-1022: `module_make_time`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_time (env: *mut emacs_env, time: timespec) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    lisp_to_value (env, timespec_to_lisp (time))
  })

  }
}

/* C 1024-1059: the comment on the two possible GMP strategies and the chosen sign-magnitude
   representation.  */

/* C 1061-1063: `module_bignum_count_max`.  C's `min (SIZE_MAX, PTRDIFF_MAX)` converts both
   operands to `size_t` (the usual arithmetic conversions), which the port's `isize` `min`
   cannot spell, so the comparison is written out: `SIZE_MAX` is the wider type and
   `PTRDIFF_MAX` < `SIZE_MAX` on every supported target, so the smaller operand is
   `PTRDIFF_MAX as usize` - but the guard keeps C's expression, not the result.  */
#[cfg (feature = "have_modules")]
pub const module_bignum_count_max: ptrdiff_t
  = ((if SIZE_MAX < PTRDIFF_MAX as usize { SIZE_MAX } else { PTRDIFF_MAX as usize })
     / std::mem::size_of::<emacs_limb_t> ()) as ptrdiff_t;

/* C 1065-1070: the two `static_assert`s on `emacs_limb_t`'s representation.  */
#[cfg (feature = "have_modules")]
const _: () = core::assert! (CHAR_BIT == 8);
#[cfg (feature = "have_modules")]
const _: () = core::assert! ((std::mem::size_of::<emacs_limb_t> () == 4
                              && EMACS_LIMB_MAX == 0xFFFFFFFFu64 as usize)
                             || (std::mem::size_of::<emacs_limb_t> () == 8
                                 && EMACS_LIMB_MAX == 0xFFFFFFFFFFFFFFFFu64 as usize));

/* C 1072-1168: `module_extract_big_integer`.  C's `int* sign` is reassigned (C 1089-1090), so
   the parameter is `mut`.  The two `enum { … }` blocks of C 1093-1101 and C 1116 are
   compile-time constant blocks, spelled as `const`s (they feed `static_assert`s and a shift
   count).  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_extract_big_integer (env: *mut emacs_env, arg: emacs_value,
                                      mut sign: *mut c_int, count: *mut ptrdiff_t,
                                      magnitude: *mut emacs_limb_t) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  /* C 1076-1083: `#if GCC_LINT && __GNUC__ && !__clang__` carries the arm that pacifies
     GCC 14.2.1 (carried per P-51; the triple is the `gcc_lint_not_clang` feature, the
     `src/eval.rs` 1604-1606 spelling).  */
  #[cfg (feature = "gcc_lint_not_clang")]
  {
    /* These useless assignments pacify GCC 14.2.1 x86-64
       <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=21161>.  */
    let sign_volatile: *mut c_int = sign;
    sign = sign_volatile;
  }

  module_with_nonlocal_catch (env, false, || /* SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C. */ unsafe {
    let o: Lisp_Object = value_to_lisp (arg);
    CHECK_INTEGER (o);
    let mut dummy: c_int = 0;
    if sign.is_null ()
      { sign = &raw mut dummy; }
    /* See
       https://gmplib.org/manual/Integer-Import-and-Export.html#index-Export. */
    const order: c_int = -1;
    const size: usize = std::mem::size_of::<emacs_limb_t> ();
    const bits: usize = size * CHAR_BIT as usize;
    const endian: c_int = 0;
    const nails: usize = 0;
    const numb: usize = 8 * size - nails;
    if FIXNUMP (o)
      {
        let x: EMACS_INT = XFIXNUM (o);
        *sign = (0 < x) as c_int - (x < 0) as c_int;
        if x == 0 || count.is_null ()
          {
            return true;
          }
        /* As a simplification we don't check how many array elements
           are exactly required, but use a reasonable static upper
           bound.  For most architectures exactly one element should
           suffice.  */
        const required: isize
          = (std::mem::size_of::<EMACS_UINT> () + size - 1) as isize / size as isize;
        const _: () = core::assert! (0 < required && required <= module_bignum_count_max);
        if magnitude.is_null ()
          {
            *count = required;
            return true;
          }
        if *count < required
          {
            let actual: ptrdiff_t = *count;
            *count = required;
            module_memory_buffer_too_small (actual, required);
          }
        /* Set u = abs(x).  See https://stackoverflow.com/a/17313717. */
        let u: EMACS_UINT;
        if 0 < x
          { u = x as EMACS_UINT; }
        else
          { u = (x as EMACS_UINT).wrapping_neg (); }
        const _: () = core::assert! (required as usize * bits < PTRDIFF_MAX as usize);
        let mut i: ptrdiff_t = 0;
        while i < required
          {
            *magnitude.offset (i as isize) = (u >> ((i * bits as isize) as usize)) as emacs_limb_t;
            i += 1;
          }
        return true;
      }
    let x: *const __mpz_struct = xbignum_val (o);
    *sign = mpz_sgn (x);
    if count.is_null ()
      {
        return true;
      }
    let required_size: usize = (mpz_sizeinbase (x, 2) + numb - 1) / numb;
    eassert! (required_size <= module_bignum_count_max as usize);
    let required: ptrdiff_t = required_size as ptrdiff_t;
    if magnitude.is_null ()
      {
        *count = required;
        return true;
      }
    if *count < required
      {
        let actual: ptrdiff_t = *count;
        *count = required;
        module_memory_buffer_too_small (actual, required);
      }
    let mut written: usize = 0;
    mpz_export (magnitude as *mut c_void, &mut written, order, size, endian, nails, x);
    eassert! (written == required_size);
    true
  })

  }
}

/* C 1170-1190: `module_make_big_integer`.  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_make_big_integer (env: *mut emacs_env, sign: c_int,
                                   count: ptrdiff_t,
                                   magnitude: *const emacs_limb_t) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, std::ptr::null_mut (), || {
    if sign == 0
      {
        return lisp_to_value (env, make_fixed_natnum (0));
      }
    const order: c_int = -1;
    const size: usize = std::mem::size_of::<emacs_limb_t> ();
    const endian: c_int = 0;
    const nails: usize = 0;
    mpz_import ((*mpz.as_mut_ptr ())[0].as_mut_ptr (), count as usize, order, size, endian, nails,
                magnitude as *const c_void);
    if sign < 0
      { mpz_neg ((*mpz.as_mut_ptr ())[0].as_mut_ptr (), (*mpz.as_mut_ptr ())[0].as_mut_ptr ()); }
    lisp_to_value (env, make_integer_mpz ())
  })

  }
}

/* C 1192-1202: `module_open_channel`.  */
// C: process.h 316: int open_channel_for_module (Lisp_Object)
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn open_channel_for_module (pipe_process: Lisp_Object) -> c_int;
}

#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_open_channel (env: *mut emacs_env, pipe_process: emacs_value) -> c_int
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  module_with_nonlocal_catch (env, -1, || {
    open_channel_for_module (value_to_lisp (pipe_process))
  })

  }
}

/* ------------------------------------------------------------------------------------------
   C 1204-1798: the subroutines, the DEFUN, the helper functions, value conversion, the
   environment lifetime, the non-local-exit handling and the segment initializer.

   Declarations the mirrors do not carry yet (each one the *owner* header's own line; P-88 ①'s
   single-consumer rule, the `src/xselect.rs` 3211 `mark_object` shape).  Reported as
   mirror gaps in `## Owed` 2.
   ------------------------------------------------------------------------------------------ */
// C: lisp.h 4816: extern struct handler *push_handler_nosignal (Lisp_Object, enum handlertype);
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn push_handler_nosignal (cond: Lisp_Object, r#type: handlertype) -> *mut handler;
}
// C: lisp.h 4259: void hash_remove_from_table (struct Lisp_Hash_Table *, Lisp_Object);
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn hash_remove_from_table (h: *mut Lisp_Hash_Table, key: Lisp_Object);
}
// C: lisp.h 4432: extern AVOID memory_full (size_t);
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn memory_full (size: size_t) -> !;
}
// C: lisp.h 4892: extern Lisp_Object make_user_ptr (void (*finalizer) (void *), void *p);
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn make_user_ptr (finalizer: emacs_finalizer, p: *mut c_void) -> Lisp_Object;
}
// C: coding.h 693-694: Lisp_Object encode_string_utf_8 (Lisp_Object, Lisp_Object, bool, Lisp_Object, Lisp_Object)
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn encode_string_utf_8 (string: Lisp_Object, coding_system: Lisp_Object,
                              nocopy: bool, handle_8_bit: Lisp_Object,
                              handle_over_uni: Lisp_Object) -> Lisp_Object;
}
// C: stdio.h: int vfprintf (FILE *, const char *, va_list);
#[cfg (feature = "have_modules")]
unsafe extern "C"
{
  pub fn vfprintf (stream: *mut FILE, format: *const c_char, ap: VaList<'_>) -> c_int;
}
// C: lisp.h 4828: extern void record_unwind_protect_module (enum specbind_tag, void *);
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind"
{
  pub fn record_unwind_protect_module (tag: specbind_tag, arg: *mut c_void);
}
/* C 3562-3563 spell the two members inside `enum specbind_tag`'s `#ifdef HAVE_MODULES` arm;
   `include/lisp.rs` 1801-1822 carries that enum's no-modules arm only (its
   `SPECPDL_LET`/`SPECPDL_LET_LOCAL`/`SPECPDL_LET_DEFAULT` are 9/10/11, C's are 11/12/13 with
   modules), so the type and the two members sit here for now - `## Owed` 1.  */
/// C: lisp.h 3546-3565: `enum specbind_tag`
#[cfg (feature = "have_modules")]
pub type specbind_tag = u32;
#[cfg (feature = "have_modules")]
/* C's two`SPECPDL_MODULE_*` enumerators are `lisp.h`'s (3560-3562); `include/lisp.rs` carries
   them under `have_modules` (the enumeration's numbering depends on that arm), so the unit does
   not define its own second copy (measured: `E0428` once the mirror's arm went live).  */

/* C 1207-1223: `module_signal_or_throw`.  C's `case emacs_funcall_exit_signal:` falls into
   `case emacs_funcall_exit_throw:`; the port's arms stay separate because the first statement
   (`xsignal`) never returns (AVOID), so there is no fall-through to spell (`## Deviations` 8).
   C's `default: eassume (false);` is the `_` arm (`switch` over an `int`-typed enum).  */
#[cfg (feature = "have_modules")]
unsafe extern "C-unwind" fn module_signal_or_throw (env: *mut emacs_env_private)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  match (*env).pending_non_local_exit
    {
      emacs_funcall_exit_return => { return; }
      emacs_funcall_exit_signal =>
        { xsignal ((*env).non_local_exit_symbol, (*env).non_local_exit_data); }
      emacs_funcall_exit_throw =>
        { Fthrow ((*env).non_local_exit_symbol, (*env).non_local_exit_data); }
      _ => { eassume! (false); }
    }

  }
}

/* C 1225-1284: `module-load`.  */
#[cfg (feature = "have_modules")]
DEFUN! ("module-load", Fmodule_load, Smodule_load, 1, 1, 0,
       doc: "Load module FILE.  ",
       (file: Lisp_Object),
       {
  let handle: dynlib_handle_ptr;
  let module_init: emacs_init_function;
  let mut gpl_sym: *mut c_void;

  CHECK_STRING (file);
  handle = dynlib_open (SSDATA (file));
  if handle.is_null ()
    { xsignal2 (Qmodule_open_failed (), file, build_string (dynlib_error ())); }

  gpl_sym = dynlib_sym (handle, c"plugin_is_GPL_compatible".as_ptr ());
  if gpl_sym.is_null ()
    { xsignal1 (Qmodule_not_gpl_compatible (), file); }

  /* C 1242: `module_init = (emacs_init_function) dynlib_func (handle, "emacs_module_init");`
     - C's function-pointer cast; Rust has no `as` cast between `Option<fn>` types, so the
     `transmute` the tree already uses for that cast (`src/comp.rs` 137) is the spelling
     (`## Deviations` 5).  */
  module_init = std::mem::transmute::<dynlib_function_ptr, emacs_init_function> (
    dynlib_func (handle, c"emacs_module_init".as_ptr ()));
  if module_init.is_none ()
    { xsignal1 (Qmissing_module_init_function (), file); }

  let mut rt_pub: emacs_runtime = { std::mem::zeroed () };
  let mut rt_priv: emacs_runtime_private = { std::mem::zeroed () };
  let mut env_pub: emacs_env = { std::mem::zeroed () };
  let mut env_priv: emacs_env_private = { std::mem::zeroed () };
  rt_priv.env = initialize_environment (&raw mut env_pub, &raw mut env_priv);

  /* If we should use module assertions, reallocate the runtime object
     from the free store, but never free it.  That way the addresses
     for two different runtime objects are guaranteed to be distinct,
     which we can use for checking the liveness of runtime
     pointers.  */
  let rt: *mut emacs_runtime;
  if (*module_assertions.as_mut_ptr ())
    {
      rt = xmalloc (std::mem::size_of::<emacs_runtime> ()) as *mut emacs_runtime;
      __lsan_ignore_object (rt as *const c_void);
    }
  else
    { rt = &raw mut rt_pub; }
  (*rt).size = std::mem::size_of::<emacs_runtime> () as ptrdiff_t;
  (*rt).private_members = &raw mut rt_priv as *mut c_void;
  (*rt).get_environment = Some (module_get_environment);

  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_module (SPECPDL_MODULE_RUNTIME, rt as *mut c_void);
  record_unwind_protect_module (SPECPDL_MODULE_ENVIRONMENT, rt_priv.env as *mut c_void);

  let r: c_int = module_init.unwrap () (rt);

  if env_priv.host_exit_pending
    {
      let code: i32 = env_priv.host_exit_code;
      unbind_to (count, Qnil ());
      crate::host_runtime::resume_host_exit_unwind (code);
    }

  /* Process the quit flag first, so that quitting doesn't get
     overridden by other non-local exits.  */
  maybe_quit ();

  if r != 0
    { xsignal2 (Qmodule_init_failed (), file, INT_TO_INTEGER (r as EMACS_INT)); }

  module_signal_or_throw (&raw mut env_priv);
  return unbind_to (count, Qt ());
});

/* C 1286-1333: `funcall_module`.  C's local named `pub` is the raw identifier `r#pub`
   (PORTING.md 0.2's escape; `pub` is a Rust keyword).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn funcall_module (function: Lisp_Object, nargs: ptrdiff_t,
                                         arglist: *mut Lisp_Object) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let func: *const lisp_module_function_def::Lisp_Module_Function
    = XMODULE_FUNCTION (function).cast::<lisp_module_function_def::Lisp_Module_Function> ();
  eassume! (0 <= (*func).min_arity);
  if ! ((*func).min_arity <= nargs
        && ((*func).max_arity < 0 || nargs <= (*func).max_arity))
    { xsignal2 (Qwrong_number_of_arguments (), function, make_fixnum (nargs)); }

  let mut r#pub: emacs_env = { std::mem::zeroed () };
  let mut priv_: emacs_env_private = { std::mem::zeroed () };
  let env: *mut emacs_env = initialize_environment (&raw mut r#pub, &raw mut priv_);
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_module (SPECPDL_MODULE_ENVIRONMENT, env as *mut c_void);

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut args: *mut emacs_value = std::ptr::null_mut ();
  /* FIXME: Is this (nargs <= 0) test needed?  Either omit it and call
     SAFE_NALLOCA unconditionally, or fix this comment to explain why
     the test is needed.  */
  if nargs <= 0
    { args = core::ptr::null_mut (); }
  else
    { SAFE_NALLOCA! (sa_avail, args, 1, nargs); }

  let mut i: ptrdiff_t = 0;
  while i < nargs
    {
      *args.offset (i as isize) = lisp_to_value (env, *arglist.offset (i as isize));
      if (*args.offset (i as isize)).is_null ()
        { memory_full (std::mem::size_of::<emacs_value_tag> ()); }
      i += 1;
    }

  /* The only possibility of getting an error until here is failure to
     allocate memory for the arguments, but then we already should
     have signaled an error before.  */
  eassert! (priv_.pending_non_local_exit == emacs_funcall_exit_return);

  let ret: emacs_value = ((*func).subr.unwrap ()) (env, nargs, args, (*func).data);

  eassert! (&raw mut priv_ as *mut c_void == (*env).private_members);

  if priv_.host_exit_pending
    {
      let code: i32 = priv_.host_exit_code;
      let _ = SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil (), sa_mark);
      crate::host_runtime::resume_host_exit_unwind (code);
    }

  /* Process the quit flag first, so that quitting doesn't get
     overridden by other non-local exits.  */
  maybe_quit ();

  module_signal_or_throw (&raw mut priv_);
  return SAFE_FREE_UNBIND_TO! (sa_count, count, value_to_lisp (ret), sa_mark);

  }
}

/* C 1335-1342: `module_function_arity` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_function_arity
  (function: *const lisp_module_function_def::Lisp_Module_Function) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let minargs: ptrdiff_t = (*function).min_arity;
  let maxargs: ptrdiff_t = (*function).max_arity;
  return Fcons (make_fixnum (minargs),
                if maxargs == MANY { Qmany () } else { make_fixnum (maxargs) });

  }
}

/* C 1344-1348: `module_function_documentation` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_function_documentation
  (function: *const lisp_module_function_def::Lisp_Module_Function) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  return (*function).documentation;

  }
}

/* C 1350-1354: `module_function_address` (a non-`static` definition).  C's cast
   `(module_funcptr) function->subr` is the same `transmute` shape as C 1242's.  */
/// C: lisp.h 4889: typedef void (*module_funcptr) (void); - a plain function pointer, the shape
/// `lisp.rs` 3857 declares (one typedef in C, one type here).  `src/print.c` 2195 casts it to
/// `void *`, which an `Option` wrapper cannot do; the nullable shape stays where C's own
/// pointer *can* be null (`dynlib_addr`'s parameter).
#[cfg (feature = "have_modules")]
pub use crate::common::other_units::module_funcptr;
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_function_address
  (function: *const lisp_module_function_def::Lisp_Module_Function) -> module_funcptr
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  return std::mem::transmute::<emacs_function, module_funcptr> ((*function).subr);

  }
}

/* C 1356-1360: `module_function_data` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn module_function_data
  (function: *const lisp_module_function_def::Lisp_Module_Function) -> *mut c_void
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  return (*function).data;

  }
}

/* C 1362-1375: `module_assert_thread`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_assert_thread ()
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  if !(*module_assertions.as_mut_ptr ())
    { return; }
  if !in_current_thread ()
    { module_abort (c"Module function called from outside the current Lisp thread".as_ptr ()); }
  if (*gc_in_progress.as_mut_ptr ())
    { module_abort (c"Module function called during garbage collection".as_ptr ()); }

  }
}

/* C 1377-1392: `module_assert_runtime`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_assert_runtime (runtime: *mut emacs_runtime)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  if ! (*module_assertions.as_mut_ptr ())
    { return; }
  let mut count: ptrdiff_t = 0;
  let mut pdl: *const specbinding = (specpdl! ());
  while pdl != (specpdl_ptr! ())
    {
      if (*pdl).kind == SPECPDL_MODULE_RUNTIME as u8
        {
          if (*pdl).unwind_ptr.arg == runtime as *mut c_void
            { return; }
          count += 1;
        }
      pdl = pdl.offset (1);
    }
  module_abort (c"Runtime pointer not found in list of %td runtimes".as_ptr (), count);

  }
}

/* C 1394-1409: `module_assert_env`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_assert_env (env: *mut emacs_env)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  if ! (*module_assertions.as_mut_ptr ())
    { return; }
  let mut count: ptrdiff_t = 0;
  let mut pdl: *const specbinding = (specpdl! ());
  while pdl != (specpdl_ptr! ())
    {
      if (*pdl).kind == SPECPDL_MODULE_ENVIRONMENT as u8
        {
          if (*pdl).unwind_ptr.arg == env as *mut c_void
            { return; }
          count += 1;
        }
      pdl = pdl.offset (1);
    }
  module_abort (c"Environment pointer not found in list of %td environments".as_ptr (), count);

  }
}

/* C 1411-1422: `module_non_local_exit_signal_1`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_non_local_exit_signal_1 (env: *mut emacs_env, sym: Lisp_Object,
                                          data: Lisp_Object)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  if (*p).pending_non_local_exit == emacs_funcall_exit_return
    {
      (*p).pending_non_local_exit = emacs_funcall_exit_signal;
      (*p).non_local_exit_symbol = sym;
      (*p).non_local_exit_data = data;
    }

  }
}

/* C 1424-1435: `module_non_local_exit_throw_1`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_non_local_exit_throw_1 (env: *mut emacs_env, tag: Lisp_Object,
                                         value: Lisp_Object)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  if (*p).pending_non_local_exit == emacs_funcall_exit_return
    {
      (*p).pending_non_local_exit = emacs_funcall_exit_throw;
      (*p).non_local_exit_symbol = tag;
      (*p).non_local_exit_data = value;
    }

  }
}

/* C 1437-1445: `module_out_of_memory`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_out_of_memory (env: *mut emacs_env)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  /* TODO: Reimplement this so it works even if memory-signal-data has
     been modified.  */
  module_non_local_exit_signal_1 (env, XCAR ((*Vmemory_signal_data.as_mut_ptr ())),
                                  XCDR ((*Vmemory_signal_data.as_mut_ptr ())));

  }
}

/* C 1447-1482: `value_to_lisp`.  C's `goto ok;` from inside the loop to C 1481's `ok:` label is
   PORTING.md 2.2's forward-jump shape (`'block: loop { … break 'block; }`); C's second
   `module_abort` argument pair is one concatenated format string (`%"pD"d` is `%td`).  */
#[cfg (feature = "have_modules")]
unsafe fn value_to_lisp (v: emacs_value) -> Lisp_Object
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  if (*module_assertions.as_mut_ptr ())
    {
      /* Check the liveness of the value by iterating over all live
         environments.  */
      'ok: loop
        {
          let mut num_environments: ptrdiff_t = 0;
          let mut num_values: ptrdiff_t = 0;
          let mut pdl: *const specbinding = (specpdl! ());
          while pdl != (specpdl_ptr! ())
            {
              if (*pdl).kind == SPECPDL_MODULE_ENVIRONMENT as u8
                {
                  let env: *const emacs_env = (*pdl).unwind_ptr.arg as *const emacs_env;
                  let r#priv: *mut emacs_env_private
                    = (*env).private_members.cast::<emacs_env_private> ();
                  if value_storage_contains_p (&(*r#priv).storage, v, &mut num_values)
                    { break 'ok; }
                  num_environments += 1;
                }
              pdl = pdl.offset (1);
            }
          /* Also check global values.  */
          if module_global_reference_p (v, &mut num_values)
            { break 'ok; }
          module_abort (c"Emacs value not found in %td values of %td environments".as_ptr (),
                        num_values, num_environments);
        }
    }

  return (*v.cast::<emacs_value_tag> ()).v;

  }
}

/* C 1484-1493: `lisp_to_value`.  */
#[cfg (feature = "have_modules")]
unsafe fn lisp_to_value (env: *mut emacs_env, o: Lisp_Object) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let p: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  if (*p).pending_non_local_exit != emacs_funcall_exit_return
    { return std::ptr::null_mut (); }
  return allocate_emacs_value (env, o);

  }
}

/* C 1495-1501: `initialize_frame`.  */
#[cfg (feature = "have_modules")]
unsafe fn initialize_frame (frame: *mut emacs_value_frame)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  (*frame).offset = 0;
  (*frame).next = core::ptr::null_mut ();

  }
}

/* C 1503-1510: `initialize_storage`.  */
#[cfg (feature = "have_modules")]
unsafe fn initialize_storage (storage: *mut emacs_value_storage)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  initialize_frame (&raw mut (*storage).initial);
  (*storage).current = &raw mut (*storage).initial;

  }
}

/* C 1512-1524: `finalize_storage`.  */
#[cfg (feature = "have_modules")]
unsafe fn finalize_storage (storage: *mut emacs_value_storage)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let mut next: *mut emacs_value_frame = (*storage).initial.next;
  while !next.is_null ()
    {
      let current: *mut emacs_value_frame = next;
      next = (*current).next;
      free (current as *mut c_void);
    }

  }
}

/* C 1526-1550: `allocate_emacs_value`.  */
#[cfg (feature = "have_modules")]
unsafe fn allocate_emacs_value (env: *mut emacs_env, obj: Lisp_Object) -> emacs_value
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let storage: *mut emacs_value_storage
    = &raw mut (*(*env).private_members.cast::<emacs_env_private> ()).storage;
  eassert! (! (*storage).current.is_null ());
  eassert! ((*(*storage).current).offset < value_frame_size);
  eassert! ((*(*storage).current).next.is_null ());
  if (*(*storage).current).offset == value_frame_size - 1
    {
      (*(*storage).current).next
        = malloc (std::mem::size_of::<emacs_value_frame> ()) as *mut emacs_value_frame;
      if (*(*storage).current).next.is_null ()
        {
          module_out_of_memory (env);
          return std::ptr::null_mut ();
        }
      initialize_frame ((*(*storage).current).next);
      (*storage).current = (*(*storage).current).next;
    }
  let value: *mut emacs_value_tag
    = (*(*storage).current).objects.as_mut_ptr ()
      .offset ((*(*storage).current).offset as isize);
  (*value).v = obj;
  (*(*storage).current).offset += 1;
  return value as emacs_value;

  }
}

/* C 1552-1565: `mark_module_environment` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mark_module_environment (ptr: *mut c_void)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let env: *mut emacs_env = ptr as *mut emacs_env;
  let r#priv: *mut emacs_env_private = (*env).private_members.cast::<emacs_env_private> ();
  mark_object ((*r#priv).non_local_exit_symbol);
  mark_object ((*r#priv).non_local_exit_data);
  let mut frame: *mut emacs_value_frame = &raw mut (*r#priv).storage.initial;
  while !frame.is_null ()
    {
      let mut i: c_int = 0;
      while i < (*frame).offset
        {
          mark_object ((*frame).objects[i as usize].v);
          i += 1;
        }
      frame = (*frame).next;
    }

  }
}

/* C 1567-1631: `initialize_environment`.  C reassigns its `env` parameter (C 1581), so the
   parameter is `mut`.  */
#[cfg (feature = "have_modules")]
unsafe fn initialize_environment (mut env: *mut emacs_env,
                                  r#priv: *mut emacs_env_private) -> *mut emacs_env
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  if (*module_assertions.as_mut_ptr ())
    {
      env = xmalloc (std::mem::size_of::<emacs_env> ()) as *mut emacs_env;
      __lsan_ignore_object (env as *const c_void);
    }

  (*r#priv).pending_non_local_exit = emacs_funcall_exit_return;
  (*r#priv).non_local_exit_symbol = Qnil ();
  (*r#priv).non_local_exit_data = Qnil ();
  (*r#priv).host_exit_pending = false;
  (*r#priv).host_exit_code = 0;
  initialize_storage (&raw mut (*r#priv).storage);
  (*env).size = std::mem::size_of::<emacs_env> () as ptrdiff_t;
  (*env).private_members = r#priv as *mut c_void;
  (*env).make_global_ref = Some (module_make_global_ref);
  (*env).free_global_ref = Some (module_free_global_ref);
  (*env).non_local_exit_check = Some (module_non_local_exit_check);
  (*env).non_local_exit_clear = Some (module_non_local_exit_clear);
  (*env).non_local_exit_get = Some (module_non_local_exit_get);
  (*env).non_local_exit_signal = Some (module_non_local_exit_signal);
  (*env).non_local_exit_throw = Some (module_non_local_exit_throw);
  (*env).make_function = Some (module_make_function);
  (*env).funcall = Some (module_funcall);
  (*env).intern = Some (module_intern);
  (*env).type_of = Some (module_type_of);
  (*env).is_not_nil = Some (module_is_not_nil);
  (*env).eq = Some (module_eq);
  (*env).extract_integer = Some (module_extract_integer);
  (*env).make_integer = Some (module_make_integer);
  (*env).extract_float = Some (module_extract_float);
  (*env).make_float = Some (module_make_float);
  (*env).copy_string_contents = Some (module_copy_string_contents);
  (*env).make_string = Some (module_make_string);
  (*env).make_user_ptr = Some (module_make_user_ptr);
  (*env).get_user_ptr = Some (module_get_user_ptr);
  (*env).set_user_ptr = Some (module_set_user_ptr);
  (*env).get_user_finalizer = Some (module_get_user_finalizer);
  (*env).set_user_finalizer = Some (module_set_user_finalizer);
  (*env).vec_get = Some (module_vec_get);
  (*env).vec_set = Some (module_vec_set);
  (*env).vec_size = Some (module_vec_size);
  (*env).should_quit = Some (module_should_quit);
  (*env).process_input = Some (module_process_input);
  (*env).extract_time = Some (module_extract_time);
  (*env).make_time = Some (module_make_time);
  (*env).extract_big_integer = Some (module_extract_big_integer);
  (*env).make_big_integer = Some (module_make_big_integer);
  (*env).get_function_finalizer = Some (module_get_function_finalizer);
  (*env).set_function_finalizer = Some (module_set_function_finalizer);
  (*env).open_channel = Some (module_open_channel);
  (*env).make_interactive = Some (module_make_interactive);
  (*env).make_unibyte_string = Some (module_make_unibyte_string);
  (*env).canvas_data = Some (module_canvas_data);
  return env;

  }
}

/* C 1633-1639: `finalize_environment`.  */
#[cfg (feature = "have_modules")]
unsafe fn finalize_environment (env: *mut emacs_env)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  finalize_storage (&raw mut (*(*env).private_members.cast::<emacs_env_private> ()).storage);

  }
}

/* C 1641-1645: `finalize_environment_unwind` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn finalize_environment_unwind (env: *mut c_void)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  finalize_environment (env as *mut emacs_env);

  }
}

/* C 1647-1652: `finalize_runtime_unwind` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn finalize_runtime_unwind (raw_ert: *mut c_void)
{
  /* No further cleanup is required, as the initial environment is
     unwound separately.  See the logic in Fmodule_load.  */
}

/* C 1654-1668: `module_reset_handlerlist`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_reset_handlerlist (ihandlerlist: *mut handler)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  eassert! (handlerlist! () == ihandlerlist);
  handlerlist! () = (*handlerlist! ()).next;

  }
}

/* C 1670-1686: `module_handle_nonlocal_exit`.  */
#[cfg (feature = "have_modules")]
unsafe fn module_handle_nonlocal_exit (env: *mut emacs_env, r#type: nonlocal_exit,
                                       data: Lisp_Object)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  match r#type
    {
      NONLOCAL_EXIT_SIGNAL =>
        { module_non_local_exit_signal_1 (env, XCAR (data), XCDR (data)); }
      NONLOCAL_EXIT_THROW =>
        { module_non_local_exit_throw_1 (env, XCAR (data), XCDR (data)); }
      _ => {}
    }

  }
}

/* C 1688-1694: `init_module_assertions` (a non-`static` definition).  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_module_assertions (enable: bool)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  (*module_assertions.as_mut_ptr ()) = enable;

  }
}

/* C 1696-1714: `value_storage_contains_p`.  */
#[cfg (feature = "have_modules")]
unsafe fn value_storage_contains_p (storage: *const emacs_value_storage, value: emacs_value,
                                    count: *mut ptrdiff_t) -> bool
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  let mut frame: *const emacs_value_frame = &(*storage).initial;
  while !frame.is_null ()
    {
      let mut i: c_int = 0;
      while i < (*frame).offset
        {
          /* C 1708: `if (&frame->objects[i] == value) return true;` */
          if (*frame).objects.as_ptr ().offset (i as isize) as emacs_value == value
            { return true; }
          *count += 1;
          i += 1;
        }
      frame = (*frame).next;
    }
  return false;

  }
}

/* C 1716-1727: `module_abort`.  C's `static AVOID ATTRIBUTE_FORMAT_PRINTF (1, 2)` attributes have
   no port counterpart (PORTING.md P-04's row); the `...` parameter is the port's `VaList`
   (`src/comp.rs` 1017-1024's shape).  */
#[cfg (feature = "have_modules")]
unsafe extern "C" fn module_abort (format: *const c_char, mut args: ...)
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  fputs (c"Emacs module assertion: ".as_ptr (), (*stderr.as_mut_ptr ()));
  let ap: VaList<'_> = args;
  vfprintf ((*stderr.as_mut_ptr ()), format, ap);
  putc ('\n' as c_int, (*stderr.as_mut_ptr ()));
  fflush (std::ptr::null_mut () as *mut FILE);
  emacs_abort ();

  }
}

/* C 1729-1798: `syms_of_module`.  */
#[cfg (feature = "have_modules")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_module ()
{
  // SAFETY: external C dynamic-module ABI boundary; validates `emacs_env` ownership and intercepts all `NonLocalExit`s into pending environment state before returning to C.
  unsafe {
  staticpro (Vmodule_refs_hash.as_mut_ptr ());
  (*Vmodule_refs_hash.as_mut_ptr ())
    = make_hash_table (hashtest_eq.as_ptr (), DEFAULT_HASH_SIZE as EMACS_INT, Weak_None);

  DEFSYM! (Qmodule_out_of_memory, "module-out-of-memory");
  Fput (Qmodule_out_of_memory (), Qerror_conditions (),
        list2 (Qmodule_out_of_memory (), Qerror ()));
  Fput (Qmodule_out_of_memory (), Qerror_message (),
        build_unibyte_string (c"Module out of memory".as_ptr ()));

  staticpro (&raw mut (*module_out_of_memory_symbol.as_mut_ptr ()).v);
  (*module_out_of_memory_symbol.as_mut_ptr ()).v = Qmodule_out_of_memory ();

  staticpro (&raw mut (*module_out_of_memory_data.as_mut_ptr ()).v);
  (*module_out_of_memory_data.as_mut_ptr ()).v = Qnil ();

  DEFSYM! (Qmodule_load_failed, "module-load-failed");
  Fput (Qmodule_load_failed (), Qerror_conditions (),
        list! (Qmodule_load_failed (), Qerror ()));
  Fput (Qmodule_load_failed (), Qerror_message (),
        build_string (c"Module load failed".as_ptr ()));

  DEFSYM! (Qmodule_open_failed, "module-open-failed");
  Fput (Qmodule_open_failed (), Qerror_conditions (),
        list! (Qmodule_open_failed (), Qmodule_load_failed (), Qerror ()));
  Fput (Qmodule_open_failed (), Qerror_message (),
        build_string (c"Module could not be opened".as_ptr ()));

  DEFSYM! (Qmodule_not_gpl_compatible, "module-not-gpl-compatible");
  Fput (Qmodule_not_gpl_compatible (), Qerror_conditions (),
        list! (Qmodule_not_gpl_compatible (), Qmodule_load_failed (), Qerror ()));
  Fput (Qmodule_not_gpl_compatible (), Qerror_message (),
        build_string (c"Module is not GPL compatible".as_ptr ()));

  DEFSYM! (Qmissing_module_init_function, "missing-module-init-function");
  Fput (Qmissing_module_init_function (), Qerror_conditions (),
        list! (Qmissing_module_init_function (), Qmodule_load_failed (),
              Qerror ()));
  Fput (Qmissing_module_init_function (), Qerror_message (),
        build_string (c"Module does not export an initialization function".as_ptr ()));

  DEFSYM! (Qmodule_init_failed, "module-init-failed");
  Fput (Qmodule_init_failed (), Qerror_conditions (),
        list! (Qmodule_init_failed (), Qmodule_load_failed (), Qerror ()));
  Fput (Qmodule_init_failed (), Qerror_message (),
        build_string (c"Module initialization failed".as_ptr ()));

  DEFSYM! (Qinvalid_arity, "invalid-arity");
  Fput (Qinvalid_arity (), Qerror_conditions (), list! (Qinvalid_arity (), Qerror ()));
  Fput (Qinvalid_arity (), Qerror_message (),
        build_string (c"Invalid function arity".as_ptr ()));

  DEFSYM! (Qmemory_buffer_too_small, "memory-buffer-too-small");
  Fput (Qmemory_buffer_too_small (), Qerror_conditions (),
        list2 (Qmemory_buffer_too_small (), Qerror ()));
  Fput (Qmemory_buffer_too_small (), Qerror_message (),
        build_unibyte_string (c"Memory buffer too small".as_ptr ()));

  DEFSYM! (Qmodule_function_p, "module-function-p");
  DEFSYM! (Qunicode_string_p, "unicode-string-p");

  core::hint::black_box (crate::nonlocal::femacs_vm_host_drop_counts as *const ());
  core::hint::black_box (femacs_vm_module_call0_with_host_guard as *const ());
  core::hint::black_box (femacs_vm_u4_exercise_safe_domains as *const ());
  core::hint::black_box (crate::roots::femacs_vm_persistent_root_alloc as *const ());
  core::hint::black_box (crate::roots::femacs_vm_persistent_root_get as *const ());
  core::hint::black_box (crate::roots::femacs_vm_persistent_root_release as *const ());
  core::hint::black_box (crate::state::femacs_vm_u4_root_stats as *const ());

  defsubr (Smodule_load.as_mut_ptr ());

  }
}
