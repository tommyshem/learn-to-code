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
                      //    assert_eq!(5, b); // compile error if uncommented as need to dereference first * to get the value
    assert_eq!(5, *b); // test value
    println!("a = {} ", a);
    println!("a = {} ", b); // will automatic dereferece *
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
}

fn under(){
    let artist = "";
    let mastered_for_itunes = "";
    let release_date = "";
    let genres = "";
    let record_label = "";
    let track_count = "";

    let _ = vec![
        ("Artist", artist, true),
        ("Mastered for iTunes", &mastered_for_itunes, true),
        ("Release Date", release_date, true),
        ("Genres", genres, true),
        ("Record Label", record_label, true),
        ("Tracks", track_count, true)
    ];
}

