# P448: complete own-voice finding reports

The reader detects names in full prose but stores a normalized 180-character
prefix for reporting. Re-searching that prefix could return no match and raise
before the remaining findings were printed. The current source contains a real
410-character case whose match begins at offset372.

Detection and display now share one explicit bare-name constant. The compiled
regex, reference/quotation rules, walks, finding counts and failure exit policy
are unchanged. No warning is suppressed and no exemption is added.

The real repository CLI completes with the same ten findings as the old-HEAD
detector's full multiset, exit1 and no traceback. This is a reporting repair;
the whole-tree policy gate remains false. Five complete small CLI inputs pass:
clean0, long tool1, long ported1, normalized context1 and multiple2 findings.
Integration took11.0225575s within the120s owner/30s process bounds,1GiB no growth.
No editor or compiler was started.

After integration, three reporter protections were added to the existing
selftest:19/19, including16 existing cases. File-walk/detection substitutes
isolate reporting in the three new components. The full source hash changes
only in the selftest body after integration; the production AST is identical.
Controller metrics remain primary-only. Original failure logs remain.

CLN185 retires16files/1directory,17rows,179,869logical/217,088allocated bytes:
nine adopted handoff copies whose exact native originals remain, three consumed
delivery helpers and four duplicate reader copies. Unique baseline and
pre-protection sources, five fixture input bodies, raw results/logs and private
plans remain. No product paths, unrelated WIP or historical manifests selected.
G1/G2/G4 remain3/7, Goal active; full G3/G5/G6/G7/performance remain open.

[Results](2026-10-31-own-voice-reporting-results.json) ·
[CLN185](../cleanup/reports/2026-10-31-185-consumed-own-voice-state.md).
