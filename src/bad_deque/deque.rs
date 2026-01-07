use std::{cell::{Ref, RefMut}, rc::Rc};

use crate::bad_deque::node::{Link, Node};

pub struct Deque<T> {
    pub head: Link<T>,
    pub tail: Link<T>
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

    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            },
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| &node.data)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.data)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.data)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.data)
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

        // ---- back -----

        // Check empty deque behaves right
        assert_eq!(deque.pop_back(), None);

        // Populate deque
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        // Check normal removal
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        deque.push_back(4);
        deque.push_back(5);

        // Check normal removal
        assert_eq!(deque.pop_back(), Some(5));
        assert_eq!(deque.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut deque = Deque::new();
        assert!(deque.peek_front().is_none());
        assert!(deque.peek_back().is_none());
        assert!(deque.peek_front_mut().is_none());
        assert!(deque.peek_back_mut().is_none());

        deque.push_front(1); deque.push_front(2); deque.push_front(3);

        assert_eq!(&*deque.peek_front().unwrap(), &3);
        assert_eq!(&mut *deque.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*deque.peek_back().unwrap(), &1);
        assert_eq!(&mut *deque.peek_back_mut().unwrap(), &mut 1);
    }

}