//! This module represent a Page in the virtual adrress space.
//!
//! A page is just a block of consecutive virtual memory that maps to a block of consecutive
//! physiacal memory (frame).
use crate::memory::{
    address::VirtualMemoryAddress,
    paging::{
        page_size::{PageSize, PageSize1GiB, PageSize2MiB, PageSize4KiB},
        page_table::PageTableLevel,
        paging_error::PagingError,
    },
};
use core::marker::PhantomData;

// use crate::memory::paging::impl_page_or_frame_for_size;

/// A virtual memory page
pub struct Page<PS: PageSize> {
    start_address: VirtualMemoryAddress,
    page_size: PhantomData<PS>,
}

/*
macro_rules! impl_page_for_size {
    ($size:ty, $size_in_bytes:expr) => {
        impl Page<$size> {
            const SIZE_IN_BYTES: u64 = $size_in_bytes;

            /// Return the page containing the address `virtual_address` as the starting address.
            /// If the address can't be an starting address for a Page (because it is not algined
            /// correctly) it will return a `PagingError::InvalidAlign`
            /// To be aligned, the virtual address must be a multiple of the page size in bytes
            /// For example for a page of 4KiB (4096 bytes):
            /// 0 -> first page,
            /// 4096 -> second page,
            /// and so on..
            ///
            /// # Arguments
            ///  * `virtual_address`: Address used as the start address of the returned page.
            pub fn from_starting_address(
                virtual_address: VirtualMemoryAddress,
            ) -> Result<Page<$size>, PagingError> {
                let virtual_address: VirtualMemoryAddress = virtual_address.into();
                if virtual_address.as_u64() % Self::SIZE_IN_BYTES != 0 {
                    return Err(PagingError::InvalidAlign);
                }

                Ok(Self {
                    start_address: virtual_address,
                    page_size: PhantomData,
                })
            }

            /// Return the page containing the address `virtual_address`
            ///
            /// # Arguments
            ///  * `virtual_address`: Address to be contained in the page
            pub fn containing_address(virtual_address: VirtualMemoryAddress) -> Page<$size> {
                let virtual_address: VirtualMemoryAddress = virtual_address.into();
                // Integer division!
                let start_address: VirtualMemoryAddress =
                    VirtualMemoryAddress::new(*virtual_address / Self::SIZE_IN_BYTES);

                Self {
                    start_address,
                    page_size: PhantomData,
                }
            }
            /// Returns page table index for this page
            ///
            /// Returns the selected level page table index.
            ///
            /// # Arguments
            ///  * `page_table_level`: The page table level we want to retrieve the index.
            pub fn get_page_table_index(&self, page_table_level: PageTableLevel) -> usize {
                self.start_address.get_page_table_index(page_table_level)
            }

            /// Returns the start address of this memory frame
            pub fn start_address(&self) -> VirtualMemoryAddress {
                self.start_address
            }
        }
    };
}
*/

impl_page_or_frame_for_size!(PageSize4KiB, VirtualMemoryAddress, 4096);
impl_page_or_frame_for_size!(PageSize2MiB, VirtualMemoryAddress, 2097152);
impl_page_or_frame_for_size!(PageSize1GiB, VirtualMemoryAddress, 1073741824);
