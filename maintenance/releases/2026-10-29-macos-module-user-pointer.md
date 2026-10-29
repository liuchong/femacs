# macOS module user-pointer predicate (P429)

## Problem and change

The macOS products enable dynamic modules, but `src/data.rs` still omitted
`user-ptrp` under the initial module-disabled frozen-target decision. Consequently
`cl-preloaded.el` did not establish the built-in `user-ptr` class. The complete
original module file failed at its generic method definition with
`Unknown specializer user-ptr`, before registering its cases.

Restore the original `Fuser_ptrp` definition and `Suser_ptrp` registration under
`have_modules`, using the existing `USER_PTRP` representation check. The original
stable source at `a360712c9d272d950d8d8255ef74570f7e90b7d9`, `src/data.c:499–508`
and `:4360–4362`, agrees with the local upstream source `fd7afced4036fc1a221567d70705d20a2bd64713`.
Feature-off products retain no definition or registration. No algorithm, module
ABI, test assertion, Lisp class implementation, or object lifetime was changed.

## Actual acceptance

- The initial cohort retained the installed GNU Emacs 31.1 reference's 39 passing
  original cases and four candidate load failures. Zero registered candidate IDs
  at that point did not mean zero-failure acceptance.
- Four fresh frozen-workspace products were built: Cocoa and terminal,
  normal/checking. Both Cocoa apps passed strict signature verification.
  The build finished in 1115.477 seconds; its owned process tree was reclaimed.
- After the fix, each new product passed all 39 unmodified original module cases:
  156 newly executed passes, zero skips/failures/timeouts. The unchanged reference
  result was reused only after matching its program, dump, Lisp, original inputs,
  header and compiled C-module identities: 195 unique case-products in total.
  The original cohort run finished in 34.089 seconds with complete tree cleanup.
- After that integration pass, two new protection cases ran on all five products:
  10 passes, zero skips/failures/timeouts. They exercise a real C-module pointer,
  generic dispatch, nine ordinary values and argument-count errors, with no
  intentional aborts or thread stress. This run finished in 23.918 seconds.
- Source, original inputs, fixtures, program/dump and packaged resources remained
  unchanged across each acceptance run. Original failures remain retained.

The original `module--test-assertions--*` cases intentionally abort child
processes and require the corresponding assertion diagnostic. macOS emitted
crash reports for these children. They are part of the original safety tests,
not evidence of normal interactive startup or GUI stability. The post-integration
protection cases do not repeat those aborts.

## Bound products

These identities belong to the tested complete frozen workspace, not to an
isolated clean checkout of this small source change.

| Product | Program SHA256 | Dump SHA256 |
| --- | --- | --- |
| cocoa-normal | `487879c78cfd32cb8255c3f9fd9c29bddeb9d591aef6c54d80a202501b856442` | `189a4090f2666ebec60c56ad784edecb46fced6bdf16a9017b03ecdcf9054a65` |
| cocoa-checking | `c2299d3dac74e0160282c29c6c973298e741b92f0f4fc894a5cf11ff21455342` | `dad187550e21decb78e86f10e5810b2e90a9299035f1c34e70c07233445e8b43` |
| terminal-normal | `48acbdac54cf12d147a26e0401a0a4f711eee6bb98ee826af90836f1ce1ec392` | `e2ce0c9a12311a900fb18fa8601fa12cef76b2329360208426cf22691db17398` |
| terminal-checking | `ee313039836547968ca9b334ae184356050d8f53e786ac34b158a5580de974fa` | `9fb6891a8850946febd525fc15a96593499c674f8440b091bf6eb2a0c9911f0f` |

The shared original C fixture SHA256 is
`15c9452d6259886d216e4c4b708dc9353cd7e9887b4891df1afc1be08a9d909a`.
It was compiled once against the pinned stable public module ABI and reused
identically. Candidate packaged module headers matched the generated API.

Local raw evidence is retained in
`build/acceptance/runtime-p429-stable-products-1`,
`build/acceptance/runtime-p429-module-original-1`,
`build/acceptance/runtime-p429-module-original-2`, and
`build/acceptance/runtime-p429-module-protection-1`.
The new guard source is `tools/fixtures/module-user-pointer-protection.el`.
Owned invocation wrappers resolve the actual product and dump and give the
original tests their expected module-resource layout; installed app files are
not modified. The ignored reference checkout is not a build dependency.

## Review and remaining scope

Independent final review accepted this limited component after checking the
actual two-hunk candidate, C source, first failure, full original results and
protection results. No blocking defect was found in the fix. The source-delivery candidate
contains only the predicate and its conditional registration, the new protection
fixture and directly related documentation; existing native/runtime work stays
separate. The build uses the complete frozen workspace, so this evidence does
not establish isolated-commit clean-checkout reproduction.

This fixes the demonstrated module-file load failure. It does not close G3 or
the full Goal. The independent terminal EBADF abort, remote notification Base64
failure, remaining compiler/native entrypoints, Tree-sitter implementation gaps,
G1/G2 and native/GC acceptance remain open. LCMS has limited product acceptance;
its runtime/TLS prerequisite source group and relocation evidence remain pending.
