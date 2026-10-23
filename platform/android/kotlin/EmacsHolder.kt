// femacs: port of java/org/gnu/emacs/EmacsHolder.java @ fd7afced403.

package org.femacs

/* This class serves as a simple reference to an object of type T.
   Nothing could be found inside the standard library.  */

/* Java's `T thing;` is package-private and Kotlin has no package-private: `internal` would mangle
   the JVM member name, so the port uses `@JvmField` with the default visibility and records the
   widening (PORTING.md P-83 ③).  */
class EmacsHolder<T> {
  @JvmField var thing: T? = null
}
