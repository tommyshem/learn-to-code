// https://www.codewars.com/kata/is-my-friend-cheating/train/rust

use colored::*;

pub fn solution(){
    println!("{} Run test suit {}","Is my friend cheating".green(),"cargo test is_my_friend_cheating_basic_test".yellow());
    let _solution = is_my_friend_cheating(100);
}

fn is_my_friend_cheating(m: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::<(i32, i32)>::with_capacity(2);
    //                          / \
    // same as above.            |
    //let result = vec![(2,4)];  |
    let m:i64 =m.into();
    for a in 1..m {
        for b in 1..m {
            if a * b == (m * (m + 1) / 2 - a - b) {
                result.push((a as i32, b as i32))
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
        assert_eq!(is_my_friend_cheating(n), exp)
    }

    #[test]
    fn is_my_friend_cheating_basic_test() {
        testing(26, vec![(15, 21), (21, 15)]);
        testing(100, vec![]);
        testing(101, vec![(55, 91), (91, 55)]);
        testing(102, vec![(70, 73), (73, 70)]);
    }
}
