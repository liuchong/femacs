"""Run the B component vectors against the actual-source native adapter in D.

Build the adapter separately under build protection, then run this driver under
runtime protection. Expected observations are deliberately not sent to it.
Retain all inputs, raw output, diagnostics and individual process wall times.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time
import unittest


REPO = Path(__file__).resolve().parent.parent
SOURCES = {"src/runtime_gc_observation.rs", "src/runtime_gc_collector.rs"}
CONFIG = None
CASES = None
RECEIPTS = []


def digest(path):
    with path.open("rb") as stream:
        result = hashlib.sha256()
        for chunk in iter(lambda: stream.read(65536), b""):
            result.update(chunk)
        return result.hexdigest()


def source_identities():
    return {name: digest(REPO / name) for name in sorted(SOURCES)}


def load_receipt(path):
    receipt = json.loads(path.read_text(encoding="utf-8"))
    if receipt.get("schema") != 1 or receipt.get("provider") != "actual-source-components":
        raise ValueError("actual-source component receipt required")
    if set(receipt.get("sources", {})) != SOURCES or receipt["sources"] != source_identities():
        raise ValueError("component compile inputs do not match current actual sources")
    support = receipt.get("supporting_source", {}).get("sources", {})
    if set(support) != {"src/alloc.rs", "src/include/lisp.rs"} or any(digest(REPO / name) != value for name,value in support.items()):
        raise ValueError("actual block/vector-mark support inputs do not match current source")
    relative = Path(receipt.get("binary", ""))
    if relative.is_absolute() or ".." in relative.parts:
        raise ValueError("component binary must have an owned relative locator")
    binary = path.parent / relative
    if binary.is_symlink() or not binary.is_file():
        raise ValueError("missing or symlinked component executable")
    if not binary.resolve().is_relative_to((REPO / "build").resolve()):
        raise ValueError("component executable must be in the owned build tree")
    if digest(binary) != receipt.get("binary_sha256"):
        raise ValueError("component executable changed after compilation")
    if not isinstance(receipt.get("stubs"), list):
        raise ValueError("component boundary/stubs must be explicitly declared")
    receipt["resolved_binary"] = binary
    return receipt


class ComponentContracts(unittest.TestCase):
    def run_case(self, case):
        root = Path(CONFIG.out)
        root.mkdir(parents=True, exist_ok=True)
        directory = Path(tempfile.mkdtemp(prefix=case["id"].replace("/", "-") + "-", dir=root))
        request = {key: value for key, value in case.items() if key != "expect"}
        if "inputs" in request:
            request["inputs"] = [{key:value for key,value in item.items()
                                  if key not in ("expected_kind", "reason")} for item in request["inputs"]]
        request["schema"] = 1
        (directory / "input.json").write_text(json.dumps(request, indent=2), encoding="utf-8")
        result = dict(case_id=case["id"], passed=False, component=case["component"],
                      sources=source_identities(), stubs=CASES["stubs"])
        binary = CASES["resolved_binary"]
        result["binary_sha256_before"] = digest(binary)
        started = time.perf_counter()
        try:
            self.assertEqual(result["binary_sha256_before"], CASES["binary_sha256"])
            with subprocess.Popen([str(binary)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                  stderr=subprocess.PIPE, text=True, start_new_session=True) as child:
                try:
                    stdout, stderr = child.communicate(json.dumps(request), timeout=CONFIG.timeout)
                except subprocess.TimeoutExpired:
                    # This group was created by this case; never touch other jobs.
                    try:
                        os.killpg(child.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    stdout, stderr = child.communicate()
                    (directory / "stdout.txt").write_text(stdout, encoding="utf-8")
                    (directory / "stderr.txt").write_text(stderr, encoding="utf-8")
                    result["status"] = "timeout"
                    self.fail("actual-source component deadline exceeded")
                (directory / "stdout.txt").write_text(stdout, encoding="utf-8")
                (directory / "stderr.txt").write_text(stderr, encoding="utf-8")
                result["exit_code"] = child.returncode
                self.assertEqual(child.returncode, 0, stderr)
            observed = json.loads(stdout)
            self.assertEqual(observed["schema"], 1)
            self.assertEqual(observed["case_id"], case["id"])
            self.assertEqual(observed["sources"], result["sources"])
            self.assertEqual(observed["provider"], "actual-source-components")
            self.assertEqual(observed["consumed_steps"], len(case.get("steps", [])))
            self.assertEqual(observed["consumed_inputs"], len(case.get("inputs", [])))
            values = observed["observations"]
            if "all_inputs_match" in case["expect"]:
                # Expected choices stay in this external assertion process.
                # The native adapter reports only actual production selections.
                selected = values.get("selected_cycles")
                self.assertIsInstance(selected, list)
                self.assertEqual(len(selected), len(case["inputs"]))
                values["all_inputs_match"] = all(
                    row.get("kind") == expected["expected_kind"] and row.get("reason") == expected["reason"]
                    for row,expected in zip(selected, case["inputs"]))
            for key, expected in case["expect"].items():
                self.assertIn(key, values, "adapter omitted an assertion")
                self.assertEqual(type(values[key]), type(expected), "observation type differs")
                if key.endswith("_at_most"):
                    self.assertGreaterEqual(values[key], 0, f"{case['id']}:{key}")
                    self.assertLessEqual(values[key], expected, f"{case['id']}:{key}")
                else:
                    self.assertEqual(values[key], expected, f"{case['id']}:{key}")
            result["observations"] = values
            result["passed"] = True
            result["status"] = "passed"
        except BaseException as error:
            result.setdefault("status", "failed")
            result["error"] = f"{type(error).__name__}: {error}"
            raise
        finally:
            result["seconds"] = time.perf_counter() - started
            try:
                result["binary_sha256_after"] = digest(binary)
            except OSError as error:
                result["binary_sha256_after"] = None
                result["product_error"] = str(error)
            if result["binary_sha256_after"] != result["binary_sha256_before"]:
                result["passed"] = False
                result["status"] = "product-changed"
            (directory / "results.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
            RECEIPTS.append(result)
        self.assertEqual(result["status"], "passed")


def install_cases(corpus):
    if corpus.get("schema") != 1 or not corpus.get("cases"):
        raise ValueError("component vectors required")
    seen = set()
    for case in corpus["cases"]:
        identity = case["id"]
        if identity in seen or case.get("component") not in ("observer", "collector"):
            raise ValueError("duplicate/unknown component case")
        seen.add(identity)
        if not isinstance(case.get("expect"), dict) or not case["expect"]:
            raise ValueError("component case requires explicit observations")
        def test(self, selected=case):
            self.run_case(selected)
        setattr(ComponentContracts, "test_" + identity.replace("/", "__").replace("-", "_"), test)
    return seen


def main():
    global CONFIG, CASES
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=120)
    CONFIG = parser.parse_args()
    if CONFIG.timeout <= 0:
        parser.error("positive component deadline required")
    if not CONFIG.out.resolve().is_relative_to((REPO / "build").resolve()) or CONFIG.out.is_symlink():
        parser.error("component output must be in the owned build tree")
    if CONFIG.out.exists():
        parser.error("fresh component output required; preserve earlier receipts")
    CASES = load_receipt(CONFIG.receipt)
    corpus = json.loads((REPO / "tools/fixtures/runtime-core-cases.json").read_text(encoding="utf-8"))
    identities = install_cases(corpus)
    started = time.perf_counter()
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(ComponentContracts)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    complete = len(RECEIPTS) == len(identities) and {r["case_id"] for r in RECEIPTS} == identities
    report = dict(passed=result.wasSuccessful() and complete and all(r["passed"] for r in RECEIPTS),
                  requested=len(identities), observed=len(RECEIPTS), complete=complete,
                  seconds=time.perf_counter() - started, cases=RECEIPTS,
                  limitation="actual-source isolated components, not full editor or backend/ABI acceptance")
    CONFIG.out.mkdir(parents=True, exist_ok=True)
    (CONFIG.out / "results.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    raise SystemExit(0 if report["passed"] else 1)


if __name__ == "__main__":
    main()
