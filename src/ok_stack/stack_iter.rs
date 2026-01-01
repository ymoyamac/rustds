use crate::ok_stack::{stack::Stack, node::Node};

pub struct Iter<'a, T> {
    // `Iter` does not own any nodes. It only holds shared references
    // to the nodes owned by `Stack`.
    //
    // Using `Option<&'a Node<T>>` instead of `Option<Box<Node<T>>>`
    // is mandatory: a `Box` implies ownership and responsibility
    // for deallocation, which an iterator must not have.
    //
    // The lifetime `'a` guarantees that the iterator cannot outlive
    // the stack it was created from, preventing dangling references
    // while allowing safe traversal of the linked structure.
    next: Option<&'a Node<T>>,
}

impl<T> Stack<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // This method creates an iterator that borrows the stack.
        //
        // `self.head` has type `Option<Box<Node<T>>>`, but the iterator
        // must not take ownership of any node. Using `as_deref()` converts
        // `Option<Box<Node<T>>>` into `Option<&Node<T>>` by applying
        // deref coercion (`Box<T> -> &T`).
        //
        // The lifetime `'a` ties the iterator to `&self`, ensuring that
        // the iterator cannot outlive the stack and preventing dangling
        // references.
        Iter {
            //next: self.head.as_ref().map(|node| &**node)
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // `self.next` represents the current node the iterator is pointing to.
        // If it is `None`, the iteration is finished.
        self.next.map(|node| {
            // Advance the iterator by updating `self.next` to point
            // to the next node in the linked list.
            //
            // `node.next` has type `Option<Box<Node<T>>>`.
            // Calling `as_ref()` converts it to `Option<&Box<Node<T>>>`,
            // and the closure then dereferences the `Box` to obtain
            // `&Node<T>`.
            //
            // At no point is ownership transferred or memory moved.
            //self.next = self.next.as_ref().map(|n| &**n);

            self.next = node.next
                .as_ref()
                .map::<&Node<T>, _>(|node| &node);

            // Yield a shared reference to the data stored in the current node.
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