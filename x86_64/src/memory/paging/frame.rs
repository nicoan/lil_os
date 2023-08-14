//! This module represent a Frame in the physical adrress space.
//!
//! A frame is just a block of consecutive physical memory.
use crate::memory::{
    address::PhysicalMemoryAddress,
    paging::{
        frame_size::{FrameSize, FrameSize1GiB, FrameSize2MiB, FrameSize4KiB},
        paging_error::PagingError,
    },
};
use core::marker::PhantomData;

/// A physical memory frame
struct Frame<PS: FrameSize> {
    start_address: PhysicalMemoryAddress,
    frame_size: PhantomData<PS>,
}

macro_rules! impl_frame_for_size {
    ($size:ty, $size_in_bytes:expr) => {
        impl Frame<$size> {
            const SIZE_IN_BYTES: u64 = $size_in_bytes;

            pub fn from_starting_address<T: Into<PhysicalMemoryAddress>>(
                physical_address: T,
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
        }
    };
}

impl_frame_for_size!(FrameSize4KiB, 4096);
impl_frame_for_size!(FrameSize2MiB, 2097152);
impl_frame_for_size!(FrameSize1GiB, 1073741824);
