# emacs/lib/malloc/scratch_buffer.h (135 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] scratch_buffer_init  (line 75, static)
- [x] scratch_buffer_free  (line 83, static)
- [x] scratch_buffer_grow  (line 99, static)
- [x] scratch_buffer_grow_preserve  (line 111, static)
- [x] scratch_buffer_set_array_size  (line 128, static)

## Macros defined — 1
- [x] _SCRATCH_BUFFER_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] scratch_buffer: void* data, size_t length

## File-scope objects — 0

## Decisions

- **This is the glibc source of the generated header, not the header a unit includes.**
  `lib/malloc/scratch_buffer.h` is what gnulib copies into
  `build/emacs-ref/lib/malloc/scratch_buffer.gl.h`; the header the three units get their
  `struct scratch_buffer` and inline bodies from is the *wrapper*, `lib/scratch_buffer.h`
  (mirrored as `femacs/lib/include/scratch_buffer.rs`), which renames the three
  `__libc_scratch_buffer_*` entries to `_gl_scratch_buffer_*` at C 104-107 and then includes
  the gl.h.  The mirror therefore keeps the header's own, unrenamed names, and no unit
  includes it (there is nothing in C that does).
- **Structure.**  C 64-70's `struct scratch_buffer` and C 69's anonymous
  `union { max_align_t __align; char __c[1024]; }` are the same text the wrapper's gl.h block
  carries: a `#[repr(C)]` union (`MaxAlign` + `[c_char; 1024]`) inside a `#[repr(C)]` struct,
  with `SCRATCH_BUFFER_SPACE_SIZE = 1024` naming the array's extent.  `MaxAlign` stands for
  `<stddef.h>`'s `max_align_t`, measured on the frozen target as `long double` - `8` bytes,
  `8`-aligned - so the union's alignment is 8 and the struct's is 8.
- **Layout assertion.**  `scratch_buffer` is not one of `tools/gen-layout-fields.py`'s keys,
  so the mirror states the C compiler's own numbers in `const _: () = assert!(…)` form: size
  1040, alignment 8, `data` 0, `length` 8, `__space` 16 (measured with the frozen target's
  clang on the header's struct verbatim).
- **`free` is `rpl_free`.**  C 82/86 call `free`; the generated `<stdlib.h>` renames it
  (`build/emacs-ref/lib/stdlib.h` 883-884: `# undef free` / `# define free rpl_free`) and the
  frozen objects call that name (`nm build/emacs-ref/lib/malloc/scratch_buffer_grow.o` prints
  `U _rpl_free`), so the port spells `rpl_free` (PORTING.md 10.13, the `rpl_mempcpy`
  precedent).  The token family `free`/`rpl_free` in docs/ports/token-aliases.tsv is what
  lets the body checks accept it.
- **The three prototypes** C 94/106/122 declare are `unsafe extern "C"` declarations here
  (C 95/107/124's `libc_hidden_proto (…)` is empty in this build: libc-config.h's
  `# define libc_hidden_proto(name)`), and C 98/110/127's `_GL_ATTRIBUTE_ALWAYS_INLINE` is
  the gl.h's spelling of `__always_inline`, i.e. P-12/P-37's `#[inline(always)]`.
- **Call-relationship differences (D7)**: none.  The include guard `_SCRATCH_BUFFER_H` has no
  Rust counterpart (PORTING.md 0.1) and is a `const` so the macro rule sees it.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/malloc/scratch_buffer.h femacs/lib/malloc/include/scratch_buffer.rs
OK emacs/lib/malloc/scratch_buffer.h -> femacs/lib/malloc/include/scratch_buffer.rs

# the C compiler's layout, on the header's struct verbatim
$ cc -o /tmp/sb_probe /tmp/sb_probe.c && /tmp/sb_probe
size=1040 data=0 length=8 space=16 align=8 maxalign=8

# the mirror is not a compilation unit (it expects the including unit's scalar scope, as
# every lib mirror does - `build-tu lib/include/scratch_buffer.rs` fails the same way), so it
# is probe-compiled with the names a unit supplies:
$ rustc --edition 2024 --crate-type=lib --emit=obj --crate-name sb_mirror_probe -C panic=abort \
    -C codegen-units=1 -C opt-level=0 /tmp/sb-mirror/root.rs -o /tmp/sb-mirror-probe.o
(exit 0, no warnings; /tmp/sb-mirror/root.rs is `use std::ffi::{c_char, c_int, c_void};` +
 the unit allow-list + `include!` of this mirror; `nm -g` is empty - nothing is emitted)

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sb cargo run -q -p xtask -- build-tu lib/malloc/scratch_buffer.h
error: rustc failed for lib/malloc/include/scratch_buffer.rs (exit status: 1)

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sb cargo run -q -p xtask -- build-tu lib/scratch_buffer.h
error: rustc failed for lib/include/scratch_buffer.rs (exit status: 1)
```

The second pair is the same failure on the pre-existing, committed wrapper mirror: a mirror
has no scalar scope of its own, so `build-tu` on a `.h` cannot succeed for either file.
