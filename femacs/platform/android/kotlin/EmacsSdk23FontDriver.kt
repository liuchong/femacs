// femacs: port of java/org/gnu/emacs/EmacsSdk23FontDriver.java @ fd7afced403.

package org.gnu.emacs

import android.graphics.Paint
import android.graphics.Rect

/* Java's class is `final`, so the port is a plain `class` (Kotlin's default) and its two
   overrides need no explicit openness (P-83 ⑦⑫).  Its private `textExtents1` shares its name
   with the private `textExtents1` of `EmacsSdk7FontDriver`, exactly as the reference's does: a
   private member is not inherited, so the two are independent (measured with the probe).  */

class EmacsSdk23FontDriver : EmacsSdk7FontDriver() {
  private fun textExtents1(font: Sdk7FontObject, code: Int, metrics: FontMetrics,
                           paint: Paint, bounds: Rect) {
    val text: CharArray

    text = CharArray(2)
    text[0] = code.toChar()
    text[1] = 'c'

    paint.getTextBounds(text, 0, 1, bounds)

    metrics.lbearing = bounds.left.toShort()
    metrics.rbearing = bounds.right.toShort()
    metrics.ascent = (-bounds.top).toShort()
    metrics.descent = bounds.bottom.toShort()
    /* C: `(short) paint.getRunAdvance (…)` - Kotlin spells the narrowing cast as `toInt` then
       `toShort`.  */
    metrics.width = paint.getRunAdvance(text, 0, 1, 0, 1, false, 1).toInt().toShort()
  }

  override fun textExtents(font: FontObject, code: IntArray, fontMetrics: FontMetrics) {
    var i: Int
    val paintCache: Paint
    val boundsCache: Rect
    val fontObject: Sdk7FontObject
    var text: CharArray
    var width: Float

    fontObject = font as Sdk7FontObject
    paintCache = fontObject.typeface.typefacePaint
    paintCache.textSize = fontObject.pixelSize.toFloat()
    boundsCache = Rect()

    if (code.size == 0) {
      fontMetrics.lbearing = 0
      fontMetrics.rbearing = 0
      fontMetrics.ascent = 0
      fontMetrics.descent = 0
      fontMetrics.width = 0
    } else if (code.size == 1)
      /* C: `textExtents1 ((Sdk7FontObject) font, …)` - the cast one line above has already put
         the type in Kotlin's smart cast, so it is not repeated (Java needs it, Kotlin does
         not).  */
      textExtents1(font, code[0], fontMetrics,
                   paintCache, boundsCache)
    else {
      text = CharArray(code.size + 1)

      for (i in 0 until code.size)
        text[i] = code[i].toChar()

      text[code.size] = 'c'

      paintCache.getTextBounds(text, 0, code.size,
                               boundsCache)
      width = paintCache.getRunAdvance(text, 0, code.size, 0,
                                       code.size,
                                       false, code.size)

      fontMetrics.lbearing = boundsCache.left.toShort()
      fontMetrics.rbearing = boundsCache.right.toShort()
      fontMetrics.ascent = (-boundsCache.top).toShort()
      fontMetrics.descent = boundsCache.bottom.toShort()
      fontMetrics.width = width.toInt().toShort()
    }
  }

  override fun hasChar(font: FontSpec, charCode: Int): Int {
    val fontObject: Sdk7FontObject
    val paint: Paint

    if (font is Sdk7FontObject) {
      fontObject = font
      paint = fontObject.typeface.typefacePaint
    } else
      paint = (font as Sdk7FontEntity).typeface.typefacePaint

    /* If the character falls within the confines of the BMP, return
       1.  */
    if (charCode < 65536)
      return if (paint.hasGlyph(charCode.toChar().toString())) 1 else 0

    /* Otherwise return 0.  */
    return 0
  }
}
