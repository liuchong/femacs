#!/usr/bin/env python3
"""Compile actual platform offset/table selections; protect compiler externally."""
from pathlib import Path
import hashlib,json,subprocess,time,uuid

ROOT=Path(__file__).resolve().parent.parent

def main():
    source=ROOT/'src/keyboard.rs';raw=source.read_text()
    offset=raw[raw.index('/* C 5505:'):raw.index("/* You'll notice that this table")]
    table_start=raw.index('#[cfg (feature = "have_android")]\nstatic mut lispy_function_keys:')
    table=raw[table_start:raw.index('/* C 5209-5461:',table_start)]
    parent=ROOT/'.femacs-cache/keyboard-platform-offset'
    if parent.parent.is_symlink() or parent.is_symlink():raise RuntimeError('linked test output')
    parent.mkdir(parents=True,exist_ok=True);out=parent/uuid.uuid4().hex;out.mkdir()
    before=hashlib.sha256(source.read_bytes()).hexdigest();steps=[]
    # Independent expected hardware key meanings, including noncontiguous slots.
    keys=[(19,'up'),(20,'down'),(21,'left'),(22,'right'),(61,'tab'),
          (66,'return'),(67,'backspace'),(111,'escape'),(112,'delete'),
          (122,'home'),(123,'end'),(124,'insert'),(131,'f1'),(337,'f24'),
          (92,'prior'),(93,'next')]
    cases='\n'.join('''case!(key_%d, {
        let code=%dusize;
        assert!(code>=FUNCTION_KEY_OFFSET as usize);
        let index=code-FUNCTION_KEY_OFFSET as usize;
        assert!(index<338);
        let pointer=unsafe { lispy_function_keys[index] };
        assert!(!pointer.is_null());
        assert_eq!(unsafe { std::ffi::CStr::from_ptr(pointer) }.to_str().unwrap(),%s);
    });'''%(code,code,json.dumps(name)) for code,name in keys)
    for name,feature,expected in [('android','have_android',0),('ntgui','have_ntgui',0),('host',None,0xff00)]:
        text='#![allow(dead_code,unused_imports)]\nuse std::ffi::c_char;\nuse std::ffi::c_int;\n'+offset+'\n'
        text+='''macro_rules! case { ($name:ident,$body:block) => {
            #[test] fn $name() { let started=std::time::Instant::now();$body
              println!("CASE {} seconds={:.6}",stringify!($name),started.elapsed().as_secs_f64()); }
        }; }
'''
        text+='case!(platform_offset, { assert_eq!(FUNCTION_KEY_OFFSET,'+str(expected)+'); });\n'
        if name=='android':
            text+=table+'\n'+cases+'\n'
            text+='case!(unmapped_key_retains_fallback, { assert!(unsafe { lispy_function_keys[104] }.is_null()); });\n'
        path=out/(name+'.rs');path.write_text(text)
        args=['rustc','--edition=2024','--test',str(path),'-o',str(out/name)]
        if feature:args+=['--cfg','feature="'+feature+'"']
        for stage,argv in [('compile',args),('cases',[str(out/name),'--test-threads=1','--nocapture'])]:
            started=time.monotonic()
            with (out/(name+'-'+stage+'.log')).open('xb') as log:
                result=subprocess.run(argv,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
            steps.append({'selection':name,'stage':stage,'exit_code':result.returncode,'seconds':time.monotonic()-started})
            print(name,stage,result.returncode,flush=True)
            if result.returncode:break
    after=hashlib.sha256(source.read_bytes()).hexdigest()
    passed=len(steps)==6 and all(s['exit_code']==0 for s in steps) and before==after
    (out/'receipt.json').write_text(json.dumps({'passed':passed,'steps':steps,'source_sha256':before,
        'source_preserved':before==after,'offset_source_sha256':hashlib.sha256(offset.encode()).hexdigest(),
        'android_table_source_sha256':hashlib.sha256(table.encode()).hexdigest(),
        'scope':'actual extracted constant/table under Android/NTGUI/host cfg; not target binary or device event acceptance'},indent=2)+'\n')
    print(out.relative_to(ROOT),flush=True)
    return 0 if passed else 1

if __name__=='__main__':raise SystemExit(main())
