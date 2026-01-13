use crate::unsafe_queue::queue::{Node, Queue};

pub struct IntoIter<T>(Queue<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> Queue<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // pub fn iter(&self) -> Iter<'_, T> {
    //     unsafe {
    //         Iter { next: self.head.as_ref() }
    //     }
    // }
}