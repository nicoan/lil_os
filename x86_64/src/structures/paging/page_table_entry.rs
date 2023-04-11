use core::fmt::Display;
use core::ops::Deref;

use crate::address::PhysicalMemoryAddress;

/// A page table entry
#[repr(transparent)]
#[derive(Debug)]
pub struct PageTableEntry(u64);

/// TODO:
/// Document with this:
/// https://wiki.osdev.org/images/4/41/64-bit_page_tables1.png
/// https://wiki.osdev.org/Paging
impl PageTableEntry {
    /// Checks if this entry is a used entry
    pub fn is_used(&self) -> bool {
        self.0 != 0
    }

    pub fn is_present(&self) -> bool {
        self.0 & 0x1 == 1
    }

    pub fn is_writable(&self) -> bool {
        self.0 & 0x2 >> 1 == 1
    }

    /// Returns the physical address pointed by this page table entry.
    ///
    /// The physical address is contained between bits 52..12.
    pub fn address(&self) -> PhysicalMemoryAddress {
        PhysicalMemoryAddress(self.0 & 0x000f_ffff_ffff_f000)
    }
}

impl Deref for PageTableEntry {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PageTableEntry - Physical Address: {:?}. Present: {} - Writable: {}",
            self.address(),
            self.is_present(),
            self.is_writable()
        )
    }
}
