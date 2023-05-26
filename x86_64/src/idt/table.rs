//! Interrupt Description Table for x86_64
//!
//! This module contains a representation of the entire IDT.
use crate::memory::address::VirtualMemoryAddress;

use super::{
    entry::Entry,
    handlers::{
        HandlerFunc, HandlerFuncWithErrCode, HandlerFuncWithErrCodeDiverging, PageFaultHandlerFunc,
    },
};
use core::{
    arch::asm,
    ops::{Index, IndexMut},
};

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
            base: VirtualMemoryAddress::new((self as *const _) as u64),
        };

        unsafe { asm!("lidt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags)) }
    }
}

// NOTE: Indexes implementations taken from
// https://docs.rs/x86_64/latest/src/x86_64/structures/idt.rs.html#536-598
impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<HandlerFunc>;

    /// Returns the IDT entry with the specified index.
    ///
    /// Panics if index is outside the IDT (i.e. greater than 255) or if the entry is an
    /// exception that pushes an error code (use the struct fields for accessing these entries).
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.divide_by_zero,
            1 => &self.debug,
            2 => &self.non_maskable_interrupt,
            3 => &self.breakpoint,
            4 => &self.overflow,
            5 => &self.bound_range_exceeded,
            6 => &self.invalid_opcode,
            7 => &self.device_not_available,
            9 => &self.coprocessor_segment_overrun,
            16 => &self.x87_floating_point,
            19 => &self.simd_floating_point,
            20 => &self.virtualization,
            i @ 32..=255 => &self.interrupts[i - 32],
            i @ 15 | i @ 31 | i @ 21..=28 => panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 29 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            }
            i @ 18 => panic!("entry {} is an diverging exception (must not return)", i),
            i => panic!("no entry with index {}", i),
        }
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    /// Returns a mutable reference to the IDT entry with the specified index.
    ///
    /// Panics if index is outside the IDT (i.e. greater than 255) or if the entry is an
    /// exception that pushes an error code (use the struct fields for accessing these entries).
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.divide_by_zero,
            1 => &mut self.debug,
            2 => &mut self.non_maskable_interrupt,
            3 => &mut self.breakpoint,
            4 => &mut self.overflow,
            5 => &mut self.bound_range_exceeded,
            6 => &mut self.invalid_opcode,
            7 => &mut self.device_not_available,
            9 => &mut self.coprocessor_segment_overrun,
            16 => &mut self.x87_floating_point,
            19 => &mut self.simd_floating_point,
            20 => &mut self.virtualization,
            i @ 32..=255 => &mut self.interrupts[i - 32],
            i @ 15 | i @ 31 | i @ 21..=28 => panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 29 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            }
            i @ 18 => panic!("entry {} is an diverging exception (must not return)", i),
            i => panic!("no entry with index {}", i),
        }
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
    base: VirtualMemoryAddress,
}
