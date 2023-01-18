use super::{ibm_pc_at_8259::InterruptIndex, PICS};
use crate::{arch::x86_64::idt::InterruptStackFrame, print};

macro_rules! create_interrupt_handler {
    ($name: ident, $irq: expr, $body: expr) => {
        pub extern "x86-interrupt" fn $name(_stack_frame: InterruptStackFrame) {
            $body

            unsafe {
                PICS.lock()
                    .end_of_interrupt($irq.as_u8());
            }
        }
    };
}

create_interrupt_handler!(timer_interrupt_handler, InterruptIndex::Timer, {
    print!(".");
});
