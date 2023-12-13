use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let patterns: Vec<Vec<String>> = parse_input("inputs/day_13.txt")
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|group| group.to_vec())
        .collect();

    let mut result = 0;

    for pattern in patterns {
        for (i, row) in pattern.iter().enumerate().take(pattern.len() - 1) {
            if row == &pattern[i + 1] && is_row_symmetrical(&pattern, i, i + 1) {
                result += (i + 1) * 100;
            }
        }

        for i in 0..pattern[0].len() - 1 {
            let column1 = extract_column(&pattern, i);
            let column2 = extract_column(&pattern, i + 1);
            if column1 == column2 && is_column_symmetrical(&pattern, i, i + 1) {
                result += i + 1;
            }
        }
    }
    result as i32
}

pub fn part_2() -> i32 {
    0
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

fn extract_column(pattern: &[String], index: usize) -> String {
    pattern
        .iter()
        .map(|row| row.chars().nth(index).unwrap())
        .collect::<String>()
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
