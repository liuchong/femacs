// femacs: port of java/org/gnu/emacs/EmacsInputConnection.java @ fd7afced403.

package org.gnu.emacs

import android.os.Build
import android.os.Bundle
import android.os.Handler

import android.view.KeyEvent

import android.view.inputmethod.CompletionInfo
import android.view.inputmethod.CorrectionInfo
import android.view.inputmethod.ExtractedText
import android.view.inputmethod.ExtractedTextRequest
import android.view.inputmethod.InputConnection
import android.view.inputmethod.InputContentInfo
import android.view.inputmethod.SurroundingText
import android.view.inputmethod.TextAttribute
import android.view.inputmethod.TextSnapshot

import android.util.Log

/* Android input methods, take number six.  See textconv.c for more
   details; this is more-or-less a thin wrapper around that file.  */

/* No part of C looks this class up by name: `androidterm.c` only notes at 5750 that the
   `performContextMenuAction` action codes are computed here, and `EmacsView` 906 constructs it on
   the Java side (`new EmacsInputConnection (this)`).  Every name that C *does* resolve by name in
   this file therefore belongs to `EmacsNative` (`androidterm.c` 5108 `beginBatchEdit`, 5184
   `commitText`, 5590 `getSelection`, 5947 `getExtractedText`, 6348 `getSurroundingText`, 6359
   `takeSnapshot`, ...), and those already carry their reference names in the ported
   `EmacsNative.kt`.  The class is `final` in the reference, so the port is a plain Kotlin `class`.

   Kotlin divergences that are forced by the language, not chosen here: the reference's
   `extractAbsoluteOffsets = syncAfterCommit = true;` is two assignments (Kotlin has no chained
   assignment); `commitCompletion`, `commitContent`, `getHandler`, `performSpellCheck`,
   `getCursorCapsMode` and `setImeConsumesInput` carry `override` although the reference omits
   `@Override` on some of them (Kotlin requires the keyword for every interface implementation);
   the reference's `@SuppressWarnings ("deprecation")` on `sendKeyEvent` becomes
   `@Suppress ("DEPRECATION", "OVERRIDE_DEPRECATION")` (P-83 ④).  Return types follow the reference
   and the C prototypes: `getSelectedText`/`getTextAfterCursor`/`getTextBeforeCursor` are `String?`
   and `getSelection` is `IntArray?` (`androidterm.c` 5651 returns `jintArray`), `getExtractedText`
   and `takeSnapshot` are nullable because the reference null-checks them, and `getSurroundingText`
   is nullable because `androidterm.c` 6376/6392 `return NULL`.  The reference's `int[] selection;`
   locals are `IntArray?`; the reference's unused `EmacsService.DEBUG_IC`-guarded logging is kept
   verbatim.  */

class EmacsInputConnection(private val view: EmacsView) : InputConnection {
  companion object {
    private const val TAG = "EmacsInputConnection"

    /* Whether or not to synchronize and call `updateIC' with the
       selection position after committing text.

       This helps with on screen keyboard programs found in some vendor
       versions of Android, which rely on immediate updates to the point
       position after text is committed in order to place the cursor
       within that text.  */

    private var syncAfterCommit: Boolean = false

    /* Whether or not to return empty text with the offset set to zero
       if a request arrives that has no flags set and has requested no
       characters at all.

       This is necessary with on screen keyboard programs found in some
       vendor versions of Android which don't rely on the documented
       meaning of `ExtractedText.startOffset', and instead take the
       selection offset inside at face value.  */

    private var extractAbsoluteOffsets: Boolean = false

    init {
      if (Build.MANUFACTURER.equals("Huawei", ignoreCase = true)
          || Build.MANUFACTURER.equals("Honor", ignoreCase = true)) {
        /* C: `extractAbsoluteOffsets = syncAfterCommit = true;` - the port writes the two
           assignments separately; the pair of flags is what matters.  */
        syncAfterCommit = true
        extractAbsoluteOffsets = true
      }

      /* The Samsung and Vivo keyboards take `selectionStart' at face
         value if some text is returned, and also search for words solely
         within that text.  However, when no text is returned, it falls
         back to getTextAfterCursor and getTextBeforeCursor.  */
      if (Build.MANUFACTURER.equals("Samsung", ignoreCase = true)
          || Build.MANUFACTURER.equals("Vivo", ignoreCase = true))
        extractAbsoluteOffsets = true
    }
  }

  /* The handle ID associated with that view's window.  */
  private var windowHandle: Long = view.window.handle

  /* Number of batch edits currently underway.  Used to avoid
     synchronizing with the Emacs thread after each
     `endBatchEdit'.  */
  private var batchEditCount: Int = 0

  /* The functions below are called by input methods whenever they
     need to perform an edit.  */

  override fun beginBatchEdit(): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "beginBatchEdit")

    EmacsNative.beginBatchEdit(windowHandle)

    /* Keep a record of the number of outstanding batch edits here as
       well.  */
    batchEditCount++
    return true
  }

  override fun endBatchEdit(): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "endBatchEdit")

    EmacsNative.endBatchEdit(windowHandle)

    /* Subtract one from the UI thread record of the number of batch
       edits currently under way.  */

    if (batchEditCount > 0)
      batchEditCount -= 1

    return batchEditCount > 0
  }

  override fun commitCompletion(info: CompletionInfo): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "commitCompletion: " + info)

    EmacsNative.commitCompletion(windowHandle,
                                 info.getText().toString(),
                                 info.getPosition())
    return true
  }

  override fun commitCorrection(info: CorrectionInfo): Boolean {
    /* The input method calls this function not to commit text, but to
       indicate that a subsequent edit will consist of a correction.
       Emacs has no use for this information.

       Of course this completely contradicts the provided
       documentation, but this is how Android actually behaves.  */
    return false
  }

  override fun commitText(text: CharSequence, newCursorPosition: Int): Boolean {
    var selection: IntArray?

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "commitText: " + text + " " + newCursorPosition)

    EmacsNative.commitText(windowHandle, text.toString(),
                           newCursorPosition)

    if (syncAfterCommit) {
      /* Synchronize with the Emacs thread, obtain the new
         selection, and report it immediately.  */

      selection = EmacsNative.getSelection(windowHandle)

      if (EmacsService.DEBUG_IC && selection != null)
        Log.d(TAG, "commitText: new selection is " + selection[0]
              + ", by " + selection[1])

      if (selection != null)
        /* N.B. that the composing region is removed after text is
           committed.  */
        view.imManager.updateSelection(view, selection[0],
                                       selection[1], -1, -1)
    }

    return true
  }

  override fun commitText(text: CharSequence, newCursorPosition: Int,
                          textAttribute: TextAttribute?): Boolean {
    return commitText(text, newCursorPosition)
  }

  override fun deleteSurroundingText(leftLength: Int, rightLength: Int): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "deleteSurroundingText: "
            + leftLength + " " + rightLength)

    EmacsNative.deleteSurroundingText(windowHandle, leftLength,
                                      rightLength)
    return true
  }

  override fun deleteSurroundingTextInCodePoints(leftLength: Int,
                                                 rightLength: Int): Boolean {
    /* Emacs returns characters which cannot be represented in a Java
       `char' as NULL characters, so code points always reflect
       characters themselves.  */
    return deleteSurroundingText(leftLength, rightLength)
  }

  override fun finishComposingText(): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "finishComposingText")

    EmacsNative.finishComposingText(windowHandle)
    return true
  }

  override fun getSelectedText(flags: Int): String? {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return null

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "getSelectedText: " + flags)

    return EmacsNative.getSelectedText(windowHandle, flags)
  }

  override fun getTextAfterCursor(length: Int, flags: Int): String? {
    var string: String?

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return null

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "getTextAfterCursor: " + length + " " + flags)

    string = EmacsNative.getTextAfterCursor(windowHandle, length,
                                            flags)

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "   --> " + string)

    return string
  }

  override fun getTextBeforeCursor(length: Int, flags: Int): String? {
    var string: String?

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return null

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "getTextBeforeCursor: " + length + " " + flags)

    string = EmacsNative.getTextBeforeCursor(windowHandle, length,
                                             flags)

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "   --> " + string)

    return string
  }

  override fun setComposingText(text: CharSequence, newCursorPosition: Int): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "setComposingText: "
            + text + " ## " + newCursorPosition)

    EmacsNative.setComposingText(windowHandle, text.toString(),
                                 newCursorPosition)
    return true
  }

  override fun setComposingText(text: CharSequence, newCursorPosition: Int,
                                textAttribute: TextAttribute?): Boolean {
    return setComposingText(text, newCursorPosition)
  }

  override fun setComposingRegion(start: Int, end: Int): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "setComposingRegion: " + start + " " + end)

    EmacsNative.setComposingRegion(windowHandle, start, end)
    return true
  }

  override fun setComposingRegion(start: Int, end: Int,
                                  textAttribute: TextAttribute?): Boolean {
    return setComposingRegion(start, end)
  }

  override fun performEditorAction(editorAction: Int): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "performEditorAction: " + editorAction)

    EmacsNative.performEditorAction(windowHandle, editorAction)
    return true
  }

  override fun performContextMenuAction(contextMenuAction: Int): Boolean {
    var action: Int

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "performContextMenuAction: " + contextMenuAction)

    /* Translate the action in Java code.  That way, a great deal of
       JNI boilerplate can be avoided.  */

    when (contextMenuAction) {
      android.R.id.selectAll ->
        action = 0

      android.R.id.startSelectingText ->
        action = 1

      android.R.id.stopSelectingText ->
        action = 2

      android.R.id.cut ->
        action = 3

      android.R.id.copy ->
        action = 4

      android.R.id.paste ->
        action = 5

      else ->
        return true
    }

    EmacsNative.performContextMenuAction(windowHandle, action)
    return true
  }

  override fun getExtractedText(request: ExtractedTextRequest,
                                flags: Int): ExtractedText? {
    var text: ExtractedText?
    var selection: IntArray?

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return null

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "getExtractedText: " + request.hintMaxChars + ", "
            + request.hintMaxLines + " " + flags)

    /* If a request arrives with hintMaxChars, hintMaxLines and flags
       set to 0, and the system is known to be buggy, return an empty
       extracted text object with the absolute selection positions.  */

    if (extractAbsoluteOffsets
        && request.hintMaxChars == 0
        && request.hintMaxLines == 0
        && flags == 0) {
      /* Obtain the selection.  */
      selection = EmacsNative.getSelection(windowHandle)
      if (selection == null)
        return null

      /* Create the workaround extracted text.  */
      text = ExtractedText()
      text.partialStartOffset = -1
      text.partialEndOffset = -1
      text.text = ""
      text.selectionStart = selection[0]
      text.selectionEnd = selection[1]
    } else
      text = EmacsNative.getExtractedText(windowHandle, request,
                                          flags)

    if (text == null) {
      if (EmacsService.DEBUG_IC)
        Log.d(TAG, "getExtractedText: text is NULL")

      return null
    }

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "getExtractedText: " + text.text + " @"
            + text.startOffset + ":" + text.selectionStart
            + ", " + text.selectionEnd)

    return text
  }

  override fun setSelection(start: Int, end: Int): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "setSelection: " + start + " " + end)

    EmacsNative.setSelection(windowHandle, start, end)
    return true
  }

  /* ACTION_MULTIPLE is apparently obsolete.  */
  @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
  override fun sendKeyEvent(key: KeyEvent): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "sendKeyEvent: " + key)

    /* Use the standard API if possible.  */

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N)
      view.imManager.dispatchKeyEventFromInputMethod(view, key)
    else {
      /* Fall back to dispatching the event manually if not.  */

      when (key.getAction()) {
        KeyEvent.ACTION_DOWN ->
          view.onKeyDown(key.getKeyCode(), key)

        KeyEvent.ACTION_UP ->
          view.onKeyUp(key.getKeyCode(), key)

        KeyEvent.ACTION_MULTIPLE ->
          view.onKeyMultiple(key.getKeyCode(),
                             key.getRepeatCount(),
                             key)
      }
    }

    return true
  }

  override fun requestCursorUpdates(cursorUpdateMode: Int): Boolean {
    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return false

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "requestCursorUpdates: " + cursorUpdateMode)

    EmacsNative.requestCursorUpdates(windowHandle, cursorUpdateMode)
    return true
  }

  override fun requestCursorUpdates(cursorUpdateMode: Int, filter: Int): Boolean {
    if (filter != 0)
      return false

    return requestCursorUpdates(cursorUpdateMode)
  }

  override fun getSurroundingText(beforeLength: Int, afterLength: Int,
                                  flags: Int): SurroundingText? {
    var text: SurroundingText?

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return null

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "getSurroundingText: " + beforeLength + ", "
            + afterLength)

    text = EmacsNative.getSurroundingText(windowHandle, beforeLength,
                                          afterLength, flags)

    if (EmacsService.DEBUG_IC && text != null)
      Log.d(TAG, "getSurroundingText: "
            + text.getSelectionStart()
            + "," + text.getSelectionEnd()
            + "+" + text.getOffset()
            + ": " + text.getText())

    return text
  }

  override fun takeSnapshot(): TextSnapshot? {
    var snapshot: TextSnapshot?

    /* Return if the input connection is out of date.  */
    if (view.icSerial < view.icGeneration)
      return null

    snapshot = EmacsNative.takeSnapshot(windowHandle)

    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "takeSnapshot: "
            + snapshot!!.getSurroundingText().getText()
            + " @ " + snapshot.getCompositionEnd()
            + ", " + snapshot.getCompositionStart())

    return snapshot
  }

  override fun closeConnection() {
    batchEditCount = 0
  }

  override fun replaceText(start: Int, end: Int, text: CharSequence,
                           newCursorPosition: Int,
                           attributes: TextAttribute?): Boolean {
    if (EmacsService.DEBUG_IC)
      Log.d(TAG, "replaceText: " + text + ":: " + start + ","
            + end + "," + newCursorPosition)

    EmacsNative.replaceText(windowHandle, start, end,
                            text.toString(), newCursorPosition,
                            attributes)
    return true
  }

  fun reset() {
    batchEditCount = 0
  }

  /* Override functions which are not implemented.  */

  override fun getHandler(): Handler? {
    return null
  }

  override fun commitContent(inputContentInfo: InputContentInfo, flags: Int,
                             opts: Bundle?): Boolean {
    return false
  }

  override fun setImeConsumesInput(imeConsumesInput: Boolean): Boolean {
    return false
  }

  override fun clearMetaKeyStates(states: Int): Boolean {
    return false
  }

  override fun reportFullscreenMode(enabled: Boolean): Boolean {
    return false
  }

  override fun performSpellCheck(): Boolean {
    return false
  }

  override fun performPrivateCommand(action: String, data: Bundle?): Boolean {
    return false
  }

  override fun getCursorCapsMode(reqModes: Int): Int {
    return 0
  }
}
