# compile *all* .asm files into a libasm.a. The linker will shake off the dead ones ;)
build-asm:
    # Ensure the target directory exists
    mkdir -p target/asm

    # Find all .asm files and compile them into .o files, placing them in the target/asm directory
    for file in $(find . -name "*.asm"); do \
        filename="$(basename "$file" .asm)"; \
        echo "Assembling $filename..."; \
        llvm-mc --filetype=obj -triple=mos "$file" -o "target/asm/$filename.o"; \
    done

    # Create the static library from all .o files
    llvm-ar rcs target/asm/libasm.a target/asm/*.o

    # clean up the object files
    rm target/asm/*.o

# objcopy a compiled example to output.bin
objcopy-example example_name:
    llvm-objcopy -O binary target/mos-unknown-none/release/examples/{{example_name}} {{example_name}}.gtr

# objdump a compiled example (so like, build it first)
objdump-example example_name:
    llvm-objdump -d --triple=mos target/mos-unknown-none/release/examples/{{example_name}}

# build example compiling asm first
build-example example_name:
    just build-asm
    cargo build --release --example {{example_name}} -Z build-std=core --target mos-unknown-none
    just objcopy-example {{example_name}}

# do you really need a doc comment?
list-examples:
    #!/bin/bash
    output=$(cargo build --example 2>&1)
    echo "$output" | tail -n +3 | sed 's/^[ \t]*//;s/[ \t]*$//'


