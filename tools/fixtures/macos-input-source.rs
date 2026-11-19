//! Acceptance-only input source controller. Restore the original ID after testing.
use std::{env, ffi::{c_char,c_void,CStr,CString}, ptr, thread, time::Duration};
#[link(name="Carbon",kind="framework")]
unsafe extern "C" {
    fn TISCreateInputSourceList(properties:*const c_void,all:bool)->*const c_void;
    fn TISCopyCurrentKeyboardInputSource()->*const c_void;
    fn TISGetInputSourceProperty(source:*const c_void,key:*const c_void)->*const c_void;
    fn TISSelectInputSource(source:*const c_void)->i32;
    static kTISPropertyInputSourceID:*const c_void;
    static kTISPropertyInputSourceIsSelectCapable:*const c_void;
    static kTISPropertyInputSourceIsEnabled:*const c_void;
}
#[link(name="CoreFoundation",kind="framework")]
unsafe extern "C" {
    fn CFArrayGetCount(array:*const c_void)->isize;
    fn CFArrayGetValueAtIndex(array:*const c_void,index:isize)->*const c_void;
    fn CFStringGetCString(s:*const c_void,buffer:*mut c_char,size:isize,encoding:u32)->bool;
    fn CFStringCreateWithCString(a:*const c_void,s:*const c_char,encoding:u32)->*const c_void;
    fn CFBooleanGetValue(v:*const c_void)->bool;
    fn CFRelease(v:*const c_void);
    static kCFBooleanTrue:*const c_void;
}
#[link(name="ApplicationServices",kind="framework")]
unsafe extern "C" {
    fn AXUIElementCreateApplication(pid:i32)->*const c_void;
    fn AXUIElementSetAttributeValue(element:*const c_void,attribute:*const c_void,value:*const c_void)->i32;
    fn AXUIElementCopyAttributeValue(element:*const c_void,attribute:*const c_void,value:*mut *const c_void)->i32;
}
unsafe fn identifier(source:*const c_void)->Result<String,String> {
    if source.is_null(){return Err("no current input source".into());}
    let value=unsafe{TISGetInputSourceProperty(source,kTISPropertyInputSourceID)};
    let mut buffer=[0i8;2048];
    if value.is_null() || !unsafe{CFStringGetCString(value,buffer.as_mut_ptr(),buffer.len() as isize,0x08000100)} {return Err("invalid input source ID".into());}
    Ok(unsafe{CStr::from_ptr(buffer.as_ptr())}.to_string_lossy().into_owned())
}
fn main()->Result<(),String>{
    let args:Vec<String>=env::args().collect();
    let command=args.get(1).map(String::as_str).unwrap_or("");
    unsafe {
        if command=="current" {
            let source=TISCopyCurrentKeyboardInputSource();let id=identifier(source);if !source.is_null(){CFRelease(source)};println!("{}",id?);return Ok(());
        }
        if command=="status" {
            let pid:i32=args.get(2).ok_or("status needs PID")?.parse().map_err(|_|"invalid PID")?;
            if pid<=0{return Err("invalid PID".into())}
            let current=TISCopyCurrentKeyboardInputSource();let id=identifier(current);if !current.is_null(){CFRelease(current)}
            let id=id?;let app=AXUIElementCreateApplication(pid);
            let mut values=Vec::new();
            for attribute in ["AXFrontmost","AXFocusedWindow"] {
                let name=CString::new(attribute).unwrap();
                let key=CFStringCreateWithCString(ptr::null(),name.as_ptr(),0x08000100);
                let mut value=ptr::null();let code=AXUIElementCopyAttributeValue(app,key,&mut value);CFRelease(key);
                let present=code==0 && !value.is_null();
                let yes=present && (attribute!="AXFrontmost" || CFBooleanGetValue(value));
                if !value.is_null(){CFRelease(value)};values.push(yes);
            }
            CFRelease(app);
            // Input-source IDs use reverse-DNS syntax; quote with JSON's escaping rules.
            let mut quoted=String::from("\"");
            for c in id.chars(){match c{'"'=>quoted.push_str("\\\""),'\\'=>quoted.push_str("\\\\"),c if c.is_control()=>quoted.push_str(&format!("\\u{:04x}",c as u32)),c=>quoted.push(c)}}
            quoted.push('"');
            println!("{{\"source\":{},\"frontmost\":{},\"focused_window\":{}}}",quoted,values[0],values[1]);return Ok(());
        }
        if command=="activate" {
            let pid:i32=args.get(2).ok_or("activate needs PID")?.parse().map_err(|_|"invalid PID")?;
            if pid<=0{return Err("invalid PID".into())}
            let app=AXUIElementCreateApplication(pid);
            let key=CString::new("AXFrontmost").unwrap();
            let attribute=CFStringCreateWithCString(ptr::null(),key.as_ptr(),0x08000100);
            let code=AXUIElementSetAttributeValue(app,attribute,kCFBooleanTrue);
            CFRelease(attribute);CFRelease(app);
            if code!=0{return Err(format!("cannot activate test app: {code}"))}
            thread::sleep(Duration::from_millis(200));
            println!("{pid}");return Ok(());
        }
        if command!="list" && command!="select" {return Err("usage: macos-input-source list|current|status PID|activate PID|select ID [PID]".into());}
        let list=TISCreateInputSourceList(ptr::null(),false);
        if list.is_null(){return Err("input source list unavailable".into());}
        let result=(||->Result<(),String>{
            for i in 0..CFArrayGetCount(list){
                let source=CFArrayGetValueAtIndex(list,i);
                let capable=TISGetInputSourceProperty(source,kTISPropertyInputSourceIsSelectCapable);
                let enabled=TISGetInputSourceProperty(source,kTISPropertyInputSourceIsEnabled);
                if capable.is_null() || enabled.is_null() || !CFBooleanGetValue(capable) || !CFBooleanGetValue(enabled){continue}
                let id=identifier(source)?;
                if command=="list" {println!("{id}");continue}
                if args.get(2)!=Some(&id){continue}
                if let Some(pid)=args.get(3){
                    let pid:i32=pid.parse().map_err(|_|"invalid PID")?;if pid<=0{return Err("invalid PID".into())}
                    let app=AXUIElementCreateApplication(pid);
                    let key=CString::new("AXFrontmost").unwrap();
                    let attribute=CFStringCreateWithCString(ptr::null(),key.as_ptr(),0x08000100);
                    let code=AXUIElementSetAttributeValue(app,attribute,kCFBooleanTrue);
                    CFRelease(attribute);CFRelease(app);
                    if code!=0{return Err(format!("cannot activate test app: {code}"))}
                    thread::sleep(Duration::from_millis(200));
                }
                let status=TISSelectInputSource(source);if status!=0{return Err(format!("TISSelectInputSource failed: {status}"))}
                thread::sleep(Duration::from_millis(200));
                let current=TISCopyCurrentKeyboardInputSource();let actual=identifier(current);if !current.is_null(){CFRelease(current)};
                if actual?!=id{return Err("input source did not become current".into())}
                println!("{id}");return Ok(())
            }
            if command=="list"{Ok(())}else{Err("requested enabled input source not found".into())}
        })();CFRelease(list);result
    }
}
