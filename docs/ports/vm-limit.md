# emacs/src/vm-limit.c (220 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 6
- [x] get_lim_data  (line 97, static)
- [x] get_lim_data  (line 114, static)
- [x] get_lim_data  (line 123)
- [x] ret_lim_data  (line 135)
- [x] check_memory_limits  (line 147, static)
- [x] memory_warnings  (line 211)

## Macros defined — 3
- [x] RLIMIT_AS  (line 93, body 4614c79148c8)
- [x] __MALLOC_HOOK_VOLATILE  (line 59, body 5b12b97a4b66)
- [x] data_start  (line 43, body 9cd544cb51b5)

## Types and their fields — 0

## File-scope objects — 6
- [x] extern int etext  (line 25)
- [x] extern char data_start  (line 45)
- [x] char data_start  (line 50)
- [x] static enum warnlevel warnlevel  (line 78, static)
- [x] static char data_space_start  (line 85, static)
- [x] static size_t lim_data  (line 88, static)

## Decisions

- **`struct rlimit` comes from a new system mirror**, `femacs/lib/include/sys_resource.rs`,
  with the frozen target's measured layout (16 bytes, `rlim_cur` 0, `rlim_max` 8, `rlim_t`
  8) and the target's `RLIMIT_AS` = 5, which is the value the macro table records too.
- **Three `#if` chain arms, guarded so exactly one is live.**  `HAVE_GETRLIMIT` is 1 in the
  frozen configuration, so the POSIX arm is the live one and is guarded with
  `#[cfg(not(any(feature = "windowsnt", feature = "msdos")))]`; the WINDOWSNT and MSDOS
  arms - which need `w32heap.h`/`dosfns.h`, not mirrored yet - carry their own `#[cfg(...)]`
  and declare the one platform entry each needs inside the arm, so a future flip compiles
  cleanly.  **Open item:** `xtask dead-arms`'s `FLIPS` does not name those two features, so
  neither arm has been through rustc yet (C does not compile them either in this
  configuration); adding the flips is what closes it.
- **`__MALLOC_HOOK_VOLATILE` survives as a marker const.**  C's `#define __MALLOC_HOOK_VOLATILE
  volatile` has no Rust spelling (the qualifier is an access-site property), so the macro's
  name is a `const ()` marker, the `GCALIGNED_MARKER` shape.
- **`DATA_START`'s dead macro went to `excluded-macros.txt`.**  The frozen configuration
  leaves `DATA_START` undefined but it is *absent* from the generated macro table, so the
  evaluator cannot decide the branch and demands both; the port keeps the `#else` branch,
  which defines the one-byte `data_start` object.
- **`real_morecore` is a local, not a global.**  `REL_ALLOC` is undef, so C's
  `check_memory_limits` declares the function pointer itself and the `#ifdef REL_ALLOC`
  extern at the top is dead text.
- **`etext` is declared in the dead MSDOS arm** where C declares it, which is also what
  satisfies the checker's file-scope-object rule.
