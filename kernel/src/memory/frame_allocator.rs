use x86_64_custom::memory::{
    frame_allocator::{self, PhysicalFrame},
    paging::page_size::PageSize4KiB,
};

/// struct used for implementing the frame allocator
struct FrameAllocator;

impl frame_allocator::FrameAllocator<PageSize4KiB> for FrameAllocator {
    unsafe fn allocate(&self) -> Option<PhysicalFrame<PageSize4KiB>> {
        None
    }
}
