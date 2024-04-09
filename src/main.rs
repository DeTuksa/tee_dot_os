// main.rs

#![no_std] //Unlink the standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // To ensure the Rust compiler outputs a function with the name _start
pub extern "C" fn _start() -> ! {
    loop {}
}