//! This modules takes care of all the x86 interruption details and then passes the information to
//! the actual kernel handlers.
//!
//! The x86-interrupt calling convention:
//
//! Since we don't know when an exception occurs, we can't backup any registers before. This means
//! we can't use a calling convention that relies on caller-saved registers for exception handlers.
//! Instead, we need a calling convention that preserves all registers. The x86-interrupt calling
//! convention is such a calling convention, so it guarantees that all register values are restored
//! to their original values on function return.
//
//! https://os.phil-opp.com/cpu-exceptions/#the-interrupt-calling-convention

pub(crate) mod hardware;
pub(crate) mod software;
