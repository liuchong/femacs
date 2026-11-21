// femacs: port of src/kqueue.c @ fd7afced403.
//
// P418: the current macOS recipes enable this backend. The frozen-config
// discussion below is historical; events now use the shared target enum and
// the SDK kevent declaration retains its four-byte packing.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The *build-level* file gate is `feature = "have_kqueue"` (P-86 ⑧), on every
// item *and on the include graph*: the frozen configuration leaves `HAVE_KQUEUE` undefined
// (`docs/baseline/config/darwin-arm64.macros.tsv` 1404), so the reference builds no object from
// this file either - while the text is carried **in full**, because a kqueue configuration
// compiles it.  The earlier "empty unit plus a deferred-declaration note" reading (status 3.31) is
// withdrawn as a *conclusion*; its measurements still hold.
//
// **The kqueue API is on this machine** (macOS `<sys/event.h>`/`<sys/resource.h>`), so the
// in-unit surface below (P-88 ①: no mirror exists for it and this is its only consumer) is
// quoted from those headers and every value and offset is a clang measurement
// (`## Evidence` in `docs/ports/kqueue.md`); the arm flip is therefore a *real compile*, not a
// name-resolution reading.  Two frozen macros are 1 in this configuration -
// `HAVE_SYS_RESOURCE_H` and `HAVE_GETRLIMIT` (macros.tsv 1627/1371) - so their live arms are the
// port's default and the `#else` side is carried under a `no_*` feature (P-51); the three
// *header* macros C tests by value (`O_EVTONLY`, `O_SYMLINK`, `EV_ENABLE`) are measured present
// here and follow the same shape with `no_o_evtonly`/`no_o_symlink`/`no_ev_enable`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

/* Local SDK declarations cover sys/event.h, sys/resource.h and the two
   Darwin-only open flags absent from the shared mirror. Common open flags use
   the existing fcntl mirror below on both terminal and Cocoa include graphs. */
/// C: fcntl.h: `#define O_EVTONLY 0x8000` - measured 32768.
#[cfg (feature = "have_kqueue")]
pub const O_EVTONLY: c_int = 32768;
/// C: fcntl.h: `#define O_SYMLINK 0x200000` - measured 2097152.
#[cfg (feature = "have_kqueue")]
pub const O_SYMLINK: c_int = 2097152;
/// C: sys/event.h: `#define NOTE_DELETE 0x00000001` … `NOTE_REVOKE 0x00000040` - measured
/// 1/2/4/8/16/32/64.
#[cfg (feature = "have_kqueue")]
pub const NOTE_DELETE: c_ushort = 1;
#[cfg (feature = "have_kqueue")]
pub const NOTE_WRITE: c_ushort = 2;
#[cfg (feature = "have_kqueue")]
pub const NOTE_EXTEND: c_ushort = 4;
#[cfg (feature = "have_kqueue")]
pub const NOTE_ATTRIB: c_ushort = 8;
#[cfg (feature = "have_kqueue")]
pub const NOTE_LINK: c_ushort = 16;
#[cfg (feature = "have_kqueue")]
pub const NOTE_RENAME: c_ushort = 32;
#[cfg (feature = "have_kqueue")]
pub const NOTE_REVOKE: c_ushort = 64;
/// C: sys/event.h: `#define EVFILT_VNODE (-4)` - measured -4; `EV_ADD` 1, `EV_ENABLE` 4,
/// `EV_CLEAR` 32.
#[cfg (feature = "have_kqueue")]
pub const EVFILT_VNODE: c_short = -4;
#[cfg (feature = "have_kqueue")]
pub const EV_ADD: c_ushort = 1;
#[cfg (feature = "have_kqueue")]
pub const EV_ENABLE: c_ushort = 4;
#[cfg (feature = "have_kqueue")]
pub const EV_CLEAR: c_ushort = 32;
/// C: sys/event.h: `struct kevent { uintptr_t ident; int16_t filter; uint16_t flags; uint32_t
/// fflags; intptr_t data; void *udata; };` - measured `sizeof` 32 with ident@0, filter@8,
/// flags@10, fflags@12, data@16, udata@24.
#[cfg (feature = "have_kqueue")]
#[repr (C, packed(4))]
#[derive (Clone, Copy)]
pub struct kevent
{
  pub ident: usize,
  pub filter: c_short,
  pub flags: c_ushort,
  pub fflags: c_uint,
  pub data: isize,
  pub udata: *mut c_void,
}
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::size_of::<kevent> () == 32);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::align_of::<kevent> () == 4);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::offset_of! (kevent, ident) == 0);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::offset_of! (kevent, filter) == 8);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::offset_of! (kevent, flags) == 10);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::offset_of! (kevent, fflags) == 12);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::offset_of! (kevent, data) == 16);
#[cfg (feature = "have_kqueue")]
const _: () = core::assert! (core::mem::offset_of! (kevent, udata) == 24);
/// C: sys/resource.h: `struct rlimit { rlim_t rlim_cur; rlim_t rlim_max; };` - measured
/// `sizeof` 16 with `rlim_cur`@0 (`rlim_t` is `uint64_t` here).
#[cfg (feature = "have_kqueue")]
#[repr (C)]
pub struct rlimit
{
  pub rlim_cur: u64,
  pub rlim_max: u64,
}
/// C: sys/resource.h: `#define RLIMIT_NOFILE 8` - measured 8.
#[cfg (feature = "have_kqueue")]
pub const RLIMIT_NOFILE: c_int = 8;
#[cfg (feature = "have_kqueue")]
unsafe extern "C" {
  /// C: sys/event.h: `int kqueue (void);`
  pub fn kqueue () -> c_int;
  /// C: sys/event.h: `int kevent (int kq, const struct kevent *changelist, int nchanges, struct
  /// kevent *eventlist, int nevents, const struct timespec *timeout);`
  pub fn kevent (kq: c_int, changelist: *const kevent, nchanges: c_int,
                 eventlist: *mut kevent, nevents: c_int, timeout: *const timespec) -> c_int;
  /// C: sys/resource.h: `int getrlimit (int resource, struct rlimit *rlim);`
  pub fn getrlimit (resource: c_int, rlim: *mut rlimit) -> c_int;
}

#[cfg (feature = "have_kqueue")]
const _: () = ();
#[cfg (feature = "have_kqueue")]
const _: () = ();
#[cfg (feature = "have_kqueue")]
const _: () = ();

// Materialize the required buffer/type closure before optional GUI edges can
// reserve it during first-include deduplication in a terminal build.
#[cfg (feature = "have_kqueue")]
const _: () = ();
#[cfg (feature = "have_kqueue")]
const _: () = ();
#[cfg (feature = "have_kqueue")]
const _: () = ();
                                                // `delete_read_fd`, `FNread_fd`)
#[cfg (feature = "have_kqueue")]
const _: () = ();
/* The two edges C 24/27's chain reaches through other headers, spelled here (P-86 ⑧, the
   `src/gfilenotify.rs` precedent): `termhooks.h` names `Emacs_Color` (dispextern.h), and
   `include/keyboard.rs` 32 reaches `buffer.rs`, whose text names character.h's
   `STRING_CHAR`/`UNIBYTE_TO_CHAR`/`BYTES_BY_CHAR_HEAD`/`string_char_and_length`/
   `raw_prev_char_len`.  */
#[cfg (feature = "have_kqueue")]
const _: () = ();
#[cfg (feature = "have_kqueue")]
const _: () = ();
#[cfg (feature = "have_kqueue")]
const _: () = ();

// FILE_NOTIFY_EVENT comes from the shared termhooks include. Its value follows
// the enabled thread, window-system and notification members on this target.
/// termhooks.h's `enum event_kind kind : 16` shares `include/termhooks.rs`'s `input_event::bits`;
/// C 95 writes `event.kind = …`, so the port keeps C's field name in a per-unit macro (the
/// `src/gtkutil.rs` 1757-1770 shape, body-identical).
#[cfg (feature = "have_kqueue")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}
/// C: lisp.h's `INLINE bool FUNCTIONP (Lisp_Object a) { return !NILP (Ffunctionp (a)); }` -
/// spelled here as lisp.h does (the `src/gfilenotify.rs` reading).
#[cfg (feature = "have_kqueue")]
pub unsafe fn FUNCTIONP (a: Lisp_Object) -> bool
{
  !NILP (Ffunctionp (a))
}

/* C 35-39: the two file-scope objects.  `kqueuefd`'s `-1` and the tentative `watch_list` (i.e.
   zero, which is `Qnil`) are C's own initialisers.  */
#[cfg (feature = "have_kqueue")]
static mut kqueuefd: c_int = -1;
#[cfg (feature = "have_kqueue")]
static mut watch_list: Lisp_Object = lisp_h_Qnil ();

/* C 41-68: `static Lisp_Object kqueue_directory_listing (Lisp_Object directory_files)`.  */
#[cfg (feature = "have_kqueue")]
unsafe fn kqueue_directory_listing (directory_files: Lisp_Object) -> Lisp_Object
{
  let mut result: Lisp_Object = Qnil ();

  let mut dl = directory_files;
  while !NILP (dl)
    {
      /* We ignore "." and "..".  */
      if strcmp (c".".as_ptr (), SSDATA (XCAR (XCAR (dl)))) == 0
         || strcmp (c"..".as_ptr (), SSDATA (XCAR (XCAR (dl)))) == 0
        {
          dl = XCDR (dl);
          continue;
        }

      result = Fcons (list5 (/* inode.  */
                             Fnth (make_fixnum (11), XCAR (dl)),
                             /* filename.  */
                             XCAR (XCAR (dl)),
                             /* last modification time.  */
                             Fnth (make_fixnum (6), XCAR (dl)),
                             /* last status change time.  */
                             Fnth (make_fixnum (7), XCAR (dl)),
                             /* size.  */
                             Fnth (make_fixnum (8), XCAR (dl))),
                      result);
      dl = XCDR (dl);
    }
  result
}

/* C 70-105: `static void kqueue_generate_event (Lisp_Object watch_object, Lisp_Object actions,
   Lisp_Object file, Lisp_Object file1)`.  */
#[cfg (feature = "have_kqueue")]
unsafe fn kqueue_generate_event (watch_object: Lisp_Object, actions: Lisp_Object,
                                 file: Lisp_Object, file1: Lisp_Object)
{
  let mut flags: Lisp_Object;
  let mut action: Lisp_Object;
  let mut entry: Lisp_Object;
  let mut event: input_event = std::mem::zeroed ();
  let mut actions = actions;

  /* Check, whether all actions shall be monitored.  */
  flags = Fnth (make_fixnum (2), watch_object);
  action = actions;
  loop
    {
      if NILP (action)
        { break; }
      entry = XCAR (action);
      if NILP (Fmember (entry, flags))
        {
          action = XCDR (action);
          actions = Fdelq (entry, actions);
        }
      else
        { action = XCDR (action); }
    }

  /* Store it into the input event queue.  */
  if !NILP (actions)
    {
      EVENT_INIT (&raw mut event);
      input_event_bits! (kind, &raw mut event, FILE_NOTIFY_EVENT);
      event.frame_or_window = Qnil ();
      event.arg = list2 (Fcons (XCAR (watch_object),
                                Fcons (actions,
                                       if NILP (file1)
                                         { list1 (file) }
                                       else
                                         { list2 (file, file1) })),
                         Fnth (make_fixnum (3), watch_object));
      kbd_buffer_store_event (&raw mut event as *mut c_void);
    }
}

/* C 107-274: `static void kqueue_compare_dir_list (Lisp_Object watch_object)`.  C's `goto the_end`
   (C 147/176/187/201) jumps to the loop's tail at C 216-218, and the fall-through reaches the same
   tail - so the conditional half is one labelled block and the tail follows it (the
   `src/gfilenotify.rs` `'cleanup:` shape, PORTING.md 2.2).  */
#[cfg (feature = "have_kqueue")]
unsafe fn kqueue_compare_dir_list (watch_object: Lisp_Object)
{
  let mut dir: Lisp_Object;
  let mut pending_dl: Lisp_Object;
  let mut deleted_dl: Lisp_Object;
  let mut old_directory_files: Lisp_Object;
  let mut old_dl: Lisp_Object;
  let mut new_directory_files: Lisp_Object;
  let mut new_dl: Lisp_Object;
  let mut dl: Lisp_Object;

  dir = XCAR (XCDR (watch_object));
  pending_dl = Qnil ();
  deleted_dl = Qnil ();

  old_directory_files = Fnth (make_fixnum (4), watch_object);
  old_dl = kqueue_directory_listing (old_directory_files);

  /* When the directory is not accessible anymore, it has been deleted.  */
  if NILP (Ffile_directory_p (dir))
    {
      kqueue_generate_event (watch_object, Fcons (Qdelete (), Qnil ()), dir, Qnil ());
      return;
    }
  new_directory_files =
    directory_files_internal (dir, Qnil (), Qnil (), Qnil (), true, Qnil (), Qnil ());
  new_dl = kqueue_directory_listing (new_directory_files);

  /* Parse through the old list.  */
  dl = old_dl;
  loop
    {
      let mut old_entry: Lisp_Object;
      let mut new_entry: Lisp_Object;
      let mut dl1: Lisp_Object;
      if NILP (dl)
        { break; }

      /* Search for an entry with the same inode.  */
      old_entry = XCAR (dl);
      new_entry = assq_no_quit (XCAR (old_entry), new_dl);

      'the_end: {
        if !NILP (Fequal (old_entry, new_entry))
          {
            /* Both entries are identical.  Nothing to do.  */
            new_dl = Fdelq (new_entry, new_dl);
            break 'the_end;
          }

        /* Both entries have the same inode.  */
        if !NILP (new_entry)
          {
            /* Both entries have the same file name.  */
            if strcmp (SSDATA (XCAR (XCDR (old_entry))),
                       SSDATA (XCAR (XCDR (new_entry)))) == 0
              {
                /* Modification time has been changed, the file has been written.  */
                if NILP (Fequal (Fnth (make_fixnum (2), old_entry),
                                 Fnth (make_fixnum (2), new_entry)))
                  { kqueue_generate_event
                       (watch_object, Fcons (Qwrite (), Qnil ()),
                        XCAR (XCDR (old_entry)), Qnil ()); }
                /* Status change time has been changed, the file attributes
                   have changed.  */
                if NILP (Fequal (Fnth (make_fixnum (3), old_entry),
                                 Fnth (make_fixnum (3), new_entry)))
                  { kqueue_generate_event
                       (watch_object, Fcons (Qattrib (), Qnil ()),
                        XCAR (XCDR (old_entry)), Qnil ()); }
              }
            else
              {
                /* The file has been renamed.  */
                kqueue_generate_event
                  (watch_object, Fcons (Qrename (), Qnil ()),
                   XCAR (XCDR (old_entry)), XCAR (XCDR (new_entry)));
                deleted_dl = Fcons (new_entry, deleted_dl);
              }
            new_dl = Fdelq (new_entry, new_dl);
            break 'the_end;
          }

        /* Search, whether there is a file with the same name but another inode.  */
        dl1 = new_dl;
        while !NILP (dl1)
          {
            new_entry = XCAR (dl1);
            if strcmp (SSDATA (XCAR (XCDR (old_entry))),
                       SSDATA (XCAR (XCDR (new_entry)))) == 0
              {
                pending_dl = Fcons (new_entry, pending_dl);
                new_dl = Fdelq (new_entry, new_dl);
                break;
              }
            dl1 = XCDR (dl1);
          }
        if !NILP (dl1)
          { break 'the_end; }

        /* Check, whether this a pending file.  */
        new_entry = assq_no_quit (XCAR (old_entry), pending_dl);

        if NILP (new_entry)
          {
            /* Check, whether this is an already deleted file (by rename).  */
            dl1 = deleted_dl;
            while !NILP (dl1)
              {
                new_entry = XCAR (dl1);
                if strcmp (SSDATA (XCAR (XCDR (old_entry))),
                           SSDATA (XCAR (XCDR (new_entry)))) == 0
                  {
                    deleted_dl = Fdelq (new_entry, deleted_dl);
                    break;
                  }
                dl1 = XCDR (dl1);
              }
            if !NILP (dl1)
              { break 'the_end; }
            /* The file has been deleted.  */
            kqueue_generate_event
              (watch_object, Fcons (Qdelete (), Qnil ()), XCAR (XCDR (old_entry)), Qnil ());
          }
        else
          {
            /* The file has been renamed.  */
            kqueue_generate_event
              (watch_object, Fcons (Qrename (), Qnil ()),
               XCAR (XCDR (old_entry)), XCAR (XCDR (new_entry)));
            pending_dl = Fdelq (new_entry, pending_dl);
          }
      }

      dl = XCDR (dl);
      old_dl = Fdelq (old_entry, old_dl);
    }

  /* Parse through the resulting new list.  */
  dl = new_dl;
  loop
    {
      let mut entry: Lisp_Object;
      if NILP (dl)
        { break; }

      /* A new file has appeared.  */
      entry = XCAR (dl);
      kqueue_generate_event
        (watch_object, Fcons (Qcreate (), Qnil ()), XCAR (XCDR (entry)), Qnil ());

      /* Check size of that file.  */
      let size: Lisp_Object = Fnth (make_fixnum (4), entry);
      if FLOATP (size) || (XFIXNUM (size) > 0)
        { kqueue_generate_event
             (watch_object, Fcons (Qwrite (), Qnil ()), XCAR (XCDR (entry)), Qnil ()); }

      dl = XCDR (dl);
      new_dl = Fdelq (entry, new_dl);
    }

  /* Parse through the resulting pending_dl list.  */
  dl = pending_dl;
  loop
    {
      let mut entry: Lisp_Object;
      if NILP (dl)
        { break; }

      /* A file is still pending.  Assume it was a write.  */
      entry = XCAR (dl);
      kqueue_generate_event
        (watch_object, Fcons (Qwrite (), Qnil ()), XCAR (XCDR (entry)), Qnil ());

      dl = XCDR (dl);
      pending_dl = Fdelq (entry, pending_dl);
    }

  /* At this point, old_dl, new_dl and pending_dl shall be empty.
     deleted_dl might not be empty when there was a rename to a
     nonexistent file.  Let's make a check for this (might be removed
     once the code is stable).  */
  if !NILP (old_dl)
    { report_file_error (c"Old list not empty".as_ptr (), old_dl); }
  if !NILP (new_dl)
    { report_file_error (c"New list not empty".as_ptr (), new_dl); }
  if !NILP (pending_dl)
    { report_file_error (c"Pending events list not empty".as_ptr (), pending_dl); }

  /* Replace old directory listing with the new one.  */
  XSETCDR (Fnthcdr (make_fixnum (3), watch_object),
           Fcons (new_directory_files, Qnil ()));
}

/* C 276-336: `static void kqueue_callback (int fd, void *data)`.  */
#[cfg (feature = "have_kqueue")]
unsafe extern "C-unwind" fn kqueue_callback (fd: c_int, data: *mut c_void)
{
  let _ = fd;
  let _ = data;
  loop
    {
      let mut kev: kevent = std::mem::zeroed ();
      /* C 283: `static const struct timespec nullts = { 0, 0 };` - a function-local static
         (PORTING.md 1.5's rule for a constant the callee reads).  */
      let nullts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
      let mut descriptor: Lisp_Object;
      let mut watch_object: Lisp_Object;
      let mut file: Lisp_Object;
      let mut actions: Lisp_Object;

      /* Read one event.  */
      let ret: c_int = kevent (kqueuefd, core::ptr::null (), 0, &raw mut kev, 1, &nullts);
      if ret < 1
        {
          /* All events read.  */
          return;
        }

      /* Determine descriptor and file name.  */
      descriptor = make_fixnum (kev.ident as isize);
      watch_object = assq_no_quit (descriptor, watch_list);
      if CONSP (watch_object)
        { file = XCAR (XCDR (watch_object)); }
      else
        { continue; }

      /* Determine event actions.  */
      actions = Qnil ();
      if kev.fflags & NOTE_DELETE as c_uint != 0
        { actions = Fcons (Qdelete (), actions); }
      if kev.fflags & NOTE_WRITE as c_uint != 0
        {
          /* Check, whether this is a directory event.  */
          if NILP (Fnth (make_fixnum (4), watch_object))
            { actions = Fcons (Qwrite (), actions); }
          else
            { kqueue_compare_dir_list (watch_object); }
        }
      if kev.fflags & NOTE_EXTEND as c_uint != 0
        { actions = Fcons (Qextend (), actions); }
      if kev.fflags & NOTE_ATTRIB as c_uint != 0
        { actions = Fcons (Qattrib (), actions); }
      if kev.fflags & NOTE_LINK as c_uint != 0
        { actions = Fcons (Qlink (), actions); }
      /* It would be useful to know the target of the rename operation.
         At this point, it is not possible.  Happens only when the upper
         directory is monitored.  */
      if kev.fflags & NOTE_RENAME as c_uint != 0
        { actions = Fcons (Qrename (), actions); }
      if kev.fflags & NOTE_REVOKE as c_uint != 0
        { actions = Fcons (Qrevoke (), actions); }

      /* Create the event.  */
      if !NILP (actions)
        { kqueue_generate_event (watch_object, actions, file, Qnil ()); }

      /* Cancel monitor if file or directory is deleted or renamed or
         the file system is unmounted.  */
      if kev.fflags & (NOTE_DELETE as c_uint | NOTE_RENAME as c_uint | NOTE_REVOKE as c_uint) != 0
        { Fkqueue_rm_watch (descriptor); }
    }
}

/* C 338-473: the first subr.  */
#[cfg (feature = "have_kqueue")]
DEFUN! ("kqueue-add-watch", Fkqueue_add_watch, Skqueue_add_watch, 3, 3, 0,
       doc: "Add a watch for filesystem events pertaining to FILE.\n\nThis arranges for filesystem events pertaining to FILE to be reported\nto Emacs.  Use `kqueue-rm-watch' to cancel the watch.\n\nReturned value is a descriptor for the added watch.  If the file cannot be\nwatched for some reason, this function signals a `file-notify-error' error.\n\nFLAGS is a list of events to be watched for.  It can include the\nfollowing symbols:\n\n  `create' -- FILE was created\n  `delete' -- FILE was deleted\n  `write'  -- FILE has changed\n  `extend' -- FILE was extended\n  `attrib' -- a FILE attribute was changed\n  `link'   -- a FILE's link count was changed\n  `rename' -- FILE was moved to FILE1\n  `revoke' -- FILE was unmounted\n\nWhen any event happens, Emacs will call the CALLBACK function passing\nit a single argument EVENT, which is of the form\n\n  (DESCRIPTOR ACTIONS FILE [FILE1])\n\nDESCRIPTOR is the same object as the one returned by this function.\nACTIONS is a list of events.\n\nFILE is the name of the file whose event is being reported.  FILE1\nwill be reported only in case of the `rename' event.  This is possible\nonly when the upper directory of the renamed file is watched.",
       (file: Lisp_Object, flags: Lisp_Object, callback: Lisp_Object),
       {
  let mut file = file;
  let mut watch_object: Lisp_Object;
  let mut dir_list: Lisp_Object;
  let mut maxfd: c_int;
  let fd: c_int;
  let mut oflags: c_int;
  let mut fflags: c_ushort = 0;
  let mut kev: kevent = std::mem::zeroed ();
  #[cfg (not (feature = "no_getrlimit"))]
  let mut rlim: rlimit = std::mem::zeroed ();

  /* Check parameters.  */
  CHECK_STRING (file);
  file = Fdirectory_file_name (Fexpand_file_name (file, Qnil ()));
  if NILP (Ffile_exists_p (file))
    { report_file_error (c"File does not exist".as_ptr (), file); }

  CHECK_LIST (flags);

  if !FUNCTIONP (callback)
    { wrong_type_argument (Qinvalid_function (), callback); }

  /* Check available file descriptors.  */
  #[cfg (not (feature = "no_getrlimit"))]
  {
    if getrlimit (RLIMIT_NOFILE, &raw mut rlim) == 0
      { maxfd = rlim.rlim_cur as c_int; }
    else
      {
        maxfd = 256;
      }
  }
  #[cfg (feature = "no_getrlimit")]
  {
    maxfd = 256;
  }
  let _ = &mut maxfd;

  /* We assume 50 file descriptors are sufficient for the rest of Emacs.  */
  let watch_list_len: isize = list_length (watch_list);
  if ((maxfd - 50) as isize) < watch_list_len
    { xsignal2
         (Qfile_notify_error (),
          build_string (c"File watching not possible, no file descriptor left".as_ptr ()),
          make_fixnum (watch_list_len)); }

  if kqueuefd < 0
    {
      /* Create kqueue descriptor.  */
      kqueuefd = kqueue ();
      if kqueuefd < 0
        { report_file_notify_error (c"File watching is not available".as_ptr (), Qnil ()); }

      /* Start monitoring for possible I/O.  */
      add_read_fd (kqueuefd, Some (kqueue_callback), core::ptr::null_mut ());

      watch_list = Qnil ();
    }

  /* Open file.  */
  let encoded_file: Lisp_Object = ENCODE_FILE (file);
  oflags = O_NONBLOCK;
  #[cfg (not (feature = "no_o_evtonly"))]
  { oflags |= O_EVTONLY; }
  #[cfg (feature = "no_o_evtonly")]
  { oflags |= O_RDONLY; }
  #[cfg (not (feature = "no_o_symlink"))]
  { oflags |= O_SYMLINK; }
  #[cfg (feature = "no_o_symlink")]
  { oflags |= O_NOFOLLOW; }
  fd = emacs_open (SSDATA (encoded_file), oflags, 0);
  if fd == -1
    { report_file_error (c"File cannot be opened".as_ptr (), file); }

  /* Assemble filter flags  */
  if !NILP (Fmember (Qdelete (), flags)) { fflags |= NOTE_DELETE; }
  if !NILP (Fmember (Qwrite (), flags))  { fflags |= NOTE_WRITE; }
  if !NILP (Fmember (Qextend (), flags)) { fflags |= NOTE_EXTEND; }
  if !NILP (Fmember (Qattrib (), flags)) { fflags |= NOTE_ATTRIB; }
  if !NILP (Fmember (Qlink (), flags))   { fflags |= NOTE_LINK; }
  if !NILP (Fmember (Qrename (), flags)) { fflags |= NOTE_RENAME; }
  if !NILP (Fmember (Qrevoke (), flags)) { fflags |= NOTE_REVOKE; }

  /* Register event.  C 447-452 is the `EV_SET` macro, which assigns the six members; the port
     writes the assignment it expands to (PORTING.md 0.1).  */
  kev = kevent { ident: fd as usize, filter: EVFILT_VNODE,
                  flags: EV_ADD
                         | if cfg! (feature = "no_ev_enable") { 0 } else { EV_ENABLE }
                         | EV_CLEAR,
                  fflags: fflags as c_uint, data: 0, udata: core::ptr::null_mut () };

  if kevent (kqueuefd, &raw const kev, 1, core::ptr::null_mut (), 0, core::ptr::null ()) < 0
    {
      emacs_close (fd);
      report_file_error (c"Cannot watch file".as_ptr (), file);
    }

  /* Store watch object in watch list.  */
  let watch_descriptor: Lisp_Object = make_fixnum (fd as isize);
  if NILP (Ffile_directory_p (file))
    { watch_object = list4 (watch_descriptor, file, flags, callback); }
  else
    {
      dir_list = directory_files_internal (file, Qnil (), Qnil (), Qnil (), true, Qnil (),
                                          Qnil ());
      watch_object = list5 (watch_descriptor, file, flags, callback, dir_list);
    }
  watch_list = Fcons (watch_object, watch_list);

  watch_descriptor
});

/* C 475-503: the second subr.  */
#[cfg (feature = "have_kqueue")]
DEFUN! ("kqueue-rm-watch", Fkqueue_rm_watch, Skqueue_rm_watch, 1, 1, 0,
       doc: "Remove an existing WATCH-DESCRIPTOR.\n\nWATCH-DESCRIPTOR should be an object returned by `kqueue-add-watch'.",
       (watch_descriptor: Lisp_Object),
       {
  let watch_object: Lisp_Object = assq_no_quit (watch_descriptor, watch_list);

  if !CONSP (watch_object)
    { xsignal2 (Qfile_notify_error (), build_string (c"Not a watch descriptor".as_ptr ()),
                 watch_descriptor); }

  eassert! (FIXNUMP (watch_descriptor));
  let fd: c_int = XFIXNUM (watch_descriptor) as c_int;
  if fd >= 0
    { emacs_close (fd); }

  /* Remove watch descriptor from watch list.  */
  watch_list = Fdelq (watch_object, watch_list);

  if NILP (watch_list) && kqueuefd >= 0
    {
      delete_read_fd (kqueuefd);
      emacs_close (kqueuefd);
      kqueuefd = -1;
    }

  Qt ()
});

/* C 505-517: the third subr.  */
#[cfg (feature = "have_kqueue")]
DEFUN! ("kqueue-valid-p", Fkqueue_valid_p, Skqueue_valid_p, 1, 1, 0,
       doc: "Check a watch specified by its WATCH-DESCRIPTOR.\n\nWATCH-DESCRIPTOR should be an object returned by `kqueue-add-watch'.\n\nA watch can become invalid if the file or directory it watches is\ndeleted, or if the watcher thread exits abnormally for any other\nreason.  Removing the watch by calling `kqueue-rm-watch' also makes it\ninvalid.",
       (watch_descriptor: Lisp_Object),
       {
  if NILP (assq_no_quit (watch_descriptor, watch_list)) { Qnil () } else { Qt () }
});

/* C 520-523: `void globals_of_kqueue (void)`.  */
#[cfg (feature = "have_kqueue")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn globals_of_kqueue ()
{
  watch_list = Qnil ();
}

/* C 526-546: `void syms_of_kqueue (void)`.  */
#[cfg (feature = "have_kqueue")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_kqueue ()
{
  defsubr (&raw mut Skqueue_add_watch);
  defsubr (&raw mut Skqueue_rm_watch);
  defsubr (&raw mut Skqueue_valid_p);

  /* Event types.  */
  DEFSYM! (Qcreate, "create");
  DEFSYM! (Qdelete, "delete");	/* NOTE_DELETE  */
  DEFSYM! (Qwrite, "write");	/* NOTE_WRITE  */
  DEFSYM! (Qextend, "extend");	/* NOTE_EXTEND  */
  DEFSYM! (Qattrib, "attrib");	/* NOTE_ATTRIB  */
  DEFSYM! (Qlink, "link");	/* NOTE_LINK  */
  DEFSYM! (Qrename, "rename");	/* NOTE_RENAME  */
  DEFSYM! (Qrevoke, "revoke");	/* NOTE_REVOKE  */

  staticpro (&raw mut watch_list);

  Fprovide (intern_c_string (c"kqueue".as_ptr ()), Qnil ());
}

/* C 548-550: the file's closing note.
   PROBLEMS
     * https://bugs.launchpad.net/ubuntu/+source/libkqueue/+bug/1514837
       prevents tests on Ubuntu.  */
