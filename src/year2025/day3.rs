use crate::file_utils::file_loader::read_lines_to_vec;

pub fn solve() {
    let response_sample_1: i64 = calculate_max_joltage("inputs/samples/day3.txt", 1);
    println!("Day 3 - Part 1 Sample: {}", response_sample_1);
    let response_part_1: i64 = calculate_max_joltage("inputs/day3.txt", 1);
    println!("Day 3 - Part 1: {}", response_part_1);

    let response_sample_2: i64 = calculate_max_joltage("inputs/samples/day3.txt", 2);
    println!("Day 3 - Part 2 Sample: {}", response_sample_2);
    let response_part_2: i64 = calculate_max_joltage("inputs/day3.txt", 2);
    println!("Day 3 - Part 2: {}", response_part_2);
}

fn calculate_max_joltage(filename: &str, part: i64) -> i64 {
    let labels: Vec<String> = read_lines_to_vec(filename);
    let mut count: i64 = 0;
    let number_values: i64 = match part {
        1 => 2,
        2 => 12,
        _ => panic!("Invalid part number: {}", part),
    };
    for label in labels {
        let max_joltage = calculate_for_label(&label, number_values);
        count += max_joltage;
    }
    count
}

fn find_max_value_and_index(values: &Vec<i64>) -> (i64, usize) {
    let mut max_value = values[0];
    let mut max_index = 0;
    for (index, value) in values.iter().enumerate() {
        if *value > max_value {
            max_value = *value;
            max_index = index;
        }
    }
    (max_value, max_index)
}

fn calculate_for_label(label: &str, number_values: i64) -> i64 {
    let values: Vec<i64> = label
        .chars()
        .map(|v| v.to_digit(10).unwrap() as i64)
        .collect();
    let num_values: usize = values.len();
    let mut current_index: usize = 0;
    let mut current_value: i64 = 0;
    for i in (0..number_values as usize).rev() {
        let (max_value, max_index) =
            find_max_value_and_index(&values[current_index..num_values - i].to_vec());
        current_index += max_index + 1;
        current_value *= 10;
        current_value += max_value;
    }
    current_value
}
