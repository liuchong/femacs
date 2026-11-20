## CLN276 — 2026-11-20: Dock artwork packaging and installation

- Retired the two replaced bundles and the owned icon packaging work tree after verifying the installed and workspace copies (`14,593` files, `616` directories, `3` symlinks, `1,267,675,583` logical bytes).
- [Report and recovery recipe](reports/2026-11-20-276-macos-dock-artwork.md).

## CLN275 — 2026-11-20: Cocoa GIF decoder build and image checks

- Released the two owned Cocoa build roots and their caches after the signed package was installed and verified (`11,449` files, `691` directories, `14` symlinks, `629,350,888` logical bytes).
- [Report and recovery recipe](reports/2026-11-20-275-cocoa-gif.md).

## CLN274 — 2026-11-20: Cocoa default-font build and verification scratch

- Released the owned build and Cocoa object roots after verifying the signed bundle and the installed app (`5,947` files, `461` directories, `8` symlinks, `342,662,761` logical bytes).
- [Report and recovery recipe](reports/2026-11-20-274-cocoa-default-font.md).

## CLN273 — 2026-11-20: final retirement of `build/acceptance`, `build/runtime`, `.femacs-cache`, and staging backups

- Released: `8` owned scopes (`159,109` files, `21,542` directories, `1,843` symlinks; `10,763,439,608` logical bytes; normalized digest `a3a6af215b15d77a519e8e686e38ae429f879f1b38f957e49ee057b20c56bfe7`), covering `build/acceptance`, `build/runtime` (`native-2` & `frozen-2`), `.femacs-cache`, `dist/macos/.Femacs-before-p466.app`, `build/terminal/{image,image-staging}`, and `P-190` hardlinked `emacs-32.0.50.1{,.pdmp}` aliases.
- Kept: solely the verified macOS trial deliverables `build/terminal/port` (`40 MB`, `a0dd02e1…` / `cd076b1d…`, `pair.sha256` `4/4` OK) and `dist/macos/Femacs.app` (`231 MB`, `02f5adaf…` / `e1515db2…`, strict `codesign` `exit 0`), both re-verified after cleanup.
- [Report](reports/2026-11-20-273-final-acceptance-runtime-and-cache-retirement.md).

## CLN272 — 2026-11-20: consumed Stage III intermediate outputs, `native-collector-1`, and unit caches

- Released: `16` owned scopes (`41,534` files, `1,621` directories, `16` symlinks; `2,389,859,343` logical bytes; normalized digest `94241a9ec3d4adf9c1b447c9a3b1a9912116f07b7fd2ceb5858b38a2d2014ccc`), covering `build/runtime/native-collector-1` (after `g7-native-write-barrier` `3/3`), `build/cargo`, `dist/candidates`, `build/femacs`, `build/generation`, superseded Stage III attempt/diagnostic/unit directories, empty `xtask` unit parent directories, and the external session cache.
- Kept: delivered trial products (`build/terminal/port/src/{emacs,emacs.pdmp}`, `dist/macos/Femacs.app`), active `runtime-comparison-plan.json` candidates (`build/runtime/{native-2,frozen-2}`), and closed Stage I / Stage III receipts under `build/acceptance/` pending quiet-host formal `GA5–GA8` measurement and final reverse audit.
- [Report](reports/2026-11-20-272-consumed-stage3-gates-and-native-collector.md).

## CLN271 — 2026-11-20: Stage II (`GA4`) historical manifest disposition and superseded artifact cleanup

- Disposed of all `47` untracked local cleanup manifests (`2026-11-11..16`, `CLN227–CLN269`; `46,990,560` disk bytes / `221,956,192` uncompressed bytes, `0` personal paths) per `57101ad7` and `ai-storage`, recording per-file SHA-256, byte/line counts, paired tracked prose report (including newly committed [CLN269](reports/2026-11-16-269-superseded-single-product-gc-candidate.md)), and recovery routes.
- Retired `12` verified zero-consumer superseded build/release/scratch scopes (`54,538` files, `2,113` directories, `20` symlinks; `4,097,095,245` logical bytes), including `build/runtime/ime-diag-1`, `build/manuals`, `dist/releases`, superseded `dist/candidates/{g4-macos-2,g4-gc-root-trace-2,g4-android-8,g4-android-9}`, `build/macos-acceptance`, `build/acceptance/runtime-build-contracts`, and empty `build/*-unit` directories.
- Kept: Stage III runtime comparison inputs (`build/runtime/{frozen-2,native-2,native-collector-1}`), `dist/candidates/g4-android-10`, Stage I deliverables (`build/terminal/port/src/emacs`, `dist/macos/Femacs.app`), and active `build/acceptance/` receipts.
- [Report](reports/2026-11-20-271-historical-manifest-disposition-and-g4-reaudit.md).

## CLN270 — 2026-11-20: consumed Stage I clean-export, build staging, and intermediate probe outputs

- Released: `21` owned Stage I scopes (`48,154` files, `2,367` directories, `27` symlinks; `2,799,843,344` logical bytes; normalized digest `91b4a523e2abb473dcd906a107f4360a4c0cd8539e72b806ae7eb4cbaa76e893`), covering `s1-clean-export`, `s1-relocated`, superseded pre-Stage-I `build/terminal` and `dist/macos/Femacs.app` swap, `build/macos`, `.femacs-cache/macos`, `build/terminal/{config,lcms,load,objects,tls,treesit}`, intermediate Stage I acceptance runs, and session-cache scratch outputs.
- Kept: Stage I delivered products (`build/terminal/port/src/{emacs,emacs.pdmp}`, `dist/macos/Femacs.app`), closed Stage I receipts under `build/acceptance/` for final reverse-audit, pinned `tree-sitter 0.27.1` SDK prefix, and native helpers; untracked historical manifests remain untouched for Stage II (`GA4`) reconciliation.
- [Report](reports/2026-11-20-270-consumed-stage1-delivery-and-probes.md).

## 2026-11-18 — retire the cohort-5 matrix source tree

- Released: `build/acceptance/current-runtime-build-5/source` (20 GiB measured by free-space delta).
- Reason: cohort 5 was superseded by cohort 6, whose own source tree was retired earlier; both cohorts are historical prose in the docs and no tool reads them. The current channel is the single-product G3 evidence.
- Kept: cohort-5 metadata (products/receipts, cargo cache, builds) plus the closed stable-6 wrapper evidence directories.
- Consumers checked: `grep -rl current-runtime-build-5 tools xtask docs` matched only `docs/status.md` and `docs/current-state.md`.
- Disk effect: ~12 GiB free before this batch's retirements, 52 GiB after both.

## 2026-11-18 — retire the superseded 61-product build tree

- Released: `build/acceptance/current-runtime-build-6/source` (26 GiB measured by free-space delta; the tree reported 27 G before deletion).
- Reason: the cohort was superseded by the single-product G3 channel (`g3-555-frozen2`, `g3-subset-native-1`); no tool reads it, and the 48/61 matrix is not being rebuilt.
- Preserved before deletion: `build/acceptance/g3-historical-stable6/index.json` plus the completion/results/partial facts of the 7 closed stable-6 runs (111 KB), so their counts and failure facts stay readable.
- Consumers checked: `grep -rl current-runtime-build-6 tools xtask docs` matched only `docs/status.md` and `docs/current-state.md` (historical prose). The new G3 evidence does not reference this tree.
- Kept: `current-runtime-build-6` metadata (products.json, build result/receipts, cargo cache 55 MB, builds 213 MB) and every current G3 product and evidence directory.
- Disk effect: 3.3 GiB free before, 30 GiB after.

## 2026-11-18 — frozen batch close-out

- Released: `build/runtime/observer-1`, `observer-2`, `observer-3`, `webp-2` (4,385 MB total).
- Reason: superseded by `build/runtime/frozen-2` (cocoa collector+observer, source HEAD `9b692fe2`); G3 observer and image/webp acceptance already recorded, Tramp captures moved to `build/acceptance/tramp-frozen2`.
- Consumers checked: `grep -rl 'observer-[123]|webp-2' tools xtask docs` matched only historical prose in `docs/status.md` and the continuation ledger; no code or tool reads these trees.
- Kept: `build/runtime/frozen-2` (current product), `build/acceptance/g7-frozen2` (44K), `build/acceptance/tramp-frozen2` (840K), `build/acceptance/frozen-batch-20261118-index.json`.
- Rebuild boundary: every removed tree is reproducible from the recorded source identity plus the pinned tree-sitter SDK; no unique input is lost.

## CLN206 — 2026-11-03: continued units and finalized native inputs

- Publish closed duplicate-unit retirements: 26 legs, 127,838 files / 4,432 directories, 4,257,275,318 logical bytes and 4,507,582,464 prior per-path allocated bytes. Exclude CLN205's separately published scope.
- Preserve the failed preparation and missing completion; independently finalize 32 profiles / 608 signed inputs, retaining all 6,368 input witnesses. Canonically retire only 192 empty ABI environment directories, zero files or logical bytes.
- Real supplied-cohort integration and six memory-only rejection protections pass. Runtime actions, full stable-corpus compatibility and performance require their own evidence.
- [Report](reports/2026-11-03-206-continued-units-and-native-input-finalization.md); [compiler units](manifests/2026-11-03-206-retired-continued-compiler-units.jsonl.gz); [empty ABI state](manifests/2026-11-03-206-retired-native-input-state.jsonl.gz).

## CLN205 — 2026-11-03: explicit interrupted-product finalization

- Preserve the interrupted failed result and missing original receipts; strictly verify both complete product receipts without claiming the original compiler exit status.
- Retire only consumed duplicate units: 10,036 files / 344 directories, 335,691,785 logical bytes and 355,475,456 prior per-path allocated bytes; preserve 9,520 product witnesses.
- Explicitly bind the 18-configuration / 55-profile checkpoint in a successful pure-file successor plan. Five subsequent rejection protections pass with no negative fixtures or product changes.
- [Report](reports/2026-11-03-205-explicit-interrupted-product-finalization.md); [retired units](manifests/2026-11-03-205-retired-finalization-units.jsonl.gz).

## CLN204 — 2026-11-02: exact captured-input recovery

- Restore 19 paths / 4,881,808 original logical bytes: seven tracked data inputs and twelve byte-exact link archives; preserve 13 parent witnesses.
- Record new physical identities explicitly. Deleted historical inodes were not retained; original journals and strict readers remain unchanged, with required scoped recovery resolution in the successor consumer.
- Complete pure-file integration and five later rejection protections pass; negative metadata mutations create no fixtures. Compiler progress remains separately reported.
- [Report](reports/2026-11-02-204-explicit-captured-input-recovery.md); [recovered inputs](manifests/2026-11-02-204-restored-captured-inputs.jsonl.gz).

## CLN203 — 2026-11-02: explicit successor evidence storage and ownership

- Preserve 32 paths / 1,330,083,289 logical bytes; per-path allocation 1,330,151,424 -> 301,666,304 bytes, not exclusive volume reclamation. Logical evidence deletion zero.
- Verify original controller ownership and complete physical ancestry; preserve original content, private plans, historical receipts, parent witnesses and all runtime products.
- Separate verification, the existing preparation consumer, three inheritance rejections and three ownership rejections pass; retire 9 reconstructed metadata copies with private recovery recipes.
- Separately remove 1,772 ended build scratch files / 909 directories: 18,282,273 logical / 22,970,368 per-path allocated bytes, without crediting other volume changes.
- New-source checkpoint remains 6/20 configurations and 15/61 profiles; unchanged 2 GiB build admission.
- [Report](reports/2026-11-02-203-owned-successor-evidence-storage.md); [storage](manifests/2026-11-02-203-transparent-evidence-storage.jsonl.gz); [test metadata](manifests/2026-11-02-203-retired-test-metadata.jsonl.gz).
- [Ended build scratch](manifests/2026-11-02-203-retired-owned-build-scratch.jsonl.gz).

## CLN202 — 2026-11-02: transparent storage preserves ended evidence

- Preserve 24 original JSON paths and 1,582,003,973 logical bytes; explicit new physical/compression identities, unchanged content bindings and parent witnesses.
- Per-path allocation 1,582,059,520 -> 296,955,904 bytes; volume observations are not exclusive reclamation. Logical evidence deletion zero.
- Independent full verification and six later rejection protections pass. Retire 19 owned test/control files with exact identities and private recovery recipes; preserve real results and failures.
- New-source checkpoint remains 6/20 configurations, 15/61 profiles; future build admission remains 2 GiB.
- [Report](reports/2026-11-02-202-transparent-evidence-storage.md); [storage](manifests/2026-11-02-202-transparent-evidence-storage.jsonl.gz); [test metadata](manifests/2026-11-02-202-retired-test-metadata.jsonl.gz).

## CLN201 — 2026-11-02: native observer state and unbound port extent transition

- Retired 10,280 files / 370 directories: 367,926,110 logical / 388,587,520 per-path allocated bytes; original manual info, logs, receipts and recovery inputs remain.
- Product sharing preserves 709,392,619 logical bytes across 6,537 files; separate port sharing preserves 174 paths / 150,461,978 bytes and 104 directory identities/times.
- Preserve 40,636,590 raw operation bytes in 2,077,393 gzip bytes; logical evidence deletion zero. Five later rejection protections pass; four reconstructed test metadata copies retired with private recovery recipes.
- Full new build remains 6/20 configurations and 15/61 profiles. Per-path allocation is not exclusive reclaimed volume space.
- [Report](reports/2026-11-02-201-native-observer-state-and-port-extents.md); [retired state](manifests/2026-11-02-201-retired-build-state.jsonl.gz), [product extents](manifests/2026-11-02-201-product-extents.jsonl.gz), [port extents](manifests/2026-11-02-201-port-extents.jsonl.gz), [evidence encoding](manifests/2026-11-02-201-evidence-encoding.jsonl.gz), [test metadata](manifests/2026-11-02-201-retired-test-metadata.jsonl.gz).

## CLN200 — 2026-11-02: completed observer state and next product extent transition

- Retired 10,280 files / 370 directories: 367,182,152 logical / 387,403,776 per-path allocated bytes; canonical recovery evidence retained.
- Chained 6,471-file product transition preserves 507,330,411 logical bytes. The next consumer retains the accepted preparation ancestry functions unchanged.
- Two private streams preserve 40,355,388 raw bytes in 2,060,249 gzip bytes; logical evidence deletion zero. New build checkpoint remains 5/20 configurations and 13/61 profiles.
- [Report](reports/2026-11-02-200-observer-state-and-product-extents.md); [retired state](manifests/2026-11-02-200-retired-build-state.jsonl.gz), [product extents](manifests/2026-11-02-200-product-extents.jsonl.gz), [evidence encoding](manifests/2026-11-02-200-evidence-encoding.jsonl.gz).

## CLN199 — 2026-11-02: retained suite input and stable cache extent sharing

- Preserve 43,452 files / 1,145,978,058 logical bytes through explicit APFS identity transitions; old 21 suite trees and all input paths remain.
- Preserve 358 zero files, four receipts, 252 links and 6,316 directory identities/times. Prior allocation 1,258,553,344 bytes is not exclusive reclaimed space.
- Remove only one owned 6,407-byte failed temporary replica; retain false failures and recovery evidence. Four subsequent proof rejection protections pass.
- Preserve all 49,775,586 raw operation bytes in 7,909,112 gzip bytes; logical evidence deletion zero.
- [Report](reports/2026-11-02-199-retained-suite-input-extents.md); [input extents](manifests/2026-11-02-199-retained-input-extents.jsonl.gz), [temporary clone](manifests/2026-11-02-199-failed-temporary-clone.jsonl.gz), [evidence encoding](manifests/2026-11-02-199-evidence-encoding.jsonl.gz).

## CLN198 — 2026-11-02: completed baseline state and next product extent transition

- Retired 10,280 files / 370 directories: 367,106,186 logical / 387,309,568 per-path allocated bytes; canonical recovery evidence retained.
- Chained 6,471-file product transition preserves 481,384,731 logical bytes. Explicit inherited preparation proof passes integration and two subsequent rejection protections.
- Two private streams preserve 39,773,418 raw bytes in 2,049,359 gzip bytes; logical evidence deletion zero. Full new build remains pending at 4/20 configurations and 11/61 profiles.
- [Report](reports/2026-11-02-198-baseline-state-and-product-extents.md); [retired state](manifests/2026-11-02-198-retired-build-state.jsonl.gz), [product extents](manifests/2026-11-02-198-product-extents.jsonl.gz), [evidence encoding](manifests/2026-11-02-198-evidence-encoding.jsonl.gz).

## CLN197 — 2026-11-02: consumed preparation copy extent transition

- Preserved 805,406,600 logical bytes across 1,695 files; logical deletion zero. Prior allocation 809,152,512 bytes is not exclusive reclaimed space.
- Explicitly resolved 1,518 prior kept witnesses; 8,192 other witnesses, 43 donors/index files and all captured product resources remain unchanged.
- Four subsequent physical-proof rejection protections pass; private negative copies are removed. Full new build and G3 acceptance remain pending.
- [Report](reports/2026-11-02-197-consumed-preparation-extents.md); [manifest](manifests/2026-11-02-197-preparation-extents.jsonl.gz).

## CLN196 — 2026-11-02: ended manual source and next product extent transition

- Retired 25,680 files / 1,050 dirs: 1,416,519,418 logical / 1,470,062,592 per-path allocated bytes; canonical ledgers retain identities and recovery bounds.
- Chained 6,489-file product transition preserves 721,133,920 logical bytes; all previous transition witnesses remain unchanged.
- Two private streams preserve 41,689,706 bytes in 2,075,502 gzip bytes; logical evidence deletion zero. Numbered hardlink aliases and port dependencies remain unselected.
- [Report](reports/2026-11-02-196-ended-manual-source-and-product-extents.md); [retired state](manifests/2026-11-02-196-retired-build-state.jsonl.gz), [product extents](manifests/2026-11-02-196-product-extents.jsonl.gz), [evidence encoding](manifests/2026-11-02-196-evidence-encoding.jsonl.gz).

## CLN195 — 2026-11-02: ended build state and product extent transfer

- Canonical retirement: 17,958 files / 1,636 directories; 697,747,701 logical / 737,787,904 per-path allocated bytes.
- APFS transfer: 13,032 new-product files; 1,336,308,816 logical bytes preserved, explicit physical identity receipt and required next-consumer binding.
- Two private evidence streams: 80,999,973 original bytes preserved in 4,135,024 gzip bytes; logical evidence deletion zero.
- Old products, source inputs, raw semantic evidence, manuals info and sealed old matrix materializations remain; free-space changes are not exclusive reclaim measurements.
- [Report](reports/2026-11-02-195-build-state-and-product-extents.md); [retired state](manifests/2026-11-02-195-retired-build-state.jsonl.gz), [extent transitions](manifests/2026-11-02-195-product-extents.jsonl.gz), [evidence encoding](manifests/2026-11-02-195-evidence-encoding.jsonl.gz).

## CLN194 — identical historical staged source copies (2026-11-02)

Retired 17,992 regular files; zero directories or links. Every selected file
matches a retained frozen recovery file by relative path, mode, size and SHA-256:
1,226,005,702 logical / 1,267,666,944 per-path allocated bytes. Unique historical
versions, 29 special generated files, all build metadata and original evidence
remain. All 21 indexed materialization trees stay intact. No editor/compiler or
semantic test started; observed volume free-space changes are not attributed to
this cleanup alone.
[Report](reports/2026-11-02-194-redundant-staged-source.md).

## CLN193 — consumed unsigned-reader build and verification state (2026-11-02)

Canonical retirement: 9,516 files, 408 directories, zero links; 316,018,906 logical /
335,040,512 allocated bytes, 9,924 fully verified identity rows. Two generated unit
copies and six successful verification containers count once, including a reused
reference container retired through separate proxy evidence. Source/products/link
inputs, original raw outcomes, skipped/failed state and WIP remain; no volume gain
or full G3/performance claim.
[Report](reports/2026-11-02-193-unsigned-bignum-build-state.md).

## CLN192 — consumed runtime copies and successful scratch state (2026-11-02)

Five independently closed retirements: 190,723 files, 292 links, 27,430 directories;
6,600,904,230 logical / 7,015,309,312 per-path allocated bytes, not an additive
whole-volume free-space claim. All 218,445 decoded identity rows match retained
private evidence. Failed/skipped/incomplete calls and original retained receipts
remain unchanged; 1,101 successful scratch containers retired through separate
canonical proxy evidence. Original source/products, staged build evidence and
unrelated WIP remain. No new editor/compiler/test execution or compatibility pass.
[Report](reports/2026-11-02-192-consumed-runtime-state.md).

## CLN191 — 2026-11-01 — consumed FD diagnostic binaries

Removed 2 idle historical diagnostic binaries after exact identity/ownership,
no-open-handle and protection checks: 912,000 logical /917,504 allocated bytes;
0 directories or links. Retained Rust source, build recipe and diagnostic logs.
All selected paths absent; Git HEAD/index and unrelated WIP unchanged by cleanup.
No build/test launched. Separate stale-reviewer source/product deletion is not
counted or certified here; see the current-runtime source-removal incident.

[Report](reports/2026-11-01-191-consumed-fd-diagnostics.md) ·
[Manifest](manifests/2026-11-01-191-consumed-fd-diagnostics.jsonl.gz).

## CLN190 — consumed calendar/custom state

13,826 files/2,916 directories/18 links, 16,760 rows;
189,001,431 logical/225,824,768 allocated bytes.
Post-review 13,826/2,286/18; automatic 630 dirs/126 parents count once.
Nine test/context/environment groups and two adopted P450 helpers retired.
29 roots absent; 32,260 intent/completion rows. Evidence/inputs/products/source
preserved. Extra read-only audit compatibility error and corrected comparison
are recorded; cleanup's own checks pass and deletion is not repeated.

[Report](reports/2026-11-01-190-consumed-calendar-custom-state.md) ·
[Manifest](manifests/2026-11-01-190-consumed-calendar-custom-state.jsonl.gz).

## CLN189 — consumed message-count state and build caches

33159 files/3276 directories/18 links,36453 rows;
955443458 logical/1031352320 allocated bytes.
Post-review 33159 files/3186 dirs/18 links; automatic 90 dirs/18 parents count once.
20244 new object/unit files, 4 receipt-selected load/libgnu-port.a archives,
436 Cargo files,8 pinned copies/9 fixtures,case/native state and 2 component outputs,
4 prior-delivery helpers retired.52 roots absent,72726 operation rows.
190 evidence/46 inputs/18 profiles/7481 frozen inputs reverified.
Products/ports/resources/old object holds and all original failures remain.
Relink/capture_product needs rebuilding objects; no whole-volume free-space claim.

[Report](reports/2026-11-01-189-consumed-message-count-state.md) ·
[Manifest](manifests/2026-11-01-189-consumed-message-count-state.jsonl.gz).

## CLN188 — consumed owned-terminal state

14904files/3694directories/87links,18685rows;
239143513logical/278228992allocated bytes.
Post-review14757files/3340dirs/87links,237159052/275931136B;
automatic147files/354dirs/60parents,1984461/2297856B counted once.
Nine actual pinned test copies; later links counted separately.
All raw failures/events/logs/undo traces/source snapshots/controller receipts
and first-failure timestamp/content anchor remain; no product/App/WIP change.
First plan scope collision stopped before deletion; failure retained.

[Report](reports/2026-11-01-188-consumed-owned-pty-state.md) ·
[Manifest](manifests/2026-11-01-188-consumed-owned-pty-state.jsonl.gz).

## CLN187 — consumed editing-operation copies

13848files/2943dirs/18links,16809rows,189664650logical/226545664allocatedB.
Postreview13830/2376/18;auto18files/567dirs/108parents,616544/663552B count once.
Nine pinned copies,nine confirmed-empty retained simple trees,18context links,
four adopted helpers/two exact native handoff duplicates retired after review.
Original container dev/inode and occupancy checked,all pre-delete identities
sealed,32448intent/outcome rows. Original retained receipts/raw/evidence stay.
545kept files/34inputs/9fullproducts/HEAD/index/WIP unchanged;no build paths selected.
Postseal7.532473s excludes preflight;no inode/identical-helper-byte/df claim.
Reconstruct from pinned/native/signed sources;retain unique replay/failures/products.
[Report](reports/2026-10-31-187-consumed-current-editing-operations-state.md) ·
[Manifest](manifests/2026-10-31-187-consumed-current-editing-operations-state.jsonl.gz).

## CLN186 — consumed current syntax/faceup/EIEIO copies

13830files/2736directories/18links,16584rows,188993126logical/225824768allocatedB.
Postreview13830/2286/18 includes9pinned copies,18context links and6adopted
helpers/handoff copies;automatic450dirs/90parents count once,never deleted again.
Exact consumer exclusions include all receipt files;27selected roots absent.
Original cleanup exits1 at post-removal zip(strict=True) verifier;not rewritten.
Separate file-only finalization checks all rows/32268intent-outcomes/410evidence/
30inputs/9products,6.114807208s,no deletion/editor/compiler/tests. Raw,original
failure,private plans,replay,products,App and unrelatedWIP retained.
Reconstruction from pinned/native/signed sources;no original inode/df claim.
[Report](reports/2026-10-31-186-consumed-current-eieio-faceup-state.md) ·
[Manifest](manifests/2026-10-31-186-consumed-current-eieio-faceup-state.jsonl.gz).

## CLN185 — reporting duplicate copies and adopted handoffs

16files/1directory/17rows,179869logical/217088allocated bytes.9adopted
handoff copies retain exact native originals;3delivery helpers and4duplicate
readers consumed;one emptytools directory retired. No automatic rows/products.
Complete pre-delete plan,occupancy,fsynced ledger and operation outcomes;
32evidence files/source/oldWIP/HEAD/index unchanged. Unique baseline and
pre-protection source remain. Original inode/helper-byte recovery not promised.
[Report](reports/2026-10-31-185-consumed-own-voice-state.md) ·
[Manifest](manifests/2026-10-31-185-consumed-own-voice-state.jsonl.gz).

## CLN184 — consumed combined runtime and P447 relocation state

113190files/19923directories/49links,133162rows;
6859645810logical/7102935040allocated file bytes.
Post-review 41105/16116/38;
automatic 72085/3807/11,counted once.
Eight old case-parent rows lack identities;16ancestor attempts lack success;
old package mtime unavailable.New P447 parents have fresh pre-delete identities.
Retained raw/failures/private plans,7,459inputs,actual products and old WIP;
only consumed owned state/build copies/test payloads/fourhelpers selected.
Two preflights stopped before deletion.Original cleanup postcheck exit1 retained:
20receipt files/12dirs recreated exact bytes,new inodes;245682448/245719040B
restored.Full receipts now pass;gross counts include recreated paths.No rewrite
of history/App/performance claim.Recovery byte copies retired separately;minimal evidence/recipes retained.
[Report](reports/2026-10-31-184-consumed-combined-runtime-state.md) ·
[Manifest](manifests/2026-10-31-184-consumed-combined-runtime-state.jsonl.gz).
Supplemental14files/2dirs/16rows,153343584/153391104B;combined gross
113204/19925/49,133178rows,7012989394/7256326144B.

## CLN183 - consumed original package-vc state (2026-10-31)

After324 observed normal cases (300 ordinary+24 skips), retire nine exact
fixture/state/context roots and four consumed adopted P446 helpers only.
5422rows: 4612files/792directories/18links;
63,025,336logical/75,309,056allocatedB. No automatic rows;
three original owner receipts stay retained, not rewritten. Full pre-delete
identities/public gzip fsynced; occupancy/identity checked, no duplicates.
Raw false/events/logs/replay/private plan/products/HEAD/index/WIP retained,
20 evidence files unchanged; selected roots/helpers absent.
1.334829s is post-sealing deletion/verification, not whole cleanup time.
No editor/compiler rerun, performance/fullG3/App acceptance.
[Report](reports/2026-10-31-183-consumed-package-vc-state.md) ·
[Manifest](manifests/2026-10-31-183-consumed-package-vc-state.jsonl.gz).

## CLN182 - consumed original-package and GPG state (2026-10-31)

After scoped123-pass normal P435 integration and10 component protections,
retire6401files/1125directories/24links;7550rows;
87,646,066logical/104,558,592allocatedB. Three automatic plans include240files/
92dirs/6parents once; postreview6161files/1033dirs/24links,84038218/100454400B.
Four test copies,contexts,interrupted state,diagnostic homes,empty protection
state and four adopted P445 helpers only. Full pre-delete identities/manifest
fsynced; exact occupancy/identity checks. Twelve roots/four helpers absent.
Preserve41 evidence files,timeout/diagnostic/success/events,private plans,
drivers,products/App,HEAD/index/WIP. Ordinary bounded file op exit0;
1.484s measured deletion/verification after sealing,not whole cleanup duration.
No editor/compiler rerun; no performance/fullG3/App acceptance.
[Report](reports/2026-10-31-182-consumed-package-state.md) ·
[Manifest](manifests/2026-10-31-182-consumed-package-state.jsonl.gz).

## CLN181 - consumed marking-proof state (2026-10-31)

After P445 source/integration/component/descriptive review: **10,290 files,
3,885 directories, no links**, **404,732,539 / 425,656,320** logical/allocated
bytes; 14,175 rows, 557 directory scopes plus six exact files. Retire own cargo,
objects/generated units, 554 process state groups, protection source/binary,
consumed owner archive and three CLN180 helpers. No shared cache deletion.
Retain 3,349 verified evidence files, 7,447 frozen inputs, current/prior products,
raw logs/observations and original P444 failures. HEAD/index/products/App stay.
Nine old WIP contents match frozen originals: six whole files and three document
bodies after excluding only this round's prefixes. Ordinary bounded file operation
exit0 in169.365s, no editor/compiler; no speedup/App/G5/G7 acceptance.
[Report](reports/2026-10-31-181-consumed-mark-block-state.md) ·
[Manifest](manifests/2026-10-31-181-consumed-mark-block-state.jsonl.gz).

## CLN-20261031-180 — consumed Lisp structures and motion state

Independent review accepts354 ordinary observations from18 unique normal P435
file processes:4 original/107 reused plus14 resumed/247 new, without replay.
Original count-assertion false and resume-preflight false remain unchanged.
Retire4,610 files/942 directories/18 links;63,002,646 logical/75,280,384 allocatedB.
Post-review4608/762/18; automatic0 files/180 directories including36 parents,
counted once; two consumed helpers11,790/16,384B. No generated/trampoline file.
All selected roots/helpers absent;5,570 manifest rows fully match private plans.
Original raw/preflight/resume/events/logs, products, inputs, HEAD/index and WIP
remain unchanged. No shared purge, whole-volume claim, production or App change.
[Report](reports/2026-10-31-180-consumed-lisp-structures-state.md) ·
[Manifest](manifests/2026-10-31-180-consumed-lisp-structures-state.jsonl.gz).

## CLN-20261029-126 — 已消费GIO诊断上下文

三轮已结束诊断的六个重复上下文根已回收：930文件/链接、34目录、83745810逻辑字节。原09失败、原输入、真实产品及全部原始记录保留；当前51项回归上下文独立，保留文件哈希不变。旧上下文重放须先再生。[逐项身份与恢复](reports/2026-10-29-126-consumed-gio-diagnostic-contexts.md)。

## CLN-20261029-125 — 已替代批处理事件候选

旧P417四候选已无消费者，回收两个生成产品/对象/资源根：19330文件/链接、716目录、1269578620逻辑字节。唯一冻结源码、原输入、失败/构建日志和24份产品元数据保留，14767个保留文件哈希不变；当前产品不受影响，旧程序需重建才能重放。[身份与恢复边界](reports/2026-10-29-125-superseded-batch-event-products.md)。

## CLN-20261029-124 — 已消费GIO分段修复编译副本

四产品构建与两个Cocoa严格签名完成后，仅回收五个Cargo/生成unit根：19910文件、903目录、703002744逻辑字节。真实源码、对象、程序/dump和失败证据保留且所选身份不变，正在运行的原通知验收不消费这些缓存；功能通过待返回。[逐项身份与恢复](reports/2026-10-29-124-consumed-gio-stream-compiler-copies.md)。

## CLN-20261029-123 — 已消费通知源码上下文副本

七轮验收/校准/诊断均退出并回收后，仅清14个重复上下文根：2216文件与链接、87目录、195406890逻辑字节。原脚本、输入、全部失败/跳过、捕获数据与真实源/产品/回执保留，当前P423构建不依赖这些旧副本；逐项身份及恢复见[报告](reports/2026-10-29-123-consumed-notification-context-copies.md)。

## CLN-20261029-122 — 已消费稳定通知编译副本

四产品/签名及有界功能完成后，回收五个Cargo/生成unit根：19910文件、903目录、703002744逻辑字节。冻结源、实际对象/产品、原始失败和复核输入保留；155例110通过45原跳过0失败不冒充全G3。[逐项清单与恢复](reports/2026-10-29-122-consumed-stable-notify-compiler-copies.md)。

## CLN-20261029-121 — 已消费Base64诊断上下文副本

两次有界诊断结束、制品不变且完整回收后，清理四个重复测试上下文根：616文件/链接、22目录、55830540逻辑字节。原失败、日志、输入和实际产品保留；不能把未复现当已修复。[逐项身份与恢复](reports/2026-10-29-121-consumed-base64-context-copies.md)。

## CLN-20261029-120 — 文件通知稳定组件成对对齐

完整库与配套root测试对齐固定stable31.1，仅一行注释规范，必需语料/断言不改。四产品及本地通知65例60通过5原跳过0失败，原八例已通过；独立复核接受限定本地交付。[退役身份与成对恢复](reports/2026-10-29-120-stable-filenotify-component.md)，远程/全G3仍待完成。

## CLN-20261029-119 — 第四轮已完成构建的编译副本

四产品和620例完成后，为补齐整组测试硬截止主动结束旧驱动，完整回收并核验26099捕获文件不变；仅清理用完的Cargo/生成单元19910文件、903目录、703116947逻辑字节，源、对象、四产品及原始结果保留。测试单独有界续接，无需重编。[逐项记录与恢复](reports/2026-10-29-119-completed-notify-compiler-copies.md)。

## CLN-20261029-118 — 第三轮通知整合已消费编译副本

第三轮终端两腿构建完成、Cocoa重复标志编译失败，控制器exit1/完整回收；清理19662文件、897目录、697483002逻辑字节。源核验通过、终端程序/dump和原始失败保留，第四轮独立构建继续。[逐项记录](reports/2026-10-29-118-failed-cocoa-notify-compiler-copies.md)。

## CLN-20261029-117 — 第一轮通知后端失败编译副本

找回并核对首轮控制器exit1/完整回收后，清理Cargo/两腿生成单元9794文件、555目录、368026198逻辑字节；源、对象和原始失败保留。[逐项清理与恢复](reports/2026-10-29-117-failed-notify-compiler-copies.md)。

## CLN-20261029-116 — 第二轮通知后端失败编译副本

仅回收已终止第二轮的Cargo/两腿生成单元：9794文件、555目录、368026934逻辑字节；冻结源、对象和原始失败保留，第三轮独立构建不受影响。[报告与恢复](reports/2026-10-29-116-failed-notify-compiler-copies.md)。

## CLN-20261029-115 — P417已消费编译缓存

回收五个已结束运行的Cargo/生成单元副本：19694文件、899目录、694904903逻辑字节；保留源码、实际对象、产品和原始失败，不把exit1冒充完整验收成功。[逐项清理与恢复](reports/2026-10-29-115-consumed-batch-event-compiler-copies.md)。

- CLN-20261029-114：ERT库与配套root测试原样对齐稳定31.1，必需验收原文不变；开发版diff UI及测试的Git身份和成对恢复见[报告](reports/2026-10-29-114-stable-ert-component.md)。620例及独立源核对接受限定组件交付；完整兼容仍未通过。

- CLN-20261028-113：kqueue旧局部事件号及三份重复open标志声明统一为共享定义，未删除文件；源码和恢复身份见[报告](reports/2026-10-28-113-event-kind-declaration.md)。新后端接线待实际验收。

- CLN-20261028-112：全量Lisp发行构建已消费Cargo/生成副本9802文件、364815946逻辑字节；冻结源、真实程序/资源、对象、日志和功能回执保留。[报告](reports/2026-10-28-112-consumed-release-bytecode-compiler-copies.md)。

- CLN-20261028-111：六轮已消费字节码诊断副本11908文件、672509897逻辑字节；原始失败、脚本、编译回执、源和正式构建保留。[报告](reports/2026-10-28-111-consumed-bytecode-diagnostics.md)。

- CLN-20261028-110：三轮语法表发行构建已消费的Cargo/生成副本39298文件、1424038924逻辑字节；源码、真实产品/对象、失败及测试证据保留。[报告](reports/2026-10-28-110-consumed-parser-compiler-copies.md)。

- CLN-20261028-109：首轮已消费失败构建的生成树与手册3936文件、290356335逻辑字节；独有源码、失败证据和后续在用版本保留。[报告](reports/2026-10-28-109-consumed-failed-client-build.md)。

- CLN-20261028-108：客户端修复复核后的已消费Cargo/四配置生成副本19694文件、694568260逻辑字节；新鲜完整产品、源/对象、失败及验收证据保留。[报告](reports/2026-10-28-108-consumed-client-compiler-copies.md)。

- CLN-20261028-107：第二轮客户端已消费Cargo与四配置生成副本19694文件、694568188逻辑字节；当前构建、原源码/产品/对象和失败证据保留。[报告](reports/2026-10-28-107-consumed-client-compiler-copies.md)。

- CLN-20261028-106：首轮客户端失败构建已消费的Cargo/生成副本5119文件、209942304逻辑字节；源码、程序和失败证据保留，第二轮独立构建继续。[报告](reports/2026-10-28-106-consumed-client-compiler-copies.md)。

# 清理台账

完整字段和记录方法见 [工程档案入口](README.md)，每行链接一份报告。本表不是产品更新日志；同批详细清单/恢复方案由报告维护。日期不含未经记录的精确时刻。前六条于2026-10-24补齐归档；后续批次逐条记录实际动作。

| 编号 | 操作日期 | 类别与结果 | 关联动作/既有证据提交 | 报告 |
| --- | --- | --- | --- | --- |
| CLN-20261016-01 | 2026-10-16 | tracked，九个旧构建脚手架删除 | `245fd341f81c86b6765133b7f333720fb5a52a65` | [依据/9项blob/恢复](reports/2026-10-16-01-autotools.md) |
| CLN-20261022-01 | 2026-10-22 | ignored产物，19个目标范围，10452项；仅有范围级汇总 | `e125bb725faad5737aff97f73366aff70dce147c`，不是Git删除提交 | [旧清单及删后证据](reports/2026-10-22-01-macos-artifacts.md) |
| CLN-20261024-01 | 2026-10-24 | tracked，六项旧工具/实验源码删除 | `2415a335836d0ad81de2564a82301bdceefb8145` | [6项身份/替代/恢复](reports/2026-10-24-01-obsolete-tools.md) |
| CLN-20261024-02 | 2026-10-24 | 零删除检查，0文件/0字节 | `c119748bfa6cb9d5ac39bde602c22702623fa44c`已有摘要 | [范围/未删除事实](reports/2026-10-24-02-export-cache-inspection.md) |
| CLN-20261024-03 | 2026-10-24 | 未提交实验撤回：三文件恢复，一文件删除 | `c119748bfa6cb9d5ac39bde602c22702623fa44c`保存最终状态，原实验无提交 | [4项身份/惰性补丁](reports/2026-10-24-03-dispatch-experiment.md) |
| CLN-20261024-04 | 2026-10-24 | ignored产物，685项；逻辑187264824字节 | `c119748bfa6cb9d5ac39bde602c22702623fa44c`已有摘要，不是Git删除提交 | [逐项清单/重建](reports/2026-10-24-04-generated-artifacts.md) |
| CLN-20261024-05 | 2026-10-24 | 未提交测试收敛：删除重复启动函数/调用，保留统一入口 | 本轮交付；无旧Git blob，前后内容哈希及惰性补丁绑定 | [范围/替代/恢复](reports/2026-10-24-05-startup-harness.md) |
| CLN-20261024-06 | 2026-10-24 | ignored产物，首次失败导出的剩余对象及Cargo缓存 | 本轮交付；逐项删前SHA-256、字节与删后状态见清单 | [范围/重建/未查明原因](reports/2026-10-24-06-build-artifacts.md) |
| CLN-20261025-01 | 2026-10-25 | 未提交诊断候选收敛及导出内生成包替换；保留原始构建证据 | 本轮交付；六份源文件前后身份、生成物身份及惰性恢复补丁 | [范围/替代/恢复](reports/2026-10-25-01-diagnostic-candidate.md) |
| CLN-20261025-02 | 2026-10-25 | 验收断言、TTY制品身份及参照输出保护候选收敛 | 本轮交付；两工具前后身份、独立恢复补丁与操作回执 | [范围/替代/恢复](reports/2026-10-25-02-acceptance-guards.md) |
| CLN-20261025-03 | 2026-10-25 | 探针/L5/linker/文档策略替换，实际D探针及修复后inline验收通过 | 源基线01bfbb7f；逐项隔离Git恢复通过，实现5118ed08已签名推送；非整产品回滚 | [身份/替代/恢复](reports/2026-10-25-03-probe-tooling.md) |
| CLN-20261025-04 | 2026-10-25 | Android构建/设备策略替换，实际正常/checking构建及API36设备验收通过 | 逐项隔离Git恢复通过，实现5118ed08已签名推送；具体范围见报告 | [身份/替代/恢复](reports/2026-10-25-04-android-tooling.md) |
| CLN-20261025-05 | 2026-10-25 | API21以下死分支及一旧类移除，保留现代路径/原始资源 | 隔离Git恢复、七值源码方法矩阵及API36验收通过；5118ed08已签名推送，不称API21设备验收 | [身份/替代/恢复](reports/2026-10-25-05-api21-floor.md) |
| CLN-20261025-06 | 2026-10-25 | 两份历史生成profile删除，源探针/结论保留 | 两份原Git内容隔离恢复/哈希通过，5118ed08已签名推送；不构成性能验收 | [身份/替代/恢复](reports/2026-10-25-06-raw-profiles.md) |
| CLN-20261025-07 | 2026-10-25 | 两份已关闭私有原始协作记录移至项目外持久归档，字节/哈希保留；无源码/记录删除、无同卷空间释放声明 | 两个内容身份及私有本机恢复台账；不入库原始对话/会话定位 | [身份/边界/恢复](reports/2026-10-25-07-private-log-archive.md) |
| CLN-20261025-08 | 2026-10-25 | 五个已停止失败设备尝试的114个可重建AVD文件；保留owner、回执与日志 | 逐项内容哈希/逻辑字节/私有精确路径台账；不声称原瞬态磁盘可逐字恢复或实际卷空间释放量 | [身份/边界/重建](reports/2026-10-25-08-owned-avd-disks.md) |
| CLN-20261025-09 | 2026-10-25 | 后续五个已停止设备尝试的61个可重建AVD文件；保留已通过的batch证据及全部失败日志 | 逐项内容哈希/逻辑字节/私有精确路径台账；仅重建新设备，不伪称恢复瞬态状态或卷空间释放量 | [身份/边界/重建](reports/2026-10-25-09-owned-avd-disks.md) |
| CLN-20261025-10 | 2026-10-25 | 两个已停止GUI失败尝试的可重建AVD文件；保留CheckJNI日志、截图、回执及产品身份 | 逐项内容哈希/字节/私有精确路径台账；只清本次设备，恢复方式为重新创建 | [身份/边界/重建](reports/2026-10-25-10-owned-avd-disks.md) |
| CLN-20261025-11 | 2026-10-25 | 5368个已退出自有构建的缓存/对象、中间APK及设备拉回副本；保留当前制品、旧通过batch所用APK、源码图和所有回执 | 逐项身份/字节/打开句柄检查；保护制品哈希不变；可重编不承诺缓存/旧签名字节复现或实际卷空间释放量 | [身份/边界/重建](reports/2026-10-25-11-idle-build-products.md) |
| CLN-20261025-12 | 2026-10-25 | 四份闲置旧构建图的20030个展开文件移入私有持久压缩归档；原项目源码、当前Android/Mac图和证据不变 | 所有归档成员逐项哈希复验，四份源码实际隔离恢复；保留精确路径/归档身份台账 | [身份/边界/恢复](reports/2026-10-25-12-generated-source-archives.md) |
| CLN-20261025-13 | 2026-10-25 | 后续两个已停止设备尝试的28个可重建设备文件；保留文件创建失败日志及全部回执 | 逐项哈希/字节/私有精确路径；已核对归属和退出，只能重建新设备、不伪称恢复瞬态状态 | [身份/边界/重建](reports/2026-10-25-13-owned-avd-disks.md) |
| CLN-20261025-14 | 2026-10-25 | 三个已停止旧Android源码导出/生成构建副本的44677个普通文件转私有持久归档；原项目源码、新候选与Mac产品保留 | 每成员内容校验及三份实际恢复；既有符号链接保留且不遍历；硬链接按内容校验，旧失败不改写 | [身份/边界/恢复](reports/2026-10-25-14-superseded-source-exports.md) |
| CLN-20261025-15 | 2026-10-25 | 四个旧隔离副本和两个旧生成图的70435个普通文件持久归档；当前源码、候选、G4产品及根试用包保留 | 每成员内容核对及六份实际恢复；保留符号链接及所有外层日志/回执，不承诺逻辑计数等于卷空间变化 | [身份/边界/恢复](reports/2026-10-25-15-prior-build-archives.md) |
| CLN-20261025-16 | 2026-10-25 | 11942个闲置编译缓存、旧展开资源副本与新包中间件；保护当前APK/SO及旧batch包身份不变 | 逐项哈希/字节和打开句柄检查；保留源/对象/日志，按归档配方及输入身份重建 | [身份/边界/重建](reports/2026-10-25-16-idle-build-products.md) |
| CLN-20261025-17 | 2026-10-25 | 四个闲置旧host工具/日志/生成图子树的4714个普通文件持久归档；当前产品与运行资源保留 | 全成员内容核对、四份实际恢复及符号链接保留；日志字节仍可恢复，不以逻辑量冒充卷空间释放 | [身份/边界/恢复](reports/2026-10-25-17-idle-host-archives.md) |
| CLN-20261025-18 | 2026-10-25 | 两份旧Mac工具树与两个旧app备份的7307个普通文件持久归档；当前试用包/G4产品不动 | 全成员核对、四份实际恢复；不把文件内容恢复当成扩展属性/签名或运行验收 | [身份/边界/恢复](reports/2026-10-25-18-old-product-archives.md) |
| CLN-20261025-19 | 2026-10-25 | 已完成新构建中21343个非产品/JSON/日志普通文件归档；APK、普通/checking库保持原路径和哈希 | 全成员核对、代表成员实际恢复、产品保护复验；原项目源码和实际设备输入均保留 | [身份/边界/恢复](reports/2026-10-25-19-current-build-graph.md) |
| CLN-20261025-20 | 2026-10-25 | 仅回收已退出device12的23个可重建AVD文件；保留CCL/JSON实际失败、截图、观察与安装身份 | 全部逐项哈希及归属/退出核对；原客体瞬态状态不承诺恢复 | [身份/边界/重建](reports/2026-10-25-20-owned-avd-disks.md) |
| CLN-20261025-21 | 2026-10-25 | 两个旧tracked-only缓存导出的41733个普通文件持久归档；主仓源码与当前Mac/终端/Android产品保持原路径和哈希 | 每成员核对、两份实际恢复、符号链接保留；原外部参照/SDK和其他任务不动 | [身份/边界/恢复](reports/2026-10-25-21-previous-cache-exports.md) |
| CLN-20261025-22 | 2026-10-25 | 四个旧工具/编译缓存根下16648个普通文件持久归档；主源码、当前产品和现用隔离构建树保留 | 全成员核对、四份实际恢复、保护产品哈希不变；精确恢复路径私有保留 | [身份/边界/恢复](reports/2026-10-25-22-legacy-tool-caches.md) |
| CLN-20261025-23 | 2026-10-25 | 两次已停止的设备13运行下46个AVD临时文件移除；逻辑1345161422字节，删除前分配量1222246400字节 | 保留产品、失败/成功回执、截图、日志及归属根；临时客户机状态仅能按配方重建，不宣称精确恢复或卷释放量 | [身份/边界/恢复](reports/2026-10-25-23-owned-avd-disks.md) |
| CLN-20261025-24 | 2026-10-25 | 四轮已停止的GC诊断设备15–18下92个AVD临时文件移除；逻辑2995327288字节，删除前分配量2780037120字节 | APK、原生库、完整/截断诊断记录、失败回执和截图保留；仅重建临时客体，不假称精确恢复或实际卷释放量 | [身份/边界/重建](reports/2026-10-25-24-owned-avd-disks.md) |
| CLN-20261025-25 | 2026-10-25 | 两个已停止诊断/生产验收设备的46个可重建AVD文件；保留失败回执、日志、截图与产品 | 逐项哈希及私有路径台账；逻辑1402439900字节，删前分配1281687552字节，不等于实际卷释放量 | [身份/边界/重建](reports/2026-10-25-25-owned-avd-disks.md) |
| CLN-20261025-26 | 2026-10-25 | 已停止通过验收设备的23个可重建AVD文件；保留安装身份、11项流程证据及全部截图/日志 | 逐项哈希及私有路径台账；逻辑670614670字节，删前分配611123200字节，不等于实际卷释放量 | [身份/边界/重建](reports/2026-10-25-26-owned-avd-disks.md) |
| CLN-20261025-27 | 2026-10-25 | 两个已停止运行时尝试D6/D8的44626个生成单元/对象/编译缓存成员持久归档，保留原冻结输入、全部日志/JSON与真实程序/dump；D9及当前试用包排除 | 全成员内容核对、每树代表性Rust/对象/归档实际恢复；1510092614逻辑字节、1601650688删前分配字节，归档388178063字节；2868项保护哈希不变，不冒充实际卷释放量 | [身份/边界/恢复](reports/2026-10-25-27-runtime-intermediates.md) |
| CLN-20261025-28 | 2026-10-25 | 已停止D9/D10中33725个生成单元/对象/缓存成员持久归档，保留原冻结输入、日志/JSON、程序/dump、资源及独立工具；另移除两份内容已被替代的准备归档 | 全成员校验与代表性实际恢复；1228855175逻辑字节、1297944576删前分配字节，最终归档321379789字节，2484项保护哈希不变；两份旧准备归档315858197字节均先验证全部内容有替代；不冒充卷空间/硬链接拓扑恢复 | [身份/边界/恢复](reports/2026-10-25-28-runtime-intermediates.md) |
| CLN-20261025-29 | 2026-10-25 | 已停止D11中9883个生成单元/对象/缓存成员持久归档；原冻结输入、两分支链接失败日志、JSON、程序/资源及独立工具保留，活动D12排除 | 全成员校验及代表性实际恢复；437888198逻辑字节、458297344删前分配字节，归档116517819字节，7433项保护哈希不变；不冒充独占卷空间释放或产品修复 | [身份/边界/恢复](reports/2026-10-25-29-runtime-intermediates.md) |
| CLN-20261026-30 | 2026-10-26 | 已停止D12的4717个生成源码副本、对象及编译缓存转持久归档；原始冻结输入、日志、配置、程序/dump、D13及试用包保留 | 全成员哈希、代表成员实际恢复、7295项保护内容复验；不承诺卷空间变化 | [身份/边界/恢复](reports/2026-10-26-30-runtime-intermediates.md) |
| CLN-20261026-31 | 2026-10-26 | 已停止D13的9433个生成单元副本、对象与编译缓存持久归档；原始冻结输入、日志、程序/dump和当前D14保留 | 全成员哈希、代表成员实际恢复、7596项保护内容复验；精确路径台账保留 | [身份/边界/恢复](reports/2026-10-26-31-runtime-intermediates.md) |
| CLN-20261026-32 | 2026-10-26 | 回收295267个旧编译物/重复副本，逻辑14364408836字节 | 逐项哈希/归属/句柄检查，8762份保护内容不变；D14/D18/D20和现用产品排除 | [身份/恢复/边界](reports/2026-10-26-32-unused-artifacts.md) |
| CLN-20261026-33 | 2026-10-26 | 回收48276个被取代的G4试验副本/编译物，逻辑1920342202字节 | 3382份保护内容不变；当前G4候选、恢复产品及运行时矩阵排除 | [身份/恢复/边界](reports/2026-10-26-33-old-g4-copies.md) |
| CLN-20261026-34 | 2026-10-26 | 回收7份D6/D8至D13旧编译缓存归档，逻辑947780933字节 | 原台账哈希不变；撤销这些包的精确恢复承诺，保留源/配置按需重建 | [身份/重建/更正](reports/2026-10-26-34-obsolete-cache-archives.md) |
| CLN-20261026-35 | 2026-10-26 | 回收6327个旧App/字节码/手册/打包残留，逻辑707190095字节 | 逐项哈希与空目录清单，原输入/日志/符号链接/现用候选排除 | [身份/重建/边界](reports/2026-10-26-35-retired-packages.md) |
| CLN-20261026-36 | 2026-10-26 | 回收已退出D14/D18/D20/D21矩阵411156个生成物/重复资源，逻辑23760244161字节 | 全部目标消失、67973项保留内容不变；冻结原输入/日志/现用候选保留，仓库现17G | [身份/恢复/边界](reports/2026-10-26-36-idle-matrices.md) |
| CLN-20261026-37 | 2026-10-26 | 回收停止D22的13379个生成物/重复资源，逻辑956847680字节 | 全部目标消失、14266项保留内容不变；新D23/当前试用包与原始源/记录排除 | [身份/恢复/边界](reports/2026-10-26-37-consumed-d22.md) |

| CLN-20261026-38 | 2026-10-26 | 回收55配置的164911个闲置搬移副本，逻辑10911717416字节 | 110925个原件内容不变；保留真实失败、现用包和6个不同文件，逐项清单可追溯 | [身份/恢复/边界](reports/2026-10-26-38-relocation-copies.md) |

| CLN-20261026-39 | 2026-10-26 | 回收40编译配置187700个已消费Rust生成副本，逻辑6256833936字节 | 136048个原始源/产品/链接输入及回执内容不变；保留仍有消费者的D23矩阵，补齐逐例/逐轮清理出口 | [身份/重建/边界](reports/2026-10-26-39-generated-rust.md) |

| CLN-20261026-40 | 2026-10-26 | 两个真实baseline搬移用例消费5979个包副本后逐例回收，逻辑323567463字节 | 原件/fixture身份不变、源码拒绝和签名通过，包外逐项清单与实际生命周期回执保留 | [身份/恢复/边界](reports/2026-10-26-40-per-case-relocation.md) |

| CLN-20261026-41 | 2026-10-26 | 两个真实native搬移用例消费6017成员后逐例回收，逻辑424945610字节 | 实际源码/后端拒绝、native编译加载与身份、签名通过，原件/fixture不变 | [身份/恢复/边界](reports/2026-10-26-41-native-relocation.md) |

| CLN-20261026-42 | 2026-10-26 | 退役已消费D23的160845个生成物/重复资源及冗余归档，逻辑17175176376字节 | 43947个保留内容不变，源码/原始回执/三个代表dump保留，明确CLN38/40/41原包恢复改经已验证内容归档 | [身份/恢复/边界](reports/2026-10-26-42-consumed-d23.md) |

| CLN-20261027-44 | 2026-10-27 | 在全量构建继续期间回收12个已完成配置的110532个生成Rust副本，逻辑3672661468字节 | 原配置均已退出并交接；84853个保留内容不变，当前配置和在用出口不动；精确清单先持久化后删除，公开逐项ID与本机精确路径台账对应 | [身份/重建/边界](reports/2026-10-27-44-consumed-d24-compiler-copies.md) |

| CLN-20261026-43 | 2026-10-27 | 新D24全量构建成功退出后自动回收其余77168个生成Rust副本，逻辑2582280788字节；编号前日预留 | 136061个保留内容不变，原包装器核对源/产品/清单身份；与CLN44无重复，公开逐项ID映射至本机精确台账 | [身份/重建/边界](reports/2026-10-26-43-generated-rust.md) |

| CLN-20261027-45 | 2026-10-27 | 退役9个旧快照中的62316份逐字节重复源副本，逻辑3190145253字节 | 6835份恢复源哈希不变，517份独有源及原日志保留；旧快照改为差量、须按原清单补齐后运行，当前D24及试用包不动 | [身份/恢复/边界](reports/2026-10-27-45-duplicate-snapshots.md) |

| CLN-20261027-46 | 2026-10-27 | 回收被后继准备归档完整覆盖的旧D24内容归档，逻辑590461454字节 | 旧10770个成员均在保留14015成员包内，所有成员流式验证；原回执不变，恢复路径明确转交后继包 | [身份/恢复/边界](reports/2026-10-27-46-superseded-archive.md) |

| CLN-20261027-47 | 2026-10-27 | 回收41216份成功进程临时文件及一份完整被覆盖的中途快照，逻辑1479334370字节 | 6110个case内容逐项比较，6120份保留结果/输入身份不变，4个失败现场保留；没有重跑产品 | [身份/恢复/边界](reports/2026-10-27-47-consumed-runtime-state.md) |

| CLN-20261027-48 | 2026-10-27 | 回收40次已消费手册导出的15240份重复源文件，逻辑999739600字节 | 对应381份冻结原输入，8744份保留内容哈希不变；Info、回执、日志及产品资源均保留 | [身份/恢复/边界](reports/2026-10-27-48-manual-input-copies.md) |

| CLN-20261027-49 | 2026-10-27 | Reclaimed3072 consumed pilot input copies and1 generated cache;42028000 logical bytes | Original1536 inputs and69 raw evidence files unchanged; live full followup untouched | [Identity/recovery/scope](reports/2026-10-27-49-consumed-pilot-inputs.md) |

| CLN-20261027-50 | 2026-10-27 | Retired179958 consumed relocation members over60 profiles,11962942294 logical bytes | Every work copy absent, raw ledger SHA verified; originals retained. Executed old fsync gap disclosed; later P385 repair separate | [Identity/recovery/scope](reports/2026-10-27-50-d25-relocation.md) |

| CLN-20261027-51 | 2026-10-27 | P385 filesystem integration removes3 owned fixture members,648 logical bytes | External sentinel unchanged; real sync-error/open-FD controls retained. Four integration and2 subsequent unit cases pass | [Identity/recovery/scope](reports/2026-10-27-51-p385-fixture.md) |

| CLN-20261027-52 | 2026-10-27 | Retired6 superseded Android APK/library files,263789634 logical bytes | Current version10 and Mac programs/old identity receipts unchanged; no new product test | [Identity/recovery/scope](reports/2026-10-27-52-superseded-android.md) |

| CLN-20261027-53 | 2026-10-27 | Retired11963 consumed diagnostic/relocation App files,615402920 logical bytes | Original logs/receipts and current trial/release Apps preserved; explicit rebuild/copy restoration | [Identity/recovery/scope](reports/2026-10-27-53-consumed-app-copies.md) |

| CLN-20261027-54 | 2026-10-27 | 回收11632份已消费dump/native镜像与负面夹具副本，1230580584逻辑字节 | 7144项保留内容哈希不变，原始准备夹具和当前候选不动；恢复须先生成独占工作制品 | [身份/恢复/边界](reports/2026-10-27-54-consumed-case-artifacts.md) |

| CLN-20261027-55 | 2026-10-27 | 回收127070份已被新源码取代的旧候选产品/资源/链接展开文件，11435235140逻辑字节 | 14015成员恢复包逐内容核对，20011项保留内容不变；旧D24路径退役，新G3须重建 | [身份/恢复/边界](reports/2026-10-27-55-superseded-runtime-expansion.md) |

| CLN-20261027-56 | 2026-10-27 | 回收340份同恢复包内容相等的旧暂存库/dump/程序，2024827592逻辑字节 | 独立记录原保护清单未列出的暂存路径/mode；试用包和原证据保留 | [身份/恢复/边界](reports/2026-10-27-56-duplicate-runtime-expansion.md) |

| CLN-20261027-57 | 2026-10-27 | 回收2800份已消费的手册输出中间副本，841532040逻辑字节 | 所选内容与保留包成员逐项相等，文档原输入/配方和结果保留 | [身份/恢复/边界](reports/2026-10-27-57-consumed-manual-outputs.md) |

每份报告本次归档提交通过 `git log --follow -- maintenance/cleanup/reports/文件名.md` 定位；这是报告的Git溯源，不把较早的源码/摘要提交误写为本次归档提交。未来新增批次先取得新编号并追加本表，不复用或覆盖旧编号。

非计划删除另记[2026-10-25 G4产物丢失事件](../incidents/2026-10-25-g4-build-artifact-loss.md)：删除者未知，不算作已授权清理批次，也不虚构恢复或仍保有产物。


| CLN-20261027-58 | 2026-10-27 | 两次失败Cocoa构建后回收19880份编译缓存，722231668逻辑字节 | 原源码、失败和回执14056项哈希不变；当前构建/试用包保留 | [范围与恢复](reports/2026-10-27-58-failed-cocoa-compiler-caches.md) |

| CLN-20261027-59 | 2026-10-27 | UI修复完成后回收10639份编译缓存，436824308逻辑字节 | 26979项源/证据及当前、回退App哈希不变；第5轮真实运行崩溃保留 | [范围与恢复](reports/2026-10-27-59-consumed-ui-compiler-caches.md) |

| CLN-20261027-60 | 2026-10-27 | 回收已消费UI标志组件程序1份，511576逻辑字节 | 8项源码/结果/当前程序身份不变，在用窗口不动；额外直接字访问静态核对16项一致 | [范围与恢复](reports/2026-10-27-60-ui-flag-probe.md) |

| CLN-20261027-61 | 2026-10-27 | 缩放修复交付后回收14955份编译副本，527335335逻辑字节 | 27063项源/失败/新证据/当前与回退App身份不变；冻结链接/诊断输入保留 | [范围与恢复](reports/2026-10-27-61-live-resize-compiler-caches.md) |

| CLN-20261027-62 | 2026-10-27 | 回收5177份已消费多窗口编译副本，202100121逻辑字节 | 13066项冻结源/程序/原失败/新诊断身份不变；真实输入缺口保留 | [范围与恢复](reports/2026-10-27-62-consumed-multiframe-build-caches.md) |

| CLN-20261027-63 | 2026-10-27 | Retired8153 consumed receiver-build/cache and failed-copy files,357814435 logical bytes | All16015 retained input/product/evidence identities unchanged; current native/release apps preserved | [Identity/recovery/scope](reports/2026-10-27-63-consumed-multiframe-build-caches.md) |

| CLN-20261027-64 | 2026-10-27 | Retired2 rejected uncommitted SVG masters plus23363 consumed generated/cache/intermediate files,1584540183 cache logical bytes | Exact backups and per-file manifest; fsync gap on initial SVG retirement disclosed; all16013 retained identities unchanged | [Identity/recovery/scope](reports/2026-10-27-64-artwork-and-consumed-release-caches.md) |

| CLN-20261027-65 | 2026-10-27 | Retired6812 consumed packaging/test/cache/superseded release files, 508068131 logical bytes | Durable identity manifest; all5980 retained identities unchanged; current F/epsilon delivery preserved | [Scope and recovery](reports/2026-10-27-65-consumed-f-artwork-outputs.md) |

| CLN-20261027-66 | 2026-10-27 | Retired6885 consumed output files, 486132014 logical bytes | Durable identity manifest; all5982 retained identities unchanged at cleanup; current products/evidence retained | [Scope and recovery](reports/2026-10-27-66-selected-horn-artwork-outputs.md) |

| CLN-20261027-67 | 2026-10-27 | Retired4058 consumed output files, 289754448 logical bytes | Durable identity manifest; all9002 retained identities unchanged at cleanup; current products/evidence retained | [Scope and recovery](reports/2026-10-27-67-consumed-contour-preview-cache.md) |

| CLN-20261027-69 | 2026-10-27 | 净回收55470份旧布局构建展开副本，3712209331逻辑字节 | 七份既有归档逐内容验证，20874项保留身份不变；34份build配置/文字额外恢复，错误与恢复如实记录 | [范围与恢复](reports/2026-10-27-69-layout-expansions.md) |

| CLN-20261027-70 | 2026-10-27 | 12个已完成配置回收112116份生成Rust副本，3737290194逻辑字节 | 88190项冻结源/产品/资源/链接证据身份不变；其他配置继续构建 | [范围与恢复](reports/2026-10-27-70-consumed-d26-compiler-copies.md) |

| CLN-20261027-71 | 2026-10-27 | 后续5个已完成配置回收48890份Rust副本，1640451148逻辑字节 | 120230项保护内容不变；在跑配置与当前产品不动，旧批不重复计数 | [范围与恢复](reports/2026-10-27-71-consumed-d26-compiler-copies.md) |

| CLN-20261027-68 | 2026-10-27 | D26结束后29770份编译副本/缓存，1039083114逻辑字节 | 实际晚于70/71；136269项删前后不变；后续缺失驱动按原归档恢复，原因未知 | [范围与恢复](reports/2026-10-27-68-consumed-d26-compiler-copies.md) |

| CLN-20261027-72 | 2026-10-27 | TLS的25项共享声明归位，并撤回3份错误/重复原型，无文件/缓存删除 | P401施工中，未作TLS验收；逐项旧片段与原文件blob可溯源 | [范围与恢复](reports/2026-10-27-72-tls-declaration-consolidation.md) |

| CLN-20261027-73 | 2026-10-27 | 首轮TLS编译失败后回收9686份生成副本/缓存、364128325逻辑字节 | 8260项保留内容哈希不变，原失败/冻结源/接口证据保留 | [范围与恢复](reports/2026-10-27-73-failed-d27-compiler-copies.md) |

| CLN-20261027-74 | 2026-10-27 | 回收656份已结束诊断副本/失败链接输入、42832592逻辑字节 | 16份最小编译根/配置身份不变；计划误写副本已精确恢复并记录；D28不动 | [范围与恢复](reports/2026-10-27-74-consumed-diagnostics.md) |

| CLN-20261027-75 | 2026-10-27 | D28首个完成配置回收9302份Rust副本、310060430逻辑字节 | 13723项保留内容不变；中断后独立核实退出并释放原保护再继续同一账本 | [范围与恢复](reports/2026-10-27-75-completed-d28-compiler-copies.md) |

| CLN-20261027-76 | 2026-10-27 | 退役旧D26的126950份无消费者制品、11457749357逻辑字节 | 45381项保留内容不变；冻结源、原始日志与回执保留，无新全量缓存归档 | [范围与恢复](reports/2026-10-27-76-superseded-d26-runtime-artifacts.md) |

| CLN-20261027-77 | 2026-10-27 | 恢复后的baseline两腿回收9302份副本、309409362逻辑字节 | 完整退出回执；产品与链接输入保留；2/20配置 | [范围与恢复](reports/2026-10-27-77-recovered-baseline-copies.md) |

| CLN-20261027-78 | 2026-10-27 | 旧D26生成手册与冗余保全包23761份、2540186628逻辑字节 | 无消费者；全部冻结源与原始回执不变，旧archive路径退役 | [范围与恢复](reports/2026-10-27-78-superseded-d26-manuals.md) |

| CLN-20261027-79 | 2026-10-27 | 旧D26残余编译制品3900份、2097411420逻辑字节 | 无消费者；全部冻结源及原始回执不变；当前D28不动 | [范围与恢复](reports/2026-10-27-79-superseded-d26-residual-products.md) |

| CLN-20261028-80 | 2026-10-28 | 旧D24/D27残余编译缓存9581份、475034633逻辑字节 | 两棵冻结源/元数据不变；四棵无法完整验证的旧树整棵排除；当前D28不动 | [范围与恢复](reports/2026-10-28-80-obsolete-matrix-caches.md) |

| CLN-20261028-81 | 2026-10-28 | 166份旧验收JSON完整保留为154内容成员，退役原路径；1份无效压缩试样清理 | 解压SHA/字节全核验；约1.73GiB分配量回收；冻结源及原始文本日志保留，P403拒绝/P404采用 | [范围与恢复](reports/2026-10-28-81-retained-evidence-storage.md) |

| CLN-20261028-82 | 2026-10-28 | 8个恢复终端配置两腿74416份生成副本、2478845236逻辑字节 | 原逐项fsynced操作账本汇总；首配置/基线已75/77不重计；后续Cocoa配置失败不冒充验收通过 | [范围与恢复](reports/2026-10-28-82-recovered-terminal-copies.md) |

| CLN-20261028-83 | 2026-10-28 | 退役配置绑定失效的D28旧制品/生成手册22985份、5864944479逻辑字节 | 冻结源及原有元数据逐项不变；原失败和日志保留；新D29、根App不动 | [范围与恢复](reports/2026-10-28-83-superseded-d28-runtime-products.md) |

| CLN-20261028-84 | 2026-10-28 | D29首对代表四腿已消费副本19130份、640352977逻辑字节 | 原逐项操作账本核验，制品/源/配置保留；未运行功能验收 | [范围与恢复](reports/2026-10-28-84-new-representative-copies.md) |

| CLN-20261028-85 | 2026-10-28 | Retire59172 superseded D28 package resource copies,2641604359 logical bytes | Released ownership and original hashes verified; frozen inputs/logs/current D29 retained; no exclusive-space claim | [Scope and recovery](reports/2026-10-28-85-superseded-d28-resource-copies.md) |

| CLN-20261028-86 | 2026-10-28 | Retire3 redundant old UI source archives,606773535 logical bytes | All21132 original source/metadata identities retained; incomplete old scopes skipped; current D29 untouched | [Scope and recovery](reports/2026-10-28-86-redundant-ui-source-archives.md) |

| CLN-20261028-87 | 2026-10-28 | Retire208 committed manifest export copies/712090173 logical bytes, plus36 consumed restoration probes | Real Git restoration verified; business inputs preserved; old exports require metadata restoration before full source verification | [Scope and recovery](reports/2026-10-28-87-committed-manifest-export-copies.md) |

| CLN-20261028-88 | 2026-10-28 | Compact10 old source containers/2822825012 bytes into reachable Git plus77045704-byte unique-content and24089514-byte mapping stores | All13337 contents actually restored;70104 source mappings checked; identified compiler caches omitted; raw journals/results retained | [Scope and recovery](reports/2026-10-28-88-compacted-source-containers.md) |

| CLN-20261028-89 | 2026-10-28 | D29后续13配置26腿已消费副本124082份、4143469900逻辑字节 | 原操作账本逐项核验；不重计CLN84，当前产品及源保留，发布器无新删除 | [范围与恢复](reports/2026-10-28-89-continuation-generated-copies.md) |

| CLN-20261028-90 | 2026-10-28 | D29完成配置30腿手册暂存12530份、1081451056逻辑字节 | 实际包内Info同字节核验；终端直接引用Info、原回执/日志和当前候选保留 | [范围与恢复](reports/2026-10-28-90-consumed-manual-staging.md) |

| CLN-20261028-91 | 2026-10-28 | 旧成功构建138份原日志734439595字节无损保留为62032435字节gzip | 每份实际解压恢复核SHA，原元数据与失败/当前日志保留；旧路径须按映射恢复 | [范围与恢复](reports/2026-10-28-91-retained-build-logs.md) |

| CLN-20261028-92 | 2026-10-28 | 两个中断Cocoa case已无消费者的编译缓存14277份、1181584740逻辑字节 | 原冻结源前后完整验证；17完成配置、配置/失败日志/元数据保留，旧半成品需重建 | [范围与恢复](reports/2026-10-28-92-interrupted-cocoa-caches.md) |

| CLN-20261028-93 | 2026-10-28 | 五个旧运行时导出已提交数据副本22970份、758888595逻辑字节 | 4524不同Git blob实际恢复，固定232f1651及原mode映射；旧完整源须结合87恢复，新D29/UI试用源不动 | [范围与恢复](reports/2026-10-28-93-committed-data-export-copies.md) |

| CLN-20261028-94 | 2026-10-28 | 首个续成Cocoa两腿已消费手册暂存910份、94583952逻辑字节 | 68/68、真程序/冻结输入及包内Info完整身份已核；原回执/日志保留，终端Info不动 | [范围与恢复](reports/2026-10-28-94-consumed-cocoa-manual-staging.md) |

| CLN-20261028-95 | 2026-10-28 | 两个旧中断Cocoa半成品重复资源5056份、179943122逻辑字节 | 每份与当前冻结源实际同字节；2494不同替代内容已核，原配置/失败/元数据保留 | [范围与恢复](reports/2026-10-28-95-interrupted-resource-copies.md) |

| CLN-20261028-96 | 2026-10-28 | 第二个续成Cocoa两腿已消费手册暂存910份、94583952逻辑字节 | 程序/68手册/包内Info完整身份核验，原回执/日志/产品保留，终端Info不动 | [范围与恢复](reports/2026-10-28-96-consumed-cocoa-manual-staging.md) |

| CLN-20261028-97 | 2026-10-28 | 最后续成Cocoa配置两腿已消费手册暂存910份、94583952逻辑字节 | 程序、68手册及包内Info身份已核；原日志、回执及产品保留 | [范围与恢复](reports/2026-10-28-97-consumed-cocoa-manual-staging.md) |

| CLN-20261028-98 | 2026-10-28 | 最终化中断后未完成保全归档671187835字节 | 全部138291原文件实际核验后才退役重复输出；原native最终回执缺失，不虚构通过或指标 | [范围与恢复](reports/2026-10-28-98-incomplete-preservation-archive.md) |

| CLN-20261028-99 | 2026-10-28 | 两份已消费host Cargo缓存及一份重复工具输入，873文件、113848544逻辑字节 | 原host与保全包逐份实际核验，当前138291文件账本未引用所选项，7099冻结源前后完整验证 | [范围与恢复](reports/2026-10-28-99-consumed-host-caches.md) |

| CLN-20261028-100 | 2026-10-28 | 已取消TLS轮次中7份冗余成功断言历史、1864480748逻辑字节 | 全量JSON解析后保留除大diagnostic外全部字段、原始SHA及真实取消回执；原超限拒绝与取消状态不改写为通过 | [范围与恢复边界](reports/2026-10-28-100-verbose-passed-responses.md) |

| CLN-20261028-101 | 2026-10-28 | 211个已消费成功用例隔离状态、23302份/591932223逻辑字节 | 逐例进程/句柄/删前身份核对，原结果哈希不变，失败与原始夹具保留；缓存可重建 | [范围与恢复](reports/2026-10-28-101-consumed-case-state.md) |

| CLN-20261028-102 | 2026-10-28 | dump48/48完成后80份试验图像/836810968逻辑字节 | 原结果SHA匹配、输出无句柄，原产品/夹具及所有回执保留；再生不承诺随机图像字节一致 | [范围与恢复](reports/2026-10-28-102-consumed-dump-images.md) |

| CLN-20261028-103 | 2026-10-28 | 4份已消费搬移副本/968654987逻辑字节 | Spotlight自然释放后全树比对及无句柄核验；独立延后回执绑定原失败SHA，修复收尾字段错误但未重复删除 | [范围与恢复](reports/2026-10-28-103-deferred-relocation-copies.md) |

| CLN-20261028-104 | 2026-10-28 | 独立复核消费后的9份小型夹具/26文件与链接/258逻辑字节 | 原结果与证明保留；逐项身份、无句柄及持久意图后回收，不计额外测量缓存释放 | [范围与恢复](reports/2026-10-28-104-consumed-p411-fixtures.md) |

| CLN-20261028-105 | 2026-10-28 | 完整ERT后的自动缓存回收41216文件/1043534381逻辑字节；negative零选中 | 10943份逐例回执与清单/私有路径映射逐项核对；无重复删除，原结果和产品保留 | [身份与恢复](reports/2026-10-28-105-runtime-functional-state.md) |

| CLN-20261029-127 | 2026-10-29 | 已消费P424 Cargo/四生成unit根，19910文件、903目录、703007544逻辑字节 | 20813条完整字段审阅、双次无句柄/逐项身份核对；744保留文件及四实际程序/dump/资源身份不变，冻结源与原失败保留 | [范围与恢复](reports/2026-10-29-127-consumed-shorthand-compiler-copies.md) |

| CLN-20261029-128 | 2026-10-29 | 已消费P425 Cargo/四生成unit根，19910文件、903目录、703037940逻辑字节 | 20813条完整字段及39820操作记录逐项对应；746保留文件和四实际程序/dump/资源不变，源/原失败/配方保留 | [范围与恢复](reports/2026-10-29-128-consumed-zlib-compiler-copies.md) |
| CLN-20261029-129 | 2026-10-29 | Consumed3Cargo/4unit/4context roots;20612 files,120links,1373dirs,699637607logical bytes | 22105full-field manifest rows and41464paired operations;source/4products/raw failures retained | [report](reports/2026-10-29-129-consumed-lcms-compiler-context-copies.md) |

## CLN130 · 2026-10-29 · consumed module compiler copies

P429 signed/pushed `ccb38ee4`; retired five ended-run Cargo/generated-unit roots: 19,910 regular files, 903 directories, 703,344,044 logical bytes. Also withdrew one unused AI-cache proposal (692 bytes), never applied. The 20,814-row manifest fully matches the private plan after machine-path aliasing; 41,628 durable operations pair. Frozen source, actual objects/four programs/dumps/resources, compiled C module and all failures/recipes remain; no new build/test for cleanup. See [report](reports/2026-10-29-130-consumed-module-compiler-copies.md) and [manifest](manifests/2026-10-29-130-consumed-module-compiler-copies.jsonl.gz). Goal remains active1/7.

## CLN131 · 2026-10-29 · consumed native compiler copies

Retired five ended-run Cargo/generated-unit roots: 19,910 regular files, 903 directories, 705,203,373 logical bytes. All 20,813 public rows match the private plan after machine-path aliasing; 41,626 durable operations pair. Frozen source, actual objects/four native products/all selected resources, original ELNs and raw five shared metadata failures remain unchanged. No build/test rerun for cleanup. See [report](reports/2026-10-29-131-consumed-native-compiler-copies.md) and [manifest](manifests/2026-10-29-131-consumed-native-compiler-copies.jsonl.gz). Goal remains active1/7.

## CLN132 · 2026-10-29 · Tree-sitter source rebase/consolidation (in progress)

P430 restores the stable feature, including loader configuration observability and shared object ownership. Removed the inactive development-only language cache with exact block/call-site hashes and Git restoration bound; shared-declaration consolidation follows in the same batch. Source/build/integration/review/delivery remain pending. See [report](reports/2026-10-29-132-treesit-declaration-rebase.md) and [manifest](manifests/2026-10-29-132-treesit-declaration-rebase.json).

## CLN133 · 2026-10-29 · consumed Tree-sitter compiler copies

Retired eleven ended-run Cargo/generated-unit roots: 30,706 files, 1,690 directories and 1,144,859,531 logical bytes. All 32,396 decoded manifest rows match the private plan; 64,792 operations pair. Frozen source, actual objects, four current products and complete resources, raw failures and two minimal old-driver source-binding records remain. No new build/test for cleanup. See [report](reports/2026-10-29-133-consumed-treesit-compiler-copies.md) and [manifest](manifests/2026-10-29-133-consumed-treesit-compiler-copies.jsonl.gz). Coherent implementation dependency delivery remains pending; Goal active1/7.


## Source consolidation delivery update · CLN72 / CLN113 / CLN132

The complete experimental foundation source group includes the TLS shared-declaration move (CLN72), notification event/open-flag consolidation (CLN113), and stable Tree-sitter loader/shared-layout rebase (CLN132). These are source changes with exact original blocks and restoration identities, not physical file/cache deletions. Historical construction checkpoints remain historical; current limited build/integration evidence and remaining Goal boundaries are in the [foundation report](../releases/2026-10-29-runtime-foundation.md). Reports: [CLN72](reports/2026-10-27-72-tls-declaration-consolidation.md), [CLN113](reports/2026-10-28-113-event-kind-declaration.md), [CLN132](reports/2026-10-29-132-treesit-declaration-rebase.md).


## CLN134 · 2026-10-29 · consumed foundation compiler copies and fixtures

After signed source delivery `d9a41b0a`, retired nine ended-run compiler/synthetic-fixture roots: 20,021 regular files, two links, 1,032 directories and 706,887,372 logical bytes. All 21,055 decoded manifest rows match the private plan; 42,110 intent/result operations pair. The frozen source, 739 retained hashes, five selected product identities and full resource groups, actual objects and original failures remain unchanged. No build/test rerun for cleanup. See [report](reports/2026-10-29-134-consumed-foundation-compiler-fixtures.md) and [manifest](manifests/2026-10-29-134-consumed-foundation-compiler-fixtures.jsonl.gz). Goal active, only G4 delivered.


## CLN135 · 2026-10-29 · consumed foundation relocation copies

Retired four temporary source-denied relocation packages: 17,936 files, 442 directories and 931,372,817 logical bytes. All 17,936 manifest rows and 36,756 paired operations verified. Original source/products/resources remain unchanged. Spotlight initially held the ordinary terminal copy; raw failure retained, exact copy retired after zero occupancy without rerunning tests. See [report](reports/2026-10-29-135-consumed-foundation-relocation.md), [manifest](manifests/2026-10-29-135-consumed-foundation-relocation.jsonl.gz) and [limited relocation evidence](../releases/2026-10-29-foundation-relocation.md). Goal remains active, only G4 delivered.


## CLN136 · 2026-10-29 · consumed standard compiler and App copies

Retired six ended-use roots: 20,977 regular files, two links, 1,054 directories and 1,194,535,878 logical bytes. All 22,033 decoded manifest rows and 44,066 paired operations verified. One requested Cocoa unit root was already absent and is not counted. Clean clone/Git, actual objects, current App/DMG, clone/root terminal pairs, previous App needed for G1 rollback, resources, failures and WIP remain unchanged. No test rerun for cleanup. See [report](reports/2026-10-29-136-consumed-standard-compiler-app-copies.md), [manifest](manifests/2026-10-29-136-consumed-standard-compiler-app-copies.jsonl.gz) and [G2 delivery](../releases/2026-10-29-standard-checkout-delivery.md). Goal active; G2 and G4 delivered, 2/7.


## CLN137 · 2026-10-29 · consumed background-repair copies

Retired six ended-use roots: repair Cargo/units, identical new App copy, consumed broken App and two component executables. 15,077 files, two links, 625 directories, 811,763,529 logical bytes; all 15,704 decoded manifest rows and 31,408 paired operations verified. Source/Git/actual objects/current App/terminal/original rollback/old DMG/user packages/raw failures and successful normal-startup evidence remain unchanged. See [report](reports/2026-10-29-137-consumed-background-repair-copies.md), [manifest](manifests/2026-10-29-137-consumed-background-repair-copies.jsonl.gz) and [P432](../incidents/2026-10-29-cocoa-background-receiver.md). No test rerun for cleanup; Goal active2/7.

## CLN138 · 2026-10-30 · consumed original compiler-test copies

After five-file original compatibility execution and independent review, retired only three isolated mutable test trees and three context links:4,785 files,753 directories,63,052,690 logical bytes,76,001,280 allocated bytes. All5,541 public identity rows verified; pinned corpus, raw false/skip evidence, logs/events/owner receipts, Git/WIP and current products unchanged. Existing retirement helper reused; no tests rerun. See [report](reports/2026-10-30-138-consumed-compiler-test-copies.md), [manifest](manifests/2026-10-30-138-consumed-compiler-test-copies.jsonl.gz) and [limited comparison](../releases/2026-10-30-original-core-libraries.md). Replay needs rematerialized test trees; G3/fullGoal remain open.

## CLN139 · 2026-10-30 · consumed runtime-support test copies

After normal runtime-support and native-cache original cases plus independent review, retired7isolatedtesttrees/7contextlinks:10,752files1,757directories146,978,664logical/175,616,000allocatedbytes. All12,516decodedpublicidentityrows verified; result/log/event/owner records, original skips/false outcomes, pinned corpus, normal/native products/resources/buildreceipts and Git/WIP unchanged. Existing retirement helper reused, no tests rerun. See [report](reports/2026-10-30-139-consumed-runtime-support-test-copies.md), [manifest](manifests/2026-10-30-139-consumed-runtime-support-test-copies.jsonl.gz), [scope](../releases/2026-10-30-original-runtime-support.md). Rematerialize before replay; fullG3/Goal remain open.

## CLN140 · 2026-10-30 · consumed evaluator-mode test copies

After 558 original source/bytecode passes and independent review, retired only three test trees, three compilation environments and three context links:4,617 files,780 directories,3 links,63,467,856 logical/75,767,808 allocated bytes. All5,400 manifest rows are verified; mode/compilation/result/log/event/owner records, bytecode fingerprints, source cache, observer, Git/WIP and products remain. Existing retirement is reused without rerunning tests. See [report](reports/2026-10-30-140-consumed-evaluator-mode-test-copies.md), [manifest](manifests/2026-10-30-140-consumed-evaluator-mode-test-copies.jsonl.gz), [scope](../releases/2026-10-30-original-evaluator-modes.md). Rematerialize and compile before replay; G3/full Goal remain open.

## CLN141 · 2026-10-30 · consumed primitive test copies

After six-file original comparison and independent review, retired only three isolated test trees and three context/test links:4,608files753directories3links62,990,856logical/75,264,000allocatedbytes. All5,364decodedidentityrows match privateplans in every field. All18individualprocesses are completed/reaped; aggregatefalse/ownerexit1 from9originalskips is preserved. Rawresults/events/controller/sourcecorpus/currentproducts/Git/WIP remain, no test rerun. See [report](reports/2026-10-30-141-consumed-primitive-test-copies.md), [manifest](manifests/2026-10-30-141-consumed-primitive-test-copies.jsonl.gz), [comparison](../releases/2026-10-30-original-primitives.md). Rematerialize before replay; fullGoal remains active.

## CLN142 · 2026-10-30 · consumed editor-primitive test copies

After168originalpasses and independentreview, retired only three isolated testtrees/threecontext/testlinks:4,608files753directories3links62,990,856logical/75,264,000allocatedbytes. All5,364decodedidentityrows match privateplans in everyfield; all12fileprocesses andownerexit0/reaped. Rawresults/events/fingerprints/corpus/currentproducts/Git/WIP remain, no tests rerun. See [report](reports/2026-10-30-142-consumed-editor-primitive-test-copies.md), [manifest](manifests/2026-10-30-142-consumed-editor-primitive-test-copies.jsonl.gz), [comparison](../releases/2026-10-30-original-editor-primitives.md). Rematerialize before replay; fullGoal remains active.

## CLN143 · 2026-10-30 · consumed editing tests and environments

Review found five omitted environments; corrected cleanup retires only3testtrees/5environments/3contextlinks:4,612files812directories3links63,024,972logical/75,313,152allocatedbytes. All5,427decodedfields match privateplans. Includes3fourbyteautosaves/one34,104byte reference nativecache; no actualproduct/userdata removed. Allrawbatch/graphicalresults/events/initialretaineddecisions/controllers/corpus/products/Git/WIP preserved; no tests rerun. See [report](reports/2026-10-30-143-consumed-editing-test-copies.md), [manifest](manifests/2026-10-30-143-consumed-editing-test-copies.jsonl.gz), [comparison](../releases/2026-10-30-original-editing.md). Rematerialize/recreateenvironments forreplay; fullGoal active.

## CLN144 · 2026-10-30 · consumed measurement output and readiness directories

Removed only 180 completed-case out directories plus seven empty environment roots for each of five readiness processes: 215 directories, zero files/file bytes and zero measured allocated blocks. Every captured identity matches the P434 production plan and every selected path is absent. All 1,419 pre-existing evidence files are unchanged. Earlier automatic directory removals lacked directory identities; this later operation does not reconstruct them. See [report](reports/2026-10-30-144-consumed-measurement-directories.md), [manifest](manifests/2026-10-30-144-consumed-measurement-directories.jsonl.gz), [measurement](../releases/2026-10-30-native-measurement-entry.md) and [ownership repair](../incidents/2026-10-30-retirement-directory-identities.md). Recreate owned empty directories through the recorded setup before replay; full Goal remains active.

## CLN145 · 2026-10-30 · consumed display/service test copies

Retired only three complete test copies/five environments/three context links after original batch and actual NS follow-up: 4,608 files, 809 directories, three links, 62,990,856 logical/75,264,000 allocated file bytes. The 5,420 decoded identities match captured plans; P434 also checks actual directory ownership. Batch raw false/skips, separate graphical passes, all prior evidence/corpus/products and HEAD/index remain. Partner source work is outside this cleanup. See [report](reports/2026-10-30-145-consumed-display-service-test-copies.md), [manifest](manifests/2026-10-30-145-consumed-display-service-test-copies.jsonl.gz) and [comparison](../releases/2026-10-30-original-display-services.md). Restore owned copies/environments/links before replay; full Goal active.

## CLN146 · 2026-10-30 · consumed GC Cargo and generated-unit copies

After actual build/functional/selected measurement completion, retired only five ignored untracked compiler/generated roots:19,456files895directories684,712,192logical/723,292,160allocatedfilebytes;20,351hash-onlyrows match durable predelete plans. Program/dump/fullresource identities, actualobjects/archives, source/export/driver, rawresults and HEAD/index unchanged. No product/userdata/peer core selection or test replay; regenerate removed compiler/generated inputs before reuse. See [report](reports/2026-10-30-146-consumed-gc-build-copies.md), [manifest](manifests/2026-10-30-146-consumed-gc-build-copies.jsonl.gz) and [attribution](../releases/2026-10-30-current-gc-attribution.md). FullGoal active.

## CLN147 · 2026-10-30 · consumed core build copies and isolated environments

Two consumed fns/process generated-unit directories and fourteen successful process environments: 138 files/100 directories, 4,868,938 logical/5,144,576 allocated bytes; 238 hash-only rows, SHA 821ee7af8425f3f1405f67a52bb775c46c0582681bbd2d568a44d9ebfe2d1994. Durable production identities and all decoded fields match; program/dump/full resources and HEAD/index remain unchanged. Objects, raw records and failures remain. [Report](reports/2026-10-30-147-consumed-core-build-copies.md).

## CLN148 · 2026-10-30 · consumed GC owner-batch build and case copies

Six consumed Cargo/generated-unit scopes plus the integration runner's 640 successful process-state scopes: 19,892 files and 4,958 directories, 739,702,832 logical / 779,845,632 allocated bytes. All 24,850 decoded ledger rows match durable pre-delete plans. Program/dump/full resources and HEAD/index remain unchanged; actual objects, archives, frozen source and raw results remain for the remaining consumers. No workload replay. [Report](reports/2026-10-30-148-consumed-owner-batch-copies.md) and [manifest](manifests/2026-10-30-148-consumed-owner-batch-copies.jsonl.gz). Optimization acceptance and the full Goal remain open.

## CLN149 · 2026-10-30 · consumed GC owner-batch case state

Retired 4,248 empty directories across 24 successful protection and 684 successful performance scopes, zero files/bytes; all decoded hash-only rows match original pre-delete plans. Program/dump/resources and HEAD/index remain unchanged, raw measurements and native-control output remain. No workload replay or space-recovery claim. [Report](reports/2026-10-30-149-consumed-owner-batch-case-state.md) and [manifest](manifests/2026-10-30-149-consumed-owner-batch-case-state.jsonl.gz).

## CLN150 · 2026-10-30 · unaccepted remembered-owner batch withdrawal

Restored only the P436 production change to the exact prior source (SHA 0f18ba7d761d3fcf4e99bacd8bd188289e816cdca933d4c22b83d37393a1b74e); no reliable eligible performance comparison supported adoption. The never-committed candidate is recoverable from an inert 1,123-byte patch, verified by exact isolated reconstruction. Useful protection cases and raw measurements remain; no new compilation, workload replay or space-recovery claim. [Report](reports/2026-10-30-150-owner-batch-withdrawal.md) and [patch](patches/2026-10-30-150-owner-batch.patch).

## CLN151 · 2026-10-30 · retired unaccepted owner-batch expansion

Removed only the completed batch/source expansion: 20,697 files, 943 directories and12 links;1,500,543,266logical/1,546,506,240allocated regular bytes. All7298 exact source identities remain recoverable via7296 retained-prior mappings plus2 unique copies, verified again after removal;24 metadata files retained. Prior products/raw results/HEAD/index unchanged; historical batch paths now require source restore and rebuild, not direct replay. [Report](reports/2026-10-30-151-retired-batch-expansion.md) and [manifest](manifests/2026-10-30-151-retired-batch-expansion.jsonl.gz).

## CLN152 — consumed current-core remote09 state (2026-10-30)

[Report](reports/2026-10-30-152-current-remote09.md) and
[manifest](manifests/2026-10-30-152-current-remote09.jsonl.gz): removed only 1,841
regular files, 298 directories and 19 context links after three reaped diagnostic
processes; 48,912,321 logical / 53,641,216 allocated regular bytes. Raw failure,
results, real products and canonical pinned caches remain. Source copies can be
regenerated from pinned manifests; temporary names are not guaranteed. No repair
claim or test replay.

## CLN153 — consumed current Cocoa compiler state (2026-10-30)

[Report](reports/2026-10-30-153-consumed-cocoa-core-state.md) and
[manifest](manifests/2026-10-30-153-consumed-cocoa-core-state.jsonl.gz): 16 owned
Cargo/generated-unit/core/checking-startup groups removed 10,472 regular files
and 659 directories; 390,090,877 logical / 410,865,664 allocated regular bytes.
Exact source, actual objects/configuration/receipts, current/previous/candidate
Apps, native-key event environment and raw results remain unchanged. Generated
copies are rebuilt from retained inputs; no workload replay or physical-volume
reclaim claim.

## CLN154 — consumed failed fallback build state (2026-10-30)

[Report](reports/2026-10-30-154-consumed-fallback-build-state.md) and
[manifest](manifests/2026-10-30-154-consumed-fallback-build-state.jsonl.gz):
9,139 regular files and954 directories retired;678,892,963logical /699,428,864allocated regularbytes.
All7,318failed-build source inputs recoverable (7,291prior mappings+27unique copies),229generated raw diagnostic/config files and all original failures retained. Prior products/resources and trialApp unchanged. Original failed source is retired, not a usable candidate; no physical-volume reclaim or GC speedup claim.

## CLN155 — consumed fallback-candidate compiler and case state (2026-10-30)

[Report](reports/2026-10-30-155-consumed-fallback-candidate-state.md) and
[manifest](manifests/2026-10-30-155-consumed-fallback-candidate-state.jsonl.gz):
505 successful owned groups retired 9,946 regular files and 3,571 directories;
370,202,575 logical / 390,307,840 allocated regular-file bytes. Frozen source,
actual objects/configuration/products, prior controls, original results and
observations and trial App are unchanged. No physical-volume reclaim claim.

## CLN156 — consumed permanent-fallback workloads (2026-10-30)

[Report](reports/2026-10-30-156-consumed-permanent-fallback-workloads.md) and
[manifest](manifests/2026-10-30-156-consumed-permanent-fallback-workloads.jsonl.gz):
90successfulgroups retired540emptydirectories,0regularfiles and0logical/allocated
regularbytes.550retainedfiles, actualproducts/resources, originalmeasurements and
trialApp unchanged; not a disk-space recovery or physical-volume reclaim claim.

## CLN157 — consumed empty-index compiler and case state (2026-10-30)

[Report](reports/2026-10-30-157-consumed-empty-index-state.md) and
[manifest](manifests/2026-10-30-157-consumed-empty-index-state.jsonl.gz):
941successfulgroups retired9,946regularfiles and6,187directories;
369,937,618logical /389,951,488allocated regularbytes. 5,611retainedfiles,
7,348frozeninputs, candidate/priorproducts and trialApp unchanged. Raw false and
passing aggregates retained. Two pre-deletion reader-bound failures preserved;
final bounded per-process cleanup complete. No physical-volume recovery claim.


## CLN158 — consumed final remote diagnostic state (2026-10-30)

[Report](reports/2026-10-30-158-consumed-final-remote-diagnostic-state.md) and
[manifest](manifests/2026-10-30-158-consumed-final-remote-diagnostic-state.jsonl.gz):
2 regular files and 15 directories retired after the final-source actual case
passed; 72,046 logical /77,824 allocated regular bytes. Original results/process
logs and product/source identities retained. No intermittent-fix or disk-volume
reclaim claim.


## CLN159 — consumed known-cons build/case state (2026-10-30)

[Report](reports/2026-10-30-159-consumed-known-cons-state.md) and
[manifest](manifests/2026-10-30-159-consumed-known-cons-state.jsonl.gz):
502selectedgroups,9,948regularfiles/3,547directories;370,962,164logical and
391,000,064allocated regularbytes. Candidate/prior products,7,355frozeninputs,
original results/observations and trialApp preserved. The first controller
measurement failure occurred before deletion; one checked retry completed.
Generated protection source/executable retired after independent review.

## CLN160 — consumed collector-root-phase state (2026-10-30)

P442: retire only consumed compiler/generated-unit copies,530 fresh integration
and12 attribution case environments, plus two reviewed protection files.
9,948files/3,811dirs;377,516,592logical/397,533,184allocated bytes;13,759 decoded
identity rows. Frozen7,360 inputs, raw receipts, prior/current products and trial
App retained and verified. Original build controller failure preserved, separate
file-only finalization verified; no source deletion or performance adoption.
[Report](reports/2026-10-30-160-consumed-root-phase-state.md).


## CLN161 — consumed original text-processing test copies (2026-10-30)

After339/339 original observations and independent review, retire three test
roots/three testlinks only:4,608files/753dirs,62,990,856logical and75,264,000
allocated regularbytes,5,364identity rows. Actual products/resources, raw
results/events, source corpus and eleven restored live terminal libraries
retained. Two failed preflights preserved; no cause assigned to missing libraries.
[Report](reports/2026-10-30-161-consumed-text-processing-test-copies.md) and
[manifest](manifests/2026-10-30-161-consumed-text-processing-test-copies.jsonl.gz).


## CLN162 — consumed undo and file-workflow state (2026-10-30)

Original123pass18platformskips/aggregatefalse retained after independent review.
Runner earlier5files/51dirs plus post-review4,608files/819dirs/3links:
combined4,613files/870dirs,63,161,320logical/75,448,320allocated bytes,
5,486identity rows. Retained raw outcomes, original automatic ledgers, corpus,
actual products and live libraries; no product or acceptance-policy change.
[Report](reports/2026-10-30-162-consumed-file-workflow-state.md) and
[manifest](manifests/2026-10-30-162-consumed-file-workflow-state.jsonl.gz).


## CLN163 — consumed original time and resource state (2026-10-30)

Original105passes3sharedSQLiteextensionfunction skips, aggregatefalse retained.
Runner earlier0files/72dirs plus post-review4,608files/789dirs/3links:
combined4,608files/861dirs/3links,62,990,856logical/75,264,000allocated bytes,
5,472identity rows. Raw skips/results/automatic ledgers/corpus/live products
retained; no compiler/product executed for cleanup or acceptance-policy change.
[Report](reports/2026-10-30-163-consumed-time-resource-state.md) and
[manifest](manifests/2026-10-30-163-consumed-time-resource-state.jsonl.gz).


## CLN164 — consumed original JSON state (2026-10-30)

After 75 original passes and independent review, account for runner retirement
of 24 directories plus post-review 4,608 files/759 directories/three links:
combined 4,608 files/783 directories/three links, 62,990,856 logical and
75,264,000 allocated regular-file bytes, 5,394 identity rows. Raw outcomes,
automatic ledgers, corpus and live products retained; no product execution for
cleanup or acceptance-policy change.
[Report](reports/2026-10-30-164-consumed-json-state.md) and
[manifest](manifests/2026-10-30-164-consumed-json-state.jsonl.gz).


## CLN165 — consumed original process state (2026-10-30)

After complete execution and independent review: 4,608 regular files,
783 directories, three links, 62,990,856 logical / 75,264,000 allocated bytes,
5,394 identity rows. Includes three exact owned short socket environments,
30 empty directories; external paths are hashed. Original false aggregate,
111 passes/six platform skips, raw records, products and corpus retained.
No auto-retired state counted again and no broader cache root selected.
[Report](reports/2026-10-30-165-consumed-process-state.md) and
[manifest](manifests/2026-10-30-165-consumed-process-state.jsonl.gz).


## CLN166 — consumed original buffer and marker state (2026-10-30)

4,612files/852directories/3links;63,024,972logical/75,313,152allocated bytes,
5,467rows. Postreview4,612files/792dirs/3links plus48ledgered automatic childdirs
and12receipt-recorded emptyparents withoutfull predelete identities.840dirs
fully listed; missing metadata explicitly marked. Original1323passes/3skips,
false aggregate, raw records/corpus/products retained. CLN164 likewise has six
extra emptyparents beyond its original783ledgered directory total; manifest
unchanged. Next fix records existing runner parent identities before deletion.
[Report](reports/2026-10-30-166-consumed-buffer-marker-state.md) and
[manifest](manifests/2026-10-30-166-consumed-buffer-marker-state.jsonl.gz).


## CLN167 — consumed parent-ledger integration state (2026-10-30)

3,072 files/528 directories/twelve links; 41,993,904 logical/50,176,000
allocated bytes, 3,612 rows. Includes twenty already-retired directories under
schema 3, including all four parents with complete private pre-delete identities.
Post-review deletion removes the two consumed test copies, empty label trees
and context links. Original receipts, inputs, products and unrelated WIP remain.
Former unledgered parent removal moves into existing ledger-bound cleanup; old
manifests and disclosed historical metadata gaps remain unchanged.
[Report](reports/2026-10-30-167-consumed-parent-ledger-state.md) and
[manifest](manifests/2026-10-30-167-consumed-parent-ledger-state.jsonl.gz).

## CLN168 — consumed original thread/image coverage state (2026-10-30)

- Scope: three consumed full test copies and owned contexts/environments after
  twelve completed processes and independent raw/source review; original
  131 passes/thirteen skips, false aggregate and unequal status maps retained.
- Combined 4,608 files, 882 directories, eighteen links; 62,990,856 logical /
  75,264,000 allocated bytes, 5,508 rows. Sixty schema-3 automatic directories
  include twelve parents and are counted once; post-review directories 822.
- Full private pre-delete identities retained; public context targets hashed.
  Original evidence, source corpus, live products and trial App unchanged.
- [Report](reports/2026-10-30-168-consumed-thread-image-state.md) ·
  [manifest](manifests/2026-10-30-168-consumed-thread-image-state.jsonl.gz)
  (`ebebfe9e3069cc00cf2d307348d9aa4d7d9d96afb333863d37701e33592de7df`).

## CLN169 — consumed startup/regex coverage state (2026-10-30)

- Three consumed test copies/contexts/environments after nine completed
  processes and independent review; original 126 passes/117 skips, equal maps,
  aggregate false and owner exit 1 retained. No grammar acceptance claim.
- 4,608 files, 852 directories, eighteen links; 62,990,856 logical /
  75,264,000 allocated bytes, 5,478 rows. Thirty automatic schema-3 directories
  include six parents counted once; post-review directories 822.
- Full pre-delete private identities retained, public context targets hashed;
  raw evidence, pinned sources, live products and trial App unchanged.
- [Report](reports/2026-10-30-169-consumed-startup-regex-state.md) ·
  [manifest](manifests/2026-10-30-169-consumed-startup-regex-state.jsonl.gz)
  (`dcf32add98b2ba954c12d008e6c9fd2bbe49dad07757ac4985c73ac17cf1c31b`).

## CLN170 - consumed current grammar coverage state (2026-10-30)

- Three consumed full test copies/contexts/environments after original 120 passes
  and independent review. Previous skips/first preflight failure remain unchanged.
- 4,608 files, 792 directories, eighteen links; 62,990,856 logical / 75,264,000 allocated
  bytes, 5,418 rows. Thirty automatic schema 3 directories include six parents;
  post-review 762 directories. Count each once.
- Public manifest matches private pre-delete plans; link targets hashed. Raw
  evidence, new grammar inputs/libraries, live products and App retained unchanged.
- [Report](reports/2026-10-30-170-consumed-grammar-state.md) /
  [manifest](manifests/2026-10-30-170-consumed-grammar-state.jsonl.gz)
  (`49526b3c1bc07b45d541f8f78ebfa5d199f19b02a4ac450f575b5c21d1a48460`).

## CLN171 — 2026-10-31 consumed local-child state

P444 consumed cargo, objects/units, 554 new-process state groups, generated
component files and build-owner archive: 10,288 verified files, 3,885 directories,
405,732,494 logical / 426,631,168 allocated bytes. Three partner notes previously
removed: 13,089 reported bytes, content identities and allocated sizes unavailable;
absence/path hashes checked, no invented recovery. Combined manifest 14,176 rows;
verified and limited peer totals remain separate. Initial monitor exit125 before
deletion retained. Current/prior products, App, raw results and required inputs
unchanged. [Report](reports/2026-10-31-171-consumed-local-child-state.md),
[manifest](manifests/2026-10-31-171-consumed-local-child-state.jsonl.gz).

## CLN172 - Consumed original Lisp boundary state (2026-10-31)

After four-file independent raw review, retire 4,609 files, 885 directories and
eighteen links: 63,024,952 logical / 75,300,864 allocated bytes. This includes
one already-retired reference native trampoline and 93 automatic directories
(eighteen parents), counted once. Three test copies and empty timer state gone.
First cleanup assertion failure and partial-operation resume preserved; actual
pre-delete plans replace the invalid empty-environment assumption. Products,
inputs, raw evidence and tracked working changes unchanged. No test rerun.
[Report](reports/2026-10-31-172-consumed-lisp-boundary-state.md),
[manifest](manifests/2026-10-31-172-consumed-lisp-boundary-state.jsonl.gz).

## CLN173 — 2026-10-31 — Consumed ERT diagnostic state

- Remove one reviewed reference test copy, isolated state and six context links:
  1,537 files, 267 directories, six links, 1,810 rows;
  21,031,048 logical / 25,124,864 allocated bytes.
- Includes the 34,096-byte automatic native message trampoline, counted once;
  no earlier automatic removal. Raw results/logs, private plans, pinned inputs,
  live products, index and unrelated WIP remain unchanged.
- Single-ID attribution only; no whole-file, compatibility or speedup acceptance.
- [Report](reports/2026-10-31-173-consumed-ert-diagnostic-state.md),
  [manifest](manifests/2026-10-31-173-consumed-ert-diagnostic-state.jsonl.gz)
  SHA-256 `0b140a9a0fe6f446a51742c9f40909626212fd1c126c23ec90ac325b85d4460a`.

## CLN174 — 2026-10-31 — Consumed generator/advice/closure state

- Reviewed three-file normal coverage; retire 4,609 files, 855 directories and
  eighteen links, 5,482 rows; 63,024,984 logical / 75,300,864 allocated bytes.
- Includes one automatic native trampoline and 93 earlier runner directories
  (eighteen parents), counted once. Post-review 4,608 files/762 dirs/18 links.
- Three consumed test/environment/context scopes gone; original evidence,
  product/input identities, HEAD/index and unrelated WIP preserved. No rerun.
- [Report](reports/2026-10-31-174-consumed-generator-advice-state.md),
  [manifest](manifests/2026-10-31-174-consumed-generator-advice-state.jsonl.gz)
  SHA-256 `afee269c89aeb96d37fde3fe715ba42c39b3001dcf6b6a4360d094ab086ff5c6`.

## CLN175 — consumed macro expansion and variable test state (2026-10-31)

- Retired 4,618 files, 885 directories and eighteen links; 5,521 identity rows,
  63,086,871 logical / 75,390,976 allocated bytes. Three test trees include
  4,608 pinned inputs plus six generated bytecodes. Eighteen context links and
  empty environment labels are recorded. One automatic native trampoline and
  123 automatic directories (24 parents) count once; three consumed operational
  helpers are separately identified, with no live replay driver removed.
- All selected roots/helpers absent; original results/logs, private plans,
  inputs, products, HEAD/index and unrelated WIP unchanged. No whole-volume
  free-space, compiler-artifact equality, product or performance claim.
- [Report](reports/2026-10-31-175-consumed-macroexp-variable-state.md) and
  [manifest](manifests/2026-10-31-175-consumed-macroexp-variable-state.jsonl.gz).

## CLN176 - consumed small Lisp utility state (2026-10-31)

- After independent 102-case review, retire three pinned test copies, empty
  environment labels, eighteen context links and two consumed operational helpers.
- 4,611 files, 885 directories, eighteen links; 63,036,399 logical /
  75,317,248 allocated bytes; 5,514 manifest rows. Includes automatic reference
  trampoline and 123 directories (24 parents) exactly once; no automatic replay.
- Original evidence/private plans/replay driver/inputs/products and unrelated WIP
  remain unchanged. Selected roots and consumed helpers are absent.
- [Report](reports/2026-10-31-176-consumed-small-utility-state.md),
  [manifest](manifests/2026-10-31-176-consumed-small-utility-state.jsonl.gz),
  [coverage](../releases/2026-10-31-original-small-utilities.md).

## CLN177 - consumed editor helper state (2026-10-31)

- After independent 96-case review, retire three pinned test copies, empty
  environment labels, eighteen context links and two consumed operational helpers.
- 4,614 files, 1,011 directories, eighteen links; 63,138,697 logical /
  75,427,840 allocated bytes; 5,643 manifest rows. Includes four automatic reference
  trampolines and 249 directories (48 parents) once, without deletion replay.
- Original evidence/private plans/replay driver/inputs/products and unrelated WIP
  remain unchanged. Selected roots and consumed helpers are absent.
- [Report](reports/2026-10-31-177-consumed-editor-helper-state.md),
  [manifest](manifests/2026-10-31-177-consumed-editor-helper-state.jsonl.gz),
  [coverage](../releases/2026-10-31-original-editor-helpers.md).

### CLN178 — consumed Lisp inspection state (2026-10-31)

Retired 4,610 files, 1,002 directories and eighteen links, 63,002,214 logical /
75,280,384 allocated bytes. Post-review scope: 4,608 pinned files, 792 directories,
eighteen links; automatic scope: zero files, 210 directories including 42 parents,
counted once; two consumed helpers: 11,358 / 16,384 bytes. All selected roots and
helpers are absent. Original raw false/path error/empty seed map, separate file
finalization, skipped-case retained receipts, products, inputs and WIP remain.
Manifest has 5,630 rows; identity plans and targeted handle checks precede removal.
No source or App replacement; no aggregate/editor/compatibility/performance pass.
[Report](reports/2026-10-31-178-consumed-lisp-inspection-state.md) ·
[Manifest](manifests/2026-10-31-178-consumed-lisp-inspection-state.jsonl.gz).

### CLN179 — consumed Lisp debugging and storage state (2026-10-31)

Retired 4,615 files, 855 directories and eighteen links; 63,173,350 logical /
75,464,704 allocated bytes. Post-review: 4,608 pinned files, 762 directories,
eighteen links; automatic: five native trampolines, 93 directories including
eighteen parents, counted once; two consumed helpers: 12,006 / 16,384 bytes.
All selected roots/helpers are absent. Original results/events/logs, sealed
private plans, inputs, live products, HEAD/index and unrelated WIP are preserved.
The complete 5,488-row public manifest matches every private identity.
No production/protection/App change; no whole ERT/checking/full G3/performance pass.
[Report](reports/2026-10-31-179-consumed-lisp-debug-storage-state.md) ·
[Manifest](manifests/2026-10-31-179-consumed-lisp-debug-storage-state.jsonl.gz).

## CLN207 — 2026-11-03: sealed runtime state

- Canonically retire consumed state from 7,171 sealed successful leaves: 30,016 files / 43,629 directories, 761,576,592 logical bytes and 812,306,432 prior per-path allocated bytes; preserve raw observations and fixed inputs.
- Preserve the failed monitor attempt. A read-only census verifies 1,680 completed and 5,491 untouched scopes; the successor completes only the pending work, and independent receipt verification passes.
- Exclude the unsealed group and its incomplete case; preserve the original action's failure and missing completion. No editor, compiler or semantic case is started and no runtime or full compatibility acceptance is claimed.
- [Report](reports/2026-11-03-207-sealed-runtime-state.md); [manifest](manifests/2026-11-03-207-retired-sealed-ert-state.jsonl.gz).


## CLN208 — 2026-11-03: continued runtime state

- Publish the closed retirement of new runtime state: 11,200 files / 19,209 directories, 287,967,599 logical bytes and 306,872,320 prior per-path allocated bytes. Exclude CLN207's inherited scope.
- The explicit runtime union completes 903 groups / 61 profiles, with 7,171 inherited + 3,164 new processes. Preserve the original failed wrapper, absent completion and unsealed suffix; retain post_hoc provenance.
- Preserve raw observations, fixed inputs and product identities. Remaining actions, stable-file compatibility and performance have separate acceptance requirements.
- [Report](reports/2026-11-03-208-continued-runtime-state.md); [manifest](manifests/2026-11-03-208-retired-continued-ert-state.jsonl.gz).


## CLN210 — 2026-11-03: native format consumed state

- Project canonical consumed-state retirement for 608 sealed format cases: 0 files / 3,648 directories, 0 logical bytes and 0 prior per-path allocated bytes.
- Preserve 323 inherited + 285 fresh observations, original failed evidence, 15 excluded completed observations and the incomplete original case. Keep separate per-window budgets and explicit post_hoc provenance.
- Retain raw observations and signed fixed inputs; format controls do not imply complete compatibility, native lowering, performance or Goal acceptance.
- [Report](reports/2026-11-03-210-native-format-state.md); [manifest](manifests/2026-11-03-210-retired-native-format-state.jsonl.gz).


## CLN211 — 2026-11-03: consumed source-denied relocation copies

- Preserve a typed union of 60 fresh profiles in two bounded windows; do not claim a single whole-action invocation.
- Project canonical package retirement: 269,758 files / 30 links / 6,950 dirs, 16,199,717,574 logical and 16,816,988,160 prior per-path allocated bytes.
- Project 0 files / 1,104 case-state dirs; retain original packages, raw observations, owner records and receipts. No new deletion or whole compatibility/performance/Goal acceptance.
- [Report](reports/2026-11-03-211-source-denied-relocation-state.md); [packages](manifests/2026-11-03-211-retired-relocation-packages.jsonl.gz); [state](manifests/2026-11-03-211-retired-relocation-case-state.jsonl.gz).


## CLN212 — 2026-11-03: empty relocation work parents

- Retire exactly 63 closed, identity-bound empty work directories; select and remove no files.
- Preserve the unselected source-owner parent timestamp, original products, source, raw evidence and typed union. Do not count these parents again as package-copy retirement or claim exclusive physical reclamation.
- [Report](reports/2026-11-03-212-empty-relocation-work-parents.md); [manifest](manifests/2026-11-03-212-retired-empty-relocation-parents.jsonl.gz).



## 2026-11-03-213: private archive file controls

- Preserved the interrupted matrix; explicit dead-owner recovery and independent file verification passed.
- Retired 827 owned files / 140 directories and 12 control links; 10,045,093 logical / 11,784,192 per-path allocated bytes, with link allocation unknown.
- Four file-only sandbox controls passed; source/products and original failed observations remain.
- Original Lisp/full compatibility, performance and Goal acceptance remain open.


## CLN214 — 2026-11-04: redundant matrix copies

- Retired 74,850 exact duplicate files and 48 directories; retained their same-pair originals and all original failed observations.
- Recorded 1,292,812,683 logical and 1,458,839,552 prior per-path allocated bytes; no causal volume-reclamation or compatibility claim.
- [Report](reports/2026-11-04-214-matrix-duplicate-copies.md); [manifest](manifests/2026-11-04-214-retired-matrix-duplicate-copies.jsonl.gz).

## CLN215 — 2026-11-04: same-pair observation storage

- Shared extents for 128,218 inherited observation copies while preserving 4,737,742,271 logical bytes, independent inodes and every original outcome.
- File metadata and explicit inode transitions are bound; zero logical bytes deleted and no semantic cases replayed.
- [Report](reports/2026-11-04-215-same-pair-physical-storage.md); [manifest](manifests/2026-11-04-215-shared-same-pair-observations.jsonl.gz).

## CLN216 — 2026-11-04: immutable fixture storage

- Shared extents for 127,540 fixture copies while preserving 3,423,407,050 logical bytes and separate inodes; README, directories and context links remain unchanged.
- Real file integration, full transition verification and 11 subsequent scope protections passed. Original tests and failures remain; whole compatibility and performance acceptance are open.
- [Report](reports/2026-11-04-216-fixture-physical-storage.md); [manifest](manifests/2026-11-04-216-shared-stable-fixtures.jsonl.gz).
- CLN217: retired 429 consumed private native-tool cache files and 219 directories; retained frozen inputs and executable.

## CLN218 — 2026-11-05: retained test input storage

Recorded exact COW sharing of retained test inputs with explicit inode transitions and all original failures preserved. See [report](reports/2026-11-05-218-retained-test-input-storage.md).


## CLN219 — 2026-11-10: superseded build trees and App backups

Retired exactly 13,460 regular files,13 links and500 directories from three
superseded build/preparation roots and two historical App backups.13,973 public
identity rows match private identities and retirement ordinals;684,553,921
logical/715,595,776 prior per-path allocated bytes. New sources/products/package
and the active installed App remain unchanged. No causal volume-reclamation or
Goal completion claim. [Scope and recovery](reports/2026-11-10-219-superseded-build-app-backups.md).

## CLN220 — 2026-11-10: consumed runtime compiler copies

The completed current-source matrix retired compiler copies after all 40 build
legs:195460 files/6800 directories,6503836334 logical/6888628224 prior per-path
allocated bytes.202260 public identity rows bind all40 canonical ledgers.
Publication adds zero deletions; frozen inputs, finished products and original
runtime observations remain. [Scope and restoration](reports/2026-11-10-220-consumed-runtime-compiler-copies.md).

After review, a separate 41-row supplemental manifest records retirement of40
consumed audit mirrors and their empty directory:12350535 logical/12451840
per-path allocated bytes. All40 original ledgers remain unchanged; these counts
are additional to, and separate from, the compiler-copy projection above.

## CLN221 — 2026-11-10: consumed runtime semantic state

385596 public identity rows bind310964 regular files,30 links and74602
directories:17254112173 logical payload/17940475904 per-path allocated bytes.
This includes consumed case state, all60 copied packages and62 empty parents;
14 packages completed through separately bound late retirement after actual
indexer holders exited. Original source/products/observations and the failed
original phase remain unchanged. The main projection adds zero deletions.
After independent reconstruction of all11248 scopes,394 consumed audit mirrors
and their empty directory were canonically retired:158178794 logical/159260672
per-path allocated bytes, recorded separately in395 auxiliary public rows.
All11316 source pins and the main public manifest remain unchanged.
[Scope and reconstruction](reports/2026-11-10-221-consumed-runtime-semantic-state.md).

## CLN222 — superseded P453 build and App backups (2026-11-10)

After P458 functional/protective/local acceptance and independent review,
retired exactly3 owned backups:9,760 files,13 links and373 directories,
548,866,166 logical bytes and569,982,976 prior per-path allocated bytes.
All10,146 public rows match the bound plan and actual operations; all selected
roots are absent and8 protected source/product/release paths are unchanged.
The running installed App and frozen runtime matrix remain outside selection.
No whole-volume causal reclamation claim.
[Scope and recovery](reports/2026-11-10-222-superseded-p453-build-app-backups.md).

## CLN223 — consumed stable context copies (2026-11-10)

After P460's full original help-file functional acceptance and post-functional
protections, retired its three owned materialized corpus/context children:
1838 files,30 links and266 directories,48,913,383 logical bytes and53,628,928
prior per-path allocated bytes. All2134 public/plan/operation rows agree;
selected roots are absent and60 protected source/product/evidence files stayed
unchanged. Original receipts, current frozen products and the installed App
remain retained. No whole-volume causal reclamation claim.
[Scope and reconstruction](reports/2026-11-10-223-consumed-stable-context-copies.md).

## CLN224 — current runtime compiler copies (2026-11-10)

Published40 canonical ledgers from the clean9e257276 build:195,460 files and
6,800 directories,6,503,533,814 logical bytes,202,260 public rows. Independent
stream reconstruction agrees in every scope; publishing adds no original
deletion. Subsequently retired41 temporary metadata files and2 owned directories,
25,227,474 logical bytes, under separate41-row and2-row auxiliary manifests.
Original source/products/observations/ledgers and the public original are retained;
all selected temporary copies are absent. No causal whole-volume reclaim claim.
[Scope and reconstruction](reports/2026-11-10-224-consumed-runtime-compiler-copies.md).


## CLN225 — consumed official reference build inputs and transition state (2026-11-10)

- Scope: scoped native cleanup of the newly fetched reference Git cache and
  three patch links; two empty owned build roots; reviewed first-probe case
  state, accepted P461 draft and redundant working publication copies.
- Actual:5,664 files,3 links,446 directories;1,909,455,588 logical bytes and
  1,923,178,496 prior per-path allocated bytes. Main6098 and auxiliary15 public
  identity rows match private operation/absence evidence. Selected roots are
  absent; protected installed programs/dump/configuration/source/results are
  unchanged. No original failed result is promoted or original ledger deleted.
- Replacement/recovery: the official pinned installed reference and its bound
  recipe/install/probe evidence remain; reacquire the same recipe/source to
  rebuild. Shared caches/dependencies are preserved, no duplicate source archive
  retained, and no whole-volume causal reclamation is claimed.
- [Report and ledgers](reports/2026-11-10-225-consumed-reference-build-inputs.md).
  Independent final review accepted with zero confirmed defects; Goal remains open.

## CLN226 — consumed stable product-library corpus copies (2026-11-10)

After P464 functional acceptance and post-functional protections, retired four
owned generated corpus/context roots:3374 files,30 links,517 directories,
69,910,355 logical bytes and78,716,928 prior per-path allocated bytes. All3921
public/plan/operation rows agree through EOF; selected roots are absent and210
protected source/product/evidence paths are unchanged. A verifier child then
precedes canonical retirement of one redundant publication file and its owned
parent:319,929 logical bytes,323,584 prior allocated bytes, two auxiliary rows.
Full61-product/source/reference postflight passes; original failures and raw
observations remain unchanged. No whole-volume causal reclamation claim.
[Scope and recovery](reports/2026-11-10-226-consumed-stable-library-copies.md).
Independent final review accepted with zero confirmed defects.

## CLN227 — 已消费 ERC 诊断副本（2026-11-11）

诊断观测收集自然闭合后清退 2,942 普通文件、12 链接、594 目录，共 74,595,544 逻辑字节、81,768,448 先前逐路径分配字节。3,548 项公开身份绑定三处生成输入和四个自有短用例容器；46 项受保护输入/原始证据未变。原四份超时不完整回执、日志、事件及源和产品保持，独立核收待完成；不作整卷因果回收声明。

[范围、恢复与两份清单](reports/2026-11-11-227-consumed-erc-diagnostic-copies.md)。

## 2026-11-11 — CLN228: consumed unified G3 diagnostics

Retired six corpus/context copies and 14 closure-verified case containers: 5,503 nodes (4,723 files, 28 links, 752 directories), 119,294,423 logical / 131,117,056 prior allocated bytes. Private/public/operation streams align; all selected roots are absent and 199 protected files unchanged. Four bounded native images remain for independent forensic review. Raw failures and current products are retained; public streams were transferred without duplicate copies. [Report](reports/2026-11-11-228-consumed-unified-g3-diagnostics.md).


## CLN229 — 统一矩阵编译复制输入账本发布（2026-11-11）

已发布40份既有canonical退场账本的202,260行公开投影：195,460文件、6,800目录，6,504,468,694逻辑字节、6,888,607,744先前逐路径分配字节；全部scope计数与规范化SHA流式对账一致。本次新增删除0，原账本、源、产品和观测保留；发布流直接转移，40份有界审计镜像仅保留到整批独立终审后退场。实际编译源为未提交工作树，绑定b6eaf19f源清单；不把63b1e531 Git基线当作已提交的当前实现，不作整卷因果回收或Goal完成声明。[报告与清单](reports/2026-11-11-229-consumed-unified-compiler-copies.md)。

2026-11-13已消费镜像实际退场：40文件、1目录，12,280,223逻辑字节、12,369,920先前分配字节。真实verifier子进程闭合后canonical删除；41行辅助公开流逐行对齐私有流，目标物理缺席，82份原pin和主公开清单不变。本轮为主开发机械核验，未作新的独立搭档终审；在用原账本、产品和源码仍有验收消费者。

## CLN230 — 已消费的第二次 authored 输出（2026-11-13）

已清退两棵闭合失败运行输出及隔离容器：23,514文件、4,290目录、27,804节点，2,032,800,513逻辑字节、2,086,072,320先前分配字节。共享父目录占用导致首次保留；确认目标自身无占用并精确隔离后canonical完成，未终止外部进程。公开流与私有流逐行一致，目标缺席、4项构建/产品/源/参照输入未变；新完整语义运行继续。失败原因和聚合读数保留，原失败不升格。[报告与清单](reports/2026-11-13-230-consumed-authored-attempt.md)。

- CLN231 (2026-11-13): consumed interrupted authored output and isolated source integration retired:112712 files,81332 directories,3021261416 logical/3235766272 prior allocated bytes.194044 public/private rows align to EOF;5 protected inputs unchanged, selected scopes absent. First preflight locator failure caused no deletion; successor completed. See [report](reports/2026-11-13-231-consumed-interrupted-and-clean-source.md).

- CLN232 (2026-11-13): consumed native container images/diagnostic code and outputs retired:11 files,5 directories,4046572 logical/4075520 prior allocated bytes.16 public/private rows align, four protected inputs unchanged. See [report](reports/2026-11-13-232-consumed-bootstrap-analysis.md).


## CLN233 — consumed unified working set (2026-11-13)

Retired143,786 files,28,900 directories and14 link nodes:172,700 public/private rows,4,608,005,859 logical bytes and4,938,006,528 prior allocated bytes. All selected paths absent,13 protected identities unchanged. The first occupancy-protected attempt retained regular files/directories; a fresh clear query after controller exit allowed canonical completion. No original failure promotion or volume-level causal reclamation claim. [Report](reports/2026-11-13-233-consumed-unified-working-set.md).


## CLN234 — 已消费的源码门禁工作集（2026-11-13）

干净工具/获取副本、旧规则展开与功能回执以及实际绑定检查中间产物共12范围已canonical清退：1,959文件+313目录=2,272节点，67,316,734逻辑字节、72,286,208先前逐路径分配字节。公开/私有流至EOF对账，全部选中项物理缺席，13项当前矩阵、参照、最小回执、维护输入、标准产品及原index身份未变。目录入口准备拒绝与清退后投影失败均保留；修正后仅重发投影，不重复删除。当前首对原事件仍供后续消费，未接受完整稳定语料或Goal。[报告](reports/2026-11-13-234-consumed-source-gate-work.md)。

## CLN235 — 已消费的版本控制工作集（2026-11-13）

版本控制1..3、短路径定位1、23个闭合保留容器、已消费规则获取/旧展开/中间装配共41个范围，清退11,966文件、29链接、2,475目录，525,515,112逻辑字节/554,741,760先前分配字节。14,470行公开/私有/操作流至EOF一致、全部缺席、11项保护身份未变；控制器12.021秒、峰值43,171,840字节、退出0并完整回收。不宣称整卷因果回收或Goal完成。[报告](reports/2026-11-13-235-consumed-version-controls.md) · [清单](manifests/2026-11-13-235-consumed-version-controls.jsonl.gz)。


### CLN236 — 2026-11-13 已消费前置门禁工具工作集

干净31项集成、564-to27,072条源码展开与30项保护完成后，16范围清退3,552文件/587目录，共4,139节点、140,849,418逻辑字节及149,999,616先前分配字节。canonical公共/私有计划、公开投影和物理缺席核对通过，21保护身份未变；在途种子、Source及61产品不在选区。耗时4.266938秒，未运行编辑器，不作整卷因果回收或Goal完成宣称。[报告](reports/2026-11-13-236-consumed-prerequisite-work.md) · [公开清单](manifests/2026-11-13-236-consumed-prerequisite-work.jsonl.gz)。


### CLN237 — 2026-11-13 已消费短路径功能控制

P471功能及四项保护通过后，10范围清退1,958文件/14链接/317目录，2,289节点，128,923,878逻辑字节、133,894,144先前分配字节。两份原闭合保留状态先复验，再按精确无跟随计划删除；公开/私有/操作流全部对账，19项保护未变，2.374085秒。当前种子及产品未动。[报告](reports/2026-11-13-237-consumed-short-path-controls.md) · [公开清单](manifests/2026-11-13-237-consumed-short-path-controls.jsonl.gz)。


### CLN238 — 2026-11-13 已被替代首对工作集

旧首对的绑定消费者结束、harness身份已被P471替代，195范围退场（含191保留容器）：26,991文件/14链接/4 FIFO/4,201目录，共31,210节点、700,630,273逻辑字节及757,010,432先前分配字节。187原闭合证明复验；四棵无证明状态未用于复用。首次占用因两个000目录拒绝，删除0；按设备/inode恢复自有目录owner0700后重新准入，特殊FIFO单列处理。198,351条当前捕获路径无选区引用，26保护身份未变，三流与缺席完整对账，154.280228秒。历史8990/476及未接受状态保持，当前种子未动。[报告](reports/2026-11-13-238-replaced-first-stable-seed.md) · [公开清单](manifests/2026-11-13-238-replaced-first-stable-seed.jsonl.gz)。

- **CLN239 (2026-11-13)**：11文件版本控制的输出、五棵保留状态及两份日志共8范围退场；2,128文件、5链接、403目录，105,823,913逻辑/111,157,248先前分配字节。五闭合证明、19保护、2,536行三流与物理缺席核验通过；历史55目标通过不改标为新harness接受。见[报告](reports/2026-11-13-239-consumed-version-controls.md)。

- **CLN240 (2026-11-13)**：567规则干净展开后，候选、私有index、获取缓存、旧展开和已消费失败/日志共9范围退场；1,992文件、316目录，78,491,884逻辑/83,517,440先前分配字节。20保护及2,308行三流/缺席复验通过。当前27,216条定义仍供实际分类消费。见[报告](reports/2026-11-13-240-consumed-source-preparation.md)。

- 2026-11-13 CLN241：清退消费后的旧第二稳定种子、5组ERC对照、3组窗口诊断、207棵闭合状态及4处编译缓存，随后清退用完的审计工作副本；61,184节点、2,106,253,796逻辑字节，主操作86项保护身份未变，公开/私有流完整对账。当前制品、源码、SDK与WIP保留。[报告](reports/2026-11-13-241-consumed-stable-and-diagnostic-workset.md)。

- 2026-11-14 CLN242: user-authorized complete build/cache removal; 1,284,922 nodes, 73,148,183,830 logical bytes, 84 protected contents unchanged. Consumed raw audit copies also retired. [Report](reports/2026-11-14-242-cleared-build-workset.md).

- CLN243: consumed minimum bootstrap build and generator tools,7,566 nodes/599,010,894 logical bytes;86 protected contents and index unchanged. build/cache absent. [Report](reports/2026-11-14-243-consumed-bootstrap-workset.md).

- CLN244: removed the entire consumed minimum build and project cache again after scoped acceptance:38,504 files/2,661 directories/32 links,2,550,815,374 logical bytes. No occupied handles;89 source/WIP contents and index unchanged. build/cache absent, repository5.6GiB; no rebuild triggered by cleanup. [Report](reports/2026-11-14-244-consumed-minimum-workset.md).

- CLN245: removed consumed private diagnostic logs, review packages, drivers and retirement metadata: 1,218 files/86 directories, 412,750,738 logical bytes. Active final matrix, declared SDKs, current provenance and two minimal unresolved-case records preserved;91 existing source/WIP identities and main index unchanged. [Report](reports/2026-11-14-245-consumed-diagnostic-workset.md).

## CLN246 — 2026-11-14: consumed semantic case state

- Complete final authored semantics passed before cleanup; owner released and original process identities absent.
- Removed418 files/11,847 directories,831,059,469 logical bytes; all targets absent, no archive.94 source/WIP files and index unchanged;16 formal metadata and12,135 indexed evidence files unchanged.
- Current products/native fixtures/reference inputs retained for actual following consumers. [Report](reports/2026-11-14-246-consumed-semantic-state.md) · [compact scope](manifests/2026-11-14-246-consumed-semantic-state.json).

## CLN247 — 2026-11-14: consumed non-reusable case state

- Four Eglot/Ispell case containers had no valid closure-time reuse proof. Their original failed/skip observations and owner records remain unchanged; future execution is fresh.
- Removed139 files/81 directories/4 FIFOs,5,202,873 logical bytes. Two owned mode000 directories were made accessible only for no-holder verification and deletion. All roots absent;96 source/WIP files,12 original evidence files and index unchanged.
- [Report](reports/2026-11-14-247-consumed-nonreusable-case-state.md) · [compact scope](manifests/2026-11-14-247-consumed-nonreusable-case-state.json).


## CLN248 — consumed runtime-control attempts (2026-11-14)

Removed15 files/3 directories,76,478,435 logical bytes from obsolete failed controls and two logs. All selected paths absent;101 source/WIP files,7 formal results and index unchanged. Corrected controls retain actual consumer inputs only. [Report](reports/2026-11-14-248-consumed-runtime-control-attempts.md) · [scope](manifests/2026-11-14-248-consumed-runtime-control-attempts.json).


## CLN249 — explicit entire-build cleanup (2026-11-14)

Stopped/reaped the owned consumer, then removed the whole22GiB build tree and182 owned external test containers:396,554 files,32,198 directories,274 links;23,673,405,779 logical bytes. Build absent, repository5.6GiB;106 existing WIP identities and index unchanged before the documentation update. No archive or rebuild. Removed raw results/seed are unavailable for further acceptance or reuse. [Report](reports/2026-11-14-249-explicit-build-cleanup.md) · [scope](manifests/2026-11-14-249-explicit-build-cleanup.json).


## CLN250 — consumed retention fixtures (2026-11-14)

Removed three owned file/process test outputs and six consumed logs after P476 functional11/11 and protection2/2 acceptance. Build absent; no archive or relocation. [Report](reports/2026-11-14-250-consumed-retention-fixtures.md) · [scope](manifests/2026-11-14-250-consumed-retention-fixtures.json).


## CLN251 — consumed shell diagnostics (2026-11-14)

Removed seven P477 worksets and consumed current/prior closed raw logs after source/PTY/reference/protection consumers ended. Build absent; source/SDK/provenance/installation preserved. [Report](reports/2026-11-14-251-consumed-shell-diagnostics.md) · [scope](manifests/2026-11-14-251-consumed-shell-diagnostics.json).


## CLN252 — consumed build acquisition (2026-11-14)

Fresh7701-file export is complete; consumed corpus cache and duplicate pkg-config wrapper immediately retired. Active current source/builder and all WIP/index preserved. [Report](reports/2026-11-14-252-consumed-build-acquisition.md) · [scope](manifests/2026-11-14-252-consumed-build-acquisition.json).


## CLN253 — consumed matrix bootstrap cache (2026-11-14)

Canonical exact-cache retirement removed 439 files and223 directories (55543312 logical bytes), including the consumed private publication metadata. The active independent build driver, current frozen source/products and119 existing WIP contents/index remain unchanged. Two rejected preliminary scopes deleted no paths. [Report](reports/2026-11-14-253-consumed-matrix-bootstrap.md).


## CLN254 — consumed failed recovery metadata (2026-11-14)

Removed3 files/1 directory,70,412,631logical bytes after recording the SDK metadata failure; no new compiler product had been started. The failed functional result and original source/products remain. [Report](reports/2026-11-14-254-consumed-recovery-attempt.md).


## CLN255 — explicit build cleanup (2026-11-14)

Removed the entire ignored build tree:233,035 files/8,443 directories/252 symlinks,19,846,511,578 logical bytes. Build absent; repository5.6GiB. Existing126 WIP contents and HEAD/index preserved before documentation updates; no replacement build started. [Report](reports/2026-11-14-255-explicit-build-cleanup.md) · [scope](manifests/2026-11-14-255-explicit-build-cleanup.json).


## CLN256 — consumed recovery work (2026-11-14)

Removed13 files/4 directories,3646337 logical bytes: unused independent builder and consumed recovery/admission outputs. Build/cache absent; source/index unchanged. [Report](reports/2026-11-14-256-consumed-recovery-work.md).


## CLN257 — consumed process-owner tests (2026-11-14)

Removed4 consumed result files/4 directories,14615 logical bytes after final-source integration5/5 and protection2/2. Build/cache absent; maintained sources/index preserved. [Report](reports/2026-11-14-257-consumed-process-owner-tests.md).


## CLN258 — consumed current bootstrap inputs (2026-11-14)

Removed1976 files/476 directories,76539942 logical bytes after export and actual release builder completion. Active independent builder/source/index unchanged; no duplicate archive. [Report](reports/2026-11-14-258-consumed-current-bootstrap.md).


## CLN259 — consumed current matrix build driver

Fresh20/61 build naturally passed and its owner released. Removed2 files/3 directories,3,635,536 logical bytes; exact driver hash and clear scoped occupancy verified, source/build receipt/index unchanged. Current products remain for real runtime consumers; no archive or physical whole-volume claim. [Report](reports/2026-11-14-259-consumed-matrix-builder.md).


## CLN260 — consumed current Python helper work

Removed136 files/107 directories/2 links,245 nodes and4,074,354 logical bytes after compact build/reference/native/helper closure was recorded. Scoped occupancy clear and all selected paths absent; production/test source, frozen inventory, current gate receipts and index unchanged. Current products/native fixtures retained for actual consumers; no archive or physical reclamation claim. [Report](reports/2026-11-14-260-consumed-current-helper-work.md).


## CLN261 — consumed current semantic case state (2026-11-14)

Current `current-runtime-semantics-4` naturally passed and its owner released. Removed 418 files/11,847 directories, 830,520,493 logical bytes; scoped occupancy clear and all selected paths absent. 144 source/WIP files, Git index, 20 formal action metadata/results files and 12,135 indexed case evidence files reverified unchanged. Current products and reference/native fixtures remain for stable, performance and components consumers; no archive or whole-volume claim. [Report](reports/2026-11-14-261-consumed-semantic-state.md).


## CLN262 — superseded incomplete stable seed 4 (2026-11-15)

Superseded `current-runtime-stable-seed-4` (`542/555` files, runner mock-shell mismatch) retired after producer/consumer alignment. Removed 26,352 files/3,901 directories/12 links, 687,414,406 logical bytes across outer stage, inner source output, 200 retained case containers and 2 closed controller logs; scoped occupancy clear and all selected paths absent. 147 source/WIP files, Git index and all 5 Cohort-4 build/reference/prepare/semantics receipts reverified unchanged. [Report](reports/2026-11-15-262-superseded-stable-seed-4.md).


## CLN263 — consumed ERC graphical diagnostics (2026-11-15)

Stopped `current-runtime-cocoa-erc-all-controls-4` and consumed `diag-erc-graphical-check` retired after `unpopulated-graphical-snapshot-wrapper` and five Cocoa `--erc-graphical` scenario controls were wired. Removed 7,265 files/1,635 directories/156 links, 307,002,494 logical bytes across both diagnostic roots, 14 device/inode-verified retained case containers and 4 closed controller logs; scoped occupancy clear and all selected paths absent. 149 source/WIP files, Git index and all 8 Cohort-4 build/reference/prepare/semantics/library/tramp/cperl receipts reverified unchanged. [Report](reports/2026-11-15-263-consumed-erc-graphical-diagnostics.md) · [scope](manifests/2026-11-15-263-consumed-erc-graphical-diagnostics.json).


## CLN264 — consumed Cohort-4 diagnostics (2026-11-15)

Retired eight closed Cohort-4 diagnostic runs (`current-runtime-cocoa-controls-4`, `diag-cocoa-erc-4`, `diag-terminal-controls-4`, `diag-version-controls-4`, `diag-version-fast4-4`, `diag-version-fast4b-4`, `diag-cocoa-misc-4`, `diag-cocoa-image-curr-4`) after recording exact raw outcomes (preserving candidate `failed` SVG states, setuid `ps` `monitor_error/125`, and Cocoa persistent-state `timeout/124`). Removed 25,614 files/3,723 directories/213 links, 999,582,248 logical bytes across the 8 diagnostic roots, 100 retained case containers and 14 closed controller logs; scoped occupancy clear and all selected paths absent. 151 source/WIP files, Git index and remaining Cohort-4 receipts reverified unchanged. [Report](reports/2026-11-15-264-consumed-cohort4-diagnostics.md) · [scope](manifests/2026-11-15-264-consumed-cohort4-diagnostics.json).


## CLN265 — superseded Cohort-4 matrix and smoke workset (2026-11-15)

Retired the superseded pre-implementation Cohort-4 build/selection/prepare/semantics/control directories and Tramp `ps` smoke worksets across the entire `build` tree, 9 ended owned case containers and 10 closed controller logs after completing the consolidated 12-file implementation batch. Removed 370,986 files/28,006 directories/559 symlinks (198,442 immutable flags cleared), 22,805,642,941 logical bytes and 23,615,508,480 prior allocated bytes; `build` and all selected paths absent, zero open handles, no archive or whole-volume causal claim. 156 protected source/WIP files, HEAD and Git index verified unchanged before documentation updates; Cohort-5 rebuilds cleanly from maintained sources and pinned SDK inputs. [Report](reports/2026-11-15-265-superseded-cohort4-matrix-and-smoke.md) · [scope](manifests/2026-11-15-265-superseded-cohort4-matrix-and-smoke.json).


## CLN266 — consumed standalone build caches and smoke workset (2026-11-15)

Retired consumed standalone Cargo/unit/generation/manual build caches, transient Darwin saved-state directory, and verified `smoke-5` scripts/logs while retaining `build/terminal/port` and `dist/macos/Femacs.app`. Removed 18,716 files/1,065 directories/6 symlinks, 826,938,981 logical bytes and 872,280,064 prior allocated bytes; zero open handles and all selected paths absent, no archive or whole-volume causal claim. 160 protected source/WIP files, HEAD, Git index and all 6 retained standalone product binaries/dumps/clients verified unchanged. [Report](reports/2026-11-15-266-consumed-standalone-build-and-smoke.md) · [scope](manifests/2026-11-15-266-consumed-standalone-build-and-smoke.json).


## CLN267 — consumed Cohort-5 bootstrap and builder caches (2026-11-15)

After `current-runtime-build-5` naturally passed (`20/20` configs, `61` profiles, `41` physical products; `40` canonical compiler-copy retirements already reclaimed `195,460` files/`6,800` directories/`6,504,860,614` logical bytes) and released its artifact owner, removed the consumed Cargo bootstrap target tree, pinned corpus acquisition cache, and independent release build-driver cache. Removed 1,975 files/479 directories, 79,917,664 logical bytes and 85,430,272 prior allocated bytes; zero open handles and all selected paths absent, no archive or whole-volume causal claim. 162 protected source/WIP files, HEAD, Git index and all 4 Cohort-5 build receipts verified unchanged. [Report](reports/2026-11-15-267-consumed-cohort5-bootstrap-and-builder.md) · [scope](manifests/2026-11-15-267-consumed-cohort5-bootstrap-and-builder.json).


## CLN268 — consumed Cohort-5 semantic case state (2026-11-15)

After `current-runtime-semantics-5` naturally passed (`ert` 10,335/10,335, `negative` 608/608, `dump` 48/48, `relocation` 60/60) and released its artifact owner, removed only indexed case-parent `out` directories and the 48 dump-test profiles' generated `runtime-image.pdmp`, 32 `dump-native.eln` units and isolated environment directories. Removed 418 files/11,847 directories, 830,558,077 logical bytes and 831,426,560 prior allocated bytes; zero open handles and all selected paths absent, no archive or whole-volume causal claim. 164 protected source/WIP files, Git index, `current-runtime-semantics-5/results.json`, all 20 formal action metadata files and all 12,135 indexed case evidence files verified unchanged. [Report](reports/2026-11-15-268-consumed-cohort5-semantic-state.md) · [scope](manifests/2026-11-15-268-consumed-cohort5-semantic-state.json).


## CLN269 — superseded single-product GC candidate and rule checks (2026-11-16)

Retired the superseded single-product `build/runtime/p466-gc-1` candidate and two throwaway source-rule expansion checks after the complete same-source candidate matrix `current-runtime-build-6` naturally passed. Measured `du -sh` allocations at deletion: 977M, 41M, 41M; candidate receipt identities `b77c10ea…`/program `a3b69143…` were retained in the continuation record before deletion, and every active evidence directory (candidate cohort, reference binding, ERC/default/terminal/driver/classification/toolchain evidence) was kept. [scope](manifests/2026-11-16-269-superseded-single-product-gc-candidate.json).
