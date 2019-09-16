#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
pub fn create_user() -> User {
    build_user(String::from("test"), String::from("hello"))
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[test]
fn create_user_test() {
    let user = create_user();
    println!("{:#?}", user);
}
