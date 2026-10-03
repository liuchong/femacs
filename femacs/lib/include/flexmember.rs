/* flexible array member support

   Copyright 2006-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with this program.  If not, see
   <https://www.gnu.org/licenses/>.

   Written by Paul Eggert.  */

// femacs: port of lib/flexmember.h @ fd7afced403 (gnulib; GNU Emacs 32.0.50); PORTING.md 0.1.
//
// `tools/check-cond-eval.py lib/flexmember.h` is VERIFIED (2 conditions), so this mirror is
// the frozen configuration's live text: C 36's `#if !defined __STDC_VERSION__ || … ||
// defined _Alignof` is false for the target's C11+ clang (its `_Alignof` is a keyword, not a
// macro), which makes C 39's `_Alignof (type)` the live `FLEXALIGNOF` arm.  C 24-26's
// `#if !_GL_CONFIG_H_INCLUDED / #error` is a guard with no Rust counterpart (the unit's
// include order does the same job), and C 28's `<stddef.h>` is the target's own header
// (the port spells `offsetof`/`sizeof` with `offset_of!`/`size_of`).

/* flexmember.h 30-40: a nonzero multiple of TYPE's alignment.  */
/* The metavariable is spelled `$type_` rather than `$t`: C's parameter is named `type`,
   which Rust reserves, and the ledger's token comparison strips a trailing `_`, so `$type_`
   is that same token at every use (the token-aliases `type` family counts it too).  */
macro_rules! FLEXALIGNOF {
  ($type_:ty) => { std::mem::align_of::<$type_> () };
}

/* flexmember.h 42-66: a properly aligned upper bound on the size of a struct TYPE whose
   flexible array member MEMBER is followed by N bytes.  */
macro_rules! FLEXSIZEOF {
  ($type_:ty, $member:ident, $n:expr) => {
    ((std::mem::offset_of! ($type_, $member) + FLEXALIGNOF! ($type_) - 1 + ($n))
     & ! (FLEXALIGNOF! ($type_) - 1))
  };
}

/* flexmember.h 68-77: the same bound for N *elements* of MEMBER.
   C's `sizeof (((type *) 0)->member[0])` does not evaluate its operand; Rust has no
   `typeof`, so the port spells the same place expression and reads only its size.  No
   ported unit expands this macro (the one user of the header, time_rz.c, uses FLEXSIZEOF);
   it is kept so the header's text and the ledger are complete.  */
macro_rules! FLEXNSIZEOF {
  ($type_:ty, $member:ident, $n:expr) => {
    FLEXSIZEOF! ($type_, $member,
                 ($n) * std::mem::size_of_val (&(*(0 as *const $type_)).$member [0]))
  };
}
