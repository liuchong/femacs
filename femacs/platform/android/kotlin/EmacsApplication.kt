// femacs: port of java/org/gnu/emacs/EmacsApplication.java @ fd7afced403.

package org.gnu.emacs

import android.app.Application
import android.content.Context
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.content.pm.PackageManager.ApplicationInfoFlags
import android.os.Build
import android.util.Log
import java.io.File
import java.io.FileFilter

/* Java's two `public static` fields become companion `@JvmField`s - which is what keeps the JVM
   static field of the same name that `EmacsThread` reads (`EmacsApplication.dumpFileName`); the
   static `findDumpFile` becomes a companion function with `@JvmStatic` (P-83 ②⑪).  The anonymous
   `FileFilter` is Kotlin's SAM literal, the same single-method interface (P-83 ①).  */
class EmacsApplication : Application() {
  private fun getApkFile(): String? {
    val manager: PackageManager
    val info: ApplicationInfo

    manager = packageManager

    try {
      info =
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU)
          manager.getApplicationInfo("org.gnu.emacs", 0)
        else
          manager.getApplicationInfo("org.gnu.emacs", ApplicationInfoFlags.of(0))

      /* Return an empty string upon failure.  */

      if (info.sourceDir != null)
        return info.sourceDir

      return null
    } catch (e: Exception) {
      return null
    }
  }

  override fun onCreate() {
    /* Block signals which don't interest the current thread and its
       descendants created by the system.  The original signal mask
       will be restored for the Emacs thread in `initEmacs'.  */
    EmacsNative.setupSystemThread()

    /* Establish the name of the APK.  */
    apkFileName = getApkFile()

    /* Locate a suitable dump file.  */
    findDumpFile(this)

    /* Start the rest of the application.  */
    super.onCreate()
  }

  companion object {
    private const val TAG: String = "EmacsApplication"

    /* The name of the dump file to use, or NULL if this Emacs binary
       has yet to be dumped.  */
    @JvmField var dumpFileName: String? = null

    /* The name of the APK file housing Emacs, or NULL if it could not
       be ascertained.  */
    @JvmField var apkFileName: String? = null

    @JvmStatic fun findDumpFile(context: Context) {
      val filesDirectory: File
      var apk: File
      val allFiles: Array<File>?
      val wantedDumpFile: String
      var i: Int

      wantedDumpFile = "emacs-" + EmacsNative.getFingerprint() + ".pdmp"

      /* Obtain a list of all files ending with ``.pdmp''.  Then, look
         for a file named ``emacs-<fingerprint>.pdmp'' and delete the
         rest.  */
      filesDirectory = context.filesDir

      allFiles = filesDirectory.listFiles(FileFilter { file ->
        !file.isDirectory && file.name.endsWith(".pdmp")
      })

      if (allFiles == null)
        return

      /* Now try to find the right dump file.  */
      for (i in allFiles.indices) {
        if (allFiles[i].name == wantedDumpFile) {
          /* Compare the last modified time of the dumpfile with
             that of apkFileName, the time at which Emacs was
             installed.  Delete it if the dump file was created
             before Emacs was installed, even if the C signature
             (representing libemacs.so) remains identical.  */

          if (apkFileName != null) {
            apk = File(apkFileName!!)

            if (apk.lastModified() > allFiles[i].lastModified()) {
              allFiles[i].delete()

              /* Don't set the dump file name in this case.  */
              continue
            }
          }

          dumpFileName = allFiles[i].absolutePath
        } else
          /* Delete this outdated dump file.  */
          allFiles[i].delete()
      }
    }
  }
}
