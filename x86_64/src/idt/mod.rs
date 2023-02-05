//! Implementation of the Interrupt Descriptor Table for x86_64
//!
//! For more information:
//! https://os.phil-opp.com/cpu-exceptions/
//! https://wiki.osdev.org/IDT
mod entry;
mod handlers;
mod table;

pub use handlers::InterruptStackFrame;
pub use table::InterruptDescriptorTable;

use lazy_static::lazy_static;
