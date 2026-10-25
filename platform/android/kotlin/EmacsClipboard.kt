// femacs: port of java/org/gnu/emacs/EmacsClipboard.java @ fd7afced403.

package org.femacs

import android.content.res.AssetFileDescriptor

/* Clipboard helper for the supported Android API21+ floor. */

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
      return EmacsSdk11Clipboard()
  }
  }
}
