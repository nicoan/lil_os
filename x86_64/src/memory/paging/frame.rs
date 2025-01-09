//! This module represent a Frame in the physical adrress space.
//!
//! A frame is just a block of consecutive physical memory.
use crate::{
    impl_page_or_frame_for_size,
    memory::{
        address::PhysicalMemoryAddress,
        paging::{
            page_size::{PageSize, Size1GiB, Size2MiB, Size4KiB},
            paging_error::PagingError,
        },
    },
};
use core::marker::PhantomData;

/// A physical memory frame
#[derive(Debug)]
pub struct Frame<PS: PageSize> {
    start_address: PhysicalMemoryAddress,
    size: PhantomData<PS>,
}

impl_page_or_frame_for_size!(Frame, Size4KiB, PhysicalMemoryAddress, 4096);
impl_page_or_frame_for_size!(Frame, Size2MiB, PhysicalMemoryAddress, 2097152);
impl_page_or_frame_for_size!(Frame, Size1GiB, PhysicalMemoryAddress, 1073741824);
