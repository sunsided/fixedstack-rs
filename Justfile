[private]
help:
    @just --list --unsorted

# Run benchmarks
bench:
    cargo bench

# Runs fuzzing (requires nightly Rust and cargo-fuzz)
fuzz MAX_TOTAL_TIME="60":
    # Nightly toolchain: rustup install nightly
    # Fuzz: cargo install cargo-fuzz
    cargo +nightly fuzz run fuzz_stack -- "-max_total_time={{ MAX_TOTAL_TIME }}"
