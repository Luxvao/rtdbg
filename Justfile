set positional-arguments

alias b := build

# Set up the project for compilation
setup:
    mkdir -p artefacts

# Build everything
build: assemble launcher gui

# Build runtime
runtime:
    cargo build --release -p rtdbg_runtime

# Build the launcher
launcher: runtime
    cargo build --release -p rtdbg

# Build assembly stuff
assemble: setup
    nasm -fbin asm/amd64/trampoline.asm -o artefacts/amd64_trampoline.bin

# Build the GUI component
gui: runtime
    cargo build --release -p rtdbg_gui

# Run the main binary, passing the parameters
run: build
    cargo run --release -p rtdbg

# Run the main GUI app
run_gui: gui
    cargo run --release -p rtdbg_gui

