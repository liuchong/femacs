// femacs: port of java/org/gnu/emacs/EmacsFillRectangle.java @ fd7afced403.

package org.gnu.emacs

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.ColorFilter
import android.graphics.ColorMatrixColorFilter
import android.graphics.Paint
import android.graphics.Rect
import android.util.Log

/* Java's `static { … }` initializer becomes the companion object's property initializer; the
   `float[]` literal becomes `floatArrayOf` (P-83 ①).  */
class EmacsFillRectangle {
  companion object {
    /* Color filter that inverts colors from the source.  */
    private val invertFilter: ColorFilter = ColorMatrixColorFilter(floatArrayOf(
      -1f, 0f, 0f, 0f, 255f,
      0f, -1f, 0f, 0f, 255f,
      0f, 0f, -1f, 0f, 255f,
      0f, 0f, 0f, 1f, 0f,
    ))

    fun perform(drawable: EmacsDrawable, gc: EmacsGC,
                x: Int, y: Int, width: Int, height: Int) {
      var paint: Paint
      var rect: Rect
      var canvas: Canvas?
      var invertBitmap: Bitmap

      canvas = drawable.lockCanvas(gc)

      /* Clip masks are not respected or implemented when specified with
         this request.  */
      if (canvas == null || gc.clip_mask != null)
        return

      rect = Rect(x, y, x + width, y + height)

      if (gc.function != EmacsGC.GC_INVERT) {
        paint = gc.gcPaint
        paint.style = Paint.Style.FILL

        if (gc.fill_style != EmacsGC.GC_FILL_OPAQUE_STIPPLED)
          canvas.drawRect(rect, paint)
        else
          gc.blitOpaqueStipple(canvas, rect)
      } else {
        paint = Paint()

        /* Simply invert the destination, which is only implemented for
           this request.  As Android doesn't permit copying a bitmap to
           itself, a copy of the source must be procured beforehand.  */
        invertBitmap = Bitmap.createBitmap(drawable.getBitmap()!!, x, y, width, height)
        paint.colorFilter = invertFilter
        canvas.drawBitmap(invertBitmap, null, rect, paint)
        paint.colorFilter = null
        invertBitmap.recycle()
      }

      drawable.damageRect(rect)
    }
  }
}
