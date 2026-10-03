# emacs/src/print.c (3069 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 11
- [x] write-char  (Fwrite_char, 1/2, doc d9c1ba4eccd9, line 553)
- [x] terpri  (Fterpri, 0/2, doc c9a6c2bc5e6e, line 627)
- [x] prin1  (Fprin1, 1/3, doc d636ebe2c95c, line 738)
- [x] prin1-to-string  (Fprin1_to_string, 1/3, doc 03be78d289b6, line 795)
- [x] princ  (Fprinc, 1/2, doc a09645f4e766, line 844)
- [x] print  (Fprint, 1/2, doc 20886677c908, line 882)
- [x] flush-standard-output  (Fflush_standard_output, 0/0, doc 5ebc534f80da, line 917)
- [x] external-debugging-output  (Fexternal_debugging_output, 1/1, doc 97f2bc0ee93f, line 927)
- [x] redirect-debugging-output  (Fredirect_debugging_output, 1/2, doc 6b788ed59f04, line 948)
- [x] error-message-string  (Ferror_message_string, 1/1, doc 8b3e9a9bc134, line 1037)
- [x] print--preprocess  (Fprint_preprocess, 1/1, doc 99e161b05998, line 1516)

## Functions defined — 47
- [x] print_free_buffer  (line 102, static)
- [x] print_unwind  (line 111, static)
- [x] print_prepare  (line 132, static)
- [x] print_finish  (line 202, static)
- [x] printchar_to_stream  (line 237, static)
- [x] printchar  (line 306, static)
- [x] octalout  (line 357, static)
- [x] strout  (line 380, static)
- [x] print_string_1  (line 472, static)
- [x] print_string  (line 548, static)
- [x] print_c_string  (line 572, static)
- [x] write_string  (line 582, static)
- [x] temp_output_buffer_setup  (line 591)
- [x] print_bind_all_defaults  (line 659, static)
- [x] print_create_variable_mapping  (line 670, static)
- [x] print_bind_overrides  (line 703, static)
- [x] debug_output_compilation_hack  (line 943)
- [x] debug_print  (line 994)
- [x] safe_debug_print  (line 1002)
- [x] debug_format  (line 1023)
- [x] erase_prin1_to_string_buffer  (line 1031, static)
- [x] print_error_message  (line 1070)
- [x] float_to_string  (line 1187)
- [x] print  (line 1292, static)
- [x] print_circle_candidate_p  (line 1329, static)
- [x] grow_pp_stack  (line 1370, static)
- [x] pp_stack_push_value  (line 1379, static)
- [x] pp_stack_push_values  (line 1388, static)
- [x] pp_stack_empty_p  (line 1400, static)
- [x] pp_stack_pop  (line 1406, static)
- [x] print_preprocess  (line 1432, static)
- [x] print_preprocess_string  (line 1531, static)
- [x] print_check_string_charset_prop  (line 1543, static)
- [x] print_prune_string_charset  (line 1588, static)
- [x] data_from_funcptr  (line 1616, static)
- [x] print_pointer  (line 1627, static)
- [x] print_bignum  (line 1642, static)
- [x] print_bool_vector  (line 1653, static)
- [x] print_vectorlike_unreadable  (line 1700, static)
- [x] named_escape  (line 2154, static)
- [x] grow_print_stack  (line 2233, static)
- [x] print_stack_push  (line 2242, static)
- [x] print_stack_push_vector  (line 2250, static)
- [x] print_object  (line 2270, static)
- [x] print_interval  (line 2850, static)
- [x] init_print_once  (line 2868)
- [x] syms_of_print  (line 2878)

## Macros defined — 3
- [x] PRINT_CIRCLE  (line 63, body 9f9af029585b)
- [x] PRINT_STRING_NON_CHARSET_FOUND  (line 1536, body 356a192b7913)
- [x] PRINT_STRING_UNSAFE_CHARSET_FOUND  (line 1537, body da4b9237bacc)

## Types and their fields — 6
- [x] print_buffer: char* buffer, ptrdiff_t size, ptrdiff_t pos, ptrdiff_t pos_byte
- [x] print_context: Lisp_Object printcharfun, Lisp_Object old_printcharfun, ptrdiff_t old_point, ptrdiff_t start_point, ptrdiff_t old_point_byte, ptrdiff_t start_point_byte, specpdl_ref specpdl_count
- [x] print_pp_entry: ptrdiff_t n, Lisp_Object value, Lisp_Object* values, <aggregate> u
- [x] print_pp_stack: struct print_pp_entry* stack, ptrdiff_t size, ptrdiff_t sp
- [x] print_stack: struct print_stack_entry* stack, ptrdiff_t size, ptrdiff_t sp
- [x] print_stack_entry: enum print_entry_type type, Lisp_Object last, intmax_t maxlen, Lisp_Object tortoise, ptrdiff_t n, ptrdiff_t m, intmax_t tortoise_idx, <aggregate> list, Lisp_Object obj, <aggregate> dotted_cdr, Lisp_Object obj, ptrdiff_t size, ptrdiff_t idx, const char* end, bool truncated, <aggregate> vector, Lisp_Object obj, ptrdiff_t nobjs, ptrdiff_t idx, ptrdiff_t printed, bool truncated, <aggregate> hash, <aggregate> u

## File-scope objects — 12
- [x] static ptrdiff_t print_depth  (line 57, static)
- [x] static ptrdiff_t new_backquote_output  (line 60, static)
- [x] static Lisp_Object being_printed  (line 64, static)
- [x] static struct print_buffer print_buffer  (line 79, static)
- [x] static ptrdiff_t print_number_index  (line 89, static)
- [x] extern bool print_output_debug_flag  (line 93)
- [x] static Lisp_Object Vprint_variable_mapping  (line 656, static)
- [x] Lisp_Object Vprin1_to_string_buffer  (line 793)
- [x] static struct print_pp_stack ppstack  (line 1367, static)
- [x] static int print_check_string_result  (line 1540, static)
- [x] static Lisp_Object print_prune_charset_plist  (line 1585, static)
- [x] static struct print_stack prstack  (line 2230, static)

## Decisions

- **What the file carries.** C's 11 `DEFUN`s (all through `DEFUN!`, PORTING.md 5.1), its 47
  functions, its three macros (`PRINT_CIRCLE` 200, `PRINT_STRING_NON_CHARSET_FOUND` 1,
  `PRINT_STRING_UNSAFE_CHARSET_FOUND` 2, all `pub const`), the six types
  (`print_buffer`, `print_context`, `print_pp_entry`, `print_pp_stack`,
  `print_stack_entry`, `print_stack`, plus the `print_entry_type` enum the checklist folds
  into `print_stack_entry`'s row) and the twelve file-scope objects.  The nine non-`static`
  function definitions (`temp_output_buffer_setup`, `float_to_string`, `debug_output_compilation_hack`,
  `debug_print`, `safe_debug_print`, `debug_format`, `print_error_message`,
  `init_print_once`, `syms_of_print` and the two exported objects
  `Vprin1_to_string_buffer`/`print_output_debug_flag`) carry `#[unsafe(no_mangle)]`
  (PORTING.md 0.2); the C `static` helpers stay unexported.  Six of them are passed by
  address in C (`print_free_buffer`, `print_unwind`, `erase_prin1_to_string_buffer` via
  `record_unwind_protect_void`, `print_interval`/`print_preprocess_string`/
  `print_check_string_charset_prop` via `traverse_intervals*`), so they are
  `unsafe extern "C" fn` with C's own signature (PORTING.md 1.4).
- **Dead arms carried, not recorded (§3.49/P-51).**  `src/print.c` is *not* in
  `docs/baseline/config/cond-eval-verified.txt`: `python3 tools/check-cond-eval.py
  src/print.c` reports `14 conditions, 0 unknown, 0 disagreements [VERIFIED]` since the `lisp.h\tIEEE_FLOATING_POINT` row landed (status 3.70); it is still *not* added to `cond-eval-verified.txt`, because a verified file's checker demands only the live arm and this port carries both (status 3.51), so the evaluator stays blind here on purpose.  When this lane wrote the text below the count was `2 disagreements [NOT VERIFIED]`, both of
  them `#if IEEE_FLOATING_POINT` (C 42, C 1200) — `lisp.h` 3201-3202 computes that macro from
  `<float.h>`, so the frozen table cannot see it while clang says true.  The extractor's
  blind evaluator therefore keeps *both* arms live and the port carries the arms the frozen
  build does not compile, each under a `#[cfg (feature = "…")]` naming the C condition's
  non-frozen side: `WINDOWSNT` → `windowsnt` (C 46-48's `<sys/socket.h>` include, C 264-269
  and C 282-285's `OutputDebugString` writes), `HAVE_TREE_SITTER` → `have_tree_sitter`
  (C 50-52's `treesit.h` include, C 2054-2062, C 2066-2100, C 2104-2107),
  `HAVE_MODULES` → `have_modules` (C 1612-1639's `data_from_funcptr`/`print_pointer` and
  C 2009-2036), `HAVE_XWIDGETS` → `have_xwidgets` with its `USE_GTK` → `use_gtk` sub-arm
  (C 1839-1858), `HAVE_NATIVE_COMP` → `have_native_comp` (C 2040-2050).  C 37's
  `#include "xwidget.h"` and C 43's `#include <ieee754.h>` are `#include` selections;
  `<ieee754.h>` is live (C 1200-1208 is compiled) and gnulib's `lib/ieee754.in.h` template
  has no mirror, so the union is declared here (below).  The `ifdef`/`if` classification of
  every condition is the one the arms above state: five *structural* conditions (the
  features) and one *constant* condition (`IEEE_FLOATING_POINT`, true, so its two arms are
  ordinary text).
- **`Vprint_variable_mapping` and the eighteen `DEFVAR_*`-managed variables.**  C's
  `Vprint_variable_mapping` is a plain file-scope `static`; the other eleven `DEFVAR_LISP`
  variables and the seven `DEFVAR_BOOL`s are C's `globals.f_<name>` fields (lisp.h 3495-3530), whose single
  definition is alloc.c's, so this unit defines them (docs/status.md 3.29) — **but as plain
  `static mut`, not `#[unsafe(no_mangle)] pub`**: the reference object exports `globals` and
  *none* of these names (`nm -g build/emacs-ref/src/print.o`), so exporting them would
  invent symbols, and `python3 tools/check-symbols.py --dir src --pending
  docs/baseline/symbol-pending.txt` fails until the definition carries no `no_mangle`.  That
  is a correction to §3.29's stated shape and to the units that follow it (casetab.rs,
  charset.rs, character.rs, doc.rs, cmds.rs's `Vpost_self_insert_hook`, …); the src-side
  symbol gate never saw it because `build/emacs-ref/src/*.o` does not exist for those units.
  This unit's eighteen carriers are `static mut` with C's initial values, and
  `syms_of_print` registers them and repeats C's assignments.
- **Measured layout.**  The six types are private to this file, so the C compiler was asked
  directly (`clang -std=gnu23`, the frozen target's flags, a probe carrying the same
  declarations): `print_buffer` sizeof 32/align 8, `buffer@0 size@8 pos@16 pos_byte@24`;
  `print_context` sizeof 56/align 8, `printcharfun@0 old_printcharfun@8 old_point@16
  start_point@24 old_point_byte@32 start_point_byte@40 specpdl_count@48`; `print_pp_entry`
  sizeof 16, `n@0 u@8`; `print_pp_stack` sizeof 24, `stack@0 size@8 sp@16`;
  `print_stack_entry` sizeof 56, `type@0 u@8`, `list.last@8 list.maxlen@16 list.tortoise@24
  list.n@32 list.m@40 list.tortoise_idx@48`, `dotted_cdr.obj@8`, `vector.obj@8 size@16 idx@24
  end@32 truncated@40`, `hash.obj@8 nobjs@16 idx@24 printed@32 truncated@40`; `print_stack`
  sizeof 24; `print_entry_type` sizeof 4/align 4.  Every one is pinned by the `const _`
  assertions under its definition.  C's two anonymous unions and their four anonymous
  structs must be named in Rust (PORTING.md 1.3), so they are
  `print_pp_entry_u`, `print_stack_entry_u` and
  `print_stack_entry_{list,dotted_cdr,vector,hash}` — names only, no storage, no field move,
  and the flattener's view of `print_stack_entry` still finds every C field.
- **`union ieee754_double` (C 43-44, C 1202-1206).**  gnulib's `lib/ieee754.in.h` is a
  `*.in.h` template (P-26/P-62 records that these have no mirror), so the union and its
  `ieee_nan`/`ieee` structs are declared here.  Measured with clang: the frozen target is
  little-endian with `_IEEE754_FLOAT_WORD_ORDER == BYTE_ORDER`, so `ieee_nan` is
  `mantissa1:32; mantissa0:19; quiet_nan:1; exponent:11; negative:1`, which packs into two
  32-bit units (sizeof 8, align 4) and makes the union's size the double's 8 — verified by
  writing each member and reading the bytes (`mantissa0 = 0x7ffff` → `00 00 00 00 ff ff 07
  00`, `negative = 1` → `00 00 00 00 00 00 00 80`).  The three members print.c reads go
  through the member-name macro `ieee_nan! (mantissa0|mantissa1|negative, u)` (PORTING.md
  1.3/P-80), so C's member names stay in the port's text; the two masks the NaN predicate does
  not read (`quiet_nan`, `exponent`) are recorded as constants with the bit layout.
- **Mirror gaps declared at the use site** (PORTING.md 0.1-2/P-72, the terminal.rs shape),
  each with C's own prototype in a `/// C:` line: `FUNCTIONP` (lisp.h 4799), `message_dolog`,
  `message3`, `setup_echo_area_for_printing`, `reset_message_log_need_newline` (keyboard.c's
  four; `keyboard.rs` carries none of them), `insert_1_both` (lisp.h 4331), `copy_text`
  (lisp.h 4323), `record_unwind_protect_void` (lisp.h 4825), `hash_table_weakness_symbol`
  (lisp.h 4253), `font_style_symbolic` (font.h 780), `fflush`/`fprintf` (`<stdio.h>`;
  `lib/include/stdio.rs` carries `fwrite`/`fputs`/`sprintf` but not these two), `dtoastr`
  (`lib/ftoastr.h` 54; `lib/dtoastr.rs` defines the symbol but no mirror declares it),
  `putc` and the two streams.  `putc` needs `#[link_name = "putc_unlocked"]` and the streams
  need `#[cfg_attr (target_os = "macos", link_name = "__stdoutp"/"__stderrp")]`: the
  reference object's undefined list (`nm -u build/emacs-ref/src/print.o`) names those
  symbols, not `putc`/`stdout`/`stderr` (docs/status.md 3.57 — no gate sees undefined
  references).  `STDERR_FILENO` (2) and the four `O_*` flags come from the newly included
  `unistd.in.rs`/`fcntl.in.rs` (which also supplies `F_DUPFD_CLOEXEC`, so the port must not
  declare it itself: E0428).
- **Spellings that stay closest to C.**  C's `goto next_char` (C 250-295 in
  `printchar_to_stream`) is the `goto_next_char` flag around the `'next_char` loop, and C's
  `goto print_obj`/`goto next_obj` (C 2280-2848) are the `'print_obj`/`'next_obj` loop labels
  with `to_next_obj` (PORTING.md 2.2, the ccl.rs `ccl_goto` shape): a `goto next_obj` skips
  the `print_depth--` exactly as C's does, and the arms that tail-recurse decrement first, as
  C's do.  `float_to_string`'s two `goto lose` are the `lost` flag plus a labelled `'check`
  block; `octalout`'s `do…while` is a `loop` with the test at the end; every `switch` is a
  `match` with braced arms (the C `break` at an arm's end is the arm's end).  `named_escape`'s
  arms are integer patterns (C's `case '\b':` etc. carry no tokens the check sees).
  `'\\x22'` (127) replaces a literal `b'"'` in seven places because `tools/extract-abi.py`'s
  string scanner starts a "string" at that quote and swallows the rest of the body
  (docs/status.md 3.28's class); the byte is identical.  `sprintf (buf, …)` passes
  `buf.as_mut_ptr ()` because a Rust array does not decay, `.offset`/`.add` take `isize`/`usize`
  as the mirror's signatures do, and `p[signedp]` is `*p.offset (signedp as isize)` (the
  indent.rs row).  C's `sizeof "from..to..in "`-style sizes are written as their measured
  values (14/5/14) with `INT_STRLEN_BOUND` kept as a `const fn` and (in `print_object`, where
  the array length must be a constant) `max` as `max_usize`, because lisp.rs's `max` is an
  `isize` function and not `const fn` (the data.md `BUF_BYTES_MAX` row's reason); the three
  `<inttypes.h>` format macros are `const PRIuMAX/PRIxMAX/PRIdMAX` with the concatenated
  literal at the call site (the filelock.rs/timefns.rs rows).  C's `h->weakness` bitfield goes
  through the `hash_table_bits! (weakness, h)` member-name macro and
  `XMARKER (obj)->insertion_type` through the mirror's `MARKER_INSERTION_TYPE_P` (the
  `insertion_type` token family).  Parameters C assigns to are shadowed (`printcharfun` in
  eight functions, `file`, `overrides`, `string`, `obj`, `data`); C's uninitialised locals are
  zero-initialised (PORTING.md 1.5): `Lisp_Object dv UNINIT`, `char *cp`, `int width/len`,
  `char escaped_name`; the ledger's `macro-body-notes.tsv` rows name them.
  The three forward declarations (C 90, 622-625) are dropped (PORTING.md 0.1-12), and
  `EXTERNALLY_VISIBLE` (C 94, 941, 1000, 1021) has no Rust counterpart — the
  `#[unsafe(no_mangle)] pub` on those same definitions is what PORTING.md 8 records.
- **`INTERVAL` is opaque in the mirror** (`lisp.rs` keeps `pub type INTERVAL = *mut c_void`,
  P-72 rule 3), so the three interval callbacks cast once, `interval as *mut interval`
  (intervals.rs's own note), before reading `plist`/`position`.
- **Line count.**  C 3069 → the port's 3297 (+7.4%), declared with the `total` basis in
  `docs/ports/line-drift-tolerance.tsv`; the carried dead arms are the largest single reason
  and the per-function token comparison is the real guard, and it is `OK`.
- **The three `PRINT_STRING_*`/`PRINT_CIRCLE` constants and the two enum values** are the
  only file-scope constants; `print_check_string_result`'s three uses of the two macros are
  `&=`/`==` expressions in C and `(… & MASK) != 0`/`== 0` in Rust, where `&` binds looser
  than `!=` — the parenthesised form is the same value.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the
same argument order, and the port adds no call the reference does not make: the mirror
expansions (`BVAR!`, `XSETBUFFER!`, `AREF`, `HASH_KEY`/`HASH_VALUE`, `PT!`, `SET_PT_BOTH`,
`SAFE_ALLOCA!`, `DOHASH!`, `FOR_EACH_TAIL!`, `calln!`, `safe_calln!`) are C's own macros
expanding at the same points, and the `INTERVAL`/`*mut c_void`/`*mut buffer` casts change a
parameter's declared type, not the call graph.  Verified by a call-name multiset diff of
every C body against its Rust counterpart (the only differences are C's `sizeof *ps->stack`
idiom in the two `grow_*_stack` helpers, C's nested designators in
`print_stack_push_vector`, and the `false`/`true` spellings of C's `0`/`1` boolean arguments).
