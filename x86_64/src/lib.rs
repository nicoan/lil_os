#![no_std]
#![cfg_attr(test, no_main)]
// Since we are in a non-standard environment, we should define our own test framework.
#![feature(custom_test_frameworks)]
// This is the entry-point to our test framework.
// #![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Enable x86 interrupt ABI
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod memory;
pub mod privilege;
pub mod registers;
