use std::cell::Ref;

use crate::bad_deque::{deque::Deque, node::Node};

pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<T> Deque<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter(
            self.head
                .as_ref()
                .map(|head| head.borrow())
        )
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // self.0.take().map(|node_ref| {
        //     let (next, data) = Ref::map_split(node_ref, |node| {
        //         (&node.next, &node.data)
        //     });
        //     self.0 = if next.is_some() {
        //         Some(Ref::map(next, |next| &**next.as_ref().unwrap()))
        //     } else {
        //         None
        //     }
        //     data
        // })
        self.0.take();
        todo!()
    }
}