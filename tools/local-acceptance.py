#!/usr/bin/env python3
"""Local artifact acceptance; no build, network or remote CI is invoked.

Build both products first using the documented controlled commands. This report binds
checks to exact files and a source snapshot; it explicitly does not infer build provenance
from an existing binary's timestamp. Requested GUI/IME layers fail if dependencies are absent.
"""
from __future__ import annotations
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import platform
import pty
import select
import subprocess
import struct
import sys
import termios
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


def user_init_workflow(binary: Path, work: Path, gui=False) -> dict:
    """Actual interactive startup: early-init -> init -> custom -> startup hook.

    Does not use -Q, --batch, -l, or manually load the init from a batch test.
    Owned fixture files and the complete PTY log are retained on failure.
    """
    work=Path(work).resolve();work.mkdir(parents=True,exist_ok=False)
    home=work/'home';config=home/'.emacs.d';config.mkdir(parents=True)
    trace=work/'trace.txt';receipt=work/'startup.json'
    quoted=lambda path:json.dumps(str(path),ensure_ascii=False)
    append=lambda label:f'(write-region "{label}\\n" nil {quoted(trace)} t \'silent)\n'
    (config/'early-init.el').write_text(append('early-init'))
    (config/'custom.el').write_text('(setq femacs-startup-custom-value 731)\n'+append('custom'))
    (config/'init.el').write_text(
        append('init')+'(setq inhibit-startup-screen t initial-scratch-message nil)\n'
        f'(setq custom-file {quoted(config/"custom.el")})\n(load custom-file nil nil)\n'
        '(require \'json)\n'
        '(add-hook \'emacs-startup-hook\n (lambda ()\n'+append('startup-hook')+
        f'(with-temp-file {quoted(receipt)}\n'
        ' (insert (json-encode `((custom . ,femacs-startup-custom-value)\n'
        ' (native . ,(if window-system (symbol-name window-system) "terminal"))\n'
        ' (pid . ,(emacs-pid))))))\n'
        '(run-with-timer 0.3 nil (lambda () (kill-emacs 0)))))\n')
    env=dict(os.environ,HOME=str(home),TERM='xterm-256color',LC_ALL='en_US.UTF-8')
    env.pop('EMACSLOADPATH',None);env.pop('EMACSDATA',None);env.pop('EMACSDOC',None)
    argv=[str(Path(binary).resolve()),'--no-site-file','--no-site-lisp',
          '--init-directory',str(config)]
    if not gui:argv.append('-nw')
    master,slave=pty.openpty();process=None;started=time.monotonic()
    fcntl.ioctl(slave,termios.TIOCSWINSZ,struct.pack('HHHH',24,80,0,0))
    def attach_terminal():
        os.setsid()
        fcntl.ioctl(0,termios.TIOCSCTTY,0)
    result={'id':'gui-user-init' if gui else 'terminal-user-init','passed':False,'argv':argv}
    try:
        process=subprocess.Popen(argv,stdin=slave,stdout=slave,stderr=slave,cwd=work,env=env,
                                 preexec_fn=attach_terminal)
        os.close(slave);slave=None
        with (work/'output.log').open('xb') as output:
            while process.poll() is None:
                if time.monotonic()-started>20:raise TimeoutError('interactive init startup did not finish')
                if select.select([master],[],[],.05)[0]:
                    try:chunk=os.read(master,65536)
                    except OSError:break
                    if chunk:output.write(chunk)
            process.wait(timeout=2)
        state=json.loads(receipt.read_text())
        actual=trace.read_text().splitlines()
        result.update(state=state,trace=actual,exit_code=process.returncode)
        result['passed']=(process.returncode==0 and state['pid']==process.pid and state['custom']==731
            and state['native']==('ns' if gui else 'terminal')
            and actual==['early-init','init','custom','startup-hook'])
    except (OSError,ValueError,KeyError,TimeoutError,subprocess.SubprocessError) as error:result['error']=str(error)
    finally:
        if slave is not None:os.close(slave)
        os.close(master)
        if process is not None and process.poll() is None:
            process.terminate()
            try:process.wait(timeout=2)
            except subprocess.TimeoutExpired:process.kill();process.wait(timeout=2)
        if process is not None:result.update(exit_code=process.returncode,pid=process.pid)
    result['seconds']=time.monotonic()-started
    (work/'results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
    return result


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
        rows.append(user_init_workflow(terminal,out/'terminal-user-init'))
        if a.gui:
            rows.append(user_init_workflow(gui,out/'gui-user-init',True))
            check('gui-background-gc',[sys.executable,'tools/test-macos-background-gc.py',
                  '--binary',gui,'--out',out/'gui-background-gc'])
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
