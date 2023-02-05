#![no_std]
#![cfg_attr(test, no_main)]
// Since we are in a non-standard environment, we should define our own test framework.
#![feature(custom_test_frameworks)]
// This is the entry-point to our test framework.
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Enable x86 interrupt ABI
#![feature(abi_x86_interrupt)]

pub mod address;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod privilege;
pub mod registers;

use crate::interrupts::{keyboard_handler, timer_handler};

use self::interrupts::{handlers::HardwareInterruptHandlers, PICS};

/// Loads the x86 system tables
fn load_tables() {
    gdt::load_gdt();
    idt::load_idt();
}

/// Initializes the x86_64 arch
pub fn initialize_x86_64_arch() {
    // Initialize system tables
    load_tables();

    // Set hardware interrupts

    // Initialize interrupts
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); // TODO: Write our own asm code for this
}
