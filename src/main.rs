use rustds::ok_stack::{FromSlice, stack::Stack};


fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{stack:?}");

    let list = stack.iter_mut()
        .map(|node| {
            *node = *node * 2;
            node
        })
        .collect::<Vec<&mut i32>>();

    println!("{list:?}");

    let slice = vec![1, 2, 3, 4, 5, 6, 7];
    let stack = Stack::from_slice(&slice[2..5]);

    println!("{stack:?}");

}
