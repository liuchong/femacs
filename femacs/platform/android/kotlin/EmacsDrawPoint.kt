// femacs: port of java/org/gnu/emacs/EmacsDrawPoint.java @ fd7afced403.

package org.femacs

class EmacsDrawPoint {
  companion object {
    fun perform(drawable: EmacsDrawable, immutableGC: EmacsGC, x: Int, y: Int) {
      /* Use EmacsFillRectangle instead of EmacsDrawRectangle, as the
         latter actually draws a rectangle one pixel wider than
         specified.  */
      EmacsFillRectangle.perform(drawable, immutableGC, x, y, 1, 1)
    }
  }
}
