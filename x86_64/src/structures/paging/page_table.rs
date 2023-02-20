use super::PageTableEntry;
use core::ops::{Index, IndexMut};

const PAGE_TABLE_SIZE: usize = 512;

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
