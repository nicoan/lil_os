//! Frame allocator
//!
//! This module gives the interface to implement a frame allocator. The frame allocator is
//! responsible of allocating the intermediate page tables (level 3, 2 and 1) into a physical
//! frame.
use crate::memory::address::PhysicalMemoryAddress;
use crate::memory::paging::page_size::PageSize;
use core::marker::PhantomData;

use super::paging::page_size::PageSize4KiB;

// Represent a mapped physical frame of a certain size
pub struct PhysicalFrame<PS: PageSize> {
    frame_address: PhysicalMemoryAddress,
    size: PhantomData<PS>,
}

pub trait FrameAllocator<PS: PageSize> {
    /// Allocates a frame
    /// This method is unsafe because the implementer must guarantee that the allocator yields only
    /// unused frames. Otherwise, undefined behavior might occur.
    unsafe fn allocate(&self) -> Option<PhysicalFrame<PS>>;
}

pub struct DummyAllocator;

impl FrameAllocator<PageSize4KiB> for DummyAllocator {
    unsafe fn allocate(&self) -> Option<PhysicalFrame<PageSize4KiB>> {
        None
    }
}
