// femacs: port of src/w32select.c @ fd7afced403.
/*
 * Notes on usage of selection-coding-system and
 * next-selection-coding-system on MS Windows:
 *
 * The selection coding system variables apply only to the version of
 * the clipboard data that is closest in type, i.e. when a 16-bit
 * Unicode coding system is given, they apply to he Unicode clipboard
 * (CF_UNICODETEXT), when a well-known console codepage is given, they
 * apply to the console version of the clipboard data (CF_OEMTEXT),
 * else they apply to the normal 8-bit text clipboard (CF_TEXT).
 *
 * When pasting (getting data from the OS), the clipboard format that
 * matches the {next-}selection-coding-system is retrieved.  If
 * Unicode is requested, but not available, 8-bit text (CF_TEXT) is
 * used.  In all other cases the OS will transparently convert
 * formats, so no other fallback is needed.
 *
 * When copying or cutting (sending data to the OS), the data is
 * announced and stored internally, but only actually rendered on
 * request.  The requestor determines the format provided.  The
 * {next-}selection-coding-system is only used, when its corresponding
 * clipboard type matches the type requested.
 *
 * Scenarios to use the facilities for customizing the selection
 * coding system are:
 *
 *   ;; Generally use KOI8-R instead of the russian MS codepage for
 *   ;; the 8-bit clipboard.
 *   (set-selection-coding-system 'koi8-r-dos)
 *
 * Or
 *
 *   ;; Create a special clipboard copy function that uses codepage
 *   ;; 1253 (Greek) to copy Greek text to a specific non-Unicode
 *   ;; application.
 *   (defun greek-copy (beg end)
 *     (interactive "r")
 *     (set-next-selection-coding-system 'cp1253-dos)
 *     (copy-region-as-kill beg end))
 *   (global-set-key "\C-c\C-c" 'greek-copy)
 */

/*
 * Ideas for further directions:
 *
 * The encoding and decoding routines could be moved to Lisp code
 * similar to how xselect.c does it (using well-known routine names
 * for the delayed rendering).  If the definition of which clipboard
 * types should be supported is also moved to Lisp, functionality
 * could be expanded to CF_HTML, CF_RTF and maybe other types.
 */


/* femacs: emacs/src/w32select.c (@ 1370 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/w32select.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
