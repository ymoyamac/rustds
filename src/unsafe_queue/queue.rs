pub struct Node<T> {
    pub data: T,
    pub next: *mut Node<T>
}

pub struct Queue<T> {
    pub head: *mut Node<T>,
    pub tail: *mut Node<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut()
        }
    }

    pub fn push(&mut self, data: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                data,
                next: std::ptr::null_mut()
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }
            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = std::ptr::null_mut();
                }
                Some(head.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.data)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.data)
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut queue = Queue::new();

        // Check empty queue behaves right
        assert_eq!(queue.pop(), None);

        // Populate queue
        queue.push(1);
        queue.push(2);
        queue.push(3);

        // Check normal removal
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.push(4);
        queue.push(5);

        // Check normal removal
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));

        // Check exhaustion
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn miri_food() {
        let mut queue = Queue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert!(queue.pop() == Some(1));
        queue.push(4);
        assert!(queue.pop() == Some(2));
        queue.push(5);

        assert!(queue.peek() == Some(&3));
        queue.push(6);
        queue.peek_mut().map(|x| *x *= 10);
        assert!(queue.peek() == Some(&30));
        assert!(queue.pop() == Some(30));

        for elem in queue.iter_mut() {
            *elem *= 100;
        }

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(queue.pop() == Some(400));
        queue.peek_mut().map(|x| *x *= 10);
        assert!(queue.peek() == Some(&5000));
        queue.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}