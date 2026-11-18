#!/usr/bin/env python3
"""Bounded release input selection tests; real byte compilation is separate acceptance."""
import importlib.util
from pathlib import Path
import tempfile
import time
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('resources',ROOT/'tools/bootstrap-resources.py')
resources=importlib.util.module_from_spec(spec);spec.loader.exec_module(resources)
COMPILER=['emacs-lisp/'+name+'.el' for name in ('byte-opt','bytecomp','macroexp','cconv','cl-extra')]

class CompilerInputs(unittest.TestCase):
    def setUp(self):
        scratch=ROOT/'build/compiler-resource-components';scratch.mkdir(parents=True,exist_ok=True)
        self.temp=tempfile.TemporaryDirectory(dir=scratch);self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name);self.log=self.root/'build/log';self.log.mkdir(parents=True)
        self.inputs=['simple.el','history.el',*COMPILER,*['international/'+n+'.el' for n in ('charscript','emoji-zwj','emoji-labels','charprop')]]
        for name in self.inputs:
            p=self.root/'lisp'/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(';; owned fixture\n')
        (self.root/'lisp/loadup.el').write_text('(load "simple")\n')
        self.calls=[];self.gen=resources.Generator.__new__(resources.Generator);self.gen.log=self.log
        def lisp(*args,**kwargs):
            if '-f' in args and 'batch-byte-compile' in args:
                selected=[Path(x) for x in args[args.index('batch-byte-compile')+1:]]
                self.calls.extend(selected)
                for p in selected:p.with_suffix('.elc').write_bytes(b'fixture output')
            else:(self.log/'bootstrap-load-history.txt').write_text(str(self.root/'lisp/history.el')+'\n')
        self.gen.lisp=lisp
        # This unit isolates selection of the original compiler inputs. The
        # generated component's real compilation/package path has its own integration.
        self.gen.library_corrections=lambda:None
        self.addCleanup(patch.stopall)
        patch.object(resources,'ROOT',self.root).start();patch.object(resources,'ALL_BYTECODE',False).start();patch.object(resources,'BYTECODE_SCOPES',[]).start()

    def test_default_preserves_closure_history_and_compiler_inputs_once(self):
        self.gen.bytecode()
        self.assertEqual(set(self.calls),{self.root/'lisp'/p for p in self.inputs})
        self.assertEqual(len(self.calls),len(set(self.calls)))

    def test_missing_required_compiler_source_fails(self):
        (self.root/'lisp'/COMPILER[0]).unlink()
        with self.assertRaisesRegex(RuntimeError,'missing release Lisp input'):self.gen.bytecode()

    def test_missing_compiler_output_fails_even_with_successful_command(self):
        for name in COMPILER:(self.root/'lisp'/name).with_suffix('.elc').write_bytes(b'stale output')
        original=self.gen.lisp
        def no_output(*args,**kwargs):
            if 'batch-byte-compile' not in args:original(*args,**kwargs)
        self.gen.lisp=no_output
        with self.assertRaisesRegex(RuntimeError,'missing release compiler bytecode'):self.gen.bytecode()

    def test_explicit_scope_stays_scoped(self):
        resources.BYTECODE_SCOPES=['lisp/simple.el'];self.gen.bytecode()
        self.assertEqual(self.calls,[self.root/'lisp/simple.el'])

class TimedResult(unittest.TextTestResult):
    def startTest(self,test):
        self.started=time.monotonic();super().startTest(test)
    def stopTest(self,test):
        self.stream.writeln(f"{test.id()}: {time.monotonic()-self.started:.6f}s")
        super().stopTest(test)

if __name__=='__main__':
    unittest.main(testRunner=unittest.TextTestRunner(verbosity=2,resultclass=TimedResult))
