"""Layered real-input acceptance; run only inside controlled-run runtime protection.

Each terminal case starts an isolated real editor PTY, records command/point/text and
optionally compares the same input with an explicitly supplied reference. GUI cases require the actual app
and the native event sender (Accessibility permission). A missing GUI is a failure,
never a skip disguised as success. No production key rebinding or weakened tests.
"""
from __future__ import annotations
import argparse, json, os, pty, re, select, signal, subprocess, time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / 'tools/fixtures/macos-keys.el'
SEED = 'alpha beta\ngamma delta\n'
CASES = [
    ('C-a', b'\x01', 7, 1, SEED), ('C-e', b'\x05', 7, 11, SEED),
    ('C-b', b'\x02', 7, 6, SEED), ('C-f', b'\x06', 7, 8, SEED),
    ('C-p', b'\x10', 18, 7, SEED), ('C-n', b'\x0e', 7, 18, SEED),
    ('left', b'\x1b[D', 7, 6, SEED), ('right', b'\x1b[C', 7, 8, SEED),
    ('up', b'\x1b[A', 18, 7, SEED), ('down', b'\x1b[B', 7, 18, SEED),
    ('home', b'\x1bOH', 7, 1, SEED), ('end', b'\x1bOF', 7, 11, SEED),
    ('M-b', b'\x1bb', 7, 1, SEED), ('M-f', b'\x1bf', 7, 11, SEED),
    ('M-<', b'\x1b<', 7, 1, SEED), ('M->', b'\x1b>', 7, 24, SEED),
    ('C-M-b', b'\x1b\x02', 7, 1, SEED), ('C-M-f', b'\x1b\x06', 7, 11, SEED),
    ('M-x', b'\x1bxbeginning-of-line\r', 7, 1, SEED),
    ('C-k', b'\x0b', 7, 7, 'alpha \ngamma delta\n'),
    ('C-k-C-y', b'\x0b\x19', 7, 11, SEED),
    ('C-g-recover', b'\x18\x07\x01', 7, 1, SEED),
    ('C-s', b'\x13delta\r', 7, 23, SEED),
    ('mark-copy-yank', b'\x00\x06\x1bw\x19', 7, 9, 'alpha bbeta\ngamma delta\n'),
    ('mark-cut', b'\x00\x06\x17', 7, 7, 'alpha eta\ngamma delta\n'),
    ('undo', b'Z\x1f', 7, 7, SEED),
]

def terminal_cursor(data):
    """Track actual VT cursor operations; SGR/erase/mode changes do not move it."""
    row=col=1;saved=(1,1);i=0
    while i<len(data):
        c=data[i]
        if c==27 and i+1<len(data):
            if data[i+1]==ord('['):
                m=re.match(rb'\x1b\[([0-9;? >!]*)([@-~])',data[i:])
                if not m: i+=1;continue
                raw=m[1];op=chr(m[2][0]);i+=len(m[0])
                if raw.startswith((b'?',b'>',b'!')):continue
                a=[int(v) if v else 0 for v in raw.strip().split(b';')];n=a[0] or 1
                if op in ('H','f'):row=n;col=(a[1] or 1) if len(a)>1 else 1
                elif op in ('G','`'):col=n
                elif op=='d':row=n
                elif op=='A':row=max(1,row-n)
                elif op=='B':row=min(24,row+n)
                elif op=='C':col=min(80,col+n)
                elif op=='D':col=max(1,col-n)
                elif op=='E':row=min(24,row+n);col=1
                elif op=='F':row=max(1,row-n);col=1
                elif op=='s':saved=(row,col)
                elif op=='u':row,col=saved
                continue
            if data[i+1] in (ord(']'),ord('P')):
                # OSC/DCS terminate at BEL or ST, never paint their contents.
                j=i+2
                while j<len(data) and data[j]!=7 and data[j:j+2]!=b'\x1b\\':j+=1
                i=j+(1 if j<len(data) and data[j]==7 else 2);continue
            op=chr(data[i+1]);i+=2
            if op=='7':saved=(row,col)
            elif op=='8':row,col=saved
            elif op=='D':row=min(24,row+1)
            elif op=='M':row=max(1,row-1)
            elif op in '()*+':i+=1
            continue
        if c==13:col=1
        elif c in (10,11,12):row=min(24,row+1)
        elif c==8:col=max(1,col-1)
        elif c==9:col=min(80,((col-1)//8+1)*8+1)
        elif c>=32 and c<127:col=min(80,col+1)
        i+=1
    return [row,col]


def snapshot(path):
    try:
        parts = path.read_text().rstrip('\n').split('\t')
        return {'command': parts[0], 'point': int(parts[1]), 'text': json.loads(parts[2]), 'keys': json.loads(parts[3])}
    except (OSError, ValueError, IndexError):
        return None

def native_strokes(sequence):
    """Physical Cocoa keys for the same logical inputs as the PTY fixtures."""
    keys=dict(zip('abcdefghijklmnopqrstuvwxyz', [0,11,8,2,14,3,5,4,34,38,40,37,46,45,31,35,12,15,1,17,32,9,13,7,16,6]))
    keys.update({' ':49,'/':44,'-':27,'.':47,',':43,';':41,':':41,'(':25,')':29,'"':39,'_':27,'<':43,'>':47})
    special={b'\x1b[D':123,b'\x1b[C':124,b'\x1b[A':126,b'\x1b[B':125,b'\x1bOH':115,b'\x1bOF':119}
    rows=[];i=0
    while i<len(sequence):
        found=next((s for s in special if sequence.startswith(s,i)),None)
        if found:rows.append((special[found],0xA00000,None));i+=len(found);continue
        flags=0;c=sequence[i];i+=1
        if c==27:
            flags|=1<<19;c=sequence[i];i+=1
        if c in (10,13):rows.append((36,flags,None));continue
        if c==31:rows.append((44,flags|(1<<18),None));continue
        if c<32:
            key=' ' if c==0 else chr(c+96);rows.append((keys[key],flags|(1<<18),None));continue
        char=chr(c)
        if char.isupper() or char in '<>:()"_':flags|=1<<17
        rows.append((keys.get(char.lower(),0),flags,char))
    return rows

def gui_case(binary,sender,name,sequence,initial,expected_point,expected_text,work):
    work.mkdir(parents=True,exist_ok=True);report=work/'snapshot.tsv';metadata=work/'native-frame.txt'
    if name=='end':expected_point=len(SEED)+1 # Native Home/End navigate the buffer.
    env=dict(os.environ,FEMACS_KEY_REPORT=str(report),FEMACS_KEY_POINT=str(initial),FEMACS_KEY_GUI=str(metadata),FEMACS_KEY_DIAGNOSTIC=str(work/'diagnostic.el'))
    pending=work/'pending.json'
    if name=='C-g-recover':env['FEMACS_KEY_PENDING_REPORT']=str(pending)
    started=time.monotonic();process=None;result={'id':name,'passed':False}
    with (work/'output.log').open('wb') as output:
        try:
            process=subprocess.Popen([str(binary),'-Q','-l',str(FIXTURE)],env=env,stdout=output,stderr=subprocess.STDOUT)
            deadline=time.monotonic()+20
            while time.monotonic()<deadline and process.poll() is None and (snapshot(report) is None or not metadata.exists() or not metadata.read_text().strip()):time.sleep(.05)
            if snapshot(report) is None:raise RuntimeError(f'GUI did not reach fixture, exit={process.poll()}')
            if not metadata.exists() or metadata.read_text().strip()!='ns':raise RuntimeError('fixture has no native Cocoa frame')
            def send(data):
                path=work/'strokes.tsv'
                path.write_text(''.join(f'{code}\t{flags}'+('' if text is None else '\t'+text)+'\n' for code,flags,text in native_strokes(data)))
                response=subprocess.run([str(sender),str(process.pid),str(path)],stdout=output,stderr=subprocess.STDOUT,timeout=8)
                if response.returncode:raise RuntimeError(f'native sender exit={response.returncode}')
            if name=='C-g-recover':
                def pending_keys():
                    try:return json.loads(pending.read_text())
                    except (OSError,ValueError):return None
                def wait_pending(label,predicate):
                    end=time.monotonic()+3
                    while time.monotonic()<end and process.poll() is None:
                        keys=pending_keys()
                        if keys is not None and predicate(keys):
                            result.setdefault('prefix_receipts',[]).append({'id':label,'keys':keys});return
                        time.sleep(.02)
                    raise RuntimeError(label+' native prefix receipt missing')
                send(sequence[:1]);wait_pending('prefix-read',lambda keys:keys=='C-x')
                send(sequence[1:2]);wait_pending('prefix-cancelled',lambda keys:keys!='C-x')
                send(sequence[2:])
            else:send(sequence)
            deadline=time.monotonic()+4
            while time.monotonic()<deadline and process.poll() is None:
                actual=snapshot(report)
                if actual and actual['point']==expected_point and actual['text']==expected_text:break
                time.sleep(.05)
            actual=snapshot(report);result.update(actual=actual,pid=process.pid,native_frame='ns')
            result['passed']=bool(actual and actual['point']==expected_point and actual['text']==expected_text and process.poll() is None)
            # Exercise the normal command loop exit, then bounded owned-process cleanup.
            if process.poll() is None:send(b'\x1b:(kill-emacs 0)\r')
        except (OSError,RuntimeError,subprocess.TimeoutExpired) as error:result['reason']=str(error)
        finally:
            if process is not None and process.poll() is None:
                try:process.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    process.terminate()
                    try:process.wait(timeout=2)
                    except subprocess.TimeoutExpired:process.kill();process.wait(timeout=2)
            result['elapsed_seconds']=round(time.monotonic()-started,3)
            result['exit_code']=None if process is None else process.returncode
            result['passed']=result['passed'] and result['exit_code']==0
    return result



def clipboard_environment():
    # The system clipboard CLI decodes stdin/stdout using locale, unlike the native
    # pasteboard API. Keep this boundary UTF-8 even under a deterministic C test locale.
    return dict(os.environ, LC_ALL='en_US.UTF-8', LANG='en_US.UTF-8')

def clipboard_read():
    return subprocess.run(['pbpaste'],capture_output=True,check=True,env=clipboard_environment()).stdout

def clipboard_write(data):
    subprocess.run(['pbcopy'],input=data,check=True,env=clipboard_environment())

def gui_file_workflow(binary, sender, work):
    """Native Cocoa file roundtrip, Unicode, clipboard and post-GC editing."""
    work.mkdir(parents=True,exist_ok=True)
    report=work/'snapshot.tsv';metadata=work/'native-frame.txt';document=work/'edited.txt'
    content='Femacs keyboard workflow\n中文 café e\u0301 😀\n'
    env=dict(os.environ,FEMACS_KEY_REPORT=str(report),FEMACS_KEY_GUI=str(metadata),
             FEMACS_KEY_ALL_BUFFERS='1',FEMACS_KEY_DIAGNOSTIC=str(work/'diagnostic.el'))
    started=time.monotonic();steps=[];process=None
    result={'id':'native-file-workflow','passed':False}
    previous_clipboard=clipboard_read()
    with (work/'output.log').open('wb') as output:
      try:
        process=subprocess.Popen([str(binary),'-Q','-l',str(FIXTURE)],env=env,stdout=output,stderr=subprocess.STDOUT)
        def wait_for(label,predicate):
            deadline=time.monotonic()+10;t=time.monotonic()
            while time.monotonic()<deadline and process.poll() is None:
                if predicate():
                    time.sleep(.15);steps.append({'id':label,'seconds':round(time.monotonic()-t,6)});return
                time.sleep(.03)
            raise RuntimeError(label+' failed; '+str(snapshot(report))+f'; exit={process.poll()}')
        def send(data=None,text=None):
            if data is not None:rows=native_strokes(data)
            else:
                rows=[]
                for char in text:
                    rows.extend(native_strokes(char.encode()) if char.isascii() else [(0,0,char)])
            path=work/'strokes.tsv'
            path.write_text(''.join(f'{code}\t{flags}'+('' if value is None else '\t'+value)+'\n' for code,flags,value in rows))
            done=subprocess.run([str(sender),str(process.pid),str(path)],stdout=output,stderr=subprocess.STDOUT,timeout=15)
            if done.returncode:raise RuntimeError(f'native sender exit={done.returncode}')
        wait_for('native-startup',lambda:snapshot(report) is not None and metadata.exists() and metadata.read_text().strip()=='ns')
        send(b'\x18\x06'+str(document).encode()+b'\r')
        wait_for('open-new-file',lambda:(snapshot(report) or {}).get('command')=='find-file')
        send(text=content)
        wait_for('unicode-and-combining-insert',lambda:(snapshot(report) or {}).get('text')==content)
        send(b'\x18\x13')
        wait_for('save',lambda:document.exists() and document.read_text()==content)
        send(b'\x1b:(garbage-collect)\r')
        wait_for('gc-return',lambda:(snapshot(report) or {}).get('command')=='eval-expression')
        send(b'\x01')
        wait_for('editing-after-gc',lambda:(snapshot(report) or {}).get('command')=='move-beginning-of-line')
        send(b'\x18k\r')
        wait_for('close-buffer',lambda:(snapshot(report) or {}).get('command')=='kill-buffer')
        send(b'\x18\x06'+str(document).encode()+b'\r')
        wait_for('reopen',lambda:(snapshot(report) or {}).get('command')=='find-file' and (snapshot(report) or {}).get('text')==content)
        # Copy through the editor's actual selection backend and inspect the OS clipboard.
        send(b'\x1b<\x00\x06\x1bw')
        wait_for('copy-to-system-clipboard',lambda:clipboard_read()==b'F')
        clipboard_write('clipboard 中文'.encode())
        send(b'\x1b>\x19')
        pasted=content+'clipboard 中文'
        wait_for('paste-from-system-clipboard',lambda:(snapshot(report) or {}).get('text')==pasted)
        send(b'\x18\x13')
        # Text mode's require-final-newline adds one during the actual save.
        saved=pasted+'\n'
        wait_for('save-pasted-content',lambda:document.read_text()==saved)
        send(b'\x18\x03')
        process.wait(timeout=8)
        result.update(passed=process.returncode==0 and document.read_text()==saved,exit_code=process.returncode,native_frame='ns')
      except (OSError,RuntimeError,subprocess.SubprocessError) as error:result['reason']=str(error)
      finally:
        clipboard_write(previous_clipboard)
        if process is not None and process.poll() is None:
            process.terminate()
            try:process.wait(timeout=2)
            except subprocess.TimeoutExpired:process.kill();process.wait(timeout=2)
        result.update(steps=steps,elapsed_seconds=round(time.monotonic()-started,6))
    return result


def terminal_case(binary, name, sequence, initial, expected_point, expected_text, work):
    work.mkdir(parents=True, exist_ok=True)
    report = work / 'snapshot.tsv'
    env = dict(os.environ, TERM='xterm-256color', LC_ALL='en_US.UTF-8',
               FEMACS_KEY_REPORT=str(report), FEMACS_KEY_POINT=str(initial))
    started = time.monotonic()
    pid, master = pty.fork()
    if pid == 0:
        os.execve(str(binary), [str(binary), '-nw', '-Q', '-l', str(FIXTURE)], env)
    os.set_blocking(master, False)
    screen = bytearray()
    def pump(timeout):
        if select.select([master], [], [], timeout)[0]:
            try: chunk = os.read(master, 16384)
            except OSError: return False
            if not chunk: return False
            screen.extend(chunk)
            if len(screen) > 131072: del screen[:-65536]
            try:
                if b'\x1b[6n' in chunk: os.write(master, b'\x1b[1;1R')
                if b'\x1b[>c' in chunk or b'\x1b[>0c' in chunk: os.write(master, b'\x1b[>0;95;0c')
                if b'\x1b[c' in chunk: os.write(master, b'\x1b[?1;2c')
            except BlockingIOError: pass
        return True
    result = {'id': name, 'binary': str(binary.relative_to(ROOT)) if binary.is_relative_to(ROOT) else binary.name}
    try:
        deadline = time.monotonic() + 12
        while snapshot(report) is None and time.monotonic() < deadline:
            if not pump(.05): break
        if snapshot(report) is None: raise RuntimeError('editor did not reach input fixture')
        before = report.stat().st_mtime_ns
        if name=='C-g-recover':
            os.write(master,sequence[:-1])
            until=time.monotonic()+.35
            while time.monotonic()<until:pump(.03)
            os.write(master,sequence[-1:])
        else:os.write(master,sequence)
        deadline = time.monotonic() + 5
        last_change = time.monotonic()
        previous = before
        while time.monotonic() < deadline:
            if not pump(.05): break
            stamp = report.stat().st_mtime_ns
            if stamp != previous: previous, last_change = stamp, time.monotonic()
            if stamp != before and time.monotonic() - last_change > .25: break
        actual = snapshot(report)
        result.update(actual=actual, expected_point=expected_point, expected_text=expected_text,
                      passed=bool(actual and actual['point'] == expected_point and actual['text'] == expected_text))
        (work/'screen.ansi').write_bytes(screen)
        prefix=expected_text[:expected_point-1]
        expected_cursor=[2+prefix.count('\n'),len(prefix.rsplit('\n',1)[-1])+1]
        result['cursor']=terminal_cursor(bytes(screen));result['expected_cursor']=expected_cursor
        result['passed']=result['passed'] and result['cursor']==expected_cursor
    except Exception as error:
        result.update(passed=False, error=str(error))
    finally:
        # Give the actual command loop a normal exit first. Close the PTY before
        # forced cleanup; waiting with the master open can deadlock tty teardown.
        try: os.write(master, b'\x1b:(kill-emacs 0)\r')
        except OSError: pass
        until=time.monotonic()+2
        ended=0
        while time.monotonic()<until:
            ended,status=os.waitpid(pid,os.WNOHANG)
            if ended:
                result['exit_code']=os.waitstatus_to_exitcode(status)
                break
            pump(.03)
        os.close(master)
        if not ended:
            try: os.kill(pid,signal.SIGTERM)
            except ProcessLookupError: pass
            until=time.monotonic()+2
            while time.monotonic()<until:
                ended,_=os.waitpid(pid,os.WNOHANG)
                if ended: break
                time.sleep(.03)
        if not ended:
            try: os.kill(pid,signal.SIGKILL)
            except ProcessLookupError: pass
            result['cleanup']='forced; remaining descendants owned by controlled-run'
        result['passed']=result.get('passed',False) and result.get('exit_code')==0
    result['seconds']=round(time.monotonic()-started,6)
    return result


def terminal_file_workflow(binary, work):
    """Real C-x C-f / C-x C-s / kill-buffer / reopen / C-x C-c."""
    work.mkdir(parents=True,exist_ok=True)
    report=work/'snapshot.tsv';document=work/'edited.txt'
    content='Femacs keyboard workflow\n中文 café e\u0301 😀\n'
    env=dict(os.environ,TERM='xterm-256color',LC_ALL='en_US.UTF-8',
             FEMACS_KEY_REPORT=str(report),FEMACS_KEY_ALL_BUFFERS='1')
    started=time.monotonic();pid,master=pty.fork()
    if pid==0:os.execve(str(binary),[str(binary),'-nw','-Q','-l',str(FIXTURE)],env)
    os.set_blocking(master,False);screen=bytearray();ended=False;steps=[]
    def pump():
        if select.select([master],[],[],.03)[0]:
            try:chunk=os.read(master,16384)
            except OSError:return
            screen.extend(chunk)
            for query,answer in [(b'\x1b[6n',b'\x1b[1;1R'),(b'\x1b[>c',b'\x1b[>0;95;0c'),(b'\x1b[c',b'\x1b[?1;2c')]:
                if query in chunk:os.write(master,answer)
    def wait_for(label,predicate):
        deadline=time.monotonic()+8;t=time.monotonic()
        while time.monotonic()<deadline:
            pump()
            if predicate():
                # Let redisplay and the post-command hook finish before the next input.
                until=time.monotonic()+.15
                while time.monotonic()<until:pump()
                steps.append({'id':label,'seconds':round(time.monotonic()-t,6)});return
        raise RuntimeError(label+' timed out; '+str(snapshot(report)))
    result={'id':'file-workflow','binary':str(binary.relative_to(ROOT))}
    try:
        wait_for('startup',lambda:snapshot(report) is not None)
        os.write(master,b'\x18\x06'+str(document).encode()+b'\r')
        wait_for('open-new-file',lambda:(snapshot(report) or {}).get('command')=='find-file')
        os.write(master,content.encode());wait_for('insert-unicode',lambda:(snapshot(report) or {}).get('text')==content)
        os.write(master,b'\x18\x13')
        wait_for('save',lambda:document.exists() and document.read_text()==content)
        os.write(master,b'\x18k\r')
        wait_for('close-buffer',lambda:(snapshot(report) or {}).get('command')=='kill-buffer')
        os.write(master,b'\x18\x06'+str(document).encode()+b'\r')
        wait_for('reopen',lambda:(snapshot(report) or {}).get('command')=='find-file' and (snapshot(report) or {}).get('text')==content)
        os.write(master,b'\x18\x03');deadline=time.monotonic()+8
        while time.monotonic()<deadline:
            pump();got,status=os.waitpid(pid,os.WNOHANG)
            if got:
                ended=True;result['exit_code']=os.waitstatus_to_exitcode(status);break
        result.update(passed=ended and result.get('exit_code')==0 and document.read_text()==content,steps=steps)
    except Exception as error:result.update(passed=False,error=str(error),steps=steps)
    finally:
        os.close(master)
        if not ended:
            try:os.kill(pid,signal.SIGTERM)
            except ProcessLookupError:pass
            deadline=time.monotonic()+2
            while time.monotonic()<deadline:
                got,_=os.waitpid(pid,os.WNOHANG)
                if got:ended=True;break
                time.sleep(.03)
            if not ended:
                try:os.kill(pid,signal.SIGKILL)
                except ProcessLookupError:pass
        (work/'screen.ansi').write_bytes(screen)
    result['seconds']=round(time.monotonic()-started,6)
    return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary',type=Path,default=ROOT/'build/terminal/port/src/emacs')
    parser.add_argument('--reference',type=Path,help='optional differential comparator; omitted for standalone acceptance')
    parser.add_argument('--out',type=Path,default=ROOT/'build/macos-acceptance')
    parser.add_argument('--case',action='append')
    parser.add_argument('--file-workflow',action='store_true')
    parser.add_argument('--gui',action='store_true')
    parser.add_argument('--event-sender',type=Path)
    args=parser.parse_args();args.out=args.out.resolve();args.out.mkdir(parents=True,exist_ok=True)
    cases=[] if args.file_workflow else [c for c in CASES if not args.case or c[0] in args.case]
    if not cases and not args.file_workflow: parser.error('no cases selected')
    rows=[]
    if args.gui:
        if not args.event_sender:parser.error('--gui requires --event-sender')
        if args.file_workflow:
            row=gui_file_workflow(args.binary.resolve(),args.event_sender.resolve(),args.out/'file-workflow')
            rows.append(row);print(row,flush=True)
        for case in cases:
            row=gui_case(args.binary.resolve(),args.event_sender.resolve(),*case,args.out/case[0])
            rows.append(row);print(row,flush=True)
        (args.out/'results.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
        return 0 if all(row['passed'] for row in rows) else 1
    if args.file_workflow:
        for label,binary in ([('reference',args.reference.resolve())] if args.reference else [])+[('port',args.binary.resolve())]:
            row=terminal_file_workflow(binary,args.out/'file-workflow'/label)
            rows.append(dict(role=label,**row));print(label,row,flush=True)
    for case in cases:
        pair={}
        for label,binary in ([('reference',args.reference.resolve())] if args.reference else [])+[('port',args.binary.resolve())]:
            row=terminal_case(binary,*case,args.out/case[0]/label);rows.append(dict(role=label,**row));pair[label]=row
            print(label,case[0],'PASS' if row['passed'] else 'FAIL',row.get('actual'),flush=True)
        if args.reference and pair['port'].get('actual')!=pair['reference'].get('actual'):
            rows.append({'id':case[0]+'-differential','passed':False,'reason':'command/point/text/keys differ'})
    (args.out/'results.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
    return 0 if all(row['passed'] for row in rows) else 1

if __name__=='__main__':raise SystemExit(main())
