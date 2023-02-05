//! Interrupt descriptor table initialization
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.init();
        idt
    };
}

pub(crate) fn load_idt() {
    IDT.load();
}
