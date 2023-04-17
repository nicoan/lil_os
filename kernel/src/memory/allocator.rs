//! This module contains the implementation for the kernel's memory heap allocator

use alloc::alloc::GlobalAlloc;

#[global_allocator]
pub static MEMORY_ALLOCATOR: MemoryAllocator = MemoryAllocator;

/// Virtual memory starting address assigned to heap memory.
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Size of the heap memory.
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub struct MemoryAllocator;

// TODO: At the moment is just a dummmy allocator for compiling reasons
unsafe impl GlobalAlloc for MemoryAllocator {
    /// Allocates heap memory.
    ///
    /// If the memory can't be allocated, a null pointer is returned and `alloc_error_handler` is
    /// called to handle the error. All of this happens implicitly.
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!("Must program deallocator")
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
