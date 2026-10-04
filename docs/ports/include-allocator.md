# emacs/lib/allocator.h (68 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_ALLOCATOR_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 1
- [x] allocator: void * (*) allocate, void * (*) reallocate, void (*) free, void (*) die

## File-scope objects — 0

## Decisions

`emacs/lib/allocator.h` @ `fd7afced403`; port `femacs/lib/include/allocator.rs`.  `struct
allocator` is four function pointers; the port keeps `Option<unsafe extern "C" fn …>` for all
four because C tests `reallocate`/`die` for null, and the pointer spellings use absolute
`core::ffi::c_void` (the stdio.rs lesson: a mirror must not depend on the includer's imports for a
type it names).  `stdlib_allocator` is declared in a whole-import module because
`femacs/lib/allocator.rs` defines it.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/allocator.h femacs/lib/include/allocator.rs
OK emacs/lib/allocator.h -> femacs/lib/include/allocator.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- build-tu lib/allocator.c
built lib/allocator.rs -> ../build/femacs/allocator.o
```

`lib/allocator.c` is the consumer: it defines the `stdlib_allocator` object (22 lines, `done` in
the inventory), so a unit that includes this mirror and that unit together sees one definition
through the whole-import module (`allocator_decls`).
