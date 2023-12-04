use crate::lib::parse_input;
use std::collections::HashSet;

#[rustfmt::skip]
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

pub fn part_1() -> i32 {
    let grid = generate_grid();
    let symbol_indices = extract_indices(&grid, false);
    let numbers = extract_numbers(&grid);
    let mut result = 0;
    for index in symbol_indices {
        let neighbors = get_adjacent_neighbors(index);
        let mut visited = HashSet::new();

        for neighbor in neighbors {
            if neighbor.0 < 0 || neighbor.1 < 0 {
                continue;
            }

            if let Some(row) = numbers.get(neighbor.0 as usize) {
                if let Some((num, (_, _))) = row.iter().find(|(_, (start, end))| {
                    *start as i32 <= neighbor.1 && *end as i32 >= neighbor.1
                }) {
                    if visited.contains(num) {
                        continue;
                    }
                    result += num;
                    visited.insert(*num);
                }
            }
        }
    }

    result
}

fn get_adjacent_neighbors(index: (usize, usize)) -> Vec<(i32, i32)> {
    let neighbors = DIRECTIONS
        .iter()
        .map(|(x, y)| (index.0 as i32 + x, index.1 as i32 + y))
        .collect::<Vec<(i32, i32)>>();
    neighbors
}

pub fn part_2() -> i32 {
    let grid = generate_grid();
    let gear_indices = extract_indices(&grid, true);
    let numbers = extract_numbers(&grid);

    let mut result = 0;

    for index in gear_indices {
        let neighbors = get_adjacent_neighbors(index);
        let mut visited = HashSet::new();
        let mut gears = vec![];

        for neighbor in neighbors {
            if neighbor.0 < 0 || neighbor.1 < 0 {
                continue;
            }

            if let Some(row) = numbers.get(neighbor.0 as usize) {
                if let Some((num, (_, _))) = row.iter().find(|(_, (start, end))| {
                    *start as i32 <= neighbor.1 && *end as i32 >= neighbor.1
                }) {
                    if visited.contains(num) {
                        continue;
                    }
                    gears.push(*num);
                    visited.insert(*num);
                }
            }
        }

        if gears.len() == 2 {
            result += gears[0] * gears[1];
        }
    }

    result
}

fn generate_grid() -> Vec<Vec<char>> {
    parse_input("inputs/day_03.txt")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn extract_numbers(grid: &[Vec<char>]) -> Vec<Vec<(i32, (usize, usize))>> {
    grid.iter()
        .enumerate()
        .map(|(_row_index, row)| {
            let (mut row_numbers, mut current_number, mut start_index) =
                (Vec::new(), String::new(), 0);

            for (i, &ch) in row.iter().enumerate() {
                match ch.is_ascii_digit() {
                    true => {
                        if current_number.is_empty() {
                            start_index = i;
                        }
                        current_number.push(ch);
                    }
                    false => {
                        if !current_number.is_empty() {
                            if let Ok(num) = current_number.parse::<i32>() {
                                row_numbers.push((num, (start_index, i - 1)));
                            }
                            current_number.clear();
                        }
                    }
                }
            }

            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<i32>() {
                    row_numbers.push((num, (start_index, row.len() - 1)));
                }
            }

            row_numbers
        })
        .collect()
}
fn extract_indices(grid: &[Vec<char>], part2: bool) -> HashSet<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter().enumerate().filter_map(move |(col_index, &ch)| {
                match (part2, is_symbol(ch)) {
                    (true, true) => Some((row_index, col_index)),
                    (false, true) => Some((row_index, col_index)),
                    _ => None,
                }
            })
        })
        .collect()
}
