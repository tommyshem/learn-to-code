use colored::*;

pub fn solution(){
    println!("{} Run test suit {}","Beer Song".green(),"cargo test test_verse".yellow());
    let _solution = verse(100);
}
pub fn verse(n: i32) -> String {
    // Numbers of bottles converted to string
    let mut number_of_bottles = n.to_string() + " bottles";
    let mut it_or_one = String::from("one");
    let mut number_bottles_left = (n - 1).to_string() + " bottles";

    if n == 0 {
        it_or_one = String::from("it");
        number_bottles_left = String::from("No more");
    }
    if n == 1 {
        number_of_bottles = n.to_string() + " bottle";
        number_bottles_left = String::from("no more bottles");
        it_or_one = String::from("it");
    }
    if n == 2 {
        number_bottles_left = (n - 1).to_string() + " bottle";
    }
    // {0}                            {0}                     {1}                         {2}
    // 1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    // 2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
    format!(
        "{0} of beer on the wall, {0} of beer.\nTake {1} down and pass it around, {2} of beer on the wall.\n",
        number_of_bottles, it_or_one,number_bottles_left)
}

pub fn _sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for x in start..end + 1 {
        song.push_str(&verse(x));
    }
    return song;
}

#[test]
fn test_verse_0() {
    assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

#[test]
#[ignore]
fn test_verse() {
    assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    assert_eq!(verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    assert_eq!(verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
    assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
