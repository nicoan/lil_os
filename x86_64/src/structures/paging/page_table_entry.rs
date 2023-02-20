/// A page table entry
#[repr(transparent)]
#[derive(Debug)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    /// Checks if this entry is a used entry
    pub fn is_used(&self) -> bool {
        self.0 != 0
    }
}
