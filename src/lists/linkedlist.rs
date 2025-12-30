use crate::lists::node::{Link, Node};

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize
}

impl <T: std::fmt::Debug> LinkedList<T> {
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
            // it is a swap
            next: std::mem::replace(&mut self.head, None),
        });
        dbg!(&new_node);
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_list() {
        let list = LinkedList::<i32>::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_values() {
        let mut list = LinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn pop_values() {
        let mut list = LinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.len(), 2);
    }
}