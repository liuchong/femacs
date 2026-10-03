# emacs/src/pdumper.h (281 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 12
- [x] pdumper_remember_scalar  (line 61)
- [x] pdumper_remember_lv_ptr_raw  (line 77)
- [x] pdumper_do_now_and_after_load  (line 92)
- [x] pdumper_do_now_and_after_late_load  (line 104)
- [x] pdumper_object_p  (line 162)
- [x] pdumper_cold_object_p  (line 180)
- [x] pdumper_find_object_type  (line 197)
- [x] pdumper_valid_object_type_p  (line 210)
- [x] pdumper_object_p_precise  (line 220)
- [x] pdumper_marked_p  (line 236)
- [x] pdumper_set_marked  (line 252)
- [x] pdumper_clear_marks  (line 266)

## Macros defined — 5
- [x] EMACS_PDUMPER_H  (line 21, body da39a3ee5e6b)
- [x] PDUMPER_IGNORE  (line 34, body b0e9f782defb)
- [x] PDUMPER_REMEMBER_SCALAR  (line 52, body ba74bc20655b)
- [x] PDUMPER_RESET  (line 118, body eaa58fdba308)
- [x] PDUMPER_RESET_LV  (line 126, body 9ec0d127085f)

## Types and their fields — 1
- [x] pdumper_loaded_dump: uintptr_t start, uintptr_t end

## File-scope objects — 1
- [x] extern struct pdumper_loaded_dump dump_public  (line 156)

## Decisions

`emacs/src/pdumper.h` @ `fd7afced403`; port `femacs/src/include/pdumper.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/pdumper.h femacs/src/include/pdumper.rs`
-> `OK`.

- **Declarations grouped into `pdumper_decls`.**  C interleaves `extern` prototypes with the
  `INLINE` definitions; the port puts every declaration that `pdumper.c` will define
  (`dump_fingerprint`, the four `*_impl` functions, `pdumper_load`, `dump_public`,
  `pdumper_record_wd`, `init_pdumper_once`, `syms_of_pdumper`) into
  `pub mod pdumper_decls { use super::*; unsafe extern "C" { … } }` with
  `use pdumper_decls::*;` after it, because a Rust definition cannot share a scope with the
  foreign declaration of the same name (E0428; P-21/P-39, the itree.h/category.h shape).
  Names, order and signatures are C's; only their position in the file moves.
- **`INLINE` -> `#[inline(always)] pub fn`** (P-17), one per C body: `pdumper_remember_scalar`,
  `pdumper_remember_lv_ptr_raw`, `pdumper_do_now_and_after_load`,
  `pdumper_do_now_and_after_late_load`, `pdumper_object_p`, `pdumper_cold_object_p`,
  `pdumper_find_object_type`, `pdumper_valid_object_type_p`, `pdumper_object_p_precise`,
  `pdumper_marked_p`, `pdumper_set_marked`, `pdumper_clear_marks`.  Each stays a *safe* `pub fn`
  and wraps the C call or the `dump_public` read in an `unsafe { … }` block (the buffer.rs
  `SET_PT` shape), because the frozen configuration has `HAVE_PDUMPER` 1
  (`docs/baseline/config/darwin-arm64.macros.tsv`).
- **Dead `#else` arms** (they are the `HAVE_PDUMPER`-less build, so they are *not* transcribed):
  `pdumper_remember_scalar`'s `(void) data; (void) nbytes;`; `pdumper_remember_lv_ptr_raw`'s
  `(void) ptr; (void) type;`; `pdumper_do_now_and_after_load`'s bare `hook ();` and its
  late-load twin; `pdumper_object_p`'s `(void) obj; return false;`;
  `pdumper_cold_object_p`'s; `pdumper_find_object_type`'s `emacs_abort ()`;
  `pdumper_object_p_precise`'s; `pdumper_marked_p`'s; `pdumper_set_marked`'s; and
  `pdumper_clear_marks`'s empty body.
- **Macro bodies** (`docs/ports/macro-body-notes.tsv`): `PDUMPER_RESET` / `PDUMPER_RESET_LV`
  — C's `do { … } while (0)` becomes a bare `if`/`else`, so the loop's `0` is the one token
  with no counterpart; `$variable` stays an assignable place (P-79).
  `PDUMPER_IGNORE (thing)` is `((void) &(thing))`: Rust has no `(void)` cast, so the port
  writes `{ let _ = &$thing; }` (same address-taking, same discarding).
  `PDUMPER_REMEMBER_SCALAR (thing)` is `pdumper_remember_scalar (&(thing), sizeof (thing))`:
  `core::ptr::addr_of_mut!` keeps C's lvalue requirement and `core::mem::size_of_val` is the
  unevaluated `sizeof` of that same place.
- **`uintptr_t`** had no home yet; the type belongs to `<stdint.h>`, so it is declared in
  `femacs/lib/include/stdint.in.rs` (`stdint.in.h` 324, the row of
  `docs/ports/include-stdint.in.md` is now ticked) next to `uint64_t`, not invented here.
- **`pdumper_load_result`** keeps C's members in order with their implicit values
  (`#[repr(C)] #[derive(Clone, Copy, PartialEq, Eq)] pub enum`, the itree.h `itree_order`
  shape); `PDUMPER_NO_OBJECT = -1` is an untagged C enum, so it is a `pub const c_int`.
- **`pdumper_hook`** is `Option<unsafe extern "C" fn ()>` (P-27, the `atimer_callback` shape).
- Include list: `stdio.rs` (pdumper.h 22), `fingerprint.rs` (23), then `config.rs` + `lisp.rs`
  (24) — `config.rs` explicitly, because C's `lisp.h` reaches `config.h` and the lisp.rs mirror
  reads `HAVE_PDUMPER`/`HAVE_NATIVE_COMP` from it (PORTING.md 0.1-14).
- Deviations: the include guard has no Rust counterpart (`pub const EMACS_PDUMPER_H: bool =
  true`); the C comments are kept verbatim above the item they describe; the file has no
  `FUNCTION`/`DEFUN`, so there is no subr work.  Line count is 228 vs C's 281 (the header is
  drift-exempt, `is_header_like`).

## Decisions (2026-10-03, when the mirror was written for profiler.c)

- Written to unblock `profiler.c` (which includes pdumper.h at C 24) and `filelock.c`
  (C 43 `syms_of_filelock`'s `pdumper_*` use).  It has not been included by a ported unit yet;
  the first unit that does will exercise the `INLINE` bodies against their C counterparts.
