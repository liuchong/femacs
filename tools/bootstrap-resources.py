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
from version_resources import render_versions
from module_api import render_module_api

ROOT = Path(__file__).resolve().parent.parent
RECIPES = ROOT/'build-support/generator-recipes.json'
FAMILIES = ('all','base','charsets','subdirs','versions','unicode','leim',
            'grammars','indexes','bytecode','doc','ja-dic','module-api')


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

    def lisp(self, *args, cwd=ROOT, prefer_newer=True):
        if not self.program or not self.program.is_file():
            raise RuntimeError('build the owned program first; pass --program PATH')
        self.run([str(self.program),'-Q','--batch','--no-site-file','--no-site-lisp',
                  '--eval','(setq load-prefer-newer '+('t' if prefer_newer else 'nil')+')',*[str(a) for a in args if str(a) not in {'-batch','--batch','--no-site-file','--no-site-lisp'}]],cwd=cwd)

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
        render_versions(ROOT)

    def base(self):
        self.charsets();self.subdirs();self.versions();self.module_api()

    def module_api(self):
        render_module_api(ROOT)

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

    def _compile_bytecode_group(self, files, *, native=False, parsers=False,
                                required=False, prefer_newer=True):
        if not files:
            return
        receipt=self.log/f'bytecode-{self.sequence+1:04d}.tsv'
        # Let the real compiler read file-local variables. A textual guess at
        # no-byte-compile could silently omit a distributable library.
        expression="""(let ((files 'FILES) (failed nil))
          (require 'bytecomp)
          (with-temp-file RECEIPT
            (dolist (file files)
              (condition-case err
                  (let* ((result (byte-compile-file file))
                         (status (cond ((eq result 'no-byte-compile) "skipped")
                                       (result "compiled") (t "failed"))))
                    (when (null result) (setq failed t))
                    (insert status "\\t" file "\\n"))
                (error (setq failed t)
                       (insert "failed\\t" file "\\t" (format "%S" err) "\\n")))))
          (when failed (kill-emacs 1)))"""
        expression=expression.replace('FILES','('+' '.join(json.dumps(str(p)) for p in files)+')')
        expression=expression.replace('RECEIPT',json.dumps(str(receipt)))
        self.lisp('--eval',"(setq byte-compile-error-on-warn nil org--inhibit-version-check t native-comp-jit-compilation nil)",
                  *(['--eval',"(require 'charprop \"international/charprop\")"]
                    if (ROOT/'lisp/international/charprop.el').is_file() else []),
                  *(['--eval','(setq native-comp-enable-subr-trampolines nil)',
                     '-l','comp'] if native else []),
                  *(['-l','cl-extra'] if parsers else []),
                  '--eval',expression,prefer_newer=prefer_newer)
        rows=[row.split('\t',2) for row in receipt.read_text().splitlines()]
        if len(rows)!=len(files):
            raise RuntimeError('incomplete release bytecode receipt')
        for p,row in zip(files,rows):
            if len(row)!=2 or row[1]!=str(p):
                raise RuntimeError('invalid release bytecode receipt')
            output=p.with_suffix('.elc')
            if row[0]=='skipped' and not required:
                if output.exists():raise RuntimeError('unexpected skipped bytecode: '+str(p.relative_to(ROOT)))
            elif row[0]!='compiled' or not output.is_file():
                raise RuntimeError('missing release bytecode: '+str(output.relative_to(ROOT)))

    def bytecode(self):
        # Release means the entire Lisp distribution, not just the dump closure.
        # Explicit scopes remain useful for developer-only incremental work.
        lisp=ROOT/'lisp'
        if BYTECODE_SCOPES:
            files=[]
            for name in BYTECODE_SCOPES:
                path=(ROOT/name).resolve()
                if not path.is_relative_to(lisp) or not path.exists():
                    raise RuntimeError('bytecode scope must be an existing owned Lisp path')
                files.extend(sorted(path.rglob('*.el')) if path.is_dir() else [path])
            files=list(dict.fromkeys(files))
            for p in files:p.with_suffix('.elc').unlink(missing_ok=True)
            for n in range(0,len(files),40):
                self._compile_bytecode_group(files[n:n+40])
            return
        files=sorted(p for p in lisp.rglob('*.el') if not p.name.startswith('.'))
        if any(not p.resolve().is_relative_to(lisp) for p in files):
            raise RuntimeError('release Lisp input escapes owned tree')
        spec=importlib.util.spec_from_file_location('program_products',ROOT/'tools/gen-program-products.py')
        products=importlib.util.module_from_spec(spec);spec.loader.exec_module(products)
        grammars=[ROOT/target for maker,_source,target in products.PRODUCTS if maker!='pinyin']
        for p in grammars:
            if not p.is_file():raise RuntimeError('missing generated parser input: '+str(p.relative_to(ROOT)))
        native_file=self.log/'bootstrap-native.txt'
        self.lisp('--eval','(with-temp-file '+json.dumps(str(native_file))+
                  " (prin1 (featurep 'native-compile) (current-buffer)))")
        native_text=native_file.read_text().strip()
        if native_text not in {'t','nil'}:raise RuntimeError('invalid owned native capability')
        native=native_text=='t'
        compiler_names=['macroexp','cconv','byte-opt','bytecomp']
        if native:compiler_names+=['comp','comp-cstr','comp-common','comp-run']
        compiler_names+=['loaddefs-gen','radix-tree','cl-extra']
        compiler=[lisp/'emacs-lisp'/(name+'.el') for name in compiler_names]
        main=[lisp/(name+'.el') for name in (
            'emacs-lisp/eieio','emacs-lisp/eieio-base','cedet/semantic/db',
            'emacs-lisp/cconv','international/ja-dic-cnv','org/ox','org/ox-texinfo',
            'org/org-macro','org/org-element','org/oc','org/ol','emacs-lisp/cl-lib')]
        for p in compiler+main:
            if not p.is_file():raise RuntimeError('missing release Lisp input: '+str(p.relative_to(ROOT)))
        # Neither previous release outputs nor orphan bytecode may satisfy this run.
        for p in lisp.rglob('*.elc'):p.unlink()
        for p in compiler:
            self._compile_bytecode_group([p],native=native,required=True,prefer_newer=False)
        main=[p for p in main if p not in compiler]
        for p in main:
            self._compile_bytecode_group([p],native=native,parsers=True,required=True)
        remaining=[p for p in files if p not in compiler+main+grammars]
        for n in range(0,len(remaining),40):
            self._compile_bytecode_group(remaining[n:n+40],native=native,parsers=True)
        # Bovine table actions must be compiled, with the object-system helper
        # explicitly loaded even with the first source-only bootstrap dump.
        self._compile_bytecode_group(grammars,native=native,parsers=True,required=True)

    def doc(self):
        self.run([sys.executable,'tools/gen-doc.py','--owned','--write'])

    def all(self):
        # base is also run before first bootstrap dump by each build entry.
        self.module_api()
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
        selected=FAMILIES[2:] if args.family=='all' else (('charsets','subdirs','versions','module-api') if args.family=='base' else (args.family,))
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
    except (OSError,RuntimeError,ValueError,subprocess.SubprocessError) as e:
        print(f'bootstrap {args.family}: {e}',file=sys.stderr)
        return 1
    return 0


if __name__=='__main__':
    raise SystemExit(main())
