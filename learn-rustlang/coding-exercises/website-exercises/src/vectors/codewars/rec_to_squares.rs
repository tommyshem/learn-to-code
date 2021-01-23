// codewars rec to square https://www.codewars.com/kata/55466989aeecab5aac00003e

use colored::*;

pub fn solution(){
    println!("{} Run test suit {} {}","Rec to square".green(),"cargo test rec_to_square_basic_test".yellow(),"Not finished".red().bold());
    let _solution = sq_in_rect(5,3);
}

// solution to the problem
fn sq_in_rect(lng:i32,wdth:i32)-> Option<Vec<i32>>{
    let mut results = Vec::new();
    if lng == wdth {return None}
    if lng >  wdth  {results.push(wdth); }
    else {results.push(lng);};
    return Some(results)
}

#[cfg(test)]
    mod tests {
    use super::*;
fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn rec_to_square_basic_test(){
    testing(5, 5, None);
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
  
}
}