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
pub type PageFaultErrorCode = u64;
/// Page Fault exception handler function
pub type PageFaultHandlerFunc =
    extern "x86-interrupt" fn(_: InterruptStackFrame, _: PageFaultErrorCode);

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
