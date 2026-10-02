/* The C side of spikes/va-probe.sh: the va_list shapes doprnt.c uses — a variadic entry,
   forwarding the list to a function that takes `va_list`, and va_copy giving the callee an
   independent cursor while the caller keeps reading its own.  Pure C on purpose: the probe
   is about the mechanism, not about the Emacs mirrors.  Every read below reads an argument
   that was actually passed: the first version read an `int` where the next argument was a
   `double`, so two of the six lines were deterministic garbage that matched on both sides
   and could not tell a correct cursor from an equally wrong pair (review D's F7).  */
#include <stdio.h>
#include <stdarg.h>

static void take_va (char const *tag, va_list ap)
{
  int i = va_arg (ap, int);
  double d = va_arg (ap, double);
  long l = va_arg (ap, long);
  unsigned long ul = va_arg (ap, unsigned long);
  char *s = va_arg (ap, char *);
  printf ("%s take %d %.1f %ld %lu %s\n", tag, i, d, l, ul, s);
}

static void copy_then_read (char const *tag, va_list ap)
{
  va_list c;
  va_copy (c, ap);
  int from_copy = va_arg (c, int);      /* the copy advances on its own */
  int from_orig = va_arg (ap, int);     /* the caller's cursor is untouched */
  double more = va_arg (c, double);     /* and the copy keeps its own position */
  va_end (c);
  printf ("%s copy %d orig %d more %.1f\n", tag, from_copy, from_orig, more);
}

static void run (char const *tag, ...)
{
  va_list ap;
  va_start (ap, tag);
  take_va (tag, ap);            /* the list is passed by value, which on this target is a
                                   `char *` copy, so its five reads do not move our cursor
                                   (PORTING.md P-04 records the platform model) */
  copy_then_read (tag, ap);
  printf ("%s after %d %.1f\n", tag, va_arg (ap, int), va_arg (ap, double));
  va_end (ap);
}

int main (void)
{
  char *s = "str";
  run ("A", 1, 2.5, 3L, 4UL, s, 11, 22, 33);
  run ("B", -7, -0.5, -9L, 10UL, s, 44, 55, 66);
  return 0;
}
