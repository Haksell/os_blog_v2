#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{exit_qemu, serial_print, serial_println, QemuExitCode, TEST_OK};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("should_panic::should_fail... ");
    assert_eq!(0, 1);
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", TEST_OK);
    exit_qemu(QemuExitCode::Success);
    loop {}
}
