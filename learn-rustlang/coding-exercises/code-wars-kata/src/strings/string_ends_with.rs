// codewars string ends with
use colored::*;

pub fn solution(){
    println!("{} Run test suit {}","string ends with".green(),"cargo test string_ends_with_test".yellow());
    let _solution = string_ends_with("testing","g");
}
fn string_ends_with(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

// another soloution for the same problem
fn _string_ends_with1(word: &str, ending: &str) -> bool {
    let word_len = word.len();
    let ending_len = ending.len();
    
    if ending_len > word_len { return false }
    
    &word[word_len-ending_len..] == ending
}

#[test]
fn string_ends_with_test() {
  assert_eq!(true, string_ends_with("abc", "c"));
  assert_eq!(false, _string_ends_with1("strawberry", "banana"));
}