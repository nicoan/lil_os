//! Abstractions for control registers
use core::arch::asm;

use crate::memory::address::PhysicalMemoryAddress;

/// The CR3 register contains the phisical address of the PML4 (Page Map Level 4, also known as
/// PDT - Page Directory Table).
///
/// The value is divided as it follows:
///
/// If CR4 PCIDE Flag is 0 then:
///  Bits
///  0  - 2  | Reserved
///  3       | PWT (Page-Level Write Through)
///  5       | PCD ( Page-Level Cache Disable)
///  6  - 11 | Reserved
///  12 - 63 | Physical Base Address of the PML4
///
/// If CR4 PCIDE Flag is 1 then:
///  0  - 11 | PCID (Process Context Identifier)
///  12 - 63 | Physical Base Address of the PML4
///
/// For more info:
/// https://en.wikipedia.org/wiki/Control_register#CR3
/// https://wiki.osdev.org/Paging#Page_Directory
/// https://wiki.osdev.org/CPU_Registers_x86-64#CR3
/// https://wiki.osdev.org/CPU_Registers_x86#CR3
pub struct Cr3;

impl Cr3 {
    // TODO: Also return the flags
    #[inline]
    pub fn read() -> PhysicalMemoryAddress {
        let mut value: u64;
        unsafe { asm!("mov {}, cr3 ", out(reg) value, options(nomem, nostack, preserves_flags)) }

        // Remove the flags from the physical address (the first 12 bits, 0 to 11). Bits 0-11 of
        // the physical base address are assumed to be 0, that is why we do not shift the address
        // 12 places to the right.
        value &= 0xffff_ffff_ffff_f000;

        PhysicalMemoryAddress::new(value)
    }
}
