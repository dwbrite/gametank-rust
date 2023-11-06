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

# Programming the gametank

This repository is _basically_ the rust SDK for the gametank.
As such, I've included a few utilities to make life easier.

## Development Docker Container

First thing is a docker container where I've built llvm-mos + rust-mos, and a few utilities to make life easier.

Just run the container with the project directory mounted as a volume. 
You'll get access to a fully configured llvm/rust-mos toolchain within the container, 
while also being able to edit code in your local editor.
I use IntelliJ as my IDE of choice, with an interactive terminal into the container at the bottom.

    docker pull dwbrite/rust-mos:gametank-edition

    docker run -it --rm -v ./:/workspace:z dwbrite/rust-mos:gametank-edition

On Windows/WSL, you may need to modify the workspace volume URL.

I would recommend creating your project as a new example, 
since it's slightly easier than creating a whole template from this repository.

## Justfile

The justfile is a sort of living document and handy tool for your workflow.
The docker container even includes autocomplete for just.

Most of the time you only need `just build-example <example-name>`, which produces an output.bin file you can load into [the emulator](https://clydeshaffer.com/builds/GameTankEmulator/wasm/?rom=badapple.gtr).

## Assembly

Unfortunately, the current version of rust-mos doesn't support inline assembly.
We work around this by including assembly as a single static library, 
compiled from every .asm file found (recursively) in this directory.
This solution is _okay_, only because dead code is shaken out by the linker.

This also means that functions aren't often (ever?) inlined in the final binary, 
and there may be some weird issues with function calls to assembly being jmp rather than jsr.
Not sure what that's about.

## Docs

You can find programming information about the gametank at 
the [gametank wiki](https://wiki.gametank.zone/doku.php?id=start), 
the [programming manual](https://gametank.zone/manual/), 
as well as some examples/games in C and assembly on 
[Clyde's github](https://github.com/clydeshaffer).
For example: [Accursed Fiend](https://github.com/clydeshaffer/fiend)