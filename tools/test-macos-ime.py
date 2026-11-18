"""Real IME acceptance: separate stable composition and switched-source pressure.

Every scenario starts a fresh editor and retains its callback/source/key timeline.
The default trace is opt-in for this acceptance process only. Use controlled-run runtime.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import signal
import sys
import time
import unicodedata
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('delivery',ROOT/'tools/test-macos-delivery.py')
delivery=importlib.util.module_from_spec(spec);spec.loader.exec_module(delivery)

def new_output(path):
    path=Path(path).resolve();path.mkdir(parents=True,exist_ok=False);return path

def commit_matches(state, expected):
    return not state.get('marked',True) and unicodedata.normalize('NFC',state.get('text','')) == unicodedata.normalize('NFC',expected)

def read_callbacks(path):
    """Read bounded complete JSONL rows, leaving a final in-flight append alone."""
    try:
        with Path(path).open('rb') as stream:data=stream.read(1_048_577)
    except FileNotFoundError:return []
    if len(data)>1_048_576:raise ValueError('native callback trace exceeds 1 MiB')
    rows=[]
    for line in data.splitlines(keepends=True):
        if not line.endswith(b'\n'):break
        row=json.loads(line)
        if not isinstance(row,dict):raise ValueError('native callback row must be an object')
        rows.append(row)
    return rows

def native_window_ready(records,title):
    """Require the target view and latest global native focus to agree."""
    if not isinstance(title,str) or not title or not records or any(not isinstance(row,dict) for row in records):return False
    target=next((row for row in reversed(records) if row.get('windowTitle')==title),None)
    if target is None:return False
    number=target.get('windowNumber');last=records[-1]
    if type(number) is not int or number<=0:return False
    if any(target.get(key) is not True for key in ['windowIsKey','viewIsFirstResponder','keyWindow','appActive']):return False
    for row in [target,last]:
        key_number=row.get('keyWindowNumber')
        if type(key_number) is not int or key_number!=number:return False
        if row.get('keyWindow') is not True or row.get('appActive') is not True:return False
    return True

def run_scenario(binary,sender,sources,work,mode,expected,pinyin_id=None):
    work=new_output(work)
    rows=[];process=None;started=time.monotonic();original=None
    result={'mode':mode,'passed':False,'steps':rows,'expected':expected}
    def source(*args):return subprocess.check_output([str(sources),*map(str,args)],text=True,timeout=8).strip()
    report=work/'snapshot.json';document=work/'ime.txt';trace=work/'callbacks.jsonl'
    expected_source=None;target_title='Femacs IME A'
    env=dict(os.environ,FEMACS_IME_REPORT=str(report),FEMACS_INPUT_TRACE=str(trace))
    if mode=='multi':env['FEMACS_IME_MULTIFRAME']='1'
    with (work/'output.log').open('wb') as output:
      try:
        original=source('current');available=source('list').splitlines()
        result.update(original_source=original,available_sources=available)
        ascii_id=next((s for s in available if s in ['com.apple.keylayout.ABC','com.apple.keylayout.US']),None)
        pinyin=pinyin_id or next((s for s in available if 'SCIM' in s and ('ITABC' in s or 'Pinyin' in s)),None)
        if pinyin is None:pinyin=next((s for s in available if s.lower().endswith('.pinyin')),None)
        if not ascii_id or pinyin not in available:raise RuntimeError('requested enabled Pinyin and ABC sources are required')
        # A shell exec has no activation context under Apple's cooperative
        # activation, so a bundled editor is opened through LaunchServices with
        # explicit arguments and environment. The helper requests a new instance
        # and prints its pid; the pid's executable is verified against the exact
        # bundle before any AX work, so a running installed editor cannot be reused.
        app=None
        for candidate in binary.parents:
            if candidate.suffix=='.app':app=candidate;break
        launch_argv=['-Q','-name','Femacs IME A','-l',str(ROOT/'tools/fixtures/macos-ime.el')]
        if app is None:
            process=subprocess.Popen([str(binary),*launch_argv],env=env,stdout=output,stderr=subprocess.STDOUT)
        else:
            helper=work/'ime-launch'
            if not helper.exists():
                built=subprocess.run(['swiftc','-O',str(ROOT/'tools/fixtures/ime-launch.swift'),'-o',str(helper)],capture_output=True,text=True)
                if built.returncode!=0:raise RuntimeError('could not build the LaunchServices helper: '+built.stderr[:300])
            spec=work/'launch.json'
            spec.write_text(json.dumps({'arguments':launch_argv,'environment':env}))
            launched=subprocess.run([str(helper),str(app),str(spec)],capture_output=True,text=True,timeout=40)
            if launched.returncode!=0:raise RuntimeError('LaunchServices could not open the bundle: '+launched.stderr[:300])
            pid=int(launched.stdout.strip().splitlines()[-1])
            image=subprocess.run(['python3','-c','import ctypes,ctypes.util,sys;lib=ctypes.CDLL(ctypes.util.find_library("proc"));buf=ctypes.create_string_buffer(4096);n=lib.proc_pidpath(int(sys.argv[1]),buf,4096);print(buf.value.decode() if n>0 else "")',str(pid)],capture_output=True,text=True).stdout.strip()
            if not image or os.path.realpath(image)!=os.path.realpath(str(binary)):
                raise RuntimeError('launched pid image differs from the requested bundle: %r' % image)
            class Launched:
                """Handle for a LaunchServices instance that is not our child."""
                def __init__(self,pid):
                    self.pid=pid;self.returncode=None
                def poll(self):
                    try:os.kill(self.pid,0)
                    except ProcessLookupError:return 0
                    return None
                def terminate(self):
                    try:os.kill(self.pid,signal.SIGTERM)
                    except ProcessLookupError:pass
                def wait(self,timeout=None):
                    started=time.monotonic()
                    while self.poll() is None:
                        if timeout is not None and time.monotonic()-started>timeout:
                            raise subprocess.TimeoutExpired('launched',timeout)
                        time.sleep(.05)
                    self.returncode=0
                    return 0
            process=Launched(pid)
            launch_record={'bundle':str(app),'pid':pid,'image':os.path.realpath(image),
                           'bundle_executable':os.path.realpath(str(binary)),
                           'same_image':os.path.realpath(image)==os.path.realpath(str(binary)),
                           'method':'NSWorkspace.OpenConfiguration arguments+environment, new instance, activates'}
        def snapshot():
            try:return json.loads(report.read_text())
            except (OSError,ValueError):return {}
        def wait(label,predicate):
            t=time.monotonic();deadline=t+8
            while time.monotonic()<deadline and process.poll() is None:
                state=snapshot()
                if predicate(state):
                    rows.append({'id':label,'at':time.monotonic(),'seconds':time.monotonic()-t,'state':state});return state
                time.sleep(.04)
            raise RuntimeError(f'{label}: exit={process.poll()}, state={snapshot()}')
        def status():return json.loads(source('status',process.pid))
        def select(ident):
            nonlocal expected_source
            t=time.monotonic();source('select',ident,process.pid)
            state=status();rows.append({'id':'source-change','requested':ident,'at':time.monotonic(),'seconds':time.monotonic()-t,'observed':state})
            if process.poll() is not None:raise RuntimeError('editor exited during source change')
            if state.get('source')!=ident or not state.get('frontmost') or not state.get('focused_window'):
                raise RuntimeError('input source selection/focus not observed')
            expected_source=ident
        def strokes(values,physical=False,allow_exit=False):
            p=work/'strokes.tsv'
            p.write_text(''.join(f'{key}\t{flags}'+('' if text is None else '\t'+text)+'\n' for key,flags,text in values))
            before=status()
            if not before.get('frontmost') or not before.get('focused_window'):raise RuntimeError('editor is not focused before keystrokes')
            if before.get('source')!=expected_source:raise RuntimeError('input source changed before keystrokes')
            callbacks_before=read_callbacks(trace)
            if not native_window_ready(callbacks_before,target_title):raise RuntimeError('intended native window/view is not focused before keystrokes')
            first_new=len(callbacks_before)
            t=time.monotonic()
            subprocess.run([str(sender),str(process.pid),str(p)],stdout=output,stderr=subprocess.STDOUT,check=True,timeout=8)
            rows.append({'id':'physical-keys' if physical else 'editor-keys','at':t,'keys':[x[0] for x in values],'before':before,'after':status()})
            if physical:
                wait('physical-native-key-receipt',lambda s:len([r for r in read_callbacks(trace)[first_new:] if r.get('event')=='keyDown'])>=len(values))
                fresh=[r for r in read_callbacks(trace)[first_new:] if r.get('event')=='keyDown']
                if any(not native_window_ready([row],target_title) for row in fresh):
                    raise RuntimeError('physical key reached an unintended/inactive native window or responder')
            if not allow_exit:
                after=status()
                if (after.get('source')!=expected_source or not after.get('frontmost')
                        or not after.get('focused_window') or not native_window_ready(read_callbacks(trace),target_title)):
                    raise RuntimeError('input source/native focus lost during keystrokes')
        def send(data,allow_exit=False):strokes(delivery.native_strokes(data),allow_exit=allow_exit)
        def pinyin_keys(keys):
            # Preserve the original burst cadence. Native keyDown traces record each received
            # key; helper observations bracket the burst without refocusing between letters.
            strokes([(key,0,None) for key in keys],physical=True)
        def focus_frame(title,label):
            nonlocal target_title
            # Cycle real windows without separately mutating AXMain: main and
            # key windows are different AppKit states. This fixture owns two.
            if snapshot().get('selected')!=title:
                path=work/'cycle.tsv';path.write_text('CYCLE_WINDOW\n')
                subprocess.run([str(sender),str(process.pid),str(path)],stdout=output,
                               stderr=subprocess.STDOUT,check=True,timeout=8)
                rows.append({'id':'native-window-cycle','target':title,'at':time.monotonic()})
            wait(label,lambda s:s.get('selected')==title and native_window_ready(read_callbacks(trace),title))
            target_title=title
        wait('native-startup',lambda s:s.get('native')=='ns')
        wait('native-window-focus',lambda s:native_window_ready(read_callbacks(trace),target_title))
        if mode=='multi':
            focus_frame('Femacs IME A','frame-a-focus')
        select(ascii_id)
        if mode=='switch':
            for _ in range(3):select(pinyin);select(ascii_id)
        select(pinyin)
        pinyin_keys([45,34,4,0,31])
        wait('pinyin-marked',lambda s:s.get('marked') and bool(s.get('working')))
        pinyin_keys([49])
        committed=wait('pinyin-commit',lambda s:commit_matches(s,expected))['text']
        pinyin_keys([45,34]);wait('cancel-composition-start',lambda s:s.get('marked'))
        pinyin_keys([53]);wait('cancel-composition',lambda s:not s.get('marked') and s.get('text')==committed)
        select(ascii_id);send(b'\x1bx');wait('minibuffer-start',lambda s:s.get('minibuffer'))
        select(pinyin);pinyin_keys([4,0,31]);wait('minibuffer-marked',lambda s:s.get('minibuffer') and s.get('marked'))
        pinyin_keys([53]);send(b'\x07');select(ascii_id)
        wait('minibuffer-cancel',lambda s:not s.get('minibuffer') and not s.get('marked') and s.get('text')==committed)
        if mode=='multi':
            focus_frame('Femacs IME B','frame-b-focus')
            select(pinyin);pinyin_keys([45,34,4,0,31]);wait('frame-b-marked',lambda s:s.get('marked'))
            pinyin_keys([49]);wait('frame-b-commit',lambda s:not s.get('marked') and s.get('second_text')==expected and s.get('text')==committed)
            focus_frame('Femacs IME A','frame-a-return')
            pinyin_keys([45,34]);wait('frame-a-pending',lambda s:s.get('marked'))
            focus_frame('Femacs IME B','pending-focus-transfer')
            # Cancel any pending native composition before a new independent commit.
            # Preserve A's platform-defined focus-loss result as an observation;
            # B must not receive A's pending text or modify A on its own commit.
            pinyin_keys([53]);before=wait('pending-cleared',lambda s:not s.get('marked'))
            a_before=before['text'];b_before=before['second_text']
            if not a_before.startswith(committed) or b_before!=expected:raise RuntimeError('composition crossed owned buffers on focus change')
            pinyin_keys([45,34,4,0,31]);wait('second-frame-recompose',lambda s:s.get('marked'))
            pinyin_keys([49]);wait('second-frame-recommit',lambda s:not s.get('marked') and s.get('second_text')==expected*2 and s.get('text')==a_before)
            result['focus_loss_observation']={'a_text':a_before,'b_text':b_before,'policy':'observed OS commit/cancel; no universal focus-loss policy claimed'}
            focus_frame('Femacs IME A','frame-a-save-focus')
            select(ascii_id);committed=a_before
        send(b'\x18\x17'+str(document).encode()+b'\r')
        wait('save-committed-text',lambda s:document.is_file() and document.read_text().rstrip('\n')==committed.rstrip('\n'))
        send(b'\x18\x03',allow_exit=True);process.wait(timeout=8)
        callbacks=read_callbacks(trace)
        if trace.stat().st_size>1_048_576 or not {'keyDown','setMarkedText','insertText'} <= {r.get('event') for r in callbacks}:
            raise RuntimeError('missing or oversized native callback evidence')
        result.update(passed=process.returncode==0,committed=committed,callback_count=len(callbacks),
                      launch=locals().get('launch_record'))
      except (OSError,ValueError,RuntimeError,subprocess.SubprocessError) as e:result['error']=str(e)
      finally:
        if process is not None and process.poll() is None:
            process.terminate()
            try:process.wait(timeout=2)
            except subprocess.TimeoutExpired:process.kill();process.wait(timeout=2)
        if original is not None:
            try:result['restored_source']=source('select',original)
            except (OSError,subprocess.SubprocessError) as e:result.update(passed=False,restore_error=str(e))
        result.update(seconds=time.monotonic()-started,exit_code=None if process is None else process.returncode)
    (work/'results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
    return result

def main():
    ap=argparse.ArgumentParser(description=__doc__)
    for name in ['binary','sender','sources','out']:ap.add_argument('--'+name,type=Path,required=True)
    ap.add_argument('--mode',choices=['steady','switch','multi','all'],default='all')
    ap.add_argument('--expected',default='你好');ap.add_argument('--pinyin-source')
    a=ap.parse_args();out=new_output(a.out)
    modes=['steady','switch'] if a.mode=='all' else [a.mode]
    rows=[run_scenario(a.binary.resolve(),a.sender.resolve(),a.sources.resolve(),out/m,m,a.expected,a.pinyin_source) for m in modes]
    result={'passed':all(r['passed'] for r in rows),'scenarios':rows}
    (out/'results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False))
    return 0 if result['passed'] else 1
if __name__=='__main__':sys.exit(main())
