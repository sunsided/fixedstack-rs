pub struct FixedStack<T> {
    buffer: Vec<T>,  // Vec manages memory safely
    capacity: usize, // Fixed capacity of the stack
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
