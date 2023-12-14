use std::collections::HashMap;
use crate::lib::parse_input;

const CYCLES: i32 = 1_000_000_000;

pub fn part_1() -> i32 {
    let grid = parse_grid();
    let mut tilted_grid = grid.clone();
    move_stones_north(&mut grid.clone(), &mut tilted_grid);
    calc_north_load(&grid, &tilted_grid)
}

pub fn part_2() -> i32 {
    let grid = parse_grid();
    let mut tilted_grid = grid.clone();
    let mut first: i32 = -1;
    let mut cycle_length = 0;
    let mut position = 0;
    let mut my_result = String::new();
    let mut store = HashMap::new();

    for i in 0..CYCLES {
        let mut new_tilted = tilted_grid.clone();

        move_stones_north(&mut tilted_grid.clone(), &mut new_tilted);
        tilted_grid = new_tilted.clone();

        move_stones_west(&mut tilted_grid.clone(), &mut new_tilted);
        tilted_grid = new_tilted.clone();

        move_stones_south(&mut tilted_grid.clone(), &mut new_tilted, &grid);
        tilted_grid = new_tilted.clone();

        move_stones_east(&mut tilted_grid.clone(), &mut new_tilted, &grid);
        tilted_grid = new_tilted.clone();

        let grid_string: String = new_tilted.iter().flat_map(|line| line.iter()).collect();

        if let Some(prev_counter) = store.get(&grid_string) {
            // found begin of cycle
            if first == -1 {
                first = i;
                cycle_length = i - prev_counter;
                position = (CYCLES - first) % cycle_length - 1;
            }
            if i == first + position {
                my_result = grid_string.clone();
                break;
            }
        } else {
            store.insert(grid_string, i);
        }
    }
    let found_grid = grid_string_to_grid(&grid, &my_result);
    calc_north_load(&grid, &found_grid)
}

fn move_stones_north(grid: &mut [Vec<char>], tilted_grid: &mut [Vec<char>]) {
    for (y, line) in grid.iter().enumerate().skip(1) {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                let mut c = 0;
                while y - c > 0 {
                    match tilted_grid[y - c - 1][x] {
                        '.' => {
                            tilted_grid[y - c - 1][x] = 'O';
                            tilted_grid[y - c][x] = '.';
                            c += 1;
                        },
                        _ => break,
                    }
                }
            }
        }
    }
}

fn move_stones_west(tilted_grid: &mut [Vec<char>], new_tilted: &mut [Vec<char>]) {
    for (y, line) in tilted_grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                let mut c = 0;
                while x - c > 0 {
                    match new_tilted[y][x - c - 1] {
                        '.' => {
                            new_tilted[y][x - c - 1] = 'O';
                            new_tilted[y][x - c] = '.';
                            c += 1;
                        },
                        _ => break,
                    }
                }
            }
        }
    }
}

fn move_stones_south(tilted_grid: &mut [Vec<char>], new_tilted: &mut [Vec<char>], grid: &[Vec<char>]) {
    for (y, line) in tilted_grid.iter().enumerate().rev().skip(1) {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                let mut c = 0;
                while y + c < grid.len() - 1 {
                    match new_tilted[y + c + 1][x] {
                        '.' => {
                            new_tilted[y + c + 1][x] = 'O';
                            new_tilted[y + c][x] = '.';
                            c += 1;
                        },
                        _ => break,
                    }
                }
            }
        }
    }
}

fn move_stones_east(tilted_grid: &mut [Vec<char>], new_tilted: &mut [Vec<char>], grid: &[Vec<char>]) {
    for (y, line) in tilted_grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate().rev().skip(1) {
            if *c == 'O' {
                let mut c = 0;
                while x + c < grid.len() - 1 {
                    match new_tilted[y][x + c + 1] {
                        '.' => {
                            new_tilted[y][x + c + 1] = 'O';
                            new_tilted[y][x + c] = '.';
                            c += 1;
                        },
                        _ => break,
                    }
                }
            }
        }
    }
}

fn grid_string_to_grid(grid: &[Vec<char>], found_string: &str) -> Vec<Vec<char>> {
    let found_grid: Vec<Vec<char>> = found_string.chars().collect::<Vec<char>>().chunks(grid.len()).map(|line| line.to_vec()).collect();
    found_grid
}

fn parse_grid() -> Vec<Vec<char>> {
    let grid = parse_input("inputs/day_14.txt").iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    grid
}

fn calc_north_load(grid: &[Vec<char>], tilted_grid: &[Vec<char>]) -> i32 {
    let mut score = 0;
    for i in 0..tilted_grid.len() {
        score += tilted_grid[i].iter().filter(|c| **c == 'O').count() * (grid.len() - i);
    }
    score as i32
}