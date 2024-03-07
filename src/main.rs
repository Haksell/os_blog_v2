#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Wesh la famille ! {}", 6 * 7);
    blog_os::init();

    #[cfg(test)]
    test_main();

    println!("ooga booga cya");
    use blog_os::print;
    let mut c: u8 = 32;
    loop {
        print!("{}", c as char);
        c = if c == 126 { 32 } else { c + 1 };
        for _ in 0..10000 {
            volatile::Volatile::new(0).read();
        }
    }
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
    blog_os::test_panic_handler(info)
}
