//! This module contains all the initialization code for the x86_64 architecture.
mod gdt;
mod idt;
mod interrupts;

use crate::os_core::synchronization::spinlock::Mutex;
use x86_64_custom::interrupts::IBMPcAt8259;

// TODO The idea here is to define a trait that abstacts away the interruption habdling either if
// it is with the IBM PC/AT 8259 Architecture or with the APIC interface.
pub static PICS: Mutex<IBMPcAt8259> = Mutex::new(IBMPcAt8259::new());

/// Loads the x86 system tables
fn load_tables() {}

/// Initializes the x86_64 arch
pub fn initialize_x86_64_arch() {
    // Initialize system tables
    gdt::load_gdt();
    idt::load_idt();

    // Initialize interrupts
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); // TODO: Write our own asm code for this
}
