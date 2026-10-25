#!/usr/bin/env python3
"""Collect every requested strict ERT chunk in a fresh owned run directory."""
from __future__ import annotations
import argparse
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
import time
import uuid

REPO=Path(__file__).resolve().parent.parent


def parse_file_list(text:str)->list[str]:
    if len(text)>16*1024*1024:raise ValueError('test enumeration exceeds its bounded input')
    rows=text.splitlines();expected=None
    if rows:
        heading=re.fullmatch(r'enumerated (\d+) file\(s\)',rows[0])
        if heading:expected=int(heading[1]);rows=rows[1:]
    files=[];seen=set()
    for row in rows:
        name=row.strip();path=PurePosixPath(name)
        if not name or name!=row or path.is_absolute() or '\\' in name or \
                any(part in {'','.','..'} for part in name.split('/')) or path.suffix!='.el':
            raise ValueError('invalid enumerated relative test file: '+repr(row))
        if name in seen:raise ValueError('duplicate enumerated file: '+name)
        seen.add(name);files.append(name)
    if not files:raise ValueError('no enumerated test files')
    if expected is not None and expected!=len(files):raise ValueError('enumeration count does not match file rows')
    return files


def chunk_ranges(total:int,start:int,size:int)->list[tuple[int,int]]:
    if min(total,start)<0 or size<=0:raise ValueError('invalid total/start/chunk size')
    return [(at,min(at+size,total)) for at in range(start,total,size)]


def new_output(requested:Path|None)->Path:
    root=REPO.resolve()
    raw=requested or root/'build/acceptance/l5'/f'{time.time_ns()}-{uuid.uuid4().hex}'
    if '..' in raw.parts:raise ValueError('output must not contain parent traversal')
    if raw.is_symlink():raise ValueError('output must not be a symlink')
    path=raw.resolve()
    if path==root or not path.is_relative_to(root):raise ValueError('output must be an owned repository child')
    relative=path.relative_to(root)
    if relative.parts[0] in {'emacs','.git','src','lib','lib-src','xtask','lisp','leim','etc',
                             'doc','test','admin','platform','maintenance','build-support','tools'}:
        raise ValueError('output must not be a source/data/reference path')
    path.mkdir(parents=True,exist_ok=False)
    return path


def main(argv:list[str]|None=None)->int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--start-at',type=int,default=0)
    parser.add_argument('--chunk',type=int,default=30)
    parser.add_argument('--timeout',type=int,default=120)
    parser.add_argument('--out',type=Path)
    parser.add_argument('--gc-stress',action='store_true')
    parser.add_argument('--port-prog')
    parser.add_argument('--ref-prog')
    args=parser.parse_args(argv)
    if args.start_at<0 or min(args.chunk,args.timeout)<=0:parser.error('start must be nonnegative; chunk/timeout must be positive')
    checker=REPO/'tools/check-ert.py'
    enumeration=subprocess.run([sys.executable,str(checker),'--list'],cwd=REPO,
                               stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    if enumeration.returncode:
        print(enumeration.stderr or enumeration.stdout,file=sys.stderr);return enumeration.returncode
    try:
        files=parse_file_list(enumeration.stdout)
        if args.start_at>=len(files):raise ValueError('start offset selects no test files')
        out=new_output(args.out)
    except (ValueError,OSError) as error:
        print('L5 sweep: '+str(error),file=sys.stderr);return 2
    (out/'enumeration.stdout.log').write_text(enumeration.stdout)
    (out/'enumeration.stderr.log').write_text(enumeration.stderr)
    extra=[]
    if args.gc_stress:extra.append('--gc-stress')
    for option,value in (('--port-prog',args.port_prog),('--ref-prog',args.ref_prog)):
        if value is not None:extra.extend([option,value])
    records=[];all_passed=True;started=time.monotonic()
    for index,(start,end) in enumerate(chunk_ranges(len(files),args.start_at,args.chunk),1):
        stem=f'ert-l5-chunk{index:03d}';tsv=out/(stem+'.tsv');result_json=out/(stem+'.json')
        command=[sys.executable,str(checker),'--timeout',str(args.timeout),'--start-at',str(start),
                 '--max-files',str(end-start),*extra,'--tsv',str(tsv),'--results-json',str(result_json)]
        tick=time.monotonic();failure=None
        with (out/(stem+'.log')).open('xb') as log:
            try:
                child=subprocess.run(command,cwd=REPO,stdout=log,stderr=subprocess.STDOUT)
                code=child.returncode
            except OSError as error:
                log.write(str(error).encode());code=127
        passed=code==0
        try:
            if not tsv.is_file() or not tsv.stat().st_size:raise ValueError('chunk TSV missing or empty')
            rows=json.loads(result_json.read_text())['files']
            actual=[row['file'] for row in rows]
            if actual!=files[start:end]:raise ValueError('chunk evidence does not cover the requested ordered files')
            if not rows or any(row.get('verdict') not in {'same','port-better'} for row in rows):
                raise ValueError('chunk contains nonpassing or untested results')
        except (OSError,ValueError,KeyError,TypeError) as error:
            passed=False;failure=str(error)
        all_passed=all_passed and passed
        record={'id':stem,'start':start,'end':end,'expected_files':files[start:end],
                'argv':command,'exit_code':code,'passed':passed,'error':failure,
                'seconds':time.monotonic()-tick,'tsv':tsv.name,'json':result_json.name}
        records.append(record)
        print(f'{stem}: start={start} end={end} exit={code} passed={passed}',flush=True)
    receipt={'schema':'femacs-l5-sweep-v1','passed':all_passed,'files':files,
             'requested_files':files[args.start_at:],'chunks':records,'gc_stress':args.gc_stress,
             'seconds':time.monotonic()-started}
    (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(f'L5 sweep complete; passed={all_passed}; evidence={out}')
    return 0 if all_passed else 1


if __name__=='__main__':raise SystemExit(main())
