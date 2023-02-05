use core::arch::asm;

use crate::{
    address::VirtualMemoryAddress,
    privilege::PrivilegeLevel,
    registers::segments::{SegmentSelector, CS},
};

use super::{descriptor::Descriptor, tss::TSS};

const MAX_LENGTH: usize = 8;
const ERROR_GDT_FULL: &str = "GDT is full. Tried to push a new value into it.";

pub struct GDTSelectors {
    pub cs: SegmentSelector,
    pub tss: SegmentSelector,
}

/// A 64 bit mode Global Descriptor Table (GDT)
///
/// The GDT has a fixed size of 8 entries. You do not need to add a null segment descriptor, this
/// is already done when initialized.
#[derive(Debug)]
pub struct GlobalDescriptorTable {
    /// Table entries.
    table: [u64; 8],

    /// Length of the table (number of present entries)
    len: usize,
}

impl GlobalDescriptorTable {
    /// Creates a new GDT with the NULL entry already set in index 0.
    #[inline]
    pub const fn new() -> Self {
        Self {
            table: [0; MAX_LENGTH],
            len: 1,
        }
    }

    /// Adds a new entry into the GDT
    ///
    /// If the table is full and this function is used, the kernel will panic. Returns the segment
    /// selector (GDT index) used for this entry.
    ///
    /// # Arguments
    /// * `entry` - A segment descriptor that will be added to the GDT.
    pub fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
        let index = match entry {
            Descriptor::UserSegment(us) => {
                if self.len == MAX_LENGTH {
                    // TODO: panic_screen is from kernel, maybe we should return a result here
                    // panic_screen!("{}\n{:?}", ERROR_GDT_FULL, self);
                }

                self.table[self.len] = us;
                self.len += 1;
                self.len - 1
            }
            Descriptor::SystemSegment(low, high) => {
                if self.len == MAX_LENGTH - 1 {
                    // TODO: panic_screen is from kernel, maybe we should return a result here
                    // panic_screen!("{}\n{:?}", ERROR_GDT_FULL, self);
                }

                self.table[self.len] = low;
                self.table[self.len + 1] = high;
                self.len += 2;
                self.len - 2
            }
        };

        // TODO: At the moment we only use Ring0 but this must change
        SegmentSelector::new(index as u16, PrivilegeLevel::Ring0)
    }

    /// Initializes the GDT.
    ///
    /// Sets up all the descriptors.
    pub fn init(&mut self) -> GDTSelectors {
        let kernel_code_segment_selector = self.add_entry(Descriptor::kernel_code_segment());
        self.add_entry(Descriptor::kernel_data_segment());
        self.add_entry(Descriptor::user_code_segment());
        self.add_entry(Descriptor::user_data_segment());
        let tss_selector = self.add_entry(Descriptor::task_state_segment(&TSS));

        GDTSelectors {
            cs: kernel_code_segment_selector,
            tss: tss_selector,
        }
    }

    /// Updates the processor's segment registers.
    ///
    /// # Safety
    /// The caller must be sure that the GDT is already loaded and the Segment Selectors given are
    /// valid
    pub fn update_selector_registers(selectors: &GDTSelectors) {
        // Update CS
        unsafe {
            CS::set_register(*selectors.cs as u8);
        }
    }

    /// Loads the TSS.
    ///
    /// # Safety
    /// The caller must be sure that the GDT is already loaded and the Segment Selector given is
    /// valid.
    pub fn load_tss(tss: &SegmentSelector) {
        // After loading the tdd we must load the TSS
        unsafe { asm!("ltr {0:x}", in(reg) **tss, options(nostack, preserves_flags)) }
    }

    /// Loads the GDT.
    ///
    /// # Safety
    ///
    /// We need this table to be valid for the whole execution of the OS. That is why its reference
    /// is marked with the lifetime 'static, to prevent shorter-lived GDTs references and
    /// use-after-free bugs.
    pub fn load(&'static self) {
        // Load the GDT
        let pointer = GDTDescriptor {
            limit: (self.len * core::mem::size_of::<u64>() - 1) as u16,
            base: VirtualMemoryAddress::new((self as *const _) as u64),
        };

        unsafe { asm!("lgdt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags)) }
    }
}

// The location of the GDT is kept in the GDT register. The named register contains a pointer to
// this structure, which contains the size of the table and the pointer to the GDT address.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct GDTDescriptor {
    /// One less than the size of the IDT in bytes.
    limit: u16,

    ///  The linear address of the Interrupt Descriptor Table (not the physical address, paging
    ///  applies).
    base: VirtualMemoryAddress,
}
