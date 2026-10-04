// femacs: port of java/org/gnu/emacs/EmacsNoninteractive.java @ fd7afced403.

package org.gnu.emacs

import android.os.Looper
import android.os.Build

import android.content.Context
import android.content.res.AssetManager

import java.lang.reflect.Method

/* Noninteractive Emacs.

   When started, libandroid-emacs.so invokes `app_process(64)' with a
   command line placing Emacs's classes.dex file in the JVM class path,
   which in turn transfers control to `main'.  `main' creates a context,
   which may be likened to a connection to the system server, and a
   class loader derived from Emacs's application package, which it loads
   beforehand.  From this class loader, it loads another instance of
   itself, and invokes `main1', to ensure the execution of
   `EmacsNative''s static initializer within the application class
   loader, where a proper library search path is in effect.  */

/* Three name-based consumers touch this class, so its names and shapes are the reference's:
   the JVM launcher calls `main`, `main` itself reflectively looks up `main1` with
   `(String[], Context)` in the class loader it acquired, and `EmacsNative`'s static initializer
   must run in the application class loader.  `public static` therefore becomes a companion
   function with `@JvmStatic` (the shape measured for `EmacsNative`'s natives, P-83 ⑪), and the
   reference's `throws Exception` is `@Throws` (P-83 ④).  Java's raw `Class` is Kotlin's
   `Class<*>`; the reference's `int.class`/`String[].class` arguments are
   `Int::class.javaPrimitiveType`/`Array<String>::class.java` (measured below), and Java's
   `@SuppressWarnings ("unchecked")` is `@Suppress ("UNCHECKED_CAST")`.  The reference imports
   `java.lang.reflect.Constructor` without using it, so it is not carried.  */
@Suppress("UNCHECKED_CAST")
class EmacsNoninteractive {
  companion object {
    /* Prepare Emacs for startup and call `initEmacs'.  This function is
       called in an instance of `EmacsNoninteractive' loaded by the APK
       ClassLoader acquired in `main', which guarantees that shared
       libraries in the APK will be considered in resolving shared
       libraries for `EmacsNative'.  */

    @JvmStatic
    @Throws(Exception::class)
    fun main1(args: Array<String>, context: Context) {
      val assets: AssetManager
      val filesDir: String
      val libDir: String
      val cacheDir: String

      /* Don't actually start the looper or anything.  Instead, obtain
         an AssetManager.  */
      assets = context.getAssets()

      /* Now configure Emacs.  The class path should already be set.  */

      filesDir = context.getFilesDir().getCanonicalPath()
      libDir = EmacsService.getLibraryDirectory(context)
      cacheDir = context.getCacheDir().getCanonicalPath()
      EmacsNative.setEmacsParams(assets, filesDir,
                                 libDir, cacheDir, 0.0f,
                                 0.0f, 0.0f, 0x0, null, null,
                                 Build.VERSION.SDK_INT)

      /* Now find the dump file that Emacs should use, if it has already
         been dumped.  */
      EmacsApplication.findDumpFile(context)

      /* Start Emacs.  */
      EmacsNative.initEmacs(args, EmacsApplication.dumpFileName)
    }

    @JvmStatic
    fun main(args: Array<String>) {
      var activityThread: Any?
      var loadedApk: Any?
      var activityThreadClass: Class<*>
      var loadedApkClass: Class<*>?
      var contextImplClass: Class<*>
      var compatibilityInfoClass: Class<*>
      var emacsNoninteractiveClass: Class<*>
      var method: Method
      var context: Context?
      var classLoader: ClassLoader?

      Looper.prepare()

      context = null
      loadedApkClass = null
      classLoader = null

      try {
        /* Get the activity thread.  */
        activityThreadClass = Class.forName("android.app.ActivityThread")

        /* Get the systemMain method.  */
        method = activityThreadClass.getMethod("systemMain")

        /* Create and attach the activity thread.  */
        activityThread = method.invoke(null)

        /* Now get an LoadedApk.  */

        try {
          loadedApkClass = Class.forName("android.app.LoadedApk")
        } catch (exception: ClassNotFoundException) {
          /* Android 2.2 has no LoadedApk class; the several following
             statements will load a context and an
             ActivityThread.PackageInfo, as is appropriate on this
             system.  */
        }

        /* Get a LoadedApk or ActivityThread.PackageInfo.  How to do
           this varies by Android version.  On Android 3.0 and earlier,
           there is no ``compatibilityInfo'' argument to
           getPackageInfo.  */

        try {
          method = activityThreadClass.getMethod("getPackageInfo",
                                                  String::class.java,
                                                  Int::class.javaPrimitiveType)
          loadedApk = method.invoke(activityThread, "org.gnu.emacs",
                                    (Context.CONTEXT_INCLUDE_CODE
                                     or Context.CONTEXT_IGNORE_SECURITY))
        } catch (exception: NoSuchMethodException) {
          compatibilityInfoClass = Class.forName("android.content.res.CompatibilityInfo")

          method = activityThreadClass.getMethod("getPackageInfo",
                                                  String::class.java,
                                                  compatibilityInfoClass,
                                                  Int::class.javaPrimitiveType)
          loadedApk = method.invoke(activityThread, "org.gnu.emacs",
                                    null, (Context.CONTEXT_INCLUDE_CODE
                                           or Context.CONTEXT_IGNORE_SECURITY))
        }

        if (loadedApk == null)
          throw RuntimeException("getPackageInfo returned NULL")

        /* If loadedApkClass remains NULL, substitute the class of
           the object returned by getPackageInfo.  */
        if (loadedApkClass == null)
          loadedApkClass = loadedApk.javaClass

        /* Now, get a context.  */
        contextImplClass = Class.forName("android.app.ContextImpl")

        try {
          method = contextImplClass.getDeclaredMethod("createAppContext",
                                                      activityThreadClass,
                                                      loadedApkClass)
          method.setAccessible(true)
          context = method.invoke(null, activityThread,
                                  loadedApk) as Context
        } catch (exception: NoSuchMethodException) {
          /* Older Android versions don't have createAppContext, but
             instead require creating a ContextImpl, and then
             calling createPackageContext.  */
          method = activityThreadClass.getDeclaredMethod("getSystemContext")
          context = method.invoke(activityThread) as Context
          method = contextImplClass.getDeclaredMethod("createPackageContext",
                                                      String::class.java,
                                                      Int::class.javaPrimitiveType)
          method.setAccessible(true)
          context = method.invoke(context, "org.gnu.emacs",
                                  0) as Context
        }

        /* Retrieve the LoadedApk's class loader and execute the
           remaining portion of the start-up process within its version
           of EmacsNoninteractive, which will indicate to the system
           that it must load shared libraries from the APK's library
           search path.  */

        method = loadedApkClass.getDeclaredMethod("getClassLoader")
        classLoader = method.invoke(loadedApk) as ClassLoader
      } catch (e: Exception) {
        System.err.println("Internal error: " + e)
        System.err.println("This means that the Android platform changed,")
        System.err.println("and that Emacs needs adjustments in order to")
        System.err.println("obtain required system internal resources.")
        System.err.println("Please report this bug to bug-gnu-emacs@gnu.org.")
        e.printStackTrace()

        System.exit(1)
      }

      try {
        emacsNoninteractiveClass = classLoader!!.loadClass("org.gnu.emacs.EmacsNoninteractive")
        method = emacsNoninteractiveClass.getMethod("main1", Array<String>::class.java,
                                                    Context::class.java)
        method.setAccessible(true)
        method.invoke(null, args, context)
      } catch (e: Exception) {
        System.err.println("Internal error during startup: " + e)
        e.printStackTrace()
        System.exit(1)
      }
    }
  }
}
