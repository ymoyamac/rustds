use std::rc::Rc;

use crate::bad_deque::node::{Link, Node};

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            },
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                },
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut deque = Deque::new();

        // Check empty deque behaves right
        assert_eq!(deque.pop_front(), None);

        // Populate deque
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        // Check normal removal
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        deque.push_front(4);
        deque.push_front(5);

        // Check normal removal
        assert_eq!(deque.pop_front(), Some(5));
        assert_eq!(deque.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }
}
