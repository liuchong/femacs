// femacs: port of java/org/gnu/emacs/EmacsDrawRectangle.java @ fd7afced403.

package org.gnu.emacs

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Paint
import android.graphics.PorterDuff.Mode
import android.graphics.PorterDuffXfermode
import android.graphics.Rect
import android.graphics.RectF
import android.graphics.Xfermode
import android.util.Log

/* The `static final Xfermode` is a companion property (P-83 ①).  `gc.clip_mask.bitmap` is nullable
   in the port, so the dereferences are asserted where the reference dereferences them (P-83 ⑤).  */
class EmacsDrawRectangle {
  companion object {
    private val srcInAlu: Xfermode = PorterDuffXfermode(Mode.SRC_IN)

    fun perform(drawable: EmacsDrawable, gc: EmacsGC,
                x: Int, y: Int, width: Int, height: Int) {
      val maskPaint: Paint
      val paint: Paint
      val maskCanvas: Canvas
      val maskBitmap: Bitmap
      val maskRect: Rect
      val dstRect: Rect
      val canvas: Canvas?
      val clipBitmap: Bitmap

      /* TODO implement stippling for this request.  */
      if (gc.fill_style == EmacsGC.GC_FILL_OPAQUE_STIPPLED
          /* And GC_INVERT also.  */
          || gc.fill_style == EmacsGC.GC_INVERT)
        return

      canvas = drawable.lockCanvas(gc)

      if (canvas == null)
        return

      paint = gc.gcPaint
      paint.style = Paint.Style.STROKE

      /* This graphics request, in contrast to X, does not presently
         respect the GC's line style.  */

      if (gc.clip_mask == null)
        /* Use canvas.drawRect with a RectF.  That seems to reliably
           get PostScript behavior.  */
        canvas.drawRect(RectF(x + 0.5f, y + 0.5f,
                             x + width + 0.5f,
                             y + height + 0.5f),
                        paint)
      else {
        /* Drawing with a clip mask involves calculating the
           intersection of the clip mask with the dst rect, and
           extrapolating the corresponding part of the src rect.  */
        clipBitmap = gc.clip_mask!!.bitmap!!
        dstRect = Rect(x, y, x + width, y + height)
        maskRect = Rect(gc.clip_x_origin,
                        gc.clip_y_origin,
                        gc.clip_x_origin + clipBitmap.width,
                        gc.clip_y_origin + clipBitmap.height)

        if (!maskRect.setIntersect(dstRect, maskRect))
          /* There is no intersection between the clip mask and the
             dest rect.  */
          return

        /* Finally, create a temporary bitmap that is the size of
           maskRect.  */

        maskBitmap = Bitmap.createBitmap(maskRect.width(), maskRect.height(),
                                         Bitmap.Config.ARGB_8888)

        /* Draw the mask onto the maskBitmap.  */
        maskCanvas = Canvas(maskBitmap)
        maskRect.offset(-gc.clip_x_origin, -gc.clip_y_origin)
        maskCanvas.drawBitmap(clipBitmap,
                              maskRect,
                              Rect(0, 0, maskRect.width(), maskRect.height()),
                              paint)
        maskRect.offset(gc.clip_x_origin, gc.clip_y_origin)

        /* Set the transfer mode to SRC_IN to preserve only the parts
           of the source that overlap with the mask.  */
        maskPaint = Paint()
        maskPaint.xfermode = srcInAlu
        maskPaint.style = Paint.Style.STROKE

        /* Draw the source.  */
        maskCanvas.drawRect(maskRect, maskPaint)

        /* Finally, draw the mask bitmap to the destination.  */
        paint.xfermode = null
        canvas.drawBitmap(maskBitmap, null, maskRect, paint)

        /* Recycle this unused bitmap.  */
        maskBitmap.recycle()
      }

      drawable.damageRect(x, y, x + width + 1, y + height + 1)
    }
  }
}
