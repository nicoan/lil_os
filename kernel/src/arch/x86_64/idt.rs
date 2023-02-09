//! Interrupt descriptor table initialization
use super::interrupts::{keyboard_interrupt_handler, timer_interrupt_handler};
use lazy_static::lazy_static;
use x86_64_custom::{idt::InterruptDescriptorTable, interrupts::InterruptIndex};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.init();

        // Setup hardware interrupts
        idt[InterruptIndex::Timer.as_usize()].set_handler_function(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_function(keyboard_interrupt_handler);

        idt
    };
}

pub(crate) fn load_idt() {
    IDT.load();
}
