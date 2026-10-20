// femacs: port of src/androidmenu.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same
// error model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2);
// the C `static` helpers stay plain `unsafe fn` except the three C passes as function pointers
// (they are `unsafe extern "C" fn`, the `callproc.rs` `call_process_kill` shape).
//
// This is P-86's first unit: the frozen configuration compiles no `androidmenu.o`
// (`configure.ac` 2598's `ANDROID_OBJ`, under `AC_DEFINE([HAVE_ANDROID])` 2604), so
// `extract-abi.py`'s checklist for the file is empty and P-86 carries **both arms** of every
// `#if` here instead.  Two gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID`) is `feature = "have_android"`, on every item *and on C
//     22-27's include graph* - which is why the frozen configuration compiles nothing here;
//   * C 29's `#ifndef ANDROID_STUBIFY` / C 829's `#else` (with C 850-853's nested `#ifndef`) is
//     `not_android_stubify`, the feature `alloc.rs` registered for C's
//     `HAVE_ANDROID && !defined ANDROID_STUBIFY`.  The real arm is therefore
//     `all (have_android, not_android_stubify)` and the stub arm `all (have_android,
//     not (not_android_stubify))`; neither is compiled outside an Android configuration.
//
// `docs/ports/androidmenu.md` carries the C line ranges, the acceptance readings and every
// deviation (the excluded `ANDROID_DELETE_LOCAL_REF` whose premise `include/jni.rs` superseded,
// the `(jboolean)` casts `E0617` promotes to `c_int`, the uninitialised locals, the `goto` token
// and the line-count tolerance this file needs).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");

/* C 22-27: `lisp.h`, `androidterm.h`, `android.h`, `blockinput.h`, `keyboard.h` and `menu.h`,
   all outside C 29's `#ifndef` but reached by no translation unit the frozen configuration
   builds, so the whole graph sits under the file gate (see the header).  The C `#if`-gated
   `<android/log.h>` (C 31) has no mirror; its one entry point is declared below.  */
#[cfg (feature = "have_android")]
#[cfg (feature = "have_android")]
include!("include/lisp.rs");
#[cfg (feature = "have_android")]
#[cfg (feature = "have_android")]
include!("include/androidterm.rs");
#[cfg (feature = "have_android")]
#[cfg (feature = "have_android")]
include!("include/android.rs");
#[cfg (feature = "have_android")]
#[cfg (feature = "have_android")]
include!("include/blockinput.rs");
#[cfg (feature = "have_android")]
#[cfg (feature = "have_android")]
include!("include/keyboard.rs");
#[cfg (feature = "have_android")]
#[cfg (feature = "have_android")]
include!("include/menu.rs");

/* C 31: `#include <android/log.h>` and the `ANDROID_LOG_FATAL` its one use site names - the
   NDK's own header, not part of the reference tree, so it has no mirror and the port declares
   what it uses (P-49/P-70, the `jni.h` class).  Measured with the NDK's clang
   (`~/Library/Android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64`):
   `log.h` 112 `int __android_log_print(int prio, const char *tag, const char *fmt, ...)`,
   and `log.h` 74-89's enum gives `ANDROID_LOG_FATAL == 7` (both measured by compilation for
   `--target=aarch64-linux-android24`).  */
#[cfg (feature = "have_android")]
pub const ANDROID_LOG_FATAL: c_int = 7;

/* `android_get_current_api_level!` (android.rs) names `__ANDROID_API__` and leaves it to the
   expansion site ("the NDK's toolchain defines it"); this unit is its first.  The NDK defines it
   as `__ANDROID_MIN_SDK_VERSION__` (measured 24 for the `--target=aarch64-linux-android24` that
   `spikes/jni-probe.sh` uses).  */
#[cfg (feature = "have_android")]
pub const __ANDROID_API__: c_int = 24;

#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}

/* Seven declarations the reference reaches through the six headers above, which the frozen
   mirrors do not carry yet (P-72 rule 4: a mirror must reach what it needs through its own
   `include!`s).  Until they land there the unit supplies them at the use site - the
   `alloc.rs` `mark_haiku_display` / `android-asset.rs` `ESPIPE` shape; each line names the C
   header it comes from, and `docs/ports/androidmenu.md` records the gap for the integrator.  */
#[cfg (feature = "have_android")]
unsafe extern "C" {
                                     // lisp.h 4786
                                 // lisp.h 3880
  pub fn memory_full (size: size_t) -> !;                            // lisp.h 4432 (`AVOID`)
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);  // lisp.h 4825
  pub fn menu_separator_name_p (name: *const c_char) -> bool;        // keyboard.h 473
  pub fn check_window_system (f: *mut frame);                        // frame.h 1504
                                        // keyboard.h 385
}

/* ------------------------------------------------------------------------------------------
   C 29-827: the `#ifndef ANDROID_STUBIFY` arm - the real JNI implementation.  Every item is
   gated on `all (feature = "have_android", feature = "not_android_stubify")`; C 829-837's
   `#else` arm is at the bottom of this file, and the three items C 839-860 defines outside the
   `#ifndef` are gated on the file gate alone.  */

/* Flag indicating whether or not a popup menu has been posted and not yet popped down.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut popup_activated_flag: c_int = 0;                    // C 36 (tentative, i.e. zero)

/* Serial number used to identify which context menu events are associated with the context
   menu currently being displayed.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut current_menu_serial: c_uint = 0;                // C 41 (tentative, i.e. zero)

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
unsafe fn popup_activated () -> c_int                          // C 43-47
{
  return popup_activated_flag;
}

/* Toolkit menu implementation.  */

/* Structure describing the EmacsContextMenu class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_context_menu                           // C 55-65
{
  pub class: jclass,
  pub create_context_menu: jmethodID,
  pub add_item: jmethodID,
  pub add_submenu: jmethodID,
  pub add_pane: jmethodID,
  pub parent: jmethodID,
  pub display: jmethodID,
  pub dismiss: jmethodID,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* Identifiers associated with the EmacsContextMenu class.  C's `static struct` is a tentative
   definition, i.e. every member zero (the `bidi.rs` shape of PORTING.md 0.1-13/1.5; the ledger
   names it).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut menu_class: android_emacs_context_menu               // C 68
  = android_emacs_context_menu { class: std::ptr::null_mut (),
                                 create_context_menu: std::ptr::null_mut (),
                                 add_item: std::ptr::null_mut (),
                                 add_submenu: std::ptr::null_mut (),
                                 add_pane: std::ptr::null_mut (),
                                 parent: std::ptr::null_mut (),
                                 display: std::ptr::null_mut (),
                                 dismiss: std::ptr::null_mut () };
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C's `ANDROID_DELETE_LOCAL_REF` (android.h 340-342) expands to a `<jni.h>` vtable read.  The
   mirror records it as an excluded macro whose premise ("`JNINativeInterface` has no mirror")
   predates `include/jni.rs`, which now carries that table (all 230 entries, `spikes/jni-probe.sh`);
   the port defines the expansion under C's own name so the token survives at every use site.  */
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_context_menu ()                    // C 70-119
{
  let mut old: jclass;

  menu_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsContextMenu".as_ptr ());
  eassert! (! menu_class.class.is_null ());

  old = menu_class.class;
  menu_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if menu_class.class.is_null ()
    { emacs_abort (); }

  /* C 89-101: `FIND_METHOD`/`FIND_METHOD_STATIC`, the two macros this function defines and
     `#undef`s at C 117-118; Rust's `macro_rules!` scope ends with the block, so the `#undef`s
     have no counterpart.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      menu_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          menu_class.class,
                                                          $name, $signature);
      eassert! (! menu_class.$c_name.is_null ());
    };
  }

  macro_rules! FIND_METHOD_STATIC {
    ($c_name:ident, $name:expr, $signature:expr) => {
      menu_class.$c_name
        = ((*(*android_java_env)).GetStaticMethodID.unwrap ()) (android_java_env,
                                                               menu_class.class,
                                                               $name, $signature);
      eassert! (! menu_class.$c_name.is_null ());
    };
  }

  FIND_METHOD_STATIC! (create_context_menu, c"createContextMenu".as_ptr (),
                       c"(Ljava/lang/String;)Lorg/femacs/EmacsContextMenu;".as_ptr ());

  FIND_METHOD! (add_item, c"addItem".as_ptr (),
                c"(ILjava/lang/String;ZZZLjava/lang/String;Z)V".as_ptr ());
  FIND_METHOD! (add_submenu, c"addSubmenu".as_ptr (),
                c"(Ljava/lang/String;Ljava/lang/String;)Lorg/femacs/EmacsContextMenu;".as_ptr ());
  FIND_METHOD! (add_pane, c"addPane".as_ptr (), c"(Ljava/lang/String;)V".as_ptr ());
  FIND_METHOD! (parent, c"parent".as_ptr (),
                c"()Lorg/femacs/EmacsContextMenu;".as_ptr ());
  FIND_METHOD! (display, c"display".as_ptr (), c"(Lorg/femacs/EmacsWindow;III)Z".as_ptr ());
  FIND_METHOD! (dismiss, c"dismiss".as_ptr (), c"(Lorg/femacs/EmacsWindow;)V".as_ptr ());
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unwind_local_frame ()              // C 121-125
{
  ((*(*android_java_env)).PopLocalFrame.unwrap ()) (android_java_env, std::ptr::null_mut ());
}

/* Push a local reference frame to the JVM stack and record it on the specpdl.  Release local
   references created within that frame when the specpdl is unwound past where it is after
   returning.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_push_local_frame ()                           // C 131-143
{
  let mut rc: c_int;

  rc = ((*(*android_java_env)).PushLocalFrame.unwrap ()) (android_java_env, 30);

  /* This means the JVM ran out of memory.  */
  if rc < 1
    { android_exception_check (); }

  record_unwind_protect_void (Some (android_unwind_local_frame));
}

/* Data for android_dismiss_menu.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_dismiss_menu_data                            // C 147-154
{
  /* The menu object.  */
  pub menu: jobject,

  /* The window object.  */
  pub window: jobject,
}

/* Cancel the context menu passed in POINTER.  Also, clear popup_activated_flag.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_dismiss_menu (pointer: *mut c_void)  // C 159-171
{
  let data: *mut android_dismiss_menu_data;

  data = pointer as *mut android_dismiss_menu_data;
  ((*(*android_java_env)).CallNonvirtualVoidMethod.unwrap ())
    (android_java_env,
     (*data).menu,
     menu_class.class,
     menu_class.dismiss,
     (*data).window);
  popup_activated_flag = 0;
}

/* Recursively process events until a ANDROID_CONTEXT_MENU event arrives.  Then, return the item
   ID specified in the event in *ID.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_process_events_for_menu (id: *mut c_int)      // C 177-212
{
  let mut blocked: c_int;

  /* Set menu_event_id to -1; handle_one_android_event will set it to the event ID upon
     receiving a context menu event.  This can cause a non-local exit.  */
  (*x_display_list).menu_event_id = -1;

  /* Unblock input completely.  */
  blocked = interrupt_input_blocked;
  totally_unblock_input ();

  /* Now wait for the menu event ID to change.  */
  while (*x_display_list).menu_event_id == -1
    {
      /* Wait for events to become available.  */
      android_wait_event ();

      /* Process pending signals.  */
      process_pending_signals ();

      /* Maybe quit.  This is important because the framework (on Android 4.0.3) can sometimes
	 fail to deliver context menu closed events if a submenu was opened, and the user still
	 needs to be able to quit.  */
      maybe_quit ();
    }

  /* Restore the input block.  */
  interrupt_input_blocked = blocked;

  /* Return the ID.  */
  *id = (*x_display_list).menu_event_id;
}

/* Structure describing a ``subprefix'' in the menu.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_menu_subprefix                               // C 216-223
{
  /* The subprefix above.  */
  pub last: *mut android_menu_subprefix,

  /* The subprefix itself.  */
  pub subprefix: Lisp_Object,
}

/* Free the subprefixes starting from *DATA.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_free_subprefixes (data: *mut c_void)  // C 227-241
{
  let head: *mut *mut android_menu_subprefix;
  let mut subprefix: *mut android_menu_subprefix;

  head = data as *mut *mut android_menu_subprefix;

  while !(*head).is_null ()
    {
      subprefix = *head;
      *head = (*subprefix).last;

      xfree (subprefix as *mut c_void);
    }
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* `<intprops.h>`'s `TYPE_MAXIMUM (t)`: C's `lisp.h` 42 reaches that header, the frozen mirror
   (`femacs/lib/include/intprops.rs`) keeps C's own text and is not compiled, and the units that
   need the semantics write the Rust primitive at the use site - `editfns.rs`'s identical
   two-line macro is the precedent.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_menu_show (f: *mut frame, x: c_int, y: c_int, menuflags: c_int,
                                 title: Lisp_Object, error_name: *mut *const c_char)
  -> Lisp_Object                                                // C 243-593
{
  let mut context_menu: jobject;
  let mut current_context_menu: jobject;
  let mut title_string: jobject;
  let mut help_string: jobject;
  let mut temp: jobject;
  let mut i: size_t;
  let mut pane_name: Lisp_Object;
  let mut prefix: Lisp_Object;
  let mut count: specpdl_ref;
  let mut count1: specpdl_ref;
  let mut item_name: Lisp_Object;
  let mut enable: Lisp_Object;
  let mut def: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut entry: Lisp_Object;
  let mut r#type: Lisp_Object;
  let mut selected: Lisp_Object;
  let mut help: Lisp_Object;
  let mut method: jmethodID;
  let mut store: jobject;
  let mut rc: bool;
  let mut window: jobject;
  let mut id: c_int = 0;            // C 258; C 511 takes its address (PORTING.md 1.5)
  let mut item_id: c_int;
  let mut submenu_depth: c_int;
  /* C 259: `struct android_dismiss_menu_data data;` - C stores its two members later (C
     505-506); Rust cannot partially initialise a value (E0381, PORTING.md 0.1-13), so the record
     starts zeroed and the two stores stay where C has them (the `bidi.rs` shape).  */
  let mut data: android_dismiss_menu_data = std::mem::zeroed ();
  let mut subprefix: *mut android_menu_subprefix;
  let mut temp_subprefix: *mut android_menu_subprefix;
  let mut subprefix_1: *mut android_menu_subprefix;
  let mut checkmark: bool;
  let mut serial: c_uint;
  let mut env: *mut JNIEnv;

  count = SPECPDL_INDEX ();
  current_menu_serial += 1;                 // C 267: `serial = ++current_menu_serial;`
  serial = current_menu_serial;
  env = android_java_env;

  block_input ();

  /* Push the first local frame.  */
  android_push_local_frame ();

  /* Set title_string to a Java string containing TITLE if non-nil.  If the menu consists of more
     than one pane, replace the title with the pane header item so that the menu looks
     consistent.  */

  title_string = std::ptr::null_mut ();
  if STRINGP (title) && menu_items_n_panes < 2
    { title_string = android_build_string (title, std::ptr::null_mut::<c_void> ()); }

  /* Push the first local frame for the context menu.  */
  method = menu_class.create_context_menu;
  /* C 285-286's chained assignment stores once and yields the value; in Rust an assignment
     yields `()`, so the same store is written twice.  */
  context_menu
    = ((*(*android_java_env)).CallStaticObjectMethod.unwrap ()) (android_java_env,
                                                                 menu_class.class,
                                                                 method,
                                                                 title_string);
  current_context_menu = context_menu;

  /* Delete the unused title reference.  */

  if !title_string.is_null ()
    { ANDROID_DELETE_LOCAL_REF! (title_string); }

  /* Push the second local frame for temporaries.  */
  count1 = SPECPDL_INDEX ();
  android_push_local_frame ();

  /* Iterate over the menu.  */
  i = 0; submenu_depth = 0;

  while i < menu_items_used as size_t
    {
      if NILP (AREF (menu_items, i as isize))
	{
	  /* This is the start of a new submenu.  However, it can be ignored here.  */
	  i += 1;
	  submenu_depth += 1;
	}
      else if EQ (AREF (menu_items, i as isize), Qlambda ())
	{
	  /* This is the end of a submenu.  Go back to the previous context menu.  */
	  store = current_context_menu;
	  current_context_menu
	    = ((*(*env)).CallNonvirtualObjectMethod.unwrap ()) (env,
                                                                current_context_menu,
                                                                menu_class.class,
                                                                menu_class.parent);
	  android_exception_check ();

	  if store != context_menu
	    { ANDROID_DELETE_LOCAL_REF! (store); }
	  i += 1;
	  submenu_depth -= 1;

	  if current_context_menu.is_null () || submenu_depth < 0
	    {
	      /* C 331-332: `__android_log_print (ANDROID_LOG_FATAL, __func__, "…")` - C's
		 `__func__` is this function's own name, and the port keeps the name.  */
	      let __func__: *const c_char = c"android_menu_show".as_ptr ();
	      __android_log_print (ANDROID_LOG_FATAL, __func__,
	                           c"unbalanced submenu pop in menu_items".as_ptr ());
	      emacs_abort ();
	    }
	}
      else if EQ (AREF (menu_items, i as isize), Qt ())
	       && submenu_depth != 0
	{ i += MENU_ITEMS_PANE_LENGTH as size_t; }
      else if EQ (AREF (menu_items, i as isize), Qquote ())
	{ i += 1; }
      else if EQ (AREF (menu_items, i as isize), Qt ())
	{
	  /* If the menu contains a single pane, then the pane is actually TITLE.  Don't duplicate
	     the text within the context menu title.  C 347-348 writes that skip as
	     `if (menu_items_n_panes < 2) goto next_item;` and C 382-383's `next_item:` label is
	     this branch's `i += MENU_ITEMS_PANE_LENGTH`; the port writes the jump as the labeled
	     block `'next_item` (PORTING.md 2.2).  */
	  'next_item: {
	    if menu_items_n_panes < 2
	      { break 'next_item; }

	    /* This is a new pane.  Switch back to the topmost context menu.  */
	      if current_context_menu != context_menu
		{ ANDROID_DELETE_LOCAL_REF! (current_context_menu); }
	      current_context_menu = context_menu;

	      /* Now figure out the title of this pane.  */
	      pane_name = AREF (menu_items, i as isize + MENU_ITEMS_PANE_NAME as isize);
	      prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);

	      /* PANE_NAME may be nil, in which case it must be set to an empty string.  */

	      if NILP (pane_name)
		{ pane_name = empty_unibyte_string; }

	      /* Remove the leading prefix character if need be.  */

	      if menuflags & MENU_KEYMAPS != 0 && ! NILP (prefix)
		 && SCHARS (prefix) != 0
		{ pane_name = Fsubstring (pane_name, make_fixnum (1), Qnil ()); }

	      /* Add the pane.  */
	      temp = android_build_string (pane_name, std::ptr::null_mut::<c_void> ());
	      android_exception_check ();

	      ((*(*env)).CallNonvirtualVoidMethod.unwrap ()) (env, current_context_menu,
                                                              menu_class.class,
                                                              menu_class.add_pane, temp);
	      android_exception_check ();
	      ANDROID_DELETE_LOCAL_REF! (temp);
	    }

	  /* C 382-383: the `next_item:` label.  */
	  i += MENU_ITEMS_PANE_LENGTH as size_t;
	}
      else
	{
	  item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);
	  enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);
	  def = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_DEFINITION as isize);
	  r#type = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_TYPE as isize);
	  selected = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_SELECTED as isize);
	  help = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_HELP as isize);

	  /* This is an actual menu item (or submenu).  Add it to the menu.  */

	  if (i + MENU_ITEMS_ITEM_LENGTH as size_t) < menu_items_used as size_t
	     && NILP (AREF (menu_items, i as isize + MENU_ITEMS_ITEM_LENGTH as isize))
	    {
	      /* This is a submenu.  Add it.  */
	      title_string = if ! NILP (item_name)
			     { android_build_string (item_name, std::ptr::null_mut::<c_void> ()) }
			   else
			     { std::ptr::null_mut () };
	      help_string = std::ptr::null_mut ();

	      /* Menu items can have tool tips on Android 26 and later.  In this case, set it to the
		 help string.  */

	      if android_get_current_api_level! () >= 26 && STRINGP (help)
		{ help_string = android_build_string (help, std::ptr::null_mut::<c_void> ()); }

	      store = current_context_menu;
	      current_context_menu
		= ((*(*env)).CallNonvirtualObjectMethod.unwrap ()) (env,
                                                                    current_context_menu,
                                                                    menu_class.class,
                                                                    menu_class.add_submenu,
                                                                    title_string,
                                                                    help_string);
	      android_exception_check ();

	      if store != context_menu
		{ ANDROID_DELETE_LOCAL_REF! (store); }

	      if !title_string.is_null ()
		{ ANDROID_DELETE_LOCAL_REF! (title_string); }

	      if !help_string.is_null ()
		{ ANDROID_DELETE_LOCAL_REF! (help_string); }
	    }
	  else if NILP (def) && menu_separator_name_p (SSDATA (item_name))
	    /* Ignore this separator item.  */
	    { }
	  else
	    {
	      /* Compute the item ID.  This is the index of value.  Make sure it doesn't overflow.
		 C 440 is `if (ckd_add (&item_id, i + MENU_ITEMS_ITEM_VALUE, 0)) memory_full (…)`:
		 the builtin adds C's `0`, stores the `size_t` sum in the `int item_id`, and reports
		 overflow when it does not fit.  The port keeps both halves (P-15's
		 `ckd_add`/`overflowing_add` family; `overflowing_add` alone cannot express the
		 narrowing store).  */
	      let (item_id_, add_overflow)
		= (i + MENU_ITEMS_ITEM_VALUE as size_t).overflowing_add (0);
	      if add_overflow || item_id_ > (TYPE_MAXIMUM! (jint) as size_t)
		{ memory_full (i + MENU_ITEMS_ITEM_VALUE as size_t
			           * std::mem::size_of::<Lisp_Object> ()); }
	      item_id = item_id_ as jint;

	      /* Add this menu item with the appropriate state.  */

	      title_string = if ! NILP (item_name)
			     { android_build_string (item_name, std::ptr::null_mut::<c_void> ()) }
			   else
			     { std::ptr::null_mut () };
	      help_string = std::ptr::null_mut ();

	      /* Menu items can have tool tips on Android 26 and later.  In this case, set it to the
		 help string.  */

	      if android_get_current_api_level! () >= 26 && STRINGP (help)
		{ help_string = android_build_string (help, std::ptr::null_mut::<c_void> ()); }

	      /* Determine whether or not to display a check box.  */

	      checkmark = EQ (r#type, QCtoggle ())
			  || EQ (r#type, QCradio ());

	      ((*(*env)).CallNonvirtualVoidMethod.unwrap ())
		(env,
		 current_context_menu,
		 menu_class.class,
		 menu_class.add_item,
		 item_id as jint,
		 title_string,
		 (! NILP (enable)) as jboolean as c_int,     // C 468: `(jboolean) !NILP (enable)`
		 checkmark as jboolean as c_int,             // C 469: `(jboolean) checkmark`
		 (! NILP (selected)) as jboolean as c_int,   // C 470: `(jboolean) !NILP (selected)`
		 help_string,
		 (EQ (r#type, QCradio ())) as jboolean as c_int);  // C 472-473
	      android_exception_check ();

	      if !title_string.is_null ()
		{ ANDROID_DELETE_LOCAL_REF! (title_string); }

	      if !help_string.is_null ()
		{ ANDROID_DELETE_LOCAL_REF! (help_string); }
	    }

	  i += MENU_ITEMS_ITEM_LENGTH as size_t;
	}
    }

  /* The menu has now been built.  Pop the second local frame.  */
  unbind_to (count1, Qnil ());

  /* C 484-593: the display half and the item search.  C's two `return`s (C 587-588 and C
     590-592) are the function's normal exit and the `finish:` path of the three `goto finish`
     jumps (C 502, C 515, C 519); the port writes the jump target as a `'finish` loop whose
     `break` is the `goto` and whose tail is the `finish:` block (PORTING.md 2.2: a forward jump
     to a cleanup path is not an early `return`).  */
  'finish: loop
    {
      /* Now, display the context menu.  */
      window = android_resolve_handle (FRAME_ANDROID_WINDOW! (f));
      rc = ((*(*env)).CallNonvirtualBooleanMethod.unwrap ()) (env, context_menu,
                                                              menu_class.class,
                                                              menu_class.display,
                                                              window, x as jint,
                                                              y as jint,
                                                              serial as jint)
	   != 0;
      android_exception_check ();

      if !rc
	/* This means displaying the menu failed.  */
	{ break 'finish; }

      /* Make sure the context menu is always dismissed.  */
      data.menu = context_menu;
      data.window = window;
      record_unwind_protect_ptr (Some (android_dismiss_menu), &raw mut data as *mut c_void);

      /* Next, process events waiting for something to be selected.  */
      popup_activated_flag = 1;
      android_process_events_for_menu (&raw mut id);

      if id == 0
	/* This means no menu item was selected.  */
	{ break 'finish; }

      /* This means the id is invalid.  */
      if id as isize >= ASIZE (menu_items)
	{ break 'finish; }

      /* Now return the menu item at that location.  */
      tem = Qnil ();
      subprefix = std::ptr::null_mut ();
      record_unwind_protect_ptr (Some (android_free_subprefixes), &raw mut subprefix as *mut c_void);

      /* Find the selected item, and its pane, to return the proper value.  */

      prefix = Qnil (); entry = Qnil ();
      i = 0;
      while i < menu_items_used as size_t
	{
	  if NILP (AREF (menu_items, i as isize))
	    {
	      temp_subprefix = xmalloc (std::mem::size_of::<android_menu_subprefix> ())
			       as *mut android_menu_subprefix;
	      (*temp_subprefix).last = subprefix;
	      subprefix = temp_subprefix;
	      (*subprefix).subprefix = prefix;

	      prefix = entry;
	      i += 1;
	    }
	  else if EQ (AREF (menu_items, i as isize), Qlambda ())
	    {
	      prefix = (*subprefix).subprefix;
	      temp_subprefix = (*subprefix).last;
	      xfree (subprefix as *mut c_void);
	      subprefix = temp_subprefix;

	      i += 1;
	    }
	  else if EQ (AREF (menu_items, i as isize), Qt ())
	    {
	      prefix = AREF (menu_items, i as isize + MENU_ITEMS_PANE_PREFIX as isize);
	      i += MENU_ITEMS_PANE_LENGTH as size_t;
	    }
	  /* Ignore a nil in the item list.  It's meaningful only for dialog boxes.  */
	  else if EQ (AREF (menu_items, i as isize), Qquote ())
	    { i += 1; }
	  else
	    {
	      entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);

	      if i + MENU_ITEMS_ITEM_VALUE as size_t == id as size_t
		{
		  if menuflags & MENU_KEYMAPS != 0
		    {
		      entry = list1 (entry);

		      if ! NILP (prefix)
			{ entry = Fcons (prefix, entry); }

		      subprefix_1 = subprefix;
		      while !subprefix_1.is_null ()
			{
			  if ! NILP ((*subprefix_1).subprefix)
			    { entry = Fcons ((*subprefix_1).subprefix, entry); }
			  subprefix_1 = (*subprefix_1).last;
			}
		    }

		  tem = entry;
		}
	      i += MENU_ITEMS_ITEM_LENGTH as size_t;
	    }
	}

      unblock_input ();
      return unbind_to (count, tem);
    }

  /* C 590-592: the `finish:` block.  */
  unblock_input ();
  return unbind_to (count, Qnil ());
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* Toolkit dialog implementation.  */

/* Structure describing the EmacsDialog class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct android_emacs_dialog                                 // C 601-607
{
  pub class: jclass,
  pub create_dialog: jmethodID,
  pub add_button: jmethodID,
  pub display: jmethodID,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* Identifiers associated with the EmacsDialog class.  Zero, as C's tentative definition is.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut dialog_class: android_emacs_dialog                   // C 610
  = android_emacs_dialog { class: std::ptr::null_mut (),
                           create_dialog: std::ptr::null_mut (),
                           add_button: std::ptr::null_mut (),
                           display: std::ptr::null_mut () };
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_emacs_dialog ()                          // C 612-651
{
  let mut old: jclass;

  dialog_class.class
    = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,
                                                    c"org/femacs/EmacsDialog".as_ptr ());
  eassert! (! dialog_class.class.is_null ());

  old = dialog_class.class;
  dialog_class.class
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env, old as jobject);
  ANDROID_DELETE_LOCAL_REF! (old);

  if dialog_class.class.is_null ()
    { emacs_abort (); }

  /* C 631-650: the two macros this function defines.  Unlike C 89-101's pair, C 638-646's
     `FIND_METHOD_STATIC` body carries *no* `eassert` (C 647 is the blank line its last
     backslash continues into), and the port keeps that difference.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      dialog_class.$c_name
        = ((*(*android_java_env)).GetMethodID.unwrap ()) (android_java_env,
                                                          dialog_class.class,
                                                          $name, $signature);
      eassert! (! dialog_class.$c_name.is_null ());
    };
  }

  macro_rules! FIND_METHOD_STATIC {
    ($c_name:ident, $name:expr, $signature:expr) => {
      dialog_class.$c_name
        = ((*(*android_java_env)).GetStaticMethodID.unwrap ()) (android_java_env,
                                                               dialog_class.class,
                                                               $name, $signature);
    };
  }

  FIND_METHOD_STATIC! (create_dialog, c"createDialog".as_ptr (),
                       c"(Ljava/lang/String;Ljava/lang/String;I)Lorg/femacs/EmacsDialog;"
                        .as_ptr ());
  FIND_METHOD! (add_button, c"addButton".as_ptr (), c"(Ljava/lang/String;IZ)V".as_ptr ());
  FIND_METHOD! (display, c"display".as_ptr (), c"()Z".as_ptr ());
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_dialog_show (f: *mut frame, title: Lisp_Object, header: Lisp_Object,
                               error_name: *mut *const c_char) -> Lisp_Object   // C 653-790
{
  let mut count: specpdl_ref;
  let mut dialog: jobject;
  let mut java_header: jobject;
  let mut java_title: jobject;
  let mut temp: jobject;
  let mut i: size_t;
  let mut item_name: Lisp_Object;
  let mut enable: Lisp_Object;
  let mut entry: Lisp_Object;
  let mut rc: bool;
  let mut id: c_int = 0;            // C 659; C 762 takes its address (PORTING.md 1.5)
  let mut method: jmethodID;
  let mut serial: c_uint;
  let mut env: *mut JNIEnv;

  /* Generate a unique ID for events from this dialog box.  */
  current_menu_serial += 1;                 // C 677: `serial = ++current_menu_serial;`
  serial = current_menu_serial;

  if menu_items_n_panes > 1
    {
      *error_name = c"Multiple panes in dialog box".as_ptr ();
      return Qnil ();
    }

  /* Do the initial setup.  */
  count = SPECPDL_INDEX ();
  *error_name = std::ptr::null ();

  android_push_local_frame ();

  /* Figure out what header to use.  */
  java_header = if ! NILP (header)
                  { android_build_jstring (c"Information".as_ptr ()) }
                else
                  { android_build_jstring (c"Question".as_ptr ()) };

  /* And the title.  */
  java_title = android_build_string (title, std::ptr::null_mut::<c_void> ());

  /* Now create the dialog.  */
  method = dialog_class.create_dialog;
  dialog = ((*(*android_java_env)).CallStaticObjectMethod.unwrap ()) (android_java_env,
                                                                     dialog_class.class,
                                                                     method, java_header,
                                                                     java_title,
                                                                     serial as jint);
  android_exception_check ();

  /* Delete now unused local references.  */
  if !java_header.is_null ()
    { ANDROID_DELETE_LOCAL_REF! (java_header); }
  ANDROID_DELETE_LOCAL_REF! (java_title);

  /* Save the JNI environment pointer prior to constructing the dialog, as typing
     (*android_java_env)->... gives rise to very long lines.  */
  env = android_java_env;

  /* Create the buttons.  */
  i = MENU_ITEMS_PANE_LENGTH as size_t;
  while i < menu_items_used as size_t
    {
      item_name = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_NAME as isize);
      enable = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_ENABLE as isize);

      /* Verify that there is no submenu here.  */

      if NILP (item_name)
	{
	  *error_name = c"Submenu in dialog items".as_ptr ();
	  return unbind_to (count, Qnil ());
	}

      /* Skip past boundaries between buttons on different sides.  The Android toolkit is too
	 silly to understand this distinction.  */

      if EQ (item_name, Qquote ())
	{ i += 1; }
      else
	{
	  /* Make sure i is within bounds.  */
	  if i > TYPE_MAXIMUM! (jint) as size_t
	    {
	      *error_name = c"Dialog box too big".as_ptr ();
	      return unbind_to (count, Qnil ());
	    }

	  /* Add the button.  */
	  temp = android_build_string (item_name, std::ptr::null_mut::<c_void> ());
	  ((*(*env)).CallNonvirtualVoidMethod.unwrap ()) (env, dialog,
                                                          dialog_class.class,
                                                          dialog_class.add_button,
                                                          temp, i as jint,
                                                          (NILP (enable)) as jboolean as c_int);
	  android_exception_check ();
	  ANDROID_DELETE_LOCAL_REF! (temp);
	  i += MENU_ITEMS_ITEM_LENGTH as size_t;
	}
    }

  /* The dialog is now built.  Run it.  */
  rc = ((*(*env)).CallNonvirtualBooleanMethod.unwrap ()) (env, dialog,
                                                          dialog_class.class,
                                                          dialog_class.display) != 0;
  android_exception_check ();

  if !rc
    { quit (); }

  /* Wait for the menu ID to arrive.  */
  android_process_events_for_menu (&raw mut id);

  if id == 0
    { quit (); }

  /* Find the selected item, and its pane, to return the proper value.  */
  i = 0;
  while i < menu_items_used as size_t
    {
      if EQ (AREF (menu_items, i as isize), Qt ())
	{ i += MENU_ITEMS_PANE_LENGTH as size_t; }
      else if EQ (AREF (menu_items, i as isize), Qquote ())
	/* This is the boundary between left-side elts and right-side elts.  */
	{ i += 1; }
      else
	{
	  entry = AREF (menu_items, i as isize + MENU_ITEMS_ITEM_VALUE as isize);

	  if id as size_t == i
	    { return entry; }

	  i += MENU_ITEMS_ITEM_LENGTH as size_t;
	}
    }

  return Qnil ();
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe fn android_popup_dialog (f: *mut frame, header: Lisp_Object, contents: Lisp_Object)
  -> Lisp_Object                                                // C 792-827
{
  let mut title: Lisp_Object;
  /* C 811 reassigns the parameter (the term.rs `let mut x = x;` shape).  */
  let mut contents = contents;
  /* C 797's `const char *error_name;` has its address taken at C 817 (PORTING.md 1.5).  */
  let mut error_name: *const c_char = std::ptr::null ();
  let mut selection: Lisp_Object;
  let mut specpdl_count: specpdl_ref = SPECPDL_INDEX ();

  check_window_system (f);

  /* Decode the dialog items from what was specified.  */
  title = Fcar (contents);
  CHECK_STRING (title);
  record_unwind_protect_void (Some (unuse_menu_items));

  if NILP (Fcar (Fcdr (contents)))
    /* No buttons specified, add an "Ok" button so users can pop down the dialog.  */
    { contents = list2 (title, Fcons (build_string (c"Ok".as_ptr ()), Qt ())); }

  list_of_panes (list1 (contents));

  /* Display them in a dialog box.  */
  block_input ();
  selection = android_dialog_show (f, title, header, &raw mut error_name);
  unblock_input ();

  unbind_to (specpdl_count, Qnil ());
  discard_menu_items ();

  if !error_name.is_null ()
    { error (c"%s".as_ptr (), error_name); }

  return selection;
}

/* ------------------------------------------------------------------------------------------
   C 829-837: `#else` - the `ANDROID_STUBIFY` arm.  It is reached only in an Android
   configuration, and only when `not_android_stubify` is off; the frozen configuration compiles
   it in no configuration at all (the file gate).  */

#[cfg (all (feature = "have_android", not (feature = "not_android_stubify")))]
#[unsafe(no_mangle)]
unsafe fn popup_activated () -> c_int                           // C 831-835
{
  return 0;
}
#[cfg (feature = "have_android")]
DEFUN! ("menu-or-popup-active-p", Fmenu_or_popup_active_p,
       Smenu_or_popup_active_p, 0, 0, 0,
       doc: "SKIP: real doc in xfns.c.  ",                     // C 839-845
       (),
       {
  return if popup_activated () != 0 { Qt () } else { Qnil () };
});
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe fn init_androidmenu ()                              // C 847-854
{
  /* C 850-853: the nested `#ifndef ANDROID_STUBIFY`.  */
  #[cfg (feature = "not_android_stubify")]
  {
    android_init_emacs_context_menu ();
    android_init_emacs_dialog ();
  }
}
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe fn syms_of_androidmenu ()                           // C 856-860
{
  defsubr (&raw mut Smenu_or_popup_active_p);
}
