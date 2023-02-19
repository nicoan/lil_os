use core::fmt::Debug;
use core::ops::Deref;

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
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct VirtualMemoryAddress(u64);

impl VirtualMemoryAddress {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn new(address: u64) -> Self {
        Self(address)
    }
}

impl Deref for VirtualMemoryAddress {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
