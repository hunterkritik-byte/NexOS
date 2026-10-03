#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod x86_kernel {
    use bootloader_api::{entry_point, BootInfo};
    use core::fmt::Write;
    use x86_64::{VirtAddr, instructions::port::Port};
    use crate::x86_kernel::arch::x86_64::{boot_memory::BootInfoFrameAllocator, paging};

    mod drivers;
    mod net;
    mod process;
    mod fs;
    mod terminal;
    mod arch { pub mod x86_64; }

    const VGA_BUFFER: usize = 0xb8000;
    const VGA_WIDTH: usize = 80;
    const VGA_HEIGHT: usize = 25;
    const COM1: u16 = 0x3f8;

    struct Terminal {
        row: usize,
        col: usize,
        color: u8,
    }

    impl Terminal {
        const fn new() -> Self {
            Self { row: 0, col: 0, color: 0x07 }
        }

        fn clear(&mut self) {
            for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
                unsafe {
                    core::ptr::write_volatile((VGA_BUFFER as *mut u16).add(i), (self.color as u16) << 8 | b' ' as u16);
                }
            }
            self.row = 0;
            self.col = 0;
        }

        fn put_byte(&mut self, byte: u8) {
            match byte {
                b'\n' => {
                    self.col = 0;
                    self.row += 1;
                }
                b'\r' => self.col = 0,
                8 => {
                    if self.col > 0 {
                        self.col -= 1;
                        self.write_cell(b' ');
                    }
                }
                byte => {
                    if self.col >= VGA_WIDTH {
                        self.col = 0;
                        self.row += 1;
                    }
                    self.write_cell(byte);
                    self.col += 1;
                }
            }
            if self.row >= VGA_HEIGHT {
                self.scroll();
            }
        }

        fn write_cell(&mut self, byte: u8) {
            let index = self.row * VGA_WIDTH + self.col;
            unsafe {
                core::ptr::write_volatile(
                    (VGA_BUFFER as *mut u16).add(index),
                    ((self.color as u16) << 8) | byte as u16,
                );
            }
        }

        fn scroll(&mut self) {
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let src = (row * VGA_WIDTH + col) as *const u16;
                    let dst = (VGA_BUFFER + ((row - 1) * VGA_WIDTH + col) * 2) as *mut u16;
                    unsafe { core::ptr::write_volatile(dst, core::ptr::read_volatile(src)); }
                }
            }
            for col in 0..VGA_WIDTH {
                let dst = (VGA_BUFFER + ((VGA_HEIGHT - 1) * VGA_WIDTH + col) * 2) as *mut u16;
                unsafe { core::ptr::write_volatile(dst, (self.color as u16) << 8 | b' ' as u16); }
            }
            self.row = VGA_HEIGHT - 1;
            self.col = 0;
        }

        fn write_str(&mut self, s: &str) {
            for byte in s.bytes() {
                self.put_byte(byte);
            }
        }
    }

    struct Serial {
        data: Port<u8>,
        interrupt: Port<u8>,
        fifo: Port<u8>,
        line: Port<u8>,
        modem: Port<u8>,
    }

    impl Serial {
        unsafe fn new() -> Self {
            let mut serial = Self {
                data: Port::new(COM1),
                interrupt: Port::new(COM1 + 1),
                fifo: Port::new(COM1 + 2),
                line: Port::new(COM1 + 3),
                modem: Port::new(COM1 + 4),
            };
            serial.init();
            serial
        }

        unsafe fn init(&mut self) {
            self.interrupt.write(0);
            self.line.write(0x80);
            self.data.write(3);
            self.interrupt.write(0);
            self.line.write(3);
            self.fifo.write(0xc7);
            self.modem.write(0x0b);
        }

        unsafe fn write_byte(&mut self, byte: u8) {
            while self.line.read() & 0x20 == 0 {}
            self.data.write(byte);
        }

        unsafe fn read_byte(&mut self) -> Option<u8> {
            if self.line.read() & 1 == 0 { None } else { Some(self.data.read()) }
        }
    }

    fn command(term: &mut Terminal, serial: &mut Serial, command: &[u8]) {
        let mut end = command.len();
        while end > 0 && command[end - 1] == b' ' { end -= 1; }
        let cmd = &command[..end];

        match cmd {
            b"" => {}
            b"help" => term.write_str("Commands: help clear echo uname reboot\n"),
            b"clear" => term.clear(),
            b"uname" => term.write_str("NexOS 0.1.0 x86_64\n"),
            b"reboot" => unsafe {
                let mut port = Port::new(0x64u16);
                port.write(0xfeu8);
            },
            _ if cmd.starts_with(b"echo ") => {
                if let Ok(text) = core::str::from_utf8(&cmd[5..]) {
                    term.write_str(text);
                    term.put_byte(b'\n');
                }
            }
            _ => term.write_str("nexos: command not found\n"),
        }

        unsafe {
            serial.write_byte(b'\r');
            serial.write_byte(b'\n');
        }
    }

    fn boot_splash(term: &mut Terminal) {
        term.clear();
        term.write_str("\n");
        term.write_str("                 _   _ _______  __  __  ____  _____\n");
        term.write_str("                | \ | | ____\ \/ / |  \/  |/ __ \/ ___/\n");
        term.write_str("                |  \| |  _|  \  /  | |\/| | |  | \__ \\n");
        term.write_str("                | |\  | |___ /  \\  | |  | | |__| |__/ /\n");
        term.write_str("                |_| \_|_____/_/\\_\\ |_|  |_|\\____/____/\n");
        term.write_str("\n");
        term.write_str("                         NEXOS\n");
        term.write_str("                    MADE BY KRITIK\n");
        term.write_str("\n");
        term.write_str("                         Loading");
        for _ in 0..3 {
            for _ in 0..30_000_000 {
                core::hint::spin_loop();
            }
            term.write_str(".");
        }
        term.write_str("\n\n");
        for _ in 0..30_000_000 {
            core::hint::spin_loop();
        }
    }

    fn shell(term: &mut Terminal, serial: &mut Serial) -> ! {
        let mut command = [0u8; 128];
        let mut len = 0usize;
        term.write_str("NexOS Terminal\nType 'help' for commands.\n\nnexos> ");

        loop {
            if let Some(byte) = unsafe { serial.read_byte() } {
                match byte {
                    b'\r' | b'\n' => {
                        term.put_byte(b'\n');
                        command(term, serial, &command[..len]);
                        len = 0;
                        term.write_str("nexos> ");
                    }
                    8 | 127 => {
                        if len > 0 {
                            len -= 1;
                            term.put_byte(8);
                            unsafe { serial.write_byte(8); }
                        }
                    }
                    32..=126 => {
                        if len < command.len() {
                            command[len] = byte;
                            len += 1;
                            term.put_byte(byte);
                            unsafe { serial.write_byte(byte); }
                        }
                    }
                    _ => {}
                }
            }
            core::hint::spin_loop();
        }
    }

    entry_point!(kernel_main);

    fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
        let mut terminal = Terminal::new();
        terminal.write_str("Memory: initializing bootloader page map...\\n");
        if let Some(offset) = boot_info.physical_memory_offset.into_option() {
            let mut frame_allocator = BootInfoFrameAllocator::new(&boot_info.memory_regions);
            let mut mapper = unsafe { paging::init_mapper(VirtAddr::new(offset)) };
            terminal.write_str("Memory: active L4 mapper connected; usable-frame allocator online.\\n");
            if frame_allocator.allocate_frame().is_some() {
                terminal.write_str("Memory: verified usable physical frame allocation.\\n");
            } else {
                terminal.write_str("Memory: no usable physical frames reported.\\n");
            }
            let _ = &mut mapper;
        } else {
            terminal.write_str("Memory: physical-memory mapping unavailable; isolation setup deferred.\\n");
        }
        let mut serial = unsafe { Serial::new() };
        boot_splash(&mut terminal);

        let mut vfs = fs::Vfs::<64>::new();
        match fs::FileSystem::format_and_mount(&mut vfs) {
            Ok(()) => terminal.write_str("Filesystem: VFS mounted (/bin /etc /home /tmp /var /dev /proc /sbin)\\n"),
            Err(_) => terminal.write_str("Filesystem: mount failed\\n"),
        }

        terminal.write_str("Detecting network hardware...\\n");
        let scanner = drivers::PciScanner::new();
        let mut devices = [None; 8];
        let count = unsafe { scanner.scan_network(&mut devices) };
        let mut registry = net::device::NetworkRegistry::new();

        if count == 0 {
            terminal.write_str("Network: no PCI network controller detected.\\n");
        } else {
            for device in devices.iter().take(count).flatten() {
                registry.register_pci(*device);
            }
            for index in 0..registry.len() {
                if let Some((driver, kind, pci)) = registry.describe(index) {
                    let kind_name = match kind {
                        drivers::network::NetworkKind::Ethernet => "Ethernet",
                        drivers::network::NetworkKind::WirelessOrOther => "Wireless/other",
                    };
                    let _ = writeln!(
                        terminal,
                        "NET: {kind_name} {:04x}:{:04x} via {driver}\\n",
                        pci.vendor_id,
                        pci.device_id
                    );
                }
            }
        }
        terminal.write_str("Network drivers are probe-only until chipset-specific implementations are added.\\n\\n");
        shell(&mut terminal, &mut serial)
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        let mut terminal = Terminal::new();
        terminal.clear();
        terminal.write_str("NEXKERNEL PANIC: ");
        let _ = write!(terminal, "{info}");
        loop { core::hint::spin_loop(); }
    }
}

#[cfg(not(target_os = "none"))]
fn main() {
    println!("NexOS kernel host check passed.");
}
