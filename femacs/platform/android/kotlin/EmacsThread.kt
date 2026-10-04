// femacs: port of java/org/gnu/emacs/EmacsThread.java @ fd7afced403.

package org.gnu.emacs

import android.util.Log
import java.util.Arrays

/* `Arrays.toString` is kept as the reference's call rather than replaced with Kotlin's
   `contentToString`, because the log line is the kind of text a runtime comparison would look at
   (P-83 ①).  */
class EmacsThread(service: EmacsService, paramsClosure: Runnable,
                  extraStartupArguments: Array<String>?) : Thread("Emacs main thread") {
  /* Whether or not Emacs should be started with additional arguments,
     and those additional arguments if non-NULL.  */
  private val extraStartupArguments: Array<String>? = extraStartupArguments

  /* Runnable run to initialize Emacs.  */
  private val paramsClosure: Runnable = paramsClosure

  override fun run() {
    val args: Array<String>

    if (extraStartupArguments == null)
      args = arrayOf("libandroid-emacs.so")
    else {
      /* Prepend "libandroid-emacs.so" to the list of arguments.  */
      args = arrayOfNulls<String>(extraStartupArguments.size + 1) as Array<String>
      args[0] = "libandroid-emacs.so"
      System.arraycopy(extraStartupArguments, 0, args, 1, extraStartupArguments.size)
    }

    paramsClosure.run()

    /* Run the native code now.  */
    Log.d(TAG, "run: " + Arrays.toString(args))
    EmacsNative.initEmacs(args, EmacsApplication.dumpFileName)
  }

  companion object {
    private const val TAG: String = "EmacsThread"
  }
}
