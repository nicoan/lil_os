//! Page Table entry structure
//!
//! Flags comments taken from
//! https://docs.rs/x86_64/latest/src/x86_64/structures/paging/page_table.rs.html
use core::fmt::Display;
use core::ops::Deref;

use crate::memory::address::PhysicalMemoryAddress;

pub struct PageTableEntryFlags;

impl PageTableEntryFlags {
    /// Specifies if the mapped frame or page table is loaded memory
    pub const PRESENT: u64 = 1;

    /// Controls whether writes to the mapped frames are allowed.
    ///
    /// If this bit is unset in a level 1 page table entry, the mapped frame is read-only.
    /// If this bit is unset in a higher level page table entry the complete range of mapped
    pub const WRITABLE: u64 = 1 << 1;

    /// Controls whether accesses from userspace (i.e. ring 3) are permitted.
    pub const USER_ACCESSIBLE: u64 = 1 << 2;

    /// If this bit is set, a “write-through” policy is used for the cache, else a “write-back”
    /// policy is used.
    pub const WRITE_THROUGH: u64 = 1 << 3;

    /// Disables caching for the pointed entry is cacheable.
    pub const NO_CACHE: u64 = 1 << 4;

    /// Set by the CPU when the mapped frame or page table is accessed.
    pub const ACCESSED: u64 = 1 << 5;

    /// Set by the CPU on a write to the mapped frame.
    pub const DIRTY: u64 = 1 << 6;

    /// Specifies that the entry maps a huge frame instead of a page table. Only allowed in
    /// P2 or P3 tables.
    pub const HUGE_PAGE: u64 = 1 << 7;

    /// Indicates that the mapping is present in all address spaces, so it isn't flushed from
    /// the TLB on an address space switch.
    pub const GLOBAL: u64 = 1 << 8;

    /// Forbid code execution from the mapped frames.
    ///
    /// Can be only used when the no-execute page protection feature is enabled in the EFER
    /// register.
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// A page table entry
///
/// A page entry contains the a physical memory address address (bits 52..12) and flags
/// https://wiki.osdev.org/Paging
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    /// Creates a new page table entry.
    ///
    /// # Arguments
    /// * `flags`: Entry's flags.
    /// * `frame_starting_address`: Physical memory address the page is pointing to.
    pub fn new(
        flags: u64, // TODO: create a newtype or something like that here
        frame_starting_address: PhysicalMemoryAddress,
    ) -> Self {
        // TODO
        Self(flags | frame_starting_address.as_u64())
    }

    /// Checks if this entry is a used entry
    pub fn is_used(&self) -> bool {
        self.0 != 0
    }

    /// Returns if this entry is present in the table
    pub fn is_present(&self) -> bool {
        self.0 & PageTableEntryFlags::PRESENT > 0
    }

    /// Returns if this entry is present in the table
    pub fn is_huge(&self) -> bool {
        self.0 & PageTableEntryFlags::HUGE_PAGE > 0
    }

    /// Returns the physical frame pointed by this page table entry.
    ///
    /// The physical address is contained between bits 52..12.
    pub fn address(&self) -> PhysicalMemoryAddress {
        PhysicalMemoryAddress(self.0 & 0x000f_ffff_ffff_f000)
    }

    /// Sets entry flags
    ///
    /// # Arguments
    /// * `flags`: Entry's flags.
    pub fn set_flags(&mut self, flags: u64) {
        self.0 |= flags
    }

    /// Returns the flags used by page table entry.
    ///
    /// The physical address is contained between bits 52..12.
    pub fn get_flags(&self) -> u64 {
        self.0 & 0x0000_0000_0000_0fff
    }
}

impl Deref for PageTableEntry {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PageTableEntry - Physical Address: {:?}. Present: {}, Writable: {}, User Accesible: {}, Write Through: {}, No Cache. {}. Accesed: {}, Dirty: {}, Huge Page: {}, Gloal: {}, No execute: {}. Flags: {:#012b}",
            self.address(),
            self.0 & PageTableEntryFlags::PRESENT > 0,
            self.0 & PageTableEntryFlags::WRITABLE > 0,
            self.0 & PageTableEntryFlags::USER_ACCESSIBLE > 0,
            self.0 & PageTableEntryFlags::WRITE_THROUGH > 0,
            self.0 & PageTableEntryFlags::NO_CACHE > 0,
            self.0 & PageTableEntryFlags::ACCESSED > 0,
            self.0 & PageTableEntryFlags::DIRTY > 0,
            self.0 & PageTableEntryFlags::HUGE_PAGE > 0,
            self.0 & PageTableEntryFlags::GLOBAL > 0,
            self.0 & PageTableEntryFlags::NO_EXECUTE > 0,
            self.get_flags()
        )
    }
}
