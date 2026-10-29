# Stable 31.1 compatibility inputs

This is a version-bound test source corpus, not a second product source tree. The source is the public upstream mirror and the exact release commit recorded in sources.json. Upstream copyright headers and GPL-3.0-or-later licensing remain in force; see the repository COPYING.

The manifest lists 1536 original test-tree files (693 Lisp files), 20996952 bytes. 1440 inputs reuse byte-identical tracked repository files; 96 original overlays (9694153 bytes) use their original test/ paths. Every row binds its original Git blob, SHA-256, size, executable mode and repository-relative input. Overlay bytes are original inputs and are not edited to make tests pass. Raw overlays are ignored; the manifest and this provenance remain versioned. Existing payloads are retained until acquisition/integration acceptance and a documented recoverable cleanup pass.

`tools/reference-corpus.py` prepares an ignored persistent cache from complete verified local inputs or a streamed public archive pinned to the exact manifest commit. Every selected input must match its declared mode, size, Git blob and SHA-256; incomplete, changed, additional or missing cache inputs fail. Existing caches are verified before reuse, never silently replaced. A fresh checkout can obtain missing payloads without the optional reference checkout. Acquisition is source preparation, not a compatibility result.

`tools/stable-suite.py` materializes an isolated test tree from the mapped local inputs or that complete cache. Its CLI acquires the pinned corpus when mapped inputs are missing, or accepts an explicit verified `--corpus-root`. The pure manifest reader never performs network access. Neither path may access the ignored reference checkout, reuse another product's libraries or overwrite source/product files. Original Makefile/manual/fixture files remain corpus data; the corpus runner does not invoke the original build system. Normal production builds do not need this corpus.

Source-preparation commands:

```sh
python3 tools/reference-corpus.py --source archive
mkdir -p build/acceptance
python3 tools/stable-suite.py --manifest compatibility/stable-31.1/sources.json --out build/acceptance/stable-corpus --materialize
```

The whole-product snapshot exporter includes the same verified manifest inputs even when raw overlays are ignored or absent. Actual integration now verifies local cache creation/reuse, damaged-cache rejection, materialization, source export and acquisition in a new Git working tree with all96 overlays absent and no optional reference checkout. All1536 original files are recovered from the pinned public archive with their declared identities. This source-preparation evidence does not prove a committed fresh clone can build/run the complete editor; that delivery check remains pending. Existing mapped inputs are validated by Git executable mode without changing their physical permissions; new cache/materialized files use canonical0644/0755. No current raw input has been deleted.

File counts do not count ERT cases or prove compatibility. Final acceptance must discover actual registered test identities on the installed stable comparison product, retain feature/platform applicability, and report missing, skipped, load-error, resource-error and failed cases explicitly. Preparation of these source bytes does not mean any test has run.


## Installed-product source-test context

Original tests also use `source-directory`, C-source navigation, and resource
paths relative to `locate-library`. Execution prepares the separate
`source-context.json` selection: 302 original source-text/resource inputs,
27915270 bytes, at the same release commit. These texts are test fixtures;
they are not compiled into the editor or needed by a production build.
`tools/stable-context.py` acquires and verifies the pinned public archive in
`.femacs-cache/stable-context`, without consulting the optional reference tree.
The archive cache includes the complete original test selection as well; all
selected blobs, modes, sizes and hashes are checked before publication/reuse.

Each run materializes those context texts once and creates a distinct context
for each product. Its `lisp` link refers to that product's own libraries, and
its `test` link refers to the unchanged isolated corpus. The runner translates
that product's library search paths through this context and binds source/C
navigation there. Test data are not added to the shipped product, and the
original installation-directory/client selection and tests remain unchanged.
Context data, link targets, product/library identities, runner and provider
sources are verified around execution. A missing or changed context is an
error, never a reason to remove original tests or count a skip as a pass.

Actual setup integration on the installed comparison product and both owned
frontends' normal/checking builds passed 20 checks, followed by six focused
component protections and nineteen existing corpus contracts. This establishes
context preparation, library-relative/source-relative lookup and navigation;
it is not a complete original-file batch run, full stable compatibility or
GUI acceptance. Those required results remain separate.

A subsequent real batch-path check used four unchanged original entrypoints on
all five products:20 file processes completed,80 ERT cases yielded76 passed and
4 failed. The four are the preprocessor-simulation case on the owned products;
reference passes both semantic cases. Source navigation, tar-mode and ls-lisp
pass on every product. Paths and pinned C fixture data are present/unchanged;
the remaining candidate-side behavior/version cause is still under diagnosis.
These results do not complete the555-entrypoint/48-candidate matrix.
