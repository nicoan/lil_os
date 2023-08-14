use super::address::{PhysicalMemoryAddress, VirtualMemoryAddress};
use crate::memory::paging::page_size::PageSize;

/// Structure that maps a virtual address to a memory frame
pub struct Mapper {
    physical_memory_offset: VirtualMemoryAddress,
}

impl Mapper {
    pub fn new(physical_memory_offset: VirtualMemoryAddress) -> Self {
        Self {
            physical_memory_offset,
        }
    }
    /*
    pub fn map<PS: PageSize>(physical_address: PhysicalMemoryAddress) -> VirtualMemoryAddress {
        VirtualMemoryAddress::zero()
    }*/
}
