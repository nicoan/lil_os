#![no_std]
#![no_main]
//! Since we are in a non-standard environment, we should define our own test framework.
#![feature(custom_test_frameworks)]
//! This is the entry-point to our test framework.
#![test_runner(lil_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;

/// Entrypoint of our OS
#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    use lil_os::arch::x86_64::TRANSLATOR;
    use lil_os::{
        arch::x86_64::initialize_x86_64_arch, os_core::messages::init_with_message, println,
    };
    use x86_64::structures::paging::PhysFrame;
    use x86_64_custom::memory::address::{PhysicalMemoryAddress, VirtualMemoryAddress};
    use x86_64_custom::memory::frame_allocator::DummyAllocator;
    use x86_64_custom::memory::mapper::Mapper;

    let physical_memory_offset = VirtualMemoryAddress::new(boot_info.physical_memory_offset);

    init_with_message("x86_64 architecture", || {
        initialize_x86_64_arch(physical_memory_offset)
    });

    println!("Translated address: {:?}", unsafe {
        TRANSLATOR.translate_address(VirtualMemoryAddress::new(0xb8000))
    });

    println!("Translated address: {:?}", unsafe {
        TRANSLATOR.translate_address(VirtualMemoryAddress::new(boot_info.physical_memory_offset))
    });

    fn test_map(physical_memory_offset: VirtualMemoryAddress) {
        use x86_64_custom::memory::paging::frame::Frame;
        use x86_64_custom::memory::paging::page::Page;
        use x86_64_custom::memory::paging::page_size::Size4KiB;
        use x86_64_custom::memory::paging::page_table::PageTableEntryFlags;
        let frame = Frame::<Size4KiB>::containing_address(PhysicalMemoryAddress::new(0xb8000));
        let frame2 = PhysFrame::<x86_64::structures::paging::Size4KiB>::containing_address(
            x86_64::PhysAddr::new(0xb8000),
        );
        println!("{:?} {:?}", frame.start_address(), frame2.start_address());
        let flags = PageTableEntryFlags::PRESENT
            | PageTableEntryFlags::WRITABLE
            | PageTableEntryFlags::USER_ACCESSIBLE;

        // map an unused page
        let page = Page::<Size4KiB>::containing_address(VirtualMemoryAddress::new(0));
        // let page2 = x86_64::structures::paging::Page::<x86_64::structures::paging::Size4KiB>::containing_address(
        //     x86_64::VirtAddr::new(0),
        // );
        // println!("{:?} {:?}", page, page2);
        let mapper = Mapper::<Size4KiB>::new(physical_memory_offset);

        unsafe {
            println!("le map: {}", mapper.map(page, frame, DummyAllocator, flags));

            println!(
                "Translated address: {:?}",
                TRANSLATOR.translate_address(VirtualMemoryAddress::new(0x0))
            );

            println!("test2");
            // write the string `New!` to the screen through the new mapping
            let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
            page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
            // println!("{:?}", &page_ptr.offset(400));

            // page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
        }
    }

    test_map(physical_memory_offset);

    #[allow(clippy::empty_loop)]
    loop {
        // Halts the CPU until the next interrupt hits. This prevents the CPU to spin endessly
        // and waste cycles doing nothing.
        x86_64::instructions::hlt();
    }
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use lil_os::println;
    println!("{}", info);
    #[allow(clippy::empty_loop)]
    loop {
        // Halts the CPU after the panic
        x86_64::instructions::hlt();
    }
}

// Unit testing entry points and handlers.
// Here we define the custom test framework entrypoint and the panic handler. We need this
// functions declared here in main.rs. Most of the test logic is contained in the test module.

/// Custom test framework entry point.
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Custom test framework panic handler.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lil_os::tests::test_panic_handler(info)
}
