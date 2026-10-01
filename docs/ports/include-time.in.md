# emacs/lib/time.in.h (646 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 26
- [ ] GNULIB_defined_TIME_UTC  (line 123, body 356a192b7913)
- [ ] GNULIB_defined_struct_time_t_must_be_integral  (line 116, body 356a192b7913)
- [ ] GNULIB_defined_struct_timespec  (line 97, body 356a192b7913)
- [ ] GNULIB_defined_timezone_t  (line 499, body 356a192b7913)
- [ ] TIME_UTC  (line 122, body 356a192b7913)
- [ ] _  (line 45, body 6564569645de)
- [ ] _POSIX_THREAD_SAFE_FUNCTIONS  (line 52, body 6d19a880eaf5)
- [ ] ctime  (line 437, body a9a50e1ed098)
- [ ] gmtime  (line 400, body 9c780a8e1938)
- [ ] gmtime_r  (line 341, body cd56458d6151)
- [ ] localtime  (line 378, body bb9db16d3c93)
- [ ] localtime_r  (line 319, body c6a96e77aa28)
- [ ] localtime_rz  (line 539, body 51cb23a8ac47)
- [ ] mktime  (line 295, body cfb74ca8ad18)
- [ ] mktime_z  (line 568, body 1928894644b3)
- [ ] nanosleep  (line 227, body 1c19a527b56d)
- [ ] strftime  (line 460, body b8e523af4963)
- [ ] time  (line 204, body 474ea98f747c)
- [ ] timegm  (line 598, body b6b11795f8b7)
- [ ] timespec  (line 91, body 556386842976)
- [ ] timespec_get  (line 150, body f753e41133a6)
- [ ] timespec_getres  (line 178, body f9b159d93189)
- [ ] timezone_t  (line 497, body 85642344ef43)
- [ ] tzfree  (line 525, body 45cbede3b328)
- [ ] tzname  (line 140, body 7adeb914e850)
- [ ] tzset  (line 277, body cf790664c255)

## Types and their fields — 1
- [ ] timespec: time_t tv_sec, long int tv_nsec

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/time.in.h` @ `fd7afced403`; port `femacs/lib/include/time.in.rs`.

Deliberately partial: it carries `time_t` and `struct timespec` (which `buffer.h`
embeds as `modtime`), reached the way C's `<time.h>` is.  `struct timespec`'s size and
the offsets inside `struct buffer` are asserted from the C compiler's numbers
(`sizeof timespec == 16` on the frozen target: `tv_sec` is `time_t`, `tv_nsec` a
`c_long`).  The unchecked boxes are the rest of the header.
