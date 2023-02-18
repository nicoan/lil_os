//! Most of this code is from https://os.phil-opp.com/testing/#exiting-qemu
//!
//! Here we code specific QEMU code to exit it and show the output in the host terminal.
//! Right now we depend on the "x86_64" crate to communicate with the port-mapped I/O. The idea is,
//! in the future to remove all these dependencies and implement those features.
//!
//! How do we communicate with QEMU? It uses port-mapped I/O called "isa-debug-exit".
//! From the phil-opp blog:
//! port-mapped I/O uses a separate I/O bus for communication. Each connected peripheral has one or
//! more port numbers. To communicate with such an I/O port there are special CPU instructions
//! called in and out, which take a port number and a data byte (there are also variations of these
//! commands that allow sending an u16 or u32).
//!
//! The isa-debug-exit devices uses port-mapped I/O. The iobase parameter specifies on which port
//! address the device should live (0xf4 is a generally unused port on the x86’s IO bus) and the
//! iosize specifies the port size (0x04 means four bytes).

use x86_64::instructions::port::Port;

/// We specified in Cargo.toml that the isa-debug-exit device is located at 0xf4 port (this port is
/// generally unused in x86) with a size of 4 bytes.
const ISA_DEBUG_EXIT_ADDRESS: u16 = 0xf4;

/// Custom exit codes for QEMU
///
/// When we write to isa-debug-exit, QEMU exit code in the host operating system is calculated as
/// (value << 1) | 1. So if we write:
/// - 0 then the exit code would be (0 << 1) | 1 = 1
/// - 1 then the exit code would be (1 << 1) | 1 = 3
///
/// That is why we don't use the classic 0 and 1 exit codes for our custom exit codes. We cont want
/// our custom exit codes to clash with the QEMU ones.
/// This error codes are mapped in Cargo.toml as 0 and 1 in the host OS.
#[repr(u32)]
pub enum QemuExitCode {
    /// This is literal 33: (0x10 << 1) | 1 = 33
    Success = 0x10,

    /// This is literal 35: (0x11 << 1) | 1 = 35
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_ADDRESS);
        port.write(exit_code as u32);
    }
}
