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
    let lines: Vec<String> = read_lines("input/day1/input.txt");
    let result: i32 = lines.iter().map(|row| part2(row)).sum();

    println!("{}", result)
}

fn part1(row: &str) -> i32 {
    let row_chars: Vec<char> = row.chars().collect::<Vec<char>>();

    let first_digit = row_chars
        .iter()
        .find(|c| c.is_ascii_digit())
        .unwrap_or_else(|| &'0');

    let last_digit = row_chars
        .iter()
        .rfind(|c| c.is_ascii_digit())
        .unwrap_or_else(|| &'0');

    format!("{first_digit}{last_digit}")
        .parse()
        .unwrap_or_else(|_| 0)
}

fn part2(row: &str) -> i32 {
    let mut conversion_table = HashMap::new();
    conversion_table.insert(String::from("1"), 1);
    conversion_table.insert(String::from("2"), 2);
    conversion_table.insert(String::from("3"), 3);
    conversion_table.insert(String::from("4"), 4);
    conversion_table.insert(String::from("5"), 5);
    conversion_table.insert(String::from("6"), 6);
    conversion_table.insert(String::from("7"), 7);
    conversion_table.insert(String::from("8"), 8);
    conversion_table.insert(String::from("9"), 9);
    conversion_table.insert(String::from("one"), 1);
    conversion_table.insert(String::from("two"), 2);
    conversion_table.insert(String::from("three"), 3);
    conversion_table.insert(String::from("four"), 4);
    conversion_table.insert(String::from("five"), 5);
    conversion_table.insert(String::from("six"), 6);
    conversion_table.insert(String::from("seven"), 7);
    conversion_table.insert(String::from("eight"), 8);
    conversion_table.insert(String::from("nine"), 9);

    let first_substring_pos = conversion_table
        .keys()
        .flat_map(|substr| row.find(substr).map(|i| (i, substr.clone())))
        .collect::<Vec<(usize, String)>>();

    let last_substring_pos = conversion_table
        .keys()
        .flat_map(|substr| row.rfind(substr).map(|i| (i, substr.clone())))
        .collect::<Vec<(usize, String)>>();

    let all_pos = [first_substring_pos, last_substring_pos].concat();

    let (_, first_substring) = all_pos.iter().min_by(|(i, _), (j, _)| i.cmp(j)).unwrap();
    let (_, second_substring) = all_pos.iter().max_by(|(i, _), (j, _)| i.cmp(j)).unwrap();

    let first_digit = conversion_table.get(first_substring).unwrap();
    let last_digit = conversion_table.get(second_substring).unwrap();

    format!("{first_digit}{last_digit}")
        .parse()
        .unwrap_or_else(|_| 0)
}
