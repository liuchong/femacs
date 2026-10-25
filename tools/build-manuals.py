#!/usr/bin/env python3
"""Generate the complete owned Info/optional HTML set into fresh staging."""
from __future__ import annotations
import argparse
from dataclasses import dataclass,replace
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time
import uuid
from version_resources import render_versions

REPO=Path(__file__).resolve().parent.parent


@dataclass(frozen=True)
class ManualSpec:
    source:Path
    info_name:str
    language:str
    org_source:bool
    include_dirs:tuple[Path,...]=()


def digest(path:Path)->str:
    result=hashlib.sha256()
    with path.open('rb') as source:
        for block in iter(lambda:source.read(1024*1024),b''):result.update(block)
    return result.hexdigest()


def owned_input(repo:Path,name:str)->Path:
    relative=Path(name)
    if relative.is_absolute() or not relative.parts or any(p in {'.','..'} for p in relative.parts):
        raise ValueError('input must be an owned relative path: '+name)
    path=(repo/relative).resolve()
    if not path.is_relative_to(repo.resolve()) or path.is_relative_to((repo/'emacs').resolve()):
        raise ValueError('input escapes owned source: '+name)
    if not path.is_file():raise ValueError('missing declared manual input: '+name)
    return path


def read_manifest(repo:Path)->dict:
    manifest=json.loads((repo/'build-support/manuals.json').read_text())
    if manifest.get('schema')!='femacs-manuals-v1':raise ValueError('unsupported manual manifest')
    inputs=manifest.get('staged_inputs',[])
    if not inputs or len(inputs)!=len(set(inputs)):raise ValueError('empty/duplicate staged inputs')
    for name in inputs:owned_input(repo,name)
    names=set()
    for row in manifest.get('manuals',[]):
        name=row['info_name']
        if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]*',name) or name in names:
            raise ValueError('invalid/duplicate manual output stem: '+name)
        names.add(name)
        if row['source'] not in inputs:raise ValueError('manual source absent from staged inputs')
        source=owned_input(repo,row['source'])
        if bool(row['org_source'])!=(source.suffix=='.org'):raise ValueError('manual source kind mismatch')
        if not row['org_source']:
            declaration=re.search(r'^@setfilename\s+(\S+)',source.read_text(),re.M)
            if declaration is None or Path(declaration[1]).name!=name+'.info':
                raise ValueError('manual declaration differs from its explicit output: '+name)
    if not names:raise ValueError('no declared manual roots')
    return manifest


def discover_manuals(repo:Path)->list[ManualSpec]:
    manifest=read_manifest(repo)
    result=[]
    for row in manifest['manuals']:
        source=owned_input(repo,row['source'])
        include_dirs=tuple(dict.fromkeys([source.parent,*[
            (repo/name).resolve() for name in manifest['include_roots']]]))
        if any(not p.is_dir() or not p.is_relative_to(repo.resolve()) for p in include_dirs):
            raise ValueError('invalid owned manual include root')
        result.append(ManualSpec(source,row['info_name'],row['language'],bool(row['org_source']),include_dirs))
    return result


def environment()->dict[str,str]:
    env=dict(os.environ)
    for name in ('EMACSLOADPATH','EMACSDATA','EMACSDOC','EMACSPATH','INFOPATH'):
        env.pop(name,None)
    env['LC_ALL']='C'
    return env


def run_step(argv:list[str],cwd:Path,log:Path)->dict:
    started=time.monotonic();log.parent.mkdir(parents=True,exist_ok=True)
    with log.open('xb') as output:
        try:
            child=subprocess.run(argv,cwd=cwd,env=environment(),stdout=output,stderr=subprocess.STDOUT)
            code=child.returncode
        except OSError as error:
            output.write(str(error).encode());code=127
    result={'argv':argv,'exit_code':code,'seconds':time.monotonic()-started,'log':str(log)}
    if code:raise RuntimeError(f'command failed ({code}); see {log}')
    return result


def export_org(program:Path,spec:ManualSpec,out:Path)->dict:
    if not spec.org_source:raise ValueError('Org exporter requires an Org source')
    if out.resolve()!=spec.source.parent.resolve():raise ValueError('Org export must preserve its staged source cwd')
    output=out/(spec.info_name+'.texi')
    if output.exists():raise ValueError('Org staging must not inherit old export '+str(output))
    org_dir=out.parent.parent/'lisp/org'
    if not (org_dir/'ox-texinfo.el').is_file():raise ValueError('staged Org exporter missing')
    # The source-only staged directory excludes ignored bytecode and user packages.
    # Reject a preloaded Org so it cannot mask the staged/hash-bound implementation.
    guard="(when (featurep 'org) (error \"Org already preloaded outside staged exporter\"))"
    verify="(dolist (entry load-history) (let ((file (car entry))) (when (and (stringp file) (string-match-p \"/org/[^/]+\\\\.el\" file) (not (file-in-directory-p file "+json.dumps(str(org_dir))+"))) (error \"Unstaged Org implementation %s\" file))))"
    args=[str(program),'-Q','--batch','--no-site-file','--no-site-lisp',
          '--eval','(setq load-prefer-newer t)',
          '--eval',guard,
          '--eval',"(add-to-list 'load-path "+json.dumps(str(org_dir))+")",
          '--eval','(setq org--inhibit-version-check t)','-l','ox-texinfo',
          '--eval','(setq gc-cons-threshold 50000000 org-confirm-babel-evaluate nil org-id-track-globally nil)',
          '--eval',verify,'-f','org-texinfo-export-to-texinfo-batch',spec.source.name,output.name]
    step=run_step(args,out,out.parent.parent.parent/'logs'/('export-'+spec.info_name+'.log'))
    if not output.is_file() or not output.stat().st_size:raise ValueError('Org export produced no current Texinfo')
    return {'output':str(output),'sha256':digest(output),'step':step}


def build_manual(spec:ManualSpec,format:str,out:Path)->dict:
    if format not in {'info','html'}:raise ValueError('unsupported manual format')
    makeinfo=shutil.which('makeinfo')
    if makeinfo is None:raise ValueError('makeinfo is required')
    destination=out/format/(spec.info_name+('.info' if format=='info' else '.html'))
    destination.parent.mkdir(parents=True,exist_ok=True)
    args=[makeinfo,'--no-split','--enable-encoding']
    if format=='html':args.append('--html')
    for directory in spec.include_dirs:args.extend(['-I',str(directory)])
    args.extend(['-o',str(destination),str(spec.source)])
    step=run_step(args,spec.source.parent,out/'logs'/(format+'-'+spec.info_name+'.log'))
    if not destination.is_file() or not destination.stat().st_size:raise ValueError('conversion output missing/empty')
    with destination.open('rb') as file:header=file.read(65536)
    if format=='info' and re.search(br'Node:\s*top\b',header,re.I) is None:
        raise ValueError('generated Info has no Top node: '+spec.info_name)
    if format=='html' and b'<html' not in header.lower():raise ValueError('generated HTML has no document root')
    return {'name':spec.info_name,'format':format,'path':str(destination.relative_to(out)),
            'sha256':digest(destination),'bytes':destination.stat().st_size,'step':step}


def new_output(requested:Path|None)->Path:
    root=REPO.resolve()
    raw=requested or root/'build/manuals'/f'{time.time_ns()}-{uuid.uuid4().hex}'
    if '..' in raw.parts or raw.is_symlink():raise ValueError('unsafe manual output')
    path=raw.resolve()
    if path==root or not path.is_relative_to(root):raise ValueError('manual output must be owned repository scratch')
    relative=path.relative_to(root)
    if relative.parts[0] not in {'build','.femacs-cache'}:raise ValueError('manual output must be a build/cache child')
    path.mkdir(parents=True,exist_ok=False)
    return path


def main(argv=None)->int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--program',type=Path,required=True)
    parser.add_argument('--format',choices=('info','html'),default='info')
    parser.add_argument('--out',type=Path)
    args=parser.parse_args(argv)
    try:
        manifest=read_manifest(REPO);specs=discover_manuals(REPO)
        program=args.program.resolve()
        if not program.is_file() or not os.access(program,os.X_OK):raise ValueError('owned editor executable is required')
        if shutil.which('makeinfo') is None:raise ValueError('makeinfo is required; install GNU Texinfo')
        if args.format=='info' and shutil.which('install-info') is None:raise ValueError('install-info from Texinfo is required')
        out=new_output(args.out)
    except (ValueError,OSError,KeyError,TypeError) as error:
        print('manual preflight: '+str(error),file=sys.stderr);return 2
    started=time.monotonic();errors=[];records=[];outputs=[];source_hashes={};exports={}
    stage=out/'src';stage.mkdir()
    try:
        for name in manifest['staged_inputs']:
            source=owned_input(REPO,name);target=stage/name
            target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,target)
            source_hashes[name]=digest(source)
        render_versions(stage)
        for spec in specs:
            source=stage/spec.source.relative_to(REPO.resolve())
            include_dirs=tuple(stage/p.relative_to(REPO.resolve()) for p in spec.include_dirs)
            staged=replace(spec,source=source,include_dirs=include_dirs)
            tick=time.monotonic()
            try:
                if staged.org_source:
                    exported=export_org(program,staged,source.parent);exports[staged.info_name]=exported
                    staged=replace(staged,source=Path(exported['output']),org_source=False)
                product=build_manual(staged,args.format,out);outputs.append(product)
                records.append({'name':spec.info_name,'passed':True,'seconds':time.monotonic()-tick})
            except (OSError,ValueError,RuntimeError) as error:
                message=spec.info_name+': '+str(error);errors.append(message)
                records.append({'name':spec.info_name,'passed':False,'seconds':time.monotonic()-tick,'error':message})
        if not errors and args.format=='info':
            install=shutil.which('install-info');index=out/'info/dir'
            for product in outputs:
                try:
                    run_step([install,str(out/product['path']),'--dir-file='+str(index)],out,
                             out/'logs'/('index-'+product['name']+'.log'))
                except RuntimeError as error:errors.append(str(error))
            if not index.is_file() or not index.stat().st_size:errors.append('complete Info directory index missing')
            else:outputs.append({'name':'dir','format':'info','path':'info/dir',
                                 'sha256':digest(index),'bytes':index.stat().st_size})
    except (OSError,ValueError,RuntimeError) as error:errors.append(str(error))
    passed=not errors and len(records)==len(specs) and all(r['passed'] for r in records)
    receipt={'schema':'femacs-manuals-v1','status':'passed' if passed else 'failed',
             'format':args.format,'expected_count':len(specs),'passed_count':sum(r['passed'] for r in records),
             'manuals':records,'products':outputs,'exports':exports,'inputs':source_hashes,
             'program_sha256':digest(program),'manifest_sha256':digest(REPO/'build-support/manuals.json'),
             'builder_sha256':digest(Path(__file__)),'version_recipe_sha256':digest(REPO/'tools/version_resources.py'),
             'errors':errors,'seconds':time.monotonic()-started}
    if passed:
        payload={'schema':'femacs-manual-set-v1','format':args.format,
                 'manifest_sha256':receipt['manifest_sha256'],
                 'program_sha256':receipt['program_sha256'],
                 'builder_sha256':receipt['builder_sha256'],
                 'version_recipe_sha256':receipt['version_recipe_sha256'],'inputs':source_hashes,
                 'files':{Path(p['path']).name:p['sha256'] for p in outputs}}
        (out/args.format/'.femacs-manuals.json').write_text(json.dumps(payload,indent=2)+'\n')
    (out/'receipt.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
    for record in records:print(f'{record["name"]}: passed={record["passed"]} seconds={record["seconds"]:.6f}')
    for error in errors:print(error,file=sys.stderr)
    print(f'manuals: {receipt["passed_count"]}/{len(specs)}; receipt={out/"receipt.json"}')
    return 0 if passed else 1


if __name__=='__main__':raise SystemExit(main())
