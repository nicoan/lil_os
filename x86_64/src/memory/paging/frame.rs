//! This module represent a Frame in the physical adrress space.
//!
//! A frame is just a block of consecutive physical memory.
use crate::memory::{
    address::PhysicalMemoryAddress,
    paging::{
        page_size::{PageSize, PageSize1GiB, PageSize2MiB, PageSize4KiB},
        paging_error::PagingError,
    },
};
use core::marker::PhantomData;

/// A physical memory frame
pub struct Frame<PS: PageSize> {
    start_address: PhysicalMemoryAddress,
    frame_size: PhantomData<PS>,
}

macro_rules! impl_frame_for_size {
    ($size:ty, $size_in_bytes:expr) => {
        impl Frame<$size> {
            const SIZE_IN_BYTES: u64 = $size_in_bytes;

            /// Return the frame containing the address `physical_address` as the starting address.
            /// If the address can't be an starting address for a Frame (because it is not algined
            /// correctly) it will return a `PagingError::InvalidAlign`
            /// To be aligned, the physical address must be a multiple of the frame size in bytes
            /// For example for a frame of 4KiB (4096 bytes):
            /// 0 -> first frame,
            /// 4096 -> second frame,
            /// and so on..
            ///
            /// # Arguments
            ///  * `physical_address`: Address used as the start address of the returned frame.
            pub fn from_starting_address(
                physical_address: PhysicalMemoryAddress,
            ) -> Result<Frame<$size>, PagingError> {
                // To be aligned, the physical address must be a multiple of the frame size in bytes
                // For example for a frame of 4KiB (4096 bytes):
                // 0 -> first frame,
                // 4096 -> second frame,
                // and so on..
                let physical_address: PhysicalMemoryAddress = physical_address.into();
                if physical_address.as_u64() % Self::SIZE_IN_BYTES != 0 {
                    return Err(PagingError::InvalidAlign);
                }

                Ok(Self {
                    start_address: physical_address,
                    frame_size: PhantomData,
                })
            }

            /// Return the frame containing the address `physical_address`
            ///
            /// # Arguments
            ///  * `physical_address`: Address to be contained in the frame
            pub fn containing_address(physical_address: PhysicalMemoryAddress) -> Frame<$size> {
                let physical_address: PhysicalMemoryAddress = physical_address.into();
                // Integer division!
                let start_address: PhysicalMemoryAddress =
                    PhysicalMemoryAddress::new(*physical_address / Self::SIZE_IN_BYTES);

                Self {
                    start_address,
                    frame_size: PhantomData,
                }
            }
        }
    };
}

impl_frame_for_size!(PageSize4KiB, 4096);
impl_frame_for_size!(PageSize2MiB, 2097152);
impl_frame_for_size!(PageSize1GiB, 1073741824);
