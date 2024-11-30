//! # Module Overview
//!
//! This module provides the implementation of fixed-capacity stacks using two different approaches:
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

pub struct FixedStack<T> {
    buffer: Vec<T>,   // Vec manages memory safely
    capacity: usize,  // Fixed capacity of the stack
}

impl<T> FixedStack<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than zero");
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.buffer.len() >= self.capacity {
            panic!("Stack overflow");
        }
        self.buffer.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_fixed_stack() {
        let mut stack = FixedStack::new(3);

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.capacity(), 3);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_safe_fixed_stack_overflow() {
        let mut stack = FixedStack::new(2);
        stack.push(1);
        stack.push(2);
        stack.push(3); // Should panic
    }
}
