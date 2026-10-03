// femacs: port of src/sound.c @ fd7afced403.
/* Tested with Luigi's driver on FreeBSD 2.2.7 with a SoundBlaster 16.  */

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

/* femacs: emacs/src/sound.c (@ 1556 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef HAVE_SOUND` at C line 39, and the frozen target's
   configuration leaves HAVE_SOUND undefined (docs/baseline/config/darwin-arm64.macros.tsv
   951, `HAVE_SOUND<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the ALSA/OSS sound primitives (`play-sound`) - needs
   <sys/soundcard.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
