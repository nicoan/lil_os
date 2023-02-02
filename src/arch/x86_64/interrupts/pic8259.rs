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

use x86_64::instructions::port::Port;

// NOTE: Only the needed ICW for initializing IBM PC AT are listed since it is not probable that we
// will use other configuration. In the future the idea is to use APIC instrad of IBM PC AT.
// For more information and all the other ICW:
/// https://www.eeeguide.com/8259-programmable-interrupt-controller/
pub const ICW1_ICW4_NEEDED: u8 = 0x01;
pub const ICW1_INIT: u8 = 0x10;
pub const ICW4_8086_MODE: u8 = 0x01;

/// Mask for disabling the PIC.
const MASK_DISABLE: u8 = 0xff;

#[repr(u8)]
pub enum Pic8259Command {
    /// Notify us that an interrupt has been handled and that we're ready for more.
    EndOfInterrupt = 0x20,
}

impl From<Pic8259Command> for u8 {
    fn from(val: Pic8259Command) -> Self {
        val as u8
    }
}

/// And individual PIC Chip.
pub struct Pic8259 {
    /// The base offset to which our interrupts are mapped.
    pub offset: u8,

    /// The I/O port to which we send commands.
    command: Port<u8>,

    /// The I/O port to which we send data.
    data: Port<u8>,
}

impl Pic8259 {
    /// Creates a new instance of the Pic8259.
    pub const fn new(offset: u8, command_port: u8, data_port: u8) -> Self {
        Self {
            offset,
            command: Port::new(command_port as u16),
            data: Port::new(data_port as u16),
        }
    }

    /// Executes a command on the PIC.
    ///
    /// # Safety
    ///
    /// This is unsafe becuase:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn execute_command<T>(&mut self, command: T)
    where
        T: Into<u8>,
    {
        self.command.write(command.into());
    }

    /// Reads the interrupt mask of this PIC.
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn read_mask(&mut self) -> u8 {
        self.data.read()
    }

    /// Reads the interrupt mask of this PIC.
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn write_mask(&mut self, mask: u8) {
        self.data.write(mask)
    }

    /// Disables this PIC
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - We are using interior mutability pattern. Programmer must be sure that the borrowing
    /// rules are followed in runtime (not borrowing mutable reference twice)
    pub unsafe fn disable(&mut self) {
        self.data.write(MASK_DISABLE);
    }
}
