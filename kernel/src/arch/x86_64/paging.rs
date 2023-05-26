use x86_64_custom::memory::{address::VirtualMemoryAddress, Translator};

pub static mut TRANSLATOR: Translator = Translator::new(VirtualMemoryAddress::zero());
