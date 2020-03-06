struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    let a = 5; // stored on the stack
    let b = Box::new(a); // stored on the heap
    assert_eq!(5, a); // test value
                      //    assert_eq!(5, b); // compile error if uncomment as need to dereference first * to get the value
    assert_eq!(5, *b); // test value
    println!("a = {} ", a);
    println!("a = {} ", b); // will automatic dereference *
    println!("b = {} ", *b); // same as above as

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // Page 321 Reference counted smart pointers
    enum List {
        Cons(i32,Rc<List>),
        Nil,
    }

    use List::{Cons,Nil};
    use std::rc::Rc;
    
    let e = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil))))); // list
    println!("count after creating e = {} ",Rc::strong_count(&e));
    {
    let _f = Cons(3,Rc::clone(&e)); // list with e added to end clone only clones reference
    println!("count after creating e = {} ",Rc::strong_count(&e));
    let _g = Cons(5,Rc::clone(&e)); // list with e added to end clone only clones reference
    println!("count after creating e = {} ",Rc::strong_count(&e));
    }
    // reference counts end and they will be cleaned up with only 1 count left
    println!("count after creating e = {} ",Rc::strong_count(&e));
    
}
