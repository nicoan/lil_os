//! Global descriptor table initialization
use lazy_static::lazy_static;
use x86_64_custom::gdt::{GDTSelectors, GlobalDescriptorTable};

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
