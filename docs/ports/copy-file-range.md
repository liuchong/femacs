# emacs/lib/copy-file-range.c (99 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] copy_file_range  (line 41)

## Macros defined — 1
- [x] CHECK_LINUX_KERNEL_VERSION  (line 34, body 7cb6efb98ba5)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **HAVE_COPY_FILE_RANGE is undefined** on this target, so the live body is C 95-97
  (`errno = ENOSYS; return -1;`) and the whole Linux arm is dead text kept under two cfg
  features (P-51).  C 56-82's kernel-version probe needs `struct utsname`/`uname`, which this
  target has not got, so those tokens are recorded in docs/ports/macro-body-notes.tsv (the
  gettimeofday.c Windows-arm class) instead of being written out; C 85-90's glibc-bug arm
  (`SYS_BUFSIZE_MAX`, from the new sys-limits mirror) *is* written out, since it is expressible.
- C 48's `#undef copy_file_range` is the port's `system` module (the getgroups.c shape).
- The port is 10% *shorter* than C for that reason; the tolerance row records it.
- No call-relationship differences (D7).
