// femacs: mirror of msdos/langinfo.h @ fd7afced403.
//
// The MS-DOS replacement header: DJGPP has no `langinfo.h`, so this one file answers the only
// question the C tree asks of it, the codeset.  The frozen configuration (darwin-arm64) never
// puts `msdos/` on its include path and compiles no `msdos/` translation unit, so the mirror is
// text only - the declaration gate's `xtask layout-check` covers `src/include` and `lib/include`.

/* C 16: `# define nl_langinfo(ignore)  "cp437"` - the whole header.  A string-literal macro is
   a `const` of the target's own type (status 3.91's mirror rule), and the port's callers spell
   the application as C's text does, `nl_langinfo! (item)`.  */
macro_rules! nl_langinfo {
  ($ignore:expr) => { c"cp437" };
}
