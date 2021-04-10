#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"This is a much longer string.";

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_string();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}