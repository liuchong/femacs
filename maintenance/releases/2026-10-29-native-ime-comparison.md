# Native two-frame input and cancellation comparison — 2026-10-29

The retained P421 Cocoa normal product completed this bounded native workflow:
real Chinese composition on A, manual B → A → B mouse transfers with further
Chinese entry, additional physical `nihao` + Space commits on both frames,
normal file saves, and normal `C-x C-c` exit. The user reported that the manual
transfers and entry worked normally. Native key-window, first-responder,
composition and commit callbacks separately record the actual frame transitions.
This is current-product workflow evidence, not complete G1 acceptance.

## Cancellation comparison

The installed stable C reference and the candidate both received physical
`n`, `i` in Chinese mode. With `ni` marked, `C-g` left composition active;
`Escape` removed the marked composition without committing text. The reference
then committed `你好` through physical `nihao` + Space. The candidate committed
the same text on B and again on A after returning to it.

The earlier candidate-only `C-g` observation is therefore not sufficient to
declare a regression. The inactive `working` string can retain its previous
value after cancellation; acceptance uses the marked-overlay state and actual
committed buffer contents rather than treating that string as active input.
This is an observed-sequence comparison: input-source identity and settings
were not captured at the reference launch, so identical input-method
configuration is not independently established by these receipts.

## Actual products and files

| Product | Executable SHA-256 | Actual packaged dump SHA-256 |
| --- | --- | --- |
| Stable C reference | `5b55e93c8304368ee37cc5c537ca3e1370e3b9b3fef049bdce76834a9db75e65` | `6ac1214186022574a70e84a04bb7f2dc44e1ea1d4c90f0d9ce421ddd0f4b4536` |
| P421 Cocoa normal | `008b20b65ebea8f1fe2426a6e1630536267f849a6c29fbe9fa8adcf6a16dc028` | `8164be808762948b4c2cde06fb3d84181540147ece815a33ec40d1b992e6095d` |

Both launch commands selected their actual packaged dump explicitly. Executable,
dump, Lisp resources, frozen source and fixture identities were unchanged across
each run. The candidate is built from the retained runtime working-tree cohort;
this does not establish a final tracked-only build from the delivered HEAD.

The saved UTF-8 files match the respective final buffer snapshots byte for byte,
including the newline added by the editor's normal save workflow:

- A: 52 bytes; SHA-256
  `f2255c36aa3b1e5627117c6a87e8f2ee6735e12a022499179ee4cab5e32d7072`.
- B: 44 bytes; SHA-256
  `7e6910f42512672b646100e1554e3dc8498bec6764868d66dcf378a9c986f73e`.

Both native processes exited 0. Both resource controllers exited 0 and confirmed
complete cleanup. Reference controller wall time was 94.509 seconds with
228114432 bytes peak accounted memory; candidate wall time was 809.707 seconds
with 195002368 bytes peak. These times include interactive operation and human
waits and are not performance measurements.

## Preserved limits and unsuccessful attempts

- The initial candidate A and B attempts inserted English text. They are
  preserved and do not count as Chinese-input acceptance. The user switched
  each test window into Chinese mode before the actual composition checks.
- The input tool rejected a standalone Shift. Two status-menu queries timed
  out, and Control-Space reached the editor's mark command. No programmatic
  input-source selector or system preference change was used.
- Physical Command-grave inserted a grave character rather than transferring
  frames. Physical `C-x 5 o` left a prefix while `o` entered composition.
  These attempts are not passing shortcut results or established regressions:
  the equivalent reference sequences have not been compared. Automatic
  follow-up frame selection used `M-x other-frame` with Unicode command text;
  the user's mouse transfers are separate actual evidence.
- A moved/background-window coordinate click failed. Raising the freshly
  identified accessible window then allowed follow-up input. This tool failure
  is not counted as a product crash or a successful mouse-transfer test.
- Other input methods, all shortcuts, current-product external Finder drag,
  the final independent build and full stable compatibility remain outside
  this workflow. Whole G1 and the seven-part Goal remain open.

Private receipts remain under `build/acceptance/g1-p421-reference-1` and
`build/acceptance/g1-p421-candidate-1`: actual launch ownership, before/after
identities, bounded observations, human-result record, saved files and native
exit results. The initial passive snapshot interpreted before the human reply
was corrected by a subsequent explicit record; raw observations were preserved.
Controller receipts are retained with the existing continuation evidence.

The existing independent partner read the raw steps, actual launch arguments,
before/after identities, later A/B callbacks, controller receipts and saved
file bytes, and explicitly accepted these bounded facts with the listed limits.
Its first response was interrupted by a provider certificate error; one
same-session follow-up completed the review. It did not rerun the UI, edit the
implementation or independently establish identical input-source configuration.
