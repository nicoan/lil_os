use x86_64_custom::memory::address::VirtualMemoryAddress;
use x86_64_custom::memory::Translator;

pub static mut TRANSLATOR: Translator = Translator::new(VirtualMemoryAddress::zero());

// NOTE: For debug
// use crate::memory::Translator; //, Translator};
