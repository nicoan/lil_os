//! Interrupt descriptor table initialization
use super::interrupts::{
    hardware::{keyboard_interrupt_handler, timer_interrupt_handler},
    software::{breakpoint_handler, divide_by_zero_handler, double_fault_handler},
};
use lazy_static::lazy_static;
use x86_64_custom::{
    gdt::DOUBLE_FAULT_IST_INDEX, idt::InterruptDescriptorTable, interrupts::InterruptIndex,
};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Setup software interrupts
        idt.breakpoint.set_handler_function(breakpoint_handler);
        idt.divide_by_zero
            .set_handler_function(divide_by_zero_handler);
        idt.double_fault
            .set_handler_function(double_fault_handler)
            .set_stack_index(DOUBLE_FAULT_IST_INDEX);

        // Setup hardware interrupts
        idt[InterruptIndex::Timer.as_usize()].set_handler_function(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_function(keyboard_interrupt_handler);

        idt
    };
}

pub(crate) fn load_idt() {
    IDT.load();
}
