use x86_64_custom::memory::address::VirtualMemoryAddress;

use crate::memory::Translator; //, Translator};

pub static mut TRANSLATOR: Translator = Translator::new(VirtualMemoryAddress::zero());
