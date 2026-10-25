#!/usr/bin/env python3
"""Compile the real key-name function with controlled JNI; protect externally."""
from pathlib import Path
import hashlib,json,subprocess,time,uuid

ROOT=Path(__file__).resolve().parent.parent

def main():
    source=ROOT/'src/android.rs'
    raw=source.read_text()
    anchor='pub unsafe extern "C" fn android_get_keysym_name ('
    if raw.count(anchor)!=1:raise RuntimeError('key-name source anchor changed')
    start=raw.index(anchor)
    end=raw.index('/* C 5887-5910:',start)
    actual=raw[start:end]
    fixture=(ROOT/'tools/fixtures/android-keysym-capacity.rs').read_text()
    if fixture.count('// ACTUAL_FUNCTION')!=1:raise RuntimeError('probe insertion changed')
    cache=ROOT/'.femacs-cache'
    parent=cache/'android-keysym-capacity'
    if cache.is_symlink() or parent.is_symlink():raise RuntimeError('linked probe output')
    parent.mkdir(parents=True,exist_ok=True)
    out=parent/uuid.uuid4().hex;out.mkdir()
    probe=out/'capacity.rs';probe.write_text(fixture.replace('// ACTUAL_FUNCTION',actual))
    before=hashlib.sha256(source.read_bytes()).hexdigest()
    steps=[]
    for name,args in [('compile',['rustc','--edition=2024','--test',str(probe),'-o',str(out/'capacity')]),
                      ('cases',[str(out/'capacity'),'--test-threads=1','--nocapture'])]:
        started=time.monotonic()
        with (out/(name+'.log')).open('xb') as log:
            result=subprocess.run(args,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
        steps.append({'name':name,'exit_code':result.returncode,'seconds':time.monotonic()-started})
        print(name,result.returncode,flush=True)
        if result.returncode:break
    after=hashlib.sha256(source.read_bytes()).hexdigest()
    passed=len(steps)==2 and all(s['exit_code']==0 for s in steps) and before==after
    (out/'receipt.json').write_text(json.dumps({'passed':passed,'steps':steps,
        'source_sha256':before,'source_preserved':before==after,
        'actual_function_sha256':hashlib.sha256(actual.encode()).hexdigest(),
        'scope':'actual Rust function capacity/termination/JNI-call ownership with controlled host responses; not Android JNI ABI or device acceptance'},indent=2)+'\n')
    print(out.relative_to(ROOT),flush=True)
    return 0 if passed else 1

if __name__=='__main__':raise SystemExit(main())
