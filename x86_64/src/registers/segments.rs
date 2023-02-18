//! This module implements all the needed abstractions for segments registers (CS, SS, DS, ES, FS,
//! GS)
use core::arch::asm;
use core::ops::Deref;

use crate::privilege::PrivilegeLevel;

/// An element from a GDT/LDT table to load into a segment
#[repr(transparent)]
#[derive(Debug)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    /// Creates a new SegmentSelector
    ///
    /// # Arguments
    ///  * `index`: index in GDT or LDT array
    ///  * `rpl`: the requested privilege level
    pub fn new(index: u16, rpl: PrivilegeLevel) -> Self {
        Self(index << 3 | rpl as u16)
    }
}

impl Deref for SegmentSelector {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CS;

impl CS {
    /// Sets the CS register with the given segment seletor. CS can not be directly set with a
    /// `mov` instruction.
    ///
    /// The asm code is from https://docs.rs/x86_64/latest/src/x86_64/instructions/segmentation.rs.html#58
    ///
    /// # Arguments
    /// * `segment_selector` - An entry index from the GDT table.
    ///
    /// # Safety
    /// This functions is unsafe because the caller be sure that the GDT index is valid when
    /// setting setting the register.
    pub unsafe fn set_register(segment_selector: u8) {
        unsafe {
            asm!(
                "push {sel}",
                "lea {tmp}, [1f + rip]",
                "push {tmp}",
                "retfq",
                "1:",
                sel = in(reg) u64::from(segment_selector),
                tmp = lateout(reg) _,
                options(preserves_flags),
            );
        }
    }

    /// Gets the value of the CS register
    pub fn get_register() -> SegmentSelector {
        let mut segment: u16;
        unsafe {
            asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
        }
        SegmentSelector(segment)
    }
}
