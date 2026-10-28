"""Focused generator component protection; does not execute an editor/compiler."""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import shutil
import sys
import time

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO/'tools'))


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def run(generator, out):
    products = load('parser_bytecode_products', REPO/'tools/gen-program-products.py')
    tables = [target for maker, _source, target in products.PRODUCTS if maker != 'pinyin']
    records = []
    for case in ('release', 'missing-source', 'missing-bytecode', 'explicit-scope', 'all-bytecode', 'source-skip', 'parser-skip', 'incomplete-receipt'):
        started = time.monotonic()
        root = (out/case).resolve()
        root.mkdir(parents=True)
        for name in ('tools', 'build-support', 'lisp/emacs-lisp'):
            (root/name).mkdir(parents=True, exist_ok=True)
        shutil.copyfile(REPO/'tools/gen-program-products.py', root/'tools/gen-program-products.py')
        (root/'build-support/generator-recipes.json').write_text('{}\n')
        (root/'lisp/loadup.el').write_text('(load "core")\n')
        (root/'lisp/core.el').write_text(';;; component fixture\n')
        for name in ('macroexp', 'cconv', 'byte-opt', 'bytecomp', 'loaddefs-gen', 'radix-tree', 'cl-extra'):
            (root/'lisp/emacs-lisp'/(name+'.el')).write_text(';;; component fixture\n')
        for target in tables:
            path = root/target
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(';;; generated-table fixture\n')
            path.with_suffix('.elc').write_bytes(b'stale table fixture')
        for name in ('emacs-lisp/eieio', 'emacs-lisp/eieio-base', 'cedet/semantic/db',
                     'international/ja-dic-cnv', 'org/ox', 'org/ox-texinfo',
                     'org/org-macro', 'org/org-element', 'org/oc', 'org/ol',
                     'emacs-lisp/cl-lib', 'lazy-library', 'declined'):
            path=root/'lisp'/(name+'.el')
            path.parent.mkdir(parents=True,exist_ok=True)
            path.write_text(';;; component fixture\n')
        original_hashes = {target: hashlib.sha256((root/target).with_suffix('.elc').read_bytes()).hexdigest() for target in tables}
        if case == 'missing-source':
            # Use an input layout without this source, not an editor/test mutation.
            absent = root/tables[0]
            absent.rename(absent.with_suffix('.unavailable'))
        if case == 'explicit-scope':
            for target in tables:
                path = root/target
                path.rename(path.with_suffix('.unavailable'))
        generator.ROOT = root
        generator.RECIPES = root/'build-support/generator-recipes.json'
        generator.ALL_BYTECODE = case == 'all-bytecode'
        generator.BYTECODE_SCOPES = ['lisp/core.el'] if case == 'explicit-scope' else []
        compiled = []
        stale_observed = []

        class FixtureGenerator(generator.Generator):
            def lisp(self, *args, **kwargs):
                (self.log/'bootstrap-load-history.txt').write_text('')
                (self.log/'bootstrap-native.txt').write_text('nil')
                expression=str(args[-1])
                if not expression.startswith("(let ((files '"):
                    return
                # Decode only the quoted JSON path arguments at the process
                # boundary; do not emulate the compiler or execute its code.
                decoder=json.JSONDecoder()
                pending=expression.split("(files '(",1)[1].lstrip()
                paths=[]
                while not pending.startswith(')'):
                    name,end=decoder.raw_decode(pending)
                    paths.append(Path(name));pending=pending[end:].lstrip()
                receipt,_=decoder.raw_decode(expression.split('(with-temp-file ',1)[1].lstrip())
                rows=[]
                for path in paths:
                    relative=str(path.relative_to(root))
                    compiled.append(relative)
                    if relative in tables:
                        stale_observed.append(path.with_suffix('.elc').exists())
                    skip=(case=='source-skip' and relative=='lisp/declined.el') or (case=='parser-skip' and relative==tables[0])
                    rows.append(('skipped' if skip else 'compiled')+'\t'+str(path)+'\n')
                    if skip or (case=='missing-bytecode' and relative==tables[0]):
                        continue
                    path.with_suffix('.elc').write_bytes(b'new compiled fixture')
                if case=='incomplete-receipt':rows=rows[:-1]
                Path(receipt).write_text(''.join(rows))

        error = None
        try:
            FixtureGenerator(None).bytecode()
        except RuntimeError as failure:
            error = str(failure)
        if case == 'missing-source':
            assert error and error.startswith('missing generated parser input:')
            assert not compiled
        elif case == 'missing-bytecode':
            assert error and error.startswith('missing release bytecode:')
            assert not (root/tables[0]).with_suffix('.elc').exists()
        elif case == 'parser-skip':
            assert error and error.startswith('missing release bytecode:')
            assert not (root/tables[0]).with_suffix('.elc').exists()
        elif case == 'incomplete-receipt':
            assert error == 'incomplete release bytecode receipt'
        elif case == 'explicit-scope':
            assert error is None and compiled == ['lisp/core.el']
            assert all(hashlib.sha256((root/target).with_suffix('.elc').read_bytes()).hexdigest() == old for target, old in original_hashes.items())
        else:
            assert error is None
            assert all(compiled.count(target) == 1 for target in tables)
            assert compiled.count('lisp/lazy-library.el')==1
            assert (root/'lisp/declined.elc').exists() == (case!='source-skip')
            assert stale_observed and not any(stale_observed)
            assert all((root/target).with_suffix('.elc').read_bytes() == b'new compiled fixture' for target in tables)
        records.append(dict(id=case, passed=True, seconds=time.monotonic()-started,
                            compiled=compiled, expected_error=error))
    started=time.monotonic()
    process_generator=generator.Generator(Path(sys.executable))
    calls=[]
    process_generator.run=lambda argv,**kwargs:calls.append(argv)
    process_generator.lisp('--eval','nil',prefer_newer=False)
    process_generator.lisp('--eval','nil')
    assert '(setq load-prefer-newer nil)' in calls[0]
    assert '(setq load-prefer-newer t)' in calls[1]
    records.append(dict(id='compiler-load-mode',passed=True,seconds=time.monotonic()-started))
    return dict(scope='generator component with filesystem/compiler substitute; not editor compatibility',
                passed=True, cases=records)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--generator', type=Path, default=REPO/'tools/bootstrap-resources.py')
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=False)
    generator = load('parser_bytecode_generator', args.generator.resolve())
    report = run(generator, args.out)
    report['generator_sha256'] = hashlib.sha256(args.generator.read_bytes()).hexdigest()
    (args.out/'results.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(dict(passed=report['passed'], cases=len(report['cases']))))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
