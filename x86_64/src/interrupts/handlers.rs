use super::ibm_pc_at_8259::InterruptIndex;
use crate::idt::InterruptStackFrame;
use crate::interrupts::{keyboard_handler, timer_handler};

type HardwareInterruptHandler = extern "x86-interrupt" fn(_stack_frame: InterruptStackFrame);

/// Creates an interrupt handler for an specific IRQ.
///
/// An interupt handler should always send and end of interrupt command. It also uses the
/// "x86-interrupt" foreing calling convention.
macro_rules! create_interrupt_handler {
    ($name: ident, $irq: expr, $interrupt_controller: ident, $body: expr) => {
        pub extern "x86-interrupt" fn $name(_stack_frame: InterruptStackFrame) {
            $body

            unsafe {
                <$interrupt_controller>.lock()
                    .end_of_interrupt($irq.as_u8());
            }
        }
    };
}

/// This struct contains pointes to the actual implementation of the handlers. for
///
/// By using this the OS can set the corresponding handlers and we can abstract away the
/// architecture details.
pub struct HardwareInterruptHandlers {
    pub timer_handler: fn(),
    pub keyboard_handler: fn(u8),
}

pub struct X86HardwareInterruptHandlers {
    pub timer_handler: HardwareInterruptHandler,
    pub keyboard_handler: HardwareInterruptHandler,
}
