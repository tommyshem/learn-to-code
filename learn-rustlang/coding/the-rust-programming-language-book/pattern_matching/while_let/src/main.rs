fn main() {
    // create a stack
    let mut stack = Vec::new();
    // add values on to stack
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // remove one item from the end and print to stdout
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
