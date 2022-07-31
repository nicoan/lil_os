#![no_std]
#![no_main]

static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Entrypoint of our OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // https://os.phil-opp.com/minimal-rust-kernel/#printing-to-screen
    let vga_buffer = 0xb8000 as *mut u8;

    let mut color = 1;

    for (i, &byte) in HELLO.iter().enumerate() {
        color += 1;
        if color > 16 {
            color = 1;
        }
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = color;
        }
    }

    loop {}
}
