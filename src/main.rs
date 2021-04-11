#![no_std]
#![no_main]

use core::panic::PanicInfo;
use woshd::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("What's up, gamers?");

    woshd::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); // new

    // unsafe {
    //     *(0xjfaljfjw as *mut u64) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    println!("Yeah!");

    loop {}
}

/// Panic handler - any kernel panics are routed to here.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Print the panic's contents to the VGA terminal.
    println!("{}", _info);

    // Loop forever - post-panic, we don't attempt recovery.
    loop {}
}