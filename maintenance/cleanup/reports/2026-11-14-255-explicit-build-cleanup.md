# CLN255 — explicit build cleanup (2026-11-14)

Deleted the entire ignored `build/` directory under the current explicit user instruction. The last recovery attempt had already exited1 with controller cleanup complete; its recovery subprocess never started. A final process check found no live build task, and the scoped open-handle query exited1 with empty output.

- Before deletion: 18,886,800 KiB (about18GiB); after deletion: directory physically absent.
- Removed233,035 files,8,443 directories and252 symbolic links;19,846,511,578 logical bytes. Symbolic-link targets were not followed; no archive or relocated copy was created.
- Repository now measures5.6GiB. Observed volume availability increased by19,195,850,752 bytes during deletion; concurrent activity prevents attributing that whole-volume change solely to this cleanup.
- All126 existing WIP file contents, HEAD and Git index matched before this documentation update. Original source, read-only reference checkout, installed products and `dist/` were preserved.

The interrupted matrix, its partial reuse candidates and both failed recovery outputs are now absent. They cannot be admitted as current products, eligible donors or live validation inputs. Recovery2 remained failed because the original output occupancy could not be established as clear at admission; no successful recovery is claimed. Required future work must build from maintained source and declared inputs into fresh output. Cleanup starts no replacement build and does not change the remaining modernization acceptance requirements.

[Compact scope](../manifests/2026-11-14-255-explicit-build-cleanup.json).
