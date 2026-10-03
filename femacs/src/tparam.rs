// femacs: port of src/tparam.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// The `#include` list mirrors tparam.c 18-21 - config.h, lisp.h (for xmalloc), tparam.h -
// with the mirrors those pull in.  The file's only conditional is its trailing
// `#ifdef DEBUG` main(), which the frozen configuration does not compile (DEBUG is
// undefined); the port keeps C's text under the matching cfg so the dead-arms build still
// compiles it (P-51).
//
// `tparam1`'s three labels and four FALLTHROUGHs are the interesting part.  PORTING.md 2.2
// and the `variable-binding-locus` decision require the *shared* statements to appear once
// and the arms to reach them, so the two chains C spells as fallthrough (`%d`/`%3`/`%2`
// into `onedigit`, and `%C`/`%+`/`%.` into `%f`) are one copy each, entered through the
// small stage variables the ledger records.  Duplicating the tails instead would have made
// the token multiset differ from C's and broken the "same statements in the same order"
// rule - the first draft did exactly that and was thrown away.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("../lib/include/stdlib.rs");
include!("include/lisp.rs");
include!("include/tparam.rs");

unsafe extern "C" {
  // <string.h> on the frozen target (HAVE_STPCPY is 1, so this is the system stpcpy).
  pub fn stpcpy (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  pub fn memcpy (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
  // <stdlib.h>: the port's stdlib mirror carries the allocator entry points the
  // ported files call; atoi is only used by the dead DEBUG main below.
  pub fn atoi (s: *const c_char) -> c_int;
}

/* Assuming STRING is the value of a termcap string entry
   containing `%' constructs to expand parameters,
   merge in parameter values and store result in block OUTSTRING points to.
   LEN is the length of OUTSTRING.  If more space is needed,
   a block is allocated with `malloc'.

   The value returned is the address of the resulting string.
   This may be OUTSTRING or may be the address of a block got with `malloc'.
   In the latter case, the caller must free the block.

   The fourth and following args to tparam serve as the parameter values.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tparam (string: *const c_char, outstring: *mut c_char, len: c_int,
                                 arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int)
  -> *mut c_char
{
  let mut arg: [c_int; 4] = [0; 4];

  arg[0] = arg0;
  arg[1] = arg1;
  arg[2] = arg2;
  arg[3] = arg3;
  return tparam1 (string, outstring, len, std::ptr::null_mut (), std::ptr::null_mut (),
                  arg.as_mut_ptr ());
}

/* tparam.h 41-42: `extern char *BC; extern char *UP;` - the two objects are *defined* in
   terminfo.c (femacs/src/terminfo.rs) and this translation unit only declares them.  The port
   had defined them here before terminfo.c was ported; corrected then.  */
unsafe extern "C" {
  pub static mut BC: *mut c_char;
  pub static mut UP: *mut c_char;
}

// tparam.c 53: `static char tgoto_buf[50];` - a bss array, zero in C and here.
static mut tgoto_buf: [c_char; 50] = [0; 50];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tgoto (cm: *const c_char, hpos: c_int, vpos: c_int)
  -> *mut c_char
{
  let mut args: [c_int; 2] = [0; 2];
  if cm.is_null ()
    { return std::ptr::null_mut (); }
  args[0] = vpos;
  args[1] = hpos;
  return tparam1 (cm, tgoto_buf.as_mut_ptr (), 50, UP, BC, args.as_mut_ptr ());
}

unsafe fn tparam1 (string: *const c_char, mut outstring: *mut c_char, len: c_int,
                   up: *mut c_char, left: *mut c_char, mut argp: *mut c_int)
  -> *mut c_char
{
  let mut c: c_int;
  let mut p: *const c_char = string;
  let mut op: *mut c_char = outstring;
  let mut outend: *mut c_char;
  let mut new: *mut c_char = std::ptr::null_mut ();
  let mut outlen: ptrdiff_t = 0;

  let mut tem: c_int = 0;
  let mut old_argp: *mut c_int = argp;             /* can move */
  let fixed_argp: *mut c_int = argp;               /* never moves */
  let mut explicit_param_p: bool = false;          /* set by %p */
  let mut doleft: ptrdiff_t = 0;
  let mut doup: ptrdiff_t = 0;
  let mut append_len: ptrdiff_t = 0;

  outend = outstring.offset (len as isize);

  while true
    {
      /* If the buffer might be too short, make it bigger.  */
      while outend.offset_from (op) - append_len <= 5
	{
	  let offset: ptrdiff_t = op.offset_from (outstring);

	  if outlen == 0
	    {
	      outlen = len as ptrdiff_t + 40;
	      new = xmalloc (outlen as usize) as *mut c_char;
	      memcpy (new as *mut c_void, outstring as *const c_void, offset as usize);
	    }
	  else
	    {
              new = xpalloc (outstring as *mut c_void, &raw mut outlen, 1, -1, 1)
                as *mut c_char;
	    }

	  op = new.offset (offset);
	  outend = new.offset (outlen);
	  outstring = new;
	}
      c = *p as c_int;
      p = p.offset (1);
      if c == 0
	{ break; }
      /* C: `ordinary:` is the tail statement, and `case '%'` jumps to it; the port reaches
         it through `ordinary` because a Rust `match` arm cannot jump into a statement.  */
      let mut ordinary: bool = c != b'%' as c_int;
      if c == b'%' as c_int
	{
	  c = *p as c_int;
	  p = p.offset (1);
	  if explicit_param_p
	    { explicit_param_p = false; }
	  else
	    { tem = *argp; }
	  /* C's `%d`/`%3`/`%2` chain: `goto onedigit` and `goto twodigit` enter it below
	     the `%3` arm, and both arms fall through into the next.  `digits` is the entry
	     stage (`1` is `%3`, `2` is the `twodigit:` label, `3` is `onedigit:`), so the
	     chain's statements appear once, as PORTING.md 2.2 requires.  */
	  let mut digits: c_int = 0;
	  /* C's `%C`/`%+`/`%.` chain and its fallthrough into `case 'f'`: `plus` selects the
	     `%+` step (`2` when `%C` has already emitted the quotient), `dot` the `%.` step and
	     `drop_arg` the `%f` step the `%.` arm falls into.  */
	  let mut plus: c_int = 0;
	  let mut dot: bool = false;
	  let mut drop_arg: bool = false;
	  match c as u8
	    {
	    b'd' =>		/* %d means output in decimal.  */
	      {
		if tem < 10
		  { digits = 3; }
		else if tem < 100
		  { digits = 2; }
		else
		  { digits = 1; }
	      }
	    b'3' =>		/* %3 means output in decimal, 3 digits.  */
	      { digits = 1; }

	    b'p' =>             /* %pN means use param N for next subst.  */
	      {
		tem = *fixed_argp.offset ((*p as c_int - b'1' as c_int) as isize);
		p = p.offset (1);
		explicit_param_p = true;
	      }

	    b'C' =>
	      {
		/* For c-100: print quotient of value by 96, if nonzero,
		   then do like %+.  */
		if tem >= 96
		  {
		    *op = (tem / 96) as c_char;
		    op = op.offset (1);
		    tem %= 96;
		  }
		plus = 2;
	      }

	    b'+' =>		/* %+x means add character code of char x.  */
	      { plus = 2; }

	    b'.' =>		/* %. means output as character.  */
	      { dot = true; }

	    b'f' =>		/* %f means discard next arg.  */
	      { drop_arg = true; }

	    b'b' =>		/* %b means back up one arg (and reuse it).  */
	      { argp = argp.offset (-1); }

	    b'r' =>		/* %r means interchange following two args.  */
	      {
		*argp.offset (0) = *argp.offset (1);
		*argp.offset (1) = tem;
		old_argp = old_argp.offset (1);
	      }

	    b'>' =>		/* %>xy means if arg is > char code of x, */
	      {
		if *argp.offset (0) > *p as c_int /* then add char code of y to the arg, */
		  { *argp.offset (0) += *p.offset (1) as c_int; } /* and in any case don't output.  */
		p = p.offset (2);		/* Leave the arg to be output later.  */
	      }

	    b'a' =>		/* %a means arithmetic.  */
	      {
		/* Next character says what operation.
		   Add or subtract either a constant or some other arg.  */
		/* First following character is + to add or - to subtract
		   or = to assign.  */
		/* Next following char is 'p' and an arg spec
		   (0100 plus position of that arg relative to this one)
		   or 'c' and a constant stored in a character.  */
		tem = *p.offset (2) as c_int & 0177;
		if *p.offset (1) as u8 == b'p'
		  { tem = *argp.offset ((tem - 0100) as isize); }
		if *p as u8 == b'-'
		  { *argp.offset (0) -= tem; }
		else if *p as u8 == b'+'
		  { *argp.offset (0) += tem; }
		else if *p as u8 == b'*'
		  { *argp.offset (0) *= tem; }
		else if *p as u8 == b'/'
		  { *argp.offset (0) /= tem; }
		else
		  { *argp.offset (0) = tem; }

		p = p.offset (3);
	      }

	    b'i' =>		/* %i means add one to arg, */
	      {
		*argp.offset (0) += 1;	/* and leave it to be output later.  */
		*argp.offset (1) += 1;	/* Increment the following arg, too!  */
	      }

	    b'%' =>		/* %% means output %; no arg.  */
	      { ordinary = true; }

	    b'n' =>		/* %n means xor each of next two args with 140.  */
	      {
		*argp.offset (0) ^= 0140;
		*argp.offset (1) ^= 0140;
	      }

	    b'm' =>		/* %m means xor each of next two args with 177.  */
	      {
		*argp.offset (0) ^= 0177;
		*argp.offset (1) ^= 0177;
	      }

	    b'B' =>		/* %B means express arg as BCD char code.  */
	      { *argp.offset (0) += 6 * (tem / 10); }

	    b'D' =>		/* %D means weird Delta Data transformation.  */
	      { *argp.offset (0) -= 2 * (tem % 16); }

	    _ => { emacs_abort (); }
	    }

	  /* C's `%3` arm: `if (tem > 999) { *op++ = tem / 1000 + '0'; tem %= 1000; }` and then
	     `*op++ = tem / 100 + '0';` which `%d` falls through into as well.  */
	  if digits != 0
	    {
	      if digits <= 1
		{
		  if tem > 999
		    {
		      *op = (tem / 1000 + b'0' as c_int) as c_char;
		      op = op.offset (1);
		      tem %= 1000;
		    }
		  *op = (tem / 100 + b'0' as c_int) as c_char;
		  op = op.offset (1);
		}
	      /* C: `twodigit: tem %= 100; *op++ = tem / 10 + '0';` */
	      if digits <= 2
		{
		  tem %= 100;
		  *op = (tem / 10 + b'0' as c_int) as c_char;
		  op = op.offset (1);
		}
	      /* C: `onedigit: *op++ = tem % 10 + '0'; argp++; break;` */
	      *op = (tem % 10 + b'0' as c_int) as c_char;
	      op = op.offset (1);
	      argp = argp.offset (1);
	    }
	  /* C's `case '+': tem += *p++;` which `%C` falls through into.  */
	  if plus == 2
	    {
	      tem += *p as c_int;
	      p = p.offset (1);
	      dot = true;
	    }
	  /* C's `case '.': …`, which `%C`, `%+` reach and which falls through into `%f`.  */
	  if dot
	    {
	      if ! left.is_null ()
		{
		  /* If want to forbid output of 0 and \n and \t,
		     and this is one of them, increment it.  */
		  while tem == 0 || tem == b'\n' as c_int || tem == b'\t' as c_int
		    {
		      let append_len_incr: ptrdiff_t;
		      tem += 1;
		      if argp == old_argp
			{ doup += 1; append_len_incr = strlen (up) as ptrdiff_t; }
		      else
			{ doleft += 1; append_len_incr = strlen (left) as ptrdiff_t; }
		      // C's `ckd_add` (P-15): the wrapped sum plus the overflow flag.
		      let (sum, overflow) = append_len.overflowing_add (append_len_incr);
		      append_len = sum;
		      if overflow
			{ memory_full_up (); }
		    }
		}
	      *op = (if tem != 0 { tem } else { 0200 }) as c_char;
	      op = op.offset (1);
	      drop_arg = true;
	    }
	  /* C's `case 'f': argp++; break;`  */
	  if drop_arg
	    { argp = argp.offset (1); }
	}

      if ordinary
	{
	  *op = c as c_char;
	  op = op.offset (1);
	}
    }
  *op = 0;
  while { doup -= 1; doup >= 0 }
    { op = stpcpy (op, up); }
  while { doleft -= 1; doleft >= 0 }
    { op = stpcpy (op, left); }
  return outstring;
}

#[cfg(feature = "debug")]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut buf: [c_char; 50] = [0; 50];
  let mut args: [c_int; 3] = [0; 3];
  args[0] = atoi (*argv.offset (2));
  args[1] = atoi (*argv.offset (3));
  args[2] = atoi (*argv.offset (4));
  tparam1 (*argv.offset (1), buf.as_mut_ptr (), 50,
           c"LEFT".as_ptr () as *mut c_char, c"UP".as_ptr () as *mut c_char,
           args.as_mut_ptr ());
  printf (c"%s\n".as_ptr (), buf.as_ptr ());
  return 0;
}
