# Explicit stable runner selection (P461)

The frozen compiled cohort predates the accepted P460 source-context repair.
The stage coordinator can now select that corrected runner without rebuilding
products or editing the frozen source. Baseline: signed `93441b3d`.

## Behavior

`--stable-runner` applies only to `execute` with `all` or `stable`. Before
creating output it rejects unsafe/symlink paths, nonordinary files, empty input,
non-`.el` suffixes and input above1MiB. A bounded single-FD copy verifies source
identity before/after reading, preserves bytes/mode, and records the selected,
captured and original frozen runner identities. Only the owned captured copy
and selection receipt enter the existing artifact-owner protection lifecycle.

Default and explicit dispatch both bind the actual runner in the bounded
completion reader through its unique non-Python `harness_sources` entry. The
frozen writer/schema and stream-bound detailed receipt are unchanged. Postflight
rechecks selected/captured/receipt/frozen identities and all compiled products.
Any mutation prevents phase acceptance. The original555 files,48 candidates,
raw outcomes and compiled source/product identities are unchanged.

## Measured verification

Business implementation preceded complete functional integration. After that
passed, the20 critical protections were added and executed.

| Layer | Actual result | Seconds |
| --- | --- | --- |
| File/process/writer/reader integration | 8 cases,9 real Python children; owned artifact protection/release exercised | 0.399 |
| New post-functional protections | 20/20, bound to the accepted functional receipt | 0.559 |
| Existing phase integration | 5 cases,16 real Python children | 0.583 |
| Existing phase protections | 15/15 | 0.065 |
| Existing stable-scope protections | 16/16 | 0.138 |

Every test-driver invocation exited0 within its hard deadline and every owned synthetic
fixture was retired. Integration rejects an old runner, missing identity,
wrong digest and altered detailed receipt, while retaining original failed
outcomes. Protections cover CLI preflight, empty/nonstable scopes and mutations
to all four bound inputs. The selected source itself is never made immutable.

Private result SHA256 bindings:

| Result | SHA256 |
| --- | --- |
| Functional | `da1be64efbcf1384ec345f1c9ac16fcbb1b14732afc5b806e5f1aed9324b3662` |
| New protections | `f0965cf15f4db4d49061e785643b2ca07dbeee3433efa7ad29d037243c775957` |
| Phase integration | `f591951e193c65b55fa630458aae6ea8fbaad7f325d449b87460aebf35bca61c` |
| Phase protections | `5ea5fa5ce0e04e3a52927fba803035a84dce7b8b5a1c905c8f7b38aa0a855300` |
| Stable-scope protections | `620cb52d79617340f77252f853a7ad51a6562f2c8e089402dbbdea81bc893682` |

These are synthetic file/process/coordinator results, not actual editor,
compiler, original corpus or performance acceptance. P460 separately supplies
the real five-product help-file evidence. Independent final review accepts the
complete unit with zero confirmed defects.

## Current execution boundary

The `9e257276` cohort's authored semantics phase closed naturally and was
independently accepted before these Root tools changed:10,335 ERT/discovery
processes,608 negative cases,48 dump products and60 relocation products passed.
Its reference remains the historical31.1 epoch; changing the system reference
does not relabel those receipts. Full stable/performance and Goal remain open.
