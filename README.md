# FixedStack: Safe and Unsafe Stack Implementations in Rust

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/rust-aligned-allocations/rust.yml)](https://github.com/sunsided/fixedstack-rs/actions/workflows/rust.yml)
[![Safety Dance][safety-image]][safety-link]
[![codecov](https://codecov.io/gh/sunsided/rust-aligned-allocations/graph/badge.svg?token=6CR3SYT3HT)](https://codecov.io/gh/sunsided/fixedstack-rs)


This project demonstrates two implementations of a stack in Rust: a **safe implementation** using `Vec` and an **unsafe implementation** using raw pointers for manual memory management. The project includes:
- **Benchmarks**: Comparing performance between safe and unsafe implementations using `criterion`.
- **Fuzzing**: Testing the robustness of the stack implementations using `cargo-fuzz`.

---

## Features

- **Safe Implementation**:
    - Uses `Vec` for memory safety.
    - Minimal risk of undefined behavior.
- **Unsafe Implementation**:
    - Manages memory manually using raw pointers.
    - Optimized for performance-critical use cases.
    - Tests allocation, deallocation, and pointer arithmetic.

---

## Requirements

- `cargo` for building and managing dependencies

For benchmarks and fuzzing:
- Install `criterion` for benchmarks.
- Install `cargo-fuzz` for fuzz testing (requires nightly toolchain).

---

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/sunsided/fixedstack-rs.git
cd fixedstack-rs
```

### Build the Project

```bash
just build
 # or
cargo build
```

### Run Tests

```bash
just test
 # or
cargo test
```

---

## Benchmarks

Benchmarks are implemented using the [Criterion](https://github.com/bheisler/criterion.rs) crate. To run benchmarks:

```bash
just bench
 # or
cargo bench
```

This compares the performance of `push` and `pop` operations for both safe and unsafe stack implementations.

---

## Fuzzing

Fuzz tests are implemented using [cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz/). Fuzzing helps uncover edge cases and undefined behavior, especially in the unsafe implementation.

### Install `cargo-fuzz`

```bash
cargo install cargo-fuzz
```

### Run the Fuzzer

```bash
just fuzz
 # or
cargo fuzz run fuzz_stack
```

### Debug Failing Inputs

When the fuzzer finds a failing input, the test case is saved in `fuzz/artifacts/fuzz_stack/`. To reproduce the failure:

```bash
cargo fuzz run fuzz_stack fuzz/artifacts/fuzz_stack/<failing-input>
```

You can also minimize failing inputs for easier debugging:

```bash
cargo fuzz tmin fuzz_stack fuzz/artifacts/fuzz_stack/<failing-input>
```

---

## Project Structure

```
.
├── src
│   ├── lib.rs                # Main library code for stack implementations
│   ├── stack_safe.rs         # Safe stack implementation
│   ├── stack_unsafe.rs       # Unsafe stack implementation
│
├── benches
│   └── benchmarks.rs         # Criterion benchmarks
│
├── fuzz
│   ├── fuzz_targets          # Directory for fuzzing targets
│   └── fuzz_stack.rs         # Fuzz target for stack testing
│
└── tests
    └── integration_tests.rs  # Integration tests for the stack implementations
```

---

## Example Usage

### Safe FixedStack

```rust
use stack_experiments::stack_unsafe::FixedStack;

fn main() {
    let mut stack = FixedStack::new(3);
    stack.push(1);
    stack.push(2);
    println!("{:?}", stack.pop()); // Output: Some(2)
    println!("{:?}", stack.pop()); // Output: Some(1)
}
```

### Unsafe FixedStack

```rust
use stack_experiments::stack_unsafe::FixedStack;

fn main() {
    let mut stack = FixedStack::new(3);
    stack.push(1);
    stack.push(2);
    println!("{:?}", stack.pop()); // Output: Some(2)
    println!("{:?}", stack.pop()); // Output: Some(1)
}
```

---

## Contributions

Contributions are welcome! Feel free to fork the repository, create a feature branch, and submit a pull request.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.


[safety-image]: https://img.shields.io/badge/unsafe-allowed-yellow.svg

[safety-link]: https://github.com/rust-secure-code/safety-dance/
