#!/usr/bin/env python3
"""Bounded release input selection tests; real byte compilation is separate acceptance.

The release path in ``bootstrap-resources.Generator.bytecode`` selects the
compiler/main/grammar inputs, drives one editor subprocess per group, and then
validates the receipt that editor wrote.  These tests pin that contract at the
subprocess boundary: the fixture supplies the same inputs the release path
requires, the editor is replaced by a stub that performs exactly the side
effects the real expression performs (native capability file, receipt rows,
bytecode outputs).  Only the editor subprocess and the separate
``library_corrections`` component are isolated; the selection rules, the required
input checks and the receipt validation are the production code.  The stub is a
test fixture, not part of the generator.
"""
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import time
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('resources',ROOT/'tools/bootstrap-resources.py')
resources=importlib.util.module_from_spec(spec);spec.loader.exec_module(resources)
spec=importlib.util.spec_from_file_location('program_products',ROOT/'tools/gen-program-products.py')
products=importlib.util.module_from_spec(spec);spec.loader.exec_module(products)

# The release path requires these inputs before it may compile anything; the
# names are the contract, not a fixture convenience.
COMPILER=['emacs-lisp/'+name for name in ('macroexp','cconv','byte-opt','bytecomp',
                                            'loaddefs-gen','radix-tree','cl-extra')]
MAIN=['emacs-lisp/eieio','emacs-lisp/eieio-base','cedet/semantic/db',
      'international/ja-dic-cnv','org/ox','org/ox-texinfo','org/org-macro',
      'org/org-element','org/oc','org/ol','emacs-lisp/cl-lib']

class CompilerInputs(unittest.TestCase):
    def setUp(self):
        scratch=ROOT/'build/compiler-resource-components';scratch.mkdir(parents=True,exist_ok=True)
        self.temp=tempfile.TemporaryDirectory(dir=scratch);self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)
        self.addCleanup(patch.stopall)
        patch.object(resources,'ROOT',self.root).start()
        patch.object(resources,'BYTECODE_SCOPES',[]).start()
        # The real constructor owns the call sequence number the release path
        # puts in its receipts; bypassing it was the stale contract here.
        self.gen=resources.Generator(None);self.log=self.gen.log
        names=['lisp/'+name+'.el' for name in COMPILER]
        names+=['lisp/'+name+'.el' for name in MAIN]
        names+=['lisp/simple.el','lisp/history.el','lisp/international/charscript.el',
                'lisp/international/emoji-zwj.el','lisp/international/emoji-labels.el']
        # The release path imports this generator from ROOT and requires every
        # parser it names, so the fixture carries the real script and creates
        # the targets it declares (their paths are already root-relative).
        (self.root/'tools').mkdir(parents=True,exist_ok=True)
        (self.root/'tools/gen-program-products.py').write_bytes((ROOT/'tools/gen-program-products.py').read_bytes())
        names+= [target for maker,_source,target in products.PRODUCTS if maker!='pinyin']
        for name in dict.fromkeys(names):
            p=self.root/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(';; owned fixture\n')
        (self.root/'lisp/loadup.el').write_text('(load "simple")\n')
        self.calls=[];self.suppress_output=False;self.native='nil'
        def lisp(*args,**kwargs):
            self.gen.sequence+=1
            text=' '.join(str(a) for a in args)
            match=re.search(r'\(with-temp-file ("(?:[^"\\]|\\.)*")',text)
            if match is None:raise AssertionError('unexpected editor invocation: '+text[:200])
            target=Path(json.loads(match.group(1)))
            if '(featurep' in text:
                target.write_text(self.native+'\n');return
            segment=text.split("(let ((files '(",1)[1].split(')',1)[0]
            files=[json.loads(item) for item in re.findall(r'"(?:[^"\\]|\\.)*"',segment)]
            self.assertTrue(files)
            self.calls.extend(Path(f) for f in files)
            rows=[]
            for f in files:
                rows.append('compiled\t'+f)
                if not self.suppress_output:
                    Path(f).with_suffix('.elc').write_bytes(b'fixture output')
            target.write_text('\n'.join(rows)+'\n')
        self.gen.lisp=lisp
        # These components are installed into the product, not into its dump;
        # their own integration covers them.
        self.gen.library_corrections=lambda:None

    def test_default_release_path_compiles_every_input_once(self):
        self.gen.bytecode()
        expected=set((self.root/'lisp').rglob('*.el'))
        self.assertEqual(set(self.calls),expected)
        self.assertEqual(len(self.calls),len(set(self.calls)))

    def test_native_capability_requires_its_compiler_sources(self):
        # With native compilation the release path also requires the native
        # compiler Lisp sources; the fixture carries only the non-native set.
        self.native='t'
        with self.assertRaisesRegex(RuntimeError,
                r'missing release Lisp input: lisp/emacs-lisp/(comp|comp-cstr|comp-common|comp-run)\.el'):
            self.gen.bytecode()

    def test_missing_required_compiler_source_fails(self):
        (self.root/'lisp'/f'{COMPILER[0]}.el').unlink()
        with self.assertRaisesRegex(RuntimeError,'missing release Lisp input'):self.gen.bytecode()

    def test_missing_compiler_output_fails_even_with_successful_command(self):
        for name in COMPILER:
            (self.root/'lisp'/f'{name}.elc').write_bytes(b'stale output')
        self.suppress_output=True
        with self.assertRaisesRegex(RuntimeError,'missing release bytecode'):self.gen.bytecode()

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
