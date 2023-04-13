use crate::{
    address::{PageTableLevel, PhysicalMemoryAddress, VirtualMemoryAddress},
    paging::PageTable,
    registers::control::Cr3,
};

pub struct Translator {
    physical_memory_offset: VirtualMemoryAddress,
}

impl Translator {
    pub const fn new(physical_memory_offset: VirtualMemoryAddress) -> Self {
        Self {
            physical_memory_offset,
        }
    }

    pub unsafe fn translate_address(
        &self,
        address: VirtualMemoryAddress,
    ) -> Option<PhysicalMemoryAddress> {
        let tables_indexes = [
            address.get_page_table_index(PageTableLevel::Level4),
            address.get_page_table_index(PageTableLevel::Level3),
            address.get_page_table_index(PageTableLevel::Level2),
            address.get_page_table_index(PageTableLevel::Level1),
        ];

        // Get the physical address where the first page table is located (also called PDPT, Page
        // Directory Pointer Table)
        let mut next_page_table_physical_address = Cr3::read();
        // Go through all the page tables until we reach the last one
        for table_index in tables_indexes {
            // Get the next level page table virtual address from the physical address plus the offset.
            let next_page_table_virtual_address =
                self.physical_memory_offset + next_page_table_physical_address;
            let next_table: &PageTable = &*next_page_table_virtual_address.as_mut_ptr();

            // Get the physical address from the next page table or frame we are going to process
            next_page_table_physical_address = next_table[table_index].address();

            // If the page is huge, we shortcircuit the execution, because we already point to a
            // physical frame where actual data is saved and not the next level page table.
            // - If we transverse 2 levels, we reach a 1GB page
            // - If we transverse 3 levels, we reach a 2MB page
            if next_table[table_index].is_huge() {
                return Some(PhysicalMemoryAddress::new(
                    next_page_table_physical_address.0 + address.get_page_offset() as u64,
                ));
            }
        }

        // Once we reach the level 1, in next_page_table_physical_address we have the actual frame
        // address. To get the actual address we need to add the offset from the virtual address.
        Some(PhysicalMemoryAddress::new(
            next_page_table_physical_address.0 + address.get_page_offset() as u64,
        ))
    }
}
