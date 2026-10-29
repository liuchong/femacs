# Cocoa background color writes into a Lisp frame field

## Failure and cause

The user reported an ordinary startup crash during automatic package
installation. The current normal Cocoa product reproduced it in 23.921 seconds
while loading the actual configuration: SIGABRT before initialization completed.
The executable, dump and configuration-file identities remained unchanged.
An isolated two-frame allocation exercise and local HTTP package-install
exercise had not reproduced it; their success was not a repair or acceptance
of normal startup.

The original system report matches the current executable UUID
`2be96930-6093-3846-899d-f4c23207a100`. It aborts at the float bitmap access in
`process_mark_stack`, index 63 for 63 words. Opt-in diagnostics instead expose
the invalid object `0xfffeffff`, decoded as pointer `0xfffefff8`, with no
allocated-memory membership and no active string interval.

A debugger stop before the invalid-object abort binds the consumed mark-stack
array cell to the actual frame's `tab_bar_items`, offset 192. The native
`face_set_background` machine code writes its pixel argument at receiver offset
192, which is correct for `face.background`. The Swift caller
`ns_set_background_color` passed its frame, despite obtaining the real default
face immediately above the call. The original setter assigns
`face->background`. The accidental color write therefore becomes an invalid
Lisp object when the frame is collected.

The fix passes the actual default face. It retains color and alpha conversion,
frame background pixels, native ownership and face invalidation. No collector
check is disabled and no object or bitmap index is ignored or clamped.

## Source and evidence binding

- Starting commit: `8051f72ef2ecdebaa28a98e7f2f2b8f4a604dd6f`.
- Old executable SHA-256:
  `ad93367b0afa71dac9cb820cba6d28328e745a45a1f3a60337a2b30c8fed6b41`.
- Old Swift file SHA-256:
  `5b68f1deb223affa301535a2d341f2075792a1f342678dde165269be944e3461`.
- Repaired Swift file SHA-256:
  `cfddaa1aac08c74db1c1a349085843ededcb8618d55aaa2ac89995e18573bc76`.
- `PORTING.md` P432 records the decision before implementation.
- Fresh source cohort: `build/acceptance/g1-background-repair-1/source`,
  a local checkout plus exactly the one-line receiver repair. Unrelated working
  changes and the ignored reference are not build inputs.
- Native crash reports, debugger addresses, configuration identities and normal
  startup logs stay in private evidence; they are not published as user data.

## Required acceptance

Fresh Cocoa build and strict signature, real background-change/collection
regression with the installed reference and repaired product, then explicitly
authorized normal configuration/package startup. The original crash and any
new startup failure remain failures until their own causes are resolved.
Receiver/sentinel component protection follows functional acceptance. Do not
call the entire Goal, arbitrary package compatibility or all GUI stability
complete from this one repair.

## Current measured result

The fresh normal Cocoa build and strict signature passed in 488.110 seconds;
all tracked build inputs remained unchanged. New executable SHA-256:
`b2fe7420ad1c3a041ea33ea9d0ccf80b852b818ea3315ffa65dd1aff84b2bcb0`;
dump SHA-256:
`95eae81514a0998426ce39e778802776ed4579987ff66344141b448d2ad5aec2`.
The exact validated bundle was published to `dist/macos/Femacs.app`, with the
old bundle temporarily retained for explicit retirement.

The installed reference and new product each passed all eight real native
background/GC cases. Initial execution of the new fixture timed out on both
products because its definition lacked one closing parenthesis. That raw
attempt remains in `integration`; only after correcting that fixture did
`integration-2` pass all 16 cases. No product assertion was weakened.

Post-functional protection compiles the actual Swift function against separate
frame/face sentinels. All four branches pass: opaque, transparent, missing face
and missing view. Restoring the old wrong receiver in a private mutant is
rejected by both face-present branches, with ordinary nonzero exit rather than
a deliberate crash. This is a caller contract test, not native layout proof.
See `tools/test-macos-background-receiver.py` and this cohort's `components`.

Normal configuration startup was checked separately. The first bounded
observation exposed 325 packages to install; its 180-second
deadline stopped an unfinished installation without a GC failure. A subsequent
1200-second run continued the remaining 243 packages, visibly reached at least
219/243, performed Lisp byte compilation and built a native terminal module.
Its deadline was reached before the initialization marker; this is neither a
new product crash nor a completed startup. Actual configuration identities
remained unchanged. These original timeouts remain failures to complete the
observation, not successful startup records.

The next run resumed the changed installation state and completed normal
startup in 25.806 seconds, then exited normally as scheduled. The report has
`after_init=true`, `init_error=false`, 443 installed package entries and one
native frame. Program, dump and configuration-file identities are unchanged;
installed packages intentionally persist. This completes the observed normal
startup path, not every feature of all installed packages, deferred services,
performance or the whole G1/G3/Goal.

The same reusable native fixture was also run on the exact old executable in
an isolated environment. It hit the original invalid-float branch with
`0xfffeffff`; the debugger stopped and killed that owned child before abort.
The first diagnostic classifier compared an unpadded hexadecimal literal with
the debugger's padded register output and reported false. Numeric analysis of
the retained raw output establishes the expected failure without rerunning the
editor; both classifications and their binding remain in private evidence.

Independent review accepted the one-line repair and limited native/caller
regressions after nine actual read-only checks. Seven further read-only checks
adopted the observed normal startup, registration and cleanup evidence, closing
only this known background-receiver GC failure. This does not establish general
GC health or complete G1/G3. Only one completed normal-startup run is recorded;
the local GUI acceptance group itself was not rerun end-to-end.
