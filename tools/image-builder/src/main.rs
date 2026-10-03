use bootloader::{BiosBoot, UefiBoot};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("usage: nexos-image-builder <kernel> <ramdisk> <bios-output> <uefi-output>");
        std::process::exit(2);
    }

    let kernel = PathBuf::from(&args[1]);
    let ramdisk = PathBuf::from(&args[2]);
    let bios = PathBuf::from(&args[3]);
    let uefi = PathBuf::from(&args[4]);

    if !kernel.exists() {
        eprintln!("kernel does not exist: {}", kernel.display());
        std::process::exit(1);
    }
    if !ramdisk.exists() {
        eprintln!("ramdisk does not exist: {}", ramdisk.display());
        std::process::exit(1);
    }

    for output in [&bios, &uefi] {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).expect("failed to create output directory");
        }
    }

    BiosBoot::new(&kernel)
        .set_ramdisk(&ramdisk)
        .create_disk_image(&bios)
        .expect("failed to create BIOS boot image");

    UefiBoot::new(&kernel)
        .set_ramdisk(&ramdisk)
        .create_disk_image(&uefi)
        .expect("failed to create UEFI boot image");

    println!("BIOS image: {}", bios.display());
    println!("UEFI image: {}", uefi.display());
    println!("Ramdisk: {}", ramdisk.display());
}
