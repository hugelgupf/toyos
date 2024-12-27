#![no_std]
#![no_main]

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
    // assert_ne!(1, 1);
    loop {}
}
