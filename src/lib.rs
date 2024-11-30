//! # stack-experiments
//!
//! This crate provides the implementation of fixed-capacity stacks using two different approaches:
//! unsafe memory management and safe memory management. It is composed of two submodules:
//! `stack_unsafe` and `stack_safe`.
//!
//! ## Submodules
//!
//! ### `stack_unsafe`
//!
//! The `stack_unsafe` module implements a fixed-size stack using raw pointers for memory management.
//! This approach allows for potentially more efficient stack operations by avoiding the overhead
//! of safe memory handling mechanisms, but it requires careful handling to avoid common pitfalls
//! such as memory leaks or undefined behavior.
//!
//! - **`FixedStack<T>`**: A stack that uses raw pointers to manage a fixed-size buffer.
//!   - Provides methods for basic stack operations like `push` and `pop`.
//!   - Includes a custom `Drop` implementation to ensure manual deallocation of the stack's buffer.
//!   - Includes tests to ensure functionality works as expected and to test for stack overflow panics.
//!
//! ### `stack_safe`
//!
//! The `stack_safe` module implements a fixed-size stack using Rust's `Vec` for automatic and safe
//! memory management. This offers a more secure and straightforward way to manage resources, with
//! Rust ensuring memory safety at compile time.
//!
//! - **`FixedStack<T>`**: A stack that utilizes a `Vec` to handle a fixed-size buffer safely.
//!   - Provides basic stack operations similar to its unsafe counterpart, including `push`, `pop`, `is_empty`,
//!     `len`, and `capacity`.
//!   - Includes tests to confirm correct behavior and handling of stack overflow scenarios.
//!
//! ## Usage Notes
//!
//! - **Safe vs Unsafe**: Choose `stack_safe` for ease of use and safety guarantees. Opt for `stack_unsafe`
//!   only if you have specific performance needs and are experienced with Rust's unsafe code.
//! - The `FixedStack` implementations in both modules require a capacity greater than zero and will panic
//!   if this condition is not met.
//!
//! This module caters to developers needing fixed-capacity stacks, providing a choice between safety
//! and potentially optimized performance.
#![allow(unsafe_code)]

pub mod stack_safe;
pub mod stack_unsafe;
