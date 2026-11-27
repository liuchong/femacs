"""Generate target-local Cocoa symbol metadata and real C-global storage.

The reference's DEFSYM/DEFVAR forms are metadata inputs only. No reference object
is linked. The terminal target's frozen globals and name table remain unchanged.
"""
from __future__ import annotations
import argparse,csv,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
PLATFORM=('macfont','nsfns','nsimage','nsmenu','nsselect','nsterm')

def literal(value):return 'c'+json.dumps(value,ensure_ascii=False)

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--out',type=Path,required=True);a=ap.parse_args();out=a.out.resolve()
 if out.is_relative_to((ROOT/'emacs').resolve()):raise SystemExit('reference source is read-only')
 out.mkdir(parents=True,exist_ok=True)
 rows=list(csv.DictReader((ROOT/'docs/baseline/globals/symbols.tsv').open(),delimiter='\t'))
 symbols=[(r['q_name'],r['lisp_name']) for r in rows]
 metadata=json.loads((ROOT/'build-support/macos/cocoa/symbols.json').read_text())
 extra=metadata['extra_symbols'];overrides=metadata['target_name_overrides'];variables={};forwards={}
 kinds={'Lisp_Object':('Lisp_Object','Lisp_Fwd_Obj','objvar','core::ptr::null_mut()'),'bool':('bool','Lisp_Fwd_Bool','boolvar','false'),'EMACS_INT':('EMACS_INT','Lisp_Fwd_Int','intvar','0')}
 for name,ty in metadata['cocoa_variables'].items():variables[name]=kinds[ty]
 for name in PLATFORM:
  swift=(ROOT/f'platform/macos/{name}.swift').read_text()
  for m in re.finditer(r'(?:fileprivate var|@_silgen_name\("femacs_fwd_[^"]+"\) var)\s+O_FWD_(\w+)\s*:\s*Lisp_Fwd',swift):forwards[m[1]]=name
 if forwards!=metadata['forwards']:raise ValueError('Cocoa forwarding metadata differs from Swift source; update the owned target description')
 symbols=[(q,overrides.get(q,name)) for q,name in symbols]+sorted(extra.items())
 gpath=ROOT/'crates/lisp-vm/src/ported/src/include/globals.rs'
 if not gpath.is_file():gpath=ROOT/'src/include/globals.rs'
 original=gpath.read_text();tail=original[original.index('// --- variables declared by DEFVAR_*'):]
 header=['// Generated target-local Cocoa metadata; nil/t and existing baseline indices are preserved.']
 for i,(q,_) in enumerate(symbols):header += [f'pub const i{q}: usize = {i};',f'#[inline(always)] pub fn {q}() -> Lisp_Object {{ builtin_lisp_symbol({i}) }}']
 header += [f'pub const lispsym_count: usize = {len(symbols)};', 'unsafe extern "C" { pub static lispsym: VmGlobal<[Lisp_Symbol; lispsym_count]>; }',tail]
 (out/'globals.rs').write_text('\n'.join(header)+'\n')
 (out/'symbol-names.rs').write_text('[\n'+''.join(f'    {literal(name)}.as_ptr(),\n' for _,name in symbols)+']\n')
 code=['// Generated actual Cocoa DEFVAR storage and statically initialized forwarding records.', 'mod cocoa_runtime_exports {', 'use super::*;']
 for name,(ty,kind,member,zero) in sorted(variables.items()):code += ['#[unsafe(no_mangle)]',f'pub static {name}: VmGlobal<{ty}> = VmGlobal::new({zero});']
 for name in sorted(forwards):
  if name not in variables:raise ValueError('forward has no Cocoa DEFVAR metadata: '+name)
  ty,kind,member,_=variables[name];pointer=f'{name}.as_mut_ptr()'+(' as *mut i64' if ty=='EMACS_INT' else '')
  code += ['#[unsafe(no_mangle)]',f'pub static femacs_fwd_{name}: VmGlobal<Lisp_Fwd> = VmGlobal::new(Lisp_Fwd {{ r#type: {kind}, u: Lisp_Fwd_u {{ {member}: {pointer} }} }});']
 for q in sorted(extra):code += [f'#[unsafe(export_name="{q}")]', f'pub unsafe extern "C-unwind" fn export_{q}() -> Lisp_Object {{ {q}() }}']
 code += ['#[unsafe(no_mangle)]', 'pub static svcsMenu: VmGlobal<*mut c_void> = VmGlobal::new(core::ptr::null_mut());']
 for name in ['ns_app_name','ns_send_types','ns_return_types','x_display_list']:
  code += ['#[unsafe(no_mangle)]',f'pub static {name}: VmGlobal<*mut c_void> = VmGlobal::new(core::ptr::null_mut());']
 code += ['#[unsafe(no_mangle)]','pub static context_menu_value: VmGlobal<isize> = VmGlobal::new(0);']

 handlers=(ROOT/'platform/macos/nsfns.swift').read_text().split('let ns_frame_parm_handlers: [frame_parm_handler?] = [',1)[1].split('\n]',1)[0]
 entries=[]
 for line in handlers.splitlines():
  line=line.strip()
  if line.startswith('nil,'):entries.append(None)
  elif line.startswith('{'):entries.append(re.search(r'in (\w+)\(',line)[1])
 code += ['type CocoaFrameParm = Option<unsafe extern "C-unwind" fn(*mut c_void, Lisp_Object, Lisp_Object)>;', 'unsafe extern "C-unwind" {']
 for name in sorted(set(entries)-{None}):code += [f'#[link_name="{name}"] fn handler_{name}(f: *mut c_void, value: Lisp_Object, old: Lisp_Object);']
 code += ['}', '#[unsafe(no_mangle)]', f'pub static ns_frame_parm_handlers: VmGlobal<[CocoaFrameParm; {len(entries)}]> = VmGlobal::new([']
 code += ['None,' if name is None else f'Some(handler_{name}),' for name in entries]
 code += [']);']
 code.append('}')
 (out/'runtime.rs').write_text('\n'.join(code)+'\n')
 (out/'runtime.json').write_text(json.dumps({'extra_symbols':extra,'target_name_overrides':overrides,'cocoa_variables':{k:v[0] for k,v in variables.items()},'forwards':forwards,'symbol_count':len(symbols)},indent=2)+'\n')
 print(f'Cocoa metadata: {len(symbols)} symbols ({len(extra)} additional), {len(variables)} actual globals, {len(forwards)} static forwarders')
 return 0
if __name__=='__main__':raise SystemExit(main())
