//! This module represent a Page in the virtual adrress space.
//!
//! A page is just a block of consecutive virtual memory that maps to a block of consecutive
//! virtual memory (page).
use crate::memory::{
    address::VirtualMemoryAddress,
    paging::{
        page_size::{PageSize, PageSize1GiB, PageSize2MiB, PageSize4KiB},
        paging_error::PagingError,
    },
};
use core::marker::PhantomData;

/// A virtual memory page
struct Page<PS: PageSize> {
    start_address: VirtualMemoryAddress,
    page_size: PhantomData<PS>,
}

macro_rules! impl_page_for_size {
    ($size:ty, $size_in_bytes:expr) => {
        impl Page<$size> {
            const SIZE_IN_BYTES: u64 = $size_in_bytes;

            pub fn from_starting_address<T: Into<VirtualMemoryAddress>>(
                virtual_address: T,
            ) -> Result<Page<$size>, PagingError> {
                // To be aligned, the virtual address must be a multiple of the page size in bytes
                // For example for a page of 4KiB (4096 bytes):
                // 0 -> first page,
                // 4096 -> second page,
                // and so on..
                let virtual_address: VirtualMemoryAddress = virtual_address.into();
                if virtual_address.as_u64() % Self::SIZE_IN_BYTES != 0 {
                    return Err(PagingError::InvalidAlign);
                }

                Ok(Self {
                    start_address: virtual_address,
                    page_size: PhantomData,
                })
            }
        }
    };
}

impl_page_for_size!(PageSize4KiB, 4096);
impl_page_for_size!(PageSize2MiB, 2097152);
impl_page_for_size!(PageSize1GiB, 1073741824);
