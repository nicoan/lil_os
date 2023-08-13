use x86_64_custom::idt::InterruptStackFrame;

use crate::{panic_screen, println};

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Exception BREAKPOINT reached\n {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("Exception DIVIDED BY ZERO reached\n {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic_screen!(
        "Exception DOUBLE FAULT reached\n\n{}Error code: {}",
        stack_frame,
        error_code
    );
}
