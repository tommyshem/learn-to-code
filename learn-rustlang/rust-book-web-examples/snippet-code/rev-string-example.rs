pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for letter in input.chars().rev() {
        result.push(letter);
    }
    return result;
}