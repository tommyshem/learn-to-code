// struct update syntax.
#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
pub fn create_struct_from_other_struct() -> User {
    let user1 = User {
        username: String::from("Bill"),
        email: String::from("test@test.mail"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,               // use user1 data
        sign_in_count: user1.sign_in_count, // use user1 data
    };

    User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2 // use user2 data using the update syntax
    }
}

#[test]
fn create_struct_from_other_struct_test() {
    create_struct_from_other_struct();
}
