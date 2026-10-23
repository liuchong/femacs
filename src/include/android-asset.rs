// femacs: port of src/android-asset.h @ fd7afced403.

// This header is the port's emulation of the Android asset manager API for the builds that need
// it (`android.c` includes it "whenever appropriate", C 29-33); the frozen target does not build
// it, but `tools/check-cond-eval.py src/android-asset.h` reports `0 conditions` - the file has no
// `#if` of its own - so every line below is live text and is carried.
//
// Two things C leaves to the *includer* the mirror must reach itself (PORTING.md P-26/P-72 rule
// 4, the `androidterm.rs` style): the JNI ABI (`include/jni.rs`, the measured NDK `<jni.h>`) and
// the C library entries (`<stdlib.h>`'s `malloc`/`calloc`/`free`, `<errno.h>`'s `errno`/`ENOMEM`/
// `EIO`/`ESPIPE`, `<limits.h>`'s `INT_MAX`, `minmax.h`'s `MIN`, `<unistd.h>`'s `SEEK_*` and
// `<sys/types.h>`'s `off_t`).  C 20's `#include <android/log.h>` is *not* mirrored: nothing in
// this file's own text names a `<android/log.h>` entry point (`android.c` 358-379 is where
// `__android_log_print` is used).
//
// Two shapes are recorded rather than invented:
//   * C's `errno` is the *frozen target's* macro (`errno.rs`'s `errno ()` -> `__error ()`), while
//     a real Android build's `errno` is bionic's `__errno ()`; that is a system-interface
//     difference for whatever freezes an Android configuration, recorded in
//     docs/ports/include-android-asset.md.
//   * `assert` keeps C's name with the frozen `NDEBUG` body (the `lib-src/ebrowse.rs` 6 precedent):
//     `NDEBUG` is defined for the frozen configuration (macros.tsv 2892), so C's `assert (cond)`
//     expands to `((void) 0)` and does *not* evaluate its operand (PORTING.md 2.5).
//   * C 91-92's two `static` prototypes (`android_scan_directory_tree`, `android_extract_long`)
//     are declared by *this* header and defined by `androidvfs.c` in the same translation unit, so
//     they sit in a glob-imported module whose import the unit's definitions shadow
//     (PORTING.md 0.1-10/P-17/P-21).

include!("jni.rs");                             // the NDK's `<jni.h>` mirror: JNIEnv, jobject, …
include!("../../lib/include/stdlib.rs");        // malloc, calloc, free
include!("../../lib/include/errno.rs");         // errno, ENOMEM, EIO
include!("../../lib/include/limits.in.rs");     // INT_MAX
include!("../../lib/include/minmax.rs");        // MIN
include!("../../lib/include/unistd.in.rs");     // SEEK_SET, SEEK_CUR, SEEK_END
include!("../../lib/include/sys_types.in.rs");  // off_t

/* The header reads `ESPIPE` from `<errno.h>` (C 520).  The port's `<errno.h>` slice
   (`lib/include/errno.rs`) carries the other errno values this file names but not that one,
   so the includer supplies it with the frozen target's number (`sys/errno.h`: `#define ESPIPE 29`),
   the way a unit supplies what its headers leave to it (PORTING.md P-26/P-72).  */
pub const ESPIPE: c_int = 29;

/* `<assert.h>`'s `assert`: the frozen configuration defines `NDEBUG`, so C's macro expands to
   `((void) 0)` and its operand is never evaluated; the port keeps the name and the same body
   (`lib-src/ebrowse.rs`'s `assert` precedent), and the layout assertions elsewhere use
   `core::assert!` explicitly because this macro shadows the prelude's.  */
macro_rules! assert {
  ($cond:expr) => { () };
}

/* Forward declarations.  */

// C 26: `static const char *directory_tree;` - a tentative definition, i.e. a null pointer.
pub static mut directory_tree: *const c_char = core::ptr::null ();

pub mod android_asset_decls {
  use super::*;

  unsafe extern "C" {
    /// C: `static const char *android_scan_directory_tree (const char *, size_t *);` - a `static`
    /// definition in the unit (androidvfs.c), no symbol either way.
    pub fn android_scan_directory_tree (arg0: *const c_char, arg1: *mut size_t) -> *const c_char;
    /// C: `static unsigned int android_extract_long (const char *);`
    pub fn android_extract_long (arg0: *const c_char) -> c_uint;
  }
}
use android_asset_decls::*;

/* This file contains an emulation of the Android asset manager API
   used on builds for Android 2.2.  It is included by android.c
   whenever appropriate.

   The replacements in this file are not thread safe and must only be
   called from the creating thread.  */

// C 37-57.
#[repr(C)]
pub struct android_asset_manager
{
  /* JNI environment.  */
  pub env: *mut JNIEnv,

  /* Asset manager class and functions.  */
  pub class: jclass,
  pub open_fd: jmethodID,
  pub open: jmethodID,

  /* Asset file descriptor class and functions.  */
  pub fd_class: jclass,
  pub get_length: jmethodID,
  pub create_input_stream: jmethodID,
  pub close: jmethodID,

  /* Input stream class and functions.  */
  pub input_stream_class: jclass,
  pub read: jmethodID,
  pub stream_close: jmethodID,

  /* Associated asset manager object.  */
  pub asset_manager: jobject,
}

// C 59.
pub type AAssetManager = android_asset_manager;

// C 61-86.
#[repr(C)]
pub struct android_asset
{
  /* The asset manager.  */
  pub manager: *mut AAssetManager,

  /* The length of the asset, or -1.  */
  pub length: jlong,

  /* The number of bytes which have already been read from this
     asset.  */
  pub bytes_read: jlong,

  /* The asset file descriptor and input stream.  */
  pub fd: jobject,
  pub stream: jobject,

  /* Alternatively, the name of the file.  */
  pub name: jstring,

  /* The mode.  */
  pub mode: c_int,
}

// C 88.
pub type AAsset = android_asset;

// C 171-176: C's anonymous enum, so the port declares the same two constants.
pub const AASSET_MODE_STREAMING: c_int = 0;
pub const AASSET_MODE_BUFFER: c_int = 1;

unsafe fn AAssetManager_fromJava (env: *mut JNIEnv, java_manager: jobject) -> *mut AAssetManager
{
  let manager: *mut AAssetManager;

  manager = malloc (std::mem::size_of::<android_asset_manager> ()) as *mut AAssetManager;

  if manager.is_null ()
    {
      return core::ptr::null_mut ();
    }

  unsafe {
    (*manager).env = env;
    (*manager).asset_manager
      = ((*(*env)).NewGlobalRef.unwrap ()) (env, java_manager);
  }

  if unsafe { (*manager).asset_manager }.is_null ()
    {
      free (manager as *mut c_void);
      return core::ptr::null_mut ();
    }

  unsafe {
    (*manager).class
      = ((*(*env)).FindClass.unwrap ()) (env, c"android/content/res/AssetManager".as_ptr ());
  }
  assert! (unsafe { (*manager).class });

  unsafe {
    (*manager).open_fd
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).class, c"openFd".as_ptr (),
                                           c"(Ljava/lang/String;)Landroid/content/res/AssetFileDescriptor;".as_ptr ());
  }
  assert! (unsafe { (*manager).open_fd });

  unsafe {
    (*manager).open
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).class, c"open".as_ptr (),
                                           c"(Ljava/lang/String;)Ljava/io/InputStream;".as_ptr ());
  }
  assert! (unsafe { (*manager).open });

  unsafe {
    (*manager).fd_class
      = ((*(*env)).FindClass.unwrap ()) (env, c"android/content/res/AssetFileDescriptor".as_ptr ());
  }
  assert! (unsafe { (*manager).fd_class });

  unsafe {
    (*manager).get_length
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).fd_class, c"getLength".as_ptr (),
                                           c"()J".as_ptr ());
  }
  assert! (unsafe { (*manager).get_length });

  unsafe {
    (*manager).create_input_stream
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).fd_class,
                                           c"createInputStream".as_ptr (),
                                           c"()Ljava/io/FileInputStream;".as_ptr ());
  }
  assert! (unsafe { (*manager).create_input_stream });

  unsafe {
    (*manager).close
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).fd_class,
                                           c"close".as_ptr (), c"()V".as_ptr ());
  }
  assert! (unsafe { (*manager).close });

  unsafe {
    (*manager).input_stream_class
      = ((*(*env)).FindClass.unwrap ()) (env, c"java/io/InputStream".as_ptr ());
  }
  assert! (unsafe { (*manager).input_stream_class });

  unsafe {
    (*manager).read
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).input_stream_class,
                                           c"read".as_ptr (), c"([B)I".as_ptr ());
  }
  assert! (unsafe { (*manager).read });

  unsafe {
    (*manager).stream_close
      = ((*(*env)).GetMethodID.unwrap ()) (env, (*manager).input_stream_class,
                                           c"close".as_ptr (), c"()V".as_ptr ());
  }
  assert! (unsafe { (*manager).stream_close });

  /* Now convert all the class references to global ones.  */
  let mut temp: jclass;
  unsafe {
    temp = (*manager).class;
    (*manager).class
      = ((*(*env)).NewGlobalRef.unwrap ()) (env, temp);
  }
  assert! (unsafe { (*manager).class });
  ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, temp);
  unsafe {
    temp = (*manager).fd_class;
    (*manager).fd_class
      = ((*(*env)).NewGlobalRef.unwrap ()) (env, temp);
  }
  assert! (unsafe { (*manager).fd_class });
  ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, temp);
  unsafe {
    temp = (*manager).input_stream_class;
    (*manager).input_stream_class
      = ((*(*env)).NewGlobalRef.unwrap ()) (env, temp);
  }
  assert! (unsafe { (*manager).input_stream_class });
  ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, temp);

  /* Return the asset manager.  */
  return manager;
}

unsafe fn AAssetManager_open (manager: *mut AAssetManager, c_name: *const c_char, mode: c_int)
  -> *mut AAsset
{
  let mut desc: jobject = core::ptr::null_mut ();
  let mut name: jstring;                     /* C: `jstring name;` (PORTING.md 1.5)  */
  let mut asset: *mut AAsset = core::ptr::null_mut ();
  let asset_dir: *const c_char;
  let mut st_size: jlong = -1;

  /* C's `goto fail` (C 220 and the six below) leaves the attempt region for the `fail:` cleanup:
     the port names the outer block after C's own jump word (`'goto`) and keeps C's `fail:` label
     on the inner block, so every jump is a `break 'fail` and the block's value carries C's own
     `return` value (PORTING.md 2.2).  */
  'goto: {
    'fail: {
      /* Push a local frame.  */
      ((unsafe { *(*(*manager).env) }).PushLocalFrame.unwrap ()) (unsafe { (*manager).env }, 3);

      if ((unsafe { *(*(*manager).env) }).ExceptionCheck.unwrap ()) (unsafe { (*manager).env }) != 0
        {
          break 'fail;
        }

      /* If the directory tree has been initialized, it is possible to avoid
         opening an AssetFileDescriptor and thereby access compressed
         assets, without sacrificing the possibility of reading the file
         size.  */
      if ! (unsafe { directory_tree }).is_null ()
        {
          /* Search for a matching asset.  */
          asset_dir = android_scan_directory_tree (c_name, core::ptr::null_mut ());
          if asset_dir.is_null ()
            {
              break 'fail;
            }

          /* Extract the size of the asset from this directory.  */
          st_size = android_extract_long (asset_dir.offset (-8)) as jlong;
        }

      /* Encoding issues can be ignored for the time being as there are only
         ASCII file names in Emacs.  */
      name = ((unsafe { *(*(*manager).env) }).NewStringUTF.unwrap ()) (unsafe { (*manager).env },
                                                                      c_name);

      if name.is_null ()
        {
          break 'fail;
        }

      /* If st_size has been set, it ought to be possible to open an input
         stream directly upon the first attempt to read from the asset,
         sidestepping the intermediate AssetFileDescriptor.  */

      if st_size < 0
        {
          /* Otherwise attempt to open an ``AssetFileDescriptor''.  */
          desc = ((unsafe { *(*(*manager).env) }).CallObjectMethod.unwrap ())
            (unsafe { (*manager).env }, unsafe { (*manager).asset_manager },
             unsafe { (*manager).open_fd }, name);
        }

      /* Allocate the asset.  */
      asset = calloc (1, std::mem::size_of::<android_asset> ()) as *mut AAsset;

      if asset.is_null ()
        {
          break 'fail;
        }

      if ! desc.is_null ()
        {
          /* Pop the local frame and return desc.  */
          desc = ((unsafe { *(*(*manager).env) }).NewGlobalRef.unwrap ()) (unsafe { (*manager).env },
                                                                          desc);

          if desc.is_null ()
            {
              break 'fail;
            }

          /* Will be released by PopLocalFrame.  */
          name = core::ptr::null_mut ();
        }
      else /* if (name) */
        {
          /* If no file descriptor was returned, more recent Android
             releases will probably have signaled an exception.  */
          ((unsafe { *(*(*manager).env) }).ExceptionClear.unwrap ()) (unsafe { (*manager).env });

          /* Pop the local frame and return name.  */
          name = ((unsafe { *(*(*manager).env) }).NewGlobalRef.unwrap ()) (unsafe { (*manager).env },
                                                                          name);

          if name.is_null ()
            {
              break 'fail;
            }
        }

      ((unsafe { *(*(*manager).env) }).PopLocalFrame.unwrap ()) (unsafe { (*manager).env },
                                                                core::ptr::null_mut ());

      unsafe {
        (*asset).manager = manager;
        (*asset).length = st_size;
        (*asset).fd = desc;
        (*asset).name = name;
        (*asset).mode = mode;
      }

      break 'goto asset;
    }

    if ! desc.is_null ()
      {
        ((unsafe { *(*(*manager).env) }).CallVoidMethod.unwrap ())
          (unsafe { (*manager).env }, desc, unsafe { (*manager).close });
      }

    ((unsafe { *(*(*manager).env) }).ExceptionClear.unwrap ()) (unsafe { (*manager).env });
    ((unsafe { *(*(*manager).env) }).PopLocalFrame.unwrap ()) (unsafe { (*manager).env },
                                                              core::ptr::null_mut ());
    free (asset as *mut c_void);
    break 'goto core::ptr::null_mut ();
  }
}

unsafe fn AAsset_close (asset: *mut AAsset)
{
  let env: *mut JNIEnv;

  env = unsafe { (*(*asset).manager).env };

  if ! (unsafe { (*asset).fd }).is_null ()
    {
      ((unsafe { *(*env) }).CallVoidMethod.unwrap ())
        (unsafe { (*(*asset).manager).env }, unsafe { (*asset).fd },
         unsafe { (*(*asset).manager).close });
      ((unsafe { *(*env) }).DeleteGlobalRef.unwrap ())
        (unsafe { (*(*asset).manager).env }, unsafe { (*asset).fd });
    }

  if ! (unsafe { (*asset).stream }).is_null ()
    {
      ((unsafe { *(*env) }).CallVoidMethod.unwrap ())
        (unsafe { (*(*asset).manager).env }, unsafe { (*asset).stream },
         unsafe { (*(*asset).manager).stream_close });
      ((unsafe { *(*env) }).DeleteGlobalRef.unwrap ())
        (unsafe { (*(*asset).manager).env }, unsafe { (*asset).stream });
    }

  if ! (unsafe { (*asset).name }).is_null ()
    {
      ((unsafe { *(*env) }).DeleteGlobalRef.unwrap ())
        (unsafe { (*(*asset).manager).env }, unsafe { (*asset).name });
    }

  free (asset as *mut c_void);
}

/* Create an input stream associated with the given ASSET.  Set
   ASSET->stream to its global reference.

   Value is 1 upon failure, else 0.  ASSET must not already have an
   input stream.  */

unsafe fn android_asset_create_stream (asset: *mut AAsset) -> c_int
{
  let stream: jobject;
  let env: *mut JNIEnv;

  assert! (unsafe { ! (*asset).fd.is_null () || ! (*asset).name.is_null () });

  env = unsafe { (*(*asset).manager).env };

  if ! (unsafe { (*asset).name }).is_null ()
    {
      stream = ((unsafe { *(*env) }).CallObjectMethod.unwrap ())
        (env, unsafe { (*(*asset).manager).asset_manager },
         unsafe { (*(*asset).manager).open }, unsafe { (*asset).name });
    }
  else
    {
      stream = ((unsafe { *(*env) }).CallObjectMethod.unwrap ())
        (env, unsafe { (*asset).fd }, unsafe { (*(*asset).manager).create_input_stream });
    }

  if stream.is_null ()
    {
      ((unsafe { *(*env) }).ExceptionClear.unwrap ()) (env);
      return 1;
    }

  unsafe {
    (*asset).stream = ((*(*env)).NewGlobalRef.unwrap ()) (env, stream);
  }

  if (unsafe { (*asset).stream }).is_null ()
    {
      ((unsafe { *(*env) }).ExceptionClear.unwrap ()) (env);
      ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, stream);
      return 1;
    }

  ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, stream);
  return 0;
}

/* Read NBYTES from the specified asset into the given BUFFER;

   Internally, allocate a Java byte array containing 4096 elements and
   copy the data to and from that array.

   Value is the number of bytes actually read, 0 at EOF, or -1 upon
   failure, in which case errno is set accordingly.  If NBYTES is
   zero, behavior is undefined.  */

unsafe fn android_asset_read_internal (asset: *mut AAsset, mut nbytes: c_int,
                                           mut buffer: *mut c_char) -> c_int
{
  let stash: jbyteArray;
  let env: *mut JNIEnv;
  let mut bytes_read: jint;
  let mut total: jint;

  /* Allocate a suitable amount of storage.  Either nbytes or 4096,
     whichever is larger.  */
  env = unsafe { (*(*asset).manager).env };
  stash = ((unsafe { *(*env) }).NewByteArray.unwrap ()) (env, MIN! (nbytes, 4096));

  if stash.is_null ()
    {
      ((unsafe { *(*env) }).ExceptionClear.unwrap ()) (env);
      *errno () = ENOMEM;
      return -1;
    }

  /* Try to create an input stream.  */

  if (unsafe { (*asset).stream }).is_null () && android_asset_create_stream (asset) != 0
    {
      ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, stash);
      *errno () = ENOMEM;
      return -1;
    }

  /* Start reading.  */

  total = 0;

  /* C's `goto out` and `goto out_errno` leave the read loop for one of two tails: the port names
     the outer block after C's own jump word (`'goto`, whose value carries C's `return` value) and
     keeps C's `out:` and `out_errno:` labels on the two inner blocks, so `break 'out` and
     `break 'out_errno` are the two jumps (PORTING.md 2.2).  */
  'goto: {
    'out_errno: {
      'out: {
        while nbytes != 0
          {
            bytes_read = ((unsafe { *(*env) }).CallIntMethod.unwrap ())
              (env, unsafe { (*asset).stream }, unsafe { (*(*asset).manager).read }, stash);

            /* Detect error conditions.  */

            if ((unsafe { *(*env) }).ExceptionCheck.unwrap ()) (env) != 0
              {
                break 'out_errno;
              }

            /* Detect EOF.  */

            if bytes_read == -1
              {
                break 'out;
              }

            /* Finally write out the amount that was read.  */
            bytes_read = MIN! (bytes_read, nbytes);
            ((unsafe { *(*env) }).GetByteArrayRegion.unwrap ())
              (env, stash, 0, bytes_read, buffer as *mut jbyte);

            buffer = unsafe { buffer.add (bytes_read as usize) };
            total += bytes_read;
            nbytes -= bytes_read;
          }

        /* Make sure the value of nbytes still makes sense.  */
        assert! (nbytes >= 0);
      }

      /* C's `out:` tail.  */
      unsafe { (*asset).bytes_read += total as jlong; }
      ((unsafe { *(*env) }).ExceptionClear.unwrap ()) (env);
      ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, stash);
      break 'goto total;
    }

    /* Return an error indication if an exception arises while the file
       is being read.  */
    unsafe { (*asset).bytes_read += total as jlong; }
    ((unsafe { *(*env) }).ExceptionClear.unwrap ()) (env);
    ((unsafe { *(*env) }).DeleteLocalRef.unwrap ()) (env, stash);
    *errno () = EIO;
    break 'goto -1;
  }
}

unsafe fn AAsset_getLength (asset: *mut AAsset) -> c_long
{
  let env: *mut JNIEnv;

  if unsafe { (*asset).length } != -1
    {
      return unsafe { (*asset).length } as c_long;
    }
  if (unsafe { (*asset).fd }).is_null ()
    {
      return 0;
    }

  env = unsafe { (*(*asset).manager).env };
  unsafe {
    (*asset).length
      = ((*(*env)).CallLongMethod.unwrap ()) (env, (*asset).fd, (*(*asset).manager).get_length)
        as jlong;
  }
  return unsafe { (*asset).length } as c_long;
}

unsafe fn AAsset_getBuffer (asset: *mut AAsset) -> *mut c_char
{
  let length: c_long = AAsset_getLength (asset);
  let buffer: *mut c_char;                   /* C: `char *buffer;` (PORTING.md 1.5)  */

  if length == 0
    {
      return core::ptr::null_mut ();
    }

  buffer = malloc (length as usize) as *mut c_char;

  if buffer.is_null ()
    {
      return core::ptr::null_mut ();
    }

  if android_asset_read_internal (asset, length as c_int, buffer) as c_long != length
    {
      free (buffer as *mut c_void);
      return core::ptr::null_mut ();
    }

  return buffer;
}

unsafe fn AAsset_read (asset: *mut AAsset, buffer: *mut c_void, size: size_t) -> size_t
{
  return android_asset_read_internal (asset, MIN! (size, INT_MAX as usize) as c_int,
                                      buffer as *mut c_char) as size_t;
}

unsafe fn AAsset_seek (asset: *mut AAsset, offset: off_t, whence: c_int) -> off_t
{
  /* It is necessary to attempt to satisfy calls to `seek' to identify
     the current offset or which do not actually move the file pointer,
     in the interests of scenarios where, e.g., Finsert_file_contents is
     called with BEG set to 0.  */

  /* C's `switch` falls through from `SEEK_SET` into `SEEK_CUR` into `SEEK_END`, and Rust has no
     fall-through: the port keeps C's three control words as the blocks' names - `'switch` on the
     dispatch, `'case` on the shared guard tests, `'default` on C's `default: break;` - and spells
     those tests once, in C's order, with C's guards (PORTING.md 2.2).  */
  'switch: {
    'case: {
      if whence == SEEK_SET || whence == SEEK_CUR || whence == SEEK_END
        {
          if whence == SEEK_SET && offset == unsafe { (*asset).bytes_read } as off_t
            {
              return unsafe { (*asset).bytes_read } as off_t;
            }
          if whence != SEEK_END && offset == 0
            {
              return unsafe { (*asset).bytes_read } as off_t;
            }
          if offset == 0 && unsafe { (*asset).bytes_read == (*asset).length }
            {
              return unsafe { (*asset).bytes_read } as off_t;
            }
        }
      break 'case;
    }
    'default: {
      break 'default;
    }
    break 'switch;
  }

  /* Java InputStreams don't support seeking at all.  */
  *errno () = ESPIPE;
  return -1;
}
