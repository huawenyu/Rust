pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {

    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() == self.maxsize {
            return false;
        }
        self.items.push(item);
        return true;
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

#[cfg(test)]

mod tests {
    #[test]
    fn test_new_with_capacity() {
        let stack = super::Stack::<u32>::with_capacity(10);
        assert_eq!(10, stack.items.capacity());
    }

    #[test]
    fn test_pop(){
        let mut stack = super::Stack::<u32>::with_capacity(1);
        stack.push(1u32);
        assert_eq!(Some(1u32), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_push() {
        let mut stack = super::Stack::<u32>::with_capacity(1);
        stack.push(32u32);
        assert_eq!(Some(&32u32), stack.peek());
        assert_eq!(1, stack.size());
    }

    #[test]
    fn test_push_maxsize() {
        let mut stack = super::Stack::<u32>::with_capacity(1);
        assert_eq!(true, stack.push(1u32));
        assert_eq!(Some(&1u32), stack.peek());
        assert_eq!(false, stack.push(2u32));
    }

    #[test]
    fn test_size() {
        let mut stack = super::Stack::<u32>::with_capacity(1);
        assert_eq!(0, stack.size());
        stack.push(1u32);
        assert_eq!(1, stack.size());
    }
}

