mod file_utils;
mod year2025;

use crate::year2025::*;
use text_io::read;

fn main() {
    print!("Enter the day number to solve: ");
    let input: i32 = read!();
    println!("Generating solutions for day {}", input);
    match input {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        _ => println!("Day {} is not yet implemented.", input),
    }
}
