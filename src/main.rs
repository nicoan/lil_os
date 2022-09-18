#![no_std]
#![no_main]
//! Since we are in a non-standard environment, we should define our own test framework.
#![feature(custom_test_frameworks)]
//! This is the entry-point to our test framework.
#![test_runner(lil_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use lil_os::{println, PrintColor};

/// Entrypoint of our OS
#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    println!([PrintColor::Yellow], "Hello world 1!");
    println!([PrintColor::Cyan, PrintColor::Blue], "Hello world 2!",);
    println!([PrintColor::Brown, PrintColor::Cyan], "Hello world 3!",);
    println!(
        [PrintColor::Cyan],
        "The numbers are {} and {}",
        42,
        1.0 / 3.0
    );
    println!("The numbers are {} and {}", 42, 1.0 / 3.0);

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    #[allow(clippy::empty_loop)]
    loop {}
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
