// femacs: port of java/org/gnu/emacs/EmacsFontDriver.java @ fd7afced403.

package org.gnu.emacs

import android.os.Build

/* This code is mostly unused.  See sfntfont-android.c for the code
   that is actually used.  */

/* `androidfont.c` finds this class and its three nested records by name
   (`org/gnu/emacs/EmacsFontDriver` at 180, `…$FontSpec` at 237, `…$FontMetrics` at 266,
   `…$FontObject` at 372), calls the eight abstract entry points through `GetMethodID` on this
   class (199-213), calls the static `createFontDriver` through `GetStaticMethodID` (215-220),
   and reads the records' fields through `GetFieldID` (247-257 with `Ljava/lang/String;` and
   `Ljava/lang/Integer;`, 286-290 with `S`, 385-399 with `I`).  Every name and descriptor is
   therefore the reference's own: the record fields are `@JvmField`, the boxed properties stay
   `Int?`/`String?` (Kotlin's `Int` would compile to the primitive descriptor `I`, not
   `Ljava/lang/Integer;`, and a missing property is null in the reference to begin with), and the
   eight entry points stay `fun`s (PORTING.md P-83 ②⑤).  `createFontDriver` is a companion member
   that C reaches on the class itself, so it carries `@JvmStatic` (P-83 ⑪).  Java's classes are
   non-final, so the nested records are `open` (Kotlin's default is final): `FontEntity` and
   `FontObject` extend `FontSpec` here, and `EmacsSdk7FontDriver` extends `FontEntity` and
   `FontObject` (P-83 ⑦).  */

abstract class EmacsFontDriver {
  companion object {
    /* Font weights.  */
    const val THIN = 0
    const val ULTRA_LIGHT = 40
    const val LIGHT = 50
    const val SEMI_LIGHT = 55
    const val REGULAR = 80
    const val MEDIUM = 100
    const val SEMI_BOLD = 180
    const val BOLD = 200
    const val EXTRA_BOLD = 205
    const val BLACK = 210
    const val ULTRA_HEAVY = 250

    /* Font slants.  */
    const val REVERSE_OBLIQUE = 0
    const val REVERSE_ITALIC = 10
    const val NORMAL = 100
    const val ITALIC = 200
    const val OBLIQUE = 210

    /* Font widths.  */
    const val ULTRA_CONDENSED = 50
    const val EXTRA_CONDENSED = 63
    const val CONDENSED = 75
    const val SEMI_CONDENSED = 87
    const val UNSPECIFIED = 100
    const val SEMI_EXPANDED = 113
    const val EXPANDED = 125
    const val EXTRA_EXPANDED = 150
    const val ULTRA_EXPANDED = 200

    /* Font spacings.  */
    const val PROPORTIONAL = 0
    const val DUAL = 90
    const val MONO = 100
    const val CHARCELL = 110

    /* Special glyph codes.  */
    /* The reference's `0xFFFFFFFF` is Java's int -1 (Kotlin reads that literal as a `Long`).  */
    const val FONT_INVALID_CODE = -0x1

    @JvmStatic fun createFontDriver(): EmacsFontDriver {
      return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M)
        EmacsSdk23FontDriver()
      else
        EmacsSdk7FontDriver()
    }
  }

  open class FontSpec {
    /* The fields below mean the same as they do in enum
       font_property_index in font.h.  */

    @JvmField var foundry: String? = null
    @JvmField var family: String? = null
    @JvmField var adstyle: String? = null
    @JvmField var registry: String? = null
    @JvmField var width: Int? = null
    @JvmField var weight: Int? = null
    @JvmField var slant: Int? = null
    @JvmField var size: Int? = null
    @JvmField var spacing: Int? = null
    @JvmField var avgwidth: Int? = null
    @JvmField var dpi: Int? = null

    override fun toString(): String {
      return "foundry: " + foundry +
        " family: " + family +
        " adstyle: " + adstyle +
        " registry: " + registry +
        " width: " + width +
        " weight: " + weight +
        " slant: " + slant +
        " spacing: " + spacing +
        " avgwidth: " + avgwidth +
        " dpi: " + dpi
    }
  }

  open class FontMetrics {
    @JvmField var lbearing: Short = 0
    @JvmField var rbearing: Short = 0
    @JvmField var width: Short = 0
    @JvmField var ascent: Short = 0
    @JvmField var descent: Short = 0

    override fun toString(): String {
      return "lbearing " + lbearing +
        " rbearing " + rbearing +
        " width " + width +
        " ascent " + ascent +
        " descent " + descent
    }
  }

  open class FontEntity : FontSpec() {
    /* No extra fields here.  */
  }

  /* Java's nested class is not static, so the port keeps it an `inner` class: an instance carries
     the driver it belongs to, exactly as the reference's does.  */
  abstract inner class FontObject : FontSpec() {
    @JvmField var minWidth: Int = 0
    @JvmField var maxWidth: Int = 0
    @JvmField var pixelSize: Int = 0
    @JvmField var height: Int = 0
    @JvmField var spaceWidth: Int = 0
    @JvmField var averageWidth: Int = 0
    @JvmField var ascent: Int = 0
    @JvmField var descent: Int = 0
    @JvmField var underlineThickness: Int = 0
    @JvmField var underlinePosition: Int = 0
    @JvmField var baselineOffset: Int = 0
    @JvmField var relativeCompose: Int = 0
    @JvmField var defaultAscent: Int = 0
    @JvmField var encodingCharset: Int = 0
    @JvmField var repertoryCharset: Int = 0

    init {
      encodingCharset = -1
      repertoryCharset = -1
    }
  }

  /* These mean the same as they do in struct font_driver.  */
  abstract fun list(fontSpec: FontSpec): Array<FontEntity>
  abstract fun match(fontSpec: FontSpec): FontEntity
  abstract fun listFamilies(): Array<String>
  abstract fun openFont(fontEntity: FontEntity, pixelSize: Int): FontObject
  abstract fun hasChar(font: FontSpec, charCode: Int): Int
  abstract fun textExtents(font: FontObject, code: IntArray, fontMetrics: FontMetrics)
  abstract fun encodeChar(fontObject: FontObject, charCode: Int): Int
  abstract fun draw(fontObject: FontObject, gc: EmacsGC, drawable: EmacsDrawable,
                    chars: IntArray, x: Int, y: Int, backgroundWidth: Int,
                    withBackground: Boolean): Int
}
