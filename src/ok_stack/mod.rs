pub mod node;
pub mod stack;
pub mod stack_into_iter;
pub mod stack_iter;
pub mod stack_iter_mut;

pub trait FromSlice<T> {
    fn from_slice(value: &[T]) -> Self;
}