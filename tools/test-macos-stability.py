#!/usr/bin/env python3
"""Component contracts for current-artifact acceptance; no editor is launched."""
import importlib.util, json, sys, tempfile, time, unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('local_acceptance',ROOT/'tools/local-acceptance.py')
acceptance=importlib.util.module_from_spec(spec);sys.modules[spec.name]=acceptance;spec.loader.exec_module(acceptance)
spec2=importlib.util.spec_from_file_location('ime_acceptance',ROOT/'tools/test-macos-ime.py')
ime=importlib.util.module_from_spec(spec2);sys.modules[spec2.name]=ime;spec2.loader.exec_module(ime)
spec3=importlib.util.spec_from_file_location('drag_acceptance',ROOT/'tools/test-macos-drag.py')
drag=importlib.util.module_from_spec(spec3);sys.modules[spec3.name]=drag;spec3.loader.exec_module(drag)
class Cases(unittest.TestCase):
 def setUp(self):
  base=ROOT/'build/stability-unit';base.mkdir(parents=True,exist_ok=True)
  self.temp=tempfile.TemporaryDirectory(dir=base);self.out=Path(self.temp.name)
 def tearDown(self):self.temp.cleanup()
 @classmethod
 def tearDownClass(cls):
  base=ROOT/'build/stability-unit'
  try:base.rmdir()
  except OSError:pass
 def test_failed_command_retains_log_and_nonzero(self):
  r=acceptance.run_check('failed',[sys.executable,'-c','print("diagnostic");raise SystemExit(7)'],self.out)
  self.assertEqual(r['exit_code'],7);self.assertFalse(r['passed']);self.assertGreaterEqual(r['seconds'],0)
  self.assertIn('diagnostic',Path(r['log']).read_text())
 def test_successful_command_has_distinct_evidence(self):
  a=acceptance.run_check('first',[sys.executable,'-c','print("one")'],self.out)
  b=acceptance.run_check('second',[sys.executable,'-c','print("two")'],self.out)
  self.assertTrue(a['passed']);self.assertNotEqual(a['log'],b['log'])
 def test_fingerprints_bind_actual_file(self):
  p=self.out/'program';p.write_bytes(b'first');a=acceptance.fingerprints({'program':p})
  p.write_bytes(b'second');b=acceptance.fingerprints({'program':p});self.assertNotEqual(a,b)
 def test_missing_product_is_error_not_skip(self):
  with self.assertRaises((FileNotFoundError,ValueError)):acceptance.fingerprints({'program':self.out/'missing'})
 def test_commit_requires_exact_text(self):
  self.assertFalse(ime.commit_matches({'marked':False,'text':'别的汉字'},'你好'))
  self.assertFalse(ime.commit_matches({'marked':True,'text':'你好'},'你好'))
  self.assertTrue(ime.commit_matches({'marked':False,'text':'你好'},'你好'))
 def test_output_reuse_rejected_without_wiping(self):
  p=self.out/'existing';p.mkdir();(p/'evidence').write_text('retain')
  with self.assertRaises((FileExistsError,ValueError)):ime.new_output(p)
  self.assertEqual((p/'evidence').read_text(),'retain')
 def test_retired_historical_gate_has_current_replacement(self):
  self.assertFalse((ROOT/'tools/verify-dod.sh').exists())
  self.assertTrue((ROOT/'tools/local-acceptance.py').is_file())
  self.assertIn('verify-dod.sh',(ROOT/'docs/retired-scaffolding.md').read_text())
 def test_clipboard_boundary_forces_utf8_without_global_locale_change(self):
  from unittest import mock
  import os
  with mock.patch.dict(os.environ,{'LC_ALL':'C','LANG':'C'}):
   with mock.patch.object(ime.delivery.subprocess,'run') as run:
    run.return_value.stdout='中文'.encode()
    self.assertEqual(ime.delivery.clipboard_read(),'中文'.encode())
    ime.delivery.clipboard_write('中文'.encode())
    for call in run.call_args_list:
     self.assertEqual(call.kwargs['env']['LC_ALL'],'en_US.UTF-8')
    self.assertEqual(os.environ['LC_ALL'],'C')
 def test_load_pdump_preserves_discovered_executable_on_path_exec_lookup(self):
  source=(ROOT/'src/emacs.rs').read_text(encoding='utf-8')
  self.assertNotIn('if ! (emacs_executable.is_null () && *emacs_executable != 0)',source)
  self.assertGreaterEqual(source.count('if ! (! emacs_executable.is_null () && *emacs_executable != 0)'),2)
 def test_titlebar_click_point_and_input_context_sample(self):
  records=[{'windowTitle':'Femacs IME B','windowFrame':[700.0,651.0,457.0,406.0],
            'event':'windowDidBecomeKey','clientHasInputContext':True,
            'clientIsCurrentInputContext':True,'selectedKeyboardInputSource':'com.apple.keylayout.ABC'}]
  pt=ime.titlebar_click_point(records,'Femacs IME B',1117.0)
  self.assertEqual(pt,(928.5,74.0))
  self.assertIsNone(ime.titlebar_click_point(records,'Missing Frame',1117.0))
  sample=ime.client_source_sample(records,'Femacs IME B',('windowDidBecomeKey',))
  self.assertTrue(sample['clientIsCurrentInputContext'])
 def test_external_drag_validators_and_reference_guard(self):
  from unittest import mock
  ref=self.out/'emacs';ref.mkdir()
  with mock.patch.object(drag,'ROOT',self.out):
   with self.assertRaises(ValueError):drag.new_output(ref/'sub')
  ok,reason=drag.validate_drag_receipt(
   {'pid':42,'native':'ns','frame':'Femacs Drag Target','opened':'hello\n'},
   [{'frame-live':True,'name':'Femacs Drag Target','x':120,'y':180}],
   [{'event':'draggingUpdated'},{'event':'performDragOperation'}],
   'hello\n',42)
  self.assertTrue(ok);self.assertIsNone(reason)
  bad_ok,_=drag.validate_drag_receipt(
   {'pid':42,'native':'ns','frame':'Femacs Drag Target','opened':'wrong'},
   [{'frame-live':True,'name':'Femacs Drag Target','x':120,'y':180}],
   [{'event':'draggingUpdated'},{'event':'performDragOperation'}],
   'hello\n',42)
  self.assertFalse(bad_ok)
  no_drop,_=drag.validate_drag_receipt(
   {'pid':42,'native':'ns','frame':'Femacs Drag Target','opened':'hello\n'},
   [{'frame-live':True,'name':'Femacs Drag Target','x':120,'y':180}],
   [{'event':'draggingUpdated'}],
   'hello\n',42)
  self.assertFalse(no_drop)
 def test_g4_retired_scaffolding_and_thin_wrappers_preserved(self):
  for rel in ('tools/verify-dod.sh','tools/gen-lisp-mk.py','tools/l1-worklist.sh',
              'tools/loadup-fixture.sh','tools/reconcile-link-ratchets.sh',
              'spikes/bytecode-hot/bytecode_hot.rs'):
   self.assertFalse((ROOT/rel).exists(),rel)
  shells=sorted((ROOT/'tools').glob('*.sh'))
  self.assertGreaterEqual(len(shells),10)
  for sh in shells:
   text=sh.read_text(encoding='utf-8')
   self.assertLessEqual(len(text.splitlines()),25,sh.name)
   self.assertNotIn('controlled-run',text,sh.name)
  matrix=(ROOT/'docs/support-matrix.md').read_text(encoding='utf-8')
  for retained in ('Linux','Windows','Haiku','Android arm64','macOS arm64 Cocoa'):
   self.assertIn(retained,matrix)
class Timing(unittest.TextTestResult):
 def startTest(self,test):self.started=time.monotonic();super().startTest(test)
 def stopTest(self,test):
  print(json.dumps({'case':test.id(),'seconds':time.monotonic()-self.started}),flush=True);super().stopTest(test)
if __name__=='__main__':unittest.main(testRunner=unittest.TextTestRunner(resultclass=Timing))
