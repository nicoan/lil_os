//! This module contains all the initialization code for the x86_64 architecture.
mod gdt;
mod idt;
mod interrupts;
mod paging;

use crate::synchronization::spinlock::Mutex;
use x86_64_custom::interrupts::IBMPcAt8259;
use x86_64_custom::memory::address::VirtualMemoryAddress;
use x86_64_custom::memory::Translator;

pub use paging::TRANSLATOR;
// TODO The idea here is to define a trait that abstacts away the interruption habdling either if
// it is with the IBM PC/AT 8259 Architecture or with the APIC interface.
pub static PICS: Mutex<IBMPcAt8259> = Mutex::new(IBMPcAt8259::new());

/// Initializes the x86_64 arch
pub fn initialize_x86_64_arch(physical_memory_offset: VirtualMemoryAddress) {
    // Initialize system tables
    gdt::load_gdt();
    idt::load_idt();

    // Initialize interrupts
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); // TODO: Write our own asm code for this

    // Setup paging translation offset
    // TODO: we are reassigning a static mut, check if we can do this in some other way
    unsafe { TRANSLATOR = Translator::new(physical_memory_offset) }
}

// NOTE: For debugging
// use crate::memory::Translator;
