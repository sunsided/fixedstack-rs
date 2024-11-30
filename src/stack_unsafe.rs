pub struct FixedStack<T> {
    buffer: *mut T,  // Raw pointer to the memory buffer
    capacity: usize, // Fixed capacity of the stack
    top: usize,      // Current index (size of the stack)
}

impl<T> FixedStack<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than zero");

        let buffer = unsafe {
            // SAFETY: `Layout::array` is safe if `capacity` is non-zero, which is guaranteed by the preceding `assert!`.
            // The `alloc` function is safe if the layout is valid and the pointer is either handled properly or deallocated later.
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            std::alloc::alloc(layout) as *mut T
        };

        if buffer.is_null() {
            // SAFETY: `handle_alloc_error` is safe to call if `alloc` fails and returns a null pointer.
            std::alloc::handle_alloc_error(std::alloc::Layout::array::<T>(capacity).unwrap());
        }

        Self {
            buffer,
            capacity,
            top: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        assert!(self.top < self.capacity, "Stack overflow");

        unsafe {
            // SAFETY: We ensure `self.top < self.capacity`, so `self.buffer.add(self.top)` is a valid pointer within the allocated range.
            self.buffer.add(self.top).write(value);
        }

        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;

            unsafe {
                // SAFETY: `self.top` was decremented, but we ensure it is within bounds, so `self.buffer.add(self.top)` is valid.
                // The `read` function is safe because the memory at this location was initialized by `write` in `push`.
                // No need to drop the value here since we're transferring ownership to the caller.
                Some(self.buffer.add(self.top).read())
            }
        }
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for FixedStack<T> {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: `self.top` represents the number of initialized elements in the stack.
            // Iterating from `0` to `self.top` ensures we drop all valid elements.
            for i in 0..self.top {
                self.buffer.add(i).drop_in_place();
            }

            // SAFETY: `self.buffer` was allocated with `Layout::array` and `alloc`, so it can be deallocated with the same layout.
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            std::alloc::dealloc(self.buffer as *mut u8, layout);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_fixed_stack() {
        let mut stack = FixedStack::new(3);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_unsafe_fixed_stack_overflow() {
        let mut stack = FixedStack::new(2);
        stack.push(1);
        stack.push(2);
        stack.push(3); // Should panic
    }
}
