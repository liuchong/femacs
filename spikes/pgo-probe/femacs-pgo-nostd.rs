#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let mut x = 0u64;
    let mut i = 0u64;
    while i < 10_000 { x = x.wrapping_add(i); i += 1; }
    if x == 49_995_000 { 0 } else { 1 }
}
