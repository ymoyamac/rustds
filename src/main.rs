use rustds::ok_stack::stack::Stack;


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
}
