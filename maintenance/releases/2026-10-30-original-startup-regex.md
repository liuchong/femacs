# Original startup, regex and tree-sitter coverage

Three complete unchanged files from pinned stable commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` run on the installed macOS
reference and current normal terminal/Cocoa products. All nine file processes
complete with exit 0 and no assertion failure, timeout, protocol error or
remaining process group. Each product registers 81 IDs; all three ID/status
maps match. Total: **126 passes and 117 skips**. Original `passed=false` and
protected-owner exit 1 remain because files with skipped cases are not passed.

| Complete file | Each product | Three-product total |
| --- | --- | --- |
| `test/src/emacs-tests.el` | 7 skips | 21 skips |
| `test/src/regex-emacs-tests.el` | 34 passes | 102 passes |
| `test/src/treesit-tests.el` | 8 passes, 32 skips | 24 passes, 96 skips |

Startup's six SECCOMP feature guards fail before spawning children; its bwrap
guard finds no executable. No seccomp filter, nested editor or bwrap probe is
created. These are the original platform/tool guards, not startup sandbox
acceptance. The possible 1,048,576-byte filter and unbounded child calls were
identified before execution but not entered.

Regex discovery resolves the earlier static-count mistake: seventeen direct
IDs plus seventeen load-generated character-class IDs, not 35. The pinned
overlay is used; the differing tracked `test/src` copy is not substituted.
All four adjacent resources are checked against the manifest: BOOST 829 lines,
PCRE 2,386, PTESTS 344 and TESTS 167; longest line 162 bytes. Original internal
line whitelists remain, so passing these four IDs does not mean every external
dataset line was evaluated. No regex ID skips. The million-character
backtracking optimization, 65,535 repeat-limit and single-match historical
crash regressions return normally. Regex file walls are 0.307538, 0.204308 and
0.217763 seconds; these are coverage timings, not comparative performance gains.

**This is not tree-sitter grammar acceptance.** All products report the compiled
feature, but the original grammar guards fail before parser creation: JSON 23,
Python four, JavaScript two, Bash one, Elixir one, and HTML/CSS/JavaScript one
per product. No grammar is exercised. The eight passes cover indentation/range/
imenu helpers and three line/column cache primitives on small temporary buffers.
The line/column case creating a JSON parser skips. Parsing, node API, queries,
narrowing, cross-boundary edits, parser ranges and navigation remain unverified
by this run. Neither guards nor grammar availability are changed to erase skips.

R42: nine serial file processes, implemented 30-second load/case and 300-second
protected-owner deadlines, expected 10–90 seconds below 1 GiB. Driver wall time
is 6.319405167 seconds. The primary controller records exit 1, complete owned
cleanup, 8.680 seconds and 121,061,376 accounted peak bytes; those metrics are
primary-side receipts, not independently remeasured performance.

Independent source/raw-result review finds no confirmed execution defect.
Inputs, programs, dumps, full resources and trial App remain unchanged; no
generated resources, production edit, new protection test or App replacement.
This is normal batch coverage, not GUI, native, collector, full G3 or performance
acceptance. G1/G2/G4 remain **3/7**, original Goal active.

The [bounded receipt](2026-10-30-original-startup-regex-results.json) retains all
243 IDs/statuses/diagnostics/timings, input/product identities and raw/event
hashes. Raw evidence stays in `build/acceptance/g3-startup-regex-treesit-1`.
Replay uses the complete pinned corpus, explicit products/dumps and existing
stable-suite runner with unchanged guards/assertions and stated deadlines.
[CLN169](../cleanup/reports/2026-10-30-169-consumed-startup-regex-state.md)
records consumed state retirement; original skips and false aggregate remain.
