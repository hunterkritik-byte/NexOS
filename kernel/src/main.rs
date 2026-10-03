#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use uart_16550::{Config, SerialPort};

entry_point!(kernel_main);

fn serial() -> SerialPort {
    let mut serial = unsafe { SerialPort::new(0x3F8) };
    serial.init();
    serial
}

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let mut serial = serial();
    writeln!(serial, "NexKernel {}", env!("CARGO_PKG_VERSION")).ok();
    writeln!(serial, "architecture: x86_64").ok();
    writeln!(serial, "status: kernel entry reached").ok();

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial = serial();
    writeln!(serial, "NEXKERNEL PANIC: {info}").ok();

    loop {
        core::hint::spin_loop();
    }
}
