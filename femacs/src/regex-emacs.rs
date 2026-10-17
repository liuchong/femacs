// femacs: port of src/regex-emacs.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, file-scope object, local), same control flow, same error
// model.  C's two non-`static` functions the header exports beyond the `re_*` API
// (`print_compiled_pattern`, `re_wctype_parse`, `re_iswctype`) and its two non-`static`
// objects (`emacs_re_safe_alloca`, `emacs_re_max_failures`) carry `#[unsafe(no_mangle)]`
// with C's own name (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`.
//
// **The `regex-emacs.h` mirror lands with this unit** (the `docs/ports/thread.md`
// precedent): `struct re_pattern_buffer`, its six bit accessors and the eight
// declarations are in `femacs/src/include/regex-emacs.rs` now, so this unit `include!`s
// them instead of repeating C's text (PORTING.md 0.1-10).  `src/search.c`'s port carried
// its own copies of that record and those declarations because the mirror stopped at the
// character-class API; those copies are deleted in the same commit (two same-named
// definitions in one unit are an `E0428`).
//
// **`src/regex-emacs.c` is not in `docs/baseline/config/cond-eval-verified.txt`**
// (`tools/check-cond-eval.py src/regex-emacs.c` reports `15 conditions, 0 unknown,
// 1 disagreements` - the `#ifdef RE_DUP_MAX` of C 42), so the frozen macro table is not
// consulted for this file and, per docs/status.md 3.49, the text of *both* arms of every
// `#if` is carried: the arm the frozen configuration does not compile sits under the
// `#[cfg (...)]` that names C's own condition (PORTING.md P-51, the timefns.rs/ccl.rs
// precedent).  The arms are C 42-44's `#ifdef RE_DUP_MAX` (`# undef`, no effect on the
// `#define` that follows - the port writes the constant once), C 446-800's
// `#if defined REGEX_EMACS_DEBUG || ENABLE_CHECKING` block (the debug printers, live
// exactly when `--cfg enable_checking` is passed) and C 766-807's `REGEX_EMACS_DEBUG`
// block (the `DEBUG_*` macros and `print_double_string`; `REGEX_EMACS_DEBUG` is a
// source-level macro, not a `config.h` key, so it is not in `xtask dead-arms` either).
//
// **Statement macros that capture a local.**  Several of C's macros reference locals of
// the function that expands them (`TRANSLATE`, `PATFETCH`, `SET_LIST_BIT`, the
// `SETUP_*_RANGE` family, `EXTEND_BUFFER`, `PUSH_FAILURE_*`, `GET_CHAR_*`), and a
// `macro_rules!` resolves locals at its **definition** site, so those are defined inside
// their function, after the locals they capture (the bytecode.rs/ccl.rs P-57 placement).
// The `do { ... } while (false/0)` epilogue of C's statement macros has no Rust
// counterpart; the token it leaves (`false`/`0`) is one row of
// `docs/ports/macro-body-notes.tsv` per macro.
//
// **Layout**: the four records this file declares (`struct range_table_work_area`, the
// `fail_stack_elt` union, `fail_stack_type`, `compile_stack_elt_t`, `compile_stack_type`)
// are `#[repr(C)]` with clang-measured `const _` assertions; the unit also re-states
// `struct re_pattern_buffer`'s ten numbers, which docs/ports/search.md measured and the
// mirror pins (three units' assertions agree on that record).
//
// Every deviation, and the D7 call-relationship check, is in `docs/ports/regex-emacs.md`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_braces, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, path_statements)]

include!("include/config.rs");                 // C 26: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 449: `sysstdio.h`'s `<stdio.h>` (FILE, putc, fprintf, stderr)
include!("../lib/include/stdlib.rs");          // C 30: `#include <stdlib.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `<string.h>` (memcmp, memcpy, memset, strlen)
include!("../lib/include/stdint.in.rs");       // lisp.h 25: `<stdint.h>` (INTPTR_MAX, PRIdPTR)
include!("../lib/include/limits.in.rs");       // lisp.h 26: `<limits.h>` (LONG_MIN/LONG_MAX)
include!("include/lisp.rs");                   // regex-emacs.h 34: `#include "lisp.h"`
include!("include/character.rs");              // C 32: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 28: `#include "itree.h"`
include!("include/buffer.rs");                 // C 33: `#include "buffer.h"`
include!("include/syntax.rs");                 // C 34: `#include "syntax.h"`
include!("include/category.rs");               // C 35: `#include "category.h"`
include!("include/dispextern.rs");             // C 36: `#include "dispextern.h"`
include!("include/regex-emacs.rs");            // C 28: `#include "regex-emacs.h"`

/* Maximum number of duplicates an interval can allow.  Some systems
   define this in other header files, but we want our value, so remove
   any previous define.  Repeat counts are stored in opcodes as 2-byte
   unsigned integers.  */
// C 42-45: `#ifdef RE_DUP_MAX` / `# undef RE_DUP_MAX` / `#endif` / `#define RE_DUP_MAX
// (0xffff)`.  Whether the `# undef` runs or not (the one condition the evaluator and clang
// disagree about) leaves the value below unchanged, so one constant answers for both arms.
pub const RE_DUP_MAX: c_int = 0xffff;

/* Make syntax table lookup grant data in gl_state.  */
macro_rules! SYNTAX {
  ($c:expr) => { syntax_property ($c, true) };
}

/* Explicit syntax lookup using the buffer-local table.  */
macro_rules! BUFFER_SYNTAX {
  ($c:expr) => { syntax_property ($c, false) };
}

macro_rules! RE_MULTIBYTE_P { ($bufp:expr) => { multibyte_ ($bufp) }; }
macro_rules! RE_TARGET_MULTIBYTE_P { ($bufp:expr) => { target_multibyte_ ($bufp) }; }
macro_rules! RE_STRING_CHAR {
  ($p:expr, $multibyte:expr) => {
    if $multibyte { STRING_CHAR ($p) } else { (*$p).into () }
  };
}
macro_rules! RE_STRING_CHAR_AND_LENGTH {
  ($p:expr, $len:ident, $multibyte:expr) =>
    { if $multibyte { string_char_and_length ($p, &raw mut $len) } else { $len = 1; (*$p).into () } };
}

macro_rules! RE_CHAR_TO_MULTIBYTE { ($c:expr) => { UNIBYTE_TO_CHAR ($c as c_int) }; }

macro_rules! RE_CHAR_TO_UNIBYTE { ($c:expr) => { CHAR_TO_BYTE_SAFE ($c as c_int) }; }

/* Set C a (possibly converted to multibyte) character before P.  P
   points into a string which is the virtual concatenation of STR1
   (which ends at END1) or STR2 (which ends at END2).  */
// C 67-82.  References `target_multibyte`, `CHAR_HEAD_P`, `STRING_CHAR`: defined inside
// `re_match_2_internal`, where that local lives (the P-57 placement).

/* Set C a (possibly converted to multibyte) character at P, and set
   LEN to the byte length of that character.  */
// C 86-96, same placement as GET_CHAR_BEFORE_2.

/* 1 if C is an ASCII character.  */
macro_rules! IS_REAL_ASCII { ($c:expr) => { (($c) < 0o200) as c_int }; }

/* 1 if C is a unibyte character.  */
macro_rules! ISUNIBYTE { ($c:expr) => { SINGLE_BYTE_CHAR_P ($c as intmax_t) as c_int }; }

/* The Emacs definitions should not be directly affected by locales.  */

/* In Emacs, these are only used for single-byte characters.  */
macro_rules! ISDIGIT { ($c:expr) => { (($c) >= '0' as c_int && ($c) <= '9' as c_int) as c_int }; }
macro_rules! ISCNTRL { ($c:expr) => { (($c) < ' ' as c_int) as c_int }; }
macro_rules! ISXDIGIT { ($c:expr) => { (0 <= char_hexdigit ($c)) as c_int }; }

/* The rest must handle multibyte characters.  */

macro_rules! ISBLANK {
  ($c:expr) => {
    (if IS_REAL_ASCII! ($c) != 0 { ($c) == ' ' as c_int || ($c) == '\t' as c_int } else { blankp ($c) }) as c_int
  };
}

macro_rules! ISGRAPH {
  ($c:expr) => {
    (if SINGLE_BYTE_CHAR_P ($c as intmax_t) { ($c) > ' ' as c_int && !(($c) >= 0o177 && ($c) <= 0o240) }
    else { graphicp ($c) }) as c_int
  };
}

macro_rules! ISPRINT {
  ($c:expr) => {
    (if SINGLE_BYTE_CHAR_P ($c as intmax_t) { ($c) >= ' ' as c_int && !(($c) >= 0o177 && ($c) <= 0o237) }
    else { printablep ($c) }) as c_int
  };
}

macro_rules! ISALNUM {
  ($c:expr) => {
    (if IS_REAL_ASCII! ($c) != 0 { ($c) >= 'a' as c_int && ($c) <= 'z' as c_int
			     || ($c) >= 'A' as c_int && ($c) <= 'Z' as c_int
			     || ($c) >= '0' as c_int && ($c) <= '9' as c_int }
    else { alphanumericp ($c) }) as c_int
  };
}

macro_rules! ISALPHA {
  ($c:expr) => {
    (if IS_REAL_ASCII! ($c) != 0 { ($c) >= 'a' as c_int && ($c) <= 'z' as c_int
			     || ($c) >= 'A' as c_int && ($c) <= 'Z' as c_int }
    else { alphabeticp ($c) }) as c_int
  };
}

macro_rules! ISLOWER { ($c:expr) => { lowercasep ($c) as c_int }; }
macro_rules! ISUPPER { ($c:expr) => { uppercasep ($c) as c_int }; }

/* The following predicates use the buffer-local syntax table and
   ignore syntax properties, for consistency with the up-front
   assumptions made at compile time.  */

macro_rules! ISPUNCT {
  ($c:expr) => {
    (if IS_REAL_ASCII! ($c) != 0 { ($c) > ' ' as c_int && ($c) < 0o177
			     && !(($c) >= 'a' as c_int && ($c) <= 'z' as c_int
				  || ($c) >= 'A' as c_int && ($c) <= 'Z' as c_int
				  || ($c) >= '0' as c_int && ($c) <= '9' as c_int) }
    else { BUFFER_SYNTAX! ($c) != Sword }) as c_int
  };
}

macro_rules! ISSPACE { ($c:expr) => { (BUFFER_SYNTAX! ($c) == Swhitespace) as c_int }; }
macro_rules! ISWORD { ($c:expr) => { (BUFFER_SYNTAX! ($c) == Sword) as c_int }; }

/* Use alloca instead of malloc.  This is because using malloc in
   re_search* or re_match* could cause memory leaks when C-g is used
   in Emacs (note that SAFE_ALLOCA could also call malloc, but does so
   via 'record_xmalloc' which uses 'unwind_protect' to ensure the
   memory is freed even in case of non-local exits); also, malloc is
   slower and causes storage fragmentation.  On the other hand, malloc
   is more portable, and easier to debug.

   Because we sometimes use alloca, some routines have to be macros,
   not functions -- 'alloca'-allocated space disappears at the end of the
   function it is called in.  */

/* This may be adjusted in main(), if the stack is successfully grown.  */
#[unsafe(no_mangle)]
pub static mut emacs_re_safe_alloca: ptrdiff_t = MAX_ALLOCA;
/* Like USE_SAFE_ALLOCA, but use emacs_re_safe_alloca.  */
macro_rules! REGEX_USE_SAFE_ALLOCA {
  ($sa_avail:ident, $sa_count:ident, $sa_mark:ident) => {
    USE_SAFE_ALLOCA! ($sa_avail, $sa_count, $sa_mark);
    $sa_avail = emacs_re_safe_alloca;
  };
}

/* Assumes a 'char *destination' variable.  */
/* True if 'size1' is nonzero and PTR is pointing anywhere inside
   'string1' or just past its end.  This works if PTR is NULL, which is
   a good thing.  */
macro_rules! FIRST_STRING_P {
  ($ptr:expr) => { size1 != 0 && string1 <= ($ptr) && ($ptr) <= string1.offset (size1) };
}

// C 184: `#define BYTEWIDTH 8 /* In bits.  */`
pub const BYTEWIDTH: c_int = 8;

/* Type of source-pattern and string chars.  */
pub type re_char = u8;

// C 189-195's two file-local prototypes (`re_compile_fastmap`, `re_match_2_internal`) have
// no Rust counterpart: Rust items are order-independent and a same-named declaration plus
// definition is an `E0428` (PORTING.md 0.1-12).  The definitions keep C's position.

/* These are the command codes that appear in compiled regular
   expressions.  Some opcodes are followed by argument bytes.  A
   command code can specify any interpretation whatsoever for its
   arguments.  Zero bytes may appear in the compiled regular expression.  */

pub type re_opcode_t = c_int;

pub const no_op: re_opcode_t = 0;

/* Succeed right away--no more backtracking.  */
pub const succeed: re_opcode_t = 1;

        /* Followed by one byte giving n, then by n literal bytes.  */
pub const exactn: re_opcode_t = 2;

        /* Matches any (more or less) character.  */
pub const anychar: re_opcode_t = 3;

        /* Matches any one char belonging to specified set.  First
           following byte is number of bitmap bytes.  Then come bytes
           for a bitmap saying which chars are in.  Bits in each byte
           are ordered low-bit-first.  A character is in the set if its
           bit is 1.  A character too large to have a bit in the map is
           automatically not in the set.

           If the length byte has the 0x80 bit set, then that stuff
           is followed by a range table:
               2 bytes of flags for character sets (low 8 bits, high 8 bits)
                   See RANGE_TABLE_WORK_BITS below.
               2 bytes, the number of pairs that follow (up to 32767)
               pairs, each 2 multibyte characters,
                   each multibyte character represented as 3 bytes.  */
pub const charset: re_opcode_t = 4;

        /* Same parameters as charset, but match any character that is
           not one of those specified.  */
pub const charset_not: re_opcode_t = 5;

        /* Start remembering the text that is matched, for storing in a
           register.  Followed by one byte with the register number, in
           the range 0 to one less than the pattern buffer's re_nsub
           field.  */
pub const start_memory: re_opcode_t = 6;

        /* Stop remembering the text that is matched and store it in a
           memory register.  Followed by one byte with the register
           number, in the range 0 to one less than 're_nsub' in the
           pattern buffer.  */
pub const stop_memory: re_opcode_t = 7;

        /* Match a duplicate of something remembered. Followed by one
           byte containing the register number.  */
pub const duplicate: re_opcode_t = 8;

        /* Fail unless at beginning of line.  */
pub const begline: re_opcode_t = 9;

        /* Fail unless at end of line.  */
pub const endline: re_opcode_t = 10;

        /* Succeeds if at beginning of buffer.  */
pub const begbuf: re_opcode_t = 11;

        /* Analogously, for end of buffer/string.  */
pub const endbuf: re_opcode_t = 12;

        /* Followed by two byte relative address to which to jump.  */
pub const jump: re_opcode_t = 13;

        /* Followed by two-byte relative address of place to resume at
           in case of failure.  */
pub const on_failure_jump: re_opcode_t = 14;

        /* Like on_failure_jump, but pushes a placeholder instead of the
           current string position when executed.  Upon failure,
           the current string position is thus not restored.
           Used only for single-char loops that don't require backtracking.  */
pub const on_failure_keep_string_jump: re_opcode_t = 15;

        /* Just like 'on_failure_jump', except that it checks that we
           don't get stuck in an infinite loop (matching an empty string
           indefinitely).  */
pub const on_failure_jump_loop: re_opcode_t = 16;

        /* Just like 'on_failure_jump_loop', except that it checks for
           a different kind of loop (the kind that shows up with non-greedy
           operators).  This operation has to be immediately preceded
           by a 'no_op'.  */
pub const on_failure_jump_nastyloop: re_opcode_t = 17;

        /* A smart 'on_failure_jump' used for greedy * and + operators.
           It analyzes the loop before which it is put and if the
           loop does not require backtracking, it changes itself to
           'on_failure_keep_string_jump' and short-circuits the loop,
           else it just defaults to changing itself into 'on_failure_jump'.
           It assumes that it is pointing to just past a 'jump'.  */
pub const on_failure_jump_smart: re_opcode_t = 18;

        /* Followed by two-byte relative address and two-byte number n.
           After matching N times, jump to the address upon failure.
           Does not work if N starts at 0: use on_failure_jump_loop
           instead.  */
pub const succeed_n: re_opcode_t = 19;

        /* Followed by two-byte relative address, and two-byte number n.
           Jump to the address N times, then fail.  */
pub const jump_n: re_opcode_t = 20;

        /* Set the following two-byte relative address to the
           subsequent two-byte number.  The address *includes* the two
           bytes of number.  */
pub const set_number_at: re_opcode_t = 21;

pub const wordbeg: re_opcode_t = 22;	/* Succeeds if at word beginning.  */
pub const wordend: re_opcode_t = 23;	/* Succeeds if at word end.  */

pub const wordbound: re_opcode_t = 24;	/* Succeeds if at a word boundary.  */
pub const notwordbound: re_opcode_t = 25;	/* Succeeds if not at a word boundary.  */

pub const symbeg: re_opcode_t = 26;       /* Succeeds if at symbol beginning.  */
pub const symend: re_opcode_t = 27;       /* Succeeds if at symbol end.  */

        /* Matches any character whose syntax is specified.  Followed by
           a byte which contains a syntax code, e.g., Sword.  */
pub const syntaxspec: re_opcode_t = 28;

        /* Matches any character whose syntax is not that specified.  */
pub const notsyntaxspec: re_opcode_t = 29;

pub const at_dot: re_opcode_t = 30;	/* Succeeds if at point.  */

  /* Matches any character whose category-set contains the specified
     category.  The operator is followed by a byte which contains a
     category code (mnemonic ASCII character).  */
pub const categoryspec: re_opcode_t = 31;

  /* Matches any character whose category-set does not contain the
     specified category.  The operator is followed by a byte which
     contains the category code (mnemonic ASCII character).  */
pub const notcategoryspec: re_opcode_t = 32;

/* Common operations on the compiled pattern.  */

/* Store NUMBER in two contiguous bytes starting at DESTINATION.  */

unsafe fn STORE_NUMBER (destination: *mut u8, number: c_int)
{
  *destination.offset (0) = (number & 0o377) as u8;
  *destination.offset (1) = (number >> 8) as u8;
}

/* Same as STORE_NUMBER, except increment DESTINATION to
   the byte after where the number is stored.  Therefore, DESTINATION
   must be an lvalue.  */

macro_rules! STORE_NUMBER_AND_INCR {
  ($destination:expr, $number:expr) => {
    {
      STORE_NUMBER ($destination, $number);
      $destination = $destination.offset (2);
    }
  };
}

/* Put into DESTINATION a number stored in two contiguous bytes starting
   at SOURCE.  */

macro_rules! EXTRACT_NUMBER {
  ($destination:expr, $source:expr) => { $destination = extract_number ($source) };
}

unsafe fn extract_number (source: *const re_char) -> c_int
{
  let leading_byte: c_char = *source.offset (1) as c_char;
  (leading_byte as c_int) * 256 + *source.offset (0) as c_int
}

unsafe fn extract_address (source: *const re_char) -> *const re_char
{
  source.offset (2 + extract_number (source) as isize)
}

/* Same as EXTRACT_NUMBER, except increment SOURCE to after the number.
   SOURCE must be an lvalue.  */

macro_rules! EXTRACT_NUMBER_AND_INCR {
  ($destination:expr, $source:expr) => { $destination = extract_number_and_incr (&raw mut $source) };
}

unsafe fn extract_number_and_incr (source: *mut *const re_char) -> c_int
{
  let num = extract_number (*source);
  *source = (*source).offset (2);
  num
}

/* Store a multibyte character in three contiguous bytes starting
   DESTINATION, and increment DESTINATION to the byte after where the
   character is stored.  Therefore, DESTINATION must be an lvalue.  */

macro_rules! STORE_CHARACTER_AND_INCR {
  ($destination:expr, $character:expr) => {
    {
      *$destination.offset (0) = (($character) & 0o377) as u8;
      *$destination.offset (1) = ((($character) >> 8) & 0o377) as u8;
      *$destination.offset (2) = (($character) >> 16) as u8;
      $destination = $destination.offset (3);
    }
  };
}

/* Put into DESTINATION a character stored in three contiguous bytes
   starting at SOURCE.  */

macro_rules! EXTRACT_CHARACTER {
  ($destination:expr, $source:expr) => {
    {
      $destination = *$source.offset (0) as c_int
		     | ((*$source.offset (1) as c_int) << 8)
		     | ((*$source.offset (2) as c_int) << 16);
    }
  };
}

/* Macros for charset. */

/* Size of bitmap of charset P in bytes.  P is a start of charset,
   i.e. *P is (re_opcode_t) charset or (re_opcode_t) charset_not.  */
macro_rules! CHARSET_BITMAP_SIZE { ($p:expr) => { *($p).offset (1) as c_int & 0x7F }; }

/* Nonzero if charset P has range table.  */
macro_rules! CHARSET_RANGE_TABLE_EXISTS_P {
  ($p:expr) => { (*($p).offset (1) & 0x80) != 0 };
}

/* Return the address of range table of charset P.  But not the start
   of table itself, but the before where the number of ranges is
   stored.  '2 +' means to skip re_opcode_t and size of bitmap,
   and the 2 bytes of flags at the start of the range table.  */
macro_rules! CHARSET_RANGE_TABLE {
  ($p:expr) => { ($p).offset (4 + CHARSET_BITMAP_SIZE! ($p) as isize) };
}

/* Extract the bit flags that start a range table.  */
macro_rules! CHARSET_RANGE_TABLE_BITS {
  ($p:expr) => {
    *($p).offset (2 + CHARSET_BITMAP_SIZE! ($p) as isize) as c_int
      + (*($p).offset (3 + CHARSET_BITMAP_SIZE! ($p) as isize) as c_int) * 0x100
  };
}

/* Return the address of end of RANGE_TABLE.  COUNT is number of
   ranges (which is a pair of (start, end)) in the RANGE_TABLE.  '* 2'
   is start of range and end of range.  '* 3' is size of each start
   and end.  */
macro_rules! CHARSET_RANGE_TABLE_END {
  ($range_table:expr, $count:expr) => { ($range_table).offset (($count) as isize * 2 * 3) };
}

/* If REGEX_EMACS_DEBUG is defined, print many voluminous messages
   (if the variable regex_emacs_debug is positive).  */

// C 446: `#if defined REGEX_EMACS_DEBUG || ENABLE_CHECKING`.  `ENABLE_CHECKING` is undef in
// the frozen configuration (macros.tsv 872) and `REGEX_EMACS_DEBUG` is a source-level
// macro, so clang compiles none of 447-762; the port keys the block on
// `#[cfg (enable_checking)]` (the search.rs/lisp.rs key) so that the same text compiles
// under `--cfg enable_checking`.  Both arms are carried (docs/status.md 3.49).

/* The debug printers name <stdio.h>'s writes and `stderr` (C 449's `sysstdio.h` chain).  The
   mirror carries `fputs`; `fprintf`/`putc`/`stderr` have no mirror, so they are declared at
   their use site (PORTING.md 0.1-2/P-72, the bytecode.rs/region-cache.rs shape).  */
unsafe extern "C" {
}

/* Use standard I/O for debugging.  */
// C 449: `# include "sysstdio.h"` - its `<stdio.h>` is included at the top.

#[cfg (any (regex_emacs_debug, enable_checking))]
unsafe fn debug_putchar (dest: *mut FILE, c: c_int)
{
  if c >= 32 && c <= 126
    { putc (c, dest); }
  else
    {
      let uc: c_uint = c as c_uint;
      fprintf (dest, c"{%02x}".as_ptr (), uc);
    }
}

/* Print the fastmap in human-readable form.  */

#[cfg (any (regex_emacs_debug, enable_checking))]
unsafe fn print_fastmap (dest: *mut FILE, fastmap: *mut c_char)
{
  let mut was_a_range = false;
  let mut i = 0;

  while i < (1 << BYTEWIDTH)
    {
      /* C: `if (fastmap[i++])` - the increment happens whatever the branch is.  */
      let this_char = *fastmap.offset (i as isize) != 0;
      i += 1;
      if this_char
	{
	  was_a_range = false;
	  debug_putchar (dest, i - 1);
	  while i < (1 << BYTEWIDTH)  &&  *fastmap.offset (i as isize) != 0
	    {
	      was_a_range = true;
	      i += 1;
	    }
	  if was_a_range
	    {
	      debug_putchar (dest, '-' as c_int);
	      debug_putchar (dest, i - 1);
	    }
	}
    }
  putc ('\n' as c_int, dest);
}

/* Print a compiled pattern string in human-readable form, starting at
   the START pointer into it and ending just before the pointer END.  */

#[cfg (any (regex_emacs_debug, enable_checking))]
unsafe fn print_partial_compiled_pattern (dest: *mut FILE, start: *const re_char,
					  end: *const re_char)
{
  let mut mcnt: c_int = 0;
  let mut mcnt2: c_int = 0;
  let mut p = start;
  let pend = end;

  if start.is_null ()
    {
      fputs (c"(null)\n".as_ptr (), dest);
      return;
    }

  /* Loop over pattern commands.  */
  while p < pend
    {
      fprintf (dest, c"%td:\t".as_ptr (), p.offset_from (start));

      match { let op = *p as re_opcode_t; p = p.offset (1); op }
	{
	  no_op => { fputs (c"/no_op".as_ptr (), dest); }

	  succeed => { fputs (c"/succeed".as_ptr (), dest); }

	  exactn => {
	    mcnt = *p as c_int; p = p.offset (1);
	    fprintf (dest, c"/exactn/%d".as_ptr (), mcnt);
	    loop
	      {
		debug_putchar (dest, '/' as c_int);
		debug_putchar (dest, *p as c_int); p = p.offset (1);
		mcnt -= 1;
		if mcnt == 0 { break; }
	      }
	  }

	  start_memory => {
	    fprintf (dest, c"/start_memory/%d".as_ptr (), *p as c_int); p = p.offset (1);
	  }

	  stop_memory => {
	    fprintf (dest, c"/stop_memory/%d".as_ptr (), *p as c_int); p = p.offset (1);
	  }

	  duplicate => {
	    fprintf (dest, c"/duplicate/%d".as_ptr (), *p as c_int); p = p.offset (1);
	  }

	  anychar => { fputs (c"/anychar".as_ptr (), dest); }

	  charset | charset_not => {
	    let mut c: c_int;
	    let mut last: c_int = -100;
	    let mut in_range = false;
	    let length = CHARSET_BITMAP_SIZE! (p.offset (-1));
	    let has_range_table = CHARSET_RANGE_TABLE_EXISTS_P! (p.offset (-1));

	    fprintf (dest, c"/charset [%s".as_ptr (),
		     if *p.offset (-1) as re_opcode_t == charset_not { c"^".as_ptr () }
		     else { c"".as_ptr () });

	    if p.offset (*p as isize & 0x7f) >= pend
	      { fputs (c" !extends past end of pattern! ".as_ptr (), dest); }

	    c = 0;
	    while c < 256
	      {
		if c / 8 < length
		    && (*p.offset (1 + (c / 8) as isize) as c_int & (1 << (c % 8))) != 0
		  {
		    /* Are we starting a range?  */
		    if last + 1 == c && ! in_range
		      {
			debug_putchar (dest, '-' as c_int);
			in_range = true;
		      }
		    /* Have we broken a range?  */
		    else if last + 1 != c && in_range
		      {
			debug_putchar (dest, last);
			in_range = false;
		      }

		    if ! in_range
		      { debug_putchar (dest, c); }

		    last = c;
		  }
		c += 1;
	      }

	    if in_range
	      { debug_putchar (dest, last); }

	    debug_putchar (dest, ']' as c_int);

	    p = p.offset (1 + length as isize);

	    if has_range_table
	      {
		let mut count: c_int = 0;
		fputs (c"has-range-table".as_ptr (), dest);

		/* ??? Should print the range table; for now, just skip it.  */
		p = p.offset (2);		/* skip range table bits */
		EXTRACT_NUMBER_AND_INCR! (count, p);
		p = CHARSET_RANGE_TABLE_END! (p, count);
	      }
	  }

	  begline => { fputs (c"/begline".as_ptr (), dest); }

	  endline => { fputs (c"/endline".as_ptr (), dest); }

	  on_failure_jump => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    fprintf (dest, c"/on_failure_jump to %td".as_ptr (),
		     p.offset (mcnt as isize).offset_from (start));
	  }

	  on_failure_keep_string_jump => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    fprintf (dest, c"/on_failure_keep_string_jump to %td".as_ptr (),
		     p.offset (mcnt as isize).offset_from (start));
	  }

	  on_failure_jump_nastyloop => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    fprintf (dest, c"/on_failure_jump_nastyloop to %td".as_ptr (),
		     p.offset (mcnt as isize).offset_from (start));
	  }

	  on_failure_jump_loop => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    fprintf (dest, c"/on_failure_jump_loop to %td".as_ptr (),
		     p.offset (mcnt as isize).offset_from (start));
	  }

	  on_failure_jump_smart => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    fprintf (dest, c"/on_failure_jump_smart to %td".as_ptr (),
		     p.offset (mcnt as isize).offset_from (start));
	  }

	  jump => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    fprintf (dest, c"/jump to %td".as_ptr (),
		     p.offset (mcnt as isize).offset_from (start));
	  }

	  succeed_n => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    EXTRACT_NUMBER_AND_INCR! (mcnt2, p);
	    fprintf (dest, c"/succeed_n to %td, %d times".as_ptr (),
		     p.offset (-2).offset (mcnt as isize).offset_from (start), mcnt2);
	  }

	  jump_n => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    EXTRACT_NUMBER_AND_INCR! (mcnt2, p);
	    fprintf (dest, c"/jump_n to %td, %d times".as_ptr (),
		     p.offset (-2).offset (mcnt as isize).offset_from (start), mcnt2);
	  }

	  set_number_at => {
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    EXTRACT_NUMBER_AND_INCR! (mcnt2, p);
	    fprintf (dest, c"/set_number_at location %td to %d".as_ptr (),
		     p.offset (-2).offset (mcnt as isize).offset_from (start), mcnt2);
	  }

	  wordbound => { fputs (c"/wordbound".as_ptr (), dest); }

	  notwordbound => { fputs (c"/notwordbound".as_ptr (), dest); }

	  wordbeg => { fputs (c"/wordbeg".as_ptr (), dest); }

	  wordend => { fputs (c"/wordend".as_ptr (), dest); }

	  symbeg => { fputs (c"/symbeg".as_ptr (), dest); }

	  symend => { fputs (c"/symend".as_ptr (), dest); }

	  syntaxspec => {
	    fputs (c"/syntaxspec".as_ptr (), dest);
	    mcnt = *p as c_int; p = p.offset (1);
	    fprintf (dest, c"/%d".as_ptr (), mcnt);
	  }

	  notsyntaxspec => {
	    fputs (c"/notsyntaxspec".as_ptr (), dest);
	    mcnt = *p as c_int; p = p.offset (1);
	    fprintf (dest, c"/%d".as_ptr (), mcnt);
	  }

	  at_dot => { fputs (c"/at_dot".as_ptr (), dest); }

	  categoryspec => {
	    fputs (c"/categoryspec".as_ptr (), dest);
	    mcnt = *p as c_int; p = p.offset (1);
	    fprintf (dest, c"/%d".as_ptr (), mcnt);
	  }

	  notcategoryspec => {
	    fputs (c"/notcategoryspec".as_ptr (), dest);
	    mcnt = *p as c_int; p = p.offset (1);
	    fprintf (dest, c"/%d".as_ptr (), mcnt);
	  }

	  begbuf => { fputs (c"/begbuf".as_ptr (), dest); }

	  endbuf => { fputs (c"/endbuf".as_ptr (), dest); }

	  _ => { fprintf (dest, c"?%d".as_ptr (), *p.offset (-1) as c_int); }
	}

      putc ('\n' as c_int, dest);
    }

  fprintf (dest, c"%td:\tend of pattern.\n".as_ptr (), p.offset_from (start));
}

#[cfg (any (regex_emacs_debug, enable_checking))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_compiled_pattern (dest: *mut FILE,
						 bufp: *mut re_pattern_buffer)
{
  let mut dest = dest;
  if dest.is_null ()
    {
      dest = stderr;
    }
  let buffer = (*bufp).buffer as *const re_char;

  print_partial_compiled_pattern (dest, buffer, buffer.offset ((*bufp).used));

  fprintf (dest, c"%td bytes used/%td bytes allocated.\n".as_ptr (),
           (*bufp).used, (*bufp).allocated);

  if fastmap_accurate_ (bufp) && !(*bufp).fastmap.is_null ()
    {
      fputs (c"fastmap: ".as_ptr (), dest);
      print_fastmap (dest, (*bufp).fastmap);
    }

  fprintf (dest, c"re_nsub: %td\t".as_ptr (), (*bufp).re_nsub);
  fprintf (dest, c"regs_alloc: %d\t".as_ptr (), regs_allocated_ (bufp) as c_int);
  fprintf (dest, c"can_be_null: %d\n".as_ptr (), can_be_null_ (bufp) as c_int);
  /* Perhaps we should print the translate table?  */
}

// C 764: `#endif` of the debug/checking block.

// C 766: `#ifdef REGEX_EMACS_DEBUG` (undefined in every configuration of this target; both
// arms carried, docs/status.md 3.49).

#[cfg (regex_emacs_debug)]
static mut regex_emacs_debug: c_int = -100000;

#[cfg (regex_emacs_debug)]
macro_rules! DEBUG_STATEMENT { ($e:expr) => { $e }; }
#[cfg (not (regex_emacs_debug))]
macro_rules! DEBUG_STATEMENT { ($e:expr) => { }; }
#[cfg (regex_emacs_debug)]
macro_rules! DEBUG_PRINT {
  ($($arg:tt)*) => {
    if regex_emacs_debug > 0 { fprintf (stderr, $($arg)*); }
  };
}
#[cfg (not (regex_emacs_debug))]
macro_rules! DEBUG_PRINT {
  ($($arg:tt)*) => { };
}
#[cfg (regex_emacs_debug)]
macro_rules! DEBUG_COMPILES_ARGUMENTS { () => { }; }
#[cfg (not (regex_emacs_debug))]
macro_rules! DEBUG_COMPILES_ARGUMENTS { () => { }; }
#[cfg (regex_emacs_debug)]
macro_rules! DEBUG_PRINT_COMPILED_PATTERN {
  ($p:expr, $s:expr, $e:expr) => {
    { if regex_emacs_debug > 0 { print_partial_compiled_pattern (stderr, $s, $e); } }
  };
}
#[cfg (not (regex_emacs_debug))]
macro_rules! DEBUG_PRINT_COMPILED_PATTERN {
  ($p:expr, $s:expr, $e:expr) => { };
}
#[cfg (regex_emacs_debug)]
macro_rules! DEBUG_PRINT_DOUBLE_STRING {
  ($w:expr, $s1:expr, $sz1:expr, $s2:expr, $sz2:expr) => {
    { if regex_emacs_debug > 0 { print_double_string ($w, $s1, $sz1, $s2, $sz2); } }
  };
}
#[cfg (not (regex_emacs_debug))]
macro_rules! DEBUG_PRINT_DOUBLE_STRING {
  ($w:expr, $s1:expr, $sz1:expr, $s2:expr, $sz2:expr) => { };
}

#[cfg (regex_emacs_debug)]
unsafe fn print_double_string (w: *const re_char, mut string1: *const re_char, size1: ptrdiff_t,
			       string2: *const re_char, size2: ptrdiff_t)
{
  let mut where_ = w;
  if where_.is_null ()
    { fputs (c"(null)".as_ptr (), stderr); }
  else
    {
      let mut i: c_int;
      if FIRST_STRING_P! (where_)
	{
	  i = 0;
	  while i < string1.offset (size1).offset_from (where_) as c_int
	    { debug_putchar (stderr, *where_.offset (i as isize) as c_int); i += 1; }
	  where_ = string2;
	}

      i = 0;
      while i < string2.offset (size2).offset_from (where_) as c_int
        { debug_putchar (stderr, *where_.offset (i as isize) as c_int); i += 1; }
    }
}

// C 800-807's `#else` arm: the four macros expand to nothing.  The two spellings above are
// the two arms; `REGEX_EMACS_DEBUG` selects one and the port keys the same pair on that
// source-level macro (not a `config.h` key, so not in `xtask dead-arms`).

pub type reg_errcode_t = c_int;

pub const REG_NOERROR: reg_errcode_t = 0;	/* Success.  */
pub const REG_NOMATCH: reg_errcode_t = 1;	/* Didn't find a match (for regexec).  */

  /* POSIX regcomp return error codes.  (In the order listed in the
     standard.)  An older version of this code supported the POSIX
     API; this version continues to use these names internally.  */
pub const REG_BADPAT: reg_errcode_t = 2;		/* Invalid pattern.  */
pub const REG_ECOLLATE: reg_errcode_t = 3;		/* Not implemented.  */
pub const REG_ECTYPE: reg_errcode_t = 4;		/* Invalid character class name.  */
pub const REG_EESCAPE: reg_errcode_t = 5;		/* Trailing backslash.  */
pub const REG_ESUBREG: reg_errcode_t = 6;		/* Invalid back reference.  */
pub const REG_EBRACK: reg_errcode_t = 7;		/* Unmatched left bracket.  */
pub const REG_EPAREN: reg_errcode_t = 8;		/* Parenthesis imbalance.  */
pub const REG_EBRACE: reg_errcode_t = 9;		/* Unmatched \{.  */
pub const REG_BADBR: reg_errcode_t = 10;		/* Invalid contents of \{\}.  */
pub const REG_ERANGE: reg_errcode_t = 11;		/* Invalid range end.  */
pub const REG_ESPACE: reg_errcode_t = 12;		/* Ran out of memory.  */
pub const REG_BADRPT: reg_errcode_t = 13;		/* No preceding re for repetition op.  */

  /* Error codes we've added.  */
pub const REG_EEND: reg_errcode_t = 14;		/* Premature end.  */
pub const REG_ESIZE: reg_errcode_t = 15;	/* Compiled pattern bigger than 2^16 bytes.  */
pub const REG_ERPAREN: reg_errcode_t = 16;	/* Unmatched ) or \); not returned from regcomp.  */
pub const REG_ERANGEX: reg_errcode_t = 17;	/* Range striding over charsets.  */
pub const REG_ESIZEBR: reg_errcode_t = 18;           /* n or m too big in \{n,m\} */

static mut re_error_msgid: [*const c_char; 19] =
  [
   c"Success".as_ptr (),                                  // [REG_NOERROR]
   c"No match".as_ptr (),                                 // [REG_NOMATCH]
   c"Invalid regular expression".as_ptr (),               // [REG_BADPAT]
   c"Invalid collation character".as_ptr (),              // [REG_ECOLLATE]
   c"Invalid character class name".as_ptr (),             // [REG_ECTYPE]
   c"Trailing backslash".as_ptr (),                       // [REG_EESCAPE]
   c"Invalid back reference".as_ptr (),                   // [REG_ESUBREG]
   c"Unmatched [ or [^".as_ptr (),                        // [REG_EBRACK]
   c"Unmatched ( or \\(".as_ptr (),                       // [REG_EPAREN]
   c"Unmatched \\{".as_ptr (),                            // [REG_EBRACE]
   c"Invalid content of \\\\{\\\\}".as_ptr (),             // [REG_BADBR]
   c"Invalid range end".as_ptr (),                        // [REG_ERANGE]
   c"Memory exhausted".as_ptr (),                         // [REG_ESPACE]
   c"Invalid preceding regular expression".as_ptr (),     // [REG_BADRPT]
   c"Premature end of regular expression".as_ptr (),      // [REG_EEND]
   c"Regular expression too big".as_ptr (),               // [REG_ESIZE]
   c"Unmatched ) or \\)".as_ptr (),                       // [REG_ERPAREN]
   c"Range striding over charsets".as_ptr (),             // [REG_ERANGEX]
   c"Invalid content of \\\\{\\\\}".as_ptr (),             // [REG_ESIZEBR]
  ];

/* For 'regs_allocated'.  */
pub const REGS_UNALLOCATED: c_int = 0;
pub const REGS_REALLOCATE: c_int = 1;
pub const REGS_FIXED: c_int = 2;

/* If 'regs_allocated' is REGS_UNALLOCATED in the pattern buffer,
   're_match_2' returns information about at least this many registers
   the first time a 'regs' structure is passed.  */
pub const RE_NREGS: c_int = 30;

/* The searching and matching functions allocate memory for the
   failure stack and registers.  Otherwise searching and matching
   routines would have much smaller memory resources at their
   disposal, and therefore might fail to handle complex regexps.  */

/* Failure stack declarations and macros; both re_compile_fastmap and
   re_match_2 use a failure stack.  These have to be macros because of
   REGEX_USE_SAFE_ALLOCA.  */

/* Approximate number of failure points for which to initially allocate space
   when matching.  If this number is exceeded, we allocate more
   space, so it is not a hard limit.  */
pub const INIT_FAILURE_ALLOC: usize = 20;

/* Roughly the maximum number of failure points on the stack.  Would be
   exactly that if failure always used TYPICAL_FAILURE_SIZE items.
   This is a variable only so users of regex can assign to it; we never
   change it ourselves.  We always multiply it by TYPICAL_FAILURE_SIZE
   before using it, so it should probably be a byte-count instead.  */
/* Note that 4400 was enough to cause a crash on Alpha OSF/1,
   whose default stack limit is 2mb.  In order for a larger
   value to work reliably, you have to try to make it accord
   with the process stack limit.  */
#[unsafe(no_mangle)]
pub static mut emacs_re_max_failures: ptrdiff_t = 40000;

#[repr(C)]
#[derive(Clone, Copy)]
pub union fail_stack_elt
{
  pub pointer: *const re_char,
  pub integer: isize,
}

pub type fail_stack_elt_t = fail_stack_elt;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct fail_stack_type
{
  pub stack: *mut fail_stack_elt_t,
  pub size: ptrdiff_t,
  pub avail: ptrdiff_t,	/* Offset of next open position.  */
  pub frame: ptrdiff_t,	/* Offset of the cur constructed frame.  */
}
const _: () = core::assert! (std::mem::size_of::<fail_stack_elt> () == 8);
const _: () = core::assert! (std::mem::size_of::<fail_stack_type> () == 32);
const _: () = core::assert! (std::mem::offset_of! (fail_stack_type, stack) == 0);
const _: () = core::assert! (std::mem::offset_of! (fail_stack_type, size) == 8);
const _: () = core::assert! (std::mem::offset_of! (fail_stack_type, avail) == 16);
const _: () = core::assert! (std::mem::offset_of! (fail_stack_type, frame) == 24);

macro_rules! FAIL_STACK_EMPTY { () => { fail_stack.frame == 0 }; }

/* Define macros to initialize and free the failure stack.  */

/* Every `macro_rules!` name C defines between here and POP_FAILURE_POINT's `do { ... }
   while (false)` (C 916-1160) names `fail_stack`, `sa_avail`, `regstart`, `regend` or
   `num_regs`, a local of `re_match_2_internal`; a `macro_rules!` resolves locals at its
   *definition* site, so the definitions sit inside that function, right after those
   locals (PORTING.md 2.2/P-57, the bytecode.rs/ccl.rs placement).  C's comments stay at
   their own positions here.  */

/* Double the size of FAIL_STACK, up to a limit
   which allows approximately 'emacs_re_max_failures' items.

   Return 1 if succeeds, and 0 if either ran out of memory
   allocating space for it or it was already too large.

   REGEX_REALLOCATE requires 'destination' be declared.   */

/* Factor to increase the failure stack size by.
   This used to be 2, but 2 was too wasteful
   because the old discarded stacks added up to as much space
   were as ultimate, maximum-size stack.  */
pub const FAIL_STACK_GROWTH_FACTOR: ptrdiff_t = 4;

/* Push a pointer value onto the failure stack.
   Assumes the variable 'fail_stack'.  Probably should only
   be called from within 'PUSH_FAILURE_POINT'.  */
/* This pushes an integer-valued item onto the failure stack.
   Assumes the variable 'fail_stack'.  Probably should only
   be called from within 'PUSH_FAILURE_POINT'.  */
/* These POP... operations complement the PUSH... operations.
   All assume that 'fail_stack' is nonempty.  */
/* Individual items aside from the registers.  */
pub const NUM_NONREG_ITEMS: ptrdiff_t = 3;

/* Used to examine the stack (to detect infinite loops).  */

/* Estimate the size of data pushed by a typical failure stack entry.
   An estimate is all we need, because all we use this for
   is to choose a limit for how big to make the failure stack.  */
/* BEWARE, the value `20' is hard-coded in emacs.c:main().  */
pub const TYPICAL_FAILURE_SIZE: ptrdiff_t = 20;

/* How many items can still be added to the stack without overflowing it.  */
// C 1115: `REMAINING_AVAIL_SLOTS` names `fail_stack`, so it is defined inside
// `re_match_2_internal` with the other failure-stack macros (P-57).

/* Pop what PUSH_FAIL_STACK pushes.

   Restore into the parameters, all of which should be lvalues:
     STR -- the saved data position.
     PAT -- the saved pattern position.
     REGSTART, REGEND -- arrays of string positions.

   Also assume the variables FAIL_STACK and (if debugging) BUFP, PEND,
   STRING1, SIZE1, STRING2, and SIZE2.  */

// C 1128-1160: `POP_FAILURE_POINT` is defined inside `re_match_2_internal` (P-57).

/* Registers are set to a sentinel when they haven't yet matched.  */
macro_rules! REG_UNSET { ($e:expr) => { ($e).is_null () }; }

/* Subroutine declarations and macros for regex_compile.  */

// C 1169-1183's eight file-local prototypes (`regex_compile`, `store_op1`, `store_op2`,
// `insert_op1`, `insert_op2`, `at_begline_loc_p`, `at_endline_loc_p`, `skip_one_char`,
// `analyze_first`) have no Rust counterpart (PORTING.md 0.1-12: a same-named declaration
// plus definition in one unit is an `E0428`); the definitions keep C's positions.

/* Fetch the next character in the uncompiled pattern, with no
   translation.  */
// C 1187-1193: `PATFETCH` names `p`, `pend` and `multibyte` and returns `REG_EEND` from the
// surrounding `regex_compile`; it is defined inside that function (P-57).

macro_rules! RE_TRANSLATE {
  ($TBL:expr, $C:expr) => { char_table_translate ($TBL, $C) };
}
// C 1197: `TRANSLATE(d)` reads the local `translate` of `regex_compile`, so it is defined
// inside that function (P-57).

/* Macros for outputting the compiled pattern into 'buffer'.  */

/* If the buffer isn't allocated when it comes in, use this.  */
pub const INIT_BUF_SIZE: ptrdiff_t = 32;

/* Ensure at least N more bytes of space in buffer.  */
// `GET_BUFFER_SPACE`, `BUF_PUSH`, `BUF_PUSH_2`, `STORE_JUMP`'s two `INSERT_*` forms,
// `FIXUP_ALT_JUMP` and `EXTEND_BUFFER` (C 1205-1277) all name `bufp`, `b` or the other
// locals of `regex_compile`; they are defined inside it (P-57).  C's `STORE_JUMP`,
// `STORE_JUMP2`, `INSERT_JUMP` and `INSERT_JUMP2` need no local beyond their arguments.

/* Store a jump with opcode OP at LOC to location TO.  Store a
   relative address offset by the three bytes the jump itself occupies.  */
macro_rules! STORE_JUMP {
  ($op:expr, $loc:expr, $to:expr) => { store_op1 ($op, $loc, (($to).offset_from ($loc) - 3) as c_int) };
}

/* Likewise, for a two-argument jump.  */
macro_rules! STORE_JUMP2 {
  ($op:expr, $loc:expr, $to:expr, $arg:expr) =>
    { store_op2 ($op, $loc, (($to).offset_from ($loc) - 3) as c_int, $arg) };
}

/* Like 'STORE_JUMP', but for inserting.  Assume B is the buffer end.  */
// C 1236: `INSERT_JUMP`'s `b` is the local of `regex_compile` (P-57).

/* Like 'STORE_JUMP2', but for inserting.  Assume B is the buffer end.  */
// C 1240: same.

/* This is not an arbitrary limit: the arguments which represent offsets
   into the pattern are two bytes long.  So if 2^15 bytes turns out to
   be too small, many things would have to change.  */
pub const MAX_BUF_SIZE: ptrdiff_t = 1 << 15;

/* Extend the buffer by at least N bytes via realloc and
   reset the pointers that pointed into the old block to point to the
   correct places in the new one.  If extending the buffer results in it
   being larger than MAX_BUF_SIZE, then flag memory exhausted.  */
// C 1253-1277: `EXTEND_BUFFER` is defined inside `regex_compile` (P-57).

/* Since we have one byte reserved for the register number argument to
   {start,stop}_memory, the maximum number of groups we can report
   things about is what fits in that byte.  */
pub const MAX_REGNUM: c_int = 255;

/* But patterns can have more than 'MAX_REGNUM' registers.  Just
   ignore the excess.  */
pub type regnum_t = c_int;

/* Macros for the compile stack.  */

pub type pattern_offset_t = c_long;
const _: () = core::assert! (c_long::MIN <= -(MAX_BUF_SIZE - 1) as c_long
		       && (MAX_BUF_SIZE - 1) as c_long <= c_long::MAX);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct compile_stack_elt_t
{
  pub begalt_offset: pattern_offset_t,
  pub fixup_alt_jump: pattern_offset_t,
  pub laststart_offset: pattern_offset_t,
  pub regnum: regnum_t,
}
const _: () = core::assert! (std::mem::size_of::<compile_stack_elt_t> () == 32);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_elt_t, begalt_offset) == 0);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_elt_t, fixup_alt_jump) == 8);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_elt_t, laststart_offset) == 16);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_elt_t, regnum) == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct compile_stack_type
{
  pub stack: *mut compile_stack_elt_t,
  pub size: ptrdiff_t,
  pub avail: ptrdiff_t,		/* Offset of next open position.  */
}
const _: () = core::assert! (std::mem::size_of::<compile_stack_type> () == 24);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_type, stack) == 0);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_type, size) == 8);
const _: () = core::assert! (std::mem::offset_of! (compile_stack_type, avail) == 16);

pub const INIT_COMPILE_STACK_SIZE: ptrdiff_t = 32;

/* The next available element.  */
/* Structure to manage work area for range table.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct range_table_work_area
{
  pub table: *mut c_int,		/* actual work area.  */
  pub allocated: c_int,		/* allocated size for work area in bytes.  */
  pub used: c_int,		/* actually used size in words.  */
  pub bits: c_int,		/* flag to record character classes */
}
const _: () = core::assert! (std::mem::size_of::<range_table_work_area> () == 24);
const _: () = core::assert! (std::mem::offset_of! (range_table_work_area, table) == 0);
const _: () = core::assert! (std::mem::offset_of! (range_table_work_area, allocated) == 8);
const _: () = core::assert! (std::mem::offset_of! (range_table_work_area, used) == 12);
const _: () = core::assert! (std::mem::offset_of! (range_table_work_area, bits) == 16);

/* Make sure that WORK_AREA can hold N more multibyte characters.
   If it can't get the space, it returns from the surrounding function.  */

macro_rules! EXTEND_RANGE_TABLE {
  ($work_area:expr, $n:expr) => {
    {
      if (($work_area).used + ($n)) * std::mem::size_of::<c_int> () as c_int
	   > ($work_area).allocated
	{
	  extend_range_table_work_area (&raw mut $work_area);
	  if ($work_area).table.is_null ()
	    { return REG_ESPACE; }
	}
    }
  };
}

macro_rules! SET_RANGE_TABLE_WORK_AREA_BIT {
  ($work_area:expr, $bit:expr) => { ($work_area).bits |= ($bit) };
}

/* Set a range (RANGE_START, RANGE_END) to WORK_AREA.  */
macro_rules! SET_RANGE_TABLE_WORK_AREA {
  ($work_area:expr, $range_start:expr, $range_end:expr) => {
    {
      EXTEND_RANGE_TABLE! ($work_area, 2);
      *($work_area).table.offset (($work_area).used as isize) = $range_start;
      ($work_area).used += 1;
      *($work_area).table.offset (($work_area).used as isize) = $range_end;
      ($work_area).used += 1;
    }
  };
}

/* Free allocated memory for WORK_AREA.  */
macro_rules! FREE_RANGE_TABLE_WORK_AREA {
  ($work_area:expr) => {
    {
      if !($work_area).table.is_null ()
	{ xfree (($work_area).table as *mut c_void); }
    }
  };
}

macro_rules! CLEAR_RANGE_TABLE_WORK_USED {
  ($work_area:expr) => { { ($work_area).used = 0; ($work_area).bits = 0; } };
}
macro_rules! RANGE_TABLE_WORK_USED { ($work_area:expr) => { ($work_area).used }; }
macro_rules! RANGE_TABLE_WORK_BITS { ($work_area:expr) => { ($work_area).bits }; }
macro_rules! RANGE_TABLE_WORK_ELT {
  ($work_area:expr, $i:expr) => { *($work_area).table.offset (($i) as isize) };
}

/* Bits used to implement the multibyte-part of the various character classes
   such as [:alnum:] in a charset's range table.  The code currently assumes
   that only the low 16 bits are used.  */
pub const BIT_WORD: c_int = 0x1;
pub const BIT_LOWER: c_int = 0x2;
pub const BIT_PUNCT: c_int = 0x4;
pub const BIT_SPACE: c_int = 0x8;
pub const BIT_UPPER: c_int = 0x10;
pub const BIT_MULTIBYTE: c_int = 0x20;
pub const BIT_ALPHA: c_int = 0x40;
pub const BIT_ALNUM: c_int = 0x80;
pub const BIT_GRAPH: c_int = 0x100;
pub const BIT_PRINT: c_int = 0x200;
pub const BIT_BLANK: c_int = 0x400;

/* Set the bit for character C in a list.  */
// C 1383: `SET_LIST_BIT` names the local `b` of `regex_compile`, so it is defined inside it
// (P-57).

/* Store characters in the range FROM to TO in the bitmap at B (for
   ASCII and unibyte characters) and WORK_AREA (for multibyte
   characters) while translating them and paying attention to the
   continuity of translated characters.

   Implementation note: It is better to implement these fairly big
   macros by a function, but it's not that easy because macros called
   in this macro assume various local variables already declared.  */

/* Both FROM and TO are ASCII characters.  */

// C 1397-1412: `SETUP_ASCII_RANGE` is defined inside `regex_compile` (P-57).

/* Both FROM and TO are unibyte characters (0x80..0xFF).  */

// C 1417-1452: `SETUP_UNIBYTE_RANGE` is defined inside `regex_compile` (P-57).

/* Both FROM and TO are multibyte characters.  */

// C 1457-1487: `SETUP_MULTIBYTE_RANGE` is defined inside `regex_compile` (P-57).

/* Get the next unsigned number in the uncompiled pattern.  */
// C 1490-1509: `GET_INTERVAL_COUNT` is defined inside `regex_compile` (P-57).

/* Parse a character class, i.e. string such as "[:name:]".  *strp
   points to the string to be parsed and limit is length, in bytes, of
   that string.

   If *strp point to a string that begins with "[:name:]", where name is
   a non-empty sequence of lower case letters, *strp will be advanced past the
   closing square bracket and RECC_* constant which maps to the name will be
   returned.  If name is not a valid character class name zero, or RECC_ERROR,
   is returned.

   Otherwise, if *strp doesn't begin with "[:name:]", -1 is returned.

   The function can be used on ASCII and multibyte (UTF-8-encoded) strings.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_wctype_parse (strp: *mut *const u8, mut limit: ptrdiff_t) -> re_wctype_t
{
  let beg = *strp as *const c_char;
  let mut it: *const c_char;

  if limit < 4 || *beg.offset (0) != '[' as c_char || *beg.offset (1) != ':' as c_char
    { return -1; }

  let mut beg = beg.offset (2);  /* skip opening "[:" */
  limit -= 3;  /* opening "[:" and half of closing ":]"; --limit handles rest */
  it = beg;
  while *it.offset (0) != ':' as c_char || *it.offset (1) != ']' as c_char
    {
      limit -= 1;
      if limit == 0
	{ return -1; }
      it = it.offset (1);
    }

  *strp = it.offset (2) as *const u8;

  /* Sort tests in the length=five case by frequency the classes to minimize
     number of times we fail the comparison.  The frequencies of character class
     names used in Emacs sources as of 2016-07-27:

     $ find \\( -name \\*.c -o -name \\*.el \\) -exec grep -h '\\[:[a-z]*:]' {} + |
           sed 's/]/]\\n/g' |grep -o '\\[:[a-z]*:]' |sort |uniq -c |sort -nr
         213 [:alnum:]
         104 [:alpha:]
          62 [:space:]
          39 [:digit:]
          36 [:blank:]
          26 [:word:]
          26 [:upper:]
          21 [:lower:]
          10 [:xdigit:]
          10 [:punct:]
          10 [:ascii:]
           4 [:nonascii:]
           4 [:graph:]
           2 [:print:]
           2 [:cntrl:]
           1 [:ff:]

     If you update this list, consider also updating chain of or'ed conditions
     in execute_charset function.
   */

  match it.offset_from (beg)
    {
      4 =>
	{ if memcmp (beg as *const c_void, c"word".as_ptr () as *const c_void, 4) == 0
	    { return RECC_WORD; } }
      5 => {
	if memcmp (beg as *const c_void, c"alnum".as_ptr () as *const c_void, 5) == 0
	  { return RECC_ALNUM; }
	if memcmp (beg as *const c_void, c"alpha".as_ptr () as *const c_void, 5) == 0
	  { return RECC_ALPHA; }
	if memcmp (beg as *const c_void, c"space".as_ptr () as *const c_void, 5) == 0
	  { return RECC_SPACE; }
	if memcmp (beg as *const c_void, c"digit".as_ptr () as *const c_void, 5) == 0
	  { return RECC_DIGIT; }
	if memcmp (beg as *const c_void, c"blank".as_ptr () as *const c_void, 5) == 0
	  { return RECC_BLANK; }
	if memcmp (beg as *const c_void, c"upper".as_ptr () as *const c_void, 5) == 0
	  { return RECC_UPPER; }
	if memcmp (beg as *const c_void, c"lower".as_ptr () as *const c_void, 5) == 0
	  { return RECC_LOWER; }
	if memcmp (beg as *const c_void, c"punct".as_ptr () as *const c_void, 5) == 0
	  { return RECC_PUNCT; }
	if memcmp (beg as *const c_void, c"ascii".as_ptr () as *const c_void, 5) == 0
	  { return RECC_ASCII; }
	if memcmp (beg as *const c_void, c"graph".as_ptr () as *const c_void, 5) == 0
	  { return RECC_GRAPH; }
	if memcmp (beg as *const c_void, c"print".as_ptr () as *const c_void, 5) == 0
	  { return RECC_PRINT; }
	if memcmp (beg as *const c_void, c"cntrl".as_ptr () as *const c_void, 5) == 0
	  { return RECC_CNTRL; }
      }
      6 =>
	{ if memcmp (beg as *const c_void, c"xdigit".as_ptr () as *const c_void, 6) == 0
	    { return RECC_XDIGIT; } }
      7 =>
	{ if memcmp (beg as *const c_void, c"unibyte".as_ptr () as *const c_void, 7) == 0
	    { return RECC_UNIBYTE; } }
      8 =>
	{ if memcmp (beg as *const c_void, c"nonascii".as_ptr () as *const c_void, 8) == 0
	    { return RECC_NONASCII; } }
      9 =>
	{ if memcmp (beg as *const c_void, c"multibyte".as_ptr () as *const c_void, 9) == 0
	    { return RECC_MULTIBYTE; } }
      _ => {}
    }

  RECC_ERROR
}

/* True if CH is in the char class CC.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_iswctype (ch: c_int, cc: re_wctype_t) -> bool
{
  match cc
    {
      RECC_ALNUM => ISALNUM! (ch) != 0,
      RECC_ALPHA => ISALPHA! (ch) != 0,
      RECC_BLANK => ISBLANK! (ch) != 0,
      RECC_CNTRL => ISCNTRL! (ch) != 0,
      RECC_DIGIT => ISDIGIT! (ch) != 0,
      RECC_GRAPH => ISGRAPH! (ch) != 0,
      RECC_LOWER => ISLOWER! (ch) != 0,
      RECC_PRINT => ISPRINT! (ch) != 0,
      RECC_PUNCT => ISPUNCT! (ch) != 0,
      RECC_SPACE => ISSPACE! (ch) != 0,
      RECC_UPPER => ISUPPER! (ch) != 0,
      RECC_XDIGIT => ISXDIGIT! (ch) != 0,
      RECC_ASCII => IS_REAL_ASCII! (ch) != 0,
      RECC_NONASCII => IS_REAL_ASCII! (ch) == 0,
      RECC_UNIBYTE => ISUNIBYTE! (ch) != 0,
      RECC_MULTIBYTE => ISUNIBYTE! (ch) == 0,
      RECC_WORD => ISWORD! (ch) != 0,
      RECC_ERROR => false,
      _ => { emacs_abort (); }
    }
}

/* Return a bit-pattern to use in the range-table bits to match multibyte
   chars of class CC.  */
unsafe fn re_wctype_to_bit (cc: re_wctype_t) -> c_int
{
  match cc
    {
      RECC_NONASCII | RECC_MULTIBYTE => BIT_MULTIBYTE,
      RECC_ALPHA => BIT_ALPHA,
      RECC_ALNUM => BIT_ALNUM,
      RECC_WORD => BIT_WORD,
      RECC_LOWER => BIT_LOWER,
      RECC_UPPER => BIT_UPPER,
      RECC_PUNCT => BIT_PUNCT,
      RECC_SPACE => BIT_SPACE,
      RECC_GRAPH => BIT_GRAPH,
      RECC_PRINT => BIT_PRINT,
      RECC_BLANK => BIT_BLANK,
      RECC_ASCII | RECC_DIGIT | RECC_XDIGIT | RECC_CNTRL |
      RECC_UNIBYTE | RECC_ERROR => 0,
      _ => { emacs_abort (); }
    }
}

/* Filling in the work area of a range.  */

/* Actually extend the space in WORK_AREA.  */

unsafe fn extend_range_table_work_area (work_area: *mut range_table_work_area)
{
  (*work_area).allocated += 16 * std::mem::size_of::<c_int> () as c_int;
  (*work_area).table = xrealloc ((*work_area).table as *mut c_void,
				 (*work_area).allocated as usize) as *mut c_int;
}

/* regex_compile and helpers.  */

// C 1671: `static bool group_in_compile_stack (compile_stack_type, regnum_t);` - the
// file-local prototype has no Rust counterpart (PORTING.md 0.1-12).

/* Insert the 'jump' from the end of last alternative to "here".
   The space for the jump has already been allocated. */
// C 1675-1679: `FIXUP_ALT_JUMP` is defined inside `regex_compile` (P-57).

/* Return, freeing storage we allocated.  */
// C 1683-1688: `FREE_STACK_RETURN` is defined inside `regex_compile` (P-57).

/* Compile PATTERN (of length SIZE) according to SYNTAX.
   Return a nonzero error code on failure, or zero for success.

   If WHITESPACE_REGEXP is given, use it instead of a space
   character in PATTERN.

   Assume the 'allocated' (and perhaps 'buffer') and 'translate'
   fields are set in BUFP on entry.

   If successful, put results in *BUFP (otherwise the
   contents of *BUFP are undefined):
     'buffer' is the compiled pattern;
     'syntax' is set to SYNTAX;
     'used' is set to the length of the compiled pattern;
     'fastmap_accurate' is false;
     're_nsub' is the number of subexpressions in PATTERN;

   The 'fastmap' field is neither examined nor set.  */

unsafe fn regex_compile (mut pattern: *const re_char, size: ptrdiff_t,
			 posix_backtracking: bool,
			 whitespace_regexp: *const c_char,
			 bufp: *mut re_pattern_buffer) -> reg_errcode_t
{
  /* Fetch characters from PATTERN here.  */
  let mut c: c_int = 0;
  let mut c1: c_int = 0;

  /* Points to the end of the buffer, where we should append.  */
  let mut b: *mut u8;

  /* Keeps track of unclosed groups.  */
  // C declares it and fills its two fields below; Rust needs the whole record initialised
  // first (PORTING.md 0.1-13, the E0381 rule), so the port zeroes it.
  let mut compile_stack: compile_stack_type = compile_stack_type
    { stack: std::ptr::null_mut (), size: 0, avail: 0 };

  /* Points to the current (ending) position in the pattern.  */
  let mut p: *const re_char = pattern;
  let mut pend: *const re_char = pattern.offset (size);

  /* How to translate the characters in the pattern.  */
  let translate: Lisp_Object = (*bufp).translate;

  /* Address of the count-byte of the most recently inserted 'exactn'
     command.  This makes it possible to tell if a new exact-match
     character can be added to that command or if the character requires
     a new 'exactn' command.  */
  let mut pending_exact: *mut u8 = std::ptr::null_mut ();

  /* Address of start of the most recently finished expression.
     This tells, e.g., postfix * where to find the start of its
     operand.  Reset at the beginning of groups and alternatives,
     and after ^ and \` for dusty-deck compatibility.  */
  let mut laststart: *mut u8 = std::ptr::null_mut ();

  /* Address of beginning of regexp, or inside of last group.  */
  let mut begalt: *mut u8;

  /* Place in the uncompiled pattern (i.e., the {) to
     which to go back if the interval is invalid.  */
  let mut beg_interval: *const re_char = std::ptr::null ();

  /* Address of the place where a forward jump should go to the end of
     the containing expression.  Each alternative of an 'or' -- except the
     last -- ends with a forward jump of this sort.  */
  let mut fixup_alt_jump: *mut u8 = std::ptr::null_mut ();

  /* Work area for range table of charset.  */
  // Same as `compile_stack` above (PORTING.md 0.1-13).
  let mut range_table_work: range_table_work_area = range_table_work_area
    { table: std::ptr::null_mut (), allocated: 0, used: 0, bits: 0 };

  /* If the regular expression is multibyte.  */
  let mut multibyte: bool = RE_MULTIBYTE_P! (bufp);

  /* Nonzero if we have pushed down into a subpattern.  */
  let mut in_subpattern: c_int = 0;

  /* These hold the values of p, pattern, and pend from the main
     pattern when we have pushed into a subpattern.  */
  let mut main_p: *const re_char = std::ptr::null ();
  let mut main_pattern: *const re_char = std::ptr::null ();
  let mut main_pend: *const re_char = std::ptr::null ();

  // C 1197: `TRANSLATE (d)` reads the local `translate` of this function (P-57).
  macro_rules! TRANSLATE {
    ($d:expr) => { if !NILP (translate) { RE_TRANSLATE! (translate, $d) } else { $d } };
  }

  macro_rules! COMPILE_STACK_EMPTY { () => { compile_stack.avail == 0 }; }
  macro_rules! COMPILE_STACK_FULL { () => { compile_stack.avail == compile_stack.size }; }

  /* The next available element.  */
  macro_rules! COMPILE_STACK_TOP {
    () => { compile_stack.stack.offset (compile_stack.avail) };
  }

  /* Fetch the next character in the uncompiled pattern, with no
     translation.  */
  macro_rules! PATFETCH {
    ($c:ident) => {
      {
	let mut len: c_int = 0;
	if p == pend { return REG_EEND; }
	$c = RE_STRING_CHAR_AND_LENGTH! (p, len, multibyte);
	p = p.offset (len as isize);
      }
    };
  }

  /* Macros for outputting the compiled pattern into 'buffer'.  */

  /* Ensure at least N more bytes of space in buffer.  */
  macro_rules! GET_BUFFER_SPACE {
    ($n:expr) => {
      if (*bufp).buffer.offset ((*bufp).allocated).offset_from (b) < ($n) as ptrdiff_t
	{
	  EXTEND_BUFFER! ((($n) as ptrdiff_t)
			  - (*bufp).buffer.offset ((*bufp).allocated).offset_from (b));
	}
    };
  }

  /* Ensure one more byte of buffer space and then add C to it.  */
  macro_rules! BUF_PUSH {
    ($c:expr) => {
      {
	GET_BUFFER_SPACE! (1);
	*b = ($c) as u8;
	b = b.offset (1);
      }
    };
  }

  /* Ensure we have two more bytes of buffer space and then append C1 and C2.  */
  macro_rules! BUF_PUSH_2 {
    ($c1:expr, $c2:expr) => {
      {
	GET_BUFFER_SPACE! (2);
	*b = ($c1) as u8;
	b = b.offset (1);
	*b = ($c2) as u8;
	b = b.offset (1);
      }
    };
  }

  /* Like 'STORE_JUMP', but for inserting.  Assume B is the buffer end.  */
  macro_rules! INSERT_JUMP {
    ($op:expr, $loc:expr, $to:expr) => { insert_op1 ($op, $loc, (($to).offset_from ($loc) - 3) as c_int, b) };
  }

  /* Like 'STORE_JUMP2', but for inserting.  Assume B is the buffer end.  */
  macro_rules! INSERT_JUMP2 {
    ($op:expr, $loc:expr, $to:expr, $arg:expr) =>
      { insert_op2 ($op, $loc, (($to).offset_from ($loc) - 3) as c_int, $arg, b) };
  }

  /* Extend the buffer by at least N bytes via realloc and
     reset the pointers that pointed into the old block to point to the
     correct places in the new one.  If extending the buffer results in it
     being larger than MAX_BUF_SIZE, then flag memory exhausted.  */
  macro_rules! EXTEND_BUFFER {
    ($n:expr) => {
      {
	let requested_extension: ptrdiff_t = $n as ptrdiff_t;
	let old_buffer: *mut u8 = (*bufp).buffer;
	if MAX_BUF_SIZE - (*bufp).allocated < requested_extension
	  { return REG_ESIZE; }
	let b_off: ptrdiff_t = b.offset_from (old_buffer);
	let begalt_off: ptrdiff_t = begalt.offset_from (old_buffer);
	let fixup_alt_jump_off: ptrdiff_t =
	  if !fixup_alt_jump.is_null () { fixup_alt_jump.offset_from (old_buffer) } else { -1 };
	let laststart_off: ptrdiff_t =
	  if !laststart.is_null () { laststart.offset_from (old_buffer) } else { -1 };
	let pending_exact_off: ptrdiff_t =
	  if !pending_exact.is_null () { pending_exact.offset_from (old_buffer) } else { -1 };
	(*bufp).buffer = xpalloc ((*bufp).buffer as *mut c_void, &raw mut (*bufp).allocated,
				  requested_extension, MAX_BUF_SIZE, 1) as *mut u8;
	let new_buffer: *mut u8 = (*bufp).buffer;
	b = new_buffer.offset (b_off);
	begalt = new_buffer.offset (begalt_off);
	if 0 <= fixup_alt_jump_off
	  { fixup_alt_jump = new_buffer.offset (fixup_alt_jump_off); }
	if 0 <= laststart_off
	  { laststart = new_buffer.offset (laststart_off); }
	if 0 <= pending_exact_off
	  { pending_exact = new_buffer.offset (pending_exact_off); }
      }
    };
  }

  /* Insert the 'jump' from the end of last alternative to "here".
     The space for the jump has already been allocated. */
  macro_rules! FIXUP_ALT_JUMP {
    () => {
      if !fixup_alt_jump.is_null ()
	{ STORE_JUMP! (jump, fixup_alt_jump, b); }
    };
  }

  /* Return, freeing storage we allocated.  */
  macro_rules! FREE_STACK_RETURN {
    ($value:expr) => {
      {
	FREE_RANGE_TABLE_WORK_AREA! (range_table_work);
	xfree (compile_stack.stack as *mut c_void);
	return $value;
      }
    };
  }

  /* Set the bit for character C in a list.  */
  macro_rules! SET_LIST_BIT {
    ($c:expr) => { *b.offset ((($c) / BYTEWIDTH) as isize) |= (1 << (($c) % BYTEWIDTH)) as u8 };
  }

  /* Store characters in the range FROM to TO in the bitmap at B (for
     ASCII and unibyte characters) and WORK_AREA (for multibyte
     characters) while translating them and paying attention to the
     continuity of translated characters.

     Implementation note: It is better to implement these fairly big
     macros by a function, but it's not that easy because macros called
     in this macro assume various local variables already declared.  */

  /* Both FROM and TO are ASCII characters.  */

  macro_rules! SETUP_ASCII_RANGE {
    ($work_area:expr, $FROM:expr, $TO:expr) => {
      {
	let mut C0: c_int;
	let mut C1: c_int;

	C0 = $FROM;
	while C0 <= $TO
	  {
	    C1 = TRANSLATE! (C0);
	    if ! ASCII_CHAR_P (C1 as isize)
	      {
		SET_RANGE_TABLE_WORK_AREA! ($work_area, C1, C1);
		C1 = RE_CHAR_TO_UNIBYTE! (C1);
		if C1 < 0
		  { C1 = C0; }
	      }
	    SET_LIST_BIT! (C1);
	    C0 += 1;
	  }
      }
    };
  }

  /* Both FROM and TO are unibyte characters (0x80..0xFF).  */

  macro_rules! SETUP_UNIBYTE_RANGE {
    ($work_area:expr, $FROM:expr, $TO:expr) => {
      {
	let mut C0: c_int;
	let mut C1: c_int;
	let mut C2: c_int;
	let mut I: c_int;
	let mut USED: c_int = RANGE_TABLE_WORK_USED! ($work_area);

	C0 = $FROM;
	while C0 <= $TO
	  {
	    C1 = RE_CHAR_TO_MULTIBYTE! (C0);
	    if CHAR_BYTE8_P (C1)
	      { SET_LIST_BIT! (C0); }
	    else
	      {
		C2 = TRANSLATE! (C1);
		if C2 == C1
		    || { C1 = RE_CHAR_TO_UNIBYTE! (C2); C1 < 0 }
		  { C1 = C0; }
		SET_LIST_BIT! (C1);
		I = RANGE_TABLE_WORK_USED! ($work_area) - 2;
		while I >= USED
		  {
		    let from = RANGE_TABLE_WORK_ELT! ($work_area, I);
		    let to = RANGE_TABLE_WORK_ELT! ($work_area, I + 1);

		    if C2 >= from - 1 && C2 <= to + 1
		      {
			if C2 == from - 1
			  { *$work_area.table.offset (I as isize) -= 1; }
			else if C2 == to + 1
			  { *$work_area.table.offset ((I + 1) as isize) += 1; }
			break;
		      }
		    I -= 2;
		  }
		if I < USED
		  { SET_RANGE_TABLE_WORK_AREA! ($work_area, C2, C2); }
	      }
	    C0 += 1;
	  }
      }
    };
  }

  /* Both FROM and TO are multibyte characters.  */

  macro_rules! SETUP_MULTIBYTE_RANGE {
    ($work_area:expr, $FROM:expr, $TO:expr) => {
      {
	let mut C0: c_int;
	let mut C1: c_int;
	let mut C2: c_int;
	let mut I: c_int;
	let mut USED: c_int = RANGE_TABLE_WORK_USED! ($work_area);

	SET_RANGE_TABLE_WORK_AREA! ($work_area, $FROM, $TO);
	C0 = $FROM;
	while C0 <= $TO
	  {
	    C1 = TRANSLATE! (C0);
	    if { C2 = RE_CHAR_TO_UNIBYTE! (C1); C2 >= 0 }
		|| (C1 != C0 && { C2 = RE_CHAR_TO_UNIBYTE! (C0); C2 >= 0 })
	      { SET_LIST_BIT! (C2); }
	    if C1 >= $FROM && C1 <= $TO
	      { C0 += 1; continue; }
	    I = RANGE_TABLE_WORK_USED! ($work_area) - 2;
	    while I >= USED
	      {
		let from = RANGE_TABLE_WORK_ELT! ($work_area, I);
		let to = RANGE_TABLE_WORK_ELT! ($work_area, I + 1);

		if C1 >= from - 1 && C1 <= to + 1
		  {
		    if C1 == from - 1
		      { *$work_area.table.offset (I as isize) -= 1; }
		    else if C1 == to + 1
		      { *$work_area.table.offset ((I + 1) as isize) += 1; }
		    break;
		  }
		I -= 2;
	      }
	    if I < USED
	      { SET_RANGE_TABLE_WORK_AREA! ($work_area, C1, C1); }
	    C0 += 1;
	  }
      }
    };
  }

  /* Get the next unsigned number in the uncompiled pattern.  */
  macro_rules! GET_INTERVAL_COUNT {
    ($num:ident) => {
      {
	if p == pend
	  { FREE_STACK_RETURN! (REG_EBRACE); }
	else
	  {
	    PATFETCH! (c);
	    while '0' as c_int <= c && c <= '9' as c_int
	      {
		if $num < 0
		  { $num = 0; }
		if RE_DUP_MAX / 10 - (if RE_DUP_MAX % 10 < c - '0' as c_int { 1 } else { 0 }) < $num
		  { FREE_STACK_RETURN! (REG_ESIZEBR); }
		$num = $num * 10 + c - '0' as c_int;
		if p == pend
		  { FREE_STACK_RETURN! (REG_EBRACE); }
		PATFETCH! (c);
	      }
	  }
      }
    };
  }

  // C 1770-1779: `#ifdef REGEX_EMACS_DEBUG` (the counter and the pattern dump).
  #[cfg (regex_emacs_debug)]
  {
    regex_emacs_debug += 1;
    DEBUG_PRINT! (c"\nCompiling pattern: ".as_ptr ());
    if regex_emacs_debug > 0
      {
	let mut debug_count: ptrdiff_t = 0;
	while debug_count < size
	  {
	    debug_putchar (stderr, *pattern.offset (debug_count) as c_int);
	    debug_count += 1;
	  }
	putc ('\n' as c_int, stderr);
      }
  }

  /* Initialize the compile stack.  */
  compile_stack.stack = xmalloc ((INIT_COMPILE_STACK_SIZE
				  * std::mem::size_of::<compile_stack_elt_t> () as ptrdiff_t)
				 as usize) as *mut compile_stack_elt_t;
  __lsan_ignore_object (compile_stack.stack as *const c_void);
  compile_stack.size = INIT_COMPILE_STACK_SIZE;
  compile_stack.avail = 0;

  range_table_work.table = std::ptr::null_mut ();
  range_table_work.allocated = 0;

  /* Initialize the pattern buffer.  */
  set_fastmap_accurate_ (bufp, false);
  set_used_syntax_ (bufp, false);

  /* Set 'used' to zero, so that if we return an error, the pattern
     printer (for debugging) will think there's no pattern.  We reset it
     at the end.  */
  (*bufp).used = 0;

  (*bufp).re_nsub = 0;

  if (*bufp).allocated == 0
    {
      /* This loses if BUFP->buffer is bogus, but that is the user's
	 responsibility.  */
      (*bufp).buffer = xrealloc ((*bufp).buffer as *mut c_void, INIT_BUF_SIZE as usize) as *mut u8;
      (*bufp).allocated = INIT_BUF_SIZE;
    }

  begalt = (*bufp).buffer;
  b = (*bufp).buffer;

  /* Loop through the uncompiled pattern until we're at the end.  */
  loop
    {
      if p == pend
	{
	  /* If this is the end of an included regexp,
	     pop back to the main regexp and try again.  */
	  if in_subpattern != 0
	    {
	      in_subpattern = 0;
	      p = main_pattern;
	      p = main_p;
	      pend = main_pend;
	      continue;
	    }
	  /* If this is the end of the main regexp, we are done.  */
	  break;
	}

      PATFETCH! (c);

      /* C's `goto normal_char` and `goto unfetch_interval` have no Rust counterpart; the
	 port carries the two destinations as the flags below, which is where C's labels
	 are (PORTING.md 2.2, the bytecode.rs/tparam.rs shape).  */
      let mut goto_normal_char = false;

      match c as u8 as char
	{
	  ' ' => {
	    let mut p1: *const re_char = p;

	    /* If there's no special whitespace regexp, treat
	       spaces normally.  And don't try to do this recursively.  */
	    if whitespace_regexp.is_null () || in_subpattern != 0
	      { goto_normal_char = true; }
	    else
	      {
		/* Peek past following spaces.  */
		while p1 != pend
		  {
		    if *p1 != ' ' as re_char
		      { break; }
		    p1 = p1.offset (1);
		  }
		/* If the spaces are followed by a repetition op,
		   treat them normally.  */
		if p1 != pend
		    && (*p1 == '*' as re_char || *p1 == '+' as re_char || *p1 == '?' as re_char
			|| (*p1 == '\\' as re_char && p1.offset (1) != pend
			    && *p1.offset (1) == '{' as re_char))
		  { goto_normal_char = true; }
		else
		  {
		    /* Replace the spaces with the whitespace regexp.  */
		    in_subpattern = 1;
		    main_p = p1;
		    main_pend = pend;
		    main_pattern = pattern;
		    pattern = whitespace_regexp as *const re_char;
		    p = pattern;
		    pend = p.offset (strlen (whitespace_regexp) as isize);
		  }
	      }
	  }

	  '^' => {
	    if ! (p == pattern.offset (1) || at_begline_loc_p (pattern, p))
	      { goto_normal_char = true; }
	    else
	      {
		/* Special case for compatibility: postfix ops after ^ become
		   literals.  */
		laststart = std::ptr::null_mut ();
		BUF_PUSH! (begline);
	      }
	  }

	  '$' => {
	    if ! (p == pend || at_endline_loc_p (p, pend))
	      { goto_normal_char = true; }
	    else
	      {
		laststart = b;
		BUF_PUSH! (endline);
	      }
	  }

	  '+' | '?' | '*' => {
	    /* If there is no previous pattern...  */
	    if laststart.is_null ()
	      { goto_normal_char = true; }
	    else
	      {
		/* 1 means zero (many) matches is allowed.  */
		let mut zero_times_ok = false;
		let mut many_times_ok = false;
		let mut greedy = true;

		/* If there is a sequence of repetition chars, collapse it
		   down to just one (the right one).  We can't combine
		   interval operators with these because of, e.g., 'a{2}*',
		   which should only match an even number of 'a's.  */

		loop
		  {
		    if c == '?' as c_int && (zero_times_ok || many_times_ok)
		      { greedy = false; }
		    else
		      {
			zero_times_ok |= c != '+' as c_int;
			many_times_ok |= c != '?' as c_int;
		      }

		    if ! (p < pend && (*p == '*' as re_char || *p == '+' as re_char
				       || *p == '?' as re_char))
		      { break; }
		    /* If we get here, we found another repeat character.  */
		    c = *p as c_int; p = p.offset (1);
		   }

		/* Star, etc. applied to an empty pattern is equivalent
		   to an empty pattern.  */
		if laststart == b
		  { }
		else
		  {
		    /* Now we know whether or not zero matches is allowed
		       and also whether or not two or more matches is allowed.  */
		    if greedy
		      {
			if many_times_ok
			  {
			    let simple = skip_one_char (laststart) == b;
			    let mut startoffset: ptrdiff_t = 0;
			    let ofj: re_opcode_t =
			      /* Check if the loop can match the empty string.  */
			      if simple || !analyze_first (bufp, laststart, b, std::ptr::null_mut ())
			      { on_failure_jump } else { on_failure_jump_loop };
			    eassert! (skip_one_char (laststart) <= b);

			    if !zero_times_ok && simple
			      { /* Since simple * loops can be made faster by using
				   on_failure_keep_string_jump, we turn P+ into PP*
	                           if P is simple.
				   We can't use `top: <BODY>; OFJS exit; J top; exit:`
				   because the OFJS needs to be at the beginning
				   so we can replace
				       top: OFJS exit; <BODY>; J top; exit
				   with
				       OFKSJ exit; loop: <BODY>; J loop; exit
				   i.e. a single OFJ at the beginning of the loop
				   rather than once per iteration.  */
				let mut p1: *mut u8;
				let mut p2: *mut u8;
				startoffset = b.offset_from (laststart);
				GET_BUFFER_SPACE! (startoffset);
				p1 = b; p2 = laststart;
				/* We presume that the code skipped
				   by `skip_one_char` is position-independent.  */
				while p2 < p1
				  { *b = *p2; b = b.offset (1); p2 = p2.offset (1); }
				zero_times_ok = true;
			      }

			    GET_BUFFER_SPACE! (6);
			    if !zero_times_ok
			      /* A + loop.  */
			      { STORE_JUMP! (ofj, b, b.offset (6)); }
			    else
			      /* Simple * loops can use on_failure_keep_string_jump
				 depending on what follows.  But since we don't know
				 that yet, we leave the decision up to
				 on_failure_jump_smart.  */
			      { INSERT_JUMP! (if simple { on_failure_jump_smart } else { ofj },
					      laststart.offset (startoffset), b.offset (6)); }
			    b = b.offset (3);
			    STORE_JUMP! (jump, b, laststart.offset (startoffset as isize));
			    b = b.offset (3);
			  }
			else
			  {
			    /* A simple ? pattern.  */
			    eassert! (zero_times_ok);
			    GET_BUFFER_SPACE! (3);
			    INSERT_JUMP! (on_failure_jump, laststart, b.offset (3));
			    b = b.offset (3);
			  }
		      }
		    else		/* not greedy */
		      { /* I wish the greedy and non-greedy cases could be merged.  */

			GET_BUFFER_SPACE! (7); /* We might use less.  */
			if many_times_ok
			  {
			    let emptyp = analyze_first (bufp, laststart, b, std::ptr::null_mut ());

			    /* The non-greedy multiple match looks like
			       a repeat..until: we only need a conditional jump
			       at the end of the loop.  */
			    if emptyp { BUF_PUSH! (no_op); }
			    STORE_JUMP! (if emptyp { on_failure_jump_nastyloop }
					 else { on_failure_jump }, b, laststart);
			    b = b.offset (3);
			    if zero_times_ok
			      {
				/* The repeat...until naturally matches one or more.
				   To also match zero times, we need to first jump to
				   the end of the loop (its conditional jump).  */
				INSERT_JUMP! (jump, laststart, b);
				b = b.offset (3);
			      }
			  }
			else
			  {
			    /* non-greedy a?? */
			    INSERT_JUMP! (jump, laststart, b.offset (3));
			    b = b.offset (3);
			    INSERT_JUMP! (on_failure_jump, laststart, laststart.offset (6));
			    b = b.offset (3);
			  }
		      }
		  }
		pending_exact = std::ptr::null_mut ();
	      }
	  }

	  '.' => {
	    laststart = b;
	    BUF_PUSH! (anychar);
	  }

	  '[' => {
	    let mut p1: *const re_char;

	    CLEAR_RANGE_TABLE_WORK_USED! (range_table_work);

	    if p == pend { FREE_STACK_RETURN! (REG_EBRACK); }

	    /* Ensure that we have enough space to push a charset: the
	       opcode, the length count, and the bitset; 34 bytes in all.  */
	    GET_BUFFER_SPACE! (34);

	    laststart = b;

	    /* Test '*p == '^' twice, instead of using an if
	       statement, so we need only one BUF_PUSH.  */
	    BUF_PUSH! (if *p == '^' as re_char { charset_not } else { charset });
	    if *p == '^' as re_char
	      { p = p.offset (1); }

	    /* Remember the first position in the bracket expression.  */
	    p1 = p;

	    /* Push the number of bytes in the bitmap.  */
	    BUF_PUSH! ((1 << BYTEWIDTH) / BYTEWIDTH);

	    /* Clear the whole map.  */
	    memset (b as *mut c_void, 0, ((1 << BYTEWIDTH) / BYTEWIDTH) as usize);

	    /* Read in characters and ranges, setting map bits.  */
	    loop
	      {
		let p2: *const u8 = p;
		let mut ch: c_int = 0;

		if p == pend { FREE_STACK_RETURN! (REG_EBRACK); }

		/* See if we're at the beginning of a possible character
		   class.  */
		let cc: re_wctype_t = re_wctype_parse (&raw mut p, pend.offset_from (p));
		if cc != -1
		  {
		    if cc == 0
		      { FREE_STACK_RETURN! (REG_ECTYPE); }

		    if p == pend
		      { FREE_STACK_RETURN! (REG_EBRACK); }

		    /* Most character classes in a multibyte match just set
		       a flag.  Exceptions are is_blank, is_digit, is_cntrl, and
		       is_xdigit, since they can only match ASCII characters.
		       We don't need to handle them for multibyte.  */

		    c = 0;
		    while c < 0x80
		      {
			if re_iswctype (c, cc)
			  {
			    SET_LIST_BIT! (c);
			    c1 = TRANSLATE! (c);
			    if c1 != c
			      {
				if ASCII_CHAR_P (c1 as isize)
				  { SET_LIST_BIT! (c1); }
				else
				  {
				    c1 = RE_CHAR_TO_UNIBYTE! (c1);
				    if c1 >= 0
				      { SET_LIST_BIT! (c1); }
				  }
			      }
			  }
			c += 1;
		      }
		    SET_RANGE_TABLE_WORK_AREA_BIT! (range_table_work, re_wctype_to_bit (cc));

		    /* In most cases the matching rule for char classes only
		       uses the syntax table for multibyte chars, so that the
		       content of the syntax-table is not hardcoded in the
		       range_table.  SPACE and WORD are the two exceptions.  */
		    if (1 << cc) & ((1 << RECC_SPACE) | (1 << RECC_WORD)) != 0
		      { set_used_syntax_ (bufp, true); }

		    /* Repeat the loop. */
		    continue;
		  }

		/* Don't translate yet.  The range TRANSLATE(X..Y) cannot
		   always be determined from TRANSLATE(X) and TRANSLATE(Y)
		   So the translation is done later in a loop.  Example:
		   (let ((case-fold-search t)) (string-match "[A-_]" "A"))  */
		PATFETCH! (c);

		/* Could be the end of the bracket expression.  If it's
		   not (i.e., when the bracket expression is '[]' so
		   far), the ']' character bit gets set way below.  */
		if c == ']' as c_int && p2 != p1
		  { break; }

		if p < pend && *p == '-' as re_char && *p.offset (1) != ']' as re_char
		  {

		    /* Discard the '-'. */
		    PATFETCH! (c1);

		    /* Fetch the character which ends the range. */
		    PATFETCH! (c1);

		    if CHAR_BYTE8_P (c1)
			&& ! ASCII_CHAR_P (c as isize) && ! CHAR_BYTE8_P (c)
		      /* Treat the range from a multibyte character to
			 raw-byte character as empty.  */
		      { c = c1 + 1; }
		  }
		else
		  /* Range from C to C. */
		  { c1 = c; }

		if c <= c1
		  {
		    if c < 128
		      {
			ch = std::cmp::min (127, c1);
			SETUP_ASCII_RANGE! (range_table_work, c, ch);
			c = ch + 1;
			if CHAR_BYTE8_P (c1)
			  { c = BYTE8_TO_CHAR (128); }
		      }
                    if c <= c1
                      {
                        if CHAR_BYTE8_P (c)
                          {
                            c = CHAR_TO_BYTE8 (c);
                            c1 = CHAR_TO_BYTE8 (c1);
                            while c <= c1
                              { SET_LIST_BIT! (c); c += 1; }
                          }
                        else if multibyte
                          { SETUP_MULTIBYTE_RANGE! (range_table_work, c, c1); }
                        else
                          { SETUP_UNIBYTE_RANGE! (range_table_work, c, c1); }
                      }
		  }
	      }

	    /* Discard any (non)matching list bytes that are all 0 at the
	       end of the map.  Decrease the map-length byte too.  */
	    while *b.offset (-1) as c_int > 0
		  && *b.offset ((*b.offset (-1) as isize) - 1) == 0
	      { *b.offset (-1) -= 1; }
	    b = b.offset (*b.offset (-1) as isize);

	    /* Build real range table from work area.  */
	    if RANGE_TABLE_WORK_USED! (range_table_work) != 0
		|| RANGE_TABLE_WORK_BITS! (range_table_work) != 0
	      {
		let mut i: c_int;
		let used: c_int = RANGE_TABLE_WORK_USED! (range_table_work);

		/* Allocate space for COUNT + RANGE_TABLE.  Needs two
		   bytes for flags, two for COUNT, and three bytes for
		   each character.  */
		GET_BUFFER_SPACE! (4 + used * 3);

		/* Indicate the existence of range table.  */
		*laststart.offset (1) |= 0x80;

		/* Store the character class flag bits into the range table.  */
		*b = (RANGE_TABLE_WORK_BITS! (range_table_work) & 0xff) as u8;
		b = b.offset (1);
		*b = (RANGE_TABLE_WORK_BITS! (range_table_work) >> 8) as u8;
		b = b.offset (1);

		STORE_NUMBER_AND_INCR! (b, used / 2);
		i = 0;
		while i < used
		  {
		    let _ = STORE_CHARACTER_AND_INCR! (b, RANGE_TABLE_WORK_ELT! (range_table_work, i));
		    i += 1;
		  }
	      }
	  }

	  '\\' => {
	    if p == pend { FREE_STACK_RETURN! (REG_EESCAPE); }

	    /* Do not translate the character after the \, so that we can
	       distinguish, e.g., \B from \b, even if we normally would
	       translate, e.g., B to b.  */
	    PATFETCH! (c);

	    let mut goto_unfetch_interval = false;
	    match c as u8 as char
	      {
		'(' => {
		  let mut shy = false;
		  let mut regnum: regnum_t = 0;
		  if p.offset (1) < pend
		    {
		      /* Look for a special (?...) construct */
		      if *p == '?' as re_char
			{
			  PATFETCH! (c); /* Gobble up the '?'.  */
			  while !shy
			    {
			      PATFETCH! (c);
			      match c as u8 as char
				{
				  ':' => { shy = true; }
				  '0' => {
				    /* An explicitly specified regnum must start
				       with non-0. */
				    if regnum == 0
				      { FREE_STACK_RETURN! (REG_BADPAT); }
				    if { let m = regnum.overflowing_mul (10); let over = m.1;
					 regnum = m.0;
					 over || { let a = regnum.overflowing_add (c - '0' as c_int);
						   regnum = a.0; a.1 } }
				      { FREE_STACK_RETURN! (REG_ESIZE); }
				  }
				  '1' | '2' | '3' | '4' |
				  '5' | '6' | '7' | '8' | '9' => {
				    if { let m = regnum.overflowing_mul (10); let over = m.1;
					 regnum = m.0;
					 over || { let a = regnum.overflowing_add (c - '0' as c_int);
						   regnum = a.0; a.1 } }
				      { FREE_STACK_RETURN! (REG_ESIZE); }
				  }
				  _ => {
				    /* Only (?:...) is supported right now. */
				    FREE_STACK_RETURN! (REG_BADPAT);
				  }
				}
			    }
			}
		    }

		  if !shy
		    {
		      (*bufp).re_nsub += 1;
		      regnum = (*bufp).re_nsub as regnum_t;
		    }
		  else if regnum != 0
		    { /* It's actually not shy, but explicitly numbered.  */
		      shy = false;
		      if regnum > (*bufp).re_nsub as regnum_t
			{ (*bufp).re_nsub = regnum as ptrdiff_t; }
		      else if regnum > (*bufp).re_nsub as regnum_t
			     /* Ideally, we'd want to check that the specified
				group can't have matched (i.e. all subgroups
				using the same regnum are in other branches of
				OR patterns), but we don't currently keep track
				of enough info to do that easily.  */
			     || group_in_compile_stack (compile_stack, regnum)
			{ FREE_STACK_RETURN! (REG_BADPAT); }
		    }
		  else
		    /* It's really shy.  */
		    { regnum = - ((*bufp).re_nsub as regnum_t); }

		  if COMPILE_STACK_FULL! ()
		    {
		      compile_stack.stack
			= xpalloc (compile_stack.stack as *mut c_void,
				   &raw mut compile_stack.size,
				   1, -1, std::mem::size_of::<compile_stack_elt_t> () as ptrdiff_t)
			    as *mut compile_stack_elt_t;
		    }

		  /* These are the values to restore when we hit end of this
		     group.  They are all relative offsets, so that if the
		     whole pattern moves because of realloc, they will still
		     be valid.  */
		  (*COMPILE_STACK_TOP! ()).begalt_offset = begalt.offset_from ((*bufp).buffer) as pattern_offset_t;
		  (*COMPILE_STACK_TOP! ()).fixup_alt_jump
		    = if !fixup_alt_jump.is_null ()
		      { fixup_alt_jump.offset_from ((*bufp).buffer) as pattern_offset_t + 1 } else { 0 };
		  (*COMPILE_STACK_TOP! ()).laststart_offset = b.offset_from ((*bufp).buffer) as pattern_offset_t;
		  (*COMPILE_STACK_TOP! ()).regnum = regnum;

		  /* Do not push a start_memory for groups beyond the last one
		     we can represent in the compiled pattern.  */
		  if regnum <= MAX_REGNUM && regnum > 0
		    { BUF_PUSH_2! (start_memory, regnum); }

		  compile_stack.avail += 1;

		  fixup_alt_jump = std::ptr::null_mut ();
		  laststart = std::ptr::null_mut ();
		  begalt = b;
		  /* If we've reached MAX_REGNUM groups, then this open
		     won't actually generate any code, so we'll have to
		     clear pending_exact explicitly.  */
		  pending_exact = std::ptr::null_mut ();
		}

		')' => {
		  if COMPILE_STACK_EMPTY! ()
		    { FREE_STACK_RETURN! (REG_ERPAREN); }

		  FIXUP_ALT_JUMP! ();

		  /* See similar code for backslashed left paren above.  */
		  if COMPILE_STACK_EMPTY! ()
		    { FREE_STACK_RETURN! (REG_ERPAREN); }

		  /* Since we just checked for an empty stack above, this
		     "can't happen".  */
		  eassert! (compile_stack.avail != 0);
		  {
		    /* We don't just want to restore into 'regnum', because
		       later groups should continue to be numbered higher,
		       as in '(ab)c(de)' -- the second group is #2.  */
		    let regnum: regnum_t;

		    compile_stack.avail -= 1;
		    begalt = (*bufp).buffer.offset ((*COMPILE_STACK_TOP! ()).begalt_offset as isize);
		    fixup_alt_jump
		      = if (*COMPILE_STACK_TOP! ()).fixup_alt_jump != 0
			{ (*bufp).buffer.offset ((*COMPILE_STACK_TOP! ()).fixup_alt_jump as isize).offset (-1) }
			else { std::ptr::null_mut () };
		    laststart = (*bufp).buffer.offset ((*COMPILE_STACK_TOP! ()).laststart_offset as isize);
		    regnum = (*COMPILE_STACK_TOP! ()).regnum;
		    /* If we've reached MAX_REGNUM groups, then this open
		       won't actually generate any code, so we'll have to
		       clear pending_exact explicitly.  */
		    pending_exact = std::ptr::null_mut ();

		    /* We're at the end of the group, so now we know how many
		       groups were inside this one.  */
		    if regnum <= MAX_REGNUM && regnum > 0
		      { BUF_PUSH_2! (stop_memory, regnum); }
		  }
		}

		'|' => {					/* '\|'.  */
		  /* Insert before the previous alternative a jump which
		     jumps to this alternative if the former fails.  */
		  GET_BUFFER_SPACE! (3);
		  INSERT_JUMP! (on_failure_jump, begalt, b.offset (6));
		  pending_exact = std::ptr::null_mut ();
		  b = b.offset (3);

		  /* The alternative before this one has a jump after it
		     which gets executed if it gets matched.  Adjust that
		     jump so it will jump to this alternative's analogous
		     jump (put in below, which in turn will jump to the next
		     (if any) alternative's such jump, etc.).  The last such
		     jump jumps to the correct final destination.  A picture:
			  _____ _____
			  |   | |   |
			  |   v |   v
			A | B	| C

		     If we are at B, then fixup_alt_jump right now points to a
		     three-byte space after A.  We'll put in the jump, set
		     fixup_alt_jump to right after B, and leave behind three
		     bytes which we'll fill in when we get to after C.  */

		  FIXUP_ALT_JUMP! ();

		  /* Mark and leave space for a jump after this alternative,
		     to be filled in later either by next alternative or
		     when know we're at the end of a series of alternatives.  */
		  fixup_alt_jump = b;
		  GET_BUFFER_SPACE! (3);
		  b = b.offset (3);

		  laststart = std::ptr::null_mut ();
		  begalt = b;
		}

		'{' => {
		  /* At least (most) this many matches must be made.  */
		  let mut lower_bound: c_int = 0;
		  let mut upper_bound: c_int = -1;

		  beg_interval = p;

		  GET_INTERVAL_COUNT! (lower_bound);

		  if c == ',' as c_int
		    { GET_INTERVAL_COUNT! (upper_bound); }
		  else
		    /* Interval such as '{1}' => match exactly once. */
		    { upper_bound = lower_bound; }

		  if lower_bound < 0
		      || (0 <= upper_bound && upper_bound < lower_bound)
		      || c != '\\' as c_int
		    { FREE_STACK_RETURN! (REG_BADBR); }
		  if p == pend
		    { FREE_STACK_RETURN! (REG_EESCAPE); }
		  if { let bad = *p != '}' as re_char; p = p.offset (1); bad }
		    { FREE_STACK_RETURN! (REG_BADBR); }

		  /* We just parsed a valid interval.  */

		  /* If it's invalid to have no preceding re.  */
		  if laststart.is_null ()
		    { goto_unfetch_interval = true; }
		  else if upper_bound == 0
		    /* If the upper bound is zero, just drop the sub pattern
		       altogether.  */
		    { b = laststart; }
		  else if lower_bound == 1 && upper_bound == 1
		    /* Just match it once: nothing to do here.  */
		    { }
		  /* Otherwise, we have a nontrivial interval.  When
		     we're all done, the pattern will look like:
		     set_number_at <jump count> <upper bound>
		     set_number_at <succeed_n count> <lower bound>
		     succeed_n <after jump addr> <succeed_n count>
		     <body of loop>
		     jump_n <succeed_n addr> <jump count>
		     (The upper bound and 'jump_n' are omitted if
		     'upper_bound' is 1, though.)  */
		  else
		    { /* If the upper bound is > 1, we need to insert
			 more at the end of the loop.  */
		      let nbytes: c_int = if upper_bound < 0 { 3 }
					  else if upper_bound > 1 { 5 } else { 0 };
		      let mut startoffset: c_int = 0;

		      GET_BUFFER_SPACE! (20); /* We might use less.  */

		      if lower_bound == 0
			{
			  /* A succeed_n that starts with 0 is really
			     a simple on_failure_jump_loop.  */
			  INSERT_JUMP! (on_failure_jump_loop, laststart,
				       b.offset ((3 + nbytes) as isize));
			  b = b.offset (3);
			}
		      else
			{
			  /* Initialize lower bound of the 'succeed_n', even
			     though it will be set during matching by its
			     attendant 'set_number_at' (inserted next),
			     because 're_compile_fastmap' needs to know.
			     Jump to the 'jump_n' we might insert below.  */
			  INSERT_JUMP2! (succeed_n, laststart,
					b.offset ((5 + nbytes) as isize),
					lower_bound);
			  b = b.offset (5);

			  /* Code to initialize the lower bound.  Insert
			     before the 'succeed_n'.  The '5' is the last two
			     bytes of this 'set_number_at', plus 3 bytes of
			     the following 'succeed_n'.  */
			  insert_op2 (set_number_at, laststart, 5,
				      lower_bound, b);
			  b = b.offset (5);
			  startoffset += 5;
			}

		      if upper_bound < 0
			{
			  /* A negative upper bound stands for infinity,
			     in which case it degenerates to a plain jump.  */
			  STORE_JUMP! (jump, b, laststart.offset (startoffset as isize));
			  b = b.offset (3);
			}
		      else if upper_bound > 1
			{ /* More than one repetition is allowed, so
			     append a backward jump to the 'succeed_n'
			     that starts this interval.

			     When we've reached this during matching,
			     we'll have matched the interval once, so
			     jump back only 'upper_bound - 1' times.  */
			  STORE_JUMP2! (jump_n, b, laststart.offset (startoffset as isize),
				       upper_bound - 1);
			  b = b.offset (5);

			  /* The location we want to set is the second
			     parameter of the 'jump_n'; that is 'b-2' as
			     an absolute address.  'laststart' will be
			     the 'set_number_at' we're about to insert;
			     'laststart+3' the number to set, the source
			     for the relative address.  But we are
			     inserting into the middle of the pattern --
			     so everything is getting moved up by 5.
			     Conclusion: (b - 2) - (laststart + 3) + 5,
			     i.e., b - laststart.

			     Insert this at the beginning of the loop
			     so that if we fail during matching, we'll
			     reinitialize the bounds.  */
			  insert_op2 (set_number_at, laststart, b.offset_from (laststart) as c_int,
				      upper_bound - 1, b);
			  b = b.offset (5);
			}
		    }
		  pending_exact = std::ptr::null_mut ();
		  beg_interval = std::ptr::null ();
		}

		'=' => {
		  laststart = b;
		  BUF_PUSH! (at_dot);
		}

		's' => {
		  laststart = b;
		  PATFETCH! (c);
		  BUF_PUSH_2! (syntaxspec, syntax_spec_code[c as usize] as c_int);
		}

		'S' => {
		  laststart = b;
		  PATFETCH! (c);
		  BUF_PUSH_2! (notsyntaxspec, syntax_spec_code[c as usize] as c_int);
		}

		'c' => {
		  laststart = b;
		  PATFETCH! (c);
		  BUF_PUSH_2! (categoryspec, c);
		}

		'C' => {
		  laststart = b;
		  PATFETCH! (c);
		  BUF_PUSH_2! (notcategoryspec, c);
		}

		'w' => {
		  laststart = b;
		  BUF_PUSH_2! (syntaxspec, Sword);
		}

		'W' => {
		  laststart = b;
		  BUF_PUSH_2! (notsyntaxspec, Sword);
		}

		'<' => {
		  laststart = b;
		  BUF_PUSH! (wordbeg);
		}

		'>' => {
		  laststart = b;
		  BUF_PUSH! (wordend);
		}

		'_' => {
		  laststart = b;
		  PATFETCH! (c);
		  if c == '<' as c_int
		    { BUF_PUSH! (symbeg); }
		  else if c == '>' as c_int
		    { BUF_PUSH! (symend); }
		  else
		    { FREE_STACK_RETURN! (REG_BADPAT); }
		}

		'b' => {
		  laststart = b;
		  BUF_PUSH! (wordbound);
		}

		'B' => {
		  laststart = b;
		  BUF_PUSH! (notwordbound);
		}

		'`' => {
		  /* Special case for compatibility: postfix ops after \` become
		     literals, as for ^ (see above).  */
		  laststart = std::ptr::null_mut ();
		  BUF_PUSH! (begbuf);
		}

		'\'' => {
		  laststart = b;
		  BUF_PUSH! (endbuf);
		}

		'1' | '2' | '3' | '4' | '5' |
		'6' | '7' | '8' | '9' => {
		  let reg: regnum_t = c - '0' as c_int;

		  if reg > (*bufp).re_nsub as regnum_t || reg < 1
		      /* Can't back reference to a subexp before its end.  */
		      || group_in_compile_stack (compile_stack, reg)
		    { FREE_STACK_RETURN! (REG_ESUBREG); }

		  laststart = b;
		  BUF_PUSH_2! (duplicate, reg);
		}

		_ => {
		  /* You might think it would be useful for \ to mean
		     not to translate; but if we don't translate it
		     it will never match anything.  */
		  goto_normal_char = true;
		}
	      }

	    if goto_unfetch_interval
	      {
		/* If an invalid interval, match the characters as literals.  */
		eassert! (!beg_interval.is_null ());
		p = beg_interval;
		beg_interval = std::ptr::null ();
		eassert! (p > pattern && *p.offset (-1) == '\\' as re_char);
		c = '{' as c_int;
		goto_normal_char = true;
	      }
	  }

	  _ => {
	    /* Expects the character in C.  */
	    goto_normal_char = true;
	  }
	}

      if goto_normal_char
	{
	  /* If no exactn currently being built.  */
	  if pending_exact.is_null ()

	      /* If last exactn not at current position.  */
	      || pending_exact.offset (*pending_exact as isize).offset (1) != b

	      /* Only one byte follows the exactn for the count.  */
	      || *pending_exact as c_int >= (1 << BYTEWIDTH) - MAX_MULTIBYTE_LENGTH

	      /* If followed by a repetition operator.  */
	      || (p != pend
		  && (*p == '*' as re_char || *p == '+' as re_char || *p == '?' as re_char))
	      || (p.offset (1) < pend && *p == '\\' as re_char && *p.offset (1) == '{' as re_char)
	    {
	      /* Start building a new exactn.  */

	      laststart = b;

	      BUF_PUSH_2! (exactn, 0);
	      pending_exact = b.offset (-1);
	    }

	  GET_BUFFER_SPACE! (MAX_MULTIBYTE_LENGTH);
	  {
	    let mut len: c_int;

	    if multibyte
	      {
		c = TRANSLATE! (c);
		len = CHAR_STRING (c, b);
		b = b.offset (len as isize);
	      }
	    else
	      {
		c1 = RE_CHAR_TO_MULTIBYTE! (c);
		if ! CHAR_BYTE8_P (c1)
		  {
		    let c2: c_int = TRANSLATE! (c1);

		    if c1 != c2
		      { c1 = RE_CHAR_TO_UNIBYTE! (c2); if c1 >= 0 { c = c1; } }
		  }
		*b = c as u8;
		b = b.offset (1);
		len = 1;
	      }
	    *pending_exact = (*pending_exact).wrapping_add (len as u8);
	  }
	}
    } /* while p != pend */

  /* Through the pattern now.  */

  FIXUP_ALT_JUMP! ();

  if !COMPILE_STACK_EMPTY! ()
    { FREE_STACK_RETURN! (REG_EPAREN); }

  /* If we don't want backtracking, force success
     the first time we reach the end of the compiled pattern.  */
  if !posix_backtracking
    { BUF_PUSH! (succeed); }

  /* Success; set the length of the buffer.  */
  (*bufp).used = b.offset_from ((*bufp).buffer);

  // C 2691-2699: `#ifdef REGEX_EMACS_DEBUG`.
  #[cfg (regex_emacs_debug)]
  {
    if regex_emacs_debug > 0
      {
	re_compile_fastmap (bufp);
	DEBUG_PRINT! (c"\nCompiled pattern:\n".as_ptr ());
	print_compiled_pattern (stderr, bufp);
      }
    regex_emacs_debug -= 1;
  }

  FREE_STACK_RETURN! (REG_NOERROR);

} /* regex_compile */

/* Subroutines for 'regex_compile'.  */

/* Store OP at LOC followed by two-byte integer parameter ARG.  */

unsafe fn store_op1 (op: re_opcode_t, loc: *mut u8, arg: c_int)
{
  *loc = op as u8;
  STORE_NUMBER (loc.offset (1), arg);
}

/* Like 'store_op1', but for two two-byte parameters ARG1 and ARG2.  */

unsafe fn store_op2 (op: re_opcode_t, loc: *mut u8, arg1: c_int, arg2: c_int)
{
  *loc = op as u8;
  STORE_NUMBER (loc.offset (1), arg1);
  STORE_NUMBER (loc.offset (3), arg2);
}

/* Copy the bytes from LOC to END to open up three bytes of space at LOC
   for OP followed by two-byte integer parameter ARG.  */

unsafe fn insert_op1 (op: re_opcode_t, loc: *mut u8, arg: c_int, end: *mut u8)
{
  let mut pfrom: *mut u8 = end;
  let mut pto: *mut u8 = end.offset (3);

  while pfrom != loc
    {
      pto = pto.offset (-1);
      pfrom = pfrom.offset (-1);
      *pto = *pfrom;
    }

  store_op1 (op, loc, arg);
}

/* Like 'insert_op1', but for two two-byte parameters ARG1 and ARG2.  */

unsafe fn insert_op2 (op: re_opcode_t, loc: *mut u8, arg1: c_int, arg2: c_int,
		      end: *mut u8)
{
  let mut pfrom: *mut u8 = end;
  let mut pto: *mut u8 = end.offset (5);

  while pfrom != loc
    {
      pto = pto.offset (-1);
      pfrom = pfrom.offset (-1);
      *pto = *pfrom;
    }

  store_op2 (op, loc, arg1, arg2);
}

/* P points to just after a ^ in PATTERN.  Return true if that ^ comes
   after an alternative or a begin-subexpression.  Assume there is at
   least one character before the ^.  */

unsafe fn at_begline_loc_p (pattern: *const re_char, mut p: *const re_char) -> bool
{
  let mut prev: *const re_char = p.offset (-2);

  match *prev as u8 as char
    {
      '(' => { } /* After a subexpression.  */
      '|' => { } /* After an alternative.  */

      ':' => { /* After a shy subexpression.  */
	/* Skip over optional regnum.  */
	while prev > pattern && '0' as re_char <= *prev.offset (-1)
	      && *prev.offset (-1) <= '9' as re_char
	  { prev = prev.offset (-1); }

	if ! (prev > pattern.offset (1) && *prev.offset (-1) == '?' as re_char
	      && *prev.offset (-2) == '(' as re_char)
	  { return false; }
	prev = prev.offset (-2);
      }

      _ => { return false; }
    }

  /* Count the number of preceding backslashes.  */
  p = prev;
  while prev > pattern && *prev.offset (-1) == '\\' as re_char
    { prev = prev.offset (-1); }
  (p.offset_from (prev) & 1) != 0
}

/* The dual of at_begline_loc_p.  This one is for $.  Assume there is
   at least one character after the $, i.e., 'P < PEND'.  */

unsafe fn at_endline_loc_p (p: *const re_char, pend: *const re_char) -> bool
{
  /* Before a subexpression or an alternative?  */
  *p == '\\' as re_char && p.offset (1) < pend
    && (*p.offset (1) == ')' as re_char || *p.offset (1) == '|' as re_char)
}

/* Returns true if REGNUM is in one of COMPILE_STACK's elements and
   false if it's not.  */

unsafe fn group_in_compile_stack (compile_stack: compile_stack_type, regnum: regnum_t) -> bool
{
  let mut this_element: ptrdiff_t;

  this_element = compile_stack.avail - 1;
  while this_element >= 0
    {
      if (*compile_stack.stack.offset (this_element)).regnum == regnum
	{ return true; }
      this_element -= 1;
    }

  false
}

/* Iterate through all the char-matching operations directly reachable from P.
   This is the inner loop of `forall_firstchar`, which see.
   LOOP_BEG..LOOP_END delimit the currently "block" of code (we assume
   the code is made of syntactically nested loops).
   LOOP_END is blindly assumed to be "safe".
   To guarantee termination, at each iteration, either LOOP_BEG should
   get bigger, or it should stay the same and P should get bigger.  */
unsafe fn forall_firstchar_1 (p: *const re_char, pend: *const re_char,
			      loop_beg: *const re_char, loop_end: *const re_char,
			      f: unsafe extern "C" fn (*const re_char, *mut c_void) -> bool,
			      arg: *mut c_void) -> bool
{
  let mut p = p;
  let mut loop_beg = loop_beg;
  let mut loop_end = loop_end;
  eassert! (p >= loop_beg);
  eassert! (p <= loop_end);
  eassert! (!pend.is_null ());

  loop
    {
      let mut newp1: *const re_char;
      let mut newp2: *const re_char;
      let mut tmp: *const re_char;
      let p_orig: *const re_char = p;
      let mut offset: c_int = 0;

      if p == pend
        /* When the regexp is under construction, this can happen and
           means we're jumping past the end of what we've already constructed.
           After that, this should happen only for POSIX matches.
           FIXME: We should probably introduce a 'posix_succeed'
           sentinel like we have for non-POSIX matches.  */
        { return false; }
      else if p == loop_end
        { return true; }
      else if p > loop_end
        {
	  // C 2858-2860: `#if ENABLE_CHECKING`.
	  #[cfg (enable_checking)]
	  {
	    fprintf (stderr, c"FORALL_FIRSTCHAR: Broken assumption1!!\n".as_ptr ());
	  }
          return false; /* FIXME: Broken assumption about the code shape.  */
        }
      else
	{
	  /* C's `do_twoway_jump:`/`do_jump:` labels: 2 runs the two-way jump's
	     checks and then `do_jump`'s, 1 runs only the latter (PORTING.md 2.2).  */
	  let mut stage: c_int = 0;
	  newp1 = std::ptr::null ();
	  newp2 = std::ptr::null ();
	  tmp = std::ptr::null ();
	  match *p as re_opcode_t
          {
            no_op => { p = p.offset (1); continue; }

            /* Cases which stop the iteration.  */
            succeed |
            exactn |
            charset |
            charset_not |
            anychar |
            syntaxspec |
            notsyntaxspec |
            categoryspec |
            notcategoryspec => { return f (p, arg); }

            /* Cases which may match the empty string.  */
            at_dot |
            begbuf |
            wordbound |
            notwordbound |
            begline |
            endline |
            endbuf |
            wordbeg |
            wordend |
            symbeg |
            symend => {
              if f (p, arg)
		{ return true; }
              p = p.offset (1);
              continue;
	    }

            jump |
            jump_n => {
	      newp1 = extract_address (p.offset (1));
	      if newp1 > p
		{ /* Forward jump, boring.  */
		  p = newp1;
		  continue;
		}
	      match *newp1 as re_opcode_t
		{
		  on_failure_jump |
		  on_failure_keep_string_jump |
		  on_failure_jump_nastyloop |
		  on_failure_jump_loop |
		  on_failure_jump_smart |
		  succeed_n => {
		    newp2 = extract_address (newp1.offset (1));
		    stage = 2;
		  }
		  _ => {
		    newp2 = loop_end; /* "Safe" choice.  */
		    stage = 1;
		  }
		}
	    }

	    on_failure_jump |
	    on_failure_keep_string_jump |
	    on_failure_jump_nastyloop |
	    on_failure_jump_loop |
	    on_failure_jump_smart => {
	      newp1 = extract_address (p.offset (1));
	      newp2 = p.offset (3);
	      /* For `+` loops, we often have an `on_failure_jump` that skips
                 forward over a subsequent `jump`.  Recognize this pattern
                 since that subsequent `jump` is the one that jumps to the
                 loop-entry.  */
	      if *newp2 as re_opcode_t == jump
		{
		  let p3 = extract_address (newp2.offset (1));
		  /* Only recognize this pattern if one of the two destinations
                     is going forward, otherwise we'll fall into the pessimistic
                     "Both destinations go backward" below.
                     This is important if the `jump` at newp2 is the end of an
                     outer loop while the `on_failure_jump` is the end of an
                     inner loop.  */
		  if p3 > p_orig || newp1 > p_orig
		    { newp2 = p3; }
		}
	      stage = 2;
	    }

            succeed_n => {
	      newp1 = extract_address (p.offset (1));
	      newp2 = p.offset (5);      /* Skip the two bytes containing the count.  */
	      stage = 2;
	    }

            set_number_at => {
              offset = extract_number (p.offset (1));
              DEBUG_STATEMENT! (eassert! (extract_number (p.offset (3)) != 0));
              /* If we're setting the counter of an immediately following
                 `succeed_n`, then this next execution of `succeed_n` will do
                 nothing but decrement its counter and "fall through".
                 So we do the fall through here to avoid considering the
                 "on failure" part of the `succeed_n` which should only be
                 considered when coming from the `jump(_n)` at the end of
                 the loop.  */
              p = p.offset (if offset == 5 && *p.offset (5) as re_opcode_t == succeed_n
			    { 10 } else { 5 });
              continue;
	    }

            start_memory |
            stop_memory => {
              p = p.offset (2);
              continue;
	    }

            /* This could match the empty string, so we may need to continue,
               but in most cases, this can match "anything", so we should
               return `false` unless told otherwise.  */
            duplicate => {
              if !f (p, arg)
		{ return false; }
              p = p.offset (2);
              continue;
	    }

            _ => {
	      // C 3022-3026: `#if ENABLE_CHECKING`.
	      #[cfg (enable_checking)]
	      {
		fprintf (stderr,
			 c"FORALL_FIRSTCHAR: Impossible case %d at position %p <= %p!!\n".as_ptr (),
			 *p as c_int, p, pend);
	      }
              emacs_abort (); /* We have listed all the cases.  */
	    }
          }

	  /* do_twoway_jump: */
	  if stage == 2
	    {
	      /* We have to check that both destinations are safe.
		 Arrange for `newp1` to be the smaller of the two.  */
	      if newp1 > newp2
		{ tmp = newp1; newp1 = newp2; newp2 = tmp; }

	      if newp2 <= p_orig /* Both destinations go backward!  */
		{
		  // C 2953-2955: `#if ENABLE_CHECKING`.
		  #[cfg (enable_checking)]
		  {
		    fprintf (stderr, c"FORALL_FIRSTCHAR: Broken assumption2!!\n".as_ptr ());
		  }
		  return false;
		}

	      if !forall_firstchar_1 (newp2, pend, loop_beg, loop_end, f, arg)
		{ return false; }
	    }

	  /* do_jump: */
	  eassert! (newp2 <= loop_end);
	  if newp1 <= p_orig
	    {
	      if newp1 < loop_beg
		{
		  // C 2968-2970: `#if ENABLE_CHECKING`.
		  #[cfg (enable_checking)]
		  {
		    fprintf (stderr, c"FORALL_FIRSTCHAR: Broken assumption3!!\n".as_ptr ());
		  }
		  return false;
		}
	      else if newp1 == loop_beg
		/* If we jump backward to the entry point of the current loop
		   it means it's a zero-length cycle through that loop;
		   this cycle itself does not break safety.  */
		{ return true; }
	      else
		/* We jump backward to a new loop, nested within the current
		   one. `newp1` is the entry point and `newp2` the exit of
		   that inner loop.  */
		/* `p` gets smaller, but termination is still ensured because
		   `loop_beg` gets bigger. */
		{ loop_beg = newp1; loop_end = newp2; }
	    }
	  p = newp1;
	  continue;
	}
    }
}

/* Iterate through all the char-matching operations directly reachable from P.
   Return true if P is "safe", meaning that PEND cannot be reached directly
   from P and all calls to F returned true.
   Return false if PEND *may* be directly reachable from P or if one of
   the calls to F returned false.
   PEND can be NULL (and hence never reachable).

   Call `F (POS, ARG)` for every POS directly reachable from P,
   before reaching PEND, where POS is the position of a char-matching
   operation (`exactn`, `charset`, ...).

   For operations that match the empty string (`wordbeg`, ...), if F
   returns true we stop going down that path immediately but if it returns
   false we don't consider it as a failure and we simply look for the
   next char-matching operations on that path.
   For `duplicate`, it is the reverse: a false is an immediate failure
   whereas a true just lets the analysis continue with the rest of the path.

   This function can be used while building the bytecode (in which case
   you should pass NULL for bufp), but if so, P and PEND need to delimit
   a valid block such that there is not jump to a location outside
   of [P...PEND].  */
unsafe fn forall_firstchar (bufp: *mut re_pattern_buffer, p: *const re_char,
			    pend: *const re_char,
			    f: unsafe extern "C" fn (*const re_char, *mut c_void) -> bool,
			    arg: *mut c_void) -> bool
{
  eassert! (if !bufp.is_null () { (*bufp).used != 0 } else { !pend.is_null () });
  forall_firstchar_1 (p, pend,
		      if !bufp.is_null () { (*bufp).buffer.offset (-1) } else { p },
		      if !bufp.is_null () { (*bufp).buffer.offset ((*bufp).used).offset (1) }
		      else { pend },
		      f, arg)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct anafirst_data
{
  pub multibyte: bool,
  pub fastmap: *mut c_char,
  pub match_any_multibyte_characters: bool,
}
const _: () = core::assert! (std::mem::size_of::<anafirst_data> () == 24);
const _: () = core::assert! (std::mem::offset_of! (anafirst_data, multibyte) == 0);
const _: () = core::assert! (std::mem::offset_of! (anafirst_data, fastmap) == 8);
const _: () = core::assert! (std::mem::offset_of! (anafirst_data, match_any_multibyte_characters) == 16);

unsafe extern "C" fn analyze_first_fastmap (mut p: *const re_char, arg: *mut c_void) -> bool
{
  let data: *mut anafirst_data = arg as *mut anafirst_data;

  let mut j: c_int;
  let mut k: c_int;
  let mut nbits: c_int;
  let mut not: bool;

  match *p as re_opcode_t
    {
      succeed => { return false; }

      duplicate => {
        /* If the first character has to match a backreference, that means
           that the group was empty (since it already matched).  Since this
           is the only case that interests us here, we can assume that the
           backreference must match the empty string and we need to
           build the fastmap from the rest of the path.  */
        return true;
      }

      /* Following are the cases which match a character.  These end
         with 'break'.  */

      exactn => {
        p = p.offset (1);
        /* If multibyte is nonzero, the first byte of each
	   character is an ASCII or a leading code.  Otherwise,
	   each byte is a character.  Thus, this works in both
	   cases. */
        *(*data).fastmap.offset (*p.offset (1) as isize) = 1;
        if (*data).multibyte
	  {
	    /* Cover the case of matching a raw char in a
	       multibyte regexp against unibyte.	*/
	    if CHAR_BYTE8_HEAD_P (*p.offset (1) as c_int)
	      {
		*(*data).fastmap.offset (
		  CHAR_TO_BYTE8 (STRING_CHAR (p.offset (1))) as isize) = 1;
	      }
	  }
        else
	  {
	    /* For the case of matching this unibyte regex
	       against multibyte, we must set a leading code of
	       the corresponding multibyte character.  */
	    let c: c_int = RE_CHAR_TO_MULTIBYTE! (*p.offset (1) as c_int);

	    *(*data).fastmap.offset (CHAR_LEADING_CODE (c) as isize) = 1;
	  }
        return true;
      }

      anychar => {
        /* We could put all the chars except for \n (and maybe \0)
	   but we don't bother since it is generally not worth it.  */
        return false;
      }

      charset_not |
      charset => {
        if *p as re_opcode_t == charset_not
	  {
	    /* Chars beyond end of bitmap are possible matches.  */
	    j = CHARSET_BITMAP_SIZE! (p) * BYTEWIDTH;
	    while j < (1 << BYTEWIDTH)
	      {
		*(*data).fastmap.offset (j as isize) = 1;
		j += 1;
	      }
	  }
        /* C's `FALLTHROUGH;` sends the charset_not arm into this text.  */
	not = *p as re_opcode_t == charset_not;
	nbits = CHARSET_BITMAP_SIZE! (p) * BYTEWIDTH;
	p = p.offset (2);
	j = 0;
	while j < nbits
	  {
	    if (((*p.offset ((j / BYTEWIDTH) as isize) as c_int)
		 & (1 << (j % BYTEWIDTH))) != 0) != not
	      { *(*data).fastmap.offset (j as isize) = 1; }
	    j += 1;
	  }

	/* To match raw bytes (in the 80..ff range) against multibyte
	   strings, add their leading bytes to the fastmap.  */
	j = 0x80;
	while j < nbits
	  {
	    if (((*p.offset ((j / BYTEWIDTH) as isize) as c_int)
		 & (1 << (j % BYTEWIDTH))) != 0) != not
	      { *(*data).fastmap.offset (CHAR_LEADING_CODE (BYTE8_TO_CHAR (j)) as isize) = 1; }
	    j += 1;
	  }

	if /* Any leading code can possibly start a character
	     which doesn't match the specified set of characters.  */
	  not
	  ||
	  /* If we can match a character class, we can match any
	     multibyte characters.  */
	  (CHARSET_RANGE_TABLE_EXISTS_P! (p.offset (-2))
	   && CHARSET_RANGE_TABLE_BITS! (p.offset (-2)) != 0)

	  {
	    if !(*data).match_any_multibyte_characters
	      {
		j = MIN_MULTIBYTE_LEADING_CODE;
		while j <= MAX_MULTIBYTE_LEADING_CODE
		  {
		    *(*data).fastmap.offset (j as isize) = 1;
		    j += 1;
		  }
		(*data).match_any_multibyte_characters = true;
	      }
	  }

	else if !not && CHARSET_RANGE_TABLE_EXISTS_P! (p.offset (-2))
		 && (*data).match_any_multibyte_characters == false
	  {
	    /* Set fastmap[I] to 1 where I is a leading code of each
	       multibyte character in the range table. */
	    let mut c: c_int;
	    let mut count: c_int;
	    let mut lc1: c_uchar;
	    let mut lc2: c_uchar;

	    /* Make P points the range table.  '+ 2' is to skip flag
	       bits for a character class.  */
	    p = p.offset (CHARSET_BITMAP_SIZE! (p.offset (-2)) as isize + 2);

	    /* Extract the number of ranges in range table into COUNT.  */
	    EXTRACT_NUMBER_AND_INCR! (count, p);
	    while count > 0
	      {
		/* Extract the start and end of each range.  */
		EXTRACT_CHARACTER! (c, p);
		lc1 = CHAR_LEADING_CODE (c) as c_uchar;
		p = p.offset (3);
		EXTRACT_CHARACTER! (c, p);
		lc2 = CHAR_LEADING_CODE (c) as c_uchar;
		j = lc1 as c_int;
		while j <= lc2 as c_int
		  {
		    *(*data).fastmap.offset (j as isize) = 1;
		    j += 1;
		  }
		count -= 1;
		p = p.offset (3);
	      }
	  }
        return true;
      }

      syntaxspec |
      notsyntaxspec => {
        /* This match depends on text properties.  These end with
	   aborting optimizations.  */
        return false;
      }

      categoryspec |
      notcategoryspec => {
        not = *p as re_opcode_t == notcategoryspec;
        p = p.offset (1);
        k = *p as c_int; p = p.offset (1);
        j = 1 << BYTEWIDTH;
        while j >= 0
	  {
	    if CHAR_HAS_CATEGORY (j, k) != not
	      { *(*data).fastmap.offset (j as isize) = 1; }
	    j -= 1;
	  }

        /* Any leading code can possibly start a character which
	   has or doesn't has the_malloc_fn specified category.  */
        if !(*data).match_any_multibyte_characters
	  {
	    j = MIN_MULTIBYTE_LEADING_CODE;
	    while j <= MAX_MULTIBYTE_LEADING_CODE
	      {
		*(*data).fastmap.offset (j as isize) = 1;
		j += 1;
	      }
	    (*data).match_any_multibyte_characters = true;
	  }
        return true;
      }

      at_dot |
      begbuf |
      wordbound |
      notwordbound |
      begline |
      endline |
      endbuf |
      wordbeg |
      wordend |
      symbeg |
      symend => {
        /* This false doesn't mean failure but rather "not succeeded yet".  */
        return false;
      }

      _ => {
	// C 3236-3238: `#if ENABLE_CHECKING`.
	#[cfg (enable_checking)]
	{
	  emacs_abort (); /* We have listed all the cases.  */
	}
        return false;
      }
    }
}

unsafe extern "C" fn analyze_first_null (p: *const re_char, _arg: *mut c_void) -> bool
{
  match *p as re_opcode_t
    {
      succeed => {
        /* This is safe: we can't reach `pend` at all from here.  */
        return true;
      }

      duplicate => {
        /* Either `duplicate` ends up matching a non-empty string, in which
           case we're good, or it matches the empty string, in which case we
           need to continue checking the rest of this path, which is exactly
           what returning `true` does, here.  */
        return true;
      }

      exactn |
      anychar |
      charset_not |
      charset |
      syntaxspec |
      notsyntaxspec |
      categoryspec |
      notcategoryspec => { return true; }

      at_dot |
      begbuf |
      wordbound |
      notwordbound |
      begline |
      endline |
      endbuf |
      wordbeg |
      wordend |
      symbeg |
      symend => {
        /* This false doesn't mean failure but rather "not succeeded yet".  */
        return false;
      }

      _ => {
	// C 3283-3285: `#if ENABLE_CHECKING`.
	#[cfg (enable_checking)]
	{
	  emacs_abort (); /* We have listed all the cases.  */
	}
        return false;
      }
    }
}

/* analyze_first.
   If fastmap is non-NULL, go through the pattern and fill fastmap
   with all the possible leading chars.  If fastmap is NULL, don't
   bother filling it up (obviously) and only return whether the
   pattern could potentially match the empty string.

   Return false if p matches at least one char before reaching pend.
   Return true  if p..pend might match the empty string
                or if fastmap was not updated accurately.  */

unsafe fn analyze_first (bufp: *mut re_pattern_buffer,
			 p: *const re_char, pend: *const re_char, fastmap: *mut c_char) -> bool
{
  eassert! (!pend.is_null ());
  let mut data: anafirst_data = anafirst_data
    { multibyte: if !bufp.is_null () { multibyte_ (bufp) } else { false },
      fastmap: fastmap,
      match_any_multibyte_characters: false };
  let safe = forall_firstchar (if (*bufp).used != 0 { bufp } else { std::ptr::null_mut () },
			       p, pend,
                               if !fastmap.is_null () { analyze_first_fastmap }
                               else { analyze_first_null },
                               &raw mut data as *mut c_void);
  !safe
}

/* Compute a fastmap for the compiled pattern in BUFP.
   A fastmap records which of the (1 << BYTEWIDTH) possible
   characters can start a string that matches the pattern.  This fastmap
   is used by re_search to skip quickly over impossible starting points.

   Character codes above (1 << BYTEWIDTH) are not represented in the
   fastmap, but the leading codes are represented.  Thus, the fastmap
   indicates which character sets could start a match.

   The caller must supply the address of a (1 << BYTEWIDTH)-byte data
   area as BUFP->fastmap.

   Set the 'fastmap', 'fastmap_accurate', and 'can_be_null' fields in
   the pattern buffer.  */

unsafe fn re_compile_fastmap (bufp: *mut re_pattern_buffer)
{
  let fastmap: *mut c_char = (*bufp).fastmap;

  eassert! (!fastmap.is_null () && !(*bufp).buffer.is_null ());

  memset (fastmap as *mut c_void, 0, 1 << BYTEWIDTH);  /* Assume nothing's valid.  */

  /* FIXME: Is the following assignment correct even when ANALYSIS < 0?  */
  set_fastmap_accurate_ (bufp, true);	    /* It will be when we're done.  */

  set_can_be_null_ (bufp, analyze_first (bufp, (*bufp).buffer as *const re_char,
				         (*bufp).buffer.offset ((*bufp).used)
					   as *const re_char, fastmap));
} /* re_compile_fastmap */

/* Set REGS to hold NUM_REGS registers, storing them in STARTS and
   ENDS.  Subsequent matches using PATTERN_BUFFER and REGS will use
   this memory for recording register information.  STARTS and ENDS
   must be allocated using the malloc library routine, and must each
   be at least NUM_REGS * sizeof (ptrdiff_t) bytes long.

   If NUM_REGS == 0, then subsequent matches should allocate their own
   register data.

   Unless this function is called, the first search or match using
   PATTERN_BUFFER will allocate its own register data, without
   freeing the old data.  */

/// C's `src/config.h` 3642-3663 renames `re_set_registers` to `rpl_re_set_registers` (gnulib's
/// replacement scheme), so the reference object exports `_rpl_re_set_registers` and this
/// definition must carry C's own symbol name (the `emacs_fclose` shape, status 3.68).
#[unsafe(export_name = "rpl_re_set_registers")]
pub unsafe extern "C" fn re_set_registers (bufp: *mut re_pattern_buffer,
					   regs: *mut re_registers,
					   num_regs: ptrdiff_t,
					   starts: *mut ptrdiff_t, ends: *mut ptrdiff_t)
{
  if num_regs != 0
    {
      set_regs_allocated_ (bufp, REGS_REALLOCATE as u8);
      (*regs).num_regs = num_regs;
      (*regs).start = starts;
      (*regs).end = ends;
    }
  else
    {
      set_regs_allocated_ (bufp, REGS_UNALLOCATED as u8);
      (*regs).num_regs = 0;
      (*regs).start = std::ptr::null_mut ();
      (*regs).end = std::ptr::null_mut ();
    }
}

/* Searching routines.  */

/* Like re_search_2, below, but only one string is specified, and
   doesn't let you say where to stop matching.  */

/// C's `src/config.h` 3642-3663 renames `re_search` to `rpl_re_search` (gnulib's
/// replacement scheme), so the reference object exports `_rpl_re_search` and this
/// definition must carry C's own symbol name (the `emacs_fclose` shape, status 3.68).
#[unsafe(export_name = "rpl_re_search")]
pub unsafe extern "C" fn re_search (bufp: *mut re_pattern_buffer, string: *const c_char,
				    size: ptrdiff_t, startpos: ptrdiff_t,
				    range: ptrdiff_t, regs: *mut re_registers) -> ptrdiff_t
{
  re_search_2 (bufp, std::ptr::null (), 0, string, size, startpos, range,
	      regs, size)
}

/* Address of POS in the concatenation of virtual string.  */
// C 3393-3394: `POS_ADDR_VSTRING` names `size1`/`string1`/`string2`, so it is defined inside
// `re_search_2` and (where C uses it) `re_match_2_internal` (P-57).
/* Using the compiled pattern in BUFP->buffer, first tries to match the
   virtual concatenation of STRING1 and STRING2, starting first at index
   STARTPOS, then at STARTPOS + 1, and so on.

   STRING1 and STRING2 have length SIZE1 and SIZE2, respectively.

   RANGE is how far to scan while trying to match.  RANGE = 0 means try
   only at STARTPOS; in general, the last start tried is STARTPOS +
   RANGE.

   In REGS, return the indices of the virtual concatenation of STRING1
   and STRING2 that matched the entire BUFP->buffer and its contained
   subexpressions.

   Do not consider matching one past the index STOP in the virtual
   concatenation of STRING1 and STRING2.

   Return either the position in the strings at which the match was
   found, -1 if no match, or -2 if error (such as failure
   stack overflow).  */

/// C's `src/config.h` 3642-3663 renames `re_search_2` to `rpl_re_search_2` (gnulib's
/// replacement scheme), so the reference object exports `_rpl_re_search_2` and this
/// definition must carry C's own symbol name (the `emacs_fclose` shape, status 3.68).
#[unsafe(export_name = "rpl_re_search_2")]
pub unsafe extern "C" fn re_search_2 (bufp: *mut re_pattern_buffer, str1: *const c_char,
				      size1: ptrdiff_t, str2: *const c_char,
				      size2: ptrdiff_t,
				      mut startpos: ptrdiff_t, mut range: ptrdiff_t,
				      regs: *mut re_registers, stop: ptrdiff_t) -> ptrdiff_t
{
  let mut val: ptrdiff_t = 0;
  let string1: *const re_char = str1 as *const re_char;
  let string2: *const re_char = str2 as *const re_char;
  let fastmap: *mut c_char = (*bufp).fastmap;
  let translate: Lisp_Object = (*bufp).translate;
  let total_size: ptrdiff_t = size1 + size2;
  let mut endpos: ptrdiff_t = startpos + range;
  let anchored_start: bool;
  /* Nonzero if we are searching multibyte string.  */
  let multibyte: bool = RE_TARGET_MULTIBYTE_P! (bufp);

  // C 1197/3393: `TRANSLATE` reads this function's own `translate`, and
  // `POS_ADDR_VSTRING` names its `size1`/`string1`/`string2` (P-57).
  macro_rules! TRANSLATE {
    ($d:expr) => { if !NILP (translate) { RE_TRANSLATE! (translate, $d) } else { $d } };
  }
  macro_rules! POS_ADDR_VSTRING {
    ($POS:expr) => {
      (if ($POS) >= size1 { string2.offset (-size1) } else { string1 }).offset ($POS)
    };
  }

  /* Check for out-of-range STARTPOS.  */
  if startpos < 0 || startpos > total_size
    { return -1; }

  /* Fix up RANGE if it might eventually take us outside
     the virtual concatenation of STRING1 and STRING2.
     Make sure we won't move STARTPOS below 0 or above TOTAL_SIZE.  */
  if endpos < 0
    { range = 0 - startpos; }
  else if endpos > total_size
    { range = total_size - startpos; }

  /* If the search isn't to be a backwards one, don't waste time in a
     search for a pattern anchored at beginning of buffer.  */
  if (*bufp).used > 0 && *(*bufp).buffer as re_opcode_t == begbuf && range > 0
    {
      if startpos > 0
	{ return -1; }
      else
	{ range = 0; }
    }

  /* In a forward search for something that starts with \=.
     don't keep searching past point.  */
  if (*bufp).used > 0 && *(*bufp).buffer as re_opcode_t == at_dot && range > 0
    {
      range = PT_BYTE! () - BEGV_BYTE! () - startpos;
      if range < 0
	{ return -1; }
    }

  /* Update the fastmap now if not correct already.  */
  if !fastmap.is_null () && !fastmap_accurate_ (bufp)
    { re_compile_fastmap (bufp); }

  /* See whether the pattern is anchored.  */
  anchored_start = *(*bufp).buffer as re_opcode_t == begline;

  RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (re_match_object, startpos);

  /* Loop through the string, looking for a place to start matching.  */
  loop
    {
      /* C's `advance:` label: the two `goto advance` sites leave this block.  */
      'advance: {
      /* If the pattern is anchored,
	 skip quickly past places we cannot match.
	 Don't bother to treat startpos == 0 specially
	 because that case doesn't repeat.  */
      if anchored_start && startpos > 0
	{
	  if ! ((if startpos <= size1 { *string1.offset (startpos - 1) }
		 else { *string2.offset (startpos - size1 - 1) })
		 == '\n' as re_char)
	    { break 'advance; }
	}

      /* If a fastmap is supplied, skip quickly over characters that
	 cannot be the start of a match.  If the pattern can match the
	 null string, however, we don't need to skip characters; we want
	 the first null string.  */
      if !fastmap.is_null () && startpos < total_size && !can_be_null_ (bufp)
	{
	  let mut d: *const re_char;
	  let mut buf_ch: c_int;

	  d = POS_ADDR_VSTRING! (startpos);

	  if range > 0	/* Searching forwards.  */
	    {
	      let irange: ptrdiff_t = range;
	      let mut lim: ptrdiff_t = 0;
	      let mut leave = false;

	      if startpos < size1 && startpos + range >= size1
		{ lim = range - (size1 - startpos); }

	      /* Written out as an if-else to avoid testing 'translate'
		 inside the loop.  */
	      if !NILP (translate)
		{
		  if multibyte
		    {
		      while range > lim
			{
			  let mut buf_charlen: c_int = 0;

			  buf_ch = string_char_and_length (d, &raw mut buf_charlen);
			  buf_ch = RE_TRANSLATE! (translate, buf_ch);
			  if *fastmap.offset (CHAR_LEADING_CODE (buf_ch) as isize) != 0
			    { break; }

			  range -= buf_charlen as ptrdiff_t;
			  d = d.offset (buf_charlen as isize);
			}
		    }
		  else
		    {
		      while range > lim
			{
			  buf_ch = *d as c_int;
			  let mut ch: c_int = RE_CHAR_TO_MULTIBYTE! (buf_ch);
			  let translated: c_int = RE_TRANSLATE! (translate, ch);
			  if translated != ch
			      && { ch = RE_CHAR_TO_UNIBYTE! (translated); ch >= 0 }
			    { buf_ch = ch; }
			  if *fastmap.offset (buf_ch as isize) != 0
			    { break; }
			  d = d.offset (1);
			  range -= 1;
			}
		    }
		}
	      else
		{
		  if multibyte
		    {
		      while range > lim
			{
			  let mut buf_charlen: c_int = 0;

			  buf_ch = string_char_and_length (d, &raw mut buf_charlen);
			  if *fastmap.offset (CHAR_LEADING_CODE (buf_ch) as isize) != 0
			    { break; }
			  range -= buf_charlen as ptrdiff_t;
			  d = d.offset (buf_charlen as isize);
			}
		    }
		  else
		    {
		      while range > lim && *fastmap.offset (*d as isize) == 0
			{
			  d = d.offset (1);
			  range -= 1;
			}
		    }
		}
	      startpos += irange - range;
	      let _ = leave;
	    }
	  else				/* Searching backwards.  */
	    {
	      if multibyte
		{
		  buf_ch = STRING_CHAR (d);
		  buf_ch = TRANSLATE! (buf_ch);
		  if *fastmap.offset (CHAR_LEADING_CODE (buf_ch) as isize) == 0
		    { break 'advance; }
		}
	      else
		{
		  buf_ch = *d as c_int;
		  let mut ch: c_int = RE_CHAR_TO_MULTIBYTE! (buf_ch);
		  let translated: c_int = TRANSLATE! (ch);
		  if translated != ch
		      && { ch = RE_CHAR_TO_UNIBYTE! (translated); ch >= 0 }
		    { buf_ch = ch; }
		  if *fastmap.offset (TRANSLATE! (buf_ch) as isize) == 0
		    { break 'advance; }
		}
	    }
	}

      /* If can't match the null string, and that's all we have left, fail.  */
      if range >= 0 && startpos == total_size && !fastmap.is_null ()
	  && !can_be_null_ (bufp)
	{ return -1; }

      val = re_match_2_internal (bufp, string1, size1, string2, size2,
				 startpos, regs, stop);

      if val >= 0
	{ return startpos; }

      if val == -2
	{ return -2; }
      }

    /* advance: */
      if range == 0
	{ break; }
      else if range > 0
	{
	  /* Update STARTPOS to the next character boundary.  */
	  if multibyte
	    {
	      let p: *const re_char = POS_ADDR_VSTRING! (startpos);
	      let len: c_int = BYTES_BY_CHAR_HEAD (*p as c_int);

	      range -= len as ptrdiff_t;
	      if range < 0
		{ break; }
	      startpos += len as ptrdiff_t;
	    }
	  else
	    {
	      range -= 1;
	      startpos += 1;
	    }
	}
      else
	{
	  range += 1;
	  startpos -= 1;

	  /* Update STARTPOS to the previous character boundary.  */
	  if multibyte
	    {
	      let p: *const re_char = POS_ADDR_VSTRING! (startpos).offset (1);
	      let len: c_int = raw_prev_char_len (p);

	      range += len as ptrdiff_t - 1;
	      if range > 0
		{ break; }
	      startpos -= len as ptrdiff_t - 1;
	    }
	}
    }
  -1
} /* re_search_2 */

/* Declarations and macros for re_match_2.  */

// C 3643-3644: `static bool bcmp_translate (re_char *, re_char *, ptrdiff_t, Lisp_Object,
// bool);` - the file-local prototype has no Rust counterpart (PORTING.md 0.1-12); the
// definition keeps C's position.

/* This converts PTR, a pointer into one of the search strings 'string1'
   and 'string2' into an offset from the beginning of that string.  */
// C 3648-3651: `POINTER_TO_OFFSET` is defined inside `re_match_2_internal` (P-57).
/* Call before fetching a character with *d.  This switches over to
   string2 if necessary.
   `reset' is executed before backtracking if there are no more characters.
   Check re_match_2_internal for a discussion of why end_match_2 might
   not be within string2 (but be equal to end_match_1 instead).  */
// C 3658-3670: `PREFETCH` is defined inside `re_match_2_internal` (it names `d`, `dend`,
// `end_match_2` and `string2`, and its `goto fail` is that function's label; P-57).

/* Call before fetching a char with *d if you already checked other limits.
   This is meant for use in lookahead operations like wordend, etc..
   where we might need to look at parts of the string that might be
   outside of the LIMITs (i.e past 'stop').  */
// C 3676-3681: `PREFETCH_NOLIMIT` is defined inside `re_match_2_internal` (P-57).

/* Test if at very beginning or at very end of the virtual concatenation
   of STRING1 and STRING2.  If only one string, it's STRING2.  */
// C 3685-3686: `AT_STRINGS_BEG`/`AT_STRINGS_END` are defined inside `re_match_2_internal`
// (P-57).
/* Disabled due to a compiler bug -- see comment at case wordbound */

/* The comment at case wordbound is following one, but we don't use
   AT_WORD_BOUNDARY anymore to support multibyte form.

   The DEC Alpha C compiler 3.x generates incorrect code for the
   test	 WORDCHAR_P (d - 1) != WORDCHAR_P (d)  in the expansion of
   AT_WORD_BOUNDARY, so this code is disabled.  Expanding the
   macro and introducing temporary variables works around the bug.  */

// C 3698-3713: `#if 0` - dead in every configuration, so it is recorded here only (the
// `#if 0` rule of P-08, the intervals.c/search.c precedent); the port writes nothing.

/* Optimization routines.  */

/* If the operation is a match against one or more chars,
   return a pointer to the next operation, else return NULL.  */
unsafe fn skip_one_char (mut p: *const re_char) -> *const re_char
{
  match { let op = *p as re_opcode_t; p = p.offset (1); op }
    {
      anychar => { }

      exactn => {
        p = p.offset (*p as isize + 1);
      }

      charset_not |
      charset => {
        if CHARSET_RANGE_TABLE_EXISTS_P! (p.offset (-1))
	  {
	    let mut mcnt: c_int = 0;
	    p = CHARSET_RANGE_TABLE! (p.offset (-1));
	    EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	    p = CHARSET_RANGE_TABLE_END! (p, mcnt);
	  }
        else
	  { p = p.offset (CHARSET_BITMAP_SIZE! (p.offset (-1)) as isize + 1); }
      }

      syntaxspec |
      notsyntaxspec |
      categoryspec |
      notcategoryspec => {
        p = p.offset (1);
      }

      _ => {
        p = std::ptr::null ();
      }
    }
  p
}

/* Test if C matches charset op.  *PP points to the charset or charset_not
   opcode.  When the function finishes, *PP will be advanced past that opcode.
   C is character to test (possibly after translations) and CORIG is original
   character (i.e. without any translations).  UNIBYTE denotes whether c is
   unibyte or multibyte character.
   CANON_TABLE is the canonicalization table for case folding or Qnil.  */
unsafe fn execute_charset (pp: *mut *const re_char, c: c_int, corig: c_int, unibyte: bool,
			   canon_table: Lisp_Object) -> bool
{
  eassume! (0 <= c && 0 <= corig);
  let mut p: *const re_char = *pp;
  let mut rtp: *const re_char = std::ptr::null ();
  let not: bool = *p as re_opcode_t == charset_not;

  if CHARSET_RANGE_TABLE_EXISTS_P! (p)
    {
      let mut count: c_int = 0;
      rtp = CHARSET_RANGE_TABLE! (p);
      EXTRACT_NUMBER_AND_INCR! (count, rtp);
      *pp = CHARSET_RANGE_TABLE_END! (rtp, count);
    }
  else
    { *pp = (*pp).offset (2 + CHARSET_BITMAP_SIZE! (p) as isize); }

  if unibyte && c < (1 << BYTEWIDTH)
    {			/* Lookup bitmap.  */
      /* Cast to 'unsigned' instead of 'unsigned char' in
	 case the bit list is a full 32 bytes long.  */
      if c < (CHARSET_BITMAP_SIZE! (p) * BYTEWIDTH) as c_uint as c_int
	  && (*p.offset ((2 + c / BYTEWIDTH) as isize) as c_int
	      & (1 << (c % BYTEWIDTH))) != 0
	{ return !not; }
    }
  else if !rtp.is_null ()
    {
      let class_bits: c_int = CHARSET_RANGE_TABLE_BITS! (p);
      let mut range_start: c_int = 0;
      let mut range_end: c_int = 0;

  /* Sort tests by the most commonly used classes with some adjustment to which
     tests are easiest to perform.  Take a look at comment in re_wctype_parse
     for table with frequencies of character class names. */

      if (class_bits & BIT_MULTIBYTE) != 0
	  || ((class_bits & BIT_ALNUM) != 0 && ISALNUM! (c) != 0)
	  || ((class_bits & BIT_ALPHA) != 0 && ISALPHA! (c) != 0)
	  || ((class_bits & BIT_SPACE) != 0 && ISSPACE! (c) != 0)
          || ((class_bits & BIT_BLANK) != 0 && ISBLANK! (c) != 0)
	  || ((class_bits & BIT_WORD) != 0  && ISWORD! (c) != 0)
	  || (((class_bits & BIT_UPPER) != 0)
	      && (ISUPPER! (corig) != 0
		  || (!NILP (canon_table) && ISLOWER! (corig) != 0)))
	  || (((class_bits & BIT_LOWER) != 0)
	      && (ISLOWER! (corig) != 0
		  || (!NILP (canon_table) && ISUPPER! (corig) != 0)))
	  || ((class_bits & BIT_PUNCT) != 0 && ISPUNCT! (c) != 0)
	  || ((class_bits & BIT_GRAPH) != 0 && ISGRAPH! (c) != 0)
	  || ((class_bits & BIT_PRINT) != 0 && ISPRINT! (c) != 0)
	{ return !not; }

      p = *pp;
      while rtp < p
	{
	  EXTRACT_CHARACTER! (range_start, rtp);
	  EXTRACT_CHARACTER! (range_end, rtp.offset (3));
	  if range_start <= c && c <= range_end
	    { return !not; }
	  rtp = rtp.offset (2 * 3);
	}
    }

  not
}

/* Case where `p2` points to an `exactn` or `endline`.  */
unsafe fn mutually_exclusive_exactn (bufp: *mut re_pattern_buffer, p1: *const re_char,
				     p2: *const re_char) -> bool
{
  let multibyte: bool = RE_MULTIBYTE_P! (bufp);
  let c: c_int
    = if *p2 as re_opcode_t == endline { '\n' as c_int }
      else { RE_STRING_CHAR! (p2.offset (2), multibyte) };

  if *p1 as re_opcode_t == exactn
    {
      if c != RE_STRING_CHAR! (p1.offset (2), multibyte)
	{
	  DEBUG_PRINT! (c"  '%c' != '%c' => fast loop.\n".as_ptr (), c, *p1.offset (2) as c_int);
	  return true;
	}
    }

  else if *p1 as re_opcode_t == charset
	   || *p1 as re_opcode_t == charset_not
    {
      let mut p1 = p1;
      if !execute_charset (&raw mut p1, c, c, !multibyte || ASCII_CHAR_P (c as isize),
                           Qnil ())
	{
	  DEBUG_PRINT! (c"\t No match => fast loop.\n".as_ptr ());
	  return true;
	}
    }
  else if *p1 as re_opcode_t == anychar
	   && c == '\n' as c_int
    {
      DEBUG_PRINT! (c"   . != \\n => fast loop.\n".as_ptr ());
      return true;
    }
  false
}

/* Case where `p2` points to an `charset`.  */
unsafe fn mutually_exclusive_charset (bufp: *mut re_pattern_buffer, p1: *const re_char,
				      p2: *const re_char) -> bool
{
  /* It is hard to list up all the character in charset
     P2 if it includes multibyte character.  Give up in
     such case.  */
  if !RE_MULTIBYTE_P! (bufp) || !CHARSET_RANGE_TABLE_EXISTS_P! (p2)
    {
      /* Now, we are sure that P2 has no range table.
	     So, for the size of bitmap in P2, 'p2[1]' is
	     enough.  But P1 may have range table, so the
	     size of bitmap table of P1 is extracted by
	     using macro 'CHARSET_BITMAP_SIZE'.

	     In a multibyte case, we know that all the character
	     listed in P2 is ASCII.  In a unibyte case, P1 has only a
	     bitmap table.  So, in both cases, it is enough to test
	     only the bitmap table of P1.  */

      if *p1 as re_opcode_t == charset
	{
	  let mut idx: c_int;
	  /* We win if the charset inside the loop
		 has no overlap with the one after the loop.  */
	  idx = 0;
	  while idx < *p2.offset (1) as c_int
		&& idx < CHARSET_BITMAP_SIZE! (p1)
	    {
	      if (*p2.offset ((2 + idx) as isize) as c_int
		  & *p1.offset ((2 + idx) as isize) as c_int) != 0
		{ break; }
	      idx += 1;
	    }

	  if idx == *p2.offset (1) as c_int
	      || idx == CHARSET_BITMAP_SIZE! (p1)
	    {
	      DEBUG_PRINT! (c"\t No match => fast loop.\n".as_ptr ());
	      return true;
	    }
	}
      else if *p1 as re_opcode_t == charset_not
	{
	  let mut idx: c_int;
	  /* We win if the charset_not inside the loop lists
		 every character listed in the charset after.  */
	  idx = 0;
	  while idx < *p2.offset (1) as c_int
	    {
	      if ! (*p2.offset ((2 + idx) as isize) == 0
		    || (idx < CHARSET_BITMAP_SIZE! (p1)
			&& (*p2.offset ((2 + idx) as isize) as c_int
			    & ! *p1.offset ((2 + idx) as isize) as c_int) == 0))
		{ break; }
	      idx += 1;
	    }

	  if idx == *p2.offset (1) as c_int
	    {
	      DEBUG_PRINT! (c"\t No match => fast loop.\n".as_ptr ());
	      return true;
	    }
	}
    }
  false
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mutexcl_data
{
  pub bufp: *mut re_pattern_buffer,
  pub p1: *const re_char,
  pub unconstrained: bool,
}
const _: () = core::assert! (std::mem::size_of::<mutexcl_data> () == 24);
const _: () = core::assert! (std::mem::offset_of! (mutexcl_data, bufp) == 0);
const _: () = core::assert! (std::mem::offset_of! (mutexcl_data, p1) == 8);
const _: () = core::assert! (std::mem::offset_of! (mutexcl_data, unconstrained) == 16);

unsafe extern "C" fn mutually_exclusive_one (p2: *const re_char, arg: *mut c_void) -> bool
{
  let data: *mut mutexcl_data = arg as *mut mutexcl_data;

  /* Remember that there may be an empty matching operator on the way.
     If we return true, this is the "end" of this control flow path,
     so it can't get in the way of a subsequent `succeed.  */
  macro_rules! RETURN_CONSTRAIN {
    ($v:expr) => {
      {
        let tmp: bool = ($v);
        if !tmp
	  { (*data).unconstrained = false; }
        return tmp;
      }
    };
  }

  match *p2 as re_opcode_t
    {
      succeed => {
        /* If `p1` matches, `succeed` can still match, so we should return
           `false`.  *BUT* when N iterations of `p1` and N+1 iterations of `p1`
           match, the `succeed` that comes after N+1 always takes precedence
           over the one after N because we always prefer a longer match, so
           the succeed after N can actually be replaced by a "fail" without
           changing the end result.
           In this sense, "if `p1` matches, `succeed` can't match".
           So we can return `true`.
           *BUT* this only holds if we're sure that the N+1 will indeed succeed,
           so we need to make sure there is no other matching operator between
           the exit of the iteration and the `succeed`.  */
        return (*data).unconstrained;
      }

      endline => {
        RETURN_CONSTRAIN! (mutually_exclusive_exactn ((*data).bufp, (*data).p1, p2));
      }
      exactn => {
        return mutually_exclusive_exactn ((*data).bufp, (*data).p1, p2);
      }
      charset => {
        if *(*data).p1 as re_opcode_t == exactn
	  { return mutually_exclusive_exactn ((*data).bufp, p2, (*data).p1); }
        else
	  { return mutually_exclusive_charset ((*data).bufp, (*data).p1, p2); }
      }

      charset_not => {
        match *(*data).p1 as re_opcode_t
	  {
	    exactn => {
	      return mutually_exclusive_exactn ((*data).bufp, p2, (*data).p1);
	    }
	    charset => {
	      return mutually_exclusive_charset ((*data).bufp, p2, (*data).p1);
	    }
	    charset_not => {
	      /* When we have two charset_not, it's very unlikely that
		 they don't overlap.  The union of the two sets of excluded
		 chars should cover all possible chars, which, as a matter of
		 fact, is virtually impossible in multibyte buffers.  */
	      return false;
	    }
	    _ => {}
	  }
        return false;
      }
      anychar => {
        return false;             /* FIXME: exactn \n ? */
      }
      syntaxspec => {
        return *(*data).p1 as re_opcode_t == notsyntaxspec
	  && *(*data).p1.offset (1) == *p2.offset (1);
      }
      notsyntaxspec => {
        return *(*data).p1 as re_opcode_t == syntaxspec
	  && *(*data).p1.offset (1) == *p2.offset (1);
      }
      categoryspec => {
        return *(*data).p1 as re_opcode_t == notcategoryspec
	  && *(*data).p1.offset (1) == *p2.offset (1);
      }
      notcategoryspec => {
        return *(*data).p1 as re_opcode_t == categoryspec
	  && *(*data).p1.offset (1) == *p2.offset (1);
      }

      endbuf => {
        return true;
      }
      wordbeg => {
        RETURN_CONSTRAIN! (*(*data).p1 as re_opcode_t == notsyntaxspec
			   && *(*data).p1.offset (1) as syntaxcode == Sword);
      }
      wordend => {
        RETURN_CONSTRAIN! (*(*data).p1 as re_opcode_t == syntaxspec
			   && *(*data).p1.offset (1) as syntaxcode == Sword);
      }
      symbeg => {
        RETURN_CONSTRAIN! (*(*data).p1 as re_opcode_t == notsyntaxspec
			   && (*(*data).p1.offset (1) as syntaxcode == Ssymbol
			       || *(*data).p1.offset (1) as syntaxcode == Sword));
      }
      symend => {
        RETURN_CONSTRAIN! (*(*data).p1 as re_opcode_t == syntaxspec
			   && (*(*data).p1.offset (1) as syntaxcode == Ssymbol
			       || *(*data).p1.offset (1) as syntaxcode == Sword));
      }

      at_dot |
      begbuf |
      wordbound |
      notwordbound |
      begline => {
        RETURN_CONSTRAIN! (false);
      }

      duplicate => {
        /* At this point, we know nothing about what this can match, sadly.  */
        return false;
      }

      _ => {
	// C 4024-4026: `#if ENABLE_CHECKING`.
	#[cfg (enable_checking)]
	{
	  emacs_abort (); /* We have listed all the cases.  */
	}
        return false;
      }
    }
}

/* True if "p1 matches something" implies "p2 fails".  */

unsafe fn mutually_exclusive_p (bufp: *mut re_pattern_buffer, p1: *const re_char,
				p2: *const re_char) -> bool
{
  let mut data: mutexcl_data = mutexcl_data { bufp: bufp, p1: p1, unconstrained: true };
  eassert! ((*bufp).used != 0);
  forall_firstchar (bufp, p2, (*bufp).buffer.offset ((*bufp).used),
		    mutually_exclusive_one, &raw mut data as *mut c_void)
}

/* Matching routines.  */

/* re_match_2 matches the compiled pattern in BUFP against the
   (virtual) concatenation of STRING1 and STRING2 (of length SIZE1
   and SIZE2, respectively).  We start matching at POS, and stop
   matching at STOP.

   If REGS is non-null, store offsets for the substring each group
   matched in REGS.

   We return -1 if no match, -2 if an internal error (such as the
   failure stack overflowing).  Otherwise, we return the length of the
   matched substring.  */

/// C's `src/config.h` 3642-3663 renames `re_match_2` to `rpl_re_match_2` (gnulib's
/// replacement scheme), so the reference object exports `_rpl_re_match_2` and this
/// definition must carry C's own symbol name (the `emacs_fclose` shape, status 3.68).
#[unsafe(export_name = "rpl_re_match_2")]
pub unsafe extern "C" fn re_match_2 (bufp: *mut re_pattern_buffer,
				     string1: *const c_char, size1: ptrdiff_t,
				     string2: *const c_char, size2: ptrdiff_t,
				     pos: ptrdiff_t, regs: *mut re_registers,
				     stop: ptrdiff_t) -> ptrdiff_t
{
  let result: ptrdiff_t;

  RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (re_match_object, pos);

  result = re_match_2_internal (bufp, string1 as *const re_char, size1,
				string2 as *const re_char, size2,
				pos, regs, stop);
  result
}

unsafe extern "C" {
  /// C: dispnew.h 603: `extern void update_redisplay_ticks (int, struct window *);` - the
  /// window is reached only as a pointer and window.rs's record is not in this unit's
  /// include chain, so the ABI-equal `*mut c_void` names it (PORTING.md 0.1-2/P-72 rule 3).
  fn update_redisplay_ticks (ticks: c_int, w: *mut c_void);
}

unsafe extern "C" fn unwind_re_match (ptr: *mut c_void)
{
  let b: *mut buffer = ptr as *mut buffer;
  (*(*b).text).bits &= ! BUFFER_TEXT_INHIBIT_SHRINKING;
}

/* This is a separate function so that we can force an alloca cleanup
   afterwards.  */
/// Diagnostic only (P-147): the opcode executed before the current one, so a report can name the
/// instruction that left `d` in the gap instead of the one that noticed it.
static mut PREV_OP_SAVE: c_int = -1;

unsafe fn re_match_2_internal (bufp: *mut re_pattern_buffer,
			       string1: *const re_char, size1: ptrdiff_t,
			       string2: *const re_char, size2: ptrdiff_t,
			       pos: ptrdiff_t, regs: *mut re_registers,
			       stop: ptrdiff_t) -> ptrdiff_t
{
  let mut string1 = string1;
  let mut size1 = size1;
  let mut string2 = string2;
  let mut size2 = size2;

  eassume! (0 <= size1);
  eassume! (0 <= size2);
  eassume! (0 <= pos && pos <= stop && stop <= size1 + size2);

  /* General temporaries.  */
  let mut mcnt: c_int = 0;

  /* Just past the end of the corresponding string.  */
  let mut end1: *const re_char;
  let mut end2: *const re_char;

  /* Pointers into string1 and string2, just past the last characters in
     each to consider matching.  */
  let mut end_match_1: *const re_char;
  let mut end_match_2: *const re_char;

  /* Where we are in the data, and the end of the current string.  */
  let mut d: *const re_char;
  let mut dend: *const re_char;

  /* Used sometimes to remember where we were before starting matching
     an operator so that we can go back in case of failure.  This "atomic"
     behavior of matching opcodes is indispensable to the correctness
     of the on_failure_keep_string_jump optimization.  */
  let mut dfail: *const re_char = std::ptr::null ();

  /* Where we are in the pattern, and the end of the pattern.  */
  let mut p: *const re_char = (*bufp).buffer as *const re_char;
  let pend: *const re_char = p.offset ((*bufp).used);

  /* We use this to map every character in the string.	*/
  let translate: Lisp_Object = (*bufp).translate;

  /* True if BUFP is setup from a multibyte regex.  */
  let multibyte: bool = RE_MULTIBYTE_P! (bufp);

  /* True if STRING1/STRING2 are multibyte.  */
  let target_multibyte: bool = RE_TARGET_MULTIBYTE_P! (bufp);

  /* Failure point stack.  Each place that can handle a failure further
     down the line pushes a failure point on this stack.  It consists of
     regstart, and regend for all registers corresponding to
     the subexpressions we're currently inside, plus the number of such
     registers, and, finally, two char *'s.  The first char * is where
     to resume scanning the pattern; the second one is where to resume
     scanning the strings.  */
  // C declares it and fills its four fields below; Rust needs the whole record initialised
  // first (PORTING.md 0.1-13).
  let mut fail_stack: fail_stack_type = fail_stack_type
    { stack: std::ptr::null_mut (), size: 0, avail: 0, frame: 0 };
  // C 4133-4135: `#ifdef DEBUG_COMPILES_ARGUMENTS`.
  #[cfg (regex_emacs_debug)]
  let mut nfailure_points_pushed: ptrdiff_t = 0;
  #[cfg (regex_emacs_debug)]
  let mut nfailure_points_popped: ptrdiff_t = 0;

  /* We fill all the registers internally, independent of what we
     return, for use in backreferences.  The number here includes
     an element for register zero.  */
  let num_regs: ptrdiff_t = (*bufp).re_nsub + 1;
  eassume! (0 < num_regs);

  /* Information on the contents of registers. These are pointers into
     the input strings; they record just what was matched (on this
     attempt) by a subexpression part of the pattern, that is, the
     regnum-th regstart pointer points to where in the pattern we began
     matching and the regnum-th regend points to right after where we
     stopped matching the regnum-th subexpression.  */
  let mut regstart: *mut *const re_char = std::ptr::null_mut ();
  let mut regend: *mut *const re_char = std::ptr::null_mut ();

  /* The following record the register info as found in the above
     variables when we find a match better than any we've seen before.
     This happens as we backtrack through the failure points, which in
     turn happens only if we have not yet matched the entire string. */
  let mut best_regs_set = false;
  let mut best_regstart: *mut *const re_char = std::ptr::null_mut ();
  let mut best_regend: *mut *const re_char = std::ptr::null_mut ();

  /* Logically, this is 'best_regend[0]'.  But we don't want to have to
     allocate space for that if we're not allocating space for anything
     else (see below).  Also, we never need info about register 0 for
     any of the other register vectors, and it seems rather a kludge to
     treat 'best_regend' differently from the rest.  So we keep track of
     the end of the best match so far in a separate variable.  We
     initialize this to NULL so that when we backtrack the first time
     and need to test it, it's not garbage.  */
  let mut match_end: *const re_char = std::ptr::null ();

  /* This keeps track of how many buffer/string positions we examined.  */
  let mut nchars: ptrdiff_t = 0;

  /* Final return value of the function.  */
  let mut retval: ptrdiff_t = -1;        /* Presumes failure to match for now.  */

  // C 4174-4177: `#ifdef DEBUG_COMPILES_ARGUMENTS`.
  #[cfg (regex_emacs_debug)]
  let mut num_regs_pushed: ptrdiff_t = 0;

  // A local `cycle` (C 1062 sets it in CHECK_INFINITE_LOOP) needs a binding at this scope;
  // the call sites test it right after the macro (C declares it at the top of its block).
  let mut cycle: bool = false;

  // C 4181's `REGEX_USE_SAFE_ALLOCA` (the mirror's `USE_SAFE_ALLOCA!` plus
  // `sa_avail = emacs_re_safe_alloca`) and C 4185's `count` declare three names the
  // failure-stack macros below capture; a `macro_rules!` resolves locals at its definition
  // site (PORTING.md 2.2/P-57), so the three declarations stand here and C's own statements
  // keep their positions among themselves (`sa_avail = emacs_re_safe_alloca` and
  // `INIT_FAIL_STACK ()` below are C 4181-4183).
  let mut sa_avail: ptrdiff_t = MAX_ALLOCA;
  let sa_count: specpdl_ref = SPECPDL_INDEX ();
  /* The port's third name (PORTING.md P-05/P-117 ⑧): this function captures the pool's height
     the way `USE_SAFE_ALLOCA!` does, so `SAFE_FREE!` below releases the scope's own requests.  */
  let sa_mark: usize = unsafe { sa_stack_mark () };
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* Failure-stack and matching macros (C 911-1160, 3658-3681): every one of them names a
     local of this function, so they are defined here, after those locals (PORTING.md
     2.2/P-57).  `REGEX_REALLOCATE`'s C body assigns the enclosing block's `destination`;
     Rust resolves that name at the definition site, so the port hands the destination in as
     the first metavariable (recorded in docs/ports/macro-body-notes.tsv).  */
  macro_rules! FAIL_STACK_EMPTY { () => { fail_stack.frame == 0 }; }

  macro_rules! REGEX_REALLOCATE {
    ($destination:ident, $source:expr, $osize:expr, $nsize:expr) => {
      {
	$destination = SAFE_ALLOCA! (sa_avail, $nsize) as *mut c_void;
	memcpy ($destination, $source as *const c_void, $osize as usize);
      }
    };
  }

  macro_rules! INIT_FAIL_STACK {
    () => {
      {
	fail_stack.stack = SAFE_ALLOCA! (sa_avail, (INIT_FAILURE_ALLOC as ptrdiff_t)
					 * TYPICAL_FAILURE_SIZE
			     * (std::mem::size_of::<fail_stack_elt_t> () as ptrdiff_t))
	  as *mut fail_stack_elt_t;
	fail_stack.size = INIT_FAILURE_ALLOC as ptrdiff_t;
	fail_stack.avail = 0;
	fail_stack.frame = 0;
      }
    };
  }

  macro_rules! GROW_FAIL_STACK {
    ($fail_stack:expr, $destination:ident) => {
      if ($fail_stack).size >= emacs_re_max_failures * TYPICAL_FAILURE_SIZE
	{ 0 }
      else
	{
	  REGEX_REALLOCATE! ($destination, ($fail_stack).stack,
	    ($fail_stack).avail * std::mem::size_of::<fail_stack_elt_t> () as ptrdiff_t,
	    std::cmp::min (emacs_re_max_failures * TYPICAL_FAILURE_SIZE,
			   ($fail_stack).size * FAIL_STACK_GROWTH_FACTOR)
	    * (std::mem::size_of::<fail_stack_elt_t> () as ptrdiff_t));
	  ($fail_stack).stack = $destination as *mut fail_stack_elt_t;
	  ($fail_stack).size
	    = std::cmp::min (emacs_re_max_failures * TYPICAL_FAILURE_SIZE,
			     ($fail_stack).size * FAIL_STACK_GROWTH_FACTOR);
	  1
	}
    };
  }

  macro_rules! PUSH_FAILURE_POINTER {
    ($item:expr) => {
      {
        fail_stack.stack.offset (fail_stack.avail).write (fail_stack_elt { pointer: $item });
        fail_stack.avail += 1;
      }
    };
  }

  macro_rules! PUSH_FAILURE_INT {
    ($item:expr) => {
      {
        fail_stack.stack.offset (fail_stack.avail).write (fail_stack_elt { integer: $item });
        fail_stack.avail += 1;
      }
    };
  }

  macro_rules! POP_FAILURE_POINTER {
    () => { { fail_stack.avail -= 1; (*fail_stack.stack.offset (fail_stack.avail)).pointer } };
  }
  macro_rules! POP_FAILURE_INT {
    () => { { fail_stack.avail -= 1; (*fail_stack.stack.offset (fail_stack.avail)).integer } };
  }

  macro_rules! FAILURE_PAT { ($h:expr) => { (*fail_stack.stack.offset (($h) - 1)).pointer }; }
  macro_rules! FAILURE_STR {
    ($h:expr) => { (*fail_stack.stack.offset (($h) - 2)).pointer };
  }
  macro_rules! NEXT_FAILURE_HANDLE {
    ($h:expr) => { (*fail_stack.stack.offset (($h) - 3)).integer };
  }
  macro_rules! TOP_FAILURE_HANDLE { () => { fail_stack.frame }; }

  /* How many items can still be added to the stack without overflowing it.  */
  macro_rules! REMAINING_AVAIL_SLOTS {
    () => { fail_stack.size - fail_stack.avail };
  }

  macro_rules! ENSURE_FAIL_STACK {
    ($space:expr, $destination:ident) => {
      while REMAINING_AVAIL_SLOTS! () <= $space {
        if GROW_FAIL_STACK! (fail_stack, $destination) == 0
	  {
	    unbind_to (count, Qnil ());
	    SAFE_FREE! (sa_count, sa_mark);
	    return -2;
	  }
        DEBUG_PRINT! (c"\n  Doubled stack; size now: %td\n".as_ptr (), fail_stack.size);
        DEBUG_PRINT! (c"\t slots available: %td\n".as_ptr (), REMAINING_AVAIL_SLOTS! ());
      }
    };
  }

  /* Push register NUM onto the stack.  */
  macro_rules! PUSH_FAILURE_REG {
    ($num:expr) => {
      {
        let mut destination: *mut c_void = std::ptr::null_mut ();
        let n: isize = $num as isize;
        eassert! (0 < n && n < num_regs);
        eassert! (REG_UNSET! (*regstart.offset (n)) <= REG_UNSET! (*regend.offset (n)));
        ENSURE_FAIL_STACK! (3, destination);
        DEBUG_PRINT! (c"    Push reg %td (spanning %p -> %p)\n".as_ptr (),
		       n, *regstart.offset (n), *regend.offset (n));
        PUSH_FAILURE_POINTER! (*regstart.offset (n));
        PUSH_FAILURE_POINTER! (*regend.offset (n));
        PUSH_FAILURE_INT! (n);
      }
    };
  }

  /* Change the counter's value to VAL, but make sure that it will
     be reset when backtracking.  */
  macro_rules! PUSH_NUMBER {
    ($ptr:expr, $val:expr) => {
      {
        let mut destination: *mut c_void = std::ptr::null_mut ();
        let mut c: c_int = 0;
        ENSURE_FAIL_STACK! (3, destination);
        EXTRACT_NUMBER! (c, $ptr);
        DEBUG_PRINT! (c"    Push number %p = %d -> %d\n".as_ptr (), $ptr, c, $val);
        PUSH_FAILURE_INT! (c as isize);
        PUSH_FAILURE_POINTER! ($ptr as *const re_char);
        PUSH_FAILURE_INT! (-1);
        STORE_NUMBER ($ptr, $val);
      }
    };
  }

  /* Pop a saved register off the stack.  */
  macro_rules! POP_FAILURE_REG_OR_COUNT {
    () => {
      {
        let mut pfreg: isize = POP_FAILURE_INT! ();
        if pfreg == -1
	  {
	    /* It's a counter.  */
	    /* Discard 'const', making re_search non-reentrant.  */
	    let ptr: *mut u8 = POP_FAILURE_POINTER! () as *mut u8;
	    pfreg = POP_FAILURE_INT! ();
	    STORE_NUMBER (ptr, pfreg as c_int);
	    DEBUG_PRINT! (c"     Pop counter %p = %td\n".as_ptr (), ptr, pfreg);
	  }
        else
	  {
	    eassert! (0 < pfreg && pfreg < num_regs);
	    *regend.offset (pfreg) = POP_FAILURE_POINTER! ();
	    *regstart.offset (pfreg) = POP_FAILURE_POINTER! ();
	    eassert! (REG_UNSET! (*regstart.offset (pfreg))
		      <= REG_UNSET! (*regend.offset (pfreg)));
	    DEBUG_PRINT! (c"     Pop reg %ld (spanning %p -> %p)\n".as_ptr (),
			   pfreg, *regstart.offset (pfreg), *regend.offset (pfreg));
	  }
      }
    };
  }

  /* Check that we are not stuck in an infinite loop.  */
  macro_rules! CHECK_INFINITE_LOOP {
    ($pat_cur:expr, $string_place:expr) => {
      {
        let mut failure: ptrdiff_t = TOP_FAILURE_HANDLE! ();
        /* Check for infinite matching loops */
        while failure > 0
	      && (FAILURE_STR! (failure) == $string_place
		  || FAILURE_STR! (failure).is_null ())
	  {
	    eassert! (FAILURE_PAT! (failure) >= (*bufp).buffer as *const re_char
		      && FAILURE_PAT! (failure) <= (*bufp).buffer.offset ((*bufp).used)
			as *const re_char);
	    if FAILURE_PAT! (failure) == $pat_cur
	      {
		cycle = true;
		break;
	      }
	    DEBUG_PRINT! (c"  Other pattern: %p\n".as_ptr (), FAILURE_PAT! (failure));
	    failure = NEXT_FAILURE_HANDLE! (failure);
	  }
        DEBUG_PRINT! (c"  Other string: %p\n".as_ptr (), FAILURE_STR! (failure));
      }
    };
  }

  /* Push the information about the state we will need
     if we ever fail back to it.

     Requires variables fail_stack, regstart, regend and
     num_regs be declared.  GROW_FAIL_STACK requires 'destination' be
     declared.

     Does 'return FAILURE_CODE' if runs out of memory.  */

  macro_rules! PUSH_FAILURE_POINT {
    ($pattern:expr, $string_place:expr) => {
      {
        let mut destination: *mut c_void = std::ptr::null_mut ();
        DEBUG_STATEMENT! (nfailure_points_pushed += 1);
        DEBUG_PRINT! (c"\nPUSH_FAILURE_POINT:\n".as_ptr ());
        DEBUG_PRINT! (c"  Before push, next avail: %td\n".as_ptr (), fail_stack.avail);
        DEBUG_PRINT! (c"\t\t\tsize: %td\n".as_ptr (), fail_stack.size);

        ENSURE_FAIL_STACK! (NUM_NONREG_ITEMS, destination);

        DEBUG_PRINT! (c"\n".as_ptr ());

        DEBUG_PRINT! (c"  Push frame index: %td\n".as_ptr (), fail_stack.frame);
        PUSH_FAILURE_INT! (fail_stack.frame);

        DEBUG_PRINT! (c"  Push string %p: \"".as_ptr (), $string_place);
        DEBUG_PRINT_DOUBLE_STRING! ($string_place, string1, size1, string2, size2);
        DEBUG_PRINT! (c"\"\n".as_ptr ());
        PUSH_FAILURE_POINTER! ($string_place);

        DEBUG_PRINT! (c"  Push pattern %p: ".as_ptr (), $pattern);
        DEBUG_PRINT_COMPILED_PATTERN! (bufp, $pattern, pend);
        PUSH_FAILURE_POINTER! ($pattern);

        /* Close the frame by moving the frame pointer past it.  */
        fail_stack.frame = fail_stack.avail;
      }
    };
  }

  /* Pop what PUSH_FAIL_STACK pushes.

     Restore into the parameters, all of which should be lvalues:
       STR -- the saved data position.
       PAT -- the saved pattern position.
       REGSTART, REGEND -- arrays of string positions.

     Also assume the variables FAIL_STACK and (if debugging) BUFP, PEND,
     STRING1, SIZE1, STRING2, and SIZE2.  */

  macro_rules! POP_FAILURE_POINT {
    ($str:expr, $pat:expr) => {
      {
        eassert! (!FAIL_STACK_EMPTY! ());

        /* Remove failure points and point to how many regs pushed.  */
        DEBUG_PRINT! (c"POP_FAILURE_POINT:\n".as_ptr ());
        DEBUG_PRINT! (c"  Before pop, next avail: %td\n".as_ptr (), fail_stack.avail);
        DEBUG_PRINT! (c"\t\t     size: %td\n".as_ptr (), fail_stack.size);

        /* Pop the saved registers.  */
        while fail_stack.frame < fail_stack.avail
	  { POP_FAILURE_REG_OR_COUNT! (); }

        $pat = POP_FAILURE_POINTER! ();
        DEBUG_PRINT! (c"  Popping pattern %p: ".as_ptr (), $pat);
        DEBUG_PRINT_COMPILED_PATTERN! (bufp, $pat, pend);

        /* If the saved string location is NULL, it came from an
	   on_failure_keep_string_jump opcode, and we want to throw away the
	   saved NULL, thus retaining our current position in the string.  */
        $str = POP_FAILURE_POINTER! ();
        DEBUG_PRINT! (c"  Popping string %p: \"".as_ptr (), $str);
        DEBUG_PRINT_DOUBLE_STRING! ($str, string1, size1, string2, size2);
        DEBUG_PRINT! (c"\"\n".as_ptr ());

        fail_stack.frame = POP_FAILURE_INT! ();
        DEBUG_PRINT! (c"  Popping  frame index: %td\n".as_ptr (), fail_stack.frame);

        eassert! (fail_stack.avail >= 0);
        eassert! (fail_stack.frame <= fail_stack.avail);

        DEBUG_STATEMENT! (nfailure_points_popped += 1);
      }
    };
  }

  /* Set C a (possibly converted to multibyte) character before P.  P
     points into a string which is the virtual concatenation of STR1
     (which ends at END1) or STR2 (which ends at END2).  */
  macro_rules! GET_CHAR_BEFORE_2 {
    ($c:expr, $p:ident, $str1:expr, $end1:expr, $str2:expr, $end2:expr) => {
      {
        if target_multibyte
	  {
	    let mut dtemp: *const re_char = if ($p) == ($str2) { ($end1) } else { ($p) };
	    let dlimit: *const re_char = if ($p) > ($str2) && ($p) <= ($end2) { ($str2) }
					 else { ($str1) };
	    /* C: `while (dtemp-- > dlimit && !CHAR_HEAD_P (*dtemp)) continue;` - the comparison
	       uses the value *before* the decrement while the character test uses the value
	       *after* it, and the failing test still decrements (P-152's family: pre- versus
	       post-decrement).  Comparing the decremented value instead looked one byte further.  */
	    while { let old: *const re_char = dtemp; dtemp = dtemp.offset (-1); old > dlimit }
		  && ! CHAR_HEAD_P (*dtemp as c_int)
	      { continue; }
	    $c = STRING_CHAR (dtemp) as _;
	  }
        else
	  {
	    $c = ((if ($p) == ($str2) { *($end1).offset (-1) }
		   else { *($p).offset (-1) }) as c_int) as _;
	    $c = RE_CHAR_TO_MULTIBYTE! ($c) as _;
	  }
      }
    };
  }

  /* Set C a (possibly converted to multibyte) character at P, and set
     LEN to the byte length of that character.  */
  macro_rules! GET_CHAR_AFTER {
    ($c:expr, $p:expr, $len:ident) => {
      {
        if target_multibyte
	  { $c = string_char_and_length ($p, &raw mut $len) as _; }
        else
	  {
	    $c = *$p as c_int as _;
	    $len = 1;
	    $c = RE_CHAR_TO_MULTIBYTE! ($c) as _;
	  }
       }
    };
  }

  /* Call before fetching a character with *d.  This switches over to
     string2 if necessary.  */
  /* Call before fetching a char with *d if you already checked other limits.  */
  macro_rules! PREFETCH_NOLIMIT {
    () => {
      if d == end1
	 {
	   d = string2;
	   dend = end_match_2;
	 }
    };
  }

  /* This converts PTR, a pointer into one of the search strings 'string1'
     and 'string2' into an offset from the beginning of that string.  */
  macro_rules! POINTER_TO_OFFSET {
    ($ptr:expr) => {
      if FIRST_STRING_P! ($ptr) { ($ptr).offset_from (string1) }
      else { ($ptr).offset_from (string2) + size1 }
    };
  }

  /* Test if at very beginning or at very end of the virtual concatenation
     of STRING1 and STRING2.  If only one string, it's STRING2.  */
  macro_rules! AT_STRINGS_BEG {
    ($d:expr) => { ($d) == (if size1 != 0 { string1 } else { string2 }) || size2 == 0 };
  }
  macro_rules! AT_STRINGS_END { ($d:expr) => { ($d) == end2 }; }

  macro_rules! TRANSLATE {
    ($d:expr) => { if !NILP (translate) { RE_TRANSLATE! (translate, $d) } else { $d } };
  }
  macro_rules! POS_ADDR_VSTRING {
    ($POS:expr) => {
      (if ($POS) >= size1 { string2.offset (-size1) } else { string1 }).offset ($POS)
    };
  }

  /* True if 'size1' is nonzero and PTR is pointing anywhere inside
     'string1' or just past its end.  This works if PTR is NULL, which is
     a good thing.  */
  macro_rules! FIRST_STRING_P {
    ($ptr:expr) => { size1 != 0 && string1 <= ($ptr) && ($ptr) <= string1.offset (size1) };
  }

  DEBUG_PRINT! (c"\nEntering re_match_2.\n".as_ptr ());

  sa_avail = emacs_re_safe_alloca;      /* C 4181's `REGEX_USE_SAFE_ALLOCA` tail.  */

  INIT_FAIL_STACK! ();                  /* C 4183.  */

  /* Prevent shrinking and relocation of buffer text if GC happens
     while we are inside this function.  The calls to
     UPDATE_SYNTAX_TABLE_* macros can call Lisp (via
     `internal--syntax-propertize`); these calls are careful to defend against
     buffer modifications, but even with no modifications, the buffer text may
     be relocated during GC by `compact_buffer` which would invalidate
     our C pointers to buffer text.  */
  if (*(*(current_buffer! ())).text).bits & BUFFER_TEXT_INHIBIT_SHRINKING == 0
    {
      record_unwind_protect_ptr (Some (unwind_re_match), (current_buffer! ()) as *mut c_void);
      (*(*(current_buffer! ())).text).bits |= BUFFER_TEXT_INHIBIT_SHRINKING;
    }

  /* Do not bother to initialize all the register variables if there are
     no groups in the pattern, as it takes a fair amount of time.  If
     there are groups, we include space for register 0 (the whole
     pattern) in REGSTART[0], even though we never use it, to avoid
     the undefined behavior of subtracting 1 from REGSTART.  */
  let re_nsub: ptrdiff_t = num_regs - 1;
  if 0 < re_nsub
    {
      regstart = SAFE_ALLOCA! (sa_avail,
			       (re_nsub * 4 + 1)
			       * std::mem::size_of::<*mut *const re_char> () as ptrdiff_t)
	as *mut *const re_char;
      regend = regstart.offset (num_regs);
      best_regstart = regend.offset (re_nsub);
      best_regend = best_regstart.offset (re_nsub);

      /* Initialize subexpression text positions to unset, to mark ones
	 that no start_memory/stop_memory has been seen for.  */
      let mut apos: *mut *const re_char = regstart.offset (1);
      while apos < best_regstart.offset (1)
	{
	  *apos = std::ptr::null ();
	  apos = apos.offset (1);
	}
    }

  /* We move 'string1' into 'string2' if the latter's empty -- but not if
     'string1' is null.  */
  if size2 == 0 && !string1.is_null ()
    {
      string2 = string1;
      size2 = size1;
      string1 = std::ptr::null ();
      size1 = 0;
    }
  end1 = string1.offset (size1);
  end2 = string2.offset (size2);

  /* P scans through the pattern as D scans through the data.
     DEND is the end of the input string that D points within.
     Advance D into the following input string whenever necessary, but
     this happens before fetching; therefore, at the beginning of the
     loop, D can be pointing at the end of a string, but it cannot
     equal STRING2.  */
  if pos >= size1
    {
      /* Only match within string2.  */
      d = string2.offset (pos - size1);
      dend = string2.offset (stop - size1);
      end_match_2 = dend;
      end_match_1 = end1;	/* Just to give it a value.  */
    }
  else
    {
      if stop < size1
	{
	  /* Only match within string1.  */
	  end_match_1 = string1.offset (stop);
	  /* BEWARE!
	     When we reach end_match_1, PREFETCH normally switches to string2.
	     But in the present case, this means that just doing a PREFETCH
	     makes us jump from 'stop' to 'gap' within the string.
	     What we really want here is for the search to stop as
	     soon as we hit end_match_1.  That's why we set end_match_2
	     to end_match_1 (since PREFETCH fails as soon as we hit
	     end_match_2).  */
	  end_match_2 = end_match_1;
	}
      else
	{ /* It's important to use this code when STOP == SIZE so that
	     moving D from end1 to string2 will not prevent the D == DEND
	     check from catching the end of string.  */
	  end_match_1 = end1;
	  end_match_2 = string2.offset (stop - size1);
	}
      d = string1.offset (pos);
      dend = end_match_1;
    }

  DEBUG_PRINT! (c"The compiled pattern is:\n".as_ptr ());
  DEBUG_PRINT_COMPILED_PATTERN! (bufp, p, pend);
  DEBUG_PRINT! (c"The string to match is: \"".as_ptr ());
  DEBUG_PRINT_DOUBLE_STRING! (d, string1, size1, string2, size2);
  DEBUG_PRINT! (c"\"\n".as_ptr ());

  /* This loops over pattern commands.  It exits by returning from the
     function if the match is complete, or it drops through if the match
     fails at this starting point in the input data.  */
  // C's labels inside this loop (`fail:` at 5247, `restore_best_regs:` at 4337 and
  // `succeed_label:` at 4359) have no Rust counterpart, so the port carries them as the
  // flags below, evaluated where C's labels are (PORTING.md 2.2, the ccl.rs/bytecode.rs
  // shape); `goto endof_re_match` is `break 'matcher`, and the post-loop
  // `if (best_regs_set) goto restore_best_regs` (C 5290) sets `best_restore` for the shared
  // block after the loop.
  let mut failed = false;
  let mut do_restore = false;
  let mut do_succeed = false;
  'matcher: loop
    {
      DEBUG_PRINT! (c"\n%p: ".as_ptr (), p);
      let mut goto_fail = false;
      // C's `on_failure:` (4826) and `unconditional_jump:` (4912) labels are jumped to from a
      // second opcode, so they are the two flags the port tests where those labels are.
      let mut goto_on_failure = false;
      let mut goto_unconditional_jump = false;

  macro_rules! PREFETCH {
    /* C calls it with an empty `reset` in several places.  */
    () => {
      while d == dend
	{
	  /* End of string2 => fail.  */
	  if dend == end_match_2
	    {
	      goto_fail = true;
	      break;
	    }
	  /* End of string1 => advance to string2.  */
	  d = string2;
	  dend = end_match_2;
	}
    };
    ($reset:expr) => {
      while d == dend
	{
	  /* End of string2 => fail.  */
	  if dend == end_match_2
	    {
	      $reset;
	      goto_fail = true;
	      break;
	    }
	  /* End of string1 => advance to string2.  */
	  d = string2;
	  dend = end_match_2;
	}
    };
  }

      if p == pend
	{
	  /* End of pattern means we might have succeeded.  */
	  DEBUG_PRINT! (c"end of pattern ... ".as_ptr ());

	  /* If we haven't matched the entire string, and we want the
	     longest match, try backtracking.  */
	  if d != end_match_2
	    {
	      /* True if this match is the best seen so far.  */
	      let best_match_p: bool;

	      {
		/* True if this match ends in the same string (string1
		   or string2) as the best previous match.  */
		let same_str_p = FIRST_STRING_P! (match_end)
				   == FIRST_STRING_P! (d);

		/* AIX compiler got confused when this was combined
		   with the previous declaration.  */
		if same_str_p
		  { best_match_p = d > match_end; }
		else
		  { best_match_p = !FIRST_STRING_P! (d); }
	      }

	      DEBUG_PRINT! (c"backtracking.\n".as_ptr ());

	      if !FAIL_STACK_EMPTY! ()
		{ /* More failure points to try.  */

		  /* If exceeds best match so far, save it.  */
		  if !best_regs_set || best_match_p
		    {
		      best_regs_set = true;
		      match_end = d;

		      DEBUG_PRINT! (c"\nSAVING match as best so far.\n".as_ptr ());

		      let mut reg: ptrdiff_t = 1;
		      while reg < num_regs
			{
			  *best_regstart.offset (reg) = *regstart.offset (reg);
			  *best_regend.offset (reg) = *regend.offset (reg);
			  reg += 1;
			}
		    }
		  goto_fail = true;
		}

	      /* If no failure points, don't restore garbage.  And if
		 last match is real best match, don't restore second
		 best one. */
	      else if best_regs_set && !best_match_p
		{
		  /* restore_best_regs: */
		  do_restore = true;
		}
	    } /* d != end_match_2 */

	  /* succeed_label: */
	  if !goto_fail
	    { do_succeed = true; }
	}

      if do_succeed
	{ break 'matcher; }

      if !goto_fail
	{
      /* Instrument (P-147/P-148): at the top of an opcode, `d`'s segment must agree with `dend`
         (the failure path recomputes `dend` before it comes back here, so a disagreement is a
         defect, not the transient the pop itself has).  Report the opcode that left it so.  */
      if size1 != 0 && (dend == end_match_1) != ((d as usize) <= (end1 as usize))
        {
          if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
            {
              fprintf (stderr,
                       c"REGTRACE crossed prev=%d opcode=%d d-s1=%td size1=%td dend-s1=%td\n".as_ptr (),
                       PREV_OP_SAVE, *p.offset (-1) as c_int,
                       (d as isize) - (string1 as isize),
                       (end1 as isize) - (string1 as isize),
                       (dend as isize) - (string1 as isize));
            }
        }

      /* Instrument (P-147): `d` must never stand between the two strings.  The buffer text is
         split at the gap, so a `d` that is past `end1` and short of `string2` is a dangling
         position: report the opcode that left it there.  */
      if (d as usize) > (end1 as usize) && (d as usize) < (string2 as usize)
        {
          if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
            {
              fprintf (stderr,
                       c"REGTRACE d-in-gap opcode=%d prev=%d pattern-offset=%td d-s1=%td size1=%td dend-s1=%td\n".as_ptr (),
                       *p.offset (-1) as c_int, PREV_OP_SAVE,
                       (p as isize) - ((*bufp).buffer as isize) - 1,
                       (d as isize) - (string1 as isize),
                       (end1 as isize) - (string1 as isize),
                       (dend as isize) - (string1 as isize));
            }
        }

      /* Instrument (P-170): a per-opcode step trace, so a two-string match can be read
         opcode by opcode.  `FEMACS_REG_TRACE` only, default off (P-115's shape).  */
      if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
        {
          fprintf (stderr,
                   c"REGTRACE step pat=%td op=%d d-s1=%td dend-s1=%td pos=%td size1=%td end1-s1=%td end2-s1=%td\n".as_ptr (),
                   (p as isize) - ((*bufp).buffer as isize),
                   *p as c_int,
                   (d as isize) - (string1 as isize),
                   (dend as isize) - (string1 as isize),
                   pos,
                   size1,
                   (end1 as isize) - (string1 as isize),
                   (end2 as isize) - (string1 as isize));
        }
      /* Otherwise match next pattern command.  */
      unsafe { if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null () { PREV_OP_SAVE = *p as c_int; } }
      match { let op = *p as re_opcode_t; p = p.offset (1); op }
	{
	/* Ignore these.  Used to ignore the n of succeed_n's which
	   currently have n == 0.  */
	no_op => {
	  DEBUG_PRINT! (c"EXECUTING no_op.\n".as_ptr ());
	}

	succeed => {
	  DEBUG_PRINT! (c"EXECUTING succeed.\n".as_ptr ());
	  do_succeed = true;
	}

	/* Match the next n pattern characters exactly.  The following
	   byte in the pattern defines n, and the n bytes after that
	   are the characters to match.  */
	exactn => 'cmd: {
	  mcnt = *p as c_int; p = p.offset (1);
	  DEBUG_PRINT! (c"EXECUTING exactn %d.\n".as_ptr (), mcnt);

	  /* Remember the start point to rollback upon failure.  */
	  dfail = d;

	  /* The cost of testing 'translate' is comparatively small.  */
	  if target_multibyte
	    {
	    loop
	      {
		let mut pat_charlen: c_int = 0;
		let mut buf_charlen: c_int = 0;
		let mut pat_ch: c_int = 0;
		let mut buf_ch: c_int = 0;

		PREFETCH! (d = dfail);
		if goto_fail { break 'cmd; }
		if multibyte
		  { pat_ch = string_char_and_length (p, &raw mut pat_charlen); }
		else
		  {
		    pat_ch = RE_CHAR_TO_MULTIBYTE! (*p as c_int);
		    pat_charlen = 1;
		  }
		buf_ch = string_char_and_length (d, &raw mut buf_charlen);

		if TRANSLATE! (buf_ch) != pat_ch
		  {
		    d = dfail;
		    goto_fail = true;
		    break 'cmd;
		  }

		p = p.offset (pat_charlen as isize);
		d = d.offset (buf_charlen as isize);
		mcnt -= pat_charlen;
		nchars += 1;
		if mcnt > 0 { continue; }
		break;
	      }
	    }
	  else
	    {
	    loop
	      {
		let mut pat_charlen: c_int = 0;
		let mut pat_ch: c_int = 0;
		let mut buf_ch: c_int = 0;

		PREFETCH! (d = dfail);
		if goto_fail { break 'cmd; }
		if multibyte
		  {
		    pat_ch = string_char_and_length (p, &raw mut pat_charlen);
		    pat_ch = RE_CHAR_TO_UNIBYTE! (pat_ch);
		  }
		else
		  {
		    pat_ch = *p as c_int;
		    pat_charlen = 1;
		  }
		buf_ch = RE_CHAR_TO_MULTIBYTE! (*d as c_int);
		if ! CHAR_BYTE8_P (buf_ch)
		  {
		    buf_ch = TRANSLATE! (buf_ch);
		    buf_ch = RE_CHAR_TO_UNIBYTE! (buf_ch);
		    if buf_ch < 0
		      { buf_ch = *d as c_int; }
		  }
		else
		  { buf_ch = *d as c_int; }
		if buf_ch != pat_ch
		  {
		    d = dfail;
		    goto_fail = true;
		    break 'cmd;
		  }
		p = p.offset (pat_charlen as isize);
		d = d.offset (1);
		mcnt -= pat_charlen;
		nchars += 1;
		if mcnt > 0 { continue; }
		break;
	      }
	    }
	}

	/* Match any character except newline.  */
	anychar => 'cmd: {
	  let mut buf_charlen: c_int = 0;
	  let mut buf_ch: c_int = 0;

	  DEBUG_PRINT! (c"EXECUTING anychar.\n".as_ptr ());

	  PREFETCH! ();
	  if goto_fail { break 'cmd; }
	  buf_ch = RE_STRING_CHAR_AND_LENGTH! (d, buf_charlen,
					       target_multibyte);
	  buf_ch = TRANSLATE! (buf_ch);
	  if buf_ch == '\n' as c_int
	    { goto_fail = true; break 'cmd; }

	  DEBUG_PRINT! (c"  Matched \"%d\".\n".as_ptr (), *d as c_int);
	  d = d.offset (buf_charlen as isize);
	  nchars += 1;
	}

	charset |
	charset_not => 'cmd: {
	  /* Whether matching against a unibyte character.  */
	  let mut unibyte_char = false;

	  DEBUG_PRINT! (c"EXECUTING charset%s.\n".as_ptr (),
		       if *p.offset (-1) as re_opcode_t == charset_not
		       { c"_not".as_ptr () } else { c"".as_ptr () });

	  PREFETCH! ();
	  if goto_fail { break 'cmd; }
	  let mut len: c_int = 0;
	  let corig: c_int = RE_STRING_CHAR_AND_LENGTH! (d, len, target_multibyte);
	  let mut c: c_int = corig;
	  if target_multibyte
	    {
	      let mut c1: c_int;

	      c = TRANSLATE! (c);
	      c1 = RE_CHAR_TO_UNIBYTE! (c);
	      if c1 >= 0
		{
		  unibyte_char = true;
		  c = c1;
		}
	    }
	  else
	    {
	      let mut c1: c_int = RE_CHAR_TO_MULTIBYTE! (c);

	      if ! CHAR_BYTE8_P (c1)
		{
		  c1 = TRANSLATE! (c1);
		  c1 = RE_CHAR_TO_UNIBYTE! (c1);
		  if c1 >= 0
		    {
		      unibyte_char = true;
		      c = c1;
		    }
		}
	      else
		{ unibyte_char = true; }
	    }

	  p = p.offset (-1);
	  if !execute_charset (&raw mut p, c, corig, unibyte_char, translate)
	    { goto_fail = true; break 'cmd; }

	  d = d.offset (len as isize);
	  nchars += 1;
	}

	/* The beginning of a group is represented by start_memory.
	   The argument is the register number.  The text
	   matched within the group is recorded (in the internal
	   registers data structure) under the register number.  */
	start_memory => {
	  DEBUG_PRINT! (c"EXECUTING start_memory %d:\n".as_ptr (), *p as c_int);
	  eassert! (0 < *p as c_int && (*p as c_int) < num_regs as c_int);

	  /* In case we need to undo this operation (via backtracking).  */
	  PUSH_FAILURE_REG! (*p as c_int);

	  *regstart.offset (*p as isize) = d;
	  DEBUG_PRINT! (c"  regstart: %td\n".as_ptr (),
			POINTER_TO_OFFSET! (*regstart.offset (*p as isize)));

	  /* Move past the register number and inner group count.  */
	  p = p.offset (1);
	}

	/* The stop_memory opcode represents the end of a group.  Its
	   argument is the same as start_memory's: the register number.  */
	stop_memory => {
	  DEBUG_PRINT! (c"EXECUTING stop_memory %d:\n".as_ptr (), *p as c_int);

	  eassert! (0 < *p as c_int && (*p as c_int) < num_regs as c_int);
	  eassert! (!REG_UNSET! (*regstart.offset (*p as isize)));
	  /* Strictly speaking, there should be code such as:

		eassert (REG_UNSET (regend[*p]));
		PUSH_FAILURE_REGSTOP (*p);

	     But the only info to be pushed is regend[*p] and it is known to
	     be UNSET, so there really isn't anything to push.
	     Not pushing anything, on the other hand deprives us from the
	     guarantee that regend[*p] is UNSET since undoing this operation
	     will not reset its value properly.  This is not important since
	     the value will only be read on the next start_memory or at
	     the very end and both events can only happen if this stop_memory
	     is *not* undone.  */

	  *regend.offset (*p as isize) = d;
	  DEBUG_PRINT! (c"      regend: %td\n".as_ptr (),
			POINTER_TO_OFFSET! (*regend.offset (*p as isize)));

	  /* Move past the register number and the inner group count.  */
	  p = p.offset (1);
	}

	/* \<digit> has been turned into a 'duplicate' command which is
	   followed by the numeric value of <digit> as the register number.  */
	duplicate => 'cmd: {
	  let mut d2: *const re_char;
	  let mut dend2: *const re_char;
	  let regno: c_int = *p as c_int; p = p.offset (1);	/* Get which register to match against.  */
	  DEBUG_PRINT! (c"EXECUTING duplicate %d.\n".as_ptr (), regno);

	  /* Can't back reference a group which we've never matched.  */
	  eassert! (0 < regno && regno < num_regs as c_int);
	  eassert! (REG_UNSET! (*regstart.offset (regno as isize))
		    <= REG_UNSET! (*regend.offset (regno as isize)));
	  if REG_UNSET! (*regend.offset (regno as isize))
	    { goto_fail = true; break 'cmd; }

	  /* Where in input to try to start matching.  */
	  d2 = *regstart.offset (regno as isize);

	  /* Remember the start point to rollback upon failure.  */
	  dfail = d;

	  /* Where to stop matching; if both the place to start and
	     the place to stop matching are in the same string, then
	     set to the place to stop, otherwise, for now have to use
	     the end of the first string.  */

	  dend2 = if FIRST_STRING_P! (*regstart.offset (regno as isize))
		     == FIRST_STRING_P! (*regend.offset (regno as isize))
		   { *regend.offset (regno as isize) } else { end_match_1 };
	  loop
	    {
	      let mut dcnt: ptrdiff_t;

	      /* If necessary, advance to next segment in register
		 contents.  */
	      while d2 == dend2
		{
		  if dend2 == end_match_2 { break; }
		  if dend2 == *regend.offset (regno as isize) { break; }

		  /* End of string1 => advance to string2. */
		  d2 = string2;
		  dend2 = *regend.offset (regno as isize);
		}
	      /* At end of register contents => success */
	      if d2 == dend2 { break; }

	      /* If necessary, advance to next segment in data.  */
	      PREFETCH! (d = dfail);
	      if goto_fail { break 'cmd; }

	      /* How many characters left in this segment to match.  */
	      dcnt = dend.offset_from (d);

	      /* Want how many consecutive characters we can match in
		 one shot, so, if necessary, adjust the count.  */
	      if dcnt > dend2.offset_from (d2)
		{ dcnt = dend2.offset_from (d2); }

	      /* Compare that many; failure if mismatch, else move
		 past them.  */
	      if if !NILP (translate)
		   { bcmp_translate (d, d2, dcnt, translate, target_multibyte) }
		 else { memcmp (d as *const c_void, d2 as *const c_void,
				dcnt as usize) != 0 }
		{
		  d = dfail;
		  goto_fail = true;
		  break 'cmd;
		}
	      d = d.offset (dcnt);
	      d2 = d2.offset (dcnt);
	      nchars += 1;
	    }
	}

	/* begline matches the empty string at the beginning of the string,
	   and after newlines.  */
	begline => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING begline.\n".as_ptr ());

	  if AT_STRINGS_BEG! (d) { }
	  else
	    {
	      let mut c: c_uint = 0;
	      GET_CHAR_BEFORE_2! (c, d, string1, end1, string2, end2);
	      if c != '\n' as c_uint
		{ goto_fail = true; break 'cmd; }
	    }
	}

	/* endline is the dual of begline.  */
	endline => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING endline.\n".as_ptr ());

	  if !AT_STRINGS_END! (d)
	    {
	      PREFETCH_NOLIMIT! ();
	      if *d != '\n' as re_char
		{ goto_fail = true; break 'cmd; }
	    }
	}

	/* Match at the very beginning of the data.  */
	begbuf => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING begbuf.\n".as_ptr ());
	  if !AT_STRINGS_BEG! (d)
	    { goto_fail = true; break 'cmd; }
	}

	/* Match at the very end of the data.  */
	endbuf => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING endbuf.\n".as_ptr ());
	  if !AT_STRINGS_END! (d)
	    { goto_fail = true; break 'cmd; }
	}

	/* on_failure_keep_string_jump is used to optimize '.*\n'.  It
	   pushes NULL as the value for the string on the stack.  Then
	   'POP_FAILURE_POINT' will keep the current value for the
	   string, instead of restoring it.  To see why, consider
	   matching 'foo\nbar' against '.*\n'.  The .* matches the foo;
	   then the . fails against the \n.  But the next thing we want
	   to do is match the \n against the \n; if we restored the
	   string value, we would be back at the foo.

	   Because this is used only in specific cases, we don't need to
	   check all the things that 'on_failure_jump' does, to make
	   sure the right things get saved on the stack.  Hence we don't
	   share its code.  The only reason to push anything on the
	   stack at all is that otherwise we would have to change
	   'anychar's code to do something besides goto fail in this
	   case; that seems worse than this.  */
	on_failure_keep_string_jump => {
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  DEBUG_PRINT! (c"EXECUTING on_failure_keep_string_jump %d (to %p):\n".as_ptr (),
			mcnt, p.offset (mcnt as isize));

	  PUSH_FAILURE_POINT! (p.offset (-3), std::ptr::null ());
	}

	/* A nasty loop is introduced by the non-greedy *? and +?.
	   ...  */
	on_failure_jump_nastyloop => {
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  DEBUG_PRINT! (c"EXECUTING on_failure_jump_nastyloop %d (to %p):\n".as_ptr (),
			mcnt, p.offset (mcnt as isize));

	  eassert! (*p.offset (-4) as re_opcode_t == no_op);
	  {
	    cycle = false;
	    CHECK_INFINITE_LOOP! (p.offset (-4), d);
	    if !cycle
	      /* If there's a cycle, just continue without pushing
		 this failure point.  The failure point is the "try again"
		 option, which shouldn't be tried.
		 We want (x?)*?y\1z to match both xxyz and xxyxz.  */
	      { PUSH_FAILURE_POINT! (p.offset (-3), d); }
	  }
	}

	  /* Simple loop detecting on_failure_jump:  just check on the
	     failure stack if the same spot was already hit earlier.  */
	on_failure_jump_loop => {
	  goto_on_failure = true;
	}

	/* Uses of on_failure_jump:
	   ... */
	on_failure_jump => {
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  DEBUG_PRINT! (c"EXECUTING on_failure_jump %d (to %p):\n".as_ptr (),
			mcnt, p.offset (mcnt as isize));

	  PUSH_FAILURE_POINT! (p.offset (-3), d);
	}

	/* This operation is used for greedy *.  ...  */
	on_failure_jump_smart => {
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  DEBUG_PRINT! (c"EXECUTING on_failure_jump_smart %d (to %p).\n".as_ptr (),
			mcnt, p.offset (mcnt as isize));
	  {
	    let p1: *const re_char = p; /* Next operation.  */
	    /* Discard 'const', making re_search non-reentrant.  */
	    let p2: *mut u8 = p as *mut u8; /* Jump dest.  */
	    let p2 = p2.offset (mcnt as isize);
	    let p3: *mut u8 = (p as *mut u8).offset (-3); /* opcode location.  */

	    p = p.offset (-3);		/* Reset so that we will re-execute the
					   instruction once it's been changed. */

	    EXTRACT_NUMBER! (mcnt, p2.offset (-2) as *const re_char);

	    /* Ensure this is indeed the trivial kind of loop
	       we are expecting.  */
	    eassert! (skip_one_char (p1) == p2.offset (-3) as *const re_char);
	    eassert! (*p2.offset (-3) as re_opcode_t == jump
		      && p2.offset (mcnt as isize) as *const re_char == p);
	    DEBUG_STATEMENT! (regex_emacs_debug += 2);
	    if mutually_exclusive_p (bufp, p1, p2 as *const re_char)
	      {
		/* Use a fast 'on_failure_keep_string_jump' loop.  */
		DEBUG_PRINT! (c"  smart exclusive => fast loop.\n".as_ptr ());
		*p3 = on_failure_keep_string_jump as u8;
		STORE_NUMBER (p2.offset (-2), mcnt + 3);
	      }
	    else
	      {
		/* Default to a safe 'on_failure_jump' loop.  */
		DEBUG_PRINT! (c"  smart default => slow loop.\n".as_ptr ());
		*p3 = on_failure_jump as u8;
	      }
	    DEBUG_STATEMENT! (regex_emacs_debug -= 2);
	  }
	}

	/* Unconditionally jump (without popping any failure points).  */
	jump => {
	  goto_unconditional_jump = true;
	}

	/* Have to succeed matching what follows at least n times.
	   After that, handle like 'on_failure_jump_loop'.  */
	succeed_n => 'cmd: {
	  /* Signedness doesn't matter since we only compare MCNT to 0.  */
	  EXTRACT_NUMBER! (mcnt, p.offset (2));
	  DEBUG_PRINT! (c"EXECUTING succeed_n %d.\n".as_ptr (), mcnt);

	  /* Originally, mcnt is how many times we HAVE to succeed.  */
	  if mcnt != 0
	    {
	      /* Discard 'const', making re_search non-reentrant.  */
	      let p2: *mut u8 = p.offset (2) as *mut u8; /* counter loc.  */
	      mcnt -= 1;
	      p = p.offset (4);
	      PUSH_NUMBER! (p2, mcnt);
	    }
	  else
	    {
	    /* The two bytes encoding mcnt == 0 are two no_op opcodes.  */
	    goto_on_failure = true;
	    break 'cmd;
	    }
	}

	jump_n => 'cmd: {
	  /* Signedness doesn't matter since we only compare MCNT to 0.  */
	  EXTRACT_NUMBER! (mcnt, p.offset (2));
	  DEBUG_PRINT! (c"EXECUTING jump_n %d.\n".as_ptr (), mcnt);

	  /* Originally, this is how many times we CAN jump.  */
	  if mcnt != 0
	    {
	      /* Discard 'const', making re_search non-reentrant.  */
	      let p2: *mut u8 = p.offset (2) as *mut u8; /* counter loc.  */
	      mcnt -= 1;
	      PUSH_NUMBER! (p2, mcnt);
	      goto_unconditional_jump = true;
	      break 'cmd;
	    }
	  /* If don't have to jump any more, skip over the rest of command.  */
	  else
	    { p = p.offset (4); }
	}

	set_number_at => {
	  let p2: *mut u8;	/* Location of the counter.  */
	  DEBUG_PRINT! (c"EXECUTING set_number_at.\n".as_ptr ());

	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  /* Discard 'const', making re_search non-reentrant.  */
	  p2 = p as *mut u8;
	  let p2 = p2.offset (mcnt as isize);
	  /* Signedness doesn't matter since we only copy MCNT's bits.  */
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  DEBUG_PRINT! (c"  Setting %p to %d.\n".as_ptr (), p2, mcnt);
	  PUSH_NUMBER! (p2, mcnt);
	}

	wordbound |
	notwordbound => 'cmd: {
	  let mut not: bool = *p.offset (-1) as re_opcode_t == notwordbound;
	  DEBUG_PRINT! (c"EXECUTING %swordbound.\n".as_ptr (),
			if not { c"not".as_ptr () } else { c"".as_ptr () });

	  /* We SUCCEED (or FAIL) in one of the following cases: */

	  /* Case 1: D is at the beginning or the end of string.  */
	  if AT_STRINGS_BEG! (d) || AT_STRINGS_END! (d)
	    { not = !not; }
	  else
	    {
	      /* C1 is the character before D, S1 is the syntax of C1, C2
		 is the character at D, and S2 is the syntax of C2.  */
	      let mut c1: c_int = 0;
	      let mut c2: c_int = 0;
	      let mut s1: c_int = 0;
	      let mut s2: c_int = 0;
	      let mut dummy: c_int = 0;
              let offset: ptrdiff_t = POINTER_TO_OFFSET! (d);
              let charpos: ptrdiff_t = RE_SYNTAX_TABLE_BYTE_TO_CHAR (offset) - 1;
	      UPDATE_SYNTAX_TABLE (charpos);
	      GET_CHAR_BEFORE_2! (c1, d, string1, end1, string2, end2);
	      nchars += 1;
	      s1 = SYNTAX! (c1) as c_int;
	      UPDATE_SYNTAX_TABLE_FORWARD (charpos + 1);
	      PREFETCH_NOLIMIT! ();
	      GET_CHAR_AFTER! (c2, d, dummy);
	      nchars += 1;
	      s2 = SYNTAX! (c2) as c_int;

	      if /* Case 2: Only one of S1 and S2 is Sword.  */
		  ((s1 == Sword) != (s2 == Sword))
		  /* Case 3: Both of S1 and S2 are Sword, and macro
		     WORD_BOUNDARY_P (C1, C2) returns nonzero.  */
		  || ((s1 == Sword) && WORD_BOUNDARY_P (c1, c2))
		{ not = !not; }
	    }
	  if !not
	    { goto_fail = true; break 'cmd; }
	}

	wordbeg => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING wordbeg.\n".as_ptr ());

	  /* We FAIL in one of the following cases: */

	  /* Case 1: D is at the end of string.  */
	  if AT_STRINGS_END! (d)
	    { goto_fail = true; break 'cmd; }
	  else
	    {
	      /* C1 is the character before D, S1 is the syntax of C1, C2
		 is the character at D, and S2 is the syntax of C2.  */
	      let mut c1: c_int = 0;
	      let mut c2: c_int = 0;
	      let mut s1: c_int = 0;
	      let mut s2: c_int = 0;
	      let mut dummy: c_int = 0;
	      let offset: ptrdiff_t = POINTER_TO_OFFSET! (d);
	      let charpos: ptrdiff_t = RE_SYNTAX_TABLE_BYTE_TO_CHAR (offset);
	      UPDATE_SYNTAX_TABLE (charpos);
	      PREFETCH! ();
	      if goto_fail { break 'cmd; }
	      GET_CHAR_AFTER! (c2, d, dummy);
	      nchars += 1;
	      s2 = SYNTAX! (c2) as c_int;

	      /* Case 2: S2 is not Sword. */
	      if s2 != Sword
		{ goto_fail = true; break 'cmd; }

	      /* Case 3: D is not at the beginning of string ... */
	      if !AT_STRINGS_BEG! (d)
		{
		  GET_CHAR_BEFORE_2! (c1, d, string1, end1, string2, end2);
		  nchars += 1;
		  UPDATE_SYNTAX_TABLE_BACKWARD (charpos - 1);
		  s1 = SYNTAX! (c1) as c_int;

		  /* ... and S1 is Sword, and WORD_BOUNDARY_P (C1, C2)
		     returns 0.  */
		  if (s1 == Sword) && !WORD_BOUNDARY_P (c1, c2)
		    { goto_fail = true; break 'cmd; }
		}
	    }
	}

	wordend => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING wordend.\n".as_ptr ());

	  /* We FAIL in one of the following cases: */

	  /* Case 1: D is at the beginning of string.  */
	  if AT_STRINGS_BEG! (d)
	    { goto_fail = true; break 'cmd; }
	  else
	    {
	      /* C1 is the character before D, S1 is the syntax of C1, C2
		 is the character at D, and S2 is the syntax of C2.  */
	      let mut c1: c_int = 0;
	      let mut c2: c_int = 0;
	      let mut s1: c_int = 0;
	      let mut s2: c_int = 0;
	      let mut dummy: c_int = 0;
              let offset: ptrdiff_t = POINTER_TO_OFFSET! (d);
              let charpos: ptrdiff_t = RE_SYNTAX_TABLE_BYTE_TO_CHAR (offset) - 1;
	      UPDATE_SYNTAX_TABLE (charpos);
	      GET_CHAR_BEFORE_2! (c1, d, string1, end1, string2, end2);
	      nchars += 1;
	      s1 = SYNTAX! (c1) as c_int;

	      /* Case 2: S1 is not Sword.  */
	      if s1 != Sword
		{ goto_fail = true; break 'cmd; }

	      /* Case 3: D is not at the end of string ... */
	      if !AT_STRINGS_END! (d)
		{
		  PREFETCH_NOLIMIT! ();
		  GET_CHAR_AFTER! (c2, d, dummy);
		  nchars += 1;
                  UPDATE_SYNTAX_TABLE_FORWARD (charpos + 1);
		  s2 = SYNTAX! (c2) as c_int;

		  /* ... and S2 is Sword, and WORD_BOUNDARY_P (C1, C2)
		     returns 0.  */
		  if (s2 == Sword) && !WORD_BOUNDARY_P (c1, c2)
		    { goto_fail = true; break 'cmd; }
		}
	    }
	}

	symbeg => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING symbeg.\n".as_ptr ());

	  /* We FAIL in one of the following cases: */

	  /* Case 1: D is at the end of string.  */
	  if AT_STRINGS_END! (d)
	    { goto_fail = true; break 'cmd; }
	  else
	    {
	      /* C1 is the character before D, S1 is the syntax of C1, C2
		 is the character at D, and S2 is the syntax of C2.  */
	      let mut c1: c_int = 0;
	      let mut c2: c_int = 0;
	      let mut s1: c_int = 0;
	      let mut s2: c_int = 0;
	      let offset: ptrdiff_t = POINTER_TO_OFFSET! (d);
	      let charpos: ptrdiff_t = RE_SYNTAX_TABLE_BYTE_TO_CHAR (offset);
	      UPDATE_SYNTAX_TABLE (charpos);
	      PREFETCH! ();
	      if goto_fail { break 'cmd; }
	      c2 = RE_STRING_CHAR! (d, target_multibyte);
	      nchars += 1;
	      s2 = SYNTAX! (c2) as c_int;

	      /* Case 2: S2 is neither Sword nor Ssymbol. */
	      if s2 != Sword && s2 != Ssymbol
		{ goto_fail = true; break 'cmd; }

	      /* Case 3: D is not at the beginning of string ... */
	      if !AT_STRINGS_BEG! (d)
		{
		  GET_CHAR_BEFORE_2! (c1, d, string1, end1, string2, end2);
		  nchars += 1;
		  UPDATE_SYNTAX_TABLE_BACKWARD (charpos - 1);
		  s1 = SYNTAX! (c1) as c_int;

		  /* ... and S1 is Sword or Ssymbol.  */
		  if s1 == Sword || s1 == Ssymbol
		    { goto_fail = true; break 'cmd; }
		}
	    }
	}

	symend => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING symend.\n".as_ptr ());

	  /* We FAIL in one of the following cases: */

	  /* Case 1: D is at the beginning of string.  */
	  if AT_STRINGS_BEG! (d)
	    { goto_fail = true; break 'cmd; }
	  else
	    {
	      /* C1 is the character before D, S1 is the syntax of C1, C2
		 is the character at D, and S2 is the syntax of C2.  */
	      let mut c1: c_int = 0;
	      let mut c2: c_int = 0;
	      let mut s1: c_int = 0;
	      let mut s2: c_int = 0;
              let offset: ptrdiff_t = POINTER_TO_OFFSET! (d);
              let charpos: ptrdiff_t = RE_SYNTAX_TABLE_BYTE_TO_CHAR (offset) - 1;
	      UPDATE_SYNTAX_TABLE (charpos);
	      GET_CHAR_BEFORE_2! (c1, d, string1, end1, string2, end2);
	      nchars += 1;
	      s1 = SYNTAX! (c1) as c_int;

	      /* Case 2: S1 is neither Ssymbol nor Sword.  */
	      if s1 != Sword && s1 != Ssymbol
		{ goto_fail = true; break 'cmd; }

	      /* Case 3: D is not at the end of string ... */
	      if !AT_STRINGS_END! (d)
		{
		  PREFETCH_NOLIMIT! ();
		  c2 = RE_STRING_CHAR! (d, target_multibyte);
		  nchars += 1;
		  UPDATE_SYNTAX_TABLE_FORWARD (charpos + 1);
		  s2 = SYNTAX! (c2) as c_int;

		  /* ... and S2 is Sword or Ssymbol.  */
		  if s2 == Sword || s2 == Ssymbol
		    { goto_fail = true; break 'cmd; }
		}
	    }
	}

	syntaxspec |
	notsyntaxspec => 'cmd: {
	  let not: bool = *p.offset (-1) as re_opcode_t == notsyntaxspec;
	  mcnt = *p as c_int; p = p.offset (1);
	  DEBUG_PRINT! (c"EXECUTING %ssyntaxspec %d.\n".as_ptr (),
			if not { c"not".as_ptr () } else { c"".as_ptr () },
			mcnt);
	  PREFETCH! ();
	  if goto_fail { break 'cmd; }
	  {
	    let offset: ptrdiff_t = POINTER_TO_OFFSET! (d);
	    let pos1: ptrdiff_t = RE_SYNTAX_TABLE_BYTE_TO_CHAR (offset);
	    UPDATE_SYNTAX_TABLE (pos1);
	  }
	  {
	    let mut len: c_int = 0;
	    let mut c: c_int = 0;

	    GET_CHAR_AFTER! (c, d, len);
	    if (SYNTAX! (c) != mcnt as syntaxcode) != not
	      { goto_fail = true; break 'cmd; }
	    d = d.offset (len as isize);
	    nchars += 1;
	  }
	}

	at_dot => 'cmd: {
	  DEBUG_PRINT! (c"EXECUTING at_dot.\n".as_ptr ());
	  if PTR_BYTE_POS (d) != PT_BYTE! ()
	    { goto_fail = true; break 'cmd; }
	}

	categoryspec |
	notcategoryspec => 'cmd: {
	  let not: bool = *p.offset (-1) as re_opcode_t == notcategoryspec;
	  mcnt = *p as c_int; p = p.offset (1);
	  DEBUG_PRINT! (c"EXECUTING %scategoryspec %d.\n".as_ptr (),
			if not { c"not".as_ptr () } else { c"".as_ptr () },
			mcnt);
	  PREFETCH! ();
	  if goto_fail { break 'cmd; }

	  {
	    let mut len: c_int = 0;
	    let mut c: c_int = 0;
	    GET_CHAR_AFTER! (c, d, len);
	    if (!CHAR_HAS_CATEGORY (c, mcnt)) != not
	      { goto_fail = true; break 'cmd; }
	    d = d.offset (len as isize);
	    nchars += 1;
	  }
	}

	_ => {
	  emacs_abort ();
	}
      }

      /* C's `on_failure:` (C 4826) and `unconditional_jump:` (C 4912) labels: both are
	 jumped to from a second opcode, so the port tests the two flags here, where C's labels
	 are (PORTING.md 2.2).  */
      if goto_on_failure
	{
	  /* on_failure: */
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);
	  DEBUG_PRINT! (c"EXECUTING on_failure_jump_loop %d (to %p):\n".as_ptr (),
			mcnt, p.offset (mcnt as isize));
	  {
	    cycle = false;
	    CHECK_INFINITE_LOOP! (p.offset (-3), d);
	    if cycle
	      /* If there's a cycle, get out of the loop, as if the matching
		 had failed.  We used to just 'goto fail' here, but that was
		 aborting the search a bit too early: we want to keep the
		 empty-loop-match and keep matching after the loop.
		 We want (x?)*y\1z to match both xxyz and xxyxz.  */
	      { p = p.offset (mcnt as isize); }
	    else
	      { PUSH_FAILURE_POINT! (p.offset (-3), d); }
	  }
	}

      if goto_unconditional_jump
	{
	  /* unconditional_jump: */
	  maybe_quit ();
	  EXTRACT_NUMBER_AND_INCR! (mcnt, p);	/* Get the amount to jump.  */
	  DEBUG_PRINT! (c"EXECUTING jump %d ".as_ptr (), mcnt);
	  p = p.offset (mcnt as isize);				/* Do the jump.  */
	  DEBUG_PRINT! (c"(to %p).\n".as_ptr (), p);
	}
	} /* `if !goto_fail` around the dispatch */

      if do_succeed
	{ break 'matcher; }

      if !goto_fail
	{ continue 'matcher; }  /* Successfully executed one pattern command; keep going.  */

    /* We goto here if a matching operation fails.
       C's `fail:` label is the `goto_fail` flag (PORTING.md 2.2).  A popped `no_op` frame jumps
       back to that same label (C 5287-5289), so the block repeats on itself (P-170).  */
      'fail: loop
	{
      maybe_quit ();
      if !FAIL_STACK_EMPTY! ()
	{
	  let mut goto_continue_failure_jump = false;
	  let mut fallthrough_to_failure_jump = false;
	  let mut str: *const re_char;
	  let mut pat: *const re_char;
	  /* A restart point is known.  Restore to that state.  */
	  DEBUG_PRINT! (c"\nFAIL:\n".as_ptr ());
	  POP_FAILURE_POINT! (str, pat);
	  match { let op = *pat as re_opcode_t; pat = pat.offset (1); op }
	    {
	      on_failure_keep_string_jump => {
		eassert! (str.is_null ());
		goto_continue_failure_jump = true;
	      }

	      on_failure_jump_nastyloop => {
		eassert! (*pat.offset (-2) as re_opcode_t == no_op);
		PUSH_FAILURE_POINT! (pat.offset (-2), str);
		fallthrough_to_failure_jump = true;
		d = str;
	      }

	      on_failure_jump_loop |
	      on_failure_jump |
	      succeed_n => {
		d = str;
		/* C 5254-5280: the popped frame's opcode decides what `d` becomes; the segment is
		   recomputed from `d` after the switch (C 5286-5287), never here.  */
		goto_continue_failure_jump = true;
	      }

	      no_op => {
		/* A special frame used for nastyloops.  C 5287-5289: `goto fail` re-enters the pop
		   block above, it does not go back to the opcode dispatch.  */
		continue 'fail;
	      }

	      _ => {
		emacs_abort ();
	      }
	    }

	  /* Instrument (P-170): what a popped frame restores, before the segment is recomputed
	     (C 5286-5287).  `FEMACS_REG_TRACE` only.  */
	  if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
	    {
	      fprintf (stderr,
		       c"REGTRACE pop op=%d strnull=%d str-s1=%td d-s1=%td dend-s1=%td em1-s1=%td size1=%td\n".as_ptr (),
		       *pat.offset (-1) as c_int,
		       str.is_null () as c_int,
		       (str as isize) - (string1 as isize),
		       (d as isize) - (string1 as isize),
		       (dend as isize) - (string1 as isize),
		       (end_match_1 as isize) - (string1 as isize),
		       (end1 as isize) - (string1 as isize));
	    }
	  if goto_continue_failure_jump || fallthrough_to_failure_jump
	    {
	      /* continue_failure_jump: */
	      p = extract_address (pat);
	    }

	  /* C 5286-5287: the segment is recomputed from the restored `d` and then the matcher
	     continues with the next opcode.  The `goto_fail` flag that *entered* this block is
	     still set here, so testing it again would skip the recompute on every pop (P-170).  */
	  eassert! (p >= (*bufp).buffer as *const re_char && p <= pend);

	  if d >= string1 && d <= end1
	    { dend = end_match_1; }
	  /* Instrument (P-149): if the recompute above did not restore agreement, the pop put `d`
	     in the other segment and C's `if` cannot express that.  */
	  if size1 != 0 && ((dend == end_match_1) != ((d as usize) <= (end1 as usize)))
	    {
	      if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
		{
		  fprintf (stderr,
			   c"REGTRACE pop-mismatch str-s1=%td size1=%td d-s1=%td dend-s1=%td\n".as_ptr (),
			   (str as isize) - (string1 as isize),
			   (end1 as isize) - (string1 as isize),
			   (d as isize) - (string1 as isize),
			   (dend as isize) - (string1 as isize));
		}
	    }
	  continue 'matcher;
	}
      else
	{ failed = true; break 'matcher; }   /* Matching at this starting point really fails.  */
	} /* 'fail */
    } /* for (;;) */

  // C 5290-5291: `if (best_regs_set) goto restore_best_regs;` - only the exhausted-failure
  // exit reaches it (`failed`), so the succeed paths never re-enter the restore block.
  if failed && best_regs_set
    { do_restore = true; }

  // C 4337's `restore_best_regs:` block (reached by a backward jump in C) runs here when
  // `do_restore` is set, and `succeed_label:` (C 4359) follows it.
  if do_restore
    {
      /* Restore best match.  It may happen that 'dend ==
	 end_match_1' while the restored d is in string2.
	 For example, the pattern 'x.*y.*z' against the
	 strings 'x-' and 'y-z-', if the two strings are
	 not consecutive in memory.  */
      DEBUG_PRINT! (c"Restoring best registers.\n".as_ptr ());

      d = match_end;
      dend = if d >= string1 && d <= end1
	       { end_match_1 } else { end_match_2 };

      let mut reg: ptrdiff_t = 1;
      while reg < num_regs
	{
	  *regstart.offset (reg) = *best_regstart.offset (reg);
	  *regend.offset (reg) = *best_regend.offset (reg);
	  eassert! (REG_UNSET! (*regstart.offset (reg))
		    <= REG_UNSET! (*regend.offset (reg)));
	  reg += 1;
	}
    }

  // C 4359's `succeed_label:` has two ways in: the fall-through from `restore_best_regs:`
  // above (`do_restore`, C 4337) and the two `goto succeed_label` sites inside the loop -
  // the `succeed` opcode (C 4443) and the end-of-pattern fall-through - which the port
  // carries as `do_succeed`.  C's third exit from the loop, the exhausted failure stack
  // (C 5287), reaches neither: it lands on `endof_re_match:` (C 5293) with `retval` still -1
  // (C 4172).  Without this guard that exit fell into the block below and returned a bogus
  // success whose `dcnt` was read off wherever `d` happened to stand (status 3.620).
  if do_succeed || do_restore
    {
    /* succeed_label: */
    DEBUG_PRINT! (c"Accepting match.\n".as_ptr ());

    /* Instrument, not behaviour (P-115's shape): with `FEMACS_REG_TRACE` set, report where the
       accepted match stands relative to the two strings.  `d` outside `[string1, end2]` is the
       signature of a bogus acceptance (the `electric-tests` hang's `me = -10`, status 4.494).  */
    if ((d as isize) < (string1 as isize) || (d as isize) > (end2 as isize))
      {
        if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
          {
            fprintf (stderr,
                     c"REGTRACE accept-outside d-string1=%td pos=%td end1-string1=%td succ=%d restore=%d\n".as_ptr (),
                     (d as isize) - (string1 as isize), pos as c_int,
                     (end1 as isize) - (string1 as isize), do_succeed as c_int,
                     do_restore as c_int);
          }
      }

    /* If caller wants register contents data back, do it.  */
    if !regs.is_null ()
      {
        /* Have the register data arrays been allocated?	*/
        if regs_allocated_ (bufp) == REGS_UNALLOCATED as u8
          { /* No.  So allocate them with malloc.  */
            let n: ptrdiff_t = std::cmp::max (RE_NREGS as ptrdiff_t, num_regs);
            (*regs).start = xnmalloc (n, std::mem::size_of::<ptrdiff_t> () as isize) as *mut ptrdiff_t;
            (*regs).end = xnmalloc (n, std::mem::size_of::<ptrdiff_t> () as isize) as *mut ptrdiff_t;
            (*regs).num_regs = n;
            set_regs_allocated_ (bufp, REGS_REALLOCATE as u8);
          }
        else if regs_allocated_ (bufp) == REGS_REALLOCATE as u8
          { /* Yes.  If we need more elements than were already
               allocated, reallocate them.  If we need fewer, just
               leave it alone.  */
            let mut n: ptrdiff_t = (*regs).num_regs;
            if n < num_regs
              {
                n = std::cmp::max (n + (n >> 1), num_regs);
                (*regs).start
                  = xnrealloc ((*regs).start as *mut c_void, n,
                               std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *mut ptrdiff_t;
                (*regs).end = xnrealloc ((*regs).end as *mut c_void, n,
                                         std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *mut ptrdiff_t;
                (*regs).num_regs = n;
              }
          }
        else
          { eassert! (regs_allocated_ (bufp) == REGS_FIXED as u8); }

        /* Convert the pointer data in 'regstart' and 'regend' to
           indices.  Register zero has to be set differently,
           since we haven't kept track of any info for it.  */
        if (*regs).num_regs > 0
          {
            *(*regs).start.offset (0) = pos;
            *(*regs).end.offset (0) = POINTER_TO_OFFSET! (d);
          }

        let mut reg: ptrdiff_t = 1;
        while reg < num_regs
          {
            eassert! (REG_UNSET! (*regstart.offset (reg))
                      <= REG_UNSET! (*regend.offset (reg)));
            if REG_UNSET! (*regend.offset (reg))
              {
                *(*regs).start.offset (reg) = -1;
                *(*regs).end.offset (reg) = -1;
              }
            else
              {
                *(*regs).start.offset (reg) = POINTER_TO_OFFSET! (*regstart.offset (reg));
                *(*regs).end.offset (reg) = POINTER_TO_OFFSET! (*regend.offset (reg));
              }
            reg += 1;
          }

        /* If the regs structure we return has more elements than
           were in the pattern, set the extra elements to -1.  */
        let mut reg: ptrdiff_t = num_regs;
        while reg < (*regs).num_regs
          {
            *(*regs).start.offset (reg) = -1;
            *(*regs).end.offset (reg) = -1;
            reg += 1;
          }
      }

    DEBUG_PRINT! (c"%td failure points pushed, %td popped (%td remain).\n".as_ptr (),
                 nfailure_points_pushed, nfailure_points_popped,
                 nfailure_points_pushed - nfailure_points_popped);
    DEBUG_PRINT! (c"%td registers pushed.\n".as_ptr (), num_regs_pushed);

    let dcnt: ptrdiff_t = POINTER_TO_OFFSET! (d) - pos;

    if dcnt < -1 && ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
      {
        fprintf (stderr,
                 c"REGTRACE dcnt=%td d-s1=%td s1-s2=%td size1=%td pos=%td first=%d succ=%d restore=%d\n".as_ptr (),
                 dcnt as isize, (d as isize) - (string1 as isize),
                 (string1 as isize) - (string2 as isize),
                 (end1 as isize) - (string1 as isize), pos as isize,
                 FIRST_STRING_P! (d) as c_int, do_succeed as c_int, do_restore as c_int);
      }

    DEBUG_PRINT! (c"Returning %td from re_match_2.\n".as_ptr (), dcnt);

    retval = dcnt;
    }

  /* endof_re_match: */
  /* Instrument, not behaviour (P-115/P-146): the reference never returns a negative value other
     than -1 or -2, so report the state that produced one.  */
  if retval < -1
    {
      if ! getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null ()
        {
          fprintf (stderr,
                   c"REGTRACE bad-retval retval=%td pos=%td d-s1=%td s1-s2=%td end1-s2=%td end2-s2=%td\n".as_ptr (),
                   retval as isize, pos as isize,
                   (d as isize) - (string1 as isize),
                   (string1 as isize) - (string2 as isize),
                   (end1 as isize) - (string2 as isize),
                   (end2 as isize) - (string2 as isize));
        }
    }
  unbind_to (count, Qnil ());
  SAFE_FREE! (sa_count, sa_mark);

  /* The factor of 50 below is a heuristic that needs to be tuned.
     It means we consider 50 buffer positions examined by this function
     roughly equivalent to the display engine iterating over a single
     buffer position.  */
  if max_redisplay_ticks > 0 && nchars > 0
    { update_redisplay_ticks ((nchars / 50 + 1) as c_int, std::ptr::null_mut ()); }

  retval
} /* re_match_2_internal */

/* Subroutine definitions for re_match_2.  */

/* Return true if TRANSLATE[S1] and TRANSLATE[S2] are not identical
   for LEN bytes.  */

unsafe fn bcmp_translate (s1: *const re_char, s2: *const re_char, len: ptrdiff_t,
			  translate: Lisp_Object, target_multibyte: bool) -> bool
{
  /* Set C a (possibly converted to multibyte) character at P, and set
     LEN to the byte length of that character.  C's own macro is defined at C 86-96 (and the
     port defines a second copy inside `re_match_2_internal`, where the same name is a local
     there rather than a parameter; PORTING.md 2.2/P-57).  */
  macro_rules! GET_CHAR_AFTER {
    ($c:expr, $p:expr, $len:ident) => {
      {
        if target_multibyte
	  { $c = string_char_and_length ($p, &raw mut $len) as _; }
        else
	  {
	    $c = *$p as c_int as _;
	    $len = 1;
	    $c = RE_CHAR_TO_MULTIBYTE! ($c) as _;
	  }
       }
    };
  }

  let mut p1: *const re_char = s1;
  let mut p2: *const re_char = s2;
  let p1_end: *const re_char = s1.offset (len);
  let p2_end: *const re_char = s2.offset (len);

  /* FIXME: Checking both p1 and p2 presumes that the two strings might have
     different lengths, but relying on a single LEN would break this. -sm  */
  while p1 < p1_end && p2 < p2_end
    {
      let mut p1_charlen: c_int = 0;
      let mut p2_charlen: c_int = 0;
      let mut p1_ch: c_int = 0;
      let mut p2_ch: c_int = 0;

      GET_CHAR_AFTER! (p1_ch, p1, p1_charlen);
      GET_CHAR_AFTER! (p2_ch, p2, p2_charlen);

      if RE_TRANSLATE! (translate, p1_ch)
	  != RE_TRANSLATE! (translate, p2_ch)
	{ return true; }

      p1 = p1.offset (p1_charlen as isize);
      p2 = p2.offset (p2_charlen as isize);
    }

  p1 != p1_end || p2 != p2_end
}

/* Entry points for GNU code.  */

/* re_compile_pattern is the GNU regular expression compiler: it
   compiles PATTERN (of length SIZE) and puts the result in BUFP.
   Returns 0 if the pattern was valid, otherwise an error string.

   Assumes the 'allocated' (and perhaps 'buffer') and 'translate' fields
   are set in BUFP on entry.

   We call regex_compile to do the actual compilation.  */

/// C's `src/config.h` 3642-3663 renames `re_compile_pattern` to `rpl_re_compile_pattern` (gnulib's
/// replacement scheme), so the reference object exports `_rpl_re_compile_pattern` and this
/// definition must carry C's own symbol name (the `emacs_fclose` shape, status 3.68).
#[unsafe(export_name = "rpl_re_compile_pattern")]
pub unsafe extern "C" fn re_compile_pattern (pattern: *const c_char, length: ptrdiff_t,
					     posix_backtracking: bool,
					     whitespace_regexp: *const c_char,
					     bufp: *mut re_pattern_buffer)
  -> *const c_char
{
  set_regs_allocated_ (bufp, REGS_UNALLOCATED as u8);

  let ret: reg_errcode_t
      = regex_compile (pattern as *const re_char, length,
		       posix_backtracking,
		       whitespace_regexp,
		       bufp);

  if ret == 0
    { return std::ptr::null (); }
  re_error_msgid[ret as usize]
}
