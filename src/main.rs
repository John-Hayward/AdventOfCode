mod file_loader;
mod day1;
mod day2;
mod day3;
use text_io::read;

fn main() {
    print!("Enter the day number to solve: ");
    let input: i32 = read!();
    println!("Generating solutions for day {}", input);
    match input {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        _ => println!("Day {} is not yet implemented.", input),
    }
}