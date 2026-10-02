/* The C side of spikes/va-probe.sh: the va_list shapes doprnt.c uses — a variadic entry,
   forwarding the list to a function that takes `va_list`, and va_copy giving the callee an
   independent cursor while the caller keeps reading its own.  Pure C on purpose: the probe
   is about the mechanism, not about the Emacs mirrors.  */
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
  int from_copy = va_arg (c, int);     /* the copy advances on its own */
  int from_orig = va_arg (ap, int);    /* the caller's cursor is untouched */
  int via_copy_again = va_arg (c, int);
  va_end (c);
  printf ("%s copy %d orig %d copy-again %d\n", tag, from_copy, from_orig, via_copy_again);
}

static void run (char const *tag, ...)
{
  va_list ap;
  va_start (ap, tag);
  take_va (tag, ap);            /* the list is passed by value (a struct copy here), so
                                   its five reads do not move our cursor */
  copy_then_read (tag, ap);
  printf ("%s after %d %d\n", tag, va_arg (ap, int), va_arg (ap, int));
  va_end (ap);
}

int main (void)
{
  char *s = "str";
  run ("A", 1, 2.5, 3L, 4UL, s, 11, 22, 33);
  run ("B", -7, -0.5, -9L, 10UL, s, 44, 55, 66);
  return 0;
}
