#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]

#![test_runner(caffy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use caffy_os::{print, println, hlt_loop};

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
    hlt_loop();
}

entry_point!(kernel_main); // Feed the entry point for the kernel defined by the boot loader.

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use caffy_os::allocator;
    use caffy_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello, World{}", "!");
    caffy_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // Allocate a number on the heap.
    let heap_value = Box::new(42);
    println!("heap_value at {:p}", heap_value);

    // Create a dynamically sized vector.
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference counted vector -> will be freed when count reaches 0.
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not chrash!");
    hlt_loop();
}

