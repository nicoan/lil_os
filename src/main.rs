#![no_std]
#![no_main]

pub mod drivers;

use core::panic::PanicInfo;

use crate::drivers::screen::text::Color;

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entrypoint of our OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Running test 1... [");
    print!([Color::Green], "ok");
    println!("]");
    print!("Running test 2... [");
    print!([Color::Red], "fail");
    println!("]");
    println!([Color::Yellow], "Hello world 1!");
    println!([Color::Cyan, Color::Blue], "Hello world 2!",);
    println!([Color::Brown, Color::Cyan], "Hello world 3!",);
    println!([Color::Cyan], "The numbers are {} and {}", 42, 1.0 / 3.0);
    println!("The numbers are {} and {}", 42, 1.0 / 3.0);

    loop {}
}
