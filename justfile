ensure-container-running:
    podman ps --filter "name=gametank" --filter "status=running" --format "{{{{.Names}}}}" | grep -q gametank || \
    podman run -d --name gametank -v $(pwd):/workspace:z --replace dwbrite/rust-mos:gametank-edition sleep infinity

# Compile all .asm files into .o files
assemble-asm-files: ensure-container-running
    podman exec -t -w /workspace gametank find . -name "*.asm" -exec bash -c ' \
        filename=$(basename "{}" .asm); \
        echo "Assembling $filename..."; \
        llvm-mc --filetype=obj -triple=mos "{}" -o "target/asm/$filename.o"' \;

# Archive the .o files into libasm.a
archive-asm-files: ensure-container-running
    podman exec -t -w /workspace gametank bash -c ' \
        llvm-ar rcs target/asm/libasm.a target/asm/*.o && \
        rm target/asm/*.o'

# Full build-asm task
build-asm: assemble-asm-files archive-asm-files

# Objcopy a compiled example to output.bin
objcopy-example example_name: ensure-container-running
    podman exec -t -w /workspace gametank llvm-objcopy -O binary target/mos-unknown-none/release/examples/{{example_name}} {{example_name}}.gtr

# Objdump a compiled example
objdump-example example_name: ensure-container-running
    podman exec -t -w /workspace gametank llvm-objdump -d --triple=mos target/mos-unknown-none/release/examples/{{example_name}}

# Build example compiling asm first
build-example example_name: build-asm
    podman exec -t -w /workspace gametank cargo +mos build --release --example {{example_name}} -Z build-std=core --target mos-unknown-none
    just objcopy-example {{example_name}}

# Run cargo fix an example with --allow-dirty
fix-example example_name:
    podman exec -t -w /workspace gametank cargo +mos fix --allow-dirty --release --example {{example_name}} -Z build-std=core --target mos-unknown-none

# List examples
list-examples: ensure-container-running
    podman exec -t -w /workspace gametank bash -c ' \
        output=$(cargo build --example 2>&1) && \
        echo "$$output" | tail -n +3 | sed "s/^[ \t]*//;s/[ \t]*$$//"'


