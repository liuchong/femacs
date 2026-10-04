// femacs: port of java/org/gnu/emacs/EmacsService.java @ fd7afced403.

package org.gnu.emacs

import java.io.ByteArrayOutputStream
import java.io.FileNotFoundException
import java.io.IOException

import java.util.ArrayList
import java.util.Arrays
import java.util.HashSet
import java.util.concurrent.Callable
import java.util.concurrent.ExecutionException
import java.util.concurrent.FutureTask

import java.util.concurrent.atomic.AtomicInteger

import android.database.Cursor

import android.graphics.Matrix
import android.graphics.Point

import android.webkit.MimeTypeMap

import android.view.InputDevice
import android.view.KeyEvent
import android.view.inputmethod.CursorAnchorInfo
import android.view.inputmethod.ExtractedText

import android.app.AlarmManager
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service

import android.content.ActivityNotFoundException
import android.content.ClipboardManager
import android.content.Context
import android.content.ContentResolver
import android.content.Intent
import android.content.IntentFilter
import android.content.UriPermission

import android.content.pm.PackageManager

import android.content.res.AssetManager
import android.content.res.Configuration
import android.content.res.Resources

import android.hardware.input.InputManager

import android.net.Uri

import android.os.BatteryManager
import android.os.Binder
import android.os.Build
import android.os.Environment
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.ParcelFileDescriptor
import android.os.VibrationEffect
import android.os.Vibrator
import android.os.VibratorManager

import android.provider.DocumentsContract
import android.provider.DocumentsContract.Document
import android.provider.OpenableColumns
import android.provider.Settings

import android.util.Log
import android.util.DisplayMetrics

/* The port's JNI obligations are all instance methods with no fields and no statics:
   `android.c' 1719-1836 does `FindClass ("org/gnu/emacs/EmacsService")' (1725) and 41
   `FIND_METHOD' lookups (1744-1835), and every name and descriptor is reproduced unchanged below
   (`fillRectangle' 1744, `ringBell (I)V' 1760, `queryTree' 1761, `nameKeysym (…)'
   1767, `openContentUri (Ljava/lang/String;ZZZ)I' 1775, `readDirectoryEntry (…)Lorg/gnu/emacs/
   EmacsDirectoryEntry;' 1804, `externalStorageAvailable ()Z' 1828, `cancelNotification
   (Ljava/lang/String;)V' 1832 and the rest).  `android.c' never reads a field of this class - it
   keeps one global `emacs_service' reference, set from the object `setEmacsParams' is handed
   (1485/1662) - and never calls a static, so the `@JvmStatic'/`@JvmField' shapes below exist for
   the Java and Kotlin callers, not for C.

   The one account this file has to settle is SERVICE.  The reference's `static EmacsService
   SERVICE' (79) is NULL before `onCreate' assigns it (251) and four readers depend on that:
   `startEmacsService' (697) exists to start the service when it isn't running,
   `EmacsOpenActivity.java' 693 / `EmacsWindowManager.java' 365 / `EmacsActivity.java' 650 /
   `EmacsDesktopNotification.java' 333 all branch on the NULL.  A non-NULL Kotlin type would have
   to be `lateinit' (a non-NULL `@JvmField' cannot be uninitialized), and a Kotlin read of an
   uninitialized `lateinit' property THROWS `UninitializedPropertyAccessException' (measured in
   the probe), which would turn those five reads - including this file's own `startEmacsService',
   reached from `EmacsActivity.onCreate' - from branches into crashes, and would make the
   reference's four `== NULL' tests unconditionally false.  `SERVICE' is therefore
   `EmacsService?' with the reference's default `null', which is also the shape
   `EmacsActivity.lastFocusedActivity'/'focusedWindow' already use.  The Kotlin callers that
   dereference it unguarded now need `!!' at 31 sites in eight already-ported files (the reference
   has the same unguarded calls: `EmacsWindowManager.java' 371, `EmacsWindow.java' 195/646,
   `EmacsDialog.java' 311/312, `EmacsSdk8Clipboard.java' 50 and the rest); that sweep is a
   ledger item for the batch, not something this file can carry, and it is listed in the status
   entry for this port.

   Otherwise this is P-83 applied: the statics Java/Kotlin read by name become `@JvmField'/
   `@JvmStatic' (`TAG', `IC_MODE_*', `DEBUG_IC', `servicingQuery' and `extraStartupArguments' are
   read from `EmacsView.java' 851/892, `EmacsInputConnection.java' 114, `EmacsActivity.java' 268
   and the ported `EmacsView.kt'/'EmacsInputConnection.kt'), the constants stay `@JvmField val's
   rather than `const's - the shape `EmacsActivity.kt' 59/62 already uses, which leaves the
   class file's `public static final boolean DEBUG_IC' field exactly the reference's - the two `@SuppressWarnings ("deprecation")' (222/540) become
   `@Suppress ("DEPRECATION", "OVERRIDE_DEPRECATION")' on `getScaledDensity' and `ringBell', and
   the six `throws' clauses (1043, 1802, 1861, 1913, 1938, 1971) become `@Throws'.

   Java's implicit numeric widening does not exist in Kotlin, so every crossing is written out
   (P-83 (16)'s family, extended to float/double and char/int here): `dpiX = pixelDensityX
   .toDouble ()' (283-285) and the `pixelDensityX.toDouble () != dpiX' comparisons (380) where
   the reference stores a `float' in a `double' field, `duration.toLong ()' for `vibrate (long)'
   (568) and `createOneShot (long, int)' (564), `offsets[0].toFloat ()' for `postTranslate'
   (1009), `string.get (i).code' for `ch = string.charAt (i)' (1148), `getIntExtra (…).toLong ()'/
   `getIntProperty (…).toLong ()' in `queryBattery19'/`queryBattery' (1277-1355) and `.toInt ()'
   for the `(int)' casts at 610/623.

   `syncRunnable' (844) is where the reference's generics meet Kotlin's: `<V> V' returns its local
   even when the `InterruptedException' arm has set it to NULL, and the ported
   `EmacsContextMenu.kt' 344 / `EmacsDialog.kt' 355 need a non-NULL `V'.  The local is `V?' and the
   return performs the same erasure-preserving conversion the Java return does (`obj as V', the
   only suppression this file carries; measured to return NULL rather than throw).

   Dropped Java locals are the four nothing reads (`getEmacsView''s `Runnable runnable',
   `openContentUri''s `String name', `createDocument''s `String separator', `createDirectory''s
   `int index'), and `syncRunnable''s `V object' is renamed to `obj' (`object' is a keyword).
   Java locals declared outside a loop and assigned inside it are declared where they are assigned
   (P-83 (24): `readDirectoryEntry' 1685, `getDocumentAuthorities' 1392, `getDocumentTrees' 1486,
   `getTreeUri' 1558, `validAuthority' 2014), and the two parameters the reference reassigns
   (`createDocument'/'createDirectory''s `documentId', `createDirectory''s `name' and
   `getTreeUri''s `tree') take a same-named `var' local (P-83 (18)/(31)).

   `Uri.encode'/`Uri.decode' are Java statics the reference calls through an instance (1240, 1506,
   1564) and are class-qualified here (P-83 (26)), and `readDirectoryEntry''s `name.equals (…)'-
   style tests are Kotlin `==' on a String, which is the same comparison the reference makes
   (1718-1726, 1741).  `buildContentName''s and `getDocumentTrees'' `new String[0]' type tokens
   (1410, 1513) become `arrayOf<String> ()' - the same empty `String[]', without the unchecked cast
   `arrayOfNulls<String> (n) as Array<String>' the batch's older files carry for `new String[n]'.
   `documentIdFromName''s `id_return' is `Array<String?>' to match `EmacsSafThread.kt' 1088 (the
   JNI descriptor is `[Ljava/lang/String;' either way), and the fields `onCreate' assigns
   (`handler', `resolver') are `lateinit'.

   Nullability is judged at each point (P-83 (5)): `viewGetSelection' (902), `queryBattery',
   `browseUrl', `getTreeUri', `statDocument', `readDirectoryEntry', `openDocument',
   `createDocument', `createDirectory', `moveDocument' and the
   `Cursor'/`Intent'/`ParcelFileDescriptor'/`Uri' locals the reference tests for NULL are nullable
   because the reference returns or holds NULL there, while `buildContentName' stays non-NULL
   (`EmacsOpenActivity.kt' 216/488/523 assign it to non-NULL locals) even though
   `EmacsWindow.kt' 1601 tests its result - that test is the one always-'true' condition the
   batch reports after this file lands.  */

/* EmacsService is the service that starts the thread running Emacs
   and handles requests by that Emacs instance.  */
class EmacsService : Service() {
  companion object {
    @JvmField val TAG: String = "EmacsService"

    /* The started Emacs service object.  */
    @JvmField var SERVICE: EmacsService? = null

    /* If non-NULL, an array of extra arguments to pass to
       `android_emacs_init'.  */
    @JvmField var extraStartupArguments: Array<String>? = null

    /* Keep this in synch with androidgui.h.  */
    @JvmField val IC_MODE_NULL: Int = 0
    @JvmField val IC_MODE_ACTION: Int = 1
    @JvmField val IC_MODE_TEXT: Int = 2
    @JvmField val IC_MODE_PASSWORD: Int = 3

    /* Flag that says whether or not to print verbose debugging
       information when responding to an input method.  */
    @JvmField val DEBUG_IC: Boolean = false

    /* Flag that says whether or not to stringently check that only the
       Emacs thread is performing drawing calls.  */
    private const val DEBUG_THREADS: Boolean = false

    /* Atomic integer used for synchronization between
       icBeginSynchronous/icEndSynchronous and viewGetSelection.

       Value is 0 if no query is in progress, 1 if viewGetSelection is
       being called, and 2 if icBeginSynchronous was called.  */
    @JvmField val servicingQuery: AtomicInteger = AtomicInteger()

    /* Return the directory leading to the directory in which native
       library files are stored on behalf of CONTEXT.  */

    @JvmStatic
    fun getLibraryDirectory(context: Context): String {
      val apiLevel: Int

      apiLevel = Build.VERSION.SDK_INT

      if (apiLevel >= Build.VERSION_CODES.GINGERBREAD)
        return context.getApplicationInfo().nativeLibraryDir

      return context.getApplicationInfo().dataDir + "/lib"
    }

    /* Return the display density, adjusted in accord with the user's
       text scaling preferences.  */

    @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
    @JvmStatic
    private fun getScaledDensity(metrics: DisplayMetrics): Float {
      /* The scaled density has been made obsolete by the introduction
         of non-linear text scaling in Android 34, where there is no
         longer a fixed relation between point and pixel sizes, but
         remains useful, considering that Emacs does not support
         non-linear text scaling.  */
      return metrics.scaledDensity
    }

    @JvmStatic
    fun checkEmacsThread() {
      if (DEBUG_THREADS) {
        /* When SERVICE is NULL, Emacs is being executed non-interactively.  */
        if (SERVICE == null
            /* It was previously assumed that only instances of
               `EmacsThread' were valid for graphics calls, but this is
               no longer true now that Lisp threads can be attached to
               the JVM.  */
            || (Thread.currentThread() != SERVICE!!.mainThread))
          return

        throw RuntimeException("Emacs thread function"
                               + " called from other thread!")
      }
    }

    /* Start the Emacs service if necessary.  On Android 26 and up,
       start Emacs as a foreground service with a notification, to avoid
       it being killed by the system.

       On older systems, simply start it as a normal background
       service.  */

    @JvmStatic
    fun startEmacsService(context: Context) {
      if (EmacsService.SERVICE == null) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O)
          /* Start the Emacs service now.  */
          context.startService(Intent(context,
                                      EmacsService::class.java))
        else
          /* Display the permanent notification and start Emacs as a
             foreground service.  */
          context.startForegroundService(Intent(context,
                                                EmacsService::class.java))
      }
    }

    /* Wait synchronously for the specified TASK to complete in the UI
       thread, then return its result.  Must be called from the Emacs
       thread.  */

    @Suppress("UNCHECKED_CAST")
    @JvmStatic
    fun <V> syncRunnable(task: FutureTask<V>): V {
      var obj: V?

      EmacsNative.beginSynchronous()
      SERVICE!!.runOnUiThread(task)

      try {
        obj = task.get()
      } catch (exception: ExecutionException) {
        /* Wrap this exception in a RuntimeException and signal it to
           the caller.  */
        throw RuntimeException(exception.cause)
      } catch (exception: InterruptedException) {
        EmacsNative.emacsAbort()
        obj = null
      }

      EmacsNative.endSynchronous()

      /* The reference returns its (possibly NULL) local here; returning
         a `V?' would change every caller's type, so the erasure
         preserving cast below performs the same conversion the Java
         return does implicitly (P-83 ⑤).  */
      return obj as V
    }

    /* IMM functions such as `updateSelection' holds an internal lock
       that is also taken before `onCreateInputConnection' (in
       EmacsView.java) is called; when that then asks the UI thread for
       the current selection, a dead lock results.  To remedy this,
       reply to any synchronous queries now -- and prohibit more queries
       for the duration of `updateSelection' -- if EmacsView may have
       been asking for the value of the region.  */

    @JvmStatic
    fun icBeginSynchronous() {
      /* Set servicingQuery to 2, so viewGetSelection knows it shouldn't
         proceed.  */
      if (servicingQuery.getAndSet(2) == 1)
        /* But if viewGetSelection is already in progress, answer it
           first.  */
        EmacsNative.answerQuerySpin()
    }

    @JvmStatic
    fun icEndSynchronous() {
      if (servicingQuery.getAndSet(0) != 2)
        throw RuntimeException("incorrect value of `servicingQuery': "
                               + "likely 1")
    }

    @JvmStatic
    fun viewGetSelection(window: Long): IntArray? {
      val selection: IntArray?

      /* See if a query is already in progress from the other
         direction.  */
      if (!servicingQuery.compareAndSet(0, 1))
        return null

      /* Now call the regular getSelection.  Note that this can't race
         with answerQuerySpin, as `android_servicing_query' can never be
         2 when icBeginSynchronous is called, so a query will always be
         started.  */
      selection = EmacsNative.getSelection(window)

      /* Finally, clear servicingQuery if its value is still 1.  If a
         query has started from the other side, it ought to be 2.  */
      servicingQuery.compareAndSet(1, 0)
      return selection
    }

    /* Return a 8 character checksum for the string STRING, after encoding
       as UTF-8 data.  */

    @JvmStatic
    private fun getDisplayNameHash(string: String): String? {
      val encoded: ByteArray
      val stream: ByteArrayOutputStream
      var i: Int
      var ch: Int

      /* Much of the VFS code expects file names to be encoded as modified
         UTF-8 data, but Android's JNI implementation produces (while not
         accepting!) regular UTF-8 sequences for all characters, even
         non-Emoji ones.  With no documentation to this effect, save for
         two comments nestled in the source code of the Java virtual
         machine, it is not sound to assume that this behavior will not be
         revised in future or modified releases of Android, and as such,
         encode STRING into modified UTF-8 by hand, to protect against
         future changes in this respect.  */

      stream = ByteArrayOutputStream()

      for (i in 0 until string.length) {
        ch = string.get(i).code

        if (ch != 0 && ch <= 127)
          stream.write(ch)
        else if (ch <= 2047) {
          stream.write(0xc0 or (0x1f and (ch shr 6)))
          stream.write(0x80 or (0x3f and ch))
        } else {
          stream.write(0xe0 or (0x0f and (ch shr 12)))
          stream.write(0x80 or (0x3f and (ch shr 6)))
          stream.write(0x80 or (0x3f and ch))
        }
      }

      encoded = stream.toByteArray()

      /* Closing a ByteArrayOutputStream has no effect.
         encoded.close ();  */

      return EmacsNative.displayNameHash(encoded)
    }

    /* Build a content file name for URI.

       Return a file name within the /contents/by-authority
       pseudo-directory that `android_get_content_name' can then
       transform back into an encoded URI.

       If a display name can be requested from URI (using the resolver
       RESOLVER), append it to this file name.

       A content name consists of any number of unencoded path segments
       separated by `/' characters, possibly followed by a question mark
       and an encoded query string.  */

    @JvmStatic
    fun buildContentName(uri: Uri, resolver: ContentResolver): String {
      val builder: StringBuilder
      var displayName: String?
      var cursor: Cursor?

      displayName = null
      cursor = null

      try {
        cursor = resolver.query(uri, null, null, null, null)

        if (cursor != null) {
          cursor.moveToFirst()
          val column = cursor.getColumnIndexOrThrow(OpenableColumns.DISPLAY_NAME)
          displayName = cursor.getString(column)

          /* Verify that the display name is valid, i.e. it contains no
             characters unsuitable for a file name and is nonempty.  */
          if (displayName.isEmpty() || displayName.contains("/"))
            displayName = null
        }
      } catch (exception: Exception) {
        /* Ignored.  */
      } finally {
        if (cursor != null)
          cursor.close()
      }

      /* If a display name is available, at this point it should be the
         value of displayName.  */

      builder = StringBuilder(if (displayName != null)
                                "/content/by-authority-named/"
                              else
                                "/content/by-authority/")
      builder.append(uri.getAuthority())

      /* First, append each path segment.  */

      for (segment in uri.getPathSegments()) {
        /* FIXME: what if segment contains a slash character? */
        builder.append('/')
        builder.append(Uri.encode(segment))
      }

      /* Now, append the query string if necessary.  */

      if (uri.getEncodedQuery() != null)
        builder.append('?').append(uri.getEncodedQuery())

      /* Append the display name.  */

      if (displayName != null) {
        builder.append('/')
        builder.append(getDisplayNameHash(displayName))
        builder.append('/')
        builder.append(displayName)
      }

      return builder.toString()
    }
  }

  /* The thread running Emacs C code.  */
  private var thread: EmacsThread? = null

  /* Handler used to run tasks on the main thread.  */
  private lateinit var handler: Handler

  /* Content resolver used to access URIs.  */
  private lateinit var resolver: ContentResolver

  /* Thread used to query document providers, or null if it hasn't
     been created yet.  */
  private var storageThread: EmacsSafThread? = null

  /* The Thread object representing the Android user interface
     thread.  */
  private var mainThread: Thread? = null

  /* The display's horizontal and vertical density and that which is
     consulted for font scaling.  */
  private var dpiX: Double = 0.0
  private var dpiY: Double = 0.0
  private var dpiScaled: Double = 0.0

  /* The display's previously observed UI mode as it relates to the
     system theme.  */
  private var uiMode: Int = 0

  override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
    val notification: Notification
    val manager: NotificationManager
    val channel: NotificationChannel
    val infoBlurb: String
    val tem: Any?

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      tem = getSystemService(Context.NOTIFICATION_SERVICE)
      manager = tem as NotificationManager
      infoBlurb = ("This notification is displayed to keep Emacs"
                   + " running while it is in the background.  You"
                   + " may disable it if you wish;"
                   + " see (emacs)Android Environment.")
      channel = NotificationChannel("emacs", "Emacs Background Service",
                                    NotificationManager.IMPORTANCE_LOW)
      manager.createNotificationChannel(channel)
      notification = (Notification.Builder(this, "emacs")
                        .setContentTitle("Emacs")
                        .setContentText(infoBlurb)
                        .setSmallIcon(android.R.drawable.sym_def_app_icon)
                        .build())
      manager.notify(1, notification)
      startForeground(1, notification)
    }

    return Service.START_NOT_STICKY
  }

  override fun onBind(intent: Intent?): IBinder? {
    return null
  }

  override fun onCreate() {
    val manager: AssetManager
    val app_context: Context
    val filesDir: String
    val libDir: String
    val cacheDir: String
    val classPath: String?
    val pixelDensityX: Float
    val pixelDensityY: Float
    val scaledDensity: Float
    var tempScaledDensity: Float
    val resources: Resources
    val metrics: DisplayMetrics
    val configuration: Configuration

    super.onCreate()

    SERVICE = this
    resources = getResources()
    handler = Handler(Looper.getMainLooper())
    manager = getAssets()
    app_context = getApplicationContext()
    metrics = resources.getDisplayMetrics()
    pixelDensityX = metrics.xdpi
    pixelDensityY = metrics.ydpi
    tempScaledDensity = ((getScaledDensity(metrics)
                          / metrics.density)
                         * pixelDensityX)
    configuration = resources.getConfiguration()
    uiMode = configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK
    resolver = getContentResolver()
    mainThread = Thread.currentThread()

    /* If the density used to compute the text size is smaller than 160,
       there's likely a bug with display density computation.  Reset it
       to 160 in that case.

       Note that Android uses 160 ``dpi'' as the density where 1 point
       corresponds to 1 pixel, not 72 or 96 as used elsewhere.  This
       difference is codified in PT_PER_INCH defined in font.h.  */

    if (tempScaledDensity < 160.0f)
      tempScaledDensity = 160.0f

    /* scaledDensity is const as required to refer to it from within
       the nested function below.  */
    scaledDensity = tempScaledDensity

    /* Save these fields for future reference.  */
    dpiX = pixelDensityX.toDouble()
    dpiY = pixelDensityY.toDouble()
    dpiScaled = scaledDensity.toDouble()

    /* Remove all tasks from previous Emacs sessions but the task
       created by the system at startup.  */
    EmacsWindowManager.MANAGER.removeOldTasks(this)

    try {
      /* Configure Emacs with the asset manager and other necessary
         parameters.  */
      filesDir = app_context.getFilesDir().getCanonicalPath()
      libDir = getLibraryDirectory(this)
      cacheDir = app_context.getCacheDir().getCanonicalPath()

      /* Now provide this application's apk file, so a recursive
         invocation of app_process (through android-emacs) can
         find EmacsNoninteractive.  */
      classPath = EmacsApplication.apkFileName

      Log.d(TAG, "Initializing Emacs, where filesDir = " + filesDir
             + ", libDir = " + libDir + ", and classPath = " + classPath
             + "; args = " + (if (extraStartupArguments != null)
                                Arrays.toString(extraStartupArguments!!)
                              else
                                "(none)")
             + "; display density: " + pixelDensityX + " by "
             + pixelDensityY + " scaled to " + scaledDensity)

      /* Start the thread that runs Emacs.  */
      thread = EmacsThread(this, Runnable {
        EmacsNative.setEmacsParams(manager, filesDir, libDir, cacheDir,
                                   pixelDensityX, pixelDensityY,
                                   scaledDensity, uiMode, classPath,
                                   this@EmacsService,
                                   Build.VERSION.SDK_INT)
      }, extraStartupArguments)
      thread!!.start()
    } catch (exception: IOException) {
      EmacsNative.emacsAbort()
      return
    }
  }

  /* The native functions the subsequent two functions call do nothing
     in the infrequent case the Emacs thread is awaiting a response
     for the main thread.  Caveat emptor! */

  override fun onDestroy() {
    /* This function is called immediately before the system kills
       Emacs.  In this respect, it is rather akin to a SIGDANGER
       signal, so force an auto-save accordingly.  */

    EmacsNative.shutDownEmacs()
    super.onDestroy()
  }

  override fun onLowMemory() {
    EmacsNative.onLowMemory()
    super.onLowMemory()
  }

  override fun onConfigurationChanged(newConfig: Configuration) {
    val metrics: DisplayMetrics
    var pixelDensityX: Float
    var pixelDensityY: Float
    var scaledDensity: Float

    metrics = getResources().getDisplayMetrics()

    /* The display configuration may have been altered.  Retrieve the
       revised display density and deliver an event if so.  */
    pixelDensityX = metrics.xdpi
    pixelDensityY = metrics.ydpi
    scaledDensity = ((getScaledDensity(metrics)
                      / metrics.density) * pixelDensityX)

    /* A density below 160 probably indicates a system bug.  See
       onCreate for more commentary.  */
    if (scaledDensity < 160.0f)
      scaledDensity = 160.0f

    if (pixelDensityX.toDouble() != dpiX || pixelDensityY.toDouble() != dpiY
        || scaledDensity.toDouble() != dpiScaled) {
      dpiX = pixelDensityX.toDouble()
      dpiY = pixelDensityY.toDouble()
      dpiScaled = scaledDensity.toDouble()
      EmacsNative.sendConfigurationChanged(0, pixelDensityX, pixelDensityY,
                                           scaledDensity, 0)
    }

    if ((newConfig.uiMode and Configuration.UI_MODE_NIGHT_MASK) != uiMode) {
      uiMode = newConfig.uiMode and Configuration.UI_MODE_NIGHT_MASK
      EmacsNative.sendConfigurationChanged(1, 0.0f, 0.0f, 0.0f, uiMode)
    }

    super.onConfigurationChanged(newConfig)
  }

  /* Functions from here on must only be called from the Emacs
     thread.  */

  fun runOnUiThread(runnable: Runnable) {
    handler.post(runnable)
  }

  fun getEmacsView(window: EmacsWindow, visibility: Int,
                   isFocusedByDefault: Boolean): EmacsView {
    val task: FutureTask<EmacsView>

    task = FutureTask<EmacsView>(Callable<EmacsView> {
      val view: EmacsView

      view = EmacsView(window)
      view.setVisibility(visibility)

      /* The following function is only present on Android 26
         or later.  */
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
        view.setFocusedByDefault(isFocusedByDefault)

      view
    })

    return EmacsService.syncRunnable<EmacsView>(task)
  }

  fun getLocationOnScreen(view: EmacsView, coordinates: IntArray) {
    val task: FutureTask<Void>

    task = FutureTask<Void>(Callable<Void> {
      view.getLocationOnScreen(coordinates)
      null
    })

    EmacsService.syncRunnable<Void>(task)
  }

  fun getLocationInWindow(view: EmacsView, coordinates: IntArray) {
    val task: FutureTask<Void>

    task = FutureTask<Void>(Callable<Void> {
      view.getLocationInWindow(coordinates)
      null
    })

    EmacsService.syncRunnable<Void>(task)
  }

  /* These drawing functions must only be called from the Emacs
     thread.  */

  fun fillRectangle(drawable: EmacsDrawable, gc: EmacsGC,
                    x: Int, y: Int, width: Int, height: Int) {
    checkEmacsThread()
    EmacsFillRectangle.perform(drawable, gc, x, y,
                               width, height)
  }

  fun fillPolygon(drawable: EmacsDrawable, gc: EmacsGC,
                  points: Array<Point>) {
    checkEmacsThread()
    EmacsFillPolygon.perform(drawable, gc, points)
  }

  fun drawRectangle(drawable: EmacsDrawable, gc: EmacsGC,
                    x: Int, y: Int, width: Int, height: Int) {
    checkEmacsThread()
    EmacsDrawRectangle.perform(drawable, gc, x, y,
                               width, height)
  }

  fun drawLine(drawable: EmacsDrawable, gc: EmacsGC,
               x: Int, y: Int, x2: Int, y2: Int) {
    checkEmacsThread()
    EmacsDrawLine.perform(drawable, gc, x, y,
                          x2, y2)
  }

  fun drawPoint(drawable: EmacsDrawable, gc: EmacsGC,
                x: Int, y: Int) {
    checkEmacsThread()
    EmacsDrawPoint.perform(drawable, gc, x, y)
  }

  @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
  fun ringBell(duration: Int) {
    val vibrator: Vibrator
    val effect: VibrationEffect
    val vibratorManager: VibratorManager
    val tem: Any?
    val amplitude: Int

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
      tem = getSystemService(Context.VIBRATOR_MANAGER_SERVICE)
      vibratorManager = tem as VibratorManager
      vibrator = vibratorManager.getDefaultVibrator()
    } else
      vibrator = getSystemService(Context.VIBRATOR_SERVICE) as Vibrator

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      amplitude = VibrationEffect.DEFAULT_AMPLITUDE
      effect = VibrationEffect.createOneShot(duration.toLong(), amplitude)
      vibrator.vibrate(effect)
    } else
      vibrator.vibrate(duration.toLong())
  }

  fun queryTree(window: EmacsWindow?): LongArray {
    val array: LongArray
    val windowList: List<EmacsWindow>
    var i: Int

    if (window == null)
      /* Just return all the windows without a parent.  */
      windowList = EmacsWindowManager.MANAGER.copyWindows()
    else
      windowList = window.children

    synchronized(windowList) {
      array = LongArray(windowList.size + 1)
      i = 1

      array[0] = (if (window == null)
                    0L
                  else (if (window.parent != null)
                          window.parent!!.handle
                        else
                          0L))

      for (treeWindow in windowList)
        array[i++] = treeWindow.handle
    }

    return array
  }

  fun getScreenWidth(mmWise: Boolean): Int {
    val metrics: DisplayMetrics

    metrics = getResources().getDisplayMetrics()

    if (!mmWise)
      return metrics.widthPixels
    else
      return ((metrics.widthPixels / metrics.xdpi) * 2540.0).toInt()
  }

  fun getScreenHeight(mmWise: Boolean): Int {
    val metrics: DisplayMetrics

    metrics = getResources().getDisplayMetrics()

    if (!mmWise)
      return metrics.heightPixels
    else
      return ((metrics.heightPixels / metrics.ydpi) * 2540.0).toInt()
  }

  fun detectMouse(): Boolean {
    val manager: InputManager
    val ids: IntArray
    var i: Int

    if (Build.VERSION.SDK_INT
        /* Android 4.0 and earlier don't support mouse input events at
           all.  */
        < Build.VERSION_CODES.JELLY_BEAN)
      return false

    manager = getSystemService(Context.INPUT_SERVICE) as InputManager
    ids = manager.getInputDeviceIds()

    for (i in ids.indices) {
      val device: InputDevice?

      device = manager.getInputDevice(ids[i])

      if (device == null)
        continue

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
        if (device.supportsSource(InputDevice.SOURCE_MOUSE))
          return true
      } else {
        /* `supportsSource' is only present on API level 21 and
           later, but earlier versions provide a bit mask
           containing each supported source.  */

        if ((device.getSources() and InputDevice.SOURCE_MOUSE) != 0)
          return true
      }
    }

    return false
  }

  fun detectKeyboard(): Boolean {
    val configuration: Configuration

    configuration = getResources().getConfiguration()
    return configuration.keyboard != Configuration.KEYBOARD_NOKEYS
  }

  fun nameKeysym(keysym: Int): String {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB_MR1)
      return KeyEvent.keyCodeToString(keysym)

    return keysym.toString()
  }

  /* Ask the system to open the specified URL in an application that
     understands how to open it.

     If SEND, tell the system to also open applications that can
     ``send'' the URL (through mail, for example), instead of only
     those that can view the URL.

     Value is NULL upon success, or a string describing the error
     upon failure.  */

  fun browseUrl(url: String, send: Boolean): String? {
    var intent: Intent
    var uri: Uri

    try {
      /* Parse the URI.  */
      if (!send) {
        uri = Uri.parse(url)

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT) {
          /* On Android 4.4 and later, check if URI is actually
             a file name.  If so, rewrite it into a content
             provider URI, so that it can be accessed by other
             programs.  */

          if (uri.getScheme().equals("file")
              && uri.getPath() != null)
            uri = DocumentsContract.buildDocumentUri("org.gnu.emacs",
                                                     uri.getPath()!!)
        }

        intent = Intent(Intent.ACTION_VIEW, uri)

        /* Set several flags on the Intent prompting the system to
           permit the recipient to read and edit the URI
           indefinitely.  */

        intent.setFlags(Intent.FLAG_ACTIVITY_NEW_TASK
                        or Intent.FLAG_GRANT_READ_URI_PERMISSION
                        or Intent.FLAG_GRANT_WRITE_URI_PERMISSION)

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT)
          intent.addFlags(Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION)
      } else {
        intent = Intent(Intent.ACTION_SEND)
        intent.setType("text/plain")
        intent.putExtra(Intent.EXTRA_SUBJECT, "Sharing link")
        intent.putExtra(Intent.EXTRA_TEXT, url)

        /* Display a list of programs able to send this URL.  */
        intent = Intent.createChooser(intent, "Send")

        /* Apparently flags need to be set after a chooser is
           created.  */
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      }

      startActivity(intent)
    } catch (exception: Exception) {
      return exception.toString()
    }

    return null
  }

  /* Get a SDK 11 ClipboardManager.

     Android 4.0.x requires that this be called from the main
     thread.  */

  fun getClipboardManager(): ClipboardManager {
    val task: FutureTask<Any?>

    task = FutureTask<Any?>(Callable<Any?> {
      getSystemService(Context.CLIPBOARD_SERVICE)
    })

    return EmacsService.syncRunnable<Any?>(task) as ClipboardManager
  }

  fun restartEmacs() {
    val intent: Intent
    val pending: PendingIntent
    val manager: AlarmManager

    intent = Intent(this, EmacsActivity::class.java)
    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK
                    or Intent.FLAG_ACTIVITY_CLEAR_TASK)

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.KITKAT)
      startActivity(intent)
    else {
      /* Experimentation has established that Android 4.3 and earlier
         versions do not attempt to recreate a process when it crashes
         immediately after requesting that an intent for itself be
         started.  Schedule an intent to start some time after Emacs
         exits instead.  */

      pending = PendingIntent.getActivity(this, 0, intent, 0)
      manager = getSystemService(Context.ALARM_SERVICE) as AlarmManager
      manager.set(AlarmManager.RTC, System.currentTimeMillis() + 100,
                  pending)
    }

    System.exit(0)
  }

  fun updateIC(window: EmacsWindow, newSelectionStart: Int,
               newSelectionEnd: Int, composingRegionStart: Int,
               composingRegionEnd: Int) {
    if (DEBUG_IC)
      Log.d(TAG, ("updateIC: " + window + " " + newSelectionStart
                  + " " + newSelectionEnd + " "
                  + composingRegionStart + " "
                  + composingRegionEnd))

    icBeginSynchronous()
    window.view.imManager.updateSelection(window.view,
                                          newSelectionStart,
                                          newSelectionEnd,
                                          composingRegionStart,
                                          composingRegionEnd)
    icEndSynchronous()
  }

  fun resetIC(window: EmacsWindow, icMode: Int) {
    val oldMode: Int

    if (DEBUG_IC)
      Log.d(TAG, "resetIC: " + window + ", " + icMode)

    oldMode = window.view.getICMode()

    /* If it's not necessary to reset the input connection for ICMODE to
       take effect, return immediately.  */
    if (oldMode == IC_MODE_NULL && icMode == IC_MODE_NULL) {
      if (DEBUG_IC)
        Log.d(TAG, "resetIC: redundant invocation ignored")
      return
    }

    if (oldMode == icMode
        && Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
      if (DEBUG_IC)
        Log.d(TAG, "resetIC: calling invalidateInput")

      /* Android 33 and later allow the IM reset to be optimized out
         and replaced by a call to `invalidateInput', which is much
         faster, as it does not involve resetting the input
         connection.  */

      icBeginSynchronous()
      window.view.imManager.invalidateInput(window.view)
      icEndSynchronous()

      return
    }

    window.view.setICMode(icMode)

    icBeginSynchronous()
    window.view.icGeneration++
    window.view.imManager.restartInput(window.view)
    icEndSynchronous()
  }

  fun updateCursorAnchorInfo(window: EmacsWindow, x: Float,
                             y: Float, yBaseline: Float,
                             yBottom: Float) {
    val info: CursorAnchorInfo
    val builder: CursorAnchorInfo.Builder
    val matrix: Matrix
    val offsets: IntArray

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP)
      return

    offsets = IntArray(2)
    builder = CursorAnchorInfo.Builder()
    matrix = Matrix(window.view.getMatrix())
    window.view.getLocationOnScreen(offsets)
    matrix.postTranslate(offsets[0].toFloat(), offsets[1].toFloat())
    builder.setMatrix(matrix)
    builder.setInsertionMarkerLocation(x, y, yBaseline, yBottom,
                                       0)
    info = builder.build()

    if (DEBUG_IC)
      Log.d(TAG, ("updateCursorAnchorInfo: " + x + " " + y
                  + " " + yBaseline + "-" + yBottom))

    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      window.view.imManager.updateCursorAnchorInfo(window.view, info)
    })
  }

  /* Content provider functions.  */

  /* Open a content URI described by the bytes BYTES, a non-terminated
     string; make it writable if WRITABLE, and readable if READABLE.
     Truncate the file if TRUNCATE.

     Value is the resulting file descriptor, -1, or an exception will be
     raised.  */

  @Throws(FileNotFoundException::class, IOException::class)
  fun openContentUri(uri: String, writable: Boolean, readable: Boolean,
                     truncate: Boolean): Int {
    var mode: String
    val fd: ParcelFileDescriptor?
    var i: Int
    val uriObject: Uri?

    /* Figure out the file access mode.  */

    mode = ""

    if (readable)
      mode += "r"

    if (writable)
      mode += "w"

    if (truncate)
      mode += "t"

    /* Decode the URI.  It might be possible for a perverse user to
       construct a content file name that Android finds unparsable, so
       punt if the result is NULL.  */

    uriObject = Uri.parse(uri)
    if (uriObject == null)
      return -1

    /* Try to open a corresponding ParcelFileDescriptor.  Though
       `fd.detachFd' is exclusive to Honeycomb and up, this function is
       never called on systems older than KitKat, which is Emacs's
       minimum requirement for access to /content/by-authority.  */

    fd = resolver.openFileDescriptor(uriObject, mode)
    if (fd == null)
      return -1

    i = fd.detachFd()
    fd.close()

    return i
  }

  /* Return whether Emacs is directly permitted to access the
     content:// URI NAME.  This is not a suitable test for files which
     Emacs can access by virtue of their containing document
     trees.  */

  fun checkContentUri(name: String, readable: Boolean, writable: Boolean): Boolean {
    val uri: Uri?
    var rc: Int
    var flags: Int

    /* Decode the URI.  It might be possible that perverse user should
       construct a content file name that Android finds unparsable, so
       punt if the result is NULL.  */

    uri = Uri.parse(name)
    if (uri == null)
      return false

    flags = 0

    if (readable)
      flags = flags or Intent.FLAG_GRANT_READ_URI_PERMISSION

    if (writable)
      flags = flags or Intent.FLAG_GRANT_WRITE_URI_PERMISSION

    /* checkCallingUriPermission deals with permissions held by callers
       of functions over the Binder IPC mechanism as contrasted with
       Emacs itself, while getCallingPid and getCallingUid, despite the
       class where they reside, return the process credentials against
       which the system will actually test URIs being opened.  */

    rc = checkUriPermission(uri, Binder.getCallingPid(),
                            Binder.getCallingUid(), flags)
    return rc == PackageManager.PERMISSION_GRANTED
  }

  private fun queryBattery19(): LongArray? {
    val filter: IntentFilter
    val battery: Intent?
    val capacity: Long
    val chargeCounter: Long
    val currentAvg: Long
    val currentNow: Long
    val status: Long
    val remaining: Long
    val plugged: Long
    val temp: Long

    filter = IntentFilter(Intent.ACTION_BATTERY_CHANGED)
    battery = registerReceiver(null, filter)

    if (battery == null)
      return null

    capacity = battery.getIntExtra(BatteryManager.EXTRA_LEVEL, 0).toLong()
    chargeCounter = (battery.getIntExtra(BatteryManager.EXTRA_SCALE, 0)
                     / battery.getIntExtra(BatteryManager.EXTRA_LEVEL, 100) * 100).toLong()
    currentAvg = 0L
    currentNow = 0L
    status = battery.getIntExtra(BatteryManager.EXTRA_STATUS, 0).toLong()
    remaining = -1L
    plugged = battery.getIntExtra(BatteryManager.EXTRA_PLUGGED, 0).toLong()
    temp = battery.getIntExtra(BatteryManager.EXTRA_TEMPERATURE, 0).toLong()

    return longArrayOf(capacity, chargeCounter, currentAvg,
                       currentNow, status, remaining, plugged,
                       temp)
  }

  /* Return the status of the battery.  See struct
     android_battery_status for the order of the elements
     returned.

     Value may be null upon failure.  */

  fun queryBattery(): LongArray? {
    val tem: Any?
    val manager: BatteryManager
    val capacity: Long
    val chargeCounter: Long
    val currentAvg: Long
    val currentNow: Long
    var status: Long
    var remaining: Long
    var plugged: Long
    var temp: Long
    var prop: Int
    val filter: IntentFilter
    val battery: Intent?

    /* Android 4.4 or earlier require applications to use a different
       API to query the battery status.  */

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP)
      return queryBattery19()

    tem = getSystemService(Context.BATTERY_SERVICE)
    manager = tem as BatteryManager
    remaining = -1L

    prop = BatteryManager.BATTERY_PROPERTY_CAPACITY
    capacity = manager.getLongProperty(prop)
    prop = BatteryManager.BATTERY_PROPERTY_CHARGE_COUNTER
    chargeCounter = manager.getLongProperty(prop)
    prop = BatteryManager.BATTERY_PROPERTY_CURRENT_AVERAGE
    currentAvg = manager.getLongProperty(prop)
    prop = BatteryManager.BATTERY_PROPERTY_CURRENT_NOW
    currentNow = manager.getLongProperty(prop)

    /* Return the battery status.  N.B. that Android 7.1 and earlier
       only return ``charging'' or ``discharging''.  */

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
      status =
        manager.getIntProperty(BatteryManager.BATTERY_PROPERTY_STATUS).toLong()
    else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M)
      status = (if (manager.isCharging())
                  BatteryManager.BATTERY_STATUS_CHARGING
                else
                  BatteryManager.BATTERY_STATUS_DISCHARGING).toLong()
    else
      status = (if (currentNow > 0)
                  BatteryManager.BATTERY_STATUS_CHARGING
                else
                  BatteryManager.BATTERY_STATUS_DISCHARGING).toLong()

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P)
      remaining = manager.computeChargeTimeRemaining()

    plugged = -1L
    temp = -1L

    /* Now obtain additional information from the battery manager.  */

    filter = IntentFilter(Intent.ACTION_BATTERY_CHANGED)
    battery = registerReceiver(null, filter)

    if (battery != null) {
      plugged = battery.getIntExtra(BatteryManager.EXTRA_PLUGGED, 0).toLong()
      temp = battery.getIntExtra(BatteryManager.EXTRA_TEMPERATURE, 0).toLong()

      /* Make status more reliable.  */
      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M)
        status = battery.getIntExtra(BatteryManager.EXTRA_STATUS, 0).toLong()
    }

    return longArrayOf(capacity, chargeCounter, currentAvg,
                       currentNow, status, remaining, plugged,
                       temp)
  }

  fun updateExtractedText(window: EmacsWindow, text: ExtractedText,
                          token: Int) {
    if (DEBUG_IC)
      Log.d(TAG, "updateExtractedText: @" + token + ", " + text)

    icBeginSynchronous()
    window.view.imManager.updateExtractedText(window.view,
                                              token, text)
    icEndSynchronous()
  }

  /* Document tree management functions.  These functions shouldn't be
     called before Android 5.0.  */

  /* Return an array of each document authority providing at least one
     tree URI that Emacs holds the rights to persistently access.  */

  fun getDocumentAuthorities(): Array<String> {
    val permissions: List<UriPermission>
    val allProviders: HashSet<String>

    permissions = resolver.getPersistedUriPermissions()
    allProviders = HashSet<String>()

    for (permission in permissions) {
      val uri: Uri = permission.getUri()

      if (DocumentsContract.isTreeUri(uri)
          && permission.isReadPermission())
        allProviders.add(uri.getAuthority()!!)
    }

    return allProviders.toArray(arrayOf<String>())
  }

  /* Start a file chooser activity to request access to a directory
     tree.

     Value is 1 if the activity couldn't be started for some reason,
     and 0 in any other case.  */

  fun requestDirectoryAccess(): Int {
    val task: FutureTask<Int>

    /* Return 1 if Android is too old to support this feature.  */

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP)
      return 1

    task = FutureTask<Int>(Callable<Int> {
      val activity: EmacsActivity
      val intent: Intent
      var id: Int
      var rc: Int

      /* Try to obtain an activity that will receive the response
         from the file chooser dialog.  */

      if (EmacsActivity.focusedActivities.isEmpty()) {
        /* If focusedActivities is empty then this dialog may
           have been displayed immediately after another popup
           dialog was dismissed.  Try the EmacsActivity to be
           focused.  */

        activity = (EmacsActivity.lastFocusedActivity
                    ?: /* Still no luck.  Return failure.  */
                    return@Callable 1)
      } else
        activity = EmacsActivity.focusedActivities.get(0)

      /* Now create the intent.  */
      intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE)
      rc = 1

      try {
        id = EmacsActivity.ACCEPT_DOCUMENT_TREE
        activity.startActivityForResult(intent, id, null)
        rc = 0
      } catch (exception: Exception) {
        exception.printStackTrace()
      }

      rc
    })

    return EmacsService.syncRunnable<Int>(task)
  }

  /* Return an array of each tree provided by the document PROVIDER
     that Emacs has permission to access.

     Value is an array if the provider really does exist, NULL
     otherwise.  */

  fun getDocumentTrees(provider: String): Array<String> {
    val treeList: MutableList<String>
    val permissions: List<UriPermission>

    permissions = resolver.getPersistedUriPermissions()
    treeList = ArrayList<String>()

    for (permission in permissions) {
      val uri: Uri = permission.getUri()

      if (DocumentsContract.isTreeUri(uri)
          && uri.getAuthority().equals(provider)
          && permission.isReadPermission())
        /* Make sure the tree document ID is encoded.  Refrain from
           encoding characters such as +:&?#, since they don't
           conflict with file name separators or other special
           characters.  */
        treeList.add(Uri.encode(DocumentsContract.getTreeDocumentId(uri),
                                " +:&?#"))
    }

    /* The empty string array that is ostensibly allocated to provide
       the first argument provides just the type of the array to be
       returned.  */
    return treeList.toArray(arrayOf<String>())
  }

  /* Find the document ID of the file within TREE_URI designated by
     NAME.

     NAME is a ``file name'' comprised of the display names of
     individual files.  Each constituent component prior to the last
     must name a directory file within TREE_URI.

     Upon success, return 0 or 1 (contingent upon whether or not the
     last component within NAME is a directory) and place the document
     ID of the named file in ID_RETURN[0].

     If the designated file can't be located, but each component of
     NAME up to the last component can and is a directory, return -2
     and the ID of the last component located in ID_RETURN[0].

     If the designated file can't be located, return -1, or signal one
     of OperationCanceledException, SecurityException,
     FileNotFoundException, or UnsupportedOperationException.  */

  fun documentIdFromName(tree_uri: String, name: String,
                         id_return: Array<String?>): Int {
    /* Start the thread used to run SAF requests if it isn't already
       running.  */

    if (storageThread == null) {
      storageThread = EmacsSafThread(resolver)
      storageThread!!.start()
    }

    return storageThread!!.documentIdFromName(tree_uri, name,
                                              id_return)
  }

  /* Return an encoded document URI representing a tree with the
     specified IDENTIFIER supplied by the authority AUTHORITY.

     Return null instead if Emacs does not have permanent access
     to the specified document tree recorded on disk.  */

  fun getTreeUri(tree: String, authority: String): String? {
    var tree: String = Uri.decode(tree)
    val uri: Uri
    val permissions: List<UriPermission>

    /* First, build the URI.  */
    uri = DocumentsContract.buildTreeDocumentUri(authority, tree)

    /* Now, search for it within the list of persisted URI
       permissions.  */
    permissions = resolver.getPersistedUriPermissions()

    for (permission in permissions) {
      /* If the permission doesn't entitle Emacs to read access,
         skip it.  */

      if (!permission.isReadPermission())
        continue

      val grantedUri: Uri = permission.getUri()

      if (grantedUri.equals(uri))
        return uri.toString()
    }

    /* Emacs doesn't have permission to access this tree URI.  */
    return null
  }

  /* Return file status for the document designated by the given
     DOCUMENTID and tree URI.  If DOCUMENTID is NULL, use the document
     ID in URI itself.

     Value is null upon failure, or an array of longs [MODE, SIZE,
     MTIM] upon success, where MODE contains the file type and access
     modes of the file as in `struct stat', SIZE is the size of the
     file in BYTES or -1 if not known, and MTIM is the time of the
     last modification to this file in milliseconds since 00:00,
     January 1st, 1970.

     If NOCACHE, refrain from placing the file status within the
     status cache.

     OperationCanceledException and other typical exceptions may be
     signaled upon receiving async input or other errors.  */

  fun statDocument(uri: String, documentId: String?,
                   noCache: Boolean): LongArray? {
    /* Start the thread used to run SAF requests if it isn't already
       running.  */

    if (storageThread == null) {
      storageThread = EmacsSafThread(resolver)
      storageThread!!.start()
    }

    return storageThread!!.statDocument(uri, documentId, noCache)
  }

  /* Find out whether Emacs has access to the document designated by
     the specified DOCUMENTID within the tree URI.  If DOCUMENTID is
     NULL, use the document ID in URI itself.

     If WRITABLE, also check that the file is writable, which is true
     if it is either a directory or its flags contains
     FLAG_SUPPORTS_WRITE.

     Value is 0 if the file is accessible, and one of the following if
     not:

       -1, if the file does not exist.
       -2, if WRITABLE and the file is not writable.
       -3, upon any other error.

     In addition, arbitrary runtime exceptions (such as
     SecurityException or UnsupportedOperationException) may be
     thrown.  */

  fun accessDocument(uri: String, documentId: String?,
                     writable: Boolean): Int {
    /* Start the thread used to run SAF requests if it isn't already
       running.  */

    if (storageThread == null) {
      storageThread = EmacsSafThread(resolver)
      storageThread!!.start()
    }

    return storageThread!!.accessDocument(uri, documentId, writable)
  }

  /* Open a cursor representing each entry within the directory
     designated by the specified DOCUMENTID within the tree URI.

     If DOCUMENTID is NULL, use the document ID within URI itself.
     Value is NULL upon failure.

     In addition, arbitrary runtime exceptions (such as
     SecurityException or UnsupportedOperationException) may be
     thrown.  */

  fun openDocumentDirectory(uri: String, documentId: String?): Cursor? {
    /* Start the thread used to run SAF requests if it isn't already
       running.  */

    if (storageThread == null) {
      storageThread = EmacsSafThread(resolver)
      storageThread!!.start()
    }

    return storageThread!!.openDocumentDirectory(uri, documentId)
  }

  /* Read a single directory entry from the specified CURSOR.  Return
     NULL if at the end of the directory stream, and a directory entry
     with `d_name' set to NULL if an error occurs.  */

  fun readDirectoryEntry(cursor: Cursor): EmacsDirectoryEntry? {
    val entry: EmacsDirectoryEntry

    entry = EmacsDirectoryEntry()

    while (true) {
      if (!cursor.moveToNext())
        return null

      /* First, retrieve the display name.  */
      var index: Int = cursor.getColumnIndex(Document.COLUMN_DISPLAY_NAME)

      if (index < 0)
        /* Return an invalid directory entry upon failure.  */
        return entry

      val name: String?

      try {
        name = cursor.getString(index)
      } catch (exception: Exception) {
        return entry
      }

      /* Skip this entry if its name cannot be represented.  NAME
         can still be null here, since some Cursors are permitted to
         return NULL if INDEX is not a string.  */

      if (name == null || name == ".."
          || name == "." || name.contains("/")
          || name.contains("\u0000"))
        continue

      /* Now, look for its type.  */

      index = cursor.getColumnIndex(Document.COLUMN_MIME_TYPE)

      if (index < 0)
        /* Return an invalid directory entry upon failure.  */
        return entry

      val type: String?

      try {
        type = cursor.getString(index)
      } catch (exception: Exception) {
        return entry
      }

      if (type != null
          && type == Document.MIME_TYPE_DIR)
        entry.d_type = 1
      entry.d_name = name
      return entry
    }
  }

  /* Open a file descriptor for a file document designated by
     DOCUMENTID within the document tree identified by URI.  If
     TRUNCATE and the document already exists, truncate its contents
     before returning.

     If READ && WRITE, open the file under either the `rw' or `rwt'
     access mode, which implies that the value must be a seekable
     on-disk file.  If TRUNC && WRITE, also truncate the file after it
     is opened.

     If only READ or WRITE is set, value may be a non-seekable FIFO or
     one end of a socket pair.

     Value is NULL upon failure or a parcel file descriptor upon
     success.  Call `ParcelFileDescriptor.close' on this file
     descriptor instead of using the `close' system call.

     FileNotFoundException and/or SecurityException and
     UnsupportedOperationException may be thrown upon failure.  */

  fun openDocument(uri: String, documentId: String,
                   read: Boolean, write: Boolean, truncate: Boolean): ParcelFileDescriptor? {
    /* Start the thread used to run SAF requests if it isn't already
       running.  */

    if (storageThread == null) {
      storageThread = EmacsSafThread(resolver)
      storageThread!!.start()
    }

    return storageThread!!.openDocument(uri, documentId, read, write,
                                        truncate)
  }

  /* Create a new document with the given display NAME within the
     directory identified by DOCUMENTID inside the document tree
     designated by URI.

     If DOCUMENTID is NULL, create the document inside the root of
     that tree.

     Either FileNotFoundException, SecurityException or
     UnsupportedOperationException may be thrown upon failure.

     Return the document ID of the new file upon success, NULL
     otherwise.  */

  @Throws(FileNotFoundException::class)
  fun createDocument(uri: String, documentId: String?, name: String): String? {
    var mimeType: String
    var documentId: String? = documentId
    var mime: String?
    val extension: String
    var index: Int
    val singleton: MimeTypeMap
    val treeUri: Uri
    val directoryUri: Uri
    val docUri: Uri?

    /* Try to get the MIME type for this document.
       Default to ``application/octet-stream''.  */

    mimeType = "application/octet-stream"

    /* Abuse WebView stuff to get the file's MIME type.  */

    index = name.lastIndexOf('.')

    if (index > 0) {
      singleton = MimeTypeMap.getSingleton()
      extension = name.substring(index + 1)
      mime = singleton.getMimeTypeFromExtension(extension)

      if (mime != null)
        mimeType = mime
    }

    /* Now parse URI.  */
    treeUri = Uri.parse(uri)

    if (documentId == null)
      documentId = DocumentsContract.getTreeDocumentId(treeUri)

    /* And build a file URI referring to the directory.  */

    directoryUri =
      DocumentsContract.buildChildDocumentsUriUsingTree(treeUri,
                                                        documentId)

    docUri = DocumentsContract.createDocument(resolver,
                                             directoryUri,
                                             mimeType, name)

    if (docUri == null)
      return null

    /* Invalidate the file status of the containing directory.  */

    if (storageThread != null)
      storageThread!!.postInvalidateStat(treeUri, documentId)

    /* Return the ID of the new document.  */
    return DocumentsContract.getDocumentId(docUri)
  }

  /* Like `createDocument', but create a directory instead of an
     ordinary document.  */

  @Throws(FileNotFoundException::class)
  fun createDirectory(uri: String, documentId: String?, name: String): String? {
    var documentId: String? = documentId
    var name: String = name
    val treeUri: Uri
    val directoryUri: Uri
    val docUri: Uri?

    /* Now parse URI.  */
    treeUri = Uri.parse(uri)

    if (documentId == null)
      documentId = DocumentsContract.getTreeDocumentId(treeUri)

    /* And build a file URI referring to the directory.  */

    directoryUri =
      DocumentsContract.buildChildDocumentsUriUsingTree(treeUri,
                                                        documentId)

    /* If name ends with a directory separator character, delete
       it.  */

    if (name.endsWith("/"))
      name = name.substring(0, name.length - 1)

    /* From Android's perspective, directories are just ordinary
       documents with the `MIME_TYPE_DIR' type.  */

    docUri = DocumentsContract.createDocument(resolver,
                                             directoryUri,
                                             Document.MIME_TYPE_DIR,
                                             name)

    if (docUri == null)
      return null

    /* Return the ID of the new document, but first invalidate the
       state of the containing directory.  */

    if (storageThread != null)
      storageThread!!.postInvalidateStat(treeUri, documentId)

    return DocumentsContract.getDocumentId(docUri)
  }

  /* Delete the document identified by ID from the document tree
     identified by URI.  Return 0 upon success and -1 upon
     failure.

     NAME should be the name of the document being deleted, and is
     used to invalidate the cache.  */

  @Throws(FileNotFoundException::class)
  fun deleteDocument(uri: String, id: String, name: String): Int {
    val uriObject: Uri
    val tree: Uri

    tree = Uri.parse(uri)
    uriObject = DocumentsContract.buildDocumentUriUsingTree(tree, id)

    if (DocumentsContract.deleteDocument(resolver, uriObject)) {
      if (storageThread != null)
        storageThread!!.postInvalidateCache(tree, id, name)

      return 0
    }

    return -1
  }

  /* Rename the document designated by DOCID inside the directory tree
     identified by URI, which should be within the directory
     designated by DIR, to NAME.  If the file can't be renamed because
     it doesn't support renaming, return -1, 0 otherwise.  */

  @Throws(FileNotFoundException::class)
  fun renameDocument(uri: String, docId: String, dir: String, name: String): Int {
    val tree: Uri
    val uriObject: Uri

    tree = Uri.parse(uri)
    uriObject = DocumentsContract.buildDocumentUriUsingTree(tree, docId)

    if (DocumentsContract.renameDocument(resolver, uriObject,
                                         name)
        != null) {
      /* Invalidate the cache.  */
      if (storageThread != null)
        storageThread!!.postInvalidateCacheDir(tree, docId,
                                               name)
      return 0
    }

    /* Handle errors specially, so `android_saf_rename_document' can
       return ENXDEV.  */
    return -1
  }

  /* Move the document designated by DOCID from the directory under
     DIR_NAME designated by SRCID to the directory designated by
     DSTID.  If the ID of the document being moved changes as a
     consequence of the movement, return the new ID, else NULL.

     URI is the document tree containing all three documents.  */

  @Throws(FileNotFoundException::class)
  fun moveDocument(uri: String, docId: String, dirName: String,
                   dstId: String, srcId: String): String? {
    val uri1: Uri
    val docId1: Uri
    val dstId1: Uri
    val srcId1: Uri
    val name: Uri?

    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N)
      throw UnsupportedOperationException("Documents aren't capable"
                                          + " of being moved on Android"
                                          + " versions before 7.0.")

    uri1 = Uri.parse(uri)
    docId1 = DocumentsContract.buildDocumentUriUsingTree(uri1, docId)
    dstId1 = DocumentsContract.buildDocumentUriUsingTree(uri1, dstId)
    srcId1 = DocumentsContract.buildDocumentUriUsingTree(uri1, srcId)

    /* Move the document; this function returns the new ID of the
       document should it change.  */
    name = DocumentsContract.moveDocument(resolver, docId1,
                                          srcId1, dstId1)

    /* Now invalidate the caches for both DIRNAME and DOCID.  */

    if (storageThread != null) {
      storageThread!!.postInvalidateCacheDir(uri1, docId, dirName)

      /* Invalidate the stat cache entries for both the source and
         destination directories, since their contents have
         changed.  */
      storageThread!!.postInvalidateStat(uri1, dstId)
      storageThread!!.postInvalidateStat(uri1, srcId)
    }

    return (if (name != null)
              DocumentsContract.getDocumentId(name)
            else
              null)
  }

  /* Return if there is a content provider by the name of AUTHORITY
     supplying at least one tree URI Emacs retains persistent rights
     to access.  */

  fun validAuthority(authority: String): Boolean {
    val permissions: List<UriPermission>

    permissions = resolver.getPersistedUriPermissions()

    for (permission in permissions) {
      val uri: Uri = permission.getUri()

      if (DocumentsContract.isTreeUri(uri)
          && permission.isReadPermission()
          && uri.getAuthority().equals(authority))
        return true
    }

    return false
  }

  /* Relinquish authorization for read and write access to the provided
     URI, which is generally a reference to a directory tree.  */

  fun relinquishUriRights(uri: String) {
    val uri1: Uri
    val flags: Int

    uri1 = Uri.parse(uri)
    flags = (Intent.FLAG_GRANT_READ_URI_PERMISSION
             or Intent.FLAG_GRANT_WRITE_URI_PERMISSION)
    resolver.releasePersistableUriPermission(uri1, flags)
  }

  /* Functions for detecting and requesting storage permissions.  */

  fun externalStorageAvailable(): Boolean {
    val readPermission: String

    readPermission = "android.permission.READ_EXTERNAL_STORAGE"

    return (if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R)
              checkSelfPermission(readPermission)
                == PackageManager.PERMISSION_GRANTED
            else
              Environment.isExternalStorageManager())
  }

  private fun requestStorageAccess23() {
    val runnable: Runnable

    runnable = Runnable {
      val activity: EmacsActivity
      val permission: String
      val permission1: String

      permission = "android.permission.READ_EXTERNAL_STORAGE"
      permission1 = "android.permission.WRITE_EXTERNAL_STORAGE"

      /* Find an activity that is entitled to display a permission
         request dialog.  */

      if (EmacsActivity.focusedActivities.isEmpty()) {
        /* If focusedActivities is empty then this dialog may
           have been displayed immediately after another popup
           dialog was dismissed.  Try the EmacsActivity to be
           focused.  */

        activity = (EmacsActivity.lastFocusedActivity
                    ?: /* Still no luck.  Return failure.  */
                    return@Runnable)
      } else
        activity = EmacsActivity.focusedActivities.get(0)

      /* Now request these permissions.  */
      activity.requestPermissions(arrayOf(permission,
                                          permission1),
                                  0)
    }

    runOnUiThread(runnable)
  }

  private fun requestStorageAccess30() {
    val runnable: Runnable
    val intent: Intent

    intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION,
                    Uri.parse("package:org.gnu.emacs"))

    runnable = Runnable {
      val activity: EmacsActivity

      /* Find an activity that is entitled to display a permission
         request dialog.  */

      if (EmacsActivity.focusedActivities.isEmpty()) {
        /* If focusedActivities is empty then this dialog may
           have been displayed immediately after another popup
           dialog was dismissed.  Try the EmacsActivity to be
           focused.  */

        activity = (EmacsActivity.lastFocusedActivity
                    ?: /* Still no luck.  Return failure.  */
                    return@Runnable)
      } else
        activity = EmacsActivity.focusedActivities.get(0)

      /* Now request these permissions.  */

      try {
        activity.startActivity(intent)
      } catch (exception: ActivityNotFoundException) {
        Log.w(TAG, "Failed to request storage access permissions: ")
        exception.printStackTrace()
      }
    }

    runOnUiThread(runnable)
  }

  fun requestStorageAccess() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R)
      requestStorageAccess23()
    else
      requestStorageAccess30()
  }

  /* Notification miscellany.  */

  /* Cancel any notification displayed with the tag TAG.  */

  fun cancelNotification(string: String) {
    val tem: Any?
    val manager: NotificationManager

    tem = getSystemService(Context.NOTIFICATION_SERVICE)
    manager = tem as NotificationManager

    runOnUiThread(Runnable {
      manager.cancel(string, 2)
    })
  }
}
