// femacs: modern presentation path for the Android frame (decision, status 4.937/4.939).
//
// The reference moves every frame through three hops: Emacs' bitmap ->
// `EmacsSurfaceView`'s front buffer -> `View.onDraw` -> the window.  On the
// emulator that chain proved timing-sensitive: the copy carries black pixels
// (status 4.932) and `onDraw` paints them, yet the glass stayed white in most
// runs.  The user's standing decision allows a modern component for the UI layer,
// so this class keeps exactly one hop: it remembers the newest frame and paints
// it in `onDraw`, which the UI thread runs after `postInvalidate`.  The C-side
// semantics are untouched: `EmacsView.getCanvas` still hands Emacs the `Canvas`
// of `bitmap`, and `EmacsView.damageRect` still records the damage.
package org.femacs

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.view.View

class EmacsSurfacePresenter(context: Context) : View(context) {
  /* The newest frame handed over by the Emacs thread.  */
  private var lastBitmap: Bitmap? = null

  /* Present BITMAP, or clear when it is null.  Called from the Emacs thread.  */
  fun present(bitmap: Bitmap?) {
    if (bitmap != null)
      lastBitmap = bitmap

    /* Invalidate this view *and* its parent: the emulator stopped redrawing
       after the first frame even when this view asked for it (status 4.946).  */
    postInvalidate()
    (parent as? View)?.postInvalidate()
  }

  fun lastFrame(): Bitmap? = lastBitmap

  override fun onDraw(canvas: Canvas) {
    canvas.drawColor(0xffffffff.toInt())
    val bitmap = lastBitmap ?: return
    if (bitmap.isRecycled)
      return
    canvas.drawBitmap(bitmap, 0f, 0f, null)
  }
}
