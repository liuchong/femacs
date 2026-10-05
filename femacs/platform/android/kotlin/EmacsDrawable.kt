// femacs: port of java/org/gnu/emacs/EmacsDrawable.java @ fd7afced403.

package org.femacs

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Rect

/* Kotlin's interfaces are abstract and its members are public; the JVM names and descriptors are
   the Java ones, so the C-ABI side is unaffected (PORTING.md P-83 ②).  `getBitmap` is nullable
   because `EmacsPixmap.destroyHandle` sets the field to null (P-83 ⑤).  */
interface EmacsDrawable {
  fun lockCanvas(gc: EmacsGC): Canvas
  fun damageRect(damageRect: Rect)
  fun damageRect(left: Int, top: Int, right: Int, bottom: Int)
  fun getBitmap(): Bitmap?
  fun isDestroyed(): Boolean
}
