// femacs: port of src/keymap.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// subr, macro, struct, union, struct field, file-scope object, local), same control flow,
// same error model.  The twenty-nine `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's eleven
// non-`static` definitions (`initial_define_lispy_key`, `get_keymap`, `access_keymap`,
// `map_keymap`, `map_keymap_canonical`, `push_key_description`, `current_minor_maps`,
// `push_text_char_description`, `syms_of_keymap` and the `F*` subrs) and its one
// non-`static` object (`current_global_map`) are external symbols, so they carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`
// except the five whose address C takes - `map_keymap_item` and
// `map_keymap_char_table_item` (handed to `map_keymap_internal`/`map_char_table`),
// `map_keymap_call` (handed to `map_keymap`), `describe_vector_princ` (handed to
// `map_char_table`) and `silly_event_symbol_error` (handed to
// `record_unwind_protect`) - which are `unsafe extern "C" fn` (PORTING.md 1.4).  The five
// `static` objects and the rest of the `static` helpers are unexported `static mut`s and
// `unsafe fn`s.
//
// **`src/keymap.c` is in `docs/baseline/config/cond-eval-verified.txt`**
// (`python3 tools/check-cond-eval.py src/keymap.c` reports
// `0 conditions, 0 unknown, 0 disagreements [VERIFIED]`): the file carries no `#if`,
// `#ifdef` or `#if 0` at all, so P-08 has nothing to record and the port transcribes C's
// whole text.  The `line-drift-tolerance.tsv` row declares the default `total` basis.
//
// **`emacs/src/keymap.h`'s mirror was complete** (`src/include/keymap.rs`, its
// ledger `docs/ports/include-keymap.md` has zero unchecked rows) and needed exactly one
// change: its `unsafe extern "C"` block declared the nine functions and the one object
// that **this unit defines**, which is a hard `E0428` at the crate root.  Those
// declarations move into `pub mod keymap_decls` + `use keymap_decls::*;`, which is
// PORTING.md 0.1-10/P-21's established shape for a header whose prototypes meet their
// definitions (the `buffer_point_decls`/`charset_decls`/`intervals_decls` precedent), and
// other consumers are unchanged (the glob import is what they were relying on).  No
// declaration was added, removed or renamed.  The names this unit's live text takes from
// headers whose mirrors do not carry them are declared at their use site below with C's
// own prototype in a `/// C:` line: `reorder_modifiers`, `parse_modifiers`,
// `parse_solitary_modifier`, `lucid_event_type_list_p`, `menu_item_eval_property` and
// `message_with_string` (keyboard.c, whose mirror stops at line 520), `larger_vector` and
// `make_specified_string` (lisp.h declarations the lisp.rs mirror does not carry yet),
// `<stdckdint.h>`'s `ckd_mul` (which has no mirror at all, PORTING.md P-26), and
// `KEYMAP_H`-adjacent helpers `Fkeymap_prompt`'s `Fkeymap_prompt` recursion is internal.
// They move to their owners' mirrors when those land, and are reported in
// `docs/ports/keymap.md`.
//
// C's uninitialised locals are zero-initialised (PORTING.md 1.5), C's forward declarations
// of its own `static` helpers (C 75-84) are dropped (PORTING.md 0.1-12), and C's
// `describe_vector` prototype's function-pointer parameter is spelled
// `map_keymap_function_t`'s sibling below.  Every deviation is recorded in
// `docs/ports/keymap.md`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_comparisons,
         overflowing_literals, unused_braces)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] ../lib/include/mini-gmp.rs imported via common
// [U1] include/bignum.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/font.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/commands.rs imported via common
// [U1] include/buffer.rs imported via common
                                                // needs its macros defined first (syntax.rs's own note)
// [U1] include/syntax.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/keyboard.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/keymap.rs imported via common
// [U1] include/blockinput.rs imported via common

/* Names C's headers give this unit that no mirror carries.  Each carries C's own
   prototype on the `pub fn` line (the minibuf.rs/ccl.rs shape), and moves to its owner's
   mirror when that lands (reported in docs/ports/keymap.md).  */
unsafe extern "C-unwind" {





  /// C: lisp.h 4397: `extern void message_with_string (const char *, Lisp_Object, bool);`
  pub fn message_with_string (m: *const c_char, string: Lisp_Object, log: bool);

  /// C: lisp.h 4566: `extern Lisp_Object make_specified_string (const char *, ptrdiff_t,
  /// ptrdiff_t, bool);` (the lisp.rs mirror does not carry it yet - the ccl.rs shape).
  pub fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t,
                                nbytes: ptrdiff_t, multibyte: bool) -> Lisp_Object;

  /// C: lisp.h 4717: `extern ptrdiff_t esprintf (char *, char const *, ...)
  /// ATTRIBUTE_FORMAT_PRINTF (2, 3);` (doprnt.c; not declared by any included mirror).
  pub fn esprintf (buf: *mut c_char, format: *const c_char, ...) -> ptrdiff_t;
}
unsafe extern "C" {
  /// C: <string.h>: `extern char *strstr (const char *, const char *);` (absent from the
  /// string.rs mirror, which carries memcpy/memset/strchr/strlen).
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/* Actually allocate storage for these variables.  */

#[unsafe(no_mangle)]
pub static mut current_global_map: Lisp_Object = lisp_h_Qnil ();	/* Current global keymap.  */

/* Alist of elements like (DEL . "\d").  */
static mut exclude_keys: Lisp_Object = lisp_h_Qnil ();

/* Pre-allocated 2-element vector for Fcommand_remapping to use.  */
static mut command_remapping_vector: Lisp_Object = lisp_h_Qnil ();

/* Char table for the backwards-compatibility part in Flookup_key.  */
static mut unicode_case_table: Lisp_Object = lisp_h_Qnil ();

/* Hash table used to cache a reverse-map to speed up calls to where-is.  */
static mut where_is_cache: Lisp_Object = lisp_h_Qnil ();
/* Which keymaps are reverse-stored in the cache.  */
static mut where_is_cache_keymaps: Lisp_Object = lisp_h_Qnil ();

// C 75-84's forward declarations are dropped (PORTING.md 0.1-12).

unsafe extern "C-unwind" fn CHECK_VECTOR_OR_CHAR_TABLE (x: Lisp_Object)
{
  CHECK_TYPE (VECTORP (x) || CHAR_TABLE_P (x), Qvector_or_char_table_p (), x);
}

/* Keymap object support - constructors and predicates.			*/

DEFUN! ("make-keymap", Fmake_keymap, Smake_keymap, 0, 1, 0,
       doc: "Construct and return a new keymap, of the form (keymap CHARTABLE . ALIST).\nCHARTABLE is a char-table that holds the bindings for all characters\nwithout modifiers.  All entries in it are initially nil, meaning\n\"command undefined\".  ALIST is an assoc-list which holds bindings for\nfunction keys, mouse events, and any other things that appear in the\ninput stream.  Initially, ALIST is nil.\n\nThe optional arg STRING supplies a menu name for the keymap\nin case you use it as a menu with `x-popup-menu'.",
       (string: Lisp_Object),
       {
  let tail = if !NILP (string) { list1 (string) } else { Qnil () };
  return Fcons (Qkeymap (),
		Fcons (Fmake_char_table (Qkeymap (), Qnil ()), tail));
});

DEFUN! ("make-sparse-keymap", Fmake_sparse_keymap, Smake_sparse_keymap, 0, 1, 0,
       doc: "Construct and return a new sparse keymap.\nIts car is `keymap' and its cdr is an alist of (CHAR . DEFINITION),\nwhich binds the character CHAR to DEFINITION, or (SYMBOL . DEFINITION),\nwhich binds the function key or mouse event SYMBOL to DEFINITION.\nInitially the alist is nil.\n\nThe optional arg STRING supplies a menu name for the keymap\nin case you use it as a menu with `x-popup-menu'.",
       (string: Lisp_Object),
       {
  if !NILP (string)
    {
      return list2 (Qkeymap (), string);
    }
  return list1 (Qkeymap ());
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn initial_define_lispy_key (keymap: Lisp_Object,
                                                   keyname: *const c_char,
                                                   defname: *const c_char)
{
  store_in_keymap (keymap, intern_c_string (keyname),
		   intern_c_string (defname), false);
}

DEFUN! ("keymapp", Fkeymapp, Skeymapp, 1, 1, 0,
       doc: "Return t if OBJECT is a keymap.\n\nA keymap is a list (keymap . ALIST),\nor a symbol whose function definition is itself a keymap.\nALIST elements look like (CHAR . DEFN) or (SYMBOL . DEFN);\na vector of densely packed bindings for small character codes\nis also allowed as an element.",
       (object: Lisp_Object),
       {
  return if KEYMAPP! (object) { Qt () } else { Qnil () };
});

DEFUN! ("keymap-prompt", Fkeymap_prompt, Skeymap_prompt, 1, 1, 0,
       doc: "Return the prompt-string of a keymap MAP.\nIf non-nil, the prompt is shown in the echo-area\nwhen reading a key-sequence to be looked-up in this keymap.",
       (map: Lisp_Object),
       {
  let mut map = map;
  map = get_keymap (map, false, false);
  while CONSP (map)
    {
      let mut tem = XCAR (map);
      if STRINGP (tem)
	{ return tem; }
      else if KEYMAPP! (tem)
	{
	  tem = Fkeymap_prompt (tem);
	  if !NILP (tem)
	    { return tem; }
	}
      map = XCDR (map);
    }
  return Qnil ();
});

/* Check that OBJECT is a keymap (after dereferencing through any
   symbols).  If it is, return it.

   If AUTOLOAD and if OBJECT is a symbol whose function value
   is an autoload form, do the autoload and try again.
   If AUTOLOAD, callers must assume GC is possible.

   ERROR_IF_NOT_KEYMAP controls how we respond if OBJECT isn't a keymap.
   If ERROR_IF_NOT_KEYMAP, signal an error; otherwise,
   just return Qnil.

   Note that most of the time, we don't want to pursue autoloads.
   Functions like Faccessible_keymaps which scan entire keymap trees
   shouldn't load every autoloaded keymap.  I'm not sure about this,
   but it seems to me that only read_key_sequence, Flookup_key, and
   Fdefine_key should cause keymaps to be autoloaded.

   This function can GC when AUTOLOAD is true, because it calls
   Fautoload_do_load which can GC.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn get_keymap (mut object: Lisp_Object,
                                     error_if_not_keymap: bool,
                                     autoload: bool) -> Lisp_Object
{
 /* C's two labels (`autoload_retry:` for the back edge and `end:` for the forward exit) are
    both kept by name: the loop is the back edge and the labelled block is the exit
    (PORTING.md 2.2 - the ccl.rs/bytecode.rs shape for C's `goto`).  */
 'end: {
  'autoload_retry: loop {
  if NILP (object)
    { break 'end; }
  if CONSP (object) && EQ (XCAR (object), Qkeymap ())
    { return object; }

  let tem = indirect_function (object);
  if CONSP (tem)
    {
      if EQ (XCAR (tem), Qkeymap ())
	{ return tem; }

      /* Should we do an autoload?  Autoload forms for keymaps have
	 Qkeymap as their fifth element.  */
      if (autoload || !error_if_not_keymap) && EQ (XCAR (tem), Qautoload ())
	  && SYMBOLP (object)
	{
	  let tail: Lisp_Object;

	  tail = Fnth (make_fixnum (4), tem);
	  if EQ (tail, Qkeymap ())
	    {
	      if autoload
		{
		  Fautoload_do_load (tem, object, Qnil ());
		  continue 'autoload_retry;
		}
	      else
		{ return object; }
	    }
	}
    }

  break 'end;
  }
 }

  if error_if_not_keymap
    { wrong_type_argument (Qkeymapp (), object); }
  Qnil ()
}

/* Return the parent map of KEYMAP, or nil if it has none.
   We assume that KEYMAP is a valid keymap.  */

unsafe fn keymap_parent (mut keymap: Lisp_Object, autoload: bool) -> Lisp_Object
{
  keymap = get_keymap (keymap, true, autoload);

  /* Skip past the initial element `keymap'.  */
  let mut list = XCDR (keymap);
  while CONSP (list)
    {
      /* See if there is another `keymap'.  */
      if KEYMAPP! (list)
	{ return list; }
      list = XCDR (list);
    }

  return get_keymap (list, false, autoload);
}

DEFUN! ("keymap-parent", Fkeymap_parent, Skeymap_parent, 1, 1, 0,
       doc: "Return the parent keymap of KEYMAP.\nIf KEYMAP has no parent, return nil.",
       (keymap: Lisp_Object),
       {
  return keymap_parent (keymap, true);
});

/* Check whether MAP is one of MAPS parents.  */
unsafe fn keymap_memberp (map: Lisp_Object, mut maps: Lisp_Object) -> bool
{
  if NILP (map) { return false; }
  while KEYMAPP! (maps) && !EQ (map, maps)
    { maps = keymap_parent (maps, false); }
  return EQ (map, maps);
}

/* Set the parent keymap of MAP to PARENT.  */

DEFUN! ("set-keymap-parent", Fset_keymap_parent, Sset_keymap_parent, 2, 2, 0,
       doc: "Modify KEYMAP to set its parent map to PARENT.\nReturn PARENT.  PARENT should be nil or another keymap.",
       (keymap: Lisp_Object, parent: Lisp_Object),
       {
  let mut keymap = keymap;
  let mut parent = parent;
  /* Flush any reverse-map cache.  */
  where_is_cache = Qnil (); where_is_cache_keymaps = Qt ();

  keymap = get_keymap (keymap, true, true);

  if !NILP (parent)
    {
      parent = get_keymap (parent, true, false);

      /* Check for cycles.  */
      if keymap_memberp (keymap, parent)
	{ error (c"Cyclic keymap inheritance".as_ptr ()); }
    }

  /* Skip past the initial element `keymap'.  */
  let mut prev = keymap;
  loop
    {
      let list = XCDR (prev);
      /* If there is a parent keymap here, replace it.
	 If we came to the end, add the parent in PREV.  */
      if !CONSP (list) || KEYMAPP! (list)
	{
	  XSETCDR (prev, parent);
	  return parent;
	}
      prev = list;
    }
});

/* Look up IDX in MAP.  IDX may be any sort of event.
   Note that this does only one level of lookup; IDX must be a single
   event, not a sequence.

   MAP must be a keymap or a list of keymaps.

   If T_OK, bindings for Qt are treated as default
   bindings; any key left unmentioned by other tables and bindings is
   given the binding of Qt.

   If not T_OK, bindings for Qt are not treated specially.

   If NOINHERIT, don't accept a subkeymap found in an inherited keymap.

   Return Qunbound if no binding was found (and return Qnil if a nil
   binding was found).  */

unsafe fn access_keymap_1 (map: Lisp_Object, idx: Lisp_Object,
			   t_ok: bool, noinherit: bool) -> Lisp_Object
{
  let mut map = map;
  let mut idx = idx;
  let mut t_ok = t_ok;
  /* If idx is a list (some sort of mouse click, perhaps?),
     the index we want to use is the car of the list, which
     ought to be a symbol.  */
  idx = EVENT_HEAD (idx);

  /* If idx is a symbol, it might have modifiers, which need to
     be put in the canonical order.  */
  if SYMBOLP (idx)
    { idx = reorder_modifiers (idx); }
  else if FIXNUMP (idx)
    /* Clobber the high bits that can be present on a machine
       with more than 24 bits of integer.  */
    { XSETFASTINT! (idx, XFIXNUM (idx) & ((CHAR_META | (CHAR_META - 1)) as EMACS_INT)); }

  /* Handle the special meta -> esc mapping.  */
  if FIXNUMP (idx) && (XFIXNAT (idx) & meta_modifier as EMACS_INT) != 0
    {
      /* See if there is a meta-map.  If there's none, there is
         no binding for IDX, unless a default binding exists in MAP.  */
      let event_meta_binding: Lisp_Object;
      let event_meta_map: Lisp_Object;
      /* A strange value in which Meta is set would cause
	 infinite recursion.  Protect against that.  */
      if (XFIXNUM (meta_prefix_char) & CHAR_META as EMACS_INT) != 0
	{ meta_prefix_char = make_fixnum (27); }
      event_meta_binding = access_keymap_1 (map, meta_prefix_char, t_ok,
					    noinherit);
      event_meta_map = get_keymap (event_meta_binding, false, true);
      if CONSP (event_meta_map)
	{
	  map = event_meta_map;
	  idx = make_fixnum (XFIXNAT (idx) & !(meta_modifier as EMACS_INT));
	}
      else if t_ok
	/* Set IDX to t, so that we only find a default binding.  */
	{ idx = Qt (); }
      else
	/* An explicit nil binding, or no binding at all.  */
	{ return if NILP (event_meta_binding) { Qnil () } else { Qunbound () }; }
    }

  /* t_binding is where we put a default binding that applies,
     to use in case we do not find a binding specifically
     for this key sequence.  */
  {
    let mut tail: Lisp_Object;
    let mut t_binding = Qunbound ();
    let mut retval = Qunbound ();
    let mut retval_tail = Qnil ();

    tail = if CONSP (map) && EQ (Qkeymap (), XCAR (map)) { XCDR (map) } else { map };
    while CONSP (tail)
	  || { tail = get_keymap (tail, false, true); CONSP (tail) }
      {
	/* Qunbound in VAL means we have found no binding.  */
	let mut val = Qunbound ();
	let binding = XCAR (tail);
	let submap = get_keymap (binding, false, true);

	if EQ (binding, Qkeymap ())
	  {
	    if noinherit || NILP (retval)
	      /* If NOINHERIT, stop here, the rest is inherited.  */
	      { break; }
	    else if !BASE_EQ (retval, Qunbound ())
	      {
		let parent_entry: Lisp_Object;
		eassert! (KEYMAPP! (retval));
		parent_entry
		  = get_keymap (access_keymap_1 (tail, idx, t_ok, false),
				false, true);
		if KEYMAPP! (parent_entry)
		  {
		    if CONSP (retval_tail)
		      { XSETCDR (retval_tail, parent_entry); }
		    else
		      {
			retval_tail = Fcons (retval, parent_entry);
			retval = Fcons (Qkeymap (), retval_tail);
		      }
		  }
		break;
	      }
	  }
	else if CONSP (submap)
	  {
	    val = access_keymap_1 (submap, idx, t_ok, noinherit);
	  }
	else if CONSP (binding)
	  {
	    let key = XCAR (binding);

	    if EQ (key, idx)
	      { val = XCDR (binding); }
	    else if t_ok && EQ (key, Qt ())
	      {
		t_binding = XCDR (binding);
		t_ok = false;
	      }
	  }
	else if VECTORP (binding)
	  {
	    if FIXNUMP (idx) && XFIXNAT (idx) < ASIZE (binding)
	      { val = AREF (binding, XFIXNAT (idx)); }
	  }
	else if CHAR_TABLE_P (binding)
	  {
	    /* Character codes with modifiers
	       are not included in a char-table.
	       All character codes without modifiers are included.  */
	    if FIXNUMP (idx) && (XFIXNAT (idx) & CHAR_MODIFIER_MASK as EMACS_INT) == 0
	      {
		val = Faref (binding, idx);
		/* nil has a special meaning for char-tables, so
		   we use something else to record an explicitly
		   unbound entry.  */
		if NILP (val)
		  { val = Qunbound (); }
	      }
	  }

	/* If we found a binding, clean it up and return it.  */
	if !BASE_EQ (val, Qunbound ())
	  {
	    if EQ (val, Qt ())
	      /* A Qt binding is just like an explicit nil binding
		 (i.e. it shadows any parent binding but not bindings in
		 keymaps of lower precedence).  */
	      { val = Qnil (); }

	    val = get_keyelt (val, true);

	    if !KEYMAPP! (val)
	      {
		if NILP (retval) || BASE_EQ (retval, Qunbound ())
		  { retval = val; }
		if !NILP (val)
		  { break; }  /* Shadows everything that follows.  */
	      }
	    else if NILP (retval) || BASE_EQ (retval, Qunbound ())
	      { retval = val; }
	    else if CONSP (retval_tail)
	      {
		XSETCDR (retval_tail, list1 (val));
		retval_tail = XCDR (retval_tail);
	      }
	    else
	      {
		retval_tail = list1 (val);
		retval = Fcons (Qkeymap (), Fcons (retval, retval_tail));
	      }
	  }
	maybe_quit ();
	tail = XCDR (tail);
      }

    return if BASE_EQ (Qunbound (), retval)
           { get_keyelt (t_binding, true) } else { retval };
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn access_keymap (map: Lisp_Object, idx: Lisp_Object,
				        t_ok: bool, noinherit: bool) -> Lisp_Object
{
  let val = access_keymap_1 (map, idx, t_ok, noinherit);
  return if BASE_EQ (val, Qunbound ()) { Qnil () } else { val };
}

unsafe extern "C-unwind" fn map_keymap_item (fun: map_keymap_function_t, args: Lisp_Object, key: Lisp_Object, val: Lisp_Object, data: *mut c_void)
{
  let mut val = val;
  if EQ (val, Qt ())
    { val = Qnil (); }
  fun.unwrap () (key, val, args, data);
}

/// C: `union map_keymap` (keymap.c 507-516).  The anonymous `struct` is named
/// `map_keymap_s` (PORTING.md 1.3: a nested anonymous struct must be named), and the
/// union carries `#[repr(align(8))]` in place of `GCALIGNED_UNION_MEMBER`
/// (`char alignas (GCALIGNMENT) gcaligned`, lisp.h 303; `GCALIGNMENT` is 8 on this target),
/// which is the thread.rs shape.  clang measures the record's layout on the frozen target:
/// `sizeof (union map_keymap)` 24, `alignof` 8, `fun`@0, `args`@8, `data`@16.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct map_keymap_s
{
  pub fun: map_keymap_function_t,
  pub args: Lisp_Object,
  pub data: *mut c_void,
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub union map_keymap
{
  pub s: map_keymap_s,
}
const _: () = core::assert! (std::mem::align_of::<map_keymap> () % 8 == 0);  // static_assert (GCALIGNED (union map_keymap));
const _: () = core::assert! (std::mem::size_of::<map_keymap_s> () == 24);
const _: () = core::assert! (std::mem::align_of::<map_keymap_s> () == 8);
const _: () = core::assert! (std::mem::offset_of! (map_keymap_s, fun) == 0);
const _: () = core::assert! (std::mem::offset_of! (map_keymap_s, args) == 8);
const _: () = core::assert! (std::mem::offset_of! (map_keymap_s, data) == 16);
const _: () = core::assert! (std::mem::size_of::<map_keymap> () == 24);
const _: () = core::assert! (std::mem::align_of::<map_keymap> () == 8);

unsafe extern "C-unwind" fn map_keymap_char_table_item (args: Lisp_Object, key: Lisp_Object, val: Lisp_Object)
{
  let mut key = key;
  if !NILP (val)
    {
      /* If the key is a range, make a copy since map_char_table modifies
	 it in place.  */
      if CONSP (key)
	{ key = Fcons (XCAR (key), XCDR (key)); }
      let md = XFIXNUMPTR (args) as *mut map_keymap;
      map_keymap_item ((*md).s.fun, (*md).s.args, key, val, (*md).s.data);
    }
}

/* Call FUN for every binding in MAP and stop at (and return) the parent.
   FUN is called with 4 arguments: FUN (KEY, BINDING, ARGS, DATA).  */
unsafe fn map_keymap_internal (map: Lisp_Object,
			       fun: map_keymap_function_t,
			       args: Lisp_Object,
			       data: *mut c_void) -> Lisp_Object
{
  let mut tail = if CONSP (map) && EQ (Qkeymap (), XCAR (map)) { XCDR (map) } else { map };

  while CONSP (tail) && !EQ (Qkeymap (), XCAR (tail))
    {
      let binding = XCAR (tail);

      if KEYMAPP! (binding)	/* An embedded parent.  */
	{ break; }
      else if CONSP (binding)
	{ map_keymap_item (fun, args, XCAR (binding), XCDR (binding), data); }
      else if VECTORP (binding)
	{
	  /* Loop over the char values represented in the vector.  */
	  let len = ASIZE (binding) as c_int;
	  let mut c: c_int;
	  c = 0;
	  while c < len
	    {
	      let mut character: Lisp_Object;
	      XSETFASTINT! (character, c as EMACS_INT);
	      map_keymap_item (fun, args, character, AREF (binding, c as isize), data);
	      c += 1;
	    }
	}
      else if CHAR_TABLE_P (binding)
	{
	  let mut mapdata = map_keymap { s: map_keymap_s { fun, args, data } };
	  map_char_table (Some (map_keymap_char_table_item), Qnil (),
			  binding,
			  make_pointer_integer (&raw mut mapdata as *mut c_void));
	}
      tail = XCDR (tail);
    }

  return tail;
}

unsafe extern "C-unwind" fn map_keymap_call (key: Lisp_Object, val: Lisp_Object, fun: Lisp_Object, dummy: *mut c_void)
{
  calln! (fun, key, val);
}

/* Same as map_keymap_internal, but traverses parent keymaps as well.
   AUTOLOAD indicates that autoloaded keymaps should be loaded.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn map_keymap (map: Lisp_Object, fun: map_keymap_function_t, args: Lisp_Object,
				     data: *mut c_void, autoload: bool)
{
  let mut map = map;
  map = get_keymap (map, true, autoload);
  while CONSP (map)
    {
      if KEYMAPP! (XCAR (map))
	{
	  map_keymap (XCAR (map), fun, args, data, autoload);
	  map = XCDR (map);
	}
      else
	{ map = map_keymap_internal (map, fun, args, data); }
      if !CONSP (map)
	{ map = get_keymap (map, false, autoload); }
    }
}

/* Same as map_keymap, but does it right, properly eliminating duplicate
   bindings due to inheritance.   */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn map_keymap_canonical (map: Lisp_Object, fun: map_keymap_function_t, args: Lisp_Object, data: *mut c_void)
{
  let mut map = map;
  /* map_keymap_canonical may be used from redisplay (e.g. when building menus)
     so be careful to ignore errors and to inhibit redisplay.  */
  map = safe_calln! (Qkeymap_canonicalize (), map);
  /* No need to use `map_keymap' here because canonical map has no parent.  */
  map_keymap_internal (map, fun, args, data);
}

DEFUN! ("map-keymap-internal", Fmap_keymap_internal, Smap_keymap_internal, 2, 2, 0,
       doc: "Call FUNCTION once for each event binding in KEYMAP.\nFUNCTION is called with two arguments: the event that is bound, and\nthe definition it is bound to.  The event may be a character range.\nIf KEYMAP has a parent, this function returns it without processing it.",
       (function: Lisp_Object, keymap: Lisp_Object),
       {
  let mut keymap = keymap;
  keymap = get_keymap (keymap, true, true);
  keymap = map_keymap_internal (keymap, Some (map_keymap_call), function, std::ptr::null_mut ());
  return keymap;
});

DEFUN! ("map-keymap", Fmap_keymap, Smap_keymap, 2, 3, 0,
       doc: "Call FUNCTION once for each event binding in KEYMAP.\nFUNCTION is called with two arguments: the event that is bound, and\nthe definition it is bound to.  The event may be a character range.\n\nIf KEYMAP has a parent, the parent's bindings are included as well.\nThis works recursively: if the parent has itself a parent, then the\ngrandparent's bindings are also included and so on.\n\nFor more information, see Info node `(elisp) Keymaps'.\n\nusage: (map-keymap FUNCTION KEYMAP)",
       (function: Lisp_Object, keymap: Lisp_Object, sort_first: Lisp_Object),
       {
  if ! NILP (sort_first)
    { return calln! (Qmap_keymap_sorted (), function, keymap); }

  map_keymap (keymap, Some (map_keymap_call), function, std::ptr::null_mut (), true);
  return Qnil ();
});

DEFUN! ("keymap--get-keyelt", Fkeymap__get_keyelt, Skeymap__get_keyelt, 2, 2, 0,
       doc: "Given OBJECT which was found in a slot in a keymap,\ntrace indirect definitions to get the actual definition of that slot.\nAn indirect definition is a list of the form\n(KEYMAP . INDEX), where KEYMAP is a keymap or a symbol defined as one\nand INDEX is the object to look up in KEYMAP to yield the definition.\n\nAlso if OBJECT has a menu string as the first element,\nremove that.  Also remove a menu help string as second element.\n\nIf AUTOLOAD, load autoloadable keymaps\nthat are referred to with indirection.",
       (object: Lisp_Object, autoload: Lisp_Object),
       {
  return get_keyelt (object, if NILP (autoload) { false } else { true });
});

/* Given OBJECT which was found in a slot in a keymap,
   trace indirect definitions to get the actual definition of that slot.
   An indirect definition is a list of the form
   (KEYMAP . INDEX), where KEYMAP is a keymap or a symbol defined as one
   and INDEX is the object to look up in KEYMAP to yield the definition.

   Also if OBJECT has a menu string as the first element,
   remove that.  Also remove a menu help string as second element.

   If AUTOLOAD, load autoloadable keymaps
   that are referred to with indirection.

   This can GC because menu_item_eval_property calls Feval.  */

unsafe fn get_keyelt (object: Lisp_Object, autoload: bool) -> Lisp_Object
{
  let mut object = object;
  loop
    {
      if !(CONSP (object))
	/* This is really the value.  */
	{ return object; }

      /* If the keymap contents looks like (menu-item name . DEFN)
	 or (menu-item name DEFN ...) then use DEFN.
	 This is a new format menu item.  */
      else if EQ (XCAR (object), Qmenu_item ())
	{
	  if CONSP (XCDR (object))
	    {
	      let mut tem: Lisp_Object;

	      object = XCDR (XCDR (object));
	      tem = object;
	      if CONSP (object)
		{ object = XCAR (object); }

	      /* If there's a `:filter FILTER', apply FILTER to the
		 menu-item's definition to get the real definition to
		 use.  */
	      while CONSP (tem) && CONSP (XCDR (tem))
		{
		  if EQ (XCAR (tem), QCfilter ()) && autoload
		    {
		      let mut filter: Lisp_Object;
		      filter = XCAR (XCDR (tem));
		      filter = list2 (filter, list2 (Qquote (), object));
		      object = menu_item_eval_property (filter);
		      break;
		    }
		  tem = XCDR (tem);
		}
	    }
	  else
	    /* Invalid keymap.  */
	    { return object; }
	}

      /* If the keymap contents looks like (STRING . DEFN), use DEFN.
	 Keymap alist elements like (CHAR MENUSTRING . DEFN)
	 will be used by HierarKey menus.  */
      else if STRINGP (XCAR (object))
	{ object = XCDR (object); }

      else
	{ return object; }
    }
}

unsafe fn store_in_keymap (keymap: Lisp_Object, idx: Lisp_Object,
			   def: Lisp_Object, remove: bool) -> Lisp_Object
{
  let mut idx = idx;
  let mut def = def;
  /* Flush any reverse-map cache.  */
  where_is_cache = Qnil ();
  where_is_cache_keymaps = Qt ();

  if EQ (idx, Qkeymap ())
    { error (c"`keymap' is reserved for embedded parent maps".as_ptr ()); }

  /* If we are preparing to dump, and DEF is a menu element
     with a menu item indicator, copy it to ensure it is not pure.  */
  if CONSP (def)
      && (EQ (XCAR (def), Qmenu_item ()) || STRINGP (XCAR (def)))
    { def = Fcons (XCAR (def), XCDR (def)); }

  if !CONSP (keymap) || !EQ (XCAR (keymap), Qkeymap ())
    { error (c"Attempt to define a key in a non-keymap".as_ptr ()); }

  /* If idx is a cons, and the car part is a character, idx must be of
     the form (FROM-CHAR . TO-CHAR).  */
  if CONSP (idx) && CHARACTERP (XCAR (idx))
    { CHECK_CHARACTER_CDR (idx); }
  else
    /* If idx is a list (some sort of mouse click, perhaps?),
       the index we want to use is the car of the list, which
       ought to be a symbol.  */
    { idx = EVENT_HEAD (idx); }

  /* If idx is a symbol, it might have modifiers, which need to
     be put in the canonical order.  */
  if SYMBOLP (idx)
    { idx = reorder_modifiers (idx); }
  else if FIXNUMP (idx)
    /* Clobber the high bits that can be present on a machine
       with more than 24 bits of integer.  */
    { XSETFASTINT! (idx, XFIXNUM (idx) & ((CHAR_META | (CHAR_META - 1)) as EMACS_INT)); }

  /* Scan the keymap for a binding of idx.  */
  {
    let mut tail: Lisp_Object;

    /* The cons after which we should insert new bindings.  If the
       keymap has a table element, we record its position here, so new
       bindings will go after it; this way, the table will stay
       towards the front of the alist and character lookups in dense
       keymaps will remain fast.  Otherwise, this just points at the
       front of the keymap.  */
    let mut insertion_point = keymap;
    'keymap_end: {
    tail = XCDR (keymap);
    while CONSP (tail)
      {
	let elt = XCAR (tail);
	if VECTORP (elt)
	  {
	    if FIXNATP (idx) && XFIXNAT (idx) < ASIZE (elt)
	      {
		ASET (elt, XFIXNAT (idx), def);
		return def;
	      }
	    else if CONSP (idx) && CHARACTERP (XCAR (idx))
	      {
		let mut from = XFIXNAT (XCAR (idx)) as c_int;
		let mut to = XFIXNAT (XCDR (idx)) as c_int;

		if to as isize >= ASIZE (elt)
		  { to = (ASIZE (elt) - 1) as c_int; }
		while from <= to
		  {
		    ASET (elt, from as isize, def);
		    from += 1;
		  }
		if to as isize == XFIXNAT (XCDR (idx))
		  /* We have defined all keys in IDX.  */
		  { return def; }
	      }
	    insertion_point = tail;
	  }
	else if CHAR_TABLE_P (elt)
	  {
	    let mut sdef = def;
	    if remove
	      { sdef = Qnil (); }
	    /* nil has a special meaning for char-tables, so
	       we use something else to record an explicitly
	       unbound entry.  */
	    else if NILP (sdef)
	      { sdef = Qt (); }

	    /* Character codes with modifiers
	       are not included in a char-table.
	       All character codes without modifiers are included.  */
	    if FIXNATP (idx) && (XFIXNAT (idx) & CHAR_MODIFIER_MASK as EMACS_INT) == 0
	      {
		Faset (elt, idx, sdef);
		return def;
	      }
	    else if CONSP (idx) && CHARACTERP (XCAR (idx))
	      {
		Fset_char_table_range (elt, idx, sdef);
		return def;
	      }
	    insertion_point = tail;
	  }
	else if CONSP (elt)
	  {
	    if EQ (Qkeymap (), XCAR (elt))
	      { /* A sub keymap.  This might be due to a lookup that found
		   two matching bindings (maybe because of a sub keymap).
		   It almost never happens (since the second binding normally
		   only happens in the inherited part of the keymap), but
		   if it does, we want to update the sub-keymap since the
		   main one might be temporary (built by access_keymap).  */
		tail = elt;
		insertion_point = elt;
	      }
	    else if EQ (idx, XCAR (elt))
	      {
		if remove
		  /* Remove the element. */
		  { insertion_point = Fdelq (elt, insertion_point); }
		else
		  /* Just set the definition. */
		  { XSETCDR (elt, def); }
		return def;
	      }
	    else if CONSP (idx)
		     && CHARACTERP (XCAR (idx))
		     && CHARACTERP (XCAR (elt))
	      {
		let from = XFIXNAT (XCAR (idx)) as c_int;
		let to = XFIXNAT (XCDR (idx)) as c_int;

		if from as isize <= XFIXNAT (XCAR (elt))
		    && to as isize >= XFIXNAT (XCAR (elt))
		  {
		    if remove
		      { insertion_point = Fdelq (elt, insertion_point); }
		    else
		      { XSETCDR (elt, def); }
		    if from == to
		      { return def; }
		  }
	      }
	  }
	else if EQ (elt, Qkeymap ())
	  /* If we find a 'keymap' symbol in the spine of KEYMAP,
	     then we must have found the start of a second keymap
	     being used as the tail of KEYMAP, and a binding for IDX
	     should be inserted before it.  */
	  { break 'keymap_end; }

	maybe_quit ();
	tail = XCDR (tail);
      }
    }

    /* We have scanned the entire keymap, and not found a binding for
       IDX.  Let's add one.  */
    if !remove
      {
	let elt: Lisp_Object;

	if CONSP (idx) && CHARACTERP (XCAR (idx))
	  {
	    /* IDX specifies a range of characters, and not all of them
	       were handled yet, which means this keymap doesn't have a
	       char-table.  So, we insert a char-table now.  */
	    elt = Fmake_char_table (Qkeymap (), Qnil ());
	    Fset_char_table_range (elt, idx, if NILP (def) { Qt () } else { def });
	  }
	else
	  { elt = Fcons (idx, def); }
	XSETCDR (insertion_point, Fcons (elt, XCDR (insertion_point)));
      }
  }

  return def;
}

// C 905's `static Lisp_Object copy_keymap_1 (Lisp_Object keymap, int depth);` is a forward
// declaration of this unit's own helper and is dropped (PORTING.md 0.1-12).

unsafe fn copy_keymap_item (elt: Lisp_Object, depth: c_int) -> Lisp_Object
{
  let mut elt = elt;
  let mut res: Lisp_Object;
  let mut tem: Lisp_Object;

  if !CONSP (elt)
    { return elt; }

  tem = elt;
  res = elt;

  /* Is this a new format menu item.  */
  if EQ (XCAR (tem), Qmenu_item ())
    {
      /* Copy cell with menu-item marker.  */
      elt = Fcons (XCAR (tem), XCDR (tem));
      res = elt;
      tem = XCDR (elt);
      if CONSP (tem)
	{
	  /* Copy cell with menu-item name.  */
	  XSETCDR (elt, Fcons (XCAR (tem), XCDR (tem)));
	  elt = XCDR (elt);
	  tem = XCDR (elt);
	}
      if CONSP (tem)
	{
	  /* Copy cell with binding and if the binding is a keymap,
	     copy that.  */
	  XSETCDR (elt, Fcons (XCAR (tem), XCDR (tem)));
	  elt = XCDR (elt);
	  tem = XCAR (elt);
	  if CONSP (tem) && EQ (XCAR (tem), Qkeymap ())
	    { XSETCAR (elt, copy_keymap_1 (tem, depth)); }
	  tem = XCDR (elt);
	}
    }
  else
    {
      /* It may be an old format menu item.
	 Skip the optional menu string.  */
      if STRINGP (XCAR (tem))
	{
	  /* Copy the cell, since copy-alist didn't go this deep.  */
	  elt = Fcons (XCAR (tem), XCDR (tem));
	  res = elt;
	  tem = XCDR (elt);
	  /* Also skip the optional menu help string.  */
	  if CONSP (tem) && STRINGP (XCAR (tem))
	    {
	      XSETCDR (elt, Fcons (XCAR (tem), XCDR (tem)));
	      elt = XCDR (elt);
	      tem = XCDR (elt);
	    }
	  if CONSP (tem) && EQ (XCAR (tem), Qkeymap ())
	    { XSETCDR (elt, copy_keymap_1 (tem, depth)); }
	}
      else if EQ (XCAR (tem), Qkeymap ())
	{ res = copy_keymap_1 (elt, depth); }
    }
  return res;
}

unsafe extern "C-unwind" fn copy_keymap_set_char_table (chartable_and_depth: Lisp_Object, idx: Lisp_Object,
			    elt: Lisp_Object)
{
  Fset_char_table_range
    (XCAR (chartable_and_depth), idx,
     copy_keymap_item (elt, XFIXNUM (XCDR (chartable_and_depth)) as c_int));
}

unsafe fn copy_keymap_1 (keymap: Lisp_Object, depth: c_int) -> Lisp_Object
{
  let mut keymap = keymap;
  let copy: Lisp_Object;
  let mut tail: Lisp_Object;

  if depth > 100
    { error (c"Possible infinite recursion when copying keymap".as_ptr ()); }

  keymap = get_keymap (keymap, true, false);
  tail = list1 (Qkeymap ());
  copy = tail;
  keymap = XCDR (keymap);		/* Skip the `keymap' symbol.  */

  while CONSP (keymap) && !EQ (XCAR (keymap), Qkeymap ())
    {
      let mut elt = XCAR (keymap);
      if CHAR_TABLE_P (elt)
	{
	  elt = Fcopy_sequence (elt);
	  map_char_table (Some (copy_keymap_set_char_table), Qnil (), elt,
			  Fcons (elt, make_fixnum ((depth + 1) as EMACS_INT)));
	}
      else if VECTORP (elt)
	{
	  elt = Fcopy_sequence (elt);
	  let mut i: c_int = 0;
	  while (i as isize) < ASIZE (elt)
	    {
	      ASET (elt, i as isize, copy_keymap_item (AREF (elt, i as isize), depth + 1));
	      i += 1;
	    }
	}
      else if CONSP (elt)
	{
	  if EQ (XCAR (elt), Qkeymap ())
	    /* This is a sub keymap.  */
	    { elt = copy_keymap_1 (elt, depth + 1); }
	  else
	    { elt = Fcons (XCAR (elt), copy_keymap_item (XCDR (elt), depth + 1)); }
	}
      XSETCDR (tail, list1 (elt));
      tail = XCDR (tail);
      keymap = XCDR (keymap);
    }
  XSETCDR (tail, keymap);
  return copy;
}

DEFUN! ("copy-keymap", Fcopy_keymap, Scopy_keymap, 1, 1, 0,
       doc: "Return a copy of the keymap KEYMAP.\n\nNote that this is almost never needed.  If you want a keymap that's like\nanother yet with a few changes, you should use keymap inheritance rather\nthan copying.  That is, something like:\n\n    (defvar-keymap foo-map\n      :parent <theirmap>\n      ...)\n\nOr, if you need to support Emacs versions older than 29:\n\n    (let ((map (make-sparse-keymap)))\n      (set-keymap-parent map <theirmap>)\n      (define-key map ...)\n      ...)\n\nAfter performing `copy-keymap', the copy starts out with the same definitions\nof KEYMAP, but changing either the copy or KEYMAP does not affect the other.\nAny key definitions that are subkeymaps are recursively copied.\nHowever, a key definition which is a symbol whose definition is a keymap\nis not copied.",
       (keymap: Lisp_Object),
       {
  return copy_keymap_1 (keymap, 0);
});

/* Simple Keymap mutators and accessors.				*/

unsafe fn possibly_translate_key_sequence (key: Lisp_Object, length: *mut ptrdiff_t) -> Lisp_Object
{
  let mut key = key;
  if VECTORP (key) && ASIZE (key) == 1 && STRINGP (AREF (key, 0))
    {
      /* KEY is on the ["C-c"] format, so translate to internal
	 format.  */
      if NILP (Ffboundp (Qkey_valid_p ()))
	{ xsignal2 (Qerror (),
		    build_string (c"`key-valid-p' is not defined, so this syntax can't be used: %s".as_ptr ()),
		    key); }
      /* If key-valid-p is unhappy about KEY, we return it as-is.
         This happens when menu items define as bindings strings that
         should be inserted into the buffer, not commands.  See
         bug#64927, for example.  */
      if NILP (calln! (Qkey_valid_p (), AREF (key, 0)))
	{ return key; }
      key = calln! (Qkey_parse (), AREF (key, 0));
      *length = CHECK_VECTOR_OR_STRING (key);
      if *length == 0
	{ xsignal2 (Qerror (), build_string (c"Invalid `key-parse' syntax: %S".as_ptr ()), key); }
    }

  return key;
}

/* GC is possible in this function if it autoloads a keymap.  */

DEFUN! ("define-key", Fdefine_key, Sdefine_key, 3, 4, 0,
       doc: "In KEYMAP, define key sequence KEY as DEF.\nThis is a legacy function; see `keymap-set' for the recommended\nfunction to use instead.\n\nKEYMAP is a keymap.\n\nKEY is a string or a vector of symbols and characters, representing a\nsequence of keystrokes and events.  Non-ASCII characters with codes\nabove 127 (such as ISO Latin-1) can be represented by vectors.\nTwo types of vector have special meanings:\n [remap COMMAND] remaps any key binding for COMMAND.\n [t] creates a default definition, which applies to any event with no\n    other definition in KEYMAP.\n\nDEF is anything that can be a key's definition:\n nil (means key is undefined in this keymap),\n a command (a Lisp function suitable for interactive calling),\n a string (treated as a keyboard macro),\n a keymap (to define a prefix key),\n a symbol (when the key is looked up, the symbol will stand for its\n    function definition, which should at that time be one of the above,\n    or another symbol whose function definition is used, etc.),\n a cons (STRING . DEFN), meaning that DEFN is the definition\n    (DEFN should be a valid definition in its own right) and\n    STRING is the menu item name (which is used only if the containing\n    keymap has been created with a menu name, see `make-keymap'),\n or a cons (MAP . CHAR), meaning use definition of CHAR in keymap MAP,\n or an extended menu item definition.\n (See info node `(elisp)Extended Menu Items'.)\n\nIf REMOVE is non-nil, the definition will be removed.  This is almost\nthe same as setting the definition to nil, but makes a difference if\nthe KEYMAP has a parent, and KEY is shadowing the same binding in the\nparent.  With REMOVE, subsequent lookups will return the binding in\nthe parent, and with a nil DEF, the lookups will return nil.\n\nIf KEYMAP is a sparse keymap with a binding for KEY, the existing\nbinding is altered.  If there is no binding for KEY, the new pair\nbinding KEY to DEF is added at the front of KEYMAP.",
       (keymap: Lisp_Object, key: Lisp_Object, def: Lisp_Object, remove: Lisp_Object),
       {
  let mut keymap = keymap;
  let mut key = key;
  let mut def = def;
  let mut metized = false;

  keymap = get_keymap (keymap, true, true);

  let mut length: ptrdiff_t = CHECK_VECTOR_OR_STRING (key);
  if length == 0
    { return Qnil (); }

  let meta_bit: c_int = if VECTORP (key) || (STRINGP (key) && STRING_MULTIBYTE (key))
		  { meta_modifier } else { 0x80 };

  if VECTORP (def) && ASIZE (def) > 0 && CONSP (AREF (def, 0))
    { /* DEF is apparently an XEmacs-style keyboard macro.  */
      let tmp = make_nil_vector (ASIZE (def));
      let mut i = ASIZE (def);
      while { i -= 1; i >= 0 }
	{
	  let mut defi = AREF (def, i);
	  if CONSP (defi) && lucid_event_type_list_p (defi)
	    { defi = Fevent_convert_list (defi); }
	  ASET (tmp, i, defi);
	}
      def = tmp;
    }

  key = possibly_translate_key_sequence (key, &mut length);

  let mut idx: ptrdiff_t = 0;
  loop
    {
      let mut c = Faref (key, make_fixnum (idx));

      if CONSP (c)
	{
	  /* C may be a Lucid style event type list or a cons (FROM .
	     TO) specifying a range of characters.  */
	  if lucid_event_type_list_p (c)
	    { c = Fevent_convert_list (c); }
	  else if CHARACTERP (XCAR (c))
	    { CHECK_CHARACTER_CDR (c); }
	}

      if SYMBOLP (c)
	{ silly_event_symbol_error (c); }

      if FIXNUMP (c)
	  && (XFIXNUM (c) & meta_bit as EMACS_INT) != 0
	  && !metized
	{
	  c = meta_prefix_char;
	  metized = true;
	}
      else
	{
	  if FIXNUMP (c)
	    { XSETINT! (c, XFIXNUM (c) & !(meta_bit as EMACS_INT)); }

	  metized = false;
	  idx += 1;
	}

      if !FIXNUMP (c) && !SYMBOLP (c)
	  && (!CONSP (c)
	      /* If C is a range, it must be a leaf.  */
	      || (FIXNUMP (XCAR (c)) && idx != length))
	{ message_with_string (c"Key sequence contains invalid event %s".as_ptr (), c, true); }

      if idx == length
	{ return store_in_keymap (keymap, c, def, !NILP (remove)); }

      let cmd = access_keymap (keymap, c, false, true);

      /* If this key is undefined, make it a prefix.  */
      let cmd = if NILP (cmd) { define_as_prefix (keymap, c) } else { cmd };

      keymap = get_keymap (cmd, false, true);
      if !CONSP (keymap)
	{
	  let trailing_esc: *const c_char = if EQ (c, meta_prefix_char) && metized
				      { if idx == 0 { c"ESC".as_ptr () } else { c" ESC".as_ptr () } }
				      else { c"".as_ptr () };

	  /* We must use Fkey_description rather than just passing key to
	     error; key might be a vector, not a string.  */
	  error (c"Key sequence %s starts with non-prefix key %s%s".as_ptr (),
		 SDATA (Fkey_description (key, Qnil ())),
		 SDATA (Fkey_description (Fsubstring (key, make_fixnum (0),
						      make_fixnum (idx)),
					  Qnil ())),
		 trailing_esc);
	}
    }
});

/* This function may GC (it calls Fkey_binding).  */

DEFUN! ("command-remapping", Fcommand_remapping, Scommand_remapping, 1, 3, 0,
       doc: "Return the remapping for command COMMAND.\nReturns nil if COMMAND is not remapped (or not a symbol).\n\nIf the optional argument POSITION is non-nil, it specifies a mouse\nposition as returned by `event-start' and `event-end', and the\nremapping occurs in the keymaps associated with it.  It can also be a\nnumber or marker, in which case the keymap properties at the specified\nbuffer position instead of point are used.  The KEYMAPS argument is\nignored if POSITION is non-nil.\n\nIf the optional argument KEYMAPS is non-nil, it should be a keymap or list of\nkeymaps to search for command remapping.  Otherwise, search for the\nremapping in all currently active keymaps.",
       (command: Lisp_Object, position: Lisp_Object, keymaps: Lisp_Object),
       {
  let mut command = command;
  if !SYMBOLP (command)
    { return Qnil (); }

  ASET (command_remapping_vector, 1, command);

  if NILP (keymaps)
    { command = Fkey_binding (command_remapping_vector, Qnil (), Qt (), position); }
  else
    { command = Flookup_key (keymaps, command_remapping_vector, Qnil ()); }
  return if FIXNUMP (command) { Qnil () } else { command };
});

unsafe fn lookup_key_1 (keymap: Lisp_Object, key: Lisp_Object, accept_default: Lisp_Object) -> Lisp_Object
{
  let mut keymap = keymap;
  let mut key = key;
  let t_ok = !NILP (accept_default);

  if !CONSP (keymap) && !NILP (keymap)
    { keymap = get_keymap (keymap, true, true); }

  let mut length: ptrdiff_t = CHECK_VECTOR_OR_STRING (key);
  if length == 0
    { return keymap; }

  key = possibly_translate_key_sequence (key, &mut length);

  let mut idx: ptrdiff_t = 0;
  loop
    {
      let mut c = Faref (key, make_fixnum (idx));
      idx += 1;

      if CONSP (c) && lucid_event_type_list_p (c)
	{ c = Fevent_convert_list (c); }

      /* Turn the 8th bit of string chars into a meta modifier.  */
      if STRINGP (key) && (XFIXNUM (c) & 0x80) != 0 && !STRING_MULTIBYTE (key)
	{ XSETINT! (c, (XFIXNUM (c) | meta_modifier as EMACS_INT) & !(0x80 as EMACS_INT)); }

      /* Allow string since binding for `menu-bar-select-buffer'
	 includes the buffer name in the key sequence.  */
      if !FIXNUMP (c) && !SYMBOLP (c) && !CONSP (c) && !STRINGP (c)
	{ message_with_string (c"Key sequence contains invalid event %s".as_ptr (), c, true); }

      let cmd = access_keymap (keymap, c, t_ok, false);
      if idx == length
	{ return cmd; }

      keymap = get_keymap (cmd, false, true);
      if !CONSP (keymap)
	{ return make_fixnum (idx); }

      maybe_quit ();
    }
}

DEFUN! ("lookup-key", Flookup_key, Slookup_key, 2, 3, 0,
       doc: "Look up key sequence KEY in KEYMAP.  Return the definition.\nThis is a legacy function; see `keymap-lookup' for the recommended\nfunction to use instead.\n\nA value of nil means undefined.  See doc of `define-key'\nfor kinds of definitions.\n\nA number as value means KEY is \"too long\";\nthat is, characters or symbols in it except for the last one\nfail to be a valid sequence of prefix characters in KEYMAP.\nThe number is how many characters at the front of KEY\nit takes to reach a non-prefix key.\nKEYMAP can also be a list of keymaps.\n\nNormally, `lookup-key' ignores bindings for t, which act as default\nbindings, used when nothing else in the keymap applies; this makes it\nusable as a general function for probing keymaps.  However, if the\nthird optional argument ACCEPT-DEFAULT is non-nil, `lookup-key' will\nrecognize the default bindings, just as `read-key-sequence' does.",
       (keymap: Lisp_Object, key: Lisp_Object, accept_default: Lisp_Object),
       {
  let mut found = lookup_key_1 (keymap, key, accept_default);
  if !NILP (found) && !NUMBERP (found)
    { return found; }

  /* Menu definitions might use mixed case symbols (notably in old
     versions of `easy-menu-define'), or use " " instead of "-".
     The rest of this function is about accepting these variations for
     backwards-compatibility.  (Bug#50752) */

  /* Just skip everything below unless this is a menu item.  */
  if !VECTORP (key) || !(ASIZE (key) > 0)
      || !EQ (AREF (key, 0), Qmenu_bar ())
    { return found; }

  /* Initialize the unicode case table, if it wasn't already.  */
  if NILP (unicode_case_table)
    {
      unicode_case_table = uniprop_table (Qlowercase ());
      /* uni-lowercase.el might be unavailable during bootstrap.  */
      if NILP (unicode_case_table)
	{ return found; }
      staticpro (&raw const unicode_case_table);
    }

  let key_len: ptrdiff_t = ASIZE (key);
  let new_key = make_vector (key_len, Qnil ());

  /* Try both the Unicode case table, and the buffer local one.
     Otherwise, we will fail for e.g. the "Turkish" language
     environment where 'I' does not downcase to 'i'.  */
  let tables: [Lisp_Object; 2] = [unicode_case_table, Fcurrent_case_table ()];
  let mut tbl_num: c_int = 0;
  while tbl_num < 2
    {
      /* First, let's try converting all symbols like "Foo-Bar-Baz" to
	 "foo-bar-baz".  */
      for i in 0..key_len
	{
	  let item = AREF (key, i);
	  if !SYMBOLP (item)
	    { ASET (new_key, i, item); }
	  else
	    {
	      let key_item = Fsymbol_name (item);
	      let new_item: Lisp_Object;
	      if !STRING_MULTIBYTE (key_item)
		{ new_item = Fdowncase (key_item); }
	      else
		{
		  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
		  let size = SCHARS (key_item);
		  let (n_prod, n_overflowed) = size.overflowing_mul (MAX_MULTIBYTE_LENGTH as ptrdiff_t);
		  let n = if n_overflowed { PTRDIFF_MAX } else { n_prod };
		  let dst = SAFE_ALLOCA! (sa_avail, n as usize) as *mut u8;
		  let mut p = dst;
		  let mut j_char: ptrdiff_t = 0;
		  let mut j_byte: ptrdiff_t = 0;

		  while j_char < size
		    {
		      let ch = fetch_string_char_advance (key_item,
							  &mut j_char, &mut j_byte);
		      let ch_conv = CHAR_TABLE_REF (tables[tbl_num as usize],
						    ch as isize);
		      if !NILP (ch_conv)
			{ CHAR_STRING (XFIXNUM (ch_conv) as c_int, p); }
		      else
			{ CHAR_STRING (ch, p); }
		      p = dst.offset (j_byte);
		    }
		  new_item = make_multibyte_string (dst as *const c_char,
						    SCHARS (key_item),
						    SBYTES (key_item));
		  SAFE_FREE! (sa_count, sa_mark);
		}
	      ASET (new_key, i, Fintern (new_item, Qnil ()));
	    }
	}

      /* Check for match.  */
      found = lookup_key_1 (keymap, new_key, accept_default);
      if !NILP (found) && !NUMBERP (found)
	{ break; }

      /* If we still don't have a match, let's convert any spaces in
	 our lowercased string into dashes, e.g. "foo bar baz" to
	 "foo-bar-baz".  */
      for i in 0..key_len
	{
	  if !SYMBOLP (AREF (new_key, i))
	    { continue; }

	  let lc_key = Fsymbol_name (AREF (new_key, i));

	  /* If there are no spaces in this symbol, just skip it.  */
	  if strstr (SSDATA (lc_key), c" ".as_ptr ()).is_null ()
	    { continue; }

	  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
	  let size = SCHARS (lc_key);
	  let (n_prod, n_overflowed) = size.overflowing_mul (MAX_MULTIBYTE_LENGTH as ptrdiff_t);
	  let n = if n_overflowed { PTRDIFF_MAX } else { n_prod };
	  let dst = SAFE_ALLOCA! (sa_avail, n as usize) as *mut u8;

	  /* We can walk the string data byte by byte, because UTF-8
	     encoding ensures that no other byte of any multibyte
	     sequence will ever include a 7-bit byte equal to an ASCII
	     single-byte character.  */
	  memcpy (dst as *mut c_void, SSDATA (lc_key) as *const c_void, SBYTES (lc_key) as usize);
	  for i in 0..SBYTES (lc_key)
	    {
	      if *dst.offset (i) == b' ' as u8
		{ *dst.offset (i) = b'-' as u8; }
	    }
	  let new_it =
	    make_multibyte_string (dst as *const c_char,
				   SCHARS (lc_key), SBYTES (lc_key));
	  ASET (new_key, i, Fintern (new_it, Qnil ()));
	  SAFE_FREE! (sa_count, sa_mark);
	}

      /* Check for match.  */
      found = lookup_key_1 (keymap, new_key, accept_default);
      if !NILP (found) && !NUMBERP (found)
	{ break; }
      tbl_num += 1;
    }

  return found;
});

/* Make KEYMAP define event C as a keymap (i.e., as a prefix).
   Assume that currently it does not define C at all.
   Return the keymap.  */

unsafe fn define_as_prefix (keymap: Lisp_Object, c: Lisp_Object) -> Lisp_Object
{
  let cmd = Fmake_sparse_keymap (Qnil ());
  store_in_keymap (keymap, c, cmd, false);

  return cmd;
}

/* Append a key to the end of a key sequence.  We always make a vector.  */

unsafe fn append_key (key_sequence: Lisp_Object, key: Lisp_Object) -> Lisp_Object
{
  AUTO_LIST1! (key_list, key);
  return CALLN! (Fvconcat, key_sequence, key_list);
}

/* Given an event type C which is a symbol,
   signal an error if is a mistake such as RET or M-RET or C-DEL, etc.  */

unsafe extern "C-unwind" fn silly_event_symbol_error (c: Lisp_Object)
{
  let mut c = c;
  let parsed = parse_modifiers (c);
  let modifiers = XFIXNAT (XCAR (XCDR (parsed))) as c_int;
  let base = XCAR (parsed);
  let name = Fsymbol_name (base);
  /* This alist includes elements such as ("RET" . "\\r").  */
  let assoc = Fassoc (name, exclude_keys, Qnil ());

  if ! NILP (assoc)
    {
      /* C: `char new_mods[sizeof ("\\A-\\C-\\H-\\M-\\S-\\s-")];` = 19 bytes.  */
      let mut new_mods: [c_char; 19] = [0; 19];
      let mut p = new_mods.as_mut_ptr ();
      let keystring: Lisp_Object;
      if (modifiers & alt_modifier) != 0
	{ *p = b'\\' as c_char; p = p.add (1); *p = b'A' as c_char; p = p.add (1); *p = b'-' as c_char; p = p.add (1); }
      if (modifiers & ctrl_modifier) != 0
	{ *p = b'\\' as c_char; p = p.add (1); *p = b'C' as c_char; p = p.add (1); *p = b'-' as c_char; p = p.add (1); }
      if (modifiers & hyper_modifier) != 0
	{ *p = b'\\' as c_char; p = p.add (1); *p = b'H' as c_char; p = p.add (1); *p = b'-' as c_char; p = p.add (1); }
      if (modifiers & meta_modifier) != 0
	{ *p = b'\\' as c_char; p = p.add (1); *p = b'M' as c_char; p = p.add (1); *p = b'-' as c_char; p = p.add (1); }
      if (modifiers & shift_modifier) != 0
	{ *p = b'\\' as c_char; p = p.add (1); *p = b'S' as c_char; p = p.add (1); *p = b'-' as c_char; p = p.add (1); }
      if (modifiers & super_modifier) != 0
	{ *p = b'\\' as c_char; p = p.add (1); *p = b's' as c_char; p = p.add (1); *p = b'-' as c_char; p = p.add (1); }
      *p = 0;

      c = reorder_modifiers (c);
      AUTO_STRING_WITH_LEN! (new_mods_string, new_mods.as_ptr (), p.offset_from (new_mods.as_ptr ()));
      keystring = concat2 (new_mods_string, XCDR (assoc));

      error (c"To bind the key %s, use [?%s], not [%s]".as_ptr (),
	     SDATA (SYMBOL_NAME (c)), SDATA (keystring),
	     SDATA (SYMBOL_NAME (c)));
    }
}

/* Global, local, and minor mode keymap stuff.				*/

/* We can't put these variables inside current_minor_maps, since under
   some systems, static gets macro-defined to be the empty string.
   Ickypoo.  */
static mut cmm_modes: *mut Lisp_Object = std::ptr::null_mut ();
static mut cmm_maps: *mut Lisp_Object = std::ptr::null_mut ();
static mut cmm_size: ptrdiff_t = 0;

/* Store a pointer to an array of the currently active minor modes in
   *modeptr, a pointer to an array of the keymaps of the currently
   active minor modes in *mapptr, and return the number of maps
   *mapptr contains.

   This function always returns a pointer to the same buffer, and may
   free or reallocate it, so if you want to keep it for a long time or
   hand it out to lisp code, copy it.  This procedure will be called
   for every key sequence read, so the nice lispy approach (return a
   new assoclist, list, what have you) for each invocation would
   result in a lot of consing over time.

   If we used xrealloc/xmalloc and ran out of memory, they would throw
   back to the command loop, which would try to read a key sequence,
   which would call this function again, resulting in an infinite
   loop.  Instead, we'll use realloc/malloc and silently truncate the
   list, let the key sequence be read, and hope some other piece of
   code signals the error.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn current_minor_maps (modeptr: *mut *mut Lisp_Object, mapptr: *mut *mut Lisp_Object) -> ptrdiff_t
{
  let mut i: ptrdiff_t = 0;
  let mut alist: Lisp_Object;
  let mut assoc: Lisp_Object;
  let mut var: Lisp_Object;
  let mut val: Lisp_Object;
  let mut emulation_alists = Vemulation_mode_map_alists;
  /* C's `Lisp_Object lists[2];` is zero-initialised (PORTING.md 1.5).  */
  let mut lists: [Lisp_Object; 2] = [Qnil (); 2];

  lists[0] = Vminor_mode_overriding_map_alist;
  lists[1] = Vminor_mode_map_alist;

  let mut list_number: c_int = 0;
  while list_number < 2
    {
      if CONSP (emulation_alists)
	{
	  alist = XCAR (emulation_alists);
	  emulation_alists = XCDR (emulation_alists);
	  if SYMBOLP (alist)
	    { alist = find_symbol_value (alist); }
	  list_number = -1;
	}
      else
	{ alist = lists[list_number as usize]; }

      while CONSP (alist)
	{
	  if { assoc = XCAR (alist); CONSP (assoc) }
	    && { var = XCAR (assoc); SYMBOLP (var) }
	    && { val = find_symbol_value (var); !BASE_EQ (val, Qunbound ()) }
	    && !NILP (val)
	  {
	    let temp: Lisp_Object;

	    /* If a variable has an entry in Vminor_mode_overriding_map_alist,
	       and also an entry in Vminor_mode_map_alist,
	       ignore the latter.  */
	    if list_number == 1
	      {
		val = assq_no_quit (var, lists[0]);
		if !NILP (val)
		  { alist = XCDR (alist); continue; }
	      }

	    if i >= cmm_size
	      {
		let newsize: ptrdiff_t;
		let allocsize: usize;
		let newmodes: *mut Lisp_Object;
		let newmaps: *mut Lisp_Object;

		/* Check for size calculation overflow.  Other code
		   (e.g., read_key_sequence) adds 3 to the count
		   later, so subtract 3 from the limit here.  */
		if (PTRDIFF_MAX as usize).min (SIZE_MAX) / (2 * std::mem::size_of::<*mut Lisp_Object> ()) - 3
		    < cmm_size as usize
		  { break; }

		newsize = if cmm_size == 0 { 30 } else { cmm_size * 2 };
		allocsize = newsize as usize * std::mem::size_of::<*mut Lisp_Object> ();

		/* Use malloc here.  See the comment above this function.
		   Avoid realloc here; it causes spurious traps on GNU/Linux [KFS] */
		block_input ();
		newmodes = malloc (allocsize) as *mut Lisp_Object;
		if !newmodes.is_null ()
		  {
		    if !cmm_modes.is_null ()
		      {
			memcpy (newmodes as *mut c_void, cmm_modes as *const c_void,
				cmm_size as usize * std::mem::size_of::<*mut Lisp_Object> ());
			free (cmm_modes as *mut c_void);
		      }
		    cmm_modes = newmodes;
		  }

		newmaps = malloc (allocsize) as *mut Lisp_Object;
		if !newmaps.is_null ()
		  {
		    if !cmm_maps.is_null ()
		      {
			memcpy (newmaps as *mut c_void, cmm_maps as *const c_void,
				cmm_size as usize * std::mem::size_of::<*mut Lisp_Object> ());
			free (cmm_maps as *mut c_void);
		      }
		    cmm_maps = newmaps;
		  }
		unblock_input ();

		if newmodes.is_null () || newmaps.is_null ()
		  { break; }
		cmm_size = newsize;
	      }

	    /* Get the keymap definition--or nil if it is not defined.  */
	    temp = Findirect_function (XCDR (assoc), Qt ());
	    if !NILP (temp)
	      {
		*cmm_modes.offset (i) = var;
		*cmm_maps.offset (i) = temp;
		i += 1;
	      }
	  }
	  alist = XCDR (alist);
	}
      list_number += 1;
    }

  if !modeptr.is_null () { *modeptr = cmm_modes; }
  if !mapptr.is_null ()  { *mapptr  = cmm_maps; }
  return i;
}

/* Return the offset of POSITION, a click position, in the style of
   the respective argument of Fkey_binding.  */
unsafe fn click_position (position: Lisp_Object) -> ptrdiff_t
{
  let pos: EMACS_INT = if FIXNUMP (position) { XFIXNUM (position) }
		   else if MARKERP (position) { marker_position (position) }
		   else { PT! () };
  if ! (BEGV! () <= pos && pos <= ZV! ())
    { args_out_of_range (Fcurrent_buffer (), position); }
  return pos;
}

DEFUN! ("current-active-maps", Fcurrent_active_maps, Scurrent_active_maps,
       0, 2, 0,
       doc: "Return a list of the currently active keymaps.\nOLP if non-nil indicates that we should obey `overriding-local-map' and\n`overriding-terminal-local-map'.  POSITION can specify a click position\nlike in the respective argument of `key-binding' or a live window which\nmeans to return the active maps for that window's buffer.",
       (olp: Lisp_Object, position: Lisp_Object),
       {
  let mut keymaps: Lisp_Object;
  let count = SPECPDL_INDEX ();

  keymaps = list1 (current_global_map);

  /* If a mouse click position is given, our variables are based on
     the buffer clicked on, not the current buffer.  So we may have to
     switch the buffer here.  */

  if CONSP (position)
    {
      let window = POSN_WINDOW (position);

      if WINDOWP (window)
	  && BUFFERP ((*XWINDOW (window)).contents)
	  && XBUFFER ((*XWINDOW (window)).contents) != (current_buffer! ())
	{
	  /* Arrange to go back to the original buffer once we're done
	     processing the key sequence.  We don't use
	     save_excursion_{save,restore} here, in analogy to
	     `read-key-sequence' to avoid saving point.  Maybe this
	     would not be a problem here, but it is easier to keep
	     things the same.
	  */
	  record_unwind_current_buffer ();
	  set_buffer_internal (XBUFFER ((*XWINDOW (window)).contents));
	}
    }
  else if WINDOW_LIVE_P (position)
    {
      if BUFFERP ((*XWINDOW (position)).contents)
	  && XBUFFER ((*XWINDOW (position)).contents) != (current_buffer! ())
	{
	  /* See comment above.  */
	  record_unwind_current_buffer ();
	  set_buffer_internal (XBUFFER ((*XWINDOW (position)).contents));
	}
    }

  if !NILP (olp)
      /* The doc said that overriding-terminal-local-map should
	 override overriding-local-map.  The code used them both,
	 but it seems clearer to use just one.  rms, jan 2005.  */
      && NILP ((*current_kboard).Voverriding_terminal_local_map_)
      && !NILP (Voverriding_local_map)
    { keymaps = Fcons (Voverriding_local_map, keymaps); }

  if NILP (XCDR (keymaps))
    {
      /* C declares `Lisp_Object *maps;` uninitialised; it is an out parameter, so the
	 port zero-initialises it (PORTING.md 1.5).  */
      let mut maps: *mut Lisp_Object = std::ptr::null_mut ();
      let mut nmaps: c_int;
      let pt = click_position (position);
      /* This usually returns the buffer's local map,
	 but that can be overridden by a `local-map' property.  */
      let mut local_map = get_local_map (pt, (current_buffer! ()), Qlocal_map ());
      /* This returns nil unless there is a `keymap' property.  */
      let mut keymap = get_local_map (pt, (current_buffer! ()), Qkeymap ());
      let otlp = (*current_kboard).Voverriding_terminal_local_map_;

      if CONSP (position)
	{
	  let mut string = POSN_STRING (position);

	  /* For a mouse click, get the local text-property keymap
	     of the place clicked on, rather than point.  */

	  local_map = Qnil ();
	  keymap = Qnil ();

	  /* If on a mode line string with a local keymap, or for a
	     click on a string, i.e. overlay string or a string
	     displayed via the `display' property, first consider the
	     `local-map' and `keymap' properties of that string.  */

	  if CONSP (string) && STRINGP (XCAR (string))
	    {
	      let pos = XCDR (string);
	      string = XCAR (string);
	      if FIXNUMP (pos)
		  && XFIXNUM (pos) >= 0
		  && XFIXNUM (pos) < SCHARS (string)
		{
		  local_map = Fget_text_property (pos, Qlocal_map (), string);
		  keymap = Fget_text_property (pos, Qkeymap (), string);
		}
	    }

	  let buffer_posn = Fposn_point (position);
	  /* Then, if the click was in the buffer, get the local
	     text-property keymap of the place clicked on.  */

	  if FIXNUMP (buffer_posn)
	      && XFIXNUM (buffer_posn) >= BEG as EMACS_INT
	      && XFIXNUM (buffer_posn) <= Z! ()
	    {
	      /* The properties in POSN_STRING take precedence, if set. */
	      if NILP (local_map)
		{ local_map = get_local_map (XFIXNUM (buffer_posn),
					   (current_buffer! ()), Qlocal_map ()); }

	      if NILP (keymap)
		{ keymap = get_local_map (XFIXNUM (buffer_posn),
					(current_buffer! ()), Qkeymap ()); }
	    }

	  /* Finally, fall back on the buffer's local map. */
	  if NILP (local_map)
	    { local_map = BVAR! ((current_buffer! ()), keymap_); }
	}

      if !NILP (local_map)
	{ keymaps = Fcons (local_map, keymaps); }

      /* Now put all the minor mode keymaps on the list.  */
      nmaps = current_minor_maps (std::ptr::null_mut (), &mut maps) as c_int;

      nmaps -= 1;
      let mut i = nmaps;
      while i >= 0
	{
	  if !NILP (*maps.offset (i as isize))
	    { keymaps = Fcons (*maps.offset (i as isize), keymaps); }
	  i -= 1;
	}

      if !NILP (keymap)
	{ keymaps = Fcons (keymap, keymaps); }

      if !NILP (olp) && !NILP (otlp)
	{ keymaps = Fcons (otlp, keymaps); }
    }

  return unbind_to (count, keymaps);
});

/* GC is possible in this function if it autoloads a keymap.  */

DEFUN! ("key-binding", Fkey_binding, Skey_binding, 1, 4, 0,
       doc: "Return the binding for command KEY in current keymaps.\nKEY is a string or vector, a sequence of keystrokes.\nThe binding is probably a symbol with a function definition.\n\nNormally, `key-binding' ignores bindings for t, which act as default\nbindings, used when nothing else in the keymap applies; this makes it\nusable as a general function for probing keymaps.  However, if the\noptional second argument ACCEPT-DEFAULT is non-nil, `key-binding' does\nrecognize the default bindings, just as `read-key-sequence' does.\n\nLike the normal command loop, `key-binding' will remap the command\nresulting from looking up KEY by looking up the command in the\ncurrent keymaps.  However, if the optional third argument NO-REMAP\nis non-nil, `key-binding' returns the unmapped command.\n\nIf KEY is a key sequence initiated with the mouse, the used keymaps\nwill depend on the clicked mouse position with regard to the buffer\nand possible local keymaps on strings.\n\nIf the optional argument POSITION is non-nil, it specifies a mouse\nposition as returned by `event-start' and `event-end', and the lookup\noccurs in the keymaps associated with it instead of KEY.  It can also\nbe a number or marker, in which case the keymap properties at the\nspecified buffer position instead of point are used.",
       (key: Lisp_Object, accept_default: Lisp_Object, no_remap: Lisp_Object, position: Lisp_Object),
       {
  let mut position = position;
  if NILP (position) && VECTORP (key)
    {
      if ASIZE (key) == 0
	{ return Qnil (); }

      /* mouse events may have a symbolic prefix indicating the
	 scrollbar or mode line */
      let event
	= AREF (key, if SYMBOLP (AREF (key, 0)) && ASIZE (key) > 1 { 1 } else { 0 });

      /* We are not interested in locations without event data */

      if EVENT_HAS_PARAMETERS (event) && CONSP (XCDR (event))
	{
	  let kind = EVENT_HEAD_KIND (EVENT_HEAD (event));
	  if EQ (kind, Qmouse_click ())
	    { position = EVENT_START (event); }
	}
    }

  let mut value = Flookup_key (Fcurrent_active_maps (Qt (), position),
			   key, accept_default);

  if NILP (value) || FIXNUMP (value)
    { return Qnil (); }

  /* If the result of the ordinary keymap lookup is an interactive
     command, look for a key binding (ie. remapping) for that command.  */

  if NILP (no_remap) && SYMBOLP (value)
    {
      let value1: Lisp_Object;
      if { value1 = Fcommand_remapping (value, position, Qnil ()); !NILP (value1) }
	{ value = value1; }
    }

  return value;
});

/* GC is possible in this function if it autoloads a keymap.  */

DEFUN! ("minor-mode-key-binding", Fminor_mode_key_binding, Sminor_mode_key_binding, 1, 2, 0,
       doc: "Find the visible minor mode bindings of KEY.\nReturn an alist of pairs (MODENAME . BINDING), where MODENAME is\nthe symbol which names the minor mode binding KEY, and BINDING is\nKEY's definition in that mode.  In particular, if KEY has no\nminor-mode bindings, return nil.  If the first binding is a\nnon-prefix, all subsequent bindings will be omitted, since they would\nbe ignored.  Similarly, the list doesn't include non-prefix bindings\nthat come after prefix bindings.\n\nIf optional argument ACCEPT-DEFAULT is non-nil, recognize default\nbindings; see the description of `lookup-key' for more details about this.",
       (key: Lisp_Object, accept_default: Lisp_Object),
       {
  /* C declares `Lisp_Object *modes, *maps;` uninitialised; both are out parameters,
     so the port zero-initialises them (PORTING.md 1.5).  */
  let mut modes: *mut Lisp_Object = std::ptr::null_mut ();
  let mut maps: *mut Lisp_Object = std::ptr::null_mut ();
  let nmaps: c_int = current_minor_maps (&mut modes, &mut maps) as c_int;
  let mut binding = Qnil ();

  let mut j: c_int;
  let mut i: c_int;
  j = 0;
  i = 0;
  while i < nmaps
    {
      if !NILP (*maps.offset (i as isize))
	  && { binding = Flookup_key (*maps.offset (i as isize), key, accept_default); !NILP (binding) }
	  && !FIXNUMP (binding)
	{
	  if KEYMAPP! (binding)
	    { *maps.offset (j as isize) = Fcons (*modes.offset (i as isize), binding); j += 1; }
	  else if j == 0
	    { return list1 (Fcons (*modes.offset (i as isize), binding)); }
	}
      i += 1;
    }

  return Flist (j as ptrdiff_t, maps);
});

DEFUN! ("use-global-map", Fuse_global_map, Suse_global_map, 1, 1, 0,
       doc: "Select KEYMAP as the global keymap.",
       (keymap: Lisp_Object),
       {
  let keymap = get_keymap (keymap, true, true);
  current_global_map = keymap;

  return Qnil ();
});

DEFUN! ("use-local-map", Fuse_local_map, Suse_local_map, 1, 1, 0,
       doc: "Select KEYMAP as the local keymap.\nIf KEYMAP is nil, that means no local keymap.",
       (keymap: Lisp_Object),
       {
  let keymap = if !NILP (keymap) { get_keymap (keymap, true, true) } else { keymap };

  bset_keymap ((current_buffer! ()), keymap);

  return Qnil ();
});

DEFUN! ("current-local-map", Fcurrent_local_map, Scurrent_local_map, 0, 0, 0,
       doc: "Return current buffer's local keymap, or nil if it has none.\nNormally the local keymap is set by the major mode with `use-local-map'.",
       (),
       {
  return BVAR! ((current_buffer! ()), keymap_);
});

DEFUN! ("current-global-map", Fcurrent_global_map, Scurrent_global_map, 0, 0, 0,
       doc: "Return the current global keymap.",
       (),
       {
  return current_global_map;
});

DEFUN! ("current-minor-mode-maps", Fcurrent_minor_mode_maps, Scurrent_minor_mode_maps, 0, 0, 0,
       doc: "Return a list of keymaps for the minor modes of the current buffer.",
       (),
       {
  /* C declares `Lisp_Object *maps;` uninitialised; it is an out parameter, so the
     port zero-initialises it (PORTING.md 1.5).  */
  let mut maps: *mut Lisp_Object = std::ptr::null_mut ();
  let nmaps: c_int = current_minor_maps (std::ptr::null_mut (), &mut maps) as c_int;

  return Flist (nmaps as ptrdiff_t, maps);
});

/* Help functions for describing and documenting keymaps.		*/

/// C: `struct accessible_keymaps_data` (keymap.c 1940-1944).  clang measures the record on
/// the frozen target: `sizeof` 32, `alignof` 8, `maps`@0, `tail`@8, `thisseq`@16,
/// `is_metized`@24.
#[repr(C)]
pub struct accessible_keymaps_data {
  pub maps: Lisp_Object,
  pub tail: Lisp_Object,
  pub thisseq: Lisp_Object,
  /* Does the current sequence end in the meta-prefix-char?  */
  pub is_metized: bool,
}

const _: () = core::assert! (std::mem::size_of::<accessible_keymaps_data> () == 32);
const _: () = core::assert! (std::mem::align_of::<accessible_keymaps_data> () == 8);
const _: () = core::assert! (std::mem::offset_of! (accessible_keymaps_data, maps) == 0);
const _: () = core::assert! (std::mem::offset_of! (accessible_keymaps_data, tail) == 8);
const _: () = core::assert! (std::mem::offset_of! (accessible_keymaps_data, thisseq) == 16);
const _: () = core::assert! (std::mem::offset_of! (accessible_keymaps_data, is_metized) == 24);

unsafe extern "C-unwind" fn accessible_keymaps_1 (key: Lisp_Object, cmd: Lisp_Object, args: Lisp_Object, data: *mut c_void)
/* Use void * data to be compatible with map_keymap_function_t.  */
{
  let d = data as *mut accessible_keymaps_data; /* Cast! */
  let mut maps = (*d).maps;
  let tail = (*d).tail;
  let thisseq = (*d).thisseq;
  let is_metized = (*d).is_metized && FIXNUMP (key);
  let mut tem: Lisp_Object;

  let mut cmd = get_keymap (get_keyelt (cmd, false), false, false);
  if NILP (cmd)
    { return; }

  /* Look for and break cycles.  */
  while { tem = Frassq (cmd, maps); !NILP (tem) }
    {
      let prefix = XCAR (tem);
      let lim: ptrdiff_t = XFIXNUM (Flength (XCAR (tem)));
      if lim <= XFIXNUM (Flength (thisseq))
	{ /* This keymap was already seen with a smaller prefix.  */
	  let mut i: ptrdiff_t = 0;
	  while i < lim && EQ (Faref (prefix, make_fixnum (i)),
				Faref (thisseq, make_fixnum (i)))
	    { i += 1; }
	  if i >= lim
	    /* `prefix' is a prefix of `thisseq' => there's a cycle.  */
	    { return; }
	}
      /* This occurrence of `cmd' in `maps' does not correspond to a cycle,
	 but maybe `cmd' occurs again further down in `maps', so keep
	 looking.  */
      maps = XCDR (Fmemq (tem, maps));
    }

  /* If the last key in thisseq is meta-prefix-char,
     turn it into a meta-ized keystroke.  We know
     that the event we're about to append is an
     ascii keystroke since we're processing a
     keymap table.  */
  if is_metized
    {
      let meta_bit: c_int = meta_modifier;
      let last = make_fixnum (XFIXNUM (Flength (thisseq)) - 1);
      tem = Fcopy_sequence (thisseq);

      Faset (tem, last, make_fixnum (XFIXNUM (key) | meta_bit as EMACS_INT));

      /* This new sequence is the same length as
	 thisseq, so stick it in the list right
	 after this one.  */
      XSETCDR (tail,
	       Fcons (Fcons (tem, cmd), XCDR (tail)));
    }
  else
    {
      tem = append_key (thisseq, key);
      nconc2 (tail, list1 (Fcons (tem, cmd)));
    }
}

/* This function cannot GC.  */

DEFUN! ("accessible-keymaps", Faccessible_keymaps, Saccessible_keymaps,
       1, 2, 0,
       doc: "Find all keymaps accessible via prefix characters from KEYMAP.\nReturns a list of elements of the form (KEYS . MAP), where the sequence\nKEYS starting from KEYMAP gets you to MAP.  These elements are ordered\nso that the KEYS increase in length.  The first element is ([] . KEYMAP).\nAn optional argument PREFIX, if non-nil, should be a key sequence;\nthen the value includes only maps for prefixes that start with PREFIX.",
       (keymap: Lisp_Object, prefix: Lisp_Object),
       {
  let mut prefix = prefix;
  let mut maps: Lisp_Object;
  let mut tail: Lisp_Object;
  let prefixlen: EMACS_INT = XFIXNAT (Flength (prefix));

  if !NILP (prefix)
    {
      /* If a prefix was specified, start with the keymap (if any) for
	 that prefix, so we don't waste time considering other prefixes.  */
      let mut tem = Flookup_key (keymap, prefix, Qt ());
      /* Flookup_key may give us nil, or a number,
	 if the prefix is not defined in this particular map.
	 It might even give us a list that isn't a keymap.  */
      tem = get_keymap (tem, false, false);
      /* If the keymap is autoloaded `tem' is not a cons-cell, but we still
	 want to return it.  */
      if !NILP (tem)
	{
	  /* Convert PREFIX to a vector now, so that later on
	     we don't have to deal with the possibility of a string.  */
	  if STRINGP (prefix)
	    {
	      let mut i_byte: ptrdiff_t = 0;
	      let copy = make_nil_vector (SCHARS (prefix));
	      let mut i: ptrdiff_t = 0;
	      while i < SCHARS (prefix)
		{
		  let i_before = i;
		  let mut c = fetch_string_char_advance (prefix, &mut i, &mut i_byte);
		  if SINGLE_BYTE_CHAR_P (c as intmax_t) && (c & 0o200) != 0
		    { c ^= 0o200 | meta_modifier; }
		  ASET (copy, i_before, make_fixnum (c as EMACS_INT));
		}
	      prefix = copy;
	    }
	  maps = list1 (Fcons (prefix, tem));
	}
      else
	{ return Qnil (); }
    }
  else
    { maps = list1 (Fcons (zero_vector, get_keymap (keymap, true, false))); }

  /* For each map in the list maps,
     look at any other maps it points to,
     and stick them at the end if they are not already in the list.

     This is a breadth-first traversal, where tail is the queue of
     nodes, and maps accumulates a list of all nodes visited.  */

  tail = maps;
  while CONSP (tail)
    {
      /* C declares `struct accessible_keymaps_data data;` and then assigns all four
	 fields; the port zero-initialises it first (PORTING.md 1.5/0.1-13).  */
      let mut data: accessible_keymaps_data = std::mem::zeroed ();
      let thismap = Fcdr (XCAR (tail));
      let last: Lisp_Object;

      data.thisseq = Fcar (XCAR (tail));
      data.maps = maps;
      data.tail = tail;
      last = make_fixnum (XFIXNUM (Flength (data.thisseq)) - 1);
      /* Does the current sequence end in the meta-prefix-char?  */
      data.is_metized = (XFIXNUM (last) >= 0
		    /* Don't metize the last char of PREFIX.  */
		    && XFIXNUM (last) >= prefixlen
		    && EQ (Faref (data.thisseq, last), meta_prefix_char));

      /* Since we can't run lisp code, we can't scan autoloaded maps.  */
      if CONSP (thismap)
	{ map_keymap (thismap, Some (accessible_keymaps_1), Qnil (), &raw mut data as *mut c_void, false); }
      tail = XCDR (tail);
    }
  return maps;
});

/* This function cannot GC.  */

DEFUN! ("key-description", Fkey_description, Skey_description, 1, 2, 0,
       doc: "Return a pretty description of key-sequence KEYS.\nOptional arg PREFIX is the sequence of keys leading up to KEYS.\nFor example, [?\\C-x ?l] is converted into the string \"C-x l\".\n\nFor an approximate inverse of this, see `kbd'.",
       (keys: Lisp_Object, prefix: Lisp_Object),
       {
  let mut len: ptrdiff_t = 0;
  let args: *mut Lisp_Object;
  let nkeys: EMACS_INT = XFIXNUM (Flength (keys));
  let nprefix: EMACS_INT = XFIXNUM (Flength (prefix));
  let sep = build_string (c" ".as_ptr ());
  let mut add_meta = false;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* This has one extra element at the end that we don't pass to Fconcat.  */
  let (size4_prod, size4_overflowed) = (nkeys + nprefix).overflowing_mul (4);
  let size4 = size4_prod;
  if size4_overflowed
    { memory_full_up (); }
  SAFE_ALLOCA_LISP! (sa_avail, args, size4);

  /* In effect, this computes
     (mapconcat 'single-key-description keys " ")
     but we shouldn't use mapconcat because it can do GC.  */

  let lists: [Lisp_Object; 2] = [ prefix, keys ];
  let listlens: [ptrdiff_t; 2] = [ nprefix, nkeys ];
  let mut li: c_int = 0;
  while (li as usize) < countof! (lists)
    {
      let mut list = lists[li as usize];
      let listlen = listlens[li as usize];
      let mut i_byte: ptrdiff_t = 0;

      if ! (NILP (list) || STRINGP (list) || VECTORP (list) || CONSP (list))
	{ wrong_type_argument (Qarrayp (), list); }

      let mut i: ptrdiff_t = 0;
      while i < listlen
	{
	  let mut key: Lisp_Object;
	  if STRINGP (list)
	    {
	      let mut c = fetch_string_char_advance (list, &mut i, &mut i_byte);
	      if !STRING_MULTIBYTE (list) && (c & 0o200) != 0
		{ c ^= 0o200 | meta_modifier; }
	      key = make_fixnum (c as EMACS_INT);
	    }
	  else if VECTORP (list)
	    {
	      key = AREF (list, i);
	      i += 1;
	    }
	  else
	    {
	      key = XCAR (list);
	      list = XCDR (list);
	      i += 1;
	    }

	  if add_meta
	    {
	      if !FIXNUMP (key)
		  || EQ (key, meta_prefix_char)
		  || (XFIXNUM (key) & meta_modifier as EMACS_INT) != 0
		{
		  *args.offset (len) = Fsingle_key_description (meta_prefix_char,
							 Qnil ());
		  len += 1;
		  *args.offset (len) = sep;
		  len += 1;
		  if EQ (key, meta_prefix_char)
		    { continue; }
		}
	      else
		{ key = make_fixnum (XFIXNUM (key) | meta_modifier as EMACS_INT); }
	      add_meta = false;
	    }
	  else if EQ (key, meta_prefix_char)
	    {
	      add_meta = true;
	      continue;
	    }
	  *args.offset (len) = Fsingle_key_description (key, Qnil ());
	  len += 1;
	  *args.offset (len) = sep;
	  len += 1;
	}
      li += 1;
    }

  let result: Lisp_Object;
  if add_meta
    {
      *args.offset (len) = Fsingle_key_description (meta_prefix_char, Qnil ());
      result = Fconcat (len + 1, args);
    }
  else if len == 0
    { result = empty_unibyte_string; }
  else
    { result = Fconcat (len - 1, args); }
  SAFE_FREE! (sa_count, sa_mark);
  return result;
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn push_key_description (ch: EMACS_INT, p: *mut c_char) -> *mut c_char
{
  let mut p = p;
  let mut c: c_int;
  let mut c2: c_int;
  let mut tab_as_ci: bool;

  /* Clear all the meaningless bits above the meta bit.  */
  c = (ch & (meta_modifier | ! - meta_modifier) as EMACS_INT) as c_int;
  c2 = c & !(alt_modifier | ctrl_modifier | hyper_modifier
	     | meta_modifier | shift_modifier | super_modifier);

  if ! CHARACTERP (make_fixnum (c2 as EMACS_INT))
    {
      /* KEY_DESCRIPTION_SIZE is large enough for this.  */
      p = p.add (sprintf (p, c"[%d]".as_ptr (), c) as usize);
      return p;
    }

  tab_as_ci = c2 == b'\t' as c_int && (c & meta_modifier) != 0;

  if (c & alt_modifier) != 0
    {
      *p = b'A' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (1);
      c -= alt_modifier;
    }
  if (c & ctrl_modifier) != 0
      || (c2 < b' ' as c_int && c2 != 27 && c2 != b'\t' as c_int && c2 != Ctl! (b'M' as c_int))
      || tab_as_ci
    {
      *p = b'C' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (1);
      c &= !ctrl_modifier;
    }
  if (c & hyper_modifier) != 0
    {
      *p = b'H' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (1);
      c -= hyper_modifier;
    }
  if (c & meta_modifier) != 0
    {
      *p = b'M' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (1);
      c -= meta_modifier;
    }
  if (c & shift_modifier) != 0
    {
      *p = b'S' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (1);
      c -= shift_modifier;
    }
  if (c & super_modifier) != 0
    {
      *p = b's' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (1);
      c -= super_modifier;
    }
  if c < 0o40
    {
      if c == 0o33
	{
	  *p = b'E' as c_char; p = p.add (1);
	  *p = b'S' as c_char; p = p.add (1);
	  *p = b'C' as c_char; p = p.add (1);
	}
      else if tab_as_ci
	{
	  *p = b'i' as c_char; p = p.add (1);
	}
      else if c == b'\t' as c_int
	{
	  *p = b'T' as c_char; p = p.add (1);
	  *p = b'A' as c_char; p = p.add (1);
	  *p = b'B' as c_char; p = p.add (1);
	}
      else if c == Ctl! (b'M' as c_int)
	{
	  *p = b'R' as c_char; p = p.add (1);
	  *p = b'E' as c_char; p = p.add (1);
	  *p = b'T' as c_char; p = p.add (1);
	}
      else
	{
	  /* `C-' already added above.  */
	  if c > 0 && c <= Ctl! (b'Z' as c_int)
	    { *p = (c + 0o140) as c_char; p = p.add (1); }
	  else
	    { *p = (c + 0o100) as c_char; p = p.add (1); }
	}
    }
  else if c == 0o177
    {
      *p = b'D' as c_char; p = p.add (1);
      *p = b'E' as c_char; p = p.add (1);
      *p = b'L' as c_char; p = p.add (1);
    }
  else if c == b' ' as c_int
   {
      *p = b'S' as c_char; p = p.add (1);
      *p = b'P' as c_char; p = p.add (1);
      *p = b'C' as c_char; p = p.add (1);
    }
  else if c < 128
    { *p = c as c_char; p = p.add (1); }
  else
    {
      /* Now we are sure that C is a valid character code.  */
      p = p.add (CHAR_STRING (c, p as *mut u8) as usize);
    }

  return p;
}

/* This function cannot GC.  */

DEFUN! ("single-key-description", Fsingle_key_description,
       Ssingle_key_description, 1, 2, 0,
       doc: "Return a pretty description of a character event KEY.\nControl characters turn into C-whatever, etc.\nOptional argument NO-ANGLES non-nil means don't put angle brackets\naround function keys and event symbols.\n\nSee `text-char-description' for describing character codes.",
       (key: Lisp_Object, no_angles: Lisp_Object),
       {
  let mut key = key;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  if CONSP (key) && lucid_event_type_list_p (key)
    { key = Fevent_convert_list (key); }

  if CONSP (key) && FIXNUMP (XCAR (key)) && FIXNUMP (XCDR (key))
    /* An interval from a map-char-table.  */
    {
      AUTO_STRING! (dot_dot, c"..".as_ptr ());
      return concat3 (Fsingle_key_description (XCAR (key), no_angles),
		      dot_dot,
		      Fsingle_key_description (XCDR (key), no_angles));
    }

  key = EVENT_HEAD (key);

  if FIXNUMP (key)		/* Normal character.  */
    {
      let mut tem: [c_char; KEY_DESCRIPTION_SIZE as usize] = [0; KEY_DESCRIPTION_SIZE as usize];
      let p = push_key_description (XFIXNUM (key), tem.as_mut_ptr ());
      *p = 0;
      return make_specified_string (tem.as_ptr (), -1, p.offset_from (tem.as_ptr ()), true);
    }
  else if SYMBOLP (key)	/* Function key or event-symbol.  */
    {
      if NILP (no_angles)
	{
	  let namestr = SYMBOL_NAME (key);
	  let sym = SSDATA (namestr);
	  let len = SBYTES (namestr);
	  /* Find the extent of the modifier prefix, like "C-M-". */
	  let mut i: c_int = 0;
	  while (i as ptrdiff_t) < len - 3 && *sym.offset ((i + 1) as isize) == b'-' as c_char && !strchr (c"CMSsHA".as_ptr (), *sym.offset (i as isize) as c_int).is_null ()
	    { i += 2; }
	  /* First I bytes of SYM are modifiers; put <> around the rest. */
	  let buffer = SAFE_ALLOCA! (sa_avail, (len + 3) as usize) as *mut c_char;
	  memcpy (buffer as *mut c_void, sym as *const c_void, i as usize);
	  *buffer.offset (i as isize) = b'<' as c_char;
	  memcpy (buffer.offset (i as isize + 1) as *mut c_void, sym.offset (i as isize) as *const c_void, (len - i as ptrdiff_t) as usize);
	  *buffer.offset (len as isize + 1) = b'>' as c_char;
	  *buffer.offset (len as isize + 2) = b'\0' as c_char;
	  let result = build_string (buffer);
	  SAFE_FREE! (sa_count, sa_mark);
	  return result;
	}
      else
	{ return Fsymbol_name (key); }
    }
  else if STRINGP (key)	/* Buffer names in the menubar.  */
    { return Fcopy_sequence (key); }
  else
    { error (c"KEY must be an integer, cons, symbol, or string".as_ptr ()); }
});

unsafe fn push_text_char_description (c: c_uint, p: *mut c_char) -> *mut c_char
{
  let mut p = p;
  if c < 0o40
    {
      *p = b'^' as c_char; p = p.add (1);
      *p = (c + 64) as c_char; p = p.add (1);		/* 'A' - 1 */
    }
  else if c == 0o177
    {
      *p = b'^' as c_char; p = p.add (1);
      *p = b'?' as c_char; p = p.add (1);
    }
  else
    { *p = c as c_char; p = p.add (1); }
  return p;
}

/* This function cannot GC.  */

DEFUN! ("text-char-description", Ftext_char_description, Stext_char_description, 1, 1, 0,
       doc: "Return the description of CHARACTER in standard Emacs notation.\nCHARACTER must be a valid character code that passes the `characterp' test.\nControl characters turn into \"^char\", and characters with Meta and other\nmodifiers signal an error, as they are not valid character codes.\nThis differs from `single-key-description' which accepts character events,\nand thus doesn't enforce the `characterp' condition, turns control\ncharacters into \"C-char\", and uses the 2**27 bit for Meta.\nSee Info node `(elisp)Describing Characters' for examples.",
       (character: Lisp_Object),
       {
  CHECK_CHARACTER (character);

  let c = XFIXNUM (character) as c_int;
  if !ASCII_CHAR_P (c as isize)
    {
      let mut str_: [c_char; MAX_MULTIBYTE_LENGTH as usize] = [0; MAX_MULTIBYTE_LENGTH as usize];
      let len = CHAR_STRING (c, str_.as_mut_ptr () as *mut u8);

      return make_multibyte_string (str_.as_ptr (), 1, len as ptrdiff_t);
    }
  else
    {
      let mut desc: [c_char; 4] = [0; 4];
      let len = push_text_char_description (c as c_uint, desc.as_mut_ptr ()).offset_from (desc.as_ptr ());
      return make_string (desc.as_ptr (), len);
    }
});

static mut where_is_preferred_modifier: c_int = 0;

/* Return 0 if SEQ uses non-preferred modifiers or non-char events.
   Else, return 2 if SEQ uses the where_is_preferred_modifier,
   and 1 otherwise.  */
unsafe fn preferred_sequence_p (seq: Lisp_Object) -> c_int
{
  let mut i: EMACS_INT;
  let len: EMACS_INT = XFIXNAT (Flength (seq));
  let mut result: c_int = 1;

  i = 0;
  while i < len
    {
      let mut ii: Lisp_Object;
      let elt: Lisp_Object;

      XSETFASTINT! (ii, i);
      elt = Faref (seq, ii);

      if !FIXNUMP (elt)
	{ return 0; }
      else
	{
	  let modifiers = (XFIXNUM (elt) & ((CHAR_MODIFIER_MASK & !CHAR_META) as EMACS_INT)) as c_int;
	  if modifiers == where_is_preferred_modifier
	    { result = 2; }
	  else if modifiers != 0
	    { return 0; }
	}
      i += 1;
    }

  return result;
}

/* where-is - finding a command in a set of keymaps.			*/

// C 2417's `static void where_is_internal_1 (Lisp_Object key, Lisp_Object binding,
// Lisp_Object args, void *data);` is a forward declaration of this unit's own helper and
// is dropped (PORTING.md 0.1-12).

/* Like Flookup_key, but with command remapping; just returns nil
   if the key sequence is too long.  */

unsafe fn shadow_lookup (keymap: Lisp_Object, key: Lisp_Object, accept_default: Lisp_Object,
	       remap: bool) -> Lisp_Object
{
  let value = Flookup_key (keymap, key, accept_default);

  if FIXNATP (value)          /* `key' is too long!  */
    { return Qnil (); }
  else if !NILP (value) && remap && SYMBOLP (value)
    {
      let remapping = Fcommand_remapping (value, Qnil (), keymap);
      return if !NILP (remapping) { remapping } else { value };
    }
  else
    { return value; }
}

static mut Vmouse_events: Lisp_Object = lisp_h_Qnil ();

/// C: `struct where_is_internal_data` (keymap.c 2482-2486).  clang measures the record on
/// the frozen target: `sizeof` 40, `alignof` 8, `definition`@0, `this`@8, `last`@16,
/// `last_is_meta`@24, `noindirect`@25, `sequences`@32.
#[repr(C)]
pub struct where_is_internal_data {
  pub definition: Lisp_Object,
  pub this: Lisp_Object,
  pub last: Lisp_Object,
  pub last_is_meta: bool,
  pub noindirect: bool,
  pub sequences: Lisp_Object,
}

const _: () = core::assert! (std::mem::size_of::<where_is_internal_data> () == 40);
const _: () = core::assert! (std::mem::align_of::<where_is_internal_data> () == 8);
const _: () = core::assert! (std::mem::offset_of! (where_is_internal_data, definition) == 0);
const _: () = core::assert! (std::mem::offset_of! (where_is_internal_data, this) == 8);
const _: () = core::assert! (std::mem::offset_of! (where_is_internal_data, last) == 16);
const _: () = core::assert! (std::mem::offset_of! (where_is_internal_data, last_is_meta) == 24);
const _: () = core::assert! (std::mem::offset_of! (where_is_internal_data, noindirect) == 25);
const _: () = core::assert! (std::mem::offset_of! (where_is_internal_data, sequences) == 32);

/* This function can't GC, AFAIK.  */
/* Return the list of bindings found.  This list is ordered "longest
   to shortest".  It may include bindings that are actually shadowed
   by others, as well as duplicate bindings and remapping bindings.
   The list returned is potentially shared with where_is_cache, so
   be careful not to modify it via side-effects.  */

unsafe fn where_is_internal (definition: Lisp_Object, keymaps: Lisp_Object,
		   noindirect: bool, nomenus: bool) -> Lisp_Object
{
  let mut maps = Qnil ();
  /* C declares `struct where_is_internal_data data;` and assigns its fields later; the
     port zero-initialises it first (PORTING.md 1.5/0.1-13).  */
  let mut data: where_is_internal_data = std::mem::zeroed ();

  /* Only important use of caching is for the menubar
     (i.e. where-is-internal called with (def nil t nil nil)).  */
  if nomenus && !noindirect
    {
      /* Check heuristic-consistency of the cache.  */
      if NILP (Fequal (keymaps, where_is_cache_keymaps))
	{ where_is_cache = Qnil (); }

      if NILP (where_is_cache)
	{
	  /* We need to create the cache.  */
	  where_is_cache = Fmake_hash_table (0, std::ptr::null_mut ());
	  where_is_cache_keymaps = Qt ();
	}
      else
	/* We can reuse the cache.  */
	{ return Fgethash (definition, where_is_cache, Qnil ()); }
    }
  else
    /* Kill the cache so that where_is_internal_1 doesn't think
       we're filling it up.  */
    { where_is_cache = Qnil (); }

  let mut found = keymaps;
  while CONSP (found)
    {
      maps =
	nconc2 (maps,
		Faccessible_keymaps (get_keymap (XCAR (found), true, false), Qnil ()));
      found = XCDR (found);
    }

  data.sequences = Qnil ();
  while CONSP (maps)
    {
      /* Key sequence to reach map, and the map that it reaches */
      let this: Lisp_Object;
      let map: Lisp_Object;
      let mut tem: Lisp_Object;

      /* In order to fold [META-PREFIX-CHAR CHAR] sequences into
	 [M-CHAR] sequences, check if last character of the sequence
	 is the meta-prefix char.  */
      let last: Lisp_Object;
      let last_is_meta: bool;

      this = Fcar (XCAR (maps));
      map  = Fcdr (XCAR (maps));
      last = make_fixnum (XFIXNUM (Flength (this)) - 1);
      last_is_meta = (XFIXNUM (last) >= 0
		      && EQ (Faref (this, last), meta_prefix_char));

      /* if (nomenus && !preferred_sequence_p (this)) */
      if nomenus && XFIXNUM (last) >= 0
	  && SYMBOLP ({ tem = Faref (this, make_fixnum (0)); tem })
	  && !NILP (Fmemq (XCAR (parse_modifiers (tem)), Vmouse_events))
	/* If no menu entries should be returned, skip over the
	   keymaps bound to `menu-bar' and `tool-bar' and other
	   non-ascii prefixes like `C-down-mouse-2'.  */
	{ maps = XCDR (maps); continue; }

      maybe_quit ();

      data.definition = definition;
      data.noindirect = noindirect;
      data.this = this;
      data.last = last;
      data.last_is_meta = last_is_meta;

      if CONSP (map)
	{ map_keymap (map, Some (where_is_internal_1), Qnil (), &raw mut data as *mut c_void, false); }
      maps = XCDR (maps);
    }

  if nomenus && !noindirect
    { /* Remember for which keymaps this cache was built.
	 We do it here (late) because we want to keep where_is_cache_keymaps
	 set to t while the cache isn't fully filled.  */
      where_is_cache_keymaps = keymaps;
      /* During cache-filling, data.sequences is not filled by
	 where_is_internal_1.  */
      return Fgethash (definition, where_is_cache, Qnil ());
    }
  else
    { return data.sequences; }
}

/* This function can GC if Flookup_key autoloads any keymaps.  */

DEFUN! ("where-is-internal", Fwhere_is_internal, Swhere_is_internal, 1, 5, 0,
       doc: "Return list of keys that invoke DEFINITION.\nIf KEYMAP is a keymap, search only KEYMAP and the global keymap.\nIf KEYMAP is nil, search all the currently active keymaps, except\n for `overriding-local-map' (which is ignored).\nIf KEYMAP is a list of keymaps, search only those keymaps.\n\nIf optional 3rd arg FIRSTONLY is non-nil, return the first key sequence found,\nrather than a list of all possible key sequences.\nIf FIRSTONLY is the symbol `non-ascii', return the first binding found,\nno matter what it is.\nIf FIRSTONLY has another non-nil value, prefer bindings\nthat use the modifier key specified in `where-is-preferred-modifier'\n(or their meta variants) and entirely reject menu bindings.\n\nIf optional 4th arg NOINDIRECT is non-nil, don't extract the commands inside\nmenu-items.  This makes it possible to search for a menu-item itself.\n\nThe optional 5th arg NO-REMAP alters how command remapping is handled:\n\n- If another command OTHER-COMMAND is remapped to DEFINITION, normally\n  search for the bindings of OTHER-COMMAND and include them in the\n  returned list.  But if NO-REMAP is non-nil, include the vector\n  [remap OTHER-COMMAND] in the returned list instead, without\n  searching for those other bindings.\n\n- If DEFINITION is remapped to OTHER-COMMAND, normally return the\n  bindings for OTHER-COMMAND.  But if NO-REMAP is non-nil, return the\n  bindings for DEFINITION instead, ignoring its remapping.\n\nKeys that are represented as events that have a `non-key-event' non-nil\nsymbol property are ignored.",
       (definition: Lisp_Object, keymap: Lisp_Object, firstonly: Lisp_Object, noindirect: Lisp_Object, no_remap: Lisp_Object),
       {
  let mut definition = definition;
  /* The keymaps in which to search.  */
  let keymaps: Lisp_Object;
  /* Potentially relevant bindings in "shortest to longest" order.  */
  let mut sequences = Qnil ();
    /* Actually relevant bindings.  */
  let mut found = Qnil ();
  /* 1 means ignore all menu bindings entirely.  */
  let nomenus = !NILP (firstonly) && !EQ (firstonly, Qnon_ascii ());
  /* List of sequences found via remapping.  Keep them in a separate
     variable, so as to push them later, since we prefer
     non-remapped binding.  */
  let mut remapped_sequences = Qnil ();
  /* Whether or not we're handling remapped sequences.  This is needed
     because remapping is not done recursively by Fcommand_remapping: you
     can't remap a remapped command.  */
  let mut remapped = false;

  /* Refresh the C version of the modifier preference.  */
  where_is_preferred_modifier
    = parse_solitary_modifier (Vwhere_is_preferred_modifier);

  /* Find the relevant keymaps.  */
  if CONSP (keymap) && KEYMAPP! (XCAR (keymap))
    { keymaps = keymap; }
  else if !NILP (keymap)
    { keymaps = list2 (keymap, current_global_map); }
  else
    { keymaps = Fcurrent_active_maps (Qnil (), Qnil ()); }

  let mut tem = Fcommand_remapping (definition, Qnil (), keymaps);
  /* If `definition' is remapped to `tem', then OT1H no key will run
     that command (since they will run `tem' instead), so we should
     return nil; but OTOH all keys bound to `definition' (or to `tem')
     will run the same command.
     So for menu-shortcut purposes, we want to find all the keys bound (maybe
     via remapping) to `tem'.  But for the purpose of finding the keys that
     run `definition', then we'd want to just return nil.
     We choose to make it work right for menu-shortcuts, since it's the most
     common use.
     Known bugs: if you remap switch-to-buffer to toto, C-h f switch-to-buffer
     will tell you that switch-to-buffer is bound to C-x b even though C-x b
     will run toto instead.  And if `toto' is itself remapped to forward-char,
     then C-h f toto will tell you that it's bound to C-f even though C-f does
     not run toto and it won't tell you that C-x b does run toto.  */
  if NILP (no_remap) && !NILP (tem)
    { definition = tem; }

  if SYMBOLP (definition)
      && !NILP (firstonly)
      && !NILP ({ tem = Fget (definition, QCadvertised_binding ()); tem })
    {
      /* We have a list of advertised bindings.  */
      /* FIXME: Not sure why we use false for shadow_lookup's remapping,
         nor why we use `EQ' here but `Fequal' in the call further down.  */
      while CONSP (tem)
	{
	  if EQ (shadow_lookup (keymaps, XCAR (tem), Qnil (), false), definition)
	    { return XCAR (tem); }
	  else
	    { tem = XCDR (tem); }
	}
      if EQ (shadow_lookup (keymaps, tem, Qnil (), false), definition)
	{ return tem; }
    }

  sequences = Freverse (where_is_internal (definition, keymaps,
					   !NILP (noindirect), nomenus));

  while CONSP (sequences)
	 /* If we're at the end of the `sequences' list and we haven't
	    considered remapped sequences yet, copy them over and
	    process them.  */
	 || (!remapped && { sequences = remapped_sequences;
			   remapped = true;
			   CONSP (sequences) })
    {
      let mut sequence: Lisp_Object;
      let mut function: Lisp_Object;

      sequence = XCAR (sequences);
      sequences = XCDR (sequences);

      /* Verify that this key binding is not shadowed by another
	 binding for the same key, before we say it exists.

	 Mechanism: look for local definition of this key and if
	 it is defined and does not match what we found then
	 ignore this key.

	 Either nil or number as value from Flookup_key
	 means undefined.  */
      if NILP (Fequal (shadow_lookup (keymaps, sequence, Qnil (), remapped),
			definition))
	{ continue; }

      /* If the current sequence is a command remapping with
	 format [remap COMMAND], find the key sequences
	 which run COMMAND, and use those sequences instead.  */
      if NILP (no_remap) && !remapped
	  && VECTORP (sequence) && ASIZE (sequence) == 2
	  && EQ (AREF (sequence, 0), Qremap ())
	  && { function = AREF (sequence, 1); SYMBOLP (function) }
	{
	  let seqs = where_is_internal (function, keymaps,
						!NILP (noindirect), nomenus);
	  remapped_sequences = nconc2 (Freverse (seqs), remapped_sequences);
	  continue;
	}

      /* Don't annoy user with strings from a menu such as the
	 entries from the "Edit => Paste from Kill Menu".
	 Change them all to "(any string)", so that there
	 seems to be only one menu item to report.  */
      if ! NILP (sequence)
	{
	  let tem1: Lisp_Object;
	  tem1 = Faref (sequence, make_fixnum (ASIZE (sequence) - 1));
	  if STRINGP (tem1)
	    { Faset (sequence, make_fixnum (ASIZE (sequence) - 1),
		   build_string (c"(any string)".as_ptr ())); }
	}

      /* It is a true unshadowed match.  Record it, unless it's already
	 been seen (as could happen when inheriting keymaps).  */
      if NILP (Fmember (sequence, found))
	  /* Filter out non key events.  */
	  && !(VECTORP (sequence)
	       && ASIZE (sequence) == 1
	       && SYMBOLP (AREF (sequence, 0))
	       && !NILP (Fget (AREF (sequence, 0), Qnon_key_event ())))
	{ found = Fcons (sequence, found); }

      /* If firstonly is Qnon_ascii, then we can return the first
	 binding we find.  If firstonly is not Qnon_ascii but not
	 nil, then we should return the first ascii-only binding
	 we find.  */
      if EQ (firstonly, Qnon_ascii ())
	{ return sequence; }
      else if !NILP (firstonly)
	       && 2 == preferred_sequence_p (sequence)
	{ return sequence; }
    }

  found = Fnreverse (found);

  /* firstonly may have been t, but we may have gone all the way through
     the keymaps without finding an all-ASCII key sequence.  So just
     return the best we could find.  */
  if NILP (firstonly)
    { return found; }
  else if where_is_preferred_modifier == 0
    { return Fcar (found); }
  else
    { /* Maybe we did not find a preferred_modifier binding, but we did find
	 some ASCII binding.  */
      let mut bindings = found;
      while CONSP (bindings)
	{
	  if preferred_sequence_p (XCAR (bindings)) != 0
	    { return XCAR (bindings); }
	  else
	    { bindings = XCDR (bindings); }
	}
      return Fcar (found);
    }
});

/* This function can GC because get_keyelt can.  */

unsafe extern "C-unwind" fn where_is_internal_1 (key: Lisp_Object, binding: Lisp_Object, args: Lisp_Object, data: *mut c_void)
{
  let d = data as *mut where_is_internal_data; /* Cast! */
  let definition = (*d).definition;
  let noindirect = (*d).noindirect;
  let this = (*d).this;
  let last = (*d).last;
  let last_is_meta = (*d).last_is_meta;
  let sequence: Lisp_Object;

  let mut key = key;
  let mut binding = binding;

  /* Search through indirections unless that's not wanted.  */
  if !noindirect
    { binding = get_keyelt (binding, false); }

  /* End this iteration if this element does not match
     the target.  */

  if !(!NILP (where_is_cache)	/* everything "matches" during cache-fill.  */
	|| EQ (binding, definition)
	|| (CONSP (definition) && !NILP (Fequal (binding, definition))))
    /* Doesn't match.  */
    { return; }

  /* We have found a match.  Construct the key sequence where we found it.  */
  if FIXNUMP (key) && last_is_meta
    {
      sequence = Fcopy_sequence (this);
      Faset (sequence, last, make_fixnum (XFIXNUM (key) | meta_modifier as EMACS_INT));
    }
  else
    {
      if CONSP (key)
	{ key = Fcons (XCAR (key), XCDR (key)); }
      sequence = append_key (this, key);
    }

  if !NILP (where_is_cache)
    {
      let sequences = Fgethash (binding, where_is_cache, Qnil ());
      Fputhash (binding, Fcons (sequence, sequences), where_is_cache);
    }
  else
    { (*d).sequences = Fcons (sequence, (*d).sequences); }
}

/* describe-bindings - summarizing all the bindings in a set of keymaps.  */

DEFUN! ("describe-buffer-bindings", Fdescribe_buffer_bindings, Sdescribe_buffer_bindings, 1, 3, 0,
       doc: "Insert the list of all defined keys and their definitions.\nThe list is inserted in the current buffer, while the bindings are\nlooked up in BUFFER.\nThe optional argument PREFIX, if non-nil, should be a key sequence;\nthen we display only bindings that start with that prefix.\nThe optional argument MENUS, if non-nil, says to mention menu bindings.\n(Ordinarily these are omitted from the output.)",
       (buffer: Lisp_Object, prefix: Lisp_Object, menus: Lisp_Object),
       {
  let nomenu = if NILP (menus) { Qt () } else { Qnil () };

  /* C's literal is written with `\`+newline continuations (translation phase 2 splices
     them, keeping the continuation lines' own indentation - which is zero); the port keeps
     that exact shape, both because the value is the same and because PORTING.md 3.28
     records that the token gate cannot see past such a continuation on either side.  */
  let mut alternate_heading: *const c_char
    = c"\
Keyboard translations:\n\n\
You type        Translation\n\
--------        -----------\n".as_ptr ();

  CHECK_BUFFER (buffer);

  let mut shadow = Qnil ();
  let outbuf = Fcurrent_buffer ();

  /* Report on alternates for keys.  */
  if STRINGP ((*current_kboard).Vkeyboard_translate_table_) && !NILP (prefix)
    {
      let mut translate = SDATA ((*current_kboard).Vkeyboard_translate_table_);
      let translate_len = SCHARS ((*current_kboard).Vkeyboard_translate_table_) as c_int;

      let mut c: c_int = 0;
      while c < translate_len
	{
	  if *translate.offset (c as isize) as c_int != c
	    {
	      let mut buf: [c_char; KEY_DESCRIPTION_SIZE as usize] = [0; KEY_DESCRIPTION_SIZE as usize];
	      let mut bufend: *mut c_char;

	      if !alternate_heading.is_null ()
		{
		  insert_string (alternate_heading);
		  alternate_heading = std::ptr::null ();
		}

	      bufend = push_key_description (*translate.offset (c as isize) as EMACS_INT, buf.as_mut_ptr ());
	      insert (buf.as_ptr (), bufend.offset_from (buf.as_ptr ()));
	      Findent_to (make_fixnum (16), make_fixnum (1));
	      bufend = push_key_description (c as EMACS_INT, buf.as_mut_ptr ());
	      insert (buf.as_ptr (), bufend.offset_from (buf.as_ptr ()));

	      insert (c"\n".as_ptr (), 1);

	      /* Insert calls signal_after_change which may GC.  */
	      translate = SDATA ((*current_kboard).Vkeyboard_translate_table_);
	    }
	  c += 1;
	}

      insert (c"\n".as_ptr (), 1);
    }

  if !NILP (Vkey_translation_map)
    {
      let msg = build_unibyte_string (c"Key translations".as_ptr ());
      calln! (Qhelp__describe_map_tree (),
	     Vkey_translation_map, Qnil (), Qnil (), prefix,
	     msg, nomenu, Qt (), Qnil (), Qnil (), buffer);
    }

  /* Print the (major mode) local map.  */
  let mut start1 = (*current_kboard).Voverriding_terminal_local_map_;

  if !NILP (start1)
    {
      let msg = build_unibyte_string (c"\x0c\nOverriding Bindings".as_ptr ());
      calln! (Qhelp__describe_map_tree (),
	     start1, Qt (), shadow, prefix,
	     msg, nomenu, Qnil (), Qnil (), Qnil (), buffer);
      shadow = Fcons (start1, shadow);
      start1 = Qnil ();
    }
  else if !NILP (Voverriding_local_map)
    { start1 = Voverriding_local_map; }

  if !NILP (start1)
    {
      let msg = build_unibyte_string (c"\x0c\nOverriding Bindings".as_ptr ());
      calln! (Qhelp__describe_map_tree (),
	     start1, Qt (), shadow, prefix,
	     msg, nomenu, Qnil (), Qnil (), Qnil (), buffer);
      shadow = Fcons (start1, shadow);
    }
  else
    {
      /* Print the minor mode and major mode keymaps.  */
      let mut modes: *mut Lisp_Object = std::ptr::null_mut ();
      let mut maps: *mut Lisp_Object = std::ptr::null_mut ();

      /* Temporarily switch to `buffer', so that we can get that buffer's
	 minor modes correctly.  */
      Fset_buffer (buffer);

      let nmaps: c_int = current_minor_maps (&mut modes, &mut maps) as c_int;
      Fset_buffer (outbuf);

      start1 = get_local_map (BUF_PT (XBUFFER (buffer)),
			      XBUFFER (buffer), Qkeymap ());
      if !NILP (start1)
	{
	  let msg = build_unibyte_string (c"\x0c\n`keymap' Property Bindings".as_ptr ());
	  calln! (Qhelp__describe_map_tree (),
		 start1, Qt (), shadow, prefix,
		 msg, nomenu, Qnil (), Qnil (), Qnil (), buffer);
	  shadow = Fcons (start1, shadow);
	}

      /* Print the minor mode maps.  */
      let mut i: c_int = 0;
      while i < nmaps
	{
	  /* The title for a minor mode keymap
	     is constructed at run time.
	     We let `help--describe-map-tree' do the actual insertion
	     because it takes care of other features when doing so.  */
	  let title: *mut c_char;
	  let mut p: *mut c_char;

	  if !SYMBOLP (*modes.offset (i as isize))
	    { emacs_abort (); }

	  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
	  p = SAFE_ALLOCA! (sa_avail, (42 + SBYTES (SYMBOL_NAME (*modes.offset (i as isize)))) as usize) as *mut c_char;
	  title = p;
	  *p = b'\x0c' as c_char; p = p.add (1);
	  *p = b'\n' as c_char; p = p.add (1);
	  *p = b'`' as c_char; p = p.add (1);
	  memcpy (p as *mut c_void, SDATA (SYMBOL_NAME (*modes.offset (i as isize))) as *const c_void,
		  SBYTES (SYMBOL_NAME (*modes.offset (i as isize))) as usize);
	  p = p.offset (SBYTES (SYMBOL_NAME (*modes.offset (i as isize))));
	  *p = b'\'' as c_char; p = p.add (1);
	  memcpy (p as *mut c_void, c" Minor Mode Bindings".as_ptr () as *const c_void, strlen (c" Minor Mode Bindings".as_ptr ()));
	  p = p.offset (strlen (c" Minor Mode Bindings".as_ptr ()) as isize);
	  *p = 0;

	  let msg = build_unibyte_string (title);
	  calln! (Qhelp__describe_map_tree (),
		 *maps.offset (i as isize), Qt (), shadow, prefix,
		 msg, nomenu, Qnil (), Qnil (), Qnil (), buffer);
	  shadow = Fcons (*maps.offset (i as isize), shadow);
	  SAFE_FREE! (sa_count, sa_mark);
	  i += 1;
	}

      start1 = get_local_map (BUF_PT (XBUFFER (buffer)),
			      XBUFFER (buffer), Qlocal_map ());
      if !NILP (start1)
	{
	  if EQ (start1, BVAR! (XBUFFER (buffer), keymap_))
	    {
	      let msg =
		CALLN! (Fformat,
		       build_unibyte_string (c"\x0c\n`%s' Major Mode Bindings".as_ptr ()),
		       (*XBUFFER (buffer)).major_mode_);
	      calln! (Qhelp__describe_map_tree (),
		     start1, Qt (), shadow, prefix,
		     msg, nomenu, Qnil (), Qnil (), Qnil (), buffer);
	    }
	  else
	    {
	      let msg = build_unibyte_string (c"\x0c\n`local-map' Property Bindings".as_ptr ());
	      calln! (Qhelp__describe_map_tree (),
		     start1, Qt (), shadow, prefix,
		     msg, nomenu, Qnil (), Qnil (), Qnil (), buffer);
	    }

	  shadow = Fcons (start1, shadow);
	}
    }

  let msg = build_unibyte_string (c"\x0c\nGlobal Bindings".as_ptr ());
  calln! (Qhelp__describe_map_tree (),
	 current_global_map, Qt (), shadow, prefix,
	 msg, nomenu, Qnil (), Qt (), Qnil (), buffer);

  /* Print the function-key-map translations under this prefix.  */
  if !NILP ((*current_kboard).Vlocal_function_key_map_)
    {
      let msg = build_unibyte_string (c"\x0c\nFunction key map translations".as_ptr ());
      calln! (Qhelp__describe_map_tree (),
	     (*current_kboard).Vlocal_function_key_map_, Qnil (), Qnil (), prefix,
	     msg, nomenu, Qt (), Qnil (), Qnil (), buffer);
    }

  /* Print the input-decode-map translations under this prefix.  */
  if !NILP ((*current_kboard).Vinput_decode_map_)
    {
      let msg = build_unibyte_string (c"\x0c\nInput decoding map translations".as_ptr ());
      calln! (Qhelp__describe_map_tree (),
	     (*current_kboard).Vinput_decode_map_, Qnil (), Qnil (), prefix,
	     msg, nomenu, Qt (), Qnil (), Qnil (), buffer);
    }
  return Qnil ();
});

unsafe extern "C-unwind" fn describe_vector_princ (elt: Lisp_Object, fun: Lisp_Object)
{
  Findent_to (make_fixnum (16), make_fixnum (1));
  calln! (fun, elt);
  Fterpri (Qnil (), Qnil ());
}

unsafe extern "C-unwind" fn describe_vector_basic (elt: Lisp_Object, fun: Lisp_Object)
{
  calln! (fun, elt);
}

DEFUN! ("describe-vector", Fdescribe_vector, Sdescribe_vector, 1, 2, 0,
       doc: "Insert a description of contents of VECTOR.\nThis is text showing the elements of vector matched against indices.\nDESCRIBER is the output function used; nil means use `princ'.",
       (vector: Lisp_Object, describer: Lisp_Object),
       {
  let mut describer = describer;
  let count = SPECPDL_INDEX ();
  if NILP (describer)
    { describer = Qprinc (); }
  specbind (Qstandard_output (), Fcurrent_buffer ());
  CHECK_VECTOR_OR_CHAR_TABLE (vector);
  describe_vector (vector, Qnil (), describer, Some (describe_vector_princ), false,
		   Qnil (), Qnil (), false, false);

  return unbind_to (count, Qnil ());
});

static mut fontify_key_properties: Lisp_Object = lisp_h_Qnil ();

unsafe fn describe_key_maybe_fontify (str_: Lisp_Object, prefix: Lisp_Object,
				   keymap_p: bool) -> Lisp_Object
{
  let key_desc = Fkey_description (str_, prefix);
  if keymap_p
    { Fadd_text_properties (make_fixnum (0),
			  make_fixnum (SCHARS (key_desc)),
			  fontify_key_properties,
			  key_desc); }
  return key_desc;
}

DEFUN! ("help--describe-vector", Fhelp__describe_vector, Shelp__describe_vector, 7, 7, 0,
       doc: "Insert in the current buffer a description of the contents of VECTOR.\nCall DESCRIBER to insert the description of one value found in VECTOR.\n\nPREFIX is a string describing the key which leads to the keymap that\nthis vector is in.\n\nIf PARTIAL, it means do not mention suppressed commands.\n\nSHADOW is a list of keymaps that shadow this map.\nIf it is non-nil, look up the key in those maps and don't mention it\nif it is defined by any of them.\n\nENTIRE-MAP is the keymap in which this vector appears.\nIf the definition in effect in the whole map does not match\nthe one in this keymap, we ignore this one.",
       (vector: Lisp_Object, prefix: Lisp_Object, describer: Lisp_Object,
   partial: Lisp_Object, shadow: Lisp_Object, entire_map: Lisp_Object,
   mention_shadow: Lisp_Object),
       {
  let count = SPECPDL_INDEX ();
  specbind (Qstandard_output (), Fcurrent_buffer ());
  CHECK_VECTOR_OR_CHAR_TABLE (vector);

  let b_partial = if NILP (partial) { false } else { true };
  let b_mention_shadow = if NILP (mention_shadow) { false } else { true };

  describe_vector (vector, prefix, describer, Some (describe_vector_basic), b_partial,
		   shadow, entire_map, true, b_mention_shadow);
  return unbind_to (count, Qnil ());
});

/* Insert in the current buffer a description of the contents of VECTOR.
   Call ELT_DESCRIBER to insert the description of one value found
   in VECTOR.

   ELT_PREFIX describes what "comes before" the keys or indices defined
   by this vector.  This is a human-readable string whose size
   is not necessarily related to the situation.

   If the vector is in a keymap, ELT_PREFIX is a prefix key which
   leads to this keymap.

   If the vector is a chartable, ELT_PREFIX is the vector
   of bytes that lead to the character set or portion of a character
   set described by this chartable.

   If PARTIAL, it means do not mention suppressed commands
   (that assumes the vector is in a keymap).

   SHADOW is a list of keymaps that shadow this map.
   If it is non-nil, then we look up the key in those maps
   and we don't mention it now if it is defined by any of them.

   ENTIRE_MAP is the keymap in which this vector appears.
   If the definition in effect in the whole map does not match
   the one in this vector, we ignore this one.

   ARGS is simply passed as the second argument to ELT_DESCRIBER.

   KEYMAP_P is 1 if vector is known to be a keymap, so map ESC to M-.

   ARGS is simply passed as the second argument to ELT_DESCRIBER.  */

unsafe fn describe_vector (vector: Lisp_Object, prefix: Lisp_Object, args: Lisp_Object,
		 elt_describer: Option<unsafe extern "C-unwind" fn (Lisp_Object, Lisp_Object)>,
		 partial: bool, shadow: Lisp_Object, entire_map: Lisp_Object,
		 keymap_p: bool, mention_shadow: bool)
{
  let mut prefix = prefix;
  let mut elt_prefix = Qnil ();
  let mut suppress = Qnil ();
  let mut first = true;
  /* Range of elements to be handled.  */
  let mut to: c_int;
  let mut stop: c_int;

  if !keymap_p
    {
      if !NILP (prefix) && XFIXNAT (Flength (prefix)) > 0
	{
	  AUTO_STRING! (space, c" ".as_ptr ());
	  elt_prefix = concat2 (Fkey_description (prefix, Qnil ()), space);
	}
      prefix = Qnil ();
    }

  /* This vector gets used to present single keys to Flookup_key.  Since
     that is done once per vector element, we don't want to cons up a
     fresh vector every time.  */
  let kludge = make_nil_vector (1);

  if partial
    { suppress = Qsuppress_keymap (); }

  /* STOP is a boundary between normal characters (-#x3FFF7F) and
     8-bit characters (#x3FFF80-), used below when VECTOR is a
     char-table.  */
  if CHAR_TABLE_P (vector)
    { stop = MAX_5_BYTE_CHAR + 1; to = MAX_CHAR + 1; }
  else
    { stop = ASIZE (vector) as c_int; to = stop; }

  /* C's `for (int i = 0; ; i++)`: the increment runs on `continue` too, so each
     `continue` below carries it explicitly (PORTING.md 2.2).  */
  let mut i: c_int = 0;
  loop
    {
      let mut this_shadowed = false;
      let mut shadowed_by = Qnil ();
      /* C leaves RANGE_BEG uninitialised (only the char-table arm sets it); the port
	 zero-initialises it (PORTING.md 1.5).  */
      let mut range_beg: c_int = 0;
      let val: Lisp_Object;
      let mut tem2: Lisp_Object;

      maybe_quit ();

      if i == stop
	{
	  if i == to
	    { break; }
	  stop = to;
	}

      let starting_i = i;

      if CHAR_TABLE_P (vector)
	{
	  /* Find the value in VECTOR for the first character in the
	     range [RANGE_BEG..STOP), and update the range to include
	     only the characters whose value is the same as that of
	     the first in the range.  */
	  range_beg = i;
	  i = stop - 1;
	  val = char_table_ref_and_range (vector, range_beg, &mut range_beg, &mut i);
	}
      else
	{ val = AREF (vector, i as isize); }
      let definition = get_keyelt (val, false);

      if NILP (definition) { i += 1; continue; }

      /* Don't mention suppressed commands.  */
      if SYMBOLP (definition) && partial
	{
	  let tem = Fget (definition, suppress);

	  if !NILP (tem) { i += 1; continue; }
	}

      let character = make_fixnum (starting_i as EMACS_INT);
      ASET (kludge, 0, character);

      /* If this binding is shadowed by some other map, ignore it.  */
      if !NILP (shadow)
	{
	  shadowed_by = shadow_lookup (shadow, kludge, Qt (), false);

	  if !NILP (shadowed_by) && !EQ (shadowed_by, definition)
	    {
	      if mention_shadow
		{ this_shadowed = true; }
	      else
		{ i += 1; continue; }
	    }
	}

      /* Ignore this definition if it is shadowed by an earlier
	 one in the same keymap.  */
      if !NILP (entire_map)
	{
	  let tem = Flookup_key (entire_map, kludge, Qt ());

	  if !EQ (tem, definition)
	    { i += 1; continue; }
	}

      if first
	{
	  insert (c"\n".as_ptr (), 1);
	  first = false;
	}

      /* Output the prefix that applies to every entry in this map.  */
      if !NILP (elt_prefix)
	{ insert1 (elt_prefix); }

      insert1 (describe_key_maybe_fontify (kludge, prefix, keymap_p));

      /* Find all consecutive characters or rows that have the same
	 definition.  */
      if !CHAR_TABLE_P (vector)
	{
	  while i + 1 < stop
		 && { tem2 = get_keyelt (AREF (vector, (i + 1) as isize), false);
		     !NILP (tem2) }
		 && !NILP (Fequal (tem2, definition))
	    { i += 1; }
	}

      /* Make sure found consecutive keys are either not shadowed or,
	 if they are, that they are shadowed by the same command.  */
      if !NILP (Vdescribe_bindings_check_shadowing_in_ranges)
	  && CHAR_TABLE_P (vector) && i != starting_i
	  && (!EQ (Vdescribe_bindings_check_shadowing_in_ranges,
		   Qignore_self_insert ())
	      || !EQ (definition, Qself_insert_command ()))
	{
	  let key = make_nil_vector (1);
	  let mut j: c_int = range_beg + 1;
	  while j <= i
	    {
	      ASET (key, 0, make_fixnum (j as EMACS_INT));
	      let tem = shadow_lookup (shadow, key, Qt (), false);
	      if NILP (Fequal (tem, shadowed_by))
		{ i = j - 1; }
	      j += 1;
	    }
	}

      /* If we have a range of more than one character,
	 print where the range reaches to.  */

      if i != starting_i
	{
	  insert (c" .. ".as_ptr (), 4);

	  ASET (kludge, 0, make_fixnum (i as EMACS_INT));

	  if !NILP (elt_prefix)
	    { insert1 (elt_prefix); }

	  insert1 (describe_key_maybe_fontify (kludge, prefix, keymap_p));
	}

      /* Print a description of the definition of this character.
	 elt_describer will take care of spacing out far enough
	 for alignment purposes.  */
      elt_describer.unwrap () (definition, args);

      if this_shadowed
	{
	  SET_PT (PT! () - 1);
	  if SYMBOLP (shadowed_by)
	    {
	      /* C: `static char const fmt[] = "  (currently shadowed by `%s')";` - a
		 string constant, so the port writes it as a C string literal and takes
		 `sizeof fmt` as its byte count plus the terminator.  */
	      let fmt = c"  (currently shadowed by `%s')";
	      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
	      let buffer =
		SAFE_ALLOCA! (sa_avail, fmt.count_bytes () + 1 + SBYTES (SYMBOL_NAME (shadowed_by)) as usize) as *mut c_char;
	      esprintf (buffer, fmt.as_ptr (), SDATA (SYMBOL_NAME (shadowed_by)));
	      insert_string (buffer);
	      SAFE_FREE! (sa_count, sa_mark);
	    }
	  else	/* Could be a keymap, a lambda, or a keyboard macro.  */
	    { insert_string (c"  (currently shadowed)".as_ptr ()); }
	  SET_PT (PT! () + 1);
	}
      i += 1;
    }

  if CHAR_TABLE_P (vector) && ! NILP ((*XCHAR_TABLE (vector)).defalt)
    {
      if !NILP (elt_prefix)
	{ insert1 (elt_prefix); }
      insert (c"default".as_ptr (), 7);
      elt_describer.unwrap () ((*XCHAR_TABLE (vector)).defalt, args);
    }
}

/* The six `DEFVAR_LISP` variables this unit owns: C stores them as fields of `struct
   emacs_globals` and this file's `syms_of_keymap` registers them, so the port defines them
   here (PORTING.md P-21, status 3.29) - the `include/globals.rs` `variables` module only
   declares them, and a definition in the unit shadows that glob import (0.1-10).  A
   `DEFVAR_*` registration is `DEFVAR_*!` with C's Lisp name and the docstring.  */
#[unsafe(no_mangle)]
pub static mut Vminibuffer_local_map: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vminor_mode_map_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vminor_mode_overriding_map_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vemulation_mode_map_alists: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwhere_is_preferred_modifier: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdescribe_bindings_check_shadowing_in_ranges: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_keymap ()
{
  DEFSYM! (Qkeymap, "keymap");
  DEFSYM! (Qhelp__describe_map_tree, "help--describe-map-tree");
  DEFSYM! (Qmap_keymap_sorted, "map-keymap-sorted");

  DEFSYM! (Qkeymap_canonicalize, "keymap-canonicalize");

  /* Now we are ready to set up this property, so we can
     create char tables.  */
  Fput (Qkeymap (), Qchar_table_extra_slots (), make_fixnum (0));

  /* Initialize the keymaps standardly used.
     Each one is the value of a Lisp variable, and is also
     pointed to by a C variable */

  current_global_map = Qnil ();
  staticpro (&raw const current_global_map);

  exclude_keys = list!
    (Fcons (build_string (c"DEL".as_ptr ()), build_string (c"\\d".as_ptr ())),
     Fcons (build_string (c"TAB".as_ptr ()), build_string (c"\\t".as_ptr ())),
     Fcons (build_string (c"RET".as_ptr ()), build_string (c"\\r".as_ptr ())),
     Fcons (build_string (c"ESC".as_ptr ()), build_string (c"\\e".as_ptr ())),
     Fcons (build_string (c"SPC".as_ptr ()), build_string (c" ".as_ptr ())));
  staticpro (&raw const exclude_keys);

  DEFVAR_LISP! (c"minibuffer-local-map", Vminibuffer_local_map,
	       "Default keymap to use when reading from the minibuffer.");
  Vminibuffer_local_map = Fmake_sparse_keymap (Qnil ());

  DEFVAR_LISP! (c"minor-mode-map-alist", Vminor_mode_map_alist,
	       "Alist of keymaps to use for minor modes.\nEach element looks like (VARIABLE . KEYMAP); KEYMAP is used to read\nkey sequences and look up bindings if VARIABLE's value is non-nil.\nIf two active keymaps bind the same key, the keymap appearing earlier\nin the list takes precedence.");
  Vminor_mode_map_alist = Qnil ();

  DEFVAR_LISP! (c"minor-mode-overriding-map-alist", Vminor_mode_overriding_map_alist,
	       "Alist of keymaps to use for minor modes, in current major mode.\nThis variable is an alist just like `minor-mode-map-alist', and it is\nused the same way (and before `minor-mode-map-alist'); however,\nit is provided for major modes to bind locally.");
  Vminor_mode_overriding_map_alist = Qnil ();

  DEFVAR_LISP! (c"emulation-mode-map-alists", Vemulation_mode_map_alists,
	       "List of keymap alists to use for emulation modes.\nIt is intended for modes or packages using multiple minor-mode keymaps.\nEach element is a keymap alist just like `minor-mode-map-alist', or a\nsymbol with a variable binding which is a keymap alist, and it is used\nthe same way.  The \"active\" keymaps in each alist are used before\n`minor-mode-map-alist' and `minor-mode-overriding-map-alist'.");
  Vemulation_mode_map_alists = Qnil ();

  DEFVAR_LISP! (c"where-is-preferred-modifier", Vwhere_is_preferred_modifier,
	       "Preferred modifier key to use for `where-is'.\nWhen a single binding is requested, `where-is' will return one that\nuses this modifier key if possible.  If nil, or if no such binding\nexists, bindings using keys without modifiers (or only with meta) will\nbe preferred.");
  Vwhere_is_preferred_modifier = Qnil ();
  where_is_preferred_modifier = 0;

  DEFVAR_LISP! (c"describe-bindings-check-shadowing-in-ranges",
	       Vdescribe_bindings_check_shadowing_in_ranges,
	       "If non-nil, consider command shadowing when describing ranges of keys.\nIf the value is t, describing bindings of consecutive keys will not\nreport them as a single range if they are shadowed by different\nminor-mode commands.\nIf the value is `ignore-self-insert', assume that consecutive keys\nbound to `self-insert-command' are not all shadowed; this speeds up\ncommands such as \\[describe-bindings] and \\[describe-mode], but could miss some shadowing.\nAny other non-nil value is treated is t.\n\nBeware: setting this non-nil could potentially slow down commands\nthat describe key bindings.  That is why the default is nil.");
  Vdescribe_bindings_check_shadowing_in_ranges = Qnil ();

  DEFSYM! (Qself_insert_command, "self-insert-command");
  DEFSYM! (Qignore_self_insert, "ignore-self-insert");

  DEFSYM! (Qmenu_bar, "menu-bar");
  DEFSYM! (Qmode_line, "mode-line");

  staticpro (&raw const Vmouse_events);
  Vmouse_events = list! (Qmenu_bar (), Qtab_bar (), Qtool_bar (), Qtab_line (),
			Qheader_line (), Qmode_line (),
			intern_c_string (c"mouse-1".as_ptr ()),
			intern_c_string (c"mouse-2".as_ptr ()),
			intern_c_string (c"mouse-3".as_ptr ()),
			intern_c_string (c"mouse-4".as_ptr ()),
			intern_c_string (c"mouse-5".as_ptr ()));

  /* Keymap used for minibuffers when doing completion.  */
  /* Keymap used for minibuffers when doing completion and require a match.  */
  DEFSYM! (Qkeymapp, "keymapp");
  DEFSYM! (Qnon_ascii, "non-ascii");
  DEFSYM! (Qmenu_item, "menu-item");
  DEFSYM! (Qremap, "remap");
  DEFSYM! (QCadvertised_binding, ":advertised-binding");

  command_remapping_vector = make_vector (2, Qremap ());
  staticpro (&raw const command_remapping_vector);

  where_is_cache_keymaps = Qt ();
  where_is_cache = Qnil ();
  staticpro (&raw const where_is_cache);
  staticpro (&raw const where_is_cache_keymaps);

  DEFSYM! (Qfont_lock_face, "font-lock-face");
  DEFSYM! (Qhelp_key_binding, "help-key-binding");
  staticpro (&raw const fontify_key_properties);
  fontify_key_properties = Fcons (Qfont_lock_face (),
				  Fcons (Qhelp_key_binding (), Qnil ()));

  defsubr (&raw mut Skeymapp);
  defsubr (&raw mut Skeymap_parent);
  defsubr (&raw mut Skeymap_prompt);
  defsubr (&raw mut Sset_keymap_parent);
  defsubr (&raw mut Smake_keymap);
  defsubr (&raw mut Smake_sparse_keymap);
  defsubr (&raw mut Smap_keymap_internal);
  defsubr (&raw mut Smap_keymap);
  defsubr (&raw mut Scopy_keymap);
  defsubr (&raw mut Scommand_remapping);
  defsubr (&raw mut Skey_binding);
  defsubr (&raw mut Sminor_mode_key_binding);
  defsubr (&raw mut Sdefine_key);
  defsubr (&raw mut Slookup_key);
  defsubr (&raw mut Suse_global_map);
  defsubr (&raw mut Suse_local_map);
  defsubr (&raw mut Scurrent_local_map);
  defsubr (&raw mut Scurrent_global_map);
  defsubr (&raw mut Scurrent_minor_mode_maps);
  defsubr (&raw mut Scurrent_active_maps);
  defsubr (&raw mut Saccessible_keymaps);
  defsubr (&raw mut Skey_description);
  defsubr (&raw mut Skeymap__get_keyelt);
  defsubr (&raw mut Shelp__describe_vector);
  defsubr (&raw mut Sdescribe_vector);
  defsubr (&raw mut Ssingle_key_description);
  defsubr (&raw mut Stext_char_description);
  defsubr (&raw mut Swhere_is_internal);
  defsubr (&raw mut Sdescribe_buffer_bindings);

  DEFSYM! (Qkey_parse, "key-parse");
  DEFSYM! (Qkey_valid_p, "key-valid-p");
  DEFSYM! (Qposn_point, "posn-point");
  DEFSYM! (Qnon_key_event, "non-key-event");
  DEFSYM! (Qprinc, "princ");
  DEFSYM! (Qsuppress_keymap, "suppress-keymap");
}
