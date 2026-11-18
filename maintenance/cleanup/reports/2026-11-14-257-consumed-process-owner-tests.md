# CLN257 — consumed process-owner tests (2026-11-14)

After the final source passed5 functional and2 post-functional protection cases, removed the four consumed initial/final result directories. Their driver had already removed all owned synthetic repositories/scripts and reaped its launched processes. Each result directory was idle before deletion.

Removed4 files/4 directories,14615 logical bytes; [exact result identities](../manifests/2026-11-14-257-consumed-process-owner-tests.json). The two maintained sources and Git index are unchanged by cleanup. Production build and builder cache remain absent. No archive, compiler/editor run or full compatibility/performance claim was introduced.

The [compact maintained result and reproduction commands](../../releases/2026-11-14-runtime-owner-lifetime.md) distinguish immediate-parent cancellation from arbitrary controller detection, and actual successful guard release from forced nested-child termination. Consumed raw paths are no longer verification inputs.
