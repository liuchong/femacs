// femacs: port of src/sound.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn`.
//
// **This file is one of P-86's class: the frozen configuration compiles none of it.**  The whole
// body sits inside C 39's `#if defined HAVE_SOUND`, `HAVE_SOUND` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` 1548 (the configure knob `--with-sound`, C 425-430
// of `emacs/configure.ac`, decides it), so the text can never be *compiled* here and
// `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②).  `src/sound.c` **is** in
// `docs/baseline/frozen-build-sources.txt` 65 (the frozen `src/Makefile` compiles it), which is why
// the frozen `build-tu` object is the 336-byte empty one rather than "no unit at all" - the P-86 ⑧
// reading this file's gating is measured against.
//
// **The file gate is `feature = "have_sound"`** (C 39's `HAVE_SOUND`), on every item *and on the
// include graph* (P-86 ⑧: an ungated `include!` would break the frozen build at once, which is why
// the reading in the ledger's Evidence ① is an empty object).  C 37's `#include <config.h>` is the
// only text outside it, as in C.  The nested conditions are carried with both arms (P-86 ①):
//
//   * C 54 `#ifndef WINDOWSNT` / C 78 `#else` - `not (feature = "windowsnt")` / `feature =
//     "windowsnt"` (`WINDOWSNT` is `undef`, macros.tsv 4010); the split runs through the whole file
//     (includes, the two type blocks, the two function halves, C 1437's and C 1549's arms).
//   * C 63/66/69 `#ifdef HAVE_MACHINE_SOUNDCARD_H` / `HAVE_SYS_SOUNDCARD_H` / `HAVE_SOUNDCARD_H` -
//     `feature = "have_machine_soundcard_h"` / `"have_sys_soundcard_h"` / `"have_soundcard_h"`
//     (all three `undef`, macros.tsv 1443/1630/1549).  `configure.ac` 2418's `AC_CHECK_HEADERS`
//     defines **each** header that exists, so in C the three arms are independent includes whose
//     *name sets are one surface* (the OSS `SNDCTL_*`/`AFMT_*`/`SOUND_MIXER_*` names); the port
//     states that surface once (Deviation 1).
//   * C 72 `#ifdef HAVE_ALSA` - `feature = "have_alsa"` (`undef`, macros.tsv 1239).
//   * C 93 `#ifndef SND_SENTRY` - `feature = "snd_sentry"` (the mingw `mmsystem.h` macro; see the
//     section at C 93).
//   * C 305/309/699/709/711/749/765/768/781 `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)`
//     and `#ifdef USABLE_SIGIO` - the frozen table has `USABLE_SIGIO 1` (macros.tsv 3829) and
//     `USABLE_SIGPOLL undef` (3830), so those arms are **live under the file gate**: the true arm is
//     spelled bare and the `#else` arm carries `no_usable_sigio_or_sigpoll` /
//     `no_usable_sigio` (both registered, `xtask/Cargo.toml` 466-467; the
//     `src/process.rs` 6044-6049 / `src/sysdep.rs` 1628-1630 reading).
//   * C 459/472 `#ifdef WORDS_BIGENDIAN` and C 485 `#ifndef WORDS_BIGENDIAN` - `feature =
//     "words_bigendian"` (registered, Cargo.toml 656; `undef`, macros.tsv 4019).
//   * C 875 `#ifndef DEFAULT_ALSA_SOUND_DEVICE` and C 1144 `#ifdef ESTRPIPE` - `feature =
//     "default_alsa_sound_device"` / `feature = "estrpip"`, two names nothing in the frozen table
//     defines (the first is a configure knob with a fallback, the second a `<errno.h>` macro).
//
// **Acquisition note - the non-Windows arm is `text-only` where its headers are, and the Windows
// arm is `text-only` in full (P-86 ③⑵).**  This machine has no OSS and no ALSA development headers
// (`/usr/include/sys/soundcard.h`, `/usr/local/include/sys/soundcard.h`,
// `/opt/homebrew/include/sys/soundcard.h`, `.../alsa/asoundlib.h` do not exist; macOS ships
// neither), and no Windows and no mingw toolchain - so **no measurement of any record, macro or
// value from `<machine/soundcard.h>`, `<sys/soundcard.h>`, `<soundcard.h>`, `<alsa/asoundlib.h>`,
// `<windows.h>`, `<mmsystem.h>` or `<mbstring.h>`** is possible.  Those headers' surfaces are
// declared in this unit at their C `#include` lines (P-88 ①) with each unmeasured item marked
// **UNVERIFIED**, and the ledger's Owed list names the toolchains and the measurements.  What the
// flip below *does* measure is the unit's own text against the port's mirror set.
//
// `docs/ports/sound.md` carries the C line ranges (C 1-1554, the reference's every line), the
// acceptance readings, every deviation and the items the integrator owes.  No behaviour of this
// file was executed anywhere: there is no OSS/ALSA device to play to and no Windows to play on.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, unused_comparisons, overflowing_literals,
         unused_must_use, clippy::all)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* C 1-20: the reference's opening block.  The port does not carry it; the ledger's C line ranges
   record what became of it (PORTING.md 0.1-19).  */

/* C 22: the file's own first note, C's words.  */
/* Tested with Luigi's driver on FreeBSD 2.2.7 with a SoundBlaster 16.  */

/* C 24-35: the Windows-port note, C's words.  */
/*
  Modified by Ben Key <Bkey1@tampabay.rr.com> to add a partial
  implementation of the play-sound specification for Windows.

  Notes:
  In the Windows implementation of play-sound-internal the :device
  keyword, if present, is ignored.

  The Windows implementation of play-sound is implemented via the
  Windows API functions mciSendString, waveOutGetVolume,
  waveOutSetVolume and PlaySound which are exported by Winmm.dll.
*/

// [U1] config.rs provided by common

/* C 39: `#if defined HAVE_SOUND` - the file gate.  C 40 through C 1553 sit inside it, and every
   `include!` below carries it (P-86 ⑧).  `/* END: Common Definitions */`-style banner comments are
   carried at their C lines.  */

/* C 41: `/* BEGIN: Common Includes */` */
#[cfg (feature = "have_sound")]
const _: () = ();
#[cfg (feature = "have_sound")]
const _: () = ();
#[cfg (feature = "have_sound")]
const _: () = ();
/* C 44's `<sys/types.h>` is where the two fixed-width spellings the WAV/AU headers use come from:
   `u_int32_t` and `u_int16_t` (the file's own field types, C 127-139 and C 148-172).  The mirror
   (`lib/include/sys_types.in.rs`) carries the POSIX aliases but not these two, so this unit declares
   them the way the tree declares what a mirror lacks (`src/sysdep.rs` 499-506), with the widths
   measured on this host (`printf '%zu %zu', sizeof (u_int32_t), sizeof (u_int16_t)` -> `4 2`).  */
/// C: <sys/types.h>: `typedef unsigned int u_int32_t;` (measured: 4 bytes on this target)
#[cfg (feature = "have_sound")]
pub type u_int32_t = u32;
/// C: <sys/types.h>: `typedef unsigned short u_int16_t;` (measured: 2 bytes on this target)
#[cfg (feature = "have_sound")]
pub type u_int16_t = u16;
#[cfg (feature = "have_sound")]
const _: () = ();
#[cfg (feature = "have_sound")]
const _: () = ();
#[cfg (feature = "have_sound")]
const _: () = ();
#[cfg (feature = "have_sound")]
const _: () = ();
/* C 50: `/* END: Common Includes */` */

/* lisp.h's own `#include <string.h>` edge, spelled here because `lisp.rs` carries neither it nor
   the mirror (the `src/alloc.rs` 59 / `src/editfns.rs` 62 reading): the common half calls
   `memcmp` (C 507/615), `memcpy` (C 1474) and `memset`, and the Windows half `strcpy`/`strcat`.  */
#[cfg (feature = "have_sound")]
const _: () = ();
/* lisp.h's `#include <stdcountof.h>` edge (C 1315's `countof`; the `src/xgselect.rs` 62 reading).  */
#[cfg (feature = "have_sound")]
const _: () = ();

/* ----------------------------------------------------------------------------------------------
   Five `lisp.h` names this unit's graph does not carry.

   `lisp.rs` declares the *storage* half of `lisp.h` (the `Q*`/`F*` names and the accessors) but not
   these five, and the reference reaches every one of them through C 47's `#include "lisp.h"`.
   Each is declared here with the tree's own spelling of the same declaration (`src/process.rs`
   7177-7178, `src/buffer.rs` 140-141, `src/callproc.rs` 97-102, `src/dired.rs` 133-134,
   `src/alloc.rs` 6265-6266) - the shape `docs/ports/w32proc.md` Owed 3 counts for the class.
   The five declaration sites, cited one per line below.  */
/// C: lisp.h 5335: `extern ptrdiff_t emacs_write_sig (int, void const *, ptrdiff_t);`
#[cfg (feature = "have_sound")]
unsafe extern "C-unwind" {
  pub fn emacs_write_sig (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
}
/// C: lisp.h 5144: `extern char *emacs_strerror (int) ATTRIBUTE_RETURNS_NONNULL;`
#[cfg (feature = "have_sound")]
unsafe extern "C" {

}
/// C: lisp.h 4743-4745 (one prototype, C's own wrapping):
/// C: `extern int openp (Lisp_Object, Lisp_Object, Lisp_Object,`
/// C: `                  Lisp_Object *, Lisp_Object, bool, bool,`
/// C: `		  void **);`
#[cfg (feature = "have_sound")]
unsafe extern "C" {

}
/// C: lisp.h 4280: `extern Lisp_Object plist_get (Lisp_Object, Lisp_Object);`
#[cfg (feature = "have_sound")]
unsafe extern "C" {
}
/// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
#[cfg (feature = "have_sound")]
unsafe extern "C-unwind" {
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C-unwind" fn ()>);
}

/* C 53: `/* BEGIN: Non Windows Includes */` */
/* C 54: `#ifndef WINDOWSNT` - the non-Windows arm's guard.  */

#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
const _: () = ();

// C 58: `#include <sys/ioctl.h>` - the header this section stands for.
/* ----------------------------------------------------------------------------------------------
   `<sys/ioctl.h>`: the one name the file takes from it.

   C 58's `#include <sys/ioctl.h>` has no file in the reference tree (P-88 ①: the declaration site
   is C's own `#include` line, and no file is added under `src/include/`).  The mirror set
   declares `ioctl` per unit (`src/keyboard.rs` 526-528, `src/sysdep.rs` 487-488, `src/callproc.rs`
   184), so this unit declares it here the same way rather than adding a `lib/include/` file.  */
/// C: <sys/ioctl.h>: `int ioctl (int, unsigned long, ...);`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C" {
  pub fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
}

/* C 60-61: C's own note on the soundcard headers.  */
/* FreeBSD has machine/soundcard.h.  Voxware sound driver docs mention
   sys/soundcard.h.  So, let's try whatever's there.  */

// C 63: `#ifdef HAVE_MACHINE_SOUNDCARD_H`
// C 64: `#include <machine/soundcard.h>`
// C 66: `#ifdef HAVE_SYS_SOUNDCARD_H`
// C 67: `#include <sys/soundcard.h>`
// C 69: `#ifdef HAVE_SOUNDCARD_H`
// C 70: `#include <soundcard.h>`
/* ----------------------------------------------------------------------------------------------
   The OSS soundcard surface (`<machine/soundcard.h>` / `<sys/soundcard.h>` / `<soundcard.h>`).

   The three headers have no file in the reference tree (P-88 ①), and `configure.ac` 2418's
   `AC_CHECK_HEADERS` may define any *combination* of their three macros: on the Voxware/OSS
   targets C includes each header that exists and the three preprocessors leave **one** set of
   names (`SNDCTL_*`/`AFMT_*`/`SOUND_MIXER_*`) behind.  A `pub const` cannot be defined three times
   without `E0428`, so the port states the surface once, under the disjunction of the three arms'
   gates, at the position of the first `#include` (Deviation 1); each arm above keeps its own
   `// C nn: \`#include ...\`` marker.

   **No measurement was possible**: this machine has no OSS header (see the acquisition note), so
   the six request/format/mixer values are **UNVERIFIED** - they are placeholders, not readings -
   and the ledger's Owed list names the missing headers.  The file's own driver half (C 682-866) is
   what names them.  ----------------------------------------------------------------------------  */
/// C: <sys/soundcard.h> 232: `#define SNDCTL_DSP_SETFMT _SIOWR ('P', 5, int)`
/// C: <sys/soundcard.h> 244: `#define SNDCTL_DSP_STEREO _SIOWR ('P', 3, int)`
/// C: <sys/soundcard.h> 246: `#define SNDCTL_DSP_SPEED _SIOWR ('P', 2, int)`
/// C: <sys/soundcard.h> 249: `#define SNDCTL_DSP_SYNC _SIOW ('P', 12, int)`
/// C: <sys/soundcard.h> 523: `#define SOUND_MIXER_WRITE_PCM MIXER_WRITE (SOUND_MIXER_PCM)`
/// C: <sys/soundcard.h>: `#define AFMT_U8 0x00000008`
/// C: <sys/soundcard.h>: `#define AFMT_MU_LAW 0x00000001`
/// C: <sys/soundcard.h>: `#define AFMT_S16_LE 0x00000010`
/// **UNVERIFIED** (no OSS header on this machine; the values are not readings - Owed 1).
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const SNDCTL_DSP_SETFMT: c_ulong = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const SNDCTL_DSP_STEREO: c_ulong = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const SNDCTL_DSP_SPEED: c_ulong = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const SNDCTL_DSP_SYNC: c_ulong = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const SOUND_MIXER_WRITE_PCM: c_ulong = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const AFMT_U8: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const AFMT_MU_LAW: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"),
            any (feature = "have_machine_soundcard_h", feature = "have_sys_soundcard_h",
                 feature = "have_soundcard_h")))]
pub const AFMT_S16_LE: c_int = 0;

/* C 72: `#ifdef HAVE_ALSA` */
// C 73: `#include <alsa/asoundlib.h>`

/* ----------------------------------------------------------------------------------------------
   The ALSA surface (`<alsa/asoundlib.h>`).

   C 73's `#include <alsa/asoundlib.h>` has no file in the reference tree (P-88 ①).  **No
   measurement was possible** (no ALSA header on this machine; see the acquisition note): the opaque
   records are spelled as the GTK mirror spells its own (`include/gtk.rs` 83's `_private: [u8; 0]`
   shape), the format/stream/access enumerators and every prototype below are **UNVERIFIED** and the
   ledger's Owed list names the header.  The C text that uses them is C 868-1203's driver.  -------- */
/// C: <alsa/asoundlib.h>: the five opaque records the driver holds pointers to.
/// **UNVERIFIED** (no ALSA header on this machine - Owed 1).
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
#[repr(C)]
pub struct snd_pcm_t { _private: [u8; 0] }
/// C: <alsa/asoundlib.h>: `typedef struct _snd_pcm_hw_params snd_pcm_hw_params_t;`
/// **UNVERIFIED**.
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
#[repr(C)]
pub struct snd_pcm_hw_params_t { _private: [u8; 0] }
/// C: <alsa/asoundlib.h>: `typedef struct _snd_pcm_sw_params snd_pcm_sw_params_t;`
/// **UNVERIFIED**.
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
#[repr(C)]
pub struct snd_pcm_sw_params_t { _private: [u8; 0] }
/// C: <alsa/asoundlib.h>: `typedef struct _snd_mixer snd_mixer_t;`  **UNVERIFIED**.
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
#[repr(C)]
pub struct snd_mixer_t { _private: [u8; 0] }
/// C: <alsa/asoundlib.h>: `typedef struct _snd_mixer_elem snd_mixer_elem_t;`  **UNVERIFIED**.
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
#[repr(C)]
pub struct snd_mixer_elem_t { _private: [u8; 0] }
/// C: <alsa/asoundlib.h>: `typedef unsigned long snd_pcm_uframes_t;`  **UNVERIFIED**.
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub type snd_pcm_uframes_t = c_ulong;
/// C: <alsa/asoundlib.h>: `typedef void (*snd_lib_error_handler_t) (const char *, int, const char *,
///                                                                  int, const char *, ...);`
/// **UNVERIFIED**.
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub type snd_lib_error_handler_t = Option<unsafe extern "C-unwind" fn (*const c_char, c_int, *const c_char,
                                                               c_int, *const c_char, ...)>;
/// C: <alsa/asoundlib.h>: `#define SND_PCM_STREAM_PLAYBACK 0`, `#define SND_PCM_ACCESS_RW_INTERLEAVED 3`,
/// `#define SND_MIXER_SCHN_LAST 0`.
/// **UNVERIFIED** (values are not readings - Owed 1).
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_STREAM_PLAYBACK: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_ACCESS_RW_INTERLEAVED: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_U8: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_S16_LE: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_MU_LAW: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_A_LAW: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_FLOAT_BE: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_FLOAT64_BE: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_S8: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_S16_BE: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_S24_BE: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_PCM_FORMAT_S32_BE: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
pub const SND_MIXER_SCHN_LAST: c_int = 0;
/// C: <alsa/asoundlib.h>: the entry points C 868-1203 calls.  **UNVERIFIED** (prototypes are not
/// readings; each return type is the calls' own use - Owed 1).
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C" {
  pub fn snd_strerror (errnum: c_int) -> *const c_char;
  pub fn snd_pcm_open (pcm: *mut *mut snd_pcm_t, name: *const c_char, stream: c_int,
                       mode: c_int) -> c_int;
  pub fn snd_pcm_close (pcm: *mut snd_pcm_t) -> c_int;
  pub fn snd_pcm_drain (pcm: *mut snd_pcm_t) -> c_int;
  pub fn snd_pcm_prepare (pcm: *mut snd_pcm_t) -> c_int;
  pub fn snd_pcm_resume (pcm: *mut snd_pcm_t) -> c_int;
  pub fn snd_pcm_format_size (format: c_int, samples: usize) -> isize;
  pub fn snd_pcm_writei (pcm: *mut snd_pcm_t, buffer: *const c_void, size: c_ulong) -> c_long;
  pub fn snd_pcm_hw_params_malloc (ptr: *mut *mut snd_pcm_hw_params_t) -> c_int;
  pub fn snd_pcm_hw_params_free (obj: *mut snd_pcm_hw_params_t);
  pub fn snd_pcm_hw_params_any (pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int;
  pub fn snd_pcm_hw_params_set_access (pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t,
                                       access: c_int) -> c_int;
  pub fn snd_pcm_hw_params_set_format (pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t,
                                       format: c_int) -> c_int;
  pub fn snd_pcm_hw_params_set_rate_near (pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t,
                                          val: *mut c_uint, dir: *mut c_int) -> c_int;
  pub fn snd_pcm_hw_params_set_channels (pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t,
                                         val: c_uint) -> c_int;
  pub fn snd_pcm_hw_params (pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int;
  pub fn snd_pcm_hw_params_get_period_size (params: *const snd_pcm_hw_params_t,
                                            frames: *mut snd_pcm_uframes_t,
                                            dir: *mut c_int) -> c_int;
  pub fn snd_pcm_hw_params_get_buffer_size (params: *const snd_pcm_hw_params_t,
                                            val: *mut snd_pcm_uframes_t) -> c_int;
  pub fn snd_pcm_sw_params_malloc (ptr: *mut *mut snd_pcm_sw_params_t) -> c_int;
  pub fn snd_pcm_sw_params_free (obj: *mut snd_pcm_sw_params_t);
  pub fn snd_pcm_sw_params_current (pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int;
  pub fn snd_pcm_sw_params_set_start_threshold (pcm: *mut snd_pcm_t,
                                                params: *mut snd_pcm_sw_params_t,
                                                val: snd_pcm_uframes_t) -> c_int;
  pub fn snd_pcm_sw_params_set_avail_min (pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t,
                                          val: snd_pcm_uframes_t) -> c_int;
  pub fn snd_pcm_sw_params (pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int;
  pub fn snd_mixer_open (mixer: *mut *mut snd_mixer_t, mode: c_int) -> c_int;
  pub fn snd_mixer_close (mixer: *mut snd_mixer_t) -> c_int;
  pub fn snd_mixer_attach (mixer: *mut snd_mixer_t, name: *const c_char) -> c_int;
  pub fn snd_mixer_load (mixer: *mut snd_mixer_t) -> c_int;
  pub fn snd_mixer_selem_register (mixer: *mut snd_mixer_t, options: *mut c_void,
                                   classp: *mut *mut c_void) -> c_int;
  pub fn snd_mixer_first_elem (mixer: *mut snd_mixer_t) -> *mut snd_mixer_elem_t;
  pub fn snd_mixer_elem_next (elem: *mut snd_mixer_elem_t) -> *mut snd_mixer_elem_t;
  pub fn snd_mixer_selem_has_playback_volume (elem: *mut snd_mixer_elem_t) -> c_int;
  pub fn snd_mixer_selem_get_playback_volume_range (elem: *mut snd_mixer_elem_t, min: *mut c_long,
                                                    max: *mut c_long) -> c_int;
  pub fn snd_mixer_selem_set_playback_volume (elem: *mut snd_mixer_elem_t, channel: c_int,
                                              value: c_long) -> c_int;
  pub fn snd_lib_error_set_handler (handler: snd_lib_error_handler_t) -> c_int;
}

/* C 76: `/* END: Non Windows Includes */`  */
/* C 78: `#else /* WINDOWSNT */` - the Windows arm's guard.  */

/* C 80: `/* BEGIN: Windows Specific Includes */` */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
const _: () = ();
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
const _: () = ();

// C 83: `#include <mbstring.h>` - the header this section stands for.
/* ----------------------------------------------------------------------------------------------
   `<mbstring.h>`: the one name the file takes from it (C 1280's `_mbspbrk`).

   No file in the reference tree (P-88 ①).  **UNVERIFIED**: this machine has no mingw header and no
   `<mbstring.h>` (see the acquisition note).  -------------------------------------------------- */
/// C: <mbstring.h>: `unsigned char *_mbspbrk (const unsigned char *, const unsigned char *);`
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
unsafe extern "C" {
  pub fn _mbspbrk (string: *const c_uchar, charset: *const c_uchar) -> *mut c_uchar;
}

// C 84: `#include <windows.h>` - the header this section stands for.
/* ----------------------------------------------------------------------------------------------
   `<windows.h>`: the SDK surface the Windows arm names.

   No file in the reference tree (P-88 ①; the declaration site is C's own `#include` line).  The
   spellings repeat `src/w32.rs` 343-410's section and `src/w32proc.rs` 243-330's, so a
   `windowsnt` translation unit sees one set of names.  **No measurement was possible** - this
   machine has no `<windows.h>` and no mingw toolchain (see the acquisition note), so every scalar
   is spelled from PORTING.md 1.1's fixed widths and every record is **UNVERIFIED** (Owed 2).  ---- */
/// C 84's `<windows.h>`: the scalar types and the handle typedef C 1239/1348/1390 cast to.
/// C: typedef unsigned long DWORD; typedef int BOOL; typedef unsigned int UINT;
/// C: typedef unsigned int MMRESULT; typedef unsigned long MCIERROR;
/// C: typedef void *HWAVEOUT; typedef const char *LPCSTR;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub type DWORD = u32;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub type BOOL = c_int;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub type UINT = c_uint;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub type MMRESULT = UINT;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub type MCIERROR = DWORD;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub type HWAVEOUT = *mut c_void;
/// C 84's `<windows.h>`: the two `BOOL` values C's text uses, and the two buffer sizes.
/// C: #define FALSE 0
/// C: #define TRUE 1
/// C: #define MAX_PATH 260
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const FALSE: BOOL = 0;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const TRUE: BOOL = 1;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const MAX_PATH: usize = 260;
/// C 84's `<windows.h>`: the audio entry points the Windows arm calls (`mciSendStringA`/`W`,
/// `mciGetErrorString`, `waveOutGetVolume`, `waveOutSetVolume`, `waveOutGetErrorText`, `PlaySound`).
/// C: MMRESULT waveOutGetVolume (HWAVEOUT, DWORD *); MMRESULT waveOutSetVolume (HWAVEOUT, DWORD);
/// C: MMRESULT waveOutGetErrorText (MMRESULT, char *, UINT);
/// C: MCIERROR mciSendStringA (LPCSTR, char *, UINT, void *);
/// C: MCIERROR mciSendStringW (const wchar_t *, wchar_t *, UINT, void *);
/// C: BOOL mciGetErrorString (DWORD, char *, UINT);
/// C: BOOL PlaySound (LPCSTR, void *, DWORD);
/// **UNVERIFIED** (no `<windows.h>`; the return types are the calls' own use - Owed 2).
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
unsafe extern "C" {
  pub fn waveOutGetVolume (hwo: HWAVEOUT, pdw_volume: *mut DWORD) -> MMRESULT;
  pub fn waveOutSetVolume (hwo: HWAVEOUT, dw_volume: DWORD) -> MMRESULT;
  pub fn waveOutGetErrorText (mmr_error: MMRESULT, psz_text: *mut c_char,
                              cch_text: UINT) -> MMRESULT;
  pub fn mciSendStringA (lpstr_command: *const c_char, lpstr_return: *mut c_char,
                         u_return_length: UINT, hwnd_callback: *mut c_void) -> MCIERROR;
  pub fn mciSendStringW (lpwstr_command: *const c_ushort, lpwstr_return: *mut c_ushort,
                         u_return_length: UINT, hwnd_callback: *mut c_void) -> MCIERROR;
  pub fn mciGetErrorString (fdw_error: DWORD, lpsz_buffer: *mut c_char, cch_buffer: UINT) -> BOOL;
  pub fn PlaySound (psz_sound: *const c_char, hmod: *mut c_void, fdw_sound: DWORD) -> BOOL;
}
/// C 84's `<windows.h>`: `WAVE_MAPPER` is the `(HWAVEOUT) -1` pseudo-handle C 1239/1348/1390 cast.
/// C: #define WAVE_MAPPER ((HWAVEOUT) -1)
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const WAVE_MAPPER: HWAVEOUT = -1isize as HWAVEOUT;

// C 85: `#include <mmsystem.h>` - the header this section stands for.
/* ----------------------------------------------------------------------------------------------
   `<mmsystem.h>`: the error/flag constants the Windows arm tests and passes.

   `MAXERRORLENGTH`, `MMSYSERR_NOERROR`, `MCIERR_FILE_NOT_FOUND`, `SND_MEMORY` and `SND_SENTRY`;
   no file in the reference tree (P-88 ①), **UNVERIFIED** (Owed 2).  ---------------------------- */
/// C: <mmsystem.h>: `#define MAXERRORLENGTH 128`
/// C: <mmsystem.h>: `#define MMSYSERR_NOERROR 0`
/// C: <mmsystem.h>: `#define MCIERR_FILE_NOT_FOUND 296`
/// C: <mmsystem.h>: `#define SND_MEMORY 0x00000004L`
/// C: <mmsystem.h>: `#define SND_SENTRY 0x00080000L` (the mingw `#ifndef` at C 93-95 is its
///    fallback, and this is the value it defines)
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const MAXERRORLENGTH: usize = 128;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const MMSYSERR_NOERROR: MMRESULT = 0;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const MCIERR_FILE_NOT_FOUND: MCIERROR = 296;
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const SND_MEMORY: DWORD = 0x00000004;

/* C 87: `#include "coding.h"` */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
const _: () = ();
/* C 88: `#include "w32common.h"` (`w32_major_version`) */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
const _: () = ();
/* C 89: `#include "w32.h"` (`w32_get_short_filename`, `filename_to_ansi`, `filename_to_utf16`) */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
const _: () = ();
/* C 90: `/* END: Windows Specific Includes */` */

/* C 92-95: `/* Missing in mingw32.  */` then `#ifndef SND_SENTRY` / `#define SND_SENTRY 0x00080000`.  */
// C 93: `#ifndef SND_SENTRY` - `SND_SENTRY` is `mmsystem.h`'s macro (C 85's section carries the
// value the field does hold); the `#if` has no configure input, so its gate is `snd_sentry`.
#[cfg (all (feature = "have_sound", feature = "windowsnt", not (feature = "snd_sentry")))]
pub const SND_SENTRY: DWORD = 0x00080000;

/* C 97: `#endif /* WINDOWSNT */`  */

/* C 99: `/* BEGIN: Common Definitions */`  */

/* C 101-110: `enum sound_attr { SOUND_FILE, SOUND_DATA, SOUND_DEVICE, SOUND_VOLUME,
   SOUND_ATTR_SENTINEL };` - the enumerators' only use in the file is as the indices of `attrs`
   (C 369-372, C 1434's array size), so they are carried as the index constants (a Rust index needs
   `usize`); the C names are kept.  */
#[cfg (feature = "have_sound")]
pub const SOUND_FILE: usize = 0;
#[cfg (feature = "have_sound")]
pub const SOUND_DATA: usize = 1;
#[cfg (feature = "have_sound")]
pub const SOUND_DEVICE: usize = 2;
#[cfg (feature = "have_sound")]
pub const SOUND_VOLUME: usize = 3;
#[cfg (feature = "have_sound")]
pub const SOUND_ATTR_SENTINEL: usize = 4;

/* C 112: `/* END: Common Definitions */`  */

/* C 114: `/* BEGIN: Non Windows Definitions */`  */
/* C 115: `#ifndef WINDOWSNT` - the non-Windows definitions' guard.  */

/* C 117-120: `/* Structure forward declarations.  */` and the two `struct sound;` /
   `struct sound_device;` forward declarations.  PORTING.md 0.1-12: C's forward declarations have no
   Rust counterpart (an item is not a declaration), so the two lines are dropped; both structs are
   *defined* below at their own C positions (C 182-231 and C 243-264), and Rust's item order is free
   (P-91 ② keeps the text in C order all the same, and it is).  */

/* C 122-140: `struct wav_header`, the RIFF-WAVE file header (always little-endian in the file).  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct wav_header
{
  pub magic: u_int32_t,                 // C 127
  pub length: u_int32_t,                // C 128
  pub chunk_type: u_int32_t,            // C 129
  pub chunk_format: u_int32_t,          // C 130
  pub chunk_length: u_int32_t,          // C 131
  pub format: u_int16_t,                // C 132
  pub channels: u_int16_t,              // C 133
  pub sample_rate: u_int32_t,           // C 134
  pub bytes_per_second: u_int32_t,      // C 135
  pub sample_size: u_int16_t,           // C 136
  pub precision: u_int16_t,             // C 137
  pub chunk_data: u_int32_t,            // C 138
  pub data_length: u_int32_t,           // C 139
}
/* Measured on this host (`clang` probe, the P-88 ② shape): `sizeof (struct wav_header) == 44`,
   `_Alignof == 4`; the ledger's Evidence carries the probe.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
const _: () = core::assert! (core::mem::size_of::<wav_header> () == 44);

/* C 142-173: `struct au_header`, the Sun audio file header (always big-endian in the file).  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct au_header
{
  /* C 147: `/* ASCII ".snd" */`  */
  pub magic_number: u_int32_t,          // C 148
  /* C 150: `/* Offset of data part from start of file. Minimum value is 24.  */`  */
  pub data_offset: u_int32_t,           // C 151
  /* C 153: `/* Size of data part, 0xffffffff if unknown.  */`  */
  pub data_size: u_int32_t,             // C 154
  /* C 156-165: C's note on the encodings (`1 8-bit ISDN u-law`, `2 8-bit linear PCM (REF-PCM)`,
     `3 16-bit linear PCM`, `4 24-bit linear PCM`, `5 32-bit linear PCM`, `6 32-bit IEEE
     floating-point`, `7 64-bit IEEE floating-point`, `23 8-bit u-law compressed using CCITT 0.721
     ADPCM voice data encoding scheme.`).  */
  pub encoding: u_int32_t,              // C 166
  /* C 168: `/* Number of samples per second.  */`  */
  pub sample_rate: u_int32_t,           // C 169
  /* C 171: `/* Number of interleaved channels.  */`  */
  pub channels: u_int32_t,              // C 172
}
/* Measured on this host: `sizeof (struct au_header) == 24`, `_Alignof == 4`.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
const _: () = core::assert! (core::mem::size_of::<au_header> () == 24);

/* C 175-178: `#define MAX_SOUND_HEADER_BYTES \
     max (sizeof (struct wav_header), sizeof (struct au_header))`.
   A `const` (the value is used as an array length at C 1451, so it must be one), with C's `max`
   fold; both sizes are measured above.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const MAX_SOUND_HEADER_BYTES: usize = 44;

/* C 180-231: `struct sound_device`, the device interface.  C's `void (*f) (T *)` is Rust's
   `Option<unsafe extern "C-unwind" fn (…)>` (PORTING.md 1.4); `period_size` is the one field C tests for
   NULL (C 562) and assigns NULL (C 854), which is what `Option` spells.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
#[repr(C)]
pub struct sound_device
{
  /* C 184-185: `/* If a string, the name of the device; otherwise use a default.  */`  */
  pub file: Lisp_Object,                // C 185
  /* C 187-188: `/* File descriptor of the device.  */`  */
  pub fd: c_int,                        // C 188
  /* C 190-191: `/* Device-dependent format.  */`  */
  pub format: c_int,                    // C 191
  /* C 193-194: `/* Volume (0..100).  Zero means unspecified.  */`  */
  pub volume: c_int,                    // C 194
  /* C 196-197: `/* Sample size.  */`  */
  pub sample_size: c_int,               // C 197
  /* C 199-200: `/* Sample rate.  */`  */
  pub sample_rate: c_int,               // C 200
  /* C 202-203: `/* Bytes per second.  */`  */
  pub bps: c_int,                       // C 203
  /* C 205-206: `/* 1 = mono, 2 = stereo, 0 = don't set.  */`  */
  pub channels: c_int,                  // C 206
  /* C 208-209: `/* Open device SD.  */`  */
  pub open: Option<unsafe extern "C-unwind" fn (*mut sound_device)>,           // C 209
  /* C 211-212: `/* Close device SD.  */`  */
  pub close: Option<unsafe extern "C-unwind" fn (*mut sound_device)>,          // C 212
  /* C 214-215: `/* Configure SD according to device-dependent parameters.  */`  */
  pub configure: Option<unsafe extern "C-unwind" fn (*mut sound_device)>,      // C 215
  /* C 217-219: `/* Choose a device-dependent format for outputting sound S.  */`  */
  pub choose_format: Option<unsafe extern "C-unwind" fn (*mut sound_device, *mut sound)>, // C 218-219
  /* C 221-223: `/* Return a preferred data size in bytes to be sent to write (below)
     each time.  2048 is used if this is NULL.  */`  */
  pub period_size: Option<unsafe extern "C-unwind" fn (*mut sound_device) -> ptrdiff_t>,  // C 223
  /* C 225-227: `/* Write NYBTES bytes from BUFFER to device SD.  */`  */
  pub write: Option<unsafe extern "C-unwind" fn (*mut sound_device, *const c_char, ptrdiff_t)>, // C 226-227
  /* C 229-230: `/* A place for devices to store additional data.  */`  */
  pub data: *mut c_void,                // C 230
}
/* Measured on this host: `sizeof (struct sound_device) == 96`, `_Alignof == 8`, `open` at 40,
   `period_size` at 72, `write` at 80, `data` at 88.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
const _: () = core::assert! (core::mem::size_of::<sound_device> () == 96);

/* C 233-239: `enum sound_type { RIFF, SUN_AUDIO };` - the tags of `struct sound`'s `type` field
   (C 246), compared at C 797/807 and C 1068/1078.  Carried as the enumerator constants of C's own
   storage type, so `s.type_ == RIFF` reads as C's `s->type == RIFF` (see the ledger's Deviations).  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const RIFF: c_int = 0;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const SUN_AUDIO: c_int = 1;

/* C 241-264: `struct sound`, the sound-file interface.  C's field `type` is a Rust keyword, so the
   port spells it `type_` (the tree's `str_`/`r#fn` shape).  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
#[repr(C)]
pub struct sound
{
  /* C 245-246: `/* The type of the file.  */`  */
  pub type_: c_int,                     // C 246
  /* C 248-249: `/* File descriptor of a sound file.  */`  */
  pub fd: c_int,                        // C 249
  /* C 251-253: `/* Pointer to sound file header.  This contains header_size bytes
     read from the start of a sound file.  */`  */
  pub header: *mut c_char,              // C 253
  /* C 255-257: `/* Number of bytes read from sound file.  This is always <=
     MAX_SOUND_HEADER_BYTES.  */`  */
  pub header_size: c_int,               // C 257
  /* C 259-260: `/* Sound data, if a string.  */`  */
  pub data: Lisp_Object,                // C 260
  /* C 262-263: `/* Play sound file S on device SD.  */`  */
  pub play: Option<unsafe extern "C-unwind" fn (*mut sound, *mut sound_device)>, // C 263
}
/* Measured on this host: `sizeof (struct sound) == 40`, `_Alignof == 8`, `header` at 8, `data` at
   24, `play` at 32.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
const _: () = core::assert! (core::mem::size_of::<sound> () == 40);

/* C 266-270: `/* These are set during `play-sound-internal' so that sound_cleanup has
   access to them.  */` and the two file-scope pointers.  Rust statics need an initializer
   (PORTING.md 1.5), so both start null - C leaves them uninitialized until C 1448-1449.  */
// C 269: `static struct sound_device *current_sound_device;`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
static mut current_sound_device: *mut sound_device = core::ptr::null_mut ();
// C 270: `static struct sound *current_sound;`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
static mut current_sound: *mut sound = core::ptr::null_mut ();

/* C 272-278: `/* Function prototypes.  */` and the five `static` prototypes (`vox_write`,
   `wav_init`, `wav_play`, `au_init`, `au_play`).  PORTING.md 0.1-12 / P-91: a Rust item is not a
   declaration, so the five lines are dropped; every one of the five is *defined* in this unit at its
   own C position (C 861, C 501, C 536, C 609, C 635), and P-91 ② keeps the definitions in C order.  */

/* C 280: `/* END: Non Windows Definitions */`  */
/* C 281: `#else /* WINDOWSNT */`  */

/* C 283-287: `/* BEGIN: Windows Specific Definitions */` and C 284's
   `static int do_play_sound (const char *, unsigned long, bool);` - the Windows arm's forward
   declaration.  Dropped with the five above (0.1-12); `do_play_sound` is defined at C 1221.  */

/* C 287: `#endif /* WINDOWSNT */`  */

/***********************************************************************
			       General
 ***********************************************************************/

/* C 294: `/* BEGIN: Common functions */`  */

/* C 296: `#ifndef WINDOWSNT` - the two Linux/GNU-only common helpers' guard.  */

/* C 297-321: `/* Like perror, but signals an error.  */` and `static AVOID sound_perror`.  C's
   `AVOID` is `-> !` in the port (PORTING.md 5.2).  */
// C 299-300: `static AVOID sound_perror (const char *msg)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn sound_perror (msg: *const c_char) -> !
{
  // C 302: `int saved_errno = errno;` - `errno` is `<errno.h>`'s `(*__error ())` (errno.rs 16).
  let saved_errno = *errno ();

  turn_on_atimers (true);                       // C 304
  /* C 305-316: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` - live under the file gate
     (`USABLE_SIGIO 1`); C 311's `#else` is the `no_usable_sigio` arm.  */
  #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
  {
    let mut unblocked: sigset_t = std::mem::zeroed ();     // C 307
    sigemptyset (&raw mut unblocked);                      // C 308
    /* C 309-313: `#ifdef USABLE_SIGIO` / `#else` `sigaddset (&unblocked, SIGPOLL);`  */
    #[cfg (not (feature = "no_usable_sigio"))]
    { sigaddset (&raw mut unblocked, SIGIO); }             // C 310
    #[cfg (feature = "no_usable_sigio")]
    { sigaddset (&raw mut unblocked, SIGPOLL); }           // C 312
    pthread_sigmask (SIG_UNBLOCK, &raw const unblocked, core::ptr::null_mut ());  // C 314
  }
  if saved_errno != 0
    { error (c"%s: %s".as_ptr (), msg, emacs_strerror (saved_errno)); }  // C 318
  else
    { error (c"%s".as_ptr (), msg); }                                    // C 320
}

/* C 323-329: `/* Display a warning message.  */` and `static void sound_warning`.  */
// C 325-326: `static void sound_warning (const char *msg)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn sound_warning (msg: *const c_char)
{
  message1 (msg);                               // C 328
}
/* C 330: `#endif	/* !WINDOWSNT */`  */

/* C 333-357: C's long note on the sound specification `parse_sound` accepts, C's words.  */
/* Parse sound specification SOUND, and fill ATTRS with what is
   found.  Value is non-zero if SOUND Is a valid sound specification.
   A valid sound specification is a list starting with the symbol
   `sound'.  The rest of the list is a property list which may
   contain the following key/value pairs:

   - `:file FILE'

   FILE is the sound file to play.  If it isn't an absolute name,
   it's searched under `data-directory'.

   - `:data DATA'

   DATA is a string containing sound data.  Either :file or :data
   may be present, but not both.

   - `:device DEVICE'

   DEVICE is the name of the device to play on, e.g. "/dev/dsp2".
   If not specified, a default device is used.

   - `:volume VOL'

   VOL must be an integer in the range [0, 100], or a float in the
   range [0, 1].  */

// C 359-360: `static bool parse_sound (Lisp_Object sound, Lisp_Object *attrs)`
#[cfg (feature = "have_sound")]
unsafe fn parse_sound (sound: Lisp_Object, attrs: *mut Lisp_Object) -> bool
{
  /* SOUND must be a list starting with the symbol `sound' followed by a
     number of keyword/value argument pairs.  */
  if !CONSP (sound) || !EQ (XCAR (sound), Qsound ())     // C 364
    { return false; }                                   // C 365
  CHECK_KEYWORD_ARGS (list_length (sound) - 1);         // C 366

  let sound = XCDR (sound);                             // C 368
  *attrs.add (SOUND_FILE) = plist_get (sound, QCfile ());      // C 369
  *attrs.add (SOUND_DATA) = plist_get (sound, QCdata ());      // C 370
  *attrs.add (SOUND_DEVICE) = plist_get (sound, QCdevice ());  // C 371
  *attrs.add (SOUND_VOLUME) = plist_get (sound, QCvolume ());  // C 372

  /* C 374-377: `/* File name or data must be specified.  */`  */
  if !STRINGP (*attrs.add (SOUND_FILE))
     && !STRINGP (*attrs.add (SOUND_DATA))
    { return false; }

  /* C 379-396: `/* Volume must be in the range 0..100 or unspecified.  */`  */
  if !NILP (*attrs.add (SOUND_VOLUME))
    {
      if FIXNUMP (*attrs.add (SOUND_VOLUME))
	{
	  let volume: EMACS_INT = XFIXNUM (*attrs.add (SOUND_VOLUME));    // C 384
	  if !(0 <= volume && volume <= 100)                              // C 385
	    { return false; }
	}
      else if FLOATP (*attrs.add (SOUND_VOLUME))
	{
	  let volume: c_double = XFLOAT_DATA (*attrs.add (SOUND_VOLUME));  // C 390
	  if !(0.0 <= volume && volume <= 1.0)                             // C 391
	    { return false; }
	}
      else
	{ return false; }                                                 // C 395
    }

  /* C 398-403: `#ifndef WINDOWSNT` - `/* Device must be a string or unspecified.  */`  */
  #[cfg (not (feature = "windowsnt"))]
  if !NILP (*attrs.add (SOUND_DEVICE))
     && !STRINGP (*attrs.add (SOUND_DEVICE))
    { return false; }
  /* C 404-407: C's comment on why the Windows arm does not check `:device`.  */
  /*
    Since device is ignored in Windows, it does not matter
    what it is.
   */
  return true;                                          // C 408
}

/* C 411: `/* END: Common functions */`  */

/* C 413: `/* BEGIN: Non Windows functions */`  */
/* C 414: `#ifndef WINDOWSNT` - the non-Windows functions' guard.  */

/* C 416-422: `/* Return S's value as a string if S is a string, otherwise DEFAULT_VALUE.  */`  */
// C 418-419: `static char const *string_default (Lisp_Object s, char const *default_value)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn string_default (s: Lisp_Object, default_value: *const c_char) -> *const c_char
{
  return if STRINGP (s) { SSDATA (s) as *const c_char } else { default_value };  // C 421
}

/* C 425-433: `/* Find out the type of the sound file whose file descriptor is FD.
   S is the sound file structure to fill in.  */`  */
// C 428-429: `static void find_sound_type (struct sound *s)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn find_sound_type (s: *mut sound)
{
  if !wav_init (s) && !au_init (s)                     // C 431
    { error (c"Unknown sound format".as_ptr ()); }     // C 432
}

/* C 436-447: `/* Function installed by play-sound-internal with record_unwind_protect_void.  */`  */
// C 438-439: `static void sound_cleanup (void)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn sound_cleanup ()
{
  if (*current_sound_device).close.is_some ()                              // C 441
    { ((*current_sound_device).close.unwrap ()) (current_sound_device); }  // C 442
  if (*current_sound).fd > 0                                              // C 443
    { emacs_close ((*current_sound).fd); }                                 // C 444
  xfree (current_sound_device as *mut c_void);                             // C 445
  xfree (current_sound as *mut c_void);                                    // C 446
}

/***********************************************************************
			Byte-order Conversion
 ***********************************************************************/

/* C 453-463: `/* Convert 32-bit value VALUE which is in little-endian byte-order
   to host byte-order.  */`  */
// C 456-457: `static u_int32_t le2hl (u_int32_t value)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn le2hl (mut value: u_int32_t) -> u_int32_t
{
  /* C 459-461: `#ifdef WORDS_BIGENDIAN` / `value = bswap_32 (value);` / `#endif`  */
  #[cfg (feature = "words_bigendian")]
  { value = bswap_32 (value); }
  return value;                                        // C 462
}

/* C 466-476: `/* Convert 16-bit value VALUE which is in little-endian byte-order
   to host byte-order.  */`  */
// C 469-470: `static u_int16_t le2hs (u_int16_t value)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn le2hs (mut value: u_int16_t) -> u_int16_t
{
  /* C 472-474: `#ifdef WORDS_BIGENDIAN` / `value = bswap_16 (value);` / `#endif`  */
  #[cfg (feature = "words_bigendian")]
  { value = bswap_16 (value); }
  return value;                                        // C 475
}

/* C 479-489: `/* Convert 32-bit value VALUE which is in big-endian byte-order
   to host byte-order.  */`  */
// C 482-483: `static u_int32_t be2hl (u_int32_t value)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn be2hl (mut value: u_int32_t) -> u_int32_t
{
  /* C 485-487: `#ifndef WORDS_BIGENDIAN` / `value = bswap_32 (value);` / `#endif`  */
  #[cfg (not (feature = "words_bigendian"))]
  { value = bswap_32 (value); }
  return value;                                        // C 488
}

/***********************************************************************
			  RIFF-WAVE (*.wav)
 ***********************************************************************/

/* C 495-499: `/* Try to initialize sound file S from S->header.  S->header
   contains the first MAX_SOUND_HEADER_BYTES number of bytes from the
   sound file.  If the file is a WAV-format file, set up interface
   functions in S and convert header fields to host byte-order.
   Value is true if the file is a WAV file.  */`  */
// C 501-502: `static bool wav_init (struct sound *s)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn wav_init (s: *mut sound) -> bool
{
  let header = (*s).header as *mut wav_header;         // C 504

  if (*s).header_size < core::mem::size_of::<wav_header> () as c_int      // C 506
     || memcmp ((*s).header as *const c_void, c"RIFF".as_ptr () as *const c_void, 4) != 0  // C 507
    { return false; }                                  // C 508

  /* C 510-511: `/* WAV files are in little-endian order.  Convert the header
     if on a big-endian machine.  */`  */
  (*header).magic = le2hl ((*header).magic);                              // C 512
  (*header).length = le2hl ((*header).length);                            // C 513
  (*header).chunk_type = le2hl ((*header).chunk_type);                    // C 514
  (*header).chunk_format = le2hl ((*header).chunk_format);                // C 515
  (*header).chunk_length = le2hl ((*header).chunk_length);                // C 516
  (*header).format = le2hs ((*header).format);                            // C 517
  (*header).channels = le2hs ((*header).channels);                        // C 518
  (*header).sample_rate = le2hl ((*header).sample_rate);                  // C 519
  (*header).bytes_per_second = le2hl ((*header).bytes_per_second);        // C 520
  (*header).sample_size = le2hs ((*header).sample_size);                  // C 521
  (*header).precision = le2hs ((*header).precision);                      // C 522
  (*header).chunk_data = le2hl ((*header).chunk_data);                    // C 523
  (*header).data_length = le2hl ((*header).data_length);                  // C 524

  /* C 526-527: `/* Set up the interface functions for WAV.  */`  */
  (*s).type_ = RIFF;                                                      // C 527
  (*s).play = Some (wav_play);                                            // C 528

  return true;                                                            // C 530
}

/* C 534-555: `/* Play RIFF-WAVE audio file S on sound device SD.  */` and C's note on the simple
   RIFF-WAVE scheme.  */
// C 536-537: `static void wav_play (struct sound *s, struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn wav_play (s: *mut sound, sd: *mut sound_device)
{
  let header = (*s).header as *mut wav_header;         // C 539

  /* C 541-543: `/* Let the device choose a suitable device-dependent format
     for the file.  */`  */
  ((*sd).choose_format.unwrap ()) (sd, s);             // C 543

  /* C 545-550: `/* Configure the device.  */`  */
  (*sd).sample_size = (*header).sample_size as c_int;      // C 546
  (*sd).sample_rate = (*header).sample_rate as c_int;      // C 547
  (*sd).bps = (*header).bytes_per_second as c_int;         // C 548
  (*sd).channels = (*header).channels as c_int;            // C 549
  ((*sd).configure.unwrap ()) (sd);                        // C 550

  if STRINGP ((*s).data)                                                   // C 556
    { ((*sd).write.unwrap ()) (sd,
        SSDATA ((*s).data).add (core::mem::size_of::<wav_header> ()),      // C 557
        SBYTES ((*s).data) - core::mem::size_of::<wav_header> () as ptrdiff_t); }  // C 558
  else
    {
      let mut nbytes: ptrdiff_t = 0;                                       // C 561
      let blksize: ptrdiff_t =                                             // C 562
        if (*sd).period_size.is_some () { ((*sd).period_size.unwrap ()) (sd) } else { 2048 };
      let mut data_left: ptrdiff_t = (*header).data_length as ptrdiff_t;   // C 563
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                               // C 564
      let buffer = SAFE_ALLOCA! (sa_avail, blksize as usize) as *mut c_char;  // C 565
      lseek ((*s).fd, core::mem::size_of::<wav_header> () as off_t, SEEK_SET);  // C 566
      while data_left > 0                                                  // C 567-568
        {
          nbytes = emacs_read ((*s).fd, buffer as *mut c_void, blksize);
          if nbytes <= 0 { break; }
          /* C 570: `/* Don't play possible garbage at the end of file */`  */
          if data_left < nbytes { nbytes = data_left; }                     // C 571
          data_left -= nbytes;                                             // C 572
          ((*sd).write.unwrap ()) (sd, buffer, nbytes);                     // C 573
        }

      if nbytes < 0                                                        // C 576
	{ sound_perror (c"Error reading sound file".as_ptr ()); }          // C 577
      SAFE_FREE! (sa_count, sa_mark);                                               // C 578
    }
}

/***********************************************************************
			   Sun Audio (*.au)
 ***********************************************************************/

/* C 587-600: `/* Sun audio file encodings.  */` and `enum au_encoding { ... }`.  The file uses the
   enumerators only as the labels of C 810's and C 1081's `switch`es over the `u_int32_t` header
   field, so they are carried as constants of that type (see the ledger's Deviations).  */
// C 591: `AU_ENCODING_ULAW_8 = 1,`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_ULAW_8: u_int32_t = 1;
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_8: u_int32_t = 2;                       // C 592
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_16: u_int32_t = 3;                      // C 593
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_24: u_int32_t = 4;                      // C 594
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_32: u_int32_t = 5;                      // C 595
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_IEEE32: u_int32_t = 6;                  // C 596
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_IEEE64: u_int32_t = 7;                  // C 597
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_COMPRESSED: u_int32_t = 23;                      // C 598
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const AU_ENCODING_ALAW_8: u_int32_t = 27;                 // C 599

/* C 603-607: `/* Try to initialize sound file S from S->header. …  */`  */
// C 609-610: `static bool au_init (struct sound *s)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn au_init (s: *mut sound) -> bool
{
  let header = (*s).header as *mut au_header;          // C 612

  if (*s).header_size < core::mem::size_of::<au_header> () as c_int       // C 614
     || memcmp ((*s).header as *const c_void, c".snd".as_ptr () as *const c_void, 4) != 0  // C 615
    { return false; }                                  // C 616

  (*header).magic_number = be2hl ((*header).magic_number);                // C 618
  (*header).data_offset = be2hl ((*header).data_offset);                  // C 619
  (*header).data_size = be2hl ((*header).data_size);                      // C 620
  (*header).encoding = be2hl ((*header).encoding);                        // C 621
  (*header).sample_rate = be2hl ((*header).sample_rate);                  // C 622
  (*header).channels = be2hl ((*header).channels);                        // C 623

  /* C 625-627: `/* Set up the interface functions for AU.  */`  */
  (*s).type_ = SUN_AUDIO;                                                 // C 626
  (*s).play = Some (au_play);                                             // C 627

  return true;                                                            // C 629
}

/* C 633-668: `/* Play Sun audio file S on sound device SD.  */`  */
// C 635-636: `static void au_play (struct sound *s, struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn au_play (s: *mut sound, sd: *mut sound_device)
{
  let header = (*s).header as *mut au_header;          // C 638

  (*sd).sample_size = 0;                               // C 640
  (*sd).sample_rate = (*header).sample_rate as c_int;  // C 641
  (*sd).bps = 0;                                       // C 642
  (*sd).channels = (*header).channels as c_int;        // C 643
  ((*sd).choose_format.unwrap ()) (sd, s);             // C 644
  ((*sd).configure.unwrap ()) (sd);                    // C 645

  if STRINGP ((*s).data)                                                   // C 647
    { ((*sd).write.unwrap ()) (sd,
        SSDATA ((*s).data).add ((*header).data_offset as usize),           // C 648
        SBYTES ((*s).data) - (*header).data_offset as ptrdiff_t); }        // C 649
  else
    {
      let blksize: ptrdiff_t =                                             // C 652
        if (*sd).period_size.is_some () { ((*sd).period_size.unwrap ()) (sd) } else { 2048 };
      let mut nbytes: ptrdiff_t = 0;                                       // C 653 (PORTING.md 1.5)
      /* C 655: `/* Seek */`  */
      lseek ((*s).fd, (*header).data_offset as off_t, SEEK_SET);           // C 656

      /* C 658: `/* Copy sound data to the device.  */`  */
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);                               // C 659
      let buffer = SAFE_ALLOCA! (sa_avail, blksize as usize) as *mut c_char;  // C 660
      loop                                                                 // C 661
        {
          nbytes = emacs_read ((*s).fd, buffer as *mut c_void, blksize);
          if nbytes <= 0 { break; }
	  ((*sd).write.unwrap ()) (sd, buffer, nbytes);                     // C 662
        }

      if nbytes < 0                                                        // C 664
	{ sound_perror (c"Error reading sound file".as_ptr ()); }          // C 665
      SAFE_FREE! (sa_count, sa_mark);                                               // C 666
    }
}

/***********************************************************************
		       Voxware Driver Interface
 ***********************************************************************/

/* C 675-676: `/* This driver is available on GNU/Linux, and the free BSDs.  FreeBSD
   has a compatible own driver aka Luigi's driver.  */`  */

/* C 679-680: `/* Open device SD.  If SD->file is a string, open that device,
   otherwise use a default device name.  */`  */
/// C: config.h: `#define DEFAULT_SOUND_DEVICE "/dev/dsp"`
/// (the frozen value, `docs/baseline/config/darwin-arm64.macros.tsv` 756; `config.rs` does not
/// carry it, so the unit declares it at its use site, the `src/msdos.rs` `IS_DIRECTORY_SEP` shape.)
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
pub const DEFAULT_SOUND_DEVICE: *const c_char = c"/dev/dsp".as_ptr ();
// C 682-683: `static void vox_open (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn vox_open (sd: *mut sound_device)
{
  /* C 685: `/* Open the sound device (eg /dev/dsp).  */`  */
  let file = string_default ((*sd).file, DEFAULT_SOUND_DEVICE);          // C 686
  (*sd).fd = emacs_open (file, O_WRONLY, 0);                             // C 687
  if (*sd).fd < 0                                                       // C 688
    { sound_perror (file); }                                            // C 689
}

/* C 693-694: `/* Configure device SD from parameters in it.  */`  */
// C 695-696: `static void vox_configure (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn vox_configure (sd: *mut sound_device)
{
  let mut val: c_int;                                  // C 698
  /* C 699-701: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` / `sigset_t oldset,
     blocked;` / `#endif` - live under the file gate.  */
  #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
  let mut oldset: sigset_t = std::mem::zeroed ();
  #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
  let mut blocked: sigset_t = std::mem::zeroed ();

  eassert! ((*sd).fd >= 0);                            // C 703

  /* C 705-708: `/* On GNU/Linux, it seems that the device driver doesn't like to be
     interrupted by a signal.  Block the ones we know to cause
     troubles.  */` and `turn_on_atimers (0);`  */
  turn_on_atimers (false);                             // C 708
  /* C 709-717: the `USABLE_SIGIO`/`USABLE_SIGPOLL` block (live).  */
  #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
  {
    sigemptyset (&raw mut blocked);                    // C 710
    #[cfg (not (feature = "no_usable_sigio"))]
    { sigaddset (&raw mut blocked, SIGIO); }           // C 712
    #[cfg (feature = "no_usable_sigio")]
    { sigaddset (&raw mut blocked, SIGPOLL); }         // C 714
    pthread_sigmask (SIG_BLOCK, &raw const blocked, &raw mut oldset);   // C 716
  }

  val = (*sd).format;                                  // C 719
  if ioctl ((*sd).fd, SNDCTL_DSP_SETFMT, &raw mut (*sd).format) < 0     // C 720
     || val != (*sd).format                                             // C 721
    { sound_perror (c"Could not set sound format".as_ptr ()); }         // C 722

  val = ((*sd).channels != 1) as c_int;                                 // C 724
  if ioctl ((*sd).fd, SNDCTL_DSP_STEREO, &raw mut val) < 0              // C 725
     || val != ((*sd).channels != 1) as c_int                           // C 726
    { sound_perror (c"Could not set stereo/mono".as_ptr ()); }          // C 727

  /* C 729-730: `/* I think bps and sampling_rate are the same, but who knows.
     Check this. and use SND_DSP_SPEED for both.  */`  */
  if (*sd).sample_rate > 0                                              // C 731
    {
      val = (*sd).sample_rate;                                          // C 733
      if ioctl ((*sd).fd, SNDCTL_DSP_SPEED, &raw mut (*sd).sample_rate) < 0  // C 734
	{ sound_perror (c"Could not set sound speed".as_ptr ()); }      // C 735
      else if val != (*sd).sample_rate                                  // C 736
	{ sound_warning (c"Could not set sample rate".as_ptr ()); }     // C 737
    }

  if (*sd).volume > 0                                                   // C 740
    {
      let mut volume: c_int = (*sd).volume & 0xff;                      // C 742
      volume |= volume << 8;                                            // C 743
      /* C 744: `/* This may fail if there is no mixer.  Ignore the failure.  */`  */
      ioctl ((*sd).fd, SOUND_MIXER_WRITE_PCM, &raw mut volume);         // C 745
    }

  turn_on_atimers (true);                                               // C 748
  /* C 749-751: the same `USABLE_SIGIO`/`USABLE_SIGPOLL` block.  */
  #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
  { pthread_sigmask (SIG_SETMASK, &raw const oldset, core::ptr::null_mut ()); }  // C 750
}

/* C 755-756: `/* Close device SD if it is open.  */`  */
// C 757-758: `static void vox_close (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn vox_close (sd: *mut sound_device)
{
  if (*sd).fd >= 0                                     // C 760
    {
      /* C 762-764: `/* On GNU/Linux, it seems that the device driver doesn't like to
	 be interrupted by a signal.  Block the ones we know to cause
	 troubles.  */`  */
      /* C 765-766: `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` /
	 `sigset_t blocked, oldset;` - the two declarations are at this block's scope in C,
	 which is why the two `#if` blocks below share them.  */
      #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
      let mut blocked: sigset_t = std::mem::zeroed ();                 // C 766
      #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
      let mut oldset: sigset_t = std::mem::zeroed ();                  // C 766
      /* C 767-773: the rest of the first `#if` block (live).  */
      #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
      {
        sigemptyset (&raw mut blocked);                               // C 767
        #[cfg (not (feature = "no_usable_sigio"))]
        { sigaddset (&raw mut blocked, SIGIO); }                      // C 769
        #[cfg (feature = "no_usable_sigio")]
        { sigaddset (&raw mut blocked, SIGPOLL); }                    // C 771
        pthread_sigmask (SIG_BLOCK, &raw const blocked, &raw mut oldset);  // C 773
      }
      turn_on_atimers (false);                                        // C 775

      /* C 777: `/* Flush sound data, and reset the device.  */`  */
      ioctl ((*sd).fd, SNDCTL_DSP_SYNC, core::ptr::null_mut::<c_void> ());  // C 778

      turn_on_atimers (true);                                         // C 780
      /* C 781-783: the same `USABLE_SIGIO`/`USABLE_SIGPOLL` block.  */
      #[cfg (not (feature = "no_usable_sigio_or_sigpoll"))]
      { pthread_sigmask (SIG_SETMASK, &raw const oldset, core::ptr::null_mut ()); }  // C 782

      /* C 785: `/* Close the device.  */`  */
      emacs_close ((*sd).fd);                                         // C 786
      (*sd).fd = -1;                                                  // C 787
    }
}

/* C 792: `/* Choose device-dependent format for device SD from sound file S.  */`  */
// C 794-795: `static void vox_choose_format (struct sound_device *sd, struct sound *s)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn vox_choose_format (sd: *mut sound_device, s: *mut sound)
{
  if (*s).type_ == RIFF                                // C 797
    {
      let h = (*s).header as *mut wav_header;          // C 799
      if (*h).precision == 8                           // C 800
	{ (*sd).format = AFMT_U8; }                    // C 801
      else if (*h).precision == 16                     // C 802
	{ (*sd).format = AFMT_S16_LE; }                // C 803
      else
	{ error (c"Unsupported WAV file format".as_ptr ()); }  // C 805
    }
  else if (*s).type_ == SUN_AUDIO                      // C 807
    {
      let header = (*s).header as *mut au_header;      // C 809
      match (*header).encoding                         // C 810
	{
	  AU_ENCODING_ULAW_8 | AU_ENCODING_IEEE32 | AU_ENCODING_IEEE64 =>   // C 812-814
	    { (*sd).format = AFMT_MU_LAW; }              // C 815
	  AU_ENCODING_8 | AU_ENCODING_16 | AU_ENCODING_24 | AU_ENCODING_32 =>  // C 818-821
	    { (*sd).format = AFMT_S16_LE; }              // C 822
	  _ =>                                           // C 825
	    { error (c"Unsupported AU file format".as_ptr ()); }  // C 826
	}
    }
  else
    { emacs_abort (); }                                // C 830
}

/* C 834-835: `/* Initialize device SD.  Set up the interface functions in the device
   structure.  */`  */
// C 837-838: `static bool vox_init (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe fn vox_init (sd: *mut sound_device) -> bool
{
  /* C 840: `/* Open the sound device (eg /dev/dsp).  */`  */
  let file = string_default ((*sd).file, DEFAULT_SOUND_DEVICE);          // C 841
  let fd = emacs_open (file, O_WRONLY, 0);                               // C 842
  if fd >= 0                                                             // C 843
    { emacs_close (fd); }                                                // C 844
  else
    { return false; }                                                    // C 846

  (*sd).fd = -1;                                                         // C 848
  (*sd).open = Some (vox_open);                                          // C 849
  (*sd).close = Some (vox_close);                                        // C 850
  (*sd).configure = Some (vox_configure);                                // C 851
  (*sd).choose_format = Some (vox_choose_format);                        // C 852
  (*sd).write = Some (vox_write);                                        // C 853
  (*sd).period_size = None;                                              // C 854

  return true;                                                           // C 856
}

/* C 859: `/* Write NBYTES bytes from BUFFER to device SD.  */`  */
// C 861-862: `static void vox_write (struct sound_device *sd, const char *buffer, ptrdiff_t nbytes)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt")))]
unsafe extern "C-unwind" fn vox_write (sd: *mut sound_device, buffer: *const c_char, nbytes: ptrdiff_t)
{
  if emacs_write_sig ((*sd).fd, buffer as *const c_void, nbytes) != nbytes   // C 864
    { sound_perror (c"Error writing to sound device".as_ptr ()); }           // C 865
}

/* C 868: `#ifdef HAVE_ALSA`  */
/***********************************************************************
		       ALSA Driver Interface
 ***********************************************************************/

/* C 873: `/* This driver is available on GNU/Linux. */`  */

/* C 875-877: `#ifndef DEFAULT_ALSA_SOUND_DEVICE` / `#define DEFAULT_ALSA_SOUND_DEVICE "default"` /
   `#endif`.  The macro is a configure knob with a fallback, not a frozen-table name, so its `#if`
   is `default_alsa_sound_device` (see the file note).  */
/// C: `#define DEFAULT_ALSA_SOUND_DEVICE "default"`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa",
            not (feature = "default_alsa_sound_device")))]
pub const DEFAULT_ALSA_SOUND_DEVICE: *const c_char = c"default".as_ptr ();

// C 879-880: `static AVOID alsa_sound_perror (const char *msg, int err)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe fn alsa_sound_perror (msg: *const c_char, err: c_int) -> !
{
  error (c"%s: %s".as_ptr (), msg, snd_strerror (err));                 // C 882
}

/* C 885-891: `struct alsa_params`.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
#[repr(C)]
pub struct alsa_params
{
  pub handle: *mut snd_pcm_t,                    // C 887
  pub hwparams: *mut snd_pcm_hw_params_t,        // C 888
  pub swparams: *mut snd_pcm_sw_params_t,        // C 889
  pub period_size: snd_pcm_uframes_t,            // C 890
}

/* C 893-894: `/* Open device SD.  If SD->file is a string, open that device,
   otherwise use a default device name.  */`  */
// C 896-897: `static void alsa_open (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn alsa_open (sd: *mut sound_device)
{
  /* C 899: `/* Open the sound device.  Default is "default".  */`  */
  let p = xmalloc (core::mem::size_of::<alsa_params> ()) as *mut alsa_params;   // C 900
  let file = string_default ((*sd).file, DEFAULT_ALSA_SOUND_DEVICE);            // C 901
  let err: c_int;                                                               // C 902

  (*p).handle = core::ptr::null_mut ();                                         // C 904
  (*p).hwparams = core::ptr::null_mut ();                                       // C 905
  (*p).swparams = core::ptr::null_mut ();                                       // C 906

  (*sd).fd = -1;                                                                // C 908
  (*sd).data = p as *mut c_void;                                                // C 909

  err = snd_pcm_open (&raw mut (*p).handle, file, SND_PCM_STREAM_PLAYBACK, 0);  // C 912
  if err < 0                                                                    // C 913
    { alsa_sound_perror (file, err); }                                          // C 914
}

// C 917-918: `static ptrdiff_t alsa_period_size (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn alsa_period_size (sd: *mut sound_device) -> ptrdiff_t
{
  let p = (*sd).data as *mut alsa_params;                                       // C 920
  let fact: c_int = (snd_pcm_format_size ((*sd).format, 1)
                     * (*sd).channels as isize) as c_int;                       // C 921
  return ((*p).period_size * (if fact > 0 { fact } else { 1 }) as c_ulong) as ptrdiff_t;  // C 922
}

// C 925-926: `static void alsa_configure (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn alsa_configure (sd: *mut sound_device)
{
  let mut val: c_int;                                                           // C 928
  let mut err: c_int;
  let mut dir: c_int = 0;                                                       // C 928 (PORTING.md 1.5)
  let mut uval: c_uint;                                                         // C 929
  let p = (*sd).data as *mut alsa_params;                                       // C 930
  let mut buffer_size: snd_pcm_uframes_t = 0;                                   // C 931

  eassert! ((*p).handle != core::ptr::null_mut ());                             // C 933

  err = snd_pcm_hw_params_malloc (&raw mut (*p).hwparams);                      // C 935
  if err < 0                                                                    // C 936
    { alsa_sound_perror (c"Could not allocate hardware parameter structure".as_ptr (), err); }  // C 937

  err = snd_pcm_sw_params_malloc (&raw mut (*p).swparams);                      // C 939
  if err < 0                                                                    // C 940
    { alsa_sound_perror (c"Could not allocate software parameter structure".as_ptr (), err); }  // C 941

  err = snd_pcm_hw_params_any ((*p).handle, (*p).hwparams);                     // C 943
  if err < 0                                                                    // C 944
    { alsa_sound_perror (c"Could not initialize hardware parameter structure".as_ptr (), err); }  // C 945

  err = snd_pcm_hw_params_set_access ((*p).handle, (*p).hwparams,               // C 947
                                      SND_PCM_ACCESS_RW_INTERLEAVED);           // C 948
  if err < 0                                                                    // C 949
    { alsa_sound_perror (c"Could not set access type".as_ptr (), err); }        // C 950

  val = (*sd).format;                                                           // C 952
  err = snd_pcm_hw_params_set_format ((*p).handle, (*p).hwparams, val);         // C 953
  if err < 0                                                                    // C 954
    { alsa_sound_perror (c"Could not set sound format".as_ptr (), err); }       // C 955

  uval = (*sd).sample_rate as c_uint;                                           // C 957
  err = snd_pcm_hw_params_set_rate_near ((*p).handle, (*p).hwparams,            // C 958
                                         &raw mut uval, core::ptr::null_mut ());
  if err < 0                                                                    // C 959
    { alsa_sound_perror (c"Could not set sample rate".as_ptr (), err); }        // C 960

  val = (*sd).channels;                                                         // C 962
  err = snd_pcm_hw_params_set_channels ((*p).handle, (*p).hwparams, val as c_uint);  // C 963
  if err < 0                                                                    // C 964
    { alsa_sound_perror (c"Could not set channel count".as_ptr (), err); }      // C 965

  err = snd_pcm_hw_params ((*p).handle, (*p).hwparams);                         // C 967
  if err < 0                                                                    // C 968
    { alsa_sound_perror (c"Could not set parameters".as_ptr (), err); }         // C 969

  err = snd_pcm_hw_params_get_period_size ((*p).hwparams, &raw mut (*p).period_size,
                                           &raw mut dir);                       // C 972
  if err < 0                                                                    // C 973
    { alsa_sound_perror (c"Unable to get period size for playback".as_ptr (), err); }  // C 974

  err = snd_pcm_hw_params_get_buffer_size ((*p).hwparams, &raw mut buffer_size);       // C 976
  if err < 0                                                                    // C 977
    { alsa_sound_perror (c"Unable to get buffer size for playback".as_ptr (), err); }  // C 978

  err = snd_pcm_sw_params_current ((*p).handle, (*p).swparams);                 // C 980
  if err < 0                                                                    // C 981
    { alsa_sound_perror (c"Unable to determine current swparams for playback".as_ptr (), err); }  // C 982-983

  /* C 985: `/* Start the transfer when the buffer is almost full */`  */
  err = snd_pcm_sw_params_set_start_threshold ((*p).handle, (*p).swparams,      // C 986-988
                                               (buffer_size / (*p).period_size) * (*p).period_size);
  if err < 0                                                                    // C 989
    { alsa_sound_perror (c"Unable to set start threshold mode for playback".as_ptr (), err); }  // C 990

  /* C 992: `/* Allow the transfer when at least period_size samples can be processed */`  */
  err = snd_pcm_sw_params_set_avail_min ((*p).handle, (*p).swparams, (*p).period_size);  // C 993
  if err < 0                                                                    // C 994
    { alsa_sound_perror (c"Unable to set avail min for playback".as_ptr (), err); }  // C 995

  err = snd_pcm_sw_params ((*p).handle, (*p).swparams);                         // C 997
  if err < 0                                                                    // C 998
    { alsa_sound_perror (c"Unable to set sw params for playback\n".as_ptr (), err); }  // C 999

  snd_pcm_hw_params_free ((*p).hwparams);                                       // C 1001
  (*p).hwparams = core::ptr::null_mut ();                                       // C 1002
  snd_pcm_sw_params_free ((*p).swparams);                                       // C 1003
  (*p).swparams = core::ptr::null_mut ();                                       // C 1004

  err = snd_pcm_prepare ((*p).handle);                                          // C 1006
  if err < 0                                                                    // C 1007
    { alsa_sound_perror (c"Could not prepare audio interface for use".as_ptr (), err); }  // C 1008

  if (*sd).volume > 0                                                           // C 1010
    {
      let mut chn: c_int;                                                       // C 1012
      let mut handle: *mut snd_mixer_t = core::ptr::null_mut ();                // C 1013
      let mut e: *mut snd_mixer_elem_t;                                         // C 1014
      if snd_mixer_open (&raw mut handle, 0) >= 0                               // C 1015
        {
	  let file = string_default ((*sd).file, DEFAULT_ALSA_SOUND_DEVICE);    // C 1017-1018
          if snd_mixer_attach (handle, file) >= 0                               // C 1019
              && snd_mixer_load (handle) >= 0                                   // C 1020
              && snd_mixer_selem_register (handle, core::ptr::null_mut (),
                                           core::ptr::null_mut ()) >= 0         // C 1021
            {
              e = snd_mixer_first_elem (handle);                                // C 1022
              while !e.is_null ()                                               // C 1023
                {
                  if snd_mixer_selem_has_playback_volume (e) != 0               // C 1026
                    {
                      let mut pmin: c_long = 0;                                 // C 1028
                      let mut pmax: c_long = 0;
                      let vol: c_long;                                          // C 1028
                      snd_mixer_selem_get_playback_volume_range (e, &raw mut pmin,
                                                                 &raw mut pmax);  // C 1029
                      vol = pmin + ((*sd).volume as c_long * (pmax - pmin)) / 100;  // C 1030

                      chn = 0;                                              // C 1032
                      while chn <= SND_MIXER_SCHN_LAST
                        {
                          snd_mixer_selem_set_playback_volume (e, chn, vol);  // C 1033
                          chn += 1;
                        }
                    }
                  e = snd_mixer_elem_next (e);                                  // C 1024
                }
            }
          snd_mixer_close (handle);                                             // C 1036
        }
    }
}

/* C 1042-1043: `/* Close device SD if it is open.  */`  */
// C 1044-1045: `static void alsa_close (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn alsa_close (sd: *mut sound_device)
{
  let p = (*sd).data as *mut alsa_params;                                       // C 1047
  if !p.is_null ()                                                              // C 1048
    {
      if !(*p).hwparams.is_null ()                                              // C 1050
        { snd_pcm_hw_params_free ((*p).hwparams); }                             // C 1051
      if !(*p).swparams.is_null ()                                              // C 1052
        { snd_pcm_sw_params_free ((*p).swparams); }                             // C 1053
      if !(*p).handle.is_null ()                                                // C 1054
        {
          snd_pcm_drain ((*p).handle);                                          // C 1056
          snd_pcm_close ((*p).handle);                                          // C 1057
        }
      xfree (p as *mut c_void);                                                 // C 1059
    }
}

/* C 1063: `/* Choose device-dependent format for device SD from sound file S.  */`  */
// C 1065-1066: `static void alsa_choose_format (struct sound_device *sd, struct sound *s)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn alsa_choose_format (sd: *mut sound_device, s: *mut sound)
{
  if (*s).type_ == RIFF                                // C 1068
    {
      let h = (*s).header as *mut wav_header;          // C 1070
      if (*h).precision == 8                           // C 1071
	{ (*sd).format = SND_PCM_FORMAT_U8; }          // C 1072
      else if (*h).precision == 16                     // C 1073
        { (*sd).format = SND_PCM_FORMAT_S16_LE; }      // C 1074
      else
	{ error (c"Unsupported WAV file format".as_ptr ()); }  // C 1076
    }
  else if (*s).type_ == SUN_AUDIO                      // C 1078
    {
      let header = (*s).header as *mut au_header;      // C 1080
      match (*header).encoding                         // C 1081
	{
	  AU_ENCODING_ULAW_8 => { (*sd).format = SND_PCM_FORMAT_MU_LAW; }     // C 1083-1084
	  AU_ENCODING_ALAW_8 => { (*sd).format = SND_PCM_FORMAT_A_LAW; }      // C 1086-1087
	  AU_ENCODING_IEEE32 => { (*sd).format = SND_PCM_FORMAT_FLOAT_BE; }   // C 1089-1090
	  AU_ENCODING_IEEE64 => { (*sd).format = SND_PCM_FORMAT_FLOAT64_BE; } // C 1092-1093
	  AU_ENCODING_8 => { (*sd).format = SND_PCM_FORMAT_S8; }              // C 1095-1096
	  AU_ENCODING_16 => { (*sd).format = SND_PCM_FORMAT_S16_BE; }         // C 1098-1099
	  AU_ENCODING_24 => { (*sd).format = SND_PCM_FORMAT_S24_BE; }         // C 1101-1102
	  AU_ENCODING_32 => { (*sd).format = SND_PCM_FORMAT_S32_BE; }         // C 1104-1105
	  _ => { error (c"Unsupported AU file format".as_ptr ()); }           // C 1108-1109
	}
    }
  else
    { emacs_abort (); }                                // C 1113
}

/* C 1117: `/* Write NBYTES bytes from BUFFER to device SD.  */`  */
// C 1119-1120: `static void alsa_write (struct sound_device *sd, const char *buffer, ptrdiff_t nbytes)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn alsa_write (sd: *mut sound_device, buffer: *const c_char, nbytes: ptrdiff_t)
{
  let p = (*sd).data as *mut alsa_params;                                      // C 1122

  /* C 1124: `/* The third parameter to snd_pcm_writei is frames, not bytes. */`  */
  let fact: c_int = (snd_pcm_format_size ((*sd).format, 1)
                     * (*sd).channels as isize) as c_int;                      // C 1125
  let mut nwritten: ptrdiff_t = 0;                                             // C 1126
  let mut err: c_int;                                                          // C 1127

  while nwritten < nbytes                                                      // C 1129
    {
      let frames: snd_pcm_uframes_t = ((nbytes - nwritten) / fact as isize) as snd_pcm_uframes_t; // C 1131
      if frames == 0 { break; }                                                // C 1132

      err = snd_pcm_writei ((*p).handle, buffer.offset (nwritten) as *const c_void, frames) as c_int; // C 1134
      if err < 0                                                               // C 1135
        {
          if err == -EPIPE                                                     // C 1137
            {	/* under-run */                                            // C 1138
              err = snd_pcm_prepare ((*p).handle);                             // C 1139
              if err < 0                                                       // C 1140
                { alsa_sound_perror (c"Can't recover from underrun, prepare failed".as_ptr (),
                                     err); }                                   // C 1141-1142
            }
          else
            {
              /* C 1144-1158: `#ifdef ESTRPIPE` … `else if (err == -ESTRPIPE) { … }` …
                 `#endif`, then C 1159-1160's `else alsa_sound_perror (…)` - shared by both
                 spellings of the chain.  */
              #[cfg (feature = "estrpip")]
              if err == -ESTRPIPE                                              // C 1145
                {
                  loop                                                         // C 1147
                    {
                      err = snd_pcm_resume ((*p).handle);
                      if err != -EAGAIN { break; }
                      sleep (1);	/* wait until the suspend flag is released */  // C 1148
                    }
                  if err < 0                                                   // C 1149
                    {
                      err = snd_pcm_prepare ((*p).handle);                     // C 1151
                      if err < 0                                               // C 1152
                        { alsa_sound_perror (c"Can't recover from suspend, prepare failed".as_ptr (),
                                             err); }                           // C 1153-1155
                    }
                }
              else
                { alsa_sound_perror (c"Error writing to sound device".as_ptr (), err); }  // C 1160
              #[cfg (not (feature = "estrpip"))]
              { alsa_sound_perror (c"Error writing to sound device".as_ptr (), err); }    // C 1160
            }
        }
      else
        { nwritten += (err * fact) as ptrdiff_t; }                             // C 1164
    }
}

/* C 1168-1172: `static void snd_error_quiet (const char *file, int line, const char *function,
   int err, const char *fmt) { }` - C's silent ALSA error handler.  */
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe extern "C-unwind" fn snd_error_quiet (_file: *const c_char, _line: c_int, _function: *const c_char,
                           _err: c_int, _fmt: *const c_char)
{
}

/* C 1174-1175: `/* Initialize device SD.  Set up the interface functions in the device
   structure.  */`  */
// C 1177-1178: `static bool alsa_init (struct sound_device *sd)`
#[cfg (all (feature = "have_sound", not (feature = "windowsnt"), feature = "have_alsa"))]
unsafe fn alsa_init (sd: *mut sound_device) -> bool
{
  /* C 1180: `/* Open the sound device.  Default is "default".  */`  */
  let file = string_default ((*sd).file, DEFAULT_ALSA_SOUND_DEVICE);           // C 1181
  let mut handle: *mut snd_pcm_t = core::ptr::null_mut ();                     // C 1182 (PORTING.md 1.5)
  let err: c_int;                                                              // C 1183

  /* C 1185's `(snd_lib_error_handler_t) snd_error_quiet` is the one C cast in the driver; the two
     function-pointer types differ only in the handler's trailing varargs, so the port transmutes
     (both are 8-byte pointers).  */
  snd_lib_error_set_handler (std::mem::transmute::<unsafe extern "C-unwind" fn (*const c_char, c_int,
      *const c_char, c_int, *const c_char), snd_lib_error_handler_t> (snd_error_quiet));
  err = snd_pcm_open (&raw mut handle, file, SND_PCM_STREAM_PLAYBACK, 0);      // C 1186
  snd_lib_error_set_handler (None);                                            // C 1187
  if err < 0                                                                   // C 1188
      { return false; }                                                        // C 1189
  snd_pcm_close (handle);                                                      // C 1190

  (*sd).fd = -1;                                                               // C 1192
  (*sd).open = Some (alsa_open);                                               // C 1193
  (*sd).close = Some (alsa_close);                                             // C 1194
  (*sd).configure = Some (alsa_configure);                                     // C 1195
  (*sd).choose_format = Some (alsa_choose_format);                             // C 1196
  (*sd).write = Some (alsa_write);                                             // C 1197
  (*sd).period_size = Some (alsa_period_size);                                 // C 1198

  return true;                                                                 // C 1200
}

/* C 1203: `#endif /* HAVE_ALSA */`  */

/* C 1206: `/* END: Non Windows functions */`  */
/* C 1207: `#else /* WINDOWSNT */` - the Windows functions' guard.  */

/* C 1209: `/* BEGIN: Windows specific functions */`  */

/* The three CRT/lisp.h names this arm's text uses that this unit's graph does not carry:
   `message_with_string` (lisp.h 5227; declared per unit, the `src/keymap.rs` 97 reading),
   `strcat` (`<string.h>`; `string.rs` carries `strcpy` but not this one) and the two wide-string
   CRT entries `wcscpy`/`wcscat`, plus the mingw `_snprintf`.  Declared here (P-88 ①), **UNVERIFIED**
   for the CRT pair (no mingw toolchain - Owed 2).  */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
unsafe extern "C-unwind" {
  /// C: lisp.h 5227: `extern void message_with_string (const char *, Lisp_Object, bool);`
  pub fn message_with_string (m: *const c_char, string: Lisp_Object, log: bool);
}
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
unsafe extern "C" {
  /// C: <string.h>: `char *strcat (char *, const char *);`
  pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  /// C: <wchar.h>: `wchar_t *wcscpy (wchar_t *, const wchar_t *);` (MinGW's 16-bit `wchar_t`)
  pub fn wcscpy (dest: *mut c_ushort, src: *const c_ushort) -> *mut c_ushort;
  /// C: <wchar.h>: `wchar_t *wcscat (wchar_t *, const wchar_t *);`
  pub fn wcscat (dest: *mut c_ushort, src: *const c_ushort) -> *mut c_ushort;
  /// C: <stdio.h>: `int _snprintf (char *, size_t, const char *, ...);` (the MSVCRT spelling)
  pub fn _snprintf (buf: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

/* C 1211-1219: `#define SOUND_WARNING(func, error, text) do { … } while (0)`.  The three
   arguments keep C's names; `func` is the error-text entry the call site names.  */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
macro_rules! SOUND_WARNING {
  ($func:expr, $error:expr, $text:expr) => {
    {
      let mut buf: [c_char; 1024] = [0; 1024];                        // C 1213
      let mut err_string: [c_char; MAXERRORLENGTH] = [0; MAXERRORLENGTH];  // C 1214
      ($func) ($error, err_string.as_mut_ptr (), err_string.len () as UINT);  // C 1215
      _snprintf (buf.as_mut_ptr (), buf.len (), c"%s\nMCI Error: %s".as_ptr (),  // C 1216-1217
                 $text, err_string.as_ptr ());
      message_with_string (c"%s".as_ptr (), build_string (buf.as_ptr ()), true);  // C 1218
    }
  };
}

/* C 1221-1358: `static int do_play_sound (const char *psz_file_or_data, unsigned long ui_volume,
   bool in_memory)`.  */
// C 1221-1222: the definition
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
unsafe fn do_play_sound (psz_file_or_data: *const c_char, ui_volume: c_ulong,
                         in_memory: bool) -> c_int
{
  let mut i_result: c_int = 0;                                          // C 1224
  let mut mci_error: MCIERROR = 0;                                      // C 1225
  let mut sz_cmd_buf_a: [c_char; 520] = [0; 520];                       // C 1226
  let mut sz_ret_buf_a: [c_char; 520] = [0; 520];                       // C 1227
  let mut mm_result: MMRESULT = MMSYSERR_NOERROR;                       // C 1228
  let mut ui_volume_org: c_ulong = 0;                                   // C 1229
  let mut b_reset_volume: BOOL = FALSE;                                 // C 1230
  let mut warn_text: [c_char; 560] = [0; 560];                          // C 1231

  if ui_volume > 0                                                      // C 1233
    {
      mm_result = waveOutGetVolume (WAVE_MAPPER, &raw mut ui_volume_org);       // C 1235
      if mm_result == MMSYSERR_NOERROR                                          // C 1236
        {
          b_reset_volume = TRUE;                                                // C 1238
          mm_result = waveOutSetVolume (WAVE_MAPPER, ui_volume);                // C 1239
          if mm_result != MMSYSERR_NOERROR                                      // C 1240
            {
	      SOUND_WARNING (waveOutGetErrorText, mm_result,                    // C 1242-1246
			     c"waveOutSetVolume: failed to set the volume level of the WAVE_MAPPER device.\nAs a result, the user selected volume level will not be used.".as_ptr ());
            }
        }
      else
        {
          SOUND_WARNING (waveOutGetErrorText, mm_result,                        // C 1251-1255
                         c"waveOutGetVolume: failed to obtain the original volume level of the WAVE_MAPPER device.\nAs a result, the user selected volume level will not be used.".as_ptr ());
        }
    }

  if in_memory                                                          // C 1259
    {
      let mut flags: c_int = SND_MEMORY as c_int;                       // C 1261
      if w32_major_version >= 6                                         // C 1262 (Vista and later)
	{ flags |= SND_SENTRY as c_int; }                               // C 1263
      i_result = (PlaySound (psz_file_or_data, core::ptr::null_mut (), flags as DWORD) == 0) as c_int;  // C 1264
    }
  else
    {
      /* C 1268-1270: `/* Since UNICOWS.DLL includes only a stub for mciSendStringW, we
	 need to encode the file in the ANSI codepage on Windows 9X even
	 if w32_unicode_filenames is non-zero.  */`  */
      if w32_major_version <= 4 || !w32_unicode_filenames              // C 1271
	{
	  let mut fname_a: [c_char; MAX_PATH] = [0; MAX_PATH];          // C 1273
	  let mut shortname: [c_char; MAX_PATH] = [0; MAX_PATH];
	  let mut fname_to_use: *mut c_char = fname_a.as_mut_ptr ();    // C 1273

	  filename_to_ansi (psz_file_or_data, fname_a.as_mut_ptr ());   // C 1275
	  fname_to_use = fname_a.as_mut_ptr ();                         // C 1276
	  /* C 1277-1279: `/* If the file name is not encodable in ANSI, try its short 8+3
	     alias.  This will only work if w32_unicode_filenames is
	     non-zero.  */`  */
	  if !_mbspbrk (fname_a.as_ptr () as *const c_uchar,            // C 1280-1281
	                c"?".as_ptr () as *const c_uchar).is_null ()
	    {
	      if w32_get_short_filename (psz_file_or_data, shortname.as_mut_ptr (),  // C 1283
	                                 MAX_PATH as c_int) != 0
		{ fname_to_use = shortname.as_mut_ptr (); }             // C 1284
	      else
		{ mci_error = MCIERR_FILE_NOT_FOUND; }                  // C 1286
	    }

	  if mci_error == 0                                             // C 1289
	    {
	      memset (sz_cmd_buf_a.as_mut_ptr () as *mut c_void, 0, sz_cmd_buf_a.len ());  // C 1291
	      memset (sz_ret_buf_a.as_mut_ptr () as *mut c_void, 0, sz_ret_buf_a.len ());  // C 1292
	      sprintf (sz_cmd_buf_a.as_mut_ptr (),                      // C 1293-1295
		       c"open \"%s\" alias GNUEmacs_PlaySound_Device wait".as_ptr (),
		       fname_to_use);
	      mci_error = mciSendStringA (sz_cmd_buf_a.as_ptr (), sz_ret_buf_a.as_mut_ptr (),  // C 1296-1297
	                                  sz_ret_buf_a.len () as UINT, core::ptr::null_mut ());
	    }
	}
      else
	{
	  let mut sz_cmd_buf_w: [c_ushort; 520] = [0; 520];             // C 1302
	  let mut sz_ret_buf_w: [c_ushort; 520] = [0; 520];             // C 1303
	  let mut fname_w: [c_ushort; MAX_PATH] = [0; MAX_PATH];        // C 1304

	  filename_to_utf16 (psz_file_or_data, fname_w.as_mut_ptr ());  // C 1306
	  memset (sz_cmd_buf_w.as_mut_ptr () as *mut c_void, 0, sz_cmd_buf_w.len ());  // C 1307
	  memset (sz_ret_buf_w.as_mut_ptr () as *mut c_void, 0, sz_ret_buf_w.len ());  // C 1308
	  /* C 1309-1310: `/* _swprintf is not available on Windows 9X, so we construct the
	     UTF-16 command string by hand.  */`  */
	  wcscpy (sz_cmd_buf_w.as_mut_ptr (), W_OPEN_QUOTE.as_ptr ());  // C 1311
	  wcscat (sz_cmd_buf_w.as_mut_ptr (), fname_w.as_ptr ());       // C 1312
	  wcscat (sz_cmd_buf_w.as_mut_ptr (), W_ALIAS_SUFFIX.as_ptr ());  // C 1313
	  mci_error = mciSendStringW (sz_cmd_buf_w.as_ptr (), sz_ret_buf_w.as_mut_ptr (),  // C 1314-1315
	                              countof! (sz_ret_buf_w) as UINT, core::ptr::null_mut ());
	}
      if mci_error != 0                                                 // C 1317
	{
	  strcpy (warn_text.as_mut_ptr (),                              // C 1319-1320
		  c"mciSendString: 'open' command failed to open sound file ".as_ptr ());
	  strcat (warn_text.as_mut_ptr (), psz_file_or_data);           // C 1321
	  SOUND_WARNING (mciGetErrorString, mci_error, warn_text.as_ptr ());  // C 1322
	  i_result = mci_error as c_int;                                // C 1323
	  return i_result;                                              // C 1324
	}
      memset (sz_cmd_buf_a.as_mut_ptr () as *mut c_void, 0, sz_cmd_buf_a.len ());  // C 1326
      memset (sz_ret_buf_a.as_mut_ptr () as *mut c_void, 0, sz_ret_buf_a.len ());  // C 1327
      strcpy (sz_cmd_buf_a.as_mut_ptr (), c"play GNUEmacs_PlaySound_Device wait".as_ptr ());  // C 1328
      mci_error = mciSendStringA (sz_cmd_buf_a.as_ptr (), sz_ret_buf_a.as_mut_ptr (),  // C 1329-1330
	                          sz_ret_buf_a.len () as UINT, core::ptr::null_mut ());
      if mci_error != 0                                                 // C 1331
	{
	  strcpy (warn_text.as_mut_ptr (),                              // C 1333-1334
		  c"mciSendString: 'play' command failed to play sound file ".as_ptr ());
	  strcat (warn_text.as_mut_ptr (), psz_file_or_data);           // C 1335
	  SOUND_WARNING (mciGetErrorString, mci_error, warn_text.as_ptr ());  // C 1336
	  i_result = mci_error as c_int;                                // C 1337
	}
      memset (sz_cmd_buf_a.as_mut_ptr () as *mut c_void, 0, sz_cmd_buf_a.len ());  // C 1339
      memset (sz_ret_buf_a.as_mut_ptr () as *mut c_void, 0, sz_ret_buf_a.len ());  // C 1340
      strcpy (sz_cmd_buf_a.as_mut_ptr (), c"close GNUEmacs_PlaySound_Device wait".as_ptr ());  // C 1341
      mci_error = mciSendStringA (sz_cmd_buf_a.as_ptr (), sz_ret_buf_a.as_mut_ptr (),  // C 1342-1343
	                          sz_ret_buf_a.len () as UINT, core::ptr::null_mut ());
    }

  if b_reset_volume == TRUE                                             // C 1346
    {
      mm_result = waveOutSetVolume (WAVE_MAPPER, ui_volume_org);        // C 1348
      if mm_result != MMSYSERR_NOERROR                                  // C 1349
	{
	  SOUND_WARNING (waveOutGetErrorText, mm_result,                    // C 1351-1353
			 c"waveOutSetVolume: failed to reset the original volume level of the WAVE_MAPPER device.".as_ptr ());
	}
    }

  return i_result;                                                      // C 1357
}

/* C 1302-1304's wide literals: Rust has no `L"..."` (the `src/w32fns.rs` 385-393 reading), so the
   port spells C 1311's open-quote literal and C 1313's suffix - the MCI device alias this program
   hands the sound system (allowlisted) - as their UTF-16 code units.  */
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const W_OPEN_QUOTE: [u16; 7] =
  [0x006F, 0x0070, 0x0065, 0x006E, 0x0020, 0x0022, 0x0000];
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
pub const W_ALIAS_SUFFIX: [u16; 39] =
  [0x0022, 0x0020, 0x0061, 0x006C, 0x0069, 0x0061, 0x0073, 0x0020, 0x0047, 0x004E, 0x0055, 0x0045,
   0x006D, 0x0061, 0x0063, 0x0073, 0x005F, 0x0050, 0x006C, 0x0061, 0x0079, 0x0053, 0x006F, 0x0075,
   0x006E, 0x0064, 0x005F, 0x0044, 0x0065, 0x0076, 0x0069, 0x0063, 0x0065, 0x0020, 0x0077, 0x0061,
   0x0069, 0x0074, 0x0000];

// C 1360-1368: `DEFUN ("w32-sound-volume", Fw32_sound_volume, Sw32_sound_volume, 0, 1, 0, …)`
#[cfg (all (feature = "have_sound", feature = "windowsnt"))]
DEFUN! ("w32-sound-volume", Fw32_sound_volume, Sw32_sound_volume, 0, 1, 0,
       doc: "Get or set the MS-Windows audio volume setting.\nIf VOLUME is specified, it should be the volume to set, either an integer\nin the range [0, 100], or a float in the range [0, 1.0].  Value of zero\nmeans mute the audio, value of 100 or 1.0 means maximum volume.\nWhen called with the VOLUME argument nil or omitted, just return the\ncurrent volume setting.\nThe return value is the integer volume setting before the change, if any.  ",
       (volume: Lisp_Object),
       {
  let mut ui_volume: DWORD = 0;                                         // C 1370
  let mut ui_volume_orig: DWORD = 0;
  let mut mm_result: MMRESULT = MMSYSERR_NOERROR;                       // C 1371

  if FIXNUMP (volume)                                                   // C 1373
    { ui_volume = XFIXNUM (volume) as DWORD; }                          // C 1374
  else if FLOATP (volume)                                               // C 1375
    { ui_volume = (XFLOAT_DATA (volume) * 100.0) as DWORD; }            // C 1376
  else if BIGNUMP (volume)                                              // C 1377
    {
      let dvolume: c_double = bignum_to_double (volume);                // C 1379
      if dvolume < 0.0                                                  // C 1380
	{ ui_volume = 0; }                                              // C 1381
      else if dvolume > 100.0                                           // C 1382
	{ ui_volume = 100; }                                            // C 1383
      else
	{ ui_volume = (dvolume + 0.5) as DWORD; }                       // C 1385
    }
  else if !NILP (volume)                                                // C 1387
    { wrong_type_argument (Qnumberp (), volume); }                      // C 1388

  mm_result = waveOutGetVolume (WAVE_MAPPER, &raw mut ui_volume_orig);  // C 1390
  if mm_result == MMSYSERR_NOERROR                                      // C 1391
    {
      /* C 1393-1394: `/* Look only at the low 16 bits, assuming left and right channels
         have identical volume settings.  */`  */
      ui_volume_orig &= 0xFFFF;                                         // C 1395
      /* C 1396: `/* The value is between 0 and 0xFFFF, convert to our scale.  */`  */
      ui_volume_orig = (ui_volume_orig as f64 * 100.0 / 65535.0 + 0.5) as DWORD;  // C 1397
    }
  else
    {
      SOUND_WARNING (waveOutGetErrorText, mm_result,                    // C 1401-1403
		     c"waveOutGetVolume: failed to obtain the original volume level of the WAVE_MAPPER device.".as_ptr ());
      ui_volume_orig = 100;                                             // C 1404
    }
  if !NILP (volume)                                                     // C 1406
    {
      if ui_volume > 100                                                // C 1408
	{ ui_volume = 100; }                                            // C 1409
      /* C 1410: `/* Set the same volume for left and right channels.  */`  */
      ui_volume = ui_volume * 0xFFFF / 100;                             // C 1411
      ui_volume = (ui_volume << 16) + ui_volume;                        // C 1412
      mm_result = waveOutSetVolume (WAVE_MAPPER, ui_volume);            // C 1413
      if mm_result != MMSYSERR_NOERROR                                  // C 1414
	{
	  SOUND_WARNING (waveOutGetErrorText, mm_result,                    // C 1416-1418
			 c"waveOutSetVolume: failed to set the volume level of the WAVE_MAPPER device.".as_ptr ());
	}
    }
  return make_fixnum (ui_volume_orig as EMACS_INT);                     // C 1421
});

/* C 1424: `/* END: Windows specific functions */`  */

/* C 1426: `#endif /* WINDOWSNT */`  */

// C 1428-1432: `DEFUN ("play-sound-internal", Fplay_sound_internal, Splay_sound_internal, 1, 1, 0, …)`
#[cfg (feature = "have_sound")]
DEFUN! ("play-sound-internal", Fplay_sound_internal, Splay_sound_internal, 1, 1, 0,
       doc: "Play sound SOUND.\n\nInternal use only, use `play-sound' instead.  ",
       (sound: Lisp_Object),
       {
  let mut attrs: [Lisp_Object; SOUND_ATTR_SENTINEL] = [Qnil (); SOUND_ATTR_SENTINEL];  // C 1434
  let count: specpdl_ref = SPECPDL_INDEX ();                                          // C 1435

  /* C 1437-1439: `#ifdef WINDOWSNT` / `unsigned long ui_volume = 0;` / `#endif`  */
  #[cfg (feature = "windowsnt")]
  let mut ui_volume: c_ulong = 0;

  /* C 1441-1443: `/* Parse the sound specification.  Give up if it is invalid.  */`  */
  if !parse_sound (sound, attrs.as_mut_ptr ())                        // C 1442
    { error (c"Invalid sound specification".as_ptr ()); }             // C 1443

  let mut file: Lisp_Object = Qnil ();                                // C 1445

  /* C 1447-1501: `#ifndef WINDOWSNT` - the non-Windows half.  */
  #[cfg (not (feature = "windowsnt"))]
  {
    current_sound_device = xzalloc (core::mem::size_of::<sound_device> ()) as *mut sound_device;  // C 1448
    current_sound = xzalloc (core::mem::size_of::<sound> ()) as *mut sound;                       // C 1449
    record_unwind_protect_void (Some (sound_cleanup));                                            // C 1450
    let mut headerbuf: [c_char; MAX_SOUND_HEADER_BYTES] = [0; MAX_SOUND_HEADER_BYTES];            // C 1451
    (*current_sound).header = headerbuf.as_mut_ptr ();                                            // C 1452

    if STRINGP (attrs[SOUND_FILE])                                                                // C 1454
      {
        /* C 1456: `/* Open the sound file.  */`  */
        (*current_sound).fd =
          openp (list1 (Vdata_directory), attrs[SOUND_FILE], Qnil (), &raw mut file, Qnil (),
                 false, false, core::ptr::null_mut ());                                           // C 1457-1459
        if (*current_sound).fd < 0                                                                // C 1460
          { sound_perror (c"Could not open sound file".as_ptr ()); }                              // C 1461

        /* C 1463: `/* Read the first bytes from the file.  */`  */
        (*current_sound).header_size
          = emacs_read ((*current_sound).fd, (*current_sound).header as *mut c_void,
                        MAX_SOUND_HEADER_BYTES as ptrdiff_t) as c_int;                            // C 1464-1466
        if (*current_sound).header_size < 0                                                       // C 1467
          { sound_perror (c"Invalid sound file header".as_ptr ()); }                              // C 1468
      }
    else
      {
        (*current_sound).data = attrs[SOUND_DATA];                                                // C 1472
        (*current_sound).header_size
          = min (MAX_SOUND_HEADER_BYTES as ptrdiff_t, SBYTES ((*current_sound).data)) as c_int;   // C 1473
        memcpy ((*current_sound).header as *mut c_void,
                SDATA ((*current_sound).data) as *const c_void,
                (*current_sound).header_size as usize);                                           // C 1474-1475
      }

    /* C 1478: `/* Find out the type of sound.  Give up if we can't tell.  */`  */
    find_sound_type (current_sound);                                                              // C 1479

    /* C 1481: `/* Set up a device.  */`  */
    (*current_sound_device).file = attrs[SOUND_DEVICE];                                            // C 1482

    if FIXNUMP (attrs[SOUND_VOLUME])                                                               // C 1484
      { (*current_sound_device).volume = XFIXNAT (attrs[SOUND_VOLUME]) as c_int; }                 // C 1485
    else if FLOATP (attrs[SOUND_VOLUME])                                                           // C 1486
      { (*current_sound_device).volume = (XFLOAT_DATA (attrs[SOUND_VOLUME]) * 100.0) as c_int; }   // C 1487

    CALLN! (Frun_hook_with_args, Qplay_sound_functions (), sound);                                  // C 1489

    /* C 1491-1495: `#ifdef HAVE_ALSA` / `if (!alsa_init (current_sound_device))` / `#endif` /
       `if (!vox_init (current_sound_device)) error ("No usable sound device driver found");` -
       the ALSA call is the outer `if`'s condition when the arm is on and absent when it is off.  */
    #[cfg (feature = "have_alsa")]
    let driver_ok = alsa_init (current_sound_device);
    #[cfg (not (feature = "have_alsa"))]
    let driver_ok = false;
    if !driver_ok
      { if !vox_init (current_sound_device)                                                        // C 1494
          { error (c"No usable sound device driver found".as_ptr ()); } }                          // C 1495

    /* C 1497: `/* Open the device.  */`  */
    ((*current_sound_device).open.unwrap ()) (current_sound_device);                                // C 1498

    /* C 1500: `/* Play the sound.  */`  */
    ((*current_sound).play.unwrap ()) (current_sound, current_sound_device);                        // C 1501
  }

  /* C 1503-1531: `#else /* WINDOWSNT */` - the Windows half.  */
  #[cfg (feature = "windowsnt")]
  {
    if FIXNUMP (attrs[SOUND_VOLUME])                                                               // C 1506
      { ui_volume = XFIXNAT (attrs[SOUND_VOLUME]) as c_ulong; }                                    // C 1507
    else if FLOATP (attrs[SOUND_VOLUME])                                                           // C 1508
      { ui_volume = (XFLOAT_DATA (attrs[SOUND_VOLUME]) * 100.0) as c_ulong; }                      // C 1509

    if ui_volume > 100                                                                             // C 1511
      { ui_volume = 100; }                                                                         // C 1512

    /* C 1514-1516: `/* For volume (32 bits), low order 16 bits are the value for left
       channel, and high order 16 bits for the right channel.  We use the
       specified volume on both channels.  */`  */
    ui_volume = ui_volume * 0xFFFF / 100;                                                          // C 1517
    ui_volume = (ui_volume << 16) + ui_volume;                                                     // C 1518

    CALLN! (Frun_hook_with_args, Qplay_sound_functions (), sound);                                 // C 1520

    if STRINGP (attrs[SOUND_FILE])                                                                 // C 1522
      {
        file = Fexpand_file_name (attrs[SOUND_FILE], Vdata_directory);                             // C 1524
        file = ENCODE_FILE (file);                                                                 // C 1525
        do_play_sound (SSDATA (file) as *const c_char, ui_volume, false);                          // C 1526
      }
    else
      { do_play_sound (SDATA (attrs[SOUND_DATA]) as *const c_char, ui_volume, true); }             // C 1529
  }

  return unbind_to (count, Qnil ());                                                               // C 1533
});

/***********************************************************************
			    Initialization
 ***********************************************************************/

// C 1540-1541: `void syms_of_sound (void)`
#[cfg (feature = "have_sound")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_sound ()
{
  DEFSYM! (QCdevice, ":device");                                        // C 1543
  DEFSYM! (QCvolume, ":volume");                                        // C 1544
  DEFSYM! (Qsound, "sound");                                            // C 1545
  DEFSYM! (Qplay_sound_functions, "play-sound-functions");              // C 1546

  defsubr (&raw mut Splay_sound_internal);                              // C 1548
  /* C 1549-1551: `#ifdef WINDOWSNT` / `defsubr (&Sw32_sound_volume);` / `#endif`  */
  #[cfg (feature = "windowsnt")]
  defsubr (&raw mut Sw32_sound_volume);                                 // C 1550
}

/* C 1554: `#endif /* HAVE_SOUND */` - the file gate's own closing line.  Every item of this unit
   carries `#[cfg (feature = "have_sound")]` instead, so the line has no counterpart; it is the end
   of `emacs/src/sound.c` and of this port.  */
