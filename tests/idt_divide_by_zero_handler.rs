#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(clippy::empty_loop)]
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use lil_os::arch::x86_64::idt::InterruptDescriptorTable;
use lil_os::tests::idt::{test_handler, test_handler_body};
use lil_os::tests::test_panic_handler;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_by_zero.set_handler_function(test_handler);
        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_handler_body("idt::divide_by_zero", &IDT, || unsafe {
        core::arch::asm!("mov dx, 0; div dx");
    });

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
