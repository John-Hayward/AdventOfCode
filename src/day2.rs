use crate::file_loader::read_comma_seperated_values;

#[derive(Debug)]
struct IdRange {
    min: i64,
    max: i64,
}

pub fn solve() {
    let response_sample_1: i64 = count_faulty_ids("inputs/samples/day2.txt", 1);
    println!("Day 2 - Part 1 Sample: {}", response_sample_1);
    let response_part_1: i64 = count_faulty_ids("inputs/day2.txt", 1);
    println!("Day 2 - Part 1: {}", response_part_1);

    let response_sample_2: i64 = count_faulty_ids("inputs/samples/day2.txt", 2);
    println!("Day 2 - Part 2 Sample: {}", response_sample_2);
    let response_part_2: i64 = count_faulty_ids("inputs/day2.txt", 2);
    println!("Day 2 - Part 2: {}", response_part_2);
}

pub fn count_faulty_ids(filename: &str, part: i8) -> i64 {
    let mut count: i64 = 0;
    let mut num_valid = 0;
    let function = match part {
        1 => is_repeated_digit,
        2 => is_any_repeated_digit,
        _ => panic!("Invalid part number: {}", part),
    };
    let id_ranges: Vec<IdRange> = parse_input_to_id_ranges(filename);
    for id_range in id_ranges {
        for id in id_range.min..=id_range.max {
            if function(id) {
                num_valid += 1;
                count += id;
            }
        }
    }
    count
}

fn parse_input_to_id_ranges(filename: &str) -> Vec<IdRange> {
    let values: Vec<String> = read_comma_seperated_values(filename);
    values.iter().map(|x| {
        let parts: Vec<&str> = x.split('-').collect();
        IdRange {
            min: parts[0].parse::<i64>().unwrap(),
            max: parts[1].parse::<i64>().unwrap(),
        }
    }).collect()
}

fn is_repeated_digit(id: i64) -> bool {
    if (id as f64).log(10f64).floor() % 2.0 == 0.0{
        return false;
    }
    let id_str = id.to_string();
    let id_str_len: usize = id_str.len();
    if id_str[0..id_str_len/2].chars().eq(id_str[id_str_len/2..id_str_len].chars()) {
        return true;
    }
    false
}

fn is_any_repeated_digit(id: i64) -> bool {
    let id_str: String = id.to_string();
    let id_str_len: usize = id_str.len();
    let mut factor_repr: Vec<u64> = get_all_factors(id_str_len as u64);
    factor_repr.push(1u64);
    factor_repr.sort();
    for factor in factor_repr {
        if factor == id_str_len as u64 {
            continue;
        }
        let string_slice = &id_str[0..(factor as usize)];
        if id_str.eq(&string_slice.repeat(id_str_len/(factor as usize))) {
            return true;
        }
    }
    
    false
}

fn get_all_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
    }
    factors
}