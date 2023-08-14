use core::fmt::Debug;
use core::ops::Deref;

/// Represents a physical memory address
///
/// On `x86_64`, only the 52 lower bits of a physical address can be used. The top 12 bits need
/// to be zero. This type guarantees that it always represents a valid physical address.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PhysicalMemoryAddress(pub u64);

impl PhysicalMemoryAddress {
    /// Creates a new Physical Memory Address.
    pub const fn new(address: u64) -> Self {
        // We set to zero the top 12 bits, since in x86_64 the lower 52 bits are used
        Self(address & 0x000f_ffff_ffff_ffff)
    }

    /// Returns the address as u64
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Debug for PhysicalMemoryAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysicalMemoryAddress(0x{:x})", &self.0)
    }
}

impl Deref for PhysicalMemoryAddress {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
