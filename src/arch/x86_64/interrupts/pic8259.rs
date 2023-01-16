//! Implementation for the PIC (Programmable Interrupt Controller) 8259
//!
//! This PIC hanldes basic I/O interrupts. For more advanced interrupt handling an APIC (advanced
//! Programmable Interrupt Controller) interface needed instead of this one.
//!
//! Systems before APIC usually had two 8259 PIC instances, a primary and a secondary one. The
//! secondary communicates with the CPU through the primary one:
//!
//!                      ____________                          ____________
//! Real Time Clock --> |            |   Timer -------------> |            |
//! ACPI -------------> |            |   Keyboard-----------> |            |      _____
//! Available --------> | Secondary  |----------------------> | Primary    |     |     |
//! Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
//! Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
//! Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
//! Primary ATA ------> |            |   Floppy disk -------> |            |
//! Secondary ATA ----> |____________|   Parallel Port 1----> |____________|
//!
//! Each controller can be configured through two I/O ports, one "command" port and one "data"
//! port. For the primary contoller, these ports are 0x20 for command and 0x21 for data. for the
//! secondary, they are 0xa0 for command and 0xa1 for data.
//!
//! The named inputs in the chips (for example "Keyboard" in the primary PIC) are called interrupt
//! lines. For example, when a key is pressed, the keyboard sends a pulse along its interrupt line
//! (this is calle IRQ - Interrupt Request, for the Keyboard it is the IRQ1) which then translates
//! the IRQ into a system interrupt and sends a message to interrupt the CPU to whatever is doing.
//!
//! This code and comments are havily based on:
//! - https://crates.io/crates/pic8259
//! - https://os.phil-opp.com/hardware-interrupts/
//! - https://wiki.osdev.org/8259_PIC

// TODO: Create a trait that abstracs aways if we are using a pic8259 or an apic or whatever
// interrupt controller we need o use.

use x86_64::instructions::port::Port;

/// And individual PIC Chip.
struct Pic {
    /// The base offset to which our interrupts are mapped.
    offset: u8,

    /// The I/O port to which we send commands.
    command: Port<u8>,

    /// The I/O port to which we send data.
    data: Port<u8>,
}
