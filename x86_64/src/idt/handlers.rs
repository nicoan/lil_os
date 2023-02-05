use core::fmt::{Display, Error, Formatter};

/// Exception handler.
pub type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);

/// Exception handler with error code.
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(_: InterruptStackFrame, _: u64);

/// Exception handler with error code that diverges. This is used in a double fault, because we do
/// not return from there.
pub type HandlerFuncWithErrCodeDiverging =
    extern "x86-interrupt" fn(_: InterruptStackFrame, _: u64) -> !;

// TODO: Change type
/// Page Fault exception handler function
pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);

/// Interrupt Stack Frame
///
/// Interrupt hanlders usually run in a different context than common functions. An exception can
/// occur at any point in time, meaning that the OS has to back up everything that is needed to
/// resume execution further in time after handling the exception. Almost all the information
/// needed by the handler is passed as an argument to the handler, contained in this stucture
/// (error code is passed in another argument)
#[repr(C)]
#[derive(Debug)]
pub struct InterruptStackFrame {
    /// Points to the instruction that should be executed when the interrupt handler returns.
    instruction_pointer: u64,

    /// Code segment selector.
    code_segment: u64,

    /// The flags register before the interrupt hanlder was invoked.
    cpu_flags: u64,

    /// The stack pointer at the time of the interrupt.
    stack_pointer: u64,

    /// Stack segment descriptor at the time of the interrupt.
    stack_segment: u64,
}

impl Display for InterruptStackFrame {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Instruction Pointer: 0x{:x}", self.instruction_pointer)?;
        writeln!(f, "Code Segment: 0x{:x}", self.code_segment)?;
        writeln!(f, "CPU Flags: 0x{:x}", self.cpu_flags)?;
        writeln!(f, "Stack Pointer: 0x{:x}", self.stack_pointer)?;
        writeln!(f, "Stack Segment: 0x{:x}", self.stack_segment)
    }
}

// The x86-interrupt calling convention:
//
// Since we don't know when an exception occurs, we can't backup any registers before. This means
// we can't use a calling convention that relies on caller-saved registers for exception handlers.
// Instead, we need a calling convention that preserves all registers. The x86-interrupt calling
// convention is such a calling convention, so it guarantees that all register values are restored
// to their original values on function return.
//
// https://os.phil-opp.com/cpu-exceptions/#the-interrupt-calling-convention
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // TODO: Println is from the kernel
    // println!("Exception BREAKPOINT reached\n {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    // TODO: Println is from the kernel
    // println!("Exception DIVIDED BY ZERO reached\n {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    // TODO: panic_screen is from the kernel
    /*
    panic_screen!(
        "Exception DOUBLE FAULT reached\n\n{}Error code: {}",
        stack_frame,
        error_code
    );
    */
    #[allow(clippy::empty_loop)]
    loop {}
}
