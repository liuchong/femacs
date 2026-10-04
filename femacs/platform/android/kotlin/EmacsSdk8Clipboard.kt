// femacs: port of java/org/gnu/emacs/EmacsSdk8Clipboard.java @ fd7afced403.

package org.gnu.emacs

/* Importing the entire package instead of just the legacy
   ClipboardManager class avoids the deprecation warning.  */

import android.content.ClipboardManager
import android.content.Context
import android.content.res.AssetFileDescriptor

/* This class implements EmacsClipboard for Android 2.2 and other
   similarly old systems.  */

/* Java's `manager` is assigned in the constructor and never null afterwards, so the port's is a
   `@JvmField` with the constructor's value (P-83 ②).  Every method that returns null in the
   reference returns a nullable type here (P-83 ⑤).  */
class EmacsSdk8Clipboard : EmacsClipboard() {
  @JvmField var manager: ClipboardManager

  init {
    val what: String = Context.CLIPBOARD_SERVICE
    val context: Context = EmacsService.SERVICE!!
    @Suppress("UNCHECKED_CAST")
    manager = context.getSystemService(what) as ClipboardManager
  }

  /* Save the STRING into the clipboard by way of text copied by the
     user.  */

  override fun setClipboard(string: String) {
    manager.text = string
  }

  /* Return whether or not Emacs owns the clipboard.  Value is 1 if
     Emacs does, 0 if Emacs does not, and -1 if that information is
     unavailable.  */

  override fun ownsClipboard(): Int {
    return -1
  }

  /* Return whether or not clipboard content currently exists.  */

  override fun clipboardExists(): Boolean {
    return manager.hasText()
  }

  /* Return the current content of the clipboard, as plain text, or
     NULL if no content is available.  */

  override fun getClipboard(): String? {
    val string: String
    val text: CharSequence? = manager.text

    if (text == null)
      return null

    string = text.toString()
    return string
  }

  /* Return an array of targets currently provided by the
     clipboard, or NULL if there are none.  */

  override fun getClipboardTargets(): Array<String>? {
    return null
  }

  /* Return the clipboard data for the given target, or NULL if it
     does not exist.

     Value is normally an asset file descriptor, which in turn holds
     three important values: the file descriptor, the start offset of
     the data, and its length; length may be
     AssetFileDescriptor.UNKNOWN_LENGTH, meaning that the data extends
     from that offset to the end of the file.

     Do not use this function to open text targets; use `getClipboard'
     for that instead, as it will handle selection data consisting
     solely of a URI.  */

  override fun getClipboardData(target: String): AssetFileDescriptor? {
    return null
  }
}
