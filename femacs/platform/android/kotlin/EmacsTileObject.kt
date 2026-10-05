// femacs: port of java/org/gnu/emacs/EmacsTileObject.java @ fd7afced403.

package org.femacs

import android.graphics.Bitmap
import android.graphics.BitmapShader
import android.graphics.Canvas
import android.graphics.ColorFilter
import android.graphics.Paint
import android.graphics.Rect
import android.graphics.Shader.TileMode

/* This is a crude facsimile of the BitmapDrawable class implementing
   just enough of its functionality to support displaying stipples in
   EmacsGC.  */

/* Java's uninitialized fields are Kotlin's nullable ones; `paint` is `final` in the reference and
   is initialized where the reference initializes it (its constructor).  */
class EmacsTileObject(stippleBitmap: Bitmap?) {
  /* Bitmap object set by EmacsGC.  */
  private var bitmap: Bitmap? = stippleBitmap

  /* Tiling modes on either axis.  */
  private var xTile: TileMode? = null
  private var yTile: TileMode? = null

  /* Destination rectangle.  */
  private var boundsRect: Rect? = null

  /* Paint providing graphics properties for drawBitmap.  */
  private val paint: Paint = Paint()

  fun setBitmap(newBitmap: Bitmap?) {
    bitmap = newBitmap
  }

  fun setBounds(bounds: Rect) {
    boundsRect = bounds
  }

  fun setTileModeXY(newXTile: TileMode, newYTile: TileMode) {
    xTile = newXTile
    yTile = newYTile
  }

  fun setColorFilter(filterObject: ColorFilter?) {
    paint.colorFilter = filterObject
  }

  fun getBitmap(): Bitmap? {
    return bitmap
  }

  /* Replicate `bitmap' over CANVAS so that boundsRect is covered with
     copies thereof on the X axis, if xTile is REPEAT, and also on the Y
     axis, if yTile is a like value.  */

  fun draw(canvas: Canvas) {
    paint.shader = BitmapShader(bitmap!!, xTile!!, yTile!!)
    canvas.drawRect(boundsRect!!, paint)
  }
}
