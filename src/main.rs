// main.rs

#![no_std] //Unlink the standard library
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // To ensure the Rust compiler outputs a function with the name _start
pub extern "C" fn _start() -> ! {
    loop {}
}