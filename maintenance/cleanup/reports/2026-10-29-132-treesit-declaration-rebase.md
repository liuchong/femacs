# CLN132: Tree-sitter source ownership and stable rebase

P430 construction is in progress. This is source consolidation/rebasing, not
physical build-output retirement or a passing product acceptance. The
[manifest](../manifests/2026-10-29-132-treesit-declaration-rebase.json) records
source identities and restoration bounds.

The original feature-disabled tree copied a later development language cache.
The pinned stable 31.1 source re-resolves grammar paths/overrides on each
lookup; a cache keyed only by language symbol would ignore later configuration
changes once enabled. P430 removes that inactive development-only block and
its two loader calls, and restores the stable allocator wrapper. The exact old
block is present in the recorded Git parent. No delivered enabled capability
is retired; no performance adoption/rejection result is inferred.

Shared parser/node/query declarations now have one concrete header owner.
The dumper's separate void-pointer query stand-in and accessor were replaced
by that owner. The case conversion function's two local extern-only blocks
were replaced by shared declarations, function-scoped pre-edit caches and the
actual post-edit notification. Their exact removed block hashes and restoration
commit are recorded in the same manifest. Existing
unrelated work is preserved. Final source, build, tests, independent review and
coherent signed delivery remain pending; do not treat this report as completion.


2026-10-29 source-delivery update: the historical construction readings above
remain historical. The declaration changes are included in the complete
experimental runtime foundation group, whose exact proposed tree built four
macOS baseline products and passed the limited integration and protection
checks in [the foundation report](../../releases/2026-10-29-runtime-foundation.md).
This records source consolidation; it is not physical deletion, full compatibility
or native/generational-GC performance acceptance.
