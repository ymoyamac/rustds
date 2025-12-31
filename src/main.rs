use rustds::ok_stack::stack::Stack;


fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("{stack:?}");


    let st = stack.into_iter()
        .map(|n| n * 2)
        .collect::<Vec<i32>>();

    println!("{st:?}");

}
