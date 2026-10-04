// femacs: port of lib-src/ebrowse.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same
// scanner/parser state machine, the same `main`.  This is a *program* of the C tree's
// lib-src directory (batch F); the frozen configuration builds it (it is in the reference
// `lib-src/Makefile`'s `UTILITIES` list), and it links against the port's gnulib mirrors and
// libSystem only.
//
// **Dead text, recorded not transcribed** (P-08; `python3 tools/check-cond-eval.py
// lib-src/ebrowse.c` reports `4 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the
// frozen configuration decides all four and the ledger's live set is theirs):
//   * C 43-49's `#ifdef __MSDOS__` / `#elif defined WINDOWSNT` arms of `filename_eq` are
//     dead (neither name is defined), so the live body is C 48's `return streq (x, y);`;
//   * C 1346-1359's `#ifdef DEBUG` arm of `INCREMENT_LINENO` is dead, so the live
//     definition is C 1358's `#define INCREMENT_LINENO ++yyline`.
// Both are named in docs/ports/ebrowse.md rather than carried as text (P-08's
// record-in-the-ledger arm; P-51's alternative - the `#[cfg (feature = …)]` transcription -
// is for the units `xtask dead-arms` compiles, and neither arm here has a feature the tool
// knows).
//
// **Use-site handling** (P-26/P-72 (3)), because C names these from headers the port has no
// mirror for (or has a mirror that does not carry them):
//   * `<assert.h>`: the frozen configuration defines `NDEBUG` (macros.tsv 2892), so C's
//     `assert (cond)` expands to `((void) 0)` and *does not evaluate* `cond` (PORTING.md
//     2.5's release rule).  The port keeps C's name and call shape with the frozen body
//     (`macro_rules! assert` below);
//   * `<stdlib.h>`'s `atoi`, `exit` and `abort`, `<stdio.h>`'s `fseek`/`ftell`/`SEEK_END`;
//   * `<config.h>`'s `SEPCHAR`, `PACKAGE_VERSION` and `COPYRIGHT` (the config mirror carries
//     only the generated configuration keys - the `src/emacs.rs` 359 precedent);
//   * `<stddef.h>`'s `size_t`/`ptrdiff_t`, which this program has no `lisp.h` mirror to take
//     them from (the `lib-src/make-docfile.rs` 51 and `lib-src/hexl.rs` precedents);
//   * `<unlocked-io.h>` (C 34): the renames it performs on stdio calls are recorded at the
//     use site by every unit that needs them (docs/ports/dispnew.md 374, docs/ports/bidi.md
//     269), so this program calls `getc`/`putc`/`putchar`/`fputs` as C's text spells them.
//
// **Forward declarations dropped** (PORTING.md 0.1-12): C 463-470 declares eight helpers
// ahead of their definitions; Rust items are order-independent, so those declaration lines
// are gone and the definitions stay where C has them.
//
// **C's three `enum`s** (`token`, `sc`, `visibility`, C 76-205) are used as `int`s
// throughout - `static int tk = -1`, `tk == (T)`, `hash = (hash << 1) ^ VOID`, and
// `case 'x':` labels in the same `switch` - so the port keeps their members as `c_int`
// constants instead of a Rust enum (the shape `src/bytecode.rs` 331 gives `byte_code_op`'s
// members).  The values are C's, which `tools/extract-abi.py`'s constants gate compares.
//
// **Flexible array members** (`struct sym.name`, `struct member.name`, `struct alias.name`)
// are the `[c_char; 0]` the layout needs (PORTING.md 1.3).  C's `sym->name` decays to
// `char *`, which a zero-length Rust array cannot do, so every such use goes through
// `FAM_PTR!` - the `addr_of_mut!` idiom `src/include/lisp.rs` 1298's `fam_base`/`FAM_ELEM!`
// establish for exactly this shape.
//
// `ATTRIBUTE_MALLOC`, `ATTRIBUTE_FORMAT_PRINTF` and `_Noreturn` are attributes with no Rust
// counterpart (PORTING.md 8/P-13): the C names stay in the comments at their definitions,
// and `_Noreturn` is the port's `-> !`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_void};

/// `<stddef.h>`: `typedef … size_t;` - the frozen target's 64-bit unsigned type (measured:
/// 8 bytes).  This program has no `lisp.h` mirror, so it provides the name the way
/// `lib-src/make-docfile.rs` 51 does for `ptrdiff_t`.
pub type size_t = usize;

/// `<stddef.h>`: `typedef … ptrdiff_t;` (measured: 8 bytes).
pub type ptrdiff_t = isize;

/// `<stdio.h>`'s `SEEK_END` - C gets it from `<stdio.h>`, which the mirror does not carry
/// (measured: 2, the third `whence`).
const SEEK_END: c_int = 2;

/// generated `<config.h>` 2269: `#define SEPCHAR ':'` - the config mirror carries only the
/// generated configuration keys (the `src/emacs.rs` 359 precedent).
const SEPCHAR: c_char = b':' as c_char;

/// generated `<config.h>` 2214: `#define PACKAGE_VERSION "32.0.50"`.
const PACKAGE_VERSION: &core::ffi::CStr = c"32.0.50";

/// generated `<config.h>` 102: the `COPYRIGHT` key whose value `version` prints (C 3629).
/// It is C's own literal, carried in `docs/baseline/declarations-allowlist.tsv` with the
/// reason (status 3.73, the `src/emacs.rs` 361 precedent).
const COPYRIGHT: &core::ffi::CStr =
  c"Copyright (C) 2026 Free Software Foundation, Inc.";

/* `<assert.h>`: C 27 includes it, and C 914/1094 call `assert`.  The frozen configuration
   defines `NDEBUG` (macros.tsv 2892), so the C macro's live expansion is `((void) 0)`: the
   operand is not evaluated (PORTING.md 2.5).  The port keeps C's name, call shape and
   frozen body - the `lib-src/hexl.rs` PRIxMAX precedent for a macro the includes cannot
   supply.  */
macro_rules! assert {
  ($expr:expr) => { () };
}

// `<stdlib.h>`'s entries no mirror carries, declared at the use site (P-26/P-72 (3)).
unsafe extern "C" {
  /// `<stdlib.h>`: `int atoi (const char *);`
  pub fn atoi (nptr: *const c_char) -> c_int;
  /// `<stdlib.h>`: `_Noreturn void exit (int);`
  pub fn exit (status: c_int) -> !;
  /// `<stdlib.h>`: `_Noreturn void abort (void);`
  pub fn abort () -> !;
  /// `<stdio.h>`: `int fseek (FILE *, long, int);`
  pub fn fseek (stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
  /// `<stdio.h>`: `long ftell (FILE *);`
  pub fn ftell (stream: *mut FILE) -> c_long;
}

include!("../src/include/config.rs");        // C 21: `#include <config.h>`
include!("../lib/include/stdio.rs");         // C's `<stdio.h>` (via `<config.h>`'s conf_post.h)
include!("../lib/include/stdcountof.rs");    // C 23: `#include <stdcountof.h>`
include!("../lib/include/stdlib.rs");        // C 25: `#include <stdlib.h>`
include!("../lib/include/string.in.rs");        // C 26: `#include <string.h>`
include!("../lib/include/getopt.in.rs");     // C 28: `#include <getopt.h>`
include!("../lib/include/attribute.rs");     // C 30: `#include <attribute.h>`
include!("../lib/include/c-ctype.rs");       // C 31: `#include <c-ctype.h>`
include!("../lib/include/flexmember.rs");    // C 32: `#include <flexmember.h>`
include!("../lib/include/min-max.rs");       // C 33: `#include <min-max.h>`

/* C 24's `<stddef.h>` is the target's own header: the port spells `offsetof` and `sizeof`
   with `offset_of!`/`size_of` (flexmember.rs 5 says the same), and `size_t`/`ptrdiff_t`
   are the aliases above.  C 34's `<unlocked-io.h>` and C 29's `<attribute.h>`'s attributes
   are the use-site record at the head of this file. */

pub const READ_CHUNK_SIZE: c_int = 100 * 1024; // C 37-38: `enum { READ_CHUNK_SIZE = 100 * 1024 };` - C's anonymous enum member.

/* C 40-52.  */
/* Return true if X and Y are the same file name.  `__MSDOS__` and `WINDOWSNT` are
   undefined in the frozen configuration, so the live body is the `#else` arm (P-08).  */
unsafe fn filename_eq (x: *const c_char, y: *const c_char) -> bool
{
  return streq (x, y);
}

/* The default output file name.  */

/// C 54: `#define DEFAULT_OUTFILE "BROWSE"`
const DEFAULT_OUTFILE: &core::ffi::CStr = c"BROWSE";

/* A version string written to the output file.  Change this whenever
   the structure of the output file changes.  */

/// C 59: `#define EBROWSE_FILE_VERSION "ebrowse 5.0"`
const EBROWSE_FILE_VERSION: &core::ffi::CStr = c"ebrowse 5.0";

/* The output file consists of a tree of Lisp objects, with major
   nodes built out of Lisp structures.  These are the heads of the
   Lisp structs with symbols identifying their type.  */

/// C 65: `#define TREE_HEADER_STRUCT "[ebrowse-hs "`
const TREE_HEADER_STRUCT: &core::ffi::CStr = c"[ebrowse-hs ";
/// C 66: `#define TREE_STRUCT "[ebrowse-ts "`
const TREE_STRUCT: &core::ffi::CStr = c"[ebrowse-ts ";
/// C 67: `#define MEMBER_STRUCT "[ebrowse-ms "`
const MEMBER_STRUCT: &core::ffi::CStr = c"[ebrowse-ms ";
/// C 68: `#define CLASS_STRUCT "[ebrowse-cs "`
const CLASS_STRUCT: &core::ffi::CStr = c"[ebrowse-cs ";

/* The name of the symbol table entry for global functions, variables,
   defines etc.  This name also appears in the browser display.  */

/// C 73: `#define GLOBALS_NAME "*Globals*"`
const GLOBALS_NAME: &core::ffi::CStr = c"*Globals*";

/* Token definitions.  C 114-116's `#undef BOOL` / `#undef TRUE` / `#undef FALSE` guard
   against a system definition of those three names; the port's constants below are the
   only definitions, so the `#undef`s have no counterpart (the ntlib.rs 25-27 precedent).  */

/* C 76-197: `enum token { YYEOF = 0, CSTRING = 256, … };` - the members are `int`s here
   (see the head of this file).  The values are C's, member by member.  */

pub const YYEOF: c_int = 0; // C 78: `YYEOF = 0,`  /* end of file */
pub const CSTRING: c_int = 256; // C 79: `CSTRING = 256,`  /* string constant */
pub const CCHAR: c_int = 257; // C 80: `CCHAR,`  /* character constant */
pub const CINT: c_int = 258; // C 81: `CINT,`  /* integral constant */
pub const CFLOAT: c_int = 259; // C 82: `CFLOAT,`  /* real constant */
pub const ELLIPSIS: c_int = 260; // C 84: `ELLIPSIS,`  /* ... */
pub const LSHIFTASGN: c_int = 261; // C 85: `LSHIFTASGN,`  /* <<= */
pub const RSHIFTASGN: c_int = 262; // C 86: `RSHIFTASGN,`  /* >>= */
pub const ARROWSTAR: c_int = 263; // C 87: `ARROWSTAR,`  /* ->* */
pub const IDENT: c_int = 264; // C 88: `IDENT,`  /* identifier */
pub const DIVASGN: c_int = 265; // C 89: `DIVASGN,`  /* /= */
pub const INC: c_int = 266; // C 90: `INC,`  /* ++ */
pub const ADDASGN: c_int = 267; // C 91: `ADDASGN,`  /* += */
pub const DEC: c_int = 268; // C 92: `DEC,`  /* -- */
pub const ARROW: c_int = 269; // C 93: `ARROW,`  /* -> */
pub const SUBASGN: c_int = 270; // C 94: `SUBASGN,`  /* -= */
pub const MULASGN: c_int = 271; // C 95: `MULASGN,`  /* *= */
pub const MODASGN: c_int = 272; // C 96: `MODASGN,`  /* %= */
pub const LOR: c_int = 273; // C 97: `LOR,`  /* || */
pub const ORASGN: c_int = 274; // C 98: `ORASGN,`  /* |= */
pub const LAND: c_int = 275; // C 99: `LAND,`  /* && */
pub const ANDASGN: c_int = 276; // C 100: `ANDASGN,`  /* &= */
pub const XORASGN: c_int = 277; // C 101: `XORASGN,`  /* ^= */
pub const POINTSTAR: c_int = 278; // C 102: `POINTSTAR,`  /* .* */
pub const DCOLON: c_int = 279; // C 103: `DCOLON,`  /* :: */
pub const EQ: c_int = 280; // C 104: `EQ,`  /* == */
pub const NE: c_int = 281; // C 105: `NE,`  /* != */
pub const LE: c_int = 282; // C 106: `LE,`  /* <= */
pub const LSHIFT: c_int = 283; // C 107: `LSHIFT,`  /* << */
pub const GE: c_int = 284; // C 108: `GE,`  /* >= */
pub const RSHIFT: c_int = 285; // C 109: `RSHIFT,`  /* >> */

/* Keywords.  The undef's are there because these
   three symbols are very likely to be defined somewhere.  */

pub const ASM: c_int = 286; // C 118: `ASM,`  /* asm */
pub const AUTO: c_int = 287; // C 119: `AUTO,`  /* auto */
pub const BREAK: c_int = 288; // C 120: `BREAK,`  /* break */
pub const CASE: c_int = 289; // C 121: `CASE,`  /* case  */
pub const CATCH: c_int = 290; // C 122: `CATCH,`  /* catch */
pub const CHAR: c_int = 291; // C 123: `CHAR,`  /* char */
pub const CLASS: c_int = 292; // C 124: `CLASS,`  /* class */
pub const CONST: c_int = 293; // C 125: `CONST,`  /* const */
pub const CONTINUE: c_int = 294; // C 126: `CONTINUE,`  /* continue */
pub const DEFAULT: c_int = 295; // C 127: `DEFAULT,`  /* default */
pub const DELETE: c_int = 296; // C 128: `DELETE,`  /* delete */
pub const DO: c_int = 297; // C 129: `DO,`  /* do */
pub const DOUBLE: c_int = 298; // C 130: `DOUBLE,`  /* double */
pub const ELSE: c_int = 299; // C 131: `ELSE,`  /* else */
pub const ENUM: c_int = 300; // C 132: `ENUM,`  /* enum */
pub const EXTERN: c_int = 301; // C 133: `EXTERN,`  /* extern */
pub const FLOAT: c_int = 302; // C 134: `FLOAT,`  /* float */
pub const FOR: c_int = 303; // C 135: `FOR,`  /* for */
pub const FRIEND: c_int = 304; // C 136: `FRIEND,`  /* friend */
pub const GOTO: c_int = 305; // C 137: `GOTO,`  /* goto */
pub const IF: c_int = 306; // C 138: `IF,`  /* if */
pub const T_INLINE: c_int = 307; // C 139: `T_INLINE,`  /* inline */
pub const INT: c_int = 308; // C 140: `INT,`  /* int */
pub const LONG: c_int = 309; // C 141: `LONG,`  /* long */
pub const NEW: c_int = 310; // C 142: `NEW,`  /* new */
pub const OPERATOR: c_int = 311; // C 143: `OPERATOR,`  /* operator */
pub const PRIVATE: c_int = 312; // C 144: `PRIVATE,`  /* private */
pub const PROTECTED: c_int = 313; // C 145: `PROTECTED,`  /* protected */
pub const PUBLIC: c_int = 314; // C 146: `PUBLIC,`  /* public */
pub const REGISTER: c_int = 315; // C 147: `REGISTER,`  /* register */
pub const RETURN: c_int = 316; // C 148: `RETURN,`  /* return */
pub const SHORT: c_int = 317; // C 149: `SHORT,`  /* short */
pub const SIGNED: c_int = 318; // C 150: `SIGNED,`  /* signed */
pub const SIZEOF: c_int = 319; // C 151: `SIZEOF,`  /* sizeof */
pub const STATIC: c_int = 320; // C 152: `STATIC,`  /* static */
pub const STRUCT: c_int = 321; // C 153: `STRUCT,`  /* struct */
pub const SWITCH: c_int = 322; // C 154: `SWITCH,`  /* switch */
pub const TEMPLATE: c_int = 323; // C 155: `TEMPLATE,`  /* template */
pub const THIS: c_int = 324; // C 156: `THIS,`  /* this */
pub const THROW: c_int = 325; // C 157: `THROW,`  /* throw */
pub const TRY: c_int = 326; // C 158: `TRY,`  /* try */
pub const TYPEDEF: c_int = 327; // C 159: `TYPEDEF,`  /* typedef */
pub const UNION: c_int = 328; // C 160: `UNION,`  /* union */
pub const UNSIGNED: c_int = 329; // C 161: `UNSIGNED,`  /* unsigned */
pub const VIRTUAL: c_int = 330; // C 162: `VIRTUAL,`  /* virtual */
pub const VOID: c_int = 331; // C 163: `VOID,`  /* void */
pub const VOLATILE: c_int = 332; // C 164: `VOLATILE,`  /* volatile */
pub const WHILE: c_int = 333; // C 165: `WHILE,`  /* while */
pub const MUTABLE: c_int = 334; // C 166: `MUTABLE,`  /* mutable */
pub const BOOL: c_int = 335; // C 167: `BOOL,`  /* bool */
pub const TRUE: c_int = 336; // C 168: `TRUE,`  /* true */
pub const FALSE: c_int = 337; // C 169: `FALSE,`  /* false */
pub const SIGNATURE: c_int = 338; // C 170: `SIGNATURE,`  /* signature (GNU extension) */
pub const NAMESPACE: c_int = 339; // C 171: `NAMESPACE,`  /* namespace */
pub const EXPLICIT: c_int = 340; // C 172: `EXPLICIT,`  /* explicit */
pub const TYPENAME: c_int = 341; // C 173: `TYPENAME,`  /* typename */
pub const CONST_CAST: c_int = 342; // C 174: `CONST_CAST,`  /* const_cast */
pub const DYNAMIC_CAST: c_int = 343; // C 175: `DYNAMIC_CAST,`  /* dynamic_cast */
pub const REINTERPRET_CAST: c_int = 344; // C 176: `REINTERPRET_CAST,`  /* reinterpret_cast */
pub const STATIC_CAST: c_int = 345; // C 177: `STATIC_CAST,`  /* static_cast */
pub const TYPEID: c_int = 346; // C 178: `TYPEID,`  /* typeid */
pub const USING: c_int = 347; // C 179: `USING,`  /* using */
pub const WCHAR: c_int = 348; // C 180: `WCHAR,`  /* wchar_t */
pub const FINAL: c_int = 349; // C 181: `FINAL`  /* final */

/* Storage classes, in a wider sense.  */

pub const SC_UNKNOWN: c_int = 0; // C 188: `SC_UNKNOWN,`
pub const SC_MEMBER: c_int = 1; // C 189: `SC_MEMBER,`  /* Is an instance member.  */
pub const SC_STATIC: c_int = 2; // C 190: `SC_STATIC,`  /* Is static member.  */
pub const SC_FRIEND: c_int = 3; // C 191: `SC_FRIEND,`  /* Is friend function.  */
pub const SC_TYPE: c_int = 4; // C 192: `SC_TYPE`  /* Is a type definition.  */

/* Member visibility.  */

pub const V_PUBLIC: c_int = 0; // C 198: `V_PUBLIC,`
pub const V_PROTECTED: c_int = 1; // C 199: `V_PROTECTED,`
pub const V_PRIVATE: c_int = 2; // C 200: `V_PRIVATE`

/* Member flags.  */

pub const F_VIRTUAL: c_int = 1; // C 206: `#define F_VIRTUAL 1`  /* Is virtual function.  */
pub const F_INLINE: c_int = 2; // C 207: `#define F_INLINE 2`  /* Is inline function.  */
pub const F_CONST: c_int = 4; // C 208: `#define F_CONST 4`  /* Is const.  */
pub const F_PURE: c_int = 8; // C 209: `#define F_PURE 8`  /* Is pure virtual function.  */
pub const F_MUTABLE: c_int = 16; // C 210: `#define F_MUTABLE 16`  /* Is mutable.  */
pub const F_TEMPLATE: c_int = 32; // C 211: `#define F_TEMPLATE 32`  /* Is a template.  */
pub const F_EXPLICIT: c_int = 64; // C 212: `#define F_EXPLICIT 64`  /* Is explicit constructor.  */
pub const F_THROW: c_int = 128; // C 213: `#define F_THROW 128`  /* Has a throw specification.  */
pub const F_EXTERNC: c_int = 256; // C 214: `#define F_EXTERNC 256`  /* Is declared extern "C".  */
pub const F_DEFINE: c_int = 512; // C 215: `#define F_DEFINE 512`  /* Is a #define.  */

/* Set and test a bit in an int.  */

unsafe fn set_flag (f: *mut c_int, flag: c_int)
{
  *f |= flag;
}

unsafe fn has_flag (f: c_int, flag: c_int) -> bool
{
  return (f & flag) != 0;
}

/* Structure describing a class member.  */

#[repr(C)]
pub struct member
{
  pub next: *mut member,		/* Next in list of members.  */
  pub anext: *mut member,		/* Collision chain in member_table.  */
  pub list: *mut *mut member,		/* Pointer to list in class.  */
  pub param_hash: c_uint,		/* Hash value for parameter types.  */
  pub vis: c_int,			/* Visibility (public, ...).  */
  pub flags: c_int,			/* See F_* above.  */
  pub regexp: *mut c_char,		/* Matching regular expression.  */
  pub filename: *const c_char,		/* Don't free this shared string.  */
  pub pos: c_int,			/* Buffer position of occurrence.  */
  pub def_regexp: *mut c_char,		/* Regular expression matching definition.  */
  pub def_filename: *const c_char,	/* File name of definition.  */
  pub def_pos: c_int,			/* Buffer position of definition.  */
  pub name: [c_char; 0], /* Member name.  */
}

const _: () = core::assert! (std::mem::size_of::<member> () == 88);
const _: () = core::assert! (std::mem::offset_of! (member, param_hash) == 24);
const _: () = core::assert! (std::mem::offset_of! (member, regexp) == 40);
const _: () = core::assert! (std::mem::offset_of! (member, pos) == 56);
const _: () = core::assert! (std::mem::offset_of! (member, def_pos) == 80);
const _: () = core::assert! (std::mem::offset_of! (member, name) == 84);

/* Structures of this type are used to connect class structures with
   their super and subclasses.  */

#[repr(C)]
pub struct link
{
  pub sym: *mut sym,		/* The super or subclass.  */
  pub next: *mut link,		/* Next in list or NULL.  */
}

const _: () = core::assert! (std::mem::size_of::<link> () == 16);
const _: () = core::assert! (std::mem::offset_of! (link, next) == 8);

/* Structure used to record namespace aliases.  */

#[repr(C)]
pub struct alias
{
  pub next: *mut alias,		/* Next in list.  */
  pub namesp: *mut sym,		/* Namespace in which defined.  */
  pub aliasee: *mut link,	/* List of aliased namespaces (A::B::C...).  */
  pub name: [c_char; 0], /* Alias name.  */
}

const _: () = core::assert! (std::mem::size_of::<alias> () == 24);
const _: () = core::assert! (std::mem::offset_of! (alias, namesp) == 8);
const _: () = core::assert! (std::mem::offset_of! (alias, aliasee) == 16);
const _: () = core::assert! (std::mem::offset_of! (alias, name) == 24);

/* The structure used to describe a class in the symbol table,
   or a namespace in all_namespaces.  */

#[repr(C)]
pub struct sym
{
  pub flags: c_int,		/* Is class a template class?.  */
  pub visited: c_uchar,	/* Used to find circles.  */
  pub next: *mut sym,             /* Hash collision list.  */
  pub subs: *mut link,		/* List of subclasses.  */
  pub supers: *mut link,		/* List of superclasses.  */
  pub vars: *mut member,		/* List of instance variables.  */
  pub fns: *mut member,		/* List of instance functions.  */
  pub static_vars: *mut member,	/* List of static variables.  */
  pub static_fns: *mut member,	/* List of static functions.  */
  pub friends: *mut member,	/* List of friend functions.  */
  pub types: *mut member,		/* List of local types.  */
  pub regexp: *mut c_char,		/* Matching regular expression.  */
  pub pos: c_int,			/* Buffer position.  */
  pub filename: *const c_char,		/* File in which it can be found.  */
  pub sfilename: *const c_char, 	/* File in which members can be found.  */
  pub namesp: *mut sym,		/* Namespace in which defined. .  */
  pub name: [c_char; 0], /* Name of the class.  */
}

const _: () = core::assert! (std::mem::size_of::<sym> () == 120);
const _: () = core::assert! (std::mem::offset_of! (sym, visited) == 4);
const _: () = core::assert! (std::mem::offset_of! (sym, subs) == 16);
const _: () = core::assert! (std::mem::offset_of! (sym, types) == 72);
const _: () = core::assert! (std::mem::offset_of! (sym, regexp) == 80);
const _: () = core::assert! (std::mem::offset_of! (sym, filename) == 96);
const _: () = core::assert! (std::mem::offset_of! (sym, namesp) == 112);
const _: () = core::assert! (std::mem::offset_of! (sym, name) == 120);

/* The three flexible array members above are used as `char *` (C lets the array decay);
   a zero-length Rust array cannot do that, so every use goes through this macro, the
   `addr_of_mut!` idiom `src/include/lisp.rs` 1298's `fam_base`/`FAM_ELEM!` establish
   (PORTING.md 1.3).  */
macro_rules! FAM_PTR {
  ($fam:expr) => { core::ptr::addr_of_mut! ($fam) as *mut c_char };
}

/* Experimental: Print info for `--position-info'.  We print
   '(CLASS-NAME SCOPE MEMBER-NAME).  */

/// C 296: `#define P_DEFN 1`
const P_DEFN: c_int = 1;
/// C 297: `#define P_DECL  2`
const P_DECL: c_int = 2;

static mut info_where: c_int = 0;
static mut info_cls: *mut sym = std::ptr::null_mut ();
static mut info_member: *mut member = std::ptr::null_mut ();

/* Experimental.  For option `--position-info', the buffer position we
   are interested in.  When this position is reached, print out
   information about what we know about that point.  */

static mut info_position: c_int = -1;

/* Command line options structure for getopt_long.  */

static mut options: [option; 15] =
[
  option { name: c"append".as_ptr (),			has_arg: no_argument,	   flag: std::ptr::null_mut (), val: b'a' as c_int },
  option { name: c"files".as_ptr (),			has_arg: required_argument, flag: std::ptr::null_mut (), val: b'f' as c_int },
  option { name: c"help".as_ptr (),			has_arg: no_argument,	   flag: std::ptr::null_mut (), val: -2 },
  option { name: c"min-regexp-length".as_ptr (), 	has_arg: required_argument, flag: std::ptr::null_mut (), val: b'm' as c_int },
  option { name: c"max-regexp-length".as_ptr (), 	has_arg: required_argument, flag: std::ptr::null_mut (), val: b'M' as c_int },
  option { name: c"no-nested-classes".as_ptr (), 	has_arg: no_argument,	   flag: std::ptr::null_mut (), val: b'n' as c_int },
  option { name: c"no-regexps".as_ptr (), 		has_arg: no_argument,	   flag: std::ptr::null_mut (), val: b'x' as c_int },
  option { name: c"no-structs-or-unions".as_ptr (), 	has_arg: no_argument,	   flag: std::ptr::null_mut (), val: b's' as c_int },
  option { name: c"output-file".as_ptr (), 		has_arg: required_argument, flag: std::ptr::null_mut (), val: b'o' as c_int },
  option { name: c"position-info".as_ptr (), 		has_arg: required_argument, flag: std::ptr::null_mut (), val: b'p' as c_int },
  option { name: c"search-path".as_ptr (), 		has_arg: required_argument, flag: std::ptr::null_mut (), val: b'I' as c_int },
  option { name: c"verbose".as_ptr (), 			has_arg: no_argument,	   flag: std::ptr::null_mut (), val: b'v' as c_int },
  option { name: c"version".as_ptr (), 			has_arg: no_argument,	   flag: std::ptr::null_mut (), val: -3 },
  option { name: c"very-verbose".as_ptr (), 		has_arg: no_argument,	   flag: std::ptr::null_mut (), val: b'V' as c_int },
  option { name: std::ptr::null (),			has_arg: 0, 		   flag: std::ptr::null_mut (), val: 0 }
];

/* Semantic values of tokens.  Set by yylex..  */

static mut yyival: c_uint = 0;		/* Set for token CINT.  */
static mut yytext: *mut c_char = std::ptr::null_mut ();		/* Set for token IDENT.  */
static mut yytext_end: *mut c_char = std::ptr::null_mut ();

/* Output file.  */

static mut yyout: *mut FILE = std::ptr::null_mut ();

/* Current line number.  */

static mut yyline: c_int = 0;

/* The name of the current input file.  */

static mut filename: *const c_char = std::ptr::null ();

/* Three character class vectors, and macros to test membership
   of characters.  */

/* C declares `static char is_ident[255]` and its own `IDENTP` is
   `is_ident[(unsigned char) (C)]`: a 0xFF byte in the input (which this scanner does see -
   a Latin-1 identifier character, or any byte of a non-UTF-8 file) indexes 255, one past
   the array.  That read is undefined in C and happens to return the first byte of the next
   static (`is_digit[0]`, also 0 for `is_digit[255]` -> `is_white[0]`, and `is_white[255]` ->
   `f_append`'s low byte, 0 on this target), so the reference treats 0xFF as "not an
   identifier character"; `tests` (the differential run recorded in
   docs/ports/ebrowse.md) confirms the same output.  The port declares the three arrays one
   element larger, so the identical access is in bounds and reads the same 0 - the
   initialisation loop below then covers 256 bytes rather than C's 255, which writes one
   more zero into an array C already zero-initialises.  */
static mut is_ident: [c_char; 256] = [0; 256];
static mut is_digit: [c_char; 256] = [0; 256];
static mut is_white: [c_char; 256] = [0; 256];

/// C 355: `#define IDENTP(C)	is_ident[(unsigned char) (C)]`
macro_rules! IDENTP { ($c:expr) => { is_ident[($c) as c_uchar as usize] != 0 }; }
/// C 356: `#define DIGITP(C)	is_digit[(unsigned char) (C)]`
macro_rules! DIGITP { ($c:expr) => { is_digit[($c) as c_uchar as usize] != 0 }; }
/// C 357: `#define WHITEP(C)	is_white[(unsigned char) (C)]`
macro_rules! WHITEP { ($c:expr) => { is_white[($c) as c_uchar as usize] != 0 }; }

/* Command line flags.  */

static mut f_append: c_int = 0;
static mut f_verbose: c_int = 0;
static mut f_very_verbose: c_int = 0;
static mut f_structs: c_int = 1;
static mut f_regexps: c_int = 1;
static mut f_nested_classes: c_int = 1;

/* Maximum and minimum lengths of regular expressions matching a
   member, class etc., for writing them to the output file.  These are
   overridable from the command line.  */

static mut min_regexp: c_int = 5;
static mut max_regexp: c_int = 50;

/* Input buffer.  */

static mut inbuffer: *mut c_char = std::ptr::null_mut ();
/* C 378's `static char *in;`: `in` is a Rust keyword, so the item is the raw identifier
   `r#in` (PORTING.md 0.2) and its name is unchanged.  `tools/extract-abi.py`'s statics
   pattern reads the first identifier after `static` and does not accept the `r#` prefix
   (its field pattern does), so the comment below also spells the name in the form that
   pattern reads - `static mut in` - and the gap is recorded in docs/ports/ebrowse.md.  */
static mut r#in: *mut c_char = std::ptr::null_mut ();
static mut inbuffer_size: size_t = 0;

/* Return the current buffer position in the input file.  */

/// C 383: `#define BUFFER_POS() (in - inbuffer)`
macro_rules! BUFFER_POS { () => { (r#in as isize - inbuffer as isize) as c_int }; }

/* If current lookahead is CSTRING, the following points to the
   first character in the string constant.  Used for recognizing
   extern "C".  */

static mut string_start: *mut c_char = std::ptr::null_mut ();

/* The size of the hash tables for classes.and members.  Should be
   prime.  */

/// C 394: `#define TABLE_SIZE 1001`
const TABLE_SIZE: c_int = 1001;

/* The hash table for class symbols.  */

static mut class_table: [*mut sym; TABLE_SIZE as usize] =
  [std::ptr::null_mut (); TABLE_SIZE as usize];

/* Hash table containing all member structures.  This is generally
   faster for member lookup than traversing the member lists of a
   `struct sym'.  */

static mut member_table: [*mut member; TABLE_SIZE as usize] =
  [std::ptr::null_mut (); TABLE_SIZE as usize];

/* Hash table for namespace aliases */

static mut namespace_alias_table: [*mut alias; TABLE_SIZE as usize] =
  [std::ptr::null_mut (); TABLE_SIZE as usize];

/* The special class symbol used to hold global functions,
   variables etc.  */

static mut global_symbols: *mut sym = std::ptr::null_mut ();

/* The current namespace.  */

static mut current_namespace: *mut sym = std::ptr::null_mut ();

/* The list of all known namespaces.  */

static mut all_namespaces: *mut sym = std::ptr::null_mut ();

/* Stack of namespaces we're currently nested in, during the parse.  */

static mut namespace_stack: *mut *mut sym = std::ptr::null_mut ();
static mut namespace_stack_size: c_int = 0;
static mut namespace_sp: c_int = 0;

/* The current lookahead token.  */

static mut tk: c_int = -1;

/* Structure describing a keyword.  */

#[repr(C)]
pub struct kw
{
  pub name: *const c_char,		/* Spelling.  */
  pub tk: c_int,			/* Token value.  */
  pub next: *mut kw,		/* Next in collision chain.  */
}

const _: () = core::assert! (std::mem::size_of::<kw> () == 24);
const _: () = core::assert! (std::mem::offset_of! (kw, tk) == 8);
const _: () = core::assert! (std::mem::offset_of! (kw, next) == 16);

/* Keywords are lookup up in a hash table of their own.  */

/// C 444: `#define KEYWORD_TABLE_SIZE 1001`
const KEYWORD_TABLE_SIZE: c_int = 1001;
static mut keyword_table: [*mut kw; KEYWORD_TABLE_SIZE as usize] =
  [std::ptr::null_mut (); KEYWORD_TABLE_SIZE as usize];

/* Search path.  */

#[repr(C)]
pub struct search_path
{
  pub path: *mut c_char,
  pub next: *mut search_path,
}

const _: () = core::assert! (std::mem::size_of::<search_path> () == 16);
const _: () = core::assert! (std::mem::offset_of! (search_path, next) == 8);

static mut search_path: *mut search_path = std::ptr::null_mut ();
static mut search_path_tail: *mut search_path = std::ptr::null_mut ();

/* Function prototypes (C 463-470): dropped, Rust items are order-independent
   (PORTING.md 0.1-12).  */

/***********************************************************************
			      Utilities
 ***********************************************************************/

/* Print an error in a printf-like style with the current input file
   name and line number.  */

unsafe fn yyerror (format: *const c_char, s: *const c_char)
{
  fprintf (stderr, c"%s:%d: ".as_ptr (), filename, yyline);
  fprintf (stderr, format, s);
  putc (b'\n' as c_int, stderr);
}

/* Like malloc but print an error and exit if not enough memory is
   available.  */

/* C 484: `static void * ATTRIBUTE_MALLOC` - the attribute has no Rust counterpart
   (PORTING.md 8/P-13).  */
unsafe fn xmalloc (nbytes: size_t) -> *mut c_void
{
  let p: *mut c_void = malloc (nbytes);
  if p.is_null ()
    {
      yyerror (c"out of memory".as_ptr (), std::ptr::null ());
      exit (EXIT_FAILURE);
    }
  return p;
}

/* Like realloc but print an error and exit if out of memory.  */

unsafe fn xrealloc (p: *mut c_void, sz: size_t) -> *mut c_void
{
  let p: *mut c_void = realloc (p, sz);
  if p.is_null ()
    {
      yyerror (c"out of memory".as_ptr (), std::ptr::null ());
      exit (EXIT_FAILURE);
    }
  return p;
}

/* Like strdup, but print an error and exit if not enough memory is
   available..  If S is null, return null.  */

unsafe fn xstrdup (s: *mut c_char) -> *mut c_char
{
  if !s.is_null ()
    { return strcpy (xmalloc (strlen (s) + 1) as *mut c_char, s); }
  return s;
}

/***********************************************************************
			       Symbols
 ***********************************************************************/

/* Initialize the symbol table.  This currently only sets up the
   special symbol for globals (`*Globals*').  */

unsafe fn init_sym ()
{
  global_symbols = add_sym (GLOBALS_NAME.as_ptr (), std::ptr::null_mut ());
}

/* Add a symbol for class NAME to the symbol table.  NESTED_IN_CLASS
   is the class in which class NAME was found.  If it is null,
   this means the scope of NAME is the current namespace.

   If a symbol for NAME already exists, return that.  Otherwise
   create a new symbol and set it to default values.  */

unsafe fn add_sym (name: *const c_char, nested_in_class: *mut sym) -> *mut sym
{
  let mut sym: *mut sym;
  let mut h: c_uint;
  let mut s: *const c_char;
  let scope: *mut sym = if !nested_in_class.is_null () { nested_in_class } else { current_namespace };

  s = name;
  h = 0;
  while *s != 0
    {
      h = (h << 1) ^ *s as c_uint;
      s = s.add (1);
    }
  h %= TABLE_SIZE as c_uint;

  sym = class_table[h as usize];
  while !sym.is_null ()
    {
      if streq (name, FAM_PTR! ((*sym).name))
	&& ((*sym).namesp.is_null () && scope.is_null ()
	    || !(*sym).namesp.is_null () && !scope.is_null ()
	       && streq (FAM_PTR! ((*(*sym).namesp).name), FAM_PTR! ((*scope).name)))
	{ break; }
      sym = (*sym).next;
    }

  if sym.is_null ()
    {
      if f_very_verbose != 0
	{
	  putchar (b'\t' as c_int);
	  puts (name);
	}

      sym = xmalloc (FLEXSIZEOF! (sym, name, strlen (name) + 1)) as *mut sym;
      memset (sym as *mut c_void, 0, std::mem::offset_of! (sym, name));
      strcpy (FAM_PTR! ((*sym).name), name);
      (*sym).namesp = scope;
      (*sym).next = class_table[h as usize];
      class_table[h as usize] = sym;
    }

  return sym;
}

/* Add links between superclass SUPER and subclass SUB.  */

unsafe fn add_link (super_: *mut sym, sub: *mut sym)
{
  let mut lnk: *mut link;
  let mut lnk2: *mut link;
  let mut p: *mut link;
  let mut prev: *mut link;

  /* See if a link already exists.  */
  prev = std::ptr::null_mut ();
  p = (*super_).subs;
  while !p.is_null () && strcmp (FAM_PTR! ((*sub).name), FAM_PTR! ((*(*p).sym).name)) > 0
    {
      prev = p;
      p = (*p).next;
    }

  /* Avoid duplicates.  */
  if p.is_null () || (*p).sym != sub
    {
      lnk = xmalloc (std::mem::size_of::<link> ()) as *mut link;
      lnk2 = xmalloc (std::mem::size_of::<link> ()) as *mut link;

      (*lnk).sym = sub;
      (*lnk).next = p;

      if !prev.is_null ()
	{ (*prev).next = lnk; }
      else
	{ (*super_).subs = lnk; }

      (*lnk2).sym = super_;
      (*lnk2).next = (*sub).supers;
      (*sub).supers = lnk2;
    }
}

/* Find in class CLS member NAME.

   VAR non-zero means look for a member variable; otherwise a function
   is searched.  SC specifies what kind of member is searched---a
   static, or per-instance member etc.  HASH is a hash code for the
   parameter types of functions.  Value is a pointer to the member
   found or null if not found.  */

unsafe fn find_member (cls: *mut sym, name: *mut c_char, var: c_int, sc: c_int,
		       hash: c_uint) -> *mut member
{
  let mut list: *mut *mut member;
  let mut p: *mut member;
  let mut name_hash: c_uint = 0;
  let mut s: *mut c_char;
  let mut i: c_int;

  match sc
    {
    SC_FRIEND =>
      { list = &raw mut (*cls).friends; }

    SC_TYPE =>
      { list = &raw mut (*cls).types; }

    SC_STATIC =>
      { list = if var != 0 { &raw mut (*cls).static_vars } else { &raw mut (*cls).static_fns }; }

    _ =>
      { list = if var != 0 { &raw mut (*cls).vars } else { &raw mut (*cls).fns }; }
    }

  s = name;
  while *s != 0
    {
      name_hash = (name_hash << 1) ^ *s as c_uint;
      s = s.add (1);
    }
  i = (name_hash % TABLE_SIZE as c_uint) as c_int;

  p = member_table[i as usize];
  while !p.is_null ()
    {
      if (*p).list == list && (*p).param_hash == hash && streq (name, FAM_PTR! ((*p).name))
	{ break; }
      p = (*p).anext;
    }

  return p;
}

/* Add to class CLS information for the declaration of member NAME.
   REGEXP is a regexp matching the declaration, if non-null.  POS is
   the position in the source where the declaration is found.  HASH is
   a hash code for the parameter list of the member, if it's a
   function.  VAR non-zero means member is a variable or type.  SC
   specifies the type of member (instance member, static, ...).  VIS
   is the member's visibility (public, protected, private).  FLAGS is
   a bit set giving additional information about the member (see the
   F_* defines).  */

unsafe fn add_member_decl (cls: *mut sym, name: *mut c_char, regexp: *mut c_char,
			   pos: c_int, hash: c_uint, var: c_int, sc: c_int, vis: c_int,
			   flags: c_int)
{
  let mut m: *mut member;

  m = find_member (cls, name, var, sc, hash);
  if m.is_null ()
    { m = add_member (cls, name, var, sc, hash); }

  /* Have we seen a new filename?  If so record that.  */
  if (*cls).filename.is_null () || !filename_eq ((*cls).filename, filename)
    { (*m).filename = filename; }

  (*m).regexp = regexp;
  (*m).pos = pos;
  (*m).flags = flags;

  match vis
    {
    PRIVATE =>
      { (*m).vis = V_PRIVATE; }

    PROTECTED =>
      { (*m).vis = V_PROTECTED; }

    PUBLIC =>
      { (*m).vis = V_PUBLIC; }

    _ =>
      { }
    }

  info_where = P_DECL;
  info_cls = cls;
  info_member = m;
}

/* Add to class CLS information for the definition of member NAME.
   REGEXP is a regexp matching the declaration, if non-null.  POS is
   the position in the source where the declaration is found.  HASH is
   a hash code for the parameter list of the member, if it's a
   function.  VAR non-zero means member is a variable or type.  SC
   specifies the type of member (instance member, static, ...).  VIS
   is the member's visibility (public, protected, private).  FLAGS is
   a bit set giving additional information about the member (see the
   F_* defines).  */

unsafe fn add_member_defn (cls: *mut sym, name: *mut c_char, regexp: *mut c_char,
			   pos: c_int, hash: c_uint, var: c_int, sc: c_int, flags: c_int)
{
  let mut m: *mut member;

  if sc == SC_UNKNOWN
    {
      m = find_member (cls, name, var, SC_MEMBER, hash);
      if m.is_null ()
	{
	  m = find_member (cls, name, var, SC_STATIC, hash);
	  if m.is_null ()
	    { m = add_member (cls, name, var, sc, hash); }
	}
    }
  else
    {
      m = find_member (cls, name, var, sc, hash);
      if m.is_null ()
	{ m = add_member (cls, name, var, sc, hash); }
    }

  if (*cls).sfilename.is_null ()
    { (*cls).sfilename = filename; }

  if !filename_eq ((*cls).sfilename, filename)
    { (*m).def_filename = filename; }

  (*m).def_regexp = regexp;
  (*m).def_pos = pos;
  (*m).flags |= flags;

  info_where = P_DEFN;
  info_cls = cls;
  info_member = m;
}

/* Add a symbol for a define named NAME to the symbol table.
   REGEXP is a regular expression matching the define in the source,
   if it is non-null.  POS is the position in the file.  */

unsafe fn add_define (name: *mut c_char, regexp: *mut c_char, pos: c_int)
{
  add_global_defn (name, regexp, pos, 0, 1, SC_FRIEND, F_DEFINE);
  add_global_decl (name, regexp, pos, 0, 1, SC_FRIEND, F_DEFINE);
}

/* Add information for the global definition of NAME.
   REGEXP is a regexp matching the declaration, if non-null.  POS is
   the position in the source where the declaration is found.  HASH is
   a hash code for the parameter list of the member, if it's a
   function.  VAR non-zero means member is a variable or type.  SC
   specifies the type of member (instance member, static, ...).  VIS
   is the member's visibility (public, protected, private).  FLAGS is
   a bit set giving additional information about the member (see the
   F_* defines).  */

unsafe fn add_global_defn (name: *mut c_char, regexp: *mut c_char, pos: c_int,
			   hash: c_uint, var: c_int, sc: c_int, flags: c_int)
{
  let mut i: c_int;
  let mut sym: *mut sym;

  /* Try to find out for which classes a function is a friend, and add
     what we know about it to them.  */
  if var == 0
    {
      i = 0;
      while i < TABLE_SIZE
	{
	  sym = class_table[i as usize];
	  while !sym.is_null ()
	    {
	      if sym != global_symbols && !(*sym).friends.is_null ()
		{
		  if !find_member (sym, name, 0, SC_FRIEND, hash).is_null ()
		    {
		      add_member_defn (sym, name, regexp, pos, hash, 0,
				       SC_FRIEND, flags);
		    }
		}
	      sym = (*sym).next;
	    }
	  i += 1;
	}
    }

  /* Add to global symbols.  */
  add_member_defn (global_symbols, name, regexp, pos, hash, var, sc, flags);
}

/* Add information for the global declaration of NAME.
   REGEXP is a regexp matching the declaration, if non-null.  POS is
   the position in the source where the declaration is found.  HASH is
   a hash code for the parameter list of the member, if it's a
   function.  VAR non-zero means member is a variable or type.  SC
   specifies the type of member (instance member, static, ...).  VIS
   is the member's visibility (public, protected, private).  FLAGS is
   a bit set giving additional information about the member (see the
   F_* defines).  */

unsafe fn add_global_decl (name: *mut c_char, regexp: *mut c_char, pos: c_int,
			   hash: c_uint, var: c_int, sc: c_int, flags: c_int)
{
  /* Add declaration only if not already declared.  Header files must
     be processed before source files for this to have the right effect.
     I do not want to handle implicit declarations at the moment.  */
  let mut m: *mut member;
  let mut found: *mut member;

  m = find_member (global_symbols, name, var, sc, hash);
  found = m;
  if m.is_null ()
    { m = add_member (global_symbols, name, var, sc, hash); }

  /* Definition already seen => probably last declaration implicit.
     Override.  This means that declarations must always be added to
     the symbol table before definitions.  */
  if found.is_null ()
    {
      if (*global_symbols).filename.is_null ()
	  || !filename_eq ((*global_symbols).filename, filename)
	{ (*m).filename = filename; }

      (*m).regexp = regexp;
      (*m).pos = pos;
      (*m).vis = V_PUBLIC;
      (*m).flags = flags;

      info_where = P_DECL;
      info_cls = global_symbols;
      info_member = m;
    }
}

/* Add a symbol for member NAME to class CLS.
   VAR non-zero means it's a variable.  SC specifies the kind of
   member.  HASH is a hash code for the parameter types of a function.
   Value is a pointer to the member's structure.  */

unsafe fn add_member (cls: *mut sym, name: *mut c_char, var: c_int, sc: c_int,
		      hash: c_uint) -> *mut member
{
  let mut m: *mut member = xmalloc (FLEXSIZEOF! (member, name, strlen (name) + 1))
			     as *mut member;
  let mut list: *mut *mut member;
  let mut p: *mut member;
  let mut prev: *mut member;
  let mut name_hash: c_uint = 0;
  let mut i: c_int;
  let mut s: *mut c_char;

  strcpy (FAM_PTR! ((*m).name), name);
  (*m).param_hash = hash;

  (*m).vis = 0;
  (*m).flags = 0;
  (*m).regexp = std::ptr::null_mut ();
  (*m).filename = std::ptr::null ();
  (*m).pos = 0;
  (*m).def_regexp = std::ptr::null_mut ();
  (*m).def_filename = std::ptr::null ();
  (*m).def_pos = 0;

  assert! (cls != std::ptr::null_mut ());

  match sc
    {
    SC_FRIEND =>
      { list = &raw mut (*cls).friends; }

    SC_TYPE =>
      { list = &raw mut (*cls).types; }

    SC_STATIC =>
      { list = if var != 0 { &raw mut (*cls).static_vars } else { &raw mut (*cls).static_fns }; }

    _ =>
      { list = if var != 0 { &raw mut (*cls).vars } else { &raw mut (*cls).fns }; }
    }

  s = name;
  while *s != 0
    {
      name_hash = (name_hash << 1) ^ *s as c_uint;
      s = s.add (1);
    }
  i = (name_hash % TABLE_SIZE as c_uint) as c_int;
  (*m).anext = member_table[i as usize];
  member_table[i as usize] = m;
  (*m).list = list;

  /* Keep the member list sorted.  It's cheaper to do it here than to
     sort them in Lisp.  */
  prev = std::ptr::null_mut ();
  p = *list;
  while !p.is_null () && strcmp (name, FAM_PTR! ((*p).name)) > 0
    {
      prev = p;
      p = (*p).next;
    }

  (*m).next = p;
  if !prev.is_null ()
    { (*prev).next = m; }
  else
    { *list = m; }
  return m;
}

/* Given the root R of a class tree, step through all subclasses
   recursively, marking functions as virtual that are declared virtual
   in base classes.  */

unsafe fn mark_virtual (r: *mut sym)
{
  let mut p: *mut link;
  let mut m: *mut member;
  let mut m2: *mut member;

  p = (*r).subs;
  while !p.is_null ()
    {
      m = (*r).fns;
      while !m.is_null ()
	{
	  if has_flag ((*m).flags, F_VIRTUAL)
	    {
	      m2 = (*(*p).sym).fns;
	      while !m2.is_null ()
		{
		  if (*m).param_hash == (*m2).param_hash && streq (FAM_PTR! ((*m).name), FAM_PTR! ((*m2).name))
		    { set_flag (&raw mut (*m2).flags, F_VIRTUAL); }
		  m2 = (*m2).next;
		}
	    }
	  m = (*m).next;
	}

      mark_virtual ((*p).sym);
      p = (*p).next;
    }
}

/* For all roots of the class tree, mark functions as virtual that
   are virtual because of a virtual declaration in a base class.  */

unsafe fn mark_inherited_virtual ()
{
  let mut r: *mut sym;
  let mut i: c_int;

  i = 0;
  while i < TABLE_SIZE
    {
      r = class_table[i as usize];
      while !r.is_null ()
	{
	  if (*r).supers.is_null ()
	    { mark_virtual (r); }
	  r = (*r).next;
	}
      i += 1;
    }
}

/* Create and return a symbol for a namespace with name NAME.  */

unsafe fn make_namespace (name: *mut c_char, context: *mut sym) -> *mut sym
{
  let s: *mut sym = xmalloc (FLEXSIZEOF! (sym, name, strlen (name) + 1)) as *mut sym;
  memset (s as *mut c_void, 0, std::mem::offset_of! (sym, name));
  strcpy (FAM_PTR! ((*s).name), name);
  (*s).next = all_namespaces;
  (*s).namesp = context;
  all_namespaces = s;
  return s;
}

/* Find the symbol for namespace NAME.  If not found, return NULL */

unsafe fn check_namespace (name: *mut c_char, context: *mut sym) -> *mut sym
{
  let mut p: *mut sym;

  p = all_namespaces;
  while !p.is_null ()
    {
      if streq (FAM_PTR! ((*p).name), name) && (*p).namesp == context
	{ break; }
      p = (*p).next;
    }

  return p;
}

/* Find the symbol for namespace NAME.  If not found, add a new symbol
   for NAME to all_namespaces.  */

unsafe fn find_namespace (name: *mut c_char, context: *mut sym) -> *mut sym
{
  let mut p: *mut sym = check_namespace (name, context);

  if p.is_null ()
    { p = make_namespace (name, context); }

  return p;
}

/* Find namespace alias with name NAME. If not found return NULL. */

unsafe fn check_namespace_alias (name: *mut c_char) -> *mut link
{
  let mut p: *mut link = std::ptr::null_mut ();
  let mut al: *mut alias;
  let mut h: c_uint;
  let mut s: *mut c_char;

  s = name;
  h = 0;
  while *s != 0
    {
      h = (h << 1) ^ *s as c_uint;
      s = s.add (1);
    }
  h %= TABLE_SIZE as c_uint;

  al = namespace_alias_table[h as usize];
  while !al.is_null ()
    {
      if streq (name, FAM_PTR! ((*al).name)) && (*al).namesp == current_namespace
	{
	  p = (*al).aliasee;
	  break;
	}
      al = (*al).next;
    }

  return p;
}

/* Register the name NEW_NAME as an alias for namespace list OLD_NAME.  */

unsafe fn register_namespace_alias (new_name: *mut c_char, old_name: *mut link)
{
  let mut h: c_uint;
  let mut s: *mut c_char;
  let mut al: *mut alias;

  s = new_name;
  h = 0;
  while *s != 0
    {
      h = (h << 1) ^ *s as c_uint;
      s = s.add (1);
    }
  h %= TABLE_SIZE as c_uint;

  /* Is it already in the table of aliases?  */
  al = namespace_alias_table[h as usize];
  while !al.is_null ()
    {
      if streq (new_name, FAM_PTR! ((*al).name)) && (*al).namesp == current_namespace
	{ return; }
      al = (*al).next;
    }

  al = xmalloc (FLEXSIZEOF! (alias, name, strlen (new_name) + 1)) as *mut alias;
  strcpy (FAM_PTR! ((*al).name), new_name);
  (*al).next = namespace_alias_table[h as usize];
  (*al).namesp = current_namespace;
  (*al).aliasee = old_name;
  namespace_alias_table[h as usize] = al;
}

/* Enter namespace with name NAME.  */

unsafe fn enter_namespace (name: *mut c_char)
{
  let p: *mut sym = find_namespace (name, current_namespace);

  if namespace_sp == namespace_stack_size
    {
      let size: c_int = max! (10, 2 * namespace_stack_size);
      namespace_stack
	= xrealloc (namespace_stack as *mut c_void,
		    size as size_t * std::mem::size_of::<*mut sym> ()) as *mut *mut sym;
      namespace_stack_size = size;
    }

  *namespace_stack.add (namespace_sp as usize) = current_namespace;
  namespace_sp += 1;
  current_namespace = p;
}

/* Leave the current namespace.  */

unsafe fn leave_namespace ()
{
  assert! (namespace_sp > 0);
  namespace_sp -= 1;
  current_namespace = *namespace_stack.add (namespace_sp as usize);
}

/***********************************************************************
		       Writing the Output File
 ***********************************************************************/

/* Write string S to the output file FP in a Lisp-readable form.
   If S is null, write out `()'.  */

unsafe fn putstr (s: *const c_char, fp: *mut FILE)
{
  if s.is_null ()
    {
      putc (b'(' as c_int, fp);
      putc (b')' as c_int, fp);
      putc (b' ' as c_int, fp);
    }
  else
    {
      putc (b'"' as c_int, fp);
      fputs (s, fp);
      putc (b'"' as c_int, fp);
      putc (b' ' as c_int, fp);
    }
}

/* A dynamically allocated buffer for constructing a scope name.  */

static mut scope_buffer: *mut c_char = std::ptr::null_mut ();
static mut scope_buffer_size: c_int = 0;
static mut scope_buffer_len: c_int = 0;

/* Make sure scope_buffer has enough room to add LEN chars to it.  */

unsafe fn ensure_scope_buffer_room (len: c_int)
{
  if scope_buffer_len + len >= scope_buffer_size
    {
      let new_size: c_int = max! (2 * scope_buffer_size, scope_buffer_len + len);
      scope_buffer = xrealloc (scope_buffer as *mut c_void, new_size as size_t) as *mut c_char;
      scope_buffer_size = new_size;
    }
}

/* Recursively add the scope names of symbol P and the scopes of its
   namespaces to scope_buffer.  Value is a pointer to the complete
   scope name constructed.  */

unsafe fn sym_scope_1 (p: *mut sym) -> *mut c_char
{
  let mut len: c_int;

  if !(*p).namesp.is_null ()
    { sym_scope_1 ((*p).namesp); }

  if *scope_buffer != 0
    {
      ensure_scope_buffer_room (3);
      strcpy (scope_buffer.add (scope_buffer_len as usize), c"::".as_ptr ());
      scope_buffer_len += 2;
    }

  len = strlen (FAM_PTR! ((*p).name)) as c_int;
  ensure_scope_buffer_room (len + 1);
  strcpy (scope_buffer.add (scope_buffer_len as usize), FAM_PTR! ((*p).name));
  scope_buffer_len += len;

  if has_flag ((*p).flags, F_TEMPLATE)
    {
      ensure_scope_buffer_room (3);
      strcpy (scope_buffer.add (scope_buffer_len as usize), c"<>".as_ptr ());
      scope_buffer_len += 2;
    }

  return scope_buffer;
}

/* Return the scope of symbol P in printed representation, i.e.
   as it would appear in a C*+ source file.  */

unsafe fn sym_scope (p: *mut sym) -> *mut c_char
{
  if scope_buffer.is_null ()
    {
      scope_buffer_size = 1024;
      scope_buffer = xmalloc (scope_buffer_size as size_t) as *mut c_char;
    }

  *scope_buffer = 0;
  scope_buffer_len = 0;

  if !(*p).namesp.is_null ()
    { sym_scope_1 ((*p).namesp); }

  return scope_buffer;
}

/* Dump the list of members M to file FP.  */

unsafe fn dump_members (fp: *mut FILE, m: *mut member)
{
  let mut m: *mut member = m;

  putc (b'(' as c_int, fp);

  while !m.is_null ()
    {
      fputs (MEMBER_STRUCT.as_ptr (), fp);
      putstr (FAM_PTR! ((*m).name), fp);
      putstr (std::ptr::null (), fp);		/* FIXME? scope for globals */
      fprintf (fp, c"%u ".as_ptr (), (*m).flags as c_uint);
      putstr ((*m).filename, fp);
      putstr ((*m).regexp, fp);
      fprintf (fp, c"%u ".as_ptr (), (*m).pos as c_uint);
      fprintf (fp, c"%u ".as_ptr (), (*m).vis as c_uint);
      putc (b' ' as c_int, fp);
      putstr ((*m).def_filename, fp);
      putstr ((*m).def_regexp, fp);
      fprintf (fp, c"%u".as_ptr (), (*m).def_pos as c_uint);
      putc (b']' as c_int, fp);
      putc (b'\n' as c_int, fp);
      m = (*m).next;
    }

  putc (b')' as c_int, fp);
  putc (b'\n' as c_int, fp);
}

/* Dump class ROOT to stream FP.  */

unsafe fn dump_sym (fp: *mut FILE, root: *mut sym)
{
  fputs (CLASS_STRUCT.as_ptr (), fp);
  putstr (FAM_PTR! ((*root).name), fp);

  /* Print scope, if any.  */
  if !(*root).namesp.is_null ()
    { putstr (sym_scope (root), fp); }
  else
    { putstr (std::ptr::null (), fp); }

  /* Print flags.  */
  fprintf (fp, c"%d".as_ptr (), (*root).flags);
  putstr ((*root).filename, fp);
  putstr ((*root).regexp, fp);
  fprintf (fp, c"%u".as_ptr (), (*root).pos as c_uint);
  putstr ((*root).sfilename, fp);
  putc (b']' as c_int, fp);
  putc (b'\n' as c_int, fp);
}

/* Dump class ROOT and its subclasses to file FP.  */

unsafe fn dump_tree (fp: *mut FILE, root: *mut sym)
{
  let mut lk: *mut link;

  dump_sym (fp, root);

  if f_verbose != 0
    {
      putchar (b'+' as c_int);
      fflush (stdout);
    }

  putc (b'(' as c_int, fp);

  lk = (*root).subs;
  while !lk.is_null ()
    {
      fputs (TREE_STRUCT.as_ptr (), fp);
      dump_tree (fp, (*lk).sym);
      putc (b']' as c_int, fp);
      lk = (*lk).next;
    }

  putc (b')' as c_int, fp);

  dump_members (fp, (*root).vars);
  dump_members (fp, (*root).fns);
  dump_members (fp, (*root).static_vars);
  dump_members (fp, (*root).static_fns);
  dump_members (fp, (*root).friends);
  dump_members (fp, (*root).types);

  /* Superclasses.  */
  putc (b'(' as c_int, fp);
  putc (b')' as c_int, fp);

  /* Mark slot.  */
  putc (b'(' as c_int, fp);
  putc (b')' as c_int, fp);

  putc (b'\n' as c_int, fp);
}

/* Dump the entire class tree to file FP.  */

unsafe fn dump_roots (fp: *mut FILE)
{
  let mut i: c_int;
  let mut r: *mut sym;

  /* Output file header containing version string, command line
     options etc.  */
  if f_append == 0
    {
      fputs (TREE_HEADER_STRUCT.as_ptr (), fp);
      putstr (EBROWSE_FILE_VERSION.as_ptr (), fp);

      putc (b'"' as c_int, fp);
      if f_structs == 0
	{ fputs (c" -s".as_ptr (), fp); }
      if f_regexps != 0
	{ fputs (c" -x".as_ptr (), fp); }
      putc (b'"' as c_int, fp);
      fputs (c" ()".as_ptr (), fp);
      fputs (c" ()".as_ptr (), fp);
      putc (b']' as c_int, fp);
    }

  /* Mark functions as virtual that are so because of functions
     declared virtual in base classes.  */
  mark_inherited_virtual ();

  /* Dump the roots of the graph.  */
  i = 0;
  while i < TABLE_SIZE
    {
      r = class_table[i as usize];
      while !r.is_null ()
	{
	  if (*r).supers.is_null ()
	    {
	      fputs (TREE_STRUCT.as_ptr (), fp);
	      dump_tree (fp, r);
	      putc (b']' as c_int, fp);
	    }
	  r = (*r).next;
	}
      i += 1;
    }

  if f_verbose != 0
    { putchar (b'\n' as c_int); }
}

/***********************************************************************
				Scanner
 ***********************************************************************/

/* C 1346-1359's `#ifdef DEBUG` arm is dead in the frozen configuration (P-08), so the live
   definition is C 1358's.  */

/// C 1358: `#define INCREMENT_LINENO	++yyline`
macro_rules! INCREMENT_LINENO { () => { yyline += 1; }; }

/* Define two macros for accessing the input buffer (current input
   file).  GET(C) sets C to the next input character and advances the
   input pointer.  UNGET retracts the input pointer.  */

/// C 1365: `#define GET(C)	((C) = *in++)`
macro_rules! GET {
  ($c:expr) => { { let v = *r#in; r#in = r#in.add (1); $c = v as _; $c } };
}
/// C 1366: `#define UNGET() (--in)`
macro_rules! UNGET { () => { r#in = r#in.sub (1) }; }

/* Process a preprocessor line.  Value is the next character from the
   input buffer not consumed.  */

unsafe fn process_pp_line () -> c_int
{
  let mut in_comment: c_int = 0;
  let mut in_string: c_int = 0;
  let mut c: c_int;
  let mut p: *mut c_char = yytext;

  /* Skip over white space.  The `#' has been consumed already.  */
  while WHITEP! (GET! (c))
    { }

  /* Read the preprocessor command (if any).  */
  while IDENTP! (c)
    {
      *p = c as c_char;
      p = p.add (1);
      GET! (c);
    }

  /* Is it a `define'?  */
  *p = 0;

  if *yytext != 0 && streq (yytext, c"define".as_ptr ())
    {
      p = yytext;
      while WHITEP! (c)
	{ GET! (c); }
      while IDENTP! (c)
	{
	  *p = c as c_char;
	  p = p.add (1);
	  GET! (c);
	}

      *p = 0;

      if *yytext != 0
	{
	  let regexp: *mut c_char = matching_regexp ();
	  let pos: c_int = BUFFER_POS! ();
	  add_define (yytext, regexp, pos);
	}
    }

  while c != 0 && (c != b'\n' as c_int || in_comment != 0 || in_string != 0)
    {
      if c == b'\\' as c_int
	{ GET! (c); }
      else if c == b'/' as c_int && in_comment == 0
	{
	  if GET! (c) == b'*' as c_int
	    { in_comment = 1; }
	}
      else if c == b'*' as c_int && in_comment != 0
	{
	  if GET! (c) == b'/' as c_int
	    { in_comment = 0; }
	}
      else if c == b'"' as c_int
	{ in_string = !in_string; }

      if c == b'\n' as c_int
	{ INCREMENT_LINENO! (); }

      GET! (c);
    }

  return c;
}

/* C defines `RSTRING_EOF_CHECK` inside `yylex` (C 1571); only that function expands it, so
   the port's `macro_rules!` sits here, before it (P-57's ordering rule).  */

/* C 1571-1578: `#define RSTRING_EOF_CHECK do { if (c == '\0') { … } } while (0)`.  The C
   macro names the enclosing function's `c`; a `macro_rules!` body is hygienic and cannot, so
   the cursor is the macro's one metavariable - the marker.c CONSIDER precedent, where the
   same difficulty puts the statement sequence in a helper.  */
macro_rules! RSTRING_EOF_CHECK {
  ($c:expr) => {
    if $c == 0
      {
	yyerror (c"unterminated c++11 rstring".as_ptr (), std::ptr::null ());
	UNGET! ();
	return CSTRING;
      }
  };
}

/* Value is the next token from the input buffer.  */

/* C's `int_suffixes:` and `mantissa:` labels (C 1860, C 1887) are statement labels of the
   `switch`, which two arms reach with a forward `goto` and two by falling through; the port
   inlines the label's statement sequence at each site inside a labelled block that keeps
   C's name, so the eight labels are never jumped to (PORTING.md 2.2).  */
#[allow(unused_labels)]
unsafe fn yylex () -> c_int
{
  let mut c: c_int;
  let mut end_char: c_char;
  let mut p: *mut c_char;

  loop
    {
      while WHITEP! (GET! (c))
        { }

      match c
	{
	_ if c == b'\n' as c_int =>
	  {
	    INCREMENT_LINENO! ();
	  }

	_ if c == b'\r' as c_int =>
	  {
	  }

	0 =>
	  {
	    /* End of file.  */
	    return YYEOF;
	  }

	_ if c == b'\\' as c_int =>
	  {
	    GET! (c);
	  }

	_ if c == b'"' as c_int || c == b'\'' as c_int =>
	  {
	    /* String and character constants.  */
	    end_char = c as c_char;
	    string_start = r#in;
	    'end_string: loop
	      {
		c = GET! (c);
		if !(c != 0 && c != end_char as c_int)
		  { break 'end_string; }
		match c
		  {
		  _ if c == b'\\' as c_int =>
		    {
		      /* Escape sequences.  */
		      c = GET! (c);
		      if c == 0
			{
			  if end_char == b'\'' as c_char
			    { yyerror (c"EOF in character constant".as_ptr (), std::ptr::null ()); }
			  else
			    { yyerror (c"EOF in string constant".as_ptr (), std::ptr::null ()); }
			  break 'end_string;
			}
		      else
			{
			  match c
			    {
			    _ if c == b'\n' as c_int =>
			      { INCREMENT_LINENO! (); }
			    _ if c == b'a' as c_int => { }
			    _ if c == b'b' as c_int => { }
			    _ if c == b'f' as c_int => { }
			    _ if c == b'n' as c_int => { }
			    _ if c == b'r' as c_int => { }
			    _ if c == b't' as c_int => { }
			    _ if c == b'v' as c_int => { }

			    _ if c == b'x' as c_int =>
			      {
				/* Hexadecimal escape sequence.  */
				let mut i: c_int;
				i = 0;
				while i < 2
				  {
				    GET! (c);

				    if c >= b'0' as c_int && c <= b'7' as c_int
				      { }
				    else if c >= b'a' as c_int && c <= b'f' as c_int
				      { }
				    else if c >= b'A' as c_int && c <= b'F' as c_int
				      { }
				    else
				      {
					UNGET! ();
					break;
				      }
				    i += 1;
				  }
			      }

			    _ if c == b'0' as c_int =>
			      {
				/* Octal escape sequence.  */
				let mut i: c_int;
				i = 0;
				while i < 3
				  {
				    GET! (c);

				    if c >= b'0' as c_int && c <= b'7' as c_int
				      { }
				    else
				      {
					UNGET! ();
					break;
				      }
				    i += 1;
				  }
			      }

			    _ =>
			      { }
			    }
			}
		    }

		  _ if c == b'\n' as c_int =>
		    {
		      if end_char == b'\'' as c_char
			{ yyerror (c"newline in character constant".as_ptr (), std::ptr::null ()); }
		      else
			{ yyerror (c"newline in string constant".as_ptr (), std::ptr::null ()); }
		      INCREMENT_LINENO! ();
		    }

		  _ =>
		    { }
		  }
	      }

	    return if end_char == b'\'' as c_char { CCHAR } else { CSTRING };
	  }

	_ if c == b'R' as c_int || c == b'a' as c_int || c == b'b' as c_int || c == b'c' as c_int
	     || c == b'd' as c_int || c == b'e' as c_int || c == b'f' as c_int
	     || c == b'g' as c_int || c == b'h' as c_int || c == b'i' as c_int
	     || c == b'j' as c_int || c == b'k' as c_int || c == b'l' as c_int
	     || c == b'm' as c_int || c == b'n' as c_int || c == b'o' as c_int
	     || c == b'p' as c_int || c == b'q' as c_int || c == b'r' as c_int
	     || c == b's' as c_int || c == b't' as c_int || c == b'u' as c_int
	     || c == b'v' as c_int || c == b'w' as c_int || c == b'x' as c_int
	     || c == b'y' as c_int || c == b'z' as c_int
	     || c == b'A' as c_int || c == b'B' as c_int || c == b'C' as c_int
	     || c == b'D' as c_int || c == b'E' as c_int || c == b'F' as c_int
	     || c == b'G' as c_int || c == b'H' as c_int || c == b'I' as c_int
	     || c == b'J' as c_int || c == b'K' as c_int || c == b'L' as c_int
	     || c == b'M' as c_int || c == b'N' as c_int || c == b'O' as c_int
	     || c == b'P' as c_int || c == b'Q' as c_int || c == b'S' as c_int
	     || c == b'T' as c_int || c == b'U' as c_int || c == b'V' as c_int
	     || c == b'W' as c_int || c == b'X' as c_int || c == b'Y' as c_int
	     || c == b'Z' as c_int || c == b'_' as c_int =>
	  {
	    /* C 1565-1583's `case 'R':` under the rstring `if`, whose tail (`UNGET ();
	       FALLTHROUGH;`) the port keeps as the first statements of this merged arm:
	       Rust's `match` does not fall through, so C's two case labels are one arm and
	       the statements stay in C's order.  */
	    if c == b'R' as c_int
	      {
		if GET! (c) == b'"' as c_int
		  {
		    /* C++11 rstrings.  */
		    let mut rstring_prefix_start: *mut c_char = r#in;

		    while GET! (c) != b'(' as c_int
		      {
			RSTRING_EOF_CHECK! (c);
			if c == b'"' as c_int
			  {
			    yyerror (c"malformed c++11 rstring".as_ptr (), std::ptr::null ());
			    return CSTRING;
			  }
		      }
		    let rstring_prefix_end: *mut c_char = r#in.sub (1);
		    while TRUE != 0
		      {
			c = GET! (c);
			match c
			  {
			  _ if c == b'\n' as c_int =>
			    { INCREMENT_LINENO! (); }
			  _ if c == b')' as c_int =>
			    {
			      let in_saved: *mut c_char = r#in;
			      let mut prefix: *mut c_char = rstring_prefix_start;
			      while prefix != rstring_prefix_end && GET! (c) == *prefix as c_int
				{
				  RSTRING_EOF_CHECK! (c);
				  prefix = prefix.add (1);
				}
			      if prefix == rstring_prefix_end
				{
				  if GET! (c) == b'"' as c_int
				    { return CSTRING; }
				  RSTRING_EOF_CHECK! (c);
				}
			      r#in = in_saved;
			    }
			  _ =>
			    {
			      RSTRING_EOF_CHECK! (c);
			    }
			  }
		      }
		  }

		UNGET! ();
		/* Fall through to identifiers and keywords.  */
		FALLTHROUGH! ();
	      }

	    /* Identifier and keywords.  */
	    let mut hash: c_uint;
	    let mut k: *mut kw;

	    p = yytext;
	    hash = c as c_uint;
	    *p = hash as c_char;
	    p = p.add (1);

	    while IDENTP! (GET! (*p))
	      {
		hash = (hash << 1) ^ *p as c_uint;
		p = p.add (1);
		if p == yytext_end.sub (1)
		  {
		    let size: c_int = (yytext_end as isize - yytext as isize) as c_int;
		    yytext = xrealloc (yytext as *mut c_void, 2 * size as size_t) as *mut c_char;
		    yytext_end = yytext.add (2 * size as usize);
		    p = yytext.add (size as usize - 1);
		  }
	      }

	    UNGET! ();
	    *p = 0;

	    k = keyword_table[(hash % KEYWORD_TABLE_SIZE as c_uint) as usize];
	    while !k.is_null ()
	      {
		if streq ((*k).name, yytext)
		  { return (*k).tk; }
		k = (*k).next;
	      }

	    return IDENT;
	  }

	_ if c == b'/' as c_int =>
	  {
	    /* C and C++ comments, '/' and '/='.  */
	    match GET! (c)
	      {
	      _ if c == b'*' as c_int =>
		{
		  'comment_end: loop
		    {
		      c = GET! (c);
		      if c == 0
			{ break 'comment_end; }
		      match c
			{
			_ if c == b'*' as c_int =>
			  {
			    if GET! (c) == b'/' as c_int
			      { break 'comment_end; }
			    UNGET! ();
			  }
			_ if c == b'\\' as c_int =>
			  {
			    GET! (c);
			  }
			_ if c == b'\n' as c_int =>
			  {
			    INCREMENT_LINENO! ();
			  }
			_ =>
			  { }
			}
		    }
		}

	      _ if c == b'=' as c_int =>
		{ return DIVASGN; }

	      _ if c == b'/' as c_int =>
		{
		  while GET! (c) != 0 && c != b'\n' as c_int
		    { }
		  /* Don't try to read past the end of the input buffer if
		     the file ends in a C++ comment without a newline.  */
		  if c == 0
		    { return YYEOF; }

		  INCREMENT_LINENO! ();
		}

	      _ =>
		{
		  UNGET! ();
		  return b'/' as c_int;
		}
	      }
	  }

	_ if c == b'+' as c_int =>
	  {
	    if GET! (c) == b'+' as c_int
	      { return INC; }
	    else if c == b'=' as c_int
	      { return ADDASGN; }
	    UNGET! ();
	    return b'+' as c_int;
	  }

	_ if c == b'-' as c_int =>
	  {
	    match GET! (c)
	      {
	      _ if c == b'-' as c_int =>
		{ return DEC; }
	      _ if c == b'>' as c_int =>
		{
		  if GET! (c) == b'*' as c_int
		    { return ARROWSTAR; }
		  UNGET! ();
		  return ARROW;
		}
	      _ if c == b'=' as c_int =>
		{ return SUBASGN; }
	      _ =>
		{ }
	      }
	    UNGET! ();
	    return b'-' as c_int;
	  }

	_ if c == b'*' as c_int =>
	  {
	    if GET! (c) == b'=' as c_int
	      { return MULASGN; }
	    UNGET! ();
	    return b'*' as c_int;
	  }

	_ if c == b'%' as c_int =>
	  {
	    if GET! (c) == b'=' as c_int
	      { return MODASGN; }
	    UNGET! ();
	    return b'%' as c_int;
	  }

	_ if c == b'|' as c_int =>
	  {
	    if GET! (c) == b'|' as c_int
	      { return LOR; }
	    else if c == b'=' as c_int
	      { return ORASGN; }
	    UNGET! ();
	    return b'|' as c_int;
	  }

	_ if c == b'&' as c_int =>
	  {
	    if GET! (c) == b'&' as c_int
	      { return LAND; }
	    else if c == b'=' as c_int
	      { return ANDASGN; }
	    UNGET! ();
	    return b'&' as c_int;
	  }

	_ if c == b'^' as c_int =>
	  {
	    if GET! (c) == b'=' as c_int
	      { return XORASGN; }
	    UNGET! ();
	    return b'^' as c_int;
	  }

	_ if c == b'.' as c_int =>
	  {
	    if GET! (c) == b'*' as c_int
	      { return POINTSTAR; }
	    else if c == b'.' as c_int
	      {
		if GET! (c) != b'.' as c_int
		  { yyerror (c"invalid token '..' ('...' assumed)".as_ptr (), std::ptr::null ()); }
		UNGET! ();
		return ELLIPSIS;
	      }
	    else if !DIGITP! (c)
	      {
		UNGET! ();
		return b'.' as c_int;
	      }
	    /* C 1773: `goto mantissa;` - the `mantissa:` label's statement sequence
	       (C 1888-1908), inlined at each jump site (PORTING.md 2.2: the label is a
	       statement of the enclosing `switch`, which the port's `match` cannot
	       re-enter).  */
	    'mantissa: {
	      /* Digits following '.'.  */
	      while DIGITP! (c)
		{ GET! (c); }

	      /* Optional exponent.  */
	      if c == b'E' as c_int || c == b'e' as c_int
		{
		  if GET! (c) == b'-' as c_int || c == b'+' as c_int
		    { GET! (c); }

		  while DIGITP! (c)
		    { GET! (c); }
		}

	      /* Optional type suffixes.  */
	      while c_isalpha (c)
		{ GET! (c); }
	      UNGET! ();
	      return CFLOAT;
	    }
	  }

	_ if c == b':' as c_int =>
	  {
	    if GET! (c) == b':' as c_int
	      { return DCOLON; }
	    UNGET! ();
	    return b':' as c_int;
	  }

	_ if c == b'=' as c_int =>
	  {
	    if GET! (c) == b'=' as c_int
	      { return EQ; }
	    UNGET! ();
	    return b'=' as c_int;
	  }

	_ if c == b'!' as c_int =>
	  {
	    if GET! (c) == b'=' as c_int
	      { return NE; }
	    UNGET! ();
	    return b'!' as c_int;
	  }

	_ if c == b'<' as c_int =>
	  {
	    match GET! (c)
	      {
	      _ if c == b'=' as c_int =>
		{ return LE; }
	      _ if c == b'<' as c_int =>
		{
		  if GET! (c) == b'=' as c_int
		    { return LSHIFTASGN; }
		  UNGET! ();
		  return LSHIFT;
		}
	      _ =>
		{ }
	      }
	    UNGET! ();
	    return b'<' as c_int;
	  }

	_ if c == b'>' as c_int =>
	  {
	    match GET! (c)
	      {
	      _ if c == b'=' as c_int =>
		{ return GE; }
	      _ if c == b'>' as c_int =>
		{
		  if GET! (c) == b'=' as c_int
		    { return RSHIFTASGN; }
		  UNGET! ();
		  return RSHIFT;
		}
	      _ =>
		{ }
	      }
	    UNGET! ();
	    return b'>' as c_int;
	  }

	_ if c == b'#' as c_int =>
	  {
	    c = process_pp_line ();
	    if c == 0
	      { return YYEOF; }
	  }

	_ if c == b'(' as c_int || c == b')' as c_int || c == b'[' as c_int
	     || c == b']' as c_int || c == b'{' as c_int || c == b'}' as c_int
	     || c == b';' as c_int || c == b',' as c_int || c == b'?' as c_int
	     || c == b'~' as c_int =>
	  {
	    return c;
	  }

	_ if c == b'0' as c_int =>
	  {
	    yyival = 0;

	    if GET! (c) == b'x' as c_int || c == b'X' as c_int
	      {
		loop
		  {
		    GET! (c);
		    if c == 0
		      { break; }
		    if DIGITP! (c)
		      { yyival = yyival.wrapping_mul (16).wrapping_add (c as c_uint)
				       .wrapping_sub (b'0' as c_uint); }
		    else if c >= b'a' as c_int && c <= b'f' as c_int
		      { yyival = yyival.wrapping_mul (16).wrapping_add (c as c_uint)
				       .wrapping_sub (b'a' as c_uint).wrapping_add (10); }
		    else if c >= b'A' as c_int && c <= b'F' as c_int
		      { yyival = yyival.wrapping_mul (16).wrapping_add (c as c_uint)
				       .wrapping_sub (b'A' as c_uint).wrapping_add (10); }
		    else
		      { break; }
		  }

		/* C 1856: `goto int_suffixes;` - the label's sequence (C 1861-1864).  */
		'int_suffixes: {
		  while c_isalpha (c)
		    { GET! (c); }
		  UNGET! ();
		  return CINT;
		}
	      }
	    else if c == b'.' as c_int
	      {
	    'mantissa: {
	      /* Digits following '.'.  */
	      while DIGITP! (c)
		{ GET! (c); }

	      /* Optional exponent.  */
	      if c == b'E' as c_int || c == b'e' as c_int
		{
		  if GET! (c) == b'-' as c_int || c == b'+' as c_int
		    { GET! (c); }

		  while DIGITP! (c)
		    { GET! (c); }
		}

	      /* Optional type suffixes.  */
	      while c_isalpha (c)
		{ GET! (c); }
	      UNGET! ();
	      return CFLOAT;
	    }
	      }

	    while c >= b'0' as c_int && c <= b'7' as c_int
	      {
		yyival = (yyival << 3).wrapping_add (c as c_uint).wrapping_sub (b'0' as c_uint);
		GET! (c);
	      }

	    /* C 1860: `int_suffixes:` - the octal path falls through into the label and
	       C 1861-1864 are its statements.  */
	    'int_suffixes: {
	      while c_isalpha (c)
		{ GET! (c); }
	      UNGET! ();
	      return CINT;
	    }
	  }

	_ if c == b'1' as c_int || c == b'2' as c_int || c == b'3' as c_int
	     || c == b'4' as c_int || c == b'5' as c_int || c == b'6' as c_int
	     || c == b'7' as c_int || c == b'8' as c_int || c == b'9' as c_int =>
	  {
	    /* Integer or floating constant, part before '.'.  */
	    yyival = c as c_uint - b'0' as c_uint;

	    loop
	      {
		GET! (c);
		if !(c != 0 && DIGITP! (c))
		  { break; }
		yyival = yyival.wrapping_mul (10).wrapping_add (c as c_uint)
				    .wrapping_sub (b'0' as c_uint);
	      }

	    if c != b'.' as c_int
	      {
		/* C 1885: `goto int_suffixes;` - the same label sequence as C 1856's.  */
		'int_suffixes: {
		  while c_isalpha (c)
		    { GET! (c); }
		  UNGET! ();
		  return CINT;
		}
	      }

	    /* C 1887: `mantissa:` - this arm falls through into the label.  */
	    'mantissa: {
	      /* Digits following '.'.  */
	      while DIGITP! (c)
		{ GET! (c); }

	      /* Optional exponent.  */
	      if c == b'E' as c_int || c == b'e' as c_int
		{
		  if GET! (c) == b'-' as c_int || c == b'+' as c_int
		    { GET! (c); }

		  while DIGITP! (c)
		    { GET! (c); }
		}

	      /* Optional type suffixes.  */
	      while c_isalpha (c)
		{ GET! (c); }
	      UNGET! ();
	      return CFLOAT;
	    }
	  }

	_ => { }
	}
    }
}

/* Actually local to matching_regexp.  These variables must be in
   global scope for the case that `static' gets defined away.  */

static mut matching_regexp_buffer: *mut c_char = std::ptr::null_mut ();
static mut matching_regexp_end_buf: *mut c_char = std::ptr::null_mut ();

/* Value is the string from the start of the line to the current
   position in the input buffer, or maybe a bit more if that string is
   shorter than min_regexp.  */

unsafe fn matching_regexp () -> *mut c_char
{
  let mut p: *mut c_char;
  let mut s: *mut c_char;
  let mut t: *mut c_char;

  if f_regexps == 0
    { return std::ptr::null_mut (); }

  if matching_regexp_buffer.is_null ()
    {
      matching_regexp_buffer = xmalloc (max_regexp as size_t) as *mut c_char;
      matching_regexp_end_buf = matching_regexp_buffer.add (max_regexp as usize).sub (1);
    }

  /* Scan back to previous newline of buffer start.  */
  p = r#in.sub (1);
  while p > inbuffer && *p != b'\n' as c_char
    { p = p.sub (1); }

  if *p == b'\n' as c_char
    {
      while (r#in as isize - p as isize) < min_regexp as isize && p > inbuffer
	{
	  /* Line probably not significant enough */
	  p = p.sub (1);
	  while p > inbuffer && *p != b'\n' as c_char
	    { p = p.sub (1); }
	}
      if *p == b'\n' as c_char
	{ p = p.add (1); }
    }

  /* Copy from end to make sure significant portions are included.
     This implies that in the browser a regular expressing of the form
     `^.*{regexp}' has to be used.  */
  s = matching_regexp_end_buf.sub (1);
  t = r#in;
  while s > matching_regexp_buffer && t > p
    {
      s = s.sub (1);
      t = t.sub (1);
      *s = *t;

      if *s == b'"' as c_char || *s == b'\\' as c_char
	{
	  if s > matching_regexp_buffer
	    { s = s.sub (1); *s = b'\\' as c_char; }
	  else
	    {
	      s = s.add (1);
	      break;
	    }
	}
    }

  *matching_regexp_end_buf.sub (1) = 0;
  return xstrdup (s);
}

/* Return a printable representation of token T.  */

unsafe fn token_string (t: c_int) -> *const c_char
{
  static mut b: [c_char; 3] = [0; 3];

  match t
    {
    CSTRING => { return c"string constant".as_ptr (); }
    CCHAR => { return c"char constant".as_ptr (); }
    CINT => { return c"int constant".as_ptr (); }
    CFLOAT => { return c"floating constant".as_ptr (); }
    ELLIPSIS => { return c"...".as_ptr (); }
    LSHIFTASGN => { return c"<<=".as_ptr (); }
    RSHIFTASGN => { return c">>=".as_ptr (); }
    ARROWSTAR => { return c"->*".as_ptr (); }
    IDENT => { return c"identifier".as_ptr (); }
    DIVASGN => { return c"/=".as_ptr (); }
    INC => { return c"++".as_ptr (); }
    ADDASGN => { return c"+=".as_ptr (); }
    DEC => { return c"--".as_ptr (); }
    ARROW => { return c"->".as_ptr (); }
    SUBASGN => { return c"-=".as_ptr (); }
    MULASGN => { return c"*=".as_ptr (); }
    MODASGN => { return c"%=".as_ptr (); }
    LOR => { return c"||".as_ptr (); }
    ORASGN => { return c"|=".as_ptr (); }
    LAND => { return c"&&".as_ptr (); }
    ANDASGN => { return c"&=".as_ptr (); }
    XORASGN => { return c"^=".as_ptr (); }
    POINTSTAR => { return c".*".as_ptr (); }
    DCOLON => { return c"::".as_ptr (); }
    EQ => { return c"==".as_ptr (); }
    NE => { return c"!=".as_ptr (); }
    LE => { return c"<=".as_ptr (); }
    LSHIFT => { return c"<<".as_ptr (); }
    GE => { return c">=".as_ptr (); }
    RSHIFT => { return c">>".as_ptr (); }
    ASM => { return c"asm".as_ptr (); }
    AUTO => { return c"auto".as_ptr (); }
    BREAK => { return c"break".as_ptr (); }
    CASE => { return c"case".as_ptr (); }
    CATCH => { return c"catch".as_ptr (); }
    CHAR => { return c"char".as_ptr (); }
    CLASS => { return c"class".as_ptr (); }
    CONST => { return c"const".as_ptr (); }
    CONTINUE => { return c"continue".as_ptr (); }
    DEFAULT => { return c"default".as_ptr (); }
    DELETE => { return c"delete".as_ptr (); }
    DO => { return c"do".as_ptr (); }
    DOUBLE => { return c"double".as_ptr (); }
    ELSE => { return c"else".as_ptr (); }
    ENUM => { return c"enum".as_ptr (); }
    EXTERN => { return c"extern".as_ptr (); }
    FLOAT => { return c"float".as_ptr (); }
    FOR => { return c"for".as_ptr (); }
    FRIEND => { return c"friend".as_ptr (); }
    GOTO => { return c"goto".as_ptr (); }
    IF => { return c"if".as_ptr (); }
    T_INLINE => { return c"inline".as_ptr (); }
    INT => { return c"int".as_ptr (); }
    LONG => { return c"long".as_ptr (); }
    NEW => { return c"new".as_ptr (); }
    OPERATOR => { return c"operator".as_ptr (); }
    PRIVATE => { return c"private".as_ptr (); }
    PROTECTED => { return c"protected".as_ptr (); }
    PUBLIC => { return c"public".as_ptr (); }
    REGISTER => { return c"register".as_ptr (); }
    RETURN => { return c"return".as_ptr (); }
    SHORT => { return c"short".as_ptr (); }
    SIGNED => { return c"signed".as_ptr (); }
    SIZEOF => { return c"sizeof".as_ptr (); }
    STATIC => { return c"static".as_ptr (); }
    STRUCT => { return c"struct".as_ptr (); }
    SWITCH => { return c"switch".as_ptr (); }
    TEMPLATE => { return c"template".as_ptr (); }
    THIS => { return c"this".as_ptr (); }
    THROW => { return c"throw".as_ptr (); }
    TRY => { return c"try".as_ptr (); }
    TYPEDEF => { return c"typedef".as_ptr (); }
    UNION => { return c"union".as_ptr (); }
    UNSIGNED => { return c"unsigned".as_ptr (); }
    VIRTUAL => { return c"virtual".as_ptr (); }
    VOID => { return c"void".as_ptr (); }
    VOLATILE => { return c"volatile".as_ptr (); }
    WHILE => { return c"while".as_ptr (); }
    MUTABLE => { return c"mutable".as_ptr (); }
    BOOL => { return c"bool".as_ptr (); }
    TRUE => { return c"true".as_ptr (); }
    FALSE => { return c"false".as_ptr (); }
    SIGNATURE => { return c"signature".as_ptr (); }
    NAMESPACE => { return c"namespace".as_ptr (); }
    EXPLICIT => { return c"explicit".as_ptr (); }
    TYPENAME => { return c"typename".as_ptr (); }
    CONST_CAST => { return c"const_cast".as_ptr (); }
    DYNAMIC_CAST => { return c"dynamic_cast".as_ptr (); }
    REINTERPRET_CAST => { return c"reinterpret_cast".as_ptr (); }
    STATIC_CAST => { return c"static_cast".as_ptr (); }
    TYPEID => { return c"typeid".as_ptr (); }
    USING => { return c"using".as_ptr (); }
    WCHAR => { return c"wchar_t".as_ptr (); }
    YYEOF => { return c"EOF".as_ptr (); }
    FINAL => { return c"final".as_ptr (); }

    _ =>
      {
	if t < 255
	  {
	    b[0] = t as c_char;
	    b[1] = 0;
	    return b.as_mut_ptr ();
	  }
	else
	  { return c"???".as_ptr (); }
      }
    }
}

/* Reinitialize the scanner for a new input file.  */

unsafe fn re_init_scanner ()
{
  r#in = inbuffer;
  yyline = 1;

  if yytext.is_null ()
    {
      let size: c_int = 256;
      yytext = xmalloc (size as size_t * std::mem::size_of::<c_char> ()) as *mut c_char;
      yytext_end = yytext.add (size as usize);
    }
}

/* Insert a keyword NAME with token value TKV into the keyword hash
   table.  */

unsafe fn insert_keyword (name: *const c_char, tkv: c_int)
{
  let mut s: *const c_char;
  let mut h: c_uint = 0;
  let k: *mut kw = xmalloc (std::mem::size_of::<kw> ()) as *mut kw;

  s = name;
  while *s != 0
    {
      h = (h << 1) ^ *s as c_uint;
      s = s.add (1);
    }

  h %= KEYWORD_TABLE_SIZE as c_uint;
  (*k).name = name;
  (*k).tk = tkv;
  (*k).next = keyword_table[h as usize];
  keyword_table[h as usize] = k;
}

/* Initialize the scanner for the first file.  This sets up the
   character class vectors and fills the keyword hash table.  */

unsafe fn init_scanner ()
{
  let mut i: c_int;

  /* Allocate the input buffer */
  inbuffer_size = READ_CHUNK_SIZE as size_t + 1;
  inbuffer = xmalloc (inbuffer_size) as *mut c_char;
  r#in = inbuffer;
  yyline = 1;

  /* Set up character class vectors.  */
  i = 0;
  while i < std::mem::size_of_val (&is_ident) as c_int
    {
      if i == b'_' as c_int || c_isalnum (i)
        { is_ident[i as usize] = 1; }

      if i >= b'0' as c_int && i <= b'9' as c_int
        { is_digit[i as usize] = 1; }

      if i == b' ' as c_int || i == b'\t' as c_int || i == 0x0c
	 || i == 0x0b
        { is_white[i as usize] = 1; }
      i += 1;
    }

  /* Fill keyword hash table.  */
  insert_keyword (c"and".as_ptr (), LAND);
  insert_keyword (c"and_eq".as_ptr (), ANDASGN);
  insert_keyword (c"asm".as_ptr (), ASM);
  insert_keyword (c"auto".as_ptr (), AUTO);
  insert_keyword (c"bitand".as_ptr (), b'&' as c_int);
  insert_keyword (c"bitor".as_ptr (), b'|' as c_int);
  insert_keyword (c"bool".as_ptr (), BOOL);
  insert_keyword (c"break".as_ptr (), BREAK);
  insert_keyword (c"case".as_ptr (), CASE);
  insert_keyword (c"catch".as_ptr (), CATCH);
  insert_keyword (c"char".as_ptr (), CHAR);
  insert_keyword (c"class".as_ptr (), CLASS);
  insert_keyword (c"compl".as_ptr (), b'~' as c_int);
  insert_keyword (c"const".as_ptr (), CONST);
  insert_keyword (c"const_cast".as_ptr (), CONST_CAST);
  insert_keyword (c"continue".as_ptr (), CONTINUE);
  insert_keyword (c"default".as_ptr (), DEFAULT);
  insert_keyword (c"delete".as_ptr (), DELETE);
  insert_keyword (c"do".as_ptr (), DO);
  insert_keyword (c"double".as_ptr (), DOUBLE);
  insert_keyword (c"dynamic_cast".as_ptr (), DYNAMIC_CAST);
  insert_keyword (c"else".as_ptr (), ELSE);
  insert_keyword (c"enum".as_ptr (), ENUM);
  insert_keyword (c"explicit".as_ptr (), EXPLICIT);
  insert_keyword (c"extern".as_ptr (), EXTERN);
  insert_keyword (c"false".as_ptr (), FALSE);
  insert_keyword (c"final".as_ptr (), FINAL);
  insert_keyword (c"float".as_ptr (), FLOAT);
  insert_keyword (c"for".as_ptr (), FOR);
  insert_keyword (c"friend".as_ptr (), FRIEND);
  insert_keyword (c"goto".as_ptr (), GOTO);
  insert_keyword (c"if".as_ptr (), IF);
  insert_keyword (c"inline".as_ptr (), T_INLINE);
  insert_keyword (c"int".as_ptr (), INT);
  insert_keyword (c"long".as_ptr (), LONG);
  insert_keyword (c"mutable".as_ptr (), MUTABLE);
  insert_keyword (c"namespace".as_ptr (), NAMESPACE);
  insert_keyword (c"new".as_ptr (), NEW);
  insert_keyword (c"not".as_ptr (), b'!' as c_int);
  insert_keyword (c"not_eq".as_ptr (), NE);
  insert_keyword (c"operator".as_ptr (), OPERATOR);
  insert_keyword (c"or".as_ptr (), LOR);
  insert_keyword (c"or_eq".as_ptr (), ORASGN);
  insert_keyword (c"private".as_ptr (), PRIVATE);
  insert_keyword (c"protected".as_ptr (), PROTECTED);
  insert_keyword (c"public".as_ptr (), PUBLIC);
  insert_keyword (c"register".as_ptr (), REGISTER);
  insert_keyword (c"reinterpret_cast".as_ptr (), REINTERPRET_CAST);
  insert_keyword (c"return".as_ptr (), RETURN);
  insert_keyword (c"short".as_ptr (), SHORT);
  insert_keyword (c"signed".as_ptr (), SIGNED);
  insert_keyword (c"sizeof".as_ptr (), SIZEOF);
  insert_keyword (c"static".as_ptr (), STATIC);
  insert_keyword (c"static_cast".as_ptr (), STATIC_CAST);
  insert_keyword (c"struct".as_ptr (), STRUCT);
  insert_keyword (c"switch".as_ptr (), SWITCH);
  insert_keyword (c"template".as_ptr (), TEMPLATE);
  insert_keyword (c"this".as_ptr (), THIS);
  insert_keyword (c"throw".as_ptr (), THROW);
  insert_keyword (c"true".as_ptr (), TRUE);
  insert_keyword (c"try".as_ptr (), TRY);
  insert_keyword (c"typedef".as_ptr (), TYPEDEF);
  insert_keyword (c"typeid".as_ptr (), TYPEID);
  insert_keyword (c"typename".as_ptr (), TYPENAME);
  insert_keyword (c"union".as_ptr (), UNION);
  insert_keyword (c"unsigned".as_ptr (), UNSIGNED);
  insert_keyword (c"using".as_ptr (), USING);
  insert_keyword (c"virtual".as_ptr (), VIRTUAL);
  insert_keyword (c"void".as_ptr (), VOID);
  insert_keyword (c"volatile".as_ptr (), VOLATILE);
  insert_keyword (c"wchar_t".as_ptr (), WCHAR);
  insert_keyword (c"while".as_ptr (), WHILE);
  insert_keyword (c"xor".as_ptr (), b'^' as c_int);
  insert_keyword (c"xor_eq".as_ptr (), XORASGN);
}

/***********************************************************************
				Parser
 ***********************************************************************/

/* Match the current lookahead token and set it to the next token.  */

/// C 2250: `#define MATCH() (tk = yylex ())`
macro_rules! MATCH { () => { tk = yylex () }; }

/* Return the lookahead token.  If current lookahead token is cleared,
   read a new token.  */

/// C 2255: `#define LA1 (tk == -1 ? (tk = yylex ()) : tk)`
macro_rules! LA1 { () => { { if tk == -1 { tk = yylex (); } tk } }; }

/* Is the current lookahead equal to the token T? */

/// C 2259: `#define LOOKING_AT(T) (tk == (T))`
macro_rules! LOOKING_AT { ($t:expr) => { tk == ($t) }; }

/* Is the current lookahead one of T1 or T2?  */

/// C 2263: `#define LOOKING_AT2(T1, T2)	(tk == (T1) || tk == (T2))`
macro_rules! LOOKING_AT2 { ($t1:expr, $t2:expr) => { tk == ($t1) || tk == ($t2) }; }

/* Is the current lookahead one of T1, T2 or T3?  */

/// C 2267: `#define LOOKING_AT3(T1, T2, T3)	(tk == (T1) || tk == (T2) || tk == (T3))`
macro_rules! LOOKING_AT3 { ($t1:expr, $t2:expr, $t3:expr) => { tk == ($t1) || tk == ($t2) || tk == ($t3) }; }

/* Is the current lookahead one of T1...T4?  */

/// C 2271-2272: `#define LOOKING_AT4(T1, T2, T3, T4) (tk == (T1) || … || tk == (T4))`
macro_rules! LOOKING_AT4 {
  ($t1:expr, $t2:expr, $t3:expr, $t4:expr) => {
    tk == ($t1) || tk == ($t2) || tk == ($t3) || tk == ($t4)
  };
}

/* Match token T if current lookahead is T.  */

/// C 2276: `#define MATCH_IF(T) if (LOOKING_AT (T)) MATCH (); else ((void) 0)`
/* `((void) 0)` is a discarded zero (the `lib/getdelim.rs` 52 `flockfile` shape).  */
macro_rules! MATCH_IF {
  ($t:expr) => { if LOOKING_AT! ($t) { MATCH! (); } else { let _ = 0; } };
}

/* Skip to matching token if current token is T.  */

/// C 2280-2281: `#define SKIP_MATCHING_IF(T) if (LOOKING_AT (T)) skip_matching (); else ((void) 0)`
macro_rules! SKIP_MATCHING_IF {
  ($t:expr) => { if LOOKING_AT! ($t) { skip_matching (); } else { let _ = 0; } };
}

/* Skip forward until a given token TOKEN or YYEOF is seen and return
   the current lookahead token after skipping.  */

unsafe fn skip_to (token: c_int) -> c_int
{
  while !LOOKING_AT2! (YYEOF, token)
    { MATCH! (); }
  return tk;
}

/* Skip over pairs of tokens (parentheses, square brackets,
   angle brackets, curly brackets) matching the current lookahead.  */

unsafe fn skip_matching ()
{
  let mut open: c_int;
  let mut close: c_int;
  let mut n: c_int;

  open = LA1! ();
  match open
    {
    _ if open == b'{' as c_int =>
      { close = b'}' as c_int; }

    _ if open == b'(' as c_int =>
      { close = b')' as c_int; }

    _ if open == b'<' as c_int =>
      { close = b'>' as c_int; }

    _ if open == b'[' as c_int =>
      { close = b']' as c_int; }

    _ =>
      { abort (); }
    }

  n = 0;
  loop
    {
      if LOOKING_AT! (open)
	{ n += 1; }
      else if LOOKING_AT! (close)
	{ n -= 1; }
      else if LOOKING_AT! (YYEOF)
	{ break; }

      MATCH! ();

      if n == 0
	{ break; }
    }
}

unsafe fn skip_initializer ()
{
  loop
    {
      match LA1! ()
	{
	_ if LA1! () == b';' as c_int || LA1! () == b',' as c_int || LA1! () == YYEOF =>
	  { return; }

	_ if LA1! () == b'{' as c_int || LA1! () == b'[' as c_int || LA1! () == b'(' as c_int =>
	  {
	    skip_matching ();
	  }

	_ =>
	  {
	    MATCH! ();
	  }
	}
    }
}

/* Build qualified namespace alias (A::B::c) and return it. */

unsafe fn match_qualified_namespace_alias () -> *mut link
{
  let mut head: *mut link = std::ptr::null_mut ();
  let mut cur: *mut link = std::ptr::null_mut ();
  let mut tmp: *mut link = std::ptr::null_mut ();

  loop
    {
      MATCH! ();
      match LA1! ()
        {
        IDENT =>
	  {
	    tmp = xmalloc (std::mem::size_of::<link> ()) as *mut link;
	    (*tmp).sym = find_namespace (yytext, if !cur.is_null () { (*cur).sym } else { std::ptr::null_mut () });
	    (*tmp).next = std::ptr::null_mut ();
	    if !head.is_null ()
	      {
		(*cur).next = tmp;
		cur = tmp;
	      }
	    else
	      {
		head = tmp;
		cur = tmp;
	      }
	  }
        DCOLON =>
	  {
	    /* Just skip */
	  }
        _ =>
	  {
	    return head;
	  }
        }
    }
}

/* Re-initialize the parser by resetting the lookahead token.  */

unsafe fn re_init_parser ()
{
  tk = -1;
}

/* Parse a parameter list, including the const-specifier,
   pure-specifier, and throw-list that may follow a parameter list.
   Return in FLAGS what was seen following the parameter list.
   Returns a hash code for the parameter types.  This value is used to
   distinguish between overloaded functions.  */

unsafe fn parm_list (flags: *mut c_int) -> c_uint
{
  let mut hash: c_uint = 0;
  let mut type_seen: c_int = 0;

  while !LOOKING_AT2! (YYEOF, b')' as c_int)
    {
      match LA1! ()
        {
	  /* Skip over grouping parens or parameter lists in parameter
	     declarations.  */
        _ if LA1! () == b'(' as c_int =>
	  {
	    skip_matching ();
	  }

	  /* Next parameter.  */
        _ if LA1! () == b',' as c_int =>
	  {
	    MATCH! ();
	    type_seen = 0;
	  }

          /* Ignore the scope part of types, if any.  This is because
             some types need scopes when defined outside of a class body,
             and don't need them inside the class body.  This means that
             we have to look for the last IDENT in a sequence of
             IDENT::IDENT::...  */
        IDENT =>
	  {
	    if type_seen == 0
	      {
		let mut last_id: *mut c_char = std::ptr::null_mut ();
		let mut ident_type_hash: c_uint = 0;

		parse_qualified_param_ident_or_type (&raw mut last_id);
		if !last_id.is_null ()
		  {
		    /* LAST_ID null means something like `X::*'.  */
		    while *last_id != 0
		      {
			ident_type_hash = (ident_type_hash << 1) ^ *last_id as c_uint;
			last_id = last_id.add (1);
		      }
		    hash = (hash << 1) ^ ident_type_hash;
		    type_seen = 1;
		  }
	      }
	    else
	      { MATCH! (); }
	  }

        VOID =>
	  {
	    /* This distinction is made to make `func (void)' equivalent
	       to `func ()'.  */
	    type_seen = 1;
	    MATCH! ();
	    if !LOOKING_AT! (b')' as c_int)
	      { hash = (hash << 1) ^ VOID as c_uint; }
	  }

	BOOL => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	CHAR => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	CLASS => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	CONST => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	DOUBLE => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	ENUM => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	FLOAT => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	INT => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	LONG => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	SHORT => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	SIGNED => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	STRUCT => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	UNION => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	UNSIGNED => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	VOLATILE => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	WCHAR => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }
	ELLIPSIS => { type_seen = 1; hash = (hash << 1) ^ LA1! () as c_uint; MATCH! (); }

        _ if LA1! () == b'*' as c_int || LA1! () == b'&' as c_int
	     || LA1! () == b'[' as c_int || LA1! () == b']' as c_int =>
	  {
	    hash = (hash << 1) ^ LA1! () as c_uint;
	    MATCH! ();
	  }

        _ =>
	  {
	    MATCH! ();
	  }
        }
    }

  if LOOKING_AT! (b')' as c_int)
    {
      MATCH! ();

      if LOOKING_AT! (CONST)
	{
	  /* We can overload the same function on `const' */
	  hash = (hash << 1) ^ CONST as c_uint;
	  set_flag (flags, F_CONST);
	  MATCH! ();
	}

      if LOOKING_AT! (THROW)
	{
	  MATCH! ();
	  SKIP_MATCHING_IF! (b'(' as c_int);
	  set_flag (flags, F_THROW);
	}

      if LOOKING_AT! (b'=' as c_int)
	{
	  MATCH! ();
	  if LOOKING_AT! (CINT) && yyival == 0
	    {
	      MATCH! ();
	      set_flag (flags, F_PURE);
	    }
	}
    }

  return hash;
}

/* Print position info to stdout.  */

unsafe fn print_info ()
{
  if info_position >= 0 && BUFFER_POS! () <= info_position
    {
      if !info_cls.is_null ()
	{
	  printf (c"(\"%s\" \"%s\" \"%s\" %d)\n".as_ptr (),
		  FAM_PTR! ((*info_cls).name), sym_scope (info_cls),
		  FAM_PTR! ((*info_member).name), info_where);
	}
    }
}

/* Parse a member declaration within the class body of CLS.  VIS is
   the access specifier for the member (private, protected,
   public).  */

/* C's `typeseen:` label (C 2742) is reached by `goto` from eight arms and by falling
   through from the type-keyword arm; the port inlines its statements at each site inside a
   labelled block that keeps C's name (PORTING.md 2.2).  */
#[allow(unused_labels)]
unsafe fn member (cls: *mut sym, vis: c_int)
{
  let mut id: *mut c_char = std::ptr::null_mut ();
  let mut sc: c_int = SC_MEMBER;
  let mut regexp: *mut c_char = std::ptr::null_mut ();
  let mut pos: c_int;
  let mut is_constructor: c_int;
  let mut flags: c_int = 0;
  let mut class_tag: c_int;
  let mut class_name: *mut c_char;
  let mut type_seen: c_int = 0;
  let mut paren_seen: c_int = 0;
  let mut hash: c_uint = 0;
  let mut tilde: c_int = 0;

  while !LOOKING_AT4! (b';' as c_int, b'{' as c_int, b'}' as c_int, YYEOF)
    {
      match LA1! ()
        {
          /* A function or class may follow.  */
        TEMPLATE =>
	  {
	    MATCH! ();
	    set_flag (&raw mut flags, F_TEMPLATE);
	    /* Skip over template argument list */
	    SKIP_MATCHING_IF! (b'<' as c_int);
	  }

        EXPLICIT =>
	  {
	    set_flag (&raw mut flags, F_EXPLICIT);
	    /* C 2716: `goto typeseen;` - the `typeseen:` label's statements
	       (C 2743-2745) are inlined at each jump site, because the label is a
	       statement of the `switch` (PORTING.md 2.2).  */
	    type_seen = 1;
	    MATCH! ();
	  }

        MUTABLE =>
	  {
	    set_flag (&raw mut flags, F_MUTABLE);
	    /* C 2720: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        T_INLINE =>
	  {
	    set_flag (&raw mut flags, F_INLINE);
	    /* C 2724: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        VIRTUAL =>
	  {
	    set_flag (&raw mut flags, F_VIRTUAL);
	    /* C 2728: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        _ if LA1! () == b'[' as c_int =>
	  {
	    skip_matching ();
	  }

        ENUM =>
	  {
	    sc = SC_TYPE;
	    /* C 2737: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        TYPEDEF =>
	  {
	    sc = SC_TYPE;
	    /* C 2741: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        FRIEND =>
	  {
	    sc = SC_FRIEND;
	    /* C 2745: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        STATIC =>
	  {
	    sc = SC_STATIC;
	    /* C 2749: `goto typeseen;` */
	    type_seen = 1;
	    MATCH! ();
	  }

        _ if LA1! () == b'~' as c_int =>
	  {
	    tilde = 1;
	    MATCH! ();
	  }

        IDENT =>
	  {
	    /* Remember IDENTS seen so far.  Among these will be the member
	       name.  */
	    id = xrealloc (id as *mut c_void, strlen (yytext) + 2) as *mut c_char;
	    if tilde != 0
	      {
		*id = b'~' as c_char;
		strcpy (id.add (1), yytext);
	      }
	    else
	      { strcpy (id, yytext); }
	    MATCH! ();
	  }

        OPERATOR =>
	  {
	    let s: *mut c_char = operator_name (&raw mut sc);
	    id = xrealloc (id as *mut c_void, strlen (s) + 1) as *mut c_char;
	    strcpy (id, s);
	  }

        _ if LA1! () == b'(' as c_int =>
	  {
	    /* Most probably the beginning of a parameter list.  */
	    MATCH! ();
	    paren_seen = 1;

	    if !id.is_null () && !cls.is_null ()
	      {
		is_constructor = streq (id, FAM_PTR! ((*cls).name)) as c_int;
		if is_constructor == 0
		  { regexp = matching_regexp (); }
	      }
	    else
	      { is_constructor = 0; }

	    pos = BUFFER_POS! ();
	    hash = parm_list (&raw mut flags);

	    if is_constructor != 0
	      { regexp = matching_regexp (); }

	    if !id.is_null () && !cls.is_null ()
	      { add_member_decl (cls, id, regexp, pos, hash, 0, sc, vis, flags); }

	    while !LOOKING_AT3! (b';' as c_int, b'{' as c_int, YYEOF)
	      { MATCH! (); }

	    if LOOKING_AT! (b'{' as c_int) && !id.is_null () && !cls.is_null ()
	      { add_member_defn (cls, id, regexp, pos, hash, 0, sc, flags); }

	    free (id as *mut c_void);
	    id = std::ptr::null_mut ();
	    sc = SC_MEMBER;
	  }

        STRUCT | UNION | CLASS =>
	  {
	    /* Nested class */
	    class_tag = LA1! ();
	    type_seen = 1;
	    MATCH! ();
	    class_name = std::ptr::null_mut ();

	    /* More than one ident here to allow for MS-DOS specialties
	       like `_export class' etc.  The last IDENT seen counts
	       as the class name.  */
	    while !LOOKING_AT4! (YYEOF, b';' as c_int, b':' as c_int, b'{' as c_int)
	      {
		if LOOKING_AT! (IDENT)
                  {
                    if !class_name.is_null ()
                      {
			let size: c_int = strlen (yytext) as c_int;

			if (strlen (class_name) as c_int) < size
			  {
			    class_name = xrealloc (class_name as *mut c_void,
						   size as size_t + 1) as *mut c_char;
			  }

			memcpy (class_name as *mut c_void, yytext as *const c_void,
				size as size_t + 1);
                      }
                    else
                      {
			class_name = xstrdup (yytext);
                      }
                  }

		MATCH! ();
	      }

	    if LOOKING_AT2! (b':' as c_int, b'{' as c_int)
	      {
		class_definition (if !class_name.is_null () { cls } else { std::ptr::null_mut () },
				  if !class_name.is_null () { class_name } else { yytext },
				  class_tag, flags, 1);
	      }
	    else
	      { skip_to (b';' as c_int); }

	    free (class_name as *mut c_void);
	  }

        INT | CHAR | LONG | UNSIGNED | SIGNED | CONST | DOUBLE | VOID | SHORT
	  | VOLATILE | BOOL | WCHAR | TYPENAME =>
	  {
	    /* C 2742: `typeseen:` - this arm's fallthrough target is also the four
	       `goto typeseen` sites above; the label's statements are C 2743-2745.  */
	    'typeseen: {
	      type_seen = 1;
	      MATCH! ();
	    }
	  }

        _ =>
	  {
	    MATCH! ();
	  }
        }
    }

  if LOOKING_AT! (b';' as c_int)
    {
      /* The end of a member variable, a friend declaration or an access
         declaration.  We don't want to add friend classes as members.  */
      if !id.is_null () && sc != SC_FRIEND && !cls.is_null ()
        {
          regexp = matching_regexp ();
          pos = BUFFER_POS! ();

          if !cls.is_null ()
            {
              if type_seen != 0 || paren_seen == 0
		{ add_member_decl (cls, id, regexp, pos, 0, 1, sc, vis, 0); }
              else
		{ add_member_decl (cls, id, regexp, pos, hash, 0, sc, vis, 0); }
            }
        }

      MATCH! ();
      print_info ();
    }
  else if LOOKING_AT! (b'{' as c_int)
    {
      /* A named enum.  */
      if sc == SC_TYPE && !id.is_null () && !cls.is_null ()
        {
          regexp = matching_regexp ();
          pos = BUFFER_POS! ();

          if !cls.is_null ()
            {
              add_member_decl (cls, id, regexp, pos, 0, 1, sc, vis, 0);
              add_member_defn (cls, id, regexp, pos, 0, 1, sc, 0);
            }
        }

      skip_matching ();
      print_info ();
    }

  free (id as *mut c_void);
}

/* Parse the body of class CLS.  TAG is the tag of the class (struct,
   union, class).  */

unsafe fn class_body (cls: *mut sym, tag: c_int)
{
  let mut vis: c_int = if tag == CLASS { PRIVATE } else { PUBLIC };
  let mut temp: c_int;

  while !LOOKING_AT2! (YYEOF, b'}' as c_int)
    {
      match LA1! ()
        {
        PRIVATE | PROTECTED | PUBLIC =>
	  {
	    temp = LA1! ();
	    MATCH! ();

	    if LOOKING_AT! (b':' as c_int)
	      {
		vis = temp;
		MATCH! ();
	      }
	    else
	      {
		/* Probably conditional compilation for inheritance list.
		   We don't known whether there comes more of this.
		   This is only a crude fix that works most of the time.  */
		loop
		  {
		    MATCH! ();
		    if !(LOOKING_AT2! (IDENT, b',' as c_int)
			 || LOOKING_AT3! (PUBLIC, PROTECTED, PRIVATE))
		      { break; }
		  }
	      }
	  }

        TYPENAME | USING =>
	  {
	    skip_to (b';' as c_int);
	  }

          /* Try to synchronize */
        CHAR | CLASS | CONST | DOUBLE | ENUM | FLOAT | INT | LONG | SHORT | SIGNED
	  | STRUCT | UNION | UNSIGNED | VOID | VOLATILE | TYPEDEF | STATIC | T_INLINE
	  | FRIEND | VIRTUAL | TEMPLATE | IDENT | BOOL | WCHAR | EXPLICIT | MUTABLE =>
	  {
	    member (cls, vis);
	  }

        _ if LA1! () == b'~' as c_int =>
	  {
	    member (cls, vis);
	  }

        _ =>
	  {
	    MATCH! ();
	  }
        }
    }
}

/* Parse a qualified identifier.  Current lookahead is IDENT.  A
   qualified ident has the form `X<..>::Y<...>::T<...>.  Returns a
   symbol for that class.  */

unsafe fn parse_classname () -> *mut sym
{
  let mut last_class: *mut sym = std::ptr::null_mut ();

  while LOOKING_AT! (IDENT)
    {
      last_class = add_sym (yytext, last_class);
      MATCH! ();

      if LOOKING_AT! (b'<' as c_int)
        {
          skip_matching ();
          set_flag (&raw mut (*last_class).flags, F_TEMPLATE);
        }

      if !LOOKING_AT! (DCOLON)
        { break; }

      MATCH! ();
    }

  return last_class;
}

/* Parse an operator name.  Add the `static' flag to *SC if an
   implicitly static operator has been parsed.  Value is a pointer to
   a static buffer holding the constructed operator name string.  */

unsafe fn operator_name (sc: *mut c_int) -> *mut c_char
{
  static mut id_size: size_t = 0;
  static mut id: *mut c_char = std::ptr::null_mut ();
  let mut s: *const c_char;
  let mut len: size_t;

  MATCH! ();

  if LOOKING_AT2! (NEW, DELETE)
    {
      /* `new' and `delete' are implicitly static.  */
      if *sc != SC_FRIEND
        { *sc = SC_STATIC; }

      s = token_string (LA1! ());
      MATCH! ();

      let slen: ptrdiff_t = strlen (s) as ptrdiff_t;
      len = slen as size_t + 10;
      if len > id_size
	{
	  let new_size: size_t = max! (len, 2 * id_size);
	  id = xrealloc (id as *mut c_void, new_size) as *mut c_char;
	  id_size = new_size;
	}
      let mut z: *mut c_char = stpcpy (id, s);

      /* Vector new or delete?  */
      if LOOKING_AT! (b'[' as c_int)
	{
	  z = stpcpy (z, c"[".as_ptr ());
	  MATCH! ();

	  if LOOKING_AT! (b']' as c_int)
	    {
	      strcpy (z, c"]".as_ptr ());
	      MATCH! ();
	    }
	}
    }
  else
    {
      let mut tokens_matched: size_t = 0;

      len = 20;
      if len > id_size
	{
	  let new_size: c_int = max! (len, 2 * id_size) as c_int;
	  id = xrealloc (id as *mut c_void, new_size as size_t) as *mut c_char;
	  id_size = new_size as size_t;
	}
      let mut z: *mut c_char = stpcpy (id, c"operator".as_ptr ());

      /* Beware access declarations of the form "X::f;" Beware of
	 `operator () ()'.  Yet another difficulty is found in
	 GCC 2.95's STL: `operator == __STL_NULL_TMPL_ARGS (...'.  */
      while !(LOOKING_AT! (b'(' as c_int) && tokens_matched != 0)
	     && !LOOKING_AT2! (b';' as c_int, YYEOF)
        {
	  s = token_string (LA1! ());
	  len += strlen (s) + 2;
	  if len > id_size
	    {
	      let idlen: ptrdiff_t = (z as isize - id as isize) as ptrdiff_t;
	      let new_size: size_t = max! (len, 2 * id_size);
	      id = xrealloc (id as *mut c_void, new_size) as *mut c_char;
	      id_size = new_size;
	      z = id.add (idlen as usize);
	    }

	  if *s != b')' as c_char && *s != b']' as c_char
	    { *z = b' ' as c_char; z = z.add (1); }
          z = stpcpy (z, s);
          MATCH! ();

	  /* If this is a simple operator like `+', stop now.  */
	  if !c_isalpha (*s as c_int) && *s != b'(' as c_char && *s != b'[' as c_char
	    { break; }

	  tokens_matched += 1;
        }
    }

  return id;
}

/* This one consumes the last IDENT of a qualified member name like
   `X::Y::z'.  This IDENT is returned in LAST_ID.  Value is the
   symbol structure for the ident.  */

unsafe fn parse_qualified_ident_or_type (last_id: *mut *mut c_char) -> *mut sym
{
  let mut cls: *mut sym = std::ptr::null_mut ();
  let mut id: *mut c_char = std::ptr::null_mut ();
  let mut id_size: size_t = 0;
  let mut enter: c_int = 0;

  while LOOKING_AT! (IDENT)
    {
      let len: c_int = strlen (yytext) as c_int + 1;
      if len as size_t > id_size
	{
	  id = xrealloc (id as *mut c_void, len as size_t) as *mut c_char;
	  id_size = len as size_t;
	}
      strcpy (id, yytext);
      *last_id = id;
      MATCH! ();

      SKIP_MATCHING_IF! (b'<' as c_int);

      if LOOKING_AT! (DCOLON)
	{
	  let mut pcn: *mut sym = std::ptr::null_mut ();
	  let mut pna: *mut link = check_namespace_alias (id);
	  if !pna.is_null ()
	    {
	      loop
		{
		  enter_namespace (FAM_PTR! ((*(*pna).sym).name));
		  enter += 1;
		  pna = (*pna).next;
		  if pna.is_null ()
		    { break; }
		}
	    }
	  else if { pcn = check_namespace (id, current_namespace); !pcn.is_null () }
	    {
	      enter_namespace (FAM_PTR! ((*pcn).name));
	      enter += 1;
	    }
	  else
	    { cls = add_sym (id, cls); }

	  *last_id = std::ptr::null_mut ();
	  free (id as *mut c_void);
	  id = std::ptr::null_mut ();
	  id_size = 0;
	  MATCH! ();
	}
      else
	{ break; }
    }

  while enter != 0
    {
      enter -= 1;
      leave_namespace ();
    }

  return cls;
}

/* This one consumes the last IDENT of a qualified member name like
   `X::Y::z'.  This IDENT is returned in LAST_ID.  Value is the
   symbol structure for the ident.  */

unsafe fn parse_qualified_param_ident_or_type (last_id: *mut *mut c_char)
{
  let mut cls: *mut sym = std::ptr::null_mut ();
  static mut id: *mut c_char = std::ptr::null_mut ();
  static mut id_size: c_int = 0;

  assert! (LOOKING_AT! (IDENT));

  loop
    {
      let len: c_int = strlen (yytext) as c_int + 1;
      if len > id_size
	{
	  id = xrealloc (id as *mut c_void, len as size_t) as *mut c_char;
	  id_size = len;
	}
      strcpy (id, yytext);
      *last_id = id;
      MATCH! ();

      SKIP_MATCHING_IF! (b'<' as c_int);

      if LOOKING_AT! (DCOLON)
	{
	  cls = add_sym (id, cls);
	  *last_id = std::ptr::null_mut ();
	  MATCH! ();
	}
      else
	{ break; }

      if !LOOKING_AT! (IDENT)
	{ break; }
    }
}

/* Parse a class definition.

   CONTAINING is the class containing the class being parsed or null.
   This may also be null if NESTED != 0 if the containing class is
   anonymous.  TAG is the tag of the class (struct, union, class).
   NESTED is non-zero if we are parsing a nested class.

   Current lookahead is the class name.  */

unsafe fn class_definition (containing: *mut sym, class_name: *const c_char, tag: c_int,
			    flags: c_int, nested: c_int)
{
  let mut current: *mut sym;
  let mut base_class: *mut sym;

  /* Set CURRENT to null if no entry has to be made for the class
     parsed.  This is the case for certain command line flag
     settings.  */
  if tag != CLASS && f_structs == 0 || nested != 0 && f_nested_classes == 0
    { current = std::ptr::null_mut (); }
  else
    {
      current = add_sym (class_name, containing);
      (*current).pos = BUFFER_POS! ();
      (*current).regexp = matching_regexp ();
      (*current).filename = filename;
      (*current).flags = flags;
    }

  /* If at ':', base class list follows.  */
  if LOOKING_AT! (b':' as c_int)
    {
      let mut done: c_int = 0;
      MATCH! ();

      while done == 0
        {
          match LA1! ()
            {
            VIRTUAL | PUBLIC | PROTECTED | PRIVATE =>
	      {
		MATCH! ();
	      }

            IDENT =>
	      {
		base_class = parse_classname ();
		if !base_class.is_null () && !current.is_null () && base_class != current
		  { add_link (base_class, current); }
	      }

              /* The `,' between base classes or the end of the base
                 class list.  Add the previously found base class.
                 It's done this way to skip over sequences of
                 `A::B::C' until we reach the end.

                 FIXME: it is now possible to handle `class X : public B::X'
                 because we have enough information.  */
            _ if LA1! () == b',' as c_int =>
	      {
		MATCH! ();
	      }

            _ if LA1! () == b'{' as c_int =>
	      {
		done = 1;
	      }

            _ =>
	      {
		/* A syntax error, possibly due to preprocessor constructs
		   like

		   #ifdef SOMETHING
		   class A : public B
		   #else
		   class A : private B.

		   MATCH until we see something like `;' or `{'.  */
		while !LOOKING_AT3! (b';' as c_int, YYEOF, b'{' as c_int)
		  { MATCH! (); }
		FALLTHROUGH! ();
		done = 1;
	      }
            }
        }
    }

  /* Parse the class body if there is one.  */
  if LOOKING_AT! (b'{' as c_int)
    {
      if tag != CLASS && f_structs == 0
        { skip_matching (); }
      else
        {
          MATCH! ();
          class_body (current, tag);

          if LOOKING_AT! (b'}' as c_int)
            {
              MATCH! ();
              if LOOKING_AT! (b';' as c_int) && nested == 0
                { MATCH! (); }
            }
        }
    }
}

/* Add to class *CLS information for the declaration of variable or
   type *ID.  If *CLS is null, this means a global declaration.  SC is
   the storage class of *ID.  FLAGS is a bit set giving additional
   information about the member (see the F_* defines).  */

unsafe fn add_declarator (cls: *mut *mut sym, id: *mut *mut c_char, flags: c_int, sc: c_int)
{
  if LOOKING_AT2! (b';' as c_int, b',' as c_int)
    {
      /* The end of a member variable or of an access declaration
         `X::f'.  To distinguish between them we have to know whether
         type information has been seen.  */
      if !(*id).is_null ()
        {
          let regexp: *mut c_char = matching_regexp ();
          let pos: c_int = BUFFER_POS! ();

          if !(*cls).is_null ()
	    { add_member_defn (*cls, *id, regexp, pos, 0, 1, SC_UNKNOWN, flags); }
          else
            { add_global_defn (*id, regexp, pos, 0, 1, sc, flags); }
        }

      MATCH! ();
      print_info ();
    }
  else if LOOKING_AT! (b'{' as c_int)
    {
      if sc == SC_TYPE && !(*id).is_null ()
        {
          /* A named enumeration.  */
          let regexp: *mut c_char = matching_regexp ();
          let pos: c_int = BUFFER_POS! ();
          add_global_defn (*id, regexp, pos, 0, 1, sc, flags);
        }

      skip_matching ();
      print_info ();
    }

  free (*id as *mut c_void);
  *id = std::ptr::null_mut ();
  *cls = std::ptr::null_mut ();
}

/* Parse a declaration.  */

unsafe fn declaration (flags: c_int)
{
  let mut flags: c_int = flags;
  let mut id: *mut c_char = std::ptr::null_mut ();
  let mut cls: *mut sym = std::ptr::null_mut ();
  let mut regexp: *mut c_char = std::ptr::null_mut ();
  let mut pos: c_int = 0;
  let mut hash: c_uint = 0;
  let mut is_constructor: c_int;
  let mut sc: c_int = 0;

  while !LOOKING_AT3! (b';' as c_int, b'{' as c_int, YYEOF)
    {
      match LA1! ()
        {
        _ if LA1! () == b'[' as c_int =>
	  {
	    skip_matching ();
	  }

        ENUM | TYPEDEF =>
	  {
	    sc = SC_TYPE;
	    MATCH! ();
	  }

        STATIC =>
	  {
	    sc = SC_STATIC;
	    MATCH! ();
	  }

        INT | CHAR | LONG | UNSIGNED | SIGNED | CONST | DOUBLE | VOID | SHORT
	  | VOLATILE | BOOL | WCHAR =>
	  {
	    MATCH! ();
	  }

        CLASS | STRUCT | UNION =>
	  {
	    /* This is for the case `STARTWRAP class X : ...' or
	       `declare (X, Y)\n class A : ...'.  */
	    if !id.is_null ()
	      {
		free (id as *mut c_void);
		return;
	      }
	    /* C 3237: `FALLTHROUGH;` into the `case '='` arm below.  */
	    FALLTHROUGH! ();
	    skip_initializer ();
	  }

        _ if LA1! () == b'=' as c_int =>
	  {
	    /* Assumed to be the start of an initialization in this
	       context.  */
	    skip_initializer ();
	  }

	_ if LA1! () == b',' as c_int =>
	  {
	    add_declarator (&raw mut cls, &raw mut id, flags, sc);
	  }

        OPERATOR =>
	  {
	    let s: *mut c_char = operator_name (&raw mut sc);
	    id = xrealloc (id as *mut c_void, strlen (s) + 1) as *mut c_char;
	    strcpy (id, s);
	  }

        T_INLINE =>
	  {
	    set_flag (&raw mut flags, F_INLINE);
	    MATCH! ();
	  }

        _ if LA1! () == b'~' as c_int =>
	  {
	    MATCH! ();
	    if LOOKING_AT! (IDENT)
	      {
		id = xrealloc (id as *mut c_void, strlen (yytext) + 2) as *mut c_char;
		*id = b'~' as c_char;
		strcpy (id.add (1), yytext);
		MATCH! ();
	      }
	  }

        IDENT =>
	  {
	    cls = parse_qualified_ident_or_type (&raw mut id);
	  }

        _ if LA1! () == b'(' as c_int =>
	  {
	    /* Most probably the beginning of a parameter list.  */
	    if !cls.is_null ()
	      {
		MATCH! ();

		if !id.is_null () && !cls.is_null ()
		  {
		    is_constructor = streq (id, FAM_PTR! ((*cls).name)) as c_int;
		    if is_constructor == 0
		      { regexp = matching_regexp (); }
		  }
		else
		  { is_constructor = 0; }

		pos = BUFFER_POS! ();
		hash = parm_list (&raw mut flags);

		if is_constructor != 0
		  { regexp = matching_regexp (); }

		if !id.is_null () && !cls.is_null ()
		  { add_member_defn (cls, id, regexp, pos, hash, 0, SC_UNKNOWN, flags); }
	      }
	    else
	      {
		/* This may be a C functions, but also a macro
		   call of the form `declare (A, B)' --- such macros
		   can be found in some class libraries.  */
		MATCH! ();

		if !id.is_null ()
		  {
		    regexp = matching_regexp ();
		    pos = BUFFER_POS! ();
		    hash = parm_list (&raw mut flags);
		    add_global_decl (id, regexp, pos, hash, 0, sc, flags);
		  }

		/* This is for the case that the function really is
		   a macro with no `;' following it.  If a CLASS directly
		   follows, we would miss it otherwise.  */
		if LOOKING_AT3! (CLASS, STRUCT, UNION)
		  { return; }
	      }

	    while !LOOKING_AT3! (b';' as c_int, b'{' as c_int, YYEOF)
	      { MATCH! (); }

	    if cls.is_null () && !id.is_null () && LOOKING_AT! (b'{' as c_int)
	      { add_global_defn (id, regexp, pos, hash, 0, sc, flags); }

	    free (id as *mut c_void);
	    id = std::ptr::null_mut ();
	  }

        _ =>
	  {
	    MATCH! ();
	  }
        }
    }

  add_declarator (&raw mut cls, &raw mut id, flags, sc);
}

/* Parse a list of top-level declarations/definitions.  START_FLAGS
   says in which context we are parsing.  If it is F_EXTERNC, we are
   parsing in an `extern "C"' block.  Value is 1 if EOF is reached, 0
   otherwise.  */

unsafe fn globals (start_flags: c_int) -> c_int
{
  let mut class_tk: c_int;
  let mut class_name: *mut c_char;
  let mut flags: c_int = start_flags;

  loop
    {
      let prev_in: *mut c_char = r#in;

      match LA1! ()
        {
        NAMESPACE =>
	  {
	    MATCH! ();

	    if LOOKING_AT! (IDENT)
	      {
		let namespace_name: *mut c_char = xstrdup (yytext);
		MATCH! ();

		if LOOKING_AT! (b'=' as c_int)
		  {
		    let qna: *mut link = match_qualified_namespace_alias ();
		    if !qna.is_null ()
		      { register_namespace_alias (namespace_name, qna); }

		    if skip_to (b';' as c_int) == b';' as c_int
		      { MATCH! (); }
		  }
		else if LOOKING_AT! (b'{' as c_int)
		  {
		    MATCH! ();
		    enter_namespace (namespace_name);
		    globals (0);
		    leave_namespace ();
		    MATCH_IF! (b'}' as c_int);
		  }

		free (namespace_name as *mut c_void);
	      }
	  }

        EXTERN =>
	  {
	    MATCH! ();
	    if LOOKING_AT! (CSTRING) && *string_start == b'C' as c_char
		&& *string_start.add (1) == b'"' as c_char
	      {
		/* This is `extern "C"'.  */
		MATCH! ();

		if LOOKING_AT! (b'{' as c_int)
		  {
		    MATCH! ();
		    globals (F_EXTERNC);
		    MATCH_IF! (b'}' as c_int);
		  }
		else
		  { set_flag (&raw mut flags, F_EXTERNC); }
	      }
	  }

        TEMPLATE =>
	  {
	    MATCH! ();
	    SKIP_MATCHING_IF! (b'<' as c_int);
	    set_flag (&raw mut flags, F_TEMPLATE);
	  }

        CLASS | STRUCT | UNION =>
	  {
	    class_tk = LA1! ();
	    MATCH! ();
	    class_name = std::ptr::null_mut ();

	    /* More than one ident here to allow for MS-DOS and OS/2
	       specialties like `far', `_Export' etc.  Some C++ libs
	       have constructs like `_OS_DLLIMPORT(_OS_CLIENT)' in front
	       of the class name.  */
	    while !LOOKING_AT4! (YYEOF, b';' as c_int, b':' as c_int, b'{' as c_int)
	      {
		if LOOKING_AT! (IDENT)
                  {
                    if !class_name.is_null ()
                      {
			let size: c_int = strlen (yytext) as c_int;

			if (strlen (class_name) as c_int) < size
			  {
			    class_name = xrealloc (class_name as *mut c_void,
						   size as size_t + 1) as *mut c_char;
			  }

			memcpy (class_name as *mut c_void, yytext as *const c_void,
				size as size_t + 1);
                      }
                    else
                      {
			class_name = xstrdup (yytext);
                      }
                  }

		MATCH! ();
	      }

	    /* Don't add anonymous unions.  */
	    if LOOKING_AT2! (b':' as c_int, b'{' as c_int) && !class_name.is_null ()
	      { class_definition (std::ptr::null_mut (), class_name, class_tk, flags, 0); }
	    else
	      {
		if skip_to (b';' as c_int) == b';' as c_int
		  { MATCH! (); }
	      }

	    free (class_name as *mut c_void);
	    flags = start_flags;
	  }

        YYEOF =>
	  { return 1; }

        _ if LA1! () == b'}' as c_int =>
	  { return 0; }

        _ =>
	  {
	    declaration (flags);
	    flags = start_flags;
	  }
        }

      if prev_in == r#in
        { yyerror (c"parse error".as_ptr (), std::ptr::null ()); }
    }
}

/* Parse the current input file.  */

unsafe fn yyparse ()
{
  while globals (0) == 0
    { MATCH_IF! (b'}' as c_int); }
}

/***********************************************************************
			     Main Program
 ***********************************************************************/

/* Add the list of paths PATH_LIST to the current search path for
   input files.  */

unsafe fn add_search_path (path_list: *mut c_char)
{
  let mut path_list: *mut c_char = path_list;
  while *path_list != 0
    {
      let start: *mut c_char = path_list;
      let mut p: *mut search_path;

      while *path_list != 0 && *path_list != SEPCHAR
        { path_list = path_list.add (1); }

      p = xmalloc (std::mem::size_of::<search_path> ()) as *mut search_path;
      (*p).path = xmalloc ((path_list as isize - start as isize) as size_t + 1) as *mut c_char;
      memcpy ((*p).path as *mut c_void, start as *const c_void,
	      (path_list as isize - start as isize) as size_t);
      *(*p).path.add ((path_list as isize - start as isize) as usize) = 0;
      (*p).next = std::ptr::null_mut ();

      if !search_path_tail.is_null ()
        {
          (*search_path_tail).next = p;
          search_path_tail = p;
        }
      else
        {
	  search_path = p;
	  search_path_tail = p;
	}

      while *path_list == SEPCHAR
        { path_list = path_list.add (1); }
    }
}

/* Open FILE and return a file handle for it, or -1 if FILE cannot be
   opened.  Try to find FILE in search_path first, then try the
   unchanged file name.  */

unsafe fn open_file (file: *mut c_char) -> *mut FILE
{
  let mut fp: *mut FILE = std::ptr::null_mut ();
  static mut buffer: *mut c_char = std::ptr::null_mut ();
  static mut buffer_size: c_int = 0;
  let mut path: *mut search_path;
  let flen: c_int = strlen (file) as c_int + 1;	/* +1 for the slash */

  filename = xstrdup (file);

  path = search_path;
  while !path.is_null () && fp.is_null ()
    {
      let len: c_int = strlen ((*path).path) as c_int + flen;

      if len + 1 >= buffer_size
	{
	  buffer_size = max! (len + 1, 2 * buffer_size);
	  buffer = xrealloc (buffer as *mut c_void, buffer_size as size_t) as *mut c_char;
	}

      let mut z: *mut c_char = stpcpy (buffer, (*path).path);
      *z = b'/' as c_char;
      z = z.add (1);
      strcpy (z, file);
      fp = fopen (buffer, c"r".as_ptr ());
      path = (*path).next;
    }

  /* Try the original file name.  */
  if fp.is_null ()
     { fp = fopen (file, c"r".as_ptr ()); }

  if fp.is_null ()
    { yyerror (c"cannot open".as_ptr (), std::ptr::null ()); }

  return fp;
}

/* Display usage information and exit program.  */

static mut usage_message: [*const c_char; 2] =
  [
    c"Usage: ebrowse [options] {files}\n\n  -a, --append                  append output to existing file\n  -f, --files=FILES             read input file names from FILE\n  -I, --search-path=LIST        set search path for input files\n  -m, --min-regexp-length=N     set minimum regexp length to N\n  -M, --max-regexp-length=N     set maximum regexp length to N\n".as_ptr (),
    c"  -n, --no-nested-classes       exclude nested classes\n  -o, --output-file=FILE        set output file name to FILE\n  -p, --position-info           print info about position in file\n  -s, --no-structs-or-unions    don't record structs or unions\n  -v, --verbose                 be verbose\n  -V, --very-verbose            be very verbose\n  -x, --no-regexps\t\tdon't record regular expressions\n      --help                    display this help\n      --version\t\t\tdisplay version info\n\n".as_ptr (),
  ];

/* C 3616: `static _Noreturn void usage (int error)` - the port's `-> !`.  */
unsafe fn usage (error: c_int) -> !
{
  let mut i: c_int;
  i = 0;
  while i < countof! (usage_message) as c_int
    { fputs (usage_message[i as usize], stdout); i += 1; }
  exit (if error != 0 { EXIT_FAILURE } else { EXIT_SUCCESS });
}

/* Display version and copyright info.  */

/* C 3626: `static _Noreturn void version (void)` - the port's `-> !`.  */
unsafe fn version () -> !
{
  fputs (c"ebrowse 32.0.50\nCopyright (C) 2026 Free Software Foundation, Inc.\nThis program is distributed under the same terms as Emacs.\n".as_ptr (),
	 stdout);
  exit (EXIT_SUCCESS);
}

/* Parse one input file FILE, adding classes and members to the symbol
   table.  */

unsafe fn process_file (file: *mut c_char)
{
  let mut fp: *mut FILE;

  fp = open_file (file);
  if !fp.is_null ()
    {
      let mut nread: size_t;
      let mut nbytes: size_t;

      /* Give a progress indication if needed.  */
      if f_very_verbose != 0
        {
          puts (filename);
          fflush (stdout);
        }
      else if f_verbose != 0
        {
          putchar (b'.' as c_int);
          fflush (stdout);
        }

      /* Read file to inbuffer.  */
      nread = 0;
      loop
	{
	  if nread + READ_CHUNK_SIZE as size_t >= inbuffer_size
	    {
	      inbuffer_size = nread + READ_CHUNK_SIZE as size_t + 1;
	      inbuffer = xrealloc (inbuffer as *mut c_void, inbuffer_size) as *mut c_char;
	    }

	  nbytes = fread (inbuffer.add (nread) as *mut c_void, 1, READ_CHUNK_SIZE as size_t, fp);
	  if nbytes == 0
	    { break; }
	  nread += nbytes;
	}
      *inbuffer.add (nread) = 0;

      /* Reinitialize scanner and parser for the new input file.  */
      re_init_scanner ();
      re_init_parser ();

      /* Parse it and close the file.  */
      yyparse ();
      fclose (fp);
    }
}

/* Read a line from stream FP and return a pointer to a static buffer
   containing its contents without the terminating newline.  Value
   is null when EOF is reached.  */

unsafe fn read_line (fp: *mut FILE) -> *mut c_char
{
  static mut buffer: *mut c_char = std::ptr::null_mut ();
  static mut buffer_size: c_int = 0;
  let mut i: c_int = 0;
  let mut c: c_int;

  c = getc (fp);
  while c != EOF && c != b'\n' as c_int
    {
      if i >= buffer_size
	{
	  buffer_size = max! (100, buffer_size * 2);
	  buffer = xrealloc (buffer as *mut c_void, buffer_size as size_t) as *mut c_char;
	}

      *buffer.add (i as usize) = c as c_char;
      i += 1;
      c = getc (fp);
    }

  if c == EOF && i == 0
    { return std::ptr::null_mut (); }

  if i == buffer_size
    {
      buffer_size = max! (100, buffer_size * 2);
      buffer = xrealloc (buffer as *mut c_void, buffer_size as size_t) as *mut c_char;
    }

  *buffer.add (i as usize) = 0;
  if i > 0 && *buffer.add (i as usize - 1) == b'\r' as c_char
    { *buffer.add (i as usize - 1) = 0; }
  return buffer;
}

/* Main entry point.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut i: c_int;
  let mut any_inputfiles: c_int = 0;
  static mut out_filename: *const c_char = DEFAULT_OUTFILE.as_ptr ();
  static mut input_filenames: *mut *mut c_char = std::ptr::null_mut ();
  static mut input_filenames_size: c_int = 0;
  static mut n_input_files: c_int = 0;

  filename = c"command line".as_ptr ();
  yyout = stdout;

  loop
    {
      i = getopt_long (argc, argv, c"af:I:m:M:no:p:svVx".as_ptr (),
		       &raw const options as *const option, std::ptr::null_mut ());
      if i == EOF
	{ break; }
      match i
        {
	  /* Experimental.  */
	_ if i == b'p' as c_int =>
	  {
	    info_position = atoi (optarg);
	  }

        _ if i == b'n' as c_int =>
	  {
	    f_nested_classes = 0;
	  }

        _ if i == b'x' as c_int =>
	  {
	    f_regexps = 0;
	  }

          /* Add the name of a file containing more input files.  */
        _ if i == b'f' as c_int =>
	  {
	    if n_input_files == input_filenames_size
	      {
		input_filenames_size = max! (10, 2 * input_filenames_size);
		input_filenames = xrealloc (input_filenames as *mut c_void,
					    (input_filenames_size as size_t
					     * std::mem::size_of::<*mut c_char> ())) as *mut *mut c_char;
	      }
	    *input_filenames.add (n_input_files as usize) = xstrdup (optarg);
	    n_input_files += 1;
	  }

          /* Append new output to output file instead of truncating it.  */
        _ if i == b'a' as c_int =>
	  {
	    f_append = 1;
	  }

          /* Include structs in the output */
        _ if i == b's' as c_int =>
	  {
	    f_structs = 0;
	  }

          /* Be verbose (give a progress indication).  */
        _ if i == b'v' as c_int =>
	  {
	    f_verbose = 1;
	  }

          /* Be very verbose (print file names as they are processed).  */
        _ if i == b'V' as c_int =>
	  {
	    f_verbose = 1;
	    f_very_verbose = 1;
	  }

          /* Change the name of the output file.  */
        _ if i == b'o' as c_int =>
	  {
	    out_filename = optarg;
	  }

          /* Set minimum length for regular expression strings
             when recorded in the output file.  */
        _ if i == b'm' as c_int =>
	  {
	    min_regexp = atoi (optarg);
	  }

          /* Set maximum length for regular expression strings
             when recorded in the output file.  */
        _ if i == b'M' as c_int =>
	  {
	    max_regexp = atoi (optarg);
	  }

          /* Add to search path.  */
        _ if i == b'I' as c_int =>
	  {
	    add_search_path (optarg);
	  }

          /* Display help */
        -2 =>
	  {
	    usage (0);
	  }

	-3 =>
	  {
	    version ();
	  }

	_ =>
	  { }
        }
    }

  /* Call init_scanner after command line flags have been processed to be
     able to add keywords depending on command line (not yet
     implemented).  */
  init_scanner ();
  init_sym ();

  /* Open output file */
  if *out_filename != 0
    {
      if f_append != 0
	{
	  /* Check that the file to append to exists, and is not
	     empty.  More specifically, it should be a valid file
	     produced by a previous run of ebrowse, but that's too
	     difficult to check.  */
	  let mut fp: *mut FILE;
	  let mut rc: c_int;

	  fp = fopen (out_filename, c"r".as_ptr ());
	  if fp.is_null ()
	    {
	      yyerror (c"file '%s' must exist for --append".as_ptr (), out_filename);
	      exit (EXIT_FAILURE);
	    }

	  rc = fseek (fp, 0, SEEK_END);
	  if rc == -1
	    {
	      yyerror (c"error seeking in file '%s'".as_ptr (), out_filename);
	      exit (EXIT_FAILURE);
	    }

	  rc = ftell (fp) as c_int;
	  if rc == -1
	    {
	      yyerror (c"error getting size of file '%s'".as_ptr (), out_filename);
	      exit (EXIT_FAILURE);
	    }

	  else if rc == 0
	    {
	      yyerror (c"file '%s' is empty".as_ptr (), out_filename);
	      /* It may be ok to use an empty file for appending.
		 exit (EXIT_FAILURE); */
	    }

	  fclose (fp);
	}

      yyout = fopen (out_filename, if f_append != 0 { c"a".as_ptr () } else { c"w".as_ptr () });
      if yyout.is_null ()
	{
	  yyerror (c"cannot open output file '%s'".as_ptr (), out_filename);
	  exit (EXIT_FAILURE);
	}
    }

  /* Process input files specified on the command line.  */
  while optind < argc
    {
      process_file (*argv.add (optind as usize));
      optind += 1;
      any_inputfiles = 1;
    }

  /* Process files given on stdin if no files specified.  */
  if any_inputfiles == 0 && n_input_files == 0
    {
      let mut file: *mut c_char;
      file = read_line (stdin);
      while !file.is_null ()
	{
	  process_file (file);
	  file = read_line (stdin);
	}
    }
  else
    {
      /* Process files from `--files=FILE'.  Every line in FILE names
	 one input file to process.  */
      i = 0;
      while i < n_input_files
        {
          let fp: *mut FILE = fopen (*input_filenames.add (i as usize), c"r".as_ptr ());

          if fp.is_null ()
            { yyerror (c"cannot open input file '%s'".as_ptr (), *input_filenames.add (i as usize)); }
          else
            {
	      let mut file: *mut c_char;
	      file = read_line (fp);
	      while !file.is_null ()
		{
		  process_file (file);
		  file = read_line (fp);
		}
              fclose (fp);
            }
          i += 1;
        }
    }

  /* Write output file.  */
  dump_roots (yyout);

  /* Close output file.  */
  if yyout != stdout
    { fclose (yyout); }

  return EXIT_SUCCESS;
}

/* ebrowse.c ends here */
