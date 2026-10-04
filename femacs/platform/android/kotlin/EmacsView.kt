// femacs: port of java/org/gnu/emacs/EmacsView.java @ fd7afced403.

package org.gnu.emacs

import android.content.Context

import android.text.InputType

import android.view.ContextMenu
import android.view.ContextThemeWrapper
import android.view.DragEvent
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import android.view.ViewTreeObserver
import android.view.WindowInsets

import android.view.inputmethod.EditorInfo
import android.view.inputmethod.InputConnection
import android.view.inputmethod.InputMethodManager

import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Rect
import android.graphics.Region

import android.os.Build
import android.util.Log

import java.util.Arrays

/* This is an Android view which has a back and front buffer.  When
   swapBuffers is called, the back buffer is swapped to the front
   buffer, and any damage is invalidated.  A front buffer bitmap defined
   in EmacsSurfaceView, and the write buffer in this file, are modified
   and used both from the UI and the Emacs thread.  As a result, there
   is a lock held during all drawing operations.

   It is also a ViewGroup, so that it may also manage the layout of its
   children.  */

/* `android.c`, `androidmenu.c`, `androidselect.c`, `androidterm.c`, `androidfont.c`,
   `androidfns.c`, `androidvfs.c` and `android-emacs.c` name no member of this class: grepping them
   for `EmacsView` and for the `Lorg/gnu/emacs/EmacsView;` descriptor returns nothing, and the two
   `FIND_METHOD`s an `EmacsView` method could plausibly carry resolve on other classes - `getBitmap`
   `()Landroid/graphics/Bitmap;` (`android.c` 1926) is looked up on `EmacsDrawable`, which this
   class does not implement, and `swapBuffers` `()V` (`android.c` 1956) on `EmacsWindow`, which
   forwards to this view in Java.  So no member here has a JNI name obligation.  The reference's
   own remaining `.java` files do read some of these members as fields, though, so those are
   `@JvmField` to keep the field (not a getter/setter pair) - `window` (`EmacsActivity.java` 278,
   `EmacsInputConnection.java` 99), `imManager` and `icGeneration` (`EmacsService.java` 939-1024,
   including `icGeneration++`) and `mustReportLayout` (`EmacsWindow.java` 398) - and the methods
   those files call (`getCanvas`, `getBitmap`, `swapBuffers`, `damageRect`, `raise`/`lower`,
   `moveAbove`/`moveBelow`, `showOnScreenKeyboard`/`hideOnScreenKeyboard`, `getICMode`/`setICMode`,
   `explicitlyDirtyBitmap`) stay `fun` (P-83 ②).

   The reference's class is `final`, so the port is a plain `class` (P-83 ①).  Its two-argument
   constructor's `super (new ContextThemeWrapper (…))` moves to the class header (Kotlin's only
   place for a superclass call), and the constructor body becomes `init`; Java's block-local
   `Context context; Object tem;` keep their names, and `imManager` is the same hard cast (`as`).
   Every `synchronized` modifier becomes `@Synchronized` (P-83 ④): `explicitlyDirtyBitmap`,
   `getBitmap`, `getCanvas`, `onCheckIsTextEditor`, `onDetachedFromWindow`; the bare
   `synchronized (this)` blocks in `onLayout`, `swapBuffers` and `onAttachedToWindow` are
   `synchronized (this) { … }`.  Kotlin parameters are immutable, so `onLayout`'s reassigned
   `changed` is carried into a same-named local (P-83 ⑱, the `EmacsActivity.onWindowFocusChanged`
   shape), and chained assignments (`bitmapDirty = needExpose = true;`,
   `measuredWidth = measuredHeight = 0;`) and compound bit ops (`info.imeOptions |= …`) become two
   statements / `= … or …` (P-83 ⑬ - Kotlin has no `|=`).  `MeasureSpec` and `GONE` are inherited
   names in Java but not in Kotlin, so they are written `View.MeasureSpec.…` and `View.GONE`;
   `window.eventModifiers (event)` is a `static` reached through an instance in Java, which Kotlin
   forbids, so it is `EmacsWindow.eventModifiers (event)`.  Java's object `==`/`!=`
   (`child == surfaceView`, `window == EmacsActivity.focusedWindow`, `parent != view.getParent ()`)
   are reference comparisons and become `===`/`!==` (P-83 ⑳).  The reference tests several mutable
   properties for null and then dereferences them (`bitmap`, `canvas`, `contextMenu`); Kotlin
   never smart-casts a `var` property, so each dereference keeps a `!!` at the reference's own
   point (P-83 ⑤).  Nullable locals narrow normally (`oldBitmap`, `savedBitmap`, `selection`,
   `rootWindowInsets`).  `onFocusChanged`'s third parameter is `@Nullable` in the stub, so it is
   `Rect?` (P-83 ⑮); `onCreateInputConnection` returns `InputConnection?` because the reference
   returns null.  `menu.menuItems` and `EmacsWindowManager.MANAGER.consumers` are read across
   targets (`EmacsContextMenu.kt` 139, `EmacsWindowManager.kt` 136), as are
   `EmacsActivity.focusedWindow` and `EmacsActivity.closeContextMenu` (inherited from `Activity`).
   Java's unused `android.graphics.Paint` import and the unused `Canvas canvas;` local of
   `swapBuffers` are not carried (Kotlin warns on an unused local).  */
class EmacsView(window: EmacsWindow) :
    ViewGroup(ContextThemeWrapper(EmacsService.SERVICE, R.style.EmacsStyle)),
    ViewTreeObserver.OnGlobalLayoutListener {
  companion object {
    private const val TAG = "EmacsView"
  }

  /* The associated EmacsWindow.  */
  @JvmField var window: EmacsWindow

  /* The buffer bitmap.  */
  @JvmField var bitmap: Bitmap? = null

  /* The associated canvases.  */
  @JvmField var canvas: Canvas? = null

  /* The damage region.  */
  @JvmField var damageRegion: Region

  /* The associated surface view.  */
  private var surfaceView: EmacsSurfaceView

  /* Whether or not a configure event must be sent for the next layout
     event regardless of what changed.  */
  @JvmField var mustReportLayout: Boolean = false

  /* Whether or not bitmaps must be recreated upon the next call to
     getBitmap.  */
  private var bitmapDirty: Boolean = false

  /* Whether or not a popup is active.  */
  private var popupActive: Boolean = false

  /* Whether the back buffer has been updated since the last swap.  */
  private var unswapped: Boolean = false

  /* The current context menu.  */
  private var contextMenu: EmacsContextMenu? = null

  /* The last measured width and height.  */
  private var measuredWidth: Int = 0
  private var measuredHeight: Int = 0

  /* The serial of the last clip rectangle change.  */
  private var lastClipSerial: Long = 0

  /* The InputMethodManager for this view's context.  */
  @JvmField var imManager: InputMethodManager

  /* Whether or not this view is attached to a window.  */
  @JvmField var isAttachedToWindow: Boolean = false

  /* Whether or not this view should have the on screen keyboard
     displayed whenever possible.  */
  @JvmField var isCurrentlyTextEditor: Boolean = false

  /* The associated input connection.  */
  private var inputConnection: EmacsInputConnection? = null

  /* The current IC mode.  See `android_reset_ic' for more
     details.  */
  private var icMode: Int = 0

  /* The number of calls to `resetIC' to have taken place the last
     time an InputConnection was created.  */
  @JvmField var icSerial: Long = 0

  /* The number of calls to `recetIC' that have taken place.  */
  @Volatile
  @JvmField var icGeneration: Long = 0

  init {
    var tem: Any?
    val context: Context

    this.window = window
    this.damageRegion = Region()

    setFocusable(true)
    setFocusableInTouchMode(true)

    /* Create the surface view.  */
    this.surfaceView = EmacsSurfaceView(this)
    addView(this.surfaceView)

    /* Get rid of the default focus highlight.  */
    if (Build.VERSION.SDK_INT > Build.VERSION_CODES.O)
      setDefaultFocusHighlightEnabled(false)

    /* Obtain the input method manager.  */
    context = getContext()
    tem = context.getSystemService(Context.INPUT_METHOD_SERVICE)
    imManager = tem as InputMethodManager

    /* Add this view as its own global layout listener.  */
    getViewTreeObserver().addOnGlobalLayoutListener(this)
  }

  private fun handleDirtyBitmap() {
    var oldBitmap: Bitmap?
    var measuredWidth: Int
    var measuredHeight: Int

    /* Load measuredWidth and measuredHeight.  */
    measuredWidth = this.measuredWidth
    measuredHeight = this.measuredHeight

    if (measuredWidth == 0 || measuredHeight == 0)
      return

    if (!isAttachedToWindow)
      return

    /* If bitmap is the same width and height as the measured width
       and height, there is no need to do anything.  Avoid allocating
       the extra bitmap.  */
    if (bitmap != null
        && (bitmap!!.getWidth() == measuredWidth
            && bitmap!!.getHeight() == measuredHeight)) {
      bitmapDirty = false
      return
    }

    /* Save the old bitmap.  */
    oldBitmap = bitmap

    /* Recreate the back buffer bitmap.  */
    bitmap = Bitmap.createBitmap(measuredWidth,
                                 measuredHeight,
                                 Bitmap.Config.ARGB_8888)
    bitmap!!.eraseColor(window.background or 0xff000000.toInt())

    /* And canvases.  */
    canvas = Canvas(bitmap!!)
    canvas!!.save()

    /* Since the clip rectangles have been cleared, clear the clip
       rectangle ID.  */
    lastClipSerial = 0

    /* Clear the bitmap reallocation flag.  */
    bitmapDirty = false

    /* Explicitly free the old bitmap's memory.  The bitmap might
       continue to be referenced by canvas or JNI objects returned by
       getBitmap or getCanvas, but the underlying storage will not be
       released until such references disappear.  See
       BitmapWrapper::freePixels in hwui/jni/Bitmap.cpp.  */
    if (oldBitmap != null)
      oldBitmap.recycle()

    /* Some Android versions still refuse to release the bitmap until
       the next GC.  */
    Runtime.getRuntime().gc()
  }

  @Synchronized
  fun explicitlyDirtyBitmap() {
    bitmapDirty = true
  }

  @Synchronized
  fun getBitmap(): Bitmap? {
    /* Never alter the bitmap if modifications have been received that
       are still to be copied to the front buffer, as this indicates
       that redisplay is in the process of copying matrix contents to
       the glass, and such events as generally prompt a complete
       regeneration of the frame's contents might not be processed.  */
    if (!unswapped && (bitmapDirty || bitmap == null))
      handleDirtyBitmap()
    unswapped = true

    return bitmap
  }

  @Synchronized
  fun getCanvas(gc: EmacsGC): Canvas? {
    if (!unswapped && (bitmapDirty || bitmap == null))
      handleDirtyBitmap()

    if (canvas == null)
      return null
    unswapped = true

    /* Update clip rectangles if necessary.  */
    if (gc.clipRectID != lastClipSerial) {
      canvas!!.restore()
      canvas!!.save()

      if (gc.real_clip_rects != null) {
        for (i in gc.real_clip_rects!!.indices)
          canvas!!.clipRect(gc.real_clip_rects!![i]!!)
      }

      lastClipSerial = gc.clipRectID
    }

    return canvas
  }

  override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
    val measurements: Rect
    var width: Int
    var height: Int

    /* Return the width and height of the window regardless of what
       the parent says.  */
    measurements = window.getGeometry()

    width = measurements.width()
    height = measurements.height()

    /* Now apply any extra requirements in widthMeasureSpec and
       heightMeasureSpec.  */

    if (View.MeasureSpec.getMode(widthMeasureSpec) == View.MeasureSpec.EXACTLY)
      width = View.MeasureSpec.getSize(widthMeasureSpec)
    else if (View.MeasureSpec.getMode(widthMeasureSpec) == View.MeasureSpec.AT_MOST
             && width > View.MeasureSpec.getSize(widthMeasureSpec))
      width = View.MeasureSpec.getSize(widthMeasureSpec)

    if (View.MeasureSpec.getMode(heightMeasureSpec) == View.MeasureSpec.EXACTLY)
      height = View.MeasureSpec.getSize(heightMeasureSpec)
    else if (View.MeasureSpec.getMode(heightMeasureSpec) == View.MeasureSpec.AT_MOST
             && height > View.MeasureSpec.getSize(heightMeasureSpec))
      height = View.MeasureSpec.getSize(heightMeasureSpec)

    /* This is strictly necessary to propagate layout requests to
       children.  */
    this.measureChildren(widthMeasureSpec, heightMeasureSpec)
    super.setMeasuredDimension(width, height)
  }

  /* Return whether this view's window is focused.  This is made
     necessary by Android 11's unreliable dispatch of
     onWindowFocusChanged prior to gesture navigation away from a
     frame.  */

  fun checkWindowFocus(): Boolean {
    var activity: EmacsActivity
    var consumer: Any?

    consumer = window.getAttachedConsumer()

    if (consumer !is EmacsActivity)
      return false

    activity = consumer
    return activity.hasWindowFocus()
  }

  /* Note that the monitor lock for the window must never be held from
     within that for the view, because the window acquires locks in the
     opposite direction.  */

  override fun onLayout(changed: Boolean, left: Int, top: Int, right: Int,
                        bottom: Int) {
    var count: Int
    var oldMeasuredWidth: Int
    var oldMeasuredHeight: Int
    var child: View
    var windowRect: Rect
    var needExpose: Boolean
    var rootWindowInsets: WindowInsets?

    /* The reference reassigns its `changed' parameter in the block below (P-83 ④); Kotlin
       parameters are immutable, so the value is carried into a local of the same name.  */
    var changed: Boolean = changed

    count = getChildCount()
    needExpose = false

    synchronized(this) {
      /* Load measuredWidth and measuredHeight.  */
      oldMeasuredWidth = measuredWidth
      oldMeasuredHeight = measuredHeight

      /* Set measuredWidth and measuredHeight.  */
      measuredWidth = right - left
      measuredHeight = bottom - top

      /* If oldMeasuredHeight or oldMeasuredWidth are wrong, set
         changed to true as well.  */

      if (right - left != oldMeasuredWidth
          || bottom - top != oldMeasuredHeight)
        changed = true

      /* Dirty the back buffer if the layout change resulted in the view
         being resized.  */

      if (changed) {
        /* Expose the window upon a change in the view's size that
           prompts the creation of a new bitmap.  */
        bitmapDirty = true
        needExpose = true

        /* This might return NULL if this view is not attached.  */
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
          /* If a toplevel view is focused and
             isCurrentlyTextEditor is enabled when the IME is
             hidden, clear isCurrentlyTextEditor so it isn't shown
             again if the user dismisses Emacs before
             returning.  */
          rootWindowInsets = getRootWindowInsets()

          if (isCurrentlyTextEditor
              && rootWindowInsets != null
              && isAttachedToWindow
              && !rootWindowInsets.isVisible(WindowInsets.Type.ime())
              /* N.B. that the keyboard is dismissed during
                 gesture navigation under Android 30, but the
                 system is quite temperamental regarding whether
                 the window is focused at that point.  Ideally
                 isCurrentlyTextEditor shouldn't be reset in that
                 case, but detecting that situation appears to be
                 impossible.  Sigh.  */
              && (window === EmacsActivity.focusedWindow
                  && hasWindowFocus()))
            isCurrentlyTextEditor = false
        }
      }
    }

    for (i in 0 until count) {
      child = getChildAt(i)

      if (child === surfaceView)
        child.layout(0, 0, right - left, bottom - top)
      else if (child.getVisibility() != View.GONE) {
        if (child !is EmacsView)
          continue

        /* What to do: lay out the view precisely according to its
           window rect.  */
        windowRect = child.window.getGeometry()
        child.layout(windowRect.left, windowRect.top,
                     windowRect.right, windowRect.bottom)
      }
    }

    /* Now report the layout change to the window.  */

    if (changed || mustReportLayout) {
      mustReportLayout = false
      window.viewLayout(left, top, right, bottom)
    }

    if (needExpose && isAttachedToWindow)
      EmacsNative.sendExpose(this.window.handle, 0, 0,
                             right - left, bottom - top)
  }

  fun damageRect(damageRect: Rect) {
    EmacsService.checkEmacsThread()
    damageRegion.union(damageRect)
  }

  /* This function enables damage to be recorded without consing a new
     Rect object.  */

  fun damageRect(left: Int, top: Int, right: Int, bottom: Int) {
    EmacsService.checkEmacsThread()
    damageRegion.op(left, top, right, bottom, Region.Op.UNION)
  }

  /* Complete deferred reconfiguration of the front buffer after a
     buffer swap completes, and generate Expose events for the same.  */

  private fun postSwapBuffers() {
    if (!unswapped)
      return

    unswapped = false

    /* If the bitmap is dirty, reconfigure the bitmap and
       generate an Expose event to produce its contents.  */

    if ((bitmapDirty || bitmap == null)
        /* Do not generate Expose events if handleDirtyBitmap will
           not create a valid bitmap, or the consequent buffer swap
           will produce another event, ad infinitum.  */
        && isAttachedToWindow && measuredWidth != 0
        && measuredHeight != 0) {
      handleDirtyBitmap()
      EmacsNative.sendExpose(this.window.handle, 0, 0,
                             measuredWidth, measuredHeight)
    }
  }

  fun swapBuffers() {
    var damageRect: Rect?

    /* Make sure this function is called only from the Emacs
       thread.  */
    EmacsService.checkEmacsThread()

    damageRect = null

    /* Now see if there is a damage region.  */

    if (damageRegion.isEmpty()) {
      postSwapBuffers()
      return
    }

    /* And extract and clear the damage region.  */

    damageRect = damageRegion.getBounds()
    damageRegion.setEmpty()

    synchronized(this) {
      /* Transfer the bitmap to the surface view, then invalidate
         it.  */
      surfaceView.setBitmap(bitmap, damageRect)
      postSwapBuffers()
    }
  }

  override fun onKeyPreIme(keyCode: Int, event: KeyEvent): Boolean {
    /* Several Android systems intercept key events representing
       C-SPC.  Avert this by detecting C-SPC events here and relaying
       them directly to onKeyDown.

       Make this optional though, since some input methods also
       leverage C-SPC as a shortcut for switching languages.  */

    if ((keyCode == KeyEvent.KEYCODE_SPACE
         && (EmacsWindow.eventModifiers(event)
             and KeyEvent.META_CTRL_MASK) != 0)
        && !EmacsNative.shouldForwardCtrlSpace())
      return onKeyDown(keyCode, event)

    return super.onKeyPreIme(keyCode, event)
  }

  override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
    if (((keyCode == KeyEvent.KEYCODE_VOLUME_UP
          || keyCode == KeyEvent.KEYCODE_VOLUME_DOWN
          || keyCode == KeyEvent.KEYCODE_VOLUME_MUTE)
         && !EmacsNative.shouldForwardMultimediaButtons())
        || keyCode == KeyEvent.KEYCODE_SCROLL_LOCK
        || keyCode == KeyEvent.KEYCODE_NUM_LOCK)
      return super.onKeyDown(keyCode, event)

    return window.onKeyDown(keyCode, event)
  }

  override fun onKeyMultiple(keyCode: Int, repeatCount: Int,
                             event: KeyEvent): Boolean {
    if (((keyCode == KeyEvent.KEYCODE_VOLUME_UP
          || keyCode == KeyEvent.KEYCODE_VOLUME_DOWN
          || keyCode == KeyEvent.KEYCODE_VOLUME_MUTE)
         && !EmacsNative.shouldForwardMultimediaButtons())
        || keyCode == KeyEvent.KEYCODE_SCROLL_LOCK
        || keyCode == KeyEvent.KEYCODE_NUM_LOCK)
      return super.onKeyMultiple(keyCode, repeatCount, event)

    return window.onKeyDown(keyCode, event)
  }

  override fun onKeyUp(keyCode: Int, event: KeyEvent): Boolean {
    if (((keyCode == KeyEvent.KEYCODE_VOLUME_UP
          || keyCode == KeyEvent.KEYCODE_VOLUME_DOWN
          || keyCode == KeyEvent.KEYCODE_VOLUME_MUTE)
         && !EmacsNative.shouldForwardMultimediaButtons())
        || keyCode == KeyEvent.KEYCODE_SCROLL_LOCK
        || keyCode == KeyEvent.KEYCODE_NUM_LOCK)
      return super.onKeyUp(keyCode, event)

    return window.onKeyUp(keyCode, event)
  }

  override fun onFocusChanged(gainFocus: Boolean, direction: Int,
                              previouslyFocusedRect: Rect?) {
    window.onFocusChanged(gainFocus)
    super.onFocusChanged(gainFocus, direction,
                         previouslyFocusedRect)
  }

  override fun onGenericMotionEvent(motion: MotionEvent): Boolean {
    return window.onGenericMotionEvent(motion)
  }

  override fun onTouchEvent(motion: MotionEvent): Boolean {
    return window.onTouchEvent(motion)
  }

  override fun onDragEvent(drag: DragEvent): Boolean {
    /* Inter-program drag and drop isn't supported under Android 23
       and earlier.  */

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N)
      return false

    return window.onDragEvent(drag)
  }

  private fun moveChildToBack(child: View) {
    val index: Int

    index = indexOfChild(child)

    if (index > 0) {
      detachViewFromParent(index)

      /* The view at 0 is the surface view.  */
      attachViewToParent(child, 1, child.getLayoutParams())
    }
  }

  /* The following four functions must not be called if the view has
     no parent, or is parented to an activity.  */

  fun raise() {
    val parent: EmacsView

    parent = getParent() as EmacsView

    if (parent.indexOfChild(this)
        == parent.getChildCount() - 1)
      return

    parent.bringChildToFront(this)
  }

  fun lower() {
    val parent: EmacsView

    parent = getParent() as EmacsView

    if (parent.indexOfChild(this) == 1)
      return

    parent.moveChildToBack(this)
  }

  fun moveAbove(view: EmacsView) {
    val parent: EmacsView
    var index: Int

    parent = getParent() as EmacsView

    if (parent !== view.getParent())
      throw IllegalStateException("Moving view above non-sibling")

    index = parent.indexOfChild(this)
    parent.detachViewFromParent(index)
    index = parent.indexOfChild(view)
    parent.attachViewToParent(this, index + 1, getLayoutParams())
  }

  fun moveBelow(view: EmacsView) {
    val parent: EmacsView
    var index: Int

    parent = getParent() as EmacsView

    if (parent !== view.getParent())
      throw IllegalStateException("Moving view above non-sibling")

    index = parent.indexOfChild(this)
    parent.detachViewFromParent(index)
    index = parent.indexOfChild(view)
    parent.attachViewToParent(this, index, getLayoutParams())
  }

  override fun onCreateContextMenu(menu: ContextMenu) {
    if (contextMenu == null)
      return

    contextMenu!!.expandTo(menu, this)
  }

  fun popupMenu(menu: EmacsContextMenu, xPosition: Int,
                yPosition: Int, force: Boolean): Boolean {
    var context: ContextThemeWrapper

    if (popupActive && !force)
      return false

    /* Android will permanently cease to display any popup menus at
       all if the list of menu items is empty.  Prevent this by
       promptly returning if there are no menu items.  */

    if (menu.menuItems!!.isEmpty())
      return false

    contextMenu = menu
    popupActive = true

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.VANILLA_ICE_CREAM) {
      context = getContext() as ContextThemeWrapper
      /* It is necessary to reload the current theme before attempting
         to display a new popup menu, or any previously applied system
         theme will continue to apply to it.  */
      context.setTheme(R.style.EmacsStyleOpen)
      context.setTheme(R.style.EmacsStyle)
    }

    /* Use showContextMenu (float, float) on N to get actual popup
       behavior.  */
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N)
      return showContextMenu(xPosition.toFloat(), yPosition.toFloat())
    else
      return showContextMenu()
  }

  fun cancelPopupMenu() {
    if (!popupActive)
      throw IllegalStateException("cancelPopupMenu called without"
                                  + " popupActive set")

    contextMenu = null
    popupActive = false

    /* It is not possible to know with 100% certainty which activity is
       currently displaying the context menu.  Loop over each activity
       and call `closeContextMenu' instead.  */

    for (consumer in EmacsWindowManager.MANAGER.consumers) {
      if (consumer is EmacsActivity)
        consumer.closeContextMenu()
    }
  }

  @Synchronized
  override fun onDetachedFromWindow() {
    var savedBitmap: Bitmap?

    savedBitmap = bitmap
    isAttachedToWindow = false
    bitmap = null
    canvas = null

    surfaceView.setBitmap(null, null)

    /* Recycle the bitmap and call GC.  */

    if (savedBitmap != null)
      savedBitmap.recycle()

    /* Collect the bitmap storage; it could be large.  */
    Runtime.getRuntime().gc()
    super.onDetachedFromWindow()
  }

  override fun onAttachedToWindow() {
    synchronized(this) {
      isAttachedToWindow = true

      /* Dirty the bitmap, as it was destroyed when
         onDetachedFromWindow was called.  */
      bitmapDirty = true

      /* Rather than unconditionally generating an exposure event upon
         window attachment, avoid delivering successive Exposure
         events if the size of the window has changed but is still to
         be reported by clearing the measured width and height, and
         requesting another layout computation.  */
      measuredWidth = 0
      measuredHeight = 0
    }

    requestLayout()
    super.onAttachedToWindow()
  }

  fun showOnScreenKeyboard() {
    /* Specifying no flags at all tells the system the user asked for
       the input method to be displayed.  */

    imManager.showSoftInput(this, 0)
    isCurrentlyTextEditor = true
  }

  fun hideOnScreenKeyboard() {
    imManager.hideSoftInputFromWindow(this.getWindowToken(),
                                      0)
    isCurrentlyTextEditor = false
  }

  override fun onCreateInputConnection(info: EditorInfo): InputConnection? {
    var mode: Int
    var selection: IntArray?

    /* Figure out what kind of IME behavior Emacs wants.  */
    mode = getICMode()

    /* Make sure the input method never displays a full screen input
       box that obscures Emacs.  */
    info.imeOptions = EditorInfo.IME_FLAG_NO_FULLSCREEN
    info.imeOptions = info.imeOptions or EditorInfo.IME_FLAG_NO_EXTRACT_UI

    /* Set a reasonable inputType.  */
    info.inputType = InputType.TYPE_CLASS_TEXT

    /* If this fails or ANDROID_IC_MODE_NULL was requested, then don't
       initialize the input connection.  */

    if (mode == EmacsService.IC_MODE_NULL) {
      info.inputType = InputType.TYPE_NULL
      return null
    }

    /* Set icSerial.  If icSerial < icGeneration, the input connection
       has been reset, and future input should be ignored until a new
       connection is created.  */

    icSerial = icGeneration

    /* Reset flags set by the previous input method.  */

    EmacsNative.clearInputFlags(window.handle)

    /* Obtain the current position of point and set it as the
       selection.  Don't do this under one specific situation: if
       `android_update_ic' is being called in the main thread, trying
       to synchronize with it can cause a dead lock in the IM manager.
       See icBeginSynchronous in EmacsService.java for more
       details.  */

    selection = EmacsService.viewGetSelection(window.handle)

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, ("onCreateInputConnection: "
                  + (if (selection != null)
                       Arrays.toString(selection)
                     else
                       "(unavailable)")))

    if (selection == null) {
      /* If the selection could not be obtained, return 0 by 0.
         However, ask for the selection position to be updated as
         soon as possible.  */

      selection = intArrayOf(0, 0)
      EmacsNative.requestSelectionUpdate(window.handle)
    }

    if (mode == EmacsService.IC_MODE_ACTION
        || mode == EmacsService.IC_MODE_PASSWORD)
      info.imeOptions = info.imeOptions or EditorInfo.IME_ACTION_DONE

    if (mode == EmacsService.IC_MODE_PASSWORD)
      info.inputType = info.inputType or InputType.TYPE_TEXT_VARIATION_PASSWORD

    /* Set the initial selection fields.  */
    info.initialSelStart = selection[0]
    info.initialSelEnd = selection[1]

    /* Create the input connection if necessary.  */

    if (inputConnection == null)
      inputConnection = EmacsInputConnection(this)
    else
      /* Clear several pieces of state in the input connection.  */
      inputConnection!!.reset()

    /* Return the input connection.  */
    return inputConnection
  }

  @Synchronized
  override fun onCheckIsTextEditor(): Boolean {
    /* If value is true, then the system will display the on screen
       keyboard.  */
    return isCurrentlyTextEditor
  }

  override fun isOpaque(): Boolean {
    /* Returning true here allows the system to not draw the contents
       of windows underneath this view, thereby improving
       performance.  */
    return true
  }

  fun setICMode(icMode: Int) {
    this.icMode = icMode
  }

  fun getICMode(): Int {
    return icMode
  }

  override fun onGlobalLayout() {
    var locations: IntArray

    /* Get the absolute offset of this view and specify its left and
       top position in subsequent ConfigureNotify events.  */

    locations = IntArray(2)
    getLocationInWindow(locations)
    window.notifyContentRectPosition(locations[0],
                                     locations[1])
  }

  override fun onApplyWindowInsets(insets: WindowInsets): WindowInsets {
    var rootWindowInsets: WindowInsets?

    /* This function is called when window insets change, which
       encompasses input method visibility changes under Android 30
       and later.  If a toplevel view is focused and
       isCurrentlyTextEditor is enabled when the IME is hidden, clear
       isCurrentlyTextEditor so it isn't shown again if the user
       dismisses Emacs before returning.  */

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R)
      return super.onApplyWindowInsets(insets)

    /* This might return NULL if this view is not attached.  */
    rootWindowInsets = getRootWindowInsets()

    if (isCurrentlyTextEditor
        && rootWindowInsets != null
        && isAttachedToWindow
        && !rootWindowInsets.isVisible(WindowInsets.Type.ime())
        && window === EmacsActivity.focusedWindow)
      isCurrentlyTextEditor = false

    return super.onApplyWindowInsets(insets)
  }
}
