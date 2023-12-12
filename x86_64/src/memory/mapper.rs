use core::marker::PhantomData;

use super::{
    address::VirtualMemoryAddress,
    frame_allocator::FrameAllocator,
    paging::{
        frame::Frame,
        page::Page,
        page_size::{Size1GiB, Size2MiB, Size4KiB},
        page_table::{PageTable, PageTableEntry, PageTableEntryFlags, PageTableLevel},
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

macro_rules! impl_mapper_for_size {
    ($size:ty, $pt_levels:expr, $pt_levels_qty:literal) => {
        impl Mapper<$size> {
            /// Ordered levels that this mapper has to go through to get to the page table that actually
            /// points to the physical frame
            const PAGE_TABLE_LEVELS: [PageTableLevel; $pt_levels_qty] = $pt_levels;

            /// Maps a virtual page to a physical frame. If a table at any level do not exist,
            /// space is allocated to save the new table.
            ///
            /// # Safety
            /// This function is unsafe because the caller must asure that the Page is not mapped
            /// yet.
            pub unsafe fn map(
                &self,
                page: Page<$size>,
                frame: Frame<$size>,
                _allocator: impl FrameAllocator<$size>,
                flags: u64, // TODO: create a newtype or something like that here
            ) -> bool {
                // TODO: Allocate new page table if not present
                // At the moment we are ignoring the allocation of tables, we just want to use existing
                // page tables to map a frame

                // TODO: Generalize to all page sizes
                // Get the level 4 page table (PDPT)
                let mut next_page_table_physical_address = Cr3::read();

                // This is initialized here loop because rust complains otherwise
                let mut next_page_table: &mut PageTable = &mut *(self.physical_memory_offset
                    + next_page_table_physical_address)
                    .as_mut_ptr();
                let level_4_page_table_index =
                    page.get_page_table_index(Self::PAGE_TABLE_LEVELS[0]);
                next_page_table_physical_address =
                    next_page_table[level_4_page_table_index].address();

                // Transverse tables
                for page_table_level in &Self::PAGE_TABLE_LEVELS[1..] {
                    next_page_table = &mut *(self.physical_memory_offset
                        + next_page_table_physical_address)
                        .as_mut_ptr();

                    // TODO: If not allocated...
                    next_page_table_physical_address =
                        next_page_table[page.get_page_table_index(*page_table_level)].address();
                }

                // At this level we must have reached the last PageTable (level 1), we need to write this
                // page entry to point the frame
                let last_level_page_table_index =
                    page.get_page_table_index(Self::PAGE_TABLE_LEVELS[$pt_levels_qty - 1]);
                next_page_table[last_level_page_table_index] = PageTableEntry::new(
                    PageTableEntryFlags::PRESENT | flags,
                    frame.start_address(),
                );
                next_page_table[last_level_page_table_index].is_used()
            }
        }
    };
}

impl_mapper_for_size!(
    Size4KiB,
    [
        PageTableLevel::Level4,
        PageTableLevel::Level3,
        PageTableLevel::Level2,
        PageTableLevel::Level1,
    ],
    4
);

impl_mapper_for_size!(
    Size2MiB,
    [
        PageTableLevel::Level4,
        PageTableLevel::Level3,
        PageTableLevel::Level2,
    ],
    3
);

impl_mapper_for_size!(
    Size1GiB,
    [PageTableLevel::Level4, PageTableLevel::Level3,],
    2
);
