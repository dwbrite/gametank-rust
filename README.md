What if Rust, on [GameTank](https://www.clydeshaffer.com/gametank/)?

# What?

The GameTank is a retro-inspired game console running dual 6502 processors. 
It has 128x128 composite out, with up to 200 colors. It's kind of like if Pico8 were real hardware -- 
with real hardware limitations, and neat tricks!

# Why?

I like Rust, I like GameTank. What if, both?

# How?

Rust compiles via LLVM. There is [LLVM-MOS](https://github.com/llvm-mos/llvm-mos/blob/main/README.md), 
which can target the 6502. Throw all that together and create some linker scripts, we _should_ have a stew.

## Requirements

- rust, I guess
- cmake
- ninja-build
- clang-devel maybe??
- libxml2-devel maybe??
- glibc-devel?

## Steps

### 1 / [Install LLVM-MOS](https://github.com/llvm-mos/llvm-mos/blob/main/README.md):

```bash
git clone https://github.com/mrk-its/llvm-mos
cd llvm-mos

echo "DO NOT RUN ME WITHOUT SETTING -DLIBXML2_LIBRARY"

cmake -C clang/cmake/caches/MOS.cmake -G "Ninja" -S llvm -B build \
   -DLLVM_INSTALL_TOOLCHAIN_ONLY=OFF \
   -DLLVM_BUILD_LLVM_DYLIB=ON -DLLVM_LINK_LLVM_DYLIB=ON \
   -DLLVM_INSTALL_UTILS=ON -DLLVM_BUILD_UTILS=ON -DLLVM_TOOLCHAIN_UTILITIES=FileCheck \
   -DLLVM_TOOLCHAIN_TOOLS="llvm-addr2line;llvm-ar;llvm-cxxfilt;llvm-dwarfdump;llvm-mc;llvm-nm;llvm-objcopy;llvm-objdump;llvm-ranlib;llvm-readelf;llvm-readobj;llvm-size;llvm-strings;llvm-strip;llvm-symbolizer;llvm-config;llc" \
   -DLIBXML2_LIBRARY=/usr/lib64/libxml2.so \
   -DLLVM_TARGETS_TO_BUILD="MOS;X86" \
   -DLLVM_ENABLE_PROJECTS="clang;lld;lldb"
cmake --build build -t install
```

The latest commit did not work for me, so I rolled back to 66eaba5, deleted the build directory, and then configuration worked.
Note that you'll need to set the LIBXML2_LIBRARY depending on your system.
For MacOS, use `-DLIBXML2_LIBRARY=/usr/local/opt/libxml2/lib/libxml2.dylib`,
debian and others `-DLIBXML2_LIBRARY=/usr/lib/x86_64-linux-gnu/libxml2.so`,
etc.


### 3 / Install rust-mos toolchain

...


### 4 / rust rust rust

`rustup override set mos`
???

### 5 / compiling and testing

`cargo build --release --example hello`
`./../tmp/llvm-mos/build/bin/llvm-objdump -d --triple=mos target/mos-unknown-none/release/examples/hello`
`./../tmp/llvm-mos/build/bin/llvm-objcopy -O binary target/mos-unknown-none/release/examples/hello output.bin`

`cargo install cargo-binutils`

# TODO: figure out why cargo binutils can't find share objects



