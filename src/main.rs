fn main() {
    println!("NexOS build workspace");
    println!("NexKernel is built with:");
    println!("  cargo build --release --package nexkernel --target x86_64-unknown-none");
    println!();
    println!("Boot-image generation is intentionally separate from the stable workspace.");
}
