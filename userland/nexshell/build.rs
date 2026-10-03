use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let linker = format!("{manifest_dir}/link.ld");
    println!("cargo:rerun-if-changed={linker}");
    println!("cargo:rustc-link-arg-bin=nexshell=-T{linker}");
}
