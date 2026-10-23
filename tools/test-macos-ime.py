"""Real installed Pinyin: source changes, composition, cancellation and saved text.

Run under controlled-run runtime. Uses physical key events, never Unicode injection
for composition. Saves original input-source identity and restores it on all exits.
"""
import argparse,json,os,re,subprocess,time
from pathlib import Path
import importlib.util
ROOT=Path(__file__).resolve().parent.parent
spec=importlib.util.spec_from_file_location('delivery',Path(__file__).with_name('test-macos-delivery.py'))
delivery=importlib.util.module_from_spec(spec);spec.loader.exec_module(delivery)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--binary',type=Path,required=True);ap.add_argument('--sender',type=Path,required=True);ap.add_argument('--sources',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);a=ap.parse_args()
    work=a.out.resolve();work.mkdir(parents=True,exist_ok=True)
    binary=a.binary.resolve();sender=a.sender.resolve();sources=a.sources.resolve()
    def source(*args):return subprocess.check_output([str(sources),*map(str,args)],text=True,timeout=8).strip()
    original=source('current');available=source('list').splitlines()
    ascii_id=next((s for s in available if s in ['com.apple.keylayout.ABC','com.apple.keylayout.US']),None)
    pinyin=next((s for s in available if 'SCIM' in s and ('ITABC' in s or 'Pinyin' in s)),None)
    if pinyin is None:pinyin=next((s for s in available if s.lower().endswith('.pinyin')),None)
    rows=[];process=None;started=time.monotonic();result={'passed':False,'steps':rows,'original_source':original,'available_sources':available}
    if not ascii_id or not pinyin:raise RuntimeError('installed ABC and Pinyin sources are required')
    report=work/'snapshot.json';document=work/'ime.txt';env=dict(os.environ,FEMACS_IME_REPORT=str(report))
    with (work/'output.log').open('wb') as output:
      try:
        process=subprocess.Popen([str(binary),'-Q','-l',str(ROOT/'tools/fixtures/macos-ime.el')],env=env,stdout=output,stderr=subprocess.STDOUT)
        def snapshot():
            try:return json.loads(report.read_text())
            except (OSError,ValueError):return {}
        def wait(label,predicate):
            t=time.monotonic();deadline=t+8
            while time.monotonic()<deadline and process.poll() is None:
                state=snapshot()
                if predicate(state):rows.append({'id':label,'seconds':round(time.monotonic()-t,4),'state':state});return state
                time.sleep(.04)
            raise RuntimeError(f'{label}: exit={process.poll()}, state={snapshot()}')
        def select(id):
            t=time.monotonic();source('select',id,process.pid);rows.append({'id':'source-change','source':id,'seconds':round(time.monotonic()-t,4)})
            if process.poll() is not None:raise RuntimeError('editor exited during input-source change')
        def strokes(values):
            p=work/'strokes.tsv';p.write_text(''.join(f'{key}\t{flags}'+('' if text is None else '\t'+text)+'\n' for key,flags,text in values))
            subprocess.run([str(sender),str(process.pid),str(p)],stdout=output,stderr=subprocess.STDOUT,check=True,timeout=8)
        def send(data):strokes(delivery.native_strokes(data))
        def pinyin_keys(keys):strokes([(k,0,None) for k in keys])
        wait('native-startup',lambda s:s.get('native')=='ns')
        select(ascii_id)
        for _ in range(3):select(pinyin);select(ascii_id)
        select(pinyin);pinyin_keys([45,34,4,0,31]) # nihao, physical US key positions
        wait('pinyin-marked',lambda s:s.get('marked') and bool(s.get('working')))
        pinyin_keys([49]) # space chooses current candidate
        committed=wait('pinyin-commit',lambda s:not s.get('marked') and bool(re.search('[\u3400-\u9fff]',s.get('text',''))))['text']
        pinyin_keys([45,34]);wait('cancel-composition-start',lambda s:s.get('marked'))
        pinyin_keys([53]);wait('cancel-composition',lambda s:not s.get('marked') and s.get('text')==committed)
        select(ascii_id);send(b'\x1bx');wait('minibuffer-start',lambda s:s.get('minibuffer'))
        select(pinyin);pinyin_keys([4,0,31]);wait('minibuffer-marked',lambda s:s.get('minibuffer') and s.get('marked'))
        pinyin_keys([53]);send(b'\x07');select(ascii_id)
        wait('minibuffer-cancel',lambda s:not s.get('minibuffer') and not s.get('marked') and s.get('text')==committed)
        send(b'\x18\x17'+str(document).encode()+b'\r')
        wait('save-committed-text',lambda s:document.is_file() and document.read_text().rstrip('\n')==committed.rstrip('\n'))
        send(b'\x18\x03');process.wait(timeout=8)
        result.update(passed=process.returncode==0,exit_code=process.returncode,committed=committed)
      except (OSError,RuntimeError,subprocess.SubprocessError) as e:result['error']=str(e)
      finally:
        if process is not None and process.poll() is None:
            process.terminate()
            try:process.wait(timeout=2)
            except subprocess.TimeoutExpired:process.kill();process.wait(timeout=2)
        try:result['restored_source']=source('select',original)
        except (OSError,subprocess.SubprocessError) as e:result.update(passed=False,restore_error=str(e))
        result.update(seconds=round(time.monotonic()-started,4),exit_code=None if process is None else process.returncode)
    (work/'results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False))
    return 0 if result['passed'] else 1
if __name__=='__main__':raise SystemExit(main())
