#!/usr/bin/env python3
"""Standalone repository acceptance; no reference checkout is required.

Run cheap source/input checks by default. --export writes only staged, tracked
files to a NEW directory for the controlled full build acceptance.
"""
from pathlib import Path
import argparse
import hashlib
import json
import subprocess
import time

ROOT = Path(__file__).resolve().parent.parent


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--before', type=Path)
    ap.add_argument('--export', type=Path)
    args = ap.parse_args()
    tests = []

    def case(name, run):
        start = time.monotonic()
        run()
        tests.append({'case': name, 'seconds': time.monotonic()-start, 'status': 'passed'})
        print(json.dumps(tests[-1]), flush=True)

    def layout():
        assert not (ROOT/'femacs').exists(), 'old implementation root remains'
        for p in ['Cargo.toml', 'rust-toolchain.toml', 'src/emacs.rs',
                  'platform/macos/nsterm.swift', 'xtask/src/main.rs',
                  'lisp/ldefs-boot.el', 'admin/unidata/UnicodeData.txt',
                  'admin/grammars/c.by', 'leim/MISC-DIC/pinyin.map']:
            assert (ROOT/p).is_file(), p
            result = subprocess.run(['git','check-ignore','--no-index','-q',p], cwd=ROOT)
            assert result.returncode == 1, f'source ignored: {p}'
    case('root layout and unignored raw inputs', layout)

    def ignore_rules():
        for name in ['lisp/loaddefs.el','lisp/theme-loaddefs.el','lisp/sample.elc',
                     'etc/charsets/8859-2.map','target/temporary-file']:
            result=subprocess.run(['git','check-ignore','--no-index','-q',name],cwd=ROOT)
            assert result.returncode==0, 'generated output exposed: '+name
        for name in ['test/data/bootstrap.xml','etc/images/source.svg','etc/icons/source.svg']:
            result=subprocess.run(['git','check-ignore','--no-index','-q',name],cwd=ROOT)
            assert result.returncode==1, 'source/fixture broadly ignored: '+name
    case('root ignore rules distinguish generated output from raw inputs',ignore_rules)

    def descriptions():
        for target in ['terminal','cocoa']:
            base = ROOT/'build-support/macos'/target
            for name in ['objects.txt','lib-objects.txt','libraries.txt','config.json']:
                assert (base/name).is_file(), base/name
            for obj in (base/'objects.txt').read_text().split():
                stem = Path(obj).stem
                assert (ROOT/'src'/(stem+'.rs')).exists() or (
                    ROOT/'platform/macos'/(stem+'.swift')).exists(), obj
        recipes = json.loads((ROOT/'build-support/generator-recipes.json').read_text())
        assert len(recipes['charsets']) > 100
        assert len(recipes['unicode']) > 10
        text = json.dumps(recipes)
        assert 'emacs/' not in text and '../' not in text, 'external recipe path'
        for source in ['tools/bootstrap-resources.py','xtask/src/macos.rs']:
            text = (ROOT/source).read_text()
            assert 'emacs/configure' not in text, source
    case('owned build descriptions', descriptions)

    if args.before:
        def preservation():
            old = json.loads(args.before.read_text())
            for name, digest in old.items():
                rel = Path(name).relative_to('femacs')
                if rel.parts[0] not in ['lisp','test','etc','doc','leim','admin']:
                    continue
                assert hashlib.sha256((ROOT/rel).read_bytes()).hexdigest() == digest, str(rel)
        case('moved data byte preservation', preservation)

    if args.export:
        dest = args.export.resolve()
        assert not dest.exists(), 'export destination must be new'
        dest.mkdir(parents=True)
        subprocess.run(['git','checkout-index','--all','--prefix',str(dest)+'/'],
                       cwd=ROOT,check=True)
        assert not any((dest/p).exists() for p in ['emacs','build','.femacs-cache','target'])
        print(f'fresh tracked-file export: {dest}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
