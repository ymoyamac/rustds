use std::rc::Rc;
use std::cell::RefCell;

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub prev: Link<T>,
    pub next: Link<T>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data,
            prev: None,
            next: None
        }))
    }
}
