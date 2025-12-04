use std::fs::read_to_string;

pub fn read_lines_to_vec(filename: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

pub fn read_comma_seperated_values(filename: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let file_content = read_to_string(filename).unwrap();
    for value in file_content.split(',') {
        result.push(value.trim().to_string());
    }

    result
}

pub fn read_vec_vec_chars(filename: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.chars().collect());
    }

    result
}
