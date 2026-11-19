"""Real IME acceptance: separate stable composition and switched-source pressure.

Every scenario starts a fresh editor and retains its callback/source/key timeline.
The default trace is opt-in for this acceptance process only. Use controlled-run runtime.
"""
import argparse
import ctypes
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

def titlebar_click_point(records,title,screen_height):
    """Compute Quartz top-left titlebar center from the latest Cocoa windowFrame."""
    if not isinstance(title,str) or not title or not records:return None
    if not isinstance(screen_height,(int,float)) or screen_height<=0:return None
    for row in reversed(records):
        if not isinstance(row,dict) or row.get('windowTitle')!=title:continue
        frame=row.get('windowFrame')
        if (isinstance(frame,list) and len(frame)==4
                and all(isinstance(v,(int,float)) and type(v) is not bool for v in frame)
                and frame[2]>20 and frame[3]>20):
            x,y_bottom,width,height=map(float,frame)
            return (x+width*0.5,float(screen_height)-(y_bottom+height)+14.0)
    return None

def main_display_height():
    cg=ctypes.CDLL('/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics')
    class _Pt(ctypes.Structure):_fields_=[('x',ctypes.c_double),('y',ctypes.c_double)]
    class _Rect(ctypes.Structure):_fields_=[('origin',_Pt),('size',_Pt)]
    cg.CGMainDisplayID.restype=ctypes.c_uint32
    cg.CGDisplayBounds.argtypes=[ctypes.c_uint32]
    cg.CGDisplayBounds.restype=_Rect
    return float(cg.CGDisplayBounds(cg.CGMainDisplayID()).size.y)

def client_source_sample(records,title,prefer_events=None):
    """Latest per-client input-source fields for one titled view, if present."""
    if not isinstance(title,str) or not title or not records:return None
    owned=[row for row in records if isinstance(row,dict) and row.get('windowTitle')==title]
    if prefer_events:
        preferred=[row for row in owned if row.get('event') in prefer_events]
        if preferred:owned=preferred
    else:
        interesting=[row for row in owned if row.get('event') in
                     ('windowDidBecomeKey','firstResponderState','becomeFirstResponder','keyDown')]
        if interesting:owned=interesting
    return owned[-1] if owned else None

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
            # Switch native key focus via a real HID click on the target window's
            # titlebar so WindowServer updates active IMK input sessions cleanly
            # even when another window holds pending marked composition.
            if snapshot().get('selected')!=title or not native_window_ready(read_callbacks(trace),title):
                path=work/'cycle.tsv'
                pt=titlebar_click_point(read_callbacks(trace),title,main_display_height())
                if pt is not None:
                    path.write_text(f'CLICK_POINT\t{pt[0]:.2f}\t{pt[1]:.2f}\n')
                else:
                    path.write_text('CYCLE_WINDOW\n')
                subprocess.run([str(sender),str(process.pid),str(path)],stdout=output,
                               stderr=subprocess.STDOUT,check=True,timeout=8)
                rows.append({'id':'native-window-focus-transfer','target':title,'at':time.monotonic(),
                             'click_point':None if pt is None else [pt[0],pt[1]]})
            wait(label,lambda s:s.get('selected')==title and native_window_ready(read_callbacks(trace),title))
            target_title=title
        def observe_client_source(label,prefer_events=None):
            # Diagnostic only: process TIS vs per-client NSTextInputContext source.
            # Missing fields (pre-rebuild binary) and mismatches stay in steps; they
            # never raise and never change scenario pass/fail.
            process_state=status()
            sample=client_source_sample(read_callbacks(trace),target_title,prefer_events)
            client=None if sample is None else sample.get('selectedKeyboardInputSource')
            has_context=None if sample is None else sample.get('clientHasInputContext')
            is_current=None if sample is None else sample.get('clientIsCurrentInputContext')
            field_present=isinstance(sample,dict) and (
                'selectedKeyboardInputSource' in sample or 'clientHasInputContext' in sample)
            rows.append({'id':label,'at':time.monotonic(),'window_title':target_title,
                         'process_source':process_state.get('source'),
                         'client_source':client,'client_has_input_context':has_context,
                         'client_is_current_input_context':is_current,
                         'diagnostic_fields_present':field_present,
                         'sources_equal':isinstance(client,str) and client==process_state.get('source'),
                         'sample_event':None if sample is None else sample.get('event'),
                         'role':'observation'})
        wait('native-startup',lambda s:s.get('native')=='ns')
        if not native_window_ready(read_callbacks(trace),target_title):
            source('activate',process.pid)
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
            select(pinyin)
            observe_client_source('client-source-after-b-focus-select',
                                  ('windowDidBecomeKey','firstResponderState','becomeFirstResponder','keyDown'))
            pinyin_keys([45,34,4,0,31])
            observe_client_source('client-source-on-b-first-keys',('keyDown',))
            wait('frame-b-marked',lambda s:s.get('marked'))
            pinyin_keys([49]);wait('frame-b-commit',lambda s:not s.get('marked') and s.get('second_text')==expected and s.get('text')==committed)
            focus_frame('Femacs IME A','frame-a-return')
            pinyin_keys([45,34]);wait('frame-a-pending',lambda s:s.get('marked'))
            focus_frame('Femacs IME B','pending-focus-transfer')
            # Cancel any remaining pending native composition before a new commit.
            # If deactivating A's inputContext already cleared marked text, do not
            # inject an extra Escape that would act as an ESC Meta prefix on B.
            focus_loss_cleared=not snapshot().get('marked')
            if not focus_loss_cleared:
                pinyin_keys([53])
            before=wait('pending-cleared',lambda s:not s.get('marked'))
            a_before=before['text'];b_before=before['second_text']
            if not a_before.startswith(committed) or b_before!=expected:raise RuntimeError('composition crossed owned buffers on focus change')
            pinyin_keys([45,34,4,0,31]);wait('second-frame-recompose',lambda s:s.get('marked'))
            pinyin_keys([49]);wait('second-frame-recommit',lambda s:not s.get('marked') and s.get('second_text')==expected*2 and s.get('text')==a_before)
            result['focus_loss_observation']={'a_text':a_before,'b_text':b_before,'focus_loss_cleared_marked':focus_loss_cleared,
                                              'policy':'observed OS commit/cancel; no universal focus-loss policy claimed'}
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
