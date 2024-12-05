use crate::lib::parse_input;

const INPUT_FILE: &str = "inputs/day_04.txt";
const NEEDLE: &str = "XMAS";
const NEEDLE_LEN: usize = NEEDLE.len();

fn format_input() -> Vec<Vec<char>> {
    parse_input(INPUT_FILE)
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_1() -> i32 {
    let grid: Vec<Vec<char>> = format_input();
    let len = grid.len();
    let width = grid[0].len();

    let mut count = 0;
    // sliding window of size 4
    // check row first
    for i in 0..len {
        for j in 0..width - 3 {
            let window = &grid[i][j..j + NEEDLE_LEN];
            let window: String = window.iter().collect();
            if window == NEEDLE || window.chars().rev().collect::<String>() == NEEDLE {
                count += 1;
            }
        }
    }

    // check column
    for i in 0..len - 3 {
        for j in 0..width {
            let window = [grid[i][j], grid[i + 1][j], grid[i + 2][j], grid[i + 3][j]];
            let window: String = window.iter().collect();
            if window == NEEDLE || window.chars().rev().collect::<String>() == NEEDLE {
                count += 1;
            }
        }
    }

    // check diagonal in all 4 directions
    for i in 0..len - 3 {
        for j in 0..width - 3 {
            let window = [
                grid[i][j],
                grid[i + 1][j + 1],
                grid[i + 2][j + 2],
                grid[i + 3][j + 3],
            ];
            let window: String = window.iter().collect();
            if window == NEEDLE || window.chars().rev().collect::<String>() == NEEDLE {
                count += 1;
            }
        }
        for j in 3..width {
            let window = [
                grid[i][j],
                grid[i + 1][j - 1],
                grid[i + 2][j - 2],
                grid[i + 3][j - 3],
            ];
            let window: String = window.iter().collect();
            if window == NEEDLE || window.chars().rev().collect::<String>() == NEEDLE {
                count += 1;
            }
        }
    }
    count
}

pub fn part_2() -> i32 {
    let grid: Vec<Vec<char>> = format_input();
    let len = grid.len();
    let width = grid[0].len();

    let mut count = 0;

    // could be easily implemented by comparing the window to all possible patterns
    // Only 4 patterns to match for a 3x3 sliding window
    let patterns: [&str; 4] = ["SSAMM", "MMASS", "MSAMS", "SMASM"];

    // Convert patterns to Vec<Vec<char>> for easier comparison and no need to convert to Vec<char> every time
    let patterns: Vec<Vec<char>> = patterns.iter().map(|p| p.chars().collect()).collect();

    // Check 3x3 sliding window
    for i in 0..=len - 3 {
        for j in 0..=width - 3 {
            let window = vec![
                // actualy dont event need the full window, just 5 chars
                grid[i][j],
                grid[i][j + 2],
                grid[i + 1][j + 1],
                grid[i + 2][j],
                grid[i + 2][j + 2],
            ];
            if patterns.iter().any(|pattern| *pattern == window) {
                count += 1;
            }
        }
    }
    count
}
