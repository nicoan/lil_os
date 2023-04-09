use core::fmt::Debug;
use core::ops::{Add, Deref};

/// Represents a physical memory address
///
/// On `x86_64`, only the 52 lower bits of a physical address can be used. The top 12 bits need
/// to be zero. This type guarantees that it always represents a valid physical address.k
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PhysicalMemoryAddress(pub u64);

impl PhysicalMemoryAddress {
    /// Creates a new Physical Memory Address.
    pub const fn new(address: u64) -> Self {
        // We set to zero the top 12 bits, since in x86_64 the lower 52 bits are used
        Self(address & 0x000f_ffff_ffff_ffff)
    }
}

impl Debug for PhysicalMemoryAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysicalMemoryAddress(0x{:x})", &self.0)
    }
}

/// Represents a virtual memory address
///
/// A virtual memory address is divided in five parts:
///
/// bits   63 .. 48  |  47 .. 39  |  38 .. 30  |  29 .. 21  |  20 .. 12  |  11 .. 0
///         16 bits      9 bits       9 bits       9 bits       9 bits      12 bits
///         not used   page table    page table   page table   page table   page offset
///                    lvl 4 index   lvl 3 index  lvl 2 index  lvl 1 index
///
/// The ranges in the tables are all included. Since bits 63 .. 48 are discarded, this means that
/// in reality we have 48 bits addresses. Even though bits 48 to 64 are discarded, they can’t be
/// set to arbitrary values. Instead, all bits in this range have to be copies of bit 47 in order
/// to keep addresses unique and allow future extensions like the 5-level page table.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct VirtualMemoryAddress(u64);

impl VirtualMemoryAddress {
    pub const fn zero() -> Self {
        Self(0)
    }

    // TODO: Create canonical virtual addresses here
    pub const fn new(address: u64) -> Self {
        Self(address)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.as_u64() as *mut T
    }
}

/// We overload the '+' to be able to add a Physycal Address to a Virtual Address to create a new
/// Virtual Address. This is useful in paging when adding a Physical Address the Virtual Adress
/// Offset to use in the level 4 page tables.
impl Add<PhysicalMemoryAddress> for VirtualMemoryAddress {
    type Output = Self;

    fn add(self, rhs: PhysicalMemoryAddress) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Deref for VirtualMemoryAddress {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
