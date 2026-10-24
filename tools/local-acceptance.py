#!/usr/bin/env python3
"""Local artifact acceptance; no build, network or remote CI is invoked.

Build both products first using the documented controlled commands. This report binds
checks to exact files and a source snapshot; it explicitly does not infer build provenance
from an existing binary's timestamp. Requested GUI/IME layers fail if dependencies are absent.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import time
ROOT=Path(__file__).resolve().parent.parent


def fingerprints(paths: dict[str,Path]) -> dict:
    result={}
    for name,path in paths.items():
        path=Path(path).resolve()
        if not path.is_file():raise FileNotFoundError(f'missing requested product/input: {path}')
        digest=hashlib.sha256()
        with path.open('rb') as source:
            for chunk in iter(lambda:source.read(1024*1024),b''):digest.update(chunk)
        result[name]={'path':str(path),'sha256':digest.hexdigest(),'bytes':path.stat().st_size}
    return result


def run_check(name,argv,out,env=None):
    if not name or any(c not in 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_' for c in name):
        raise ValueError('check name must be a simple evidence filename')
    out=Path(out);out.mkdir(parents=True,exist_ok=True)
    log=out/(name+'.log')
    if log.exists():raise FileExistsError(f'check evidence already exists: {log}')
    started=time.monotonic()
    with log.open('xb') as output:
        try:
            p=subprocess.run([str(x) for x in argv],cwd=ROOT,env=env,stdout=output,stderr=subprocess.STDOUT)
            code=p.returncode
        except OSError as e:
            output.write(str(e).encode());code=127
    return {'id':name,'passed':code==0,'exit_code':code,'seconds':time.monotonic()-started,'log':str(log),'argv':[str(x) for x in argv]}


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--terminal',type=Path,default=ROOT/'build/terminal/port/src/emacs')
    ap.add_argument('--app',type=Path,default=ROOT/'dist/macos/Femacs.app')
    ap.add_argument('--out',type=Path,required=True)
    ap.add_argument('--gui',action='store_true');ap.add_argument('--ime',action='store_true')
    ap.add_argument('--sender',type=Path);ap.add_argument('--sources',type=Path)
    a=ap.parse_args();out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
    rows=[];started=time.monotonic()
    report={'passed':False,'artifact_origin':'existing/reused; consult build logs for source-to-binary provenance',
            'host':platform.platform(),'checks':rows,'requested_gui':a.gui,'requested_ime':a.ime}
    try:
        terminal=a.terminal.resolve();app=a.app.resolve();gui=app/'Contents/MacOS/Femacs'
        paths={'terminal':terminal,'terminal_dump':terminal.with_suffix('.pdmp'),'gui':gui,
               'gui_dump':app/'Contents/Resources/libexec/Femacs.pdmp','toolchain':ROOT/'rust-toolchain.toml',
               'terminal_config':ROOT/'build-support/macos/terminal/config.json','cocoa_config':ROOT/'build-support/macos/cocoa/config.json'}
        if a.gui or a.ime:
            if not a.sender:raise ValueError('--sender required for requested native input layer')
            paths['sender']=a.sender
        if a.ime:
            if not a.sources:raise ValueError('--sources required for requested IME layer')
            paths['sources']=a.sources
        report['products']=fingerprints(paths)
        report['source_head']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
        patch=subprocess.check_output(['git','diff','--no-ext-diff','--binary','HEAD'],cwd=ROOT)
        report['tracked_patch_sha256']=hashlib.sha256(patch).hexdigest()
        report['worktree_status']=subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True)
        env=dict(os.environ,HOME=str(out/'home'),LC_ALL='C');(out/'home').mkdir()
        def check(name,argv):rows.append(run_check(name,argv,out,env))
        check('root-inputs',[sys.executable,'tools/test-repository-bootstrap.py'])
        check('ert-components',[sys.executable,'tools/test-ert-harness.py','--json-out',out/'ert-components.json'])
        check('acceptance-components',[sys.executable,'tools/test-macos-stability.py'])
        check('bundle-signature',['codesign','--verify','--deep','--strict',app])
        edit='(progn (with-temp-buffer (insert "你好 abc") (goto-char (point-min)) (forward-char 2) (unless (= (point) 3) (error "Unicode movement failed"))) (princ "owned batch ok"))'
        check('terminal-batch',[terminal,'-Q','--batch','--eval',edit])
        check('gui-batch',[gui,'-Q','--batch','--eval',edit])
        for name,binary,opts in [('terminal',terminal,[])]+([('gui',gui,['--gui','--event-sender',a.sender.resolve()])] if a.gui else []):
            check(name+'-keys',[sys.executable,'tools/test-macos-delivery.py','--binary',binary,*opts,'--out',out/(name+'-keys')])
            check(name+'-files',[sys.executable,'tools/test-macos-delivery.py','--binary',binary,*opts,'--file-workflow','--out',out/(name+'-files')])
        if a.ime:
            check('ime',[sys.executable,'tools/test-macos-ime.py','--binary',gui,'--sender',a.sender.resolve(),'--sources',a.sources.resolve(),'--mode','all','--out',out/'ime'])
        report['passed']=bool(rows) and all(r['passed'] for r in rows)
        report['unrequested_layers']=[x for x,requested in [('native keyboard/files',a.gui),('real IME',a.ime)] if not requested]
        report['excluded']='full ERT differential parity, real drag, other platforms and performance require separate recorded acceptance'
    except (OSError,ValueError,subprocess.SubprocessError) as e:report['error']=str(e)
    report['seconds']=time.monotonic()-started
    (out/'results.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'passed':report['passed'],'seconds':report['seconds'],'report':str(out/'results.json'),'error':report.get('error')}))
    return 0 if report['passed'] else 1
if __name__=='__main__':sys.exit(main())
