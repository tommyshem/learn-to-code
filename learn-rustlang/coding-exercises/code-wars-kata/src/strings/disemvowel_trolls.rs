// code wars - disemvowel trolls
// different ways of doing the same problem below.

fn disemvowel(s: &str) -> String {
    s.chars().filter(|&x| !"aeiouAEIOU".contains(x)).collect()
}

fn disemvowel1(s: &str) -> String {
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

fn disemvowel2(string: &str) -> String {
    string.chars()
        .filter(|letter| {
            match letter.to_lowercase().next().unwrap() {
                'a' | 'e' | 'i' | 'o' | 'u' => false,
                _ => true
            }
        })
        .collect()
}

fn disemvowel3(s: &str) -> String {
    s.chars().filter(|c| !is_vowel(c)).collect::<String>()
}

fn is_vowel(c: &char) -> bool {
    let lower = c.to_lowercase().to_string();

    lower == "a" || lower == "e" || lower == "i" || lower == "o" || lower == "u"
}

fn disemvowel4(s: &str) -> String {
    s.chars().filter(|c| !matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')).collect()
}
fn disemvowel5(s: &str) -> String {
    let vowels = ['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    s.chars().filter(|ch| !vowels.contains(ch)).collect()
}

fn disemvowel6(s: &str) -> String {
    let mut new = String::new();
    for i in s.chars() {
        match i {
            'a' | 'A' =>{}
            'e' | 'E' =>{}
            'i' | 'I' =>{}
            'o' | 'O' =>{}
            'u' | 'U' =>{}
            _ => new.push(i)
        }
    }
    new
}
// vowels A, E, I, O, U,
fn disemvowel7(s: &str) -> String {
    let mut string = String::new();
    for c in s.chars(){
        // check for vowels
        if c == 'a' {}
        else if c == 'A' {}
        else if c == 'i' {}
        else if c == 'I' {}
        else if c == 'e' {}
        else if c == 'E' {}
        else if c == 'u' {}
        else if c == 'U' {}
        else if c == 'o' {}
        else if c == 'O' {}
        // not a vowel then add to string
        else {string.push(c);}
        }
    string
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn test_disemvowel() {
        assert_eq!(disemvowel(""), "");
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
    
}