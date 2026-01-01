use crate::ok_stack::{FromSlice, node::{Link, Node}};

#[derive(Debug)]
pub struct Stack<T> {
    pub head: Link<T>,
    pub len: usize
}

impl <T: std::fmt::Debug + Clone> Stack<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(self) -> usize {
        self.len
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            // The 'push' method cannot be implemented without using 'take',
            // because the ownership rules do not allow it.
            next: self.head.take()
        });
        dbg!(&new_node);
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            // The 'take' method is an abstraction of std::mem::replace
            // Moves the value out of `self.head` and leaves `None` in its place
            next: std::mem::replace(&mut self.head, None),
        });
        dbg!(&new_node);
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            dbg!(&node);
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            dbg!(node);
            node.data()
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            dbg!(&node);
            node.data_mut()
        })
    }

}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T: std::fmt::Debug + Clone> From<Vec<T>> for Stack<T>  {
    fn from(value: Vec<T>) -> Self {
        let mut stack = Stack::<T>::new();
        for v in value {
            stack.push(v);
        }
        stack
    }
}

impl<T: std::fmt::Debug + Clone> FromSlice<T> for Stack<T> {
    fn from_slice(value: &[T]) -> Self {
        let mut stack = Stack::<T>::new();
        for v in value {
            stack.push(v.clone());
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_stack() {
        let stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn push_values() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push_front(3);
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn pop_values() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn peek() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn peek_mut() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        stack.peek_mut().map(|value| {
            *value = *value *2 + 42
        });

        assert_eq!(stack.peek(), Some(&48));
        assert_eq!(stack.len(), 3);
    }
}