# CLN249 — explicit build cleanup (2026-11-14)

The current user explicitly requested deletion of the build outputs. The owned49-product diagnostic was stopped and reaped before removal: controller exit143, cleanup complete;42 partial product records do not constitute completed acceptance.

Deleted the entire `build/` tree, including the frozen export,61-profile products, preparation inputs, raw run outputs, generated definitions and copied harnesses. Also removed182 task-owned retained test containers outside the repository, identified from their closure-time roots and inode records. Symbolic-link targets were not followed. No archive or relocated copy was created.

- `build/` measured22GiB before deletion and is physically absent.
- Removed396,554 files,32,198 directories and274 symbolic links;23,673,405,779 logical bytes. These per-node sums are not a claim of uniquely allocated or physically reclaimed bytes.
- Repository now measures5.6GiB. Volume availability changed from104GiB to125GiB during the work; concurrent activity prevents causal attribution of that whole-volume difference.
- All106 pre-existing source/WIP file contents, HEAD and Git index matched before the documentation update. Original source, the read-only reference checkout, installed reference and `dist/` were preserved.

Prior build/semantic/stable/control results are historical conclusions only. Their removed raw paths are unavailable for final verification or reuse; the deleted stable seed is not an eligible donor. Full555/48, performance, workflows, installation and independent final acceptance remain incomplete. The continuation record now marks the missing working set explicitly. Cleanup did not start another build.

Rebuild only for a concrete subsequent consumer using maintained source, declared inputs and [repository bootstrap](../../../docs/repository-bootstrap.md). Do not restore deleted caches or reproduce historical timestamps/results. [Compact scope](../manifests/2026-11-14-249-explicit-build-cleanup.json).
