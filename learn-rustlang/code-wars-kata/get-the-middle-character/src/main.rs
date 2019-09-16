fn main() {
    println!("{}", get_middle("testing"));
}

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
fn example_tests() {
    assert_eq!(get_middle("test"), "es");
}

#[test]
fn examples_test1() {
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("A"), "A");
}
#[test]
fn examples_test2() {
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("of"), "of");
}
