mod page_table;
mod page_table_entry;
mod translator;

pub use page_table::PageTable;
pub use page_table_entry::PageTableEntry;
pub use translator::Translator;

use crate::{
    address::{PageTableLevel, PhysicalMemoryAddress, VirtualMemoryAddress},
    registers::control::Cr3,
};

// https://docs.rs/x86_64/latest/src/x86_64/structures/paging/mapper/mapped_page_table.rs.html#530
// https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2003/cc736309(v=ws.10)?redirectedfrom=MSDN
// https://wiki.osdev.org/Physical_Address_Extension
// https://www.iaik.tugraz.at/teaching/materials/os/tutorials/paging-on-intel-x86-64/
// https://medium.com/@geri.bod/pae-paging-memory-mapping-on-x86-8e8ba0879c5
// TODO: Translate huge pages
/// Translates a virtual address into a physical address.
///
/// This function performs the translation going through all four page tables until it reaches the
/// physical address
///
/// # Safety
/// Caller must be sure that the virtual address can be mapped to a physical frame.
pub unsafe fn translate_address(
    address: VirtualMemoryAddress,
    physical_memory_offset: VirtualMemoryAddress,
) -> Option<PhysicalMemoryAddress> {
    let tables_indexes = [
        address.get_page_table_index(PageTableLevel::Level4),
        address.get_page_table_index(PageTableLevel::Level3),
        address.get_page_table_index(PageTableLevel::Level2),
        address.get_page_table_index(PageTableLevel::Level1),
    ];

    let mut next_page_table_physical_address = Cr3::read();
    // Go through all the page tables until we reach the last one
    for table_index in tables_indexes {
        // Get the next level page table virtual address from the physical address plus the offset.
        let next_page_table_virtual_address =
            physical_memory_offset + next_page_table_physical_address;
        let next_table: &PageTable = &*next_page_table_virtual_address.as_mut_ptr();

        // Get the physical address from the next page table we are going to process
        next_page_table_physical_address = next_table[table_index].address();
    }

    // Once we reach the level 1, in next_page_table_physical_address we have the actual frame
    // address. To get the actual address we need to add the offset from the virtual address.
    Some(PhysicalMemoryAddress::new(
        next_page_table_physical_address.0 + address.get_page_offset() as u64,
    ))
}
