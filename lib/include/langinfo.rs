// femacs: mirror of the frozen target's <langinfo.h> (system header, P-26).
//
// regex_internal.c 208 (the live arm) asks `nl_langinfo (CODESET)` for the locale's
// encoding; the frozen target's `CODESET` is the `nl_item` of that name.


// [U1-MODULE-MIRROR]
use super::*;
/// <langinfo.h>'s `nl_item` (the frozen target: `int`).
pub type nl_item = i32;

/// The frozen target's `CODESET`, measured with `clang -E -dM -include langinfo.h`: **0**.
/// (The mirror said 14 until status 3.86, which is glibc's `_NL_CTYPE_CODESET_NAME` - so
/// `Flocale_info`'s `codeset` branch asked `nl_langinfo` for the wrong item on this target.
/// The fns lane found it by comparing the mirror against the frozen preprocessor.)
pub const CODESET: nl_item = 0;

/* The day and month item macros (measured the same way: DAY_1 7 ... DAY_7 13, MON_1 21 ...
   MON_12 32).  `src/fns.c`'s `Flocale_info` reads all of them; the unit declared them for
   itself until this landed (status 3.86).  */
/// <langinfo.h>: `#define DAY_1 7`.
pub const DAY_1: nl_item = 7;
/// <langinfo.h>: `#define DAY_2 8`.
pub const DAY_2: nl_item = 8;
/// <langinfo.h>: `#define DAY_3 9`.
pub const DAY_3: nl_item = 9;
/// <langinfo.h>: `#define DAY_4 10`.
pub const DAY_4: nl_item = 10;
/// <langinfo.h>: `#define DAY_5 11`.
pub const DAY_5: nl_item = 11;
/// <langinfo.h>: `#define DAY_6 12`.
pub const DAY_6: nl_item = 12;
/// <langinfo.h>: `#define DAY_7 13`.
pub const DAY_7: nl_item = 13;
/// <langinfo.h>: `#define MON_1 21`.
pub const MON_1: nl_item = 21;
/// <langinfo.h>: `#define MON_2 22`.
pub const MON_2: nl_item = 22;
/// <langinfo.h>: `#define MON_3 23`.
pub const MON_3: nl_item = 23;
/// <langinfo.h>: `#define MON_4 24`.
pub const MON_4: nl_item = 24;
/// <langinfo.h>: `#define MON_5 25`.
pub const MON_5: nl_item = 25;
/// <langinfo.h>: `#define MON_6 26`.
pub const MON_6: nl_item = 26;
/// <langinfo.h>: `#define MON_7 27`.
pub const MON_7: nl_item = 27;
/// <langinfo.h>: `#define MON_8 28`.
pub const MON_8: nl_item = 28;
/// <langinfo.h>: `#define MON_9 29`.
pub const MON_9: nl_item = 29;
/// <langinfo.h>: `#define MON_10 30`.
pub const MON_10: nl_item = 30;
/// <langinfo.h>: `#define MON_11 31`.
pub const MON_11: nl_item = 31;
/// <langinfo.h>: `#define MON_12 32`.
pub const MON_12: nl_item = 32;

unsafe extern "C" {
  /* char *nl_langinfo (nl_item item);  */
  pub fn nl_langinfo (item: nl_item) -> *mut c_char;
}
