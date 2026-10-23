// femacs: port of java/org/gnu/emacs/EmacsClipboard.java @ fd7afced403.

package org.femacs

import android.content.res.AssetFileDescriptor
import android.os.Build

/* This class provides helper code for accessing the clipboard,
   abstracting between the different interfaces on API 8 and 11.  */

/* `makeClipboard` is looked up by name and descriptor from C
   (`androidselect.c` 110-112, `()Lorg/gnu/emacs/EmacsClipboard;`), so the companion function carries
   `@JvmStatic` - without it the JVM member would live on the companion class and the lookup would
   fail (P-83 ⑪).  */
abstract class EmacsClipboard {
  abstract fun setClipboard(string: String)
  abstract fun ownsClipboard(): Int
  abstract fun clipboardExists(): Boolean
  abstract fun getClipboard(): String?

  abstract fun getClipboardTargets(): Array<String>?
  abstract fun getClipboardData(target: String): AssetFileDescriptor?

  companion object {
    /* Create the correct kind of clipboard for this system.  */

    @JvmStatic fun makeClipboard(): EmacsClipboard {
      return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB)
        EmacsSdk11Clipboard()
      else
        EmacsSdk8Clipboard()
    }
  }
}
