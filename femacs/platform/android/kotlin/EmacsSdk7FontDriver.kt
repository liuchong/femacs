// femacs: port of java/org/gnu/emacs/EmacsSdk7FontDriver.java @ fd7afced403.

package org.femacs

import java.util.LinkedList

import android.graphics.Paint
import android.graphics.Rect
import android.graphics.Typeface
import android.graphics.Canvas

import android.util.DisplayMetrics

import kotlin.math.roundToInt

/* EmacsSdk7FontDriver implements a fallback font driver under
   Android.  This font driver is enabled when the SFNT font driver (in
   sfntfont-android.c) proves incapable of locating any fonts, which
   has hitherto not been observed in practice.

   This font driver does not supply each font installed on the system,
   in lieu of which it provides a list of fonts for each conceivable
   style and sub-type of the system's own Typefaces, which arises from
   Android's absence of suitable APIs for loading individual font
   files.  */

/* The reference imports `java.io.File`, `java.util.List` and `android.util.Log` without using
   them; Kotlin needs no import it does not use, so those three are not carried (the three class
   constants below are kept although the reference only reads `TOFU_STRING`: they are part of its
   declarations, and a private constant costs nothing).  Java's class is non-final, so the port is
   `open` - `EmacsSdk23FontDriver` extends it (P-83 ⑦) - and `Sdk7FontObject` is an `inner` class
   because the reference's `Sdk7Typeface`/`Sdk7FontEntity` are `static` while it is not (the
   `FontObject` it extends is `inner` too).  `Math.round (float)` is `kotlin.math.roundToInt`, the
   same call under the token (P-83 ①).  `scaledDensity` carries the reference's own
   `@SuppressWarnings ("deprecation")` as Kotlin's `@Suppress ("DEPRECATION")`.  */

open class EmacsSdk7FontDriver : EmacsFontDriver() {
  companion object {
    private const val TOFU_STRING = "\uDB3F\uDFFD"
    private const val EM_STRING = "m"
    private const val TAG = "EmacsSdk7FontDriver"
  }

  protected class Sdk7Typeface(val familyName: String, val typeface: Typeface) {
    /* The typeface and paint.  */
    val typefacePaint: Paint = Paint()
    var slant: Int = 0
    var width: Int = 0
    var weight: Int = 0
    var spacing: Int = 0

    init {
      /* Initialize the font style fields and create a paint object
         linked with that typeface.  */

      slant = NORMAL
      weight = REGULAR
      width = UNSPECIFIED
      spacing = PROPORTIONAL

      typefacePaint.isAntiAlias = true
      typefacePaint.typeface = typeface
    }

    override fun toString(): String {
      return "Sdk7Typeface (" +
        familyName + ", " +
        slant + ", " +
        width + ", " +
        weight + ", " +
        spacing + ")"
    }
  }

  @Suppress("DEPRECATION")
  protected class Sdk7FontEntity(val typeface: Sdk7Typeface) : FontEntity() {
    init {
      val metrics: DisplayMetrics

      foundry = "Google"
      family = typeface.familyName
      adstyle = null
      weight = typeface.weight
      slant = typeface.slant
      spacing = typeface.spacing
      width = typeface.width
      metrics = EmacsService.SERVICE!!.getResources().getDisplayMetrics()
      dpi = Math.round(metrics.scaledDensity * 160f)

      /* C: `this.typeface = typeface;` - the property above is that assignment, made where the
         reference makes it.  */
    }
  }

  @Suppress("DEPRECATION")
  protected inner class Sdk7FontObject(val typeface: Sdk7Typeface, pixelSize: Int)
    : FontObject() {
    init {
      var totalWidth: Float
      var testString: String
      val metrics: DisplayMetrics

      this.pixelSize = pixelSize

      family = typeface.familyName
      adstyle = null
      weight = typeface.weight
      slant = typeface.slant
      spacing = typeface.spacing
      width = typeface.width
      metrics = EmacsService.SERVICE!!.getResources().getDisplayMetrics()
      dpi = Math.round(metrics.scaledDensity * 160f)

      /* Compute the ascent and descent.  */
      typeface.typefacePaint.textSize = pixelSize.toFloat()
      ascent = (-typeface.typefacePaint.ascent()).roundToInt()
      descent = typeface.typefacePaint.descent().roundToInt()

      /* Compute the average width.  */
      testString = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
      totalWidth = typeface.typefacePaint.measureText(testString)

      if (totalWidth > 0)
        avgwidth = (totalWidth / testString.length).roundToInt()

      /* Android doesn't expose the font average width and height
         information, so this will have to do.  */
      /* C: `minWidth = maxWidth = avgwidth;` - the reference unboxes the boxed `avgwidth` here,
         which is null unless the measurement above ran (P-83 ⑤).  */
      maxWidth = avgwidth!!
      minWidth = maxWidth

      /* This is different from avgwidth in the font spec! */
      averageWidth = avgwidth!!

      /* Set the space width.  */
      totalWidth = typeface.typefacePaint.measureText(" ")
      spaceWidth = totalWidth.roundToInt()

      /* Set the height and default ascent.  */
      height = ascent + descent
      defaultAscent = ascent
    }
  }

  private var fontFamilyList: Array<String>? = null
  private var typefaceList: Array<Sdk7Typeface?>? = null
  private var fallbackTypeface: Sdk7Typeface? = null

  init {
    var typeface: Typeface

    typefaceList = arrayOfNulls<Sdk7Typeface>(5)

    /* Initialize the default monospace and Sans Serif typefaces.
       Initialize the same typeface with various distinct styles.  */
    fallbackTypeface = Sdk7Typeface("Sans Serif", Typeface.DEFAULT)
    typefaceList!![1] = fallbackTypeface

    fallbackTypeface = Sdk7Typeface("Sans Serif",
                                    Typeface.create(Typeface.DEFAULT,
                                                    Typeface.BOLD))
    fallbackTypeface!!.weight = BOLD
    typefaceList!![2] = fallbackTypeface

    fallbackTypeface = Sdk7Typeface("Sans Serif",
                                    Typeface.create(Typeface.DEFAULT,
                                                    Typeface.ITALIC))
    fallbackTypeface!!.slant = ITALIC
    typefaceList!![3] = fallbackTypeface

    fallbackTypeface = Sdk7Typeface("Sans Serif",
                                    Typeface.create(Typeface.DEFAULT,
                                                    Typeface.BOLD_ITALIC))
    fallbackTypeface!!.weight = BOLD
    fallbackTypeface!!.slant = ITALIC
    typefaceList!![4] = fallbackTypeface

    fallbackTypeface = Sdk7Typeface("Monospace", Typeface.MONOSPACE)
    fallbackTypeface!!.spacing = MONO
    typefaceList!![0] = fallbackTypeface

    fontFamilyList = arrayOf("Monospace", "Sans Serif")
  }

  private fun checkMatch(typeface: Sdk7Typeface, fontSpec: FontSpec): Boolean {
    if (fontSpec.family != null
        && fontSpec.family != typeface.familyName)
      return false

    /* C: `fontSpec.slant != null && !fontSpec.weight.equals (typeface.weight)` - the reference
       guards `slant` but dereferences `weight` (a copy-paste slip of its own); the port keeps
       both, so the `!!` is that dereference (P-83 ⑤).  */
    if (fontSpec.slant != null
        && fontSpec.weight!! != typeface.weight)
      return false

    if (fontSpec.spacing != null
        && fontSpec.spacing != typeface.spacing)
      return false

    if (fontSpec.weight != null
        && fontSpec.weight != typeface.weight)
      return false

    if (fontSpec.width != null
        && fontSpec.width != typeface.width)
      return false

    return true
  }

  override fun list(fontSpec: FontSpec): Array<FontEntity> {
    val list: LinkedList<FontEntity> = LinkedList<FontEntity>()
    var i: Int

    for (i in 0 until typefaceList!!.size) {
      if (checkMatch(typefaceList!![i]!!, fontSpec))
        list.add(Sdk7FontEntity(typefaceList!![i]!!))
    }

    return list.toTypedArray()
  }

  override fun match(fontSpec: FontSpec): FontEntity {
    val entities: Array<FontEntity>
    var i: Int

    entities = this.list(fontSpec)

    if (entities.size == 0)
      return Sdk7FontEntity(fallbackTypeface!!)

    return entities[0]
  }

  override fun listFamilies(): Array<String> {
    return fontFamilyList!!
  }

  override fun openFont(fontEntity: FontEntity, pixelSize: Int): FontObject {
    return Sdk7FontObject((fontEntity as Sdk7FontEntity).typeface,
                          pixelSize)
  }

  override fun hasChar(font: FontSpec, charCode: Int): Int {
    var missingGlyphWidth: Float
    var width: Float
    val rect1: Rect
    val rect2: Rect
    val paint: Paint

    /* Ignore characters outside the BMP.  */

    if (charCode > 65535)
      return 0

    if (font is Sdk7FontObject) {
      paint = font.typeface.typefacePaint
    } else
      paint = (font as Sdk7FontEntity).typeface.typefacePaint

    paint.textSize = 10f

    if (Character.isWhitespace(charCode.toChar()))
      return 1

    missingGlyphWidth = paint.measureText(TOFU_STRING)
    width = paint.measureText("" + charCode)

    if (width == 0f)
      return 0

    if (width != missingGlyphWidth)
      return 1

    rect1 = Rect()
    rect2 = Rect()

    paint.getTextBounds(TOFU_STRING, 0, TOFU_STRING.length,
                        rect1)
    paint.getTextBounds("" + charCode.toChar(), 0, 1, rect2)
    return if (rect1 == rect2) 0 else 1
  }

  private fun textExtents1(font: Sdk7FontObject, code: Int, metrics: FontMetrics,
                           paint: Paint, bounds: Rect) {
    val text: CharArray

    text = CharArray(1)
    text[0] = code.toChar()

    paint.getTextBounds(text, 0, 1, bounds)

    /* bounds is the bounding box of the glyph corresponding to CODE.
       Translate these into XCharStruct values.

       The origin is at 0, 0, and lbearing is the distance counting
       rightwards from the origin to the left most pixel in the glyph
       raster.  rbearing is the distance between the origin and the
       rightmost pixel in the glyph raster.  ascent is the distance
       counting upwards between the topmost pixel in the glyph
       raster.  descent is the distance (once again counting
       downwards) between the origin and the bottommost pixel in the
       glyph raster.

       width is the distance between the origin and the origin of any
       character to the right.  */

    metrics.lbearing = bounds.left.toShort()
    metrics.rbearing = bounds.right.toShort()
    metrics.ascent = (-bounds.top).toShort()
    metrics.descent = bounds.bottom.toShort()
    /* C: `(short) paint.measureText (…)` - the reference's narrowing cast is `toInt` then
       `toShort` in Kotlin (a direct `Float.toShort` is deprecated as ambiguous).  */
    metrics.width = paint.measureText("" + text[0]).toInt().toShort()
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
      text = CharArray(code.size)

      for (i in 0 until code.size)
        text[i] = code[i].toChar()

      paintCache.getTextBounds(text, 0, code.size,
                               boundsCache)
      width = paintCache.measureText(text, 0, code.size)

      fontMetrics.lbearing = boundsCache.left.toShort()
      fontMetrics.rbearing = boundsCache.right.toShort()
      fontMetrics.ascent = (-boundsCache.top).toShort()
      fontMetrics.descent = boundsCache.bottom.toShort()
      fontMetrics.width = width.roundToInt().toShort()
    }
  }

  override fun encodeChar(fontObject: FontObject, charCode: Int): Int {
    if (charCode > 65535)
      return FONT_INVALID_CODE

    return charCode
  }

  override fun draw(fontObject: FontObject, gc: EmacsGC, drawable: EmacsDrawable,
                    chars: IntArray, x: Int, y: Int, backgroundWidth: Int,
                    withBackground: Boolean): Int {
    val backgroundRect: Rect
    val bounds: Rect
    var sdk7FontObject: Sdk7FontObject
    var i: Int
    var canvas: Canvas?
    val paint: Paint
    val array: CharArray

    sdk7FontObject = fontObject as Sdk7FontObject

    backgroundRect = Rect()
    backgroundRect.top = y - sdk7FontObject.ascent
    backgroundRect.left = x
    backgroundRect.right = x + backgroundWidth
    backgroundRect.bottom = y + sdk7FontObject.descent

    canvas = drawable.lockCanvas(gc)

    if (canvas == null)
      return 0

    paint = gc.gcPaint
    paint.style = Paint.Style.FILL

    /* Java reassigns the `x` parameter inside the loop below; Kotlin's parameters are immutable,
       so the port keeps the running origin in a local (the shape `EmacsDrawLine.polyDashPattern`
       uses).  */
    var positionX = x

    if (withBackground) {
      paint.color = gc.background or 0xff000000.toInt()
      canvas.drawRect(backgroundRect, paint)
      paint.color = gc.foreground or 0xff000000.toInt()
    }

    paint.textSize = sdk7FontObject.pixelSize.toFloat()
    paint.typeface = sdk7FontObject.typeface.typeface
    paint.isAntiAlias = true

    /* Android applies kerning to non-monospaced fonts by default,
       which brings the dimensions of strings drawn via `drawText' out
       of agreement with measurements previously provided to redisplay
       by textExtents.  To avert such disaster, draw each character
       individually, advancing the origin point by hand.  */

    bounds = Rect()
    array = CharArray(1)

    for (i in 0 until chars.size) {
      /* Retrieve the text bounds for this character so as to
         compute the damage rectangle.  */
      array[0] = chars[i].toChar()
      paint.getTextBounds(array, 0, 1, bounds)
      bounds.offset(positionX, y)
      backgroundRect.union(bounds)

      /* Draw this character.  */
      canvas.drawText(array, 0, 1, positionX.toFloat(), y.toFloat(), paint)

      /* Advance the origin point by that much.  C: `x += paint.measureText (…);` - Java
         narrows the sum back into the int parameter, so the port writes that narrowing.  */
      positionX = (positionX + paint.measureText("" + array[0])).toInt()
    }

    drawable.damageRect(backgroundRect)
    paint.isAntiAlias = false
    return 1
  }
}
