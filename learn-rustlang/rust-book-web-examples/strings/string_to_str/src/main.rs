// String rules for function returns and calling.
// If you created the string from inside the function, return String

// if the function borrowed the string, you can return and use &str. 
// You can copy the value in &str and make a new String, via copying the contents. 
// This is possibly not performant.

// &String can be coerced to &str. The compiler can accept a &String, even though the function wants a &str.
// But not the other way around.

// str is on the stack.
// String is on the heap.

fn main() {
    converting_str_to_string();
    converting_string_to_str();
}

// Converting a &str to a String
fn converting_str_to_string() {
    let _new_string1 = "My string sliceto string".to_string(); // one way
    let _new_string2 = String::from("My string slice to string again"); // another way
    let _new_string3 = "My string slice to string again".to_owned(); // jet another way
    let _new_string4: String = "My string slice to string agaian".into();  // into will use the type on the let side
    let _new_string5 = format!("My string slice to string again"); // Don’t use format! if you can use to_string, less overhead. Used for complex string adding
    
}

fn converting_string_to_str() {
    let _my_string = String::from("Hello World!"); // String
    let _my_immutable_string = &_my_string; //This is a &String type
    let _my_str: &str = &_my_string; //This is an &str type need reference to string
    let _my_str1 = &_my_string[..];  // &str
}
