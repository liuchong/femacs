// femacs: port of java/org/gnu/emacs/EmacsSdk11Clipboard.java @ fd7afced403.

package org.gnu.emacs

import android.content.ClipData
import android.content.ClipDescription
import android.content.ClipboardManager
import android.content.ContentResolver
import android.content.Context
import android.content.res.AssetFileDescriptor
import android.net.Uri
import android.os.Build
import java.io.FileNotFoundException

/* This class implements EmacsClipboard for Android 3.0 and later
   systems.  */

/* Java's `synchronized` methods become `@Synchronized` (P-83 ④), and the two catch clauses that
   return null in the reference return a nullable type here (P-83 ⑤).  */
class EmacsSdk11Clipboard : EmacsClipboard(), ClipboardManager.OnPrimaryClipChangedListener {
  @JvmField var manager: ClipboardManager
  @JvmField var ownsClipboard: Boolean = false
  @JvmField var clipboardChangedCount: Int = 0
  @JvmField var monitoredClipboardChangedCount: Int = 0
  @JvmField var resolver: ContentResolver

  init {
    manager = EmacsService.SERVICE.getClipboardManager()

    /* The system forbids Emacs from reading clipboard data in the
       background under Android 10 or later.  */

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q)
      manager.addPrimaryClipChangedListener(this)

    /* Now obtain the content resolver used to open file
       descriptors.  */

    resolver = EmacsService.SERVICE.contentResolver
  }

  @Synchronized
  override fun onPrimaryClipChanged() {
    /* Increment monitoredClipboardChangeCount.  If it is now greater
       than clipboardChangedCount, then Emacs no longer owns the
       clipboard.  */
    monitoredClipboardChangedCount++

    if (monitoredClipboardChangedCount > clipboardChangedCount) {
      ownsClipboard = false

      /* Reset both values back to 0.  */
      monitoredClipboardChangedCount = 0
      clipboardChangedCount = 0
    }
  }

  /* Save the STRING into the clipboard by way of text copied by the
     user.  */

  @Synchronized
  override fun setClipboard(string: String) {
    val data: ClipData

    data = ClipData.newPlainText("Emacs", string)
    manager.setPrimaryClip(data)
    ownsClipboard = true

    /* onPrimaryClipChanged will be called again.  Use this
       variable to keep track of how many times the clipboard has
       been changed.  */
    ++clipboardChangedCount
  }

  /* Return whether or not Emacs owns the clipboard.  Value is 1 if
     Emacs does, 0 if Emacs does not, and -1 if that information is
     unavailable.  */

  @Synchronized
  override fun ownsClipboard(): Int {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q)
      return -1

    return if (ownsClipboard) 1 else 0
  }

  /* Return whether or not clipboard content currently exists.  */

  override fun clipboardExists(): Boolean {
    return manager.hasPrimaryClip()
  }

  /* Return the current content of the clipboard, as plain text, or
     NULL if no content is available.  */

  override fun getClipboard(): String? {
    val clip: ClipData?
    val text: CharSequence
    val context: Context

    clip = manager.primaryClip

    if (clip == null || clip.itemCount < 1)
      return null

    context = EmacsService.SERVICE
    text = clip.getItemAt(0).coerceToText(context)
    return text.toString()
  }

  /* Return an array of targets currently provided by the
     clipboard, or NULL if there are none.  */

  override fun getClipboardTargets(): Array<String>? {
    val clip: ClipData?
    val description: ClipDescription
    val typeArray: Array<String>
    var i: Int

    /* N.B. that Android calls the clipboard the ``primary clip''; it
       is not related to the X primary selection.  */
    clip = manager.primaryClip

    if (clip == null)
      return null

    description = clip.description
    i = description.mimeTypeCount
    typeArray = arrayOfNulls<String>(i) as Array<String>

    for (i in 0 until description.mimeTypeCount)
      typeArray[i] = description.getMimeType(i)

    return typeArray
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
    val data: ClipData?
    val mimeType: String
    val assetFd: AssetFileDescriptor?
    val uri: Uri?

    /* Now obtain the clipboard data and the data corresponding to
       that MIME type.  */

    mimeType = target
    data = manager.primaryClip

    if (data == null || data.itemCount < 1)
      return null

    try {
      uri = data.getItemAt(0).uri

      if (uri == null)
        return null

      /* Now open the file descriptor.  */
      assetFd = resolver.openTypedAssetFileDescriptor(uri, mimeType, null)
      return assetFd
    } catch (e: SecurityException) {
      return null
    } catch (e: FileNotFoundException) {
      return null
    }
  }
}
