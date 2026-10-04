# emacs/src/keymap.c (3487 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 29
- [x] make-keymap  (Fmake_keymap, 0/1, doc 6f0fe59b54a1, line 93)
- [x] make-sparse-keymap  (Fmake_sparse_keymap, 0/1, doc 30fecab5daef, line 110)
- [x] keymapp  (Fkeymapp, 1/1, doc d25e3acb6699, line 135)
- [x] keymap-prompt  (Fkeymap_prompt, 1/1, doc 76c1a7286a3d, line 148)
- [x] keymap-parent  (Fkeymap_parent, 1/1, doc 56e2c2593cc7, line 253)
- [x] set-keymap-parent  (Fset_keymap_parent, 2/2, doc 36b5b42b6e00, line 273)
- [x] map-keymap-internal  (Fmap_keymap_internal, 2/2, doc 450b725b5a36, line 613)
- [x] map-keymap  (Fmap_keymap, 2/3, doc 0a2aa5e3e763, line 625)
- [x] keymap--get-keyelt  (Fkeymap__get_keyelt, 2/2, doc 5907525ed917, line 646)
- [x] copy-keymap  (Fcopy_keymap, 1/1, doc 2d0a92ce0cce, line 1018)
- [x] define-key  (Fdefine_key, 3/4, doc 6104843eafff, line 1077)
- [x] command-remapping  (Fcommand_remapping, 1/3, doc 27dc64306d2d, line 1216)
- [x] lookup-key  (Flookup_key, 2/3, doc af129df0a038, line 1290)
- [x] current-active-maps  (Fcurrent_active_maps, 0/2, doc 6cb62b688ac1, line 1649)
- [x] key-binding  (Fkey_binding, 1/4, doc d024e4ea2a86, line 1788)
- [x] minor-mode-key-binding  (Fminor_mode_key_binding, 1/2, doc 51c5ec194d62, line 1857)
- [x] use-global-map  (Fuse_global_map, 1/1, doc b69f686ab8a7, line 1890)
- [x] use-local-map  (Fuse_local_map, 1/1, doc d45bd17af58b, line 1900)
- [x] current-local-map  (Fcurrent_local_map, 0/0, doc 13ecc3b6911d, line 1913)
- [x] current-global-map  (Fcurrent_global_map, 0/0, doc e23407fa894f, line 1921)
- [x] current-minor-mode-maps  (Fcurrent_minor_mode_maps, 0/0, doc 1bfc1e61d975, line 1928)
- [x] accessible-keymaps  (Faccessible_keymaps, 1/2, doc b60fb9f9af43, line 2010)
- [x] key-description  (Fkey_description, 1/2, doc ff4484db99a4, line 2092)
- [x] single-key-description  (Fsingle_key_description, 1/2, doc 2304c532f9c9, line 2307)
- [x] text-char-description  (Ftext_char_description, 1/1, doc b5b3e8f573ed, line 2391)
- [x] where-is-internal  (Fwhere_is_internal, 1/5, doc 3ff592a84273, line 2588)
- [x] describe-buffer-bindings  (Fdescribe_buffer_bindings, 1/3, doc 2f4a78bf33fe, line 2834)
- [x] describe-vector  (Fdescribe_vector, 1/2, doc c1994f3164d9, line 3042)
- [x] help--describe-vector  (Fhelp__describe_vector, 7/7, doc 6405850ee26d, line 3074)

## Functions defined — 37
- [x] CHECK_VECTOR_OR_CHAR_TABLE  (line 86, static)
- [x] initial_define_lispy_key  (line 129)
- [x] get_keymap  (line 192)
- [x] keymap_parent  (line 237, static)
- [x] keymap_memberp  (line 263, static)
- [x] access_keymap_1  (line 327, static)
- [x] access_keymap  (line 491)
- [x] map_keymap_item  (line 499, static)
- [x] map_keymap_char_table_item  (line 519, static)
- [x] map_keymap_internal  (line 535, static)
- [x] map_keymap_call  (line 575, static)
- [x] map_keymap  (line 583)
- [x] map_keymap_canonical  (line 604)
- [x] get_keyelt  (line 678, static)
- [x] store_in_keymap  (line 730, static)
- [x] copy_keymap_item  (line 907, static)
- [x] copy_keymap_set_char_table  (line 967, static)
- [x] copy_keymap_1  (line 976, static)
- [x] possibly_translate_key_sequence  (line 1050, static)
- [x] lookup_key_1  (line 1245, static)
- [x] define_as_prefix  (line 1445, static)
- [x] append_key  (line 1456, static)
- [x] silly_event_symbol_error  (line 1466, static)
- [x] current_minor_maps  (line 1531)
- [x] click_position  (line 1639, static)
- [x] accessible_keymaps_1  (line 1947, static)
- [x] push_key_description  (line 2192)
- [x] push_text_char_description  (line 2372, static)
- [x] preferred_sequence_p  (line 2426, static)
- [x] shadow_lookup  (line 2464, static)
- [x] where_is_internal  (line 2496, static)
- [x] where_is_internal_1  (line 2787, static)
- [x] describe_vector_princ  (line 3029, static)
- [x] describe_vector_basic  (line 3037, static)
- [x] describe_key_maybe_fontify  (line 3062, static)
- [x] describe_vector  (line 3139, static)
- [x] syms_of_keymap  (line 3335)

## Macros defined — 0

## Types and their fields — 3
- [x] accessible_keymaps_data: Lisp_Object maps, Lisp_Object tail, Lisp_Object thisseq, bool is_metized
- [x] map_keymap: map_keymap_function_t fun, Lisp_Object args, void* data, <aggregate> s
- [x] where_is_internal_data: Lisp_Object definition, Lisp_Object this, Lisp_Object last, bool last_is_meta, bool noindirect, Lisp_Object sequences

## File-scope objects — 10
- [x] static Lisp_Object exclude_keys  (line 62, static)
- [x] static Lisp_Object command_remapping_vector  (line 65, static)
- [x] static Lisp_Object unicode_case_table  (line 68, static)
- [x] static Lisp_Object where_is_cache  (line 71, static)
- [x] static Lisp_Object where_is_cache_keymaps  (line 73, static)
- [x] static Lisp_Object cmm_modes  (line 1509, static)
- [x] static ptrdiff_t cmm_size  (line 1510, static)
- [x] static int where_is_preferred_modifier  (line 2420, static)
- [x] static Lisp_Object Vmouse_events  (line 2480, static)
- [x] static Lisp_Object fontify_key_properties  (line 3059, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/keymap.c` @ `fd7afced403`; port `femacs/src/keymap.rs` (3,456 lines vs C's 3,487
total — **-0.9% on the default `total` basis**, inside the 5% the rule allows every file, so
**no row was added to `docs/ports/line-drift-tolerance.tsv`**; the lines the port saves are
mostly the twenty-nine `DEFUN!` docstrings, which collapse C's multi-line `doc: /* … */`
comments into one literal each, and they roughly pay for the port's fixed costs (the
`// femacs:` note, the `#![allow]` block, the include list and the use-site declarations).  The per-function token comparison is the real
guard and it is `OK`.  All 79 ledger rows are checked.  The gates, verbatim:

    build-tu src/keymap.c         -> built src/keymap.rs -> ../build/femacs/keymap.o
    extract-abi.py --check        -> OK emacs/src/keymap.c -> femacs/src/keymap.rs
    check-symbols.py              -> check-symbols: 78 units compared, 1 mismatched, 0 still pending
    check-globals.py              -> check-globals: 255 units, 0 DEFVAR_* variables without a definition in their own unit
    check-undef.py                -> check-undef: 504 unit object(s), 88 reference object(s)/archive(s): 566 pending, 0 new
    ledger                        -> 79/79 rows checked, 0 unchecked
    (extra) rustc --cfg enable_checking on the materialized unit -> exit 0, 0 errors
      run from `femacs/`: `rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort
      -C codegen-units=1 -C opt-level=0 --cfg enable_checking -o /tmp/keymap-ec.o
      ../build/femacs/units/keymap/_root.rs`; the only diagnostics are the two pre-existing
      mirror warnings (include/lisp.rs 3836's unused doc comment and include/frame.rs 589's
      duplicate `#[inline(always)]`), neither of which is this unit's.

The `1 mismatched` unit in `check-symbols` is `lib/regex` (`_rpl_re_*` missing), which is
another worker's in-flight unit — `src/keymap` is compared and matches.  The same is true of
the dirty `tools/extract-abi.py`, `docs/ports/include-regex-emacs.md` and
`femacs/src/include/regex-emacs.rs` in the working tree: not this unit's.

### Dead arms: none (P-08 has nothing to record, status 3.49 does not apply)

`src/keymap.c` **is** in `docs/baseline/config/cond-eval-verified.txt`, and
`python3 tools/check-cond-eval.py src/keymap.c` reports `0 conditions, 0 unknown,
0 disagreements [VERIFIED]`: the file contains **no** `#if`/`#ifdef`/`#ifndef`/`#if 0` at all
(`grep -n '#[[:space:]]*if' emacs/src/keymap.c` returns only the thirteen `#include` lines).
So there is no dead arm to record and no arm to carry under `#[cfg (feature = "…")]`; the port
transcribes C's whole text and the drift basis stays the default `total`.

### Layout (`#[repr(C)]` + clang-measured `const _` assertions)

The three types this unit declares (the union and the two `struct`s C declares inside the
`.c`; `tools/gen-layout-fields.py`'s `STRUCTS` list does not name them, the ccl.rs precedent)
are measured with `clang -std=gnu23` on the frozen target — a probe with `<stddef.h>` and the
three records, printing `sizeof`/`_Alignof`/`offsetof`:

    accessible: 32/8 maps@0 tail@8 thisseq@16 is_metized@24
    where_is: 40/8 definition@0 this@8 last@16 last_is_meta@24 noindirect@25 sequences@32
    map_keymap: 24/8 union-s@0 fun@0 args@8 data@16

`struct accessible_keymaps_data` carries 6 `const _` assertions, `struct
where_is_internal_data` 6, `map_keymap_s` 5 and `union map_keymap` 4 (13 for the union and its
member struct together).  `GCALIGNED_UNION_MEMBER` (`char alignas (GCALIGNMENT) gcaligned;`,
lisp.h 303) is the union's `#[repr(align(8))]` plus the marker member not written, and
`static_assert (GCALIGNED (union map_keymap))` is the `align_of % 8 == 0` assertion (the
thread.rs shape).  The anonymous `struct` inside the union is named `map_keymap_s` (PORTING.md
1.3 requires a name for a nested anonymous record) and is `#[derive (Clone, Copy)]`, which a
union member requires.

### Use-site declarations (no mirror was asked to grow)

Eight names this unit's live text needs are declared at their use site with C's own prototype on
the `pub fn` line (the minibuf.rs/ccl.rs shape).  They move to their owner's mirror when it
lands, and are reported here rather than added to a mirror that is not this unit's:

- `keyboard.h`'s `reorder_modifiers` (462), `parse_modifiers` (461),
  `parse_solitary_modifier` (463, `int`), `lucid_event_type_list_p` (497),
  `menu_item_eval_property` (516) and `message_with_string` (`lisp.h` 4397) — the
  `include/keyboard.rs` mirror is WIP and stops at line 520 of the header; its own note says so.
  **Four of these six citations were wrong** until 3.227, and one carried a wrong type:
  `parse_solitary_modifier` was declared `-> Lisp_Object` while C declares `extern int`
  (keyboard.h 463) and defines `int` (keyboard.c 7920) — the call site at `keymap.rs:2609`
  papered over it with `as isize as c_int`, which is exactly the "a wrong name/type here is
  invisible until it links" class `tools/check-undef.py` exists for.  Fixed in place with the
  file's line count unchanged (3,456 before and after), so no `citations-allowlist.tsv` key
  moved; `build-tu src/keymap.c` -> built.
- `lisp.h`'s `nconc2` (4271) and `esprintf` (4717) — two lisp.h declarations the `lisp.rs`
  mirror does not carry yet (the ccl.rs/callproc.rs shape).
- `<string.h>`'s `strstr` — the `string.rs` mirror carries memcpy/memset/strchr/strlen but not
  strstr.

`<stdckdint.h>`'s `ckd_mul` is **not** declared: the three uses are written as the
`overflowing_mul` pair at the use site (the profiler.rs/casefiddle.rs shape, PORTING.md P-15,
the `ckd_mul` family in `docs/ports/token-aliases.tsv`), and `SIZE_MAX`/`PTRDIFF_MAX` come from
`lib/include/stdint.in.rs` through `include/lisp.rs`.

### One mirror edit, and why it was unavoidable

`femacs/src/include/keymap.rs` was complete (its ledger has 0 unchecked rows) and needed
exactly one change: its `unsafe extern "C"` block declared the **nine functions and one object
that this very unit defines** (`current_global_map`, `push_key_description`, `access_keymap`,
`get_keymap`, `current_minor_maps`, `initial_define_lispy_key`, `syms_of_keymap`,
`map_keymap`, `map_keymap_canonical`), and a same-named declaration at the crate root is a hard
`E0428` against the definition — `unsafe extern` declarations are *not* shadowed the way a glob
import is.  The declarations moved into `pub mod keymap_decls { use super::*; … } use
keymap_decls::*;`, which is PORTING.md 0.1-10/P-21's established shape for a header whose
prototypes meet their definitions (the `buffer_point_decls`/`charset_decls`/`intervals_decls`
precedent, status 3.53/3.56/3.57).  No declaration was added, removed, renamed or re-signed, and
every other consumer is unchanged because the glob import is what it was already relying on.
All eleven other units that include the mirror were rebuilt after the change:
`casefiddle`, `category`, `cmds`, `data`, `doc`, `emacs`, `intervals`, `macros`, `menu`,
`minibuf`, `undo` — `built <unit>.rs -> ../build/femacs/<unit>.o` for each.  The mirror's own
ledger (`docs/ports/include-keymap.md`) does not yet mention the move; it is left to the
integrator, who owns cross-file bookkeeping.

### `DEFVAR_*` storages (status 3.29)

This unit's `syms_of_keymap` registers six `DEFVAR_LISP` variables, so the port **defines** all
six at the crate root with `#[unsafe(no_mangle)] pub static mut …: Lisp_Object =
lisp_h_Qnil ();` — `Vminibuffer_local_map`, `Vminor_mode_map_alist`,
`Vminor_mode_overriding_map_alist`, `Vemulation_mode_map_alists`,
`Vwhere_is_preferred_modifier`, `Vdescribe_bindings_check_shadowing_in_ranges`.  The
`include/globals.rs` `variables` module only declares them; a definition in the defining unit
shadows that glob import (PORTING.md 0.1-10/P-21), which is why they do not `E0428`.
`check-globals.py` is the gate for exactly this (`0 DEFVAR_* variables without a definition in
their own unit`).  `Vmouse_events` and `fontify_key_properties` are C `static`s, so they stay
unexported `static mut`s; `current_global_map` is C's one non-`static` object and carries
`#[unsafe(no_mangle)]`.

### Spelling deviations, one by one

1. **C `bool` literals.**  Every `0`/`1` argument of a `bool` parameter is `false`/`true`,
   which is what the token notes for those bodies record; the nineteen functions whose C bodies therefore no longer *mention* a `0`/`1`
   are recorded in `docs/ports/macro-body-notes.tsv` (the Fread_from_minibuffer precedent),
   together with `goto` ×2, the `while (1)` of `get_keyelt`, `KVAR` ×2 and `doc` ×1.
2. **`goto` → labelled loop/block.**  C's five labels are all kept *by name*, so the token
   gate still sees them: `get_keymap`'s `autoload_retry:` (the loop, whose one
   `continue 'autoload_retry` is C's backward `goto`) and `end:` (a labelled block whose
   `break 'end` is C's forward `goto end`); `store_in_keymap`'s
   `keymap_end:` (a labelled block around the scan loop).  `ccl.rs`/`bytecode.rs`'s shape.
3. **`KVAR`.**  keyboard.h 38's token-pasting macro has no Rust counterpart; the port writes
   the `kboard` field directly (`(*current_kboard).Voverriding_terminal_local_map_` etc., four
   in `Fdescribe_buffer_bindings`), which is the rule `include/keyboard.rs` itself records and
   `docs/ports/excluded-macros.txt` already carries.
4. **`BVAR`.**  The port's `BVAR!` does not append the macro's trailing `_`, so the two fields
   this unit reaches are spelled with the mirror's own names: `keymap_` (three sites).
5. **Object-like `Q*` are functions** (P-17), so each use carries `()` — `Qnil ()`, `Qt ()`,
   `Qkeymap ()`, … — and `DEFSYM`/`DEFVAR_LISP` are `DEFSYM!`/`DEFVAR_LISP!`.
6. **`CHAR_META`-class masks.**  `meta_modifier`, `alt_modifier`, … are `c_int` constants in
   `termhooks.rs` while `XFIXNUM`/`XFIXNAT` are `EMACS_INT`, so the C promotion is written
   explicitly (`XFIXNAT (idx) & meta_modifier as EMACS_INT`); where the mask is *built* from a
   constant (`~ - meta_modifier`, `~(alt_modifier | …)`) the operation is kept in `c_int` and
   cast once, which is C's own arithmetic type.
7. **`XSETFASTINT`/`XSETINT`** are `macro_rules!` (not functions), so the two uses are
   `XSETFASTINT! (…)`/`XSETINT! (…)`; `XSETCAR`/`XSETCDR` are `fn`s and keep C's call shape.
8. **`min (PTRDIFF_MAX, SIZE_MAX)`** in `current_minor_maps` is evaluated in the common
   unsigned type, so the port writes `(PTRDIFF_MAX as usize).min (SIZE_MAX)`, and the
   surrounding `sizeof *newmodes` arithmetic is `std::mem::size_of::<*mut Lisp_Object> ()`
   (the doprnt.rs shape).
9. **Strings.**  Rust has no `` escape, so C's `"
…"` is `c"
…"`; the value is the
   same and so is what the token gate sees, because both spellings are a complete (and
   therefore dropped) literal on their side.  C's **multi-line** `alternate_heading` literal is
   written with the same `\`+newline continuation shape it has in C (the continuation lines
   start at column 0, so the spliced value is byte-identical) — that is what PORTING.md
   status 3.28 prescribes, and it is the reason the token gate agrees here at all (with the
   literal on one line the whole leaked region of C's text was missing on the Rust side).
   C's `len + 3`-sized `SAFE_ALLOCA` buffer keeps C's `sizeof fmt` as `count_bytes () + 1` for
   the `fmt` string literal, and `strlen (" Minor Mode Bindings")` is
   `strlen (c" Minor Mode Bindings".as_ptr ())`.
10. **C's `char new_mods[sizeof ("\A-\C-\H-\M-\S-\s-")]`** is
    `[c_char; 19]` (`sizeof` of that literal), and the `*p++ = c` chains are
    `{ *p = … ; p = p.add (1); }` (PORTING.md 2.2, the character.rs shape) with
    `p - new_mods` as `p.offset_from (new_mods.as_ptr ())`.
11. **C locals C declares uninitialised** are zero-initialised (PORTING.md 1.5) and named in
    the port at the declaration: `Lisp_Object *maps`/`*modes` (out parameters, `null_mut ()`,
    three functions), `struct accessible_keymaps_data data` and `struct
    where_is_internal_data data` (`std::mem::zeroed ()`), `int range_beg` (`0`), the
    `char buf[KEY_DESCRIPTION_SIZE]`/`char tem[KEY_DESCRIPTION_SIZE]`/`char desc[4]`/
    `char str[MAX_MULTIBYTE_LENGTH]` buffers (`[0; …]`), and the deferred
    `let mut tail: Lisp_Object;`/`let elt: Lisp_Object;` forms where C assigns in every path.
12. **C's forward declarations** of this unit's own `static` helpers (`copy_keymap_1`,
    `describe_vector`, `where_is_internal_1`, `store_in_keymap`, `define_as_prefix`,
    `silly_event_symbol_error`, `get_keyelt`, C 75-84 and C 2417) are dropped (PORTING.md
    0.1-12), and named here.
13. **`static char const fmt[] = "  (currently shadowed by `%s')";`** in `describe_vector` is a
    function-local static in C; the port writes it as a `c"…"` value at the use site (its only
    reader is the `esprintf` call next to it) and takes `sizeof fmt` as `count_bytes () + 1`.
14. **`register`** (`store_in_keymap`'s `register Lisp_Object idx`,
    `push_text_char_description`'s two parameters, `where_is_internal`'s three) is C's storage
    hint with no Rust counterpart and is not written (PORTING.md 1.1).
15. **`describe_vector`'s `void (*elt_describer) (Lisp_Object, Lisp_Object)`** is
    `Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>` (the port's NULL-able C function
    pointer), so the two definitions it takes the address of — `describe_vector_princ` and
    `describe_vector_basic` — are `unsafe extern "C" fn`, and the two call sites are
    `elt_describer.unwrap () (…)`.

### Call-relationship differences (D7)

None found.  Every call in the C text appears in the port with the same callee and the same
argument order, and the port adds no call the reference does not make.  The differences are the
spellings above: a `macro_rules!` that expands at the same point C's macro does (`KEYMAPP!`,
`BVAR!`, `XSETFASTINT!`, `LIST!`, `CALLN!`, `SAFE_ALLOCA!`, `DEFUN!`, `DEFSYM!`, `DEFVAR_LISP!`,
`AUTO_STRING!`, `AUTO_STRING_WITH_LEN!`, `AUTO_LIST1!`, `USE_SAFE_ALLOCA!`, `SAFE_FREE!`,
`eassert!`), the `*_decls`-style use-site declarations (no call is renamed), the three
`char_table_ref_and_range`/`map_char_table`/`map_keymap` callbacks passed as `Some (…)`, and
`(*fun).unwrap () (…)` for C's `(*fun) (…)` on the function-pointer parameters.  `--check`'s
own body-token rule is the mechanical version of this claim: it runs `body_token_seq` over all
66 C bodies (37 functions + 29 subrs) and their port counterparts, and after the nineteen rows
above have exempted the `0`/`1`/`goto`/`kvar`/`doc` tokens, every one of the 66 missing-token
sets is empty.
