mod enumerate;
mod newstring;
mod slice;
mod update_string;

// add to string and pass back
fn add_oomph(t: &mut String) {
    t.push('!');
}
// main starting point
fn main() {
    let mut s = "Hello, world".to_string();
    add_oomph(&mut s); // send reference to function
    println!("{}", s);
}

// below code does not compile as the function takes ownership if the string.
//fn add_oomph(mut t: String) {
// t.push('!');
//}
//fn main(){
//let mut s = "Hello, world".to_string();
//add_oomph(s);
//println!("{}", s);
//}
