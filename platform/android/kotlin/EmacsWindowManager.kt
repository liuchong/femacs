// femacs: port of java/org/gnu/emacs/EmacsWindowManager.java @ fd7afced403.

package org.femacs

import java.util.ArrayList

import android.app.ActivityManager.AppTask
import android.app.ActivityManager.RecentTaskInfo
import android.app.ActivityManager
import android.app.ActivityOptions

import android.content.ComponentName
import android.content.Context
import android.content.Intent

import android.os.Build

import android.util.Log

/* Code to paper over the differences in lifecycles between
   "activities" and windows.

   Four of the five interfaces to be implemented by an instance of this
   class are relevant on all versions of Android:

     registerWindowConsumer (WindowConsumer)
     registerWindow (EmacsWindow)
     removeWindowConsumer (WindowConsumer)
     removeWindow (EmacsWindow)

   A WindowConsumer is expected to allow an EmacsWindow to be attached
   to it, and be created or destroyed.

   Whenever a window is created, registerWindow examines the list of
   window consumers.  If a consumer exists and does not currently have a
   window of its own attached, it gets the new window, while otherwise,
   the window attachment manager starts a new consumer.  Whenever a
   consumer is registered, registerWindowConsumer checks the list of
   available windows.  If a window exists and is not currently attached
   to a consumer, then the consumer gets it.  Finally, every time a
   window is removed, the consumer is destroyed.

     getAttachmentToken ()

   should return a token uniquely identifying a consumer, which, on API
   21 and up, enables attributing the tasks of activities to the windows
   for which they were created, and with that, consistent interaction
   between user-visible window state and their underlying frames.  */

/* Nothing on the C side names this class (`android.c`/`androidterm.c` do not mention it), but its
   statics and its two list fields are read by name from both Java and Kotlin: the already-ported
   `EmacsActivity.kt` 296/326/421 uses `EmacsWindowManager.MANAGER` and 53 uses
   `EmacsWindowManager.WindowConsumer` as a supertype, `EmacsMultitaskActivity.kt` 23 reads
   `EmacsWindowManager.ACTIVITY_TOKEN`, and the reference's own `EmacsView` 757-758,
   `EmacsWindow` 289-320/1414 and `EmacsService` 289/580 read `MANAGER` and the `consumers` field
   through `.` - so `MANAGER`, `ACTIVITY_TOKEN`, `consumers` and `windows` are companion
   `@JvmField`s/`@JvmField` fields (P-83 ②, the `EmacsActivity.EXTRA_STARTUP_ARGUMENTS` shape),
   and the two lists are `MutableList`s because the reference assigns an `ArrayList` to them.
   The reference's `static { MANAGER = new EmacsWindowManager (); }` (83-86) becomes the property's
   initializer, and `nextActivityToken` (78) is a private companion `var` that nothing outside this
   class reads.

   `WindowConsumer` is package-private in the reference (88), which Kotlin cannot express - the
   port's is a public nested interface (P-83 ③) with the same five members; `getAttachedWindow ()`
   returns `EmacsWindow?` because both callers test it for null (131-133/165-169) and
   `EmacsActivity.getAttachedWindow` returns its nullable `window` (P-83 ⑤).  `isWindowEligible`
   is `public static` (116) and so keeps the JVM static shape as a companion `@JvmStatic` (P-83 ⑪,
   the `EmacsDialog.createDialog` shape); the `private static` helpers `getTaskToken` (334) and the
   whole static block go into the same companion, which is the only place Kotlin allows them.

   The six `synchronized` methods (`registerWindowConsumer`, `registerWindow`,
   `removeWindowConsumer`, `detachWindow`, `copyWindows`, `pruneWindows`) become `@Synchronized`
   (P-83 ④).  `!(consumer instanceof EmacsMultitaskActivity)` (261-262) becomes
   `consumer !is EmacsMultitaskActivity` (P-83 ⑳'s reference-identity family, the
   `EmacsActivity.kt` 374 shape).  Java's three assignment-expressions are split into a statement
   plus a test (P-83 ⑬) - `(consumer = window.getAttachedConsumer ()) != null` (285),
   `(name = info.baseIntent.getComponent ()) != null` (452) and `token = ++nextActivityToken`
   (94) - and Java's two `Object object;` temps (359/431) are dropped, both because `object` is a
   Kotlin keyword and because Kotlin's `as` cast needs no intermediate (the `tem` temp of
   `EmacsDesktopNotification.display1` is kept where the reference's own name is usable).
   Every Long/Int mix gets its `L` (P-83 ⑯): `attachmentToken == 0L`, `< 1L`, `token = -2L`,
   `getLongExtra (ACTIVITY_TOKEN, -1L)`.  The reference's `long taskToken`/`boolean set`/`info`
   locals are declared outside their loops and assigned inside (362-385/427-447); Kotlin assigns a
   `val` once per execution path, so those three are declared where they are assigned.
   `activityManager` (81) is dereferenced with `!!` after its null test, the same shape
   `EmacsActivity.kt` 149/210 uses for its `window` field.  */
class EmacsWindowManager {
  companion object {
    private const val TAG = "EmacsWindowManager"

    @JvmField val ACTIVITY_TOKEN: String = "emacs:activity_token"

    /* The single window attachment manager ``object''.  */
    @JvmField val MANAGER: EmacsWindowManager = EmacsWindowManager()

    /* Monotonically increasing counter from which multitasking activity
       tokens are produced.  */
    private var nextActivityToken: Long = 0

    /* Return whether the provided WINDOW should be attached to the window
       consumer CONSUMER.  */

    @JvmStatic
    fun isWindowEligible(consumer: WindowConsumer, window: EmacsWindow): Boolean {
      return (/* The window has yet to be bound.  */
              window.attachmentToken == 0L
              /* Or has already been bound to CONSUMER.  */
              || (window.attachmentToken
                  == consumer.getAttachmentToken()))
    }

    /* Return the activity token specified in the intent giving rise to
       TASK, or 0 if absent.  */

    private fun getTaskToken(task: AppTask): Long {
      val info: RecentTaskInfo

      info = task.getTaskInfo()

      /* baseIntent is a member of info's superclass, TaskInfo, on Android
         10 and later.  Prior to this release, it had been a member of
         RecentTaskInfo since SDK 1, and whatever the misleading
         documentation might suggest, a reference to `baseIntent' through
         TaskInfo is just as good a reference to RecentTaskInfo.  */
      return if (info.baseIntent != null)
        info.baseIntent.getLongExtra(ACTIVITY_TOKEN, -1L)
      else
        0L
    }
  }

  /* The ActivityManager.  */
  private var activityManager: ActivityManager? = null

  /* List of currently attached window consumers.  */
  @JvmField var consumers: MutableList<WindowConsumer> = ArrayList<WindowConsumer>()

  /* List of currently attached windows.  */
  @JvmField var windows: MutableList<EmacsWindow> = ArrayList<EmacsWindow>()

  interface WindowConsumer {
    fun attachWindow(window: EmacsWindow)
    fun getAttachedWindow(): EmacsWindow?
    fun detachWindow()
    fun destroy()
    fun getAttachmentToken(): Long
  }

  @Synchronized
  fun registerWindowConsumer(consumer: WindowConsumer) {
    consumers.add(consumer)
    pruneWindows()

    for (window in windows) {
      if (window.getAttachedConsumer() == null
          /* Don't attach this window to CONSUMER if incompatible.  */
          && isWindowEligible(consumer, window)) {
        /* Permanently bind this window to the consumer.  */
        window.attachmentToken = consumer.getAttachmentToken()
        window.previouslyAttached = true
        consumer.attachWindow(window)
        return
      }
    }

    EmacsNative.sendWindowAction(0L, 0)
  }

  @Synchronized
  fun registerWindow(window: EmacsWindow) {
    var intent: Intent
    var options: ActivityOptions
    var token: Long

    if (windows.contains(window))
      /* The window is already registered.  */
      return

    windows.add(window)

    for (consumer in consumers) {
      if (consumer.getAttachedWindow() == null
          && isWindowEligible(consumer, window)) {
        /* Permanently bind this window to the consumer.  */
        window.attachmentToken = consumer.getAttachmentToken()
        window.previouslyAttached = true
        consumer.attachWindow(window)
        return
      }
    }

    /* Do not create a multitasking activity for the initial frame,
       but arrange to start EmacsActivity.  */
    if (window.attachmentToken == -1L) {
      intent = Intent(EmacsService.SERVICE, EmacsActivity::class.java)
      intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)

      try {
        EmacsService.SERVICE!!.startActivity(intent)
      } catch (exception: Exception) {
        Log.w(TAG, "an activity could not be started on behalf"
                   + " of the mapped default window " + window.handle)
      }

      return
    }

    intent = Intent(EmacsService.SERVICE, EmacsMultitaskActivity::class.java)

    /* FLAG_ACTIVITY_MULTIPLE_TASK would appear appropriate, but that
       is not so: on Android 2.3 and earlier, this flag combined with
       FLAG_ACTIVITY_NEW_TASK prompts the task switcher to create a
       new instance of EmacsMultitaskActivity, rather than return to
       an existing instance, and is entirely redundant, inasmuch as
       only one multitasking task can exist at any given moment.  */
    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)

    /* Intent.FLAG_ACTIVITY_NEW_DOCUMENT is lamentably unavailable on
       older systems than Lollipop.  */
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
      intent.addFlags(Intent.FLAG_ACTIVITY_NEW_DOCUMENT
                        or Intent.FLAG_ACTIVITY_MULTIPLE_TASK)

      /* Bind this window to the activity in advance, i.e., before its
         creation, so that its ID will be recorded in the RecentTasks
         list.  */
      nextActivityToken += 1
      token = nextActivityToken
    } else
      /* APIs required for linking activities to windows are not
         available in earlier Android versions.  */
      token = -2L

    window.attachmentToken = token
    intent.putExtra(ACTIVITY_TOKEN, token)

    try {
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N)
        EmacsService.SERVICE!!.startActivity(intent)
      else {
        /* Specify the desired window size.  */
        options = ActivityOptions.makeBasic()
        options.setLaunchBounds(window.getGeometry())
        EmacsService.SERVICE!!.startActivity(intent, options.toBundle())
      }
    } catch (exception: Exception) {
      Log.w(TAG, "an activity could not be started on behalf"
                 + " of a mapped window, " + window.handle)
    }

    pruneWindows()
  }

  @Synchronized
  fun removeWindowConsumer(consumer: WindowConsumer, isFinishing: Boolean) {
    val window: EmacsWindow?

    window = consumer.getAttachedWindow()

    if (window != null) {
      consumer.detachWindow()

      /* Though pruneWindows will likely remove the same window(s),
         call onActivityDetached anyway if isFinishing is set, if
         CONSUMER not be a multitasking activity, as in obscure
         circumstances pruneWindows will not remove frames bound to
         the system-started task.  */
      if (isFinishing
          && (consumer !is EmacsMultitaskActivity
              || Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP))
        window.onActivityDetached()
    }

    pruneWindows()
    consumers.remove(consumer)
  }

  @Synchronized
  fun detachWindow(window: EmacsWindow) {
    val consumer: WindowConsumer?

    /* Reset window management state.  */
    window.previouslyAttached = false
    window.attachmentToken = 0L

    /* Remove WINDOW from the list of active windows.  */
    windows.remove(window)

    consumer = window.getAttachedConsumer()

    if (consumer != null) {
      consumers.remove(consumer)
      consumer.destroy()
    }

    pruneWindows()
  }

  fun noticeIconified(consumer: WindowConsumer) {
    val window: EmacsWindow?

    /* If a window is attached, send the appropriate iconification
       events.  */
    window = consumer.getAttachedWindow()

    if (window != null)
      window.noticeIconified()
  }

  fun noticeDeiconified(consumer: WindowConsumer) {
    val window: EmacsWindow?

    /* If a window is attached, send the appropriate iconification
       events.  */
    window = consumer.getAttachedWindow()

    if (window != null)
      window.noticeDeiconified()
  }

  @Synchronized
  fun copyWindows(): List<EmacsWindow> {
    return ArrayList<EmacsWindow>(windows)
  }

  /* Iterate over each of Emacs's tasks and remove remaining registered
     windows whose tasks no longer exist.  This function should be
     called upon any event that could plausibly indicate changes in the
     task list or as to window management.  */

  @Synchronized
  private fun pruneWindows() {
    val appTasks: List<AppTask>

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP
        || EmacsService.SERVICE == null)
      return

    if (activityManager == null)
      activityManager =
        EmacsService.SERVICE!!.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager

    appTasks = activityManager!!.getAppTasks()

    /* Clear the preserve flag on all toplevel windows.  */

    for (window in windows)
      window.preserve = false

    for (task in appTasks) {
      val taskToken: Long = getTaskToken(task)
      var set: Boolean = false

      if (taskToken == 0L)
        continue

      /* Search for a window with this token.  */
      for (window in windows) {
        if (window.attachmentToken == taskToken) {
          window.preserve = true
          set = true
        }
      }

      if (!set)
        task.finishAndRemoveTask()
    }

    /* Now remove toplevel windows without activity tasks.  */

    for (window in windows) {
      if (window.preserve
          /* This is not the initial window.  */
          || (window.attachmentToken < 1L)
          /* Nor has it never been attached.  */
          || !window.previouslyAttached)
        continue

      window.onActivityDetached()
    }
  }

  /* Iterate over each of Emacs's tasks to delete such as belong to a
     previous Emacs session, i.e., tasks created for a previous
     session's non-initial frames.  CONTEXT should be a context from
     which to obtain a reference to the activity manager.  */

  fun removeOldTasks(context: Context) {
    val appTasks: List<AppTask>
    val target: String

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP)
      return

    if (activityManager == null)
      activityManager =
        context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager

    appTasks = activityManager!!.getAppTasks()
    target = ".EmacsMultitaskActivity"

    for (task in appTasks) {
      val info: RecentTaskInfo = task.getTaskInfo()
      val name: ComponentName?

      /* Test whether info is a reference to
         EmacsMultitaskActivity.  */
      name = info.baseIntent?.getComponent()

      if (name != null && name.getShortClassName() == target)
        /* Delete the task.  */
        task.finishAndRemoveTask()
    }
  }
}
