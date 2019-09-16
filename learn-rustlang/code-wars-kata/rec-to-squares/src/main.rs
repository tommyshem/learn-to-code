fn sq_in_rect(lng:i32,wdth:i32)-> Option<Vec<i32>>{
let mut results = Vec::new();
// square return None
if lng == wdth {return None}
// recangle return Option vector
let mut old_lng = lng;
let mut old_wdth = wdth;

while old_lng > 0{
    dbg!(old_lng);
    if lng > wdth {
        results.push(wdth);
        old_lng - wdth;
        dbg!(old_lng);
    }
    else {results.push(lng);
        old_lng - lng;
        dbg!(old_lng);
        }
        dbg!(old_lng);
}
return Some(results);
}

fn main() {
    sq_in_rect(30,30);
}

#[cfg(test)]
    mod tests {
    use super::*;

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq(){
    testing(5, 5, None);
  
}

#[test]
fn tests_sq_in_rect() {

    testing(5, 3, Some(vec![3, 2, 1, 1]));
  }
#[test]
fn tests_sq_sample(){
    testing(3, 5, Some(vec![3, 2, 1, 1]));
  }
  


