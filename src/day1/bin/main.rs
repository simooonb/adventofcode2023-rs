use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let lines = read_lines("input/day1/input.txt");
    let result: i32 = lines
        .iter()
        .map(|row| part2(row))
        .sum();

    println!("{}", result)
}

fn part1(row: &str) -> i32 {
    let row_chars: Vec<char> = row.chars().collect();

    let first_digit = row_chars
        .iter()
        .find(|c| c.is_ascii_digit())
        .unwrap_or(&'0');

    let last_digit = row_chars
        .iter()
        .rfind(|c| c.is_ascii_digit())
        .unwrap_or(&'0');

    format!("{first_digit}{last_digit}")
        .parse()
        .unwrap_or(0)
}

fn part2(row: &str) -> i32 {
    let conversion_table = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let first_substring_pos: Vec<_> = conversion_table
        .keys()
        .flat_map(|&substr| row.find(substr).map(|i| (i, substr)))
        .collect();

    let last_substring_pos: Vec<_> = conversion_table
        .keys()
        .flat_map(|&substr| row.rfind(substr).map(|i| (i, substr)))
        .collect();

    let all_pos = [first_substring_pos, last_substring_pos].concat();

    let (_, first_substring) = all_pos.iter().min_by(|(i, _), (j, _)| i.cmp(j)).unwrap();
    let (_, second_substring) = all_pos.iter().max_by(|(i, _), (j, _)| i.cmp(j)).unwrap();

    let first_digit = conversion_table.get(first_substring).unwrap();
    let last_digit = conversion_table.get(second_substring).unwrap();

    format!("{first_digit}{last_digit}")
        .parse()
        .unwrap_or(0)
}
