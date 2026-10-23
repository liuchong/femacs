#!/usr/bin/env python3
"""Generate owned resources without a reference checkout or installed editor.

Run through the build/runtime controller when this launches the owned binary.
base and doc need only source files and the declared external command tools.
"""
from __future__ import annotations
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
RECIPES = ROOT/'build-support/generator-recipes.json'
FAMILIES = ('all','base','charsets','subdirs','versions','unicode','leim',
            'grammars','indexes','bytecode','doc','ja-dic')


class Generator:
    def __init__(self, program: Path | None):
        self.program = program.resolve() if program else None
        self.recipes = json.loads(RECIPES.read_text())
        self.log = ROOT/'build/generation'/str(time.monotonic_ns())
        self.log.mkdir(parents=True, exist_ok=True)
        self.sequence = 0

    def run(self, argv, *, cwd=ROOT, output=None):
        self.sequence += 1
        path = self.log/f'{self.sequence:04d}-{Path(argv[0]).name}.log'
        start = time.monotonic()
        env = dict(os.environ)
        # No user init, user load-path, installed editor or stale dump may supply
        # bootstrap inputs. The explicit binary and owned Lisp tree are mandatory.
        env.update(EMACSLOADPATH=str(ROOT/'lisp'), LC_ALL='C', AWK='gawk')
        env.pop('EMACSDATA',None)
        env.pop('EMACSDOC',None)
        with path.open('wb') as log:
            if output:
                dest=ROOT/output
                dest.parent.mkdir(parents=True,exist_ok=True)
                with dest.open('wb') as out:
                    p=subprocess.run(argv,cwd=cwd,env=env,stdout=out,stderr=log)
            else:
                p=subprocess.run(argv,cwd=cwd,env=env,stdout=log,stderr=subprocess.STDOUT)
        print(f'{self.sequence}: {Path(argv[0]).name} {time.monotonic()-start:.2f}s exit={p.returncode}',flush=True)
        if p.returncode:
            print(path.read_text(errors='replace')[-5000:],file=sys.stderr)
            raise RuntimeError(f'generator failed; see {path.relative_to(ROOT)}')

    def lisp(self, *args, cwd=ROOT):
        if not self.program or not self.program.is_file():
            raise RuntimeError('build the owned program first; pass --program PATH')
        self.run([str(self.program),'-Q','--batch','--no-site-file','--no-site-lisp',
                  '--eval','(setq load-prefer-newer t)',*[str(a) for a in args if str(a) not in {'-batch','--batch','--no-site-file','--no-site-lisp'}]],cwd=cwd)

    def charsets(self):
        if not shutil.which('gawk'):
            raise RuntimeError('GNU awk is required: brew install gawk')
        # The JIS helper is generated before the conversion that reads it.
        rows=sorted(self.recipes['charsets'],key=lambda r:r['output']!='admin/charsets/jisx2131-filter')
        for row in rows:
            (ROOT/row['output']).parent.mkdir(parents=True,exist_ok=True)
            self.run(['bash','-o','pipefail','-c',row['shell']])
        self.run(['gawk','-f','admin/unidata/blocks.awk',
                  'admin/unidata/Blocks.txt','admin/unidata/emoji-data.txt'],
                 output='lisp/international/charscript.el')
        self.run(['gawk','-f','admin/unidata/emoji-zwj.awk',
                  *['admin/unidata/'+n for n in ('emoji-zwj-sequences.txt',
                    'emoji-sequences.txt','emoji-variation-sequences.txt','emoji-test.txt')]],
                 output='lisp/international/emoji-zwj.el')

    def subdirs(self):
        dirs=sorted((p.name for p in (ROOT/'lisp').iterdir() if p.is_dir()
                     and p.name not in {'term','RCS','CVS','Old'} and not p.name.startswith(('.','='))))
        dirs=[p for p in reversed(dirs) if p!='obsolete']+(['obsolete'] if 'obsolete' in dirs else [])
        text=';; Generated from owned Lisp directory names. -*- lexical-binding: t -*-\n'
        text+="(normal-top-level-add-to-load-path '("+' '.join(json.dumps(p) for p in dirs)+'))\n'
        text+=';; Local Variables:\n;; no-byte-compile: t\n;; no-update-autoloads: t\n;; End:\n'
        (ROOT/'lisp/subdirs.el').write_text(text)

    def versions(self):
        text=(ROOT/'src/emacs.rs').read_text()
        version=re.search(r'PACKAGE_VERSION: &CStr = c"([^"]+)"',text)[1]
        for source,target,token,value in [
            ('doc/emacs/emacsver.texi.in','doc/emacs/emacsver.texi','@version@',version),
            ('etc/refcards/emacsver.tex.in','etc/refcards/emacsver.tex','@majorversion@',version.split('.')[0])]:
            (ROOT/target).write_text((ROOT/source).read_text().replace(token,value))

    def base(self):
        self.charsets();self.subdirs();self.versions()

    def unicode(self):
        source=ROOT/'admin/unidata/UnicodeData.txt'
        lines=[]
        for line in source.read_text().splitlines():
            head,_,rest=line.partition(';')
            lines.append('(#x'+head+' "'+rest.replace(';','" "')+'")')
        (ROOT/'admin/unidata/unidata.txt').write_text('\n'.join(lines)+'\n')
        # Bootstrap must not load a charprop that names not-yet-generated tables.
        required=[a[a.index('-f')+2] for a in self.recipes['unicode']
                  if a[a.index('-f')+1]=='unidata-gen-file']
        if any(not (ROOT/p).is_file() for p in required):
            (ROOT/'lisp/international/charprop.el').unlink(missing_ok=True)
            (ROOT/'lisp/international/charprop.elc').unlink(missing_ok=True)
        self.lisp('-L','admin/unidata','-f','batch-byte-compile','admin/unidata/unidata-gen.el')
        for args in sorted(self.recipes['unicode'],key=lambda a:'unidata-gen-charprop' in a):
            absolute=[str(ROOT/a) if a.startswith(('admin/','lisp/')) else a for a in args]
            self.lisp(*absolute,cwd=ROOT/'admin/unidata')
        self.lisp('-l','international/emoji','-f','emoji--generate-file',
                  str(ROOT/'lisp/international/emoji-labels.el'))

    def leim(self):
        (ROOT/'lisp/leim/quail').mkdir(parents=True,exist_ok=True)
        for args in self.recipes['leim']:
            self.lisp(*args)
        target=ROOT/'lisp/leim/leim-list.el'
        target.unlink(missing_ok=True)
        self.lisp('-l','international/quail','--eval',
                  '(update-leim-list-file "'+str(ROOT/'lisp/leim')+'")')
        with target.open('a') as out:
            for line in (ROOT/'leim/leim-ext.el').read_text().splitlines(True):
                if not line.startswith(';'):out.write(line)
                elif re.match(r'^;;*inc ',line):out.write(re.sub(r'^;(;*)inc ',r';\1 ',line))
        # Some consumers look under leim/ rather than lisp/leim/.
        shutil.copyfile(target,ROOT/'leim/leim-list.el')

    def ja_dic(self):
        (ROOT/'lisp/leim/ja-dic').mkdir(parents=True,exist_ok=True)
        self.lisp(*self.recipes['ja-dic'])

    def grammars(self):
        spec=importlib.util.spec_from_file_location('program_products',ROOT/'tools/gen-program-products.py')
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
        for maker,source,target in m.PRODUCTS:
            if maker=='pinyin':continue # already covered by leim recipes
            function='bovine-batch-make-parser' if maker=='bovine' else 'wisent-batch-make-parser'
            library='semantic/'+('bovine' if maker=='bovine' else 'wisent')+'/grammar'
            (ROOT/target).parent.mkdir(parents=True,exist_ok=True)
            self.lisp('-l',library,'-f',function,'-o',str(ROOT/target),str(ROOT/source))

    def indexes(self):
        lisp=ROOT/'lisp'
        dirs=['.']+sorted(str(p.relative_to(lisp)) for p in lisp.rglob('*') if p.is_dir()
                         and not any(x.startswith('.') or x in {'obsolete','term'} for x in p.relative_to(lisp).parts))
        self.lisp('-f','batch-byte-compile','emacs-lisp/loaddefs-gen.el',cwd=lisp)
        self.lisp('-l','emacs-lisp/loaddefs-gen.elc','-f','loaddefs-generate--emacs-batch',*dirs,cwd=lisp)
        self.lisp('-l','cus-dep','--eval','(setq generated-custom-dependencies-file "cus-load.el")',
                  '-f','custom-make-dependencies',*dirs,cwd=lisp)
        self.lisp('-l','finder','--eval','(setq generated-finder-keywords-file \"finder-inf.el\")','-f','finder-compile-keywords-make-dist',*[d for d in dirs if not d.startswith('leim')],cwd=lisp)

    def bytecode(self):
        # Compile the dump/loadup closure for the release. Other libraries remain
        # usable source; --all-bytecode explicitly compiles every Lisp source.
        names=re.findall(r'^\s*\(load "([^"]+)"', (ROOT/'lisp/loadup.el').read_text(),re.M)
        files=[]
        for name in names:
            p=ROOT/'lisp'/(name if name.endswith('.el') else name+'.el')
            if p.is_file() and p not in files:files.append(p)
        # Source-only bootstrap loads additional libraries through require.
        # Their actual load history, plus generated tables, forms the dump closure.
        history=self.log/'bootstrap-load-history.txt'
        self.lisp('--eval','(with-temp-file '+json.dumps(str(history))+' (dolist (entry load-history) (when (stringp (car entry)) (insert (car entry) "\\n"))))')
        for name in history.read_text().splitlines():
            p=Path(name)
            if p.suffix=='.elc':p=p.with_suffix('.el')
            if p.is_file() and p.is_relative_to(ROOT/'lisp') and p not in files:files.append(p)
        for name in ['charscript','emoji-zwj','emoji-labels','charprop']:
            p=ROOT/'lisp/international'/(name+'.el')
            if p.is_file() and p not in files:files.append(p)
        if ALL_BYTECODE:files=sorted((ROOT/'lisp').rglob('*.el'))
        if BYTECODE_SCOPES:
            files=[]
            for name in BYTECODE_SCOPES:
                path=(ROOT/name).resolve()
                if not path.is_relative_to(ROOT/'lisp') or not path.exists():raise RuntimeError('bytecode scope must be an existing owned Lisp path')
                files.extend(sorted(path.rglob('*.el')) if path.is_dir() else [path])
        for n in range(0,len(files),40):
            self.lisp('--eval',"(setq byte-compile-error-on-warn nil org--inhibit-version-check t)",
                      '-f','batch-byte-compile',*files[n:n+40])

    def doc(self):
        self.run([sys.executable,'tools/gen-doc.py','--owned','--write'])

    def all(self):
        # base is also run before first bootstrap dump by each build entry.
        self.unicode();self.leim();self.ja_dic();self.grammars()
        self.indexes();self.bytecode();self.doc()


ALL_BYTECODE=False
BYTECODE_SCOPES=[]


def main():
    global ALL_BYTECODE, BYTECODE_SCOPES
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('scopes',nargs='*',help='optional owned Lisp paths for bytecode generation')
    ap.add_argument('--family',choices=FAMILIES,default='all')
    ap.add_argument('--program',type=Path)
    ap.add_argument('--all-bytecode',action='store_true')
    ap.add_argument('--list',action='store_true',help='print the owned generation plan without running it')
    args=ap.parse_args()
    ALL_BYTECODE=args.all_bytecode
    BYTECODE_SCOPES=args.scopes
    if args.scopes and args.family!="bytecode":ap.error("positional paths apply only to --family bytecode")
    if args.list:
        recipes=json.loads(RECIPES.read_text())
        selected=FAMILIES[2:] if args.family=='all' else (('charsets','subdirs','versions') if args.family=='base' else (args.family,))
        for family in selected:
            print(family)
            if family=='charsets':
                for row in recipes[family]:print('  '+row['output'])
            elif family=='unicode':
                for row in recipes[family]:print('  '+row[row.index('-f')+2])
        return 0
    program=args.program
    if not program:
        candidates=[ROOT/'build/terminal/port/src/emacs',ROOT/'build/macos/port/src/emacs',ROOT/'dist/macos/Femacs.app/Contents/MacOS/Femacs']
        program=next((p for p in candidates if p.is_file()),candidates[0])
    try:
        getattr(Generator(program),args.family.replace('-','_'))()
    except (OSError,RuntimeError,subprocess.SubprocessError) as e:
        print(f'bootstrap {args.family}: {e}',file=sys.stderr)
        return 1
    return 0


if __name__=='__main__':
    raise SystemExit(main())
