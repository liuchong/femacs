// femacs: port of java/org/gnu/emacs/EmacsLauncherPreferencesActivity.java @ fd7afced403.

package org.gnu.emacs

/* This class only exists because EmacsPreferencesActivity is already
   defined as an activity, the system wants a new class in order to
   define a new activity, and only activities can be enabled or
   disabled per the API level of the host.  */

class EmacsLauncherPreferencesActivity : EmacsPreferencesActivity()
