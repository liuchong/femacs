# emacs/lib/scratch_buffer.h (117 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] _GL_LIKELY  (line 111, body 3dd405fb64c2)
- [x] _GL_SCRATCH_BUFFER_H  (line 20, body da39a3ee5e6b)
- [x] _GL_UNLIKELY  (line 112, body f06ed8d0852f)
- [x] __libc_scratch_buffer_grow  (line 105, body d638d4916363)
- [x] __libc_scratch_buffer_grow_preserve  (line 106, body e1f4dc54f5cf)
- [x] __libc_scratch_buffer_set_array_size  (line 107, body 6f7cafe7426c)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/scratch_buffer.h` @ `fd7afced403`; port `femacs/lib/include/scratch_buffer.rs`.  Every checklist item is
ticked and `extract-abi.py --check` reports `OK` for the pair.  The frozen target's arm is
the one implemented; PORTING.md P-24 does not compare a macro that has several definitions,
so the non-frozen arm's spellings are recorded here instead of being carried as text.

- The C 62-102 `#if 0` block's declarations are dead text in C, so the port takes the struct
  and the inline bodies from the generated `<malloc/scratch_buffer.gl.h>` that the header
  includes, as the frozen build does (its `build/emacs-ref/lib/malloc/scratch_buffer.gl.h`).
- `_GL_LIKELY`/`_GL_UNLIKELY` keep C's `__builtin_expect` hint through a port-local
  `__builtin_expect` macro (PORTING.md 2.6).  The three `__libc_scratch_buffer_*` macros name
  the `_gl_scratch_buffer_*` symbols this build's objects reference (measured with `nm` on
  `build/emacs-ref/lib/canonicalize-lgpl.o`).
