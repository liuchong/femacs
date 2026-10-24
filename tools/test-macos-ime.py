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

def run_scenario(binary,sender,sources,work,mode,expected,pinyin_id=None):
    work=new_output(work)
    rows=[];process=None;started=time.monotonic();original=None
    result={'mode':mode,'passed':False,'steps':rows,'expected':expected}
    def source(*args):return subprocess.check_output([str(sources),*map(str,args)],text=True,timeout=8).strip()
    report=work/'snapshot.json';document=work/'ime.txt'
    env=dict(os.environ,FEMACS_IME_REPORT=str(report),FEMACS_INPUT_TRACE=str(work/'callbacks.jsonl'))
    with (work/'output.log').open('wb') as output:
      try:
        original=source('current');available=source('list').splitlines()
        result.update(original_source=original,available_sources=available)
        ascii_id=next((s for s in available if s in ['com.apple.keylayout.ABC','com.apple.keylayout.US']),None)
        pinyin=pinyin_id or next((s for s in available if 'SCIM' in s and ('ITABC' in s or 'Pinyin' in s)),None)
        if pinyin is None:pinyin=next((s for s in available if s.lower().endswith('.pinyin')),None)
        if not ascii_id or pinyin not in available:raise RuntimeError('requested enabled Pinyin and ABC sources are required')
        process=subprocess.Popen([str(binary),'-Q','-l',str(ROOT/'tools/fixtures/macos-ime.el')],env=env,stdout=output,stderr=subprocess.STDOUT)
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
            t=time.monotonic();source('select',ident,process.pid)
            state=status();rows.append({'id':'source-change','requested':ident,'at':time.monotonic(),'seconds':time.monotonic()-t,'observed':state})
            if process.poll() is not None:raise RuntimeError('editor exited during source change')
            if state.get('source')!=ident or not state.get('frontmost') or not state.get('focused_window'):
                raise RuntimeError('input source selection/focus not observed')
        def strokes(values,physical=False):
            p=work/'strokes.tsv'
            p.write_text(''.join(f'{key}\t{flags}'+('' if text is None else '\t'+text)+'\n' for key,flags,text in values))
            before=status()
            if not before.get('frontmost') or not before.get('focused_window'):raise RuntimeError('editor is not focused before keystrokes')
            t=time.monotonic()
            subprocess.run([str(sender),str(process.pid),str(p)],stdout=output,stderr=subprocess.STDOUT,check=True,timeout=8)
            rows.append({'id':'physical-keys' if physical else 'editor-keys','at':t,'keys':[x[0] for x in values],'before':before,'after':status()})
        def send(data):strokes(delivery.native_strokes(data))
        def pinyin_keys(keys):
            # Preserve the original burst cadence. Native keyDown traces record each received
            # key; helper observations bracket the burst without refocusing between letters.
            strokes([(key,0,None) for key in keys],physical=True)
        wait('native-startup',lambda s:s.get('native')=='ns')
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
        send(b'\x18\x17'+str(document).encode()+b'\r')
        wait('save-committed-text',lambda s:document.is_file() and document.read_text().rstrip('\n')==committed.rstrip('\n'))
        send(b'\x18\x03');process.wait(timeout=8)
        trace=work/'callbacks.jsonl'
        callbacks=[json.loads(line) for line in trace.read_text().splitlines()]
        if trace.stat().st_size>1_048_576 or not {'keyDown','setMarkedText','insertText'} <= {r.get('event') for r in callbacks}:
            raise RuntimeError('missing or oversized native callback evidence')
        result.update(passed=process.returncode==0,committed=committed,callback_count=len(callbacks))
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
    ap.add_argument('--mode',choices=['steady','switch','all'],default='all')
    ap.add_argument('--expected',default='你好');ap.add_argument('--pinyin-source')
    a=ap.parse_args();out=new_output(a.out)
    modes=['steady','switch'] if a.mode=='all' else [a.mode]
    rows=[run_scenario(a.binary.resolve(),a.sender.resolve(),a.sources.resolve(),out/m,m,a.expected,a.pinyin_source) for m in modes]
    result={'passed':all(r['passed'] for r in rows),'scenarios':rows}
    (out/'results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False))
    return 0 if result['passed'] else 1
if __name__=='__main__':sys.exit(main())
