# Limited foundation package relocation evidence

The four baseline products built from the exact foundation tree were each copied
outside their original source/build roots and run under a Darwin sandbox denying
reads of those original roots and original native-cache directories. The original
source, all four selected product/resource identities and copied package bytes
remained unchanged. No new build was needed for this acceptance.

Each moved normal/checking Cocoa/terminal product passed strict package/runtime
library signatures, demonstrated that the three existing original source/program/
dump paths were unreadable, and passed the existing lexical-closure/redefinition
Lisp case. The four-product execution took 22.515 seconds. Its controlled run
finished in 31.007 seconds, with a 50,921,736-byte accounted peak and complete
process cleanup. The original controlled run exited **1**, because one
package retirement was still blocked.

Raw `results.json` remains **false** because the ordinary terminal copy was
held by Spotlight during the initial retirement check. All its signature, denial
and language checks had passed; this was a cleanup failure. After the holder
was gone, the copy was checked against its unchanged original package again and
retired. `cleanup-followup.json` records that resolution without changing the
original failure or rerunning any test. All four temporary copies are now absent.

Raw evidence is in `build/acceptance/runtime-foundation-relocation-1`.
The [CLN135 report](../cleanup/reports/2026-10-29-135-consumed-foundation-relocation.md)
records each retirement and recovery boundary.

## Limits and other open failures

This is relocated **batch** execution of four baseline products, one language
case each. It does not prove physical GUI/IME/Finder interaction, all optional
capabilities after relocation, whole stable compatibility, native/collector
performance, or a fresh actual committed-checkout build. Native/collector remain
explicitly opt-in. See the [source foundation report](2026-10-29-runtime-foundation.md).

P427 is still unresolved. One bounded diagnostic replay of the original 39-case
process file used the unchanged first-failing terminal-checking program with a
self-checked private descriptor recorder. The real dyld loader confirmed that
exact recorder and its interposition table loaded. The run completed 35 passes,
two original-condition skips and two known environment failures (synthetic DNS
for an invalid domain and a long socket service name), with no EBADF capture or
abort. The program, dump, original file and runner were unchanged. The initial
preparation failed before running any case because a retired context path was
used; a new private context linked the retained original inputs for the actual
replay. Preparation and diagnostic evidence are retained privately. Neither
nonreproduction nor instrumentation is a product fix or a compatibility pass.

The seven-part Goal remains active with only G4 delivered.
