#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello ");
    print!("Wörld!");
    vga_buffer::WRITER.lock().write_byte(b' ');
    println!("The numbers are {} and {}", 42, 1.0 / 3.0);
    vga_buffer::WRITER
        .lock()
        .write_string("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
