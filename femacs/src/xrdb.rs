// femacs: port of src/xrdb.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// `src/xrdb.c` is not a member of the frozen build (docs/baseline/frozen-build-sources.txt): the
// frozen configuration has no window system at all, so nothing here compiles on this target and
// every item below carries `#[cfg (feature = "have_x_windows")]` - the arm whose target supplies
// `<X11/Xlib.h>` and `<X11/Xresource.h>` (P-51).  `tools/check-cond-eval.py src/xrdb.c` ->
// `6 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the file's `#if`s are decidable, and
// the arms the frozen reading leaves out are carried behind the lowercase feature its macro
// names; the two conditions the frozen *table* cannot decide the way C's own target reads them
// are `docs/ports/xrdb.md`'s Decisions 1-3.
//
// Where the names come from: the Xrm/Xlib surface from `include/xlib.rs` (reached through
// `include/xterm.rs`), the Emacs-side helpers (`xmalloc`/`xpalloc`/`xfree`/`xstrdup`/`strlen`,
// the `FIXNUMP`/`XFIXNUM`/`XFIXNAT`/`STRINGP`/`SBYTES` accessors, `lispstpcpy`, the `V*` globals)
// from `include/lisp.rs` and the `globals.rs` it includes.  `get_homedir`/`splice_dir_file`
// are C's too - lisp.h 4968/4971 declares them and `src/fileio.rs` defines them - but the
// mirror that should declare them (`include/lisp.rs`) does not carry the two prototypes yet,
// so they are five of the X11 arm probe's unresolved names (docs/ports/xrdb.md Decision 10).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("../lib/include/unistd.in.rs");       // C 24: `#include <unistd.h>`
include!("../lib/include/errno.rs");           // C 25: `#include <errno.h>`
include!("../lib/include/stdlib.rs");          // C 27: `#include <stdlib.h>` (getenv)
include!("../lib/include/stdio.rs");           // C 28: `#include <stdio.h>` (sprintf)
include!("include/lisp.rs");                   // C 32: `#include "lisp.h"`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (strlen/memcpy)
include!("include/xterm.rs");                  // C 36: `#include "xterm.h"`
include!("include/xlib.rs");                   // C 37-41: `<X11/Xlib.h>`, `<X11/Xatom.h>`,
//                                            //   `<X11/X.h>`, `<X11/Xutil.h>` and
//                                            //   `<X11/Xresource.h>` (one mirror for the five)
include!("include/systime.rs");               // C 36 brings systime.h (`Time`)
include!("include/character.rs");             // C 36 brings dispextern.h `MAX_CHAR`/
//                                            //   `CHARACTERP`/`CHAR_VALID_P`
include!("include/coding.rs");                // C 36 brings termhooks.h `coding_system`
include!("include/composite.rs");             // C 36 brings termhooks.h `composition_method`
//                                            //   and `MAX_COMPOSITION_COMPONENTS`
include!("../lib/include/pwd.rs");             // C 42: `#include <pwd.h>` (HAVE_PWD_H is 1)

/* C 26: `#include <epaths.h>` - a generated header, so the port keeps the value of the one name
   this file uses and cites the generated file: `build/emacs-ref/src/epaths.h` 82 is
   `#define PATH_X_DEFAULTS ""` (the frozen build has no X, so configure spliced the empty
   string).  The `callproc.rs` shape for the same header.  */
#[cfg (feature = "have_x_windows")]
const PATH_X_DEFAULTS: &std::ffi::CStr = c"";

/* C 48-51: "The string which gets substituted for the %C escape in XFILESEARCHPATH and friends,
   or zero if none was specified."  */
#[cfg (feature = "have_x_windows")]
pub static mut x_customization_string: *mut c_char = std::ptr::null_mut ();

/* C 55-67: "Return the value of the emacs.customization (Emacs.Customization) resource, for later
   use in search path decoding.  If we find no such resource, return zero."  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_get_customization_string (mut db: XrmDatabase, name: *const c_char,
                                     class: *const c_char) -> *mut c_char
{
  /* C 60-61: `alloca (strlen (name) + sizeof "customization" + 3)` - the literal's `sizeof` is
     14 (thirteen bytes and C's terminator), and C's "Customization" on the next line is the same
     length.  C's bare `alloca` is the port's stack pool (PORTING.md P-05's decision), whose
     blocks are released by `safe_free` rather than by the frame - the deviation the pool's own
     note records.  */
  let full_name: *mut c_char = sa_stack_alloc (strlen (name) + 14 + 3) as *mut c_char;
  let full_class: *mut c_char = sa_stack_alloc (strlen (class) + 14 + 3) as *mut c_char;
  let result: *const c_char;

  sprintf (full_name, c"%s.%s".as_ptr (), name, c"customization".as_ptr ());
  sprintf (full_class, c"%s.%s".as_ptr (), class, c"Customization".as_ptr ());

  /* C 65 passes `&db` (the address of the parameter) to `void *`.  */
  result = x_get_string_resource (&raw mut db as *mut c_void, full_name, full_class);
  if !result.is_null () { return xstrdup (result); }
  return std::ptr::null_mut ();
}

/* C 95-186: the escape expansion for X file search paths.  C 69-93's table of the recognised
   escapes (`%N`, `%T`, `%S`, `%L`, `%l`, `%t`, `%c`, `%C`) is the reference's own comment and is
   not repeated here.  */
#[cfg (feature = "have_x_windows")]
unsafe fn magic_db (mut string: *const c_char, mut string_len: ptrdiff_t,
                    class: *const c_char, mut escaped_suffix: *const c_char) -> XrmDatabase
{
  let mut db: XrmDatabase;
  let lang: *mut c_char = getenv (c"LANG".as_ptr ());

  let mut path_size: ptrdiff_t = 100;
  let mut path: *mut c_char = xmalloc (path_size as usize) as *mut c_char;
  let mut path_len: ptrdiff_t = 0;

  let mut p: *const c_char = string;

  while p < string.offset (string_len)
    {
      /* The chunk we're about to stick on the end of result.  */
      let mut next: *const c_char = p;
      let mut next_len: ptrdiff_t = 1;

      if *p as u8 == b'%'
        {
          p = p.offset (1);

          if p >= string.offset (string_len)
            { next_len = 0; }
          else
            {
              match *p as u8
                {
                  b'%' =>
                    {
                      next = c"%".as_ptr ();
                      next_len = 1;
                    }

                  b'C' =>
                    {
                      if !x_customization_string.is_null ()
                        {
                          next = x_customization_string;
                          next_len = strlen (next) as ptrdiff_t;
                        }
                      else
                        { next_len = 0; }
                    }

                  b'N' =>
                    {
                      next = class;
                      next_len = strlen (class) as ptrdiff_t;
                    }

                  b'T' =>
                    {
                      next = c"app-defaults".as_ptr ();
                      next_len = strlen (next) as ptrdiff_t;
                    }

                  /* C's `default:` and `case 'S':` share one body.  */
                  b'S' =>
                    { next_len = 0; }

                  b'L' | b'l' =>
                    {
                      if lang.is_null ()
                        {
                          xfree (path as *mut c_void);
                          return std::ptr::null_mut ();
                        }

                      next = lang;
                      next_len = strlen (next) as ptrdiff_t;
                    }

                  b't' | b'c' =>
                    {
                      xfree (path as *mut c_void);
                      return std::ptr::null_mut ();
                    }

                  _ =>
                    { next_len = 0; }
                }
            }
        }

      /* Do we have room for this component followed by a '\0'?  */
      if path_size - path_len <= next_len
        {
          /* C 176-177: `xpalloc (path, &path_size, path_len - path_size + next_len + 1, -1,
             sizeof *path)` - `sizeof *path` is one.  */
          path = xpalloc (path as *mut c_void, &raw mut path_size,
                          path_len - path_size + next_len + 1, -1, 1) as *mut c_char;
        }

      memcpy (path.offset (path_len) as *mut c_void, next as *const c_void,
              next_len as usize);
      path_len += next_len;

      p = p.offset (1);

      /* If we've reached the end of the string, append ESCAPED_SUFFIX.  */
      if p >= string.offset (string_len) && !escaped_suffix.is_null ()
        {
          string = escaped_suffix;
          string_len = strlen (string) as ptrdiff_t;
          p = string;
          escaped_suffix = std::ptr::null ();
        }
    }

  /* C 184: `path[path_len] = '\0';` - a raw pointer has no indexing in Rust.  */
  *path.offset (path_len) = 0;
  db = XrmGetFileDatabase (path);
  xfree (path as *mut c_void);
  return db;
}

/* C 192-218: "Find the first element of SEARCH_PATH which exists and is readable, after
   expanding the %-escapes.  Return 0 if we didn't find any, and the path name of the one we
   found otherwise."  C's two `for` loops are the port's `while` (PORTING.md 2.1).  */
#[cfg (feature = "have_x_windows")]
unsafe fn search_magic_path (search_path: *const c_char, class: *const c_char,
                             escaped_suffix: *const c_char) -> XrmDatabase
{
  let mut s: *const c_char;
  let mut p: *const c_char;

  s = search_path;
  while *s != 0
    {
      p = s;
      while *p != 0 && *p as u8 != b':'
        { p = p.offset (1); }

      if p > s
        {
          let db: XrmDatabase = magic_db (s, p.offset_from (s), class, escaped_suffix);
          if !db.is_null () { return db; }
        }
      else if *p as u8 == b':'
        {
          /* C 211: `static char const ns[] = "%N%S";` - the port's `const` has the lifetime C's
             `static` array has.  */
          const ns: [c_char; 5] =
            [b'%' as c_char, b'N' as c_char, b'%' as c_char, b'S' as c_char, 0];
          let db: XrmDatabase = magic_db (ns.as_ptr (), strlen (ns.as_ptr ()) as ptrdiff_t,
                                          class, escaped_suffix);
          if !db.is_null () { return db; }
        }

      if *p as u8 == b':'
        { p = p.offset (1); }
      s = p;
    }

  return std::ptr::null_mut ();
}

/* C 224-232: "Producing databases for individual sources."  */
#[cfg (feature = "have_x_windows")]
unsafe fn get_system_app (class: *const c_char) -> XrmDatabase
{
  let mut path: *const c_char;

  path = getenv (c"XFILESEARCHPATH".as_ptr ());
  if path.is_null () { path = PATH_X_DEFAULTS.as_ptr (); }

  return search_magic_path (path, class, 0 as *const c_char);
}

/* C 237-241.  C's body is the literal `return NULL;`, so the port's is too; `display` is unused
   in C as well.  */
#[cfg (feature = "have_x_windows")]
unsafe fn get_fallback (display: *mut c_void) -> XrmDatabase
{
  return std::ptr::null_mut ();
}

#[cfg (feature = "have_x_windows")]
unsafe fn get_user_app (class: *const c_char) -> XrmDatabase
{
  let mut db: XrmDatabase = 0 as XrmDatabase;
  let mut path: *const c_char;

  /* Check for XUSERFILESEARCHPATH.  It is a path of complete file names, not directories.  */
  path = getenv (c"XUSERFILESEARCHPATH".as_ptr ());
  if !path.is_null ()
    { db = search_magic_path (path, class, 0 as *const c_char); }

  if db.is_null ()
    {
      /* Check for APPLRESDIR; it is a path of directories.  In each, we have to search for
         LANG/CLASS and then CLASS.  */
      path = getenv (c"XAPPLRESDIR".as_ptr ());
      if !path.is_null ()
        {
          db = search_magic_path (path, class, c"/%L/%N".as_ptr ());
          if db.is_null ()
            { db = search_magic_path (path, class, c"/%N".as_ptr ()); }
        }
    }

  if db.is_null ()
    {
      /* Check in the home directory.  This is a bit of a hack; let's hope one's home directory
         doesn't contain ':' or '%'.  */
      let home: *const c_char = get_homedir ();
      db = search_magic_path (home, class, c"/%L/%N".as_ptr ());
      if db.is_null ()
        { db = search_magic_path (home, class, c"/%N".as_ptr ()); }
    }

  return db;
}

/* C 300: `static char const xdefaults[] = ".Xdefaults";` - eleven bytes (the ten characters and
   C's terminator), which is the `sizeof` C 322 adds to the home directory's length.  */
#[cfg (feature = "have_x_windows")]
static xdefaults: [c_char; 11] =
  [b'.' as c_char, b'X' as c_char, b'd' as c_char, b'e' as c_char, b'f' as c_char,
   b'a' as c_char, b'u' as c_char, b'l' as c_char, b't' as c_char, b's' as c_char, 0];

#[cfg (feature = "have_x_windows")]
unsafe fn get_user_db (display: *mut c_void) -> XrmDatabase
{
  let mut db: XrmDatabase;
  let mut xdefs: *mut c_char;

  /* C 308-312: `#ifdef PBaseSize` - "Cheap way to test for X11R4 or later."  `PBaseSize` is
     `<X11/Xutil.h>`'s (measured: this machine's Xutil.h 111 is `#define PBaseSize (1L << 8)`), so
     every X11R4+ build takes the `#ifdef` arm.  The frozen macro table has no `PBaseSize` row, and
     the arm its reading takes (C 310-311, `xdefs = display->xdefaults;`) reads Xlib's *private*
     display record (`_XPrivDisplay`, Xlib.h 489-546), which no mirror here carries and which this
     configuration never compiles - so the port takes the arm the file's own platform takes and
     records the `#else` text instead of transcribing it (P-08; docs/ports/xrdb.md Decision 2).  */
  // C 310-311: `#else` -> `xdefs = display->xdefaults;`
  xdefs = XResourceManagerString (display);

  if !xdefs.is_null ()
    { db = XrmGetStringDatabase (xdefs); }
  else
    {
      /* Use ~/.Xdefaults.  */
      let home: *const c_char = get_homedir ();
      let filename: *mut c_char = xmalloc (strlen (home) + 1 + 11) as *mut c_char;
      splice_dir_file (filename, home, xdefaults.as_ptr ());
      db = XrmGetFileDatabase (filename);
      xfree (filename as *mut c_void);
    }

  /* C 326-334: `#ifdef HAVE_XSCREENRESOURCESTRING` - "Get the screen-specific resources too."
     The frozen table has the macro `undef` (it is a configure result of a build that has Xlib),
     so this is the arm the frozen reading leaves out and it carries its feature.  `XFree` is
     Xlib's and no mirror in this tree declares it yet (it is one of the arm's two unresolved
     names, with `XScreenResourceString`/`DefaultScreenOfDisplay` coming from `include/xlib.rs`),
     so the arm is the first thing a target that turns this feature on will need.  */
  #[cfg (feature = "have_xscreenresourcestring")]
    {
      xdefs = XScreenResourceString (DefaultScreenOfDisplay (display));
      if !xdefs.is_null ()
        {
          XrmMergeDatabases (XrmGetStringDatabase (xdefs), &raw mut db);
          XFree (xdefs);
        }
    }

  return db;
}

#[cfg (feature = "have_x_windows")]
unsafe fn get_environ_db () -> XrmDatabase
{
  let db: XrmDatabase;
  let mut p: *mut c_char = getenv (c"XENVIRONMENT".as_ptr ());
  let mut filename: *mut c_char = 0 as *mut c_char;

  if p.is_null ()
    {
      let system_name: Lisp_Object = Fsystem_name ();
      if STRINGP (system_name)
        {
          /* Use ~/.Xdefaults-HOSTNAME.  */
          let home: *const c_char = get_homedir ();
          /* C 352-354: `p = filename = xmalloc (...)` - C's chained assignment, which Rust
             spells as two statements (it has no assignment expression, E0605).  */
          filename = xmalloc (strlen (home) + 1 + 11 + 1 + SBYTES (system_name) as usize)
                     as *mut c_char;
          p = filename;
          let mut e: *mut c_char = splice_dir_file (p, home, xdefaults.as_ptr ());
          *e = b'-' as c_char;
          e = e.offset (1);
          lispstpcpy (e, system_name);
        }
    }

  db = XrmGetFileDatabase (p);

  xfree (filename as *mut c_void);

  return db;
}

/* C 370-374: "Types of values that we can find in a database".  C 372's `#define XrmStringType
   "String"` is carried by `include/xlib.rs` (the mirror owns it, so that `--check` compares C
   372's constant against the port's one spelling); C 374's object is here.  */
#[cfg (feature = "have_x_windows")]
pub static mut x_rm_string: XrmRepresentation = 0;

/* C 378-465: "Load X resources based on the display and a possible -xrm option."  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_load_resources (display: *mut c_void, xrm_string: *const c_char,
                                           myname: *const c_char, myclass: *const c_char)
                                           -> XrmDatabase
{
  let user_database: XrmDatabase;
  let mut rdb: XrmDatabase;
  let mut db: XrmDatabase;
  let mut line: [c_char; 256] = [0; 256];

  x_rm_string = XrmStringToQuark (XrmStringType.as_ptr ());
  /* C 387-391: `#ifndef USE_X_TOOLKIT` - "pmr@osf.org says this shouldn't be done if
     USE_X_TOOLKIT.  I suspect it's because the toolkit version does this elsewhere."  */
  #[cfg (not (feature = "use_x_toolkit"))]
  XrmInitialize ();
  rdb = XrmGetStringDatabase (c"".as_ptr ());

  /* C 394-406: `#ifdef USE_MOTIF` - "Set double click time of list boxes in the file selection
     dialog from `double-click-time'."  C's `%"pI"d` conversion is `%ld` (lisp.h's `pI` is "l" on
     this target).  */
  #[cfg (feature = "use_motif")]
    {
      if FIXNUMP (Vdouble_click_time) && XFIXNUM (Vdouble_click_time) > 0
        {
          sprintf (line.as_mut_ptr (), c"%s*fsb*DirList.doubleClickInterval: %ld".as_ptr (),
                   myclass, XFIXNAT (Vdouble_click_time));
          XrmPutLineResource (&raw mut rdb, line.as_ptr ());
          sprintf (line.as_mut_ptr (), c"%s*fsb*ItemsList.doubleClickInterval: %ld".as_ptr (),
                   myclass, XFIXNAT (Vdouble_click_time));
          XrmPutLineResource (&raw mut rdb, line.as_ptr ());
        }
    }
  /* C 406-425: `#else /* not USE_MOTIF */` - "Add some font defaults.  If the font `helv'
     doesn't exist, widgets will use some other default font."  */
  #[cfg (not (feature = "use_motif"))]
    {
      sprintf (line.as_mut_ptr (), c"Emacs.dialog*.background: grey75".as_ptr ());
      XrmPutLineResource (&raw mut rdb, line.as_ptr ());
      /* C 411-418: `#if !(defined USE_CAIRO || defined HAVE_XFT) || !defined (USE_LUCID)` -
         C's condition, not its De Morgan simplification (the two differ when only `USE_CAIRO`
         is defined).  */
      #[cfg (any (not (any (feature = "use_cairo", feature = "have_xft")),
                 not (feature = "use_lucid")))]
        {
          sprintf (line.as_mut_ptr (), c"Emacs.dialog*.font: %s".as_ptr (),
                   c"-*-helvetica-medium-r-*--*-120-*-*-*-*-iso8859-1".as_ptr ());
          XrmPutLineResource (&raw mut rdb, line.as_ptr ());
          sprintf (line.as_mut_ptr (), c"*XlwMenu*font: %s".as_ptr (),
                   c"-*-helvetica-medium-r-*--*-120-*-*-*-*-iso8859-1".as_ptr ());
          XrmPutLineResource (&raw mut rdb, line.as_ptr ());
        }
      sprintf (line.as_mut_ptr (), c"*XlwMenu*background: grey75".as_ptr ());
      XrmPutLineResource (&raw mut rdb, line.as_ptr ());
      sprintf (line.as_mut_ptr (), c"Emacs*verticalScrollBar.background: grey75".as_ptr ());
      XrmPutLineResource (&raw mut rdb, line.as_ptr ());
      sprintf (line.as_mut_ptr (), c"Emacs*horizontalScrollBar.background: grey75".as_ptr ());
      XrmPutLineResource (&raw mut rdb, line.as_ptr ());
    }

  user_database = get_user_db (display);

  /* Figure out what the "customization string" is, so we can use it to decode paths.  */
  xfree (x_customization_string as *mut c_void);
  x_customization_string = x_get_customization_string (user_database, myname, myclass);

  /* Get application system defaults */
  db = get_system_app (myclass);
  if db != std::ptr::null_mut () { XrmMergeDatabases (db, &raw mut rdb); }

  /* Get Fallback resources */
  db = get_fallback (display);
  if db != std::ptr::null_mut () { XrmMergeDatabases (db, &raw mut rdb); }

  /* Get application user defaults */
  db = get_user_app (myclass);
  if db != std::ptr::null_mut () { XrmMergeDatabases (db, &raw mut rdb); }

  /* get User defaults */
  if user_database != std::ptr::null_mut ()
    { XrmMergeDatabases (user_database, &raw mut rdb); }

  /* Get Environment defaults. */
  db = get_environ_db ();
  if db != std::ptr::null_mut () { XrmMergeDatabases (db, &raw mut rdb); }

  /* Last, merge in any specification from the command line. */
  if xrm_string != std::ptr::null ()
    {
      db = XrmGetStringDatabase (xrm_string);
      if db != std::ptr::null_mut () { XrmMergeDatabases (db, &raw mut rdb); }
    }

  return rdb;
}

/* C 468-487: "Retrieve the value of the resource specified by NAME with class CLASS and of type
   TYPE from database RDB.  The value is returned in RET_VALUE."  `type` is a Rust keyword, hence
   `r#type` (the port's `r#` spelling, PORTING.md 1.2).  */
#[cfg (feature = "have_x_windows")]
unsafe fn x_get_resource (rdb: XrmDatabase, name: *const c_char, class: *const c_char,
                          expected_type: XrmRepresentation, ret_value: *mut XrmValue) -> c_int
{
  let mut value: XrmValue = std::mem::zeroed ();
  let mut namelist: [XrmName; 100] = [0; 100];
  let mut classlist: [XrmClass; 100] = [0; 100];
  let mut r#type: XrmRepresentation = 0;

  XrmStringToNameList! (name, namelist.as_mut_ptr ());
  XrmStringToClassList! (class, classlist.as_mut_ptr ());

  if XrmQGetResource (rdb, namelist.as_mut_ptr (), classlist.as_mut_ptr (),
                      &raw mut r#type, &raw mut value) == True
      && (r#type == expected_type)
    {
      *ret_value = value;
      return value.size as c_int;
    }

  return 0;
}

/* C 489-505: "Retrieve the string resource specified by NAME with CLASS from database RDB."  */
#[cfg (feature = "have_x_windows")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_get_string_resource (v_rdb: *mut c_void, name: *const c_char,
                                                class: *const c_char) -> *const c_char
{
  let rdb: *mut XrmDatabase = v_rdb as *mut XrmDatabase;
  let mut value: XrmValue = std::mem::zeroed ();

  if inhibit_x_resources
    {
      /* --quick was passed, so this is a no-op.  */
      return std::ptr::null ();
    }

  if x_get_resource (*rdb, name, class, x_rm_string, &raw mut value) != 0
    { return value.addr as *const c_char; }

  return std::ptr::null ();
}
