/* Spike 1 (C side): print the sizes and offsets that the Rust mirror must match.
   Compile with the frozen configure output; see spikes/run.sh.  */
#include <config.h>
#include <stdio.h>
#include <stddef.h>
#include "lisp.h"
#include "buffer.h"
#include "window.h"
#include "bignum.h"

#define SZ(T)      printf("sizeof %-34s %zu\n", #T, sizeof (T))
#define AL(T)      printf("alignof %-33s %zu\n", #T, _Alignof (T))
#define OFF(T, F)  printf("offsetof %-32s %zu\n", #T "." #F, offsetof (T, F))

int
main (void)
{
  printf ("spike layout; system %s; EMACS_INT %d bits\n", SYSTEM_TYPE,
          (int) (8 * sizeof (EMACS_INT)));

  SZ (Lisp_Object);            AL (Lisp_Object);
  SZ (union vectorlike_header); AL (union vectorlike_header);
  SZ (struct Lisp_String);     AL (struct Lisp_String);
  SZ (struct Lisp_Cons);       AL (struct Lisp_Cons);
  SZ (struct Lisp_Vector);     AL (struct Lisp_Vector);
  SZ (struct Lisp_Symbol);     AL (struct Lisp_Symbol);
  SZ (struct Lisp_Subr);       AL (struct Lisp_Subr);
  SZ (union Aligned_Lisp_Subr);
  SZ (struct Lisp_Marker);
  SZ (struct Lisp_Float);
  SZ (struct handler);
  SZ (struct buffer);
  SZ (struct window);
  SZ (jmp_buf); SZ (sigjmp_buf);

  /* bignum.h's own layout and the gmp.h subset its mirror carries (P-49) */
  SZ (__mpz_struct);           AL (__mpz_struct);
  OFF (__mpz_struct, _mp_alloc);
  OFF (__mpz_struct, _mp_size);
  OFF (__mpz_struct, _mp_d);
  SZ (mpz_t);
  SZ (mpz);
  SZ (struct Lisp_Bignum);     AL (struct Lisp_Bignum);

  /* every field of the mirrored structs (generated) */
#include "layout_fields.h"

  OFF (union vectorlike_header, size);
  OFF (struct Lisp_String, u.s.size);
  OFF (struct Lisp_String, u.s.size_byte);
  OFF (struct Lisp_String, u.s.intervals);
  OFF (struct Lisp_String, u.s.data);
  OFF (struct Lisp_String, u.next);
  OFF (struct Lisp_Cons, u.s.car);
  OFF (struct Lisp_Cons, u.s.u.cdr);
  OFF (struct Lisp_Cons, u.s.u.chain);
  OFF (struct Lisp_Vector, header);
  OFF (struct Lisp_Vector, contents);
  OFF (struct Lisp_Symbol, u.s.name);
  OFF (struct Lisp_Symbol, u.s.val);
  OFF (struct Lisp_Symbol, u.s.function);
  OFF (struct Lisp_Symbol, u.s.plist);
  OFF (struct Lisp_Symbol, u.s.next);
  OFF (struct Lisp_Subr, function);
  OFF (struct Lisp_Subr, min_args);
  OFF (struct Lisp_Subr, max_args);
  OFF (struct Lisp_Subr, symbol_name);
  OFF (struct Lisp_Subr, intspec);
  OFF (struct Lisp_Subr, command_modes);
  OFF (struct Lisp_Subr, doc);
  OFF (struct Lisp_Marker, buffer);
  OFF (struct Lisp_Marker, bytepos);
  OFF (struct Lisp_Marker, charpos);
  OFF (struct handler, jmp);
  OFF (struct handler, next);
  OFF (struct handler, nextfree);
  OFF (struct handler, tag_or_ch);
  OFF (struct handler, val);
  SZ (struct buffer_text);
  OFF (struct buffer, name_);
  OFF (struct buffer, last_name_);
  OFF (struct buffer, filename_);
  OFF (struct buffer, directory_);
  OFF (struct buffer, pt_marker_);
  OFF (struct buffer, begv_marker_);
  OFF (struct buffer, zv_marker_);
  OFF (struct buffer_text, beg);
  OFF (struct buffer_text, gpt);
  OFF (struct buffer_text, z);
  OFF (struct buffer_text, gap_size);
  return 0;
}
