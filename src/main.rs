#![no_std]
#![no_main]
//! Since we are in a non-standard environment, we should define our own test framework.
#![feature(custom_test_frameworks)]
//! This is the entry-point to our test framework.
#![test_runner(lil_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

/// Entrypoint of our OS
#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    use lil_os::arch::x86_64::initialize_x86_64_arch;
    use lil_os::os_core::messages::init_with_message;

    init_with_message("x86_64 architecture", initialize_x86_64_arch);

    #[allow(clippy::empty_loop)]
    loop {
        // Halts the CPU until the next interrupt hits. This prevents the CPU to spin endessly
        // and waste cycles doing nothing.
        x86_64::instructions::hlt();
    }
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use lil_os::println;
    println!("{}", info);
    #[allow(clippy::empty_loop)]
    loop {
        // Halts the CPU after the panic
        x86_64::instructions::hlt();
    }
}

// Unit testing entry points and handlers.
// Here we define the custom test framework entrypoint and the panic handler. We need this
// functions declared here in main.rs. Most of the test logic is contained in the test module.

/// Custom test framework entry point.
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Custom test framework panic handler.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lil_os::tests::test_panic_handler(info)
}
