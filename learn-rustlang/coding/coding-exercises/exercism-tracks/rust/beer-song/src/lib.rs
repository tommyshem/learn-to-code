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

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for x in start..end + 1 {
        song.push_str(&verse(x));
    }
    return song;
}
