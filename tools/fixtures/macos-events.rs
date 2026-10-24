//! Native keyboard-event acceptance sender; never linked into the editor.
use std::{env, fs, thread, time::Duration};
use std::ffi::c_void;
#[link(name="CoreGraphics", kind="framework")]
unsafe extern "C" {
    fn CGEventSourceCreate(state: i32) -> *mut c_void;
    fn CGEventCreateKeyboardEvent(source: *mut c_void, keycode: u16, down: bool) -> *mut c_void;
    fn CGEventSetFlags(event: *mut c_void, flags: u64);
    fn CGEventSetType(event: *mut c_void, event_type: u32);
    fn CGEventKeyboardSetUnicodeString(event: *mut c_void, length: usize, units: *const u16);
    fn CGEventPostToPid(pid: i32, event: *mut c_void);
}
#[link(name="ApplicationServices", kind="framework")]
unsafe extern "C" { fn AXIsProcessTrusted() -> bool; }
#[link(name="CoreFoundation", kind="framework")]
unsafe extern "C" { fn CFRelease(object: *mut c_void); }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len()!=3 { return Err("usage: macos-events PID strokes.tsv (keycode TAB flags TAB optional text)".into()); }
    let pid: i32=args[1].parse()?;
    if pid<=0 { return Err("PID must identify the acceptance editor".into()); }
    let text=fs::read_to_string(&args[2])?;
    if text.len()>1_048_576 { return Err("oversized stroke fixture".into()); }
    unsafe {
        if !AXIsProcessTrusted() {
            eprintln!("Native keyboard acceptance requires Accessibility permission for the sender.");
            std::process::exit(77);
        }
        let source=CGEventSourceCreate(1);
        if source.is_null() { return Err("CGEventSourceCreate failed".into()); }
        let result=(|| -> Result<(),Box<dyn std::error::Error>> {
            for line in text.lines().filter(|l| !l.is_empty()) {
                if line=="CYCLE_WINDOW" {
                    // Represent the complete native Command-grave chord,
                    // including modifier changes across the window transition.
                    let events:[(u16,bool,u32,u64);4]=[
                        (55,true,12,1<<20),(50,true,10,1<<20),
                        (50,false,11,1<<20),(55,false,12,0)];
                    for (key,down,kind,flags) in events {
                        let event=CGEventCreateKeyboardEvent(source,key,down);
                        if event.is_null(){return Err("window cycle event failed".into())}
                        CGEventSetType(event,kind);CGEventSetFlags(event,flags);
                        CGEventPostToPid(pid,event);CFRelease(event);
                        thread::sleep(Duration::from_millis(35));
                    }
                    continue;
                }
                let fields: Vec<&str>=line.splitn(3,'\t').collect();
                if fields.len()<2 { return Err("stroke needs keycode and flags".into()); }
                let keycode: u16=fields[0].parse()?;let flags: u64=fields[1].parse()?;
                let units: Option<Vec<u16>>=fields.get(2).map(|s| s.encode_utf16().collect());
                for down in [true,false] {
                    let event=CGEventCreateKeyboardEvent(source,keycode,down);
                    if event.is_null() { return Err("CGEventCreateKeyboardEvent failed".into()); }
                    CGEventSetFlags(event,flags);
                    if let Some(units)=&units { CGEventKeyboardSetUnicodeString(event,units.len(),units.as_ptr()); }
                    CGEventPostToPid(pid,event);CFRelease(event);
                }
                thread::sleep(Duration::from_millis(35));
            }
            Ok(())
        })();
        CFRelease(source);result
    }
}
