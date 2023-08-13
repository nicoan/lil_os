mod page_table_entry;
mod page_table_level;

use core::ops::{Index, IndexMut};

pub use page_table_entry::PageTableEntry;
pub use page_table_level::PageTableLevel;

const PAGE_TABLE_SIZE: usize = 512;

/// Represents a page table.
///
/// This is just a wrapper type that contains an array of page table entries.
#[repr(C)]
pub struct PageTable([PageTableEntry; PAGE_TABLE_SIZE]);

impl PageTable {
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.0.iter()
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PageTable {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
