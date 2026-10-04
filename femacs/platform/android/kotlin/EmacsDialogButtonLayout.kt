// femacs: port of java/org/gnu/emacs/EmacsDialogButtonLayout.java @ fd7afced403.

package org.gnu.emacs

import android.content.Context
import android.view.View
import android.view.View.MeasureSpec
import android.view.ViewGroup
import kotlin.math.max

/* This ``view group'' implements a container widget for multiple
   buttons of the type found in pop-up dialogs.  It is used when
   displaying a dialog box that contains more than three buttons, as
   the default dialog box widget is not capable of holding more than
   that many.  */

/* The Java declares `public EmacsDialogButtonLayout (Context)` and only `EmacsDialog` (same package)
   constructs it, so the port's primary constructor keeps the signature and the class keeps its
   default finality (PORTING.md P-83 ①).  `x = y = height = 0` has no Kotlin form - assignment is a
   statement, not an expression - so the three bindings are written out; `Math.max` becomes
   `kotlin.math.max`, the same two-argument call under the token C's line has.  */
class EmacsDialogButtonLayout(context: Context) : ViewGroup(context) {
  override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
    var width: Int
    var count: Int
    var i: Int
    var x: Int
    var y: Int
    var height: Int
    var spec: Int
    var tempSpec: Int
    var view: View

    /* Obtain the width of this widget and create the measure
       specification used to measure children.  */

    width = MeasureSpec.getSize(widthMeasureSpec)
    spec = MeasureSpec.makeMeasureSpec(0, MeasureSpec.UNSPECIFIED)
    tempSpec = MeasureSpec.makeMeasureSpec(width, MeasureSpec.AT_MOST)
    x = 0
    y = 0
    height = 0

    /* Run through each widget.  */

    count = childCount

    for (i in 0 until count) {
      view = getChildAt(i)

      /* Measure this view.  */
      view.measure(spec, spec)

      if (width - x < view.measuredWidth) {
        /* Move onto the next line, unless this line is empty.  */

        if (x != 0) {
          y += height
          height = 0
          x = 0
        }

        if (view.measuredWidth > width)
          /* Measure the view again, this time forcing it to be at
             most width wide, if it is not already.  */
          view.measure(tempSpec, spec)
      }

      height = max(height, view.measuredHeight)
      x += view.measuredWidth
    }

    /* Now set the measured size of this widget.  */
    setMeasuredDimension(width, y + height)
  }

  override fun onLayout(changed: Boolean, left: Int, top: Int, right: Int, bottom: Int) {
    var width: Int
    var count: Int
    var i: Int
    var x: Int
    var y: Int
    var height: Int
    var spec: Int
    var tempSpec: Int
    var view: View

    /* Obtain the width of this widget and create the measure
       specification used to measure children.  */

    width = measuredWidth
    spec = MeasureSpec.makeMeasureSpec(0, MeasureSpec.UNSPECIFIED)
    tempSpec = MeasureSpec.makeMeasureSpec(width, MeasureSpec.AT_MOST)
    x = 0
    y = 0
    height = 0

    /* Run through each widget.  */

    count = childCount

    for (i in 0 until count) {
      view = getChildAt(i)

      /* Measure this view.  */
      view.measure(spec, spec)

      if (width - x < view.measuredWidth) {
        /* Move onto the next line, unless this line is empty.  */

        if (x != 0) {
          y += height
          height = 0
          x = 0
        }

        if (view.measuredWidth > width)
          /* Measure the view again, this time forcing it to be at
             most width wide, if it is not already.  */
          view.measure(tempSpec, spec)
      }

      /* Now assign this view its position.  */
      view.layout(x, y, x + view.measuredWidth, y + view.measuredHeight)

      /* And move on to the next widget.  */
      height = max(height, view.measuredHeight)
      x += view.measuredWidth
    }
  }
}
