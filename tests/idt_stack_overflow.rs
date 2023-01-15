#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
use core::format_args;
use core::panic::PanicInfo;
use lil_os::tests::{exit_qemu, test_panic_handler, QemuExitCode};
use lil_os::{arch::x86_64::initialize_x86_64_arch, os_core::volatile, serial_print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    initialize_x86_64_arch();

    // TODO: I cant trigger an stack overflow even with volatile :/
    // trigger a stack overflow
    // stack_overflow();

    // panic!("Execution continued after stack overflow");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[allow(unconditional_recursion, dead_code)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(&0).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
