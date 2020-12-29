fn main() {
    solution("abc", "bc");
}

fn solution(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

fn solution1(word: &str, ending: &str) -> bool {
    let word_len = word.len();
    let ending_len = ending.len();
    
    if ending_len > word_len { return false }
    
    &word[word_len-ending_len..] == ending
}

#[test]
fn returns_expected() {
  assert_eq!(true, solution("abc", "c"));
  assert_eq!(false, solution("strawberry", "banana"));
}