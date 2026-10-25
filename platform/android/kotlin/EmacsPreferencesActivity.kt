// femacs: port of java/org/gnu/emacs/EmacsPreferencesActivity.java @ fd7afced403.

package org.femacs

import java.io.File

import android.content.Intent

import android.os.Bundle
import android.os.Build

import android.view.View

import android.widget.Toast

import android.preference.*

/* This module provides a ``preferences'' display for Emacs.  It is
   supposed to be launched from inside the Settings application to
   perform various actions, such as starting Emacs with the ``-Q''
   option, which would not be possible otherwise, as there is no
   command line on Android.

   This file extends a deprecated preferences activity, but no suitable
   alternative exists that is identical in appearance to system settings
   forms.  */

/* The reference's `@SuppressWarnings ("deprecation")` becomes Kotlin's `@Suppress`, which splits
   that diagnostic in two: `DEPRECATION` for the deprecated `android.preference` API and
   `OVERRIDE_DEPRECATION` for the members that override deprecated declarations (`onCreate` and
   the three listeners' `onPreferenceClick`) - measured: `DEPRECATION` alone leaves four
   warnings, so both names are suppressed.  `onCreate` is `final` in the reference, so the port
   writes `final override` (P-83 ⑫ - Kotlin's overrides are open by default); the class is
   non-final there and `EmacsLauncherPreferencesActivity` extends it, so the port is `open`
   (P-83 ⑦ - measured).  Java's three anonymous `Preference.OnPreferenceClickListener` classes
   are Kotlin **SAM literals**, the single-method-interface shape `EmacsApplication`'s
   `FileFilter` port established (P-83 ①), so the `preference` parameter they never read has no
   Kotlin spelling.  */
@Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
open class EmacsPreferencesActivity : PreferenceActivity() {
  /* Restart Emacs with -Q.  Call EmacsThread.exit to kill Emacs now,
     and tell the system to start EmacsActivity with some parameters
     later.  */

  private fun startEmacsQ() {
    val intent: Intent

    intent = Intent(this, EmacsActivity::class.java)
    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK
                    or Intent.FLAG_ACTIVITY_CLEAR_TASK)
    intent.putExtra(EmacsActivity.EXTRA_STARTUP_ARGUMENTS,
                    arrayOf("--quick"))
    startActivity(intent)
    System.exit(0)
  }

  /* Restart Emacs with `--debug-init'.  Call EmacsThread.exit to kill
     Emacs now, and tell the system to EmacsActivity with some
     parameters later.  */

  private fun startEmacsDebugInit() {
    val intent: Intent

    intent = Intent(this, EmacsActivity::class.java)
    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK
                    or Intent.FLAG_ACTIVITY_CLEAR_TASK)
    intent.putExtra(EmacsActivity.EXTRA_STARTUP_ARGUMENTS,
                    arrayOf("--debug-init"))
    startActivity(intent)
    System.exit(0)
  }

  /* Erase Emacs's dump file.  */

  private fun eraseDumpFile() {
    val wantedDumpFile: String
    val file: File
    val toast: Toast

    wantedDumpFile = ("emacs-" + EmacsNative.getFingerprint()
                      + ".pdmp")
    file = File(filesDir, wantedDumpFile)

    if (file.exists())
      file.delete()

    /* Make sure to clear EmacsApplication.dumpFileName, or
       starting Emacs without restarting this program will
       make Emacs try to load a nonexistent dump file.  */
    EmacsApplication.dumpFileName = null

    /* Display a message stating that the dump file has been
       erased.  */
    toast = Toast.makeText(this, "Dump file removed",
                           Toast.LENGTH_SHORT)
    toast.show()
  }

  final override fun onCreate(savedInstanceState: Bundle?) {
    var tem: Preference
    /* The reference reassigns `listener` three times (once per preference item); Kotlin locals
       are immutable by default, so the port declares it `var` (the same treatment Java's
       reassigned parameters get).  */
    var listener: Preference.OnPreferenceClickListener
    val view: View

    setTheme(android.R.style.Theme_DeviceDefault_Settings)

    /* This must come before using any preference APIs.  */
    super.onCreate(savedInstanceState)

    /* Add preferences from the XML file where they are defined.  */
    addPreferencesFromResource(R.xml.preferences)

    /* Now, set up on click handlers for each of the preferences
       items.  */

    tem = findPreference("start_quick")
    listener = Preference.OnPreferenceClickListener {
      startEmacsQ()
      true
    }

    tem.setOnPreferenceClickListener(listener)
    tem = findPreference("start_debug_init")
    listener = Preference.OnPreferenceClickListener {
      startEmacsDebugInit()
      true
    }

    tem.setOnPreferenceClickListener(listener)
    tem = findPreference("erase_dump")
    listener = Preference.OnPreferenceClickListener {
      eraseDumpFile()
      true
    }

    tem.setOnPreferenceClickListener(listener)

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.VANILLA_ICE_CREAM) {
      /* Align the list view to system windows, or they will be
         obstructed by the title bar.  */
      view = this.listView
      view.setFitsSystemWindows(true)
    }
  }
}
