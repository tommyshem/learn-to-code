use std::fs::File;

// panic if and error
fn main() {
    dont_panic();
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn dont_panic() {
    // do not panic if error
    let f = File::open("hello.txt");
    let _t: Option<std::fs::File> = match f {
        Ok(file) => Some(file),
        Err(_) => None,
    };
}
