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
/// A virtual memory address is divided in six parts:
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
///
/// The page offset is where is located the the piece of data we are pointing to in the physical
/// frame.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct VirtualMemoryAddress(u64);

impl VirtualMemoryAddress {
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Creates a new canonical virtual address.
    ///
    /// This function performs the sign extension to the passed address to create a canonical
    /// virtual address. Bits 63 to 48 must be all zeroes or all ones, if not, the function will
    /// panic.
    pub const fn new(address: u64) -> Self {
        let bit_47 = address >> 47 & 1;
        let bits_63_48 = address >> 48;

        // Check if bits 63 to 48 are all 0 or all 1. If not the virtual address is not valid.
        if bits_63_48 != 0 && bits_63_48 != 0xffff {
            // TODO: Maybe return a result and remove the panic
            panic!("Bits 63 to 48 must not have information");
        }

        // If most significant bit (47) is one
        if bit_47 == 1 {
            // And bits 63 to 48 are not 1, we perform sign extension.
            if bits_63_48 != 0xffff {
                Self(address | 0xffff000000000000)
            } else {
                Self(address)
            }
        } else {
            // And bits 63 to 48 are not 0, we perform sign extension.
            if bits_63_48 != 0 {
                Self(address & 0x0000ffffffffffff)
            } else {
                Self(address)
            }
        }
    }

    /// Returns the address as u64
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Returns the address as a mutable pointer
    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.as_u64() as *mut T
    }

    // TODO: Add description
    pub fn get_page_table_index(&self, page_table_level: PageTableLevel) -> usize {
        // First we remove the uneeded rightmost bits depending on the level
        let index = match page_table_level {
            PageTableLevel::Level1 => self.0 >> 12,
            PageTableLevel::Level2 => self.0 >> 21,
            PageTableLevel::Level3 => self.0 >> 30,
            PageTableLevel::Level4 => self.0 >> 39,
        };

        // And then the leftmost ones
        (index as u16 & 0x1ff).into()
    }

    pub fn get_page_offset(&self) -> u16 {
        self.0 as u16 & 0x0fff
    }
}

/// Represents a level of the multilevel Page Table
pub enum PageTableLevel {
    Level1,
    Level2,
    Level3,
    Level4,
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
