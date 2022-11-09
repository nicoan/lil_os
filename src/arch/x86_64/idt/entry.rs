//! Interrupt Descriptor table entry
//!
//! This module holds a representation of a IDT's entry

use super::handlers::{
    HandlerFunc, HandlerFuncWithErrCode, HandlerFuncWithErrCodeDiverging, PageFaultHandlerFunc,
};
use bit_field::BitField;
use core::marker::PhantomData;

/// Represents an entry un the Interrupt descriptor table.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Entry<F> {
    /// Lower bits of the pointer to the handler function.
    function_pointer_low: u16,

    /// Seletor of the global descriptor table.
    gdt_selector: u16,

    /// Entry options.
    options: Options,

    /// Middle bits of the pointer to the hanlder function.
    function_pointer_middle: u16,

    /// Higher bits of the pointer to the hanlder function.
    function_pointer_high: u32,

    reserved: u32,

    phantom: PhantomData<F>,
}

impl<F> Entry<F> {
    /// Returns an entry representing a missing handler function.
    #[inline]
    pub const fn missing() -> Self {
        Self {
            function_pointer_low: 0,
            gdt_selector: 0,
            options: Options::new(),
            function_pointer_middle: 0,
            function_pointer_high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }
}

macro_rules! implement_set_handler_function {
    ($t: ty) => {
        impl Entry<$t> {
            /// Sets a hanlder function for the given entry.
            ///
            /// # Arguments
            /// * `hanlder` - A handler function.
            pub fn set_handler_function(&mut self, handler: $t) {
                // Set the function pointer
                let handler = handler as usize;
                self.function_pointer_low = handler as u16;
                self.function_pointer_middle = (handler >> 16) as u16;
                self.function_pointer_high = (handler >> 32) as u32;

                // https://wiki.osdev.org/Segment_Selector
                // https://wiki.osdev.org/Segmentation
                // Set the gdt_selector to code segment
                self.gdt_selector = 0x08;

                // Set the present flag
                self.options.set_present(true);
                self.options.disable_interrupts(true);
            }
        }
    };
}

implement_set_handler_function!(HandlerFunc);
implement_set_handler_function!(HandlerFuncWithErrCodeDiverging);
implement_set_handler_function!(HandlerFuncWithErrCode);
// implement_set_handler_function!(PageFaultHandlerFunc);

/// Idt entry's options
///
/// The options has the following format (list is bits index, name and description):
/// * 0 - 2: Interrupt Stack Table Index - 0: Don’t switch stacks, 1-7: Switch to the n-th stack in
///   the Interrupt Stack Table when this handler is called.
/// * 3 - 7: (Reserved)
/// * 8: Interrupt/Trap gate - If this bit is 0, interrupts are disabled when this handler is
///   called.
/// * 9 - 11: Must be one (try to expand this)
/// * 12: Must be zero (try to expand this)
/// * 13-14: Descriptor Privilege Level - The minimal privilege level required for calling this
/// handler.
/// * 15: Present - If the entry is present.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
struct Options(u16);

impl Options {
    /// Returns default Options.
    ///
    /// This will return the minimum valid representation. Divided by index:
    /// indexes: 15 | 14-13 | 12 | 11-9  | 8 | 7-3   | 1-2
    /// values:   0 |   00  |  0 |  111  | 0 | 00000 | 000
    #[inline]
    const fn new() -> Self {
        #[allow(clippy::unusual_byte_groupings)]
        Options(0b0_00_0_111_0_00000_000)
    }

    /// Sets if the entry is present
    pub fn set_present(&mut self, present: bool) {
        self.0.set_bit(15, present);
    }

    /// Disables/Enables interrupts when another interrupt is beign handled.
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }
}
