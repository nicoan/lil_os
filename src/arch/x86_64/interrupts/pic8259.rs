//! Implementation for the PIC (Programmable Interrupt Controller) 8259
//!
//! This PIC hanldes basic I/O interrupts. For more advanced interrupt handling an APIC (advanced
//! Programmable Interrupt Controller) interface needed instead of this one.
//!
//! Connecting all hardware devices directly to the CPU is not possible. Instead, a separate
//! interrupt controller aggregates the interrupts from all devices and then notifies the CPU:
//!                                    ____________             _____
//!               Timer ------------> |            |           |     |
//!               Keyboard ---------> | Interrupt  |---------> | CPU |
//!               Other Hardware ---> | Controller |           |_____|
//!               Etc. -------------> |____________|
//!
//! The controller can be configured through two I/O ports, one "command" port and one "data"
//! port.
//!
//! The named inputs in the chips (for example "Keyboard" in the primary PIC) are called interrupt
//! lines. For example, when a key is pressed, the keyboard sends a pulse along its interrupt line
//! (this is called IRQ - Interrupt Request, for the Keyboard it is the IRQ1) which then translates
//! the IRQ into a system interrupt and sends a message to interrupt the CPU to whatever is doing.
//!
//! This code and comments are havily based on:
//! - https://crates.io/crates/pic8259
//! - https://os.phil-opp.com/hardware-interrupts/
//! - https://wiki.osdev.org/8259_PIC

use core::cell::RefCell;

use x86_64::instructions::port::Port;

/// End of interrupt (EOI) command
const CMD_END_OF_INTERRUPT: u8 = 0x20;

/// Initialization command. This is public because is used by the architecture implementations
pub const CMD_INITIALIZE: u8 = 0x11;

/// Mask for disabling the PIC.
const MASK_DISABLE: u8 = 0xff;

/// And individual PIC Chip.
pub struct Pic8259 {
    /// The base offset to which our interrupts are mapped.
    offset: u8,

    /// The I/O port to which we send commands.
    command: RefCell<Port<u8>>,

    /// The I/O port to which we send data.
    data: RefCell<Port<u8>>,
}

impl Pic8259 {
    /// Creates a new instance of the Pic8259.
    pub const fn new(offset: u8, command_port: u8, data_port: u8) -> Self {
        Self {
            offset,
            command: RefCell::new(Port::new(command_port as u16)),
            data: RefCell::new(Port::new(data_port as u16)),
        }
    }

    /// Notify us that an interrupt has been handled and that we're ready
    /// for more.
    ///
    /// # Safety
    ///
    /// This is unsafe becuase:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn end_of_interrupt(&self) {
        self.command.borrow_mut().write(CMD_END_OF_INTERRUPT);
    }

    /// Reads the interrupt mask of this PIC.
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn read_mask(&self) -> u8 {
        self.data.borrow_mut().read()
    }

    /// Reads the interrupt mask of this PIC.
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn write_mask(&self, mask: u8) {
        self.data.borrow_mut().write(mask)
    }

    /// Disables this PIC
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn disable(&self) {
        self.data.borrow_mut().write(MASK_DISABLE);
    }
}
