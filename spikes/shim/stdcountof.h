/* compile-only stand-in for the C23 <stdcountof.h>. */
#ifndef SPIKE_STDCOUNTOF_H
#define SPIKE_STDCOUNTOF_H
#define countof(a) (sizeof (a) / sizeof *(a))
#endif
