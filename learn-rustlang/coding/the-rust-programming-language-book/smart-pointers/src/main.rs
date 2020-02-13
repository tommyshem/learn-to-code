fn main() {
    let a = 5; // stored on the stack
    let b = Box::new(a); // stored on the heap
    assert_eq!(5, a); // test value
                      //    assert_eq!(5, b); // compile error if uncommented as need to dereference first * to get the value
    assert_eq!(5, *b); // test value
    println!("a = {} ", a);
    println!("a = {} ", b); // will automatic dereferece *
    println!("b = {} ", *b); // same as above as
}
