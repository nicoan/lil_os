//! Virtual to Physical address translator
use super::address::{PhysicalMemoryAddress, VirtualMemoryAddress};

// Marker trait used to limit the generic parameter of the mapper struct to types that implements
// this marker
trait PageSize {}

/// Represents a page size of 4 KiB
pub struct Size4KiB;
impl PageSize for Size4KiB {}

/// Represents a (huge) page size of 2 MiB
pub struct Size2MiB;
impl PageSize for Size2MiB {}

/// Represents a (huge) page size of 1 GiB
pub struct Size1GiB;
impl PageSize for Size1GiB {}

struct Mapper;

impl Mapper {
    pub fn map<PS: PageSize>(pagephysical_address: PhysicalMemoryAddress) -> VirtualMemoryAddress {
        VirtualMemoryAddress::zero()
    }
}
