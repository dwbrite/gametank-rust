fn main() {
    // Print the linker flags
    println!("cargo:rustc-link-search=native=target/asm");
    println!("cargo:rustc-link-lib=static=asm");
}