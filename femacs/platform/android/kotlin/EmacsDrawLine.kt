// femacs: port of java/org/gnu/emacs/EmacsDrawLine.java @ fd7afced403.

package org.gnu.emacs

import android.graphics.Canvas
import android.graphics.Paint
import android.graphics.Rect
import kotlin.math.abs
import kotlin.math.hypot
import kotlin.math.max
import kotlin.math.min

/* `Math.hypot`/`Math.abs`/`Math.min`/`Math.max` are `kotlin.math`, the same calls under the tokens
   (P-83 ①); `gc.dashes` is nullable here, so it is asserted where the reference dereferences it
   (P-83 ⑤).  */
class EmacsDrawLine {
  companion object {
    /* Return the normalized slope and magnitude of a line whose extrema
       are DX and DY removed, on the X and Y axes respectively, from its
       origin point.  */

    private fun measureLine(dx: Float, dy: Float): FloatArray {
      val hypot: Float

      if (dx == 0f && dy == 0f)
        return floatArrayOf(0f, 0f, 0f)

      if (dx == 0f)
        return floatArrayOf(0f, if (dy > 0f) 1f else -1f, abs(dy))
      else if (dy == 0f)
        return floatArrayOf(if (dx > 0f) 1f else -1f, 0f, abs(dx))
      else {
        hypot = hypot(dx, dy)
        return floatArrayOf(dx / hypot, dy / hypot, hypot)
      }
    }

    private fun polyDashPattern(gc: EmacsGC, canvas: Canvas, paint: Paint, x0: Float,
                                y0: Float, x1: Float, y1: Float) {
      var patternTotal: Int
      var i: Int
      var offset: Int
      var dx: Float
      var dy: Float
      var mag: Float
      var dash_mag: Float
      var rem: Float
      var lx1: Float
      var ly1: Float
      val measured: FloatArray
      var which: Boolean

      /* Compute the total length of this pattern.  */
      patternTotal = 0
      for (i in gc.dashes!!.indices)
        patternTotal += gc.dashes!![i]
      if ((gc.dashes!!.size and 1) != 0)
        patternTotal += patternTotal

      /* Subtract as much of the offset as does not contribute to the
         phase at the first pixel of the line.  */
      offset = gc.dash_offset % patternTotal

      /* Set I to the first dash that ought to be drawn and WHICH to its
         phase.  */
      i = 0
      which = true
      while (offset >= gc.dashes!![i]) {
        offset -= gc.dashes!![i++]
        if (i >= gc.dashes!!.size)
          i = 0
        which = !which
      }

      /* Compute the length of the first visible segment.  */
      dash_mag = (gc.dashes!![i] - offset).toFloat()

      /* Compute the slope of the line.  */
      dx = x1 - x0
      dy = y1 - y0
      measured = measureLine(dx, dy)
      dx = measured[0]
      dy = measured[1]
      rem = measured[2]
      mag = measured[2]
      lx1 = x0
      ly1 = y0

      /* The Java reassigns `x1`/`y1` inside the loop; a Kotlin parameter is immutable, so the
         two are re-bound to locals here (P-83 ① - the same rendering as `projection` in
         EmacsDocumentsProvider).  */
      var px1: Float = x1
      var py1: Float = y1

      while (rem > 0f) {
        dash_mag = min(dash_mag, rem)
        rem -= dash_mag

        /* End of this segment.  */
        px1 = (mag - rem) * dx + x0
        py1 = (mag - rem) * dy + y0

        if (which)
          canvas.drawLine(lx1, ly1, px1, py1, paint)
        which = !which

        /* Start of the next segment.  */
        lx1 = px1
        ly1 = py1
        i++
        if (i >= gc.dashes!!.size)
          i = 0
        dash_mag = gc.dashes!![i].toFloat()
      }
    }

    fun perform(drawable: EmacsDrawable, gc: EmacsGC,
                x: Int, y: Int, x2: Int, y2: Int) {
      val canvas: Canvas?
      val paint: Paint
      val x0: Int
      val x1: Int
      val y0: Int
      val y1: Int

      /* TODO implement stippling.  */
      if (gc.fill_style == EmacsGC.GC_FILL_OPAQUE_STIPPLED)
        return

      /* Calculate the leftmost and rightmost points.  */

      x0 = min(x, x2 + 1)
      x1 = max(x, x2 + 1)
      y0 = min(y, y2 + 1)
      y1 = max(y, y2 + 1)

      /* And the clip rectangle.  */

      paint = gc.gcPaint
      canvas = drawable.lockCanvas(gc)

      if (canvas == null)
        return

      if (gc.clip_mask == null) {
        if (gc.line_style != EmacsGC.GC_LINE_ON_OFF_DASH)
          canvas.drawLine(x.toFloat(), y.toFloat(), x2.toFloat(), y2.toFloat(), paint)
        else
          polyDashPattern(gc, canvas, paint, x.toFloat(), y.toFloat(),
                          x2.toFloat(), y2.toFloat())
      }

      /* DrawLine with clip mask not implemented; it is not used by
         Emacs.  */
      drawable.damageRect(x0, y0, x1, y1)
    }
  }
}
