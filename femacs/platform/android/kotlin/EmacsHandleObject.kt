// femacs: port of java/org/gnu/emacs/EmacsHandleObject.java @ fd7afced403.

package org.gnu.emacs

import java.lang.IllegalStateException

/* This defines something that is a so-called ``handle''.  Handles
   must be created by C code, and will remain existing until
   destroyHandle is called.  C code then refers to the handle by a
   number which maps into the Java object representing the handle.

   All handle operations must be done from the Emacs thread.  */

abstract class EmacsHandleObject {
  /* Whether or not this handle has been destroyed.  */
  @Volatile
  @JvmField var destroyed: Boolean = false

  /* The handle associated with this object, set in
     android_globalize_reference.  `android.c` 2082 finds it as
     `GetFieldID (…, "handle", "J")`, so the field keeps its name and its `J` descriptor
     (PORTING.md P-83 ②).  */
  @JvmField var handle: Long = 0

  /* `android.c` 2076 and 3099 look this method up as
     `"destroyHandle", "()V"`: a Kotlin `fun` keeps both.  Java's `throws` clause becomes
     `@Throws` so the JVM attribute stays (P-83 ⑤).  */
  @Throws(IllegalStateException::class)
  open fun destroyHandle() {
    synchronized(this) { destroyed = true }
  }

  open fun isDestroyed(): Boolean = destroyed
}
