fn main() {
    println!("Hello, world!");
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn newvalue(value: i32) -> i32 {
    if value < 1 {
        panic!(
            "Guess value must be greater than or equal to 1, got {}.",
            value
        );
    } else if value > 100 {
        panic!(
            "Guess value must be less than or equal to 100, got {}.",
            value
        );
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", // custom error message
            result
        );
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); // assert equal example
    }

    #[test]
    fn it_not_five() {
        assert_ne!(5, add_two(2)); // assert not equal example
    }

    // example of testing a function when it should panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        newvalue(200);
    }

    // example of testing a function when it should panic with an exact message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_test_panic_message() {
        newvalue(200);
    }

    // example using an result - pass error for the test to fail
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // example will not run when cargo test is run
    // to run this test function  use cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

}
