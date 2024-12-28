#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupts;
mod serial;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("MESS KERNEL PANIC: {}", info);
    loop {}
}

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    serial_println!("Hello World");

    interrupts::init_idt();

    // invoke panic handler.
    //assert_ne!(1, 1);
    // invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    loop {}
}
