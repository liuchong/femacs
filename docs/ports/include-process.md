# emacs/src/process.h (320 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] PROCESSP  (line 217)
- [x] CHECK_PROCESS  (line 223)
- [x] XPROCESS  (line 229)
- [x] pset_childp  (line 244)
- [x] pset_status  (line 250)

## Macros defined — 2
- [x] ChannelMask  (line 238, body fcfe9bc9855f)
- [x] EMACS_PROCESS_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] Lisp_Process: union vectorlike_header header, Lisp_Object tty_name, Lisp_Object name, Lisp_Object command, Lisp_Object filter, Lisp_Object sentinel, Lisp_Object log, Lisp_Object buffer, Lisp_Object childp, Lisp_Object plist, Lisp_Object type, Lisp_Object mark, Lisp_Object status, Lisp_Object decode_coding_system, Lisp_Object decoding_buf, Lisp_Object encode_coding_system, Lisp_Object encoding_buf, Lisp_Object write_queue, Lisp_Object stderrproc, Lisp_Object thread, pid_t pid, int infd, uintmax_t nbytes_read, int outfd, int open_fd, EMACS_INT tick, EMACS_INT update_tick, int decoding_carryover, int read_output_delay, unsigned int adaptive_read_buffering : 2, bool_bf read_output_skip : 1, ptrdiff_t readmax, bool_bf kill_without_query : 1, bool_bf pty_in : 1, bool_bf pty_out : 1, bool_bf inherit_coding_system_flag : 1, bool_bf alive : 1, bool_bf raw_status_new : 1, bool_bf is_non_blocking_client : 1, bool_bf is_server : 1, int raw_status, int backlog, int port, int socktype

## File-scope objects — 2
- [x] extern bool inhibit_sentinels  (line 265)
- [x] extern bool glib_installs_sigchld_handler  (line 267)

# ----------------------------------------------------------------------
## Decisions

- **Bitfield storage.**  `struct Lisp_Process` has two 4-byte bitfield units: `bits1` at
  offset 232 (`adaptive_read_buffering : 2` plus `read_output_skip : 1`, measured `07 00 00 00`
  with both set and `04 00 00 00` with only the skip flag) and `bits2` at 248 (eight
  `bool_bf : 1`, measured `ff 00 00 00`).  The port stores the units and names each member's
  shift (PORTING.md 1.3, the `glyph_slice` precedent); `sizeof` 272 / `alignof` 8 are asserted.
- **Dead platform members.**  `gnutls_*` and `dns_request` are inside `HAVE_GNUTLS` /
  `HAVE_GETADDRINFO_A`, both `#undef` in the frozen `config.h`, so neither the members nor the
  `<gnutls/…>` includes exist on this target.
- **`pid_t`.**  Missing from the port until now; added to `lib/include/sys_types.in.rs` with
  its measured `sizeof (pid_t) == 4`, because `Lisp_Process.pid` is its first user.
- **`enum { PROCESS_OPEN_FDS = 6 }`** is the array bound of `open_fd`, so the port declares the
  same constant rather than spelling `6`.
- **Not yet done: none for this header.**  `extract-abi.py --check emacs/src/process.h
  femacs/src/include/process.rs` reports `OK`; the `fd_callback` typedef and the prototypes
  after line 277 are declarations, which the checker does not inventory (PORTING.md P-64
  records that limitation), and they land with `process.c`/`sysdep.c`.

## Declarations (status 4.996)

C's prototypes for functions other units define now stand in `process_fn_decls` (module + glob
import, P-297/P-39): **13** rows, each with its C line and text, `pub` so the glob brings them out,
and carrying the gate of the records it names.

The use-site copies were withdrawn only where `tools/check-decl-withdraw.py` could prove both
things P-323 asks for: the unit includes this mirror itself and the mirror's declaration carries no
cfg, and the two signatures are identical after whitespace and `pub` are normalised. Copies that
fail either test stay, and are ratcheted in `docs/baseline/mirror-dupes.tsv` with the reason
(the four readings in the batch: 342 withdrawn, 57 duplicates, 0 failures).
