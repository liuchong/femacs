# Owned shell startup and bounded reference diagnosis — P477 (2026-11-14)

The owned mock startup now finishes job-control initialization on the first following command and removes its DEBUG hook immediately. The transport identity is version2. Existing terminal noncanonical mode, disabled echo and ICRNL remain unchanged. Explicit later set-m still works, as shown by a positive control with a distinct child process group. Old transport observations are not same-environment donors.

The platform distributor's [matching-version shell source](https://github.com/apple-oss-distributions/bash/blob/51bf3fc6f26e9517c3a2e4bc3d208f9b39b87178/bash-3.2/shell.c) saves/disables/restores internal job control around startup-file processing. [Child launch](https://github.com/apple-oss-distributions/bash/blob/51bf3fc6f26e9517c3a2e4bc3d208f9b39b87178/bash-3.2/jobs.c) emits the setpgid warning only with internal job control enabled. This is source evidence for the initialization gap; it is not a claim that the published source was rebuilt into the installed binary, or that the exact historical warning cause has been proven.

Actual PTY functional acceptance passed2/2. The first attempt had already observed the correct process groups, but its hook check mistook the interactive shell's exit banner for a remaining hook; it stayed failed. The corrected check bounds the hook output between explicit markers. No passing assertion or raw result was rewritten.

The existing remote collector additionally accepts one explicitly receipt-bound installed reference without matrix products. This uses the same execution/fixture/retirement path, pins all declared original corpus inputs, verifies installation/product/input identities at exit and labels the result full_matrix_admission=false. Both formal collection readers reject this bounded proof. There is no implicit installed fallback.

The real reference diagnosis naturally closed in21.260 seconds,153,862,144 peak accounted bytes; all six declared postflight checks passed and owned case state retired. Eight ready-handler signal forms and three watch lifecycles passed. Original Tramp28 passed this diagnostic with no setpgid warning; original Tramp31 retained its immediate-liveness assertion failure. Original remote monitor validity skipped. Original unmount failed because the reference reported the parent directory instead of the watched file, also reproduced by the authored metadata control. That reference behavior is not correct or accepted. These focused runs do not execute the complete original Tramp file,555-file corpus or48 candidates, and do not replace the removed formal results.

Subsequent protections passed6/6: bounded editor evidence cannot enter matrix classification, four mixed admission options are rejected before output creation, and altered installation provenance is rejected. Existing mock input/mode/symlink protections passed2/2; the affected real-child cancellation regression also passed. A concurrent admission attempt was refused before execution by the resource guard, then the same group passed serially. No guard or memory restriction was bypassed.

All consumed test/input/diagnostic outputs and old closed controller logs were then deleted underCLN251; build is physically absent. The maintained source, declared SDK/provenance inputs and user installation remain. No compiler or matrix rebuild ran. Full compatibility, performance/GC, workflows, installation and independent final review remain incomplete. The DNS decision remains unanswered, and the historical shell-warning causality remains open until matching full-file observations resolve it.

Maintained entries: `tools/test-stable-mock-shell.py` (integration, then protection with its actual reference receipt), `tools/runtime-reference-probe.py capture-inputs`, and `tools/runtime-tramp-process-probe.py --reference-inputs`. Always declare installation paths explicitly and use the current resource runner. Consume and remove generated outputs at the end of their purpose. [Cleanup scope](../cleanup/manifests/2026-11-14-251-consumed-shell-diagnostics.json).

Compact actual observations and source identities:

```json
{
  "schema": "femacs-owned-shell-initialization-result-v1",
  "shell_profile": "owned-darwin-mock-noncanonical-icrnl-v2",
  "suite_source_sha256": "55757938ec93149740e8b95f49458187fd3c55d0ccdaeb502bcbfb630c7ee424",
  "actual_reference": {
    "results_sha256": "3f5e62bf1410321ebf224e9db06eaa0aa6f21a9282a0987b3503d56037efc292",
    "diagnostic_collection_complete": true,
    "full_matrix_admission": false,
    "checks": {
      "selected_reference_identity_unchanged": true,
      "reference_installation_inputs_unchanged": true,
      "declared_original_inputs_unchanged": true,
      "driver_and_fixture_sources_unchanged": true,
      "captured_fixture_unchanged": true,
      "original_test_unchanged": true
    },
    "signal_controls_passed": 8,
    "signal_controls": 8,
    "watch_lifecycles_passed": 3,
    "watch_lifecycles": 3,
    "watch_correct_behavior": false,
    "original_tramp_cases": [
      {
        "id": "tramp-test28-process-file",
        "status": "passed",
        "seconds": 1.1687898635864258,
        "process_file_calls": 33,
        "setpgid_warning_observed": false
      },
      {
        "id": "tramp-test31-signal-process",
        "status": "failed",
        "seconds": 0.32256317138671875,
        "process_file_calls": 0,
        "setpgid_warning_observed": false
      }
    ],
    "original_watch_cases": [
      {
        "id": "file-notify-test06-dir-validity-remote",
        "status": "skipped",
        "seconds": 0.7701210975646973
      },
      {
        "id": "file-notify-test12-unmount-remote",
        "status": "failed",
        "seconds": 14.456791877746582
      }
    ],
    "watch_rows": [
      {
        "id": "remove-watch",
        "valid_before": true,
        "valid_after": false,
        "passed": true,
        "events": [
          "stopped"
        ],
        "stopped_file_matches_watched": true
      },
      {
        "id": "delete-watched-directory",
        "valid_before": true,
        "valid_after": false,
        "passed": true,
        "events": [
          "stopped"
        ],
        "stopped_file_matches_watched": true
      },
      {
        "id": "emulated-unmount",
        "valid_before": true,
        "valid_after": false,
        "passed": true,
        "events": [
          "stopped"
        ],
        "stopped_file_matches_watched": false
      }
    ],
    "state_retired_before_output_cleanup": true
  },
  "upstream_source": {
    "repository": "apple-oss-distributions/bash",
    "commit": "51bf3fc6f26e9517c3a2e4bc3d208f9b39b87178",
    "version": "3.2.57(1)",
    "paths": [
      "bash-3.2/shell.c",
      "bash-3.2/jobs.c",
      "bash-3.2/flags.c",
      "version.h"
    ],
    "installed_binary_rebuilt_or_source_equivalence_claimed": false
  },
  "historical_warning_exact_cause_proven": false,
  "raw_results_modified": false,
  "original_failure_waivers": 0,
  "goal_complete": false,
  "runs": [
    {
      "run": "p477-mock-shell-integration-1",
      "results_sha256": "dd425f6389201f59cf5fb17737f48f47735e43499edb9818cba1860f4dc16398",
      "passed": false,
      "controller": {
        "root_pid": 9009,
        "exit_code": 1,
        "cleanup_complete": true,
        "elapsed_seconds": 0.271,
        "peak_accounted_bytes": 5144576
      },
      "cases": [
        {
          "id": "owned-initialization",
          "passed": false,
          "error": "AssertionError: DEBUG hook remained installed",
          "seconds": 0.054005875
        },
        {
          "id": "explicit-job-control-positive-control",
          "passed": false,
          "error": "AssertionError: DEBUG hook remained installed",
          "seconds": 0.047354625
        }
      ]
    },
    {
      "run": "p477-mock-shell-integration-2",
      "results_sha256": "000ed11f4b02e87275ab422625e9ba71fad8fcf9285e720b9c22988f14dc40bc",
      "passed": true,
      "controller": {
        "root_pid": 11052,
        "exit_code": 0,
        "cleanup_complete": true,
        "elapsed_seconds": 0.273,
        "peak_accounted_bytes": 81992
      },
      "cases": [
        {
          "id": "owned-initialization",
          "passed": true,
          "seconds": 0.04904725,
          "shell_pid": 11053,
          "child": {
            "pid": 11056,
            "pgid": 11053,
            "sid": 11053,
            "icanon": false,
            "echo": false,
            "icrnl": true
          },
          "self_removing_hook": true,
          "explicit_monitor_reenable": false
        },
        {
          "id": "explicit-job-control-positive-control",
          "passed": true,
          "seconds": 0.047512958999999994,
          "shell_pid": 11057,
          "child": {
            "pid": 11059,
            "pgid": 11059,
            "sid": 11057,
            "icanon": false,
            "echo": false,
            "icrnl": true
          },
          "self_removing_hook": true,
          "explicit_monitor_reenable": true
        }
      ]
    },
    {
      "run": "p477-reference-tramp-controls-1",
      "results_sha256": "3f5e62bf1410321ebf224e9db06eaa0aa6f21a9282a0987b3503d56037efc292",
      "passed": true,
      "controller": {
        "root_pid": 11112,
        "exit_code": 0,
        "cleanup_complete": true,
        "elapsed_seconds": 21.26,
        "peak_accounted_bytes": 153862144
      }
    },
    {
      "run": "p477-mock-shell-protection-1",
      "results_sha256": "b18eb3ff7f41df7b83666b0d8a7714743a780d2b791a795a010f61c9efe93f9a",
      "passed": true,
      "controller": {
        "root_pid": 19273,
        "exit_code": 0,
        "cleanup_complete": true,
        "elapsed_seconds": 0.27,
        "peak_accounted_bytes": 5406720
      },
      "cases": [
        {
          "id": "reference-only-collection-is-not-matrix-proof",
          "passed": true,
          "details": {
            "actual_editor_diagnostic_rejected_as_matrix": true
          },
          "seconds": 0.010822667000000001
        },
        {
          "id": "mixed-build-rejected",
          "passed": true,
          "details": {
            "returncode": 2,
            "output_absent": true
          },
          "seconds": 0.026449083999999998
        },
        {
          "id": "mixed-selection-rejected",
          "passed": true,
          "details": {
            "returncode": 2,
            "output_absent": true
          },
          "seconds": 0.025943042
        },
        {
          "id": "mixed-profile-rejected",
          "passed": true,
          "details": {
            "returncode": 2,
            "output_absent": true
          },
          "seconds": 0.026218207999999993
        },
        {
          "id": "mixed-include-reference-rejected",
          "passed": true,
          "details": {
            "returncode": 2,
            "output_absent": true
          },
          "seconds": 0.02664133299999999
        },
        {
          "id": "changed-installation-provenance-rejected",
          "passed": true,
          "details": {
            "changed_installation_provenance_rejected": true,
            "output_absent": true
          },
          "seconds": 0.0647025
        }
      ],
      "functional_acceptance_sha256": {
        "build/acceptance/p477-mock-shell-integration-2/results.json": "000ed11f4b02e87275ab422625e9ba71fad8fcf9285e720b9c22988f14dc40bc",
        "build/acceptance/p477-reference-tramp-controls-1/results.json": "3f5e62bf1410321ebf224e9db06eaa0aa6f21a9282a0987b3503d56037efc292"
      }
    },
    {
      "run": "p477-cancel-regression-1",
      "results_sha256": "04fe33c2209e8c23f3e109d46ed3ceedecaa9eb1cf30a7ba2cb4305b26d6d338",
      "passed": true,
      "controller": {
        "root_pid": 20612,
        "exit_code": 0,
        "cleanup_complete": true,
        "elapsed_seconds": 3.139,
        "peak_accounted_bytes": 57065472
      },
      "case": "remote-diagnostic-cancellation",
      "seconds": 2.9038110830000003,
      "owned_child_reaped": true
    }
  ],
  "existing_environment_protection_regression": {
    "passed": true,
    "tests": 2,
    "failures": 0,
    "errors": 0,
    "skipped": 0,
    "case_seconds": [
      {
        "test": "__main__.MockShellEnvironmentContracts.test_changed_or_symlinked_mock_startup_file_is_rejected",
        "seconds": 0.0011159160000000015
      },
      {
        "test": "__main__.MockShellEnvironmentContracts.test_only_named_darwin_mock_files_receive_identical_owned_initialization",
        "seconds": 0.002706333000000005
      }
    ],
    "results_sha256": "ec4f2f6246f587550d4e42b6cb84f0210f934f0ace07aff7c6b608f73468183a",
    "controller": {
      "root_pid": 21785,
      "exit_code": 0,
      "cleanup_complete": true,
      "elapsed_seconds": 0.276,
      "peak_accounted_bytes": 5390336
    }
  },
  "serial_admission_attempt": {
    "started": false,
    "exit_code": 125,
    "error_code": "serial_probe_required",
    "resolved": "Ran the same group serially after the other controlled child exited."
  },
  "current_source_sha256": {
    "tools/stable-suite.py": "55757938ec93149740e8b95f49458187fd3c55d0ccdaeb502bcbfb630c7ee424",
    "tools/runtime-tramp-process-probe.py": "8787d46101f9a21f6ae927038f412c09ba667b55fa864b9bba36b70d081c1981",
    "tools/stable-classification.py": "d116ba6a02e4cd6ec580a38f086bc558c57359cb48b9c1093d44a16876961910",
    "tools/test-stable-mock-shell.py": "d7f3903e2eeb45585b3c9c6c0561cecffb9e158b1be7c323e55f3665bb77026c"
  }
}
```
