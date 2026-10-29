# CLN142: consumed primitive test copies

After four-file original character/keymap/marker comparison and independent review,
retire only the three `test` roots and their three `context/test` links under
`build/acceptance/g3-editor-primitives-1`. They have no remaining consumer. Existing
`tools/runtime-retire.py` is reused; no editor, test or compiler is rerun.

## Exact scope and result

There are 4,608 regular files, 753 directories and three links. Each product
view has 1,536 manifest-verified originals and no generated resource files.
Logical file bytes total 62,990,856; allocation is 75,264,000 bytes. These
allocation totals do not prove the physical space reclaimed from the volume.

The public [manifest](../manifests/2026-10-30-142-consumed-editor-primitive-test-copies.jsonl.gz)
contains 5,364 rows, SHA-256
`29e66076350f5ce270eacfff0ebe722e99d856c436758e06991bebe17c8a03d1`.
All decoded file/directory/link fields match the captured private identity
plans. Link targets are repository-relative. All six selected roots/links
are absent after retirement.

## Safety, preservation and replay

The aggregate test owner exits 0 and fully reaps its tree. All twelve individual
file processes complete with exit 0, all 168 original observations pass and no
remaining group is terminated. Retirement uses those actual successfully
reaped process receipts, without rerunning or changing the accepted tests.
The existing helper checks targeted open handles, saves durable file ledgers
before deletion and revalidates each file's identity. Directory/link plans are
captured before their removal as well.

Original passing result, case events/timings, controller receipts,
input fingerprints and retained pinned corpus remain unchanged. Current
programs, selected dumps and complete selected Lisp libraries are rechecked
unchanged, as are Git HEAD/index and prior tracked working changes. No user
files, packages, actual products, unique sources or unrelated build roots are
deleted. This cleanup changes no acceptance policy or Goal status.

To restore, rematerialize separate test trees from the pinned source manifest
and cache, recreate each context link, then use the recorded products and
existing bridge. Original inode/time values are not promised; raw evidence is
retained but the former paths need materialization before replay. See
[the comparison and replay recipe](../../releases/2026-10-30-original-editor-primitives.md).
