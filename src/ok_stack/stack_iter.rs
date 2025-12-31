use crate::ok_stack::{stack::Stack, node::Node};

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl <T> Stack<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            //next: self.head.as_ref().map(|node| &**node)
            next: self.head.as_deref()
        }
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            //self.next = self.next.as_ref().map(|n| &**n);
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            node.data()
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iter() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        stack.push(4);

        stack.iter().for_each(|node: &i32| {
            dbg!(node);
        });
    }
}