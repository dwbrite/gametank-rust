use std::process::Command;
use std::env;

fn main() {
    // Get the current directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // Assemble the 6502 assembly code using llvm-mc
    Command::new("llvm-mc")
        .args(&["--filetype=obj", "-triple=mos", "examples/my_function.asm", "-o"])
        .arg(&format!("{}/my_function.o", out_dir))
        .status()
        .expect("Failed to assemble the 6502 assembly code");

    // Create a static library using llvm-ar
    Command::new("llvm-ar")
        .args(&["rcs", &format!("{}/libmy_function.a", out_dir), &format!("{}/my_function.o", out_dir)])
        .status()
        .expect("Failed to create static library");

    // Print the linker flags
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=my_function");
}