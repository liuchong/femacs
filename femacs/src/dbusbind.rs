// femacs: port of src/dbusbind.c @ fd7afced403.
/* femacs: emacs/src/dbusbind.c (@ 2193 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef HAVE_DBUS` at C line 21, and the frozen target's
   configuration leaves HAVE_DBUS undefined (docs/baseline/config/darwin-arm64.macros.tsv
   662, `HAVE_DBUS<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the D-Bus bindings - needs
   <dbus/dbus.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
