# CLN256 — consumed recovery work (2026-11-14)

After CLN255 removed the whole build and the narrow occupancy integration passed, removed the unused independent build-driver cache and consumed recovery launches/logs/admission output. All selected scopes were idle before deletion. No build was restarted or restored.

Removed13 files/4 directories,3646337 logical bytes. The builder matched its preservation receipt; all selected identities are recorded in the [compact manifest](../manifests/2026-11-14-256-consumed-recovery-work.json). Source WIP and the Git index were unchanged by removal. The build directory and project build-driver cache are physically absent; dependency SDK/provenance and installed products remain outside this scope.

The interrupted matrix and both failed recovery attempts remain unaccepted. Their failure summaries and actual admission3/3 boundary are maintained in the [recovery note](../../releases/2026-11-14-controller-loss-recovery.md); consumed raw paths are no longer verification inputs. No archive or relocated copy was made.
