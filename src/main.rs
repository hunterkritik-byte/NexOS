use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::env;
use std::process::{exit, Command};

fn main() {
    let mode = match env::args().nth(1).as_deref() {
        Some("uefi") => true,
        Some("bios") => false,
        Some("-h") | Some("--help") => {
            println!("Usage: cargo run -- <uefi|bios>");
            return;
        }
        _ => {
            eprintln!("Usage: cargo run -- <uefi|bios>");
            exit(2);
        }
    };

    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-serial").arg("mon:stdio");
    qemu.arg("-display").arg("none");
    qemu.arg("-device").arg("isa-debug-exit,iobase=0xf4,iosize=0x04");

    if mode {
        let prebuilt =
            Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("failed to fetch OVMF firmware");
        let code = prebuilt.get_file(Arch::X64, FileType::Code);
        let vars = prebuilt.get_file(Arch::X64, FileType::Vars);

        qemu.arg("-drive")
            .arg(format!("format=raw,file={}", env!("NEXOS_UEFI_IMAGE")));
        qemu.arg("-drive").arg(format!(
            "if=pflash,format=raw,unit=0,file={},readonly=on",
            code.display()
        ));
        qemu.arg("-drive").arg(format!(
            "if=pflash,format=raw,unit=1,file={},snapshot=on",
            vars.display()
        ));
    } else {
        qemu.arg("-drive")
            .arg(format!("format=raw,file={}", env!("NEXOS_BIOS_IMAGE")));
    }

    let status = qemu.status().expect("failed to start qemu-system-x86_64");
    match status.code() {
        Some(0x10) | Some(0) => {}
        Some(0x11) => exit(1),
        _ => exit(2),
    }
}
