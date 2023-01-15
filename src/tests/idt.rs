#![allow(clippy::empty_loop)]
use crate::arch::x86_64::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::arch::x86_64::initialize_x86_64_arch;
use crate::serial_print;
use crate::{
    serial_println,
    tests::{exit_qemu, QemuExitCode},
};

/// Tests if an interruption handler triggers.
///
/// The trigger is contained in the test() function.
pub fn test_handler_body(
    description: &str,
    idt: &'static InterruptDescriptorTable,
    test: impl Fn(),
) {
    serial_print!("{description}...\t");
    initialize_x86_64_arch();
    idt.load();

    test();

    panic!("Execution continued after the handler test");
}

/// Function for testing IDT hanlders
pub extern "x86-interrupt" fn test_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("[\x1b[1;32mOK\x1b[0m]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
