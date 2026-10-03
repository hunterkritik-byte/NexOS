use bootloader::{BiosBoot, UefiBoot};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "usage: nexos-image-builder <kernel> <bios-output> <uefi-output>"
        );
        std::process::exit(2);
    }

    let kernel = PathBuf::from(&args[1]);
    let bios = PathBuf::from(&args[2]);
    let uefi = PathBuf::from(&args[3]);

    if !kernel.exists() {
        eprintln!("kernel does not exist: {}", kernel.display());
        std::process::exit(1);
    }

    if let Some(parent) = bios.parent() {
        std::fs::create_dir_all(parent).expect("failed to create BIOS output directory");
    }

    if let Some(parent) = uefi.parent() {
        std::fs::create_dir_all(parent).expect("failed to create UEFI output directory");
    }

    BiosBoot::new(&kernel)
        .create_disk_image(&bios)
        .expect("failed to create BIOS boot image");

    UefiBoot::new(&kernel)
        .create_disk_image(&uefi)
        .expect("failed to create UEFI boot image");

    println!("BIOS image: {}", bios.display());
    println!("UEFI image: {}", uefi.display());
}
