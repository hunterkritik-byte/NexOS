fn main() {
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rustc-link-arg-bin=nexshell=-Tuserland/nexshell/link.ld");
}
