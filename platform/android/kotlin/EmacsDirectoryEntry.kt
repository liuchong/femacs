// femacs: port of java/org/gnu/emacs/EmacsDirectoryEntry.java @ fd7afced403.

package org.femacs

/* Structure holding a single ``directory entry'' from a document
   provider.  */

class EmacsDirectoryEntry {
  /* The type of this directory entry.  0 means a regular file and 1
     means a directory.  */
  /* `androidvfs.c` 366-369 finds both fields through the JNI
     (`GetFieldID (…, "d_type", "I")` and `"d_name"`), so they keep their names and their JVM
     types: `@JvmField var d_type: Int` and `var d_name: String?` render `I` and
     `Ljava/lang/String;` (PORTING.md P-83 ②).  */
  @JvmField var d_type: Int = 0

  /* The display name of the file represented.  */
  @JvmField var d_name: String? = null
}
