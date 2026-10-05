// femacs: port of java/org/gnu/emacs/EmacsFillPolygon.java @ fd7afced403.

package org.femacs

import android.graphics.Canvas
import android.graphics.Paint
import android.graphics.Path
import android.graphics.Point
import android.graphics.Rect
import android.graphics.RectF
import android.os.Build
import kotlin.math.ceil
import kotlin.math.floor

/* Java's static method is Kotlin's companion-object member; nothing here is looked up by JNI (the C
   side names `EmacsFillPolygon.class` only in its own comments), so no `@JvmStatic` is needed
   (PORTING.md P-83 ②).  `Math.floor`/`Math.ceil` become `kotlin.math`, the same two calls under the
   tokens.  */
class EmacsFillPolygon {
  companion object {
    fun perform(drawable: EmacsDrawable, gc: EmacsGC, points: Array<Point>) {
      var canvas: Canvas?
      var path: Path
      var paint: Paint
      var rect: Rect
      var rectF: RectF
      var i: Int

      canvas = drawable.lockCanvas(gc)

      if (canvas == null)
        return

      paint = gc.gcPaint

      /* Build the path from the given array of points.  */
      path = Path()

      if (points.size >= 1) {
        path.moveTo(points[0].x.toFloat(), points[0].y.toFloat())

        for (i in 1 until points.size)
          path.lineTo(points[i].x.toFloat(), points[i].y.toFloat())

        path.close()
      }

      /* Compute the damage rectangle.  */
      rectF = RectF(0f, 0f, 0f, 0f)

      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.BAKLAVA)
        path.computeBounds(rectF, true)
      else
        path.computeBounds(rectF)

      rect = Rect(floor(rectF.left).toInt(),
                  floor(rectF.top).toInt(),
                  ceil(rectF.right).toInt(),
                  ceil(rectF.bottom).toInt())

      paint.style = Paint.Style.FILL

      if (gc.clip_mask == null)
        canvas!!.drawPath(path, paint)

      drawable.damageRect(rect)

      /* FillPolygon with clip mask not implemented; it is not used by
         Emacs.  */
    }
  }
}
