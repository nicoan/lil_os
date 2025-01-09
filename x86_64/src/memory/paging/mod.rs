pub mod frame;
pub mod page;
pub mod page_size;
pub mod page_table;
pub mod paging_error;

#[macro_export]
macro_rules! impl_page_or_frame_for_size {
    ($struct:ident, $size:ty, $address_type:ty, $size_in_bytes:expr) => {
        impl $struct<$size> {
            const SIZE_IN_BYTES: u64 = $size_in_bytes;

            /// Return the page containing the address `address` as the starting address.
            /// If the address can't be an starting address for a Page (because it is not algined
            /// correctly) it will return a `PagingError::InvalidAlign`
            /// To be aligned, the address must be a multiple of the page size in bytes
            /// For example for a page of 4KiB (4096 bytes):
            /// 0 -> first page,
            /// 4096 -> second page,
            /// and so on..
            ///
            /// # Arguments
            ///  * `address`: Address used as the start address of the returned page.
            pub fn from_starting_address(
                address: $address_type,
            ) -> Result<$struct<$size>, PagingError> {
                if address.as_u64() % Self::SIZE_IN_BYTES != 0 {
                    return Err(PagingError::InvalidAlign);
                }

                Ok(Self {
                    start_address: address,
                    size: PhantomData,
                })
            }

            /// Return the page containing the address `address`
            ///
            /// # Arguments
            ///  * `address`: Address to be contained in the page
            pub fn containing_address(address: $address_type) -> $struct<$size> {
                // TODO: Check this implementation, took from
                // https://docs.rs/x86_64/0.14.10/src/x86_64/addr.rs.html#641
                let start_address: $address_type =
                    <$address_type>::new(*address & !(Self::SIZE_IN_BYTES - 1));

                Self {
                    start_address,
                    size: PhantomData,
                }
            }

            /// Returns the start address of this memory frame
            pub fn start_address(&self) -> $address_type {
                self.start_address
            }
        }
    };
}
