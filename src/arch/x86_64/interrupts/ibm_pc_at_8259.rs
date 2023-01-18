//! Implementation of the IBM PC/AT 8259 PIC Architecture
//!
//! Systems before APIC usually had two 8259 PIC instances, a primary and a secondary one. The
//! secondary communicates with the CPU through the primary one.
//!
//! The IBM PC/AT extended the IBM PC architecture by adding a second 8259 PIC chip. This was
//! possible due to the 8259A's ability to cascade interrupts, that is, have them flow through
//! one chip and into another. This gives a total of 15 interrupts. Why 15 and not 16? That's
//! because when you cascade chips, the PIC needs to use one of the interrupt lines to signal
//! the other chip.
//!
//! Thus, in an AT, IRQ line 2 is used to signal the second chip. Because of this, IRQ 2 is not
//! available for use by hardware devices, which got wired to IRQ 9 on the slave PIC instead.
//! The real mode BIOS used to set up an interrupt handler for IRQ 9 that redirects to the IRQ 2
//! handler.
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
//! For the primary contoller, the ports are 0x20 for command and 0x21 for data. for the
//! secondary, they are 0xa0 for command and 0xa1 for data.
//!
//! This code and comments are havily based on:
//! - https://crates.io/crates/pic8259
//! - https://os.phil-opp.com/hardware-interrupts/
//! - https://wiki.osdev.org/8259_PIC

use crate::arch::x86_64::interrupts::pic8259::Pic8259Command;

use super::pic8259::Pic8259;

// I/O Command port number
const PIC_1_COMMAND: u8 = 0x20;
const PIC_2_COMMAND: u8 = 0xa0;

// I/O Data port number
const PIC_1_DATA: u8 = PIC_1_COMMAND + 1;
const PIC_2_DATA: u8 = PIC_2_COMMAND + 1;

// The default configuration of the PICs is not usable because it sends interrupt vector numbers
// in the range of 0–15 to the CPU. These numbers are already occupied by CPU exceptions.
// For example, number 8 corresponds to a double fault. To fix this overlapping issue, we need to
// remap the PIC interrupts to different numbers. The actual range doesn’t matter as long as it
// does not overlap with the exceptions, but typically the range of 32 (0x20) – 47 is chosen,
// because these are the first free numbers after the 32 exception slots.
const PIC_1_OFFSET: u8 = 0x20;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

struct IBMPcAt8259 {
    pic1: Pic8259,
    pic2: Pic8259,
}

impl IBMPcAt8259 {
    /// Creates a new instance of the IBM PC AT Interrupt architecture.
    pub const fn new() -> Self {
        Self {
            pic1: Pic8259::new(PIC_1_OFFSET, PIC_1_COMMAND, PIC_1_DATA),
            pic2: Pic8259::new(PIC_2_OFFSET, PIC_2_COMMAND, PIC_2_DATA),
        }
    }

    /// More info on the initialization process here:
    /// https://www.eeeguide.com/8259-programmable-interrupt-controller/
    pub unsafe fn initialize(&self) {
        // First we save the original interupt masks to restore them at the end of the
        // initialization sequence
        let pic_1_saved_mask = self.pic1.read_mask();
        let pic_2_saved_mask = self.pic2.read_mask();

        // Initialization command word 1 (ICW1) - Initialization of both pics
        self.pic1.execute_command(Pic8259Command::Initialize);
        self.pic2.execute_command(Pic8259Command::Initialize);

        // Initialization command word 2 (ICW2) - Set the vector offset
        self.pic1.write_mask(self.pic1.offset);
        self.pic2.write_mask(self.pic1.offset);

        // ICW3 - Configure the PICS in cascade mode (secondary -> primary)
        // First, we tell the primary pic which request line will be used for receiving
        // interruption from the secondary. The bits set in 1 will be the ones chained with other
        // pic. In this case is IRQ2 (counting from right to left)
        self.pic1.write_mask(0b00000100);
        // After that we set the cascade ID for the secondary PIC
        self.pic2.write_mask(2);

        todo!();
    }

    pub unsafe fn read_mask(&self, irq: u8) -> u8 {
        let pic = self.get_pic(irq);
        pic.read_mask()
    }

    pub unsafe fn write_mask(&self, irq: u8, mask: u8) {
        let pic = self.get_pic(irq);
        pic.write_mask(mask)
    }

    pub unsafe fn disable(&self) {
        self.pic1.disable();
        self.pic2.disable();
    }

    ///  This is issued to the PIC chips at the end of an IRQ-based interrupt routine. If the IRQ
    ///  came from the Master PIC, it is sufficient to issue this command only to the Master PIC;
    ///  however if the IRQ came from the Slave PIC, it is necessary to issue the command to both
    ///  PIC chips.
    ///
    /// # Safety
    ///
    /// This is unsafe because:
    /// - IRQ index must be valid (0 <= IRQ <= 15)
    /// - Programmer must be sure that the I/O port we are using is valid and initialized.
    /// - The I/O port could have side effects that violate memory safety.
    pub unsafe fn end_of_interrupt(&self, irq: u8) {
        if irq >= 8 {
            self.pic2.end_of_interrupt();
        }

        self.pic1.end_of_interrupt();
    }

    fn get_pic(&self, irq: u8) -> &Pic8259 {
        if irq < 8 {
            &self.pic1
        } else {
            &self.pic2
        }
    }
}
