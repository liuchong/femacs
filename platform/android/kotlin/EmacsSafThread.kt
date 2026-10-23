// femacs: port of java/org/gnu/emacs/EmacsSafThread.java @ fd7afced403.

package org.femacs

import java.io.IOException

import android.content.ContentResolver
import android.database.Cursor
import android.net.Uri

import android.os.Build
import android.os.CancellationSignal
import android.os.Handler
import android.os.HandlerThread
import android.os.OperationCanceledException
import android.os.ParcelFileDescriptor
import android.os.SystemClock

import android.util.Log

import android.provider.DocumentsContract
import android.provider.DocumentsContract.Document

/* Emacs runs long-running SAF operations on a second thread running
   its own handler.  These operations include opening files and
   maintaining the path to document ID cache.

   Because Emacs paths are based on file display names, while Android
   document identifiers have no discernible hierarchy of their own,
   each file name lookup must carry out a repeated search for
   directory documents with the names of all of the file name's
   constituent components, where each iteration searches within the
   directory document identified by the previous iteration.

   A time limited cache tying components to document IDs is maintained
   in order to speed up consecutive searches for file names sharing
   the same components.  Since listening for changes to each document
   in the cache is prohibitively expensive, Emacs instead elects to
   periodically remove entries that are older than a predetermined
   amount of a time.

   The cache is split into two levels: the first caches the
   relationships between display names and document IDs, while the
   second caches individual document IDs and their contents (children,
   type, etc.)

   Long-running operations are also run on this thread for another
   reason: Android uses special cancellation objects to terminate
   ongoing IPC operations.  However, the functions that perform these
   operations block instead of providing mechanisms for the caller to
   wait for their completion while also reading async input, as a
   consequence of which the calling thread is unable to signal the
   cancellation objects that it provides.  Performing the blocking
   operations in this auxiliary thread enables the main thread to wait
   for completion itself, signaling the cancellation objects when it
   deems necessary.  */

/* Nothing in C looks this class up: `android.c` 1724-1725 resolves the *facade* class
   `org/gnu/emacs/EmacsService` and finds the SAF entry points on it (`android.c` 1791
   `documentIdFromName` `(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)I`, 1797
   `statDocument` `(Ljava/lang/String;Ljava/lang/String;Z)[J`, 1799 `accessDocument`, 1801
   `openDocumentDirectory`, 1807 `openDocument`), which then forward to this thread's methods
   (`EmacsService.java` 1547, 1618, 1652, 1677, 1783).  So the member names below are obligations of
   `EmacsService.kt`'s future port, not of JNI, and are kept unchanged.  The class is `final` in the
   reference, so the port is a plain Kotlin `class`.

   Kotlin shapes forced by the language, recorded in `docs/ports/EmacsSafThread.md`: Java's octal
   mode constants (`0000400` ...) are written as the exact hexadecimal literals (Kotlin has no
   octal), `&`/`|`/`~` become the infix `and`/`or`/`inv` functions, and `Long` is compared against
   `0L` because Kotlin's `==` will not mix `Long` and `Int`; `handler` (assigned only in `start`)
   is `lateinit`, the reference's `Throwable throwable` temporaries are kept, the anonymous
   `Runnable`s are `object : Runnable` because two of them `return` early (a `return` is not
   permitted inside a SAM-converted lambda), `pruneCache1`'s `Collection` is a `MutableCollection`
   because it removes through the iterator, and the reference's generic `throws T` sneaky-throw
   collapses to a plain `throw` (Kotlin has no checked exceptions).  */

class EmacsSafThread(private val resolver: ContentResolver) :
  HandlerThread("Document provider access thread") {
  companion object {
    private const val TAG = "EmacsSafThread"

    /* File access mode constants.  See `man 7 inode'.  The reference writes these in octal
       (0000400, 0000200, 0000100, 0020000, 0040000, 0100000); Kotlin has no octal literals, so the
       port writes the same values in hexadecimal.  */
    const val S_IRUSR = 0x100
    const val S_IWUSR = 0x80
    const val S_IXUSR = 0x40
    const val S_IFCHR = 0x2000
    const val S_IFDIR = 0x4000
    const val S_IFREG = 0x8000

    /* Number of seconds in between each attempt to prune the storage
       cache.  */
    const val CACHE_PRUNE_TIME: Int = 10

    /* Number of seconds after which an entry in the cache is to be
       considered invalid.  */
    const val CACHE_INVALID_TIME: Int = 10

    /* Throw the specified EXCEPTION.  The reference's method is
       `private static <T extends Throwable> void throwException (Throwable) throws T' - a trick to
       defeat Java's checked exception rules.  Kotlin has no checked exceptions, so the port throws
       directly; the effect is identical.  */

    private fun throwException(exception: Throwable) {
      throw exception
    }
  }

  /* Map between tree URIs and the cache entry representing its
     toplevel directory.  */
  private val cacheToplevels: HashMap<Uri, CacheToplevel> = HashMap<Uri, CacheToplevel>()

  /* Handler for this thread's main loop.  */
  private lateinit var handler: Handler

  override fun start() {
    super.start()

    /* Set up the handler after the thread starts.  */
    handler = Handler(getLooper())

    /* And start periodically pruning the cache.  */
    postPruneMessage()
  }

  /* The three maps are filled in by `getCache' immediately after construction, as in the
     reference (which would NPE otherwise), so they are `lateinit`.  */
  private class CacheToplevel {
    /* Map between document names and children.  */
    lateinit var children: HashMap<String, DocIdEntry>

    /* Map between document names and file status.  */
    lateinit var statCache: HashMap<String, StatCacheEntry>

    /* Map between document IDs and cache items.  */
    lateinit var idCache: HashMap<String, CacheEntry>
  }

  private class StatCacheEntry {
    /* The time at which this cache entry was created.  */
    var time: Long = 0

    /* Flags, size, and modification time of this file.  */
    var flags: Long = 0
    var size: Long = 0
    var mtime: Long = 0

    /* Whether or not this file is a directory.  */
    var isDirectory: Boolean = false

    init {
      time = SystemClock.uptimeMillis()
    }

    fun isValid(): Boolean {
      return ((SystemClock.uptimeMillis() - time)
              < CACHE_INVALID_TIME * 1000)
    }
  }

  private class DocIdEntry {
    /* The document ID.  This is assigned immediately after construction in every path, so it
       is `lateinit` (the reference's field is only ever set before it is read).  */
    lateinit var documentId: String

    /* The time this entry was created.  */
    var time: Long = 0

    init {
      time = SystemClock.uptimeMillis()
    }

    /* Return a cache entry comprised of the state of the file
       identified by `documentId'.  TREE is the URI of the tree
       containing this entry, and TOPLEVEL is the toplevel
       representing it.  SIGNAL is a cancellation signal.

       RESOLVER is the content provider used to retrieve file
       information.

       Value is NULL if the file cannot be found.  */

    fun getCacheEntry(resolver: ContentResolver, tree: Uri,
                      toplevel: CacheToplevel,
                      signal: CancellationSignal): CacheEntry? {
      var uri: Uri?
      var projection: Array<String>
      var type: String?
      var cursor: Cursor?
      var column: Int
      var entry: CacheEntry

      /* Create a document URI representing DOCUMENTID within URI's
         authority.  */

      uri = DocumentsContract.buildDocumentUriUsingTree(tree,
                                                        documentId)
      projection = arrayOf(Document.COLUMN_MIME_TYPE)

      cursor = null

      try {
        cursor = resolver.query(uri, projection, null,
                                null, null, signal)

        if (!cursor!!.moveToFirst())
          return null

        column = cursor.getColumnIndex(Document.COLUMN_MIME_TYPE)

        if (column < 0)
          return null

        type = cursor.getString(column)

        if (type == null)
          return null

        entry = CacheEntry()
        entry.type = type
        toplevel.idCache.put(documentId, entry)
        return entry
      } catch (e: OperationCanceledException) {
        throw e
      } catch (e: Throwable) {
        return null
      } finally {
        if (cursor != null)
          cursor.close()
      }
    }

    fun isValid(): Boolean {
      return ((SystemClock.uptimeMillis() - time)
              < CACHE_INVALID_TIME * 1000)
    }
  }

  private class CacheEntry {
    /* The type of this document.  */
    var type: String? = null

    /* Map between document names and children.  */
    var children: HashMap<String, DocIdEntry> = HashMap<String, DocIdEntry>()

    /* The time this entry was created.  */
    var time: Long = 0

    init {
      time = SystemClock.uptimeMillis()
    }

    fun isValid(): Boolean {
      return ((SystemClock.uptimeMillis() - time)
              < CACHE_INVALID_TIME * 1000)
    }
  }

  /* Create or return a toplevel for the given tree URI.  */

  private fun getCache(uri: Uri): CacheToplevel {
    var toplevel: CacheToplevel?

    toplevel = cacheToplevels.get(uri)

    if (toplevel != null)
      return toplevel

    toplevel = CacheToplevel()
    toplevel.children = HashMap<String, DocIdEntry>()
    toplevel.statCache = HashMap<String, StatCacheEntry>()
    toplevel.idCache = HashMap<String, CacheEntry>()
    cacheToplevels.put(uri, toplevel)
    return toplevel
  }

  /* Remove each cache entry within COLLECTION older than
     CACHE_INVALID_TIME.  */

  private fun pruneCache1(collection: MutableCollection<DocIdEntry>) {
    var iter: MutableIterator<DocIdEntry>
    var tem: DocIdEntry

    iter = collection.iterator()
    while (iter.hasNext()) {
      /* Get the cache entry.  */
      tem = iter.next()

      /* If it's not valid anymore, remove it.  Iterating over a
         collection whose contents are being removed is undefined
         unless the removal is performed using the iterator's own
         `remove' function, so tem.remove cannot be used here.  */

      if (tem.isValid())
        continue

      iter.remove()
    }
  }

  /* Remove every entry older than CACHE_INVALID_TIME from each
     toplevel inside `cachedToplevels'.  */

  private fun pruneCache() {
    var iter: MutableIterator<CacheEntry>
    var statIter: MutableIterator<StatCacheEntry>
    var tem: CacheEntry
    var stat: StatCacheEntry

    for (toplevel in cacheToplevels.values) {
      /* First, clean up expired cache entries.  */
      iter = toplevel.idCache.values.iterator()

      while (iter.hasNext()) {
        /* Get the cache entry.  */
        tem = iter.next()

        /* If it's not valid anymore, remove it.  Iterating over a
           collection whose contents are being removed is
           undefined unless the removal is performed using the
           iterator's own `remove' function, so tem.remove cannot
           be used here.  */

        if (tem.isValid()) {
          /* Otherwise, clean up expired items in its document
             ID cache.  */
          pruneCache1(tem.children.values)
          continue
        }

        iter.remove()
      }

      statIter = toplevel.statCache.values.iterator()

      while (statIter.hasNext()) {
        /* Get the cache entry.  */
        stat = statIter.next()

        /* If it's not valid anymore, remove it.  Iterating over a
           collection whose contents are being removed is
           undefined unless the removal is performed using the
           iterator's own `remove' function, so tem.remove cannot
           be used here.  */

        if (stat.isValid())
          continue

        statIter.remove()
      }
    }

    postPruneMessage()
  }

  /* Cache file information within TOPLEVEL, under the list of
     children CHILDREN.

     NAME, ID, and TYPE should respectively be the display name of the
     document within its parent document (the CacheEntry whose
     `children' field is CHILDREN), its document ID, and its MIME
     type.

     If ID_ENTRY_EXISTS, don't create a new document ID entry within
     CHILDREN indexed by NAME.

     Value is the cache entry saved for the document ID.  */

  private fun cacheChild(toplevel: CacheToplevel,
                         children: HashMap<String, DocIdEntry>,
                         name: String, id: String, type: String?,
                         id_entry_exists: Boolean): CacheEntry {
    var idEntry: DocIdEntry
    var cacheEntry: CacheEntry

    if (!id_entry_exists) {
      idEntry = DocIdEntry()
      idEntry.documentId = id
      children.put(name, idEntry)
    }

    cacheEntry = CacheEntry()
    cacheEntry.type = type
    toplevel.idCache.put(id, cacheEntry)
    return cacheEntry
  }

  /* Cache file status for DOCUMENTID within TOPLEVEL.  Value is the
     new cache entry.  CURSOR is the cursor from where to retrieve the
     file status, in the form of the columns COLUMN_FLAGS,
     COLUMN_SIZE, COLUMN_MIME_TYPE and COLUMN_LAST_MODIFIED.

     If NO_CACHE, don't cache the file status; just return the
     entry.  */

  private fun cacheFileStatus(documentId: String, toplevel: CacheToplevel,
                              cursor: Cursor, no_cache: Boolean): StatCacheEntry? {
    var entry: StatCacheEntry
    var flagsIndex: Int
    var columnIndex: Int
    var typeIndex: Int
    var sizeIndex: Int
    var mtimeIndex: Int
    var type: String?

    /* Obtain the indices for columns wanted from this cursor.  */
    flagsIndex = cursor.getColumnIndex(Document.COLUMN_FLAGS)
    sizeIndex = cursor.getColumnIndex(Document.COLUMN_SIZE)
    typeIndex = cursor.getColumnIndex(Document.COLUMN_MIME_TYPE)
    mtimeIndex = cursor.getColumnIndex(Document.COLUMN_LAST_MODIFIED)

    /* COLUMN_LAST_MODIFIED is allowed to be absent in a
       conforming documents provider.  */
    if (flagsIndex < 0 || sizeIndex < 0 || typeIndex < 0)
      return null

    /* Get the file status from CURSOR.  */
    entry = StatCacheEntry()
    entry.flags = cursor.getInt(flagsIndex).toLong()
    type = cursor.getString(typeIndex)

    if (type == null)
      return null

    entry.isDirectory = type.equals(Document.MIME_TYPE_DIR)

    if (cursor.isNull(sizeIndex))
      /* The size is unknown.  */
      entry.size = -1
    else
      entry.size = cursor.getLong(sizeIndex)

    /* mtimeIndex is potentially unset, since document providers
       aren't obligated to provide modification times.  */

    if (mtimeIndex >= 0 && !cursor.isNull(mtimeIndex))
      entry.mtime = cursor.getLong(mtimeIndex)

    /* Finally, add this entry to the cache and return.  */
    if (!no_cache)
      toplevel.statCache.put(documentId, entry)
    return entry
  }

  /* Cache the type and as many of the children of the directory
     designated by DOCUMENTID as possible into TOPLEVEL.

     CURSOR should be a cursor representing an open directory stream,
     with its projection consisting of at least the display name,
     document ID and MIME type columns.

     Rewind the position of CURSOR to before its first element after
     completion.  */

  private fun cacheDirectoryFromCursor(toplevel: CacheToplevel, documentId: String,
                                       cursor: Cursor) {
    var entry: CacheEntry
    var constituent: CacheEntry
    var nameColumn: Int
    var idColumn: Int
    var typeColumn: Int
    var id: String?
    var name: String?
    var type: String?
    var idEntry: DocIdEntry

    /* Find the numbers of the columns wanted.  */

    nameColumn = cursor.getColumnIndex(Document.COLUMN_DISPLAY_NAME)
    idColumn = cursor.getColumnIndex(Document.COLUMN_DOCUMENT_ID)
    typeColumn = cursor.getColumnIndex(Document.COLUMN_MIME_TYPE)

    if (nameColumn < 0 || idColumn < 0 || typeColumn < 0)
      return

    entry = CacheEntry()

    /* We know this is a directory already.  */
    entry.type = Document.MIME_TYPE_DIR
    toplevel.idCache.put(documentId, entry)

    /* Now, try to cache each of its constituents.  */

    while (cursor.moveToNext()) {
      try {
        name = cursor.getString(nameColumn)
        id = cursor.getString(idColumn)
        type = cursor.getString(typeColumn)

        if (name == null || id == null || type == null)
          continue

        /* First, add the name and ID to ENTRY's map of
           children.  */
        idEntry = DocIdEntry()
        idEntry.documentId = id
        entry.children.put(id, idEntry)

        /* Cache the file status for ID within TOPELVEL too; if a
           directory listing is being requested, it's very likely
           that a series of calls for file status will follow.  */

        cacheFileStatus(id, toplevel, cursor, false)

        /* If this constituent is a directory, don't cache any
           information about it.  It cannot be cached without
           knowing its children.  */

        if (type.equals(Document.MIME_TYPE_DIR))
          continue

        /* Otherwise, create a new cache entry comprised of its
           type.  */
        constituent = CacheEntry()
        constituent.type = type
        toplevel.idCache.put(documentId, entry)
      } catch (e: Exception) {
        e.printStackTrace()
        continue
      }
    }

    /* Rewind cursor back to the beginning.  */
    cursor.moveToPosition(-1)
  }

  /* Post a message to run `pruneCache' every CACHE_PRUNE_TIME
     seconds.  */

  private fun postPruneMessage() {
    handler.postDelayed(object : Runnable {
      override fun run() {
        pruneCache()
      }
    }, (CACHE_PRUNE_TIME * 1000).toLong())
  }

  /* Invalidate the cache entry denoted by DOCUMENT_ID, within the
     document tree URI.
     Call this after deleting a document or directory.

     At the same time, remove the final component within the file name
     CACHENAME from the cache if it exists.  */

  fun postInvalidateCache(uri: Uri, documentId: String,
                          cacheName: String) {
    handler.post(object : Runnable {
      override fun run() {
        var toplevel: CacheToplevel
        var children: HashMap<String, DocIdEntry>?
        var components: List<String>
        var entry: CacheEntry?
        var idEntry: DocIdEntry?

        toplevel = getCache(uri)
        toplevel.idCache.remove(documentId)
        toplevel.statCache.remove(documentId)

        /* If the parent of CACHENAME is cached, remove it.  */

        children = toplevel.children
        components = cacheName.split("/")

        for (component in components) {
          /* Java `split' removes trailing empty matches but not
             leading or intermediary ones.  */
          if (component.isEmpty())
            continue

          if (component === components[components.size - 1]) {
            /* This is the last component, so remove it from
               children.  */
            children!!.remove(component)
            return
          } else {
            /* Search for this component within the last level
               of the cache.  */

            idEntry = children!!.get(component)

            if (idEntry == null)
              /* Not cached, so return.  */
              return

            entry = toplevel.idCache.get(idEntry.documentId)

            if (entry == null)
              /* Not cached, so return.  */
              return

            /* Locate the next component within this
               directory.  */
            children = entry.children
          }
        }
      }
    })
  }

  /* Invalidate the cache entry denoted by DOCUMENT_ID, within the
     document tree URI.
     Call this after deleting a document or directory.

     At the same time, remove the child referring to DOCUMENTID from
     within CACHENAME's cache entry if it exists.  */

  fun postInvalidateCacheDir(uri: Uri, documentId: String,
                             cacheName: String) {
    handler.post(object : Runnable {
      override fun run() {
        var toplevel: CacheToplevel
        var children: HashMap<String, DocIdEntry>?
        var components: List<String>
        var entry: CacheEntry?
        var idEntry: DocIdEntry?
        var iter: MutableIterator<DocIdEntry>

        toplevel = getCache(uri)
        toplevel.idCache.remove(documentId)
        toplevel.statCache.remove(documentId)

        /* Now remove DOCUMENTID from CACHENAME's cache entry, if
           any.  */

        children = toplevel.children
        components = cacheName.split("/")

        for (component in components) {
          /* Java `split' removes trailing empty matches but not
             leading or intermediary ones.  */
          if (component.isEmpty())
            continue

          /* Search for this component within the last level
             of the cache.  */

          idEntry = children!!.get(component)

          if (idEntry == null)
            /* Not cached, so return.  */
            return

          entry = toplevel.idCache.get(idEntry.documentId)

          if (entry == null)
            /* Not cached, so return.  */
            return

          /* Locate the next component within this
             directory.  */
          children = entry.children
        }

        iter = children!!.values.iterator()
        while (iter.hasNext()) {
          idEntry = iter.next()

          if (idEntry.documentId.equals(documentId)) {
            iter.remove()
            break
          }
        }
      }
    })
  }

  /* Invalidate the file status cache entry for DOCUMENTID within URI.
     Call this when the contents of a file (i.e. the constituents of a
     directory file) may have changed, but the document's display name
     has not.  */

  fun postInvalidateStat(uri: Uri, documentId: String) {
    handler.post(object : Runnable {
      override fun run() {
        var toplevel: CacheToplevel

        toplevel = getCache(uri)
        toplevel.statCache.remove(documentId)
      }
    })
  }

  /* ``Prototypes'' for nested functions that are run within the SAF
     thread and accepts a cancellation signal.  They differ in their
     return types.  */

  private abstract inner class SafIntFunction {
    /* The ``throws Throwable'' here is a Java idiosyncrasy that tells
       the compiler to allow arbitrary error objects to be signaled
       from within this function.

       Later, runIntFunction will try to re-throw any error object
       generated by this function in the Emacs thread, using a trick
       to avoid the compiler requirement to expressly declare that an
       error (and which types of errors) will be signaled.  */

    @Throws(Throwable::class)
    abstract fun runInt(signal: CancellationSignal): Int
  }

  private abstract inner class SafObjectFunction {
    /* The ``throws Throwable'' here is a Java idiosyncrasy that tells
       the compiler to allow arbitrary error objects to be signaled
       from within this function.

       Later, runObjectFunction will try to re-throw any error object
       generated by this function in the Emacs thread, using a trick
       to avoid the compiler requirement to expressly declare that an
       error (and which types of errors) will be signaled.  */

    @Throws(Throwable::class)
    abstract fun runObject(signal: CancellationSignal): Any?
  }

  /* Functions that run cancel-able queries.  These functions are
     internally run within the SAF thread.  */

  /* Run the given function (or rather, its `runInt' field) within the
     SAF thread, waiting for it to complete.

     If async input arrives in the meantime and sets Vquit_flag,
     signal the cancellation signal supplied to that function.

     Rethrow any exception thrown from that function, and return its
     value otherwise.  */

  private fun runIntFunction(function: SafIntFunction): Int {
    val result: EmacsHolder<Any>
    val signal: CancellationSignal
    var throwable: Throwable

    result = EmacsHolder<Any>()
    signal = CancellationSignal()

    handler.post(object : Runnable {
      override fun run() {
        try {
          result.thing = Integer.valueOf(function.runInt(signal))
        } catch (e: Throwable) {
          result.thing = e
        }

        EmacsNative.safPostRequest()
      }
    })

    if (EmacsNative.safSyncAndReadInput() != 0) {
      signal.cancel()

      /* Now wait for the function to finish.  Either the signal has
         arrived after the query took place, in which case it will
         finish normally, or an OperationCanceledException will be
         thrown.  */

      EmacsNative.safSync()
    }

    if (result.thing is Throwable) {
      throwable = result.thing as Throwable
      throwException(throwable)
    }

    return result.thing as Int
  }

  /* Run the given function (or rather, its `runObject' field) within
     the SAF thread, waiting for it to complete.

     If async input arrives in the meantime and sets Vquit_flag,
     signal the cancellation signal supplied to that function.

     Rethrow any exception thrown from that function, and return its
     value otherwise.  */

  private fun runObjectFunction(function: SafObjectFunction): Any? {
    val result: EmacsHolder<Any>
    val signal: CancellationSignal
    var throwable: Throwable

    result = EmacsHolder<Any>()
    signal = CancellationSignal()

    handler.post(object : Runnable {
      override fun run() {
        try {
          result.thing = function.runObject(signal)
        } catch (e: Throwable) {
          result.thing = e
        }

        EmacsNative.safPostRequest()
      }
    })

    if (EmacsNative.safSyncAndReadInput() != 0) {
      signal.cancel()

      /* Now wait for the function to finish.  Either the signal has
         arrived after the query took place, in which case it will
         finish normally, or an OperationCanceledException will be
         thrown.  */

      EmacsNative.safSync()
    }

    if (result.thing is Throwable) {
      throwable = result.thing as Throwable
      throwException(throwable)
    }

    return result.thing
  }

  /* The crux of `documentIdFromName1', run within the SAF thread.
     SIGNAL should be a cancellation signal run upon quitting.  */

  /* The reference passes `idEntry != null' to `cacheChild' below, but on that path `idEntry' is
     always null (the `if' above it `continue's when it is not), so the expression is a constant
     false; the port keeps the reference's expression verbatim and suppresses the compiler's note
     about it.  */
  @Suppress("SENSELESS_COMPARISON")
  private fun documentIdFromName1(tree_uri: String, name: String,
                                  id_return: Array<String?>,
                                  signal: CancellationSignal): Int {
    var uri: Uri
    var treeUri: Uri
    var id: String?
    var type: String?
    var newId: String?
    var newType: String?
    var components: List<String>
    var projection: Array<String>
    var cursor: Cursor?
    var nameColumn: Int
    var idColumn: Int
    var typeColumn: Int
    var toplevel: CacheToplevel
    var idEntry: DocIdEntry?
    var children: HashMap<String, DocIdEntry>?
    var next: HashMap<String, DocIdEntry>?
    var cache: CacheEntry?

    /* The reference reassigns its `name' parameter inside the loop below (P-83 ④: Kotlin
       parameters are immutable); the scratch value lives in this local, and the parameter is left
       to the initial split.  */
    var currentName: String?

    projection = arrayOf(
      Document.COLUMN_DISPLAY_NAME,
      Document.COLUMN_DOCUMENT_ID,
      Document.COLUMN_MIME_TYPE,
    )

    /* Parse the URI identifying the tree first.  */
    uri = Uri.parse(tree_uri)

    /* Now, split NAME into its individual components.  */
    components = name.split("/")

    /* Set id and type to the value at the root of the tree.  */
    type = null
    id = null
    cursor = null

    /* Obtain the top level of this cache.  */
    toplevel = getCache(uri)

    /* Set the current map of children to this top level.  */
    children = toplevel.children

    /* For each component... */

    try {
      for (component in components) {
        /* Java split doesn't behave very much like strtok when it
           comes to trailing and leading delimiters...  */
        if (component.isEmpty())
          continue

        /* Search for component within the currently cached list
           of children.  */

        idEntry = children!!.get(component)

        if (idEntry != null) {
          /* The document ID is known.  Now find the
             corresponding document ID cache.  */

          cache = toplevel.idCache.get(idEntry.documentId)

          /* Fetch just the information for this document.  */

          if (cache == null)
            cache = idEntry.getCacheEntry(resolver, uri, toplevel,
                                          signal)

          if (cache == null) {
            /* File status matching idEntry could not be
               obtained.  Treat this as if the file does not
               exist.  */

            children.remove(component)

            if (id == null)
              id = DocumentsContract.getTreeDocumentId(uri)

            id_return[0] = id

            if ((type == null
                 || type.equals(Document.MIME_TYPE_DIR))
                /* ... and type and id currently represent the
                   penultimate component.  */
                && component === components[components.size - 1])
              return -2

            return -1
          }

          /* Otherwise, use the cached information.  */
          id = idEntry.documentId
          type = cache.type
          children = cache.children
          continue
        }

        /* Create the tree URI for URI from ID if it exists, or
           the root otherwise.  */

        if (id == null)
          id = DocumentsContract.getTreeDocumentId(uri)

        treeUri = DocumentsContract.buildChildDocumentsUriUsingTree(uri, id)

        /* Look for a file in this directory by the name of
           component.  */

        cursor = resolver.query(treeUri, projection,
                                (Document.COLUMN_DISPLAY_NAME
                                 + " = ?"),
                                arrayOf(component),
                                null, signal)

        if (cursor == null)
          return -1

        /* Find the column numbers for each of the columns that
           are wanted.  */

        nameColumn = cursor.getColumnIndex(Document.COLUMN_DISPLAY_NAME)
        idColumn = cursor.getColumnIndex(Document.COLUMN_DOCUMENT_ID)
        typeColumn = cursor.getColumnIndex(Document.COLUMN_MIME_TYPE)

        if (nameColumn < 0 || idColumn < 0 || typeColumn < 0)
          return -1

        next = null

        while (true) {
          /* Even though the query selects for a specific
             display name, some content providers nevertheless
             return every file within the directory.  */

          if (!cursor.moveToNext()) {
            /* If a component has been found, break out of the
               loop.  */

            if (next != null)
              break

            /* If the last component considered is a
               directory... */
            if ((type == null
                 || type.equals(Document.MIME_TYPE_DIR))
                /* ... and type and id currently represent the
                   penultimate component.  */
                && component === components[components.size - 1]) {
              /* The cursor is empty.  In this case, return
                 -2 and the current document ID (belonging
                 to the previous component) in
                 ID_RETURN.  */

              id_return[0] = id

              /* But return -1 on the off chance that id is
                 null.  */

              if (id == null)
                return -1

              return -2
            }

            /* The last component found is not a directory, so
               return -1.  */
            return -1
          }

          /* So move CURSOR to a row with the right display
             name.  */

          currentName = cursor.getString(nameColumn)
          newId = cursor.getString(idColumn)
          newType = cursor.getString(typeColumn)

          /* Any of the three variables above may be NULL if the
             column data is of the wrong type depending on how
             the Cursor returned is implemented.  */

          if (currentName == null || newId == null || newType == null)
            return -1

          /* Cache this name, even if it isn't the document
             that's being searched for.  */

          cache = cacheChild(toplevel, children, currentName,
                             newId, newType,
                             idEntry != null)

          /* Record the desired component once it is located,
             but continue reading and caching items from the
             cursor.  */

          if (currentName.equals(component)) {
            id = newId
            next = cache.children
            type = newType
          }
        }

        children = next

        /* Now close the cursor.  */
        cursor.close()
        cursor = null

        /* ID may have become NULL if the data is in an invalid
           format.  */
        if (id == null)
          return -1
      }
    } finally {
      /* If an error is thrown within the block above, let
         android_saf_exception_check handle it, but make sure the
         cursor is closed.  */

      if (cursor != null)
        cursor.close()
    }

    /* Here, id is either NULL (meaning the same as TREE_URI), and
       type is either NULL (in which case id should also be NULL) or
       the MIME type of the file.  */

    /* First return the ID.  */

    if (id == null)
      id_return[0] = DocumentsContract.getTreeDocumentId(uri)
    else
      id_return[0] = id

    /* Next, return whether or not this is a directory.  */
    if (type == null || type.equals(Document.MIME_TYPE_DIR))
      return 1

    return 0
  }

  /* Find the document ID of the file within TREE_URI designated by
     NAME.

     NAME is a ``file name'' comprised of the display names of
     individual files.  Each constituent component prior to the last
     must name a directory file within TREE_URI.

     Upon success, return 0 or 1 (contingent upon whether or not the
     last component within NAME is a directory) and place the document
     ID of the named file in ID_RETURN[0].

     If the designated file can't be located, but each component of
     NAME up to the last component can and is a directory, return -2
     and the ID of the last component located in ID_RETURN[0].

     If the designated file can't be located, return -1, or signal one
     of OperationCanceledException, SecurityException,
     FileNotFoundException, or UnsupportedOperationException.  */

  fun documentIdFromName(tree_uri: String, name: String,
                         id_return: Array<String?>): Int {
    return runIntFunction(object : SafIntFunction() {
      override fun runInt(signal: CancellationSignal): Int {
        return documentIdFromName1(tree_uri, name, id_return,
                                   signal)
      }
    })
  }

  /* The bulk of `statDocument'.  SIGNAL should be a cancellation
     signal.  */

  private fun statDocument1(uri: String, documentId: String?,
                            signal: CancellationSignal, noCache: Boolean): LongArray? {
    var uriObject: Uri
    var tree: Uri
    var projection: Array<String>
    var stat: LongArray
    var cursor: Cursor?
    var toplevel: CacheToplevel
    var cache: StatCacheEntry?

    tree = Uri.parse(uri)

    /* The reference reassigns its `documentId' parameter here (P-83 ④); `?:' carries it into a
       non-null local with the same name, which is what the rest of the function needs.  */
    var documentId: String = documentId ?: DocumentsContract.getTreeDocumentId(tree)

    /* Create a document URI representing DOCUMENTID within URI's
       authority.  */

    uriObject = DocumentsContract.buildDocumentUriUsingTree(tree, documentId)

    /* See if the file status cache currently contains this
       document.  */

    toplevel = getCache(tree)
    cache = toplevel.statCache.get(documentId)

    if (cache == null || !cache.isValid()) {
      /* Stat this document and enter its information into the
         cache.  */

      projection = arrayOf(
        Document.COLUMN_FLAGS,
        Document.COLUMN_LAST_MODIFIED,
        Document.COLUMN_MIME_TYPE,
        Document.COLUMN_SIZE,
      )

      cursor = resolver.query(uriObject, projection, null,
                              null, null, signal)

      if (cursor == null)
        return null

      try {
        if (!cursor.moveToFirst())
          return null

        cache = cacheFileStatus(documentId, toplevel, cursor,
                                noCache)
      } finally {
        cursor.close()
      }

      /* If cache is still null, return null.  */

      if (cache == null)
        return null
    }

    /* Create the array of file status and populate it with the
       information within cache.  */
    stat = LongArray(3)

    stat[0] = stat[0] or S_IRUSR.toLong()
    if ((cache.flags and Document.FLAG_SUPPORTS_WRITE.toLong()) != 0L)
      stat[0] = stat[0] or S_IWUSR.toLong()

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N
        && (cache.flags and Document.FLAG_VIRTUAL_DOCUMENT.toLong()) != 0L)
      stat[0] = stat[0] or S_IFCHR.toLong()

    stat[1] = cache.size

    /* Check if this is a directory file.  */
    if (cache.isDirectory
        /* Files shouldn't be specials and directories at the same
           time, but Android doesn't forbid document providers
           from returning this information.  */
        && (stat[0] and S_IFCHR.toLong()) == 0L) {
      /* Since FLAG_SUPPORTS_WRITE doesn't apply to directories,
         just assume they're writable.  */
      stat[0] = stat[0] or (S_IFDIR or S_IWUSR or S_IXUSR).toLong()

      /* Directory files cannot be modified if
         FLAG_DIR_SUPPORTS_CREATE is not set.  */

      if ((cache.flags and Document.FLAG_DIR_SUPPORTS_CREATE.toLong()) == 0L)
        stat[0] = stat[0] and S_IWUSR.inv().toLong()
    }

    /* If this file is neither a character special nor a
       directory, indicate that it's a regular file.  */

    if ((stat[0] and (S_IFDIR or S_IFCHR).toLong()) == 0L)
      stat[0] = stat[0] or S_IFREG.toLong()

    stat[2] = cache.mtime
    return stat
  }

  /* Return file status for the document designated by the given
     DOCUMENTID and tree URI.  If DOCUMENTID is NULL, use the document
     ID in URI itself.

     Value is null upon failure, or an array of longs [MODE, SIZE,
     MTIM] upon success, where MODE contains the file type and access
     modes of the file as in `struct stat', SIZE is the size of the
     file in BYTES or -1 if not known, and MTIM is the time of the
     last modification to this file in milliseconds since 00:00,
     January 1st, 1970.

     If NOCACHE, refrain from placing the file status within the
     status cache.

     OperationCanceledException and other typical exceptions may be
     signaled upon receiving async input or other errors.  */

  fun statDocument(uri: String, documentId: String?,
                   noCache: Boolean): LongArray? {
    return runObjectFunction(object : SafObjectFunction() {
      override fun runObject(signal: CancellationSignal): Any? {
        return statDocument1(uri, documentId, signal, noCache)
      }
    }) as LongArray?
  }

  /* The bulk of `accessDocument'.  SIGNAL should be a cancellation
     signal.  */

  private fun accessDocument1(uri: String, documentId: String?, writable: Boolean,
                              signal: CancellationSignal): Int {
    var uriObject: Uri
    var projection: Array<String>
    var tem: Int
    var index: Int
    var tem1: String?
    var cursor: Cursor?
    var toplevel: CacheToplevel
    var entry: CacheEntry?

    uriObject = Uri.parse(uri)

    /* The reference reassigns its `documentId' parameter here (P-83 ④); see `statDocument1'.  */
    var documentId: String = documentId ?: DocumentsContract.getTreeDocumentId(uriObject)

    /* If WRITABLE is false and the document ID is cached, use its
       cached value instead.  This speeds up
       `directory-files-with-attributes' a little.  */

    if (!writable) {
      toplevel = getCache(uriObject)
      entry = toplevel.idCache.get(documentId)

      if (entry != null)
        return 0
    }

    /* Create a document URI representing DOCUMENTID within URI's
       authority.  */

    uriObject = DocumentsContract.buildDocumentUriUsingTree(uriObject, documentId)

    /* Now stat this document.  */

    projection = arrayOf(
      Document.COLUMN_FLAGS,
      Document.COLUMN_MIME_TYPE,
    )

    cursor = resolver.query(uriObject, projection, null,
                            null, null, signal)

    if (cursor == null)
      return -1

    try {
      if (!cursor.moveToFirst())
        return -1

      if (!writable)
        return 0

      index = cursor.getColumnIndex(Document.COLUMN_MIME_TYPE)
      if (index < 0)
        return -3

      /* Get the type of this file to check if it's a directory.  */
      tem1 = cursor.getString(index)

      /* Check if this is a directory file.  */
      if (tem1.equals(Document.MIME_TYPE_DIR)) {
        /* If so, don't check for FLAG_SUPPORTS_WRITE.
           Check for FLAG_DIR_SUPPORTS_CREATE instead.  */

        index = cursor.getColumnIndex(Document.COLUMN_FLAGS)
        if (index < 0)
          return -3

        tem = cursor.getInt(index)
        if ((tem and Document.FLAG_DIR_SUPPORTS_CREATE) == 0)
          return -3

        return 0
      }

      index = cursor.getColumnIndex(Document.COLUMN_FLAGS)
      if (index < 0)
        return -3

      tem = cursor.getInt(index)
      if (writable && (tem and Document.FLAG_SUPPORTS_WRITE) == 0)
        return -3
    } finally {
      /* Close the cursor if an exception occurs.  */
      cursor.close()
    }

    return 0
  }

  /* Find out whether Emacs has access to the document designated by
     the specified DOCUMENTID within the tree URI.  If DOCUMENTID is
     NULL, use the document ID in URI itself.

     If WRITABLE, also check that the file is writable, which is true
     if it is either a directory or its flags contains
     FLAG_SUPPORTS_WRITE.

     Value is 0 if the file is accessible, and one of the following if
     not:

     -1, if the file does not exist.
     -2, if WRITABLE and the file is not writable.
     -3, upon any other error.

     In addition, arbitrary runtime exceptions (such as
     SecurityException or UnsupportedOperationException) may be
     thrown.  */

  fun accessDocument(uri: String, documentId: String?,
                     writable: Boolean): Int {
    return runIntFunction(object : SafIntFunction() {
      override fun runInt(signal: CancellationSignal): Int {
        return accessDocument1(uri, documentId, writable,
                               signal)
      }
    })
  }

  /* The crux of openDocumentDirectory.  SIGNAL must be a cancellation
     signal.  */

  private fun openDocumentDirectory1(uri: String, documentId: String?,
                                     signal: CancellationSignal): Cursor? {
    var uriObject: Uri
    var tree: Uri
    var cursor: Cursor?
    var projection: Array<String>
    var toplevel: CacheToplevel

    tree = Uri.parse(uri)
    uriObject = tree

    /* If documentId is not set, use the document ID of the tree URI
       itself.  The reference reassigns its parameter (P-83 ④); see
       `statDocument1'.  */

    var documentId: String = documentId ?: DocumentsContract.getTreeDocumentId(uriObject)

    /* Build a URI representing each directory entry within
       DOCUMENTID.  */

    uriObject = DocumentsContract.buildChildDocumentsUriUsingTree(uriObject,
                                                          documentId)

    projection = arrayOf(
      Document.COLUMN_DISPLAY_NAME,
      Document.COLUMN_DOCUMENT_ID,
      Document.COLUMN_MIME_TYPE,
      Document.COLUMN_FLAGS,
      Document.COLUMN_LAST_MODIFIED,
      Document.COLUMN_SIZE,
    )

    cursor = resolver.query(uriObject, projection, null, null,
                            null, signal)

    /* Create a new cache entry tied to this document ID.  */

    if (cursor != null) {
      toplevel = getCache(tree)
      cacheDirectoryFromCursor(toplevel, documentId,
                               cursor)
    }

    /* Return the cursor.  */
    return cursor
  }

  /* Open a cursor representing each entry within the directory
     designated by the specified DOCUMENTID within the tree URI.

     If DOCUMENTID is NULL, use the document ID within URI itself.
     Value is NULL upon failure.

     In addition, arbitrary runtime exceptions (such as
     SecurityException or UnsupportedOperationException) may be
     thrown.  */

  fun openDocumentDirectory(uri: String, documentId: String?): Cursor? {
    return runObjectFunction(object : SafObjectFunction() {
      override fun runObject(signal: CancellationSignal): Any? {
        return openDocumentDirectory1(uri, documentId, signal)
      }
    }) as Cursor?
  }

  /* The crux of `openDocument'.  SIGNAL must be a cancellation
     signal.  */

  fun openDocument1(uri: String, documentId: String, read: Boolean,
                    write: Boolean, truncate: Boolean,
                    signal: CancellationSignal): ParcelFileDescriptor? {
    var treeUri: Uri
    var documentUri: Uri
    var mode: String
    var fileDescriptor: ParcelFileDescriptor?
    var toplevel: CacheToplevel

    treeUri = Uri.parse(uri)

    /* documentId must be set for this request, since it doesn't make
       sense to ``open'' the root of the directory tree.  */

    documentUri = DocumentsContract.buildDocumentUriUsingTree(treeUri, documentId)

    /* Select the mode used to open the file.  */

    if (write) {
      if (read) {
        if (truncate)
          mode = "rwt"
        else
          mode = "rw"
      } else
        /* Set mode to w when WRITE && !READ, disregarding TRUNCATE.
           In contradiction with the ContentResolver documentation,
           document providers seem to truncate files whenever w is
           specified, at least superficially.  (But see below.)  */
        mode = "w"
    } else
      mode = "r"

    fileDescriptor = resolver.openFileDescriptor(documentUri, mode,
                                    signal)

    /* If a writable on-disk file descriptor is requested and TRUNCATE
       is set, then probe the file descriptor to detect if it is
       actually readable.  If not, close this file descriptor and
       reopen it with MODE set to rw; some document providers granting
       access to Samba shares don't implement rwt, but these document
       providers invariably truncate the file opened even when the
       mode is merely w.

       This may be ascribed to a mix-up in Android's documentation
       regarding DocumentsProvider: the `openDocument' function is only
       documented to accept r or rw, whereas the default implementation
       of the `openFile' function (which documents rwt) delegates to
       `openDocument'.  */

    if (read && write && truncate && fileDescriptor != null
        && !EmacsNative.ftruncate(fileDescriptor.getFd())) {
      try {
        fileDescriptor.closeWithError("File descriptor requested"
                                        + " is not writable")
      } catch (e: IOException) {
        Log.w(TAG, "Leaking unclosed file descriptor " + e)
      }

      fileDescriptor = resolver.openFileDescriptor(documentUri, "rw", signal)

      /* Try to truncate fileDescriptor just to stay on the safe
         side.  */
      if (fileDescriptor != null)
        EmacsNative.ftruncate(fileDescriptor.getFd())
    } else if (!read && write && truncate && fileDescriptor != null)
      /* Moreover, document providers that return actual seekable
         files characteristically neglect to truncate the file
         returned when the access mode is merely w, so attempt to
         truncate it by hand.  */
      EmacsNative.ftruncate(fileDescriptor.getFd())

    /* Every time a document is opened, remove it from the file status
       cache.  */
    toplevel = getCache(treeUri)
    toplevel.statCache.remove(documentId)

    return fileDescriptor
  }

  /* Open a file descriptor for a file document designated by
     DOCUMENTID within the document tree identified by URI.  If
     TRUNCATE and the document already exists, truncate its contents
     before returning.

     If READ && WRITE, open the file under either the `rw' or `rwt'
     access mode, which implies that the value must be a seekable
     on-disk file.  If WRITE && !READ or TRUNC && WRITE, also truncate
     the file after it is opened.

     If only READ or WRITE is set, value may be a non-seekable FIFO or
     one end of a socket pair.

     Value is NULL upon failure or a parcel file descriptor upon
     success.  Call `ParcelFileDescriptor.close' on this file
     descriptor instead of using the `close' system call.

     FileNotFoundException and/or SecurityException and/or
     UnsupportedOperationException and/or OperationCanceledException
     may be thrown upon failure.  */

  fun openDocument(uri: String, documentId: String,
                   read: Boolean, write: Boolean,
                   truncate: Boolean): ParcelFileDescriptor? {
    var tem: Any?

    tem = runObjectFunction(object : SafObjectFunction() {
      override fun runObject(signal: CancellationSignal): Any? {
        return openDocument1(uri, documentId, read,
                             write, truncate, signal)
      }
    })

    return tem as ParcelFileDescriptor?
  }
}
