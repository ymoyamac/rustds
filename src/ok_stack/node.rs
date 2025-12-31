pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>
}

impl <T> Node<T> {
    
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn next_into(self) -> Link<T> {
        self.next
    }

    pub fn next_as_ref(&self) -> Option<&Box<Node<T>>> {
        self.next.as_ref()
    }

    pub fn next(&self) -> &Link<T> {
        &self.next
    }

    /// This method takes full ownership of the Node<T> struct
    /// and returns the data inside of the node
    pub fn data_into(self) -> T {
        self.data
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    /// This method return a mutable reference of data that means
    /// data is available to change without counsuming the resource
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new_node() {
        let node = Node::new(3);
        assert!(node.next().is_none());
    }

    #[test]
    fn get_data() {
        let node = Node::new(3);
        // The call to the data_into() method moves the field data
        // out of the Node structure. The variable data is the owner
        let data = node.data_into();
        assert_eq!(data, 3);
        // This call can not occur dbg!(node);
    } // at the end of this scope drop is called and the missing field
    // .next is released

    #[test]
    fn get_data_ref() {
        let node = Node::new(3);
        assert_eq!(node.data(), &3);
    }

    #[test]
    fn mut_data() {
        let mut node = Node::new(3);
        dbg!(node.data_mut());
        let data = node.data_mut();
        *data = *data * 2;
        dbg!(node.data_mut());
        assert_eq!(node.data_mut(), &mut 6);
    }
}