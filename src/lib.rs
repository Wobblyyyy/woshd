#![no_std]
#![feature(abi_x86_interrupt)]

#[path = "interrupts.rs"]
pub mod interrupts;

#[path = "vga.rs"]
pub mod vga;

#[path = "gdt.rs"]
pub mod gdt;

pub fn init() {
    interrupts::init_idt();
    gdt::init_gdt();
}
