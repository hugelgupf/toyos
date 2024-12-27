#![no_std]
#![no_main]

mod serial;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    serial_println!("Hello World");
    loop {}
}
