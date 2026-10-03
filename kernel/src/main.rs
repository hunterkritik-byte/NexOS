#![cfg_attr(target_arch = "x86_64", no_std)]
#![cfg_attr(target_arch = "x86_64", no_main)]

#[cfg(target_arch = "x86_64")]
mod x86_kernel {
    use bootloader_api::{BootInfo, entry_point};
    use core::fmt::Write;
    use uart_16550::{Config, Uart16550Tty};

    entry_point!(kernel_main);

    fn serial() -> Uart16550Tty<uart_16550::backend::PioBackend> {
        unsafe {
            Uart16550Tty::new_port(0x3F8, Config::default()).expect("failed to initialize COM1")
        }
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
}

#[cfg(not(target_arch = "x86_64"))]
fn main() {
    // The kernel is x86_64-only. This host stub lets cargo check run on ARM64 Termux.
}
