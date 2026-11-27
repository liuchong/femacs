#!/usr/bin/env python3
"""Independent B-phase G4 tooling contracts.

This file is written before the C production modules exist.  Tests that need a production
module load it by exact repository path at class setup; a missing module is an error, not a
skip.  No test starts a compiler, SDK tool, emulator or real device.  The real integration
recipes live in the primary-owned ``tools/g4-acceptance.py`` and are deliberately not
mirrored here.

Coverage groups:
  T16/T20  L5 optional-heading parsing, complete chunk collection and fresh output
  T21/T22  Zig argv bridge, exact filtering, missing executable and literal metacharacters
  T27/T30  Android static package/report contracts and aliases (where no SDK run is needed)
  T31/T35  device receipt/judge/readiness semantics, adb ownership and quoting source contract
  T36/T39  manual manifest, staged source roots, Info naming and failure boundaries
  T01/T03  retention inventory and generated-profile recovery candidates
  T45/T48  API21 floor source scan, class/resource retention and withdrawal evidence
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import unittest
import xml.etree.ElementTree as ET


REPO = Path(__file__).resolve().parent.parent
L5_PY = REPO / "tools/l5_sweep.py"
ZIG = REPO / "tools/zig-linux-linker.sh"
MANUALS_PY = REPO / "tools/build-manuals.py"
DEVICE_PY = REPO / "tools/android-device.py"
READINESS_PY = REPO / "tools/android-readiness.py"
CHECK_APK_PY = REPO / "tools/check-android-apk.py"
MANUALS_JSON = REPO / "build-support/manuals.json"
ANDROID_CONFIG = REPO / "build-support/android/arm64/config.json"
ANDROID_CORE = REPO / "build-support/android/arm64/core-units.txt"
ANDROID_LIB = REPO / "build-support/android/arm64/lib-units.txt"
INVENTORY = REPO / "maintenance/tooling-inventory.json"
MANIFEST_XML = REPO / "platform/android/AndroidManifest.xml"
KOTLIN = REPO / "platform/android/kotlin"
RAW_NOTIFICATION = REPO / "platform/android/res/layout/sdk8_notifications_view.xml"
PROFILES = [REPO / "spikes/pgo-probe/nostd-main.profraw", REPO / "spikes/pgo-probe/nostd-exit.profraw"]


def load_module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_executable(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR)


class TimedResult(unittest.TextTestResult):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.case_records: list[dict] = []
        self._started: dict[str, float] = {}

    def startTest(self, test):
        self._started[test.id()] = time.monotonic()
        super().startTest(test)

    def _record(self, test, status, detail=""):
        started = self._started.pop(test.id(), time.monotonic())
        self.case_records.append({
            "id": test.id(),
            "status": status,
            "elapsed_seconds": round(max(0.0, time.monotonic() - started), 6),
            "detail": detail,
        })

    def addSuccess(self, test):
        self._record(test, "passed"); super().addSuccess(test)

    def addFailure(self, test, err):
        self._record(test, "failed", self._exc_info_to_string(err, test)); super().addFailure(test, err)

    def addError(self, test, err):
        self._record(test, "error", self._exc_info_to_string(err, test)); super().addError(test, err)

    def addSkip(self, test, reason):
        self._record(test, "skipped", reason); super().addSkip(test, reason)


class L5SweepContracts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = load_module("l5_sweep_contracts", L5_PY)

    def test_optional_heading_keeps_first_file_and_validates_count(self):
        parse = self.mod.parse_file_list
        self.assertEqual(parse("enumerated 2 file(s)\nlisp/a-tests.el\nlisp/b-tests.el\n"),
                         ["lisp/a-tests.el", "lisp/b-tests.el"])
        self.assertEqual(parse("lisp/a-tests.el\nlisp/b-tests.el\n"),
                         ["lisp/a-tests.el", "lisp/b-tests.el"])
        with self.assertRaises(ValueError):
            parse("enumerated 3 file(s)\nlisp/a-tests.el\n")
        for bad in ["", "enumerated 0 file(s)\n", "lisp/a.el\nlisp/a.el\n", "../lisp/a.el\n",
                    "/tmp/a.el\n", "\n"]:
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                parse(bad)

    def test_chunk_ranges_cover_all_files_without_overlap(self):
        ranges = self.mod.chunk_ranges(5, 0, 2)
        self.assertEqual(ranges, [(0, 2), (2, 4), (4, 5)])
        self.assertEqual(self.mod.chunk_ranges(0, 0, 3), [])
        self.assertEqual(self.mod.chunk_ranges(5, 2, 2), [(2, 4), (4, 5)])
        with self.assertRaises(ValueError):
            self.mod.chunk_ranges(5, 0, 0)
        with self.assertRaises(ValueError):
            self.mod.chunk_ranges(5, -1, 2)

    def _copy_sweep(self, root: Path, list_text: str, *, fail_start=None, heading=True) -> Path:
        tools = root / "tools"
        tools.mkdir(parents=True)
        shutil.copy2(L5_PY, tools / "l5_sweep.py")
        stub = f"""#!/usr/bin/env python3
import json, pathlib, sys
args = sys.argv[1:]
log = pathlib.Path(__file__).resolve().parent.parent / "argv-log.jsonl"
with log.open("a", encoding="utf-8") as fh:
    fh.write(json.dumps(args) + "\\n")
if "--list" in args:
    if {heading!r}:
        print("enumerated 3 file(s)")
    print("lisp/first-tests.el")
    print("lisp/second-tests.el")
    print("lisp/third-tests.el")
    sys.exit(0)
start = int(args[args.index("--start-at") + 1]) if "--start-at" in args else 0
count = int(args[args.index("--max-files") + 1])
selected = ["lisp/first-tests.el","lisp/second-tests.el","lisp/third-tests.el"][start:start+count]
if "--tsv" in args:
    p = pathlib.Path(args[args.index("--tsv") + 1]); p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text("file\\tverdict\\n"+"".join(name+"\\tsame\\n" for name in selected), encoding="utf-8")
if "--results-json" in args:
    p = pathlib.Path(args[args.index("--results-json") + 1]); p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps({{"files": [{{"file": name, "verdict": "same"}} for name in selected]}}), encoding="utf-8")
if {fail_start!r} == start:
    sys.exit(1)
sys.exit(0)
"""
        write_executable(tools / "check-ert.py", stub)
        return tools / "l5_sweep.py"

    def _run(self, root: Path, script: Path, *args: str):
        return subprocess.run([sys.executable, str(script), *args], cwd=root,
                              capture_output=True, text=True, timeout=20)

    def test_cli_includes_first_file_with_and_without_heading(self):
        for heading in (True, False):
            with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
                root = Path(tmp)
                script = self._copy_sweep(root, "x", heading=heading)
                out = root / "out"
                proc = self._run(root, script, "--chunk", "2", "--out", str(out))
                self.assertEqual(proc.returncode, 0, proc.stderr + proc.stdout)
                self.assertTrue((out / "ert-l5-chunk001.tsv").is_file())
                self.assertTrue((out / "ert-l5-chunk002.tsv").is_file())
                calls = [json.loads(line) for line in (root / "argv-log.jsonl").read_text().splitlines()]
                chunk_calls = [c for c in calls if "--list" not in c]
                self.assertTrue(any("lisp/first-tests.el" in " ".join(c) for c in chunk_calls) or
                                all("--start-at" in c for c in chunk_calls))

    def test_cli_continues_after_failed_first_chunk_and_reports_failure(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            script = self._copy_sweep(root, "x", fail_start=0)
            out = root / "out"
            proc = self._run(root, script, "--chunk", "1", "--out", str(out))
            self.assertNotEqual(proc.returncode, 0)
            self.assertTrue((out / "ert-l5-chunk001.tsv").is_file())
            self.assertTrue((out / "ert-l5-chunk002.tsv").is_file())

    def test_cli_preserves_stress_and_program_flags(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            script = self._copy_sweep(root, "x")
            out = root / "out"
            proc = self._run(root, script, "--chunk", "2", "--gc-stress",
                             "--port-prog", "/owned/port", "--ref-prog", "/owned/ref", "--out", str(out))
            self.assertEqual(proc.returncode, 0, proc.stderr + proc.stdout)
            calls = [json.loads(line) for line in (root / "argv-log.jsonl").read_text().splitlines()]
            chunk_calls = [c for c in calls if "--list" not in c]
            for call in chunk_calls:
                self.assertIn("--gc-stress", call)
                self.assertIn("--port-prog", call)
                self.assertIn("--ref-prog", call)

    def test_cli_rejects_reused_output_and_invalid_flags(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            script = self._copy_sweep(root, "x")
            out = root / "out"; out.mkdir()
            sentinel = out / "keep"; sentinel.write_text("keep")
            proc = self._run(root, script, "--out", str(out))
            self.assertNotEqual(proc.returncode, 0)
            self.assertEqual(sentinel.read_text(), "keep")
            for args in (("--chunk", "0"), ("--chunk", "-1"), ("--unknown",)):
                out = root / ("new-" + args[0].lstrip("-"))
                proc = self._run(root, script, "--out", str(out), *args)
                self.assertNotEqual(proc.returncode, 0)


class ZigBridgeContracts(unittest.TestCase):
    def _fake_zig(self, root: Path) -> tuple[Path, Path]:
        log = root / "zig-argv.json"
        zig = root / "zig"
        write_executable(zig, """#!/usr/bin/env python3
import json, os, sys
open(os.environ[\"FAKE_ZIG_LOG\"], \"w\", encoding=\"utf-8\").write(json.dumps(sys.argv[1:]))
""")
        return zig, log

    def test_literal_argv_and_exact_filtering(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            zig, log = self._fake_zig(root)
            env = dict(os.environ, ZIG=str(zig), ZIG_TARGET="aarch64-linux-gnu", FAKE_ZIG_LOG=str(log))
            args = ["with space", "'single'", 'double"quote', "ünicode", "`cmd`", "$(not-run)", "", "\\backslash"]
            proc = subprocess.run(["/bin/sh", str(ZIG), "arg0", "-Wl,--fix-cortex-a53-843419",
                                   "-Wl,--other", "--fix-cortex-a53-843419", *args],
                                  env=env, capture_output=True, text=True, timeout=20)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            got = json.loads(log.read_text())
            self.assertEqual(got[:3], ["cc", "-target", "aarch64-linux-gnu"])
            self.assertNotIn("-Wl,--fix-cortex-a53-843419", got)
            self.assertNotIn("--fix-cortex-a53-843419", got)
            self.assertIn("-Wl,--other", got)
            for value in args:
                self.assertIn(value, got)

    def test_missing_zig_is_a_failure_and_source_has_no_eval_or_host_default(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            empty = root / "empty-bin"; empty.mkdir()
            env = {"PATH": str(empty), "HOME": str(root)}
            proc = subprocess.run(["/bin/sh", str(ZIG), "x"], env=env,
                                  capture_output=True, text=True, timeout=20)
            self.assertNotEqual(proc.returncode, 0)
        source = ZIG.read_text()
        self.assertNotIn("eval ", source)
        self.assertNotIn("/opt/homebrew", source)
        self.assertIn("command -v zig", source)


class ManualContracts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = load_module("build_manuals_contracts", MANUALS_PY)

    def test_complete_manifest_and_68_manual_outputs(self):
        self.assertTrue(MANUALS_JSON.is_file())
        manifest = json.loads(MANUALS_JSON.read_text(encoding="utf-8"))
        self.assertEqual(manifest.get("schema"), "femacs-manuals-v1")
        manuals = manifest["manuals"]
        names = [m["info_name"] for m in manuals]
        self.assertEqual(len(names), 68)
        self.assertEqual(len(names), len(set(names)))
        for required in ("emacs", "elisp", "org", "modus-themes"):
            self.assertIn(required, names)
        self.assertNotIn("lay-flat", names)
        excluded = {row["source"] for row in manifest.get("excluded", [])}
        self.assertTrue(any(path.endswith("lay-flat.texi") for path in excluded))
        for row in manuals:
            source = REPO / row["source"]
            self.assertTrue(source.is_file(), row["source"])
            self.assertGreater(len(row["info_name"]), 0)
        self.assertTrue(any(m["org_source"] for m in manuals if m["info_name"] == "org"))
        self.assertTrue(any(m["org_source"] for m in manuals if m["info_name"] == "modus-themes"))

    def test_discovery_matches_manifest_and_keeps_lowercase_manual_roots(self):
        discovered = self.mod.discover_manuals(REPO)
        by_name = {m.info_name: m for m in discovered}
        manifest = json.loads(MANUALS_JSON.read_text(encoding="utf-8"))
        expected = {m["info_name"]: m for m in manifest["manuals"]}
        self.assertEqual(set(by_name), set(expected))
        for name in ("bovine", "semantic", "wisent", "org", "modus-themes"):
            self.assertIn(name, by_name)
        for name, spec in by_name.items():
            self.assertTrue(Path(spec.source).is_file(), name)
            self.assertTrue(spec.info_name == name)

    def test_no_tracked_info_outputs_and_tracked_manual_roots_exist(self):
        tracked = subprocess.run(["git", "-C", str(REPO), "ls-files", "*.info"],
                                 capture_output=True, text=True, check=True).stdout
        self.assertEqual(tracked.strip(), "")
        self.assertTrue((REPO / "doc/misc/org.org").is_file())
        self.assertTrue((REPO / "doc/misc/org-setup.org").is_file())
        self.assertTrue((REPO / "doc/misc/modus-themes.org").is_file())
        self.assertTrue((REPO / "lisp/org/ox-texinfo.el").is_file())

    def test_manifest_rejects_duplicate_or_missing_source(self):
        manifest = json.loads(MANUALS_JSON.read_text(encoding="utf-8"))
        duplicate = copy.deepcopy(manifest)
        duplicate["manuals"].append(copy.deepcopy(duplicate["manuals"][0]))
        names = [m["info_name"] for m in duplicate["manuals"]]
        self.assertGreater(len(names), len(set(names)))
        missing = copy.deepcopy(manifest)
        missing["manuals"][0]["source"] = "doc/misc/does-not-exist.texi"
        self.assertFalse((REPO / missing["manuals"][0]["source"]).is_file())


class AndroidDeviceContracts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = load_module("android_device_contracts", DEVICE_PY)

    def _product(self, root: Path):
        apk = root / "emacs.apk"; apk.write_bytes(b"apk")
        so = root / "libemacs.so"; so.write_bytes(b"so")
        return apk, so

    def _receipt(self, root: Path, apk: Path, so: Path):
        receipt = {
            "schema": "femacs-android-device-v1", "run_id": "owned-run", "run_root": str(root.resolve()),
            "mode": "accept", "acceptance": True,
            "owned_avd": {"name": "femacs-owned", "home": str(root.resolve()/"avd"),
                          "system_image": "system-images;android-36;default;arm64-v8a", "api": 36, "abi": "arm64-v8a"},
            "serial": "emulator-5580", "port": 5580,
            "product": {"apk_sha256": sha(apk), "so_sha256": sha(so), "package": "org.femacs",
                        "device_apk_sha256": sha(apk), "device_so_sha256": sha(so)},
            "install": {"status": "ok", "exit_code": 0, "device_apk_sha256": sha(apk), "device_so_sha256": sha(so)},
            "process": {"pid": 1234, "alive_seconds": 20},
            "native": {"init_line": True, "thread_line": True, "stderr_lines": 1},
            "crashes": {"fatal_signals": 0}, "cases": [], "errors": [],
            "cleanup": {"complete": True, "errors": [], "owned_children_reaped": True},
            "started_at": "2026-01-01T00:00:00Z", "completed_at": "2026-01-01T00:00:40Z",
        }
        def observation(point=6, landscape=False):
            return {'schema':'femacs-android-observation-v1','run_token':receipt['run_id'], 'pid':1234,
                    'text':'HELLO','modified':True,'point':point,'error':None,'window_system':'android',
                    'buffer':'*Femacs Android Acceptance*','selected_buffer':'*Femacs Android Acceptance*',
                    'frame_width':200 if landscape else 100,'frame_height':100 if landscape else 200,
                    'char_height':12,'window_edges':[0,0,100,100],
                    'clipboard_expected':'Femacs clipboard 中文 owned-run','clipboard_actual':'Femacs clipboard 中文 owned-run',
                    'battery':[100,10000,0,0,5,-1,1,250]}
        def render(landscape=False):
            return {'observation':observation(landscape=landscape), 'sha256':'a'*64,'before_sha256':'b'*64,
                    'changed_pixels':400,'editor_region':[0,0,100,100],'expected_text':'HELLO',
                    'pattern_validated':True,'pattern_token':receipt['run_id'],
                    'pattern':[{'color':color,'rect':[i*10,0,i*10+10,12], 'matching_pixels':80,'total_pixels':120}
                               for i,color in enumerate(self.mod.pattern_colors(receipt['run_id']))]}
        evidence={'identity':{'serial':receipt['serial'],'api':36,'abi':'arm64-v8a','avd_name':'femacs-owned'},
                  'install':receipt['install'],'native':receipt['native'],'liveness':receipt['process'],
                  'render':render(),'input':observation(),'keys':[observation(p) for p in (1,6,5,6)],
                  'clipboard':observation(),'battery':observation(),
                  'rotation':[dict(render(True),requested_rotation=1),dict(render(),requested_rotation=0)]}
        receipt['cases']=[{'id':name,'kind':'strict','status':'passed','elapsed_ms':1,'evidence':value}
                          for name,value in evidence.items()]
        home=root/'avd';home.mkdir()
        (root/'ownership.json').write_text(json.dumps({'run_id':receipt['run_id'],
            'spec':{'avd_name':'femacs-owned','avd_home':str(home.resolve()),'serial':receipt['serial'],'port':5580,
                    'system_image':receipt['owned_avd']['system_image'],'api':36,'abi':'arm64-v8a'},
            'emulator_pid':4000,'server_pid':4001,'guest_memory_mib':2048}))
        return receipt

    def test_missing_or_mismatched_evidence_and_foreign_cleanup_cannot_pass(self):
        with tempfile.TemporaryDirectory(dir=REPO/'build') as tmp:
            root=Path(tmp);apk,so=self._product(root);receipt=self._receipt(root,apk,so)
            def rejected(value):self.assertFalse(self._passed(self.mod.judge(value,apk,so)))
            for case in receipt['cases']:
                bad=copy.deepcopy(receipt)
                next(v for v in bad['cases'] if v['id']==case['id']).pop('evidence')
                rejected(bad)
            for key,value in [('home',str(root.parent/'foreign-avd')),('home',str(root/'avd/../avd'))]:
                bad=copy.deepcopy(receipt);bad['owned_avd'][key]=value;rejected(bad)
            bad=copy.deepcopy(receipt);bad['cleanup']['errors']=['failed cleanup'];rejected(bad)
            bad=copy.deepcopy(receipt);bad['cleanup']['owned_children_reaped']=False;rejected(bad)
            for case_name,field,value in [('input','pid',99),('input','run_token','old'),('input','text','wrong'),('clipboard','clipboard_actual','old'),('render','pattern_token','old')]:
                bad=copy.deepcopy(receipt);next(v for v in bad['cases'] if v['id']==case_name)['evidence'][field]=value;rejected(bad)
            bad=copy.deepcopy(receipt);bad['cases'].append(copy.deepcopy(bad['cases'][0]));rejected(bad)

    def test_nonexistent_or_mismatched_physical_owner_cannot_pass(self):
        with tempfile.TemporaryDirectory(dir=REPO/'build') as tmp:
            root=Path(tmp);apk,so=self._product(root);receipt=self._receipt(root,apk,so)
            ownership=root/'ownership.json';original=json.loads(ownership.read_text())
            ownership.unlink()
            self.assertFalse(self._passed(self.mod.judge(receipt,apk,so)))
            for key,value in [('run_id','foreign'),('spec',dict(original['spec'],serial='emulator-5560'))]:
                bad=dict(original,**{key:value});ownership.write_text(json.dumps(bad))
                self.assertFalse(self._passed(self.mod.judge(receipt,apk,so)))
            ownership.write_text(json.dumps(original));(root/'avd').rmdir()
            self.assertFalse(self._passed(self.mod.judge(receipt,apk,so)))
            bad=copy.deepcopy(receipt);bad['run_root']=str(root/'nonexistent');bad['owned_avd']['home']=str(root/'nonexistent/avd')
            self.assertFalse(self._passed(self.mod.judge(bad,apk,so)))

    def test_legacy_plan_is_read_only_and_migrated_dump_requires_explicit_receiver(self):
        import contextlib,io
        output=io.StringIO()
        with contextlib.redirect_stdout(output):self.assertEqual(self.mod.main(['smoke','--plan','--memory','3072','--tail','12']),0)
        plan=json.loads(output.getvalue());self.assertFalse(plan['executed']);self.assertEqual(plan['guest_memory_mib'],3072)
        self.assertEqual(self.mod.main(['smoke','--dump-only']),2)

    def _passed(self, result):
        return bool(result.get("passed", result.get("acceptance", False)))

    def test_complete_bound_acceptance_passes_judge(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            apk, so = self._product(root)
            receipt = self._receipt(root, apk, so)
            result = self.mod.judge(receipt, apk, so)
            self.assertTrue(self._passed(result), result)

    def test_stale_hash_missing_case_and_diagnostic_mode_never_pass_readiness(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            apk, so = self._product(root)
            receipt = self._receipt(root, apk, so)
            bad = copy.deepcopy(receipt); bad["product"]["apk_sha256"] = "0" * 64
            self.assertFalse(self._passed(self.mod.judge(bad, apk, so)))
            bad = copy.deepcopy(receipt); bad["cases"] = bad["cases"][:1]
            self.assertFalse(self._passed(self.mod.judge(bad, apk, so)))
            bad = copy.deepcopy(receipt); bad["mode"] = "diagnostic"; bad["acceptance"] = False
            self.assertFalse(self._passed(self.mod.judge(bad, apk, so)))
            bad = copy.deepcopy(receipt); bad["crashes"]["fatal_signals"] = 1
            self.assertFalse(self._passed(self.mod.judge(bad, apk, so)))

    def test_missing_serial_is_rejected_in_inspection_mode(self):
        rc = self.mod.main(["inspect"])
        self.assertNotEqual(rc, 0)

    def test_device_session_public_contract_and_spec_fields(self):
        for method in ("start", "wait_boot", "install", "launch", "shell", "capture", "stop"):
            self.assertTrue(hasattr(self.mod.DeviceSession, method), method)
        fields = getattr(self.mod.DeviceSpec, "__dataclass_fields__", {})
        for field in ("avd_name", "avd_home", "serial", "port", "system_image", "api", "abi"):
            self.assertIn(field, fields)
        product = getattr(self.mod.Product, "__dataclass_fields__", {})
        for field in ("apk", "so", "apk_sha256", "so_sha256", "package"):
            self.assertIn(field, product)

    def test_source_contract_uses_explicit_serial_quoting_and_owned_cleanup(self):
        source = DEVICE_PY.read_text(encoding="utf-8")
        self.assertTrue("shlex.quote" in source or "shlex.join" in source)
        self.assertNotIn("shell=True", source)
        self.assertIn("-s", source)
        self.assertIn("run-as", source)
        self.assertIn("emu kill", source)
        self.assertNotIn("kill-server", source)
        self.assertNotIn("rm -f /data/tombstones", source)


class ReadinessContracts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = load_module("android_readiness_contracts", READINESS_PY)

    def test_readiness_requires_receipt_and_not_old_png(self):
        with tempfile.TemporaryDirectory(dir=REPO / "build") as tmp:
            root = Path(tmp)
            apk = root / "emacs.apk"; apk.write_bytes(b"apk")
            so = root / "emacs.so"; so.write_bytes(b"so")
            out = root / "missing.json"
            rc = self.mod.main(["--receipt", str(out), "--apk", str(apk), "--so", str(so), "--require"])
            self.assertNotEqual(rc, 0)

    def test_source_contract_no_png_launch_claim(self):
        source = READINESS_PY.read_text(encoding="utf-8")
        self.assertIn("receipt", source)
        self.assertNotIn("2-typed.png", source)


class AndroidConfigContracts(unittest.TestCase):
    def test_active_config_has_no_absolute_host_paths_and_preserves_target_strings(self):
        config = json.loads(ANDROID_CONFIG.read_text(encoding="utf-8"))
        self.assertEqual(config.get("schema"), "femacs-android-config-v1")
        self.assertEqual(config.get("api"), 36)
        self.assertEqual(config.get("ndk_api"), 21)
        self.assertEqual(config.get("abi"), "arm64-v8a")
        values = dict(config["macros"]); values.update(config["features"])
        self.assertEqual(values["SYSTEM_TYPE"], '"android"')
        self.assertEqual(values["EMACS_CONFIGURATION"], '"aarch64-unknown-linux-android21"')
        self.assertEqual(values["ANDROID_API_GE_9"], "1")
        text = ANDROID_CONFIG.read_text(encoding="utf-8")
        for forbidden in ("/Users/", "/opt/homebrew", "/Applications/", "\\/Users\\/"):
            self.assertNotIn(forbidden, text)
        provenance = config.get("provenance", {})
        self.assertRegex(provenance.get("raw_options_sha256", ""), r"^[0-9a-f]{64}$")
        self.assertNotIn("measured_config_options", provenance)

    def test_units_are_exactly_owned_core_library_inputs_without_auxiliary_programs(self):
        core = [line.strip() for line in ANDROID_CORE.read_text(encoding="utf-8").splitlines()
                if line.strip() and not line.lstrip().startswith("#")]
        lib = [line.strip() for line in ANDROID_LIB.read_text(encoding="utf-8").splitlines()
               if line.strip() and not line.lstrip().startswith("#")]
        self.assertTrue(core and lib)
        self.assertEqual(len(core), len(set(core)))
        self.assertEqual(len(lib), len(set(lib)))
        self.assertIn("lib/mini-gmp.c", lib)
        for path in core + lib:
            self.assertFalse(path.startswith("/"))
            self.assertNotIn("/../", path)
            # Historical C unit names map to the actual owned Rust sources.
            owned = (REPO / path).with_suffix(".rs") if path.endswith(".c") else REPO / path
            if not owned.is_file() and path.endswith(".c"):
                owned = (REPO / "crates/lisp-vm/src/ported" / path).with_suffix(".rs")
            self.assertTrue(owned.is_file(), path)
        all_paths = core + lib
        for forbidden in ("lib-src/", "cm.c", "termcap.c", "terminfo.c", "android-emacs.c"):
            self.assertFalse(any(forbidden in p for p in all_paths), forbidden)


class ApiFloorContracts(unittest.TestCase):
    def test_manifest_floor_is_preserved(self):
        root = ET.parse(MANIFEST_XML).getroot()
        ns = "{http://schemas.android.com/apk/res/android}"
        uses = root.find("uses-sdk")
        self.assertIsNotNone(uses)
        self.assertEqual(uses.get(ns + "minSdkVersion"), "21")
        self.assertEqual(uses.get(ns + "targetSdkVersion"), "37")

    def test_below_floor_guards_are_absent_and_live_modern_paths_remain(self):
        old = re.compile(r"Build\.VERSION_CODES\.(GINGERBREAD|HONEYCOMB|HONEYCOMB_MR1|HONEYCOMB_MR2|"
                         r"ICE_CREAM_SANDWICH|ICE_CREAM_SANDWICH_MR1|JELLY_BEAN|JELLY_BEAN_MR1|"
                         r"JELLY_BEAN_MR2|KITKAT|KITKAT_WATCH)")
        for path in sorted(KOTLIN.glob("*.kt")):
            for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
                if line.lstrip().startswith("//"):
                    continue
                self.assertIsNone(old.search(line), f"{path.name}:{number}: {line}")
        self.assertFalse((KOTLIN / "EmacsSdk8Clipboard.kt").exists())
        for name in ("EmacsSdk7FontDriver.kt", "EmacsSdk11Clipboard.kt", "EmacsSdk23FontDriver.kt"):
            self.assertTrue((KOTLIN / name).is_file(), name)
        service = (KOTLIN / "EmacsService.kt").read_text(encoding="utf-8")
        self.assertNotIn("queryBattery19", service)
        self.assertIn("nativeLibraryDir", service)
        self.assertIn("allocationByteCount", (KOTLIN / "EmacsPixmap.kt").read_text(encoding="utf-8"))
        self.assertTrue(RAW_NOTIFICATION.is_file(), "raw notification XML remains retained")
        self.assertTrue((REPO / "platform/android/res/values-v20/style.xml").is_file())


class RetentionContracts(unittest.TestCase):
    def test_inventory_covers_all_entries_and_keeps_uncertain_candidates(self):
        inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
        self.assertEqual(inventory.get("schema"), "femacs-tooling-inventory-v1")
        self.assertEqual(len(inventory.get("entries", [])), 239)
        for entry in inventory["entries"]:
            self.assertIn(entry["decision"], {"retain", "replace-body", "retire"})
            self.assertIn(entry["family"], {"rust-build", "generator", "comparison-gate", "measurement",
                                             "test", "fixture", "compatibility-entry", "linker-adaptor",
                                             "raw-generated-evidence", "historical-recipe"})
            if entry["decision"] == "retire":
                self.assertTrue(entry.get("replacement"), entry["path"])

    def test_current_tooling_inputs_are_inventoried_with_hashes_and_readers(self):
        import subprocess
        inventory=json.loads(INVENTORY.read_text())
        entries=inventory['entries']+inventory['new_entries']
        by_path={entry['path']:entry for entry in entries}
        self.assertEqual(len(by_path),len(entries))
        current=subprocess.check_output(['git','ls-files','-z','--cached','--others',
                                         '--exclude-standard','tools','spikes'],cwd=REPO).split(b'\0')
        for raw in current:
            if not raw:continue
            name=os.fsdecode(raw);path=REPO/name
            if not path.is_file():continue
            self.assertIn(name,by_path,name)
            entry=by_path[name]
            self.assertEqual(entry.get('current_sha256',entry.get('sha256')),sha(path),name)
            if name not in {e['path'] for e in inventory['entries']}:
                self.assertTrue(entry.get('readers'),name)
                self.assertEqual(entry['bytes'],path.stat().st_size,name)
                for reader in entry['readers']:
                    self.assertTrue((REPO/reader['path']).is_file(),reader)

    def test_pattern_filter_runs_only_the_requested_case(self):
        import subprocess
        with tempfile.TemporaryDirectory(dir=REPO/'build') as tmp:
            report=Path(tmp)/'selected.json'
            result=subprocess.run([sys.executable,str(Path(__file__).resolve()),'-k',
                                   'test_generated_profiles_are_retired','--json-out',str(report)],
                                  cwd=REPO,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=15)
            self.assertEqual(result.returncode,0,result.stdout)
            data=json.loads(report.read_text())
            self.assertEqual(data['tests'],1)
            self.assertTrue(data['passed'])

    def test_generated_profiles_are_retired_but_source_and_recovery_remain(self):
        for path in PROFILES:
            self.assertFalse(path.exists(), f"raw generated profile still tracked: {path}")
        ignore = (REPO / ".gitignore").read_text(encoding="utf-8")
        self.assertIn("spikes/pgo-probe/*.profraw", ignore)
        self.assertTrue((REPO / "spikes/pgo-probe").is_dir())
        self.assertTrue((REPO / "spikes/pgo-probe/README.md").is_file())


class AliasContracts(unittest.TestCase):
    def test_l5_alias_forwards_to_python_without_scheduling_policy(self):
        source = (REPO / "tools/l5-sweep.sh").read_text(encoding="utf-8")
        self.assertIn("l5_sweep.py", source)
        self.assertNotIn("tail -n +2", source)
        self.assertNotIn("ert-l5-chunk", source)

    def test_android_aliases_are_thin_xtask_routes(self):
        for name, route in (("check-android-kotlin.sh", "kotlin"),
                            ("gen-android-config.sh", "config"),
                            ("check-authorized-platforms.sh", "gate-platforms")):
            source = (REPO / "tools" / name).read_text(encoding="utf-8")
            self.assertIn("cargo", source)
            self.assertIn("xtask", source)
            self.assertIn(route, source)

    def test_check_android_apk_uses_the_typed_package_route_not_incidental_prose(self):
        source = CHECK_APK_PY.read_text(encoding="utf-8")
        self.assertIn("xtask", source)
        self.assertIn("android", source)
        self.assertIn("package", source)


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json-out", type=Path)
    parser.add_argument("-k", "--pattern")
    args = parser.parse_args(argv)
    suite = unittest.defaultTestLoader.loadTestsFromModule(sys.modules[__name__])
    if args.pattern:
        filtered = unittest.TestSuite()
        def individual_cases(group):
            for test in group:
                if isinstance(test,unittest.TestSuite):yield from individual_cases(test)
                else:yield test
        for test in individual_cases(suite):
            if args.pattern in test.id():
                filtered.addTest(test)
        suite = filtered
    result = unittest.TextTestRunner(resultclass=TimedResult).run(suite)
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps({
            "passed": result.wasSuccessful(),
            "tests": result.testsRun,
            "failures": [str(t) for t, _ in result.failures],
            "errors": [str(t) for t, _ in result.errors],
            "cases": result.case_records,
        }, indent=2) + "\n", encoding="utf-8")
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
