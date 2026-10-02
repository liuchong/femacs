# emacs/lib/filename.h (115 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 11
- [x] FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE  (line 99, body b6589fc6ab0d)
- [x] FILE_SYSTEM_PREFIX_LEN  (line 98, body f3fa11f508e4)
- [x] HAS_DEVICE  (line 97, body f3fa11f508e4)
- [x] ISSLASH  (line 96, body 7ed32635b7b6)
- [x] IS_ABSOLUTE_FILE_NAME  (line 100, body bafdf8e0efc2)
- [x] IS_ABSOLUTE_PATH  (line 107, body 9511e4b1c95f)
- [x] IS_FILE_NAME_WITH_DIR  (line 102, body dd92c670eaa0)
- [x] IS_PATH_WITH_DIR  (line 108, body 92fd0528907f)
- [x] IS_RELATIVE_FILE_NAME  (line 101, body 8f6975cddb28)
- [x] _FILENAME_H  (line 22, body da39a3ee5e6b)
- [x] _IS_DRIVE_LETTER  (line 70, body 87ad89ab3bf1)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/filename.h` @ `fd7afced403`; port `femacs/lib/include/filename.rs`.  Every checklist item is
ticked and `extract-abi.py --check` reports `OK` for the pair.  The frozen target's arm is
the one implemented; PORTING.md P-24 does not compare a macro that has several definitions,
so the non-frozen arm's spellings are recorded here instead of being carried as text.

- The Windows/Cygwin/OS2/DOS arm (C 55-91) is dead on darwin-arm64; every macro it defines is
  also defined by the Unix arm except `_IS_DRIVE_LETTER`, whose Windows spelling the port
  keeps (nothing on the frozen target calls it).
