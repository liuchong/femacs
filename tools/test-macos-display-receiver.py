"""Protect P-394's generated field accessor after actual native UI acceptance.

Compile the real generated setter and real frame-to-display macros against
isolated, disjoint sentinel records. This component test does not replace Cocoa
input acceptance or certify target structure layout.
"""
from pathlib import Path
import argparse, hashlib, importlib.util, json, subprocess, time
ROOT = Path(__file__).resolve().parent.parent

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--out', type=Path, required=True)
    out = ap.parse_args().out.absolute()
    if not out.is_relative_to(ROOT/'build') or out == ROOT/'build' or any(p.is_symlink() for p in [out,*out.parents]):
        raise ValueError('fresh nonsymlink owned build child required')
    out.mkdir(parents=True, exist_ok=False)
    source = ROOT/'tools/gen-macos-exports.py'
    spec = importlib.util.spec_from_file_location('exports',source)
    provider = importlib.util.module_from_spec(spec); spec.loader.exec_module(provider)
    imports = provider.swift_imports()
    names = ['ns_display_info_set_last_mouse_scroll_bar', 'ns_display_info_set_n_fonts']
    code, mapping = provider.record_exports({name:imports[name] for name in names},set())
    assert {m['name'] for m in mapping} == set(names)
    header = (ROOT/'src/include/nsterm.rs').read_text()
    macros = []
    for name in ['FRAME_DISPLAY_INFO','FRAME_OUTPUT_DATA']:
        start = header.index('macro_rules! '+name+' {'); opening = header.index('{',start)
        end = provider.balanced(header,opening,'{','}')
        macros.append(header[start:end+1])
    harness = r'''
#![allow(non_camel_case_types, unsafe_op_in_unsafe_fn, unused_parens)]
use std::{ffi::c_void,ptr};
#[repr(C)] struct ns_display_info { prefix: [u8;264], n_fonts:i32, pad:i32, last_mouse_scroll_bar:*mut c_void, tail:[u8;32] }
#[repr(C)] union output { ns:*mut c_void }
#[repr(C)] struct frame { output_data:output, sentinel:[u8;312] }
#[repr(C)] struct ns_output { display_info:*mut ns_display_info }
fn main() { unsafe {
 let mut a:ns_display_info=std::mem::zeroed(); let mut b:ns_display_info=std::mem::zeroed();
 a.prefix.fill(0xa5);a.tail.fill(0x5a);b.prefix.fill(0xb5);b.tail.fill(0x6a);
 let mut oa=ns_output{display_info:&mut a};let mut ob=ns_output{display_info:&mut b};
 let mut fa=frame{output_data:output{ns:ptr::from_mut(&mut oa).cast()},sentinel:[0x35;312]};
 let mut fb=frame{output_data:output{ns:ptr::from_mut(&mut ob).cast()},sentinel:[0x45;312]};
 for (name, action) in [("clear",0usize),("set",1),("alternate",2),("display-neighbor",3)] {
  let start=std::time::Instant::now();
  let marker=ptr::from_mut(&mut oa).cast::<c_void>();
  match action {
   0=>{a.last_mouse_scroll_bar=marker;view_ns_display_info_set_last_mouse_scroll_bar(ptr::from_mut(&mut fa).cast(),ptr::null_mut());assert!(a.last_mouse_scroll_bar.is_null());},
   1=>{view_ns_display_info_set_last_mouse_scroll_bar(ptr::from_mut(&mut fa).cast(),marker);assert_eq!(a.last_mouse_scroll_bar,marker);},
   2=>{for _ in 0..128 {view_ns_display_info_set_last_mouse_scroll_bar(ptr::from_mut(&mut fb).cast(),marker);view_ns_display_info_set_last_mouse_scroll_bar(ptr::from_mut(&mut fa).cast(),ptr::null_mut());}assert_eq!(b.last_mouse_scroll_bar,marker);assert!(a.last_mouse_scroll_bar.is_null());},
   _=>{view_ns_display_info_set_n_fonts(ptr::from_mut(&mut a).cast(),73);assert_eq!(a.n_fonts,73);assert_eq!(b.n_fonts,0);}
  }
  assert_eq!(fa.sentinel,[0x35;312]);assert_eq!(fb.sentinel,[0x45;312]);
  assert_eq!(fa.output_data.ns,ptr::from_mut(&mut oa).cast());assert_eq!(fb.output_data.ns,ptr::from_mut(&mut ob).cast());
  assert_eq!(a.prefix,[0xa5;264]);assert_eq!(a.tail,[0x5a;32]);assert_eq!(b.prefix,[0xb5;264]);assert_eq!(b.tail,[0x6a;32]);
  println!("{{\"case\":\"{}\",\"passed\":true,\"seconds\":{}}}",name,start.elapsed().as_secs_f64());
 }
}}
'''
    binding = {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [source,ROOT/'src/include/nsterm.rs',ROOT/'platform/macos/nsterm.swift']}
    (out/'probe.rs').write_text(harness[:harness.index('fn main()')]+ '\n'.join(macros+code)+'\n'+harness[harness.index('fn main()'):])
    report=dict(schema='femacs-display-receiver-components-v1',source_sha256=binding,passed=False)
    try:
        for phase, argv in [('compile',['rustc','--edition=2024','-O',str(out/'probe.rs'),'-o',str(out/'probe')]),('execute',[str(out/'probe')])]:
            start=time.monotonic()
            with (out/(phase+'.stdout')).open('xb') as stdout,(out/(phase+'.stderr')).open('xb') as stderr:
                result=subprocess.run(argv,cwd=ROOT,stdout=stdout,stderr=stderr)
            report[phase]=dict(exit_code=result.returncode,seconds=time.monotonic()-start)
            if result.returncode:return 1
        report['cases']=[json.loads(line) for line in (out/'execute.stdout').read_text().splitlines()]
        report['passed']=len(report['cases'])==4 and all(x['passed'] for x in report['cases'])
        return 0 if report['passed'] else 1
    finally:
        (out/'results.json').write_text(json.dumps(report,indent=2)+'\n')
        print(json.dumps(report))
if __name__=='__main__':raise SystemExit(main())
