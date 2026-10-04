// femacs: port of java/org/gnu/emacs/EmacsDesktopNotification.java @ fd7afced403.

package org.gnu.emacs

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

import android.net.Uri

import android.os.Build

import android.widget.RemoteViews

/* Structure designating a single desktop notification.

   New versions of Android also organize notifications into individual
   ``channels'', which are used to implement groups.  Unlike on other
   systems, notification importance is set for each group, not for
   each individual notification.  */

/* `androidselect.c` 664-693 finds this class by name and calls two of its members by name and
   descriptor - `<init>` `(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;`
   `Ljava/lang/String;II[Ljava/lang/String;[Ljava/lang/String;J)V` (689-692) and `display` `()V`
   (693) - so the port's primary constructor keeps the reference's parameter list and thus that
   descriptor, and `display` stays a `fun` (P-83 ②).  The reference's class is `final`, so the
   port is a plain `class`.

   `NOTIFICATION_TAG`/`NOTIFICATION_ACTION` have to be JVM static fields rather than Kotlin
   companion properties: the already-ported `EmacsActivity.kt` 560-561 reads both by name
   (`intent.getStringExtra (EmacsDesktopNotification.NOTIFICATION_TAG)`), which is a `getstatic`
   on `EmacsDesktopNotification` itself, so the three constants are companion `@JvmField`s
   (P-83 ②, the `EmacsActivity.EXTRA_STARTUP_ARGUMENTS` shape).

   The reference's `public final` fields are declared without initializers and assigned in `init`
   at the reference's own points, which is what keeps Java's field-declaration order (content
   before title) and the assignment order (P-83 ⑨, the `EmacsPixmap.width/height/depth` shape).
   `actions`/`titles` are nullable because `androidselect.c` 849-889 creates the two arrays only
   when the action list is non-empty and passes `NULL` for both otherwise, and the reference tests
   `actions` for null there (P-83 ⑤); `insertActions` narrows both through same-named locals
   (P-83 ⑱), and `titles[i]` keeps its untested dereference as `!!` (P-83 ⑤).  `title`, `content`,
   `group` and `tag` are non-null: `androidselect.c` 839-841 builds them with
   `android_build_string`, which returns a string except after an OOM abort, and the reference
   never tests them (P-83 ⑤).

   The reference's `@SuppressWarnings ("deprecation")` on `insertActions` and `display1` becomes
   `@Suppress("DEPRECATION")` (P-83 ④) - neither member overrides anything, so there is no
   `OVERRIDE_DEPRECATION` diagnostic to silence, and it is what covers the deprecated
   `Notification.Builder (Context)`, `addAction (int, CharSequence, PendingIntent)`,
   `getNotification`, `setPriority`, `Notification.PRIORITY_*` and `Notification ()`'s fields.
   Java's unused local `String action;` in `CancellationReceiver.onReceive` is not carried (Kotlin
   warns on an unused local - the `EmacsActivity.isReallyFinishing` precedent), and the
   `Object tem;` temp of `display1` is kept because its own name is not reserved, while the
   `RemoteViews contentView` local is declared non-null and assigned before its uses, which is how
   Java's `notification.contentView = contentView = new RemoteViews (...)` chain is written under
   Kotlin's assignment-is-not-an-expression rule (P-83 ⑬).  */
class EmacsDesktopNotification(title: String, content: String,
                               group: String, tag: String, icon: Int,
                               importance: Int, actions: Array<String>?,
                               titles: Array<String>?, delay: Long) {
  companion object {
    /* Intent tag for notification action data.  */
    @JvmField val NOTIFICATION_ACTION: String = "emacs:notification_action"

    /* Intent tag for notification IDs.  */
    @JvmField val NOTIFICATION_TAG: String = "emacs:notification_tag"

    /* Action ID assigned to the broadcast receiver which should be
       notified of any notification's being dismissed.  */
    @JvmField val NOTIFICATION_DISMISSED: String = "org.gnu.emacs.DISMISSED"
  }

  /* The content of this desktop notification.  */
  @JvmField val content: String

  /* The title of this desktop notification.  */
  @JvmField val title: String

  /* The notification group.  */
  @JvmField val group: String

  /* String identifying this notification for future replacement.
     Typically a string resembling ``XXXX.NNNN.YYYY'', where XXXX is
     the system boot time, NNNN is the PID of this Emacs instance, and
     YYYY is the counter value returned by the notifications display
     function.  */
  @JvmField val tag: String

  /* The identifier of this notification's icon.  */
  @JvmField val icon: Int

  /* The importance of this notification's group.  */
  @JvmField val importance: Int

  /* Array of actions and their user-facing text to be offered by this
     notification.  */
  @JvmField val actions: Array<String>?
  @JvmField val titles: Array<String>?

  /* Delay in milliseconds after which this notification should be
     automatically dismissed.  */
  @JvmField val delay: Long

  init {
    this.content    = content
    this.title      = title
    this.group      = group
    this.tag        = tag
    this.icon       = icon
    this.importance = importance
    this.actions    = actions
    this.titles     = titles
    this.delay      = delay
  }

  /* Functions for displaying desktop notifications.  */

  /* Insert each action in actions and titles into the notification
     builder BUILDER, with pending intents created with CONTEXT holding
     suitable metadata.  */

  @Suppress("DEPRECATION")
  private fun insertActions(context: Context, builder: Notification.Builder) {
    var pending: PendingIntent
    var intent: Intent
    var action: Notification.Action.Builder

    /* Java reads the `actions`/`titles` fields directly; the same-named locals shadow the
       properties so the test below narrows the value (P-83 ⑱).  */
    val actions = this.actions
    val titles = this.titles

    if (actions == null)
      return

    for (i in actions.indices) {
      /* Actions named default should not be displayed.  */
      if (actions[i] == "default")
        continue

      intent = Intent(context, EmacsActivity::class.java)
      intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)

      /* Pending intents are specific to combinations of class, action
         and data, but not information provided as extras.  In order
         that its target may be invoked with the action and tag set
         below, generate a URL from those two elements and specify it
         as the intent data, which ensures that the intent allocated
         fully reflects the duo.  */

      intent.setData(Uri.Builder().scheme("action")
                       .appendPath(tag).appendPath(actions[i])
                       .build())
      intent.putExtra(NOTIFICATION_ACTION, actions[i])
      intent.putExtra(NOTIFICATION_TAG, tag)

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S)
        pending = PendingIntent.getActivity(context, 0, intent,
                                            PendingIntent.FLAG_IMMUTABLE)
      else
        pending = PendingIntent.getActivity(context, 0, intent, 0)

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
        action = Notification.Action.Builder(0, titles!![i], pending)
        builder.addAction(action.build())
      } else
        builder.addAction(0, titles!![i], pending)
    }
  }

  /* Internal helper for `display' executed on the main thread.  */

  @Suppress("DEPRECATION") /* Notification.Builder (Context).  */
  private fun display1(context: Context) {
    val manager: NotificationManager
    val channel: NotificationChannel
    var notification: Notification
    val tem: Any?
    var contentView: RemoteViews
    var intent: Intent
    var pending: PendingIntent
    var priority: Int
    var builder: Notification.Builder

    tem = context.getSystemService(Context.NOTIFICATION_SERVICE)
    manager = tem as NotificationManager

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      /* Create the notification channel for this group.  If a group
         already exists with the same name, its linked attributes
         (such as its importance) will be overridden.  */
      channel = NotificationChannel(group, group, importance)
      manager.createNotificationChannel(channel)
      builder = Notification.Builder(context, group)

      /* Create and configure a notification object and display
         it.  */

      builder.setContentTitle(title)
      builder.setContentText(content)
      builder.setSmallIcon(icon)
      builder.setTimeoutAfter(delay)

      insertActions(context, builder)
      notification = builder.build()
    } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB) {
      /* Android 7.1 and earlier don't segregate notifications into
         distinct categories, but permit an importance to be
         assigned to each individual notification.  */

      builder = Notification.Builder(context)
      builder.setContentTitle(title)
      builder.setContentText(content)
      builder.setSmallIcon(icon)

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.JELLY_BEAN) {
        when (importance) {
          3 -> priority = Notification.PRIORITY_DEFAULT /* IMPORTANCE_DEFAULT */
          4 -> priority = Notification.PRIORITY_HIGH    /* IMPORTANCE_HIGH */
          else -> priority = Notification.PRIORITY_LOW  /* 2: IMPORTANCE_LOW */
        }

        builder.setPriority(priority)
        insertActions(context, builder)
        notification = builder.build()
      } else
        notification = builder.getNotification()
    } else {
      notification = Notification()
      notification.icon = icon

      /* This remote widget tree is defined in
         java/res/layout/sdk8_notifications_view.xml.  */
      contentView = RemoteViews("org.gnu.emacs",
                                R.layout.sdk8_notifications_view)
      notification.contentView = contentView
      contentView.setTextViewText(R.id.sdk8_notifications_title,
                                  title)
      contentView.setTextViewText(R.id.sdk8_notifications_content,
                                  content)
    }

    /* Provide a content intent which starts Emacs when the
       notification is clicked.  */

    intent = Intent(context, EmacsActivity::class.java)
    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
    intent.setData(Uri.Builder().scheme("action")
                     .appendPath(tag)
                     .build())
    intent.putExtra(NOTIFICATION_ACTION, "default")
    intent.putExtra(NOTIFICATION_TAG, tag)

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S)
      pending = PendingIntent.getActivity(context, 0, intent,
                                          PendingIntent.FLAG_IMMUTABLE)
    else
      pending = PendingIntent.getActivity(context, 0, intent, 0)

    notification.contentIntent = pending

    /* Provide a cancellation intent to respond to notification
       dismissals.  */

    intent = Intent(context, CancellationReceiver::class.java)
    intent.setAction(NOTIFICATION_DISMISSED)
    intent.setPackage("org.gnu.emacs")
    intent.setData(Uri.Builder().scheme("action")
                     .appendPath(tag)
                     .build())
    intent.putExtra(NOTIFICATION_TAG, tag)

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S)
      pending = PendingIntent.getBroadcast(context, 0, intent,
                                           PendingIntent.FLAG_IMMUTABLE)
    else
      pending = PendingIntent.getBroadcast(context, 0, intent, 0)

    notification.deleteIntent = pending
    manager.notify(tag, 2, notification)
  }

  /* Display this desktop notification.

     Create a notification channel named GROUP or update its
     importance if such a channel is already defined.  */

  fun display() {
    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      display1(EmacsService.SERVICE!!)
    })
  }

  /* Broadcast receiver.  This is something of a system-wide callback
     arranged to be invoked whenever a notification posted by Emacs is
     dismissed, in order to relay news of its dismissal to
     androidselect.c and run or remove callbacks as appropriate.  */

  class CancellationReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent?) {
      val tag: String?

      if (intent == null || EmacsService.SERVICE == null)
        return

      tag = intent.getStringExtra(NOTIFICATION_TAG)

      if (tag == null)
        return

      EmacsNative.sendNotificationDeleted(tag)
    }
  }
}
