use x86_64_custom::address::VirtualMemoryAddress;
use x86_64_custom::paging::Translator;

pub static mut TRANSLATOR: Translator = Translator::new(VirtualMemoryAddress::zero());
