[private]
help:
    @just --list --unsorted

# Builds the project using cargo
build:
    cargo build

# Runs the tests
test:
    cargo test --lib
    cargo test --doc

# Runs the benchmarks
bench:
    cargo bench

# Runs fuzzing (requires nightly Rust and cargo-fuzz)
fuzz MAX_TOTAL_TIME="60":
    # Nightly toolchain: rustup install nightly
    # Fuzz: cargo install cargo-fuzz
    cargo +nightly fuzz run fuzz_stack -- "-max_total_time={{ MAX_TOTAL_TIME }}"
