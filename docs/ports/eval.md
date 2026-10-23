# emacs/src/eval.c (4680 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 54
- [x] or  (For, 0/UNEVALLED, doc 58d94ed2b4e7, line 344)
- [x] and  (Fand, 0/UNEVALLED, doc f401d046e0b1, line 365)
- [x] if  (Fif, 2/UNEVALLED, doc 378652dbcb16, line 386)
- [x] cond  (Fcond, 0/UNEVALLED, doc c005e478f89c, line 403)
- [x] progn  (Fprogn, 0/UNEVALLED, doc 6390607cd6f8, line 433)
- [x] prog1  (Fprog1, 1/UNEVALLED, doc e4cfed571c49, line 458)
- [x] setq  (Fsetq, 0/UNEVALLED, doc 37425028d9f5, line 470)
- [x] quote  (Fquote, 1/UNEVALLED, doc 83828b50ec75, line 507)
- [x] make-interpreted-closure  (Fmake_interpreted_closure, 3/5, doc 623a264e092d, line 525)
- [x] function  (Ffunction, 1/UNEVALLED, doc b13eb8ed5cbd, line 560)
- [x] defvaralias  (Fdefvaralias, 2/3, doc cea4f491ac76, line 631)
- [x] internal-delete-indirect-variable  (Finternal_delete_indirect_variable, 1/1, doc 0fbc71fbd376, line 728)
- [x] default-toplevel-value  (Fdefault_toplevel_value, 1/1, doc 379794d2b9fe, line 811)
- [x] set-default-toplevel-value  (Fset_default_toplevel_value, 2/2, doc 41f35b57b91d, line 827)
- [x] buffer-local-toplevel-value  (Fbuffer_local_toplevel_value, 1/2, doc 9a5a23e99ec3, line 842)
- [x] set-buffer-local-toplevel-value  (Fset_buffer_local_toplevel_value, 2/3, doc 587f0e6fb52f, line 861)
- [x] internal--define-uninitialized-variable  (Finternal__define_uninitialized_variable, 1/2, doc 9e41c9d1093e, line 889)
- [x] defvar  (Fdefvar, 1/UNEVALLED, doc f4ee57c77494, line 944)
- [x] defvar-1  (Fdefvar_1, 2/3, doc 441678ffced6, line 1010)
- [x] defconst  (Fdefconst, 2/UNEVALLED, doc dcec7bc59369, line 1018)
- [x] defconst-1  (Fdefconst_1, 2/3, doc fca586c635d5, line 1051)
- [x] internal-make-var-non-special  (Fmake_var_non_special, 1/1, doc 352a4260511a, line 1065)
- [x] let*  (FletX, 1/UNEVALLED, doc 8d5cb55cf077, line 1076)
- [x] let  (Flet, 1/UNEVALLED, doc 5eccb608151a, line 1133)
- [x] while  (Fwhile, 1/UNEVALLED, doc f213f47bdcb8, line 1200)
- [x] funcall-with-delayed-message  (Ffuncall_with_delayed_message, 3/3, doc 1e95902375ef, line 1253)
- [x] macroexpand  (Fmacroexpand, 1/2, doc 73d5e0fc08e3, line 1283)
- [x] catch  (Fcatch, 1/UNEVALLED, doc 0dbe0880c04f, line 1351)
- [x] throw  (Fthrow, 2/2, doc 6c10f6dc9de5, line 1464)
- [x] unwind-protect  (Funwind_protect, 1/UNEVALLED, doc cfc711042181, line 1484)
- [x] condition-case  (Fcondition_case, 2/UNEVALLED, doc e004ef79ea61, line 1500)
- [x] handler-bind-1  (Fhandler_bind_1, 1/MANY, doc af4118f00c23, line 1551)
- [x] signal  (Fsignal, 1/2, doc c24aa0b5e4c0, line 1897)
- [x] commandp  (Fcommandp, 1/2, doc 3ea1a581ac99, line 2301)
- [x] autoload  (Fautoload, 2/5, doc db4482605e79, line 2411)
- [x] autoload-do-load  (Fautoload_do_load, 1/3, doc 31e8592ef7d6, line 2494)
- [x] eval  (Feval, 1/2, doc c51962808e3d, line 2548)
- [x] apply  (Fapply, 1/MANY, doc 5e419ac23671, line 2803)
- [x] run-hooks  (Frun_hooks, 0/MANY, doc e26efb27c6eb, line 2883)
- [x] run-hook-with-args  (Frun_hook_with_args, 1/MANY, doc 50e3ea2acb97, line 2908)
- [x] run-hook-with-args-until-success  (Frun_hook_with_args_until_success, 1/MANY, doc 3dcb8005662b, line 2927)
- [x] run-hook-with-args-until-failure  (Frun_hook_with_args_until_failure, 1/MANY, doc 765d85bcffb5, line 2951)
- [x] run-hook-wrapped  (Frun_hook_wrapped, 2/MANY, doc ff6b55f9e424, line 2981)
- [x] functionp  (Ffunctionp, 1/1, doc de8f4dd3928c, line 3087)
- [x] debugger-trap  (Fdebugger_trap, 0/0, doc 6b2737b0a565, line 3133)
- [x] funcall  (Ffuncall, 1/MANY, doc 5a2e8de93533, line 3185)
- [x] func-arity  (Ffunc_arity, 1/1, doc aab69875ba32, line 3476)
- [x] special-variable-p  (Fspecial_variable_p, 1/1, doc c31e1db5d38a, line 3966)
- [x] backtrace-debug  (Fbacktrace_debug, 2/3, doc e547b27e32fd, line 4035)
- [x] mapbacktrace  (Fmapbacktrace, 1/2, doc 773be872c70a, line 4050)
- [x] backtrace-frame--internal  (Fbacktrace_frame_internal, 3/3, doc d7ff71ac4416, line 4079)
- [x] backtrace--frames-from-thread  (Fbacktrace_frames_from_thread, 1/1, doc b7a9659abd6f, line 4088)
- [x] backtrace-eval  (Fbacktrace_eval, 2/3, doc 13e1be33e20a, line 4241)
- [x] backtrace--locals  (Fbacktrace__locals, 1/2, doc a6b428deb795, line 4263)

## Functions defined — 109
- [x] specpdl_symbol  (line 61, static)
- [x] specpdl_kind  (line 68, static)
- [x] specpdl_old_value  (line 75, static)
- [x] set_specpdl_old_value  (line 82, static)
- [x] specpdl_where  (line 89, static)
- [x] specpdl_kboard  (line 96, static)
- [x] specpdl_arg  (line 103, static)
- [x] backtrace_function_body  (line 119, static)
- [x] backtrace_nargs  (line 127, static)
- [x] backtrace_args_body  (line 134, static)
- [x] set_backtrace_args  (line 144, static)
- [x] set_backtrace_debug_on_exit  (line 152, static)
- [x] backtrace_p  (line 161)
- [x] backtrace_thread_p  (line 169, static)
- [x] backtrace_top  (line 173)
- [x] backtrace_thread_top  (line 188, static)
- [x] backtrace_next  (line 197)
- [x] backtrace_thread_next  (line 208, static)
- [x] init_eval_once  (line 217)
- [x] init_eval_once_for_pdumper  (line 225, static)
- [x] init_eval  (line 234)
- [x] restore_stack_limits  (line 255, static)
- [x] max_ensure_room  (line 266, static)
- [x] call_debugger  (line 282)
- [x] do_debug_on_call  (line 336)
- [x] prog_ignore  (line 453)
- [x] default_toplevel_binding  (line 747, static)
- [x] local_toplevel_binding  (line 766, static)
- [x] lexbound_p  (line 788, static)
- [x] defvar  (line 918, static)
- [x] with_delayed_message_display  (line 1230, static)
- [x] with_delayed_message_cancel  (line 1242, static)
- [x] pop_handler  (line 1373)
- [x] internal_catch  (line 1383)
- [x] unwind_to_catch  (line 1423, static)
- [x] push_handler_bind  (line 1542)
- [x] internal_lisp_condition_case  (line 1588)
- [x] internal_condition_case  (line 1699)
- [x] internal_condition_case_1  (line 1722)
- [x] internal_condition_case_2  (line 1747)
- [x] internal_condition_case_n  (line 1774)
- [x] internal_catch_all  (line 1806)
- [x] push_handler  (line 1831)
- [x] push_handler_nosignal  (line 1840)
- [x] process_quit_flag  (line 1875, static)
- [x] probably_quit  (line 1887)
- [x] quit  (line 1932)
- [x] signal_or_quit  (line 1943, static)
- [x] xsignal0  (line 2078)
- [x] xsignal1  (line 2084)
- [x] xsignal2  (line 2090)
- [x] xsignal3  (line 2096)
- [x] signal_error  (line 2105)
- [x] define_error  (line 2116)
- [x] overflow_error  (line 2131)
- [x] wants_debugger  (line 2141, static)
- [x] skip_debugger  (line 2165, static)
- [x] signal_quit_p  (line 2200)
- [x] maybe_call_debugger  (line 2217, static)
- [x] find_handler_clause  (line 2241, static)
- [x] vformat_string  (line 2266)
- [x] verror  (line 2285)
- [x] error  (line 2294)
- [x] un_autoload  (line 2447, static)
- [x] grow_specpdl_allocation  (line 2566)
- [x] eval_sub  (line 2585)
- [x] funcall_nil  (line 2877, static)
- [x] funcall_not  (line 2946, static)
- [x] run_hook_wrapped_funcall  (line 2970, static)
- [x] run_hook_with_args  (line 2999)
- [x] run_hook  (line 3067)
- [x] run_hook_with_args_2  (line 3075)
- [x] apply1  (line 3082)
- [x] FUNCTIONP  (line 3103)
- [x] funcall_general  (line 3149)
- [x] safe_eval_handler  (line 3222, static)
- [x] safe_funcall  (line 3230)
- [x] safe_eval  (line 3245)
- [x] funcall_subr  (line 3254)
- [x] apply_lambda  (line 3315, static)
- [x] funcall_lambda  (line 3350, static)
- [x] lambda_arity  (line 3541, static)
- [x] let_shadows_buffer_binding_p  (line 3593)
- [x] do_specbind  (line 3613, static)
- [x] specbind  (line 3656)
- [x] record_unwind_protect  (line 3720)
- [x] record_unwind_protect_array  (line 3730)
- [x] record_unwind_protect_ptr  (line 3739)
- [x] record_unwind_protect_ptr_mark  (line 3751)
- [x] record_unwind_protect_int  (line 3762)
- [x] record_unwind_protect_intmax  (line 3771)
- [x] record_unwind_protect_excursion  (line 3780)
- [x] record_unwind_protect_void  (line 3788)
- [x] record_unwind_protect_module  (line 3796)
- [x] do_one_unbind  (line 3806, static)
- [x] do_nothing  (line 3888, static)
- [x] record_unwind_protect_nothing  (line 3895)
- [x] clear_unwind_protect  (line 3904)
- [x] set_unwind_protect  (line 3916)
- [x] set_unwind_protect_ptr  (line 3927)
- [x] unbind_to  (line 3940)
- [x] get_backtrace_starting_at  (line 3978, static)
- [x] get_backtrace_frame  (line 4002, static)
- [x] backtrace_frame_apply  (line 4017, static)
- [x] specpdl_unrewind  (line 4134)
- [x] backtrace_eval_unrewind  (line 4236, static)
- [x] mark_specpdl  (line 4334)
- [x] get_backtrace  (line 4404)
- [x] syms_of_eval  (line 4422)

## Macros defined — 2
- [x] GDB_FUNCPTR  (line 114, body 9fdaa13d4c71)
- [x] clobbered_eassert  (line 1370, body fbb1b9b9fd49)

## Types and their fields — 1
- [x] funcall_with_delayed_message_data: Lisp_Object* message, struct atimer* timer

## File-scope objects — 4
- [x] Lisp_Object Vautoload_queue  (line 39)
- [x] Lisp_Object Vrun_hooks  (line 44)
- [x] Lisp_Object Vsignaling_function  (line 49)
- [x] static Lisp_Object Qcatch_all_memory_full  (line 1799, static)

## Decisions

- **What the file carries.** C's 54 `DEFUN`s (all `DEFUN!` forms, docstrings decoded to the
  values `make-docfile` produces), its 109 functions (the non-`static` ones carry
  `#[unsafe(no_mangle)]`, the `static` helpers stay plain `unsafe fn`, and the eleven whose
  address C takes are `unsafe extern "C" fn`: `restore_stack_limits`,
  `with_delayed_message_display`, `with_delayed_message_cancel`, `backtrace_function_body`,
  `backtrace_args_body`, `do_nothing`, `un_autoload`, `backtrace_eval_unrewind`,
  `funcall_nil`, `funcall_not`, `run_hook_wrapped_funcall`), the two macros (`GDB_FUNCPTR`, `clobbered_eassert`), the one
  type it declares (`struct funcall_with_delayed_message_data`, `#[repr(C)]` with three
  clang-shaped `const _` assertions: size 16, `message`@0, `timer`@8), the four file-scope
  objects (`Vautoload_queue`, `Vrun_hooks`, `Vsignaling_function`, `Qcatch_all_memory_full`)
  and the 19 `V*`/`defvar`-owned storages of `syms_of_eval` (docs/status.md 3.29): the
  `DEFVAR_*` calls in this file's `syms_of_eval` are the ones whose variables it owns, so
  `max_lisp_eval_depth`, `lisp_eval_depth_reserve`, `Vquit_flag`, `Vinhibit_quit`,
  `Vinhibit_debugger`, `Vdebug_on_error`, `Vdebug_ignored_errors`, `debug_on_quit`,
  `debug_on_next_call`, `backtrace_on_redisplay_error`, `debugger_may_continue`,
  `debugger_stack_frame_as_list`, `Vdebugger`, `Vsignal_hook_function`, `Vdebug_on_signal`,
  `backtrace_on_error_noninteractive`, `when_entered_debugger`,
  `Vinternal_interpreter_environment` and `Vinternal_make_interpreted_closure_function` are
  defined here with `#[unsafe(no_mangle)] pub static mut …`; the extractor's "File-scope
  objects" list cannot see them, which is why `tools/check-globals.py` exists (it reports
  `260 units, 0 DEFVAR_* variables without a definition in their own unit` after this unit).
  `list_of_t` is the one `static` object the extractor's list misses (C 2545 has a trailing
  comment on the definition line); the port defines it as a plain `static mut` with C's
  zero initial value and `syms_of_eval`'s `staticpro (&raw mut list_of_t)`.

- **The dead-arm policy applied (P-08 vs status 3.49).** `src/eval.c` is **not** in
  `docs/baseline/config/cond-eval-verified.txt`, so `tools/extract-abi.py`'s blind evaluator
  keeps both arms of every `#if` live and the per-function token check demands C's text on
  both sides: per docs/status.md 3.49 (PORTING.md P-51) the arms this configuration does not
  compile are **carried** here as `#[cfg (feature = "…")]` text naming the C condition's
  non-frozen side.  `python3 tools/check-cond-eval.py src/eval.c` reports
  `11 conditions, 0 unknown, 0 disagreements`; the eleven, in C order, and where each arm
  went:

  | C lines | condition | frozen verdict | carried as |
  | ---: | --- | --- | --- |
  | 293-296 | `HAVE_WINDOW_SYSTEM` | false | `#[cfg (feature = "have_window_system")]` block in `call_debugger`, with the two declarations it needs (`display_hourglass_p`, `cancel_hourglass`) inside it |
  | 316-319 | `#if 0` | dead everywhere | **recorded here, not transcribed** (the literal `#if 0` is dead for the inventory too): it binds `Qinhibit_eval_during_redisplay` around redisplay |
  | 1438-1443 | `HAVE_X_WINDOWS` | false | `#[cfg (feature = "have_x_windows")]` block in `unwind_to_catch` (the `x_unwind_errors_to` call and its declaration) |
  | 1643-1647 | `GCC_LINT && __GNUC__ && !__clang__` | false | `#[cfg (feature = "gcc_lint_not_clang")]` block in `internal_lisp_condition_case` (the `var = var_volatile` that pacifies GCC 14.2.1); a compiler predicate, so there is no `config.rs` constant to bind it to (status 3.9/3.49) |
  | 1862-1864 | `HAVE_X_WINDOWS` | false | `#[cfg (feature = "have_x_windows")]` block in `push_handler_nosignal` (`x_error_handler_depth` and its declaration) |
  | 2349-2357 | `HAVE_MODULES` | false | `#[cfg (feature = "have_modules")]` block in `Fcommandp`, with `module_function_interactive_form` declared inside it |
  | 3378-3381 | `HAVE_MODULES` | false | `#[cfg (feature = "have_modules")]` arm inside `funcall_lambda`'s branch chain (`funcall_module` declared there) |
  | 3382-3388 | `HAVE_NATIVE_COMP` | false | `#[cfg (feature = "have_native_comp")]` arm in the same chain (`lambda_list`) |
  | 3513-3516 | `HAVE_MODULES` | false | `#[cfg (feature = "have_modules")]` arm in `Ffunc_arity` (`module_function_arity`) |
  | 3840-3847 | `HAVE_MODULES` | false | `#[cfg (feature = "have_modules")]` arm in `do_one_unbind` (`finalize_runtime_unwind`, `finalize_environment_unwind`) |
  | 4364-4370 | `HAVE_MODULES` | false | `#[cfg (feature = "have_modules")]` arm in `mark_specpdl` (`mark_module_environment`) |

  `const _: () = assert! (!cfg! (feature = "have_modules") || HAVE_MODULES);` and its
  `have_native_comp`/`have_x_windows`/`have_window_system` siblings bind the features to the frozen
  constants, so a build that passes one of them without the constant fails; `have_modules` and
  `have_window_system`/`have_x_windows` are platform/preprocessor predicates, not `xtask dead-arms`
  FLIPS (the doc.rs precedent).  The `#if 0` and C 3378-3388's `HAVE_NATIVE_COMP` arm are
  also recorded in the "no Rust counterpart" sense in `docs/ports/excluded-macros.txt`'
  territory only in that they have no live text of their own — nothing was added there.

- **Statement-level deviations (each is a shape Rust cannot spell the same way).**
  * **`switch` on the specpdl `kind`** (`default_toplevel_binding`, `local_toplevel_binding`,
    `lexbound_p`, `do_one_unbind`, `specpdl_unrewind`, `mark_specpdl`,
    `Fbacktrace__locals`): the mirror carries `SPECPDL_*` as `u32` constants while
    `specbinding.kind` is `u8`, and a `match` pattern cannot carry a cast, so the arms are
    `if`/`else if` comparisons in C's own order, each reading `let kind: u8 = (*pdl).kind;`
    once (the data.rs precedent).  Every `break` becomes the arm's end and the two `FALLTHROUGH`s
    (`SPECPDL_LET` → `SPECPDL_LET_DEFAULT` in `do_one_unbind`/`specpdl_unrewind`, and
    `SPECPDL_LET_DEFAULT`/`_LOCAL` → `SPECPDL_LET` in `mark_specpdl`) are the shared body the
    fallthrough reaches.
  * **`switch` on `max_args` / `maxargs` / `h->type` / `redirect`**: a `match` with the same
    arm order (`eval_sub`, `funcall_subr`, `Fdefparam`'s chain, `Fdefvaralias`,
    `Ffunc_arity`'s `goto retry` → `continue 'retry`).
  * **`goto`**: `eval_sub`'s `retry:`, `funcall_general`'s `retry:`, `Ffunc_arity`'s
    `retry:` are labelled Rust loops with `continue 'retry` (PORTING.md 2.2: no early
    `return` is introduced anywhere).
  * **`for` loops with a single increment**: the C `for (init; cond; step)` with no
    `continue` becomes `init; while cond { …; step; }` (`or`, `and`, `cond`, `setq`,
    `Frun_hooks`, `Fhandler_bind_1`, `Fapply`'s spread loop, `apply_lambda`'s evaluator,
    `get_backtrace_frame`, `lambda_arity`, `get_backtrace`); where C *has* a `continue`
    (`signal_or_quit`'s handler scan, `Flet`'s two passes) the increment is written before
    the `continue 'label` so the loop's meaning is C's.
  * **`while (1)`** (`Fmacroexpand`) is `loop`.
  * **bitfields**: `sym->u.s.redirect`, `.trapped_write`, `.declared_special` and
    `blv->local_if_set`-style members are the packed `u.s.bits` word in the mirror
    (lisp.rs 221-228), read/written through `SYMBOL_REDIRECT_MASK`/`_SHIFT`,
    `SYMBOL_TRAPPED_WRITE_MASK`/`_SHIFT` and `SYMBOL_DECLARED_SPECIAL_BIT`
    (PORTING.md 1.3/P-80).  Each function that touches them has a
    `docs/ports/macro-body-notes.tsv` row (`Fdefvaralias`, `Finternal_delete_indirect_variable`,
    `Finternal__define_uninitialized_variable`, `Fdefvar`, `Fmake_var_non_special`, `FletX`,
    `Flet`, `Fspecial_variable_p`, `do_specbind`, `specbind`, `do_one_unbind`,
    `let_shadows_buffer_binding_p`, `specpdl_unrewind`), so the tokens the packed word
    cannot spell (`redirect`, `trapped_write`, `declared_special`) are exempted **per
    function** and every other token stays checked.
  * **`0`/`1` assigned to a `bool`** are `false`/`true` (`debug_on_next_call`,
    `redisplaying_p`, `debug_on_quit`, `debugger_may_continue`,
    `debugger_stack_frame_as_list`, `backtrace_on_error_noninteractive`,
    `backtrace_on_redisplay_error`, `skip_debugger`'s `first_string`, `wants_debugger`'s and
    `maybe_call_debugger`'s returns, `syms_of_eval`'s four assignments); the token rows
    record the other direction of the literal (`0`, `1`) for exactly those functions.
  * **`alloca`** (C 1630's bare `alloca`, not one of the `SAFE_ALLOCA` wrappers) is the same
    stack pool every other unit uses: `sa_stack_alloc (clausenb * size_of::<Lisp_Object> ())`
    (PORTING.md P-05's decision, docs/status.md 3.2).  This is the one place in the file
    where a C allocation has no same-name Rust counterpart, and it is the reason for the
    `alloca` token row on `internal_lisp_condition_case`.
  * **`record_unwind_protect_module (enum specbind_tag, void *)`** takes the tag as `u8`
    (the union's `kind` field is one byte; the `SPECPDL_*` names are the mirror's `u32`
    constants, and the port casts at the call).
  * **`NULL`** for the `unwind_ptr.mark` slots is `None` (the mirror's field is
    `Option<unsafe extern "C" fn (*mut c_void)>`), and C's `NULL` pointer tests are
    `is_null ()`/`! … is_null ()` (`backtrace_frame_apply`, the three `record_unwind_*` and
    `set_unwind_protect_ptr` have `null` rows; the pointer *comparisons* use the mirror's
    `is_null ()`, which `token-aliases.tsv`'s `null` family folds to C's spelling).
  * **`clobbered_eassert (E)`** is C 1370-1373's `static_assert (sizeof (E) != 0)`.  Rust
    cannot take the size of an expression that mentions the function's locals inside a
    `const` item (E0435), so the macro asserts the size of the expression's type family and
    keeps `E`'s spelling through `stringify!` — C does not evaluate `E` either (C 1373's own
    comment).  The body's only required token (`static_assert`) is present.
  * **`GDB_FUNCPTR`** keeps C's parameter names (`func`, `resulttype`, `params`) and C's
    pasted `_body` name as its fourth argument; `EXTERNALLY_VISIBLE` has no Rust counterpart
    besides the `#[unsafe(no_mangle)] pub` the port already writes (PORTING.md 8), which is
    the macro's one token row.
  * **`xsignal0`…`xsignal3`, `Fthrow`, `Fsignal`** are `-> !`: C's declarations make them
    `AVOID` (lisp.h 4792-4795) and §5.2/F8 requires the definition to agree.  `Fsignal`'s
    body therefore ends with `eassume! (false)` **plus** the `core::hint::unreachable_unchecked ()`
    C's `eassume` stands for in this configuration (`__builtin_unreachable`, lib/verify.h
    389) — the one extra statement, marked in the source.
  * **`funcall_lambda`** zero-initialises `syms_left`/`lexenv` because C's branch chain
    assigns them and the two `#ifdef` arms can only be reached in a build with
    `have_modules`/`have_native_comp` (PORTING.md 0.1-13's shape); the port's `else if { … }` arm keeps
    C's chain and `emacs_abort ()` tail.
  * **`unbind_to`** copies the binding out of the specpdl slot and then decrements the
    pointer, exactly as C's `this_binding = *--specpdl_ptr;` does — two statements
    (decrement, then copy) because Rust cannot dereference-and-decrement in a place
    expression, and the copy is what keeps C's "unbind what we copied even if unbinding
    signals" property.
  * **Parameter shadows**: every parameter C assigns to is shadowed with `let mut x = x;`
    (`or`, `and`, `cond`, `progn`, `setq`, `Fdefvaralias`'s `p`, `FletX`'s `varlist`… the
    `cmds.rs`/`marker.rs` shape); `run_hook` copies `hook` into a local because C passes the
    parameter's address to `Frun_hook_with_args`, which writes through it.
  * **No invented helpers**: the port defines exactly the functions C does, and adds no
    static of its own beyond `list_of_t` (C 2545) and `Qcatch_all_memory_full` (C 1799).

- **Use-site declarations (P-72 rule 3 / status 3.60).** lisp.rs's `other_units` is a
  partial mirror, so the 18 names this unit *calls* but does not define are declared in one
  block at the crate root, each with C's prototype in a `/// C:` line (the bytecode.rs
  shape): `abort` (libc), `add_to_log`, `default_value`, `evxprintf`, `exec_byte_code`,
  `fatal`, `get_byte_code_arity`, `malloc_probe`, `mark_object`, `memory_full`, `message3`,
  `notify_variable_watchers`, `process_pending_signals`, `save_excursion_save`,
  `save_excursion_restore`, `set_internal`, `set_poll_suppress_count`, plus the two
  `dispextern.h` counters `redisplaying_p`/`redisplay_counter` (as `pub static mut`).  The
  functions a `#[cfg]` arm needs (`cancel_hourglass`, `display_hourglass_p`,
  `x_unwind_errors_to`, `x_error_message_count`, `funcall_module`,
  `module_function_arity`, `module_function_interactive_form`, `finalize_runtime_unwind`,
  `finalize_environment_unwind`, `mark_module_environment`) are declared **inside** their
  arm, so a frozen build never sees them.  Nothing declared in that block is defined here
  (E0428 would fire), and `buffer_local_value`/`set_buffer_if_live` are *not* declared here
  because buffer.rs already declares them.  `tools/check-undef.py` is the gate for this
  direction: it reports no new undefined name from `eval.o`.
  **`get_act_rec`/`set_act_rec`** are the exception: they are `INLINE` *definitions* in
  lisp.h 5233-5243 that the mirror does not carry (status P-20's open item), and they are
  `static inline` in C, so the port defines them where C's per-unit copy lives — as local
  `#[inline(always)] unsafe fn`s with C's bodies and `/// C:` lines.  When `lisp.rs` gains
  them, the two definitions here must be dropped in the same commit (the ccl_spec
  precedent).

- **Mirror defects this unit found (integrator owns the file).**
  1. **`safe_calln!` was wrong; FIXED by the integrator (status 3.69)** - `src/include/lisp.rs`: C's
     `safe_calln` is `CALLMANY (safe_funcall, ((Lisp_Object []) {__VA_ARGS__}))`
     (lisp.h 4845-4847) — a call to the error-catching `safe_funcall` you find at
     `emacs/src/eval.c:3230`.  The mirror expands to `calln!` (`CALLN! (Ffuncall, …)`)
     **plus** a `maybe_quit ()` C does not make there.  Two differences (callee and the extra
     call), and every `safe_calln!` call site in the tree (composite.rs 1102, keymap.rs 671,
     print.rs 1299, syntax.rs 533, terminal.rs 519, plus this unit) loses the
     `inhibit-redisplay` binding and the `internal_condition_case_n` error catch.  This unit
     spells C's expansion out in `safe_eval` instead of becoming unfaithful through the
     macro, and records the token in `macro-body-notes.tsv`.
  2. **`lisp.rs` declares `Vquit_flag`… (the `V*` storages) but the test of an indirect
     variable's `where` in `specbind`'s `eassert`** needs `Lisp_Buffer_Local_Value`, which
     the mirror does carry (data.rs uses it) — not a defect, kept for the record.
  3. **`dispextern.h` 3608-3609's `redisplaying_p`/`redisplay_counter`** have no mirror
     declaration; this unit declares them at the use site (above).  They belong in
     `dispextern.rs` when it lands.

- **The `#if 0` and other text not transcribed.** C 316-319's `#if 0` binds
  `Qinhibit_eval_during_redisplay` so that redisplay cannot run Lisp code; it is dead for the
  inventory (a literal `#if 0` is dead everywhere) and is recorded here rather than carried.
  C's 12 `static` forward declarations are dropped (PORTING.md 0.1-12: they are E0428 in
  Rust) — `funcall_lambda`, `apply_lambda`, `lambda_arity`, `init_eval_once_for_pdumper`,
  `signal_or_quit`, `find_handler_clause`, `maybe_call_debugger`, `safe_eval_handler`,
  `with_delayed_message_display`/`_cancel`, `do_one_unbind` — and the definitions stay where C
  has them.

- **Line-count relation.** C 4,680 lines (live 4,681 by the tool's own count, which reads the
  file's `\n` count); the port is 4,741 lines, a drift of **+1.3 %** against C's total — inside
  the default 5 %, so **no `docs/ports/line-drift-tolerance.tsv` row** is added and the
  baseline is the default `total`.  The port is longer because of the statement-per-line
  `if`/`else if` chains that replace C's `switch` arms, the 27 use-site declarations, the
  `#[cfg]`-carried dead arms, the `#[repr(C)]` type with its assertions, and the 19 owned
  `V*` definitions.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the
same argument order, and the port adds no call C does not make.  The names that *look*
different are the mirror expansions at the same points — `eval_sub`/`Ffuncall`/`CALLMANY`/
`calln`/`apply1`, `SAFE_ALLOCA`/`SAFE_FREE`/`SAFE_FREE_UNBIND_TO`, `XSETSYMBOL`,
`SET_SYMBOL_VAL`, `XSETSUBR`, `FOR_EACH_TAIL` — plus the `*mut c_void`/`*mut KBOARD`
spellings P-72 rule 3 forces on `set_default_internal`'s fourth parameter and the
`get_act_rec`/`set_act_rec` pair (above).  `safe_eval`'s expansion of `safe_calln` *used to* call the mirror's wrong form; the mirror now expands to `CALLMANY! (safe_funcall, arr)` - C's own body - and `safe_funcall` is declared in the mirror through `lisp_safe_decls` + glob import (this unit defines it).

  The integrator's fix, in full (status 3.69): 
`safe_funcall`, which is C's own callee.  Verified by diffing the call-name multiset of every
C body against its Rust counterpart; the only differences are C's `memcpy`/`memclear` sizes
written as multiplies (`word_size`), C's `countof (argbuf)` spelled `8`, and the
`false`/`true` spellings of C's `0`/`1` boolean arguments.

### Integrator note (the two things this unit needed from outside itself)

1. **`tools/cinventory.py` could not see two of this file's C definitions** — the exact shape
   the gate then reported as an invented exported symbol:
   * `Lisp_Object backtrace_top_function (void)` (C 4415) — the *typed* one-line declarator
     whose `{` is on the next line.  `function_spans`'s `elif typed: pass` arm accepted a
     typed declarator "only when `{` shares its line".
   * `Lisp_Object\nload_with_autoload_queue\n  (…)` (C 2464-2467) — a declarator whose *name*
     has a line of its own, the emacs.c-`main` class (status 3.60) in a second spelling.
   Both definitions are non-`static` in C and are referenced by other units
   (`lread.c`/`profiler.c`), so the port keeps `#[unsafe(no_mangle)]`: dropping it to satisfy
   the gate would have been a silent loss of two symbols, which is the class status 3.60
   exists to catch.  `tools/cinventory.py` was therefore fixed — see the patch below — and
   the fix's blast radius was measured over all 330 `*.c` of the eight linked reference
   directories: exactly **8 files change, each by one added function** (this one by two),
   and no pre-existing function's span or linkage moves, so no other unit's body
   comparison can change.  Of those, `buffer.c`,
   `fns.c`, `keyboard.c`, `window.c` and `oldXMenu/XLookAssoc.c` have no port yet, `w32fns.c`
   is not built on this target, `eval.c` is this unit, and `minibuf.c`'s port already carries
   `minibuf_c_loop_level` — its `--check` was re-run after the change and still prints `OK`.
   The patch: keep `Lisp_Object … (params)` declarators whose parameter list ends the line
   (`{` on a following live line), and key the bare-name declarator shape through a new
   `BARE_NAME` regex; both places carry the measurement in their comments.
2. **`lisp.rs`'s `safe_calln!`** — see the mirror-defect item above (callee and the extra
   `maybe_quit ()`); this unit reports it rather than editing the mirror.
