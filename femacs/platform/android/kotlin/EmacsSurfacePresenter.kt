// femacs: modern presentation path for the Android frame (decision, status 4.937).
//
// The reference moves every frame through three hops: Emacs' bitmap ->
// `EmacsSurfaceView`'s front buffer -> `View.onDraw` -> the window.  That chain
// proved timing-sensitive on the emulator: the front buffer is fed correctly
// (a copy really carries black pixels, status 4.932) and `onDraw` paints it,
// yet the glass stayed white in most runs.  The user's standing decision allows a
// modern component for the UI layer, so this class presents the frame through a
// real `SurfaceView` and a single `unlockCanvasAndPost` hop instead.  The C-side
// semantics are untouched: `EmacsView.getCanvas` still hands Emacs the `Canvas`
// of `bitmap`, and `EmacsView.damageRect` still records the damage.
package org.femacs

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Paint
import android.view.SurfaceHolder
import android.view.SurfaceView

class EmacsSurfacePresenter(context: Context) : SurfaceView(context),
  SurfaceHolder.Callback {
  private val paint = Paint()

  init {
    holder.addCallback(this)
  }

  override fun surfaceCreated(holder: SurfaceHolder) {
  }

  override fun surfaceChanged(holder: SurfaceHolder, format: Int,
                              width: Int, height: Int) {
  }

  override fun surfaceDestroyed(holder: SurfaceHolder) {
  }

  /* Present BITMAP, or clear the surface when it is null.  Called from the
     Emacs thread, like the reference's `setBitmap`.  */
  fun present(bitmap: Bitmap?) {
    val holder = holder
    if (!holder.surface.isValid)
      return

    var canvas: Canvas? = null

    try {
      canvas = holder.lockCanvas()
      if (canvas == null)
        return

      if (bitmap == null) {
        canvas.drawColor(0xffffffff.toInt())
        return
      }

      canvas.drawColor(0xffffffff.toInt())
      canvas.drawBitmap(bitmap, 0f, 0f, paint)
    } finally {
      if (canvas != null)
        holder.unlockCanvasAndPost(canvas)
    }
  }
}
