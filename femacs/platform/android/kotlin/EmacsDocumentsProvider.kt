// femacs: port of java/org/gnu/emacs/EmacsDocumentsProvider.java @ fd7afced403.

package org.gnu.emacs

import android.content.Context
import android.database.Cursor
import android.database.MatrixCursor
import android.net.Uri
import android.os.Build
import android.os.CancellationSignal
import android.os.ParcelFileDescriptor
import android.provider.DocumentsContract.Document
import android.provider.DocumentsContract.Root
import android.provider.DocumentsContract.buildChildDocumentsUri
import android.provider.DocumentsProvider
import android.webkit.MimeTypeMap
import java.io.File
import java.io.FileInputStream
import java.io.FileNotFoundException
import java.io.FileOutputStream
import java.io.IOException

/* ``Documents provider''.  This allows Emacs's home directory to be
   modified by other programs holding permissions to manage system
   storage, which is useful to (for example) correct misconfigurations
   which prevent Emacs from starting up.

   This functionality is only available on Android 19 and later.  */

/* Kotlin has no `|=` and no assignment inside a condition, so two Java shapes are spelled
   differently here and nowhere else: `flags |= X` becomes `flags = flags or X` (P-83 ①⑤), and
   `while ((length = stream.read (buffer)) > 0)` becomes a `loop`-shaped `while (true)` with the
   read and the `break` on separate lines - the same rendering the C side uses for C's
   assignment-in-condition.  A Java parameter is also immutable in Kotlin, so `projection` is
   re-bound to a local instead of being overwritten.

   `getContext()` also has a different Kotlin signature here than the Java compiled against -
   `Context?`, not `Context` - so the port asserts non-null (`!!`) exactly where the reference
   dereferences it, which is the NPE the Java would raise in the same case (P-83 ⑤).  */
class EmacsDocumentsProvider : DocumentsProvider() {
  /* Home directory.  This is the directory whose contents are
     initially returned to requesting applications.  */
  private lateinit var baseDir: File

  companion object {
    /* The default projection for requests for the root directory.  */
    private val DEFAULT_ROOT_PROJECTION: Array<String> = arrayOf(
      Root.COLUMN_ROOT_ID,
      Root.COLUMN_MIME_TYPES,
      Root.COLUMN_FLAGS,
      Root.COLUMN_ICON,
      Root.COLUMN_TITLE,
      Root.COLUMN_SUMMARY,
      Root.COLUMN_DOCUMENT_ID,
      Root.COLUMN_AVAILABLE_BYTES,
    )

    /* The default projection for requests for a file.  */
    private val DEFAULT_DOCUMENT_PROJECTION: Array<String> = arrayOf(
      Document.COLUMN_DOCUMENT_ID,
      Document.COLUMN_MIME_TYPE,
      Document.COLUMN_DISPLAY_NAME,
      Document.COLUMN_LAST_MODIFIED,
      Document.COLUMN_FLAGS,
      Document.COLUMN_SIZE,
    )
  }

  override fun onCreate(): Boolean {
    /* Set the base directory to Emacs's files directory.  */
    baseDir = context!!.filesDir
    return true
  }

  override fun queryRoots(projection: Array<out String>?): Cursor? {
    val result: MatrixCursor
    val row: MatrixCursor.RowBuilder

    /* If the requestor asked for nothing at all, then it wants some
       data by default.  */

    val columns: Array<out String>? = projection ?: DEFAULT_ROOT_PROJECTION

    result = MatrixCursor(columns)
    row = result.newRow()

    /* Now create and add a row for each file in the base
       directory.  */
    row.add(Root.COLUMN_ROOT_ID, baseDir.absolutePath)
    row.add(Root.COLUMN_SUMMARY, "Emacs home directory")

    /* Add the appropriate flags.  */

    row.add(Root.COLUMN_FLAGS, Root.FLAG_SUPPORTS_CREATE or Root.FLAG_SUPPORTS_IS_CHILD)
    row.add(Root.COLUMN_ICON, R.drawable.emacs)
    row.add(Root.FLAG_LOCAL_ONLY)
    row.add(Root.COLUMN_TITLE, "Emacs")
    row.add(Root.COLUMN_DOCUMENT_ID, baseDir.absolutePath)

    return result
  }

  private fun getNotificationUri(file: File): Uri {
    val updatedUri: Uri

    updatedUri = buildChildDocumentsUri("org.gnu.emacs", file.absolutePath)

    return updatedUri
  }

  /* Inform the system that FILE's contents (or FILE itself) has
     changed.  */

  private fun notifyChange(file: File) {
    val updatedUri: Uri
    val context: Context

    context = this.context!!
    updatedUri = buildChildDocumentsUri("org.gnu.emacs", file.absolutePath)
    context.contentResolver.notifyChange(updatedUri, null)
  }

  /* Inform the system that FILE's contents (or FILE itself) has
     changed.  FILE is a string describing containing the file name of
     a directory as opposed to a File.  */

  private fun notifyChangeByName(file: String) {
    val updatedUri: Uri
    val context: Context

    context = this.context!!
    updatedUri = buildChildDocumentsUri("org.gnu.emacs", file)
    context.contentResolver.notifyChange(updatedUri, null)
  }

  /* Return the MIME type of a file FILE.  */

  private fun getMimeType(file: File): String {
    val name: String
    val mime: String?
    val extensionSeparator: Int

    if (file.isDirectory)
      return Document.MIME_TYPE_DIR

    /* Abuse WebView stuff to get the file's MIME type.  */
    name = file.name
    extensionSeparator = name.lastIndexOf('.')

    if (extensionSeparator > 0) {
      val singleton: MimeTypeMap = MimeTypeMap.getSingleton()
      val extension: String = name.substring(extensionSeparator + 1)
      mime = singleton.getMimeTypeFromExtension(extension)

      if (mime != null)
        return mime
    }

    return "application/octet-stream"
  }

  /* Append the specified FILE to the query result RESULT.
     Handle both directories and ordinary files.  */

  private fun queryDocument1(result: MatrixCursor, file: File) {
    val row: MatrixCursor.RowBuilder
    val fileName: String
    val displayName: String
    val mimeType: String
    var flags: Int

    row = result.newRow()
    flags = 0

    /* fileName is a string that the system will ask for some time in
       the future.  Here, it is just the absolute name of the file.  */
    fileName = file.absolutePath

    /* If file is a directory, add the right flags for that.  */

    if (file.isDirectory) {
      if (file.canWrite()) {
        flags = flags or Document.FLAG_DIR_SUPPORTS_CREATE
        flags = flags or Document.FLAG_SUPPORTS_DELETE

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP)
          flags = flags or Document.FLAG_SUPPORTS_RENAME

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N)
          flags = flags or Document.FLAG_SUPPORTS_MOVE
      }
    } else if (file.canWrite()) {
      /* Apply the correct flags for a writable file.  */
      flags = flags or Document.FLAG_SUPPORTS_WRITE
      flags = flags or Document.FLAG_SUPPORTS_DELETE

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP)
        flags = flags or Document.FLAG_SUPPORTS_RENAME

      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
        flags = flags or Document.FLAG_SUPPORTS_REMOVE
        flags = flags or Document.FLAG_SUPPORTS_MOVE
      }
    }

    displayName = file.name
    mimeType = getMimeType(file)

    row.add(Document.COLUMN_DOCUMENT_ID, fileName)
    row.add(Document.COLUMN_DISPLAY_NAME, displayName)
    row.add(Document.COLUMN_SIZE, file.length())
    row.add(Document.COLUMN_MIME_TYPE, mimeType)
    row.add(Document.COLUMN_LAST_MODIFIED, file.lastModified())
    row.add(Document.COLUMN_FLAGS, flags)
  }

  @Throws(FileNotFoundException::class)
  override fun queryDocument(documentId: String, projection: Array<out String>?): Cursor? {
    val result: MatrixCursor
    val file: File
    val context: Context

    file = File(documentId)
    context = this.context!!

    val columns: Array<out String>? = projection ?: DEFAULT_DOCUMENT_PROJECTION

    result = MatrixCursor(columns)
    queryDocument1(result, file)

    /* Now allow interested applications to detect changes.  */
    result.setNotificationUri(context.contentResolver, getNotificationUri(file))

    return result
  }

  @Throws(FileNotFoundException::class)
  override fun queryChildDocuments(parentDocumentId: String, projection: Array<out String>?,
                                   sortOrder: String?): Cursor? {
    val result: MatrixCursor
    val directory: File
    val files: Array<File>?
    val context: Context

    val columns: Array<out String>? = projection ?: DEFAULT_DOCUMENT_PROJECTION

    result = MatrixCursor(columns)

    /* Try to open the file corresponding to the location being
       requested.  */
    directory = File(parentDocumentId)

    /* Look up each child.  */
    files = directory.listFiles()

    if (files != null) {
      /* Now add each child.  */
      for (child in files)
        queryDocument1(result, child)
    }

    context = this.context!!

    /* Now allow interested applications to detect changes.  */
    result.setNotificationUri(context.contentResolver, getNotificationUri(directory))

    return result
  }

  @Throws(FileNotFoundException::class)
  override fun openDocument(documentId: String, mode: String,
                            signal: CancellationSignal?): ParcelFileDescriptor? {
    return ParcelFileDescriptor.open(File(documentId),
                                     ParcelFileDescriptor.parseMode(mode))
  }

  @Throws(FileNotFoundException::class)
  override fun createDocument(documentId: String, mimeType: String?,
                             displayName: String): String? {
    var file: File
    var parentFile: File?
    var rc: Boolean

    file = File(documentId, displayName)

    try {
      rc = false

      if (Document.MIME_TYPE_DIR == mimeType) {
        file.mkdirs()

        if (file.isDirectory)
          rc = true
      } else {
        file.createNewFile()

        if (file.isFile
            && file.setWritable(true)
            && file.setReadable(true))
          rc = true
      }

      if (!rc)
        throw FileNotFoundException("rc != 1")
    } catch (e: IOException) {
      throw FileNotFoundException(e.toString())
    }

    parentFile = file.parentFile

    if (parentFile != null)
      notifyChange(parentFile)

    return file.absolutePath
  }

  private fun deleteDocument1(child: File) {
    val children: Array<File>?

    /* Don't delete symlinks recursively.

       Calling readlink or stat is problematic due to file name
       encoding problems, so try to delete the file first, and only
       try to delete files recursively afterword.  */

    if (child.delete())
      return

    children = child.listFiles()

    if (children != null) {
      for (file in children)
        deleteDocument1(file)
    }

    child.delete()
  }

  @Throws(FileNotFoundException::class)
  override fun deleteDocument(documentId: String) {
    val file: File
    val parent: File?
    val children: Array<File>?

    /* Java makes recursively deleting a file hard.  File name
       encoding issues also prevent easily calling into C...  */

    file = File(documentId)
    parent = file.parentFile

    if (parent == null)
      throw RuntimeException("trying to delete file without parent!")

    if (file.delete()) {
      /* Tell the system about the change.  */
      notifyChange(parent)
      return
    }

    children = file.listFiles()

    if (children != null) {
      for (child in children)
        deleteDocument1(child)
    }

    if (file.delete())
      /* Tell the system about the change.  */
      notifyChange(parent)
  }

  @Throws(FileNotFoundException::class)
  override fun removeDocument(documentId: String, parentDocumentId: String) {
    deleteDocument(documentId)
  }

  override fun getDocumentType(documentId: String): String {
    return getMimeType(File(documentId))
  }

  @Throws(FileNotFoundException::class)
  override fun renameDocument(documentId: String, displayName: String): String? {
    val newName: File
    val parent: File?
    var file: File

    file = File(documentId)
    parent = file.parentFile

    if (parent == null)
      throw FileNotFoundException("parent is null")

    newName = File(parent, displayName)

    file = File(documentId)

    if (!file.renameTo(newName))
      return null

    notifyChange(parent)
    return newName.absolutePath
  }

  override fun isChildDocument(parentDocumentId: String, documentId: String): Boolean {
    return documentId.startsWith(parentDocumentId)
  }

  @Throws(FileNotFoundException::class)
  override fun moveDocument(sourceDocumentId: String, sourceParentDocumentId: String,
                            targetParentDocumentId: String): String? {
    val file: File
    val newName: File
    var inputStream: FileInputStream? = null
    var outputStream: FileOutputStream? = null
    val buffer: ByteArray
    var length: Int

    file = File(sourceDocumentId)

    /* Now, create the file name of the parent document.  */
    newName = File(targetParentDocumentId, file.name)

    /* Try to perform a simple rename, before falling back to
       copying.  */

    if (file.renameTo(newName)) {
      notifyChangeByName(file.parent)
      notifyChangeByName(targetParentDocumentId)
      return newName.absolutePath
    }

    /* If that doesn't work, create the new file and copy over the old
       file's contents.  */

    try {
      if (!newName.createNewFile()
          || !newName.setWritable(true)
          || !newName.setReadable(true))
        throw FileNotFoundException("failed to create new file")

      /* Open the file in preparation for a copy.  */

      inputStream = FileInputStream(file)
      outputStream = FileOutputStream(newName)

      /* Allocate the buffer used to hold data.  */

      buffer = ByteArray(4096)

      while (true) {
        length = inputStream.read(buffer)
        if (length <= 0)
          break
        outputStream.write(buffer, 0, length)
      }
    } catch (e: IOException) {
      throw FileNotFoundException("IOException: " + e)
    } finally {
      try {
        if (inputStream != null)
          inputStream.close()
      } catch (e: IOException) {
        /* The reference swallows both close failures; the port keeps
           that (P-83 ①, no "improvements" over the reference).  */
      }

      try {
        if (outputStream != null)
          outputStream.close()
      } catch (e: IOException) {
      }
    }

    file.delete()
    notifyChangeByName(file.parent)
    notifyChangeByName(targetParentDocumentId)

    return newName.absolutePath
  }
}
