# Standard committed-checkout macOS delivery

The Cocoa product and disk image measured below predate the P432 background
receiver repair. The retained image is historical evidence, not the current
trial build. Use the repaired `dist/macos/Femacs.app`; see the
[defect and current startup evidence](../incidents/2026-10-29-cocoa-background-receiver.md).
The terminal delivery evidence is unaffected.

## Outcome and scope

A real remote shallow checkout of signed `cc774482` (tree
`e7acbb6c89c05e69d9be81605b2a602637d04935`) completed both documented standard
builds with no reference checkout, old objects, bytecode, product overlay or
installed editor as a build input. This is an actual committed checkout,
separate from the earlier proposed-tree/runtime-build acceptance.

The standard terminal remains checkout-bound. The separately rebuilt root
terminal uses the root resources; the exported terminal was not copied there.
The self-contained standard Cocoa app was packaged and atomically published
only after verification. This is G2 delivery evidence, not whole G1/G3,
100-percent compatibility, performance, native compilation or generational-GC acceptance.

## Build and functional evidence

| Boundary | Actual result |
| --- | --- |
| Remote checkout | Clean tracked state, valid Git signature; no ignored reference/build/dist/target inputs |
| Standard terminal build | `cargo run -p xtask -- build-terminal`, 451.003 seconds, exit 0 |
| Standard Cocoa build | `cargo run -p xtask -- build-macos`, 315.986 seconds, exit 0; strict app signature passes |
| Owned functional fixtures | 42 per product: language 9, XML 13, SQLite 10, LCMS 7, parser/GC lifecycle 3; 84 unique final passes, zero skips |
| Fresh-checkout file workflows | Two passes: Unicode/combining text, save, close, reopen, normal exit; Cocoa also clipboard and post-GC editing |
| Standard release packaging | Actual App/DMG and checksums; disk image verification, payload/dump/icon and strict signature checks pass; every app member matches the input app |
| Source-denied moved app | Three existing original source/program/dump probes unreadable in the child; four real language/XML/SQLite/LCMS cases pass; originals unchanged |
| Root terminal build | Standard recipe, 252.004 seconds, exit 0; tracked inputs unchanged |
| Current default paths | 26 classic keys per product and two file workflows: 54 passes; app and terminal/dump identities unchanged |

The first functional attempt recorded 80 passes and four failures because the
acceptance owner omitted the XML/SQLite output-directory variables. The original
false result remains; only those four unchanged cases were rerun with isolated
owned directories and all passed. Do not count those retries as 88 unique cases.
The first root-build preparation stopped before compilation after discovering
one test fixture's mode difference; this is not a product-build failure.

The root worktree is explicitly **not** a second clean checkout: documentation
WIP, an Android JNI table change, an unused untracked receipt helper and the
same-content fake-aspell fixture's mode difference remain. All tracked files were
compared with the clean clone. The JNI include is conditional on the Android
features, absent from the macOS target; the receipt helper/test-fixture mode are
not build inputs. These differences were retained, not overwritten or published
as part of this delivery. The remote clone supplies the clean-build proof.

## Product identity and trial

| Current artifact | SHA-256 |
| --- | --- |
| Root terminal program | `d4896d6f03d3f5f18f1aa57f189dfac1cb52ab6e8c8feeb71b5eb4b689ed9d26` |
| Root terminal dump | `c028a6bc2be4f231fa9b807abb4b092b07a9f16c5bf8c15329b389cbe85f1068` |
| Published Cocoa program | `ad93367b0afa71dac9cb820cba6d28328e745a45a1f3a60337a2b30c8fed6b41` |
| Published Cocoa dump | `598a4453ab6386fdea3f8956fc5df3ddbd8c436e52212d2c5cb045994f7369e0` |
| Release disk image | `99f949c8f2c5eb7875179eb54eebd2a9bd25968d3a6d369d580c47b94551dfb4` |

From the repository root:

```sh
build/terminal/port/src/emacs -nw -Q
open dist/macos/Femacs.app
```

The generated distribution is
`build/acceptance/g2-standard-package-1/release/Fe-macos.dmg`, with `release.json`
and `SHA256SUMS` beside it. It is ad-hoc signed, not notarized. The previous app
is retained at `dist/macos/.Femacs-before-g2-standard.app` for rollback while G1
IME/focus acceptance remains open. Publication used exclusive verified staging
and an atomic directory exchange, without deleting the previous app.

## Receipts and remaining work

Raw receipts: `build/acceptance/g2-committed-standard-1`,
`g2-standard-functional-1`, `g2-standard-functional-2`,
`g2-standard-package-1`, `g2-root-terminal-1`, `g2-root-terminal-2`, and
`g2-current-delivery-1` under the same acceptance root. They retain original
process exits, exact products/resources, fixtures, per-case timings and failures.
All workload controllers ended with complete owned-process cleanup. The clean
build reported 1,052,131,328 peak accounted bytes; root rebuild 661,848,064;
packaging 330,137,600; current input delivery 203,112,448. These are recorded
run peaks, not account budgets or whole-project resource totals.

No production or test assertion changed in this verification delivery. Existing
unaffected foundation protections remain valid. The same independent partner
accepted the limited G2 delivery after five actual read-only checks. CLN136
retired 1,194,535,878 logical bytes of consumed compiler data and two exact App
copies; 22,033 manifest rows and 44,066 paired operations were verified, with
source/products/resources/failures retained. See the
[cleanup report](../cleanup/reports/2026-10-29-136-consumed-standard-compiler-app-copies.md).
G2 is accepted; only G2 and G4 are delivered (2 of 7). G1 pending
composition/focus/mouse, full stable compatibility, P427 EBADF, intermittent
remote Base64, five common compiler metadata comparisons and native/GC
measurement/adoption remain open. No remote CI was run.
