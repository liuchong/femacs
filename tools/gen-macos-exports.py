"""Reify actual Rust header providers imported by the Cocoa ports.

Generated code is target-local, not a second implementation of Lisp semantics.
Unknown symbols are listed explicitly for the complete link to diagnose: no stubs,
no default-function results, and no dynamic/undefined-symbol linker escape hatch.
"""
from __future__ import annotations
import argparse,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent

def balanced(text,start,opening='(',closing=')'):
    depth=0;quote=None;escaped=False
    for i in range(start,len(text)):
        c=text[i]
        if quote:
            if escaped:escaped=False
            elif c=='\\':escaped=True
            elif c==quote:quote=None
            continue
        if c=='"':quote=c;continue
        if c==opening:depth+=1
        elif c==closing:
            depth-=1
            if depth==0:return i
    raise ValueError('unbalanced declaration')

def split_args(text):
    out=[];start=0;depth=0
    for i,c in enumerate(text):
        if c in '(<[':depth+=1
        if c in ')]' or c=='>' and (i==0 or text[i-1]!='-'):depth-=1
        if c==',' and depth==0:out.append(text[start:i].strip());start=i+1
    if text[start:].strip():out.append(text[start:].strip())
    return out

def swift_imports():
    imports={}
    for p in sorted((ROOT/'femacs/platform/macos').glob('*.swift')):
        if p.stem not in ('macfont','nsfns','nsimage','nsmenu','nsselect','nsterm'):continue
        text=p.read_text()
        for m in re.finditer(r'@_silgen_name\("([^"]+)"\)\s*(?:(?:fileprivate|private|public|internal)\s+)?func\s+(\w+)\s*\(',text):
            end=balanced(text,m.end()-1)
            tail=re.match(r'\s*->\s*([^\n{]+)',text[end+1:])
            imports.setdefault(m[1],{'args':text[m.end():end],'return':tail[1].split('//')[0].strip() if tail else 'Void','file':p.name})
    return imports

def record_exports(imports, existing):
    """Cocoa views of actual header fields; never a parallel record layout."""
    expressions={}; scalar={'Int':'isize','UInt':'usize','Int16':'i16','Int32':'i32','UInt32':'u32','Bool':'bool','Double':'f64','Void':'()','Lisp_Object':'Lisp_Object'}
    def record(prefix,ty,fields,base=None):
        base=base or f'(*a0.cast::<{ty}>())'
        for field in fields.split():
            expressions[prefix+'_'+field]=f'{base}.{field}'
            expressions[prefix+'_set_'+field]=f'{base}.{field} = a1 as _;'
    record('glyph_string','glyph_string','x y ybase width height background_width left_overhang right_overhang f w row area hl face img clip_head clip_tail next prev char2b num_clips underline_position underline_thickness')
    record('glyph','glyph','ascent descent')
    record('image','image','height width hmargin vmargin mask relief')
    record('font','font','average_width baseline_offset descent underline_position underline_thickness')
    record('face','face','id box_color font stipple underline_color overline_color strike_through_color box_horizontal_line_width box_vertical_line_width underline_pixels_above_descent_line')
    record('terminal','terminal','kboard name rif')
    record('kboard','kboard','reference_count')
    record('window','window','vertical_scroll_bar horizontal_scroll_bar')
    record('mouse_highlight','Mouse_HLInfo','mouse_face_face_id mouse_face_mouse_x mouse_face_mouse_y')
    record('ns_display_info','ns_display_info','next name_list_element color_p n_fonts n_planes root_window resx resy smallest_char_width smallest_font_height terminal horizontal_scroll_bar_cursor vertical_scroll_bar_cursor last_mouse_scroll_bar')
    record('ns_output','ns_output','baseline_offset font fontset dark_relief_color light_relief_color relief_background_color',base='(*(*a0.cast::<frame>()).output_data.ns.cast::<ns_output>())')
    for field in ('dark_relief_color','light_relief_color','relief_background_color'):
        expressions['ns_output_set_'+field] = f'replace_native_object(core::ptr::addr_of_mut!((*(*a0.cast::<frame>()).output_data.ns.cast::<ns_output>()).{field}), a1);'
    record('frame','frame','column_width line_height ns_appearance ns_transparent_titlebar')
    record('run','run','current_y desired_y height')
    for name in 'last_mouse_frame last_mouse_movement_time'.split():
        expressions['ns_display_info_'+name]=f'(*FRAME_DISPLAY_INFO!(a0.cast::<frame>())).{name}'
    expressions.update({
        # C's TYPE_RANGED_FIXNUMP(int, value) specializes to the C int bounds.
        # The shared Rust helper accepts (lower, value, upper); its ABI differs.
        'TYPE_RANGED_FIXNUMP':'TYPE_RANGED_FIXNUMP(i32::MIN as EMACS_INT, a0, i32::MAX as EMACS_INT)',
        'ns_display_info_mouse_highlight':'core::ptr::addr_of_mut!((*a0.cast::<ns_display_info>()).mouse_highlight)',
        'ns_display_info_bitmaps_img':'(*(*a0.cast::<ns_display_info>()).bitmaps.offset(a1 as isize)).img',
        'buffer_directory':'(*a0.cast::<buffer>()).directory_',
        'kboard_default_minibuffer_frame':'(*a0.cast::<kboard>()).Vdefault_minibuffer_frame_',
        'current_buffer_begv':'BEGV!()', 'current_buffer_begv_byte':'BEGV_BYTE!()',
        'char_table_extras':'*(*a0.cast::<Lisp_Char_Table>()).extras.as_ptr().offset(a1 as isize)',
        'composition_offsets':'*(*a0.cast::<composition>()).offsets.offset(a1 as isize)',
        'glyph_advance':'a0.cast::<glyph>().offset(a1 as isize)',
        'glyph_row_glyphs':'(*a0.cast::<glyph_row>()).glyphs[a1 as usize]',
        'glyph_string_slice_ptr':'core::ptr::addr_of_mut!((*a0.cast::<glyph_string>()).slice)',
        'glyph_slice_cmp_to':'(*a0.cast::<glyph>()).slice.cmp.to',
        'terminal_increment_reference_count':'(*a0.cast::<terminal>()).reference_count += 1;',
        'terminal_set_display_info_ns':'(*a0.cast::<terminal>()).display_info.ns = a1;',
        'frame_set_output_data_ns':'(*a0.cast::<frame>()).output_data.ns = a1;',
        'frame_set_no_focus_on_map':'frame_set_flag(a0.cast::<frame>(), FRAME_NO_FOCUS_ON_MAP_UNIT, FRAME_NO_FOCUS_ON_MAP_SHIFT, a1 as u32);',
        'frame_set_z_group':'frame_set_field(a0.cast::<frame>(), FRAME_Z_GROUP_FLAG_UNIT, FRAME_Z_GROUP_FLAG_SHIFT, FRAME_Z_GROUP_MASK, a1 as u32);',
        'frame_set_tooltip':'frame_set_flag(a0.cast::<frame>(), FRAME_TOOLTIP_UNIT, FRAME_TOOLTIP_SHIFT, a1 as u32);',
        'PVEC_SUBR':'pvec_type::PVEC_SUBR',
        'RES_TYPE_NUMBER':'0', 'RES_TYPE_BOOLEAN':'2', 'RES_TYPE_STRING':'3', 'RES_TYPE_SYMBOL':'4',
        'CHECK_FRAME':'CHECK_TYPE(FRAMEP(a0), Qframep(), a0);',
        'XSETTERMINAL':'XSETTERMINAL!(*a0, a1.cast::<terminal>());',
        'XNS_SCROLL_BAR':'XNS_SCROLL_BAR!(a0)', 'flush_frame':'let rif = FRAME_RIF!(a0.cast::<frame>()).cast::<redisplay_interface>(); if !rif.is_null() { if let Some(flush) = (*rif).flush_display { flush(a0); } }',
    })
    for field,unit,shift in [('x',0,0),('y',0,16),('width',1,0),('height',1,16)]:expressions['glyph_string_slice_'+field]=f'((*a0.cast::<glyph_string>()).slice.bits[{unit}] >> {shift}) & 65535'
    for field in 'upper_xoff upper_yoff lower_xoff lower_yoff'.split():expressions['glyph_slice_glyphless_'+field]=f'(*a0.cast::<glyph>()).slice.glyphless.{field}'
    for field in 'left_box_line_p right_box_line_p'.split():expressions['glyph_'+field]=f'glyph_bits!({field}, *a0.cast::<glyph>())'
    for field in 'mouse_face_p reversed_p stipple_p'.split():
        expressions['glyph_row_'+field]=f'glyph_row_bits!({field}, a0.cast::<glyph_row>())'
        expressions['glyph_row_set_'+field]=f'glyph_row_bits!({field}, a0.cast::<glyph_row>(), a1);'
    for field in 'background_filled_p extends_to_end_of_line_p font_not_found_p stippled_p for_overlaps'.split():
        expressions['glyph_string_'+field]=f'glyph_string_bits!({field}, *a0.cast::<glyph_string>())'
        expressions['glyph_string_set_'+field]=f'glyph_string_bits!({field}, *a0.cast::<glyph_string>(), a1);'
    for field,shift,width in [('automatic',0,1),('id',1,31)]:
        expression=f'((*a0.cast::<glyph>()).u.cmp.bits >> {shift}) & {2**width-1}'
        expressions['glyph_u_cmp_'+field]=f'({expression}) != 0' if field=='automatic' else expression
    for field,shift,width in [('method',0,2),('for_no_font',2,1),('len',3,4),('ch',7,25)]:
        expression=f'((*a0.cast::<glyph>()).u.glyphless.bits >> {shift}) & {2**width-1}'
        expressions['glyph_u_glyphless_'+field]=f'({expression}) != 0' if field=='for_no_font' else expression
    for field,shift,width in [('box',0,2),('underline',2,3),('use_box_color_for_shadows_p',5,1),('overline_p',6,1),('strike_through_p',7,1),('underline_defaulted_p',10,1),('overline_color_defaulted_p',11,1),('strike_through_color_defaulted_p',12,1),('underline_at_descent_line_p',14,1),('overstrike',21,1)]:
        expression=f'((*a0.cast::<face>()).bits >> {shift}) & {2**width-1}'
        expressions['face_'+field]=f'({expression}) != 0' if width==1 else expression
    for field,shift in [('must_be_updated_p',10),('pseudo_window_p',11)]:
        expressions['window_set_'+field]=f'(*a0.cast::<window>()).bits = ((*a0.cast::<window>()).bits & !(1 << {shift})) | ((a1 as u32) << {shift});'
    code=[];mapping=[]
    for name,decl in sorted(imports.items()):
        if name in existing:continue
        expression=expressions.get(name)
        if name.startswith('terminal_set_') and name.endswith('_hook') or name in ('terminal_set_free_pixmap','terminal_set_get_focus_frame','terminal_set_query_frame_background_color'):
            field=name.removeprefix('terminal_set_')
            expression=f'(*a0.cast::<terminal>()).{field} = core::mem::transmute(a1);'
        if name in ('font_driver_draw','font_driver_encode_char'):
            field=name.removeprefix('font_driver_');expression=f'core::mem::transmute::<_, *mut c_void>((*a0.cast::<font_driver>()).{field})'
        if expression is None:continue
        args=[]
        for i,arg in enumerate(split_args(decl['args'])):
            st=arg.split(':',1)[1].strip()
            ty=scalar.get(st)
            if ty is None:
                ty='*mut Lisp_Object' if st=='UnsafeMutablePointer<Lisp_Object>' else '*mut c_void'
            args.append(f'a{i}: {ty}')
        rt=scalar.get(decl['return'],'*mut c_void')
        if rt not in ('()','bool','Lisp_Object'):expression=f'({expression}) as {rt}'
        code += [f'#[unsafe(export_name="{name}")]', f'pub unsafe extern "C" fn view_{name}({", ".join(args)}) -> {rt} {{ {expression} }}']
        mapping.append({'name':name,'kind':'record-view','provider':'actual target header fields'})
    return code,mapping

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--out',type=Path,required=True);a=ap.parse_args()
    imports=swift_imports();existing=set();providers={};constants={};macros={}
    paths=list((ROOT/'femacs/src/include').glob('*.rs'))+list((ROOT/'femacs/lib/include').glob('*.rs'))+[ROOT/'femacs/lib/timespec.rs']
    for p in paths:
        text=p.read_text()
        for m in re.finditer(r'pub\s+(?:(?:unsafe|const)\s+)*(?:extern\s+"C"\s+)?fn\s+(\w+)\s*\(',text):
            end=balanced(text,m.end()-1);tail=text[end+1:];stop=re.search(r'[;{]',tail)
            if stop is None:continue
            rt=tail[:stop.start()].strip();rt=rt.removeprefix('->').strip() or '()'
            # Only already implemented non-generic inline bodies, not unresolved declarations.
            if tail[stop.start()]=='{' and '<' not in m[1]:
                providers.setdefault(m[1],(text[m.end():end],rt,str(p.relative_to(ROOT))))
        for m in re.finditer(r'macro_rules!\s+(\w+)',text):macros.setdefault(m[1],str(p.relative_to(ROOT)))
        for m in re.finditer(r'pub\s+const\s+(\w+)\s*:\s*([^=\n]+)=',text):constants.setdefault(m[1],str(p.relative_to(ROOT)))
    for p in (ROOT/'femacs').rglob('*.rs'):
        if not (p.parts[-2]=='include' or p.parent.name=='src'):continue
        text=p.read_text()
        existing.update(re.findall(r'#\[unsafe\(no_mangle\)\]\s*(?:pub\s+)?(?:unsafe\s+)?extern\s+"C"\s+fn\s+(\w+)',text))
    code=['// Generated real header providers; do not edit.','mod cocoa_header_exports {','use super::*;', 'use super::dispextern_unit_names::*;', 'use super::frame_face_accessors::*;', 'use super::frame_image_accessors::*;']
    mapping=[];unresolved=[]
    scalar={'Int':'isize','UInt':'usize','Int8':'i8','UInt8':'u8','Int16':'i16','UInt16':'u16','Int32':'i32','UInt32':'u32','Int64':'i64','UInt64':'u64','Bool':'bool','Double':'f64','Float':'f32','CGFloat':'f64'}
    record_code,record_mapping=record_exports(imports,existing)
    code+=record_code;mapping+=record_mapping
    existing.update(row['name'] for row in record_mapping)
    for name,decl in sorted(imports.items()):
        if name in existing:continue
        if name in providers:
            args,rt,source=providers[name];parsed=split_args(args)
            if len(parsed)!=len(split_args(decl['args'])):
                raise ValueError(f'{name}: Swift and Rust provider argument counts differ')
            if any(':' not in arg or '...' in arg for arg in parsed):unresolved.append(name);continue
            names=[arg.split(':',1)[0].strip().removeprefix('mut ') for arg in parsed]
            if any(not re.fullmatch(r'\w+',n) for n in names):unresolved.append(name);continue
            # Use the provider's actual signature and implementation, including its actual
            # pointer/record types. Do not infer layout from the Swift facade.
            code += [f'#[unsafe(export_name="{name}")]',f'pub unsafe extern "C" fn cocoa_{name}({args}) -> {rt} {{ {name}({", ".join(names)}) }}']
            mapping.append({'name':name,'provider':source,'kind':'inline'})
        elif name in constants and not decl['args'].strip() and decl['return'] in scalar:
            rt=scalar[decl['return']]
            expression=f'{name}' if rt=='bool' else f'{name} as {rt}'
            code += [f'#[unsafe(export_name="{name}")]',f'pub unsafe extern "C" fn cocoa_{name}() -> {rt} {{ {expression} }}']
            mapping.append({'name':name,'provider':constants[name],'kind':'constant'})
        elif name in macros and name.startswith(('FRAME_','FONT_','SET_FRAME_','NS_FACE_','MOUSE_HL_INFO','WINDOW_')):
            parsed=split_args(decl['args']);arguments=[];calls=[];good=True
            for i,arg in enumerate(parsed):
                if ':' not in arg:good=False;break
                label,swift_type=arg.split(':',1);swift_type=swift_type.strip()
                ident='a'+str(i)
                if swift_type=='Lisp_Object':rt='Lisp_Object';value=ident
                elif swift_type in scalar:rt=scalar[swift_type];value=ident
                elif swift_type=='UnsafePointer<CChar>?':rt='*const c_char';value=ident
                elif re.fullmatch(r'UnsafeMutablePointer<(font|frame|window)>\??',swift_type):rt='*mut '+re.search(r'<(\w+)>',swift_type)[1];value=ident
                elif swift_type=='UnsafeMutableRawPointer?':
                    record='face' if name.startswith('NS_FACE_') else 'window' if name.startswith('WINDOW_') else 'font' if name.startswith('FONT_') else 'frame'
                    rt='*mut '+record;value=ident
                else:good=False;break
                arguments.append(ident+': '+rt);calls.append(value)
            rt=scalar.get(decl['return'],{'Void':'()','Lisp_Object':'Lisp_Object','UnsafeMutableRawPointer?':'*mut core::ffi::c_void'}.get(decl['return']))
            if rt is None and (decl['return'].startswith('UnsafeMutablePointer<') or decl['return'] in ('NSView?','NSColor?','NSWindow?','NSObject?')):rt='*mut core::ffi::c_void'
            if not good or rt is None:unresolved.append(name);continue
            expression=f'{name}!({", ".join(calls)})'
            if rt not in ('()','Lisp_Object','bool'):expression=f'({expression}) as {rt}'
            code += [f'#[unsafe(export_name="{name}")]',f'pub unsafe extern "C" fn cocoa_{name}({", ".join(arguments)}) -> {rt} {{ {expression} }}']
            mapping.append({'name':name,'provider':macros[name],'kind':'macro'})
        else:unresolved.append(name)
    code.append('}')
    code.insert(1, '')
    if a.out.resolve().is_relative_to((ROOT/'emacs').resolve()):raise ValueError('reference source is read-only')
    a.out.parent.mkdir(parents=True,exist_ok=True);a.out.write_text('\n'.join(code)+'\n')
    a.out.with_suffix('.json').write_text(json.dumps({'providers':mapping,'not_reified':unresolved,'note':'Not-reified entries may be real core/Swift/SDK exports or require integration repair. The complete static link is authoritative; never stub these.'},indent=2)+'\n')
    print(f'generated {len(mapping)} actual header providers; {len(unresolved)} other imports await complete target link')
    return 0
if __name__=='__main__':raise SystemExit(main())
