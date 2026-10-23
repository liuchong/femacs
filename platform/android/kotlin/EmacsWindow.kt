// femacs: port of java/org/gnu/emacs/EmacsWindow.java @ fd7afced403.

package org.femacs

import java.lang.IllegalStateException

import java.util.ArrayList
import java.util.LinkedHashMap

import java.util.concurrent.Callable
import java.util.concurrent.FutureTask

import android.app.Activity

import android.content.ClipDescription
import android.content.Context

import android.graphics.Rect
import android.graphics.Canvas
import android.graphics.Bitmap
import android.graphics.PixelFormat

import android.net.Uri

import android.view.DragEvent
import android.view.Gravity
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.View
import android.view.ViewManager
import android.view.WindowManager

import android.util.SparseArray
import android.util.Log

import android.os.Build
import android.os.SystemClock

/* This defines a window, which is a handle.  Windows represent a
   rectangular subset of the screen with their own contents.

   Windows either have a parent window, in which case their views are
   attached to the parent's view, or are "floating", in which case
   their views are attached to the parent activity (if any), else
   nothing.

   Views are also drawables, meaning they can accept drawing
   requests.  */

/* The reference is `public final class EmacsWindow extends EmacsHandleObject implements
   EmacsDrawable' (74-75); Kotlin's plain `class' is final too, so the shape matches (P-83 ①).

   Nothing on the C side reads an EmacsWindow *field* through the JNI, but two fields and a
   handful of members are read *by name* from the reference's own Java, from the already-ported
   `.kt' targets and from the mirror `EmacsWindowManager.kt':
     - `attachmentToken' (`long', mutable), `previouslyAttached' (`boolean', mutable),
       `preserve' (`boolean', mutable), `handle' (inherited from `EmacsHandleObject.kt'),
       `getAttachedConsumer ()', `onActivityDetached ()', `getGeometry ()', `noticeIconified ()'
       and `noticeDeiconified ()' are all bound by name from `EmacsWindowManager.kt'
       (106/108/155/159/160/185/186/194/202/234/243/248/271/283/284/289/307/318/348/359/360/372/
       374/376/379);
     - `view', `parent', `children', `background' and `fullscreen' are read by the reference's
       `EmacsActivity.java' 106-246/546, `EmacsService.java' 582-594 and `EmacsView.java' 198, and
       `wmName' by `EmacsActivity.java' 618/621.
   They therefore keep their JVM names and types as `@JvmField's (VOLATILE for the reference's
   `volatile boolean isMapped'/`volatile int background'), and the `Long'/`Int' boundary keeps its
   `L's (P-83 ②⑯).

   The JNI's `android_init_emacs_window' (android.c 1959-1988) looks the methods up by name and
   descriptor, so every public method below stays a `fun' of the same name; `eventModifiers' is
   `public static' (707) and is called as `window.eventModifiers (event)' by `EmacsView.java' 524,
   therefore it is a companion `@JvmStatic' (P-83 ⑪).  The constructor keeps the JNI descriptor
   `(Lorg/gnu/emacs/EmacsWindow;IIIIZ)V' (android.c 3199-3202).

   The reference's two `@SuppressWarnings ("deprecation")' members are `@Suppress("DEPRECATION",
   "OVERRIDE_DEPRECATION")' (P-83 ④).  Its `LinkedHashMap' subclass (`removeEldestEntry') becomes
   an `object :' expression; its `Runnable's whose body contains a `return' become
   `object : Runnable' (a SAM-converted `return' would leave the enclosing function, cf.
   `EmacsSafThread.kt'), the rest stay SAM conversions.  Two unused locals are dropped (`int j' at
   1857 and `Rect geometry' at 1798, neither read anywhere) and `final int width, height' in
   `mapWindow' (477-483) likewise - Kotlin would warn on each.  The `Object tem' temps of
   `setWmName'/`setFullscreen' are kept where their only use is an `instanceof' test, and Java's
   assignment-expression `(activity = EmacsActivity.lastFocusedActivity) != null' (1955) is
   hoisted one line, the P-83 ⑬ move.  `EmacsWindow object' (1509) cannot keep its name (`object'
   is a keyword) and becomes `current' (P-83 ㉓ family).  */
class EmacsWindow(parent: EmacsWindow?, x: Int, y: Int, width: Int, height: Int,
                  overrideRedirect: Boolean) : EmacsHandleObject(), EmacsDrawable {
  companion object {
    private const val TAG = "EmacsWindow"

    /* Whether any windows have yet been created in this session.  */
    private var initialWindowCreated: Boolean = false

    /* Return the modifier mask associated with the specified keyboard
       input EVENT.  Replace bits representing Left or Right keys with
       their corresponding general modifier bits.  */

    @JvmStatic
    fun eventModifiers(event: KeyEvent): Int {
      var state: Int

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB_MR2)
        state = KeyEvent.normalizeMetaState(event.getMetaState())
      else {
        /* Replace this with getMetaState and manual
           normalization.  */
        state = event.getMetaState()

        /* Normalize the state by setting the generic modifier bit if
           either a left or right modifier is pressed.  */

        if ((state and KeyEvent.META_ALT_LEFT_ON) != 0
            || (state and KeyEvent.META_ALT_RIGHT_ON) != 0)
          state = state or KeyEvent.META_ALT_MASK

        if ((state and KeyEvent.META_CTRL_LEFT_ON) != 0
            || (state and KeyEvent.META_CTRL_RIGHT_ON) != 0)
          state = state or KeyEvent.META_CTRL_MASK
      }

      return state
    }
  }

  private class Coordinate(var x: Int, var y: Int, val button: Int, val id: Int)

  /* The view associated with the window.  */
  @JvmField var view: EmacsView

  /* The geometry of the window.  */
  private var rect: Rect

  /* The parent window, or null if it is the root window.  */
  @JvmField var parent: EmacsWindow? = parent

  /* List of all children in stacking order.  This must be kept
     consistent with their Z order!

     Synchronize access to this list with itself.  */
  @JvmField var children: ArrayList<EmacsWindow>

  /* Map between pointer identifiers and last known position.  Used to
     compute which pointer changed upon a touch event.  */
  private var pointerMap: SparseArray<Coordinate>

  /* The window consumer currently attached, if it exists.  */
  private var attached: EmacsWindowManager.WindowConsumer? = null

  /* The window background scratch GC.  foreground is always the
     window background.  */
  private var scratchGC: EmacsGC

  /* The button state and keyboard modifier mask at the time of the
     last button press or release event.  */
  @JvmField var lastButtonState: Int = 0

  /* Whether or not the window is mapped.  */
  @Volatile private var isMapped: Boolean = false

  /* Whether or not to ask for focus upon being mapped.  */
  private var dontFocusOnMap: Boolean = false

  /* Whether or not the window is override-redirect.  An
     override-redirect window always has its own system window.  */
  private var overrideRedirect: Boolean = overrideRedirect

  /* The window manager that is the parent of this window.  NULL if
     there is no such window manager.  */
  private var windowManager: WindowManager? = null

  /* The time of the last release of the quit keycode, generally
     KEYCODE_VOLUME_DOWN.  This is used to signal quit upon two rapid
     presses of such key.  */
  private var lastQuitKeyRelease: Long = 0

  /* Linked list of character strings which were recently sent as
     events.  */
  @JvmField var eventStrings: LinkedHashMap<Int, String>

  /* Whether or not this window is fullscreen.  */
  @JvmField var fullscreen: Boolean = false

  /* The window background pixel.  This is used by EmacsView when
     creating new bitmaps.  */
  @Volatile @JvmField var background: Int = 0

  /* The position of this window relative to the root window.  */
  @JvmField var xPosition: Int = 0
  @JvmField var yPosition: Int = 0

  /* The position of the last drag and drop event received; both
     values are -1 if no drag and drop operation is under way.  */
  private var dndXPosition: Int = 0
  private var dndYPosition: Int = 0

  /* Identifier binding this window to the activity created for it, or
     -1 if the window should be attached to system-created activities
     (i.e. the activity launched by the system at startup).  Value is
     meaningless under API level 29 and earlier.  */
  @JvmField var attachmentToken: Long = 0

  /* Whether this window should be preserved during window pruning,
     and whether this window has previously been attached to a task.  */
  @JvmField var preserve: Boolean = false
  @JvmField var previouslyAttached: Boolean = false

  /* The window manager name of this window, which supplies the name of
     activities in which it is displayed as a toplevel window, or
     NULL.  `EmacsWindow.java' 184 declares `public String wmName;', so
     the field starts NULL and the port keeps both that default and the
     type: an earlier draft made it non-null with a default of "Emacs"
     to satisfy a non-null local in `EmacsActivity.kt', which invented a
     value the reference never stores (its fallback to "Emacs" is in
     `updateWmName', not in the field).  The local there is nullable now,
     so the reference's own `window.wmName == null' test is the
     meaningful check it is in C (status 3.358).  `setWmName' below keeps
     the non-null parameter the reference's setter takes (P-83 ⑤).  */
  @JvmField var wmName: String? = null

  init {
    rect = Rect(x, y, x + width, y + height)
    pointerMap = SparseArray<Coordinate>()

    /* Create the view from the context's UI thread.  The window is
       unmapped, so the view is GONE.  */
    view = EmacsService.SERVICE!!.getEmacsView(this, View.GONE,
                                             parent == null)

    /* The initial frame should always be bound to the startup
       activity.  */
    if (!initialWindowCreated) {
      attachmentToken = -1
      initialWindowCreated = true
    }

    /* Create the list of children.  */
    children = ArrayList<EmacsWindow>()

    if (parent != null) {
      synchronized(parent.children) {
        parent.children.add(this)
      }

      EmacsService.SERVICE!!.runOnUiThread(Runnable {
        parent.view.addView(view)
      })
    }

    scratchGC = EmacsGC()

    /* Create the map of input method-committed strings.  Keep at most
       ten strings in the map.  */

    eventStrings =
      object : LinkedHashMap<Int, String>() {
        override fun removeEldestEntry(eldest: MutableMap.MutableEntry<Int, String>): Boolean {
          return size > 10
        }
      }

    dndXPosition = -1
    dndYPosition = -1
  }

  fun changeWindowBackground(pixel: Int) {
    /* scratchGC is used as the argument to a FillRectangles req.  */
    scratchGC.foreground = pixel
    scratchGC.markDirty(false)

    /* Make the background known to the view as well.  */
    background = pixel
  }

  @Synchronized
  fun getGeometry(): Rect {
    return Rect(rect)
  }

  @Throws(IllegalStateException::class)
  @Synchronized
  override fun destroyHandle() {
    if (parent != null) {
      synchronized(parent!!.children) {
        parent!!.children.remove(this)
      }
    }

    /* This is just a sanity test and is not reliable since `children'
       may be modified between isEmpty and handle destruction.  */
    if (!children.isEmpty())
      throw IllegalStateException("Trying to destroy window with "
                                  + "children!")

    /* Remove the view from its parent and make it invisible.  */
    EmacsService.SERVICE!!.runOnUiThread(object : Runnable {
      override fun run() {
        val parent: ViewManager?
        val manager: EmacsWindowManager

        /* Invalidate the focus; this should transfer the input focus
           to the next eligible window as this window is no longer
           present in parent.children.  */
        EmacsActivity.invalidateFocus(4)

        if (EmacsActivity.focusedWindow === this@EmacsWindow)
          EmacsActivity.focusedWindow = null

        manager = EmacsWindowManager.MANAGER
        view.setVisibility(View.GONE)

        /* If the window manager is set, use that instead.  */
        if (windowManager != null)
          parent = windowManager
        else
          parent = view.getParent() as ViewManager?
        windowManager = null

        if (parent != null)
          parent.removeView(view)

        manager.detachWindow(this@EmacsWindow)
      }
    })

    super.destroyHandle()
  }

  fun setConsumer(consumer: EmacsWindowManager.WindowConsumer?) {
    attached = consumer
  }

  fun getAttachedConsumer(): EmacsWindowManager.WindowConsumer? {
    return attached
  }

  @Synchronized
  fun viewLayout(left: Int, top: Int, right: Int, bottom: Int): Long {
    val rectWidth: Int
    val rectHeight: Int

    /* If this is an override-redirect window, don't ever modify
       rect.left and rect.top, as its WM window will always have been
       moved in unison with itself.  */

    if (overrideRedirect) {
      rect.right = rect.left + (right - left)
      rect.bottom = rect.top + (bottom - top)
    }
    /* If parent is null, use xPosition and yPosition instead of the
       geometry rectangle positions.  */
    else if (parent == null) {
      rect.left = xPosition
      rect.top = yPosition
      rect.right = rect.left + (right - left)
      rect.bottom = rect.top + (bottom - top)
    }
    /* Otherwise accept the new position offered by the toolkit.  FIXME:
       isn't there a potential race condition here if the toolkit lays
       out EmacsView after a child frame's rect is set but before it
       calls onLayout to read the modified rect?  */
    else {
      rect.left = left
      rect.top = top
      rect.right = right
      rect.bottom = bottom
    }

    rectWidth = right - left
    rectHeight = bottom - top

    return EmacsNative.sendConfigureNotify(this.handle,
                                           System.currentTimeMillis(),
                                           left, top, rectWidth,
                                           rectHeight)
  }

  fun requestViewLayout() {
    view.explicitlyDirtyBitmap()

    EmacsService.SERVICE!!.runOnUiThread(object : Runnable {
      override fun run() {
        if (overrideRedirect) {
          val params: WindowManager.LayoutParams

          /* Set the layout parameters again.  */
          params = getWindowLayoutParams()
          view.setLayoutParams(params)

          /* Announce this update to the window server.  */
          if (windowManager != null)
            windowManager!!.updateViewLayout(view, params)
        }

        view.mustReportLayout = true
        view.requestLayout()
      }
    })
  }

  @Synchronized
  fun resizeWindow(width: Int, height: Int) {
    rect.right = rect.left + width
    rect.bottom = rect.top + height

    requestViewLayout()
  }

  @Synchronized
  fun moveWindow(x: Int, y: Int) {
    val width: Int
    val height: Int

    width = rect.width()
    height = rect.height()

    rect.left = x
    rect.top = y
    rect.right = x + width
    rect.bottom = y + height

    requestViewLayout()
  }

  @Synchronized
  fun moveResizeWindow(x: Int, y: Int, width: Int, height: Int) {
    rect.left = x
    rect.top = y
    rect.right = x + width
    rect.bottom = y + height
    requestViewLayout()
  }

  /* Return WM layout parameters for an override redirect window with
     the geometry provided here.  */

  private fun getWindowLayoutParams(): WindowManager.LayoutParams {
    val params: WindowManager.LayoutParams
    var flags: Int
    val type: Int
    val rect: Rect

    flags = 0
    rect = getGeometry()
    flags = flags or WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE
    flags = flags or WindowManager.LayoutParams.FLAG_NOT_TOUCHABLE
    type = WindowManager.LayoutParams.TYPE_APPLICATION_PANEL

    params = WindowManager.LayoutParams(rect.width(), rect.height(),
                                        rect.left, rect.top,
                                        type, flags,
                                        PixelFormat.RGBA_8888)
    params.gravity = Gravity.TOP or Gravity.LEFT
    return params
  }

  private fun findSuitableActivityContext(): Activity? {
    /* Find a recently focused activity.  */
    if (!EmacsActivity.focusedActivities.isEmpty())
      return EmacsActivity.focusedActivities.get(0)

    /* Resort to the last activity to be focused.  */
    return EmacsActivity.lastFocusedActivity
  }

  @Synchronized
  fun mapWindow() {
    if (isMapped)
      return

    isMapped = true

    if (parent == null) {
      EmacsService.SERVICE!!.runOnUiThread(object : Runnable {
        override fun run() {
          /* Make the view visible, first of all.  */
          view.setVisibility(View.VISIBLE)

          if (!overrideRedirect) {
            val manager = EmacsWindowManager.MANAGER

            /* If parent is the root window, notice that there are new
               children available for interested activities to pick
               up.  */
            manager.registerWindow(this@EmacsWindow)

            if (!getDontFocusOnMap())
              /* Eventually this should check no-focus-on-map.  */
              view.requestFocus()
          } else {
            /* But if the window is an override-redirect window,
               then:

               - Find an activity that is currently active.

               - Map the window as a panel on top of that
                 activity using the system window manager.  */

            val ctx = findSuitableActivityContext()

            if (ctx == null) {
              Log.w(TAG, "failed to attach override-redirect window"
                         + " for want of activity")
              return
            }

            val tem = ctx.getSystemService(Context.WINDOW_SERVICE)
            val windowManager = tem as WindowManager

            /* Calculate layout parameters and propagate the
               activity's token into it.  */

            val params = getWindowLayoutParams()
            params.token = (ctx.findViewById<View>(android.R.id.content)
                            .getWindowToken())
            view.setLayoutParams(params)

            /* Attach the view.  */
            try {
              windowManager.addView(view, params)

              /* Record the window manager being used in the
                 EmacsWindow object.  */
              this@EmacsWindow.windowManager = windowManager
            } catch (e: Exception) {
              Log.w(TAG,
                    "failed to attach override-redirect window, " + e)
            }
          }
        }
      })
    } else {
      /* Do the same thing as above, but don't register this
         window.  */
      EmacsService.SERVICE!!.runOnUiThread(Runnable {
        view.setVisibility(View.VISIBLE)

        if (!getDontFocusOnMap())
          view.requestFocus()
      })
    }
  }

  @Synchronized
  fun unmapWindow() {
    if (!isMapped)
      return

    isMapped = false

    view.post(Runnable {
      val manager = EmacsWindowManager.MANAGER

      view.setVisibility(View.GONE)

      /* Detach the view from the window manager if possible.  */
      if (windowManager != null)
        windowManager!!.removeView(view)
      windowManager = null

      /* Now that the window is unmapped, unregister it as
         well.  */
      manager.detachWindow(this@EmacsWindow)
    })
  }

  override fun lockCanvas(gc: EmacsGC): Canvas? {
    /* `EmacsView.getCanvas` (EmacsView.java 254-255) returns null while the bitmap is not yet
       allocated, and the reference's callers test for null (EmacsFillRectangle.java 68 and the
       other draw requests).  The port asserted non-null here, so the null became a
       NullPointerException on the render thread the first time a window was cleared before its
       view had been laid out (measured on the device, status 4.901).  */
    return view.getCanvas(gc)
  }

  override fun damageRect(damageRect: Rect) {
    view.damageRect(damageRect.left,
                    damageRect.top,
                    damageRect.right,
                    damageRect.bottom)
  }

  override fun damageRect(left: Int, top: Int, right: Int, bottom: Int) {
    view.damageRect(left, top, right, bottom)
  }

  fun swapBuffers() {
    view.swapBuffers()
  }

  fun clearWindow() {
    EmacsService.SERVICE!!.fillRectangle(this, scratchGC,
                                       0, 0, rect.width(),
                                       rect.height())
  }

  fun clearArea(x: Int, y: Int, width: Int, height: Int) {
    EmacsService.SERVICE!!.fillRectangle(this, scratchGC,
                                       x, y, width, height)
  }

  override fun getBitmap(): Bitmap? {
    return view.getBitmap()
  }

  /* event.getCharacters is used because older input methods still
     require it.  */
  @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
  fun getEventUnicodeChar(event: KeyEvent, state: Int): Int {
    val characters: String?

    if (event.getUnicodeChar(state) != 0)
      return event.getUnicodeChar(state)

    characters = event.getCharacters()

    if (characters != null && characters.length == 1)
      return characters[0].code

    return if (characters == null) 0 else -1
  }

  fun saveUnicodeString(serial: Int, string: String) {
    eventStrings.put(serial, string)
  }

  /* event.getCharacters is used because older input methods still
     require it.  */
  @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
  fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
    var state: Int
    var state_1: Int
    var extra_ignored: Int
    var unicode_char: Int
    val serial: Long
    val characters: String?

    if (keyCode == KeyEvent.KEYCODE_BACK) {
      /* New Android systems display Back navigation buttons on a
         row of virtual buttons at the bottom of the screen.  These
         buttons function much as physical buttons do, in that key
         down events are produced when a finger taps them, even if
         the finger is not ultimately released after the OS's
         gesture navigation is activated.

         Deliver onKeyDown events in onKeyUp instead, so as not to
         navigate backwards during gesture navigation.  */

      return true
    }

    state = eventModifiers(event)

    /* Meta isn't supported by systems older than Android 3.0.  */

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB)
      extra_ignored = KeyEvent.META_META_MASK
    else
      extra_ignored = 0

    /* Ignore meta-state understood by Emacs for now, or key presses
       such as Ctrl+C and Meta+C will not be recognized as ASCII key
       press events.  */

    state_1 = state and ((KeyEvent.META_ALT_MASK or KeyEvent.META_CTRL_MASK
                          or KeyEvent.META_SYM_ON or extra_ignored).inv())

    /* There's no distinction between Right Alt and Alt Gr on Android,
       so restore META_ALT_RIGHT_ON if set in state to enable composing
       characters.  (bug#69321) */

    if ((state and KeyEvent.META_ALT_RIGHT_ON) != 0) {
      state_1 = state_1 or KeyEvent.META_ALT_ON or KeyEvent.META_ALT_RIGHT_ON

      /* If Alt is also not depressed, remove its bit from the mask
         reported to Emacs.  */
      if ((state and KeyEvent.META_ALT_LEFT_ON) == 0)
        state = state and (KeyEvent.META_ALT_MASK.inv())
    }

    unicode_char = getEventUnicodeChar(event, state_1)

    /* If a NUMPAD_ key is detected for which no character is returned,
       return false without sending the key event, as this will prompt
       the system to send an event with the corresponding action
       key.  */

    if (keyCode >= KeyEvent.KEYCODE_NUMPAD_0
        && keyCode <= KeyEvent.KEYCODE_NUMPAD_RIGHT_PAREN
        && unicode_char == 0)
      return false

    synchronized(eventStrings) {
      serial = EmacsNative.sendKeyPress(this.handle,
                                        event.getEventTime(),
                                        state, keyCode,
                                        unicode_char)

      characters = event.getCharacters()

      if (characters != null && characters.length > 1)
        saveUnicodeString(serial.toInt(), characters)
    }

    return true
  }

  fun onKeyUp(keyCode: Int, event: KeyEvent): Boolean {
    var state: Int
    var state_1: Int
    var unicode_char: Int
    var extra_ignored: Int
    val time: Long

    /* Compute the event's modifier mask.  */
    state = eventModifiers(event)

    /* Meta isn't supported by systems older than Android 3.0.  */

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB)
      extra_ignored = KeyEvent.META_META_MASK
    else
      extra_ignored = 0

    /* Ignore meta-state understood by Emacs for now, or key presses
       such as Ctrl+C and Meta+C will not be recognized as ASCII key
       press events.  */

    state_1 = state and ((KeyEvent.META_ALT_MASK or KeyEvent.META_CTRL_MASK
                          or KeyEvent.META_SYM_ON or extra_ignored).inv())

    /* There's no distinction between Right Alt and Alt Gr on Android,
       so restore META_ALT_RIGHT_ON if set in state to enable composing
       characters.  */

    if ((state and KeyEvent.META_ALT_RIGHT_ON) != 0) {
      state_1 = state_1 or KeyEvent.META_ALT_ON or KeyEvent.META_ALT_RIGHT_ON

      /* If Alt is also not depressed, remove its bit from the mask
         reported to Emacs.  */
      if ((state and KeyEvent.META_ALT_LEFT_ON) == 0)
        state = state and (KeyEvent.META_ALT_MASK.inv())
    }

    unicode_char = getEventUnicodeChar(event, state_1)

    if (keyCode == KeyEvent.KEYCODE_BACK) {
      /* If the key press's been canceled, return immediately.  */

      if ((event.getFlags() and KeyEvent.FLAG_CANCELED) != 0)
        return true

      /* Dispatch the key press event that was deferred till now.  */
      EmacsNative.sendKeyPress(this.handle, event.getEventTime(),
                               state, keyCode, unicode_char)
    }
    /* If a NUMPAD_ key is detected for which no character is returned,
       return false without sending the key event, as this will prompt
       the system to send an event with the corresponding action
       key.  */
    else if (keyCode >= KeyEvent.KEYCODE_NUMPAD_0
             && keyCode <= KeyEvent.KEYCODE_NUMPAD_RIGHT_PAREN
             && unicode_char == 0)
      return false

    EmacsNative.sendKeyRelease(this.handle, event.getEventTime(),
                               state, keyCode, unicode_char)

    if (keyCode == EmacsNative.getQuitKeycode()) {
      /* Check if this volume down press should quit Emacs.
         Most Android devices have no physical keyboard, so it
         is unreasonably hard to press C-g.  */

      time = event.getEventTime()

      if (time - lastQuitKeyRelease < 350)
        EmacsNative.quit()

      lastQuitKeyRelease = time
    }

    return true
  }

  fun onFocusChanged(gainFocus: Boolean) {
    EmacsActivity.invalidateFocus(if (gainFocus) 6 else 5)
  }

  /* Notice that the activity (or its task) has been detached or
     destroyed by explicit user action.  */

  fun onActivityDetached() {
    EmacsNative.sendWindowAction(this.handle, 0)
  }

  /* Dispatch a back gesture invocation as a KeyPress event.  Lamentably
     the platform does not appear to support reporting keyboard
     modifiers with these events.  */

  fun onBackInvoked() {
    val time = SystemClock.uptimeMillis()
    EmacsNative.sendKeyPress(this.handle, time, 0,
                             KeyEvent.KEYCODE_BACK, 0)
    EmacsNative.sendKeyRelease(this.handle, time, 0,
                               KeyEvent.KEYCODE_BACK, 0)
  }

  /* Look through the button state to determine what button EVENT was
     generated from.  DOWN is true if EVENT is a button press event,
     false otherwise.  Value is the X number of the button.  */

  private fun whatButtonWasIt(event: MotionEvent, down: Boolean): Int {
    var eventState: Int
    var notIn: Int

    /* Obtain the new button state.  */
    eventState = event.getButtonState()

    /* Compute which button is now set or no longer set.  */

    notIn = if (down) eventState and (lastButtonState.inv())
            else lastButtonState and (eventState.inv())

    if ((notIn and (MotionEvent.BUTTON_PRIMARY
                    or MotionEvent.BUTTON_SECONDARY
                    or MotionEvent.BUTTON_TERTIARY)) == 0)
      /* No buttons have been pressed, so this is a touch event.  */
      return 0

    if ((notIn and MotionEvent.BUTTON_PRIMARY) != 0)
      return 1

    if ((notIn and MotionEvent.BUTTON_SECONDARY) != 0)
      return 3

    if ((notIn and MotionEvent.BUTTON_TERTIARY) != 0)
      return 2

    /* Buttons 4, 5, 6 and 7 are actually scroll wheels under X.
       Thus, report additional buttons starting at 8.  */

    if ((notIn and MotionEvent.BUTTON_BACK) != 0)
      return 8

    if ((notIn and MotionEvent.BUTTON_FORWARD) != 0)
      return 9

    /* Report stylus events as touch screen events.  */

    if ((notIn and MotionEvent.BUTTON_STYLUS_PRIMARY) != 0)
      return 0

    if ((notIn and MotionEvent.BUTTON_STYLUS_SECONDARY) != 0)
      return 0

    /* Not a real value.  */
    return 11
  }

  /* Return the mouse button associated with the specified ACTION_DOWN
     or ACTION_POINTER_DOWN EVENT.

     Value is 0 if no mouse button was pressed, or the X number of
     that mouse button.  */

  private fun buttonForEvent(event: MotionEvent): Int {
    /* ICS and earlier don't support true mouse button events, so
       treat all down events as touch screen events.  */

    if (Build.VERSION.SDK_INT
        < Build.VERSION_CODES.ICE_CREAM_SANDWICH)
      return 0

    return whatButtonWasIt(event, true)
  }

  /* Return the coordinate object associated with the specified
     EVENT, or null if it is not known.  */

  private fun figureChange(event: MotionEvent): Coordinate? {
    var coordinate: Coordinate?

    /* Initialize this variable now.  */
    coordinate = null

    when (event.getActionMasked()) {
      MotionEvent.ACTION_DOWN -> {
        /* Primary pointer pressed with index 0.  */

        val pointerID = event.getPointerId(0)
        coordinate = Coordinate(event.getX(0).toInt(),
                                event.getY(0).toInt(),
                                buttonForEvent(event),
                                pointerID)
        pointerMap.put(pointerID, coordinate)
      }

      MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> {
        /* Primary pointer released with index 0.  */
        val pointerID = event.getPointerId(0)
        coordinate = pointerMap.get(pointerID)
        pointerMap.delete(pointerID)
      }

      MotionEvent.ACTION_POINTER_DOWN -> {
        /* New pointer.  Find the pointer ID from the index and place
           it in the map.  */
        val pointerIndex = event.getActionIndex()
        val pointerID = event.getPointerId(pointerIndex)
        coordinate = Coordinate(event.getX(pointerIndex).toInt(),
                                event.getY(pointerIndex).toInt(),
                                buttonForEvent(event),
                                pointerID)
        pointerMap.put(pointerID, coordinate)
      }

      MotionEvent.ACTION_POINTER_UP -> {
        /* Pointer removed.  Remove it from the map.  */
        val pointerIndex = event.getActionIndex()
        val pointerID = event.getPointerId(pointerIndex)
        coordinate = pointerMap.get(pointerID)
        pointerMap.delete(pointerID)
      }

      else -> {
        /* Loop through each pointer in the event.  */

        val count = event.getPointerCount()
        var i = 0
        while (i < count) {
          val pointerID = event.getPointerId(i)

          /* Look up that pointer in the map.  */
          coordinate = pointerMap.get(pointerID)

          if (coordinate != null) {
            /* See if coordinates have changed.  */
            val truncatedX = event.getX(i).toInt()
            val truncatedY = event.getY(i).toInt()

            if (truncatedX != coordinate.x
                || truncatedY != coordinate.y) {
              /* The pointer changed.  Update the coordinate and
                 break out of the loop.  */
              coordinate.x = truncatedX
              coordinate.y = truncatedY

              break
            }
          }

          i++
        }

        /* Set coordinate to NULL if the loop failed to find any
           matching pointer.  */

        if (i == count)
          coordinate = null
      }
    }

    /* Return the pointer ID.  */
    return coordinate
  }

  /* Return the modifier mask associated with the specified motion
     EVENT.  Replace bits corresponding to Left or Right keys with
     their corresponding general modifier bits.  */

  private fun motionEventModifiers(event: MotionEvent): Int {
    var state: Int

    state = event.getMetaState()

    /* Normalize the state by setting the generic modifier bit if
       either a left or right modifier is pressed.  */

    if ((state and KeyEvent.META_ALT_LEFT_ON) != 0
        || (state and KeyEvent.META_ALT_RIGHT_ON) != 0)
      state = state or KeyEvent.META_ALT_MASK

    if ((state and KeyEvent.META_CTRL_LEFT_ON) != 0
        || (state and KeyEvent.META_CTRL_RIGHT_ON) != 0)
      state = state or KeyEvent.META_CTRL_MASK

    return state
  }

  /* Process a single ACTION_DOWN, ACTION_POINTER_DOWN, ACTION_UP,
     ACTION_POINTER_UP, ACTION_CANCEL, or ACTION_MOVE event.

     Ascertain which coordinate changed and send an appropriate mouse
     or touch screen event.  */

  private fun motionEvent(event: MotionEvent) {
    val coordinate: Coordinate?
    var modifiers: Int
    val time: Long

    /* Find data associated with this event's pointer.  Namely, its
       current location, whether or not a change has taken place, and
       whether or not it is a button event.  */

    coordinate = figureChange(event)

    if (coordinate == null)
      return

    time = event.getEventTime()

    if (coordinate.button != 0) {
      /* This event is tied to a mouse click, so report mouse motion
         and button events.  */

      modifiers = motionEventModifiers(event)

      when (event.getAction()) {
        MotionEvent.ACTION_POINTER_DOWN, MotionEvent.ACTION_DOWN ->
          EmacsNative.sendButtonPress(this.handle, coordinate.x,
                                      coordinate.y, time, modifiers,
                                      coordinate.button)

        MotionEvent.ACTION_POINTER_UP, MotionEvent.ACTION_UP,
        MotionEvent.ACTION_CANCEL ->
          EmacsNative.sendButtonRelease(this.handle, coordinate.x,
                                        coordinate.y, time, modifiers,
                                        coordinate.button)

        MotionEvent.ACTION_MOVE ->
          EmacsNative.sendMotionNotify(this.handle, coordinate.x,
                                       coordinate.y, time)
      }
    } else {
      /* This event is a touch event, and the touch ID is the
         pointer ID.  */

      when (event.getActionMasked()) {
        MotionEvent.ACTION_DOWN, MotionEvent.ACTION_POINTER_DOWN ->
          /* Touch down event.  */
          EmacsNative.sendTouchDown(this.handle, coordinate.x,
                                    coordinate.y, time,
                                    coordinate.id, 0)

        MotionEvent.ACTION_UP, MotionEvent.ACTION_POINTER_UP ->
          /* Touch up event.  */
          EmacsNative.sendTouchUp(this.handle, coordinate.x,
                                  coordinate.y, time,
                                  coordinate.id, 0)

        MotionEvent.ACTION_CANCEL ->
          /* Touch sequence cancellation event.  */
          EmacsNative.sendTouchUp(this.handle, coordinate.x,
                                  coordinate.y, time,
                                  coordinate.id,
                                  1 /* ANDROID_TOUCH_SEQUENCE_CANCELED */)

        MotionEvent.ACTION_MOVE ->
          /* Pointer motion event.  */
          EmacsNative.sendTouchMove(this.handle, coordinate.x,
                                    coordinate.y, time,
                                    coordinate.id, 0)
      }
    }

    if (Build.VERSION.SDK_INT
        < Build.VERSION_CODES.ICE_CREAM_SANDWICH)
      return

    /* Now update the button state.  */
    lastButtonState = event.getButtonState()
  }

  fun onTouchEvent(event: MotionEvent): Boolean {
    when (event.getActionMasked()) {
      MotionEvent.ACTION_DOWN,
      MotionEvent.ACTION_POINTER_DOWN,
      MotionEvent.ACTION_UP,
      MotionEvent.ACTION_POINTER_UP,
      MotionEvent.ACTION_CANCEL,
      MotionEvent.ACTION_MOVE -> {
        motionEvent(event)
        return true
      }
    }

    return false
  }

  fun onGenericMotionEvent(event: MotionEvent): Boolean {
    when (event.getAction()) {
      MotionEvent.ACTION_HOVER_ENTER -> {
        EmacsNative.sendEnterNotify(this.handle, event.getX().toInt(),
                                    event.getY().toInt(),
                                    event.getEventTime())
        return true
      }

      MotionEvent.ACTION_HOVER_MOVE -> {
        EmacsNative.sendMotionNotify(this.handle, event.getX().toInt(),
                                     event.getY().toInt(),
                                     event.getEventTime())
        return true
      }

      MotionEvent.ACTION_HOVER_EXIT -> {
        /* If the exit event comes from a button press, its button
           state will have extra bits compared to the last known
           button state.  Since the exit event will interfere with
           tool bar button presses, ignore such splurious events.  */

        if ((event.getButtonState() and (lastButtonState.inv())) == 0)
          EmacsNative.sendLeaveNotify(this.handle, event.getX().toInt(),
                                      event.getY().toInt(),
                                      event.getEventTime())

        return true
      }

      MotionEvent.ACTION_DOWN,
      MotionEvent.ACTION_POINTER_DOWN,
      MotionEvent.ACTION_UP,
      MotionEvent.ACTION_POINTER_UP,
      MotionEvent.ACTION_CANCEL,
      MotionEvent.ACTION_MOVE -> {
        /* MotionEvents may either be sent to onGenericMotionEvent or
           onTouchEvent depending on if Android thinks it is a mouse
           event or not, but we detect them ourselves.  */
        motionEvent(event)
        return true
      }

      MotionEvent.ACTION_SCROLL -> {
        /* Send a scroll event with the specified deltas.  */
        EmacsNative.sendWheel(this.handle, event.getX().toInt(),
                              event.getY().toInt(),
                              event.getEventTime(),
                              motionEventModifiers(event),
                              event.getAxisValue(MotionEvent.AXIS_HSCROLL),
                              event.getAxisValue(MotionEvent.AXIS_VSCROLL))
        return true
      }
    }

    return false
  }

  @Synchronized
  fun reparentTo(otherWindow: EmacsWindow?, x: Int, y: Int) {
    val width: Int
    val height: Int

    /* Reparent this window to the other window.  */

    if (parent != null) {
      synchronized(parent!!.children) {
        parent!!.children.remove(this)
      }
    }

    if (otherWindow != null) {
      synchronized(otherWindow.children) {
        otherWindow.children.add(this)
      }
    }

    parent = otherWindow

    /* Move this window to the new location.  */
    width = rect.width()
    height = rect.height()
    rect.left = x
    rect.top = y
    rect.right = x + width
    rect.bottom = y + height

    /* Now do the work necessary on the UI thread to reparent the
       window.  */
    EmacsService.SERVICE!!.runOnUiThread(object : Runnable {
      override fun run() {
        val manager: EmacsWindowManager
        val parent: ViewManager?

        /* Invalidate the focus; this should transfer the input focus
           to the next eligible window as this window is no longer
           present in parent.children.  */
        EmacsActivity.invalidateFocus(7)

        /* First, detach this window if necessary.  */
        manager = EmacsWindowManager.MANAGER
        manager.detachWindow(this@EmacsWindow)

        /* Also unparent this view.  */

        /* If the window manager is set, use that instead.  */
        if (windowManager != null)
          parent = windowManager
        else
          parent = view.getParent() as ViewManager?
        windowManager = null

        if (parent != null)
          parent.removeView(view)

        /* Next, either add this window as a child of the new
           parent's view, or make it available again.  */
        if (otherWindow != null)
          otherWindow.view.addView(view)
        else if (this@EmacsWindow.isMapped)
          manager.registerWindow(this@EmacsWindow)

        /* Request relayout.  */
        view.requestLayout()
      }
    })
  }

  fun makeInputFocus(time: Long) {
    /* TIME is currently ignored.  Request the input focus now.  */

    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      view.requestFocus()
    })
  }

  @Synchronized
  fun raise() {
    /* This does nothing here.  */
    if (parent == null)
      return

    synchronized(parent!!.children) {
      /* Remove and add this view again.  */
      parent!!.children.remove(this)
      parent!!.children.add(this)
    }

    /* Request a relayout.  */
    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      view.raise()
    })
  }

  @Synchronized
  fun lower() {
    /* This does nothing here.  */
    if (parent == null)
      return

    synchronized(parent!!.children) {
      /* Remove and add this view again.  */
      parent!!.children.remove(this)
      parent!!.children.add(this)
    }

    /* Request a relayout.  */
    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      view.lower()
    })
  }

  @Synchronized
  fun reconfigure(window: EmacsWindow?, stackMode: Int) {
    val iterator: MutableListIterator<EmacsWindow>

    /* This does nothing here.  */
    if (parent == null)
      return

    /* If window is NULL, call lower or upper subject to
       stackMode.  */

    if (window == null) {
      if (stackMode == 1) /* ANDROID_BELOW */
        lower()
      else
        raise()

      return
    }

    /* Otherwise, if window.parent is distinct from this, return.  */
    if (window.parent !== this.parent)
      return

    /* Synchronize with the parent's child list.  Iterate over each
       item until WINDOW is encountered, before moving this window to
       the location prescribed by STACKMODE.  */

    synchronized(parent!!.children) {
      /* Remove this window from parent.children, for it will be
         reinserted before or after WINDOW.  */
      parent!!.children.remove(this)

      /* Create an iterator.  */
      iterator = parent!!.children.listIterator()

      while (iterator.hasNext()) {
        val current = iterator.next()

        if (current === window) {
          /* Now place this before or after the cursor of the
             iterator.  */

          if (stackMode == 0) /* ANDROID_ABOVE */
            iterator.add(this)
          else {
            iterator.previous()
            iterator.add(this)
          }

          /* Effect the same adjustment upon the view
             hierarchy.  */

          EmacsService.SERVICE!!.runOnUiThread(Runnable {
            if (stackMode == 0)
              view.moveAbove(window.view)
            else
              view.moveBelow(window.view)
          })
        }
      }

      /* parent.children does not list WINDOW, which should never
         transpire.  */
      EmacsNative.emacsAbort()
    }
  }

  @Synchronized
  fun getWindowGeometry(): IntArray {
    val array: IntArray

    array = IntArray(4)

    array[0] = if (parent != null) rect.left else xPosition
    array[1] = if (parent != null) rect.top else yPosition
    array[2] = rect.width()
    array[3] = rect.height()

    return array
  }

  fun noticeIconified() {
    EmacsNative.sendIconified(this.handle)
  }

  fun noticeDeiconified() {
    EmacsNative.sendDeiconified(this.handle)
  }

  @Synchronized
  fun setDontAcceptFocus(dontAcceptFocus: Boolean) {
    /* Update the view's focus state.  */
    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      view.setFocusable(!dontAcceptFocus)
      view.setFocusableInTouchMode(!dontAcceptFocus)
    })
  }

  @Synchronized
  fun setDontFocusOnMap(dontFocusOnMap: Boolean) {
    this.dontFocusOnMap = dontFocusOnMap
  }

  @Synchronized
  fun getDontFocusOnMap(): Boolean {
    return dontFocusOnMap
  }

  fun setWmName(wmName: String) {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP)
      return

    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      this@EmacsWindow.wmName = wmName

      /* If an activity is already attached, replace its task
         description.  */

      val tem = getAttachedConsumer()

      if (tem is EmacsActivity)
        tem.updateWmName()
    })
  }

  fun translateCoordinates(x: Int, y: Int): IntArray {
    val array: IntArray

    /* This is supposed to translate coordinates to the root window,
       whose origin point, in this context, is that of the toplevel
       activity host to this view.  */
    array = IntArray(2)
    EmacsService.SERVICE!!.getLocationInWindow(view, array)

    /* Now, the coordinates of the view should be in array.  Offset X
       and Y by them.  */
    array[0] += x
    array[1] += y

    /* In the case of an override redirect window, the WM window's
       extents and position match the Emacs window exactly.  */

    if (overrideRedirect) {
      synchronized(this) {
        array[0] += rect.left
        array[1] += rect.top
      }
    }

    /* Return the resulting coordinates.  */
    return array
  }

  fun toggleOnScreenKeyboard(on: Boolean) {
    val task: FutureTask<Void>

    /* Even though InputMethodManager functions are thread safe,
       `showOnScreenKeyboard' etc must be called from the UI thread in
       order to avoid deadlocks if the calls happen in tandem with a
       call to a synchronizing function within
       `onCreateInputConnection'.  */

    task = FutureTask<Void>(Callable<Void> {
      if (on)
        view.showOnScreenKeyboard()
      else
        view.hideOnScreenKeyboard()
      null
    })

    /* Block Lisp until this request to display the on-screen keyboard
       is registered by the UI thread, or updates arising from a
       redisplay that are reported between the two events will be liable
       to run afoul of the IMM's cache of selection positions and never
       reach the input method, if it is currently hidden, as input
       methods receive outdated selection information reported during
       the previous call to `onCreateInputConnection' when first
       displayed.

       Chances are this is a long-standing bug in the system.  */
    EmacsService.syncRunnable(task)
  }

  fun lookupString(eventSerial: Int): String? {
    val any: String?

    synchronized(eventStrings) {
      any = eventStrings.remove(eventSerial)
    }

    return any
  }

  fun setFullscreen(isFullscreen: Boolean) {
    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      fullscreen = isFullscreen

      val tem = getAttachedConsumer()

      if (tem is EmacsActivity)
        tem.syncFullscreenWith(this@EmacsWindow)
    })
  }

  fun defineCursor(cursor: EmacsCursor?) {
    /* Don't post this message if pointer icons aren't supported.  */

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N)
      view.post(Runnable {
        if (cursor != null)
          view.setPointerIcon(cursor.icon)
        else
          view.setPointerIcon(null)
      })
  }

  @Synchronized
  fun notifyContentRectPosition(xPosition: Int, yPosition: Int) {
    /* Ignore these notifications if not a child of the root
       window.  */
    if (parent != null)
      return

    /* xPosition and yPosition are the position of this window
       relative to the screen.  Set them and request a ConfigureNotify
       event.  */

    if (this.xPosition != xPosition
        || this.yPosition != yPosition) {
      this.xPosition = xPosition
      this.yPosition = yPosition

      EmacsNative.sendConfigureNotify(this.handle,
                                      System.currentTimeMillis(),
                                      xPosition, yPosition,
                                      rect.width(), rect.height())
    }
  }

  /* Register the drag and drop event EVENT.  */

  fun onDragEvent(event: DragEvent): Boolean {
    val x = event.getX().toInt()
    val y = event.getY().toInt()

    when (event.getAction()) {
      DragEvent.ACTION_DRAG_STARTED ->
        /* Return true to continue the drag and drop operation.  */
        return true

      DragEvent.ACTION_DRAG_LOCATION -> {
        /* Send this drag motion event to Emacs.  Skip this when the
           integer position hasn't changed, for Android sends events
           even if the movement from the previous position of the drag
           is less than 1 pixel on either axis.  */

        if (x != dndXPosition || y != dndYPosition) {
          EmacsNative.sendDndDrag(handle, x, y)
          dndXPosition = x
          dndYPosition = y
        }

        return true
      }

      DragEvent.ACTION_DROP -> {
        /* Reset this view's record of the previous drag and drop
           event's position.  */
        dndXPosition = -1
        dndYPosition = -1

        /* Judge whether this is plain text, or if it's a file URI for
           which permissions must be requested.  */

        val data = event.getClipData()
        val description = data.getDescription()
        val itemCount = data.getItemCount()

        /* If there are insufficient items within the clip data,
           return false.  */

        if (itemCount < 1)
          return false

        var type: String?

        /* Search for plain text data within the clipboard.  */

        for (i in 0 until description.getMimeTypeCount()) {
          type = description.getMimeType(i)

          if (type == ClipDescription.MIMETYPE_TEXT_PLAIN
              || type == ClipDescription.MIMETYPE_TEXT_HTML) {
            /* The data being dropped is plain text; encode it
               suitably and send it to the main thread.  */
            type = (data.getItemAt(0).coerceToText(EmacsService.SERVICE)
                    .toString())
            EmacsNative.sendDndText(handle, x, y, type)
            return true
          } else if (type == ClipDescription.MIMETYPE_TEXT_URILIST) {
            /* The data being dropped is a list of URIs; encode it
               suitably and send it to the main thread.  */
            type = (data.getItemAt(0).coerceToText(EmacsService.SERVICE)
                    .toString())
            EmacsNative.sendDndUri(handle, x, y, type)
            return true
          }
        }

        /* There's no plain text data within this clipboard item, so
           each item within should be treated as a content URI
           designating a file.  */

        /* Collect the URIs into a string with each suffixed
           by newlines, much as in a text/uri-list.  */
        val builder = StringBuilder()

        for (i in 0 until itemCount) {
          /* If the item dropped is a URI, send it to the
             main thread.  */

          var uri = data.getItemAt(i).getUri()

          /* Attempt to acquire permissions for this URI;
             failing which, insert it as text instead.  */

          val activity = EmacsActivity.lastFocusedActivity
          if (uri != null
              && uri.getScheme() != null
              && uri.getScheme() == "content"
              && activity != null) {
            if ((activity.requestDragAndDropPermissions(event) == null))
              uri = null
            else {
              val resolver = activity.getContentResolver()

              /* Substitute a content file name for the URI, if
                 possible.  */
              val uriString = EmacsService.buildContentName(uri, resolver)

              if (uriString != null) {
                builder.append(uriString).append("\n")
                continue
              }
            }
          }

          if (uri != null)
            builder.append(uri.toString()).append("\n")
          else {
            /* Treat each URI that Emacs cannot secure
               permissions for as plain text.  */
            type = (data.getItemAt(i)
                    .coerceToText(EmacsService.SERVICE)
                    .toString())
            EmacsNative.sendDndText(handle, x, y, type)
          }
        }

        /* Now send each URI to Emacs.  */

        if (builder.length > 0)
          EmacsNative.sendDndUri(handle, x, y, builder.toString())
        return true
      }

      else -> {
        /* Reset this view's record of the previous drag and drop
           event's position.  */
        dndXPosition = -1
        dndYPosition = -1
      }
    }

    return true
  }

  /* Recreate the activity to which this window is attached, if any.
     This is nonfunctional on Android 2.3.7 and earlier.  */

  fun recreateActivity() {
    val attached: EmacsWindowManager.WindowConsumer?

    attached = this.attached

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.HONEYCOMB)
      return

    view.post(Runnable {
      if (attached is EmacsActivity)
        attached.recreate()
    })
  }
}
