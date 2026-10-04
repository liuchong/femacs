# emacs/exec/exec.h (215 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _EXEC_H_  (line 23, body da39a3ee5e6b)

## Types and their fields — 7
- [x] dt_entry_32: uint32_t d_tag, uint32_t d_val
- [x] dt_entry_64: xint64_t d_tag, xint64_t d_val
- [x] elf_header_32: unsigned char e_ident, uint16_t e_type, uint16_t e_machine, uint32_t e_version, uint32_t e_entry, uint32_t e_phoff, uint32_t e_shoff, uint32_t e_flags, uint16_t e_ehsize, uint16_t e_phentsize, uint16_t e_phnum, uint16_t e_shentsize, uint16_t e_shnum, uint16_t e_shstrndx
- [x] elf_header_64: unsigned char e_ident, uint16_t e_type, uint16_t e_machine, uint32_t e_version, xint64_t e_entry, xint64_t e_phoff, xint64_t e_shoff, uint32_t e_flags, uint16_t e_ehsize, uint16_t e_phentsize, uint16_t e_phnum, uint16_t e_shentsize, uint16_t e_shnum, uint16_t e_shstrndx
- [x] exec_tracee: struct exec_tracee* next, USER_WORD sp, USER_WORD callno, char* exec_file, char* exec_data, size_t data_size, pid_t pid, bool waiting_for_syscall : 1, bool new_child : 1
- [x] program_header_32: uint32_t p_type, uint32_t p_offset, uint32_t p_vaddr, uint32_t p_paddr, uint32_t p_filesz, uint32_t p_memsz, uint32_t p_flags, uint32_t p_align
- [x] program_header_64: uint32_t p_type, uint32_t p_flags, xint64_t p_offset, xint64_t p_vaddr, xint64_t p_paddr, xint64_t p_filesz, xint64_t p_memsz, xint64_t p_align

## File-scope objects — 0

## Decisions

Mirror of exec/exec.h, complete for this target: the two ELF structure sets (32- and 64-bit
headers, program headers and dynamic entries), the typedefs that select one set, `struct
exec_tracee` with its two `bool : 1` members, and the loader's thirteen prototypes.

**Four names in this header are the host's, not the file's** - `exec/configure.ac` supplies them
and the reference build never runs it on this target: `USER_WORD` is `uintptr_t` on every host
the file configures (configure.ac 252/286/318/351), so the port spells the measured `uintptr_t`
(PORTING.md 1.1: `usize`); `USER_REGS_STRUCT` is the host's register structure
(`struct user_regs_struct`, `struct user_regs` or `struct pt_regs` - 242/276/308/341/367) and the
port has no mirror for any of them, so the pointers are spelled the way the `nt/inc` mirrors
spell `PSECURITY_DESCRIPTOR` (`*mut c_void`, P-72 (3)) with the macro named in the note;
`USER_HEADER` is that structure's header and is recorded (an `#include` has no Rust counterpart);
`HAVE_STDINT_H` is C 20-22's guard on `<stdint.h>`.

`exec/exec.h` is not in `docs/baseline/config/cond-eval-verified.txt`, so both arms of C 30's
`UINT64_MAX` test and of C 135's `EXEC_64` test are carried (PORTING.md P-51) under the features
`no_uint64_max` and `exec_64`.  The two `bool : 1` members share one storage byte in C, so the
mirror keeps `bits: u8` with the bit masks and the `exec_tracee_bits!` accessor family
(PORTING.md P-80: the member names are what the token gate looks for).

Nothing compiles the mirror: `xtask layout-check` covers `src/include` and `lib/include`, and
the reference build reaches no `exec/` rule.

Evidence: `python3 tools/extract-abi.py --check emacs/exec/exec.h femacs/exec/include/exec.rs`
-> `OK` (8 ledger items).
