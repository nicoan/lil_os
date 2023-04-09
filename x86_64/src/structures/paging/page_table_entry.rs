use core::fmt::Display;

/// A page table entry
#[repr(transparent)]
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
}

impl Display for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PageTableEntry - 0x{:016x}. Present: {} - Writable: {}",
            self.0,
            self.is_present(),
            self.is_writable()
        )
    }
}
