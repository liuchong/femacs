# emacs/lib/stdio-impl.h (308 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 31
- [ ] HASUB  (line 221, body 278e62466e6a)
- [ ] WINDOWS_OPAQUE_FILE  (line 284, body da39a3ee5e6b)
- [ ] _IOEOF  (line 305, body 34dfcaaf6c38)
- [ ] _IOERR  (line 306, body 9042d4228c01)
- [ ] _IOREAD  (line 302, body 1cb7a2e9afbe)
- [ ] _IORW  (line 304, body 60a17847c279)
- [ ] _IOWRT  (line 303, body 6b865327cda5)
- [ ] _IO_EOF_SEEN  (line 66, body 9042d4228c01)
- [ ] _IO_IN_BACKUP  (line 69, body bd60cfec3dba)
- [ ] _IO_LINE_BUF  (line 72, body f3504e0d733d)
- [ ] _IO_UNBUFFERED  (line 63, body 6b865327cda5)
- [ ] __SEOF  (line 234, body a6d133efc0d7)
- [ ] __SERR  (line 235, body bf3d90382fd9)
- [ ] __SLBF  (line 229, body 356a192b7913)
- [ ] __SNBF  (line 230, body da4b9237bacc)
- [ ] __SOFF  (line 238, body 987a3bdd4907)
- [ ] __SRD  (line 231, body 1b6453892473)
- [ ] __SRW  (line 233, body 9042d4228c01)
- [ ] __SWR  (line 232, body fe5dbbcea5ce)
- [ ] _base  (line 277, body 5257bfeb683c)
- [ ] _cnt  (line 275, body 2918c1f9d1f8)
- [ ] _flag  (line 278, body 2baca7ae4fcc)
- [ ] _flags  (line 108, body 8a67afa35101)
- [ ] _gl_file_offset_t  (line 167, body a27521b94e91)
- [ ] _gl_flags_file_t  (line 161, body a0f4ea7d9149)
- [ ] _p  (line 107, body f27062bddb08)
- [ ] _ptr  (line 276, body f62b4ab33b7c)
- [ ] _r  (line 109, body 3a97f7e1a2ad)
- [ ] _w  (line 110, body 40574b2d8632)
- [ ] fp_  (line 298, body 5a33c0cd7c41)
- [ ] fp_ub  (line 218, body 1e0b479bc03b)

## Types and their fields — 2
- [ ] __sfileext: struct  __sbuf _ub
- [ ] _gl_real_FILE: unsigned char* _ptr, unsigned char* _base, int _cnt, int _flag, int _file, int _charbuf, int _bufsiz

## File-scope objects — 0

## Decisions

- **WIP mirror, deliberately partial.**  Only the two things the port reaches are carried:
  the `fp_`/`fp_ub` decisions for this target (plain `fp`, `fp_->_ub`) and the measured
  `struct __sFILE`/`struct __sbuf` prefix that lib/fpending.c, lib/fseterr.c and
  lib/close-stream.c read.  The rest of the header is the twenty-odd other platforms'
  field-layout macros (`_IO_*`, `_IOERR`, `EPLAN9`, `__EMX__`, `__UCLIBC__`, `__QNX__`,
  `__MINT__`, the Windows `_gl_real_FILE`) — dead text for this configuration, and the
  unchecked boxes above are exactly that text.
- The struct is cut off after `_bf` with a 112-byte tail so `sizeof` stays the measured 152;
  the offsets the units read are asserted, and nothing in the port reads past `_bf`.
- `FILE` is named here so every unit keeps writing `*mut FILE` as C does; `stdio.rs` includes
  this file where its opaque `FILE` used to be.
