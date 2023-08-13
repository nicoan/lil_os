use super::address::{PhysicalMemoryAddress, VirtualMemoryAddress};
use crate::memory::paging::page_size::PageSize;

pub struct Mapper;

impl Mapper {
    pub fn map<PS: PageSize>(physical_address: PhysicalMemoryAddress) -> VirtualMemoryAddress {
        VirtualMemoryAddress::zero()
    }
}
