use std::collections::HashMap;
use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let patterns = generate_patterns();
    patterns.iter().map(calculate_symmetry_score).sum()
}

pub fn part_2() -> i32 {
    let mut patterns = generate_patterns();
    let map = build_symmetry_map(&patterns);
    patterns.iter_mut().enumerate().map(|(index, pattern)| process_pattern(pattern, &map, index)).sum()
}

fn generate_patterns() -> Vec<Vec<String>> {
    let patterns: Vec<Vec<String>> = parse_input("inputs/day_13.txt")
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|group| group.to_vec())
        .collect();
    patterns
}

fn build_symmetry_map(patterns: &[Vec<String>]) -> HashMap<usize, (usize, char)> {
    let mut map = HashMap::new();
    for (index, pattern) in patterns.iter().enumerate() {
        for (i, row) in pattern.iter().enumerate().take(pattern.len() - 1) {
            if row == &pattern[i + 1] && is_row_symmetrical(pattern, i, i + 1) {
                map.insert(index, (i, 'r'));
            }
        }
        for i in 0..pattern[0].len() - 1 {
            if is_column_symmetrical(pattern, i, i + 1) {
                map.insert(index, (i, 'c'));
            }
        }
    }
    map
}

fn process_pattern(pattern: &mut Vec<String>, map: &HashMap<usize, (usize, char)>, index: usize) -> i32 {
    let mut score = 0;

    for x in 0..pattern.len() {
        for y in 0..pattern[0].len() {
            flip_char_at(pattern, x, y);
            if check_and_update_symmetry(pattern, map, index, &mut score) {
                return score;
            }
            flip_char_at(pattern, x, y); // Reset pattern
        }
    }
    score
}

fn check_and_update_symmetry(pattern: &Vec<String>, map: &HashMap<usize, (usize, char)>, index: usize, score: &mut i32) -> bool {
    let cached = match map.get(&index) {
        Some(cached) => cached,
        None => return false,
    };

    for (i, row) in pattern.iter().enumerate().take(pattern.len() - 1) {
        if row == &pattern[i + 1] && is_row_symmetrical(pattern, i, i + 1) && (cached.0 != i || cached.1 != 'r') {
            *score += (i as i32 + 1) * 100;
            return true; // Early exit
        }
    }

    for i in 0..pattern[0].len() - 1 {
        if is_column_symmetrical(pattern, i, i + 1) && (cached.0 != i || cached.1 != 'c') {
            *score += i as i32 + 1;
            return true; // Early exit
        }
    }
    false
}

fn extract_column(pattern: &[String], index: usize) -> String {
    pattern
        .iter()
        .map(|row| row.chars().nth(index).unwrap())
        .collect::<String>()
}

fn is_row_symmetrical(pattern: &[String], start_index: usize, end_index: usize) -> bool {
    let mut up = start_index as i32;
    let mut down = end_index as i32;

    while up >= 0 && (down as usize) < pattern.len() {
        if pattern[up as usize] != pattern[down as usize] {
            return false;
        }
        up -= 1;
        down += 1;
    }
    true
}

fn is_column_symmetrical(pattern: &[String], left_index: usize, right_index: usize) -> bool {
    let mut left = left_index as i32;
    let mut right = right_index as i32;

    while left >= 0 && right < pattern[0].len() as i32 {
        let column_left = extract_column(pattern, left as usize);
        let column_right = extract_column(pattern, right as usize);

        if column_left != column_right {
            return false;
        }
        left -= 1;
        right += 1;
    }
    true
}

fn flip_char_at(pattern: &mut [String], x: usize, y: usize) {
    let c = pattern[x].chars().nth(y).unwrap();
    match c {
        '#' => pattern[x].replace_range(y..y + 1, "."),
        '.' => pattern[x].replace_range(y..y + 1, "#"),
        _ => panic!("Invalid character at position ({}, {})", x, y),
    }
}

fn calculate_symmetry_score(pattern: &Vec<String>) -> i32 {
    let mut score = 0;
    for (i, row) in pattern.iter().enumerate().take(pattern.len() - 1) {
        if row == &pattern[i + 1] && is_row_symmetrical(pattern, i, i + 1) {
            score += (i + 1) * 100;
        }
    }
    for i in 0..pattern[0].len() - 1 {
        let column1 = extract_column(pattern, i);
        let column2 = extract_column(pattern, i + 1);
        if column1 == column2 && is_column_symmetrical(pattern, i, i + 1) {
            score += i + 1;
        }
    }
    score as i32
}