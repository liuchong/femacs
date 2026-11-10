# Bytecode indirect-call frames (P458)

The current original advice test exposed a real bytecode regression: adding an
outer before advice made the inner around advice lose its interactive caller.
The candidate returned `(((1) 11) t)` instead of `(((1 . t) 11) t)`.

Relative to `920c9896`, `src/bytecode.rs` removes the funcall/apply shortcuts
that replaced an existing backtrace frame, including their sole-use spread
helper and nested arithmetic/closure shortcuts. Indirect calls now use normal
MANY-subr entry and `Fapply`/`Ffuncall`, retaining both outer and target frames.
Ordinary direct bytecode closure dispatch and other VM optimizations remain.
Original test files and packaged Lisp were not modified.

## Current products

Both standard products were rebuilt from empty output roots, using the same
pinned Tree-sitter 0.27.1 SDK. Terminal took298.747s, peak415,596,544 bytes;
Cocoa took301.729s, peak858,193,920 bytes. Both controllers exited0 and fully
reaped their process trees. Core source SHA256 is
`0ebe8f3561c6af9b9131b4ebded2fcf80498c529aa63945e5c4fabddcdce26ea`.

| Product | SHA256 |
| --- | --- |
| `build/terminal/port/src/emacs` | `7ea63c5f3d0ef9e0de4160845b838974ca0ea4dcbf8112b2534720270ebd0506` |
| Terminal dump | `5c165a8a87fcb121f98e6bab3cb23625b943abed7a57ef54016caa92ed6f7010` |
| `dist/macos/Femacs.app` program | `f99ef8e2317ec613ad51525cd77fbd2a3d43f56bced877f716401d52439d7557` |
| Cocoa dump | `4d9486d4b0d19a783cf73e74c47d1b7fdba1a8af3356e9c68328375357dcf4cd` |
| `dist/releases/p458-reviewed/Fe-macos.dmg` | `d0293c6f4cff5e6a79a3197b622512812eca67f7db9d05ce12da074edbbd2fbf` |

The release App has the same program/dump identities, passes strict bundle
signature verification, and remains ad-hoc signed and not notarized. The user's
installed App is still running; replacement is pending a safe exit.

## Functional acceptance and protections

- `p458-functional-3`: three real products run the complete seven-case owned
  call-frame fixture and the original13-test advice file. All21 owned cases
  pass; original status maps agree:33 ordinary passes,6 original declared
  expected failures, zero unexpected. The previously regressing interactive
  advice test passes on all three products. No original failure is waived.
- The owned fixture forces actual indirect bytecode instructions, checks
  immediate target/funcall-or-apply/wrapper frames, nil/single/multiple spread
  and fixed-prefix arguments, GC root survival, error and throw unwinding,
  absence of residual frames, and recovery calls. State retirement and product
  and source identities pass. The controlled run exits0 in6.441s.
- Only after functional acceptance, `rhp09` and `rhp10` add critical frame,
  unwind/recovery and nested interactive-advice protections. All10 hot-path
  protections and all10 existing bytecode tests pass on each product:60 cases,
  zero unexpected, matching maps, retired state and unchanged products.
- Local acceptance passes all9 checks, including current bundle signature,
  Terminal/Cocoa batch operation, actual terminal init/custom startup, keys
  and file workflows. Six current Terminal TTY recovery cases also pass.
  These refresh affected local layers, without claiming physical GUI/IME or
  whole-corpus acceptance.

Failed preparation is preserved separately: the first driver was rejected
before editor launch because its output was outside the owned build root.
The first real owned fixture failed on all three products because it bound
`byte-compile-warnings` before requiring its defining library; explicit
`bytecomp` loading corrects that fixture. Its initial reader also incorrectly
required all original cases to be ordinary passes; the original source itself
declares two expected failures. Corrected acceptance retains those exact raw
statuses and requires zero unexpected results and matching complete maps.
Neither failed aggregate was rewritten.

## Remaining scope

Independent review accepts the complete source, actual functional and
post-functional protection unit. [CLN222](../cleanup/reports/2026-11-10-222-superseded-p453-build-app-backups.md)
retires the three superseded owned backups, with current products and both
reviewed release images protected.

The interrupted first555-file pair remains incomplete at336 files. Its frozen
source and61 profiles passed separate preservation postflight; those earlier
products do not certify this new VM source. Full current-source compatibility,
runtime matrix, performance/components decisions and installation/final review
remain open. Goal stays active3/7.
