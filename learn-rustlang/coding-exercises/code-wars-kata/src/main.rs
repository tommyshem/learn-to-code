mod strings;
mod vectors;

use colored::*;
fn main() {
    println!("");
    println!("{}","Codewars completed".blue().bold());
    println!("{}","string solutions".red().bold());
    strings::disemvowel_trolls::solution();
    strings::get_drink_by_profession::solution();
    strings::get_the_middle_character::solution();
    strings::string_ends_with::solution();
    strings::two_to_one::solution();
    println!("{}","vector solutions".red().bold());
    vectors::rec_to_squares::solution();
    vectors::count_positives_sum_of_negatives::solution();
    vectors::is_my_friend_cheating::solution();
}
