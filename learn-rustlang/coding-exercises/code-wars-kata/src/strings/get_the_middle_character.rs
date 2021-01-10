// code wars - get the middle character
use colored::*;

pub fn solution(){
    println!("{} Run test suit {}","Get the middle character".green(),"cargo test get_middle_test".yellow());
    let _solution = get_middle("testing");
}

// solution to the problem
fn get_middle(s: &str) -> &str {
    let length = s.len();
    let half = length / 2;
    let even = length % 2 == 0;
    if even == false {
        return &s[half..half + 1];
    }

    return &s[half - 1..half + 1];
}

#[test]
fn get_middle_test() {
    assert_eq!(get_middle("test"), "es");
}

#[test]
fn get_middle_test1() {
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("A"), "A");
}
#[test]
fn get_middle_test2() {
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("of"), "of");
}
