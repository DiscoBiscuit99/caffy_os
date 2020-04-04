#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(caffy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use caffy_os::{print, println};

pub mod serial;
pub mod interrupts;
pub mod gdt;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    caffy_os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);
    loop {}
}

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `_start` by default
    print!("Hello, World{}", "!");

    caffy_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not chrash!");

    loop {}
}

