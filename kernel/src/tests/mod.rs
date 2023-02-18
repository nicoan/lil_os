//! This file sets up a basic testing framework. The tests runner is defined in the standard
//! library. Since we are in a no_std environment we have to define our own test framework.
//!
//! This framework has three basic components:
//! - test_runner function: Grabs all the tests marked with the #[test_case] macro and executes
//! them.
//! - panic: A panic handler for the testing environment. This function prints the fail message.
//! - Testable trait: A trait used to print test information such as the test name and if it
//! succeeded.
//!
//! Everything is "printed" in a serial port. This is because we want the tests to appear in the
//! host OS console and not directly in QEMU. Writing directly in the serial port outputs the data
//! in the host console (this is configured in Cargo.toml, package.metadata.bootimage section.
// #![cfg(test)]
pub mod idt;
mod qemu;

use crate::{hlt_loop, serial_print, serial_println};
use core::any::type_name;
use core::panic::PanicInfo;

pub use crate::tests::qemu::{exit_qemu, QemuExitCode};

/// This trait is used to print the tests statements automatically. The trick is to print the
/// messages directly in the run function, and implement this trait for every T: Fn()
pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        // Print the function name through the type_name function
        serial_print!("{}... ", type_name::<T>());

        // Execute the test
        self();

        // In case it does not panic, we print an "[OK]" message at the end of the test name. If
        // its panic the fail printing is managed in the "panic" function
        serial_println!("[\x1b[1;32mOK\x1b[0m]");
    }
}

/// This function is called on panic. This panic handler prints the messages of the failed tests
/// directly in the serial port.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[\x1b[1;31mFAILED\x1b[0m]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Grabs all the tests marked with #[test_case] macro and executes them.
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests...", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}
