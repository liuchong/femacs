// femacs: port of src/menu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The three
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's ten non-`static` definitions
// (`init_menu_items`, `finish_menu_items`, `unuse_menu_items`, `discard_menu_items`,
// `save_menu_items`, `list_of_panes`, `parse_single_submenu`, `menu_item_width`,
// `x_popup_menu_1`, `syms_of_menu`) and its seven file-scope objects are exported symbols, so
// they carry `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52); the twelve `static` helpers stay
// plain `fn` (`have_boxes`) or `unsafe fn`, and `restore_menu_items` (C 128) and
// `single_menu_item` (C 314) are `unsafe extern "C" fn` because C hands their addresses to
// `record_unwind_protect` and `map_keymap_canonical` (PORTING.md 1.4).  C's three
// forward declarations (`single_menu_item` at C 262-263 and the two `restore_menu_items`-style
// ones) have no Rust counterpart and are deleted, the definitions keeping their C place
// (PORTING.md 0.1-12).
//
// C's two `DEFVAR_LISP` variables (`Vx_pre_popup_menu_hook`, `Vx_popup_menu_function`) live in
// the generated globals.h in C; the port's globals.rs mirrors them as declarations and the
// owning unit defines them here (P-21, status 3.29).  The three `DEFUN` records are the
// port's `static mut S…`, which C also leaves to its own unit (PORTING.md 0.2).
//
// The dead regions are recorded in docs/ports/menu.md rather than transcribed.  This target
// has no window system: `HAVE_WINDOW_SYSTEM`, `HAVE_X_WINDOWS`, `HAVE_NTGUI`, `HAVE_NS`,
// `HAVE_PGTK`, `HAVE_HAIKU`, `HAVE_ANDROID`, `USE_GTK`, `USE_X_TOOLKIT`, `USE_LUCID` and
// `MSDOS` are all `undef` in docs/baseline/config/darwin-arm64.macros.tsv, and lisp.h
// 3929-3932 defines `HAVE_EXT_MENU_BAR` only under the first five of those.  Dead, therefore:
// C 36-37 (`TERM_HEADER`), C 40-41 (`unicode_append_menu`), C 50-53 (`have_boxes`' window
// arm), C 170-193 (`push_submenu_start`/`push_submenu_end`), C 422-432 (the toolkit submenu of
// `single_menu_item`), C 566-873 (the `widget_value` tree) and C 875-964
// (`find_and_call_menu_selection`), C 967-1031 (`find_and_return_menu_selection`, `#ifdef
// HAVE_NS`), C 1193-1209 and C 1307-1312 (inside `x_popup_menu_1`), C 1371-1374, C 1377-1388,
// C 1395-1403 and C 1600-1605 (the `#ifdef HAVE_NTGUI`/`HAVE_WINDOW_SYSTEM` bits).  Their
// header counterparts (`menu.h`'s `#ifdef HAVE_EXT_MENU_BAR` block and the four platform
// blocks) sit in docs/ports/menu.md too.
//
// Names this unit calls that the mirrors it includes do not carry are written at their use
// site (the terminal.rs/scroll.rs shape), with C's own bodies for the macros so that each
// argument is evaluated as often as C evaluates it: `larger_vector`, `record_unwind_protect_void`,
// `parse_menu_item`, `init_raw_keybuf_count`, `pixel_to_glyph_coords`, `window_box_left`,
// `is_tty_frame`, `decode_any_frame`, `estimate_mode_line_height` and `selected_window`.  (The
// `WINDOW_*` geometry and the `window_wants_*`/`CURRENT_*`/`MATRIX_*` families used to sit here
// too; they are the mirrors' now - include/window.rs and include/dispextern.rs - and this unit
// only calls them.)

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/limits.in.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] ../lib/include/mini-gmp.rs imported via common
// [U1] include/bignum.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/keyboard.rs imported via common
// [U1] include/keymap.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/blockinput.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/menu.rs imported via common
#[cfg(feature = "have_ns")]
const _: () = ();

/* The names above their C prototypes are window.h/frame.h/dispextern.h/lisp.h/keyboard.h's,
   written here because the mirrors do not carry them (P-55's `/// C:` line names the
   prototype they were taken from).  */

/// C: extern Lisp_Object larger_vector (Lisp_Object, ptrdiff_t, ptrdiff_t)  lisp.h 4245
unsafe extern "C" { }
/// C: extern void record_unwind_protect_void (void (*) (void))  lisp.h 4825
unsafe extern "C-unwind" { pub fn record_unwind_protect_void
  (function: Option<unsafe extern "C-unwind" fn ()>); }
/// C: extern bool parse_menu_item (Lisp_Object, int)  keyboard.h 474
unsafe extern "C" { }
/// C: extern void init_raw_keybuf_count (void)  keyboard.h 476
unsafe extern "C" { }
/// C: void pixel_to_glyph_coords (struct frame *, int, int, int *, int *,
///                                NativeRectangle *, bool)  dispextern.h 3584
unsafe extern "C-unwind" { pub fn pixel_to_glyph_coords (f: *mut frame, x: c_int, y: c_int,
						  px: *mut c_int, py: *mut c_int,
						  bounds: *mut c_void, native_p: bool); }
/// C: int window_box_left (struct window *, enum glyph_row_area)  dispextern.h 3579
unsafe extern "C-unwind" { pub fn window_box_left (w: *mut window,
					    area: glyph_row_area) -> c_int; }
/// C: bool is_tty_frame (struct frame *f)  dispextern.h 4012
unsafe extern "C-unwind" { pub fn is_tty_frame (f: *mut frame) -> bool; }
/// C: extern struct frame *decode_any_frame (Lisp_Object)  frame.h 1477
unsafe extern "C" { }
/// C: int estimate_mode_line_height (struct frame *, enum face_id)  dispextern.h 3582
unsafe extern "C-unwind" { pub fn estimate_mode_line_height (f: *mut frame,
						      face_id: face_id) -> c_int; }
// `selected_window` (window.h 1100) comes from the window.rs mirror this unit includes.

/* Return non-zero if menus can handle radio and toggle buttons.  */
fn have_boxes () -> bool
{
  // C 50-53's `#if defined (USE_X_TOOLKIT) || defined (USE_GTK) || defined (HAVE_NTGUI)
  // || defined (HAVE_NS) || defined (HAVE_HAIKU) || defined (HAVE_ANDROID)` arm is dead on
  // this target; C's live text is the `return 0` below (docs/ports/menu.md).
  #[cfg(feature = "have_ns")]
  if /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_WINDOW_P(XFRAME((*Vmenu_updating_frame.as_mut_ptr ()))) } { return true; }
  return false;
}

#[unsafe(no_mangle)]
pub static menu_items: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* Whether the global vars defined here are already in use.
   Used to detect cases where we try to re-enter this non-reentrant code.  */
#[unsafe(no_mangle)]
pub static menu_items_inuse: VmGlobal<bool> = VmGlobal::<bool>::new (false);

/* Number of slots currently allocated in menu_items.  */
#[unsafe(no_mangle)]
pub static menu_items_allocated: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

/* This is the index in menu_items of the first empty slot.  */
#[unsafe(no_mangle)]
pub static menu_items_used: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

/* The number of panes currently recorded in menu_items,
   excluding those within submenus.  */
#[unsafe(no_mangle)]
pub static menu_items_n_panes: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

/* Current depth within submenus.  */
static menu_items_submenu_depth: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn init_menu_items ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if (*menu_items_inuse.as_mut_ptr ())
    { error (c"Trying to use a menu from within a menu-entry".as_ptr ()); }

  if NILP ((*menu_items.as_mut_ptr ()))
    {
      (*menu_items_allocated.as_mut_ptr ()) = 60;
      (*menu_items.as_mut_ptr ()) = make_nil_vector ((*menu_items_allocated.as_mut_ptr ()) as isize);
    }

  (*menu_items_inuse.as_mut_ptr ()) = true;
  (*menu_items_used.as_mut_ptr ()) = 0;
  (*menu_items_n_panes.as_mut_ptr ()) = 0;
  (*menu_items_submenu_depth.as_mut_ptr ()) = 0;

  }
}

/* Call at the end of generating the data in menu_items.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn finish_menu_items ()
{
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn unuse_menu_items ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  (*menu_items_inuse.as_mut_ptr ()) = false;

  }
}

/* Call when finished using the data for the current menu
   in menu_items.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn discard_menu_items ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  /* Free the structure if it is especially large.
     Otherwise, hold on to it, to save time.  */
  if (*menu_items_allocated.as_mut_ptr ()) > 200
    {
      (*menu_items.as_mut_ptr ()) = Qnil ();
      (*menu_items_allocated.as_mut_ptr ()) = 0;
    }
  eassert! (!(*menu_items_inuse.as_mut_ptr ()));

  }
}

/* This undoes save_menu_items, and it is called by the specpdl unwind
   mechanism.  */

unsafe extern "C-unwind" fn restore_menu_items (mut saved: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  (*menu_items.as_mut_ptr ()) = XCAR (saved);
  (*menu_items_inuse.as_mut_ptr ()) = ! NILP ((*menu_items.as_mut_ptr ()));
  (*menu_items_allocated.as_mut_ptr ()) = if VECTORP ((*menu_items.as_mut_ptr ())) { ASIZE ((*menu_items.as_mut_ptr ())) as c_int } else { 0 };
  saved = XCDR (saved);
  (*menu_items_used.as_mut_ptr ()) = XFIXNUM (XCAR (saved)) as c_int;
  saved = XCDR (saved);
  (*menu_items_n_panes.as_mut_ptr ()) = XFIXNUM (XCAR (saved)) as c_int;
  saved = XCDR (saved);
  (*menu_items_submenu_depth.as_mut_ptr ()) = XFIXNUM (XCAR (saved)) as c_int;

  }
}

/* Push the whole state of menu_items processing onto the specpdl.
   It will be restored when the specpdl is unwound.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn save_menu_items ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let saved: Lisp_Object = list4 (if (*menu_items_inuse.as_mut_ptr ()) { (*menu_items.as_mut_ptr ()) } else { Qnil () },
				  make_fixnum ((*menu_items_used.as_mut_ptr ()) as EMACS_INT),
				  make_fixnum ((*menu_items_n_panes.as_mut_ptr ()) as EMACS_INT),
				  make_fixnum ((*menu_items_submenu_depth.as_mut_ptr ()) as EMACS_INT));
  record_unwind_protect (Some (restore_menu_items), saved);
  (*menu_items_inuse.as_mut_ptr ()) = false;
  (*menu_items.as_mut_ptr ()) = Qnil ();

  }
}

/* Ensure that there is room for ITEMS items in the menu_items vector.  */

unsafe fn ensure_menu_items (items: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let incr: c_int = items - ((*menu_items_allocated.as_mut_ptr ()) - (*menu_items_used.as_mut_ptr ()));
  if incr > 0
    {
      (*menu_items.as_mut_ptr ()) = larger_vector ((*menu_items.as_mut_ptr ()), incr as ptrdiff_t, INT_MAX as ptrdiff_t);
      (*menu_items_allocated.as_mut_ptr ()) = ASIZE ((*menu_items.as_mut_ptr ())) as c_int;
    }

  }
}

// C 170-193's `#if defined HAVE_EXT_MENU_BAR || defined HAVE_ANDROID` block
// (`push_submenu_start` at C 175 and `push_submenu_end` at C 186) is dead on this target:
// no window system, so lisp.h 3929-3932 does not define `HAVE_EXT_MENU_BAR`, and Android is
// undefined too (docs/ports/menu.md).

/* Indicate boundary between left and right.  */

unsafe fn push_left_right_boundary ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  ensure_menu_items (1);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize, Qquote ());
  (*menu_items_used.as_mut_ptr ()) += 1;

  }
}

/* Start a new menu pane in menu_items.
   NAME is the pane name.  PREFIX_VEC is a prefix key for this pane.  */

unsafe fn push_menu_pane (name: Lisp_Object, prefix_vec: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  ensure_menu_items (MENU_ITEMS_PANE_LENGTH);
  if (*menu_items_submenu_depth.as_mut_ptr ()) == 0
    { (*menu_items_n_panes.as_mut_ptr ()) += 1; }
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize, Qt ());
  (*menu_items_used.as_mut_ptr ()) += 1;
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize, name);
  (*menu_items_used.as_mut_ptr ()) += 1;
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize, prefix_vec);
  (*menu_items_used.as_mut_ptr ()) += 1;

  }
}

/* Push one menu item into the current pane.  NAME is the string to
   display.  ENABLE if non-nil means this item can be selected.  KEY
   is the key generated by choosing this item, or nil if this item
   doesn't really have a definition.  DEF is the definition of this
   item.  EQUIV is the textual description of the keyboard equivalent
   for this item (or nil if none).  TYPE is the type of this menu
   item, one of nil, `toggle' or `radio'. */

unsafe fn push_menu_item (name: Lisp_Object, enable: Lisp_Object, key: Lisp_Object,
			  def: Lisp_Object, equiv: Lisp_Object, r#type: Lisp_Object,
			  selected: Lisp_Object, help: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  ensure_menu_items (MENU_ITEMS_ITEM_LENGTH);

  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_NAME as isize,	name);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_ENABLE as isize,	enable);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_VALUE as isize,	key);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_EQUIV_KEY as isize, equiv);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_DEFINITION as isize, def);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_TYPE as isize,	r#type);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_SELECTED as isize, selected);
  ASET ((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize + MENU_ITEMS_ITEM_HELP as isize,	help);

  (*menu_items_used.as_mut_ptr ()) += MENU_ITEMS_ITEM_LENGTH;

  }
}

/* Args passed between single_keymap_panes and single_menu_item.  */
#[repr(C)]
pub struct skp
  {
     pub pending_maps: Lisp_Object,
     pub maxdepth: c_int,
     pub notbuttons: c_int,
  }

// C 262-263's `static void single_menu_item (Lisp_Object, Lisp_Object, Lisp_Object, void *);`
// is a forward declaration with no Rust counterpart (PORTING.md 0.1-12): it is deleted and
// the definition keeps its C place below.

/* This is a recursive subroutine of keymap_panes.
   It handles one keymap, KEYMAP.
   The other arguments are passed along
   or point to local variables of the previous function.

   If we encounter submenus deeper than MAXDEPTH levels, ignore them.  */

unsafe fn single_keymap_panes (keymap: Lisp_Object, pane_name: Lisp_Object,
			       prefix: Lisp_Object, maxdepth: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut skp: skp = std::mem::zeroed ();

  skp.pending_maps = Qnil ();
  skp.maxdepth = maxdepth;
  skp.notbuttons = 0;

  if maxdepth <= 0
    { return; }

  push_menu_pane (pane_name, prefix);

  if !have_boxes ()
    {
      /* Remember index for first item in this pane so we can go back
	 and add a prefix when (if) we see the first button.  After
	 that, notbuttons is set to 0, to mark that we have seen a
	 button and all non button items need a prefix.  */
      skp.notbuttons = (*menu_items_used.as_mut_ptr ());
    }

  map_keymap_canonical (keymap, Some (single_menu_item), Qnil (),
			&mut skp as *mut skp as *mut c_void);

  /* Process now any submenus which want to be panes at this level.  */
  while CONSP (skp.pending_maps)
    {
      let elt: Lisp_Object;
      let eltcdr: Lisp_Object;
      let string: Lisp_Object;
      elt = XCAR (skp.pending_maps);
      eltcdr = XCDR (elt);
      string = XCAR (eltcdr);
      /* We no longer discard the @ from the beginning of the string here.
	 Instead, we do this in *menu_show.  */
      single_keymap_panes (Fcar (elt), string, XCDR (eltcdr), maxdepth - 1);
      skp.pending_maps = XCDR (skp.pending_maps);
    }

  }
}

/* This is a subroutine of single_keymap_panes that handles one
   keymap entry.
   KEY is a key in a keymap and ITEM is its binding.
   SKP->PENDING_MAPS_PTR is a list of keymaps waiting to be made into
   separate panes.
   If we encounter submenus deeper than SKP->MAXDEPTH levels, ignore them.  */

unsafe extern "C-unwind" fn single_menu_item (key: Lisp_Object, item: Lisp_Object,
				       dummy: Lisp_Object, skp_v: *mut c_void)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let map: Lisp_Object;
  let mut item_string: Lisp_Object;
  let enabled: Lisp_Object;
  let res: bool;
  let skp: *mut skp = skp_v as *mut skp;

  /* Parse the menu item and leave the result in item_properties.  */
  res = parse_menu_item (item, 0);
  if !res
    { return; }			/* Not a menu item.  */

  map = AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_MAP as isize);

  enabled = AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_ENABLE as isize);
  item_string = AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_NAME as isize);

  if !NILP (map) && SREF (item_string, 0) == b'@'
    {
      if !NILP (enabled)
	{ /* An enabled separate pane. Remember this to handle it later.  */
	  (*skp).pending_maps = Fcons (Fcons (map, Fcons (item_string, key)),
				       (*skp).pending_maps); }
      return;
    }

  /* Simulate radio buttons and toggle boxes by putting a prefix in
     front of them.  */
  if !have_boxes ()
    {
      let mut prefix: *const c_char = std::ptr::null ();
      let r#type: Lisp_Object = AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_TYPE as isize);
      if !NILP (r#type)
	{
	  let selected: Lisp_Object
	    = AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_SELECTED as isize);

	  if (*skp).notbuttons != 0
	    /* The first button. Line up previous items in this menu.  */
	    {
	      let mut idx: c_int = (*skp).notbuttons; /* Index for first item this menu.  */
	      let mut submenu: c_int = 0;
	      let mut tem: Lisp_Object;
	      while idx < (*menu_items_used.as_mut_ptr ())
		{
		  tem
		    = AREF ((*menu_items.as_mut_ptr ()), (idx + MENU_ITEMS_ITEM_NAME) as isize);
		  if NILP (tem)
		    {
		      idx += 1;
		      submenu += 1;		/* Skip sub menu.  */
		    }
		  else if EQ (tem, Qlambda ())
		    {
		      idx += 1;
		      submenu -= 1;		/* End sub menu.  */
		    }
		  else if EQ (tem, Qt ())
		    { idx += 3; }	/* Skip new pane marker. */
		  else if EQ (tem, Qquote ())
		    { idx += 1; }	/* Skip a left, right divider. */
		  else
		    {
		      if submenu == 0 && SREF (tem, 0) != b'\0'
			  && SREF (tem, 0) != b'-'
			{
			  AUTO_STRING! (spaces, c"    ".as_ptr ());
			  ASET ((*menu_items.as_mut_ptr ()), (idx + MENU_ITEMS_ITEM_NAME) as isize,
				concat2 (spaces, tem));
			}
		      idx += MENU_ITEMS_ITEM_LENGTH;
		    }
		}
	      (*skp).notbuttons = 0;
	    }

	  /* Calculate prefix, if any, for this item.  */
	  if EQ (r#type, QCtoggle ())
	    { prefix = if NILP (selected) { c"[ ] ".as_ptr () } else { c"[X] ".as_ptr () }; }
	  else if EQ (r#type, QCradio ())
	    { prefix = if NILP (selected) { c"( ) ".as_ptr () } else { c"(*) ".as_ptr () }; }
	}
      /* Not a button. If we have earlier buttons, then we need a prefix.  */
      else if (*skp).notbuttons == 0 && SREF (item_string, 0) != b'\0'
	       && SREF (item_string, 0) != b'-'
	{ prefix = c"    ".as_ptr (); }

      if !prefix.is_null ()
	{
	  AUTO_STRING_WITH_LEN! (prefix_obj, prefix, 4);
	  item_string = concat2 (prefix_obj, item_string);
	}
    }

  if is_tty_frame (XFRAME ((*Vmenu_updating_frame.as_mut_ptr ())))
      && !NILP (map)
    /* Indicate visually that this is a submenu.  */
    {
      AUTO_STRING! (space_gt, c" >".as_ptr ());
      item_string = concat2 (item_string, space_gt);
    }

  push_menu_item (item_string, enabled, key,
		  AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_DEF as isize),
		  AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_KEYEQ as isize),
		  AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_TYPE as isize),
		  AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_SELECTED as isize),
		  AREF ((*item_properties.as_mut_ptr ()), ITEM_PROPERTY_HELP as isize));

  #[cfg(feature = "have_ns")]
  if FRAME_WINDOW_P(XFRAME((*Vmenu_updating_frame.as_mut_ptr ()))) && !(NILP(map) || NILP(enabled)) {
    push_submenu_start();
    single_keymap_panes(map, Qnil(), key, (*skp).maxdepth - 1);
    push_submenu_end();
  }

  }
}

/* Look through KEYMAPS, a vector of keymaps that is NMAPS long,
   and generate menu panes for them in menu_items.  */

unsafe fn keymap_panes (keymaps: *mut Lisp_Object, nmaps: ptrdiff_t)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut mapno: ptrdiff_t;

  init_menu_items ();

  /* Loop over the given keymaps, making a pane for each map.
     But don't make a pane that is empty--ignore that map instead.
     P is the number of panes we have made so far.  */
  mapno = 0;
  while mapno < nmaps
    {
      single_keymap_panes (*keymaps.offset (mapno as isize),
			   Fkeymap_prompt (*keymaps.offset (mapno as isize)), Qnil (), 10);
      mapno += 1;
    }

  finish_menu_items ();

  }
}

/* Encode a menu string as appropriate for menu-updating-frame's type.  */
unsafe fn encode_menu_string (str: Lisp_Object) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  /* TTY menu strings are encoded by write_glyphs, when they are
     delivered to the glass, so no need to encode them here.  */
  if FRAME_TERMCAP_P (XFRAME ((*Vmenu_updating_frame.as_mut_ptr ())))
    { return str; }
  return ENCODE_MENU_STRING (str);

  }
}

/* Push the items in a single pane defined by the alist PANE.  */
unsafe fn list_of_items (pane: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut tail: Lisp_Object;
  let mut item: Lisp_Object;
  let mut item1: Lisp_Object;

  tail = pane;
  while CONSP (tail)
    {
      item = XCAR (tail);
      if STRINGP (item)
	{ push_menu_item (encode_menu_string (item), Qnil (), Qnil (), Qt (),
			  Qnil (), Qnil (), Qnil (), Qnil ()); }
      else if CONSP (item)
	{
	  item1 = XCAR (item);
	  CHECK_STRING (item1);
	  push_menu_item (encode_menu_string (item1), Qt (), XCDR (item),
			  Qt (), Qnil (), Qnil (), Qnil (), Qnil ());
	}
      else
	{ push_left_right_boundary (); }

      tail = XCDR (tail);
    }

  }
}

/* Push all the panes and items of a menu described by the
   alist-of-alists MENU.
   This handles old-fashioned calls to x-popup-menu.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn list_of_panes (menu: Lisp_Object)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut tail: Lisp_Object;

  init_menu_items ();

  tail = menu;
  while CONSP (tail)
    {
      let elt: Lisp_Object;
      let pane_name: Lisp_Object;
      let pane_data: Lisp_Object;
      elt = XCAR (tail);
      pane_name = Fcar (elt);
      CHECK_STRING (pane_name);
      push_menu_pane (encode_menu_string (pane_name), Qnil ());
      pane_data = Fcdr (elt);
      CHECK_CONS (pane_data);
      list_of_items (pane_data);
      tail = XCDR (tail);
    }

  finish_menu_items ();

  }
}

/* Set up data in menu_items for a menu bar item
   whose event type is ITEM_KEY (with string ITEM_NAME)
   and whose contents come from the list of keymaps MAPS.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn parse_single_submenu (item_key: Lisp_Object,
					       item_name: Lisp_Object,
					       mut maps: Lisp_Object) -> bool
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mapvec: *mut Lisp_Object;
  let mut top_level_items: bool = false;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  let len: ptrdiff_t = list_length (maps);

  /* Convert the list MAPS into a vector MAPVEC.  */
  SAFE_ALLOCA_LISP! (sa_avail, mapvec, len);
  let mut i: ptrdiff_t = 0;
  while i < len
    {
      *mapvec.offset (i) = Fcar (maps);
      maps = Fcdr (maps);
      i += 1;
    }

  /* Loop over the given keymaps, making a pane for each map.
     But don't make a pane that is empty--ignore that map instead.  */
  i = 0;
  while i < len
    {
      if !KEYMAPP! (*mapvec.offset (i))
	{
	  /* Here we have a command at top level in the menu bar
	     as opposed to a submenu.  */
	  top_level_items = true;
	  push_menu_pane (Qnil (), Qnil ());
	  push_menu_item (item_name, Qt (), item_key, *mapvec.offset (i),
			  Qnil (), Qnil (), Qnil (), Qnil ());
	}
      else
	{
	  let prompt: Lisp_Object;
	  prompt = Fkeymap_prompt (*mapvec.offset (i));
	  single_keymap_panes (*mapvec.offset (i),
			       if !NILP (prompt) { prompt } else { item_name },
			       item_key, 10);
	}
      i += 1;
    }

  SAFE_FREE! (sa_count, sa_mark);
  return top_level_items;

  }
}

// C menu.c 170-193: the toolkit submenu delimiters (Cocoa arm).
#[cfg(feature = "have_ns")]
unsafe fn push_submenu_start() {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  ensure_menu_items(1);
  ASET((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize, Qnil());
  (*menu_items_used.as_mut_ptr ()) += 1;
  (*menu_items_submenu_depth.as_mut_ptr ()) += 1;

  }
}
#[cfg(feature = "have_ns")]
unsafe fn push_submenu_end() {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  ensure_menu_items(1);
  ASET((*menu_items.as_mut_ptr ()), (*menu_items_used.as_mut_ptr ()) as isize, Qlambda());
  (*menu_items_used.as_mut_ptr ()) += 1;
  (*menu_items_submenu_depth.as_mut_ptr ()) -= 1;

  }
}

// C menu.c 566-873: the native widget-value tree, including its GC ordering.
#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn make_widget_value(name: *const c_char, value: *mut c_char,
                                           enabled: bool, help: Lisp_Object) -> *mut widget_value {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  block_input();
  let wv = xzalloc(core::mem::size_of::<widget_value>()).cast::<widget_value>();
  unblock_input();
  (*wv).name = name as *mut c_char;
  (*wv).value = value;
  (*wv).enabled = enabled;
  (*wv).help = help;
  wv

  }
}

#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn free_menubar_widget_value_tree(wv: *mut widget_value) {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  if wv.is_null() { return; }
  (*wv).key = 0xDEADBEEFusize as *mut c_char;
  (*wv).value = (*wv).key;
  (*wv).name = (*wv).value;
  if !(*wv).contents.is_null() && (*wv).contents as usize != 1 {
    free_menubar_widget_value_tree((*wv).contents);
    (*wv).contents = 0xDEADBEEFusize as *mut widget_value;
  }
  if !(*wv).next.is_null() {
    free_menubar_widget_value_tree((*wv).next);
    (*wv).next = 0xDEADBEEFusize as *mut widget_value;
  }
  block_input();
  xfree(wv.cast());
  unblock_input();

  }
}

#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn digest_single_submenu(start: c_int, end: c_int, top_level_items: bool) -> *mut widget_value {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut submenu_depth: c_int = 0;
  let mut submenu_stack: *mut *mut widget_value = core::ptr::null_mut();
  let mut panes_seen = false;
  let f = XFRAME((*Vmenu_updating_frame.as_mut_ptr ()));
  USE_SAFE_ALLOCA!(sa_avail, sa_count, sa_mark);
  SAFE_NALLOCA!(sa_avail, submenu_stack, 1, (*menu_items_used.as_mut_ptr ()));
  let mut wv = make_widget_value(c"menu".as_ptr(), core::ptr::null_mut(), true, Qnil());
  (*wv).button_type = BUTTON_TYPE_NONE;
  let mut first_wv = wv;
  let mut save_wv: *mut widget_value = core::ptr::null_mut();
  let mut prev_wv: *mut widget_value = core::ptr::null_mut();
  let mut i = start;
  while i < end {
    if NILP(AREF((*menu_items.as_mut_ptr ()), i as isize)) {
      *submenu_stack.offset(submenu_depth as isize) = save_wv;
      submenu_depth += 1;
      save_wv = prev_wv;
      prev_wv = core::ptr::null_mut();
      i += 1;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qlambda()) {
      prev_wv = save_wv;
      submenu_depth -= 1;
      save_wv = *submenu_stack.offset(submenu_depth as isize);
      i += 1;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qt()) && submenu_depth != 0 {
      i += MENU_ITEMS_PANE_LENGTH;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qquote()) {
      i += 1;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qt()) {
      panes_seen = true;
      let mut pane_name = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_PANE_NAME) as isize);
      if !FRAME_TERMCAP_P(f) && STRINGP(pane_name) && STRING_MULTIBYTE(pane_name) {
        pane_name = ENCODE_MENU_STRING(pane_name);
        ASET((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_PANE_NAME) as isize, pane_name);
      }
      let mut pane_string = if NILP(pane_name) { c"".as_ptr() } else { SSDATA(pane_name) };
      if (*menu_items_n_panes.as_mut_ptr ()) == 1 { pane_string = c"".as_ptr(); }
      if strcmp(pane_string, c"".as_ptr()) != 0 {
        wv = make_widget_value(core::ptr::null(), 1usize as *mut c_char, true, Qnil());
        if !save_wv.is_null() { (*save_wv).next = wv; }
        else { (*first_wv).contents = wv; }
        (*wv).lname = pane_name;
        (*wv).button_type = BUTTON_TYPE_NONE;
        save_wv = wv;
      } else { save_wv = first_wv; }
      prev_wv = core::ptr::null_mut();
      i += MENU_ITEMS_PANE_LENGTH;
    } else {
      if !panes_seen { emacs_abort(); }
      let mut item_name = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_NAME) as isize);
      let enable = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_ENABLE) as isize);
      let mut descrip = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_EQUIV_KEY) as isize);
      let def = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_DEFINITION) as isize);
      let typ = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_TYPE) as isize);
      let selected = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_SELECTED) as isize);
      let help = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_HELP) as isize);
      if !FRAME_TERMCAP_P(f) {
        if STRING_MULTIBYTE(item_name) {
          item_name = ENCODE_MENU_STRING(item_name);
          ASET((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_NAME) as isize, item_name);
        }
        if STRINGP(descrip) && STRING_MULTIBYTE(descrip) {
          descrip = ENCODE_MENU_STRING(descrip);
          ASET((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_EQUIV_KEY) as isize, descrip);
        }
      }
      wv = make_widget_value(core::ptr::null(), core::ptr::null_mut(), !NILP(enable), if STRINGP(help) { help } else { Qnil() });
      if !prev_wv.is_null() { (*prev_wv).next = wv; }
      else { (*save_wv).contents = wv; }
      (*wv).lname = item_name;
      if !NILP(descrip) { (*wv).lkey = descrip; }
      (*wv).call_data = if !NILP(def) { i as isize as *mut c_void } else { core::ptr::null_mut() };
      (*wv).button_type = if NILP(typ) { BUTTON_TYPE_NONE }
          else if EQ(typ, QCradio()) { BUTTON_TYPE_RADIO }
          else if EQ(typ, QCtoggle()) { BUTTON_TYPE_TOGGLE }
          else { emacs_abort(); };
      (*wv).selected = !NILP(selected);
      prev_wv = wv;
      i += MENU_ITEMS_ITEM_LENGTH;
    }
  }
  if top_level_items && !(*first_wv).contents.is_null() && (*(*first_wv).contents).next.is_null() {
    wv = first_wv;
    first_wv = (*first_wv).contents;
    xfree(wv.cast());
  }
  SAFE_FREE!(sa_count, sa_mark);
  first_wv

  }
}

#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn update_submenu_strings(first_wv: *mut widget_value) {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut wv = first_wv;
  while !wv.is_null() {
    if STRINGP((*wv).lname) {
      (*wv).name = SSDATA((*wv).lname) as *mut c_char;
      if (*wv).value as usize == 1 {
        if *(*wv).name == b'@' as c_char { (*wv).name = (*wv).name.add(1); }
        (*wv).value = core::ptr::null_mut();
      }
    }
    if STRINGP((*wv).lkey) { (*wv).key = SSDATA((*wv).lkey) as *mut c_char; }
    if !(*wv).contents.is_null() { update_submenu_strings((*wv).contents); }
    wv = (*wv).next;
  }

  }
}

// C menu.c 875-964: enqueue every selected prefix in original order.
#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn find_and_call_menu_selection(f: *mut frame, menu_bar_items_used: c_int,
                                                       vector: Lisp_Object, client_data: *mut c_void) {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut entry = Qnil();
  let mut subprefix_stack: *mut Lisp_Object = core::ptr::null_mut();
  let mut submenu_depth: c_int = 0;
  USE_SAFE_ALLOCA!(sa_avail, sa_count, sa_mark);
  SAFE_NALLOCA!(sa_avail, subprefix_stack, 1, menu_bar_items_used);
  let mut prefix = Qnil();
  let mut i = 0;
  while i < menu_bar_items_used {
    if NILP(AREF(vector, i as isize)) {
      *subprefix_stack.offset(submenu_depth as isize) = prefix;
      submenu_depth += 1; prefix = entry; i += 1;
    } else if EQ(AREF(vector, i as isize), Qlambda()) {
      submenu_depth -= 1; prefix = *subprefix_stack.offset(submenu_depth as isize); i += 1;
    } else if EQ(AREF(vector, i as isize), Qt()) {
      prefix = AREF(vector, (i + MENU_ITEMS_PANE_PREFIX) as isize); i += MENU_ITEMS_PANE_LENGTH;
    } else {
      entry = AREF(vector, (i + MENU_ITEMS_ITEM_VALUE) as isize);
      if client_data as isize == i as isize {
        let mut buf: input_event = core::mem::zeroed();
        let mut frame = Qnil();
        EVENT_INIT(&raw mut buf);
        XSETFRAME!(frame, f);
        buf.bits = (buf.bits & !INPUT_EVENT_KIND_MASK) | MENU_BAR_EVENT as u32;
        buf.frame_or_window = frame; buf.arg = frame;
        kbd_buffer_store_event((&raw mut buf).cast());
        let mut j = 0;
        while j < submenu_depth {
          if !NILP(*subprefix_stack.offset(j as isize)) {
            buf.bits = (buf.bits & !INPUT_EVENT_KIND_MASK) | MENU_BAR_EVENT as u32;
            buf.frame_or_window = frame; buf.arg = *subprefix_stack.offset(j as isize);
            kbd_buffer_store_event((&raw mut buf).cast());
          }
          j += 1;
        }
        if !NILP(prefix) {
          buf.bits = (buf.bits & !INPUT_EVENT_KIND_MASK) | MENU_BAR_EVENT as u32;
          buf.frame_or_window = frame; buf.arg = prefix;
          kbd_buffer_store_event((&raw mut buf).cast());
        }
        buf.bits = (buf.bits & !INPUT_EVENT_KIND_MASK) | MENU_BAR_EVENT as u32;
        buf.frame_or_window = frame; buf.arg = entry;
        kbd_buffer_store_event((&raw mut buf).cast());
        break;
      }
      i += MENU_ITEMS_ITEM_LENGTH;
    }
  }
  SAFE_FREE!(sa_count, sa_mark);

  }
}

// C menu.c 967-1031: return the selection and its keymap prefixes.
#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn find_and_return_menu_selection(_f: *mut frame, keymaps: bool, client_data: *mut c_void) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut prefix = Qnil(); let mut entry = Qnil(); let mut i = 0;
  let mut subprefix_stack: *mut Lisp_Object = core::ptr::null_mut();
  let mut submenu_depth: c_int = 0;
  USE_SAFE_ALLOCA!(sa_avail, sa_count, sa_mark);
  SAFE_ALLOCA_LISP!(sa_avail, subprefix_stack, (*menu_items_used.as_mut_ptr ()));
  while i < (*menu_items_used.as_mut_ptr ()) {
    if NILP(AREF((*menu_items.as_mut_ptr ()), i as isize)) {
      *subprefix_stack.offset(submenu_depth as isize) = prefix;
      submenu_depth += 1; prefix = entry; i += 1;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qlambda()) {
      submenu_depth -= 1; prefix = *subprefix_stack.offset(submenu_depth as isize); i += 1;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qt()) {
      prefix = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_PANE_PREFIX) as isize); i += MENU_ITEMS_PANE_LENGTH;
    } else if EQ(AREF((*menu_items.as_mut_ptr ()), i as isize), Qquote()) { i += 1; }
    else {
      entry = AREF((*menu_items.as_mut_ptr ()), (i + MENU_ITEMS_ITEM_VALUE) as isize);
      if aref_addr((*menu_items.as_mut_ptr ()), i as isize).cast::<c_void>() == client_data {
        if keymaps {
          entry = list1(entry);
          if !NILP(prefix) { entry = Fcons(prefix, entry); }
          let mut j = submenu_depth - 1;
          while j >= 0 {
            if !NILP(*subprefix_stack.offset(j as isize)) { entry = Fcons(*subprefix_stack.offset(j as isize), entry); }
            j -= 1;
          }
        }
        SAFE_FREE!(sa_count, sa_mark);
        return entry;
      }
      i += MENU_ITEMS_ITEM_LENGTH;
    }
  }
  SAFE_FREE!(sa_count, sa_mark);
  Qnil()

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn menu_item_width (str: *const u8) -> ptrdiff_t
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut len: ptrdiff_t;
  let mut p: *const u8;

  len = 0;
  p = str;
  while *p != 0
    {
      let mut ch_len: c_int = 0;
      let ch: c_int = string_char_and_length (p, &mut ch_len);
      len += CHARACTER_WIDTH (ch) as ptrdiff_t;
      p = p.offset (ch_len as isize);
    }
  return len;

  }
}

DEFUN! ("menu-bar-menu-at-x-y", Fmenu_bar_menu_at_x_y, Smenu_bar_menu_at_x_y,
       2, 3, 0,
       doc: "Return the menu-bar menu on FRAME at pixel coordinates X, Y.\nX and Y are frame-relative pixel coordinates, assumed to define\na location within the menu bar.\nIf FRAME is nil or omitted, it defaults to the selected frame.\n\nValue is the symbol of the menu at X/Y, or nil if the specified\ncoordinates are not within the FRAME's menu bar.  The symbol can\nbe used to look up the menu like this:\n\n     (lookup-key MAP [menu-bar SYMBOL])\n\nwhere MAP is either the current global map or the current local map,\nsince menu-bar items come from both.\n\nThis function can return non-nil only on a text-terminal frame\nor on an X frame that doesn't use any GUI toolkit.  Otherwise,\nEmacs does not manage the menu bar and cannot convert coordinates\ninto menu items.",
       (x: Lisp_Object, y: Lisp_Object, frame: Lisp_Object),
       {
  let mut row: c_int = 0;
  let mut col: c_int = 0;
  let f: *mut frame = decode_any_frame (frame);

  if !FRAME_LIVE_P (f)
    { return Qnil (); }

  pixel_to_glyph_coords (f, XFIXNUM (x) as c_int, XFIXNUM (y) as c_int, &mut col, &mut row,
			 std::ptr::null_mut (), true);
  if 0 <= row && row < FRAME_MENU_BAR_LINES (f)
    {
      let items: Lisp_Object;
      let mut item: Lisp_Object;
      let mut i: c_int;

      /* Find the menu bar item under `col'.  */
      item = Qnil ();
      items = FRAME_MENU_BAR_ITEMS (f);
      /* This loop assumes a single menu-bar line, and will fail to
	 find an item if it is not in the first line.  Note that
	 make_lispy_event in keyboard.c makes the same assumption.  */
      i = 0;
      while i < ASIZE (items) as c_int
	{
	  let pos: Lisp_Object;
	  let str: Lisp_Object;

	  str = AREF (items, i as isize + 1);
	  pos = AREF (items, i as isize + 3);
	  if NILP (str)
	    { return item; }
	  if XFIXNUM (pos) <= col as EMACS_INT
	      /* We use <= so the blank between 2 items on a TTY is
		 considered part of the previous item.  */
	      && col as EMACS_INT <= XFIXNUM (pos) + menu_item_width (SDATA (str)) as EMACS_INT
	    {
	      item = AREF (items, i as isize);
	      return item;
	    }
	  i += 4;
	}
    }
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn x_popup_menu_1 (position: Lisp_Object, mut menu: Lisp_Object)
  -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut keymap: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut tem2: Lisp_Object = Qnil ();
  let mut xpos: c_int = 0;
  let mut ypos: c_int = 0;
  let mut title: Lisp_Object;
  let mut error_name: *const c_char = std::ptr::null ();
  let mut selection: Lisp_Object = Qnil ();
  let mut f: *mut frame = std::ptr::null_mut ();
  let mut x: Lisp_Object = lisp_h_Qnil ();
  let mut y: Lisp_Object = lisp_h_Qnil ();
  let mut window: Lisp_Object = lisp_h_Qnil ();
  let mut menuflags: c_int = 0;
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();

  if NILP (position)
    /* This is an obsolete call, which wants us to precompute the
       keybinding equivalents, but we don't do that any more anyway.  */
    { return Qnil (); }

  {
    let mut get_current_pos_p: bool = false;

    /* Decode the first argument: find the window and the coordinates.  */
    if EQ (position, Qt ())
	|| (CONSP (position)
	    && (EQ (XCAR (position), Qmenu_bar ())
		|| EQ (XCAR (position), Qtab_bar ())
		|| (CONSP (XCDR (position))
		    && EQ (XCAR (XCDR (position)), Qtab_bar ()))
		|| EQ (XCAR (position), Qtool_bar ())))
      {
	get_current_pos_p = true;
      }
    else
      {
	tem = Fcar (position);
	if CONSP (tem)
	  {
	    window = Fcar (Fcdr (position));
	    x = XCAR (tem);
	    y = Fcar (XCDR (tem));
	  }
	else
	  {
	    menuflags |= MENU_FOR_CLICK;
	    tem = EVENT_START (position);    /* EVENT_START (position) */
	    window = Fcar (tem);	     /* POSN_WINDOW (tem) */
	    tem2 = Fcar (Fcdr (tem));	     /* POSN_POSN (tem) */
	    /* The MENU_KBD_NAVIGATION field is set when the menu
	       was invoked by F10, which probably means they have no
	       mouse.  In that case, we let them switch between
	       top-level menu-bar menus by using C-f/C-b and
	       horizontal arrow keys, since they cannot click the
	       mouse to open a different submenu.  This flag is only
	       supported by tty_menu_show.  We set it when POSITION
	       and last_nonmenu_event are different, which means we
	       constructed POSITION by hand (in popup-menu, see
	       menu-bar.el) to look like a mouse click on the menu bar
	       event.  */
	    if !EQ (POSN_POSN ((*last_nonmenu_event.as_mut_ptr ())),
		    POSN_POSN (position))
		&& CONSP (tem2) && EQ (XCAR (tem2), Qmenu_bar ())
	      { menuflags |= MENU_KBD_NAVIGATION; }
	    tem = Fcar (Fcdr (Fcdr (tem))); /* POSN_WINDOW_POSN (tem) */
	    x = Fcar (tem);
	    y = Fcdr (tem);
	  }

	/* If a click happens in an external tool bar or a detached
	   tool bar, x and y is NIL.  In that case, use the current
	   mouse position.  This happens for the help button in the
	   tool bar.  Ideally popup-menu should pass NIL to
	   this function, but it doesn't.  */
	if NILP (x) && NILP (y)
	  { get_current_pos_p = true; }
      }

    if get_current_pos_p
      {
	/* Use the mouse's current position.  */
	let mut new_f: *mut frame = SELECTED_FRAME ();

	XSETFASTINT! (x, 0);
	XSETFASTINT! (y, 0);
	// C 1193-1209's `#ifdef HAVE_X_WINDOWS` arm (`FRAME_X_P` and
	// `x_relative_mouse_position`) is dead on this target (docs/ports/menu.md).
	  {
	    let mut bar_window: Lisp_Object = lisp_h_Qnil ();
	    let mut part: scroll_bar_part = 0;
	    let mut time: Time = 0;
	    let mouse_position_hook: Option<unsafe extern "C-unwind" fn (*mut *mut c_void, c_int,
								   *mut Lisp_Object,
								   *mut scroll_bar_part,
								   *mut Lisp_Object,
								   *mut Lisp_Object,
								   *mut Time)> =
	      (*FRAME_TERMINAL! (new_f)).mouse_position_hook;

	    if let Some (hook) = mouse_position_hook
	      { hook (&mut new_f as *mut *mut frame as *mut *mut c_void, 1, &mut bar_window,
		      &mut part, &mut x, &mut y, &mut time); }
	  }

	if !new_f.is_null ()
	  { XSETFRAME! (window, new_f); }
	else
	  {
	    window = (*selected_window.as_mut_ptr ());
	    XSETFASTINT! (x, 0);
	    XSETFASTINT! (y, 0);
	  }
      }

    /* Decode where to put the menu.  */

    if FRAMEP (window)
      {
	f = XFRAME (window);
	xpos = 0;
	ypos = 0;
      }
    else if WINDOWP (window)
      {
	let win: *mut window = XWINDOW (window);
	CHECK_LIVE_WINDOW (window);
	f = XFRAME (WINDOW_FRAME! (win));

	if FIXNUMP (tem2)
	  {
	    /* Clicks in the text area, where TEM2 is a buffer
	       position, are relative to the top-left edge of the text
	       area, see keyboard.c:make_lispy_position.  */
	    xpos = window_box_left (win, TEXT_AREA);
	    ypos = (WINDOW_TOP_EDGE_Y! (win)
		    + WINDOW_TAB_LINE_HEIGHT! (win)
		    + WINDOW_HEADER_LINE_HEIGHT! (win));
	  }
	else
	  {
	    xpos = WINDOW_LEFT_EDGE_X! (win);
	    ypos = WINDOW_TOP_EDGE_Y! (win);
	  }
      }
    else
      /* ??? Not really clean; should be Qwindow_or_framep
	 but I don't want to make one now.  */
      { wrong_type_argument (Qwindowp (), window); }

    xpos += check_integer_range (x,
				 if xpos < INT_MIN - MOST_NEGATIVE_FIXNUM as c_int
				   { (INT_MIN as EMACS_INT - xpos as EMACS_INT) as intmax_t }
				   else { MOST_NEGATIVE_FIXNUM as intmax_t },
				 (INT_MAX as EMACS_INT - xpos as EMACS_INT) as intmax_t)
      as c_int;
    ypos += check_integer_range (y,
				 if ypos < INT_MIN - MOST_NEGATIVE_FIXNUM as c_int
				   { (INT_MIN as EMACS_INT - ypos as EMACS_INT) as intmax_t }
				   else { MOST_NEGATIVE_FIXNUM as intmax_t },
				 (INT_MAX as EMACS_INT - ypos as EMACS_INT) as intmax_t)
      as c_int;

    XSETFRAME! ((*Vmenu_updating_frame.as_mut_ptr ()), f);
  }

  /* Now parse the lisp menus.  */
  record_unwind_protect_void (Some (unuse_menu_items));

  title = Qnil ();

  /* Decode the menu items from what was specified.  */

  keymap = get_keymap (menu, false, false);
  if CONSP (keymap)
    {
      /* We were given a keymap.  Extract menu info from the keymap.  */
      let prompt: Lisp_Object;

      /* Extract the detailed info to make one pane.  */
      keymap_panes (&raw mut menu, 1);

      /* Search for a string appearing directly as an element of the keymap.
	 That string is the title of the menu.  */
      prompt = Fkeymap_prompt (keymap);

      // C 1307-1312's `#if defined (USE_GTK) || defined (HAVE_NS)` arm (the `Qhide`
      // text-property test) is dead on this target (docs/ports/menu.md).
      if !NILP (prompt)
	{ title = prompt; }

      /* Make that be the pane title of the first pane.  */
      if !NILP (prompt) && (*menu_items_n_panes.as_mut_ptr ()) >= 0
	{ ASET ((*menu_items.as_mut_ptr ()), MENU_ITEMS_PANE_NAME as isize, prompt); }

      menuflags |= MENU_KEYMAPS;
    }
  else if CONSP (menu) && KEYMAPP! (XCAR (menu))
    {
      /* We were given a list of keymaps.  */
      let nmaps: ptrdiff_t = list_length (menu);
      let maps: *mut Lisp_Object;
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      let mut i: ptrdiff_t;

      SAFE_ALLOCA_LISP! (sa_avail, maps, nmaps);
      title = Qnil ();

      /* The first keymap that has a prompt string
	 supplies the menu title.  */
      tem = menu;
      i = 0;
      while CONSP (tem)
	{
	  let prompt: Lisp_Object;

	  *maps.offset (i) = { keymap = get_keymap (XCAR (tem), true, false); keymap };
	  i += 1;

	  prompt = Fkeymap_prompt (keymap);
	  if NILP (title) && !NILP (prompt)
	    { title = prompt; }
	  tem = XCDR (tem);
	}

      /* Extract the detailed info to make one pane.  */
      keymap_panes (maps, nmaps);

      /* Make the title be the pane title of the first pane.  */
      if !NILP (title) && (*menu_items_n_panes.as_mut_ptr ()) >= 0
	{ ASET ((*menu_items.as_mut_ptr ()), MENU_ITEMS_PANE_NAME as isize, title); }

      menuflags |= MENU_KEYMAPS;

      SAFE_FREE! (sa_count, sa_mark);
    }
  else
    {
      /* We were given an old-fashioned menu.  */
      title = Fcar (menu);
      CHECK_STRING (title);

      list_of_panes (Fcdr (menu));

      menuflags &= !MENU_KEYMAPS;
    }

  unbind_to (specpdl_count, Qnil ());

  // C 1371-1374's `#ifdef HAVE_WINDOW_SYSTEM` arm (`Fx_hide_tip`), C 1377-1388's
  // `#ifdef HAVE_NTGUI` block, C 1395-1403's `#ifdef HAVE_WINDOW_SYSTEM` arm
  // (`cancel_hourglass`) and C 1420-1427's `#ifdef HAVE_NTGUI` block are dead on this target
  // (docs/ports/menu.md).

  record_unwind_protect_void (Some (discard_menu_items));

  run_hook (Qx_pre_popup_menu_hook ());

  /* Display them in a menu, but not if F is the initial frame that
     doesn't have its hooks set (e.g., in a batch session), because
     such a frame cannot display menus.  */
  if !FRAME_INITIAL_P (f)
    {
      if FRAME_TERMCAP_P (f) && !NILP ((*Vx_popup_menu_function.as_mut_ptr ()))
	{ selection = calln! ((*Vx_popup_menu_function.as_mut_ptr ()), position, menu); }
      else if let Some (hook) = (*FRAME_TERMINAL! (f)).menu_show_hook
	// C calls the hook unconditionally (`FRAME_TERMINAL (f)->menu_show_hook (...)`);
	// the port's `Option` carries the same call and skips it only for the NULL hook C
	// would have dereferenced (docs/ports/menu.md).
	{ selection = hook (f as *mut c_void, xpos, ypos, menuflags,
			    title, &mut error_name); }
    }

  unbind_to (specpdl_count, Qnil ());

  if !error_name.is_null ()
    { error (c"%s".as_ptr (), error_name); }
  return selection;

  }
}

DEFUN! ("x-popup-menu", Fx_popup_menu, Sx_popup_menu, 2, 2, 0,
       doc: "Pop up a deck-of-cards menu and return user's selection.\nPOSITION is a position specification.  This is either a mouse button event\nor a list ((XOFFSET YOFFSET) WINDOW)\nwhere XOFFSET and YOFFSET are positions in pixels from the top left\ncorner of WINDOW.  (WINDOW may be a window or a frame object.)\nThis controls the position of the top left of the menu as a whole.\nIf POSITION is t, it means to use the current mouse position.\n\nMENU is a specifier for a menu.  For the simplest case, MENU is a keymap.\nThe menu items come from key bindings that have a menu string as well as\na definition; actually, the \"definition\" in such a key binding looks like\n(STRING . REAL-DEFINITION).  To give the menu a title, put a string into\nthe keymap as a top-level element.\n\nIf REAL-DEFINITION is nil, that puts a nonselectable string in the menu.\nOtherwise, REAL-DEFINITION should be a valid key binding definition.\n\nYou can also use a list of keymaps as MENU.\n  Then each keymap makes a separate pane.\n\nWhen MENU is a keymap or a list of keymaps, the return value is the\nlist of events corresponding to the user's choice. Note that\n`x-popup-menu' does not actually execute the command bound to that\nsequence of events.\n\nAlternatively, you can specify a menu of multiple panes\n  with a list of the form (TITLE PANE1 PANE2...),\nwhere each pane is a list of form (TITLE ITEM1 ITEM2...).\nEach ITEM is normally a cons cell (STRING . VALUE);\nbut a string can appear as an item--that makes a nonselectable line\nin the menu.\nWith this form of menu, the return value is VALUE from the chosen item.\n\nIf POSITION is nil, don't display the menu at all, just precalculate the\ncached information about equivalent key sequences.\n\nIf the user gets rid of the menu without making a valid choice, for\ninstance by clicking the mouse away from a valid choice or by typing\nkeyboard input, then this normally results in a quit and\n`x-popup-menu' does not return.  But if POSITION is a mouse button or\ntouch screen event (indicating that the user invoked the menu with the\na pointing device) then no quit occurs and `x-popup-menu' returns\nnil.",
       (position: Lisp_Object, menu: Lisp_Object),
       {
  init_raw_keybuf_count ();
  return x_popup_menu_1 (position, menu);
});

/* If F's terminal is not capable of displaying a popup dialog,
   emulate it with a menu.  */

unsafe fn emulate_dialog_with_menu (f: *mut frame, contents: Lisp_Object) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut x: Lisp_Object;
  let mut y: Lisp_Object;
  let frame: Lisp_Object;
  let newpos: Lisp_Object;
  let prompt: Lisp_Object = Fcar (contents);
  let mut x_coord: c_int;
  let mut y_coord: c_int;

  if FRAME_WINDOW_P (f)
    {
      x_coord = FRAME_PIXEL_WIDTH (f);
      y_coord = FRAME_PIXEL_HEIGHT (f);
    }
  else
    {
      x_coord = FRAME_COLS (f);
      /* Center the title at frame middle.  (TTY menus have
	 their upper-left corner at the given position.)  */
      if STRINGP (prompt)
	{ x_coord -= SCHARS (prompt) as c_int; }
      y_coord = FRAME_TOTAL_LINES (f);
    }

  XSETFRAME! (frame, f);
  XSETINT! (x, (x_coord / 2) as EMACS_INT);
  XSETINT! (y, (y_coord / 2) as EMACS_INT);
  newpos = list2 (list2 (x, y), frame);

  return Fx_popup_menu (newpos, list2 (prompt, contents));

  }
}

DEFUN! ("x-popup-dialog", Fx_popup_dialog, Sx_popup_dialog, 2, 3, 0,
       doc: "Pop up a dialog box and return user's selection.\nPOSITION specifies which frame to use.\nThis is normally a mouse button event or a window or frame.\nIf POSITION is t, it means to use the frame the mouse is on.\nThe dialog box appears in the middle of the specified frame.\n\nCONTENTS specifies the alternatives to display in the dialog box.\nIt is a list of the form (DIALOG ITEM1 ITEM2...).\nEach ITEM is a cons cell (STRING . VALUE).\nThe return value is VALUE from the chosen item.\n\nAn ITEM may also be just a string--that makes a nonselectable item.\nAn ITEM may also be nil--that means to put all preceding items\non the left of the dialog box and all following items on the right.\n(By default, approximately half appear on each side.)\n\nIf HEADER is non-nil, the frame title for the box is \"Information\",\notherwise it is \"Question\".\n\nIf the user gets rid of the dialog box without making a valid choice,\nfor instance using the window manager, then this produces a quit and\n`x-popup-dialog' does not return.",
       (position: Lisp_Object, contents: Lisp_Object, header: Lisp_Object),
       {
  let mut f: *mut frame = std::ptr::null_mut ();
  let mut window: Lisp_Object;

  /* Decode the first argument: find the window or frame to use.  */
  if EQ (position, Qt ())
      || (CONSP (position) && (EQ (XCAR (position), Qmenu_bar ())
			       || EQ (XCAR (position), Qtab_bar ())
			       || EQ (XCAR (position), Qtool_bar ())))
    { window = (*selected_window.as_mut_ptr ()); }
  else if CONSP (position)
    {
      let mut tem: Lisp_Object = XCAR (position);
      if CONSP (tem)
	{ window = Fcar (XCDR (position)); }
      else
	{
	  tem = Fcar (XCDR (position));  /* EVENT_START (position) */
	  window = Fcar (tem);	     /* POSN_WINDOW (tem) */
	}
    }
  else if WINDOWP (position) || FRAMEP (position)
    { window = position; }
  else
    { window = Qnil (); }

  /* Decode where to put the menu.  */

  if FRAMEP (window)
    { f = XFRAME (window); }
  else if WINDOWP (window)
    {
      CHECK_LIVE_WINDOW (window);
      f = XFRAME (WINDOW_FRAME! (XWINDOW (window)));
    }
  else
    /* ??? Not really clean; should be CHECK_WINDOW_OR_FRAME,
       but I don't want to make one now.  */
    { CHECK_WINDOW (window); }

  /* Note that xw_popup_dialog can call menu code, so
     Vmenu_updating_frame should be set (Bug#17891).  */
  eassume! (!f.is_null () && FRAME_LIVE_P (f));
  XSETFRAME! ((*Vmenu_updating_frame.as_mut_ptr ()), f);

  /* Force a redisplay before showing the dialog.  If a frame is created
     just before showing the dialog, its contents may not have been fully
     drawn, as this depends on timing of events from the X server.  Redisplay
     is not done when a dialog is shown.  If redisplay could be done in the
     X event loop (i.e. the X event loop does not run in a signal handler)
     this would not be needed.

     Do this before creating the widget value that points to Lisp
     string contents, because Fredisplay may GC and relocate them.  */
  Fredisplay (Qt ());

  /* Display the popup dialog by a terminal-specific hook ... */
  if let Some (hook) = (*FRAME_TERMINAL! (f)).popup_dialog_hook
    {
      let selection: Lisp_Object
	= hook (f as *mut c_void, header, contents);
      // C 1600-1605's `#ifdef HAVE_NTGUI` arm (the `unsupported--w32-dialog` test) is dead
      // on this target (docs/ports/menu.md).
	return selection;
    }
  /* ... or emulate it with a menu.  */
  return emulate_dialog_with_menu (f, contents);
});

/* C's two `DEFVAR_LISP` variables live in the generated globals.h in C; the port's
   globals.rs mirrors them as declarations and the owning unit defines them (P-21,
   status 3.29, the filelock.rs shape).  */
#[unsafe(no_mangle)]
pub static Vx_pre_popup_menu_hook: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vx_popup_menu_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_menu ()
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  (*menu_items.as_mut_ptr ()) = Qnil ();
  staticpro (menu_items.as_ptr ());

  DEFSYM! (Qhide, c"hide");
  DEFSYM! (Qx_pre_popup_menu_hook, c"x-pre-popup-menu-hook");

  DEFVAR_LISP! (c"x-pre-popup-menu-hook", Vx_pre_popup_menu_hook,
		"Hook run before `x-popup-menu' displays a popup menu.\nIt is only run before the menu is really going to be displayed.  It\nwon't be run if `x-popup-menu' fails or returns for some other reason\n(such as the keymap is invalid).");
  (*Vx_pre_popup_menu_hook.as_mut_ptr ()) = Qnil ();

  DEFVAR_LISP! (c"x-popup-menu-function", Vx_popup_menu_function,
		"Function to call to pop up a menu.\n The function is called like `x-popup-menu'.  This is currently only\n used for frames on text terminals.");
  (*Vx_popup_menu_function.as_mut_ptr ()) = Qnil ();

  defsubr (Sx_popup_menu.as_mut_ptr ());
  defsubr (Sx_popup_dialog.as_mut_ptr ());
  defsubr (Smenu_bar_menu_at_x_y.as_mut_ptr ());

  }
}
