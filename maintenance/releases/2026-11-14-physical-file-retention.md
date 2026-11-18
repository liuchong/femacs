# Physical-file retention and cancellation — P476 (2026-11-14)

Runtime stages now use verified physical-file retention by default and accept only the explicit `files` mode. The duplicate whole-source/product archive writer has been removed. Retention still verifies content, permission mode and symlink identities, while immutable inputs are released through the existing owner guard.

The remote diagnostic records its owned case roots before launching. SIGINT/SIGTERM reaps the real child group, writes a cancelled/incomplete result, skips acceptance-only full-product rehashing and retention, then releases the guard. Successful collection still requires all seven postflight checks. Interrupted state remains explicitly named for scoped cleanup; it is never accepted as a completed control.

Functional integration passed11/11 through actual files, macOS immutable flags, CLI children and signal delivery:3.328 seconds,55,787,592 peak accounted bytes. The original failed attempt remained failed (exit1): two old fixture registries lacked the current61-entry/source-cohort contract, and the CLI fixture omitted declared build arguments. Updating those synthetic inputs produced the passing run; production admission was not weakened.

After functional success,2/2 key protections passed in0.270 seconds: `archive` is rejected before output creation, and the real cancelled-process result is rejected by the production classification reader. The protection receipt binds the functional result SHA256. These are owner/OS/tooling results with synthetic nonexecuted product files and a Python stand-in; they are not editor, original-corpus or performance acceptance.

All three consumed output trees and their six temporary logs were then removed underCLN250. `build/` is again physically absent. No VM or matrix rebuild was performed. P475's all49-product editor collection was cancelled underCLN249 and remains incomplete; no original failure, skip, count, exit or classification was changed. Full555/48, performance/GC, workflows, installed delivery and independent final acceptance remain open.

Maintained reproduction:

```sh
python3 tools/test-runtime-preservation-functional.py --out build/acceptance/retention-functional
python3 tools/test-runtime-preservation-functional.py --mode protection --functional build/acceptance/retention-functional/results.json --out build/acceptance/retention-protection
```

Use the current controlled resource runner around the commands. Consume their compact conclusions, release the owner and remove the generated outputs afterward. [Cleanup scope](../cleanup/manifests/2026-11-14-250-consumed-retention-fixtures.json).

Actual receipts and current maintained business/source identities:

```json
{
  "runs": [
    {
      "run": "p476-files-integration-1",
      "results_sha256": "191f7728e6e1c485426c39841d1478c4ae1df11a7ec00c945ef479f7b17a7e7f",
      "passed": false,
      "complete": true,
      "cases": [
        {
          "id": "completed-leg-child-files-release",
          "passed": true,
          "seconds": 0.207339
        },
        {
          "id": "error-scope-unwind",
          "passed": true,
          "seconds": 0.047746958000000006
        },
        {
          "id": "dead-owner-recovery",
          "passed": true,
          "seconds": 0.112205375
        },
        {
          "id": "sigterm-owned-producer-reaped",
          "passed": true,
          "seconds": 0.12479054100000003
        },
        {
          "id": "sigterm-during-journal-write",
          "passed": true,
          "seconds": 0.086755833
        },
        {
          "id": "frozen-source-action-boundaries",
          "passed": false,
          "seconds": 0.0054579580000000405
        },
        {
          "id": "mode-and-symlink-file-invariants",
          "passed": true,
          "seconds": 0.08365266599999999
        },
        {
          "id": "replaced-inode-release-boundary",
          "passed": true,
          "seconds": 0.111704625
        },
        {
          "id": "finished-product-execution-preservation",
          "passed": false,
          "seconds": 0.042481667
        },
        {
          "id": "default-stage-files-retention",
          "passed": false,
          "seconds": 0.04423404199999992
        },
        {
          "id": "remote-diagnostic-cancellation",
          "passed": true,
          "seconds": 1.659509417
        }
      ],
      "functional_acceptance_sha256": null,
      "controller": {
        "root_pid": 74060,
        "exit_code": 1,
        "cleanup_complete": true,
        "elapsed_seconds": 2.814,
        "peak_accounted_bytes": 70582272
      },
      "controller_log_sha256": "2683a91984ee82a6d4869ce6c119c655850cbcdbd01850e81f6b398981872d7a"
    },
    {
      "run": "p476-files-integration-2",
      "results_sha256": "c8126e946fec28e44de8336362ffb1e29126073c9a7d2821133a17b464c4f195",
      "passed": true,
      "complete": true,
      "cases": [
        {
          "id": "completed-leg-child-files-release",
          "passed": true,
          "seconds": 0.21083900000000003
        },
        {
          "id": "error-scope-unwind",
          "passed": true,
          "seconds": 0.05643854100000001
        },
        {
          "id": "dead-owner-recovery",
          "passed": true,
          "seconds": 0.125368584
        },
        {
          "id": "sigterm-owned-producer-reaped",
          "passed": true,
          "seconds": 0.14286512499999993
        },
        {
          "id": "sigterm-during-journal-write",
          "passed": true,
          "seconds": 0.09674129200000003
        },
        {
          "id": "frozen-source-action-boundaries",
          "passed": true,
          "seconds": 0.10883541699999999
        },
        {
          "id": "mode-and-symlink-file-invariants",
          "passed": true,
          "seconds": 0.09510845900000009
        },
        {
          "id": "replaced-inode-release-boundary",
          "passed": true,
          "seconds": 0.12509566699999997
        },
        {
          "id": "finished-product-execution-preservation",
          "passed": true,
          "seconds": 0.22219125000000006
        },
        {
          "id": "default-stage-files-retention",
          "passed": true,
          "seconds": 0.10446050000000007
        },
        {
          "id": "remote-diagnostic-cancellation",
          "passed": true,
          "seconds": 1.83837125
        }
      ],
      "functional_acceptance_sha256": null,
      "controller": {
        "root_pid": 77126,
        "exit_code": 0,
        "cleanup_complete": true,
        "elapsed_seconds": 3.328,
        "peak_accounted_bytes": 55787592
      },
      "controller_log_sha256": "422185b761ccb72a5edd8f9b05e032d733121d302976df8dd080b4e057a56c07"
    },
    {
      "run": "p476-files-protection-1",
      "results_sha256": "c6ef3b005e6d8559cae394d0e07288809cfc2429464ab74f5d3ff2101472bcdb",
      "passed": true,
      "complete": true,
      "cases": [
        {
          "id": "archive-selection-rejected-before-output",
          "passed": true,
          "seconds": 0.039781624999999994
        },
        {
          "id": "cancelled-collection-rejected-by-reader",
          "passed": true,
          "seconds": 0.010590291000000002
        }
      ],
      "functional_acceptance_sha256": "c8126e946fec28e44de8336362ffb1e29126073c9a7d2821133a17b464c4f195",
      "controller": {
        "root_pid": 81012,
        "exit_code": 0,
        "cleanup_complete": true,
        "elapsed_seconds": 0.27,
        "peak_accounted_bytes": 5144576
      },
      "controller_log_sha256": "e7e784fa47b83b19234249d33f9d39e965f06a607c3041f9729389ba4e5195d0"
    }
  ],
  "source_pins": {
    "tools/runtime-stage.py": "331711b3eefd9bd69fd6d8c8a2134741642cbca8210384fc79152b48bc11b7e4",
    "tools/runtime-owner.py": "caf300f810044f3e319c385e2d55f6b37b081804448be47b3b972528be7c13c5",
    "tools/runtime-tramp-process-probe.py": "7f72ee52b40d6abd482f4bb25775c4974acc2775b7c851c29e6bf8b64760db24",
    "tools/test-runtime-preservation-functional.py": "c3779b93f9a5c0a3b28545bd5524737c72ff6a501e17d29684780550188d0bf0"
  }
}
```
