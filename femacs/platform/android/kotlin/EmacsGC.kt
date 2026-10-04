// femacs: port of java/org/gnu/emacs/EmacsGC.java @ fd7afced403.

package org.gnu.emacs

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.ColorFilter
import android.graphics.Paint
import android.graphics.PorterDuff.Mode
import android.graphics.PorterDuffColorFilter
import android.graphics.Rect
import android.graphics.Shader.TileMode
import android.os.Build

/* X like graphics context structures.  Keep the enums in synch with
   androidgui.h! */

/* The fields `android.c` reads through the JNI keep their names and descriptors
   (`"foreground"`/`"background"`/`"function"`/`"fill_style"` `I` at android.c 3306-3341,
   `"clip_rects"` an array, `"stipple"` an object), so they are `@JvmField` and Java's
   uninitialized values are Kotlin's `0`/null (P-83 ②⑤).  `dashes` and the clip-rect arrays are
   `IntArray?`/`Array<Rect>?` for the same reason: the reference leaves them null until the C side
   sets them.  */
class EmacsGC : EmacsHandleObject() {
  @JvmField var function: Int = 0
  @JvmField var fill_style: Int = 0
  @JvmField var foreground: Int = 0
  @JvmField var background: Int = 0
  @JvmField var clip_x_origin: Int = 0
  @JvmField var clip_y_origin: Int = 0
  @JvmField var ts_origin_x: Int = 0
  @JvmField var ts_origin_y: Int = 0
  @JvmField var line_style: Int = 0
  @JvmField var line_width: Int = 0
  @JvmField var dashes: IntArray? = null
  @JvmField var dash_offset: Int = 0
  @JvmField var clip_rects: Array<Rect?>? = null
  @JvmField var real_clip_rects: Array<Rect?>? = null
  @JvmField var clip_mask: EmacsPixmap? = null
  @JvmField var stipple: EmacsPixmap? = null
  @JvmField var gcPaint: Paint = Paint()

  /* Drawable object for rendering the stipple bitmap.  */
  @JvmField var tileObject: EmacsTileObject? = null

  /* The value of clipRectID after the last time this GCs clip
     rectangles changed.  0 if there are no clip rectangles.  */
  @JvmField var clipRectID: Long = 0

  init {
    /* For historical reasons the C code has an extra layer of
       indirection above this GC handle.  struct android_gc is the GC
       used by Emacs code, while android_gcontext is the type of the
       handle.  */
    fill_style = GC_FILL_SOLID
    function = GC_COPY
    foreground = 0
    background = 0xffffff

    /* Android S and above enable anti-aliasing unless explicitly told
       otherwise.  */
    gcPaint.isAntiAlias = false
  }

  /* Mark this GC as dirty.  Apply parameters to the paint and
     recompute real_clip_rects.  */

  fun markDirty(clipRectsChanged: Boolean) {
    var i: Int
    var stippleBitmap: Bitmap?

    if (clipRectsChanged) {
      if ((ts_origin_x != 0 || ts_origin_y != 0) && clip_rects != null) {
        val rects: Array<Rect?> = arrayOfNulls<Rect>(clip_rects!!.size)
        real_clip_rects = rects

        for (i in clip_rects!!.indices) {
          rects[i] = Rect(clip_rects!![i])
          rects[i]!!.offset(ts_origin_x, ts_origin_y)
        }
      } else
        real_clip_rects = clip_rects

      clipRectID = ++clip_serial
    }

    /* A line_width of 0 is equivalent to that of 1.  */
    gcPaint.strokeWidth = if (line_width < 1) 1f else line_width.toFloat()
    gcPaint.color = foreground or 0xff000000.toInt()

    /* Update the stipple object with the new stipple bitmap, or delete
       it if the stipple has been cleared on systems too old to support
       modifying such objects.  */

    if (stipple != null) {
      stippleBitmap = stipple!!.getBitmap()

      /* Allocate a new tile object if none is already present or it
         cannot be reconfigured.  */
      if (tileObject == null) {
        tileObject = EmacsTileObject(stippleBitmap)
        tileObject!!.setTileModeXY(TileMode.REPEAT, TileMode.REPEAT)
      } else
        /* Otherwise, update the existing tile object with the new
           bitmap.  */
        tileObject!!.setBitmap(stippleBitmap)
    } else if (tileObject != null)
      tileObject!!.setBitmap(null)
  }

  /* Prepare the tile object to draw a stippled image onto a section of
     a drawable defined by RECT.  It is an error to call this function
     unless the `stipple' field of the GContext is set.  */

  private fun prepareStipple(rect: Rect) {
    val sx: Int /* Stipple origin.  */
    val sy: Int /* Stipple origin.  */
    val bw: Int /* Stipple size.  */
    val bh: Int /* Stipple size.  */
    val bitmap: Bitmap?
    val boundsRect: Rect

    /* Retrieve the dimensions of the stipple bitmap, which doubles as
       the unit of advance for this stipple.  */
    bitmap = tileObject!!.getBitmap()
    bw = bitmap!!.width
    bh = bitmap.height

    /* Align the lower left corner of the bounds rectangle to the
       initial position of the stipple.  */
    sx = (rect.left % bw) * -1 + (-ts_origin_x % bw) * -1
    sy = (rect.top % bh) * -1 + (-ts_origin_y % bh) * -1
    boundsRect = Rect(rect.left + sx, rect.top + sy, rect.right, rect.bottom)
    tileObject!!.setBounds(boundsRect)
  }

  /* Fill the rectangle BOUNDS in the provided CANVAS with the stipple
     pattern defined for this GContext, in the foreground color where
     the pattern is on, and in the background color where off.  */

  protected fun blitOpaqueStipple(canvas: Canvas, rect: Rect) {
    var filter: ColorFilter

    prepareStipple(rect)
    filter = PorterDuffColorFilter(foreground or 0xff000000.toInt(), Mode.SRC_IN)
    tileObject!!.setColorFilter(filter)

    canvas.save()
    canvas.clipRect(rect)

    tileObject!!.draw(canvas)
    filter = PorterDuffColorFilter(background or 0xff000000.toInt(), Mode.SRC_OUT)
    tileObject!!.setColorFilter(filter)
    tileObject!!.draw(canvas)
    canvas.restore()
  }

  companion object {
    const val GC_COPY: Int = 0
    const val GC_INVERT: Int = 1

    const val GC_FILL_SOLID: Int = 0
    const val GC_FILL_OPAQUE_STIPPLED: Int = 1

    const val GC_LINE_SOLID: Int = 0
    const val GC_LINE_ON_OFF_DASH: Int = 1

    /* ID incremented every time the clipping rectangles of any GC
       changes.  */
    private var clip_serial: Long = 0
  }
}
