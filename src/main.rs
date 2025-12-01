mod file_loader;
mod day1;

fn main() {
    let response1: i32 = day1::solve_part1("inputs/day1.txt", 50);
    let response2: i32 = day1::solve_part2("inputs/day1.txt", 50);
    let response3: i32 = day1::solve_part2("inputs/samples/test1.txt", 50);
    println!("Answer1: {:?}", response1);
    println!("Answer2: {:?}", response2);
}