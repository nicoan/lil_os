//! This module represent a Page in the virtual adrress space.
//!
//! A page is just a block of consecutive virtual memory that maps to a block of consecutive
//! physical memory (frame).
use crate::memory::{
    address::VirtualMemoryAddress,
    paging::{
        page_size::{PageSize, PageSize4KiB},
        paging_error::PagingError,
    },
};
use core::marker::PhantomData;

/// A virtual memory page
struct Page<PS: PageSize> {
    start_address: VirtualMemoryAddress,
    page_size: PhantomData<PS>,
}

impl Page<PageSize4KiB> {
    pub fn from_starting_address<T: Into<VirtualMemoryAddress>>(
        virtual_address: T,
    ) -> Result<Page<PageSize4KiB>, PagingError> {
        // To be aligned, the virtual address must be a multiple of the page size in bytes (0 ->
        // first page, 4096 -> second page, etc...)
        let virtual_address: VirtualMemoryAddress = virtual_address.into();
        if virtual_address.as_u64() % 4096 != 0 {
            return Err(PagingError::InvalidPageAlign);
        }

        Ok(Self {
            start_address: virtual_address,
            page_size: PhantomData,
        })
    }
}
