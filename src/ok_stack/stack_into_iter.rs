use crate::ok_stack::stack::Stack;

pub struct IntoIter<T>(Stack<T>);

impl <T: std::fmt::Debug + Clone> Stack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <T: std::fmt::Debug + Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn into_iter() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn into_iter_map() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let result = stack.into_iter()
            .map(|n| n * 2)
            .collect::<Vec<i32>>();

        assert_eq!(result[0], 6);
    }
}