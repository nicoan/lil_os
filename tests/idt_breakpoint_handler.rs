#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(clippy::empty_loop)]
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use lil_os::{
    arch::x86_64::idt::InterruptDescriptorTable,
    tests::{
        idt::{test_handler, test_handler_body},
        test_panic_handler,
    },
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_function(test_handler);
        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_handler_body("idt::breakpoint_handler", &IDT, || unsafe {
        core::arch::asm!("int3", options(nomem, nostack));
    });

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
