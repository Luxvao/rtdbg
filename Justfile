set positional-arguments

alias b := build

# Set up the project for compilation
setup:
    mkdir -p artefacts

# Build everything
build: assemble build_cli

# Builds a specific C example
build_c_example example:
    clang -o ./artefacts/{{example}} ./c_examples/{{example}}.c

# Build runtime
runtime: assemble
    cargo build --release -p rtdbg_runtime

# Build the launcher
build_cli: runtime
    cargo build --release -p rtdbg

# Build assembly stuff
assemble: setup
    nasm -fbin asm/amd64/trampoline.asm -o artefacts/amd64_trampoline.bin

# Build the GUI component
build_gui: runtime
    cargo build --release -p rtdbg_gui

build_tui: runtime
    cargo build --release -p rtdbg_tui

# Run the main binary, passing the arguments
cli *args: build
    cargo run --release -p rtdbg -- {{args}}

# Run the tui, passing in the arguments
tui *args: build_tui
    cargo run --release -p rtdbg_tui -- {{args}}

# Run the tui with scripts (rhai examples passed in)
tui_ws:
    just tui --scripts ./rhai_examples

# Run the main GUI app
gui: build_gui
    cargo run --release -p rtdbg_gui

# Generate a .d.rhai file to be consumed by the rhai LSP
completions:
    cargo run --example gen_api

# Clean everything
clean:
    cargo clean
    rm -fr artefacts
