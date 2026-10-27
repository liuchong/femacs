"""Protect the actual frame-flag bridge against selecting the wrong flag word.

Compile the production selector bodies against a two-word component fixture.
This checks word selection and neighbor preservation, not the full frame ABI;
real Cocoa layout/tab-bar acceptance covers the complete editor boundary.
Invoke under controlled-run's build profile. Output is a fresh owned build child.
"""
from pathlib import Path
import argparse, hashlib, json, re, subprocess, time

ROOT = Path(__file__).resolve().parent.parent
NAMES = ('frame_set_can_set_window_size', 'frame_tab_bar_redisplayed',
         'frame_tab_bar_resized', 'frame_set_tab_bar_resized')
STEMS = ('CAN_SET_WINDOW_SIZE', 'TAB_BAR_REDISPLAYED', 'TAB_BAR_RESIZED')

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def function(text, name):
    match = re.search(r'pub (?:unsafe )?(?:extern "C" )?fn '+re.escape(name)+r'\s*\(', text)
    if not match:
        raise ValueError('missing production function: '+name)
    opening = text.index('{', match.start())
    level = 1
    end = opening + 1
    while level:
        if text[end] == '{': level += 1
        if text[end] == '}': level -= 1
        end += 1
    return text[match.start():end]

def constant(text, name, cocoa):
    pattern = r'#\[cfg \(([^\n]+)\)\]\npub const '+name+r': u32 = (\d+);'
    matches = re.findall(pattern, text)
    condition = 'feature = "have_window_system"' if cocoa else 'not (feature = "have_window_system")'
    values = [int(value) for cfg, value in matches if cfg == condition]
    if len(values) != 1:
        raise ValueError('missing or ambiguous selected constant: '+name)
    return values[0]

def generate(header, bridge, ledger):
    measured = {}
    for line in ledger.splitlines():
        fields = line.split('\t')
        if len(fields) == 6 and fields[0] in ('frozen', 'ns_cocoa'):
            measured[(fields[0], fields[1])] = tuple(map(int, fields[3:5]))
    modules = []
    for profile, cocoa, arm in [('terminal', False, 'frozen'), ('cocoa', True, 'ns_cocoa')]:
        constants = []
        for stem in STEMS:
            unit = constant(header, 'FRAME_'+stem+'_UNIT', cocoa)
            shift = constant(header, 'FRAME_'+stem+'_SHIFT', cocoa)
            assert (unit, shift) == measured[(arm, stem.lower())]
            constants += [f'const FRAME_{stem}_UNIT: u32 = {unit};',
                          f'const FRAME_{stem}_SHIFT: u32 = {shift};']
        bodies = '\n'.join(function(header, name) for name in ('frame_word','frame_flag','frame_set_flag'))
        bodies += '\n'+'\n'.join(function(bridge, name) for name in NAMES)
        modules.append('mod '+profile+' {\nuse std::ffi::c_void;\n#[allow(non_camel_case_types)]\nstruct frame { bits1:u32,bits2:u32 }\n'+
                       '\n'.join(constants)+'\n'+bodies+RUST_TESTS.replace('PROFILE', profile)+'\n}\n')
    return '#![allow(dead_code,unused_unsafe)]\n'+''.join(modules)+'fn main(){terminal::run();cocoa::run();}\n'

RUST_TESTS = r'''
pub fn run() {
  for value in [false,true] {
    let start=std::time::Instant::now();
    let mut f=frame{bits1:0x69696969,bits2:0x96969696};
    let before=[f.bits1,f.bits2];
    unsafe{frame_set_can_set_window_size((&raw mut f).cast(),value);}
    let mut expected=before;
    let slot=FRAME_CAN_SET_WINDOW_SIZE_UNIT as usize-1;
    let mask=1<<FRAME_CAN_SET_WINDOW_SIZE_SHIFT;
    expected[slot]=(before[slot]&!mask)|((value as u32)<<FRAME_CAN_SET_WINDOW_SIZE_SHIFT);
    assert_eq!([f.bits1,f.bits2],expected);
    println!("{{\"case\":\"PROFILE:can-set:{}\",\"passed\":true,\"seconds\":{}}}",value,start.elapsed().as_secs_f64());
    let start=std::time::Instant::now();
    let mut f=frame{bits1:0x69696969,bits2:0x96969696};
    let before=[f.bits1,f.bits2];
    unsafe{frame_set_tab_bar_resized((&raw mut f).cast(),value);}
    let mut expected=before;
    let slot=FRAME_TAB_BAR_RESIZED_UNIT as usize-1;
    let mask=1<<FRAME_TAB_BAR_RESIZED_SHIFT;
    expected[slot]=(before[slot]&!mask)|((value as u32)<<FRAME_TAB_BAR_RESIZED_SHIFT);
    assert_eq!([f.bits1,f.bits2],expected);
    assert_eq!(unsafe{frame_tab_bar_resized((&raw mut f).cast())},value);
    println!("{{\"case\":\"PROFILE:resized-write:{}\",\"passed\":true,\"seconds\":{}}}",value,start.elapsed().as_secs_f64());
    for (unit,shift,reader,name) in [
      (FRAME_TAB_BAR_RESIZED_UNIT,FRAME_TAB_BAR_RESIZED_SHIFT,frame_tab_bar_resized as unsafe extern "C" fn(*mut c_void)->bool,"resized-read"),
      (FRAME_TAB_BAR_REDISPLAYED_UNIT,FRAME_TAB_BAR_REDISPLAYED_SHIFT,frame_tab_bar_redisplayed as unsafe extern "C" fn(*mut c_void)->bool,"redisplayed-read")
    ] {
      let start=std::time::Instant::now();
      let mut words=[if value{0}else{u32::MAX};2];
      let mask=1<<shift;
      words[unit as usize-1]=(words[unit as usize-1]&!mask)|((value as u32)<<shift);
      let mut f=frame{bits1:words[0],bits2:words[1]};
      assert_eq!(unsafe{reader((&raw mut f).cast())},value);
      assert_eq!([f.bits1,f.bits2],words);
      println!("{{\"case\":\"PROFILE:{}:{}\",\"passed\":true,\"seconds\":{}}}",name,value,start.elapsed().as_secs_f64());
    }
  }
}
'''

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,required=True)
    args=parser.parse_args()
    out=args.out.absolute()
    if (not out.is_relative_to(ROOT/'build') or out == ROOT/'build'
            or any(p.is_symlink() for p in [out,*out.parents])):
        raise ValueError('output must be a distinct nonsymlink owned build child')
    out.mkdir(parents=True,exist_ok=False)
    paths=[ROOT/'src/include/frame.rs',ROOT/'src/abi_shim.rs',ROOT/'docs/baseline/frame-bits.tsv']
    binding={str(p.relative_to(ROOT)):sha(p) for p in paths}
    text=generate(*(p.read_text() for p in paths))
    (out/'probe.rs').write_text(text)
    report=dict(schema='femacs-frame-flag-components-v1',inputs=binding,passed=False,cases=[])
    try:
        for name,argv in [('compile',['rustc','--edition','2024','-O',str(out/'probe.rs'),'-o',str(out/'probe')]),
                          ('execute',[str(out/'probe')])]:
            start=time.monotonic()
            with (out/(name+'.stdout')).open('xb') as stdout,(out/(name+'.stderr')).open('xb') as stderr:
                result=subprocess.run(argv,cwd=ROOT,stdout=stdout,stderr=stderr)
            report[name]=dict(exit_code=result.returncode,seconds=time.monotonic()-start)
            if result.returncode: return 1
        report['cases']=[json.loads(line) for line in (out/'execute.stdout').read_text().splitlines()]
        report['inputs_unchanged']=binding=={str(p.relative_to(ROOT)):sha(p) for p in paths}
        report['passed']=(len(report['cases'])==16 and len({x['case'] for x in report['cases']})==16
                          and all(x['passed'] for x in report['cases']) and report['inputs_unchanged'])
        return 0 if report['passed'] else 1
    finally:
        (out/'results.json').write_text(json.dumps(report,indent=2)+'\n')
        print(json.dumps(dict(passed=report['passed'],cases=len(report['cases']))))

if __name__=='__main__':
    raise SystemExit(main())
