#!/usr/bin/env python3
"""Bounded report of one existing runtime performance result.

Read a ``femacs-runtime-execution-v1`` document whose action is ``performance``
and emit Markdown or JSON. Summary figures are copied from the input summary.
The tool does not run a workload, build a product, recompute statistics, or
turn a zero error count into performance acceptance.
"""
from __future__ import annotations

import argparse
import json
import math
import os
from pathlib import Path
import stat
import sys

MAX_INPUT_BYTES = 16 * 1024 * 1024
MAX_PROCESSES = 8192
MAX_SAMPLES = 8192
MAX_STARTUPS = 8192
MAX_CASES = 1024
MAX_COMPARISONS = 1024
MAX_ERRORS = 1024
MAX_EXCLUDED = 256
MAX_TOKEN = 80
MAX_TEXT = 160
# Existing summary invariant from the performance writer. Used only to reject
# a summary that disagrees with itself. It is not a new acceptance threshold.
MEASURED_ROUNDS = 7
ABSURD_MAGNITUDE = 1e15
# Same bound as an integer so a huge JSON integer is rejected before float().
ABSURD_LIMIT = 10**15
MAX_COUNT = 1_000_000
MAX_RSS = 2**62
READ_CHUNK = 1024 * 1024

SCHEMA = "femacs-runtime-execution-v1"
ACTION = "performance"
REPORT_SCHEMA = "femacs-runtime-measurement-report-v1"
SCOPE = (
    "selected input only; copied summary; not a speedup, compatibility, "
    "or performance-acceptance result"
)
LIMITATION = (
    "measurement collection only; busy/inconclusive data, unresolved "
    "compatibility and required layout comparisons prevent promotion"
)

TOP_KEYS = {
    "schema", "action", "processes", "samples", "startups", "summary",
    "comparisons", "errors", "excluded", "fixture_sources",
    "fixtures_unchanged", "passed", "performance_accepted", "limitation",
    "full_product_inputs_unchanged", "driver_sources_unchanged",
    "goal_complete", "compatibility_accepted", "state_retirement_complete",
}
FALLBACK_KEYS = {"permanent_full_fallback", "startup_measured"}
SUMMARY_KEYS = {"samples", "cases", "failures", "inconclusive"}
CASE_FIELDS = (
    "case", "mode", "execution_identity", "n", "median_ms", "mad_ms",
    "min_ms", "max_ms", "range_ms", "pause_count", "max_pause_ms",
    "p95_pause_ms", "p99_pause_ms", "peak_rss_bytes", "max_load_per_core",
    "quiet", "correctness", "inconclusive", "reasons",
)
REFERENCE_KEYS = {
    "reference", "candidate", "case", "eligible", "interval", "mode",
    "paired_rounds",
}
LAYOUT_KEYS = {
    "kind", "major", "young", "case", "mode", "paired_rounds", "interval",
    "eligible",
}
INTERVAL_KEYS = {"median_delta_ms", "low_ms", "high_ms", "reliable_gain"}
ERROR_KEYS = {"profile", "mode", "observer", "round", "warmup", "error"}
EXCLUDED_KEYS = {"profile", "reason"}
MODES = {"interpreted", "bytecode", "native"}
OBSERVERS = {"off", "phases", "survival", "all"}
BOOL_FIELDS = (
    "fixtures_unchanged", "passed", "performance_accepted",
    "full_product_inputs_unchanged", "driver_sources_unchanged",
    "goal_complete", "compatibility_accepted", "state_retirement_complete",
)
FORBIDDEN_TEXT = (
    "heap_before", "heap_after", "phase_snapshot", "pauses_ms",
    "evidence_directory", "fixture_sources", "median_delta_ms",
    "sample_errors", "allocation_delta", "phase_snapshot",
)
FORBIDDEN_KEYS = {
    "argv", "cwd", "pid", "processes", "startups", "samples", "errors",
    "interval", "workloads", "heap_before", "heap_after", "phase_snapshot",
    "pauses_ms", "fixture_sources", "evidence_directory", "command",
}


class ReportError(Exception):
    def __init__(self, code):
        super().__init__(code)
        self.code = code


def malformed(where):
    raise ReportError("malformed-report: " + where)


def reject_nonfinite(_token):
    raise ReportError("non-finite-number")


def parse_finite_float(text):
    """Reject exponent overflow such as 1e400 without echoing the token."""
    try:
        value = float(text)
    except (ValueError, OverflowError):
        raise ReportError("non-finite-number") from None
    if not math.isfinite(value):
        raise ReportError("non-finite-number")
    return value


def assert_regular_file(path):
    absolute = Path(os.path.abspath(path))
    current = Path(absolute.anchor)
    info = None
    for part in absolute.parts[1:]:
        current = current / part
        try:
            info = current.lstat()
        except OSError:
            raise ReportError("not-regular-file") from None
        if stat.S_ISLNK(info.st_mode):
            raise ReportError("not-regular-file")
    if info is None or not stat.S_ISREG(info.st_mode):
        raise ReportError("not-regular-file")
    return absolute, info


def read_at_most(fd, limit):
    """Read no more than limit bytes. A later size change cannot extend this."""
    remaining = limit
    chunks = []
    while remaining > 0:
        try:
            block = os.read(fd, min(remaining, READ_CHUNK))
        except OSError:
            raise ReportError("not-regular-file") from None
        if not block:
            break
        if len(block) > remaining:
            block = block[:remaining]
        chunks.append(block)
        remaining -= len(block)
    return b"".join(chunks)


def load_document(path, max_bytes=MAX_INPUT_BYTES):
    if type(max_bytes) is not int or max_bytes <= 0 or max_bytes > MAX_INPUT_BYTES:
        raise ReportError("input-too-large")
    absolute, info = assert_regular_file(path)
    if info.st_size > max_bytes:
        raise ReportError("input-too-large")
    # O_NONBLOCK covers only this open. A FIFO swapped in after lstat must not
    # block before fstat can reject it. Regular-file reads are unchanged.
    try:
        descriptor = os.open(
            absolute, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC | os.O_NONBLOCK)
    except OSError:
        raise ReportError("not-regular-file") from None
    try:
        try:
            opened = os.fstat(descriptor)
        except OSError:
            raise ReportError("not-regular-file") from None
        if not stat.S_ISREG(opened.st_mode) or stat.S_ISLNK(opened.st_mode):
            raise ReportError("not-regular-file")
        if opened.st_dev != info.st_dev or opened.st_ino != info.st_ino:
            raise ReportError("input-changed")
        if opened.st_size > max_bytes:
            raise ReportError("input-too-large")
        raw = read_at_most(descriptor, max_bytes + 1)
        try:
            after = os.fstat(descriptor)
        except OSError:
            raise ReportError("not-regular-file") from None
    finally:
        os.close(descriptor)
    if len(raw) > max_bytes:
        raise ReportError("input-too-large")
    if (after.st_dev != info.st_dev or after.st_ino != info.st_ino
            or not stat.S_ISREG(after.st_mode) or stat.S_ISLNK(after.st_mode)
            or after.st_size != info.st_size or len(raw) != info.st_size):
        raise ReportError("input-changed")
    try:
        text = raw.decode("utf-8")
    except UnicodeError:
        raise ReportError("invalid-json") from None
    try:
        document = json.loads(
            text, parse_float=parse_finite_float, parse_constant=reject_nonfinite)
    except ReportError:
        raise
    except RecursionError:
        raise ReportError("invalid-json") from None
    except (json.JSONDecodeError, ValueError, TypeError, OverflowError):
        raise ReportError("invalid-json") from None
    if type(document) is not dict:
        malformed("top")
    return document


def finite_number(value, where, *, nonnegative):
    if type(value) is int:
        magnitude = -value if value < 0 else value
        if magnitude > ABSURD_LIMIT:
            malformed(where)
        if nonnegative and value < 0:
            malformed(where)
        return value
    if type(value) is not float:
        malformed(where)
    try:
        finite = math.isfinite(value)
        oversized = abs(value) > ABSURD_MAGNITUDE
    except OverflowError:
        raise ReportError("malformed-report: " + where) from None
    if not finite:
        raise ReportError("non-finite-number")
    if nonnegative and value < 0:
        malformed(where)
    if oversized:
        malformed(where)
    return value


def require_bool(value, where):
    if type(value) is not bool:
        malformed(where)
    return value


def require_count(value, where, limit, minimum=0):
    if type(value) is not int or value < minimum or value > limit:
        malformed(where)
    return value


def require_token(value, where, allowed=None):
    if type(value) is not str or not value or len(value) > MAX_TOKEN:
        malformed(where)
    if any(ord(char) < 32 or char in "/\\|`\"'" for char in value):
        malformed(where)
    if allowed is not None and value not in allowed:
        malformed(where)
    return value


def require_case(value, where):
    text = require_token(value, where)
    if text[:1].isdigit() or "--" in text:
        malformed(where)
    for char in text:
        if char != "-" and not char.islower() and not char.isdigit():
            malformed(where)
    return text


def require_identity(value, where):
    text = require_token(value, where)
    if not text[0].isalnum():
        malformed(where)
    for char in text:
        if char != "." and char != "_" and char != "-" and not char.isalnum():
            malformed(where)
    return text


def require_execution_identity(value, where):
    text = require_token(value, where)
    product, separator, observer = text.rpartition(":")
    if separator != ":" or observer not in OBSERVERS:
        malformed(where)
    require_identity(product, where)
    return text


def require_mode(value, where):
    return require_token(value, where, MODES)


def expected_reasons(count, quiet):
    reasons = []
    if count < MEASURED_ROUNDS:
        reasons.append("fewer-than-seven-rounds")
    if quiet is False:
        reasons.append("busy-host")
    return reasons


def optional_pause(value, where, present):
    if present:
        finite_number(value, where, nonnegative=True)
    elif value is not None:
        malformed(where)


def validate_case(row, where):
    if type(row) is not dict or set(row) != set(CASE_FIELDS):
        malformed(where)
    case = require_case(row["case"], where + ".case")
    mode = require_mode(row["mode"], where + ".mode")
    execution_identity = require_execution_identity(
        row["execution_identity"], where + ".execution_identity")
    count = require_count(row["n"], where + ".n", MAX_COUNT, minimum=1)
    for key in ("median_ms", "mad_ms", "min_ms", "max_ms", "range_ms",
                "max_load_per_core"):
        finite_number(row[key], where + "." + key, nonnegative=True)
    pause_count = require_count(row["pause_count"], where + ".pause_count", MAX_COUNT)
    optional_pause(row["max_pause_ms"], where + ".max_pause_ms", pause_count != 0)
    optional_pause(row["p95_pause_ms"], where + ".p95_pause_ms", pause_count >= 20)
    optional_pause(row["p99_pause_ms"], where + ".p99_pause_ms", pause_count >= 100)
    require_count(row["peak_rss_bytes"], where + ".peak_rss_bytes", MAX_RSS)
    quiet = require_bool(row["quiet"], where + ".quiet")
    correctness = require_bool(row["correctness"], where + ".correctness")
    inconclusive = require_bool(row["inconclusive"], where + ".inconclusive")
    reasons = row["reasons"]
    if type(reasons) is not list or reasons != expected_reasons(count, quiet):
        malformed(where + ".reasons")
    if inconclusive is not bool(reasons):
        malformed(where + ".inconclusive")
    copied = {key: row[key] for key in CASE_FIELDS}
    copied["reasons"] = list(reasons)
    return copied, (execution_identity, mode, case)


def validate_interval(value, where):
    if value is None:
        return
    if type(value) is not dict or set(value) != INTERVAL_KEYS:
        malformed(where)
    for key in ("median_delta_ms", "low_ms", "high_ms"):
        finite_number(value[key], where + "." + key, nonnegative=False)
    require_bool(value["reliable_gain"], where + ".reliable_gain")


def validate_comparison(row, index, groups, error_free, seen):
    where = "comparisons[{0}]".format(index)
    if type(row) is not dict:
        malformed(where)
    keys = set(row)
    if keys == REFERENCE_KEYS:
        kind = "reference"
        left_id = require_identity(row["reference"], where + ".reference")
        right_id = require_identity(row["candidate"], where + ".candidate")
    elif keys == LAYOUT_KEYS:
        if row.get("kind") != "same-layout-major-young":
            malformed(where + ".kind")
        kind = "layout"
        left_id = require_identity(row["major"], where + ".major")
        right_id = require_identity(row["young"], where + ".young")
    else:
        malformed(where)
    mode = require_mode(row["mode"], where + ".mode")
    case = require_case(row["case"], where + ".case")
    paired = require_count(row["paired_rounds"], where + ".paired_rounds", MAX_COUNT)
    validate_interval(row["interval"], where + ".interval")
    eligible = require_bool(row["eligible"], where + ".eligible")
    identity = (kind, left_id, right_id, mode, case)
    if identity in seen:
        malformed(where)
    seen.add(identity)
    left = groups.get((left_id + ":off", mode, case))
    right = groups.get((right_id + ":off", mode, case))
    if left is None or right is None:
        malformed(where)
    expected = (paired == MEASURED_ROUNDS and left["quiet"] is True
                and right["quiet"] is True and error_free)
    if eligible is not expected:
        malformed(where + ".eligible")
    return eligible


def validate_error(row, index):
    where = "errors[{0}]".format(index)
    if type(row) is not dict or set(row) != ERROR_KEYS:
        malformed(where)
    require_identity(row["profile"], where + ".profile")
    require_mode(row["mode"], where + ".mode")
    require_token(row["observer"], where + ".observer", OBSERVERS)
    require_count(row["round"], where + ".round", 10000, minimum=-100)
    require_bool(row["warmup"], where + ".warmup")
    message = row["error"]
    if type(message) is not str or len(message) > 4000:
        malformed(where + ".error")


def validate_excluded(row, index):
    where = "excluded[{0}]".format(index)
    if type(row) is not dict or set(row) != EXCLUDED_KEYS:
        malformed(where)
    require_identity(row["profile"], where + ".profile")
    if row.get("reason") != "diagnostic test-hooks product":
        malformed(where + ".reason")


def bounded_object_list(value, where, limit):
    if type(value) is not list or len(value) > limit:
        raise ReportError("too-many-rows" if type(value) is list else "malformed-report: " + where)
    for index, item in enumerate(value):
        if type(item) is not dict:
            malformed("{0}[{1}]".format(where, index))
    return value


def guard_report(report):
    def walk(value):
        if type(value) is dict:
            for key, item in value.items():
                if key in FORBIDDEN_KEYS:
                    raise ReportError("unsafe-text")
                walk(item)
        elif type(value) is list:
            for item in value:
                walk(item)
        elif type(value) is str:
            if (len(value) > MAX_TEXT or "\\" in value or "/Users/" in value
                    or value.startswith("/") or "\n" in value):
                raise ReportError("unsafe-text")
            for marker in FORBIDDEN_TEXT:
                if marker in value:
                    raise ReportError("unsafe-text")
        elif type(value) in (bool, int) or value is None:
            return
        elif type(value) is float:
            if not math.isfinite(value):
                raise ReportError("non-finite-number")
        else:
            raise ReportError("unsafe-text")
    walk(report)


def build_report(document):
    if type(document) is not dict or set(document) not in (TOP_KEYS, TOP_KEYS | FALLBACK_KEYS):
        malformed("top")
    if document.get("schema") != SCHEMA:
        raise ReportError("schema-mismatch")
    if document.get("action") != ACTION:
        raise ReportError("action-mismatch")
    flags = {key: require_bool(document[key], key) for key in BOOL_FIELDS}
    if FALLBACK_KEYS <= set(document):
        permanent_full_fallback = require_bool(
            document["permanent_full_fallback"], "permanent_full_fallback")
        startup_measured = require_bool(
            document["startup_measured"], "startup_measured")
        if startup_measured is not (not permanent_full_fallback):
            malformed("startup_measured")
    else:
        permanent_full_fallback = False
        startup_measured = True
    if document.get("limitation") != LIMITATION:
        malformed("limitation")
    processes = bounded_object_list(document["processes"], "processes", MAX_PROCESSES)
    samples = bounded_object_list(document["samples"], "samples", MAX_SAMPLES)
    startups = bounded_object_list(document["startups"], "startups", MAX_STARTUPS)
    if permanent_full_fallback and startups:
        malformed("startups")
    comparisons = document["comparisons"]
    errors = document["errors"]
    excluded = document["excluded"]
    if type(comparisons) is not list or len(comparisons) > MAX_COMPARISONS:
        raise ReportError("too-many-rows" if type(comparisons) is list else "malformed-report: comparisons")
    if type(errors) is not list or len(errors) > MAX_ERRORS:
        raise ReportError("too-many-rows" if type(errors) is list else "malformed-report: errors")
    if type(excluded) is not list or len(excluded) > MAX_EXCLUDED:
        raise ReportError("too-many-rows" if type(excluded) is list else "malformed-report: excluded")
    sources = document["fixture_sources"]
    if type(sources) is not dict or len(sources) > 32:
        malformed("fixture_sources")
    for key, value in sources.items():
        if (type(key) is not str or type(value) is not str or not key or not value
                or len(key) > 256 or len(value) > 128 or "\n" in key or "\n" in value):
            malformed("fixture_sources")
    for index, row in enumerate(errors):
        validate_error(row, index)
    for index, row in enumerate(excluded):
        validate_excluded(row, index)
    process_warmup = 0
    for index, row in enumerate(processes):
        if require_bool(row.get("warmup"), "processes[{0}].warmup".format(index)):
            process_warmup += 1
    startup_warmup = 0
    for index, row in enumerate(startups):
        if require_bool(row.get("warmup"), "startups[{0}].warmup".format(index)):
            startup_warmup += 1
    summary = document["summary"]
    if type(summary) is not dict or set(summary) != SUMMARY_KEYS:
        malformed("summary")
    summary_samples = bounded_object_list(summary["samples"], "summary.samples", MAX_SAMPLES)
    if len(summary_samples) != len(samples):
        malformed("summary.samples")
    case_rows = summary["cases"]
    if type(case_rows) is not list or len(case_rows) > MAX_CASES:
        raise ReportError("too-many-rows" if type(case_rows) is list else "malformed-report: summary.cases")
    failures = summary["failures"]
    inconclusive = summary["inconclusive"]
    if type(failures) is not list or type(inconclusive) is not list:
        malformed("summary")
    if len(failures) > MAX_CASES or len(inconclusive) > MAX_CASES * 2:
        raise ReportError("too-many-rows")
    copied_cases = []
    groups = {}
    reason_counts = {}
    quiet_count = 0
    inconclusive_count = 0
    for index, row in enumerate(case_rows):
        copied, identity = validate_case(row, "summary.cases[{0}]".format(index))
        if permanent_full_fallback and (
                copied["mode"] == "native"
                or not copied["execution_identity"].endswith(":off")):
            malformed("summary.cases[{0}]".format(index))
        if identity in groups:
            malformed("summary.cases[{0}]".format(index))
        groups[identity] = copied
        copied_cases.append(copied)
        if copied["quiet"] is True:
            quiet_count += 1
        if copied["inconclusive"] is True:
            inconclusive_count += 1
        for reason in copied["reasons"]:
            reason_counts[reason] = reason_counts.get(reason, 0) + 1
    expected_inconclusive = []
    expected_failures = []
    for row in copied_cases:
        label = row["case"] + "/" + row["mode"]
        for reason in row["reasons"]:
            expected_inconclusive.append(label + ": " + reason)
        if row["correctness"] is False:
            expected_failures.append(label + ": correctness failure")
    if inconclusive != expected_inconclusive:
        malformed("summary.inconclusive")
    if failures != expected_failures:
        malformed("summary.failures")
    seen = set()
    eligible_count = 0
    error_free = len(errors) == 0
    for index, row in enumerate(comparisons):
        if validate_comparison(row, index, groups, error_free, seen):
            eligible_count += 1
    report = {
        "schema": REPORT_SCHEMA,
        "input_schema": SCHEMA,
        "action": ACTION,
        "passed": flags["passed"],
        "performance_accepted": flags["performance_accepted"],
        "goal_complete": flags["goal_complete"],
        "compatibility_accepted": flags["compatibility_accepted"],
        "permanent_full_fallback": permanent_full_fallback,
        "startup_measured": startup_measured,
        "scope": SCOPE,
        "limitation": LIMITATION,
        "counts": {
            "process_count": len(processes),
            "process_warmup_count": process_warmup,
            "process_measured_count": len(processes) - process_warmup,
            "sample_count": len(samples),
            "startup_count": len(startups),
            "startup_warmup_count": startup_warmup,
            "startup_measured_count": len(startups) - startup_warmup,
            "error_count": len(errors),
            "excluded_count": len(excluded),
        },
        "eligibility": {
            "case_count": len(copied_cases),
            "quiet_count": quiet_count,
            "busy_count": len(copied_cases) - quiet_count,
            "inconclusive_count": inconclusive_count,
            "failure_count": len(expected_failures),
            "comparison_count": len(comparisons),
            "eligible_count": eligible_count,
            "ineligible_count": len(comparisons) - eligible_count,
        },
        "reason_counts": reason_counts,
        "cases": copied_cases,
    }
    guard_report(report)
    return report


def escape_text(value):
    if type(value) is not str or any(ord(char) < 32 for char in value):
        raise ReportError("unsafe-text")
    return value.replace("\\", "\\\\").replace("|", "\\|").replace("`", "\\`")


def markdown_cell(value):
    if value is None:
        text = "null"
    elif type(value) is bool:
        text = "true" if value else "false"
    elif type(value) is int or type(value) is float:
        text = json.dumps(value, allow_nan=False)
    elif type(value) is list:
        text = ", ".join(value) if value else "[]"
    else:
        text = value
    return escape_text(text)


def render_json(report):
    return json.dumps(report, indent=2, ensure_ascii=True, allow_nan=False) + "\n"


def render_markdown(report):
    lines = ["# Runtime measurement report", ""]
    for key in ("passed", "performance_accepted", "goal_complete", "compatibility_accepted",
                "permanent_full_fallback", "startup_measured"):
        lines.append("`{0}`: {1}".format(key, "true" if report[key] else "false"))
    lines.extend(["", "Scope: " + escape_text(report["scope"]), "",
                  "Limitation: " + escape_text(report["limitation"]), "",
                  "## Counts", "", "| item | count |", "| --- | ---: |"])
    for key, value in report["counts"].items():
        lines.append("| {0} | {1} |".format(escape_text(key), markdown_cell(value)))
    lines.extend(["", "## Eligibility", "", "| item | count |", "| --- | ---: |"])
    for key, value in report["eligibility"].items():
        lines.append("| {0} | {1} |".format(escape_text(key), markdown_cell(value)))
    lines.extend(["", "## Reason counts", "", "| reason | count |", "| --- | ---: |"])
    if report["reason_counts"]:
        for key, value in report["reason_counts"].items():
            lines.append("| {0} | {1} |".format(escape_text(key), markdown_cell(value)))
    else:
        lines.append("| none | 0 |")
    lines.extend([
        "", "## Case summary", "",
        "Figures below are copied from the input summary. They are not recomputed. "
        "A zero error count is not performance acceptance.", "",
        "| " + " | ".join(CASE_FIELDS) + " |",
        "| " + " | ".join("---" for _ in CASE_FIELDS) + " |",
    ])
    for row in report["cases"]:
        lines.append("| " + " | ".join(markdown_cell(row[key]) for key in CASE_FIELDS) + " |")
    lines.append("")
    return "\n".join(lines)


def write_text(text, out):
    if out is None:
        sys.stdout.write(text)
        return
    destination = Path(out)
    try:
        destination.lstat()
    except FileNotFoundError:
        pass
    except OSError:
        raise ReportError("output-unavailable") from None
    else:
        raise ReportError("output-exists")
    parent = destination.parent
    try:
        parent_info = parent.lstat()
    except OSError:
        raise ReportError("output-unavailable") from None
    if stat.S_ISLNK(parent_info.st_mode) or not stat.S_ISDIR(parent_info.st_mode):
        raise ReportError("output-unavailable")
    try:
        with destination.open("x", encoding="utf-8") as stream:
            stream.write(text)
    except OSError:
        raise ReportError("output-unavailable") from None


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--format", choices=("markdown", "json"), default="markdown")
    parser.add_argument("--out", type=Path)
    options = parser.parse_args(argv)
    try:
        document = load_document(options.input)
        report = build_report(document)
        text = render_json(report) if options.format == "json" else render_markdown(report)
        if "\\" in text or "/Users/" in text:
            raise ReportError("unsafe-text")
        write_text(text, options.out)
    except ReportError as error:
        print("runtime-measurement-report: " + error.code, file=sys.stderr)
        return 1
    except RecursionError:
        print("runtime-measurement-report: invalid-json", file=sys.stderr)
        return 1
    except OverflowError:
        print("runtime-measurement-report: malformed-report: number", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
