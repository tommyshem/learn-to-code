mod associated_functions;
mod simple_example_struct;
mod simple_struct;
mod tuple_struct;
mod update_syntax;

#[derive(Debug)]
// define user structure
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Creating an instance of the User struct. The field names can be in any order.
    let user1 = User {
        username: String::from("Bill"),
        email: String::from("test@test.mail"),
        active: true,
        sign_in_count: 1,
    };
    // Print the user1 instance of user struct.
    println!("{:?}", user1);

    // Print the created user instance from the called function build_user
    println!(
        "{:#?}",
        build_user(String::from("John"), String::from("john@test.mail"))
    );
}

// Function to return a created user struct
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 2,
    }
}

// Function to return a created user struct with shorthand syntax
// if the parameter name and struct name are the same.
#[allow(dead_code)]
fn build_user_with_shorthand(email: String, username: String) -> User {
    User {
        email,    // shorthand
        username, // shorthand
        active: true,
        sign_in_count: 2,
    }
}
