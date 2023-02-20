//! TODO: Write about the paging solution chosen: Mapping the complete phisycal memory in the
//! virtual space with an offset.
use x86_64_custom::address::VirtualMemoryAddress;
use x86_64_custom::registers::control::Cr3;
use x86_64_custom::structures::paging::PageTable;

/// Returns a mutable reference to the active level 4 page table.
///
/// This function is unsafe because the caller must guarantee that the complete physical memory is
/// mapped to virtual memory at the passed `physical_memory_offset`. Also, this function must be only
/// called once to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn get_active_lvl4_page_table(
    physical_memory_offset: VirtualMemoryAddress,
) -> &'static mut PageTable {
    // Read the current physical address of the active level 4 page table.
    let page_table_physical_address = Cr3::read();

    // Add the physical memory offset to the address to create the virtual address of the table, so
    // we can access it (remember, in paging we can never access physical addresses directly)
    let page_table_virtual_address = physical_memory_offset + page_table_physical_address;

    // Create a pointer to the page table using its virtual address and cast it as PageTable
    let page_table_ptr: *mut PageTable = page_table_virtual_address.as_mut_ptr();

    // Return a mutable reference to it.
    &mut *page_table_ptr // unsafe
}
