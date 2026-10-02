# emacs/src/undo.c (490 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] undo-boundary  (Fundo_boundary, 0/0, doc 709ec77bf065, line 251)

## Functions defined — 10
- [x] prepare_record  (line 36, static)
- [x] record_point  (line 48, static)
- [x] record_insert  (line 87)
- [x] record_marker_adjustments  (line 127, static)
- [x] record_delete  (line 163)
- [x] record_change  (line 199)
- [x] record_first_change  (line 210)
- [x] record_property_change  (line 229)
- [x] truncate_undo_list  (line 290)
- [x] syms_of_undo  (line 423)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 1
- [x] static Lisp_Object pending_boundary  (line 32, static)

## Decisions

- **`BVAR` is written as the field it pastes.**  buffer.h's `BVAR (buf, field)` ends in
  `field ## _`, so the port passes the underscored field name (`BVAR! (current_buffer,
  undo_list_)`); the token check canonicalises the trailing `_` away, which is why the C
  token `undo_list` still has to be spelled (the `KVAR` precedent in macros.rs).
- **`MODIFF`, `SAVE_MODIFF`, `PT`, `Z` and `BUF_MARKERS` are the mirror's macros**, so they
  are spelled `MODIFF! ()`, `SAVE_MODIFF! ()`, `PT! ()`, `Z! ()`, `BUF_MARKERS (...)`: C's
  `MODIFF`/`PT`/`Z` are object-like macros over `current_buffer`, and the port keeps that
  shape (buffer.rs 78-160).
- **`m->insertion_type` has no Rust field.**  lisp.h 313-314 makes it a 1-bit `bool_bf`
  next to `need_adjustment`; the mirror packs both into `Lisp_Marker::bits` (LSB first) and
  reads the second through `MARKER_INSERTION_TYPE_P`.  The read is kept honest by the new
  `insertion_type` family in `docs/ports/token-aliases.tsv`, whose occurrence check fails if
  the port stops mentioning it (the `local_if_set` precedent, P-78).
- **`sizeof (struct Lisp_Cons)`/`Lisp_String` are `size_of::<Lisp_Cons> ()` etc.**, and the
  string term is cast the way C's size_t arithmetic does it (`- 1 + SCHARS (…)`).
- **`make_lisp_ptr`'s first parameter is the mirror's `*mut c_void`**, so the marker pointer
  is cast, as in the port's other `make_lisp_ptr` sites.
- **The three cross-unit calls it needs** (`inhibit_garbage_collection` lisp.h 4654,
  `make_buffer_string` 4926, `buffer_visited_file_modtime` 4989) were added to lisp.rs; the
  last takes `*mut c_void` with a `/// C:` prototype because `struct buffer` is incomplete in
  lisp.h (P-55/P-72).
