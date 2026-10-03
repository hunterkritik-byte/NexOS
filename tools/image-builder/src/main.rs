use bootloader::{BiosBoot, UefiBoot};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn create_initramfs(shell: &Path, output: &Path) {
    let elf = fs::read(shell).expect("failed to read NexShell ELF");
    let name = b"/bin/nexshell";
    let header_len = 16usize;
    let entry_len = 80usize;
    let data_offset = header_len + entry_len;
    let mut image = vec![0u8; data_offset + elf.len()];

    image[0..4].copy_from_slice(b"NEXR");
    image[4..8].copy_from_slice(&1u32.to_le_bytes());
    image[8..12].copy_from_slice(&1u32.to_le_bytes());
    image[12..16].copy_from_slice(&(data_offset as u32).to_le_bytes());

    image[16..16 + name.len()].copy_from_slice(name);
    image[80..88].copy_from_slice(&(data_offset as u64).to_le_bytes());
    image[88..96].copy_from_slice(&(elf.len() as u64).to_le_bytes());
    image[data_offset..].copy_from_slice(&elf);

    fs::write(output, image).expect("failed to write NexOS initramfs");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("usage: nexos-image-builder <kernel> <nexshell> <bios-output> <uefi-output>");
        std::process::exit(2);
    }

    let kernel = PathBuf::from(&args[1]);
    let shell = PathBuf::from(&args[2]);
    let bios = PathBuf::from(&args[3]);
    let uefi = PathBuf::from(&args[4]);
    let initramfs = shell.with_file_name("NexOS-initramfs.bin");

    if !kernel.exists() {
        eprintln!("kernel does not exist: {}", kernel.display());
        std::process::exit(1);
    }
    if !shell.exists() {
        eprintln!("NexShell does not exist: {}", shell.display());
        std::process::exit(1);
    }

    for output in [&bios, &uefi] {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).expect("failed to create output directory");
        }
    }

    create_initramfs(&shell, &initramfs);

    BiosBoot::new(&kernel)
        .set_ramdisk(&initramfs)
        .create_disk_image(&bios)
        .expect("failed to create BIOS boot image");

    UefiBoot::new(&kernel)
        .set_ramdisk(&initramfs)
        .create_disk_image(&uefi)
        .expect("failed to create UEFI boot image");

    println!("BIOS image: {}", bios.display());
    println!("UEFI image: {}", uefi.display());
    println!("Initramfs: {}", initramfs.display());
}
