use crate::arch::x86_64::address::VirtualMemoryAddress;
use lazy_static::lazy_static;

// Lazy initialize the TSS
lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.init();
        tss
    };
}

/// Index for the double fault interrupt stack. Could be any number from 0 to 7, we chose 0.
pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

/// Creates an stack and returns its top address. Top address is returned because stacks grow
/// downwards in x86, and because of that, the last address in the stack is its starting
/// adddress.
macro_rules! create_stack {
    ($size: expr) => {{
        // Initialize static as mut so the bootloader does not map this as read-only memory. Then
        // we fill the stack with zeroes.
        static mut STACK: [u8; $size] = [0; $size];

        // Get the bottom address of the stack
        let stack_start = unsafe { (&STACK as *const _) as usize };

        // And return the top address
        VirtualMemoryAddress::new((stack_start + $size) as u64)
    }};
}

#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved_1: u32,
    privilege_stack_table: [VirtualMemoryAddress; 3],
    reserved_2: u64,
    interrupt_stack_table: [VirtualMemoryAddress; 7],
    reserved_3: u64,
    reserved_4: u16,
    io_map_base_address: u16,
}

impl TaskStateSegment {
    /// Creates a new TSS.
    pub const fn new() -> Self {
        Self {
            privilege_stack_table: [VirtualMemoryAddress::zero(); 3],
            interrupt_stack_table: [VirtualMemoryAddress::zero(); 7],
            io_map_base_address: 0,
            reserved_1: 0,
            reserved_2: 0,
            reserved_3: 0,
            reserved_4: 0,
        }
    }

    /// Initializes the TSS
    ///
    /// Sets up the interrupt stack table.
    pub fn init(&mut self) {
        // Initialize double fault interruption exception stack
        self.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = create_stack!(4096 * 5);
    }
}
