// https://www.codewars.com/kata/is-my-friend-cheating/train/rust

fn remove_nb(m: i32) -> Vec<(i32, i32)> {
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
        assert_eq!(remove_nb(n), exp)
    }

    #[test]
    fn basics_remove_nb() {
        testing(26, vec![(15, 21), (21, 15)]);
    }
    #[test]
    fn basics_remove_nb1() {
        testing(100, vec![]);
    }
    #[test]
    fn basics_remove_nb2() {
        testing(101, vec![(55, 91), (91, 55)]);
    }
    #[test]
    fn basics_remove_nb3() {
        testing(102, vec![(70, 73), (73, 70)]);
    }
}
