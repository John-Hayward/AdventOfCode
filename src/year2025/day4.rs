use crate::file_utils::file_loader::read_vec_vec_chars;

pub fn solve() {
    let response_sample_1: i32 = find_total_adjacent_rolls("inputs/samples/day4.txt", 1);
    println!("Day 4 - Part 1 Sample: {}", response_sample_1);
    let response_part_1: i32 = find_total_adjacent_rolls("inputs/day4.txt", 1);
    println!("Day 4 - Part 1: {}", response_part_1);

    let response_sample_2: i32 = find_total_adjacent_rolls("inputs/samples/day4.txt", 2);
    println!("Day 3 - Part 2 Sample: {}", response_sample_2);
    let response_part_2: i32 = find_total_adjacent_rolls("inputs/day4.txt", 2);
    println!("Day 3 - Part 2: {}", response_part_2);
}

fn find_total_adjacent_rolls(filename: &str, part: i32) -> i32 {
    let mut roll_positions: Vec<Vec<char>> = read_vec_vec_chars(filename);
    let mut num_valid: i32 = 0;
    let mut num_just_removed: i32 = 1;
    while num_just_removed > 0 {
        num_just_removed = 0;
        let mut positions_to_remove: Vec<(usize, usize)> = Vec::new();
        for (row_index, row) in roll_positions.iter().enumerate() {
            for (col_index, value) in row.iter().enumerate() {
                if (count_adjacent_rolls(&roll_positions, row_index, col_index) < 4)
                    && (value == &'@')
                {
                    positions_to_remove.push((row_index, col_index));
                }
            }
        }
        num_just_removed = positions_to_remove.len() as i32;
        num_valid += num_just_removed;
        for (row_index, col_index) in positions_to_remove {
            roll_positions[row_index][col_index] = '.';
        }
        if part == 1 {
            num_just_removed = 0;
        }
    }
    num_valid
}

fn count_adjacent_rolls(rolls: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count: i32 = 0;
    let num_rows = rolls.len() as i32;
    let num_cols = rolls[0].len() as i32;

    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;
        if new_row >= 0 && new_row < num_rows && new_col >= 0 && new_col < num_cols {
            if rolls[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}
