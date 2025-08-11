#![no_std] // no standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // This is where the code starts
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}