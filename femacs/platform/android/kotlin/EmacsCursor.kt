// femacs: port of java/org/gnu/emacs/EmacsCursor.java @ fd7afced403.

package org.gnu.emacs

import android.os.Build
import android.view.PointerIcon

/* Cursor wrapper.  Note that pointer icons are not supported prior to
   Android 24.  */

/* The Java returns early from the constructor when the API is too old; Kotlin's `init` cannot
   return, so the same branch becomes the `else` of one expression (P-83 ①).  */
class EmacsCursor(glyph: Int) : EmacsHandleObject() {
  /* The pointer icon associated with this cursor.  */
  @JvmField val icon: PointerIcon?

  init {
    icon =
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N)
        null
      else
        PointerIcon.getSystemIcon(EmacsService.SERVICE!!, glyph)
  }
}
