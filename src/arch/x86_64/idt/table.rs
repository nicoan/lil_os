//! Interrupt Description Table for x86_64
//!
//! This module contains a representation of the entire IDT.
use super::{
    entry::Entry,
    handlers::{
        breakpoint_handler, divide_by_zero_handler, double_fault_handler, HandlerFunc,
        HandlerFuncWithErrCode, HandlerFuncWithErrCodeDiverging, PageFaultHandlerFunc,
    },
};
use core::arch::asm;

/// Interrupt Descriptor Table
///
/// This structure represents the entire Interrupt Descriptor Tabled used by the x86 architecture
/// to handle CPU exceptions. Every entry of this table represents a single handler function.
///
/// This structure is aligned by 16 bytes because all the table (entries)[super::entry::Entry] are
/// of 16 bytes of size.
#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    pub divide_by_zero: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<HandlerFuncWithErrCodeDiverging>,
    pub coprocessor_segment_overrun: Entry<HandlerFunc>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    pub reserved_1: Entry<HandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<HandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    pub reserved_2: [Entry<HandlerFunc>; 9],
    pub security_exception: Entry<HandlerFunc>,
    pub reserved_3: Entry<HandlerFunc>,
    pub interrupts: [Entry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    /// Creates a new IDT with all the entries missing.
    #[inline]
    pub const fn new() -> Self {
        Self {
            divide_by_zero: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved_1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            reserved_2: [Entry::missing(); 9],
            security_exception: Entry::missing(),
            reserved_3: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    /// Loads the IDT into the IDT register.
    ///
    /// # Safety
    ///
    /// We need this table to be valid for the whole execution of the OS. That is why its reference
    /// is marked with the lifetime 'static, to prevent shorter-lived IDTs references and
    /// use-after-free bugs.
    pub fn load(&'static self) {
        let pointer = IDTDescriptor {
            limit: (core::mem::size_of::<InterruptDescriptorTable>() - 1) as u16,
            base: (self as *const _) as u64,
        };

        unsafe { asm!("lidt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags)) }
    }

    /// Initializes the IDT.
    ///
    /// Sets up all the handler functions.
    pub fn init(&mut self) {
        self.breakpoint.set_handler_function(breakpoint_handler);
        self.divide_by_zero
            .set_handler_function(divide_by_zero_handler);
        self.double_fault.set_handler_function(double_fault_handler);
    }
}

// The location of the IDT is kept in the IDT register. The named register contains a pointer to
// this structure, which contains the size of the table and the pointer to the IDT address.
//
// For more information:
// https://wiki.osdev.org/Interrupt_Descriptor_Table#IDTR
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct IDTDescriptor {
    /// One less than the size of the IDT in bytes.
    limit: u16,

    ///  The linear address of the Interrupt Descriptor Table (not the physical address, paging
    ///  applies).
    base: u64,
}
