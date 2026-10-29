# CLN141: consumed primitive test copies

After six-file original primitive/Lisp-I/O comparison and independent review,
retire only the three `test` roots and their three `context/test` links under
`build/acceptance/g3-primitives-1`. They have no remaining consumer. Existing
`tools/runtime-retire.py` is reused; no editor, test or compiler is rerun.

## Exact scope and result

There are 4,608 regular files, 753 directories and three links. Each product
view has 1,536 manifest-verified originals and no generated resource files.
Logical file bytes total 62,990,856; allocation is 75,264,000 bytes. These
allocation totals do not prove the physical space reclaimed from the volume.

The public [manifest](../manifests/2026-10-30-141-consumed-primitive-test-copies.jsonl.gz)
contains 5,364 rows, SHA-256
`4515ac3f02e1cd44972b43c52010df3afee7a8523aabe52e9a90c36c526dd2f3`.
All decoded file/directory/link fields match the captured private identity
plans. Link targets are repository-relative. All six selected roots/links
are absent after retirement.

## Safety, preservation and replay

The aggregate test owner exits 1 because nine original skips keep the result
false; it fully reaps its tree. All eighteen individual file processes complete
with exit 0 and no remaining group terminated. Retirement uses those actual
successfully reaped process receipts, not a fabricated passing aggregate.
The existing helper checks targeted open handles, saves durable file ledgers
before deletion and revalidates each file's identity. Directory/link plans are
captured before their removal as well.

Original result/skips/false aggregate, case events/timings, controller receipts,
input fingerprints and retained pinned corpus remain unchanged. Current
programs, selected dumps and complete selected Lisp libraries are rechecked
unchanged, as are Git HEAD/index and prior tracked working changes. No user
files, packages, actual products, unique sources or unrelated build roots are
deleted. This cleanup changes no acceptance policy or Goal status.

To restore, rematerialize separate test trees from the pinned source manifest
and cache, recreate each context link, then use the recorded products and
existing bridge. Original inode/time values are not promised; raw evidence is
retained but the former paths need materialization before replay. See
[the comparison and replay recipe](../../releases/2026-10-30-original-primitives.md).
