use core::marker::PhantomData;

use super::{
    address::VirtualMemoryAddress,
    frame_allocator::FrameAllocator,
    paging::{
        frame::Frame,
        page::Page,
        page_size::PageSize4KiB,
        page_table::{PageTable, PageTableLevel},
    },
};
use crate::{memory::paging::page_size::PageSize, registers::control::Cr3};

/// Structure that maps a virtual address to a memory frame
pub struct Mapper<PS: PageSize> {
    physical_memory_offset: VirtualMemoryAddress,
    phantom: PhantomData<PS>,
}

// https://docs.rs/x86_64/latest/src/x86_64/structures/paging/mapper/mapped_page_table.rs.html#52

impl<PS: PageSize> Mapper<PS> {
    pub fn new(physical_memory_offset: VirtualMemoryAddress) -> Self {
        Self {
            physical_memory_offset,
            phantom: PhantomData,
        }
    }
}

impl Mapper<PageSize4KiB> {
    pub unsafe fn map(
        &self,
        page: Page<PageSize4KiB>,
        frame: Frame<PageSize4KiB>,
        allocator: impl FrameAllocator<PageSize4KiB>,
        page_table_entry_flags: u64, // TODO: create a newtype or something like that here
    ) -> bool {
        let tables_indexes = [
            page.get_page_table_index(PageTableLevel::Level4),
            page.get_page_table_index(PageTableLevel::Level3),
            page.get_page_table_index(PageTableLevel::Level2),
            page.get_page_table_index(PageTableLevel::Level1),
        ];
        // TODO: Allocate new page table if not present
        // At the moment we are ignoring the allocation of tables, we just want to use existing
        // page tables to map a frame

        // TODO: Generalize to all page sizes
        // Get the level 4 page table (PDPT)
        let mut next_page_table_physical_address = Cr3::read();

        // This is initialized here and rewritten in the for loop because rust complains otherwise
        let mut next_page_table: &PageTable =
            &*(self.physical_memory_offset + next_page_table_physical_address).as_mut_ptr();
        // Transverse tables
        for (_transversed_level, table_index) in tables_indexes.iter().enumerate() {
            next_page_table =
                &*(self.physical_memory_offset + next_page_table_physical_address).as_mut_ptr();

            next_page_table_physical_address = next_page_table[*table_index].address();
        }

        // At this level we must have reached the last PageTable (level 1), we need to write this
        // page entry to point the frame
        next_page_table[tables_indexes[3]].is_used()
    }
}
