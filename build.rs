use std::{fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let kernel = PathBuf::from(
        std::env::var_os("CARGO_BIN_FILE_NEXKERNEL_nexkernel")
            .expect("kernel artifact dependency was not built"),
    );

    let uefi_path = out_dir.join("nexos-uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .expect("failed to create UEFI disk image");

    let bios_path = out_dir.join("nexos-bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("failed to create BIOS disk image");

    let dist = PathBuf::from("dist");
    fs::create_dir_all(&dist).expect("failed to create dist");
    fs::copy(&uefi_path, dist.join("NexOS-x86_64-UEFI.img"))
        .expect("failed to copy UEFI image");
    fs::copy(&bios_path, dist.join("NexOS-x86_64-BIOS.img"))
        .expect("failed to copy BIOS image");

    println!("cargo:rerun-if-changed=kernel/src");
}
