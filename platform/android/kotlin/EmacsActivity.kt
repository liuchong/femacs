// femacs: port of java/org/gnu/emacs/EmacsActivity.java @ fd7afced403.

package org.femacs

import java.util.ArrayList
import java.util.concurrent.TimeUnit

import android.app.Activity
import android.app.ActivityManager.TaskDescription

import android.content.ContentResolver
import android.content.Intent

import android.os.Build
import android.os.Bundle
import android.os.SystemClock

import android.util.Log

import android.net.Uri

import android.view.Menu
import android.view.View
import android.view.ViewTreeObserver
import android.view.Window
import android.view.WindowInsets
import android.view.WindowInsetsController

import android.widget.FrameLayout

import android.window.BackEvent
import android.window.OnBackAnimationCallback
import android.window.OnBackInvokedCallback
import android.window.OnBackInvokedDispatcher

/* `android.c` never resolves this class by name - the activity is reached through
   `EmacsWindowManager` and the manifest, and its statics are read by the already-ported Kotlin
   targets (`EmacsDialog.kt` 267/282/290 reads `focusedActivities` and `lastFocusedActivity`, and
   `EmacsOpenActivity.kt`/`EmacsPreferencesActivity.kt` read `EXTRA_STARTUP_ARGUMENTS`), so the
   reference's `public static` fields become companion `@JvmField`s (P-83 ②, the
   `EmacsApplication.dumpFileName` shape) and its two static methods become `@JvmStatic`
   `invalidateFocus`/`invalidateFocus1` - `EmacsWindow.java` 294/892/1411 calls
   `EmacsActivity.invalidateFocus (n)` as a static.

   The class is *not* `final` in the reference (`EmacsMultitaskActivity` extends it), so the port
   is `open` and its two non-final overrides are `open override`; every other override is `final`
   in the reference, so the port writes `final override` (P-83 ⑦⑫).  The reference's
   `@SuppressWarnings ("deprecation")` on `syncFullscreenWith` and `updateWmName` becomes
   `@Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")` (P-83 ④), and the unused local `int hours;`
   in `isReallyFinishing` is not carried (Kotlin warns on an unused local).  */

open class EmacsActivity : Activity(),
                           EmacsWindowManager.WindowConsumer,
                           ViewTreeObserver.OnGlobalLayoutListener {
  companion object {
    private const val TAG = "EmacsActivity"

    /* Key of intent value providing extra startup argument.  */
    @JvmField val EXTRA_STARTUP_ARGUMENTS: String = "org.femacs.STARTUP_ARGUMENTS"

    /* ID for URIs from a granted document tree.  */
    @JvmField val ACCEPT_DOCUMENT_TREE: Int = 1

    /* List of activities with focus.  */
    @JvmField val focusedActivities: MutableList<EmacsActivity>
      = ArrayList<EmacsActivity>()

    /* The last activity to have been focused.  */
    @JvmField var lastFocusedActivity: EmacsActivity? = null

    /* The currently focused window.  */
    @JvmField var focusedWindow: EmacsWindow? = null

    /* The last context menu to be closed.  */
    @JvmField var lastClosedMenu: Menu? = null

    /* The time of the most recent call to onStop.  */
    @JvmField var timeOfLastInteraction: Long = 0

    @JvmStatic
    fun invalidateFocus1(window: EmacsWindow, resetWhenChildless: Boolean) {
      if (window.view.isFocused())
        focusedWindow = window

      synchronized(window.children) {
        for (child in window.children)
          invalidateFocus1(child, false)

        /* If no focused window was previously detected among WINDOW's
           children and RESETWHENCHILDLESS is set (implying it is a
           toplevel window), request that it be focused, to avoid
           creating a situation where no windows exist focused or can be
           transferred the input focus by user action.  */
        if (focusedWindow == null && resetWhenChildless) {
          window.view.requestFocus()
          focusedWindow = window
        }
      }
    }

    @JvmStatic
    fun invalidateFocus(whence: Int) {
      val oldFocus: EmacsWindow?

      /* Walk through each focused activity and assign the window focus
         to the bottom-most focused window within.  Record the old focus
         as well.  */
      oldFocus = focusedWindow
      focusedWindow = null

      for (activity in focusedActivities) {
        if (activity.window != null)
          invalidateFocus1(activity.window!!, focusedWindow == null)
      }

      /* Send focus in- and out- events to the previous and current
         focus.  */

      if (oldFocus != null)
        EmacsNative.sendFocusOut(oldFocus.handle,
                                 System.currentTimeMillis())

      if (focusedWindow != null)
        EmacsNative.sendFocusIn(focusedWindow!!.handle,
                                System.currentTimeMillis())
    }
  }

  /* The currently attached EmacsWindow, or null if none.  */
  private var window: EmacsWindow? = null

  /* The frame layout associated with the activity.  */
  private var layout: FrameLayout? = null

  /* Whether or not this activity is paused.  */
  private var isStopped: Boolean = false

  /* Whether or not this activity is fullscreen.  */
  private var isFullscreen: Boolean = false

  final override fun detachWindow() {
    syncFullscreenWith(null)

    if (window == null)
      Log.w(TAG, "detachWindow called, but there is no window")
    else {
      /* Clear the window's pointer to this activity and remove the
         window's view.  */
      window!!.setConsumer(null)

      /* The window can't be iconified any longer.  */
      window!!.noticeDeiconified()
      layout!!.removeView(window!!.view)
      window = null

      /* Reset the WM name.  */
      updateWmName()

      invalidateFocus(0)
    }
  }

  final override fun attachWindow(child: EmacsWindow) {
    val defaultParams: FrameLayout.LayoutParams

    if (window != null)
      throw IllegalStateException("trying to attach window when one"
                                  + " already exists")

    syncFullscreenWith(child)

    /* Record and attach the view.  */

    /* Reset residual LayoutParams that might remain in effect on this
       window, or some distributions of Android (e.g. Huawei HarmonyOS
       4.2) will retain the size of this window as a child frame.  */
    defaultParams = FrameLayout.LayoutParams(FrameLayout.LayoutParams.MATCH_PARENT,
                                             FrameLayout.LayoutParams.MATCH_PARENT)
    syncFullscreenWith(child)
    window = child
    layout!!.addView(window!!.view, defaultParams)
    child.setConsumer(this)

    /* If the window isn't no-focus-on-map, focus its view.  */
    if (!child.getDontFocusOnMap())
      window!!.view.requestFocus()

    /* If the activity is iconified, send that to the window.  */
    if (isStopped)
      window!!.noticeIconified()

    /* Invalidate the focus.  Since attachWindow may be called from
       either the main or the UI thread, post this to the UI thread.  */

    runOnUiThread(Runnable {
      invalidateFocus(1)
    })

    /* Synchronize the window's window manager name with this activity's
       task in the recents list.  */
    updateWmName()
  }

  final override fun destroy() {
    if (window != null) {
      /* Clear the window's pointer to this activity and remove the
         window's view.  */
      window!!.setConsumer(null)

      /* The window can't be iconified any longer.  */
      window!!.noticeDeiconified()
      layout!!.removeView(window!!.view)
      window = null
    }

    finish()
  }

  final override fun getAttachedWindow(): EmacsWindow? {
    return window
  }

  private fun interceptBackGesture() {
    val dispatcher: OnBackInvokedDispatcher
    val priority: Int = OnBackInvokedDispatcher.PRIORITY_DEFAULT
    val callback: OnBackInvokedCallback

    dispatcher = getOnBackInvokedDispatcher()
    callback = object : OnBackAnimationCallback {
      override fun onBackInvoked() {
        val view = this@EmacsActivity.getCurrentFocus()
        var emacsWindow: EmacsWindow

        if (view is EmacsView) {
          emacsWindow = view.window
          emacsWindow.onBackInvoked()
        }
      }

      /* The three functions are overridden to prevent a misleading
         back animation from being displayed, as Emacs intercepts all
         back gestures and will not return to the home screen.  */

      override fun onBackCancelled() {
      }

      override fun onBackProgressed(gestureEvent: BackEvent) {
      }

      override fun onBackStarted(gestureEvent: BackEvent) {
      }
    }
    dispatcher.registerOnBackInvokedCallback(priority, callback)
  }

  override fun onCreate(savedInstanceState: Bundle?) {
    val params: FrameLayout.LayoutParams
    val intent: Intent
    var decorView: View
    var observer: ViewTreeObserver
    val matchParent: Int

    /* See if Emacs should be started with any extra arguments, such
       as `--quick'.  */
    intent = getIntent()
    EmacsService.extraStartupArguments = intent.getStringArrayExtra(EXTRA_STARTUP_ARGUMENTS)

    matchParent = FrameLayout.LayoutParams.MATCH_PARENT
    params = FrameLayout.LayoutParams(matchParent,
                                      matchParent)

    /* Make the frame layout.  */
    layout = FrameLayout(this)
    layout!!.setLayoutParams(params)

    /* Set it as the content view.  */
    setContentView(layout)

    /* Android 15 also realigns activity contents to originate beneath
       system windows, e.g. the navigation bar, so request the original
       behavior.  */
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.VANILLA_ICE_CREAM)
      layout!!.setFitsSystemWindows(true)

    /* Android 16 replaces KEYCODE_BACK with a callback registered at
       the window level.  */
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.BAKLAVA)
      interceptBackGesture()

    /* Maybe start the Emacs service if necessary.  */
    EmacsService.startEmacsService(this)

    /* Add this activity to the list of available activities.  */
    EmacsWindowManager.MANAGER.registerWindowConsumer(this)

    /* Start observing global layout changes between Jelly Bean and Q.
       This is required to restore the fullscreen state whenever the
       on screen keyboard is displayed, as there is otherwise no way
       to determine when the on screen keyboard becomes visible.  */

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) {
      decorView = getWindow().getDecorView()
      observer = decorView.getViewTreeObserver()
      observer.addOnGlobalLayoutListener(this)
    }

    super.onCreate(savedInstanceState)

    /* Call `onWindowFocusChanged' to read the focus state, which fails
       to be called after an activity is recreated.  */
    onWindowFocusChanged(false)
  }

  final override fun onGlobalLayout() {
    syncFullscreenWith(window)
  }

  final override fun onStop() {
    /* Iconification was previously reported in onPause, but that was
       misinformed, as `onStop' is the actual callback activated upon
       changes in an activity's visibility.  */
    isStopped = true
    EmacsWindowManager.MANAGER.noticeIconified(this)

    timeOfLastInteraction = SystemClock.elapsedRealtime()
    super.onStop()
  }

  /* Return whether the task is being finished in response to explicit
     user action.  That is to say, Activity.isFinished, but as
     documented.  */

  fun isReallyFinishing(): Boolean {
    val atime: Long
    val dtime: Long

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N)
      return isFinishing()

    /* When the number of tasks retained in the recents list exceeds a
       threshold, Android 7 and later so destroy activities in trimming
       them from recents on the expiry of a timeout that isFinishing
       returns true, in direct contradiction to the documentation.  This
       timeout is generally 6 hours, but admits of customization by
       individual system distributors, so to err on the side of the
       caution, the timeout Emacs applies is a more conservative figure
       of 4 hours.  */

    if (timeOfLastInteraction == 0L)
      return isFinishing()

    atime = timeOfLastInteraction

    /* Compare atime with the current system time.  */
    dtime = SystemClock.elapsedRealtime() - atime
    if (dtime + 1000000 < TimeUnit.HOURS.toMillis(4))
      return isFinishing()

    return false
  }

  final override fun onDestroy() {
    val manager: EmacsWindowManager
    val isMultitask: Boolean
    val reallyFinishing: Boolean

    manager = EmacsWindowManager.MANAGER

    /* The activity will die shortly hereafter.  If there is a window
       attached, close it now.  */
    isMultitask = this is EmacsMultitaskActivity
    reallyFinishing = isReallyFinishing()
    manager.removeWindowConsumer(this, isMultitask || reallyFinishing)
    focusedActivities.remove(this)
    invalidateFocus(2)

    /* Remove this activity from the static field, lest it leak.  */
    if (lastFocusedActivity === this)
      lastFocusedActivity = null

    super.onDestroy()
  }

  final override fun onWindowFocusChanged(isFocused: Boolean) {
    /* The reference reassigns its `isFocused' parameter below (P-83 ④); Kotlin parameters are
       immutable, so the value is carried into a local of the same name.  */
    var isFocused: Boolean = isFocused

    /* At times and on certain versions of Android ISFOCUSED does not
       reflect whether the window actually holds focus, so replace it
       with the value of `hasWindowFocus'.  */
    isFocused = hasWindowFocus()

    if (isFocused) {
      if (!focusedActivities.contains(this))
        focusedActivities.add(this)

      lastFocusedActivity = this

      /* Update the window insets as the focus change may have
         changed the window insets as well, and the system does not
         automatically restore visibility flags.  */

      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R
          && isFullscreen)
        syncFullscreenWith(window)
    } else
      focusedActivities.remove(this)

    invalidateFocus(3)
  }

  final override fun onResume() {
    isStopped = false
    timeOfLastInteraction = 0

    EmacsWindowManager.MANAGER.noticeDeiconified(this)
    super.onResume()
  }

  final override fun onContextMenuClosed(menu: Menu) {
    val serial: Int

    /* See the comment inside onMenuItemClick.  */

    if (((EmacsContextMenu.wasSubmenuSelected == (-2).toLong())
         || (EmacsContextMenu.wasSubmenuSelected >= 0
             && ((System.currentTimeMillis()
                  - EmacsContextMenu.wasSubmenuSelected)
                 <= 300)))
        || menu === lastClosedMenu) {
      EmacsContextMenu.wasSubmenuSelected = -1
      lastClosedMenu = menu
      return
    }

    /* lastClosedMenu is set because Android apparently calls this
       function twice.  */

    lastClosedMenu = null

    /* Send a context menu event given that no menu item has already
       been selected.  */
    if (!EmacsContextMenu.itemAlreadySelected) {
      serial = EmacsContextMenu.lastMenuEventSerial
      EmacsNative.sendContextMenu(0, 0, serial)
    }

    super.onContextMenuClosed(menu)
  }

  @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
  fun syncFullscreenWith(emacsWindow: EmacsWindow?) {
    val controller: WindowInsetsController
    var window: Window?
    var behavior: Int
    var flags: Int
    var view: View

    if (emacsWindow != null)
      isFullscreen = emacsWindow.fullscreen
    else
      isFullscreen = false

    /* On Android 11 or later, use the window insets controller to
       control whether or not the view is fullscreen.  */

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      window = getWindow()

      /* If there is no attached window, return immediately.  */
      if (window == null)
        return

      behavior = WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
      controller = window.getInsetsController()!!
      controller.setSystemBarsBehavior(behavior)

      if (isFullscreen)
        controller.hide(WindowInsets.Type.statusBars()
                        or WindowInsets.Type.navigationBars())
      else
        controller.show(WindowInsets.Type.statusBars()
                        or WindowInsets.Type.navigationBars())

      return
    }

    /* On the supported pre-R path, use `setSystemUiVisibility'. */

    window = getWindow()

    if (window == null)
      return

    view = window.getDecorView()

    if (isFullscreen) {
      flags = 0
      flags = flags or View.SYSTEM_UI_FLAG_FULLSCREEN

      /* These flags mean that the editor will be full screen as
         long as the state flag is set.  */
      flags = flags or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
      flags = flags or View.SYSTEM_UI_FLAG_IMMERSIVE
      flags = flags or View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY

      /* Apply the given flags.  */
      view.setSystemUiVisibility(flags)
    } else
      view.setSystemUiVisibility(View.SYSTEM_UI_FLAG_VISIBLE)
  }

  /* Update the name of this activity's task description from the
     current window, or reset the same if no window is attached.  */

  @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
  fun updateWmName() {
    val wmName: String?
    val description: TaskDescription

    if (window == null || window!!.wmName == null)
      wmName = "Emacs"
    else
      wmName = window!!.wmName

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU)
      description = TaskDescription(wmName)
    else
      description = (TaskDescription.Builder()
                     .setLabel(wmName).build())
    setTaskDescription(description)
  }

  final override fun onAttachedToWindow() {
    super.onAttachedToWindow()

    /* Update the window insets.  */
    syncFullscreenWith(window)
  }

  final override fun onNewIntent(intent: Intent?) {
    val tag: String?
    val action: String?

    /* This function is called when EmacsActivity is relaunched from a
       notification.  */

    if (intent == null || EmacsService.SERVICE == null)
      return

    tag = intent.getStringExtra(EmacsDesktopNotification.NOTIFICATION_TAG)
    action = intent.getStringExtra(EmacsDesktopNotification.NOTIFICATION_ACTION)

    if (tag == null || action == null)
      return

    EmacsNative.sendNotificationAction(tag, action)
  }

  open override fun getAttachmentToken(): Long {
    return -1 /* This is overridden by EmacsMultitaskActivity.  */
  }

  final override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
    val resolver: ContentResolver
    var uri: Uri?
    val flags: Int

    when (requestCode) {
      ACCEPT_DOCUMENT_TREE -> {
        /* A document granted through
           EmacsService.requestDirectoryAccess.  */

        if (resultCode == RESULT_OK) {
          resolver = getContentResolver()
          uri = data!!.getData()
          flags = (Intent.FLAG_GRANT_READ_URI_PERMISSION
                   or Intent.FLAG_GRANT_WRITE_URI_PERMISSION)

          try {
            if (uri != null)
              resolver.takePersistableUriPermission(uri, flags)
          } catch (exception: Exception) {
            /* Permission to access URI might've been revoked in
               between selecting the file and this callback being
               invoked.  Don't crash in such cases.  */
          }
        }
      }
    }
  }
}
