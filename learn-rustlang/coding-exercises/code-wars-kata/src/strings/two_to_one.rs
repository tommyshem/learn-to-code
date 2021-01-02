// code wars - two to one

fn longest(a1: &str, a2: &str) -> String {
    // Copy chars into a vector, sort and remove duplicates
    // 2 lines below can be removed for the lines in the comments below.
    let mut chars1: Vec<char> = a1.chars().collect();
    chars1.extend(a2.chars());
    // can use this line below instaed of the 2 lines above.
    // let mut chars1: Vec<char> = format!("{}{}", a1, a2).chars().collect();
    // or this line below instaed of the 2 lines above
    // let mut chars1: Vec<_> = [a1, a2].concat().chars().collect();
    // or this
    // let mut chars1: Vec<char> = a1.chars().chain(a2.chars()).collect(); 
    // sort order
    chars1.sort();
    // remove duplicated chars
    chars1.dedup();
    // Pass back as a string
    chars1.into_iter().collect()
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
        
    }
}
