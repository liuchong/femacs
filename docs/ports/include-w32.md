# emacs/src/w32.h (273 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 23
- [x] CHILD_ACTIVE  (line 117, body a34f0ef898f5)
- [x] EMACS_W32_H  (line 2, body da39a3ee5e6b)
- [x] FD_CLR  (line 48, body e2bff60d0e12)
- [x] FD_ISSET  (line 54, body e13c63378c93)
- [x] FD_SET  (line 42, body 701fdab21973)
- [x] FD_SETSIZE  (line 36, body 27285271b352)
- [x] FD_ZERO  (line 55, body 74fba1f8a77b)
- [x] FILE_AT_EOF  (line 136, body 7232895c5fbc)
- [x] FILE_BINARY  (line 134, body d73afeb43f0b)
- [x] FILE_CONNECT  (line 133, body 92df71781c5c)
- [x] FILE_DONT_CLOSE  (line 142, body 987a3bdd4907)
- [x] FILE_LAST_CR  (line 135, body 19fbaf699b05)
- [x] FILE_LISTEN  (line 132, body eeed498aee81)
- [x] FILE_NDELAY  (line 140, body e0b28e99ef1e)
- [x] FILE_PIPE  (line 138, body ecdf501256ce)
- [x] FILE_READ  (line 130, body f2b1649be8f5)
- [x] FILE_SEND_SIGCHLD  (line 137, body 3be01b55ecdc)
- [x] FILE_SERIAL  (line 141, body 96d5e6218d19)
- [x] FILE_SOCKET  (line 139, body d228a45608c3)
- [x] FILE_WRITE  (line 131, body 5f399b2ba212)
- [x] MAXDESC  (line 115, body 29262fd8aede)
- [x] MAX_CHILDREN  (line 116, body 1db7f9a3063e)
- [x] SELECT_TYPE  (line 57, body fefd40c7954b)

## Types and their fields — 3
- [x] <anon@120>: unsigned flags, HANDLE hnd, child_process* cp
- [x] <anon@37>: unsigned int bits
- [x] _child_process: int fd, int pid, HANDLE char_avail, HANDLE char_consumed, HANDLE thrd, HWND hwnd, PROCESS_INFORMATION procinfo, volatile int status, volatile int errcode, char chr, OVERLAPPED ovl_read, OVERLAPPED ovl_write

## File-scope objects — 4
- [x] extern filedesc fd_info  (line 127)
- [x] extern MultiByteToWideChar_Proc pMultiByteToWideChar  (line 191)
- [x] extern WideCharToMultiByte_Proc pWideCharToMultiByte  (line 192)
- [x] extern DWORD multiByteToWideCharFlags  (line 193)

## Decisions

- **The header is Windows-only, and the frozen target compiles none of it.**  Its readers are the
  Windows units (`src/w32.c`, `src/w32proc.c`, `src/w32fns.c`, `src/w32term.c`, …), `WINDOWSNT` and
  `HAVE_NTGUI` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, and C 23-25's
  `#ifdef CYGWIN / #error "w32.h is not compatible with Cygwin"` is the header's own statement that
  it belongs to one configuration only.  `femacs/src/w32.rs` is the empty unit that records the
  exclusion, no unit `include!`s this mirror, and the declarations are deferred to the Windows
  target with their definitions.  The `#error` is ported as `const _: () = assert! (!cfg!
  (feature = "cygwin"));` - it is a hard exclusion rather than a branch, so the port expresses it
  as the assertion that fails when the Cygwin feature is on.
- **The `FD_SETSIZE` cond-eval disagreement is recorded, not resolved.**
  `python3 tools/check-cond-eval.py src/w32.h` reports `4 conditions, 0 unknown, 1 disagreement
  [NOT VERIFIED]`, with `DISAGREE src/w32.h:32 `ifdef FD_SETSIZE` — clang False, evaluator True`.
  C 32-34 only `#undef`s a macro the MSVC runtime may have defined first and C 36 then defines the
  name to 2048, so **both arms end with the same text**; the file stays NOT VERIFIED and the
  extractor keeps its conservative "both arms live" reading (the `docs/ports/include-assert.in.md`
  precedent for an undecidable condition, status 3.157).  Rust has no pre-existing macro to
  remove, so the `#undef` itself has no counterpart (PORTING.md 0.1-4).
- **Names C takes from `<windows.h>` (C 25) are degraded, each with C's text in a `/// C:` line**
  (PORTING.md P-55/P-72): `HANDLE`/`HWND`/`HMODULE`/`HKEY` -> `*mut c_void` (opaque handles are
  pointers at the ABI); `BOOL` -> `c_int`; `UINT`/`unsigned` -> `c_uint`; `DWORD` -> `c_ulong`;
  `LPBYTE` -> `*mut u8`; `LPDWORD` -> `*mut c_ulong`; `CPINFO` only ever appears as a pointer;
  `WINAPI` is the `extern "system"` ABI of C 189-190's two entry-point pointers (nullable, so each
  is an `Option`, PORTING.md P-27); `HANDLE handles[3]` (C 172, C 177) decays to
  `*mut *mut c_void`; `ptrdiff_t` is `isize`; `size_t` is `usize`; and `wchar_t` is spelled
  `c_ushort` because *this* header's is the Windows 16-bit one, while `lib/include/wchar.rs` 10
  owns the frozen target's `int` (a `/// C:` line records each).
- **`_child_process` (C 84-110) is deferred, not written.**  C embeds `<windows.h>`'s
  `PROCESS_INFORMATION` (C 103) and two `OVERLAPPED` records (C 107-108) **by value**; the port has
  no mirror of that header and no compiler on this target could measure those records, so no Rust
  spelling is written rather than an unmeasured one (the `include/w32heap.rs` `file_data` rule and
  the `include/w32common.rs` C 35-36 deferral, from the same class).  Its checklist box stays
  unchecked, which is what makes the extractor print `missing type deferred: src/w32.h
  _child_process/child_process (the ledger has unchecked boxes)` on stderr instead of failing a
  check; every pointer to the record keeps C's depth as `*mut c_void` (`filedesc.cp` C 122,
  `new_child` C 145, `delete_child` C 146).  A Windows target must add the record *together with*
  the three `<windows.h>` records it embeds.
- **Three groups carry `#[cfg (feature = "windowsnt")]`, because the port's other mirrors already
  declare those names** (Rust holds one declaration per name in a crate; the
  `lib/include/errno.in.rs` 95-99 `feature = "windows"` shape, and the `feature = "msdos"` reading
  `include/msdos.rs` records from the other side):
  1. **the file-descriptor-set group** (C 30-57, C 115-116, C 127): this header is the Windows
     build's *replacement* for the system `<sys/select.h>`, which the port spells for this target
     as `lib/include/sys_select.rs` 31/39 (`FD_SETSIZE` 1024 and its own `fd_set`).  Masked:
     `FD_SETSIZE`, `fd_set`, `FD_SET`, `FD_CLR`, `FD_ISSET`, `FD_ZERO`, `SELECT_TYPE`, `MAXDESC`,
     `MAX_CHILDREN` (the two sizes' values *are* `FD_SETSIZE`) and `fd_info` (C 127's length *is*
     `MAXDESC`).  That masking is also what a Windows build needs: `src/process.rs` 7479's live
     `eassert! ((*p).infd < FD_SETSIZE)` reads the *system* mirror's 1024 today, and on that target
     it must read this header's 2048.
  2. **the POSIX-name prototypes** the port's system mirrors own: `realpath` (C 197),
     `srandom`/`random` (C 254-255), `fchmod`/`fchmodat`/`lchmod` (C 264-265), `pipe2` (C 266),
     `openat` (C 262) - `lib/include/sys_stat.in.rs` 184-185, `lib/include/fcntl.in.rs` 287 -
     plus `w32_strerror`/`w32_get_resource` (C 149/181), which `include/cygw32.rs` 35/43 declares
     for the Cygwin build (C 23-25's `#error` is exactly what keeps those two apart), and
     `init_environment`/`dostounix_filename`/`unixtodos_filename` (C 199/204/205), which
     `include/msdos.rs` 101-104 declares with the MS-DOS shapes.
  3. **C 269-277 (`HAVE_GNUTLS`)** is dead text on this target (the macro is `undef`), so it is
     carried under `#[cfg (feature = "have_gnutls")]` (PORTING.md P-51 names the non-frozen side); its
     `gnutls_transport_ptr_t` is not resolved unless the feature is on.
- **The macros keep C's shapes.**  `FD_SET`/`FD_CLR` (C 41-53) are `do { … } while (0)` statement
  macros; the port writes the `if` the loop encloses, so C's `do`/`while` words (token-table
  skips) disappear - but C's loop *condition* contributes a `0` that the Rust body then cannot
  mention, which is the one class this mirror cannot close by itself (see `## Evidence`: it needs
  two `docs/ports/macro-body-notes.tsv` rows, the `PDUMPER_RESET` row's class, PORTING.md 2.2).
  `FD_ISSET` (C 54) is an expression macro, so C's `? :` is an `if`/`else` expression and its `0`
  stays as the else arm; `FD_ZERO` (C 55) is `std::ptr::write_bytes ($p, 0, std::mem::size_of::<fd_set> ())`
  (`memset` is a token-aliases family, docs/ports/token-aliases.tsv, and `sizeof (fd_set)` keeps
  the type's name); `SELECT_TYPE` (C 57) is a *type* macro and so a nullary `macro_rules!` (the
  `haikugui.rs` 63 spelling); `CHILD_ACTIVE` (C 117) keeps C's member name and `NULL` (the port's
  `null_mut ()`, P-44's family table).
- **`fd_set.bits` is an array, not a pointer**: C 38's `unsigned int bits[FD_SETSIZE / 32]` is 64
  `c_uint`, and the mirror keeps C's `FD_SETSIZE / 32` expression as the array's length.  The
  record's layout is **not** asserted anywhere - the C side needs this target's `<windows.h>`,
  which no compiler here has (the `include/w32heap.rs` `file_data` rule); a Windows target that
  freezes this header must add it to `tools/gen-layout-fields.py` and measure it there.
- **Parameter names are C's**: the header's own where it names them (`w32_get_long_filename`'s
  `name`/`buf`/`size`, `prepare_standard_handles`'s `in`/`out`/`err` - `in` is a Rust keyword and
  becomes `in_`, the `include/cygw32.rs` `type_` rule) and the definitions' where it does not
  (`src/w32.c` 1552/1707/1766/1793/1820/1855/2409/2670/2721/2752/3290/3533/4255/4451/4537/4785/6604/6937/7719/7835/10388/10863/10947/11320-11343,
  `src/w32proc.c` 650/668/1488/1836/1936/2014/2213/2725/3839/3890/3904/4657/4764,
  `src/w32fns.c` 10912, `src/w32reg.c`, `src/w32term.c`) - as PORTING.md 0.2's "same names" rule
  wants.
- **`--check` asks for less than fidelity does**: C 145-277 is ~55 declarations and typedefs with no
  checklist row (the extractor's items are the 23 macros, the 3 records and the 4 objects), and
  they are carried because they are the header's live text.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32.h femacs/src/include/w32.rs` -> **FAIL**
  with exactly these lines and no others:
  ```
  missing type deferred: src/w32.h _child_process/child_process (the ledger has unchecked boxes)
  - macro FD_CLR does not mention 0 (C body: do { if ((n) < FD_SETSIZE) { (p)->bits[(n)/32] &= ~(1 << (n))
  - macro FD_SET does not mention 0 (C body: do { if ((n) < FD_SETSIZE) { (p)->bits[(n)/32] |= (1 << (n)%)
  ```
  The first line is on **stderr** and is the deferred record above, not a problem; the two `- `
  lines are the `do…while (0)` class, and every *other* item - the 23 macros (values and bodies,
  including `FD_ISSET`'s `0` and `FD_ZERO`'s `fd_set`/`0`), the two `FILE_*`/`STATUS_*` constant
  groups, `filedesc`'s three fields, `MAXDESC`/`MAX_CHILDREN`'s `FD_SETSIZE`/`MAXDESC` tokens and
  the four objects - is accepted.
- **The two rows this file needs in `docs/ports/macro-body-notes.tsv`** (the lane may not write
  them; reported for the integrator, with the text the tool expects -
  `<macro>\t<token>[,<token>…]\t<reason>`):
  `FD_SET	0	C 41-47's `do { … } while (0)` statement macro has no Rust counterpart: the port writes the if the loop encloses (PORTING.md 2.2), so the loop's own `0` is the one token without a match (the PDUMPER_RESET row)`
  `FD_CLR	0	C 48-53's `do { … } while (0)`, the FD_SET row's sibling`
- `python3 tools/check-cond-eval.py src/w32.h` -> `src/w32.h: 4 conditions, 0 unknown, 1
  disagreements [NOT VERIFIED]` with `DISAGREE src/w32.h:32 `ifdef FD_SETSIZE` — clang False,
  evaluator True`, and the tail line `4 conditions, 0 unknown to the evaluator, 1 disagreements`.
- `python3 tools/extract-abi.py emacs/src/w32.h` -> this ledger: 0 subrs, 0 functions defined,
  23 macros, 3 types (`<anon@37>` = `fd_set`, `<anon@120>` = `filedesc`, `_child_process`),
  4 file-scope objects (`fd_info` C 127, `pMultiByteToWideChar` C 191, `pWideCharToMultiByte`
  C 192, `multiByteToWideCharFlags` C 193).
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask
  -- layout-check` -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped
  (android-asset.rs, android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs,
  haiku_support.rs, haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)` and `layout-check passed
  (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it is not in that skip
  list), so the gate compiles it every round - with its `windowsnt`/`have_gnutls` arms masked.  The
  first attempt failed with two real collisions,
  `error[E0428]: the name FD_SETSIZE is defined multiple times … w32.rs:71:1` and
  `error[E0428]: the name fd_set is defined multiple times … w32.rs:82:1` (against
  `lib/include/sys_select.rs` 31/39), which is what the first masked group above records.
- `rg -n 'include/w32"' femacs` -> no unit `include!`s this mirror.  Its live consumers exist only
  as dead text in other lanes' arms: `src/process.rs` 7487-7490 is C 7299-7307's `#ifdef WINDOWSNT`
  block under `#[cfg (feature = "windowsnt")]`, and it uses `fd_info`/`FILE_SERIAL` (C 127/141)
  and `PurgeComm` with no declaration of its own - so a Windows-target pass will point that arm at
  this mirror (or at the platform layer that replaces it).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 576 .rs files / 592 map targets / 0 findings.

## Decisions (stale-box sweep: **not stale**, 2026-10-04)

The 1 box(es) this sweep tentatively ticked are **real pending work, not stale bookkeeping**,
and the tick was reverted.  Measured: while the ledger has unchecked boxes,
`python3 tools/extract-abi.py --check emacs/src/w32.h femacs/src/include/w32.rs` prints `0` `- ` lines **because the tool
defers this file's missing items to stderr** (`missing type deferred: src/w32.h <name> (the ledger has
unchecked boxes)`); with the boxes ticked the deferral lifts and the same command prints

    - missing type _child_process

i.e. the type is genuinely absent from femacs/src/include/w32.rs.  So: src/w32.h's `_child_process`/`child_process` is the Windows subprocess record; dead-platform text (no `_WIN32`), P-51/3.9 shape.

## Decisions / Evidence (the eight true gaps, 2026-10-04)

`_child_process`/`child_process` (C 74-113) landed as dead platform text under `windows` (no `_WIN32` on the frozen target, P-51/3.9); member names are C's, in C's order.  The other boxes this ledger lists were already satisfied and were ticked in the same
pass (they had been left unchecked, and the earlier sweep had un-ticked some of them by
position rather than by name - corrected here).

    $ python3 tools/extract-abi.py --check emacs/src/w32.h femacs/src/include/w32.rs
    OK emacs/src/w32.h -> femacs/src/include/w32.rs                     # `- ` lines: 0, no `missing type deferred`
    $ cargo run -q -p xtask -- layout-check
    layout-check passed (see build/femacs/layout_asserts.rs)

(**before** this pass: 1 unchecked box(es); **after**: 0)

# ----------------------------------------------------------------------
## Four names move into `w32_system_names` (2026-10-05, `docs/ports/w32term.md` owed 11)

**C 31-39 and C 264-265.**  `FD_SETSIZE` and `fd_set` are also declared by
`lib/include/sys_select.rs` 31/39 and `fchmod`/`fchmodat` by `lib/include/sys_stat.in.rs`
184-185, so a `windowsnt` crate that reaches both headers cannot compile - measured, four
`error[E0428]` naming exactly those pairs in `xtask probe src/w32term.rs …
--cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`.  The four now sit in
`pub mod w32_system_names` with `use w32_system_names::*;` (the same shape as the w32term
modules), which removes the collision because a crate-root definition shadows a glob import.

**Owed, and not disguised by that move.**  A *real* Windows build has to read C's own values here
(`FD_SETSIZE` 2048, `MAXDESC` 2048), and the arbitration that expresses C's `#undef`/`#define`
ordering is a *complementary* gate - `not (feature = "windowsnt")` - on the two `lib/include`
mirrors, which are outside this lane's writable set (`femacs/lib/include/**`).  Until it lands,
these names are reachable in a `windowsnt` crate only where nothing else declares them, and the
`MAXDESC`/`SELECT_TYPE`/`FD_*` items still at this file's top level resolve `FD_SETSIZE`/`fd_set`
through the glob.

**Evidence.**  `extract-abi --check emacs/src/w32.h femacs/src/include/w32.rs` -> `OK`;
`xtask layout-check` passed; the `w32term` flip above falls from 26 to 0 `E0428`;
`check-all-units.sh` 83/0 both legs.
