// femacs: port of java/org/gnu/emacs/EmacsPixmap.java @ fd7afced403.

package org.gnu.emacs

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Rect
import android.os.Build
import java.lang.IllegalArgumentException

/* Drawable backed by bitmap.  */

/* `depth`/`width`/`height` are `final` in the reference and are assigned in its constructor after
   the bitmap work, so the port declares them without initializers and assigns them at the same
   point; `@JvmField` keeps the names and descriptors `android.c` reads (P-83 ②).  `bitmap` is set
   to null by `destroyHandle`, which is why it is nullable.  */
class EmacsPixmap(width: Int, height: Int, depth: Int) : EmacsHandleObject(), EmacsDrawable {
  /* The depth of the bitmap.  This is not actually used, just defined
     in order to be consistent with X.  */
  @JvmField val depth: Int
  @JvmField val width: Int
  @JvmField val height: Int

  /* The bitmap itself.  */
  @JvmField var bitmap: Bitmap? = null

  /* The canvas used to draw to BITMAP.  */
  @JvmField var canvas: Canvas? = null

  /* Whether or not GC should be explicitly triggered upon
     release.  */
  private val needCollect: Boolean

  /* ID used to determine whether or not the GC clip rects
     changed.  */
  private var gcClipRectID: Long = 0

  init {
    if (depth != 1 && depth != 24)
      throw IllegalArgumentException("Invalid depth specified for pixmap: " + depth)

    when (depth) {
      1 ->
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
          bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ALPHA_8, false)
        else
          bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ALPHA_8)

      24 ->
        /* Emacs doesn't just use the first kind of `createBitmap'
           because the latter allows specifying that the pixmap is
           always opaque, which really increases efficiency.  */
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O)
          bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
        else
          bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888, false)
    }

    needCollect =
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.HONEYCOMB_MR1)
        /* On these old versions of Android, Bitmap.recycle frees bitmap
           contents immediately.  */
        false
      else if (Build.VERSION.SDK_INT < Build.VERSION_CODES.KITKAT)
        bitmap!!.byteCount >= 1024 * 512
      else
        bitmap!!.allocationByteCount >= 1024 * 512

    bitmap!!.eraseColor(0xff000000.toInt())

    this.width = width
    this.height = height
    this.depth = depth
  }

  override fun lockCanvas(gc: EmacsGC): Canvas {
    var i: Int

    if (canvas == null) {
      canvas = Canvas(bitmap!!)
      canvas!!.save()
    }

    /* Now see if clipping has to be redone.  */
    if (gc.clipRectID == gcClipRectID)
      return canvas!!

    /* It does have to be redone.  Reapply gc.real_clip_rects.  */
    canvas!!.restore()
    canvas!!.save()

    if (gc.real_clip_rects != null) {
      for (i in gc.real_clip_rects!!.indices)
        canvas!!.clipRect(gc.real_clip_rects!![i]!!)
    }

    /* Save the clip rect ID again.  */
    gcClipRectID = gc.clipRectID
    return canvas!!
  }

  override fun damageRect(damageRect: Rect) {
  }

  override fun damageRect(left: Int, top: Int, right: Int, bottom: Int) {
  }

  override fun getBitmap(): Bitmap? {
    return bitmap
  }

  override fun destroyHandle() {
    bitmap!!.recycle()
    bitmap = null

    /* Collect the bitmap storage if the bitmap is big.  */
    if (needCollect)
      Runtime.getRuntime().gc()
  }
}
