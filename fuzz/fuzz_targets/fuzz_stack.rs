#![no_main]
use libfuzzer_sys::fuzz_target;
use stack_experiments::stack_safe::FixedStack as SafeFixedStack;
use stack_experiments::stack_unsafe::FixedStack as UnsafeFixedStack;

// Fuzz target for safe and unsafe FixedStack implementations
fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return; // Ensure there's enough data to define size and operations
    }

    // First byte determines the stack capacity (limit to a reasonable size)
    let capacity = (data[0] as usize % 256) + 1; // Capacity: 1 to 256
    let mut safe_stack = SafeFixedStack::new(capacity);
    let mut unsafe_stack = UnsafeFixedStack::new(capacity);

    // Remaining bytes determine operations (push/pop)
    for &byte in &data[1..] {
        match byte % 2 {
            0 => {
                // Push operation
                let value = byte; // Use the byte as the value
                if safe_stack.len() < safe_stack.capacity() {
                    safe_stack.push(value);
                }
                if unsafe_stack.len() < unsafe_stack.capacity() {
                    unsafe_stack.push(value);
                }
            }
            1 => {
                // Pop operation
                let safe_value = safe_stack.pop();
                let unsafe_value = unsafe_stack.pop();
                assert_eq!(safe_value, unsafe_value);
            }
            _ => unreachable!(),
        }
    }

    // Verify both stacks are equally filled at the end
    assert_eq!(safe_stack.len(), unsafe_stack.len());
    drop(safe_stack);
    drop(unsafe_stack);
});
