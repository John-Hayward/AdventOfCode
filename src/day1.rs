use crate::file_loader::read_lines_to_vec;

#[derive(Debug)]
pub struct Rotation {
    direction: char,
    degrees: i32,
}

pub fn solve(){
    let response_sample_1: i32 = solve_part1("inputs/samples/day1.txt", 50);
    println!("Day 1 - Part 1 Sample: {}", response_sample_1);
    let response_part_1: i32 = solve_part1("inputs/day1.txt", 50);
    println!("Day 1 - Part 1: {}", response_part_1);

    let response_sample_2: i32 = solve_part2("inputs/samples/day1.txt", 50);
    println!("Day 1 - Part 2 Sample: {}", response_sample_2);
    let response_part_2: i32 = solve_part2("inputs/day1.txt", 50);
    println!("Day 1 - Part 2: {}", response_part_2);
}

fn solve_part1(filename: &str, starting_value: i32) -> i32 {
    let rotations: Vec<Rotation> = generate_rotations(filename);
    let mut current_value: i32 = starting_value.clone();
    let mut turn_direction: i32; // 1 for right, -1 for left
    let mut num_zeros: i32 = 0;
    for rotation in rotations {
        match rotation.direction {
            'R' => turn_direction = 1,
            'L' => turn_direction = -1,
            _ => panic!("Unexpected rotation direction: {}", rotation.direction),
        }

        current_value += turn_direction * rotation.degrees;
        if (current_value % 100) == 0 {
            num_zeros += 1;
        }
    }

    num_zeros
}

fn solve_part2(filename: &str, starting_value: i32) -> i32 {
    let rotations: Vec<Rotation> = generate_rotations(filename);
    let mut current_value: i32 = starting_value.clone();
    let mut num_zeros: i32 = 0;
    for rotation in rotations {
        match rotation.direction {
            'R' => {
                num_zeros += (current_value + rotation.degrees) / 100;
                current_value = ((current_value + rotation.degrees) % 100 + 100) % 100;
            },
            'L' => {
                let t0: i32 = if current_value == 0 {100} else {current_value};
                if rotation.degrees >= t0 {
                    num_zeros += (rotation.degrees - t0) / 100 + 1;
                }
                current_value = ((current_value - rotation.degrees) % 100 + 100) % 100;
            },
            _ => panic!("Unexpected rotation direction: {}", rotation.direction),
        }
    }
    num_zeros
}

pub fn generate_rotations(filename: &str) -> Vec<Rotation> {
    let mut file_input: Vec<String> = read_lines_to_vec(filename);
    file_input.iter().map(|x|
        Rotation{
            direction: x.chars().take(1).next().unwrap(),
            degrees: x.chars().skip(1).into_iter().collect::<String>().parse::<i32>().unwrap(),}
    ).collect()
}