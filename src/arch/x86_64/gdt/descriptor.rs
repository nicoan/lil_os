//! Global Descriptor Table Entry
//!
//! We are using overlapping segments that use the whole memory available because we will use
//! paging as memory protection mechanism (that's why kernel code segment/data segment and user
//! code/data segment have the same base and limit values).
//!
//! This module holds a representation of a GDT's entry.
//!
//! Most of the code (and comments) were taken from:
//! https://docs.rs/x86_64/latest/src/x86_64/structures/gdt.rs.html
//!
//! More useful info:
//! https://www.reddit.com/r/osdev/comments/f0n5vr/gdt_and_user_mode/
//! https://wiki.osdev.org/Global_Descriptor_Table
//! https://wiki.osdev.org/GDT_Tutorial
//!
//! Most comments taken from:
//! https://web.archive.org/web/20190217233448/https://www.flingos.co.uk/docs/reference/Global-Descriptor-Table/

//! Segment Descriptor
//!
//! Represents an entry in the Global Descriptor table.

use bit_field::BitField;

use super::tss::TaskStateSegment;
pub enum Descriptor {
    /// Descriptor for a code or data segment
    UserSegment(u64),

    /// A system segment descriptor such as a LDT or TSS descriptor.
    SystemSegment(u64, u64),
}

// All flags used in 32-bit mode were not included to simplify things since this OS is 64 bit oly.
impl Descriptor {
    /// Set by the processor if this segment has been accessed. Only cleared by software.
    const ACCESSED: u64 = 1 << 40;

    /// Sets the segment as executable. This flag must be set for code segments and unset for data
    /// segments
    const EXECUTABLE: u64 = 1 << 43;

    /// This flag must be set for user segments.
    const USER_SEGMENT: u64 = 1 << 44;

    /// The DPL for this descriptor is Ring 3. In 64-bit mode, ignored for data segments
    const DPL_RING_3: u64 = 3 << 45;

    /// Must be set for any segment, causes a segment not present exception if not set.
    const PRESENT: u64 = 1 << 47;

    /// Must be set for 64-bit code segments, unset otherwise.
    const LONG_MODE: u64 = 1 << 53;

    /// Bits `0..=15` of the limit field
    const LIMIT_0_15: u64 = 0xFFFF;

    /// Bits `16..=19` of the limit field
    const LIMIT_16_19: u64 = 0xF << 48;

    /// Flags that we set for all the segments
    /// All the segments have base 0 and limit 0xFFFFF, they use the whole memory, since the memory
    /// protection mechanism are implemented through paging.
    const COMMON: u64 =
        Self::USER_SEGMENT | Self::PRESENT | Self::ACCESSED | Self::LIMIT_0_15 | Self::LIMIT_16_19;

    /// Returns a Kernel Data Segment Descriptor.
    ///
    /// This segment is marked as USER_SEGMENT because we use paging instead of segmentation for
    /// protecting kernel segments from user segments.
    #[inline]
    pub const fn kernel_data_segment() -> Self {
        Self::UserSegment(Self::COMMON)
    }

    /// Returns a Kernel Code Segment Descriptor.
    ///
    /// This segment is marked as USER_SEGMENT because we use paging instead of segmentation for
    /// protecting kernel segments from user segments.
    #[inline]
    pub const fn kernel_code_segment() -> Self {
        Self::UserSegment(Self::COMMON | Self::EXECUTABLE | Self::LONG_MODE)
    }

    /// Returns a User Data Segment Descriptor.
    #[inline]
    pub const fn user_data_segment() -> Self {
        Self::UserSegment(Self::COMMON | Self::DPL_RING_3)
    }

    /// Returns a User Code Segment Descriptor.
    #[inline]
    pub const fn user_code_segment() -> Self {
        Self::UserSegment(Self::COMMON | Self::EXECUTABLE | Self::LONG_MODE | Self::DPL_RING_3)
    }

    /// Returns a System Segment Descriptor that points to the TSS
    #[inline]
    pub fn task_state_segment(tss: &'static TaskStateSegment) -> Descriptor {
        let tss_pointer = (tss as *const _) as u64;
        let tss_size = (core::mem::size_of::<TaskStateSegment>() - 1) as u64;

        let mut low = Self::PRESENT;
        low.set_bits(16..40, tss_pointer.get_bits(0..24));

        // Set access bytes
        low.set_bits(40..44, 0b1001);

        low.set_bits(56..64, tss_pointer.get_bits(24..32));
        low.set_bits(0..16, tss_size.get_bits(0..16));

        let mut high = 0;
        high.set_bits(0..32, tss_pointer.get_bits(32..64));

        Descriptor::SystemSegment(low, high)
    }
}
