

fn string_ends_with(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

fn string_ends_with1(word: &str, ending: &str) -> bool {
    let word_len = word.len();
    let ending_len = ending.len();
    
    if ending_len > word_len { return false }
    
    &word[word_len-ending_len..] == ending
}

#[test]
fn returns_expected() {
  assert_eq!(true, string_ends_with("abc", "c"));
  assert_eq!(false, string_ends_with1("strawberry", "banana"));
}