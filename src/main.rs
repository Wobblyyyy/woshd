#![no_std]
#![no_main]

use core::panic::PanicInfo;
use woshd::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("What the fuck is up, gamers?");
    println!("This is a really long run-on sentence designed to test what happens when newlines are used mid-sentence. Hmm. I wonder.");
    woshd::init(); // new

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); // new

    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    println!("Yeah!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}