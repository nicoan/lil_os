use core::ops::Deref;

/// Represents a phisical memory address
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MemoryAddress(u64);

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
