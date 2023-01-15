#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lil_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]
use lil_os::println;

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lil_os::tests::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
