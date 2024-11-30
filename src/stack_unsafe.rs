pub struct FixedStack<T> {
    buffer: *mut T,  // Raw pointer to the memory buffer
    capacity: usize, // Fixed capacity of the stack
    top: usize,      // Current index (size of the stack)
}

impl<T> FixedStack<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than zero");

        let buffer = unsafe {
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            std::alloc::alloc(layout) as *mut T
        };

        if buffer.is_null() {
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
            self.buffer.add(self.top).write(value);
        }

        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;

            unsafe { Some(self.buffer.add(self.top).read()) }
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
            for i in 0..self.top {
                self.buffer.add(i).drop_in_place();
            }
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
