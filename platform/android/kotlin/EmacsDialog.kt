// femacs: port of java/org/gnu/emacs/EmacsDialog.java @ fd7afced403.

package org.femacs

import java.util.concurrent.Callable
import java.util.concurrent.FutureTask

import android.app.AlertDialog

import android.content.Context
import android.content.DialogInterface

import android.content.res.Resources.NotFoundException
import android.content.res.Resources.Theme
import android.content.res.TypedArray

import android.os.Build

import android.provider.Settings

import android.widget.Button
import android.widget.FrameLayout

import android.view.ContextThemeWrapper
import android.view.View
import android.view.ViewGroup
import android.view.Window
import android.view.WindowManager

/* Toolkit dialog implementation.  This object is built from JNI and
   describes a single alert dialog.  Then, `inflate' turns it into
   AlertDialog.  */

/* `androidmenu.c` 618-650 finds this class and calls three of its members by name and descriptor -
   the static `createDialog` `(Ljava/lang/String;Ljava/lang/String;I)Lorg/gnu/emacs/EmacsDialog;`,
   `addButton` `(Ljava/lang/String;IZ)V` and `display` `()Z` - so those keep the reference's names
   and parameter lists, and the static factory carries `@JvmStatic` (P-83 ⑪).  `EmacsButton` is a
   non-static nested class in the reference, so the port keeps it `inner` (P-83, the
   `FontObject`/`Sdk7FontObject` shape).  `toAlertDialog`'s parameter is reassigned in the
   reference (`context = new ContextThemeWrapper (…)`), and Kotlin's parameters are immutable, so
   the wrapped context is a local (the reassigned-parameter shape 3.290 ③ records); Java's
   `Window window;` local in `toAlertDialog`, declared and never read, is not carried (Kotlin
   warns on an unused local), and neither is the unused `android.util.Log` import.  The
   reference's anonymous `Callable` is a Kotlin SAM literal (P-83 ①).  */
class EmacsDialog : DialogInterface.OnDismissListener {
  companion object {
    private const val TAG = "EmacsDialog"

    /* Create a popup dialog with the title TITLE and the text TEXT.
       TITLE may be NULL.  MENUEVENTSERIAL is a number which will
       identify this popup dialog inside events it sends.  */

    @JvmStatic
    fun createDialog(title: String?, text: String?,
                     menuEventSerial: Int): EmacsDialog {
      val dialog: EmacsDialog

      dialog = EmacsDialog()
      dialog.buttons = ArrayList<EmacsButton>()
      dialog.title = title
      dialog.text = text
      dialog.menuEventSerial = menuEventSerial

      return dialog
    }
  }

  /* List of buttons in this dialog.  */
  private var buttons: MutableList<EmacsButton>? = null

  /* Dialog title.  */
  private var title: String? = null

  /* Dialog text.  */
  private var text: String? = null

  /* Whether or not a selection has already been made.  */
  private var wasButtonClicked: Boolean = false

  /* Dialog to dismiss after click.  */
  private var dismissDialog: AlertDialog? = null

  /* The menu serial associated with this dialog box.  */
  private var menuEventSerial: Int = 0

  private inner class EmacsButton : View.OnClickListener,
                                   DialogInterface.OnClickListener {
    /* Name of this button.  */
    var name: String? = null

    /* ID of this button.  */
    var id: Int = 0

    /* Whether or not the button is enabled.  */
    var enabled: Boolean = false

    override fun onClick(view: View) {
      wasButtonClicked = true
      EmacsNative.sendContextMenu(0, id, menuEventSerial)
      dismissDialog!!.dismiss()
    }

    override fun onClick(dialog: DialogInterface, which: Int) {
      wasButtonClicked = true
      EmacsNative.sendContextMenu(0, id, menuEventSerial)
    }
  }

  /* Add a button named NAME, with the identifier ID.  If DISABLE,
     disable the button.  */

  fun addButton(name: String?, id: Int, disable: Boolean) {
    val button: EmacsButton

    button = EmacsButton()
    button.name = name
    button.id = id
    button.enabled = !disable
    buttons!!.add(button)
  }

  /* Turn this dialog into an AlertDialog for the specified
     CONTEXT.

     Upon a button being selected, the dialog will send an
     ANDROID_CONTEXT_MENU event with the id of that button.

     Upon the dialog being dismissed, an ANDROID_CONTEXT_MENU event
     will be sent with an id of 0.  */

  fun toAlertDialog(context: Context): AlertDialog {
    var dialog: AlertDialog
    var size: Int
    var styleId: Int
    var flag: Int
    val attrs: IntArray
    var button: EmacsButton
    val layout: EmacsDialogButtonLayout
    var buttonView: Button
    val layoutParams: ViewGroup.LayoutParams
    val theme: Theme
    var attributes: TypedArray

    /* Wrap the context within a style wrapper.  Any dialog properties
       tied to EmacsStyle (such as those applied by the system ``dark
       theme'') will thus affect the dialog irrespective of whether
       CONTEXT is an activity or the service.  */

    /* C: `context = new ContextThemeWrapper (context, R.style.EmacsStyle);` - see the class
       comment on the reassigned parameter.  */
    val wrappedContext = ContextThemeWrapper(context, R.style.EmacsStyle)

    size = buttons!!.size
    styleId = -1

    if (size <= 3) {
      dialog = AlertDialog.Builder(wrappedContext).create()
      dialog.setMessage(text)
      dialog.setCancelable(true)
      dialog.setOnDismissListener(this)

      if (title != null)
        dialog.setTitle(title)

      /* There are less than 4 buttons.  Add the buttons the way
         Android intends them to be added.  */

      if (size >= 1) {
        button = buttons!!.get(0)
        dialog.setButton(DialogInterface.BUTTON_POSITIVE,
                         button.name, button)
      }

      if (size >= 2) {
        button = buttons!!.get(1)
        dialog.setButton(DialogInterface.BUTTON_NEGATIVE,
                         button.name, button)
      }

      if (size >= 3) {
        button = buttons!!.get(2)
        dialog.setButton(DialogInterface.BUTTON_NEUTRAL,
                         button.name, button)
      }
    } else {
      /* There are more than 3 buttons.  Add them all to a special
         container widget that handles wrapping.  First, create the
         layout.  */

      layout = EmacsDialogButtonLayout(wrappedContext)
      layoutParams = FrameLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT,
                                              ViewGroup.LayoutParams.WRAP_CONTENT)
      layout.setLayoutParams(layoutParams)

      /* Add that layout to the dialog's custom view.

         android.R.id.custom is documented to work.  But looking it
         up returns NULL, so setView must be used instead.  */

      dialog = AlertDialog.Builder(wrappedContext).setView(layout).create()
      dialog.setMessage(text)
      dialog.setCancelable(true)
      dialog.setOnDismissListener(this)

      if (title != null)
        dialog.setTitle(title)

      /* Now that the dialog has been created, set the style of each
         custom button to match the usual dialog buttons found on
         Android 5 and later.  */

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
        /* Obtain the Theme associated with the dialog.  */
        theme = dialog.getContext().getTheme()

        /* Resolve the dialog button style.  */
        attrs = intArrayOf(android.R.attr.buttonBarNeutralButtonStyle)

        try {
          attributes = theme.obtainStyledAttributes(attrs)

          /* Look for the style ID.  Default to -1 if it could
             not be found.  */
          styleId = attributes.getResourceId(0, -1)

          /* Now clean up the TypedAttributes object.  */
          attributes.recycle()
        } catch (e: NotFoundException) {
          /* Nothing to do here.  */
        }
      }

      /* Create each button and add it to the layout.  Set the style
         if necessary.  */

      for (emacsButton in buttons!!) {
        if (styleId == -1)
          /* No specific style... */
          buttonView = Button(wrappedContext)
        else
          /* Use the given styleId.  */
          buttonView = Button(wrappedContext, null, 0, styleId)

        /* Set the text and on click handler.  */
        buttonView.setText(emacsButton.name)
        buttonView.setOnClickListener(emacsButton)
        buttonView.setEnabled(emacsButton.enabled)
        layout.addView(buttonView)
      }
    }

    return dialog
  }

  /* Internal helper for display run on the main thread.  */

  @Suppress("DEPRECATION")
  private fun display1(): Boolean {
    var context: Context?
    var size: Int
    var type: Int
    var buttonView: Button
    var button: EmacsButton
    var dialog: AlertDialog
    var window: Window

    if (EmacsActivity.focusedActivities.isEmpty()) {
      /* If focusedActivities is empty then this dialog may have
         been displayed immediately after another popup dialog was
         dismissed.  Or Emacs might legitimately be in the
         background, possibly displaying this popup in response to
         an Emacsclient request.  Try the service context if it will
         work, then any focused EmacsOpenActivity, and finally the
         last EmacsActivity to be focused.  */

      if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M
          || Settings.canDrawOverlays(EmacsService.SERVICE))
        context = EmacsService.SERVICE
      else if (EmacsOpenActivity.currentActivity != null)
        context = EmacsOpenActivity.currentActivity
      else
        context = EmacsActivity.lastFocusedActivity

      if (context == null)
        return false
    } else
      /* Display using the activity context when Emacs is in the
         foreground, as this allows the dialog to be dismissed more
         consistently.  */
      context = EmacsActivity.focusedActivities.get(0)

    /* C: `dialog = dismissDialog = toAlertDialog (context);` - Java sets `dismissDialog` first and
       `dialog` from it; the port writes the same value to both.  */
    val shownDialog = toAlertDialog(context)
    dismissDialog = shownDialog
    dialog = shownDialog

    try {
      if (context === EmacsService.SERVICE) {
        /* Apply the system alert window type to make sure this
           dialog can be displayed.  */

        window = dialog.getWindow()!!
        type = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
          WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY
        else
          WindowManager.LayoutParams.TYPE_PHONE
        window.setType(type)
      }

      dismissDialog!!.show()
    } catch (exception: Exception) {
      /* This can happen when the system decides Emacs is not in the
         foreground any longer.  */
      return false
    }

    /* If there are less than four buttons, then they must be
       individually enabled or disabled after the dialog is
       displayed.  */
    size = buttons!!.size

    if (size <= 3) {
      if (size >= 1) {
        button = buttons!!.get(0)
        buttonView = dialog.getButton(DialogInterface.BUTTON_POSITIVE)
        buttonView.setEnabled(button.enabled)
      }

      if (size >= 2) {
        button = buttons!!.get(1)
        buttonView = dialog.getButton(DialogInterface.BUTTON_NEGATIVE)
        buttonView.setEnabled(button.enabled)
      }

      if (size >= 3) {
        button = buttons!!.get(2)
        buttonView = dialog.getButton(DialogInterface.BUTTON_NEUTRAL)
        buttonView.setEnabled(button.enabled)
      }
    }

    return true
  }

  /* Display this dialog for a suitable activity.
     Value is false if the dialog could not be displayed,
     and true otherwise.  */

  fun display(): Boolean {
    val task: FutureTask<Boolean>

    task = FutureTask<Boolean>(Callable<Boolean> { display1() })

    return EmacsService.syncRunnable(task)
  }

  override fun onDismiss(dialog: DialogInterface) {
    if (wasButtonClicked)
      return

    EmacsNative.sendContextMenu(0, 0, menuEventSerial)
  }
}
