//! Global Descriptor Table
//!
//! The GDT is a table of segment descriptors. The table will contain three types of descriptors:
//!
//! - NULL descriptor: Used as a protection mechanism, if any segment registers ever contain a
//! selector value of zero, it will refer to the NULL segment, which will cause a General
//! Protection Fault. This allows the kernel to catch processes which might intentionally or
//! accidentally try to misconfigure the segment selectors.
//!
//! - Call Gate Descriptors: To be completed
//!
//! - Task Segment State (TSS): It holds two stack tables, the Interrupt Stack Table (IST) used for
//! stack switching when an interrupt occurs and a Privilege Stack Table, used when the privilege
//! levels change (Ring 0 - Ring 3).
//!
//! Since we are using (or going to use, depending when you read this =]) paging, we are going to use four segments:
//! - Data Segment for Kernel (Ring 0)
//! - Data Segment for User (Ring 3)
//! - Code Segment for Kernel (Ring 0)
//! - Code Segment for User (Ring 0)
//!

mod descriptor;
mod table;
pub(crate) mod tss;

use lazy_static::lazy_static;

pub(crate) use table::{GDTSelectors, GlobalDescriptorTable};

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, GDTSelectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        // Initialize the segment selectors
        let selectors = gdt.init();
        (gdt, selectors)
    };
}

/// Loads the GDT
pub(crate) fn load_gdt() {
    GDT.0.load();
    // Segment registers and tss should be updated AFTER the GDT is loaded on memory
    GlobalDescriptorTable::update_selector_registers(&GDT.1);
    GlobalDescriptorTable::load_tss(&GDT.1.tss);
}
