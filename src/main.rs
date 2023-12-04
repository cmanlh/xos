#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(xos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use xos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    xos::init();

    println!("It did not crash!");
    xos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xos::test_panic_handler(info)
}
