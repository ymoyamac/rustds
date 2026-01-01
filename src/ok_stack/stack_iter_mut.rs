use crate::ok_stack::{node::Node, stack::Stack};

pub struct IterMut<'a, T> {
    ne: Option<&'a mut Node<T>>,
}

impl<T> Stack<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { ne: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ne.take().map(|node| {
            self.ne = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn iter_mut() {
        
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
