"""Bounded closeout evidence, not a comprehensive VM or ERT acceptance gate."""
import ast
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
assert ROOT.name != "emacs"
OUT = ROOT / "docs/baseline/vm-closeout-20261021.json"
assert not OUT.resolve().is_relative_to((ROOT / "emacs").resolve())

def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1048576), b""):
            h.update(block)
    return h.hexdigest()

def main():
    report = {"scope": "four VM inputs per binary and four summary-classifier fixtures",
              "port_commit": subprocess.check_output(
                  ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
              "vm": [], "summary_counterexamples": []}
    tree = ast.parse((ROOT / "tools/check-ert.py").read_text())
    node = next(n for n in ast.walk(tree) if isinstance(n, ast.If)
                and ast.unparse(n.test) == "v1 != 'ok' or v2 != 'ok'")
    classifier = compile(ast.fix_missing_locations(
        ast.Module(body=[node], type_ignores=[])), "summary-classifier", "exec")
    fixtures = [
        ("equal-passes", 10, 0, 10, 0, "same", []),
        ("different-failure-identities", 2, 1, 2, 1, "both-fail",
         ["port fails A and passes B", "reference passes A and fails B"]),
        ("different-run-counts", 10, 0, 9, 0, "same", []),
        ("no-tests", 0, 0, 0, 0, "same", []),
    ]
    for name, r1, u1, r2, u2, expected, detail in fixtures:
        start = time.perf_counter_ns()
        env = dict(v1="ok", v2="ok", r1=r1, u1=u1, r2=r2, u2=u2, n1="", n2="")
        exec(classifier, {}, env)
        assert env["verdict"] == expected, (name, env)
        report["summary_counterexamples"].append(dict(
            case=name, actual=env["verdict"], port_ran=r1, ref_ran=r2,
            port_unexpected=u1, ref_unexpected=u2, detail=detail,
            elapsed_ns=time.perf_counter_ns()-start))
    source = (ROOT / "femacs/src/bytecode.rs").read_text()
    values = {int(v, 0) if v.startswith("0o") else int(v)
              for v in re.findall(r"DEFINE!\s*\(\s*\w+,\s*(0o[0-7]+|[0-9]+)\s*\)", source)}
    constant = re.search(r"DEFINE!\s*\(\s*Bconstant,\s*(0o[0-7]+|[0-9]+)", source).group(1)
    constant = int(constant, 0) if constant.startswith("0o") else int(constant)
    unused = next(i for i in range(1, constant) if i not in values)
    report["unused_opcode"] = unused
    cases = [
        ("compiled-add", "(funcall (byte-compile '(lambda (x) (+ x 1))) 41)", "42"),
        ("compiled-error", "(funcall (byte-compile '(lambda () (error \"closeout\"))))", "error"),
        ("invalid-zero", "(funcall (make-byte-code 0 (unibyte-string 0) [nil] 4))", "invalid-opcode"),
        ("invalid-unused", f"(funcall (make-byte-code 0 (unibyte-string {unused}) [nil] 4))", "invalid-opcode"),
    ]
    ok = True
    report["binary_identity"] = {}
    for side, rel in [("port", "build/port/src/emacs"),
                      ("reference", "build/emacs-ref/src/emacs")]:
        binary = ROOT / rel
        report["binary_identity"][side] = {"sha256": digest(binary),
                                          "dump_sha256": digest(binary.with_name("emacs.pdmp"))}
        for name, expr, expected in cases:
            code = ('(princ (format "CLOSEOUT:%S\\n" (condition-case err ' + expr
                    + ' (error (if (string-match-p "Invalid byte opcode" '
                    + '(error-message-string err)) (quote invalid-opcode) (car err))))))')
            start = time.perf_counter_ns()
            try:
                result = subprocess.run([str(binary), "-Q", "--batch", "--eval", code],
                                        cwd=ROOT, text=True, capture_output=True, timeout=10)
                markers = [line[9:] for line in result.stdout.splitlines()
                           if line.startswith("CLOSEOUT:")]
                actual = markers[-1] if markers else None
                passed = result.returncode == 0 and actual == expected
                row = dict(side=side, case=name, exit_code=result.returncode,
                           actual=actual, expected=expected, passed=passed)
            except subprocess.TimeoutExpired:
                passed = False
                row = dict(side=side, case=name, passed=False, timeout_seconds=10)
            row["elapsed_ns"] = time.perf_counter_ns()-start
            report["vm"].append(row)
            ok = ok and passed
    report["all_vm_inputs_passed"] = ok
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"vm_passed": sum(r["passed"] for r in report["vm"]),
                      "vm_total": len(report["vm"]), "unused_opcode": unused,
                      "classifier_fixtures": len(fixtures)}))
    return 0 if ok else 1

if __name__ == "__main__":
    sys.exit(main())
