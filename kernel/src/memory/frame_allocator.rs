use x86_64_custom::memory::{
    frame_allocator::{self, PhysicalFrame},
    paging::page_size::Size4KiB,
};

/// struct used for implementing the frame allocator
struct FrameAllocator;

impl frame_allocator::FrameAllocator<Size4KiB> for FrameAllocator {
    unsafe fn allocate(&self) -> Option<PhysicalFrame<Size4KiB>> {
        None
    }
}
