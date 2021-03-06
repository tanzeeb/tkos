#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tkos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tkos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "Loading {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tkos::test_panic_handler(info)
}
