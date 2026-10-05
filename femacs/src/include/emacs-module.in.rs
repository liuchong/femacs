// femacs: port of src/emacs-module.in.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  The source is a *template*: `configure` substitutes
// `@emacs_major_version@` and the eight `@module_env_snippet_NN@` placeholders to produce the
// header modules are compiled against.  **The judge is the generated header**, exactly as
// `lib/dirent.in.h` and `lib/errno.in.h` are judged by `build/emacs-ref/lib/*.h` (status 3.159):
//
//     $ ls -l build/emacs-ref/src/emacs-module.h        # 49,460 bytes, 1,494 lines
//     $ grep -n 'EMACS_MAJOR_VERSION\|typedef struct emacs_env_\|^struct emacs_env_' \
//           build/emacs-ref/src/emacs-module.h | head
//     38:#define EMACS_MAJOR_VERSION 32
//     72:typedef struct emacs_env_32 emacs_env;
//     141:struct emacs_env_25
//     1291:struct emacs_env_32
//
// `tools/check-cond-eval.py src/emacs-module.in.h` reports `9 conditions, 3 unknown,
// 5 disagreements [NOT VERIFIED]`: the template's conditions ask about `__STDC_VERSION__`,
// `__cplusplus`, `__GNUC__`, `__GNUC_MINOR__` and `__has_attribute`, none of which the frozen
// macro table decides the way the template's text assumes.  The file stays NOT VERIFIED, so
// `tools/extract-abi.py` keeps both arms of every condition live (P-66's conservative reading);
// the *port* follows the generated header, which is what the frozen clang produced.
//
// **The eight `module-env-NN.h` fragments are substituted into this file, and their members
// cannot live in their own mirrors.**  In C the template's `struct emacs_env_NN` bodies are the
// snippets pasted in order (`struct emacs_env_32` is snippets 25-32, C 218-235), and the judge has
// them inlined (the `module-env-32.h` comment sits at generated line 1473).  In Rust a mirror can
// only be pasted where a macro or an `include!` may appear, and a struct's member list is not
// such a place - measured with the pinned toolchain:
//
//     $ printf 'pub a: u8,\n' > /tmp/frag.rs
//     $ rustc --edition 2024 --crate-type lib - <<'EOF'
//     #[repr(C)] pub struct S { include!("/tmp/frag.rs") }
//     EOF
//     error: unexpected token: `!`
//       = note: macros cannot expand to struct fields
//
// So each fragment has a mirror of its own that *quotes C's text verbatim* and records the
// substitution (`include/module-env-25.rs` .. `include/module-env-32.rs`; the three 0-byte ones
// are disposition records), while the member *spellings* live here, in the eight `#[repr(C)]`
// records whose order and names are the judge's.  That is the one place where this mirror is not
// a copy of one C file's text, and it is why the two halves point at each other.
//
// **The attribute family has no Rust item.**  `EMACS_NOEXCEPT` and `EMACS_NOEXCEPT_TYPEDEF`
// expand to nothing on this target and `EMACS_ATTRIBUTE_NONNULL` to an attribute, as the frozen
// clang reports:
//
//     $ clang -std=gnu23 -include config.h -I build/emacs-ref/src -I build/emacs-ref/lib \
//         -I emacs/src -I emacs/lib -E -dM /tmp/m.c | grep -E 'EMACS_(NOEXCEPT|ATTRIBUTE_NONNULL)'
//     #define EMACS_ATTRIBUTE_NONNULL(...) __attribute__ ((__nonnull__ (__VA_ARGS__)))
//     #define EMACS_NOEXCEPT
//     #define EMACS_NOEXCEPT_TYPEDEF
//
// A `macro_rules!` cannot produce an attribute (the `_Alignas` row of
// docs/ports/excluded-macros.txt states the rule), so the three names are left to
// docs/ports/excluded-macros.txt (kind=syntax) and the declarations below carry the
// attribute-free shapes the port needs - see docs/ports/include-emacs-module.in.md.
//
// **Degradations (PORTING.md P-55/P-72 rule 3), each with the C text in a `/// C:` line.**
// `struct emacs_value_tag` is defined by `src/emacs-module.c` 125 - the *unit*, whose port is the
// empty shape this configuration gives it (`HAVE_MODULES` is `undef`) - and
// `struct emacs_env_private`/`struct emacs_runtime_private` by that same file's 163/177, so those
// three pointers keep C's depth with an untyped interior; `emacs_value` is still an alias with
// C's name, which is what every member below spells (P-26: one spelling per type).  The
// generated header's `SIZE_MAX` for `EMACS_LIMB_MAX` comes from `<stdint.h>` (C 30), whose port
// mirror is included below; `size_t`/`ptrdiff_t`/`intmax_t` are `include/lisp.rs` 50-53's
// aliases, which every unit and the layout probe carry (PORTING.md 0.1-9 - C reaches them
// through `<stddef.h>` at C 29, which has no port mirror yet).
//
// **Layout is asserted from the judge's own compiler**: the numbers in the `const _` assertions
// below are `clang -std=gnu23 -include config.h -I build/emacs-ref/src … -Xclang
// -fdump-record-layouts-complete` readings of `build/emacs-ref/src/emacs-module.h`, i.e. of the
// header modules are actually compiled against.  The ledger records the dump (295 member offsets
// across the nine records) and the Rust-side comparison of every one of them.

// C 26-27: the include guard has no Rust counterpart (PORTING.md 0.1-4); the generated header
// spells the same name.
pub const EMACS_MODULE_H: bool = true;

include!("../../lib/include/stdint.in.rs");   // C 30: `#include <stdint.h>` (`SIZE_MAX`)
include!("../../lib/include/time.in.rs");     // C 31: `#include <time.h>` (`struct timespec`, C 31)

/// C 38: `#define EMACS_MAJOR_VERSION @emacs_major_version@` - the template's placeholder, whose
/// frozen substitution the generated header spells (38: `#define EMACS_MAJOR_VERSION 32`), so the
/// port carries the substituted value.  `32` is the name every `emacs_env_NN` field of the judge
/// uses for the current environment.
/// C: #define EMACS_MAJOR_VERSION @emacs_major_version@
pub const EMACS_MAJOR_VERSION: c_int = 32;

/// C 67-69: `#ifdef __cplusplus` / `extern "C" {` - C's linkage guard; a Rust mirror has no
/// `extern "C"` block of its own (every entry point below is declared with C's linkage).  The
/// `__cplusplus` arms are part of what makes this file blind to the frozen macro table.

/// C 71-72: `typedef struct emacs_env_@emacs_major_version@ emacs_env;` - the substituted name is
/// `emacs_env_32` (judge 72), which is the record C 218-235 defines below.
pub type emacs_env = emacs_env_32;

/// C 74-76: `typedef struct emacs_value_tag *emacs_value;` - the tag is defined by
/// `src/emacs-module.c` 125, not by this header, so the pointee keeps C's pointer depth with an
/// untyped interior and the *alias* keeps C's name (PORTING.md P-26; the `include/font.rs`
/// `font_driver` reading).
/// C: typedef struct emacs_value_tag *emacs_value;
pub type emacs_value = *mut c_void;

/// C 78: `enum { emacs_variadic_function = -2 };` - the sentinel for a variadic module function.
pub const emacs_variadic_function: c_int = -2;

/// C 80-92: `struct emacs_runtime` - what `emacs_module_init` receives.  `struct
/// emacs_runtime_private` is `src/emacs-module.c` 177's record (see the file note), and
/// `get_environment` is C's nullable function pointer (PORTING.md P-27).
/// C: struct emacs_runtime
/// C: {
/// C:   ptrdiff_t size;
/// C:   struct emacs_runtime_private *private_members;
/// C:   emacs_env *(*get_environment) (struct emacs_runtime *runtime)
/// C:     EMACS_ATTRIBUTE_NONNULL (1);
/// C: };
#[repr(C)]
pub struct emacs_runtime
{
  pub size: ptrdiff_t,

  /// C: `struct emacs_runtime_private *private_members;`
  pub private_members: *mut c_void,

  pub get_environment: Option<unsafe extern "C" fn (runtime: *mut emacs_runtime) -> *mut emacs_env>,
}

/// C 99-104: `typedef emacs_value (*emacs_function) (emacs_env *env, ptrdiff_t nargs,
/// emacs_value *args, void *data)` - the port's spelling of C's nullable function pointer
/// (PORTING.md P-27); the generated header's `EMACS_ATTRIBUTE_NONNULL (1)` and
/// `EMACS_NOEXCEPT_TYPEDEF` are attributes with no Rust counterpart (file note).
/// C: typedef emacs_value (*emacs_function) (emacs_env *env, ptrdiff_t nargs,
/// C:                                        emacs_value *args,
/// C:                                        void *data)
/// C:   EMACS_NOEXCEPT_TYPEDEF EMACS_ATTRIBUTE_NONNULL (1);
pub type emacs_function
  = Option<unsafe extern "C" fn (env: *mut emacs_env, nargs: ptrdiff_t, args: *mut emacs_value,
                                 data: *mut c_void) -> emacs_value>;

/// C 106-108: `typedef void (*emacs_finalizer) (void *data) EMACS_NOEXCEPT_TYPEDEF;`
/// C: typedef void (*emacs_finalizer) (void *data) EMACS_NOEXCEPT_TYPEDEF;
pub type emacs_finalizer = Option<unsafe extern "C" fn (data: *mut c_void)>;

/// C 110-121: `enum emacs_funcall_exit` - the port spells an anonymous-typed enum as its type
/// alias plus one constant per member (PORTING.md 1.3; the `include/termhooks.rs` `event_kind`
/// shape), and the constant gate compares the values the judge gives (0, 1, 2).
/// C: enum emacs_funcall_exit
/// C: {
/// C:   emacs_funcall_exit_return = 0,
/// C:   emacs_funcall_exit_signal = 1,
/// C:   emacs_funcall_exit_throw = 2
/// C: };
pub type emacs_funcall_exit = c_int;
pub const emacs_funcall_exit_return: emacs_funcall_exit = 0;
pub const emacs_funcall_exit_signal: emacs_funcall_exit = 1;
pub const emacs_funcall_exit_throw: emacs_funcall_exit = 2;

/// C 123-131: `enum emacs_process_input_result` - the same shape (0 and 1 in the judge).
/// C: enum emacs_process_input_result
/// C: {
/// C:   emacs_process_input_continue = 0,
/// C:   emacs_process_input_quit = 1
/// C: };
pub type emacs_process_input_result = c_int;
pub const emacs_process_input_continue: emacs_process_input_result = 0;
pub const emacs_process_input_quit: emacs_process_input_result = 1;

/// C 133-139: `typedef size_t emacs_limb_t;` (the limb type the bignum members use) and the
/// `SIZE_MAX` bound - `size_t` is `include/lisp.rs` 51's alias and `SIZE_MAX` comes from the
/// `<stdint.h>` mirror included above, which is exactly the name C 139's body spells.
/// C: typedef size_t emacs_limb_t;
/// C: #define EMACS_LIMB_MAX SIZE_MAX
pub type emacs_limb_t = size_t;
pub const EMACS_LIMB_MAX: usize = SIZE_MAX;

/// C 141-235: `struct emacs_env_25` - the version-25 module environment: snippet 25's members
/// followed by snippets 26..25, which is what the judge inlines (its line 141).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-25.rs`.
#[repr(C)]
pub struct emacs_env_25
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
}
/// C 141-235: `struct emacs_env_26` - the version-26 module environment: snippet 25's members
/// followed by snippets 26..26, which is what the judge inlines (its line 276).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-26.rs`.
#[repr(C)]
pub struct emacs_env_26
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
}
/// C 141-235: `struct emacs_env_27` - the version-27 module environment: snippet 25's members
/// followed by snippets 26..27, which is what the judge inlines (its line 415).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-27.rs`.
#[repr(C)]
pub struct emacs_env_27
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
  pub process_input: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_process_input_result>,
  pub extract_time: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> timespec>,
  pub make_time: Option<unsafe extern "C" fn (*mut emacs_env, timespec) -> emacs_value>,
  pub extract_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_int, *mut ptrdiff_t, *mut emacs_limb_t) -> bool>,
  pub make_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, c_int, ptrdiff_t, *const emacs_limb_t) -> emacs_value>,
}
/// C 141-235: `struct emacs_env_28` - the version-28 module environment: snippet 25's members
/// followed by snippets 26..28, which is what the judge inlines (its line 573).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-28.rs`.
#[repr(C)]
pub struct emacs_env_28
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
  pub process_input: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_process_input_result>,
  pub extract_time: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> timespec>,
  pub make_time: Option<unsafe extern "C" fn (*mut emacs_env, timespec) -> emacs_value>,
  pub extract_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_int, *mut ptrdiff_t, *mut emacs_limb_t) -> bool>,
  pub make_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, c_int, ptrdiff_t, *const emacs_limb_t) -> emacs_value>,
  pub get_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub open_channel: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_int>,
  pub make_interactive: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_unibyte_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
}
/// C 141-235: `struct emacs_env_29` - the version-29 module environment: snippet 25's members
/// followed by snippets 26..29, which is what the judge inlines (its line 751).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-29.rs`.
#[repr(C)]
pub struct emacs_env_29
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
  pub process_input: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_process_input_result>,
  pub extract_time: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> timespec>,
  pub make_time: Option<unsafe extern "C" fn (*mut emacs_env, timespec) -> emacs_value>,
  pub extract_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_int, *mut ptrdiff_t, *mut emacs_limb_t) -> bool>,
  pub make_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, c_int, ptrdiff_t, *const emacs_limb_t) -> emacs_value>,
  pub get_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub open_channel: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_int>,
  pub make_interactive: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_unibyte_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
}
/// C 141-235: `struct emacs_env_30` - the version-30 module environment: snippet 25's members
/// followed by snippets 26..30, which is what the judge inlines (its line 930).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-30.rs`.
#[repr(C)]
pub struct emacs_env_30
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
  pub process_input: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_process_input_result>,
  pub extract_time: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> timespec>,
  pub make_time: Option<unsafe extern "C" fn (*mut emacs_env, timespec) -> emacs_value>,
  pub extract_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_int, *mut ptrdiff_t, *mut emacs_limb_t) -> bool>,
  pub make_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, c_int, ptrdiff_t, *const emacs_limb_t) -> emacs_value>,
  pub get_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub open_channel: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_int>,
  pub make_interactive: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_unibyte_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
}
/// C 141-235: `struct emacs_env_31` - the version-31 module environment: snippet 25's members
/// followed by snippets 26..31, which is what the judge inlines (its line 1110).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-31.rs`.
#[repr(C)]
pub struct emacs_env_31
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
  pub process_input: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_process_input_result>,
  pub extract_time: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> timespec>,
  pub make_time: Option<unsafe extern "C" fn (*mut emacs_env, timespec) -> emacs_value>,
  pub extract_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_int, *mut ptrdiff_t, *mut emacs_limb_t) -> bool>,
  pub make_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, c_int, ptrdiff_t, *const emacs_limb_t) -> emacs_value>,
  pub get_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub open_channel: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_int>,
  pub make_interactive: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_unibyte_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
}
/// C 141-235: `struct emacs_env_32` - the version-32 module environment: snippet 25's members
/// followed by snippets 26..32, which is what the judge inlines (its line 1291).  C's member order
/// is the module ABI, so the fields below keep it; every one is a pointer-sized slot, and
/// `Option<unsafe extern "C" fn …>` is this port's reading of C's nullable function pointer
/// (PORTING.md P-27).  The C text of every member is quoted verbatim in
/// `include/module-env-25.rs`..`include/module-env-32.rs`.
#[repr(C)]
pub struct emacs_env_32
{
  pub size: ptrdiff_t,
  pub private_members: *mut c_void,
  pub make_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub free_global_ref: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value)>,
  pub non_local_exit_check: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_funcall_exit>,
  pub non_local_exit_clear: Option<unsafe extern "C" fn (*mut emacs_env)>,
  pub non_local_exit_get: Option<unsafe extern "C" fn (*mut emacs_env, *mut emacs_value, *mut emacs_value) -> emacs_funcall_exit>,
  pub non_local_exit_signal: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub non_local_exit_throw: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_function: Option<unsafe extern "C" fn (*mut emacs_env, ptrdiff_t, ptrdiff_t, emacs_function, *const c_char, *mut c_void) -> emacs_value>,
  pub funcall: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, *mut emacs_value) -> emacs_value>,
  pub intern: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char) -> emacs_value>,
  pub type_of: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_value>,
  pub is_not_nil: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> bool>,
  pub eq: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value) -> bool>,
  pub extract_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> intmax_t>,
  pub make_integer: Option<unsafe extern "C" fn (*mut emacs_env, intmax_t) -> emacs_value>,
  pub extract_float: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_double>,
  pub make_float: Option<unsafe extern "C" fn (*mut emacs_env, c_double) -> emacs_value>,
  pub copy_string_contents: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_char, *mut ptrdiff_t) -> bool>,
  pub make_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub make_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_finalizer, *mut c_void) -> emacs_value>,
  pub get_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut c_void>,
  pub set_user_ptr: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_void)>,
  pub get_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_user_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub vec_get: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t) -> emacs_value>,
  pub vec_set: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, ptrdiff_t, emacs_value)>,
  pub vec_size: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> ptrdiff_t>,
  pub should_quit: Option<unsafe extern "C" fn (*mut emacs_env) -> bool>,
  pub process_input: Option<unsafe extern "C" fn (*mut emacs_env) -> emacs_process_input_result>,
  pub extract_time: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> timespec>,
  pub make_time: Option<unsafe extern "C" fn (*mut emacs_env, timespec) -> emacs_value>,
  pub extract_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, *mut c_int, *mut ptrdiff_t, *mut emacs_limb_t) -> bool>,
  pub make_big_integer: Option<unsafe extern "C" fn (*mut emacs_env, c_int, ptrdiff_t, *const emacs_limb_t) -> emacs_value>,
  pub get_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> emacs_finalizer>,
  pub set_function_finalizer: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_finalizer)>,
  pub open_channel: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> c_int>,
  pub make_interactive: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value, emacs_value)>,
  pub make_unibyte_string: Option<unsafe extern "C" fn (*mut emacs_env, *const c_char, ptrdiff_t) -> emacs_value>,
  pub canvas_data: Option<unsafe extern "C" fn (*mut emacs_env, emacs_value) -> *mut u32>,
}

/* The judge's own compiler measured the nine records above (and the offsets below) from
   build/emacs-ref/src/emacs-module.h; each assertion is one reading, so a member whose type
   changes width, or a member that moves, fails the layout probe rather than quietly keeping
   the ABI claim.  */
const _: () = core::assert! (std::mem::size_of::<emacs_runtime> () == 24);
const _: () = core::assert! (std::mem::align_of::<emacs_runtime> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_25> () == 232);
const _: () = core::assert! (std::mem::align_of::<emacs_env_25> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_26> () == 240);
const _: () = core::assert! (std::mem::align_of::<emacs_env_26> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_27> () == 280);
const _: () = core::assert! (std::mem::align_of::<emacs_env_27> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_28> () == 320);
const _: () = core::assert! (std::mem::align_of::<emacs_env_28> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_29> () == 320);
const _: () = core::assert! (std::mem::align_of::<emacs_env_29> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_30> () == 320);
const _: () = core::assert! (std::mem::align_of::<emacs_env_30> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_31> () == 320);
const _: () = core::assert! (std::mem::align_of::<emacs_env_31> () == 8);
const _: () = core::assert! (std::mem::size_of::<emacs_env_32> () == 328);
const _: () = core::assert! (std::mem::align_of::<emacs_env_32> () == 8);
const _: () = core::assert! (std::mem::offset_of! (emacs_runtime, size) == 0);
const _: () = core::assert! (std::mem::offset_of! (emacs_runtime, private_members) == 8);
const _: () = core::assert! (std::mem::offset_of! (emacs_runtime, get_environment) == 16);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, size) == 0);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, private_members) == 8);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_global_ref) == 16);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, free_global_ref) == 24);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, non_local_exit_check) == 32);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, non_local_exit_clear) == 40);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, non_local_exit_get) == 48);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, non_local_exit_signal) == 56);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, non_local_exit_throw) == 64);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_function) == 72);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, funcall) == 80);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, intern) == 88);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, type_of) == 96);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, is_not_nil) == 104);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, eq) == 112);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, extract_integer) == 120);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_integer) == 128);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, extract_float) == 136);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_float) == 144);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, copy_string_contents) == 152);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_string) == 160);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_user_ptr) == 168);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, get_user_ptr) == 176);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, set_user_ptr) == 184);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, get_user_finalizer) == 192);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, set_user_finalizer) == 200);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, vec_get) == 208);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, vec_set) == 216);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, vec_size) == 224);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, should_quit) == 232);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, process_input) == 240);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, extract_time) == 248);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_time) == 256);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, extract_big_integer) == 264);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_big_integer) == 272);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, get_function_finalizer) == 280);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, set_function_finalizer) == 288);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, open_channel) == 296);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_interactive) == 304);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, make_unibyte_string) == 312);
const _: () = core::assert! (std::mem::offset_of! (emacs_env_32, canvas_data) == 320);

/// C 237-240: `extern int emacs_module_init (struct emacs_runtime *runtime) EMACS_NOEXCEPT
/// EMACS_ATTRIBUTE_NONNULL (1);` - the one function every module defines (the port declares it
/// with C's linkage and without the two attributes; a Rust `extern "C" fn` has no exception
/// specification, and nullness is not a type).
/// C: extern int emacs_module_init (struct emacs_runtime *runtime)
/// C:   EMACS_NOEXCEPT
/// C:   EMACS_ATTRIBUTE_NONNULL (1);
unsafe extern "C" {
  pub fn emacs_module_init (runtime: *mut emacs_runtime) -> c_int;
}
