// femacs: port of java/org/gnu/emacs/EmacsMultitaskActivity.java @ fd7afced403.

package org.femacs

import android.content.Intent
import android.os.Bundle

/* In large measure, this class only exists because EmacsActivity is
   already defined as an activity, and the system requires that every
   new activity be defined by a new class.  */

/* The reference marks both overrides `final`; Kotlin's overriding members are open unless marked, so
   `final override` keeps the reference's shape (P-83 ⑫).  */
class EmacsMultitaskActivity : EmacsActivity() {
  /* Token provided by the creator.  */
  private var activityToken: Long = 0

  final override fun onCreate(savedInstanceState: Bundle?) {
    val intent: Intent?
    val token: String

    intent = getIntent()
    token = EmacsWindowManager.ACTIVITY_TOKEN

    if (intent != null)
      activityToken = intent.getLongExtra(token, -2)

    super.onCreate(savedInstanceState)
  }

  final override fun getAttachmentToken(): Long {
    return activityToken
  }
}
