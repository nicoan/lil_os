pub static PICS: Mutex<IBMPcAt8259> = Mutex::new(IBMPcAt8259::new());

create_interrupt_handler!(timer_interrupt_handler, InterruptIndex::Timer, PICS, {
    timer_handler();
});

create_interrupt_handler!(
    keyboard_interrupt_handler,
    InterruptIndex::Keyboard,
    PICS,
    {
        use x86_64::instructions::port::Port;

        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };

        keyboard_handler(scancode);
    }
);
