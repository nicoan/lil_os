//! Global descriptor table initialization
use lazy_static::lazy_static;
use x86_64_custom::gdt::{Descriptor, GDTSelectors, GlobalDescriptorTable, TaskStateSegment};

use crate::panic_screen;

const ERROR_GDT_FULL: &str = "GDT is full. Tried to push a new value into it.";

// Lazy initialize the TSS
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.init();
        tss
    };

    pub static ref GDT: (GlobalDescriptorTable, GDTSelectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        // Initialize the segment selectors
        let Ok(kernel_code_segment_selector) = gdt.add_entry(Descriptor::kernel_code_segment()) else {
            // TODO: Panic screen should diverge, but it does not, thats why it is the panic there
             panic_screen!("{}\n{:?}", ERROR_GDT_FULL, gdt);
             panic!();
        };
        gdt.add_entry(Descriptor::kernel_data_segment());
        gdt.add_entry(Descriptor::user_code_segment());
        gdt.add_entry(Descriptor::user_data_segment());
        let Ok(tss_selector) = gdt.add_entry(Descriptor::task_state_segment(&TSS)) else {
            // TODO: Panic screen should diverge, but it does not, thats why it is the panic there
             panic_screen!("{}\n{:?}", ERROR_GDT_FULL, gdt);
             panic!();
        };

        let selectors = GDTSelectors {
            cs: kernel_code_segment_selector,
            tss: tss_selector,
        };

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
