#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link_section = ".text._start"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}