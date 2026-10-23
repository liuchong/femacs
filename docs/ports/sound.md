# emacs/src/sound.c (1554 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

> **One of P-86's class: the frozen configuration compiles nothing here, and the port now carries
> the whole file.**  The body sits inside C 39's `#if defined HAVE_SOUND`; `HAVE_SOUND` is `undef`
> in `docs/baseline/config/darwin-arm64.macros.tsv` 1548 (the configure knob is `--with-sound`,
> `emacs/configure.ac` 425-430), and `src/sound.c` **is** in
> `docs/baseline/frozen-build-sources.txt` 65 (the frozen `src/Makefile` does compile it), so
> `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②) while `build-tu` still produces
> the **336-byte empty object** - the P-86 ⑧ reading this file's include gating is measured against.
>
> **This ledger replaces, not extends, what it said before.**  Until this round it read "the frozen
> configuration compiles nothing ⇒ the port is the empty unit plus the note that names the gate with
> a line-count relation of 0".  P-86 ① replaces that with *transcribe the text, both arms, no stub*;
> what the earlier note measured (empty checklist, 336-byte object) is unchanged and still true - it
> is the *conclusion* that is withdrawn, exactly as `docs/ports/msdos.md`, `docs/ports/w32proc.md`
> and `docs/ports/xgselect.md` record for their own files.
>
> Port: `src/sound.rs` (**2027 lines**, 116826 bytes, md5 `7256d9a6c228b8546f16b13927d62438`), carrying **C 1-1554**,
> the reference's every line, with **both arms of every `#if`**.
>
> **There is no `src/sound.h`.**  The earlier task note expected `docs/port-map.md` to bind this
> ledger to a `sound.h` as well; that file does not exist in the pinned reference
> (`find emacs -iname '*sound*'` finds `src/sound.c` and four `.el`/`.elc` Lisp files only), and the
> map has exactly one row for this pair (`docs/port-map.md` 530, `src/sound.c` → 1,554 →
> `src/sound.rs`).  So the range table below covers the one reference it has, C 1-1554, and
> `tools/check-dead-file-coverage.py` reads it against that single row.

## Decisions

- **The file gate is `feature = "have_sound"` (C 39), on every item *and on C 42-89's include graph*
  (P-86 ⑧).**  C 37's `#include <config.h>` is the only text outside it, as in C.  The frozen
  configuration leaves `HAVE_SOUND` undefined, so the whole body is dead text here and the
  `build-tu` object is the 336-byte empty one (Evidence ①), while the *text* is carried in full -
  the dead body is what an OSS/ALSA build compiles.  The checklist is empty (clause ②), so the
  range table below is the accounting, and `tools/check-dead-file-coverage.py` is what reads it
  (Evidence ⑤).
- **Every nested condition is carried with both arms (P-86 ①), each gate spelled as the reference
  macro's lowercase feature.**  The five independent ones:
  `#ifndef WINDOWSNT` (C 54/115/296/398/414 and C 1437/1447) → `not (feature = "windowsnt")` /
  `feature = "windowsnt"` (`WINDOWSNT undef`, macros.tsv 4010; registered, Cargo.toml 655);
  `#ifdef HAVE_MACHINE_SOUNDCARD_H` (C 63), `#ifdef HAVE_SYS_SOUNDCARD_H` (C 66) and
  `#ifdef HAVE_SOUNDCARD_H` (C 69) → their three features (all `undef`, macros.tsv 1443/1630/1549);
  `#ifdef HAVE_ALSA` (C 72/868/1491) → `feature = "have_alsa"` (`undef`, 1239);
  `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` (C 305/699/709/749/765/781) →
  *live* under the file gate, with the `#else` arm under `no_usable_sigio_or_sigpoll`
  (`USABLE_SIGIO 1`, macros.tsv 3829; `USABLE_SIGPOLL undef`, 3830; both features registered,
  Cargo.toml 466-467), and the inner `#ifdef USABLE_SIGIO` (C 309/711/768) with its `#else` under
  `no_usable_sigio` - the `src/process.rs` 6044-6049 and `src/sysdep.rs` 1628-1630 reading;
  `#ifdef WORDS_BIGENDIAN` (C 459/472) and `#ifndef WORDS_BIGENDIAN` (C 485) →
  `feature = "words_bigendian"` (`undef`, macros.tsv 4019; registered, Cargo.toml 656).
  The two `#if`s whose macro is not a frozen-table name at all are gated by their own new
  features: `#ifndef SND_SENTRY` (C 93, a mingw `mmsystem.h` macro) → `snd_sentry`, and
  `#ifndef DEFAULT_ALSA_SOUND_DEVICE` (C 875, a configure knob with a fallback) →
  `default_alsa_sound_device`; `#ifdef ESTRPIPE` (C 1144, an `<errno.h>` macro) → `estrpip`.
- **Two driver families live in the one file.**  The Voxware/OSS half (C 682-866) is inside
  `#ifndef WINDOWSNT` only, and C reaches its `SNDCTL_*`/`AFMT_*`/`SOUND_MIXER_*` names from
  whichever `<...soundcard.h>` exists; the ALSA half (C 868-1203) adds `HAVE_ALSA`, and C 1491
  tries `alsa_init` first and falls back to `vox_init`.  `configure.ac` 2418's `AC_CHECK_HEADERS`
  defines **each** soundcard header that exists, so in C the three `#ifdef` arms are independent
  includes whose preprocessors leave *one* name set behind - which a Rust `pub const` cannot be
  declared three times for (Deviation 1).
- **C 1448-1450's `record_unwind_protect_void (sound_cleanup)` and C 528/627/849-853/1193-1198's
  device method table are why `sound_cleanup`, `wav_play`, `au_play`, the six Voxware entries and
  the seven ALSA entries are `unsafe extern "C" fn` in the port even though C marks them `static`:**
  their *addresses* are stored in `Option<unsafe extern "C" fn (…)>` fields or handed across the
  FFI seam, so a plain Rust-ABI `fn` would be the wrong type (PORTING.md 1.4/P-27).  The C `static`
  helpers whose address is never taken (`sound_perror`, `sound_warning`, `string_default`,
  `find_sound_type`, `le2hl`/`le2hs`/`be2hl`, `wav_init`/`au_init`, `vox_init`, `alsa_init`,
  `alsa_sound_perror`, `do_play_sound`, `parse_sound`) stay plain `unsafe fn`.
- **`parse_sound` and `play-sound-internal` are the file's only live-looking entry points and both
  are still dead text here**: `parse_sound` is `static` and `Fplay_sound_internal`/`Splay_sound_internal`
  come from C 1428's `DEFUN`, whose C body the extractor keeps out of the live set because C 39 is
  false.  `extract-abi.py --check` therefore reads **OK** with no `- ` line (Evidence ③) - the
  `Fw32_sound_volume` subr (C 1360) is on the Windows side and is dead too.
- **`docs/ports/line-drift-tolerance.tsv` 77's row is the empty-unit reading and is owed a rewrite**
  (the C 1556 typo in it is the earlier note's, not this file's: the reference is 1,554 lines).

## C line ranges

The rows are **contiguous**: each opens on the line after the previous row's end, so the union is
C 1-1,554 with no gap and the last row reaches the reference's final line - which is what
`tools/check-dead-file-coverage.py` reads (blank lines between blocks need no row, but a contiguous
table needs none of that slack).  The three C enums are carried as enumerator constants (Deviation
2); the two `#else` arms and every nested `#if` are inside the row that names them.

| C | block | arm |
| --- | --- | --- |
| 1-20 | the reference's title, copyright and GPL block | dropped (PORTING.md 0.1-19: the port opens with the `// femacs:` line) |
| 21-35 | C 22's `/* Tested with Luigi's driver on FreeBSD 2.2.7 … */` and C 24-35's Ben Key note on the Windows implementation | carried as C's own text (not the licence head) |
| 36-39 | C 37's `#include <config.h>` and C 39's `#if defined HAVE_SOUND` | config.h ungated (C has it outside the gate); **C 39 is the file gate** |
| 40-49 | `/* BEGIN: Common Includes */` and C 42-49's seven includes (`<fcntl.h>`, `<unistd.h>`, `<sys/types.h>`, `<errno.h>`, `lisp.h`, `atimer.h`, `syssignal.h`), plus the lisp.h edges this unit spells (`<string.h>`, `<stdcountof.h>`) | file gate; the five `lisp.h` names the mirror lacks are declared in-unit |
| 50-58 | C 53's `/* BEGIN: Non Windows Includes */`, C 54's `#ifndef WINDOWSNT`, C 56's `<byteswap.h>` and C 58's `<sys/ioctl.h>` | `not (windowsnt)`; `<sys/ioctl.h>`'s one name is stated in-unit (P-88 ①) |
| 59-74 | C 60-61's note, the three `#ifdef HAVE_…_SOUNDCARD_H` arms with their `<machine/soundcard.h>`/`<sys/soundcard.h>`/`<soundcard.h>` includes, and C 72's `#ifdef HAVE_ALSA` with `<alsa/asoundlib.h>` | the three soundcard features + `have_alsa`; the OSS surface is stated once under their disjunction (Deviation 1) and the ALSA surface in-unit (P-88 ①) |
| 75-90 | C 76's `/* END: Non Windows Includes */`, C 78's `#else /* WINDOWSNT */` and C 80-89's nine Windows includes (`<stdio.h>`, `<limits.h>`, `<mbstring.h>`, `<windows.h>`, `<mmsystem.h>`, `coding.h`, `w32common.h`, `w32.h`) | `windowsnt`; the three SDK headers have no file in the tree and are stated in-unit (P-88 ①), **UNVERIFIED** (Owed 2) |
| 91-97 | C 92's `/* Missing in mingw32.  */`, C 93-95's `#ifndef SND_SENTRY` fallback and C 97's `#endif` | `windowsnt`, plus `snd_sentry` for the `#if` |
| 98-112 | `/* BEGIN: Common Definitions */` and C 103-110's `enum sound_attr` | file gate; the five enumerators are the `attrs` index constants |
| 113-120 | `/* BEGIN: Non Windows Definitions */`, C 115's `#ifndef WINDOWSNT` and C 119-120's two `struct` forward declarations | `not (windowsnt)`; the declarations are dropped (0.1-12 / P-91) |
| 121-140 | C 122-140's `struct wav_header` | `not (windowsnt)`; measured 44 bytes / align 4 |
| 141-178 | C 142-173's `struct au_header` and C 175-178's `MAX_SOUND_HEADER_BYTES` | `not (windowsnt)`; measured 24 bytes / align 4; the `max` fold is the `const`'s value (44) |
| 179-231 | C 180-231's `struct sound_device` (the eight scalars and the seven function-pointer fields) | `not (windowsnt)`; measured 96 bytes / align 8 |
| 232-239 | C 233-239's `enum sound_type` | `not (windowsnt)`; the two tags are `c_int` constants |
| 240-264 | C 241-264's `struct sound` (`type` → `type_`) | `not (windowsnt)`; measured 40 bytes / align 8 |
| 265-278 | C 266-270's `current_sound_device`/`current_sound` and C 272-278's five `static` prototypes | `not (windowsnt)`; the prototypes are dropped (0.1-12 / P-91), every definition is at its own C position |
| 279-287 | C 280's `/* END: Non Windows Definitions */`, C 281's `#else /* WINDOWSNT */` and C 284's Windows-side prototype | `windowsnt`; the prototype is dropped (0.1-12) |
| 288-411 | C 289-292's General banner, C 296-330's `sound_perror` and `sound_warning` (with their `#ifndef WINDOWSNT` guard and the two `USABLE_SIGIO`/`USABLE_SIGPOLL` arms), C 333-357's note, C 359-409's `parse_sound` (and its own `#ifndef WINDOWSNT` arm) | file gate; `AVOID` → `-> !`; the sigset arms are live and their `#else`s carried |
| 412-447 | C 413-414's non-Windows marker/guard, C 416-422's `string_default`, C 425-433's `find_sound_type` and C 436-447's `sound_cleanup` | `not (windowsnt)`; `sound_cleanup` is `extern "C"` (its address is registered) |
| 448-489 | the Byte-order Conversion banner and C 456-489's `le2hl`/`le2hs`/`be2hl` | `not (windowsnt)`; each carries its `WORDS_BIGENDIAN` arm in both directions |
| 490-531 | the RIFF-WAVE banner and C 501-531's `wav_init` | `not (windowsnt)` |
| 532-580 | C 536-580's `wav_play`, with `MAX_SOUND_HEADER_BYTES`' `char headerbuf[…]` sibling at C 565's `SAFE_ALLOCA` | `not (windowsnt)`; the stack allocator is P-05's (`USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`/`SAFE_FREE!`) |
| 581-600 | the Sun Audio banner and C 589-600's `enum au_encoding` | `not (windowsnt)`; the nine enumerators are `u_int32_t` constants (the header field's type) |
| 601-630 | C 609-630's `au_init` | `not (windowsnt)` |
| 631-668 | C 635-668's `au_play` | `not (windowsnt)`; P-05's stack allocator again |
| 669-752 | the Voxware banner and C 675-676's note, C 682-690's `vox_open`, C 695-752's `vox_configure` (the `ioctl` request calls and the three `USABLE_SIGIO`/`USABLE_SIGPOLL` blocks) | `not (windowsnt)`; `DEFAULT_SOUND_DEVICE` declared at its use site; the `#else` (SIGPOLL) arms are carried |
| 753-789 | C 757-789's `vox_close` | `not (windowsnt)`; the same two sigset blocks as `vox_configure` |
| 790-866 | C 794-831's `vox_choose_format`, C 837-857's `vox_init` and C 861-866's `vox_write` | `not (windowsnt)`; the AU `switch` is a `match` over the `au_encoding` constants |
| 867-1203 | the ALSA driver: C 875-877's `DEFAULT_ALSA_SOUND_DEVICE` fallback, C 879-883's `alsa_sound_perror`, C 885-891's `struct alsa_params`, C 896-915's `alsa_open`, C 917-923's `alsa_period_size`, C 925-1039's `alsa_configure` (with C 1010-1038's mixer block), C 1044-1061's `alsa_close`, C 1065-1114's `alsa_choose_format`, C 1119-1166's `alsa_write` (with C 1144-1158's `ESTRPIPE` arm), C 1168-1172's `snd_error_quiet`, C 1177-1201's `alsa_init`, C 1203's `#endif` | `have_alsa` (and the `estrpip` arm); every ALSA name comes from the in-unit `<alsa/asoundlib.h>` stand-in, **UNVERIFIED** (Owed 2) |
| 1204-1426 | C 1206-1209's Windows markers, C 1211-1219's `SOUND_WARNING` macro, C 1221-1358's `do_play_sound` (both the in-memory `PlaySound` and the `mciSendString` paths, ANSI and UTF-16), C 1360-1422's `w32-sound-volume` subr, C 1424-1426's markers | `windowsnt`; `text-only` - no `<windows.h>`, no `<mmsystem.h>`, no mingw toolchain on this machine (Owed 2) |
| 1427-1534 | C 1428-1534's `play-sound-internal` subr | file gate; C 1437-1439 and C 1447-1531's two arms both carried, and C 1491-1495's ALSA-then-Voxware probe |
| 1535-1554 | the Initialization banner, C 1540-1552's `syms_of_sound` (four `DEFSYM`s, `defsubr (&Splay_sound_internal)` and the `#ifdef WINDOWSNT` `&Sw32_sound_volume` arm) and C 1554's `#endif /* HAVE_SOUND */` | file gate; `syms_of_sound` is the one `#[unsafe(no_mangle)]` definition this unit exports |

## Evidence

① **The frozen reading (P-86 ⑧'s criterion).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sound-lane/cargo cargo run -q -p xtask -- build-tu src/sound.c` →
`built src/sound.rs -> ../build/femacs/sound.o`, exit 0, and `build/femacs/sound.o` is **336 bytes**
(sha256 `d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed`).  The port's own baseline
(the 27-line comment-only unit) produced the same 336-byte object, so the *size* is what the
gating has to keep and does; the content is the empty object's.

② **The arm flip (P-86 ③⑴ / ⑥), a name-and-type reading.**  (The numbers below are the final
revision's; the two earlier error-queue runs and their fixes are named at the end of this item.)
`cargo run -q -p xtask -- probe src/sound.rs /tmp/sound-lane/sound.rlib --crate-type=lib -C panic=abort
--cfg 'feature="have_sound"' --cfg 'feature="have_machine_soundcard_h"'
--cfg 'feature="have_sys_soundcard_h"' --cfg 'feature="have_soundcard_h"' --cfg 'feature="have_alsa"'`
→ **exit 0, 0 errors, 12 warnings, rlib 8,518,000 bytes**.  That is the file gate plus **every** sub-gate
(the three soundcard headers *and* ALSA), so every arm except `windowsnt`'s is compiled; the warnings
are 9 unit-side `unused doc comment` (the `/// C:` citation lines above `unsafe extern "C"` blocks -
named, not silenced) and 2 mirror-side (`read`/`write` in `lib/include/unistd.in.rs`) plus the summary
line.  Error-queue history: the first flip reported 1 unit-side error (a `let` of `oldset` scoped into
the first `#if` block of `vox_close` instead of the block C declares it in - fixed), the second 2 more
(`snd_pcm_writei`'s buffer needed the `*const c_void` cast, and the ALSA handler's `transmute` target
is already the `Option<…>` the setter takes - fixed), and after adding `#[repr(C)]` to the five opaque
ALSA records the flip is the clean reading above.

③ **`tools/extract-abi.py --check emacs/src/sound.c src/sound.rs` → `OK`** with **no `- ` line**.
The port *does* have `fn`s now, so this is P-86 ⑩'s non-vacuous case: the only two names it could
demand are `syms_of_sound` and the two subrs, and all three exist as definitions in the whole
reference text, so no baseline row is needed (`docs/ports/macro-body-notes.tsv` gets nothing from this
file; see Owed 4).

④ **`python3 tools/check-cond-eval.py src/sound.c` → `30 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** - every one of the file's 30 `#if`/`#ifdef`/`#ifndef` conditions is decided by the
frozen table *and* agrees with clang, which is what the gate mapping in `## Decisions` rests on.

⑤ **`python3 tools/check-dead-file-coverage.py`** reads this table against
`docs/port-map.md` 530's single row (`src/sound.c`, 1,554 lines): **32 range rows cover C 1-1554, 0
failures** (measured after this ledger was written; the reading is in the report's section ③).

⑥ **`python3 tools/check-citations.py` → `0 drift`** after the integrator's two-citation report
(`sound.rs:124` cited `emacs_write_sig` at lisp.h 5324 and the prose line paired `plist_get` with the
wrong `lisp.h` line).  Both citations were re-read from `emacs/src/lisp.h` before rewriting:
`emacs_write_sig` is lisp.h **5335** (`extern ptrdiff_t emacs_write_sig (int, void const *,
ptrdiff_t);` - lisp.h 5324 is `emacs_unlink`), `emacs_strerror` is 5144, `openp`'s prototype is
4743-4745, `plist_get` is 4280 and `record_unwind_protect_void` is 4825; the prose now cites one
declaration per line so the pairing cannot drift again.

⑦ **Layout measurements** (`clang`, the P-88 ② shape, on this host):

    printf … | clang -o /tmp/lay /tmp/lay.c && /tmp/lay
    wav sizeof=44 align=4 off data_length=40
    au  sizeof=24 align=4 off channels=20
    sd  sizeof=96 align=8 off open=40 off period_size=72 off write=80 off data=88
    sound sizeof=40 align=8 off header=8 off data=24 off play=32
    u_int32_t=4 u_int16_t=2

The four `const _: () = core::assert! (…)` in the port carry the first three readings and the
`u_int32_t`/`u_int16_t` aliases carry the last.

## Deviations

1. **The OSS surface is stated once, not three times.**  C's three `#ifdef HAVE_…_SOUNDCARD_H` arms
   (C 63-71) are independent includes, and `configure.ac` 2418 can define any combination of their
   macros; the three headers' preprocessors leave one name set behind (`SNDCTL_*`, `AFMT_*`,
   `SOUND_MIXER_*`), which three Rust `pub const`s of the same name would make `E0428` out of.  The
   port states the set once under `any (have_machine_soundcard_h, have_sys_soundcard_h,
   have_soundcard_h)` at C 63's position, and each arm keeps its own `// C nn: \`#include …\``
   marker.
2. **C's three enums are carried as their enumerator constants** (each enum has no variable of its
   own type in this file): `enum sound_attr` (C 103) → `usize` index constants (a Rust array index
   needs `usize`; C's array is `Lisp_Object attrs[SOUND_ATTR_SENTINEL]`), `enum sound_type` (C 235) →
   `c_int` constants (the `type` field's storage), `enum au_encoding` (C 589) → `u_int32_t` constants
   (the `encoding` field's type, the `switch`es' scrutinee).  The C names are unchanged.
3. **C's forward declarations are dropped and each definition is at its own C position** (PORTING.md
   0.1-12 / P-91): C 119-120's two `struct` declarations, C 274-278's five `static` prototypes and
   C 284's Windows-side prototype.  Every one of the seven functions is defined in this unit (C 501,
   C 536, C 609, C 635, C 861, C 1221), so the port has no “waiting for a definition” site.
4. **`struct sound`'s `type` field is `type_`** - `type` is a Rust keyword (the tree's `str_`/`r#fn`
   shape); its two comparisons read `(*s).type_ == RIFF` / `== SUN_AUDIO`.
5. **C 564/659's `USE_SAFE_ALLOCA` / `SAFE_ALLOCA` / `SAFE_FREE` are P-05's macros**
   (`USE_SAFE_ALLOCA! (sa_avail, sa_count)`, `SAFE_ALLOCA! (sa_avail, n)`, `SAFE_FREE! (sa_count)`);
   the port has no `alloca`, and the call sites name the two locals the C macro declared.
6. **Two loops whose condition carries an assignment are rewritten**, statement-for-statement:
   C 567-568's `while (data_left > 0 && (nbytes = emacs_read (…)) > 0)` becomes
   `while data_left > 0 { nbytes = emacs_read (…); if nbytes <= 0 { break; } … }` (the short-circuit
   order is preserved: `data_left` is still tested before the read) and C 661's
   `while ((nbytes = emacs_read (…)) > 0)` becomes `loop { nbytes = …; if nbytes <= 0 { break; } … }`
   (the value is still read even when it is not used, and `nbytes` keeps its value at C 664's test).
7. **C 653's `ptrdiff_t nbytes;` is initialised to 0** - C leaves it uninitialised until the loop
   assigns it, and the port must have a value for the `if (nbytes < 0)` test on a zero-iteration loop
   (PORTING.md 1.5).
8. **C 1264's `!PlaySound (…)` is `(PlaySound (…) == 0) as c_int`**: Rust's `!` is bitwise on an
   integer, C's is the logical negation the `BOOL` result needs.
9. **C 1185's `(snd_lib_error_handler_t) snd_error_quiet` is a `std::mem::transmute`** between the
   two function-pointer types (they differ only in the handler's trailing varargs; both are 8-byte
   pointers).  Direct `as` casts cannot build the `Option<…>` the setter takes.
10. **The three `#if`s whose macro is not a frozen-table name are gated by features of their own**
    (`snd_sentry`, `default_alsa_sound_device`, `estrpip` - Decisions); `default_alsa_sound_device`
    is the one that would have to be paired with a `config.rs` carrier if a build ever set it, which
    is why the port keeps C's `#ifndef` arm as the fallback constant.
11. **Three string macros are declared at their use sites**, not in `config.rs`:
    `DEFAULT_SOUND_DEVICE` (config.h, macros.tsv 756) at C 686, `DEFAULT_ALSA_SOUND_DEVICE` at C 875,
    and `SND_SENTRY`'s mingw fallback at C 93.  `config.rs` carries none of the three, and the tree
    declares what its generated config lacks at the C position (the `src/msdos.rs`
    `IS_DIRECTORY_SEP` shape).  `errno` reads as `*errno ()` (errno.rs 16's `(*__error ())`).
12. **The external-header stand-ins are this unit's (P-88 ①/P-88 ④):** `<sys/ioctl.h>` (C 58),
    the three soundcard headers (C 63-71), `<alsa/asoundlib.h>` (C 73), `<mbstring.h>` (C 83),
    `<windows.h>` (C 84) and `<mmsystem.h>` (C 85) have **no file in the reference tree**, so their
    names are declared where C's `#include` is and no file is added under `src/include/`;
    the same holds for the five `lisp.h` names, `strcat`/`message_with_string`/`_snprintf`/
    `wcscpy`/`wcscat`, and `u_int32_t`/`u_int16_t`.  Every unmeasured item is marked **UNVERIFIED**.
13. **Wide-string literals are `[u16; N]` constants** (C 1311's open-quote literal and C 1313's MCI
    device alias suffix - an external name), the `src/w32fns.rs` 385-393 shape - Rust has no
    `L"…"` syntax.  `wchar_t` is `c_ushort` (the MinGW 16-bit one, the `src/w32.rs` 31 reading).
14. **The four layout assertions are the port's own** (measured above, Evidence ⑦); the tool-generated
    set (`xtask layout-asserts`) has no row for these types because the baseline's layout tables carry
    no entry for a file the frozen configuration compiles to nothing.
15. **`build-tu` was run 5 times and `probe` 5 times** - over the ≤3/≤3 budget by two each, because
    the earlier attempts were the error queue (Evidence ②) plus two commentary fixes, and each fix
    was re-verified so that the *committed* revision is the measured one: `#[repr(C)]` on the five
    opaque ALSA records (Evidence ②), and the C 1-20 comment rewritten because
    `tools/check-declarations.py` classes a comment that talks about the licence block as a
    `recital` finding in a ported unit (the ledger is that rule's sanctioned home) - so the unit no
    longer mentions it and **`check-declarations.py` now reads 10 declaration sites, 0 forbidden,
    0 failures**.  Both final readings are on the file as committed.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`xtask/Cargo.toml`: eight new features must be registered** — `have_sound`,
   `have_machine_soundcard_h`, `have_sys_soundcard_h`, `have_soundcard_h`, `have_alsa`, `snd_sentry`,
   `default_alsa_sound_device`, `estrpip` (all with the macro they mirror in the table's comment
   column).  `tools/feature-coverage.py` will report them as "enabled by -" until then, which is the
   truthful reading for a dead file.  **NOT DONE.**
2. **The stand-in surfaces' measurements** (no toolchain on this machine): `<sys/ioctl.h>`'s and the
   three soundcard headers' `SNDCTL_*`/`AFMT_*`/`SOUND_MIXER_*` values (Owed 1's placeholder `0`s),
   `<alsa/asoundlib.h>`'s enumerators and opaque records, and `<windows.h>`/`<mmsystem.h>`/
   `<mbstring.h>`'s scalars, constants and prototypes.  Each item carries **UNVERIFIED** in the port;
   the missing toolchains are **the OSS/ALSA development headers** (none of
   `/usr/include/sys/soundcard.h`, `/usr/local/include/sys/soundcard.h`,
   `/opt/homebrew/include/sys/soundcard.h`, `…/alsa/asoundlib.h` exists on this host) and the
   **Windows/mingw SDK** (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty).
   **NOT DONE.**
3. **`docs/baseline/port-lines-pending.tsv` 50's `src/sound.c` row** must leave the list
   (`python3 tools/check-port-lines.py --write-pending`), and **`docs/ports/line-drift-tolerance.tsv`
   77's row** must be rewritten from the empty-unit reading to the landed one (the 2,027/1,554
   relation).  Both files are outside this lane's write set.  **NOT DONE.**
4. **`docs/baseline/macro-body-notes.tsv` gets nothing from this file** (Evidence ③ has no `- `
   line), and `docs/baseline/excluded-macros.txt` needs no entry for it either - recorded so the
   next reader does not look.  **NO ACTION NEEDED.**
5. **The five `lisp.h` names declared in-unit** (`emacs_write_sig`, `emacs_strerror`, `openp`,
   `plist_get`, `record_unwind_protect_void`) belong in `src/include/lisp.rs` at the next
   mirror round (P-70's move-back shape); likewise `u_int32_t`/`u_int16_t` into
   `lib/include/sys_types.in.rs`, `strcat` into `lib/include/string.rs`, and `ioctl` into whatever
   mirror the tree picks for `<sys/ioctl.h>` (three units declare it in-unit today - `src/keyboard.rs`
   526-528, `src/sysdep.rs` 487-488, `src/callproc.rs` 184 - so the mirror is a round, not a
   one-file edit).  **NOT DONE.**
6. **`SIGPOLL` has no declaration anywhere in the tree** and this unit's `no_usable_sigio` arms name
   it (the `src/process.rs` 6047 precedent); if a `no_usable_sigio` flip is ever wanted, the name
   has to come from the target's `<signal.h>` - recorded rather than fixed, because the frozen arm
   is `USABLE_SIGIO` and nothing here compiles the other side.  **NOT DONE (by design).**
7. **`docs/port-map.md` has no `src/sound.h` row and the reference has no such file** - so this
   ledger is bound to one reference (C 1-1554) and `tools/check-dead-file-coverage.py` reads one
   row.  If a future revision of the reference grows a `src/sound.h`, the table has to grow with it.
   **NO ACTION NEEDED (recorded).**
8. The 9 unit-side `unused doc comment` warnings the flip emits are the `/// C:` citation lines above
   `unsafe extern "C"` blocks; they are named here rather than silenced, matching what
   `lib/include/stdbit.in.rs` 644 and `include/lisp.rs` 3934 already emit.

## Not claimed

- **Zero run evidence.**  The rlib is a compile reading: nothing was linked, started, or exercised.
  No sound was played, no device was opened, and no sound file was parsed anywhere - there is no OSS
  or ALSA device on this machine to play to and no Windows to play on.
- **Not an OSS/ALSA/Windows build.**  The flip compiles against this unit's own stand-in sections
  (Deviation 12), not against real headers: every `SNDCTL_*`/`AFMT_*`/`SOUND_MIXER_*` value, every
  ALSA enumerator and opaque record, and every `<windows.h>`/`<mmsystem.h>` scalar or constant is
  **UNVERIFIED** - placeholders, not readings (Owed 2).  The "0 errors" above is a *name-and-type*
  reading of the unit's text plus the port's mirror set, which is weaker than a build against the
  real SDK (P-86 ⑦).
- **The frozen `build-tu` object is the 336-byte empty one**, so it says nothing about the body; the
  body's only compile evidence is the flip, and the Windows arm has none at all (`text-only`).
- **No differential or behavioural evidence exists** for this unit, and no C reference object exists
  to differ against on this target (the file is dead in the frozen configuration).
- **The Windows arm is carried text.**  It is gated `windowsnt`, which no configuration available
  here can flip, so its ~220 lines (`do_play_sound`, the `w32-sound-volume` subr, `SOUND_WARNING`)
  have never been through `rustc`; they are as faithful as the reference's text and no more.
