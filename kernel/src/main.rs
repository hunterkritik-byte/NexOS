#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::fmt::Write;
use uart_16550::backend::PioBackend;
use uart_16550::{Config, Uart16550Tty};

entry_point!(kernel_main);

fn serial() -> Uart16550Tty<PioBackend> {
    unsafe { Uart16550Tty::new_port(0x3F8, Config::default()) }
        .expect("COM1 must be available during early x86_64 bring-up")
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut serial = serial();

    writeln!(serial, "NexKernel {}", env!("CARGO_PKG_VERSION")).ok();
    writeln!(serial, "architecture: x86_64").ok();
    writeln!(serial, "status: kernel entry reached").ok();
    writeln!(serial, "boot_info: {boot_info:?}").ok();

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
