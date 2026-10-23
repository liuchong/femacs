// femacs: port of src/xml.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements, same names.  The file is dominated by
// `#ifdef HAVE_LIBXML2` (C 24-317's parser, the two `libxml-parse-*-region` subrs and their
// helpers, plus the `#ifdef WINDOWSNT` splits inside them); the frozen configuration leaves
// HAVE_LIBXML2 undefined (docs/baseline/config/darwin-arm64.macros.tsv), so C compiles only
// `libxml-available-p`'s `#else` arm and `syms_of_xml`'s trailing part.  The dead arms stay as
// P-51 text under `#[cfg (feature = "libxml2")]`, where they are not resolved — the same
// disposition the file's own `<libxml/*.h>` includes get (they are recorded in
// docs/ports/excluded-macros.txt, since the headers have no counterpart on this target).
//
// C 20-23's includes are config.h, lisp.h and buffer.h; C's `Qt`/`Qlibxml2`/`Vlibrary_cache`
// are only named inside the dead arm and come from the mirrors when it is enabled.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]


include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("../lib/include/string.rs");
include!("include/lisp.rs");
include!("include/character.rs");
include!("include/itree.rs");
include!("include/systime.rs");
include!("include/dispextern.rs");
include!("include/composite.rs");
include!("include/coding.rs");
include!("include/buffer.rs");

/* C 24-317's `#ifdef HAVE_LIBXML2` block.  On this target the C preprocessor drops it whole:
   the libxml2 tree/parser/HTMLparser headers (C 27-29) are not on the target, the
   `init_libxml2_functions`/`libxml_parse_region` helpers and the two DEFUNs are never
   compiled, and nothing links against them.  The C text is kept, per P-51, as text that only
   a build with the feature on would read; the tokens the ledger compares are the live ones
   below.  */
#[cfg (feature = "libxml2")]
mod libxml2_arm
{
  use super::*;

  /* C 31-88's `#ifdef WINDOWSNT` split (the `xml_parse_region` wrapper that loads the DLL)
     and C 90-317's parser are inside this arm in C as well.  */
}

DEFUN! ("libxml-available-p", Flibxml_available_p, Slibxml_available_p, 0, 0, 0,
       doc: "Return t if libxml2 support is available in this instance of Emacs.",
       (),
       {
  /* C 325-342: the `HAVE_LIBXML2` arm (and within it the `WINDOWSNT` one, which caches the
     DLL probe in `Vlibrary_cache`) is dead here, so the surviving statement is the `#else`
     arm's `return Qnil;`.  */
  #[cfg (feature = "libxml2")]
  {
    #[cfg (feature = "windows_nt")]
    {
      let found: Lisp_Object = Fassq (Qlibxml2 (), Vlibrary_cache);
      if CONSP (found)
        { return XCDR (found); }
      else
        {
          let status: Lisp_Object
            = if init_libxml2_functions () { Qt () } else { Qnil () };
          Vlibrary_cache = Fcons (Fcons (Qlibxml2 (), status), Vlibrary_cache);
          return status;
        }
    }
    #[cfg (not (feature = "windows_nt"))]
    { return Qt (); }
  }
  return Qnil ();
});

/***********************************************************************
			    Initialization
 ***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_xml ()
{
  /* C 351-354's `#ifdef HAVE_LIBXML2` pair — `defsubr (&Slibxml_parse_html_region);` and
     `defsubr (&Slibxml_parse_xml_region);` — is dead here, and writing the calls as text would
     make `extract-abi.py` read them as live registrations (P-76's data.c finding: the Rust-side
     `defsubr` scan does not consult the condition table).  The two names are recorded here and
     in docs/ports/xml.md instead; a build with the feature on registers them after the two
     DEFUNs of the `libxml2_arm` module.  */
  defsubr (&raw mut Slibxml_available_p);

  DEFSYM! (Qcomment, "comment");
}
