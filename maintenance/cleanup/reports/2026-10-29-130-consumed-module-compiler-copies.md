# CLN130: consumed module compiler copies

## Retired scope

P429's four-product build, complete original module acceptance, non-aborting
protection and independent final review have all ended with complete owned-tree
cleanup. The source fix is signed and pushed as `ccb38ee417259bcfef48c1992571dcc20d27d7fb`.
Only the following consumed, ignored compiler-copy roots were retired:

- `build/acceptance/runtime-p429-stable-products-1/cargo`
- `build/acceptance/runtime-p429-stable-products-1/source/build/runtime/stable-notify-cocoa/objects/units`
- `build/acceptance/runtime-p429-stable-products-1/source/build/runtime/stable-notify-cocoa/checking/objects/units`
- `build/acceptance/runtime-p429-stable-products-1/source/build/runtime/stable-notify-terminal/objects/units`
- `build/acceptance/runtime-p429-stable-products-1/source/build/runtime/stable-notify-terminal/checking/objects/units`

They contained 19,910 regular files, no symlinks, 903 directories and 703,344,044
logical bytes (about 670.8 MiB). All five selected roots are now absent. There
were no initially absent items counted as deleted.

An unused AI-created auxiliary proposal, `controlled-run-state-dir-fix.patch`,
was also withdrawn from the owned AI cache: one file, 692 bytes, SHA256
`f6399834e4d09fc68de81988cf2a6caf239b1d1c2c39e4a7a2b280baa79f7cc1`.
It was never applied; there is no installed-tool change to revert. This file is
counted separately from the five compiler roots.

## Identity, operation and privacy evidence

The public manifest contains 20,814 rows: 20,813 compiler-file/directory rows
and one auxiliary-file row. Its full 10,195,244 decoded bytes were checked field
by field against the private exact plan after publication transformation.
Every unlink and rmdir has a durable intent/result pair: 41,628 operation
records, with no missing or duplicate paths.

18,798 paths containing machine-derived source names use irreversible aliases;
the exact original names stay in the private plan. Public rows retain content
hashes and filesystem identity. All published rows were decoded, matched and
checked without publishing personal machine paths. The auxiliary row uses a
logical cache label, without its host directory or conversation ID.

Manifest SHA256:
`62a286c22440a2c5ed41825ad1bfd86e378bf406413036f06b40e6c7588fbc37`.

## Preserved and checked

The frozen source export, 727 selected retained files, real object files,
programs/dumps, Lisp/LCMS/license product identities, compiled stable C module,
original failures, raw results and reconstruction recipes remain. Their bound
identities and frozen-source verification were unchanged after retirement.
TLS/package directories, source contexts, installed apps, the reference corpus,
other owners' files and unrelated native/runtime work were outside the selected
roots and were not swept. No new compiler or test was started for cleanup.
Git HEAD, the staged tree and tracked entries remained unchanged during removal.

The observed filesystem availability changed from 133,195,812 to 133,919,628
KiB. This is a shared-filesystem observation; its entire difference is not
attributed to this cleanup.

## Restore and remaining work

Rebuild the named Cargo and generated-unit copies from the retained frozen
source, manifests and exact build recipes into fresh owned output. Do not replay
one-shot output paths blindly. File contents can be regenerated; old inode
numbers are not promised. The unused auxiliary proposal is intentionally
unneeded, not a missing source fix.

This is compiler-copy retirement after the P429 source delivery, not completion
of G3 or the full seven-part Goal. P427 EBADF, remote notification Base64,
remaining compiler/native and Tree-sitter work, final G1/G2 and native/GC
acceptance remain open. No already-passing original abort tests were rerun.
