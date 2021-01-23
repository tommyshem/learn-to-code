mod strings;
mod vectors;
mod integers;

use std::println;

use colored::*;
fn main() {
    println!("");
    println!("{}","Codewars completed".purple());
    println!("{}","string solutions".blue());
    strings::codewars::disemvowel_trolls::solution();
    strings::codewars::get_the_middle_character::solution();
    strings::codewars::string_ends_with::solution();
    strings::codewars::two_to_one::solution();
    strings::codewars::get_drink_by_profession::solution();
    println!("{}","vector solutions".blue());
    vectors::codewars::rec_to_squares::solution();
    vectors::codewars::count_positives_sum_of_negatives::solution();
    vectors::codewars::is_my_friend_cheating::solution();
    // leetcode solution
    println!("{}","Leetcode completed".purple());
    println!("{}","string solutions".blue());
    integers::leetcode::binary_convert::solution();
    // exercism solutions
    println!("{}","Exercism completed".purple());
    println!("{}","string solutions".blue());
    strings::exercism::beer_song::solution();
    
}
