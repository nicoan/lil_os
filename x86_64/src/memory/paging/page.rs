//! This module represent a Page in the virtual adrress space.
//!
//! A page is just a block of consecutive virtual memory that maps to a block of consecutive
//! physiacal memory (frame).
use crate::{
    impl_page_or_frame_for_size,
    memory::{
        address::VirtualMemoryAddress,
        paging::{
            page_size::{PageSize, Size1GiB, Size2MiB, Size4KiB},
            page_table::PageTableLevel,
            paging_error::PagingError,
        },
    },
};
use core::marker::PhantomData;

/// A virtual memory page
#[derive(Clone, Copy, Debug)]
pub struct Page<PS: PageSize> {
    start_address: VirtualMemoryAddress,
    size: PhantomData<PS>,
}

macro_rules! impl_page_for_size {
    ($size:ty) => {
        impl Page<$size> {
            /// Returns page table index for this page
            ///
            /// Returns the selected level page table index.
            ///
            /// # Arguments
            ///  * `page_table_level`: The page table level we want to retrieve the index.
            pub fn get_page_table_index(&self, page_table_level: PageTableLevel) -> usize {
                self.start_address.get_page_table_index(page_table_level)
            }
        }
    };
}

impl_page_or_frame_for_size!(Page, Size4KiB, VirtualMemoryAddress, 4096);
impl_page_or_frame_for_size!(Page, Size2MiB, VirtualMemoryAddress, 2097152);
impl_page_or_frame_for_size!(Page, Size1GiB, VirtualMemoryAddress, 1073741824);

impl_page_for_size!(Size4KiB);
impl_page_for_size!(Size2MiB);
impl_page_for_size!(Size1GiB);
