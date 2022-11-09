#![no_std]
#![cfg_attr(test, no_main)]
// Since we are in a non-standard environment, we should define our own test framework.
#![feature(custom_test_frameworks)]
// This is the entry-point to our test framework.
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Enable x86 interrupt ABI
#![feature(abi_x86_interrupt)]

pub mod arch;
pub mod drivers;
pub mod os_core;
pub mod tests;

// "Global scope" exports
pub use drivers::screen::text::PrintColor;

// Integration testing entry points and handlers.
// Here we define the custom test framework entrypoint and the panic handler. We need this
// functions declared here in lib.rs. Most of the test logic is contained in the test module.
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    tests::test_panic_handler(info)
}
