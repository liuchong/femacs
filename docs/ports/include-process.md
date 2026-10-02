# emacs/src/process.h (320 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [ ] PROCESSP  (line 217)
- [ ] CHECK_PROCESS  (line 223)
- [ ] XPROCESS  (line 229)
- [ ] pset_childp  (line 244)
- [ ] pset_status  (line 250)

## Macros defined — 2
- [ ] ChannelMask  (line 238, body fcfe9bc9855f)
- [ ] EMACS_PROCESS_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [ ] Lisp_Process: union vectorlike_header header, Lisp_Object tty_name, Lisp_Object name, Lisp_Object command, Lisp_Object filter, Lisp_Object sentinel, Lisp_Object log, Lisp_Object buffer, Lisp_Object childp, Lisp_Object plist, Lisp_Object type, Lisp_Object mark, Lisp_Object status, Lisp_Object decode_coding_system, Lisp_Object decoding_buf, Lisp_Object encode_coding_system, Lisp_Object encoding_buf, Lisp_Object write_queue, Lisp_Object stderrproc, Lisp_Object thread, pid_t pid, int infd, uintmax_t nbytes_read, int outfd, int open_fd, EMACS_INT tick, EMACS_INT update_tick, int decoding_carryover, int read_output_delay, unsigned int adaptive_read_buffering : 2, bool_bf read_output_skip : 1, ptrdiff_t readmax, bool_bf kill_without_query : 1, bool_bf pty_in : 1, bool_bf pty_out : 1, bool_bf inherit_coding_system_flag : 1, bool_bf alive : 1, bool_bf raw_status_new : 1, bool_bf is_non_blocking_client : 1, bool_bf is_server : 1, int raw_status, int backlog, int port, int socktype

## File-scope objects — 2
- [ ] extern bool inhibit_sentinels  (line 265)
- [ ] extern bool glib_installs_sigchld_handler  (line 267)
