pub mod allocator;
pub mod frame_allocator;
pub mod volatile;

/*
// TODO: BELOW IS JUST FOR DEBUGGING, REMOVE!
use x86_64_custom::{
    memory::{
        address::{PhysicalMemoryAddress, VirtualMemoryAddress},
        paging::page_table::{PageTable, PageTableLevel},
    },
    registers::control::Cr3,
};

use crate::println;

pub struct Translator {
    /// This is the virtual memory offset where the page tables are allocated.
    physical_memory_offset: VirtualMemoryAddress,
}

impl Translator {
    pub const fn new(physical_memory_offset: VirtualMemoryAddress) -> Self {
        Self {
            physical_memory_offset,
        }
    }

    /// Translates a virtual address into a physical address.
    ///
    /// This function performs the translation going through all four page tables until it reaches the
    /// physical address
    ///
    /// # Safety
    /// Caller must be sure that the virtual address can be mapped to a physical frame.
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
        for (transversed_level, table_index) in tables_indexes.iter().enumerate() {
            // Get the next level page table virtual address from the physical address plus the offset.
            let next_page_table_virtual_address =
                self.physical_memory_offset + next_page_table_physical_address;
            let next_table: &PageTable = &*next_page_table_virtual_address.as_mut_ptr();

            /*
            println!(
                "Next PT physical address: {:?}",
                next_table[*table_index].address()
            );
            */
            if transversed_level == 3 {
                println!("Level: {transversed_level:?} - Table index {table_index:?}");
                println!(
                    "PageTable entry at index {table_index}: {:?}",
                    next_table[*table_index]
                );
            }
            // If the entry is not present then we return None
            if !next_table[*table_index].is_present() {
                return None;
            }

            // Get the physical address from the next page table or frame we are going to process
            next_page_table_physical_address = next_table[*table_index].address();

            // If the page is huge, we shortcircuit the execution, because we already point to a
            // physical frame where actual data is saved and not the next level page table.
            // - If we transverse 2 levels, we reach a 1GB page (Cr3 -> PT Lvl4 -> PT Lvl3)
            // - If we transverse 3 levels, we reach a 2MB page (Cr3 -> PT Lvl4 -> PT Lvl3 -> PT
            // Lvl2)
            if next_table[*table_index].is_huge() {
                // If we transversed two levels we are in the level 3 page table, meaning that we
                // have a 1GB page mapped to phisical memory. In this case bits 30 to 1 are used as
                // offset
                // TODO: Move this to get_offset in VirtualMemoryAddress
                let offset = if transversed_level == 2 {
                    address.as_u64() & 0x3fffffff
                }
                // If we transversed two levels we are in the level 2 page table, meaning that we
                // have a 2MB page mapped to phisical memory. In this case bits 21 to 1 are used as
                // offset
                else if transversed_level == 3 {
                    address.as_u64() & 0x1fffff
                } else {
                    // TODO: Return a result and remove this panic!
                    panic!(
                        "Found a huge page in Page Table level {}",
                        4 - transversed_level
                    );
                };

                return Some(PhysicalMemoryAddress::new(
                    next_page_table_physical_address.0 + offset,
                ));
            }
        }

        println!("end");

        // Once we reach the level 1, in next_page_table_physical_address we have the actual frame
        // address. To get the actual address we need to add the offset from the virtual address.
        Some(PhysicalMemoryAddress::new(
            next_page_table_physical_address.0 + address.get_page_offset() as u64,
        ))
    }
}
*/
