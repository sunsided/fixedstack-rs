#![no_main]
use libfuzzer_sys::fuzz_target;
use stack_experiments::stack_safe::FixedStack as SafeFixedStack;
use stack_experiments::stack_unsafe::FixedStack as UnsafeFixedStack;

// Fuzz target for safe and unsafe FixedStack implementations
fuzz_target!(|data: &[u8]| {
    // Use the data as test inputs
    if data.is_empty() {
        return;
    }

    let mut safe_stack = SafeFixedStack::new(256);
    let mut unsafe_stack = UnsafeFixedStack::new(256);

    for &byte in data.iter().take(256) {
        // Attempt to push the byte onto the stack
        if safe_stack.len() < safe_stack.capacity() {
            safe_stack.push(byte);
        }
        if unsafe_stack.len() < unsafe_stack.capacity() {
            unsafe_stack.push(byte);
        }
    }

    // Pop all elements and compare outputs
    while let Some(safe_value) = safe_stack.pop() {
        let unsafe_value = unsafe_stack.pop();
        assert_eq!(Some(safe_value), unsafe_value);
    }

    // Ensure both stacks are empty
    assert!(safe_stack.is_empty());
});
