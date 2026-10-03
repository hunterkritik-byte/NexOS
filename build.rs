use std::path::PathBuf;

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

    println!("cargo:rustc-env=NEXOS_UEFI_IMAGE={}", uefi_path.display());
    println!("cargo:rustc-env=NEXOS_BIOS_IMAGE={}", bios_path.display());
    println!("cargo:rerun-if-changed=kernel/src");
}
