set positional-arguments

alias b := build

# Build everything
build: runtime launcher gui

# Build runtime
runtime:
    cargo build --release -p rtdbg_runtime

# Build the launcher
launcher:
    cargo build --release -p rtdbg

# Build the GUI component
gui:
    cargo build --release -p rtdbg_gui

# Run the main binary, passing the parameters
run: build
    cargo run --release -p rtdbg

# Run the main GUI app
run_gui: gui
    cargo run --release -p rtdbg_gui

